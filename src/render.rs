//! Transform sampling and the tiled frame renderer, per document 21 and ADR-011.
//!
//! Two contracts meet here. Document 21's "Coordinate system" and "Resampling and outside
//! bounds" fix the maths: origin top-left, pixel `(i,j)` centred at `(i+0.5, j+0.5)`, the
//! transform order `T(position) * R(rotation) * S(scale) * T(-anchor)`, sampling by the
//! inverse transform from destination pixel centre into source space, bilinear weights taken
//! from source pixel centres, and transparent black outside the source extent. Document 21's
//! tile contract fixes the execution: the frame is cut into tiles that are evaluated
//! independently and never depend on one another, and "a tiled render and a hypothetical
//! whole-frame render of the same request must be byte-identical".
//!
//! Determinism is structural rather than tested-in. A tile owns its own accumulator, reads
//! only immutable source buffers, and is copied into the frame at a position that does not
//! depend on when it finished. Nothing accumulates across tiles, so no float addition changes
//! order with thread count. `tests/b05a_transform.rs` proves it anyway, across thread counts
//! and tile sizes, because ADR-011 says B-05a proves this rather than assuming it.
//!
//! Steps 2 and 5 of document 21 -- the polygon mask, in layer space, and the alpha matte, in
//! composition space -- arrived with B-06. The matte is sampled through its own inverse transform
//! rather than the drawn layer's, because document 21 evaluates the matte layer through its own
//! source, mask and transform at the same frame.
//!
//! Not here: step 3, effects, which is B-07. A layer carrying effects renders without them and
//! says so in the frame log rather than rendering silently.

use rayon::prelude::*;

use crate::WorkingBuffer;

/// A 2D affine map, `x' = a*x + c*y + tx`, `y' = b*x + d*y + ty`.
///
/// Stored rather than composed from parameters at sample time so that the whole
/// `T*R*S*T` chain of document 21 is built once per layer and inverted once per layer.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Affine {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub tx: f64,
    pub ty: f64,
}

impl Affine {
    pub const IDENTITY: Affine = Affine {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        tx: 0.0,
        ty: 0.0,
    };

    pub fn translation(tx: f64, ty: f64) -> Affine {
        Affine {
            tx,
            ty,
            ..Affine::IDENTITY
        }
    }

    pub fn scaling(sx: f64, sy: f64) -> Affine {
        Affine {
            a: sx,
            d: sy,
            ..Affine::IDENTITY
        }
    }

    /// Document 21: "Positive rotation is clockwise in the screen-coordinate system."
    ///
    /// With +y downward the ordinary counter-clockwise matrix reads as clockwise on screen,
    /// so this is the textbook form and not its transpose.
    pub fn rotation_degrees(degrees: f64) -> Affine {
        let (sin, cos) = degrees.to_radians().sin_cos();
        Affine {
            a: cos,
            b: sin,
            c: -sin,
            d: cos,
            tx: 0.0,
            ty: 0.0,
        }
    }

    /// Apply `self` first, then `outer`.
    pub fn then(self, outer: Affine) -> Affine {
        outer.compose(self)
    }

    fn compose(self, rhs: Affine) -> Affine {
        Affine {
            a: self.a * rhs.a + self.c * rhs.b,
            b: self.b * rhs.a + self.d * rhs.b,
            c: self.a * rhs.c + self.c * rhs.d,
            d: self.b * rhs.c + self.d * rhs.d,
            tx: self.a * rhs.tx + self.c * rhs.ty + self.tx,
            ty: self.b * rhs.tx + self.d * rhs.ty + self.ty,
        }
    }

    pub fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        (
            self.a * x + self.c * y + self.tx,
            self.b * x + self.d * y + self.ty,
        )
    }

    /// `None` when the map is singular, which is a scale of zero on either axis. A layer
    /// scaled to nothing has no source pixel behind any destination pixel; the renderer
    /// treats that as an empty draw rather than dividing by zero.
    pub fn invert(&self) -> Option<Affine> {
        let det = self.a * self.d - self.b * self.c;
        if det == 0.0 || !det.is_finite() {
            return None;
        }
        let inv = 1.0 / det;
        Some(Affine {
            a: self.d * inv,
            b: -self.b * inv,
            c: -self.c * inv,
            d: self.a * inv,
            tx: (self.c * self.ty - self.d * self.tx) * inv,
            ty: (self.b * self.tx - self.a * self.ty) * inv,
        })
    }

    /// Document 21: `p_comp = T(position) * R(rotation) * S(scale) * T(-anchor) * p_layer`.
    ///
    /// Scale is a unit factor here: 1.0 is identity. Document 21 writes `S(scale/100)`, which
    /// reads the stored number as a percentage; document 19 says scale "is percentage-like in
    /// UI but serialized as explicit numeric pairs". D-22 resolves the two in favour of the
    /// unit factor and puts the divide by 100 at the UI boundary. Rotation is in degrees,
    /// anchor and position in pixels, all as document 19 states.
    pub fn from_transform(
        anchor: (f64, f64),
        position: (f64, f64),
        scale: (f64, f64),
        rotation_degrees: f64,
    ) -> Affine {
        Affine::translation(-anchor.0, -anchor.1)
            .then(Affine::scaling(scale.0, scale.1))
            .then(Affine::rotation_degrees(rotation_degrees))
            .then(Affine::translation(position.0, position.1))
    }
}

