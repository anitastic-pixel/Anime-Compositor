//! B-25b: a shape layer's drawing, against D-78.
//!
//! D-78, accepted by the owner on 2026-09-20: a layer whose drawing is a list of shapes it
//! carries rather than a file on disk. Its space is its composition's — it has no width and no
//! height of its own — and it starts transparent black, with the shapes drawn into it first to
//! last, each shape's fill and then its stroke over that fill.
//!
//! # What is borrowed and what is new
//!
//! A shape's path is [`crate::mask::MaskPoint`] and [`crate::mask::MaskKey`], D-77's record
//! unchanged, so the pen, the rectangle, the ellipse, the flattening rule and B-24d's
//! interpolation all serve a shape without a second copy of any of them. [`crate::mask::flatten`]
//! and [`crate::mask::points_at`] are those rules, asked here as well as there.
//!
//! Three things are new, and they are the whole of this file:
//!
//! - **An open path.** `closed` false means the last point is not joined to the first. It changes
//!   the flattening by one segment and the stroke by its two ends; it does not change the fill,
//!   because a fill closes an open path with a straight line, which is what the even-odd ray
//!   already does when it wraps.
//! - **A fill**, which is ADR-016's coverage of the even-odd interior, unamended: the same 4x4
//!   ordered grid, the same sixteenths, the same tie-break. A shape's edge is the quantum a
//!   mask's edge has always been.
//! - **A stroke**, which is every point within `width_px / 2` of the flattened path. D-78 chose
//!   that definition over offset curves because it settles joins and caps without enumerating
//!   them — they are round, and only round — and because the distance it needs is the distance
//!   [`crate::mask::distance_to_path`] already computes for a mask's expansion.
//!
//! # Where a shape and a mask part company
//!
//! A mask whose points cross is kept, takes no part and raises `MASK_INVALID_OUTLINE`, because
//! nobody can say which side of a crossed mask should be kept. The even-odd rule *does* say what
//! a crossed shape fills, and a five-pointed star drawn in one stroke is the ordinary way to draw
//! one, so a shape that crosses itself is drawn. [`is_simple`](crate::mask::is_simple) is
//! therefore never asked about a shape. What is diagnosed instead is a path of fewer than two
//! points: there is nothing to fill and nothing to stroke between, so it is kept, draws nothing
//! and raises `SHAPE_INVALID_OUTLINE`.
//!
//! The reference tool is `tools/shape_reference.py`, written from D-78 before this file existed,
//! and `Fixtures/shapes/` is what it produced. `tests/b25b_shapes.rs` walks every case.

use crate::mask::{MaskKey, MaskPoint, SAMPLES_PER_SIDE};
use crate::WorkingBuffer;

/// D-78's largest stroke, in pixels: a solid's largest side, for the same reason.
pub const MAX_STROKE_WIDTH: f64 = 8192.0;

/// D-78: a shape's fill — one colour over its whole even-odd interior.
///
/// The colour is linear working-space RGB from 0 to 1, as a solid's is and as the tint effect's
/// is. Neither it nor the opacity is animated at first; D-78 leaves that with D-76's question
/// about a solid's colour.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Fill {
    pub color: [f64; 3],
    /// 0 to 1.
    pub opacity: f64,
}

/// D-78: a shape's stroke — one colour over every point within half its width of the path.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Stroke {
    pub color: [f64; 3],
    /// 0 to 1.
    pub opacity: f64,
    /// Above 0 and at most 8192, in pixels. The band is half of it either side of the path.
    pub width_px: f64,
}

/// D-78's shape: a path that may be open, with an optional fill and an optional stroke.
///
/// A shape with neither is kept and draws nothing. That is a path a person has drawn and not yet
/// decided about, not a fault, so it raises nothing.
#[derive(Clone, PartialEq, Debug)]
pub struct Shape {
    pub name: String,
    pub enabled: bool,
    /// False means the last point is not joined to the first.
    pub closed: bool,
    pub points: Vec<MaskPoint>,
    /// Where the path moves, by D-77's rule and B-24d's machinery. Empty on a path that stands
    /// still, which is most of them.
    pub keys: Vec<MaskKey>,
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
}

