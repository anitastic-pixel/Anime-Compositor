//! B-06: the polygon mask, rasterized in layer/source space.
//!
//! Document 21 step 2: "Apply the layer polygon mask in layer/source space." Step 2 is before
//! the transform at step 4, so the mask is baked into the layer's source pixels and the renderer
//! never learns that masks exist. That is not a shortcut. A mask defined in source space and
//! applied after the transform would scale and rotate differently from the drawing it is cutting,
//! which is a different feature from the one document 19 specifies.
//!
//! Document 21: "Mask coverage `m` in 0..1 multiplies both premultiplied RGB and alpha."
//! [`apply`] does exactly that and nothing else.
//!
//! # The rasterization rule, which document 25 line 51 asks B-06 to select and record
//!
//! Document 25 line 51: "Mask rasterization fixtures must record the exact rasterizer/reference
//! tool once selected in B-06; until then, polygon interior/exterior topology tests are
//! authoritative but subpixel edge goldens are OPEN."
//!
//! The rule is: **a 4x4 ordered grid of sample points per pixel, coverage is the count of
//! samples inside the polygon divided by sixteen, and insideness is the even-odd rule.** The
//! sample for grid cell (i, j) of pixel (x, y) is at `(x + (i + 0.5)/4, y + (j + 0.5)/4)`, which
//! keeps document 21's convention that geometry is continuous and a pixel's centre is at
//! `(x + 0.5, y + 0.5)` — the sixteen offsets are symmetric about that centre.
//!
//! Three things follow, and they are the reason this rule was chosen over an analytic one:
//!
//! - **Coverage is an exact rational.** Sixteen integer decisions, divided by sixteen. There is
//!   no accumulation of floating-point area, so the result does not depend on vertex order,
//!   summation order, or the machine. A second implementation that agrees on which side of an
//!   edge each of the sixteen points falls agrees exactly, byte for byte.
//! - **The even-odd choice is not load-bearing.** Even-odd and nonzero winding differ only on
//!   self-intersecting polygons, and document 19 says self-intersection "is unsupported in G1 and
//!   must be rejected", which [`is_simple`] does at the command boundary. On the polygons this
//!   build accepts the two rules give identical answers, so the record here is a statement of
//!   what the code does rather than a decision that shapes the pictures.
//! - **The edge quantum is stated rather than implied.** Coverage lands on one of seventeen
//!   values, so an edge is accurate to 1/16. That is the honest resolution of this build and it
//!   is what the fixture checks against. It is not subpixel-exact and this module does not claim
//!   to be; document 21 says "multisample details must be fixture-tested before claiming subpixel
//!   equivalence", and nothing here claims it.
//!
//! The reference tool is the one this project has used since H-03: a second implementation,
//! written separately from this one against the specification rather than against this code, in
//! the fixture. A polygon rasterizer from a drawing library would have been a weaker check, not a
//! stronger one, because it would bring its own fill rule and its own antialiasing and the
//! disagreement would say nothing about whether this file is right.

use crate::WorkingBuffer;

/// The side of the sample grid inside each pixel. Sixteen samples per pixel.
///
/// Raising it costs time in proportion to its square and buys a finer edge quantum. It is
/// recorded in the module documentation and in `verification/B-06_mask_table.md` because the
/// expected coverage values in the fixture are derived from it: changing it changes them, which
/// makes it a specification decision and not a tuning knob.
pub const SAMPLES_PER_SIDE: usize = 4;

/// Document 19's PolygonMask: "an ordered list of vec2 vertices, closed by definition, plus
/// enabled/inverted flags."
///
/// Closed by definition means the last vertex joins the first; no vertex is repeated to close it.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PolygonMask {
    pub vertices: Vec<(f64, f64)>,
    pub enabled: bool,
    pub inverted: bool,
}

impl PolygonMask {
    /// A mask that is on and not inverted.
    pub fn new(vertices: Vec<(f64, f64)>) -> PolygonMask {
        PolygonMask {
            vertices,
            enabled: true,
            inverted: false,
        }
    }

    /// Document 19's invariant, minus self-intersection, which [`is_simple`] answers separately
    /// because it is the expensive half and the caller wants the two reasons apart.
    ///
    /// Fewer than three vertices is not a polygon. It has no interior, so it cannot be
    /// normalized into one and there is nothing to draw.
    pub fn has_enough_vertices(&self) -> bool {
        self.vertices.len() >= 3
    }

    /// Whether this mask is one this build will draw.
    ///
    /// A disabled mask is carried in the project and takes no part in a frame, which is what
    /// lets one be switched off without losing its shape. A mask with no interior, or one that
    /// crosses itself, is also carried and also not drawn — document 19 rejects
    /// self-intersection rather than normalizing it, and the layer draws unmasked. `persist`
    /// says so on load; this is the gate that makes it true.
    ///
    /// The simplicity test is the expensive half, so it is asked last and only of a polygon that
    /// has already passed the cheap checks.
    pub fn is_renderable(&self) -> bool {
        self.enabled && self.has_enough_vertices() && is_simple(&self.vertices)
    }
}