/// Bilinear sample of a premultiplied linear-light buffer at continuous coordinates.
///
/// Document 21: "Bilinear weights are computed from source pixel-center coordinates" and
/// "Samples outside the source extent are transparent black". Both are literal here: the
/// neighbour indices come from `floor(x - 0.5)`, and a neighbour off the edge contributes its
/// weight times `(0,0,0,0)` rather than being dropped or clamped to the edge pixel. Clamping
/// would smear the border outward; dropping would renormalise the weights and brighten the
/// edge. Working in premultiplied RGBA is what keeps a zero-alpha neighbour from dragging a
/// meaningless straight colour into the result.
pub fn sample_bilinear(src: &WorkingBuffer, x: f64, y: f64) -> [f32; 4] {
    let (w, h) = (src.width() as isize, src.height() as isize);
    let (fx, fy) = (x - 0.5, y - 0.5);
    let (x0, y0) = (fx.floor(), fy.floor());
    let (ux, uy) = (fx - x0, fy - y0);
    let (x0, y0) = (x0 as isize, y0 as isize);

    let mut out = [0.0f32; 4];
    for (dy, wy) in [(0isize, 1.0 - uy), (1, uy)] {
        let sy = y0 + dy;
        if wy == 0.0 || sy < 0 || sy >= h {
            continue;
        }
        for (dx, wx) in [(0isize, 1.0 - ux), (1, ux)] {
            let sx = x0 + dx;
            if wx == 0.0 || sx < 0 || sx >= w {
                continue;
            }
            let weight = (wx * wy) as f32;
            let px = src.pixel(sx as usize, sy as usize);
            for i in 0..4 {
                out[i] += px[i] * weight;
            }
        }
    }
    out
}

/// One layer's contribution to a frame: a source already in the working space, the map from
/// its pixels into composition pixels, and document 21's step 6, animated layer opacity.
#[derive(Clone, Debug)]
pub struct LayerDraw {
    /// The model layer this draw came from. Carried so ADR-012's trace mode can tag an
    /// intermediate image with the layer it belongs to; the renderer itself never reads it.
    pub id: crate::model::Id,
    pub source: std::sync::Arc<WorkingBuffer>,
    pub transform: Affine,
    pub opacity: f32,
    /// Document 21's step 5, the alpha matte, or `None` for a layer that has no matte.
    ///
    /// Step 5 is after the transform at step 4, so unlike the mask this cannot be baked into the
    /// source: "Alpha matte coverage is the matte layer's post-transform alpha sampled at the
    /// destination pixel." Both layers are sampled at the same composition pixel, each through
    /// its own transform, which is exactly what makes a matte follow the matte layer's animation
    /// rather than the masked layer's.
    pub matte: Option<Box<MatteDraw>>,
    /// Document 21's step 7. Applied after opacity, against whatever is already accumulated.
    pub blend: crate::model::BlendMode,
    /// D-66: `Some` makes this an adjustment layer. `source` is then its shape, an opaque
    /// composition-sized rectangle with its mask cut out, and the stack runs on the frame drawn
    /// so far rather than on `source`; where the shape's coverage `c` reaches, the frame becomes
    /// `B + c*(E(B) - B)`. `blend` is normal and is not read.
    pub adjust: Option<Vec<crate::effects::EffectInstance>>,
    /// D-67: `Some` for a composition layer: the composition `source` is a render of, and the
    /// frame of it. The renderer does not read it; the trace names it.
    pub nested: Option<(crate::model::Id, i32)>,
    /// B-46, B-47: the effects the layer's stack ends in that are left for the graphics card
    /// (`compose::plan_frame_for_card`), in stack order (B-155): `source` is the drawing before
    /// them. The CPU runs them itself in [`render`], so a plan made for the card is the same frame
    /// on either.
    /// D-132: the layer's Light Wraps at this frame, in stack order, disabled and bypassed ones
    /// included. They read the frame beneath the layer, so they run as the layer is drawn onto
    /// it ([`wrap_layer`]), after its other effects, mask, transform and matte and before its
    /// opacity and blend. Empty for an adjustment layer, which has no drawing to wrap.
    pub wrap: Vec<crate::effects::EffectInstance>,
    pub on_card: Vec<OnCard>,
    /// D-188: `source`, or the matte's, is already the average of a motion-blurred layer's
    /// moments. The renderer does not read it; the graphics card hands such a frame to the CPU
    /// when it has no effect of its own to draw (B-153b).
    pub motion_blur: bool,
    /// D-216: `source`, or the matte's, is a frame mix or a drawing dissolve. Read as
    /// `motion_blur` is.
    pub mixed: bool,
    /// D-226 (B-156b): in a plan made for the card, a motion-blurred layer's moments, each the
    /// map from `source` into the plan's pixels, left for the card or [`render`] to average
    /// ([`average`]); `transform` is then the average's, the identity once the plan is scaled.
    /// Empty everywhere else, where the average is already `source`.
    pub moments: Vec<Option<Affine>>,
}