impl Default for Shape {
    fn default() -> Shape {
        Shape {
            name: String::new(),
            enabled: true,
            closed: true,
            points: Vec::new(),
            keys: Vec::new(),
            fill: None,
            stroke: None,
        }
    }
}

impl Shape {
    /// A closed path of corners with a fill and no stroke, which is what the rectangle and the
    /// ellipse tools hand over before anybody touches the panel.
    pub fn filled(name: impl Into<String>, vertices: Vec<(f64, f64)>, fill: Fill) -> Shape {
        Shape {
            name: name.into(),
            points: vertices
                .into_iter()
                .map(|(x, y)| MaskPoint::corner(x, y))
                .collect(),
            fill: Some(fill),
            ..Shape::default()
        }
    }

    /// This shape as it is at a composition frame, with no keys left in it.
    ///
    /// Resolved before the rasterizer and before document 27's cache key, exactly as
    /// [`crate::mask::Mask::at`] is and for the same reason.
    pub fn at(&self, frame: i32) -> Shape {
        Shape {
            points: self.points_at(frame),
            keys: Vec::new(),
            ..self.clone()
        }
    }

    /// The path at a composition frame: document 20's rules, shared with a mask's path.
    pub fn points_at(&self, frame: i32) -> Vec<MaskPoint> {
        crate::mask::points_at(&self.points, &self.keys, frame)
    }

    /// The path flattened into points to sample between, open or closed as the shape says.
    pub fn outline(&self) -> Vec<(f64, f64)> {
        crate::mask::flatten(&self.points, self.closed)
    }

    /// D-78: fewer than two points cannot be filled or stroked. Kept, drawn as nothing, and
    /// diagnosed as `SHAPE_INVALID_OUTLINE` — never refused, so one bad shape never costs a
    /// project. A shape that crosses itself is *not* this case: it is drawn.
    pub fn has_enough_points(&self) -> bool {
        self.points.len() >= 2
    }

    /// Whether this shape puts anything into the layer's picture.
    pub fn is_renderable(&self) -> bool {
        self.enabled && self.has_enough_points()
    }

    /// What is outside D-78's ranges, as a sentence, or `None` for a shape that may exist.
    ///
    /// The commands refuse on this with `COMMAND_INVALID_VALUE` and the loader with
    /// `PROJECT_SCHEMA_INVALID`; both say the same sentence, because it is the same rule.
    pub fn problem(&self) -> Option<String> {
        for (what, color, opacity) in [
            self.fill.map(|f| ("fill", f.color, f.opacity)),
            self.stroke.map(|s| ("stroke", s.color, s.opacity)),
        ]
        .into_iter()
        .flatten()
        {
            if let Some(c) = color.iter().find(|c| !(0.0..=1.0).contains(*c)) {
                return Some(format!(
                    "a {what} colour of three numbers from 0 to 1, not {c}"
                ));
            }
            if !(0.0..=1.0).contains(&opacity) {
                return Some(format!("a {what} opacity from 0 to 1, not {opacity}"));
            }
        }
        if let Some(s) = self.stroke {
            if !(s.width_px > 0.0 && s.width_px <= MAX_STROKE_WIDTH) {
                return Some(format!(
                    "a stroke width above 0 and at most {MAX_STROKE_WIDTH}, not {}",
                    s.width_px
                ));
            }
        }
        // D-77's rule, which D-78 adopts unchanged: a key holds the whole outline, so it holds as
        // many points as the base. There is no honest way to pair the points of two outlines of
        // different lengths.
        self.keys
            .iter()
            .find(|k| k.points.len() != self.points.len())
            .map(|k| {
                format!(
                    "every key to hold the path's {} points, and the key at frame {} holds {}",
                    self.points.len(),
                    k.frame,
                    k.points.len()
                )
            })
    }
}

