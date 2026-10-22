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
//!   [`crate::mask::distance_to_path`] already computes for a mask's expansion. D-170 adds mitre
//!   and bevel joins and butt and square caps, drawn by [`stroke_parts`]; round and round is still
//!   D-78's distance.
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

use std::collections::BTreeMap;

use crate::mask::{MaskKey, MaskPoint, SAMPLES_PER_SIDE};
use crate::model::{Property, Value};
use crate::WorkingBuffer;

/// D-78's largest stroke, in pixels: a solid's largest side, for the same reason.
pub const MAX_STROKE_WIDTH: f64 = 8192.0;

/// D-168: a gradient holds 2 to this many stops.
pub const MAX_STOPS: usize = 64;

/// D-168: along the line from start to end, or out from the start by the start-to-end length.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GradientKind {
    Linear,
    Radial,
}

impl GradientKind {
    pub fn as_str(self) -> &'static str {
        match self {
            GradientKind::Linear => "linear",
            GradientKind::Radial => "radial",
        }
    }

    pub fn named(name: &str) -> Option<GradientKind> {
        match name {
            "linear" => Some(GradientKind::Linear),
            "radial" => Some(GradientKind::Radial),
            _ => None,
        }
    }
}

/// D-168: one colour and opacity at an offset from 0 to 1 along the gradient.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Stop {
    pub offset: f64,
    /// Linear working-space RGB, 0 to 1.
    pub color: [f64; 3],
    pub opacity: f64,
}

/// D-168: a fill's or stroke's colour worked per pixel from `stops`, instead of its flat colour.
///
/// `start` and `end` are points in the layer's space, which is the composition's, keyed as any
/// property is. The stops are kept as written; [`Ramp`] puts them in offset order when drawing.
#[derive(Clone, PartialEq, Debug)]
pub struct Gradient {
    pub kind: GradientKind,
    pub start: Property,
    pub end: Property,
    pub stops: Vec<Stop>,
}

impl Gradient {
    /// This gradient with its points resolved to a frame and no keys left in them.
    fn at(&self, frame: i32) -> Gradient {
        Gradient {
            start: Property::constant(self.start.value_at(frame)),
            end: Property::constant(self.end.value_at(frame)),
            ..self.clone()
        }
    }

    /// What is outside D-168's ranges, as a sentence, for [`Shape::problem`].
    fn problem(&self, what: &str) -> Option<String> {
        if !(2..=MAX_STOPS).contains(&self.stops.len()) {
            return Some(format!(
                "a {what} gradient of 2 to {MAX_STOPS} stops, not {}",
                self.stops.len()
            ));
        }
        for stop in &self.stops {
            if !(0.0..=1.0).contains(&stop.offset) {
                return Some(format!(
                    "a {what} gradient stop offset from 0 to 1, not {}",
                    stop.offset
                ));
            }
            if let Some(c) = stop.color.iter().find(|c| !(0.0..=1.0).contains(*c)) {
                return Some(format!(
                    "a {what} gradient stop colour of three numbers from 0 to 1, not {c}"
                ));
            }
            if !(0.0..=1.0).contains(&stop.opacity) {
                return Some(format!(
                    "a {what} gradient stop opacity from 0 to 1, not {}",
                    stop.opacity
                ));
            }
        }
        for (end, point) in [("start", &self.start), ("end", &self.end)] {
            let pair = |v: Value| v.as_vec2().is_some();
            if point.split().is_some()
                || !pair(point.base())
                || point.keyframes().iter().any(|k| !pair(k.value))
            {
                return Some(format!("a {what} gradient {end} of two numbers"));
            }
            if point.expression().is_some() {
                return Some(format!("no expression on a {what} gradient {end}"));
            }
            if point.keyframes().iter().any(|k| k.spatial.is_some()) {
                return Some(format!("no motion path on a {what} gradient {end}"));
            }
        }
        None
    }
}

/// Every name [`Shape::channels`] answers to. D-738 (a): a parametric shape's settings are
/// `path_` and the setting's name in [`ParametricKind::settings`].
pub const SHAPE_PROPERTIES: [&str; 21] = [
    "fill_start",
    "fill_end",
    "stroke_start",
    "stroke_end",
    "trim_start",
    "trim_end",
    "trim_offset",
    "fill_color",
    "fill_opacity",
    "stroke_color",
    "stroke_opacity",
    "stroke_width",
    "path_size",
    "path_position",
    "path_roundness",
    "path_points",
    "path_rotation",
    "path_inner_radius",
    "path_outer_radius",
    "path_inner_roundness",
    "path_outer_roundness",
];

/// D-738 (a): what a parametric shape is drawn as.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParametricKind {
    Rectangle,
    Ellipse,
    Polygon,
    Star,
}

impl ParametricKind {
    pub const NAMES: [&str; 4] = ["rectangle", "ellipse", "polygon", "star"];
    pub fn as_str(self) -> &'static str {
        Self::NAMES[self as usize]
    }
    pub fn named(name: &str) -> Option<ParametricKind> {
        use ParametricKind::*;
        [Rectangle, Ellipse, Polygon, Star]
            .into_iter()
            .find(|k| k.as_str() == name)
    }
    /// The settings this kind is drawn from, by the names the file gives them.
    pub fn settings(self) -> &'static [&'static str] {
        match self {
            ParametricKind::Rectangle => &["size", "position", "roundness"],
            ParametricKind::Ellipse => &["size", "position"],
            ParametricKind::Polygon => &[
                "points",
                "position",
                "rotation",
                "outer_radius",
                "outer_roundness",
            ],
            ParametricKind::Star => &[
                "points",
                "position",
                "rotation",
                "inner_radius",
                "outer_radius",
                "inner_roundness",
                "outer_roundness",
            ],
        }
    }
}

/// Whether a parametric setting is two numbers; the rest are one.
pub fn is_pair(setting: &str) -> bool {
    matches!(setting, "size" | "position")
}

/// 4/3 (sqrt 2 - 1): the handle, as a share of the radius, that draws a quarter circle.
const KAPPA: f64 = 0.552_284_749_830_793_4;

/// D-738 (a): a rectangle, an ellipse, a polygon or a star by its settings, each keyed as any
/// number is, its outline worked from them as After Effects draws it (lottie-web, MIT, writes
/// the same arithmetic down).
#[derive(Clone, PartialEq, Debug)]
pub struct Parametric {
    pub kind: ParametricKind,
    /// The kind's settings by their names in [`ParametricKind::settings`]; a star switched to a
    /// polygon keeps its inner ones here, unused, for switching back.
    pub settings: BTreeMap<String, Property>,
}

impl Parametric {
    /// The outline at a composition frame, or at the settings' bases with `None`: centred on
    /// `position`, starting at the top right (a rectangle), the top (an ellipse) or straight up
    /// turned by `rotation` (a polygon or a star), then clockwise on screen.
    pub fn outline(&self, frame: Option<i32>) -> Vec<MaskPoint> {
        let get = |name: &str| {
            self.settings.get(name).map(|p| match frame {
                Some(f) => p.value_at(f),
                None => p.base(),
            })
        };
        let one = |name: &str| get(name).and_then(|v| v.as_scalar()).unwrap_or(0.0);
        let two = |name: &str| get(name).and_then(|v| v.as_vec2()).unwrap_or((0.0, 0.0));
        let (x, y) = two("position");
        let at = |px: f64, py: f64, i: (f64, f64), o: (f64, f64)| MaskPoint {
            point: (px, py),
            in_handle: i,
            out_handle: o,
        };
        let none = (0.0, 0.0);
        match self.kind {
            ParametricKind::Ellipse => {
                let (w, h) = two("size");
                let (w, h) = (w / 2.0, h / 2.0);
                let (kw, kh) = (KAPPA * w, KAPPA * h);
                vec![
                    at(x, y - h, (-kw, 0.0), (kw, 0.0)),
                    at(x + w, y, (0.0, -kh), (0.0, kh)),
                    at(x, y + h, (kw, 0.0), (-kw, 0.0)),
                    at(x - w, y, (0.0, kh), (0.0, -kh)),
                ]
            }
            ParametricKind::Rectangle => {
                let (w, h) = two("size");
                let (w, h) = (w / 2.0, h / 2.0);
                let r = one("roundness").min(w).min(h);
                if r == 0.0 {
                    return vec![
                        at(x + w, y - h, none, none),
                        at(x + w, y + h, none, none),
                        at(x - w, y + h, none, none),
                        at(x - w, y - h, none, none),
                    ];
                }
                // Each point beside a corner has one handle, pointing to that corner.
                let k = KAPPA * r;
                vec![
                    at(x + w, y - h + r, (0.0, -k), none),
                    at(x + w, y + h - r, none, (0.0, k)),
                    at(x + w - r, y + h, (k, 0.0), none),
                    at(x - w + r, y + h, none, (-k, 0.0)),
                    at(x - w, y + h - r, (0.0, k), none),
                    at(x - w, y - h + r, none, (0.0, -k)),
                    at(x - w + r, y - h, (-k, 0.0), none),
                    at(x + w - r, y - h, none, (k, 0.0)),
                ]
            }
            ParametricKind::Polygon | ParametricKind::Star => {
                let n = one("points").floor().max(1.0);
                let star = self.kind == ParametricKind::Star;
                let count = if star { 2.0 * n } else { n } as usize;
                let step = std::f64::consts::TAU / count as f64;
                let turn = -std::f64::consts::FRAC_PI_2 + one("rotation").to_radians();
                (0..count)
                    .map(|i| {
                        let outer = !star || i % 2 == 0;
                        let (rad, round) = if outer {
                            (one("outer_radius"), one("outer_roundness"))
                        } else {
                            (one("inner_radius"), one("inner_roundness"))
                        };
                        let a = turn + i as f64 * step;
                        // Along the circle's tangent, clockwise on screen.
                        let len = std::f64::consts::TAU * rad / (4.0 * n) * round / 100.0;
                        let (tx, ty) = (-a.sin() * len, a.cos() * len);
                        at(x + rad * a.cos(), y + rad * a.sin(), (-tx, -ty), (tx, ty))
                    })
                    .collect()
            }
        }
    }