/// D-188: a motion-blurred layer's moments drawn one by one onto a clear `width` by `height`
/// picture, summed in order and divided by their number once, in linear premultiplied light. A
/// moment that is `None`, behind the camera, counts and adds nothing.
pub fn average(source: &WorkingBuffer, moments: &[Option<Affine>], width: usize, height: usize) -> WorkingBuffer {
    // P-23: each moment is summed straight from the drawing, pixel by pixel as `render_tile`
    // draws one layer onto a clear frame, without drawing a whole frame for it. The pixels
    // skipped are those whose bilinear footprint lies wholly outside the part of the drawing
    // that holds anything but +0, the drawing's `shown` rectangle: there the moment is exactly
    // +0 (P-05's culling rule), and a sum of what is drawn is never -0, so adding it would change
    // no bit. Each value is its own sum, so spread over the threads it is the same bits too;
    // `verification/P-23_fourth_batch_audit.md` compares the frames.
    let (sw, sh) = (source.width(), source.height());
    let held = |p: &[f32]| p.iter().any(|v| v.to_bits() != 0);
    let rows: Vec<usize> = (0..sh).into_par_iter().filter(|&y| held(&source.data()[y * sw * 4..(y + 1) * sw * 4])).collect();
    let cols: Vec<usize> = (0..sw)
        .into_par_iter()
        .filter(|&x| rows.iter().any(|&y| held(&source.data()[(y * sw + x) * 4..][..4])))
        .collect();
    let mut sum = WorkingBuffer::transparent(width, height);
    if let (Some(&t), Some(&b), Some(&l), Some(&r)) = (rows.first(), rows.last(), cols.first(), cols.last()) {
        let (l, t, r, b) = (l as f64 - 1.0, t as f64 - 1.0, r as f64 + 2.0, b as f64 + 2.0);
        for transform in moments.iter().flatten() {
            let Some(inverse) = transform.invert() else { continue };
            let corners = [transform.apply(l, t), transform.apply(r, t), transform.apply(l, b), transform.apply(r, b)];
            let low = corners.iter().fold((f64::INFINITY, f64::INFINITY), |m, c| (m.0.min(c.0), m.1.min(c.1)));
            let high = corners.iter().fold((f64::NEG_INFINITY, f64::NEG_INFINITY), |m, c| (m.0.max(c.0), m.1.max(c.1)));
            let clip = |v: f64, n: usize| (v.max(0.0) as usize).min(n);
            let (x0, y0, x1, y1) = if [low.0, low.1, high.0, high.1].iter().all(|v| v.is_finite()) {
                (clip(low.0.floor(), width), clip(low.1.floor(), height), clip(high.0.ceil(), width), clip(high.1.ceil(), height))
            } else {
                (0, 0, width, height)
            };
            if x0 >= x1 || y0 >= y1 {
                continue;
            }
            sum.data_mut()[y0 * width * 4..y1 * width * 4]
                .par_chunks_mut(width * 4)
                .enumerate()
                .for_each(|(row, out)| {
                    for x in x0..x1 {
                        let (dx, dy) = (x as f64 + 0.5, (y0 + row) as f64 + 0.5);
                        let (sx, sy) = inverse.apply(dx, dy);
                        let one = crate::composite::blend_pixel(crate::model::BlendMode::Normal, sample_bilinear(source, sx, sy), [0.0; 4]);
                        for c in 0..4 {
                            out[x * 4 + c] += one[c];
                        }
                    }
                });
        }
    }
    let n = moments.len() as f32;
    sum.data_mut().par_iter_mut().for_each(|s| *s /= n);
    sum
}

/// An effect left for the graphics card, in the pixels of the buffer it runs on.
#[derive(Clone, Debug, PartialEq)]
pub enum OnCard {
    Radial(Radial),
    Bloom(Bloom),
    Directional(Directional),
    Gaussian(Gaussian),
    Glow(Glow),
    Fx(Fx),
}

/// B-65: one of the batch of ten (D-122), its distances already divided for Draft, with the
/// drawing's corner in the buffer after the effects before it grew it, and how far it grows it
/// across and down (B-115: a Motion Tile's two can differ).
#[derive(Clone, Debug, PartialEq)]
pub struct Fx {
    pub instance: crate::effects::EffectInstance,
    pub origin: (usize, usize),
    pub grow: (usize, usize),
}

/// B-51: a Glow's settings (`glow::settings`), the radius already divided for Draft. Left for the
/// card only when it glows somewhere, so the drawing grows by exactly the blur's kernel radius.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Glow {
    /// Bright parts, over `threshold` percent; otherwise the first `count` of `targets`, within
    /// `tolerance` 8-bit levels.
    pub bright: bool,
    pub threshold: f64,
    pub targets: [[u8; 3]; 8],
    pub count: usize,
    pub tolerance: f64,
    pub radius: f64,
    pub intensity: f64,
    /// Screen rather than add.
    pub screen: bool,
    /// The 8-bit colour every glowing pixel's light takes, if any.
    pub tint: Option<[u8; 3]>,
    /// D-322: After Effects' radius and strength rather than D-89's.
    pub after_effects: bool,
}

/// B-50: a Gaussian Blur's sigma (`effects::blur`), already divided for Draft and large enough
/// to reach a pixel, so the drawing grows by exactly `effects::kernel_radius` of it, or (D-109)
/// with `repeat`, by nothing (`effects::held_blur`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gaussian {
    pub sigma: f64,
    pub repeat: bool,
    /// D-321: the long reach of a Gaussian Blur in Blurriness, `effects::reach_radius`.
    pub long: bool,
}

impl Gaussian {
    /// How far the drawing grows on each side.
    pub fn grow(&self) -> usize {
        if self.repeat { 0 } else { crate::effects::reach_radius(self.sigma, self.long) }
    }
}

/// B-49: a Directional Blur's settings (`blurs::directional_blur`), the length already divided
/// for Draft and never 0, so the drawing grows by exactly half the length, rounded up, or
/// (D-109) with `repeat`, by nothing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Directional {
    pub direction: f64,
    pub length: f64,
    pub repeat: bool,
}

impl Directional {
    /// How far the drawing grows on each side.
    pub fn grow(&self) -> usize {
        if self.repeat { 0 } else { (self.length / 2.0).ceil() as usize }
    }
}

/// B-47: a Bloom's settings (`bloom::bloom`), distances already divided for Draft. Left for the
/// card only when it lights something, so the drawing grows by exactly `bloom::reach`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bloom {
    pub threshold: f64,
    pub radius: f64,
    pub intensity: f64,
    pub lines: usize,
    pub length: f64,
    pub angle: f64,
}

/// B-46: a Radial Blur's settings in the pixels of the buffer it runs on (`blurs::radial_blur`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Radial {
    pub spin: bool,
    pub amount: f64,
    pub center: (f64, f64),
    /// D-109.
    pub repeat: bool,
}

/// The matte layer as the renderer needs it: a source in the working space and the map from its
/// pixels into composition pixels.
///
/// Document 21: "The matte layer is evaluated through its own source, mask, effects and
/// transform at the same frame." Its own mask is already baked into `source` by the time this is
/// built, the same way a drawn layer's is. Effects are B-07 and are not here yet.
///
/// It carries no opacity and no blend mode on purpose. A matte contributes alpha, and document
/// 21 says which alpha: the post-transform alpha of the matte layer. Layer opacity is step 6 and
/// blending is step 7, both of which are about how a layer joins the stack it is drawn into —
/// and a matte-only layer is not drawn into the stack at all.
#[derive(Clone, Debug)]
pub struct MatteDraw {
    pub source: std::sync::Arc<WorkingBuffer>,
    pub transform: Affine,
}

