//! B-06 and B-24b: a layer's masks, rasterized in layer/source space.
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
//! # What changed at D-77, and what did not
//!
//! B-06 gave a layer one polygon: a list of vertices, on or off, inverted or not. D-77, accepted
//! on 2026-09-19, gives it a *list* of masks, each with a mode, an opacity, an even feather, an
//! expansion, and a path whose points may carry Bezier handles.
//!
//! **Nothing drawn before it moves.** A segment whose two handles are both zero is one straight
//! line, so [`Mask::outline`] hands the sampler the same vertices B-06 handed it; and an
//! expansion of zero takes the [`point_inside`] branch and touches no distance arithmetic at all.
//! A B-06 mask is a D-77 mask with one entry, mode Add, opacity 1, feather 0, expansion 0 and no
//! handles, and `Fixtures/masks/fx_msk_001.json` is that claim pinned as a fixture.
//!
//! # The rasterization rule, which document 25 line 51 asks B-06 to select and record
//!
//! The rule is: **a 4x4 ordered grid of sample points per pixel, coverage is the count of
//! samples inside the polygon divided by sixteen, and insideness is the even-odd rule.** The
//! sample for grid cell (i, j) of pixel (x, y) is at `(x + (i + 0.5)/4, y + (j + 0.5)/4)`, which
//! keeps document 21's convention that geometry is continuous and a pixel's centre is at
//! `(x + 0.5, y + 0.5)` — the sixteen offsets are symmetric about that centre. ADR-016 is
//! unamended by D-77: the grid, the even-odd rule and the sixteenths are what they were.
//!
//! Three things follow, and they are the reason this rule was chosen over an analytic one:
//!
//! - **Coverage is an exact rational.** Sixteen integer decisions, divided by sixteen. There is
//!   no accumulation of floating-point area, so the result does not depend on vertex order,
//!   summation order, or the machine. A second implementation that agrees on which side of an
//!   edge each of the sixteen points falls agrees exactly, byte for byte. A feather is the one
//!   thing that leaves that grid, because a Gaussian is a weighted sum and not a count.
//! - **The even-odd choice is load-bearing only for curves.** Even-odd and nonzero winding
//!   differ only on outlines that cross themselves. D-77 keeps [`is_simple`]'s refusal of a
//!   *point* polygon that crosses itself, and does not test the handles, so a curve that loops
//!   over itself is drawn — by even-odd, which is defined for it.
//! - **The edge quantum is stated rather than implied.** Coverage lands on one of seventeen
//!   values, so an edge is accurate to 1/16. That is the honest resolution of this build and it
//!   is what the fixture checks against. It is not subpixel-exact and this module does not claim
//!   to be; document 21 says "multisample details must be fixture-tested before claiming subpixel
//!   equivalence", and nothing here claims it.
//!
//! The reference tool is the one this project has used since H-03: a second implementation,
//! written separately from this one against the specification rather than against this code, in
//! `tools/mask_reference.py` and in `tests/b06_mask.rs`. A polygon rasterizer from a drawing
//! library would have been a weaker check, not a stronger one, because it would bring its own
//! fill rule and its own antialiasing and the disagreement would say nothing about whether this
//! file is right.

use crate::WorkingBuffer;

/// The side of the sample grid inside each pixel. Sixteen samples per pixel.
///
/// Raising it costs time in proportion to its square and buys a finer edge quantum. It is
/// recorded in the module documentation and in `verification/B-06_mask_table.md` because the
/// expected coverage values in the fixture are derived from it: changing it changes them, which
/// makes it a specification decision and not a tuning knob.
pub const SAMPLES_PER_SIDE: usize = 4;

/// D-77: how far a mask may be grown or shrunk, in pixels, either way.
pub const MAX_EXPANSION: f64 = 8192.0;

/// D-77: the shortest a flattened curve may be, and the longest.
///
/// A curved segment is cut into pieces of about two pixels, held between these. The ceiling is
/// why a curve longer than about 1024 pixels shows its flattening; raising it is an amendment to
/// D-77 and changes every fixture it touches.
const MIN_PIECES: usize = 16;
const MAX_PIECES: usize = 512;

/// One point of a mask's path, with its two Bezier handles.
///
/// D-53's convention, which D-77 adopts for masks: a handle is an offset **in pixels from its
/// own point**, not an absolute position, so moving a point carries its handles with it. A
/// handle of `(0.0, 0.0)` is no handle, and a segment between two such is a straight line.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct MaskPoint {
    pub point: (f64, f64),
    pub in_handle: (f64, f64),
    pub out_handle: (f64, f64),
}