    /// What is outside D-738's ranges, as a sentence: every setting the kind has, at its base
    /// and at every key, and keys only.
    fn problem(&self) -> Option<String> {
        let kind = self.kind.as_str();
        for name in self.kind.settings() {
            let what = format!("a {kind}'s {}", name.replace('_', " "));
            let Some(p) = self.settings.get(*name) else {
                return Some(format!("{what}, which is missing"));
            };
            if p.expression().is_some() {
                return Some(format!("no expression on {what}: it takes keys only"));
            }
            let (lo, hi) = match *name {
                "position" | "rotation" => (f64::NEG_INFINITY, f64::INFINITY),
                "points" => (3.0, 100.0),
                "inner_roundness" | "outer_roundness" => (-1000.0, 1000.0),
                _ => (0.0, 100000.0),
            };
            for v in std::iter::once(p.base()).chain(p.keyframes().iter().map(|k| k.value)) {
                let numbers = match (is_pair(name), v) {
                    (true, Value::Vec2(a, b)) => vec![a, b],
                    (false, Value::Scalar(n)) => vec![n],
                    _ => {
                        let count = if is_pair(name) {
                            "two numbers"
                        } else {
                            "one number"
                        };
                        return Some(format!("{what} of {count}"));
                    }
                };
                if let Some(n) = numbers.iter().find(|n| !(lo..=hi).contains(*n)) {
                    return Some(format!("{what} from {lo} to {hi}, not {n}"));
                }
            }
        }
        None
    }
}

/// D-738 (b): a group's transform, by the names the file gives its numbers.
pub const GROUP_PROPERTIES: [&str; 7] = [
    "anchor",
    "position",
    "scale",
    "rotation",
    "opacity",
    "skew",
    "skew_axis",
];

/// D-738 (c): a repeater's numbers, by the names the file gives them.
pub const REPEATER_PROPERTIES: [&str; 8] = [
    "copies",
    "offset",
    "anchor",
    "position",
    "scale",
    "rotation",
    "start_opacity",
    "end_opacity",
];

/// D-738 (d): each path operation's type, its numbers with their ranges and After Effects' new
/// values (`center` is two numbers, the rest one), and its choice with its words, the first
/// After Effects' new one. `merge` is Merge Paths, worked in [`drawn`] rather than path by path.
#[allow(clippy::type_complexity)]
pub const OPERATIONS: [(&str, &[(&str, f64, f64, f64)], Option<(&str, &[&str])>); 7] = [
    ("round_corners", &[("radius", 0.0, 100000.0, 10.0)], None),
    ("pucker_bloat", &[("amount", -100.0, 100.0, 0.0)], None),
    (
        "twist",
        &[
            ("angle", f64::NEG_INFINITY, f64::INFINITY, 0.0),
            ("center", f64::NEG_INFINITY, f64::INFINITY, 0.0),
        ],
        None,
    ),
    (
        "zig_zag",
        &[("size", 0.0, 10000.0, 10.0), ("ridges", 0.0, 100.0, 5.0)],
        Some(("points", &["corner", "smooth"])),
    ),
    (
        "wiggle",
        &[
            ("size", 0.0, 10000.0, 10.0),
            ("detail", 0.0, 100.0, 3.0),
            ("wiggles_per_second", 0.0, 1000.0, 2.0),
            ("correlation", 0.0, 100.0, 50.0),
            ("seed", 0.0, 100000.0, 0.0),
        ],
        None,
    ),
    (
        "offset",
        &[
            ("amount", -10000.0, 10000.0, 10.0),
            ("miter_limit", 1.0, 100.0, 4.0),
        ],
        Some(("join", &["miter", "round", "bevel"])),
    ),
    (
        "merge",
        &[],
        Some((
            "mode",
            &["add", "merge", "subtract", "intersect", "exclude"],
        )),
    ),
];

/// D-738 (d): at most this many operations on one group.
pub const MOST_OPERATIONS: usize = 8;

/// D-738 (d): one of a group's path operations, as After Effects' Round Corners, Pucker & Bloat,
/// Twist, Zig Zag, Wiggle Paths, Offset Paths or Merge Paths at the foot of a group's contents.
#[derive(Clone, PartialEq, Debug)]
pub struct Operation {
    /// Its type, a name in [`OPERATIONS`].
    pub kind: String,
    /// Its choice's word, where its type has a choice.
    pub choice: Option<String>,
    /// Its numbers by the file's names.
    pub numbers: BTreeMap<String, Property>,
    /// ADR-008: the record this operation was read from, which saving writes over.
    pub record: crate::model::FileRecord,
}

impl Operation {
    /// After Effects' new operation of a type, or `None` for a type there is not.
    pub fn new(kind: &str) -> Option<Operation> {
        let (_, numbers, choice) = OPERATIONS.iter().find(|o| o.0 == kind)?;
        let numbers = numbers
            .iter()
            .map(|&(name, _, _, d)| {
                let v = if name == "center" {
                    Value::Vec2(d, d)
                } else {
                    Value::Scalar(d)
                };
                (name.to_string(), Property::constant(v))
            })
            .collect();
        Some(Operation {
            kind: kind.to_string(),
            choice: choice.map(|(_, words)| words[0].to_string()),
            numbers,
            record: Default::default(),
        })
    }

    /// The path this operation gives at a frame, `seconds` in.
    fn apply(&self, pts: Vec<MaskPoint>, closed: bool, frame: i32, seconds: f64) -> Vec<MaskPoint> {
        use crate::shape_ops as ops;
        let get = |n: &str| self.numbers.get(n).map(|p| p.value_at(frame));
        let one = |n: &str| get(n).and_then(|v| v.as_scalar()).unwrap_or(0.0);
        let choice = self.choice.as_deref().unwrap_or("");
        match self.kind.as_str() {
            "round_corners" => ops::round_corners(pts, closed, one("radius")),
            "pucker_bloat" => ops::pucker_bloat(pts, one("amount")),
            "twist" => {
                let center = get("center").and_then(|v| v.as_vec2()).unwrap_or_default();
                ops::twist(pts, closed, one("angle"), center)
            }
            "zig_zag" => ops::zig_zag(pts, closed, one("size"), one("ridges"), choice == "smooth"),
            "wiggle" => {
                let w = ops::Wiggle {
                    size: one("size"),
                    detail: one("detail"),
                    per_second: one("wiggles_per_second"),
                    correlation: one("correlation"),
                    seed: one("seed"),
                };
                ops::wiggle(pts, closed, &w, seconds)
            }
            "offset" => {
                let join = match choice {
                    "round" => ops::Join::Round,
                    "bevel" => ops::Join::Bevel,
                    _ => ops::Join::Miter,
                };
                ops::offset(pts, closed, one("amount"), join, one("miter_limit"))
            }
            _ => pts,
        }
    }

    /// What is wrong with this operation, the `k`th of `whose`, as a sentence.
    fn problem(&self, whose: &str, k: usize) -> Option<String> {
        let what = format!("{whose} operation {} ({})", k + 1, self.kind);
        let Some((_, numbers, choice)) = OPERATIONS.iter().find(|o| o.0 == self.kind) else {
            let names: Vec<&str> = OPERATIONS.iter().map(|o| o.0).collect();
            return Some(format!(
                "{whose} operation {} to be one of {}, not \"{}\"",
                k + 1,
                names.join(", "),
                self.kind
            ));
        };
        for &(name, lo, hi, _) in *numbers {
            let said = number_problem(
                &format!("{what} {name}"),
                self.numbers.get(name),
                (lo, hi),
                name == "center",
            );
            if said.is_some() {
                return said;
            }
        }
        let (name, words) = (*choice)?;
        match self.choice.as_deref() {
            Some(w) if words.contains(&w) => None,
            other => Some(format!(
                "{what} {name} to be one of {}, not {}",
                words.join(", "),
                other.map_or("missing".to_string(), |w| format!("\"{w}\""))
            )),
        }
    }
}

/// D-738 (b): a turn of the plane, `[a, b, c, d, e, f]` taking (x, y) to
/// (a x + c y + e, b x + d y + f).
pub type Turn = [f64; 6];

const STILL: Turn = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];

/// `a` after `b`.
fn mul(a: &Turn, b: &Turn) -> Turn {
    [
        a[0] * b[0] + a[2] * b[1],
        a[1] * b[0] + a[3] * b[1],
        a[0] * b[2] + a[2] * b[3],
        a[1] * b[2] + a[3] * b[3],
        a[0] * b[4] + a[2] * b[5] + a[4],
        a[1] * b[4] + a[3] * b[5] + a[5],
    ]
}