/// One frame of work: the output extent and the layers, bottom of the stack first.
#[derive(Clone, Debug)]
pub struct FramePlan {
    pub width: usize,
    pub height: usize,
    /// D-319: the composition works in Float depth, so Add and Screen are not held to white.
    pub float: bool,
    pub layers: Vec<LayerDraw>,
}

/// A rectangular region of the output frame, evaluated independently of every other tile.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

/// Whether any pixel of `tile` can sample anything but zero out of this layer.
///
/// The source rectangle grown by one pixel on every side - the bilinear footprint, since a
/// sample at `w + 1` or beyond has no source pixel with a nonzero weight - mapped through the
/// layer's own transform, and its bounding box tested against the tile. The map is affine, so
/// the image of the grown rectangle is a parallelogram and every destination whose source lands
/// inside the rectangle is inside it, which is what makes the bounding-box test safe rather than
/// merely likely.
///
/// An effect's `bounds_expansion` needs no term here: document 21 has the stack run whole-layer
/// before the frame plan (ADR-017), so a blur's growth is already pixels of `layer.source` by
/// the time the renderer sees it. The exceptions are a Bloom (B-47), a Directional Blur (B-49), a
/// Gaussian Blur (B-50) and a Glow (B-51) left for the card, which grow the drawing on every side, so the box counts that growth.
/// A layer's box in frame pixels: `(left, top, right, bottom)`.
///
/// Computed once per layer per frame, never per tile: the corners do not change between the
/// tiles of one frame, and on the fixtures the box excludes nothing, so every recomputation
/// would have been spent on a skip that never fires.
pub fn bounds(layer: &LayerDraw) -> (f64, f64, f64, f64) {
    let even = |g: usize| (2 * g, 2 * g);
    let (gx, gy) = layer.on_card.iter().fold((0, 0), |(x, y), card| {
        let (gx, gy) = match card {
            OnCard::Bloom(b) => even(crate::bloom::reach(b.radius, b.lines, b.length)),
            OnCard::Directional(d) => even(d.grow()),
            OnCard::Gaussian(g) => even(g.grow()),
            OnCard::Glow(g) => even(crate::effects::kernel_radius(g.radius / 3.0)),
            OnCard::Fx(f) => (2 * f.grow.0, 2 * f.grow.1),
            OnCard::Radial(_) => (0, 0),
        };
        (x + gx, y + gy)
    });
    let (w, h) = ((layer.source.width() + gx) as f64, (layer.source.height() + gy) as f64);
    let corners = [
        layer.transform.apply(-1.0, -1.0),
        layer.transform.apply(w + 1.0, -1.0),
        layer.transform.apply(-1.0, h + 1.0),
        layer.transform.apply(w + 1.0, h + 1.0),
    ];
    (
        corners.iter().fold(f64::INFINITY, |m, c| m.min(c.0)),
        corners.iter().fold(f64::INFINITY, |m, c| m.min(c.1)),
        corners.iter().fold(f64::NEG_INFINITY, |m, c| m.max(c.0)),
        corners.iter().fold(f64::NEG_INFINITY, |m, c| m.max(c.1)),
    )
}

fn stencil(layer: &LayerDraw) -> bool {
    matches!(layer.blend, crate::model::BlendMode::StencilAlpha | crate::model::BlendMode::StencilLuma)
}

/// Whether a layer's box meets a tile. Public so that `tests/p05_culling.rs` can count how many
/// layer-and-tile pairs the box actually excludes: a table saying the frame is unchanged reads
/// the same whether the skip works or never fires, and the count is what tells those two apart.
pub fn reaches(bounds: (f64, f64, f64, f64), tile: Tile) -> bool {
    let (left, top, right, bottom) = bounds;
    // A NaN corner - a transform built from one - compares false everywhere, and a layer whose
    // box cannot be computed is drawn rather than skipped.
    !(right <= tile.x as f64
        || left >= (tile.x + tile.width) as f64
        || bottom <= tile.y as f64
        || top >= (tile.y + tile.height) as f64)
}

/// Cut an extent into tiles of at most `size` on a side. Edge tiles are short, not padded.
///
/// Document 21: "Tile size is a tunable measured on the reference machine, not a constant
/// chosen in advance", so this takes it as an argument and no default lives in this file.
pub fn tiles(width: usize, height: usize, size: usize) -> Vec<Tile> {
    let size = size.max(1);
    let mut out = Vec::new();
    let mut y = 0;
    while y < height {
        let h = size.min(height - y);
        let mut x = 0;
        while x < width {
            out.push(Tile {
                x,
                y,
                width: size.min(width - x),
                height: h,
            });
            x += size;
        }
        y += h;
    }
    out
}

/// Render one frame, tiled, across the current rayon thread pool.
///
/// Wrap the call in `pool.install(...)` to fix the worker count; the result does not depend on
/// it. The only shared state is the immutable plan and the frame, and no two tiles are ever
/// handed the same pixel of it: the frame is carved into one disjoint set of row slices per tile
/// **before any thread starts**, which is the property `tests/b05a_transform.rs` proves for
/// ADR-011, kept exactly as it was when each tile owned a buffer of its own (P-03(d)).
///
/// A tile used to accumulate into a buffer it allocated and that buffer was then copied into a
/// separately zeroed frame. It now accumulates into the frame, which starts at the same zero the
/// buffer did, so the arithmetic per pixel is unchanged and one 33 MB allocation, one 33 MB
/// zero-fill and one 33 MB copy a frame are not done at all.
pub fn render(plan: &FramePlan, tile_size: usize) -> WorkingBuffer {
    render_maybe_culled(plan, tile_size, true, None)
}