impl MaskPoint {
    /// A corner: a point with no handles, which is what every mask written before D-77 holds.
    pub fn corner(x: f64, y: f64) -> MaskPoint {
        MaskPoint {
            point: (x, y),
            in_handle: (0.0, 0.0),
            out_handle: (0.0, 0.0),
        }
    }
}

/// D-77: how one mask joins the masks before it.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum MaskMode {
    #[default]
    Add,
    Subtract,
    Intersect,
    Difference,
    /// Drawn on screen, and takes no part in the picture.
    None,
}

impl MaskMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            MaskMode::Add => "add",
            MaskMode::Subtract => "subtract",
            MaskMode::Intersect => "intersect",
            MaskMode::Difference => "difference",
            MaskMode::None => "none",
        }
    }

    /// `None` for a word this build does not know, which the caller refuses rather than guesses.
    pub fn from_str(s: &str) -> Option<MaskMode> {
        match s {
            "add" => Some(MaskMode::Add),
            "subtract" => Some(MaskMode::Subtract),
            "intersect" => Some(MaskMode::Intersect),
            "difference" => Some(MaskMode::Difference),
            "none" => Some(MaskMode::None),
            _ => None,
        }
    }
}

/// D-77's mask: a closed path, plus everything that shapes the coverage it gives.
///
/// Closed by definition, as document 19 has always said: the last point joins the first, and no
/// point is repeated to close it.
#[derive(Clone, PartialEq, Debug)]
pub struct Mask {
    pub name: String,
    pub enabled: bool,
    pub inverted: bool,
    pub mode: MaskMode,
    /// 0 to 1.
    pub opacity: f64,
    /// 0 or more, in pixels. The Gaussian's sigma is half of it.
    pub feather_px: f64,
    /// -8192 to 8192, in pixels. Positive grows.
    pub expansion_px: f64,
    pub points: Vec<MaskPoint>,
}

impl Default for Mask {
    fn default() -> Mask {
        Mask {
            name: String::new(),
            enabled: true,
            inverted: false,
            mode: MaskMode::Add,
            opacity: 1.0,
            feather_px: 0.0,
            expansion_px: 0.0,
            points: Vec::new(),
        }
    }
}

impl Mask {
    /// A B-06 mask: corners only, on, not inverted, Add at full opacity with no feather and no
    /// expansion. This is the shape a file written before D-77 is read into.
    pub fn polygon(vertices: Vec<(f64, f64)>) -> Mask {
        Mask {
            name: "Mask 1".to_string(),
            points: vertices
                .into_iter()
                .map(|(x, y)| MaskPoint::corner(x, y))
                .collect(),
            ..Mask::default()
        }
    }

    /// The points alone, without their handles: what [`is_simple`] is asked about.
    pub fn vertices(&self) -> Vec<(f64, f64)> {
        self.points.iter().map(|p| p.point).collect()
    }

    /// Document 19's invariant, minus self-intersection, which [`is_simple`] answers separately
    /// because it is the expensive half and the caller wants the two reasons apart.
    ///
    /// Fewer than three points is not a polygon. It has no interior, so it cannot be normalized
    /// into one and there is nothing to draw.
    pub fn has_enough_points(&self) -> bool {
        self.points.len() >= 3
    }

    /// Whether this mask is one this build will draw.
    ///
    /// A disabled mask is carried in the project and takes no part in a frame, which is what
    /// lets one be switched off without losing its shape. A mask with no interior, or one whose
    /// points cross, is also carried and also not drawn — document 19 rejects self-intersection
    /// rather than normalizing it, and the layer draws without that mask. `persist` says so on
    /// load; this is the gate that makes it true.
    ///
    /// The simplicity test is the expensive half, so it is asked last and only of a polygon that
    /// has already passed the cheap checks. D-77 asks it of the points and not of the flattened
    /// curve, which keeps it the handful of pairs it has always been.
    pub fn is_renderable(&self) -> bool {
        self.enabled && self.has_enough_points() && is_simple(&self.vertices())
    }