/// Clockwise on screen, y running down.
fn rotate(degrees: f64) -> Turn {
    let (s, c) = degrees.to_radians().sin_cos();
    [c, s, -s, c, 0.0, 0.0]
}

/// A point by the whole turn, or a handle (`linear`) by the turn without its move.
pub fn apply(m: &Turn, (x, y): (f64, f64), linear: bool) -> (f64, f64) {
    let (e, f) = if linear { (0.0, 0.0) } else { (m[4], m[5]) };
    (m[0] * x + m[2] * y + e, m[1] * x + m[3] * y + f)
}

/// The turn that undoes `m`, or `None` where `m` flattens the plane.
pub fn invert(m: &Turn) -> Option<Turn> {
    let det = m[0] * m[3] - m[1] * m[2];
    if det.abs() < 1e-12 || !det.is_finite() {
        return None;
    }
    let (a, b, c, d) = (m[3] / det, -m[1] / det, -m[2] / det, m[0] / det);
    Some([a, b, c, d, -(a * m[4] + c * m[5]), -(b * m[4] + d * m[5])])
}

/// D-738 (b): some of a shape layer's shapes, placed together by one transform, as After
/// Effects' group is; a group may sit in another.
#[derive(Clone, PartialEq, Debug)]
pub struct Group {
    pub name: String,
    /// The place in the layer's groups of the group this one sits in; `None` is the layer.
    pub parent: Option<usize>,
    /// Every name in [`GROUP_PROPERTIES`]: scale in per cent, opacity 0 to 1, angles in degrees.
    pub transform: BTreeMap<String, Property>,
    /// D-738 (c): what draws the group's contents again and again, as After Effects' Repeater at
    /// the foot of a group's contents does.
    pub repeater: Option<Repeater>,
    /// D-738 (d): what changes every path drawn in the group, in order, before its repeater and
    /// its transform place it.
    pub operations: Vec<Operation>,
    /// ADR-008: the record this group was read from, which saving writes over.
    pub record: crate::model::FileRecord,
}

/// D-738 (c): a group's repeater.
#[derive(Clone, PartialEq, Debug)]
pub struct Repeater {
    /// Each copy drawn over the one before it; false is After Effects' Below, the first on top.
    pub above: bool,
    /// Every name in [`REPEATER_PROPERTIES`]: scale in per cent, opacities 0 to 1, rotation in
    /// degrees.
    pub numbers: BTreeMap<String, Property>,
    /// ADR-008: the record this repeater was read from, which saving writes over.
    pub record: crate::model::FileRecord,
}

impl Default for Repeater {
    /// After Effects' new Repeater: three copies, each 100 to the right of the one before.
    fn default() -> Repeater {
        let pair = |x, y| Property::constant(Value::Vec2(x, y));
        let one = |n| Property::constant(Value::Scalar(n));
        let numbers = [
            ("copies", one(3.0)),
            ("offset", one(0.0)),
            ("anchor", pair(0.0, 0.0)),
            ("position", pair(100.0, 0.0)),
            ("scale", pair(100.0, 100.0)),
            ("rotation", one(0.0)),
            ("start_opacity", one(1.0)),
            ("end_opacity", one(1.0)),
        ];
        Repeater {
            above: false,
            numbers: numbers
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
            record: Default::default(),
        }
    }
}

impl Repeater {
    /// Each copy's turn and opacity at a frame, first to last; `None` for a copy whose turn is
    /// not finite, which is not drawn. Copy i is turned by R(i + offset), R(k + f) being the
    /// whole step k times (undone, below 0) and then the fraction f of it; the step turns about
    /// the anchor, then scales about it, then moves.
    pub fn copies(&self, frame: i32) -> Vec<Option<(Turn, f64)>> {
        let get = |n: &str| self.numbers.get(n).map(|p| p.value_at(frame));
        let two = |n: &str| get(n).and_then(|v| v.as_vec2()).unwrap_or((0.0, 0.0));
        let one = |n: &str| get(n).and_then(|v| v.as_scalar()).unwrap_or(0.0);
        let ((ax, ay), (px, py), (sx, sy)) = (two("anchor"), two("position"), two("scale"));
        let rotation = one("rotation");
        let about = |m: Turn| {
            mul(
                &[1.0, 0.0, 0.0, 1.0, ax, ay],
                &mul(&m, &[1.0, 0.0, 0.0, 1.0, -ax, -ay]),
            )
        };
        let part = |f: f64| {
            let s = [
                1.0 + (sx / 100.0 - 1.0) * f,
                0.0,
                0.0,
                1.0 + (sy / 100.0 - 1.0) * f,
                0.0,
                0.0,
            ];
            let m = mul(&about(s), &about(rotate(rotation * f)));
            mul(&[1.0, 0.0, 0.0, 1.0, px * f, py * f], &m)
        };
        let whole = part(1.0);
        let back = invert(&whole);
        // D-738 (c)'s range holds copies to 0..=1000; the clamp only keeps a bad number cheap.
        let copies = one("copies").clamp(0.0, 1000.0);
        let count = copies.ceil() as usize;
        let (offset, start, end) = (one("offset"), one("start_opacity"), one("end_opacity"));
        (0..count)
            .map(|i| {
                let n = i as f64 + offset;
                let k = n.floor();
                let by = if k >= 0.0 { Some(whole) } else { back };
                let mut m = STILL;
                for _ in 0..k.abs() as usize {
                    m = mul(&m, &by?);
                }
                m = mul(&m, &part(n - k));
                let ramp = if count > 1 {
                    i as f64 / (count - 1) as f64
                } else {
                    0.0
                };
                let mut o = start + (end - start) * ramp;
                if i + 1 == count && copies != count as f64 {
                    o *= copies - copies.floor();
                }
                m.iter().all(|v| v.is_finite()).then_some((m, o))
            })
            .collect()
    }
}

impl Group {
    /// D-740: every key of its transform, its repeater and its operations `by` frames later.
    pub(crate) fn shift_keys(&mut self, by: i32) {
        let repeater = self
            .repeater
            .iter_mut()
            .flat_map(|r| r.numbers.values_mut());
        let operations = self
            .operations
            .iter_mut()
            .flat_map(|o| o.numbers.values_mut());
        for property in self
            .transform
            .values_mut()
            .chain(repeater)
            .chain(operations)
        {
            property.shift_keyframes(by);
        }
    }

    /// A group that places its shapes where their paths are.
    pub fn new(name: impl Into<String>, parent: Option<usize>) -> Group {
        let pair = |x, y| Property::constant(Value::Vec2(x, y));
        let one = |n| Property::constant(Value::Scalar(n));
        let transform = [
            ("anchor", pair(0.0, 0.0)),
            ("position", pair(0.0, 0.0)),
            ("scale", pair(100.0, 100.0)),
            ("rotation", one(0.0)),
            ("opacity", one(1.0)),
            ("skew", one(0.0)),
            ("skew_axis", one(0.0)),
        ];
        Group {
            name: name.into(),
            parent,
            transform: transform
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
            repeater: None,
            operations: Vec::new(),
            record: Default::default(),
        }
    }

    /// This group's own turn and opacity at a frame: T(position) R(rotation) K(skew) S(scale)
    /// T(-anchor), K being the shear x' = x - tan(skew) y along the skew axis, as lottie-web
    /// (MIT) writes After Effects' group transform down.
    fn turn(&self, frame: i32) -> (Turn, f64) {
        let get = |n: &str| self.transform.get(n).map(|p| p.value_at(frame));
        let two = |n: &str| get(n).and_then(|v| v.as_vec2()).unwrap_or((0.0, 0.0));
        let one = |n: &str| get(n).and_then(|v| v.as_scalar()).unwrap_or(0.0);
        let ((ax, ay), (px, py), (sx, sy)) = (two("anchor"), two("position"), two("scale"));
        let axis = one("skew_axis");
        let shear = [1.0, 0.0, -one("skew").to_radians().tan(), 1.0, 0.0, 0.0];
        let mut m = [1.0, 0.0, 0.0, 1.0, -ax, -ay];
        m = mul(&[sx / 100.0, 0.0, 0.0, sy / 100.0, 0.0, 0.0], &m);
        m = mul(&mul(&rotate(-axis), &mul(&shear, &rotate(axis))), &m);
        m = mul(&rotate(one("rotation")), &m);
        (mul(&[1.0, 0.0, 0.0, 1.0, px, py], &m), one("opacity"))
    }

    /// What is outside D-738's ranges in this group's transform, its repeater and its
    /// operations, as a sentence.
    fn problem(&self) -> Option<String> {
        let whose = format!("group \"{}\"'s", self.name);
        if self.operations.len() > MOST_OPERATIONS {
            return Some(format!(
                "{whose} operations to be at most {MOST_OPERATIONS}, not {}",
                self.operations.len()
            ));
        }
        numbers_problem(&whose, &self.transform, &GROUP_PROPERTIES)
            .or_else(|| {
                let r = self.repeater.as_ref()?;
                numbers_problem(
                    &format!("{whose} repeater"),
                    &r.numbers,
                    &REPEATER_PROPERTIES,
                )
            })
            .or_else(|| {
                let mut each = self.operations.iter().enumerate();
                each.find_map(|(k, o)| o.problem(&whose, k))
            })
    }
}