/// B-158 (G8): only `part` of the frame, a buffer `part.width` by `part.height`, byte for byte
/// those pixels of [`render`]. Each pixel is sampled at its own place in the frame, and the
/// arithmetic per pixel does not depend on the tiles (B-07), so drawing fewer tiles changes
/// nothing but the clock. A frame with an adjustment layer or a Light Wrap, which read the whole
/// frame drawn so far, is drawn whole and cut. The viewer's alone: an export never asks for it.
pub fn render_part(plan: &FramePlan, tile_size: usize, part: Tile) -> WorkingBuffer {
    render_maybe_culled(plan, tile_size, true, Some(part))
}

/// The same frame with P-05's culling test turned off, which is what
/// `verification/P-05_culling_table.md` compares against.
///
/// It exists for that comparison and for nothing else. A skipped layer is only correct if the
/// frame is the same frame without the skip, and the only way to say that in bytes is to render
/// it both ways.
pub fn render_without_culling(plan: &FramePlan, tile_size: usize) -> WorkingBuffer {
    render_maybe_culled(plan, tile_size, false, None)
}

fn render_maybe_culled(plan: &FramePlan, tile_size: usize, cull: bool, part: Option<Tile>) -> WorkingBuffer {
    let (mut frame, at, cut) = canvas(plan, part);
    draw(plan, 0, &mut frame, at, None, tile_size, cull);
    cut_out(frame, cut)
}

/// B-158: what a frame, or `part` of it, is drawn on: a clear picture, where its first pixel is
/// in the whole frame, and the part to cut from it once drawn. A frame with an adjustment layer
/// or a Light Wrap, which read the whole frame drawn so far, is drawn whole and cut.
fn canvas(plan: &FramePlan, part: Option<Tile>) -> (WorkingBuffer, (usize, usize), Option<Tile>) {
    let whole_frame_read = plan.layers.iter().any(|l| l.adjust.is_some() || l.wrap.iter().any(|i| i.enabled && i.is_valid()));
    match part {
        Some(p) if !whole_frame_read => (WorkingBuffer::transparent(p.width, p.height), (p.x, p.y), None),
        _ => (WorkingBuffer::transparent(plan.width, plan.height), (0, 0), part),
    }
}

/// `part` of `frame`, or `frame` itself.
fn cut_out(frame: WorkingBuffer, part: Option<Tile>) -> WorkingBuffer {
    let Some(part) = part else {
        return frame;
    };
    let mut cut = WorkingBuffer::transparent(part.width, part.height);
    for (y, row) in cut.data_mut().chunks_exact_mut(part.width * 4).enumerate() {
        let from = ((part.y + y) * frame.width() + part.x) * 4;
        row.copy_from_slice(&frame.data()[from..from + part.width * 4]);
    }
    cut
}

/// Layers `from..` of `plan` drawn onto `frame`, which holds layers `..from` already; `at` is
/// where `frame`'s first pixel is in the whole frame. With `keep`, a layer past `from`, the frame
/// as it stood below that layer is returned as well (B-159).
///
/// D-66: the frame is drawn in segments, each ending at an adjustment layer, whose stack runs on
/// the whole frame drawn so far before the next segment is drawn onto it. D-132: a layer with a
/// Light Wrap that runs ends one too, as it reads the frame beneath it. Where a segment is cut
/// changes nothing: each layer is laid on what is beneath it, pixel by pixel, in order.
fn draw(
    plan: &FramePlan,
    from: usize,
    frame: &mut WorkingBuffer,
    at: (usize, usize),
    keep: Option<usize>,
    tile_size: usize,
    cull: bool,
) -> Option<WorkingBuffer> {
    let mut layers = std::borrow::Cow::Borrowed(&plan.layers[from..]);
    // B-46, B-47, B-49, B-50, B-51, B-65: an effect left for the card that the CPU is drawing after all is run first,
    // exactly as `apply_stack` would have run it.
    // B-156b: as are the moments of a motion-blurred layer.
    if layers.iter().any(|l| !l.on_card.is_empty() || !l.moments.is_empty()) {
        for layer in layers.to_mut().iter_mut() {
            if !layer.moments.is_empty() {
                let moments = std::mem::take(&mut layer.moments);
                layer.source = std::sync::Arc::new(average(&layer.source, &moments, plan.width, plan.height));
            }
            // B-155: in stack order, each on what the one before it drew.
            for card in std::mem::take(&mut layer.on_card) {
                match card {
                    OnCard::Radial(r) => crate::perf::time(crate::perf::Stage::EffectRadial, || {
                        crate::blurs::radial_blur(std::sync::Arc::make_mut(&mut layer.source), r.spin, r.amount, r.center, r.repeat)
                    }),
                    OnCard::Bloom(b) => {
                        crate::perf::time(crate::perf::Stage::EffectBloom, || {
                            let source = std::sync::Arc::make_mut(&mut layer.source);
                            crate::bloom::bloom(source, b.threshold, b.radius, b.intensity, b.lines, b.length, b.angle)
                        });
                    }
                    OnCard::Directional(d) => {
                        crate::perf::time(crate::perf::Stage::EffectDirBlur, || {
                            crate::blurs::directional_blur(std::sync::Arc::make_mut(&mut layer.source), d.direction, d.length, d.repeat)
                        });
                    }
                    OnCard::Gaussian(g) => {
                        crate::perf::time(crate::perf::Stage::EffectBlur, || {
                            let source = std::sync::Arc::make_mut(&mut layer.source);
                            let taps = crate::effects::reach_weights(g.sigma, g.long);
                            if g.repeat {
                                crate::effects::held_blur_axes(source, &taps, (true, true));
                            } else {
                                crate::effects::blur_axes(source, &taps, (true, true));
                            }
                        });
                    }
                    OnCard::Glow(g) => {
                        crate::perf::time(crate::perf::Stage::EffectGlow, || {
                            crate::glow::glow(std::sync::Arc::make_mut(&mut layer.source), &g)
                        });
                    }
                    OnCard::Fx(f) => {
                        let source = std::sync::Arc::make_mut(&mut layer.source);
                        crate::effects::apply_stack_at(source, std::slice::from_ref(&f.instance), f.origin, false, |_, _, _| {});
                    }
                }
            }
        }
    }
    let mut kept = None;
    let mut start = 0;
    for (index, layer) in layers.iter().enumerate() {
        if keep == Some(from + index) {
            render_layers(&layers[start..index], frame, tile_size, cull, at, plan.float);
            kept = Some(frame.clone());
            start = index;
        }
        if let Some(stack) = &layer.adjust {
            render_layers(&layers[start..index], frame, tile_size, cull, at, plan.float);
            adjust_frame(layer, stack, frame, plan.float);
            start = index + 1;
        } else if layer.wrap.iter().any(|i| i.enabled && i.is_valid()) {
            render_layers(&layers[start..index], frame, tile_size, cull, at, plan.float);
            wrap_layer(layer, frame, tile_size, cull, plan.float);
            start = index + 1;
        }
    }
    render_layers(&layers[start..], frame, tile_size, cull, at, plan.float);
    kept
}