    /// The closed outline this mask's path draws, as points to sample between.
    ///
    /// A segment whose two handles are both zero is one straight line and adds nothing but its
    /// own start, which is why a mask of corners hands the sampler exactly the vertices B-06
    /// handed it. A curved segment is cut into pieces of about two pixels at equal steps of `t`,
    /// counted from the control polygon's length summed point, out handle, in handle, point — in
    /// that order, so the count is the same on every machine.
    pub fn outline(&self) -> Vec<(f64, f64)> {
        let n = self.points.len();
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let p0 = self.points[i];
            let p1 = self.points[(i + 1) % n];
            let a = p0.point;
            let b = (a.0 + p0.out_handle.0, a.1 + p0.out_handle.1);
            let d = p1.point;
            let c = (d.0 + p1.in_handle.0, d.1 + p1.in_handle.1);
            out.push(a);
            if p0.out_handle == (0.0, 0.0) && p1.in_handle == (0.0, 0.0) {
                continue;
            }
            let mut length = (b.0 - a.0).hypot(b.1 - a.1);
            length += (c.0 - b.0).hypot(c.1 - b.1);
            length += (d.0 - c.0).hypot(d.1 - c.1);
            let pieces = ((length / 2.0).ceil() as usize).clamp(MIN_PIECES, MAX_PIECES);
            for k in 1..pieces {
                let t = k as f64 / pieces as f64;
                let u = 1.0 - t;
                let w = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
                out.push((
                    w.0 * a.0 + w.1 * b.0 + w.2 * c.0 + w.3 * d.0,
                    w.0 * a.1 + w.1 * b.1 + w.2 * c.1 + w.3 * d.1,
                ));
            }
        }
        out
    }
}