/// D-738 (b), (c): what is outside the ranges in a group's or a repeater's numbers, as a
/// sentence, each named after `whose`.
fn numbers_problem(
    whose: &str,
    numbers: &BTreeMap<String, Property>,
    names: &[&str],
) -> Option<String> {
    for &name in names {
        let what = format!("{whose} {}", name.replace('_', " "));
        let range = match name {
            "scale" => (-100000.0, 100000.0),
            "opacity" | "start_opacity" | "end_opacity" => (0.0, 1.0),
            "skew" => (-85.0, 85.0),
            "copies" => (0.0, 1000.0),
            "offset" => (-1000.0, 1000.0),
            _ => (f64::NEG_INFINITY, f64::INFINITY),
        };
        let pair = matches!(name, "anchor" | "position" | "scale");
        if let Some(said) = number_problem(&what, numbers.get(name), range, pair) {
            return Some(said);
        }
    }
    None
}

/// D-738 (b), (c), (d): what is wrong with one keyed number, `what`, as a sentence: missing, an
/// expression, a motion path, the wrong count, or a value or key outside `lo..=hi`.
fn number_problem(
    what: &str,
    p: Option<&Property>,
    (lo, hi): (f64, f64),
    pair: bool,
) -> Option<String> {
    let Some(p) = p else {
        return Some(format!("{what}, which is missing"));
    };
    if p.expression().is_some() {
        return Some(format!("no expression on {what}: it takes keys only"));
    }
    if p.split().is_some() || p.keyframes().iter().any(|k| k.spatial.is_some()) {
        return Some(format!("{what} kept whole, with no motion path"));
    }
    for v in std::iter::once(p.base()).chain(p.keyframes().iter().map(|k| k.value)) {
        let numbers = match (pair, v) {
            (true, Value::Vec2(a, b)) => vec![a, b],
            (false, Value::Scalar(n)) => vec![n],
            _ => {
                let count = if pair { "two numbers" } else { "one number" };
                return Some(format!("{what} of {count}"));
            }
        };
        if let Some(n) = numbers.iter().find(|n| !(lo..=hi).contains(*n)) {
            return Some(format!("{what} from {lo} to {hi}, not {n}"));
        }
    }
    None
}

/// D-738 (b): the whole turn and opacity of `group`, every group it sits in included. A
/// repeater is left out: this is where its first copy is with an offset of 0.
pub fn placing(groups: &[Group], group: Option<usize>, frame: i32) -> (Turn, f64) {
    chain_turn(groups, group, frame, |_| Some((STILL, 1.0))).expect("every copy is there")
}

/// D-738 (b), (c): [`placing`], each group's turn being its transform after `copy`'s turn for
/// it; `None` where `copy` has none, a copy that is not drawn.
fn chain_turn(
    groups: &[Group],
    mut group: Option<usize>,
    frame: i32,
    copy: impl Fn(usize) -> Option<(Turn, f64)>,
) -> Option<(Turn, f64)> {
    let (mut m, mut o) = (STILL, 1.0);
    // At most one step a group: [`groups_problem`] refuses a loop, and this never hangs on one.
    for _ in 0..groups.len() {
        let Some(g) = group.and_then(|at| groups.get(at).map(|g| (at, g))) else {
            break;
        };
        let (gm, go) = g.1.turn(frame);
        let (cm, co) = copy(g.0)?;
        (m, o) = (mul(&mul(&gm, &cm), &m), o * go * co);
        group = g.1.parent;
    }
    Some((m, o))
}

/// D-738 (c): every shape a layer draws at a frame, in drawing order, later over earlier, each
/// placed by its groups. A repeating group's shapes are drawn together where the first of them is
/// in the list, copy by copy, each copy holding them in the list's order; with no repeater this
/// is each shape placed, in the list's order, as D-738 (b) draws them. D-738 (d): each group's
/// operations change its paths, in its own space, before its repeater and transform place them;
/// `fps` is the composition's, which Wiggle Paths keeps time by.
pub fn drawn(shapes: &[Shape], groups: &[Group], frame: i32, fps: f64) -> Vec<Shape> {
    let operated = groups.iter().any(|g| !g.operations.is_empty());
    if !operated && groups.iter().all(|g| g.repeater.is_none()) {
        return shapes.iter().map(|s| s.placed(groups, frame)).collect();
    }
    let chains: Vec<Vec<usize>> = shapes
        .iter()
        .map(|s| {
            let mut c = Vec::new();
            let mut up = s.group;
            while let Some(g) = up.filter(|_| c.len() < groups.len()) {
                c.push(g);
                up = groups.get(g).and_then(|g| g.parent);
            }
            c
        })
        .collect();
    let copies: Vec<Option<(bool, Vec<Option<(Turn, f64)>>)>> = groups
        .iter()
        .map(|g| g.repeater.as_ref().map(|r| (r.above, r.copies(frame))))
        .collect();
    let mut out = Vec::new();
    let mut chosen = vec![0; groups.len()];
    let walk = Walk {
        shapes,
        groups,
        frame,
        chains: &chains,
        copies: &copies,
    };
    if operated {
        let seconds = frame as f64 / fps;
        return walk
            .gather(None, &mut chosen, seconds)
            .into_iter()
            .map(Item::into_shape)
            .collect();
    }
    walk.scope(None, &mut chosen, &mut out);
    out
}

/// D-738 (d): a shape on its way through [`Walk::gather`]: its paint placed, its paths in the
/// space of the group being worked, and whether a merge made it.
struct Item {
    shape: Shape,
    paths: Vec<(Vec<MaskPoint>, bool)>,
    merged: bool,
}

impl Item {
    fn into_shape(self) -> Shape {
        let mut shape = self.shape;
        if self.merged {
            shape.points = Vec::new();
            shape.outlines = Some(self.paths);
        } else if let Some((points, _)) = self.paths.into_iter().next() {
            shape.points = points;
        }
        shape
    }
}

/// D-738 (d): Merge Paths over every shape a group has drawn so far: one shape painted as the
/// first, its outlines theirs (`merge`) or those of their areas' sum, difference, overlap or
/// odd overlap.
fn merge(items: Vec<Item>, mode: &str) -> Vec<Item> {
    let Some(first) = items.first() else {
        return items;
    };
    let paths = if mode == "merge" {
        items
            .iter()
            .flat_map(|it| it.paths.iter().cloned())
            .collect()
    } else {
        let shapes: Vec<&[(Vec<MaskPoint>, bool)]> = items.iter().map(|it| &it.paths[..]).collect();
        let Some(outlines) = crate::shape_ops::merge(&shapes, mode) else {
            return items;
        };
        outlines
            .into_iter()
            .map(|c| {
                (
                    c.into_iter()
                        .map(|(x, y)| MaskPoint::corner(x, y))
                        .collect(),
                    true,
                )
            })
            .collect()
    };
    vec![Item {
        shape: first.shape.clone(),
        paths,
        merged: true,
    }]
}

/// D-738 (c): what [`drawn`] works through.
struct Walk<'a> {
    shapes: &'a [Shape],
    groups: &'a [Group],
    frame: i32,
    /// Each shape's groups, its own first, out to the layer.
    chains: &'a [Vec<usize>],
    /// Each repeating group's composite and copies.
    copies: &'a [Option<(bool, Vec<Option<(Turn, f64)>>)>],
}

impl Walk<'_> {
    /// Group `g`'s copy `chosen` holds, its turn and opacity; `None` for one not drawn.
    fn copy(&self, g: usize, chosen: &[usize]) -> Option<(Turn, f64)> {
        match &self.copies[g] {
            None => Some((STILL, 1.0)),
            Some((_, list)) => list.get(chosen[g]).copied().flatten(),
        }
    }

    /// D-738 (d): [`Walk::scope`]'s shapes, each group's operations, its merges included,
    /// worked on what it has drawn before its repeater's copy and its transform place it, `scope`'s
    /// own included. Paint is placed whole, as [`Walk::scope`] places it.
    fn gather(&self, scope: Option<usize>, chosen: &mut [usize], seconds: f64) -> Vec<Item> {
        let mut items = Vec::new();
        let mut done = vec![false; self.groups.len()];
        for (i, chain) in self.chains.iter().enumerate() {
            let at = match scope {
                None => Some(chain.len()),
                Some(g) => chain.iter().position(|&c| c == g),
            };
            match at {
                None => {}
                Some(0) => {
                    let s = &self.shapes[i];
                    let copy = |g: usize| self.copy(g, chosen);
                    if let Some((m, o)) = chain_turn(self.groups, s.group, self.frame, copy) {
                        items.push(Item {
                            shape: s.placed_by(self.frame, s.group.map(|_| (m, o))),
                            paths: vec![(s.points_at(self.frame), s.closed)],
                            merged: false,
                        });
                    }
                }
                Some(at) => {
                    let g = chain[at - 1];
                    if std::mem::replace(&mut done[g], true) {
                        continue;
                    }
                    let Some((above, list)) = &self.copies[g] else {
                        items.extend(self.gather(Some(g), chosen, seconds));
                        continue;
                    };
                    let (above, n) = (*above, list.len());
                    for c in 0..n {
                        chosen[g] = if above { c } else { n - 1 - c };
                        items.extend(self.gather(Some(g), chosen, seconds));
                    }
                }
            }
        }
        let Some(g) = scope else {
            return items;
        };
        let group = &self.groups[g];
        for o in &group.operations {
            if o.kind == "merge" {
                items = merge(items, o.choice.as_deref().unwrap_or("add"));
                continue;
            }
            for (pts, closed) in items.iter_mut().flat_map(|it| &mut it.paths) {
                if pts.len() <= crate::shape_ops::LIMIT {
                    *pts = o.apply(std::mem::take(pts), *closed, self.frame, seconds);
                }
            }
        }
        let (cm, _) = self.copy(g, chosen).unwrap_or((STILL, 1.0));
        let m = mul(&group.turn(self.frame).0, &cm);
        for p in items
            .iter_mut()
            .flat_map(|it| &mut it.paths)
            .flat_map(|(pts, _)| pts)
        {
            p.point = apply(&m, p.point, false);
            p.in_handle = apply(&m, p.in_handle, true);
            p.out_handle = apply(&m, p.out_handle, true);
        }
        items
    }

    /// The shapes in `scope`, a group or the layer, each copy of a repeating group inside it in
    /// turn, `chosen` holding the copy of each repeating group outside it.
    fn scope(&self, scope: Option<usize>, chosen: &mut [usize], out: &mut Vec<Shape>) {
        let mut done = vec![false; self.groups.len()];
        for (i, chain) in self.chains.iter().enumerate() {
            let at = match scope {
                None => Some(chain.len()),
                Some(g) => chain.iter().position(|&c| c == g),
            };
            match at {
                None => {}
                Some(0) => {
                    let s = &self.shapes[i];
                    let copy = |g: usize| self.copy(g, chosen);
                    if let Some((m, o)) = chain_turn(self.groups, s.group, self.frame, copy) {
                        out.push(s.placed_by(self.frame, s.group.map(|_| (m, o))));
                    }
                }
                Some(at) => {
                    let g = chain[at - 1];
                    if std::mem::replace(&mut done[g], true) {
                        continue;
                    }
                    let Some((above, list)) = &self.copies[g] else {
                        self.scope(Some(g), chosen, out);
                        continue;
                    };
                    let (above, n) = (*above, list.len());
                    for c in 0..n {
                        chosen[g] = if above { c } else { n - 1 - c };
                        self.scope(Some(g), chosen, out);
                    }
                }
            }
        }
    }
}