/// B-159 (G10): what the viewer keeps between two draws of one frame: the plan it last drew, and
/// the frame as it stood below one of that plan's layers, the one last edited.
#[derive(Default)]
pub struct Below {
    /// The layers last drawn, with the frame's width and height, the tile size, the part and
    /// the working depth.
    last: Option<(Vec<LayerDraw>, (usize, usize, usize, Option<Tile>, bool))>,
    /// The picture with the last plan's layers `..n` drawn, and `n`.
    kept: Option<(usize, WorkingBuffer)>,
    reused: u64,
}

impl Below {
    /// Keep nothing; the count stays.
    pub fn forget(&mut self) {
        self.last = None;
        self.kept = None;
    }

    /// How many draws started from a kept picture.
    pub fn reused(&self) -> u64 {
        self.reused
    }
}

/// B-159 (G10): [`render`], or [`render_part`] of `part`, for a viewer drawing a frame again after
/// an edit. The layers are compared with the ones `below` last drew, bottom first: the same
/// drawing (the very buffer, which `below` holds, so it cannot have been written on or its memory
/// used again), the same map, opacity and matte to the bit, and the same blend mode and effects.
/// If the picture kept is of layers that are all still the same, the frame starts from a copy of
/// it and only the layers above are drawn; each layer is laid on what is beneath it, pixel by
/// pixel, so where the drawing starts changes nothing but the clock. The picture below the first
/// layer that differs is kept for next time, when that is a new place: the layer edited is
/// usually edited again. With nothing kept and the bottom layer differing, as when the last edit
/// was taken back before another layer is edited, it is the picture below the next layer up that
/// differs. The viewer's alone; nothing else keeps a `Below`.
// ponytail: a layer whose drawing is made afresh for every plan (an adjustment layer's white, a
// motion-blur average made on the processor) never compares the same, so an edit above one starts
// no higher than it; keep those drawings in the cel cache if that matters.
pub fn render_below(plan: &FramePlan, tile_size: usize, part: Option<Tile>, below: &mut Below) -> WorkingBuffer {
    let shape = (plan.width, plan.height, tile_size, part, plan.float);
    let (same, above) = match &below.last {
        Some((last, was)) if *was == shape => {
            let same = last.iter().zip(&plan.layers).take_while(|(a, b)| same_draw(a, b)).count();
            let above = (last.len() == plan.layers.len())
                .then(|| (same.max(1)..last.len()).find(|&i| !same_draw(&last[i], &plan.layers[i])))
                .flatten();
            (same, above)
        }
        _ => (0, None),
    };
    let (mut frame, at, cut) = canvas(plan, part);
    let kept = below
        .kept
        .take()
        .filter(|(n, k)| (1..=same).contains(n) && k.width() == frame.width() && k.height() == frame.height());
    let from = match &kept {
        Some((n, k)) => {
            frame.data_mut().copy_from_slice(k.data());
            below.reused += 1;
            *n
        }
        None => 0,
    };
    let keep = match from {
        _ if same > from && same < plan.layers.len() => Some(same),
        0 => above,
        _ => None,
    };
    below.kept = match (keep, draw(plan, from, &mut frame, at, keep, tile_size, true)) {
        (Some(n), Some(picture)) => Some((n, picture)),
        _ => kept,
    };
    below.last = Some((plan.layers.clone(), shape));
    cut_out(frame, cut)
}

/// B-159: `a` and `b` draw the same, as [`render_below`] says.
fn same_draw(a: &LayerDraw, b: &LayerDraw) -> bool {
    let bits = |t: &Affine| [t.a, t.b, t.c, t.d, t.tx, t.ty].map(f64::to_bits);
    let matte = match (&a.matte, &b.matte) {
        (None, None) => true,
        (Some(x), Some(y)) => std::sync::Arc::ptr_eq(&x.source, &y.source) && bits(&x.transform) == bits(&y.transform),
        _ => false,
    };
    std::sync::Arc::ptr_eq(&a.source, &b.source)
        && bits(&a.transform) == bits(&b.transform)
        && a.opacity.to_bits() == b.opacity.to_bits()
        && matte
        && a.blend == b.blend
        && a.adjust == b.adjust
        && a.wrap == b.wrap
        && a.on_card == b.on_card
        && a.moments.len() == b.moments.len()
        && a.moments.iter().zip(&b.moments).all(|(x, y)| x.map(|t| bits(&t)) == y.map(|t| bits(&t)))
}

