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
//! Not here, and deliberately: masks (parked with R-04), effects (B-06), alpha mattes (B-06,
//! and they need the matte layer rendered through its own transform first) and the multiply,
//! screen and add blend modes. Those are steps 2, 3, 5 and part of 7 of document 21's layer
//! render order. A [`LayerDraw`] carries no blend mode at all rather than carrying one this
//! renderer would quietly ignore.

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
    pub source: WorkingBuffer,
    pub transform: Affine,
    pub opacity: f32,
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
/// it. Each tile allocates its own accumulator, so the only shared state is the immutable
/// plan, and the assembly step writes each tile to a position fixed before any thread started.
pub fn render(plan: &FramePlan, tile_size: usize) -> WorkingBuffer {
    let tiles = tiles(plan.width, plan.height, tile_size);
    let rendered: Vec<(Tile, Vec<f32>)> = tiles
        .par_iter()
        .map(|&tile| (tile, render_tile(plan, tile)))
        .collect();

    let mut frame = WorkingBuffer::transparent(plan.width, plan.height);
    let data = frame.data_mut();
    for (tile, pixels) in rendered {
        for row in 0..tile.height {
            let dst = ((tile.y + row) * plan.width + tile.x) * 4;
            let src = row * tile.width * 4;
            data[dst..dst + tile.width * 4].copy_from_slice(&pixels[src..src + tile.width * 4]);
        }
    }
    frame
}

/// One tile of the frame: the whole layer stack, bottom to top, over one small accumulator.
fn render_tile(plan: &FramePlan, tile: Tile) -> Vec<f32> {
    let mut acc = vec![0.0f32; tile.width * tile.height * 4];
    for layer in &plan.layers {
        let Some(inverse) = layer.transform.invert() else {
            continue;
        };
        for row in 0..tile.height {
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
                let i = (row * tile.width + col) * 4;
                let dst = [acc[i], acc[i + 1], acc[i + 2], acc[i + 3]];
                acc[i..i + 4].copy_from_slice(&crate::composite::over_pixel(src, dst));
            }
        }
    }
    acc
}