/// D-738 (b): what is wrong with a layer's groups and its shapes' places in them, as a sentence.
/// The loader refuses on it with `PROJECT_SCHEMA_INVALID` and the commands with
/// `COMMAND_INVALID_VALUE`.
pub fn groups_problem(groups: &[Group], shapes: &[Shape]) -> Option<String> {
    for (i, g) in groups.iter().enumerate() {
        if let Some(p) = g.parent {
            if p >= groups.len() || p == i {
                return Some(format!(
                    "group \"{}\"'s parent to be another of the layer's {} groups, not {p}",
                    g.name,
                    groups.len()
                ));
            }
        }
        // A chain longer than the list has come back on itself.
        let mut up = g.parent;
        for _ in 0..groups.len() {
            up = up.and_then(|u| groups[u].parent);
        }
        if up.is_some() {
            return Some(format!(
                "group \"{}\" not inside itself, but its parent leads back to it",
                g.name
            ));
        }
        if let Some(p) = g.problem() {
            return Some(p);
        }
    }
    shapes.iter().find_map(|s| {
        s.group.filter(|g| *g >= groups.len()).map(|g| {
            format!(
                "shape \"{}\"'s group to be one of the layer's {} groups, not {g}",
                s.name,
                groups.len()
            )
        })
    })
}

/// D-170: how a stroke turns a corner. Round is D-78's, and what a stroke has unless told.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Join {
    Miter,
    Round,
    Bevel,
}

/// D-170: how an open line ends. Round is D-78's.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cap {
    Butt,
    Round,
    Square,
}

impl Join {
    pub const NAMES: [&str; 3] = ["miter", "round", "bevel"];
    pub fn as_str(self) -> &'static str {
        Join::NAMES[self as usize]
    }
    pub fn named(name: &str) -> Option<Join> {
        [Join::Miter, Join::Round, Join::Bevel]
            .into_iter()
            .find(|j| j.as_str() == name)
    }
}

impl Cap {
    pub const NAMES: [&str; 3] = ["butt", "round", "square"];
    pub fn as_str(self) -> &'static str {
        Cap::NAMES[self as usize]
    }
    pub fn named(name: &str) -> Option<Cap> {
        [Cap::Butt, Cap::Round, Cap::Square]
            .into_iter()
            .find(|c| c.as_str() == name)
    }
}

/// D-169: the stretch of the path a stroke is drawn along, as After Effects' Trim Paths.
///
/// `start` and `end` are percent of the way along, 0 to 100; `offset` is degrees, 360 once along.
/// Each is a one-number property keyed as any is. The fill is not trimmed.
#[derive(Clone, PartialEq, Debug)]
pub struct Trim {
    pub start: Property,
    pub end: Property,
    pub offset: Property,
}

impl Trim {
    /// The whole path, no offset: what the window puts on a shape when trimming is turned on.
    pub fn whole() -> Trim {
        let n = |v| Property::constant(Value::Scalar(v));
        Trim {
            start: n(0.0),
            end: n(100.0),
            offset: n(0.0),
        }
    }

    fn at(&self, frame: i32) -> Trim {
        let now = |p: &Property| Property::constant(p.value_at(frame));
        Trim {
            start: now(&self.start),
            end: now(&self.end),
            offset: now(&self.offset),
        }
    }

    /// What is outside D-169's ranges, as a sentence, for [`Shape::problem`].
    fn problem(&self) -> Option<String> {
        for (name, p, ranged) in [
            ("start", &self.start, true),
            ("end", &self.end, true),
            ("offset", &self.offset, false),
        ] {
            let values: Vec<Option<f64>> = std::iter::once(p.base())
                .chain(p.keyframes().iter().map(|k| k.value))
                .map(|v| v.as_scalar())
                .collect();
            if p.split().is_some() || values.iter().any(Option::is_none) {
                return Some(format!("a trim {name} of one number"));
            }
            if p.expression().is_some() {
                return Some(format!("no expression on a trim {name}"));
            }
            if let Some(v) = values
                .into_iter()
                .flatten()
                .find(|v| ranged && !(0.0..=100.0).contains(v))
            {
                return Some(format!("a trim {name} from 0 to 100, not {v}"));
            }
        }
        None
    }

    /// D-169's stretch of `path` as open lines to stroke, or `None` where the stroke is whole.
    ///
    /// The trim must already be at the frame. `path_segments` in order are the pieces measured.
    /// D-170: a closed path's stretch that runs through its first point is one line through it.
    fn lines(&self, path: &[(f64, f64)], closed: bool) -> Option<Vec<Vec<(f64, f64)>>> {
        let n = |p: &Property| p.base().as_scalar().unwrap_or(0.0);
        let (s, e) = (n(&self.start) / 100.0, n(&self.end) / 100.0);
        let (s, e) = (s.min(e), s.max(e));
        let pieces = path_segments(path, closed);
        let runs: Vec<f64> = pieces
            .iter()
            .map(|(a, b)| (b.0 - a.0).hypot(b.1 - a.1))
            .collect();
        let mut along = vec![0.0];
        for r in &runs {
            along.push(along[along.len() - 1] + r);
        }
        let total = along[along.len() - 1];
        if e - s >= 1.0 || total == 0.0 {
            return None;
        }
        if e == s {
            return Some(Vec::new());
        }
        let place = |d: f64| {
            let i = (0..runs.len())
                .find(|&i| runs[i] > 0.0 && along[i + 1] >= d)
                .unwrap_or_else(|| {
                    runs.iter()
                        .rposition(|&r| r > 0.0)
                        .expect("the path has length")
                });
            let f = (d - along[i]) / runs[i];
            let (p, q) = pieces[i];
            (p.0 + (q.0 - p.0) * f, p.1 + (q.1 - p.1) * f)
        };
        let mut a = s + n(&self.offset) / 360.0;
        a -= a.floor();
        let b = a + (e - s);
        let spans = if b <= 1.0 {
            vec![(a, b)]
        } else {
            vec![(a, 1.0), (0.0, b - 1.0)]
        };
        let mut out = Vec::new();
        for (u0, u1) in spans {
            let (d0, d1) = (u0 * total, u1 * total);
            let mut line = vec![place(d0)];
            line.extend(
                (1..pieces.len())
                    .filter(|&j| d0 < along[j] && along[j] < d1)
                    .map(|j| pieces[j].0),
            );
            line.push(place(d1));
            out.push(line);
        }
        if closed && out.len() == 2 {
            let wrapped = out.pop().expect("two");
            out[0].extend_from_slice(&wrapped[1..]);
        }
        Some(out)
    }
}

/// D-78: a shape's fill — one colour over its whole even-odd interior.
///
/// The colour is linear working-space RGB from 0 to 1, as a solid's is and as the tint effect's
/// is. D-170 lets both be keyed; the keys are in [`Shape::tracks`] and these are the bases.
#[derive(Clone, PartialEq, Debug)]
pub struct Fill {
    pub color: [f64; 3],
    /// 0 to 1.
    pub opacity: f64,
    /// D-168: drawn instead of `color` when present; `color` is kept for when it is taken off.
    pub gradient: Option<Gradient>,
}

/// D-78: a shape's stroke — one colour over every point within half its width of the path.
#[derive(Clone, PartialEq, Debug)]
pub struct Stroke {
    pub color: [f64; 3],
    /// 0 to 1.
    pub opacity: f64,
    /// Above 0 and at most 8192, in pixels. The band is half of it either side of the path.
    pub width_px: f64,
    /// D-168: as a fill's. It lies across the frame by position, not along the line.
    pub gradient: Option<Gradient>,
    /// D-170: round and round, with a limit of 4, unless the file or the panel says otherwise.
    pub join: Join,
    /// 1 to 100, in half-widths from the corner.
    pub miter_limit: f64,
    pub cap: Cap,
}

