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
    /// B-46, B-47: `Some` when the layer's stack ends in an effect left for the graphics card
    /// (`compose::plan_frame_for_card`): `source` is the drawing before it. The CPU runs it
    /// itself in [`render`], so a plan made for the card is the same frame on either.
    pub on_card: Option<OnCard>,
}

/// An effect left for the graphics card, in the pixels of the buffer it runs on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OnCard {
    Radial(Radial),
    Bloom(Bloom),
    Directional(Directional),
}

/// B-49: a Directional Blur's settings (`blurs::directional_blur`), the length already divided
/// for Draft and never 0, so the drawing grows by exactly half the length, rounded up.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Directional {
    pub direction: f64,
    pub length: f64,
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
/// the time the renderer sees it. The exceptions are a Bloom (B-47) and a Directional Blur (B-49)
/// left for the card, which grow the drawing on every side, so the box counts that growth.
/// A layer's box in frame pixels: `(left, top, right, bottom)`.
///
/// Computed once per layer per frame, never per tile: the corners do not change between the
/// tiles of one frame, and on the fixtures the box excludes nothing, so every recomputation
/// would have been spent on a skip that never fires.
pub fn bounds(layer: &LayerDraw) -> (f64, f64, f64, f64) {
    let grow = match layer.on_card {
        Some(OnCard::Bloom(b)) => 2 * crate::bloom::reach(b.radius, b.lines, b.length),
        Some(OnCard::Directional(d)) => 2 * (d.length / 2.0).ceil() as usize,
        _ => 0,
    };
    let (w, h) = ((layer.source.width() + grow) as f64, (layer.source.height() + grow) as f64);
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
    render_maybe_culled(plan, tile_size, true)
}

/// The same frame with P-05's culling test turned off, which is what
/// `verification/P-05_culling_table.md` compares against.
///
/// It exists for that comparison and for nothing else. A skipped layer is only correct if the
/// frame is the same frame without the skip, and the only way to say that in bytes is to render
/// it both ways.
pub fn render_without_culling(plan: &FramePlan, tile_size: usize) -> WorkingBuffer {
    render_maybe_culled(plan, tile_size, false)
}

fn render_maybe_culled(plan: &FramePlan, tile_size: usize, cull: bool) -> WorkingBuffer {
    // B-46, B-47, B-49: an effect left for the card that the CPU is drawing after all is run first,
    // exactly as `apply_stack` would have run it.
    if plan.layers.iter().any(|l| l.on_card.is_some()) {
        let mut plan = plan.clone();
        for layer in &mut plan.layers {
            match layer.on_card.take() {
                None => {}
                Some(OnCard::Radial(r)) => crate::perf::time(crate::perf::Stage::EffectRadial, || {
                    crate::blurs::radial_blur(std::sync::Arc::make_mut(&mut layer.source), r.spin, r.amount, r.center)
                }),
                Some(OnCard::Bloom(b)) => {
                    crate::perf::time(crate::perf::Stage::EffectBloom, || {
                        let source = std::sync::Arc::make_mut(&mut layer.source);
                        crate::bloom::bloom(source, b.threshold, b.radius, b.intensity, b.lines, b.length, b.angle)
                    });
                }
                Some(OnCard::Directional(d)) => {
                    crate::perf::time(crate::perf::Stage::EffectDirBlur, || {
                        crate::blurs::directional_blur(std::sync::Arc::make_mut(&mut layer.source), d.direction, d.length)
                    });
                }
            }
        }
        return render_maybe_culled(&plan, tile_size, cull);
    }
    let mut frame = WorkingBuffer::transparent(plan.width, plan.height);
    // D-66: the frame is drawn in segments, each ending at an adjustment layer, whose stack
    // runs on the whole frame drawn so far before the next segment is drawn onto it.
    let mut from = 0;
    for (index, layer) in plan.layers.iter().enumerate() {
        if let Some(stack) = &layer.adjust {
            render_layers(&plan.layers[from..index], &mut frame, tile_size, cull);
            adjust_frame(layer, stack, &mut frame);
            from = index + 1;
        }
    }
    render_layers(&plan.layers[from..], &mut frame, tile_size, cull);
    frame
}

/// D-66: `frame = B + c*(E(B) - B)`, with `B` the frame as drawn so far, `E(B)` its pixels
/// through the adjustment layer's stack, and `c` the layer's coverage at the pixel times its
/// opacity. The stack runs on the whole frame, as a layer's stack runs on the whole layer
/// (ADR-017); the mix is per pixel, one row at a time in parallel.
///
/// The bypasses `apply_stack` reports were already reported when the plan was made
/// (`compose::resolve_layer`), so the callback here is deliberately empty.
fn adjust_frame(
    layer: &LayerDraw,
    stack: &[crate::effects::EffectInstance],
    frame: &mut WorkingBuffer,
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
                let e = effected.pixel(x + ox, y + oy);
                let i = x * 4;
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

/// One segment of the stack, bottom to top, onto `frame` as it stands.
fn render_layers(layers: &[LayerDraw], frame: &mut WorkingBuffer, tile_size: usize, cull: bool) {
    let (width, height) = (frame.width(), frame.height());
    let tiles = tiles(width, height, tile_size);
    let boxes: Option<Vec<(f64, f64, f64, f64)>> =
        cull.then(|| layers.iter().map(bounds).collect());

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
            .for_each(|(rows, &tile)| render_tile(layers, tile, rows, boxes.as_deref()));
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
) {
    for (index, layer) in layers.iter().enumerate() {
        let Some(inverse) = layer.transform.invert() else {
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
                None => continue,
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
                out[i..i + 4].copy_from_slice(&crate::composite::blend_pixel(
                    layer.blend,
                    src,
                    dst,
                ));
            }
        }
    }
}
