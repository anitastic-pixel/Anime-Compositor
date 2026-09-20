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

/// Coverage of every pixel of a `width` by `height` picture, from ADR-016's grid.
///
/// The sample for grid cell (i, j) of pixel (x, y) is at `(x + (i + 0.5)/4, y + (j + 0.5)/4)`,
/// which is [`crate::mask::pixel_coverage`]'s rule written once more because what counts as
/// inside differs — there it is a polygon, here it is a polygon *or* a distance.
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
            let f = field(width, height, |x, y| {
                crate::mask::point_inside(&outline, x, y)
            });
            paint(data, &f, fill.color, fill.opacity);
        }
        if let Some(stroke) = shape.stroke {
            let radius = stroke.width_px / 2.0;
            let f = field(width, height, |x, y| {
                crate::mask::distance_to_path(&outline, shape.closed, x, y) <= radius
            });
            paint(data, &f, stroke.color, stroke.opacity);
        }
    }
    picture
}