impl Stroke {
    /// A new stroke from the window: opaque, no gradient, and D-78's round joins and caps, which
    /// D-170 keeps as the window's choice (After Effects starts mitred with butt caps).
    pub fn new(color: [f64; 3], width_px: f64) -> Stroke {
        Stroke {
            color,
            opacity: 1.0,
            width_px,
            gradient: None,
            join: Join::Round,
            miter_limit: 4.0,
            cap: Cap::Round,
        }
    }
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
    /// D-169: where along the path the stroke is drawn; `None` is all of it.
    pub trim: Option<Trim>,
    /// D-170: the keys of a style that has any, by its name in `SHAPE_PROPERTIES`
    /// (`fill_color` and so on), one one-number property per channel, a colour's three sharing
    /// their frames. The fill's and stroke's own numbers are the bases. A style without keys has
    /// no entry, as an effect's setting has none (D-68).
    pub tracks: BTreeMap<String, Vec<Property>>,
    /// ADR-008: the record this shape was read from, which saving writes over.
    pub record: crate::model::FileRecord,
    /// D-738 (a): a rectangle, ellipse, polygon or star whose outline is worked from its
    /// settings at each frame. `points` then holds its outline at the settings' bases, which is
    /// what the file's `path` is written from, and `keys` is empty.
    pub parametric: Option<Parametric>,
    /// D-738 (b): the place in the layer's groups of the group this shape is in.
    pub group: Option<usize>,
    /// D-738 (d): a shape Merge Paths made, drawn from these outlines, each open or closed, in
    /// place of `points`. Only [`drawn`] gives one; never saved.
    pub outlines: Option<Vec<(Vec<MaskPoint>, bool)>>,
}