/// Whether the polygon is simple: no edge crosses another, and no two vertices coincide.
///
/// Document 19: self-intersection "is unsupported in G1 and must be rejected or normalized only
/// through an explicit command". This build rejects; it does not normalize, because normalizing
/// silently would hand back a different shape from the one that was drawn.
///
/// ponytail: O(n^2) over edge pairs. A mask is a handful of points drawn by hand, so the
/// sweep-line version would be more code than the thing it replaces. If masks ever arrive from
/// an import with thousands of points, this is the place that needs the sweep line.
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
pub fn point_inside(vertices: &[(f64, f64)], x: f64, y: f64) -> bool {
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

/// The shortest distance from a point to the outline itself, never signed.
fn distance_to_outline(outline: &[(f64, f64)], x: f64, y: f64) -> f64 {
    let n = outline.len();
    let mut best = f64::INFINITY;
    for i in 0..n {
        let (x0, y0) = outline[i];
        let (x1, y1) = outline[(i + 1) % n];
        let (dx, dy) = (x1 - x0, y1 - y0);
        let run = dx * dx + dy * dy;
        let t = if run == 0.0 {
            0.0
        } else {
            (((x - x0) * dx + (y - y0) * dy) / run).clamp(0.0, 1.0)
        };
        best = best.min((x - (x0 + t * dx)).hypot(y - (y0 + t * dy)));
    }
    best
}

/// Whether one sample point is inside a mask grown or shrunk by `expansion` pixels.
///
/// D-77: an expansion of zero is exactly B-06's rule and touches no distance arithmetic, which
/// is what keeps every mask drawn before D-77 pixel for pixel what it was. Otherwise the sample
/// is inside when its distance to the outline, signed positive within, plus the expansion is
/// zero or more — so a corner rounds off, as After Effects rounds one.
fn sample_inside(outline: &[(f64, f64)], x: f64, y: f64, expansion: f64) -> bool {
    let inside = point_inside(outline, x, y);
    if expansion == 0.0 {
        return inside;
    }
    let signed = distance_to_outline(outline, x, y) * if inside { 1.0 } else { -1.0 };
    signed + expansion >= 0.0
}

/// Coverage of one pixel: the fraction of the 4x4 sample grid inside the polygon.
///
/// Returns one of seventeen values, `0/16` through `16/16`.
pub fn pixel_coverage(vertices: &[(f64, f64)], x: usize, y: usize) -> f32 {
    pixel_coverage_at(vertices, x as f64, y as f64, 0.0)
}

/// The same, at a pixel that may sit outside the frame and under an expansion.
fn pixel_coverage_at(outline: &[(f64, f64)], x: f64, y: f64, expansion: f64) -> f32 {
    let n = SAMPLES_PER_SIDE;
    let mut hits = 0u32;
    for j in 0..n {
        for i in 0..n {
            let sx = x + (i as f64 + 0.5) / n as f64;
            let sy = y + (j as f64 + 0.5) / n as f64;
            if sample_inside(outline, sx, sy, expansion) {
                hits += 1;
            }
        }
    }
    hits as f32 / (n * n) as f32
}

/// One mask's own coverage over the layer: expansion, then feather, then invert, then opacity.
///
/// D-77 fixes that order. Feather after expansion means a grown edge is the one that softens;
/// invert after feather means the soft band is inverted with the rest; opacity last means it
/// scales what the person sees rather than what the blur is given.
fn mask_field(mask: &Mask, width: usize, height: usize) -> Vec<f32> {
    let outline = mask.outline();
    let sigma = mask.feather_px / 2.0;
    let radius = if sigma > 0.0 {
        (3.0 * sigma).ceil() as usize
    } else {
        0
    };
    let (w, h) = (width + 2 * radius, height + 2 * radius);
    // The coverage outside the frame is worked out like any other coverage rather than invented,
    // so a mask lying against the frame's edge does not darken when it is feathered.
    let mut field: Vec<f32> = Vec::with_capacity(w * h);
    for y in 0..h {
        for x in 0..w {
            field.push(pixel_coverage_at(
                &outline,
                x as f64 - radius as f64,
                y as f64 - radius as f64,
                mask.expansion_px,
            ));
        }
    }

    if radius == 0 {
        // No feather: the field is the frame already.
        return finish(field, mask);
    }

    // Document 21's Gaussian: separable, normalised weights, radius ceil(3 * sigma). The field
    // is exactly `radius` wider on every side than the frame, so every coverage the frame's
    // pixels need was worked out above and none of it is invented.
    let mut weights: Vec<f32> = (0..=2 * radius)
        .map(|i| {
            let d = i as f64 - radius as f64;
            (-(d * d) / (2.0 * sigma * sigma)).exp() as f32
        })
        .collect();
    let total: f32 = weights.iter().sum();
    for weight in &mut weights {
        *weight /= total;
    }
    let mut across = vec![0.0f32; w * h];
    for y in 0..h {
        for x in 0..w {
            let mut acc = 0.0;
            for (k, weight) in weights.iter().enumerate() {
                let sx = (x + k).saturating_sub(radius).min(w - 1);
                acc += field[y * w + sx] * weight;
            }
            across[y * w + x] = acc;
        }
    }
    let mut done = vec![0.0f32; width * height];
    for y in 0..height {
        for x in 0..width {
            let mut acc = 0.0;
            for (k, weight) in weights.iter().enumerate() {
                acc += across[(y + k) * w + (x + radius)] * weight;
            }
            done[y * width + x] = acc;
        }
    }
    finish(done, mask)
}

/// Invert, then opacity: the last two steps of D-77's order, shared by both paths above.
fn finish(mut field: Vec<f32>, mask: &Mask) -> Vec<f32> {
    let opacity = mask.opacity as f32;
    for v in &mut field {
        if mask.inverted {
            *v = 1.0 - *v;
        }
        *v *= opacity;
    }
    field
}

/// Every mask of a layer, first to last, worked into one coverage.
///
/// `None` means no mask takes part, which is a whole layer and not an empty one.
///
/// D-77: the accumulation starts at nothing, except that a first mask in Subtract or Intersect
/// starts from the whole layer — there would otherwise be nothing to take away from, and such a
/// mask could only ever give an empty frame, which is not what an animator drawing a hole means.
pub fn coverage(masks: &[Mask], width: usize, height: usize) -> Option<Vec<f32>> {
    let mut acc: Option<Vec<f32>> = None;
    for mask in masks {
        if mask.mode == MaskMode::None || !mask.is_renderable() {
            continue;
        }
        let m = mask_field(mask, width, height);
        let into = acc.get_or_insert_with(|| {
            let start = match mask.mode {
                MaskMode::Subtract | MaskMode::Intersect => 1.0,
                _ => 0.0,
            };
            vec![start; width * height]
        });
        for (a, m) in into.iter_mut().zip(m) {
            *a = match mask.mode {
                MaskMode::Add => *a + m - *a * m,
                MaskMode::Subtract => *a * (1.0 - m),
                MaskMode::Intersect => *a * m,
                MaskMode::Difference => *a + m - 2.0 * *a * m,
                MaskMode::None => *a,
            }
            .clamp(0.0, 1.0);
        }
    }
    acc
}

/// Apply a layer's masks to its source, in place.
///
/// Document 21: coverage multiplies "both premultiplied RGB and alpha", so all four channels are
/// scaled by the same `m`. Scaling alpha alone would change the colour of a partly covered edge;
/// scaling RGB alone would leave a hole that still occludes.
///
/// A disabled mask, or one with too few points to have an interior, changes nothing. That is
/// checked here as well as at the command boundary because a project file can be loaded with a
/// mask this build did not create, and a frame is not the place to refuse one.
///
/// The buffer must be the caller's own copy. `CelCache::decoded` returns an owned clone of what
/// it holds, so masking what it hands back cannot reach the cached cel — `tests/b06_mask.rs`
/// asserts that rather than trusting this sentence.
pub fn apply(buffer: &mut WorkingBuffer, masks: &[Mask]) {
    let width = buffer.width();
    let height = buffer.height();
    let Some(field) = coverage(masks, width, height) else {
        return;
    };
    let data = buffer.data_mut();
    for (i, m) in field.iter().enumerate() {
        if *m == 1.0 {
            continue;
        }
        for c in &mut data[i * 4..i * 4 + 4] {
            *c *= m;
        }
    }
}
