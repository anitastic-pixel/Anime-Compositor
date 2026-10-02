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

/// Every name [`Shape::channels`] answers to.
pub const SHAPE_PROPERTIES: [&str; 12] = [
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
];

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
        [Join::Miter, Join::Round, Join::Bevel].into_iter().find(|j| j.as_str() == name)
    }
}

impl Cap {
    pub const NAMES: [&str; 3] = ["butt", "round", "square"];
    pub fn as_str(self) -> &'static str {
        Cap::NAMES[self as usize]
    }
    pub fn named(name: &str) -> Option<Cap> {
        [Cap::Butt, Cap::Round, Cap::Square].into_iter().find(|c| c.as_str() == name)
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
        Trim { start: n(0.0), end: n(100.0), offset: n(0.0) }
    }

    fn at(&self, frame: i32) -> Trim {
        let now = |p: &Property| Property::constant(p.value_at(frame));
        Trim { start: now(&self.start), end: now(&self.end), offset: now(&self.offset) }
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
            if let Some(v) = values.into_iter().flatten().find(|v| ranged && !(0.0..=100.0).contains(v)) {
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
        let runs: Vec<f64> = pieces.iter().map(|(a, b)| (b.0 - a.0).hypot(b.1 - a.1)).collect();
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
                .unwrap_or_else(|| runs.iter().rposition(|&r| r > 0.0).expect("the path has length"));
            let f = (d - along[i]) / runs[i];
            let (p, q) = pieces[i];
            (p.0 + (q.0 - p.0) * f, p.1 + (q.1 - p.1) * f)
        };
        let mut a = s + n(&self.offset) / 360.0;
        a -= a.floor();
        let b = a + (e - s);
        let spans = if b <= 1.0 { vec![(a, b)] } else { vec![(a, 1.0), (0.0, b - 1.0)] };
        let mut out = Vec::new();
        for (u0, u1) in spans {
            let (d0, d1) = (u0 * total, u1 * total);
            let mut line = vec![place(d0)];
            line.extend((1..pieces.len()).filter(|&j| d0 < along[j] && along[j] < d1).map(|j| pieces[j].0));
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
        let mut shape = self.clone();
        if let Some(values) = shape.style_mut(name) {
            return Some(self.tracks.get(name).cloned().unwrap_or_else(|| {
                values.iter().map(|v| Property::constant(Value::Scalar(*v))).collect()
            }));
        }
        shape.property_mut(name).map(|p| vec![p.clone()])
    }

    /// Puts back what [`Shape::channels`] handed out. A style keeps its bases as its plain
    /// numbers and its keys, if it has any left, as its track.
    pub fn set_channels(&mut self, name: &str, channels: Vec<Property>) -> Option<()> {
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
                return Some(format!("a mitre limit from 1 to 100, not {}", s.miter_limit));
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
        (!(width > 0.0 && width <= MAX_STROKE_WIDTH)).then(|| {
            format!("a stroke width above 0 and at most {MAX_STROKE_WIDTH}, not {width}")
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
        let outline = shape.outline();
        // The fill first, then the stroke over this shape's own fill. D-78 fixes that order, and
        // it is why FX-SHP-008's shared columns carry the stroke and not a mixture.
        if let Some(fill) = &shape.fill {
            // An open path is closed for the purpose of filling it, by a straight line from its
            // last point to its first: the even-odd ray already wraps, so nothing is added here.
            let f = crate::mask::scanline_field(&outline, width, height, 0);
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
            let lines: Vec<(Vec<(f64, f64)>, bool)> =
                match shape.trim.as_ref().and_then(|t| t.lines(&outline, shape.closed)) {
                    Some(stretch) => stretch.into_iter().map(|l| (l, false)).collect(),
                    None => vec![(outline.clone(), shape.closed)],
                };
            let r = stroke.width_px / 2.0;
            let f = if stroke.join == Join::Round && stroke.cap == Cap::Round {
                // D-78's coverage, unchanged.
                let segments: Vec<Segment> =
                    lines.iter().flat_map(|(l, closed)| path_segments(l, *closed)).collect();
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
fn stroke_field(segments: &[Segment], w: usize, h: usize, reach: f64) -> Vec<f32> {
    use rayon::prelude::*;
    let n = SAMPLES_PER_SIDE;
    let mut field = vec![0.0f32; w * h];
    if segments.is_empty() {
        return field;
    }
    field.par_chunks_mut(w).enumerate().for_each_init(
        || (Vec::new(), vec![0u32; w]),
        |(band, hits), (yi, row)| {
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

type Segment = ((f64, f64), (f64, f64));

/// The straight pieces of a flattened path in order, a closed one's closing side last. One point
/// is a segment from it to itself, which is what `distance_to_path` measures then.
fn path_segments(path: &[(f64, f64)], closed: bool) -> Vec<Segment> {
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
    Disc { c: (f64, f64), r: f64 },
    Square { c: (f64, f64), r: f64 },
    /// A piece's band: from `a` along (dx, dy), `low <= p.d <= high` and `|p x d| <= reach`.
    Body { a: (f64, f64), dx: f64, dy: f64, low: f64, high: f64, reach: f64 },
    /// A convex polygon, either way round.
    Corner(Vec<(f64, f64)>),
}

impl Part {
    /// The box only passes over samples, never decides one, so it is padded past any rounding.
    fn new(region: Region, points: &[(f64, f64)], pad: f64) -> Part {
        let pad = pad + 1e-6;
        let (left, top, right, bottom) = points.iter().fold(
            (f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
            |(l, t, r, b), p| (l.min(p.0), t.min(p.1), r.max(p.0), b.max(p.1)),
        );
        Part { left: left - pad, top: top - pad, right: right + pad, bottom: bottom + pad, region }
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
            Region::Body { a, dx, dy, low, high, reach } => {
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
    let region = Region::Body { a, dx, dy, low: -before * run, high: run2 + after * run, reach: r * run };
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
        let segs: Vec<Segment> =
            points.windows(2).map(|w| (w[0], w[1])).filter(|(p, q)| p != q).collect();
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
                        if band.iter().any(|p| p.left <= sx && sx <= p.right && p.covers(sx, sy)) {
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