/// D-66: `frame = B + c*(E(B) - B)`, with `B` the frame as drawn so far, `E(B)` its pixels
/// through the adjustment layer's stack, and `c` the layer's coverage at the pixel times its
/// opacity. D-297: in a blend mode other than normal, `E(B)` is first laid on `B` in that mode at
/// full cover; the card leaves such a layer to the CPU. The stack runs on the whole frame, as a
/// layer's stack runs on the whole layer (ADR-017); the mix is per pixel, one row at a time in
/// parallel.
///
/// The bypasses `apply_stack` reports were already reported when the plan was made
/// (`compose::resolve_layer`), so the callback here is deliberately empty.
fn adjust_frame(
    layer: &LayerDraw,
    stack: &[crate::effects::EffectInstance],
    frame: &mut WorkingBuffer,
    float: bool,
) {
    let Some(inverse) = layer.transform.invert() else {
        return;
    };
    let matte = match &layer.matte {
        None => None,
        Some(m) => match m.transform.invert() {
            Some(inv) => Some((m, inv)),
            None => return,
        },
    };
    let mut effected = frame.clone();
    // A blur grows the buffer; `(ox, oy)` is where the frame's origin ended up inside it, and
    // the growth past the frame is what D-66 cuts off.
    let (ox, oy) = crate::effects::apply_stack(&mut effected, stack, |_, _, _| {});
    let width = frame.width();
    frame
        .data_mut()
        .par_chunks_mut(width * 4)
        .enumerate()
        .for_each(|(y, row)| {
            for x in 0..width {
                let (dx, dy) = (x as f64 + 0.5, y as f64 + 0.5);
                let (sx, sy) = inverse.apply(dx, dy);
                let mut c = sample_bilinear(&layer.source, sx, sy)[3] * layer.opacity;
                if let Some((m, matte_inverse)) = &matte {
                    let (mx, my) = matte_inverse.apply(dx, dy);
                    c *= sample_bilinear(&m.source, mx, my)[3];
                }
                if c == 0.0 {
                    continue;
                }
                let i = x * 4;
                let e = match layer.blend {
                    crate::model::BlendMode::Normal => effected.pixel(x + ox, y + oy),
                    mode => crate::composite::blend_pixel_at(mode, effected.pixel(x + ox, y + oy), [row[i], row[i + 1], row[i + 2], row[i + 3]], float),
                };
                // D-90: at full cover the answer is `E(B)` exactly. `b + (e - b)` rounds at
                // `b`'s size, which is most of a value as small as 20 stops down leaves.
                if c == 1.0 {
                    row[i..i + 4].copy_from_slice(&e);
                    continue;
                }
                for k in 0..4 {
                    let b = row[i + k];
                    row[i + k] = b + c * (e[k] - b);
                }
            }
        });
}

/// D-132: a layer with a Light Wrap, drawn onto the frame drawn so far. The layer is placed
/// first, through its transform and matte at full opacity, a picture the frame's size; each
/// Light Wrap that runs lays the light of the frame beneath on its edges, in stack order; then
/// the layer is drawn by its opacity and blend mode, as any layer is.
///
/// The bypasses were already reported when the plan was made, by the layer's own stack, where a
/// Light Wrap does nothing.
fn wrap_layer(layer: &LayerDraw, frame: &mut WorkingBuffer, tile_size: usize, cull: bool, float: bool) {
    let mut placed = WorkingBuffer::transparent(frame.width(), frame.height());
    let alone = LayerDraw {
        opacity: 1.0,
        blend: crate::model::BlendMode::Normal,
        wrap: Vec::new(),
        ..layer.clone()
    };
    render_layers(std::slice::from_ref(&alone), &mut placed, tile_size, cull, (0, 0), float);
    for instance in layer.wrap.iter().filter(|i| i.enabled && i.is_valid()) {
        if let crate::effects::Effect::LightWrap {
            width,
            intensity,
            blend,
        } = &instance.effect
        {
            // D-202: the placed layer, laid back under the light by the Mix.
            let given = (instance.mix < 100.0).then(|| placed.clone());
            crate::perf::time(crate::perf::Stage::EffectLightWrap, || {
                light_wrap(&mut placed, frame, *width, *intensity, blend == "add")
            });
            if let Some(given) = given {
                crate::effects::mix_back(&mut placed, &given, (0, 0), instance.mix / 100.0);
            }
        }
    }
    frame
        .data_mut()
        .par_chunks_exact_mut(4)
        .zip(placed.data().par_chunks_exact(4))
        .for_each(|(dst, src)| {
            let mut src = [src[0], src[1], src[2], src[3]];
            if layer.opacity != 1.0 {
                for c in &mut src {
                    *c *= layer.opacity;
                }
            }
            let under = [dst[0], dst[1], dst[2], dst[3]];
            dst.copy_from_slice(&crate::composite::blend_pixel_at(layer.blend, src, under, float));
        });
}

/// D-132's rule on the placed layer `l`, with `b` the frame beneath, both the frame's size.
/// The light is `b` and the layer's covering each through document 21's Gaussian at a third of
/// `width`, cut back to the frame: `w = intensity / 100 * (1 - covering) * light`. A pixel the
/// layer does not cover, or that no light reaches, is left exactly as it is.
// ponytail: the covering is blurred with its three empty colour channels, four times the work
// it needs; a one-channel blur when a profile says the wrap is slow.
fn light_wrap(l: &mut WorkingBuffer, b: &WorkingBuffer, width: f64, intensity: f64, add: bool) {
    if intensity == 0.0 {
        return;
    }
    let s = width / 3.0;
    let mut light = b.clone();
    let rl = crate::effects::blur(&mut light, s);
    let mut cover = l.clone();
    let rc = crate::effects::blur(&mut cover, s);
    let k = intensity / 100.0;
    let w = l.width();
    l.data_mut()
        .par_chunks_mut(w * 4)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, px) in row.chunks_exact_mut(4).enumerate() {
                let a = px[3] as f64;
                if a <= 0.0 {
                    continue;
                }
                let reach = k * (1.0 - cover.pixel(x + rc, y + rc)[3] as f64);
                let bb = light.pixel(x + rl, y + rl);
                let lit: [f64; 3] = std::array::from_fn(|c| reach * bb[c] as f64);
                if lit == [0.0; 3] {
                    continue;
                }
                for c in 0..3 {
                    let v = px[c] as f64 / a;
                    let v = if add {
                        v + lit[c]
                    } else {
                        1.0 - (1.0 - v) * (1.0 - lit[c].clamp(0.0, 1.0))
                    };
                    px[c] = (v * a) as f32;
                }
            }
        });
}