impl Shape {
    /// D-740: every key of the path, its styles, gradients, trim and parametric settings `by`
    /// frames later.
    pub(crate) fn shift_keys(&mut self, by: i32) {
        self.keys.iter_mut().for_each(|key| key.frame += by);
        let gradients = [
            self.fill.as_mut().and_then(|f| f.gradient.as_mut()),
            self.stroke.as_mut().and_then(|s| s.gradient.as_mut()),
        ];
        let points = gradients
            .into_iter()
            .flatten()
            .flat_map(|g| [&mut g.start, &mut g.end]);
        let trim = self
            .trim
            .iter_mut()
            .flat_map(|t| [&mut t.start, &mut t.end, &mut t.offset]);
        let settings = self
            .parametric
            .iter_mut()
            .flat_map(|p| p.settings.values_mut());
        for property in (self.tracks.values_mut().flatten())
            .chain(points)
            .chain(trim)
            .chain(settings)
        {
            property.shift_keyframes(by);
        }
    }
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
            trim: None,
            tracks: BTreeMap::new(),
            record: Default::default(),
            parametric: None,
            group: None,
            outlines: None,
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
        let mut now = Shape {
            points: self.points_at(frame),
            keys: Vec::new(),
            parametric: None,
            ..self.clone()
        };
        if let Some(g) = now.fill.as_mut().and_then(|f| f.gradient.as_mut()) {
            *g = g.at(frame);
        }
        if let Some(g) = now.stroke.as_mut().and_then(|s| s.gradient.as_mut()) {
            *g = g.at(frame);
        }
        now.trim = self.trim.as_ref().map(|t| t.at(frame));
        now.tracks.clear();
        for (name, track) in &self.tracks {
            if let Some(values) = now.style_mut(name) {
                for (v, channel) in values.iter_mut().zip(track) {
                    *v = channel.value_at(frame).as_scalar().unwrap_or(*v);
                }
            }
        }
        now
    }

    /// D-738 (b): this shape at a frame, placed by its group and every group that group is in:
    /// points by the whole turn, handles by its linear part, gradient points as points, fill and
    /// stroke opacity by every group's opacity, stroke width by the square root of the turn's
    /// size. A shape in no group is [`Shape::at`] unchanged.
    pub fn placed(&self, groups: &[Group], frame: i32) -> Shape {
        let turn = self.group.map(|_| placing(groups, self.group, frame));
        self.placed_by(frame, turn)
    }

    /// [`Shape::placed`] by a turn and opacity already worked out; `None` leaves it as it is.
    fn placed_by(&self, frame: i32, turn: Option<(Turn, f64)>) -> Shape {
        let mut now = self.at(frame);
        let Some((m, o)) = turn else {
            return now;
        };
        for p in &mut now.points {
            p.point = apply(&m, p.point, false);
            p.in_handle = apply(&m, p.in_handle, true);
            p.out_handle = apply(&m, p.out_handle, true);
        }
        let place = |g: &mut Gradient| {
            for end in [&mut g.start, &mut g.end] {
                let (x, y) = apply(&m, end.base().as_vec2().unwrap_or_default(), false);
                *end = Property::constant(Value::Vec2(x, y));
            }
        };
        if let Some(f) = now.fill.as_mut() {
            f.opacity *= o;
            if let Some(g) = f.gradient.as_mut() {
                place(g);
            }
        }
        if let Some(s) = now.stroke.as_mut() {
            s.opacity *= o;
            s.width_px *= (m[0] * m[3] - m[1] * m[2]).abs().sqrt();
            if let Some(g) = s.gradient.as_mut() {
                place(g);
            }
        }
        now
    }

    /// D-170: the numbers a style's name stands for, where the shape has that paint.
    fn style_mut(&mut self, name: &str) -> Option<&mut [f64]> {
        Some(match name {
            "fill_color" => &mut self.fill.as_mut()?.color[..],
            "fill_opacity" => std::slice::from_mut(&mut self.fill.as_mut()?.opacity),
            "stroke_color" => &mut self.stroke.as_mut()?.color[..],
            "stroke_opacity" => std::slice::from_mut(&mut self.stroke.as_mut()?.opacity),
            "stroke_width" => std::slice::from_mut(&mut self.stroke.as_mut()?.width_px),
            _ => return None,
        })
    }

    /// A keyed number of this shape by its name in `SHAPE_PROPERTIES`, as one property per
    /// channel: three for a colour, one for anything else. `None` where the shape has no such
    /// thing, as a gradient point on a fill without a gradient.
    pub fn channels(&self, name: &str) -> Option<Vec<Property>> {
        if let Some(setting) = name.strip_prefix("path_") {
            let p = self.parametric.as_ref()?;
            p.kind.settings().contains(&setting).then_some(())?;
            return p.settings.get(setting).map(|p| vec![p.clone()]);
        }
        let mut shape = self.clone();
        if let Some(values) = shape.style_mut(name) {
            return Some(self.tracks.get(name).cloned().unwrap_or_else(|| {
                values
                    .iter()
                    .map(|v| Property::constant(Value::Scalar(*v)))
                    .collect()
            }));
        }
        shape.property_mut(name).map(|p| vec![p.clone()])
    }

    /// Puts back what [`Shape::channels`] handed out. A style keeps its bases as its plain
    /// numbers and its keys, if it has any left, as its track.
    pub fn set_channels(&mut self, name: &str, channels: Vec<Property>) -> Option<()> {
        if let Some(setting) = name.strip_prefix("path_") {
            let p = self.parametric.as_mut()?;
            p.kind.settings().contains(&setting).then_some(())?;
            p.settings
                .insert(setting.to_string(), channels.into_iter().next()?);
            self.points = p.outline(None);
            return Some(());
        }
        if let Some(values) = self.style_mut(name) {
            for (v, channel) in values.iter_mut().zip(&channels) {
                *v = channel.base().as_scalar().unwrap_or(*v);
            }
            if channels.iter().any(|c| !c.keyframes().is_empty()) {
                self.tracks.insert(name.to_string(), channels);
            } else {
                self.tracks.remove(name);
            }
            return Some(());
        }
        *self.property_mut(name)? = channels.into_iter().next()?;
        Some(())
    }

    /// B-109b: a gradient's two points and a trim's three numbers by their window names.
    fn property_mut(&mut self, name: &str) -> Option<&mut Property> {
        let (group, which) = name.split_once('_')?;
        let (start, end, offset) = match group {
            "fill" => {
                let g = self.fill.as_mut()?.gradient.as_mut()?;
                (&mut g.start, &mut g.end, None)
            }
            "stroke" => {
                let g = self.stroke.as_mut()?.gradient.as_mut()?;
                (&mut g.start, &mut g.end, None)
            }
            "trim" => {
                let t = self.trim.as_mut()?;
                (&mut t.start, &mut t.end, Some(&mut t.offset))
            }
            _ => return None,
        };
        match which {
            "start" => Some(start),
            "end" => Some(end),
            "offset" => offset,
            _ => None,
        }
    }

    /// The path at a composition frame: document 20's rules, shared with a mask's path.
    pub fn points_at(&self, frame: i32) -> Vec<MaskPoint> {
        if let Some(p) = &self.parametric {
            return p.outline(Some(frame));
        }
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
        self.enabled && (self.outlines.is_some() || self.has_enough_points())
    }

    /// What is outside D-78's ranges, as a sentence, or `None` for a shape that may exist.
    ///
    /// The commands refuse on this with `COMMAND_INVALID_VALUE` and the loader with
    /// `PROJECT_SCHEMA_INVALID`; both say the same sentence, because it is the same rule.
    pub fn problem(&self) -> Option<String> {
        if let Some(p) = &self.parametric {
            let kind = p.kind.as_str();
            if !self.closed {
                return Some(format!("a {kind} to be closed, not open"));
            }
            if !self.keys.is_empty() {
                return Some(format!(
                    "no keys on a {kind}'s path: its settings take the keys"
                ));
            }
            if let Some(said) = p.problem() {
                return Some(said);
            }
        }
        if let Some(p) = self.style_problem() {
            return Some(p);
        }
        // D-170: every key of a keyed style is held to the plain value's range, by asking the
        // shape as it stands on that key's frame.
        for track in self.tracks.values() {
            for channel in track {
                if channel.expression().is_some() {
                    return Some("no expression on a shape's colour, opacity or width".into());
                }
                for key in channel.keyframes() {
                    if let Some(p) = self.at(key.frame).style_problem() {
                        return Some(p);
                    }
                }
            }
        }
        for (what, gradient) in [
            self.fill.as_ref().map(|f| ("fill", &f.gradient)),
            self.stroke.as_ref().map(|s| ("stroke", &s.gradient)),
        ]
        .into_iter()
        .flatten()
        {
            if let Some(p) = gradient.as_ref().and_then(|g| g.problem(what)) {
                return Some(p);
            }
        }
        if let Some(p) = self.trim.as_ref().and_then(Trim::problem) {
            return Some(p);
        }
        if let Some(s) = &self.stroke {
            if !(1.0..=100.0).contains(&s.miter_limit) {
                return Some(format!(
                    "a mitre limit from 1 to 100, not {}",
                    s.miter_limit
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

    /// D-78's ranges on the plain colours, opacities and width.
    fn style_problem(&self) -> Option<String> {
        for (what, color, opacity) in [
            self.fill.as_ref().map(|f| ("fill", f.color, f.opacity)),
            self.stroke.as_ref().map(|s| ("stroke", s.color, s.opacity)),
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
        let width = self.stroke.as_ref()?.width_px;
        (!(width > 0.0 && width <= MAX_STROKE_WIDTH))
            .then(|| format!("a stroke width above 0 and at most {MAX_STROKE_WIDTH}, not {width}"))
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

/// D-168's gradient at one frame, made ready to ask per pixel: the stops in offset order (a stable
/// sort, so two at one offset keep their written order and make a hard step) and their colours
/// already on the sRGB curve, which is where they are mixed.
struct Ramp {
    radial: bool,
    start: (f64, f64),
    along: (f64, f64),
    run: f64,
    stops: Vec<(f64, [f64; 3], f64)>,
}

impl Ramp {
    fn new(g: &Gradient) -> Ramp {
        let point = |p: &Property| p.base().as_vec2().unwrap_or((0.0, 0.0));
        let (start, end) = (point(&g.start), point(&g.end));
        let along = (end.0 - start.0, end.1 - start.1);
        let mut stops = g.stops.clone();
        stops.sort_by(|a, b| a.offset.total_cmp(&b.offset));
        Ramp {
            radial: g.kind == GradientKind::Radial,
            start,
            along,
            run: along.0 * along.0 + along.1 * along.1,
            stops: stops
                .iter()
                .map(|s| (s.offset, s.color.map(crate::grade::to_srgb), s.opacity))
                .collect(),
        }
    }

    /// The linear colour and the stop opacity at pixel `i` of a picture `width` across.
    fn at(&self, i: usize, width: usize) -> ([f64; 3], f64) {
        let px = (i % width) as f64 + 0.5 - self.start.0;
        let py = (i / width) as f64 + 0.5 - self.start.1;
        let t = if self.run == 0.0 {
            1.0
        } else if self.radial {
            px.hypot(py) / self.run.sqrt()
        } else {
            (px * self.along.0 + py * self.along.1) / self.run
        }
        .clamp(0.0, 1.0);
        let s = &self.stops;
        let (first, last) = (s[0], s[s.len() - 1]);
        let (a, b, f) = if t < first.0 {
            (first, first, 0.0)
        } else {
            let i = s
                .iter()
                .rposition(|stop| stop.0 <= t)
                .expect("t is past the first");
            match s.get(i + 1) {
                None => (last, last, 0.0),
                Some(&b) => (s[i], b, (t - s[i].0) / (b.0 - s[i].0)),
            }
        };
        let color = [0, 1, 2].map(|c| crate::grade::to_linear(a.1[c] + (b.1[c] - a.1[c]) * f));
        (color, a.2 + (b.2 - a.2) * f)
    }
}

/// One coverage field of one colour, laid over what the layer holds so far.
///
/// Document 21's normal blend in premultiplied linear RGBA: the source is the colour times its
/// own alpha, and `src + dst * (1 - src_a)` is the whole of it. With a gradient (D-168) the colour
/// and a stop opacity come from it, pixel by pixel.
// ponytail: one thread and three sRGB curves a covered pixel; a 1080p gradient fill is some tens
// of milliseconds. Run the rows across rayon, as the stroke field does, if that shows.
pub(crate) fn paint(
    picture: &mut [f32],
    field: &[f32],
    width: usize,
    color: [f64; 3],
    opacity: f64,
    gradient: Option<&Gradient>,
) {
    let ramp = gradient.map(Ramp::new);
    for (i, coverage) in field.iter().enumerate() {
        let mut a = *coverage * opacity as f32;
        if a == 0.0 {
            continue;
        }
        let mut color = color;
        if let Some(ramp) = &ramp {
            let (c, o) = ramp.at(i, width);
            color = c;
            a *= o as f32;
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
        // D-738 (d): a merged shape's outlines, each flattened as one shape's path is.
        let outlines: Vec<(Vec<(f64, f64)>, bool)> = match &shape.outlines {
            Some(o) => o
                .iter()
                .map(|(p, closed)| (crate::mask::flatten(p, *closed), *closed))
                .collect(),
            None => vec![(shape.outline(), shape.closed)],
        };
        // The fill first, then the stroke over this shape's own fill. D-78 fixes that order, and
        // it is why FX-SHP-008's shared columns carry the stroke and not a mixture.
        if let Some(fill) = &shape.fill {
            // An open path is closed for the purpose of filling it, by a straight line from its
            // last point to its first: the even-odd ray already wraps, so nothing is added here.
            // Several outlines are filled as one ring: each closed, then a step back to the first
            // one's first point, so every step out to the next is walked there and back and
            // crosses nothing.
            let ring;
            let outline: &[(f64, f64)] = match &outlines[..] {
                [(one, _)] => one,
                many => {
                    let home = many.iter().find_map(|(l, _)| l.first().copied());
                    let mut steps = Vec::new();
                    for (l, _) in many {
                        if let (Some(&start), Some(home)) = (l.first(), home) {
                            steps.extend_from_slice(l);
                            steps.extend([start, home]);
                        }
                    }
                    ring = steps;
                    &ring
                }
            };
            let f = crate::mask::scanline_field(outline, width, height, 0);
            paint(
                data,
                &f,
                width,
                fill.color,
                fill.opacity,
                fill.gradient.as_ref(),
            );
        }
        if let Some(stroke) = &shape.stroke {
            let mut lines: Vec<(Vec<(f64, f64)>, bool)> = Vec::new();
            for (outline, closed) in &outlines {
                match shape.trim.as_ref().and_then(|t| t.lines(outline, *closed)) {
                    Some(stretch) => lines.extend(stretch.into_iter().map(|l| (l, false))),
                    None => lines.push((outline.clone(), *closed)),
                }
            }
            let r = stroke.width_px / 2.0;
            let f = if stroke.join == Join::Round && stroke.cap == Cap::Round {
                // D-78's coverage, unchanged.
                let segments: Vec<Segment> = lines
                    .iter()
                    .flat_map(|(l, closed)| path_segments(l, *closed))
                    .collect();
                stroke_field(&segments, width, height, r)
            } else {
                styled_field(&stroke_parts(&lines, r, stroke), width, height)
            };
            paint(
                data,
                &f,
                width,
                stroke.color,
                stroke.opacity,
                stroke.gradient.as_ref(),
            );
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
///
/// Along a sample row the samples move right, so the band is swept in order of its left ends: an
/// edge joins the ones asked when a sample comes within `reach` of its left end and leaves for
/// good once the samples are past its right end by more than `reach`. The nearest of the same
/// edges is the same nearest, whatever order they are asked in.
pub(crate) fn stroke_field(segments: &[Segment], w: usize, h: usize, reach: f64) -> Vec<f32> {
    use rayon::prelude::*;
    let n = SAMPLES_PER_SIDE;
    let mut field = vec![0.0f32; w * h];
    if segments.is_empty() {
        return field;
    }
    field.par_chunks_mut(w).enumerate().for_each_init(
        || (Vec::new(), vec![0u32; w], Vec::new()),
        |(band, hits, near), (yi, row)| {
            hits.iter_mut().for_each(|hit| *hit = 0);
            for j in 0..n {
                let sy = yi as f64 + (j as f64 + 0.5) / n as f64;
                band.clear();
                for &(a, b) in segments {
                    let (low, high) = if a.1 <= b.1 { (a.1, b.1) } else { (b.1, a.1) };
                    if low - reach <= sy && sy <= high + reach {
                        let (left, right) = if a.0 <= b.0 { (a.0, b.0) } else { (b.0, a.0) };
                        band.push((a, b, left, right));
                    }
                }
                if band.is_empty() {
                    continue;
                }
                band.sort_by(|p: &(_, _, f64, f64), q| p.2.total_cmp(&q.2));
                near.clear();
                let mut next = 0;
                for (xi, hit) in hits.iter_mut().enumerate() {
                    for i in 0..n {
                        let sx = xi as f64 + (i as f64 + 0.5) / n as f64;
                        while next < band.len() && !(band[next].2 - reach > sx) {
                            near.push(band[next]);
                            next += 1;
                        }
                        near.retain(|p: &(_, _, f64, f64)| sx <= p.3 + reach);
                        let mut nearest = f64::INFINITY;
                        for &(a, b, _, _) in near.iter() {
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

type Segment = ((f64, f64), (f64, f64));

/// The straight pieces of a flattened path in order, a closed one's closing side last. One point
/// is a segment from it to itself, which is what `distance_to_path` measures then.
pub(crate) fn path_segments(path: &[(f64, f64)], closed: bool) -> Vec<Segment> {
    match path.len() {
        0 => Vec::new(),
        1 => vec![(path[0], path[0])],
        points => (0..if closed { points } else { points - 1 })
            .map(|i| (path[i], path[(i + 1) % points]))
            .collect(),
    }
}

/// One of the pieces D-170 makes a stroke of, with the box outside which it covers nothing.
struct Part {
    left: f64,
    top: f64,
    right: f64,
    bottom: f64,
    region: Region,
}

enum Region {
    Disc {
        c: (f64, f64),
        r: f64,
    },
    Square {
        c: (f64, f64),
        r: f64,
    },
    /// A piece's band: from `a` along (dx, dy), `low <= p.d <= high` and `|p x d| <= reach`.
    Body {
        a: (f64, f64),
        dx: f64,
        dy: f64,
        low: f64,
        high: f64,
        reach: f64,
    },
    /// A convex polygon, either way round.
    Corner(Vec<(f64, f64)>),
}

impl Part {
    /// The box only passes over samples, never decides one, so it is padded past any rounding.
    fn new(region: Region, points: &[(f64, f64)], pad: f64) -> Part {
        let pad = pad + 1e-6;
        let (left, top, right, bottom) = points.iter().fold(
            (
                f64::INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::NEG_INFINITY,
            ),
            |(l, t, r, b), p| (l.min(p.0), t.min(p.1), r.max(p.0), b.max(p.1)),
        );
        Part {
            left: left - pad,
            top: top - pad,
            right: right + pad,
            bottom: bottom + pad,
            region,
        }
    }

    fn round(c: (f64, f64), r: f64) -> Part {
        Part::new(Region::Disc { c, r }, &[c], r)
    }

    fn corner(poly: Vec<(f64, f64)>) -> Part {
        let points = poly.clone();
        Part::new(Region::Corner(poly), &points, 0.0)
    }

    /// Document 21's D-170 tests, in the reference tool's arithmetic, operation for operation.
    fn covers(&self, x: f64, y: f64) -> bool {
        match &self.region {
            Region::Disc { c, r } => (x - c.0) * (x - c.0) + (y - c.1) * (y - c.1) <= r * r,
            Region::Square { c, r } => (x - c.0).abs() <= *r && (y - c.1).abs() <= *r,
            Region::Body {
                a,
                dx,
                dy,
                low,
                high,
                reach,
            } => {
                let (px, py) = (x - a.0, y - a.1);
                let dot = px * dx + py * dy;
                let cross = px * dy - py * dx;
                *low <= dot && dot <= *high && cross.abs() <= *reach
            }
            Region::Corner(poly) => {
                let (mut above, mut below) = (true, true);
                for (i, p) in poly.iter().enumerate() {
                    let q = poly[(i + 1) % poly.len()];
                    let side = (q.0 - p.0) * (y - p.1) - (q.1 - p.1) * (x - p.0);
                    above &= side >= 0.0;
                    below &= side <= 0.0;
                }
                above || below
            }
        }
    }
}

/// D-170: a piece from `a` to `b`, carried on `before` and `after` pixels past its ends.
fn body(a: (f64, f64), b: (f64, f64), r: f64, before: f64, after: f64) -> Part {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let run2 = dx * dx + dy * dy;
    let run = run2.sqrt();
    let region = Region::Body {
        a,
        dx,
        dy,
        low: -before * run,
        high: run2 + after * run,
        reach: r * run,
    };
    Part::new(region, &[a, b], r + before + after)
}

/// D-170: the join at `v` between a piece from `a` and one on to `c`.
fn joint(a: (f64, f64), v: (f64, f64), c: (f64, f64), r: f64, stroke: &Stroke) -> Option<Part> {
    if stroke.join == Join::Round {
        return Some(Part::round(v, r));
    }
    let run1 = ((v.0 - a.0) * (v.0 - a.0) + (v.1 - a.1) * (v.1 - a.1)).sqrt();
    let run2 = ((c.0 - v.0) * (c.0 - v.0) + (c.1 - v.1) * (c.1 - v.1)).sqrt();
    let u1 = ((v.0 - a.0) / run1, (v.1 - a.1) / run1);
    let u2 = ((c.0 - v.0) / run2, (c.1 - v.1) / run2);
    let turn = u1.0 * u2.1 - u1.1 * u2.0;
    if turn == 0.0 {
        return None;
    }
    let s = if turn > 0.0 { -1.0 } else { 1.0 };
    let (n1, n2) = ((-u1.1, u1.0), (-u2.1, u2.0));
    let p1 = (v.0 + s * r * n1.0, v.1 + s * r * n1.1);
    let p2 = (v.0 + s * r * n2.0, v.1 + s * r * n2.1);
    let d = u1.0 * u2.0 + u1.1 * u2.1;
    let limit = stroke.miter_limit;
    if stroke.join == Join::Miter && 2.0 <= limit * limit * (1.0 + d) {
        let k = s * r / (1.0 + d);
        let m = (v.0 + k * (n1.0 + n2.0), v.1 + k * (n1.1 + n2.1));
        return Some(Part::corner(vec![v, p1, m, p2]));
    }
    Some(Part::corner(vec![v, p1, p2]))
}

/// D-170: every piece of a stroke whose join or cap is not round, over all its lines.
fn stroke_parts(lines: &[(Vec<(f64, f64)>, bool)], r: f64, stroke: &Stroke) -> Vec<Part> {
    let mut parts = Vec::new();
    for (line, closed) in lines {
        let Some(&first) = line.first() else { continue };
        let mut points = line.clone();
        if *closed {
            points.push(first);
        }
        let segs: Vec<Segment> = points
            .windows(2)
            .map(|w| (w[0], w[1]))
            .filter(|(p, q)| p != q)
            .collect();
        if segs.is_empty() {
            match stroke.cap {
                Cap::Round => parts.push(Part::round(first, r)),
                Cap::Square => parts.push(Part::new(Region::Square { c: first, r }, &[first], r)),
                Cap::Butt => {}
            }
            continue;
        }
        let square = !closed && stroke.cap == Cap::Square;
        let last = segs.len() - 1;
        for (i, &(a, b)) in segs.iter().enumerate() {
            let before = if square && i == 0 { r } else { 0.0 };
            let after = if square && i == last { r } else { 0.0 };
            parts.push(body(a, b, r, before, after));
        }
        if !closed && stroke.cap == Cap::Round {
            parts.push(Part::round(segs[0].0, r));
            parts.push(Part::round(segs[last].1, r));
        }
        let wrap = closed.then(|| (segs[last], segs[0]));
        for ((a, v), (_, c)) in segs.windows(2).map(|w| (w[0], w[1])).chain(wrap) {
            parts.extend(joint(a, v, c, r, stroke));
        }
    }
    parts
}

/// Every pixel's coverage by `parts`: a sample counts if any part covers it. Rows run across the
/// thread pool, each asking only the parts whose box reaches its sample row, as [`stroke_field`]
/// asks only the segments in its band.
fn styled_field(parts: &[Part], w: usize, h: usize) -> Vec<f32> {
    use rayon::prelude::*;
    let n = SAMPLES_PER_SIDE;
    let mut field = vec![0.0f32; w * h];
    if parts.is_empty() {
        return field;
    }
    field.par_chunks_mut(w).enumerate().for_each_init(
        || (Vec::new(), vec![0u32; w]),
        |(band, hits): &mut (Vec<&Part>, Vec<u32>), (yi, row)| {
            hits.iter_mut().for_each(|hit| *hit = 0);
            for j in 0..n {
                let sy = yi as f64 + (j as f64 + 0.5) / n as f64;
                band.clear();
                band.extend(parts.iter().filter(|p| p.top <= sy && sy <= p.bottom));
                if band.is_empty() {
                    continue;
                }
                for (xi, hit) in hits.iter_mut().enumerate() {
                    for i in 0..n {
                        let sx = xi as f64 + (i as f64 + 0.5) / n as f64;
                        if band
                            .iter()
                            .any(|p| p.left <= sx && sx <= p.right && p.covers(sx, sy))
                        {
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
                let fast = stroke_field(&path_segments(path, closed), w, h, reach);
                let slow = field(w, h, |x, y| {
                    crate::mask::distance_to_path(path, closed, x, y) <= reach
                });
                assert_eq!(fast, slow, "stroke {reach} of {path:?}, closed {closed}");
            }
        }
    }
}