/// Coverage of every pixel of a `width` by `height` picture, from ADR-016's grid, a sample at a
/// time.
///
/// The sample for grid cell (i, j) of pixel (x, y) is at `(x + (i + 0.5)/4, y + (j + 0.5)/4)`,
/// which is [`crate::mask::pixel_coverage`]'s rule written once more because what counts as
/// inside differs — there it is a polygon, here it is a polygon *or* a distance.
///
/// B-25c playtest, 2026-09-23: this was how a shape was drawn until the owner's window measured
/// 12,976 ms for one frame of a shape layer - every sample asking every edge, on one thread, the
/// cost P-15 took out of masks. [`draw`] now uses P-15's fill and [`stroke_field`]; this stays as
/// the reference the test below holds them to.
#[cfg(test)]
fn field(width: usize, height: usize, inside: impl Fn(f64, f64) -> bool) -> Vec<f32> {
    let n = SAMPLES_PER_SIDE;
    let mut out = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let mut hits = 0u32;
            for j in 0..n {
                for i in 0..n {
                    let sx = x as f64 + (i as f64 + 0.5) / n as f64;
                    let sy = y as f64 + (j as f64 + 0.5) / n as f64;
                    if inside(sx, sy) {
                        hits += 1;
                    }
                }
            }
            out.push(hits as f32 / (n * n) as f32);
        }
    }
    out
}

/// One coverage field of one colour, laid over what the layer holds so far.
///
/// Document 21's normal blend in premultiplied linear RGBA: the source is the colour times its
/// own alpha, and `src + dst * (1 - src_a)` is the whole of it.
fn paint(picture: &mut [f32], field: &[f32], color: [f64; 3], opacity: f64) {
    for (i, coverage) in field.iter().enumerate() {
        let a = *coverage * opacity as f32;
        if a == 0.0 {
            continue;
        }
        let px = &mut picture[i * 4..i * 4 + 4];
        for c in 0..3 {
            px[c] = color[c] as f32 * a + px[c] * (1.0 - a);
        }
        px[3] = a + px[3] * (1.0 - a);
    }
}

/// Document 21 step 1 for a shape layer: the composition's size in transparent black, then every
/// shape drawn into it, first to last, so a later shape covers an earlier one.
///
/// The shapes must already be at the frame — `Shape::at` — as a mask's path is resolved before
/// the rasterizer and for the same reason. A shape that cannot be drawn is skipped here and
/// diagnosed by the caller, which is the only place that knows the frame number to say it
/// against.
pub fn draw(shapes: &[Shape], width: usize, height: usize) -> WorkingBuffer {
    let mut picture = WorkingBuffer::transparent(width, height);
    let data = picture.data_mut();
    for shape in shapes {
        if !shape.is_renderable() {
            continue;
        }
        let outline = shape.outline();
        // The fill first, then the stroke over this shape's own fill. D-78 fixes that order, and
        // it is why FX-SHP-008's shared columns carry the stroke and not a mixture.
        if let Some(fill) = shape.fill {
            // An open path is closed for the purpose of filling it, by a straight line from its
            // last point to its first: the even-odd ray already wraps, so nothing is added here.
            let f = crate::mask::scanline_field(&outline, width, height, 0);
            paint(data, &f, fill.color, fill.opacity);
        }
        if let Some(stroke) = shape.stroke {
            let f = stroke_field(&outline, shape.closed, width, height, stroke.width_px / 2.0);
            paint(data, &f, stroke.color, stroke.opacity);
        }
    }
    picture
}