/// Whether the polygon is simple: no edge crosses another, and no two vertices coincide.
///
/// Document 19: self-intersection "is unsupported in G1 and must be rejected or normalized only
/// through an explicit command". This build rejects; it does not normalize, because normalizing
/// silently would hand back a different shape from the one that was drawn.
///
/// ponytail: O(n^2) over edge pairs. A G1 mask is a handful of vertices drawn by hand, so the
/// sweep-line version would be more code than the thing it replaces. If masks ever arrive from
/// an import with thousands of vertices, this is the place that needs the sweep line.
pub fn is_simple(vertices: &[(f64, f64)]) -> bool {
    let n = vertices.len();
    if n < 3 {
        return false;
    }
    for i in 0..n {
        for j in (i + 1)..n {
            if vertices[i] == vertices[j] {
                return false;
            }
        }
    }
    for i in 0..n {
        let a0 = vertices[i];
        let a1 = vertices[(i + 1) % n];
        for j in (i + 1)..n {
            // Edges sharing a vertex meet there by construction; that is a corner, not a
            // crossing. Adjacent pairs and the wrap-around pair are the ones to skip.
            if j == i || (j + 1) % n == i || (i + 1) % n == j {
                continue;
            }
            let b0 = vertices[j];
            let b1 = vertices[(j + 1) % n];
            if segments_cross(a0, a1, b0, b1) {
                return false;
            }
        }
    }
    true
}

/// Whether two closed segments share any point.
///
/// The orientation test is exact in sign for the coordinates a mask holds, and the collinear
/// case is settled by projection onto the segment rather than by a tolerance, so there is no
/// epsilon here to tune or to get wrong.
fn segments_cross(p0: (f64, f64), p1: (f64, f64), q0: (f64, f64), q1: (f64, f64)) -> bool {
    let d1 = orientation(q0, q1, p0);
    let d2 = orientation(q0, q1, p1);
    let d3 = orientation(p0, p1, q0);
    let d4 = orientation(p0, p1, q1);

    if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0))
        && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
    {
        return true;
    }

    (d1 == 0.0 && on_segment(q0, q1, p0))
        || (d2 == 0.0 && on_segment(q0, q1, p1))
        || (d3 == 0.0 && on_segment(p0, p1, q0))
        || (d4 == 0.0 && on_segment(p0, p1, q1))
}

fn orientation(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

/// Whether `p`, already known to be collinear with `a`-`b`, lies within the segment.
fn on_segment(a: (f64, f64), b: (f64, f64), p: (f64, f64)) -> bool {
    p.0 >= a.0.min(b.0) && p.0 <= a.0.max(b.0) && p.1 >= a.1.min(b.1) && p.1 <= a.1.max(b.1)
}

/// The even-odd rule at one point: count the edges crossed by a ray, inside if the count is odd.
///
/// The ray is cast in +x. An edge is counted when the sample's y lies in the half-open span
/// `[min(y0, y1), max(y0, y1))` of that edge, which is what makes a vertex shared by two edges
/// count once rather than twice or not at all. That half-open convention is the whole of the
/// rule's determinism at a vertex, and it is why this is written out rather than borrowed.
fn point_inside(vertices: &[(f64, f64)], x: f64, y: f64) -> bool {
    let n = vertices.len();
    let mut inside = false;
    for i in 0..n {
        let (x0, y0) = vertices[i];
        let (x1, y1) = vertices[(i + 1) % n];
        // Half-open in y: an edge spans the sample when exactly one of its endpoints is at or
        // below it. A horizontal edge has y0 == y1 and is never counted, which is correct - it
        // cannot separate an inside from an outside along a +x ray.
        if (y0 <= y) != (y1 <= y) {
            // The x where this edge meets the horizontal line through the sample.
            let t = (y - y0) / (y1 - y0);
            if x < x0 + t * (x1 - x0) {
                inside = !inside;
            }
        }
    }
    inside
}

/// Coverage of one pixel: the fraction of the 4x4 sample grid inside the polygon.
///
/// Returns one of seventeen values, `0/16` through `16/16`.
pub fn pixel_coverage(vertices: &[(f64, f64)], x: usize, y: usize) -> f32 {
    let n = SAMPLES_PER_SIDE;
    let mut hits = 0u32;
    for j in 0..n {
        for i in 0..n {
            let sx = x as f64 + (i as f64 + 0.5) / n as f64;
            let sy = y as f64 + (j as f64 + 0.5) / n as f64;
            if point_inside(vertices, sx, sy) {
                hits += 1;
            }
        }
    }
    hits as f32 / (n * n) as f32
}

/// Apply a mask to a layer's source, in place.
///
/// Document 21: coverage multiplies "both premultiplied RGB and alpha", so all four channels are
/// scaled by the same `m`. Scaling alpha alone would change the colour of a partly covered edge;
/// scaling RGB alone would leave a hole that still occludes.
///
/// A disabled mask, or one with too few vertices to have an interior, changes nothing. That is
/// checked here as well as at the command boundary because a project file can be loaded with a
/// mask this build did not create, and a frame is not the place to refuse one.
///
/// The buffer must be the caller's own copy. `CelCache::decoded` returns an owned clone of what
/// it holds, so masking what it hands back cannot reach the cached cel — `tests/b06_mask.rs`
/// asserts that rather than trusting this sentence.
pub fn apply(buffer: &mut WorkingBuffer, mask: &PolygonMask) {
    if !mask.is_renderable() {
        return;
    }
    let width = buffer.width();
    let height = buffer.height();
    let data = buffer.data_mut();
    for y in 0..height {
        for x in 0..width {
            let mut m = pixel_coverage(&mask.vertices, x, y);
            if mask.inverted {
                m = 1.0 - m;
            }
            if m == 1.0 {
                continue;
            }
            let i = (y * width + x) * 4;
            for c in &mut data[i..i + 4] {
                *c *= m;
            }
        }
    }
}