/// One segment of the stack, bottom to top, onto `frame` as it stands. `at` is where `frame`'s
/// first pixel is in the whole frame: `(0, 0)` but for B-158's part.
fn render_layers(layers: &[LayerDraw], frame: &mut WorkingBuffer, tile_size: usize, cull: bool, at: (usize, usize), float: bool) {
    let (width, height) = (frame.width(), frame.height());
    let tiles = tiles(width, height, tile_size);
    // D-301: a stencil clears what is under it where it is empty, so it reaches every tile.
    let boxes: Option<Vec<(f64, f64, f64, f64)>> = cull.then(|| {
        layers
            .iter()
            .map(|l| if stencil(l) { (f64::NEG_INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::INFINITY) } else { bounds(l) })
            .collect()
    });

    // The carve-up is what is left of the assembly step, and it is where every destination is
    // decided. It is serial on purpose: it hands out borrows, it does not touch a pixel.
    let mut rows_for: Vec<Vec<&mut [f32]>> =
        crate::perf::time(crate::perf::Stage::FrameAssembly, || {
            let stride = width * 4;
            let mut rows_for: Vec<Vec<&mut [f32]>> =
                tiles.iter().map(|t| Vec::with_capacity(t.height)).collect();
            let mut rest: &mut [f32] = frame.data_mut();
            for y in 0..height {
                let (row, tail) = rest.split_at_mut(stride);
                rest = tail;
                let mut row_rest = row;
                for (index, tile) in tiles.iter().enumerate() {
                    if y < tile.y || y >= tile.y + tile.height {
                        continue;
                    }
                    let (piece, tail) = row_rest.split_at_mut(tile.width * 4);
                    row_rest = tail;
                    rows_for[index].push(piece);
                }
                debug_assert!(row_rest.is_empty(), "the tiles do not cover row {y}");
            }
            rows_for
        });

    crate::perf::time(crate::perf::Stage::TileLoop, || {
        rows_for
            .par_drain(..)
            .zip(tiles.par_iter())
            .for_each(|(rows, &tile)| {
                let tile = Tile { x: tile.x + at.0, y: tile.y + at.1, ..tile };
                render_tile(layers, tile, rows, boxes.as_deref(), float)
            });
    });
}

/// One tile of the frame: the whole layer stack, bottom to top, over the frame's own pixels.
///
/// `rows` is the tile's own rows of the frame, top to bottom, and nothing else borrows them
/// (P-03(d)). They arrive at zero, which is what the tile's private accumulator used to be
/// initialised to, and every layer blends onto what the layers below it left, in the same order
/// and with the same arithmetic as when the tile owned that memory.
fn render_tile(
    layers: &[LayerDraw],
    tile: Tile,
    mut rows: Vec<&mut [f32]>,
    boxes: Option<&[(f64, f64, f64, f64)]>,
    float: bool,
) {
    for (index, layer) in layers.iter().enumerate() {
        // D-301: a stencil that has collapsed to nothing keeps nothing under it.
        let nothing = |rows: &mut Vec<&mut [f32]>| {
            if stencil(layer) {
                rows.iter_mut().for_each(|out| out.fill(0.0));
            }
        };
        let Some(inverse) = layer.transform.invert() else {
            nothing(&mut rows);
            continue;
        };
        // P-05: a tile the layer cannot reach is a tile the layer cannot change. Every pixel of
        // it would sample more than a pixel outside the source, which is exactly `[0, 0, 0, 0]`,
        // and `blend_pixel` is the identity for that source in all four of document 21's modes.
        // The skip is a saving and not a decision: `verification/P-05_culling_table.md` renders
        // both fixtures both ways and compares all 2,073,600 pixels byte for byte.
        if let Some(boxes) = boxes {
            if !reaches(boxes[index], tile) {
                continue;
            }
        }
        // A matte whose own transform cannot be inverted has collapsed to nothing, and nothing
        // has no alpha anywhere. Skipping the layer is the honest reading: the matte covers no
        // pixel, so the layer it mattes shows at no pixel either. Drawing it unmatted would show
        // the whole layer, which is the opposite of what the project asks for.
        let matte = match &layer.matte {
            None => None,
            Some(m) => match m.transform.invert() {
                Some(inv) => Some((m, inv)),
                None => {
                    nothing(&mut rows);
                    continue;
                }
            },
        };
        for (row, out) in rows.iter_mut().enumerate() {
            for col in 0..tile.width {
                // Document 21: geometry is continuous and pixel (i,j) is centred at
                // (i+0.5, j+0.5), so the sample point is the centre, not the corner.
                let (dx, dy) = ((tile.x + col) as f64 + 0.5, (tile.y + row) as f64 + 0.5);
                let (sx, sy) = inverse.apply(dx, dy);
                let mut src = sample_bilinear(&layer.source, sx, sy);
                if layer.opacity != 1.0 {
                    for c in &mut src {
                        *c *= layer.opacity;
                    }
                }
                // Document 21 step 5: `C' = C * m` and `A' = A * m`, where m is the matte
                // layer's post-transform alpha at this same destination pixel. The source is
                // premultiplied, so the same factor multiplies all four channels.
                if let Some((m, matte_inverse)) = &matte {
                    let (mx, my) = matte_inverse.apply(dx, dy);
                    let coverage = sample_bilinear(&m.source, mx, my)[3];
                    if coverage != 1.0 {
                        for c in &mut src {
                            *c *= coverage;
                        }
                    }
                }
                let i = col * 4;
                let dst = [out[i], out[i + 1], out[i + 2], out[i + 3]];
                out[i..i + 4].copy_from_slice(&crate::composite::blend_pixel_at(
                    layer.blend,
                    src,
                    dst,
                    float,
                ));
            }
        }
    }
}