/// Every pixel's stroke coverage: the samples within `reach` of the path.
///
/// P-15's band, which [`crate::mask`] uses for an expanded mask, asked of a stroke: an edge whose
/// run of y lies more than `reach` above or below a sample row is further than `reach` from every
/// sample on it, and one whose run of x lies more than `reach` beside a sample is further than
/// that from the sample, so neither can put it in the stroke. What is left is asked in
/// [`crate::mask::distance_to_segment`]'s arithmetic, which is `distance_to_path`'s, and the
/// nearest of them is the same nearest, so the answer is the one a sample at a time gave, not an
/// approximation; `the_fast_fields_are_the_slow_ones` below holds it to that. Rows run across the
/// thread pool, each writing only its own pixels.
fn stroke_field(path: &[(f64, f64)], closed: bool, w: usize, h: usize, reach: f64) -> Vec<f32> {
    use rayon::prelude::*;
    let n = SAMPLES_PER_SIDE;
    let mut field = vec![0.0f32; w * h];
    let points = path.len();
    if points == 0 {
        return field;
    }
    // One point is a segment from it to itself, which is what `distance_to_path` measures then.
    let segments: Vec<((f64, f64), (f64, f64))> = match points {
        1 => vec![(path[0], path[0])],
        _ => (0..if closed { points } else { points - 1 })
            .map(|i| (path[i], path[(i + 1) % points]))
            .collect(),
    };
    field.par_chunks_mut(w).enumerate().for_each_init(
        || (Vec::new(), vec![0u32; w]),
        |(band, hits), (yi, row)| {
            hits.iter_mut().for_each(|hit| *hit = 0);
            for j in 0..n {
                let sy = yi as f64 + (j as f64 + 0.5) / n as f64;
                band.clear();
                for &(a, b) in &segments {
                    let (low, high) = if a.1 <= b.1 { (a.1, b.1) } else { (b.1, a.1) };
                    if low - reach <= sy && sy <= high + reach {
                        let (left, right) = if a.0 <= b.0 { (a.0, b.0) } else { (b.0, a.0) };
                        band.push((a, b, left, right));
                    }
                }
                if band.is_empty() {
                    continue;
                }
                for (xi, hit) in hits.iter_mut().enumerate() {
                    for i in 0..n {
                        let sx = xi as f64 + (i as f64 + 0.5) / n as f64;
                        let mut nearest = f64::INFINITY;
                        for &(a, b, left, right) in band.iter() {
                            if sx < left - reach || sx > right + reach {
                                continue;
                            }
                            nearest = nearest.min(crate::mask::distance_to_segment(a, b, sx, sy));
                        }
                        if nearest <= reach {
                            *hit += 1;
                        }
                    }
                }
            }
            for (v, hit) in row.iter_mut().zip(hits.iter()) {
                *v = *hit as f32 / (n * n) as f32;
            }
        },
    );
    field
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The fill and the stroke as [`draw`] now works them out, against the sample-at-a-time
    /// [`field`] they replaced, pixel for pixel and with no tolerance: a star that crosses itself,
    /// open and closed, a two-point line, one point and a path with nothing in it.
    #[test]
    fn the_fast_fields_are_the_slow_ones() {
        let (w, h) = (61, 47);
        let star: Vec<(f64, f64)> = (0..7)
            .map(|k| {
                let a = k as f64 * std::f64::consts::TAU * 3.0 / 7.0 + 0.3;
                (30.3 + 21.7 * a.cos(), 23.1 + 18.9 * a.sin())
            })
            .collect();
        let cases: [(&[(f64, f64)], bool); 5] = [
            (&star, true),
            (&star, false),
            (&[(3.2, 40.5), (57.9, 6.25)], false),
            (&[(20.5, 20.5)], true),
            (&[], true),
        ];
        for (path, closed) in cases {
            let fill = crate::mask::scanline_field(path, w, h, 0);
            let slow = field(w, h, |x, y| crate::mask::point_inside(path, x, y));
            assert_eq!(fill, slow, "fill of {path:?}");
            for reach in [0.5, 2.0, 7.25] {
                let fast = stroke_field(path, closed, w, h, reach);
                let slow = field(w, h, |x, y| {
                    crate::mask::distance_to_path(path, closed, x, y) <= reach
                });
                assert_eq!(fast, slow, "stroke {reach} of {path:?}, closed {closed}");
            }
        }
    }
}
