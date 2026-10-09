//! Reading and writing the project document, per ADR-008 and documents 07 and 26.
//!
//! ADR-008: "A versioned, human-inspectable JSON project document validated against
//! `Schemas/project-v0.schema.json`. Media and caches stay external. Writes go to a temporary
//! sibling file, are flushed and closed, validated where practical, then atomically replaced."
//!
//! # Why this reads JSON values rather than deriving a struct
//!
//! The same ADR says "Unknown additive data is preserved." A derived deserializer drops every
//! field it has no member for, silently, which is exactly the fidelity loss document 28
//! forbids: a project written by a later build, opened here and saved, would come back with
//! its unknown records quietly deleted. So loading keeps the original document as a
//! [`Preserved`] value, and saving writes the model's fields *over* that value rather than
//! from scratch. Anything this build does not model survives the trip untouched.
//!
//! The cost is that the mapping between JSON and the model is written out by hand below. That
//! is the point of the exercise, not an accident of it.
//!
//! # Why the file is written by a serializer of our own
//!
//! ADR-008 again: "Inspectable JSON carries additional weight under this project's
//! verification model: it is one of the few places the owner can check behavior directly by
//! opening the file." So the output is not a library's idea of pretty-printing. Keys come out
//! in the order `Schemas/project-v0.schema.json` lists them, numbers that are whole print
//! without a decimal point, and the layout matches the fixtures under `Fixtures/projects/`
//! exactly — so exactly that loading a fixture and saving it again reproduces the file byte
//! for byte. That equality is a test, and it is the strongest statement available here that
//! nothing was lost on the way through.
//!
//! # Scale
//!
//! The project file stores scale as a percentage: `Fixtures/projects/minimal_project.json` and
//! its siblings write `"scale": { "base": [100, 100] }` for a layer at natural size, and
//! document 21 composes `S(scale/100)`. The model stores the factor, so [`Transform::default`]
//! is `(1, 1)`. The divide and the multiply live here, at the file boundary, and nowhere else.
//! Registered as D-22.
//!
//! [`Transform::default`]: crate::model::Transform::default
//!
//! # Not here
//!
//! Migrations. `schema_version` 0 is the only version that has ever existed, so there is no
//! older form to migrate from and writing a migration framework now would be writing untested
//! machinery for a case that cannot occur. A newer version is refused by name with
//! `PROJECT_SCHEMA_NEWER`, which is the half of document 07's rule that can be honoured today.
//!
//! Persisted undo history. Document 26 is explicit that "undo/redo after project reopen is
//! empty", so [`load`] hands back a [`Document`] with empty stacks, and a test asserts it.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use serde_json::{Map, Value as J};

use crate::command::{Command, Document};
use crate::diagnostics::{Diagnostic, DiagnosticId, Severity};
use crate::media::{self, SequenceAsset};
use crate::model::{
    Asset, AssetKind, BlendMode, Composition, Expression, Id, Interp, Interpretation, Keyframe,
    Layer, LayerKind, MatteMode, MatteReference, Project, Prop, Property, Value,
};
use crate::time::{ExposureMap, ExposureSpan, FrameRate};
use crate::{AlphaMode, ColorSpace};

/// The only schema version that has ever existed.
pub const SCHEMA_VERSION: i64 = 0;

/// Document 07: "retain five rotating snapshots".
pub const AUTOSAVE_SLOTS: usize = 5;

// ---------------------------------------------------------------------------------------
// Preserved data
// ---------------------------------------------------------------------------------------

/// Everything in a loaded project file that this build does not model.
///
/// Opaque on purpose: the only thing a caller may do with it is hand it back to [`to_json`] or
/// [`save`]. Masks, effects, work areas, content fingerprints and any field written by a
/// future version all ride along in here.
#[derive(Clone, Debug, Default)]
pub struct Preserved {
    root: J,
}

impl Preserved {
    /// For a project created in the application rather than loaded from a file.
    pub fn none() -> Self {
        Preserved { root: J::Null }
    }

    fn root_object(&self) -> Option<&Map<String, J>> {
        self.root.as_object()
    }
}

/// What [`load`] produced.
pub struct Loaded {
    pub document: Document,
    pub preserved: Preserved,
    /// Everything the person opening the project should be told, in document 28's shape.
    pub warnings: Vec<Diagnostic>,
}

// ---------------------------------------------------------------------------------------
// Writing JSON
// ---------------------------------------------------------------------------------------

/// Key order, taken from `Schemas/project-v0.schema.json` reading top to bottom.
///
/// One flat list rather than one per object kind: no key means two different things at two
/// depths, so a single ranking is enough and cannot drift out of step with itself.
const KEY_ORDER: &[&str] = &[
    "schema_version",
    "project_id",
    "color_settings",
    "working_space",
    "alpha_mode",
    "assets",
    "compositions",
    "application_metadata",
    "id",
    "kind",
    "name",
    "solid",
    "source_text",
    "color",
    "path",
    "pattern",
    "redistribute",
    "frames",
    "interpretation",
    "color_space",
    "alpha",
    "width",
    "height",
    "pixel_aspect_ratio",
    "frame_rate",
    "numerator",
    "denominator",
    "start_frame",
    "duration_frames",
    "work_area",
    "end_frame_exclusive",
    "camera",
    "layer_order",
    "layers",
    "asset_id",
    "composition_id",
    "enabled",
    "locked",
    "in_frame",
    "out_frame",
    "source_offset_frames",
    "gain_db",
    "transform",
    "anchor",
    "position",
    "scale",
    "rotation",
    "opacity",
    "base",
    "keyframes",
    "frame",
    "value",
    "interp",
    "exposure_spans",
    "drawing_number",
    "mask",
    "masks",
    "inverted",
    "vertices",
    "points",
    "point",
    "in",
    "out",
    "path",
    "opacity",
    "feather_px",
    "expansion_px",
    "matte",
    "layer_id",
    "mode",
    "blend_mode",
    "effects",
    "parent",
    "depth",
    "timesheet",
    "sheet",
    "column",
    "track",
    "key_drawings",
    "sheet_text",
    "entries",
    "text",
    "sheet_details",
    "episode",
    "scene",
    "cut",
    "animator",
    "instance_id",
    "type_id",
    "parameters",
    "zoom",
    "expression",
    // D-188: a composition's shutter, after its switch, which is `enabled` above.
    "motion_blur",
    "shutter_angle",
    "shutter_phase",
    "samples",
    // D-216.
    "time_stretch",
    "frame_blend",
    "drawing_dissolve",
    "frame_blending",
    "background_color",
    // D-319.
    "float_depth",
    "eight_bpc",
    // D-333.
    "ae_32bpc",
    // D-323.
    "time_remap",
];

/// An effect record is the one place a flat list is not enough: it spells `enabled` after
/// `type_id`, while a layer spells it near the top, so the two cannot share a ranking. Effect
/// records are recognised by `instance_id`, which nothing else in the schema has.
/// D-202: `mix` after `enabled`.
const EFFECT_KEY_ORDER: &[&str] = &["instance_id", "type_id", "enabled", "mix", "parameters"];

/// D-59's expression record is the other: `text`, then `enabled`. Recognised by `text`, which
/// nothing else in the schema has.
const EXPRESSION_KEY_ORDER: &[&str] = &["text", "enabled"];

fn order_for(map: &Map<String, J>) -> &'static [&'static str] {
    if map.contains_key("instance_id") {
        EFFECT_KEY_ORDER
    } else if map.contains_key("text") && !map.contains_key("start_frame") {
        // A D-84c sheet text entry has `text` too, and is ordered as the schema lists it.
        EXPRESSION_KEY_ORDER
    } else {
        KEY_ORDER
    }
}

fn key_rank(order: &[&str], key: &str) -> usize {
    order.iter().position(|k| *k == key).unwrap_or(order.len())
}

/// Sort keys the schema names into schema order, and everything else after them.
///
/// Unknown keys fall back to numeric order when they are all numbers, which is what an asset's
/// `frames` map is: `"2"` must come before `"10"`, and asciibetical order would not.
fn sorted_keys(map: &Map<String, J>) -> Vec<&String> {
    let order = order_for(map);
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort_by(|a, b| {
        let (ra, rb) = (key_rank(order, a), key_rank(order, b));
        if ra != rb {
            return ra.cmp(&rb);
        }
        match (a.parse::<u64>(), b.parse::<u64>()) {
            (Ok(x), Ok(y)) => x.cmp(&y),
            _ => a.cmp(b),
        }
    });
    keys
}

/// JSON has one number type, so `1.0` and `1` are the same value; the shorter form keeps a
/// diff about what actually changed.
fn number(v: &serde_json::Number) -> String {
    if let Some(i) = v.as_i64() {
        return i.to_string();
    }
    match v.as_f64() {
        Some(f) => float(f),
        None => v.to_string(),
    }
}

fn float(f: f64) -> String {
    if f.fract() == 0.0 && f.abs() < 1e15 {
        format!("{}", f as i64)
    } else {
        format!("{f}")
    }
}

fn quote(text: &str) -> String {
    let mut s = String::with_capacity(text.len() + 2);
    s.push('"');
    for c in text.chars() {
        match c {
            '"' => s.push_str("\\\""),
            '\\' => s.push_str("\\\\"),
            '\n' => s.push_str("\\n"),
            '\t' => s.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(s, "\\u{:04x}", c as u32);
            }
            c => s.push(c),
        }
    }
    s.push('"');
    s
}

fn write_value(out: &mut String, value: &J, indent: usize) {
    let pad = "  ".repeat(indent);
    let inner = "  ".repeat(indent + 1);
    match value {
        J::Null => out.push_str("null"),
        J::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        J::Number(n) => out.push_str(&number(n)),
        J::String(s) => out.push_str(&quote(s)),
        J::Array(items) => {
            if items.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push_str("[\n");
            for (i, item) in items.iter().enumerate() {
                out.push_str(&inner);
                write_value(out, item, indent + 1);
                out.push_str(if i + 1 == items.len() { "\n" } else { ",\n" });
            }
            out.push_str(&pad);
            out.push(']');
        }
        J::Object(map) => {
            if map.is_empty() {
                out.push_str("{}");
                return;
            }
            let keys = sorted_keys(map);
            out.push_str("{\n");
            for (i, key) in keys.iter().enumerate() {
                out.push_str(&inner);
                out.push_str(&quote(key));
                out.push_str(": ");
                write_value(out, &map[key.as_str()], indent + 1);
                out.push_str(if i + 1 == keys.len() { "\n" } else { ",\n" });
            }
            out.push_str(&pad);
            out.push('}');
        }
    }
}

// ---------------------------------------------------------------------------------------
// Model to JSON
// ---------------------------------------------------------------------------------------

fn merge(base: Option<&J>, owned: Vec<(&str, J)>) -> J {
    let mut map = base
        .and_then(J::as_object)
        .cloned()
        .unwrap_or_else(Map::new);
    for (key, value) in owned {
        map.insert(key.to_string(), value);
    }
    J::Object(map)
}

/// The entry of `array_key` in `base` whose `"id"` is `id`, if there is one.
fn by_id<'a>(base: Option<&'a J>, array_key: &str, id: &str) -> Option<&'a J> {
    base?
        .get(array_key)?
        .as_array()?
        .iter()
        .find(|entry| entry.get("id").and_then(J::as_str) == Some(id))
}

fn num(v: f64) -> J {
    // Whole values become JSON integers so the file reads `100` rather than `100.0`. A
    // non-finite value cannot reach here: the command layer rejects one, and loading rejects
    // one, so there is no path that puts a null in a number's place.
    if v.fract() == 0.0 && v.abs() < 1e15 {
        J::from(v as i64)
    } else {
        J::from(v)
    }
}

fn value_json(value: Value, factor: f64) -> J {
    match value {
        Value::Scalar(n) => num(n * factor),
        Value::Vec2(x, y) => J::Array(vec![num(x * factor), num(y * factor)]),
    }
}

/// D-168's gradient record, its two points written as any property is and merged over the
/// file's own, as a transform property is.
fn gradient_json(was: Option<&J>, g: &crate::shape::Gradient) -> J {
    let stops = g
        .stops
        .iter()
        .map(|s| {
            let mut m = Map::new();
            m.insert("offset".into(), num(s.offset));
            m.insert("color".into(), J::from(s.color.to_vec()));
            m.insert("opacity".into(), num(s.opacity));
            J::Object(m)
        })
        .collect();
    let point = |key: &str, p: &Property| property_json(was.and_then(|w| w.get(key)), p, 1.0);
    let mut m = Map::new();
    m.insert("type".into(), J::from(g.kind.as_str()));
    m.insert("start".into(), point("start", &g.start));
    m.insert("end".into(), point("end", &g.end));
    m.insert("stops".into(), J::Array(stops));
    J::Object(m)
}

fn property_json(base: Option<&J>, property: &Property, factor: f64) -> J {
    // D-69: a separated position is written as its X and its Y and nothing else.
    // ponytail: unknown fields inside x and y are not carried; nothing writes any today.
    if let Some((x, y)) = property.split() {
        let mut m = Map::new();
        m.insert("x".into(), property_json(None, x, factor));
        m.insert("y".into(), property_json(None, y, factor));
        return J::Object(m);
    }
    let keyframes: Vec<J> = property
        .keyframes()
        .iter()
        .map(|k| {
            let mut m = Map::new();
            m.insert("frame".into(), J::from(k.frame));
            m.insert("value".into(), value_json(k.value, factor));
            m.insert("interp".into(), J::from(k.interp.as_str()));
            // Document 19: the four numbers go with `ease` and with nothing else, in this order.
            if let Interp::Ease { x1, y1, x2, y2 } = k.interp {
                m.insert(
                    "ease".into(),
                    J::Array(vec![J::from(x1), J::from(y1), J::from(x2), J::from(y2)]),
                );
            }
            // D-53: the four handle offsets go with `spatial`, in this order, on position only.
            if let Some(handles) = k.spatial {
                m.insert(
                    "spatial".into(),
                    J::Array(handles.iter().map(|n| num(*n)).collect()),
                );
            }
            // D-69: a plain key writes neither, so a file from before D-69 saves as it was.
            if k.kind != crate::model::Kind::Bezier {
                m.insert("kind".into(), J::from(k.kind.as_str()));
            }
            if k.roving {
                m.insert("roving".into(), J::from(true));
            }
            J::Object(m)
        })
        .collect();
    let mut out = merge(
        base,
        vec![
            ("base", value_json(property.base(), factor)),
            ("keyframes", J::Array(keyframes)),
        ],
    );
    // D-59: the text and its switch, or no field at all when the property has no expression.
    let map = out.as_object_mut().expect("merge makes an object");
    match property.expression() {
        Some(e) => {
            let mut m = Map::new();
            m.insert("text".into(), J::from(e.text.as_str()));
            m.insert("enabled".into(), J::from(e.enabled));
            map.insert("expression".into(), J::Object(m));
        }
        None => {
            map.remove("expression");
        }
    }
    out
}

fn transform_json(base: Option<&J>, layer: &Layer) -> J {
    let t = &layer.transform;
    // Document 21 writes `S(scale/100)`, so the file carries the percentage and the model the
    // factor. D-22.
    merge(
        base,
        vec![
            (
                "anchor",
                property_json(base.and_then(|b| b.get("anchor")), &t.anchor, 1.0),
            ),
            (
                "position",
                property_json(base.and_then(|b| b.get("position")), &t.position, 1.0),
            ),
            (
                "scale",
                property_json(base.and_then(|b| b.get("scale")), &t.scale, 100.0),
            ),
            (
                "rotation",
                property_json(base.and_then(|b| b.get("rotation")), &t.rotation, 1.0),
            ),
            (
                "opacity",
                property_json(base.and_then(|b| b.get("opacity")), &t.opacity, 1.0),
            ),
        ],
    )
}

fn interpretation_json(base: Option<&J>, interpretation: Interpretation) -> J {
    merge(
        base,
        vec![
            (
                "color_space",
                J::from(match interpretation.color_space {
                    ColorSpace::Srgb => "srgb",
                    ColorSpace::LinearLight => "linear-srgb",
                }),
            ),
            (
                "alpha",
                J::from(match interpretation.alpha {
                    AlphaMode::Straight => "straight",
                    AlphaMode::Premultiplied => "premultiplied",
                }),
            ),
        ],
    )
}

fn asset_json(base: Option<&J>, asset: &Asset) -> J {
    let mut owned = vec![
        ("id", J::from(asset.id.as_str())),
        ("kind", J::from(asset.kind.as_str())),
        ("name", J::from(asset.name.as_str())),
        (
            "interpretation",
            interpretation_json(
                base.and_then(|b| b.get("interpretation")),
                asset.interpretation,
            ),
        ),
    ];
    // D-182: nor does a lookup file.
    if matches!(asset.kind, AssetKind::Audio | AssetKind::Lut) {
        owned.retain(|(key, _)| *key != "interpretation");
    }
    match &asset.path {
        Some(p) => owned.push(("path", J::from(p.as_str()))),
        None => owned.push(("path", J::Null)),
    }
    match &asset.pattern {
        Some(p) => owned.push(("pattern", J::from(p.as_str()))),
        None => owned.push(("pattern", J::Null)),
    }
    // D-61: written only when false, so every project saved before it is unchanged.
    owned.push((
        "redistribute",
        if asset.redistribute {
            J::Null
        } else {
            J::from(false)
        },
    ));
    if !asset.frames.is_empty() {
        let mut frames = Map::new();
        for (number, file) in &asset.frames {
            frames.insert(number.to_string(), J::from(file.as_str()));
        }
        owned.push(("frames", J::Object(frames)));
    }
    // D-254: as a layer's label.
    if asset.label != 0 || base.is_some_and(|b| b.get("label").is_some()) {
        owned.push(("label", J::from(asset.label)));
    }
    let mut json = merge(base, owned);
    // The schema makes `path`, `pattern` and `frames` optional rather than nullable, so an
    // absent one is written as absent rather than as null.
    if let Some(map) = json.as_object_mut() {
        map.retain(|_, v| !v.is_null());
    }
    json
}

fn layer_json(base: Option<&J>, layer: &Layer) -> J {
    // D-71: an audio layer is written as the few things it has.
    if layer.kind == LayerKind::Audio {
        let mut owned = vec![
            ("id", J::from(layer.id.as_str())),
            ("kind", J::from(layer.kind.as_str())),
            ("name", J::from(layer.name.as_str())),
            ("asset_id", J::from(layer.asset_id.as_str())),
            ("enabled", J::from(layer.enabled)),
            ("locked", J::from(layer.locked)),
            ("in_frame", J::from(layer.in_frame)),
            ("out_frame", J::from(layer.out_frame)),
            ("source_offset_frames", J::from(layer.source_offset_frames)),
            ("gain_db", J::from(layer.gain_db)),
        ];
        if layer.label != 0 || base.is_some_and(|b| b.get("label").is_some()) {
            owned.push(("label", J::from(layer.label)));
        }
        if layer.shy || base.is_some_and(|b| b.get("shy").is_some()) {
            owned.push(("shy", J::from(layer.shy)));
        }
        return merge(base, owned);
    }
    let spans: Vec<J> = layer
        .exposure_spans
        .iter()
        .map(|s| {
            let mut m = Map::new();
            m.insert("start_frame".into(), J::from(s.start_frame));
            m.insert("end_frame_exclusive".into(), J::from(s.end_frame_exclusive));
            m.insert("drawing_number".into(), J::from(s.drawing_number));
            J::Object(m)
        })
        .collect();
    let matte = match &layer.matte {
        Some(m) => {
            let mut map = base
                .and_then(|b| b.get("matte"))
                .and_then(J::as_object)
                .cloned()
                .unwrap_or_else(Map::new);
            map.insert("layer_id".into(), J::from(m.layer_id.as_str()));
            map.insert("mode".into(), J::from(m.mode.as_str()));
            map.insert("matte_only".into(), J::from(m.matte_only));
            J::Object(map)
        }
        None => J::Null,
    };
    // Each mask merges over whatever the file held at the same place in the list, the way the
    // matte does, so a key this build does not know about inside a mask record survives the
    // round trip. B-24d writes a path's `keyframes` from the model rather than inheriting them,
    // because from B-24d on the build moves them. D-77: the `masks` list is written and the old
    // single `mask` key never is, so a file saved by this build and opened by one older than it
    // has no mask rather than a disagreeing pair.
    let points_json = |points: &[crate::mask::MaskPoint]| {
        J::Array(
            points
                .iter()
                .map(|p| {
                    let pair = |(x, y): (f64, f64)| J::Array(vec![J::from(x), J::from(y)]);
                    let mut point = Map::new();
                    point.insert("point".into(), pair(p.point));
                    point.insert("in".into(), pair(p.in_handle));
                    point.insert("out".into(), pair(p.out_handle));
                    J::Object(point)
                })
                .collect(),
        )
    };
    // D-77's `path` record, written from the model and merged over whatever the file held at the
    // same place. One writer for a mask's path and a shape's path, because D-78 makes them one
    // record.
    let path_json = |was: Option<&Map<String, J>>,
                     points: &[crate::mask::MaskPoint],
                     keys: &[crate::mask::MaskKey]| {
        let mut path = was
            .and_then(|w| w.get("path"))
            .and_then(J::as_object)
            .cloned()
            .unwrap_or_else(Map::new);
        let mut path_base = path
            .get("base")
            .and_then(J::as_object)
            .cloned()
            .unwrap_or_else(Map::new);
        path_base.insert("points".into(), points_json(points));
        path.insert("base".into(), J::Object(path_base));
        path.insert(
            "keyframes".into(),
            J::Array(
                keys.iter()
                    .map(|k| {
                        let mut key = Map::new();
                        key.insert("frame".into(), J::from(k.frame));
                        let mut value = Map::new();
                        value.insert("points".into(), points_json(&k.points));
                        key.insert("value".into(), J::Object(value));
                        key.insert("interp".into(), J::from(k.interp.as_str()));
                        // Document 19: the four numbers go with `ease` and nothing else.
                        if let Interp::Ease { x1, y1, x2, y2 } = k.interp {
                            key.insert(
                                "ease".into(),
                                J::Array(vec![
                                    J::from(x1),
                                    J::from(y1),
                                    J::from(x2),
                                    J::from(y2),
                                ]),
                            );
                        }
                        J::Object(key)
                    })
                    .collect(),
            ),
        );
        J::Object(path)
    };
    let base_masks = base
        .and_then(|b| b.get("masks"))
        .and_then(J::as_array)
        .map(|a| a.as_slice())
        .unwrap_or(&[]);
    let masks: Vec<J> = layer
        .masks
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let was = base_masks.get(i).and_then(J::as_object);
            let mut map = was.cloned().unwrap_or_else(Map::new);
            map.insert("name".into(), J::from(m.name.as_str()));
            map.insert("enabled".into(), J::from(m.enabled));
            map.insert("inverted".into(), J::from(m.inverted));
            map.insert("mode".into(), J::from(m.mode.as_str()));
            // D-298: a keyed number is its record, a still one its plain value.
            let number = |name: &str, plain: f64| match m.tracks.get(name) {
                Some(track) => track_json(std::slice::from_ref(track), J::from(plain)),
                None => J::from(plain),
            };
            map.insert("opacity".into(), number("opacity", m.opacity));
            map.insert("feather_px".into(), number("feather", m.feather_px));
            map.insert("expansion_px".into(), number("expansion", m.expansion_px));
            map.insert("path".into(), path_json(was, &m.points, &m.keys));
            J::Object(map)
        })
        .collect();
    let mut owned = vec![
        ("id", J::from(layer.id.as_str())),
        ("kind", J::from(layer.kind.as_str())),
        ("name", J::from(layer.name.as_str())),
        ("asset_id", J::from(layer.asset_id.as_str())),
        (
            "composition_id",
            J::from(layer.composition_id.as_ref().map_or("", |c| c.as_str())),
        ),
        ("enabled", J::from(layer.enabled)),
        ("locked", J::from(layer.locked)),
        ("in_frame", J::from(layer.in_frame)),
        ("out_frame", J::from(layer.out_frame)),
        ("source_offset_frames", J::from(layer.source_offset_frames)),
        (
            "transform",
            transform_json(base.and_then(|b| b.get("transform")), layer),
        ),
        ("exposure_spans", J::Array(spans)),
        ("matte", matte),
        ("blend_mode", J::from(layer.blend_mode.as_str())),
    ];
    // D-77: the list is written where the file already had one, or where there is a mask to
    // write; a layer with no mask in a file that never had the key keeps it that way. The old
    // single key, if the file held one, is written back as null: `merge` keeps what the file
    // said for a key this build does not write, and a file holding both records refuses to open.
    if !masks.is_empty() || base.is_some_and(|b| b.get("masks").is_some()) {
        owned.push(("masks", J::Array(masks)));
    }
    if base.is_some_and(|b| b.get("mask").is_some_and(|m| !m.is_null())) {
        owned.push(("mask", J::Null));
    }
    // D-78: a shape layer's drawing is its `shapes` list, written from the model and merged over
    // what the file held at the same place, as the masks are.
    if layer.kind == LayerKind::Shape {
        let base_shapes = base
            .and_then(|b| b.get("shapes"))
            .and_then(J::as_array)
            .map(|a| a.as_slice())
            .unwrap_or(&[]);
        let shapes: Vec<J> = layer
            .shapes
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let was = base_shapes.get(i).and_then(J::as_object);
                let mut map = was.cloned().unwrap_or_else(Map::new);
                map.insert("name".into(), J::from(s.name.as_str()));
                map.insert("enabled".into(), J::from(s.enabled));
                map.insert("closed".into(), J::from(s.closed));
                map.insert("path".into(), path_json(was, &s.points, &s.keys));
                // Absent and null both mean "no fill" on the way in, and null is written on the
                // way out so that a file that had one and lost it says so rather than keeping it.
                let was_paint = |key: &str| {
                    was.and_then(|w| w.get(key))
                        .and_then(|p| p.get("gradient"))
                };
                // D-170: a keyed style is its record, a still one its plain value.
                let style = |name: &str, plain: J| match s.tracks.get(name) {
                    Some(track) => track_json(track, plain),
                    None => plain,
                };
                map.insert(
                    "fill".into(),
                    match &s.fill {
                        None => J::Null,
                        Some(f) => {
                            let mut m = Map::new();
                            m.insert("color".into(), style("fill_color", J::from(f.color.to_vec())));
                            m.insert("opacity".into(), style("fill_opacity", J::from(f.opacity)));
                            if let Some(g) = &f.gradient {
                                m.insert("gradient".into(), gradient_json(was_paint("fill"), g));
                            }
                            J::Object(m)
                        }
                    },
                );
                map.insert(
                    "stroke".into(),
                    match &s.stroke {
                        None => J::Null,
                        Some(t) => {
                            let mut m = Map::new();
                            m.insert("color".into(), style("stroke_color", J::from(t.color.to_vec())));
                            m.insert("opacity".into(), style("stroke_opacity", J::from(t.opacity)));
                            m.insert("width_px".into(), style("stroke_width", J::from(t.width_px)));
                            if let Some(g) = &t.gradient {
                                m.insert("gradient".into(), gradient_json(was_paint("stroke"), g));
                            }
                            // D-170: written only where they are not D-78's round and round.
                            if t.join != crate::shape::Join::Round {
                                m.insert("join".into(), J::from(t.join.as_str()));
                            }
                            if t.miter_limit != 4.0 {
                                m.insert("miter_limit".into(), J::from(t.miter_limit));
                            }
                            if t.cap != crate::shape::Cap::Round {
                                m.insert("cap".into(), J::from(t.cap.as_str()));
                            }
                            J::Object(m)
                        }
                    },
                );
                match &s.trim {
                    None => {
                        map.remove("trim");
                    }
                    Some(t) => {
                        let was = was.and_then(|w| w.get("trim"));
                        let mut m = Map::new();
                        for (key, p) in [("start", &t.start), ("end", &t.end), ("offset", &t.offset)] {
                            m.insert(key.into(), property_json(was.and_then(|w| w.get(key)), p, 1.0));
                        }
                        map.insert("trim".into(), J::Object(m));
                    }
                }
                J::Object(map)
            })
            .collect();
        owned.push(("shapes", J::Array(shapes)));
    }
    // D-66: an adjustment layer has no drawing, so the three keys about one are not written.
    // D-74: nor are they for a solid, whose drawing is its `solid` record.
    // D-78: nor for a shape layer, whose drawing is its `shapes` list.
    // D-82: nor for a null, which has none at all.
    // D-263: nor for a text layer, whose drawing is its `source_text` record.
    if layer.is_adjustment()
        || layer.solid.is_some()
        || matches!(layer.kind, LayerKind::Shape | LayerKind::Null | LayerKind::Text)
    {
        owned.retain(|(key, _)| {
            !matches!(*key, "asset_id" | "source_offset_frames" | "exposure_spans")
        });
    }
    // D-67: a composition layer names a composition where a drawn layer names an asset, and
    // keeps its source offset, which is where in the inner composition it starts.
    if layer.kind == LayerKind::Composition {
        owned.retain(|(key, _)| !matches!(*key, "asset_id" | "exposure_spans"));
    } else {
        owned.retain(|(key, _)| *key != "composition_id");
    }
    let effects: Vec<J> = layer
        .effects
        .iter()
        .map(|e| effect_json(effect_base(base, e.instance_id.as_str()), e))
        .collect();
    if let Some(s) = &layer.solid {
        let mut map = base
            .and_then(|b| b.get("solid"))
            .and_then(J::as_object)
            .cloned()
            .unwrap_or_default();
        map.insert("color".into(), J::from(s.color.to_vec()));
        map.insert("width".into(), J::from(s.width));
        map.insert("height".into(), J::from(s.height));
        owned.push(("solid", J::Object(map)));
    }
    // D-263: merged over what the file held, so a field no build writes yet is kept.
    if let Some(t) = &layer.text {
        let mut map = base
            .and_then(|b| b.get("source_text"))
            .and_then(J::as_object)
            .cloned()
            .unwrap_or_default();
        map.insert("text".into(), J::from(t.text.as_str()));
        map.insert("font".into(), J::from(t.font.as_str()));
        map.insert("size".into(), J::from(t.size));
        map.insert("color".into(), J::from(t.color.to_vec()));
        map.insert("at".into(), J::from(t.at.to_vec()));
        map.insert("align".into(), J::from(t.align.as_str()));
        // D-264: each written only when it is not its default, and taken out when it returns to
        // it, so a D-263 record is written as it was.
        let mut put = |key: &str, value: Option<J>| match value {
            Some(v) => {
                map.insert(key.into(), v);
            }
            None => {
                map.remove(key);
            }
        };
        put("tracking", (t.tracking != 0.0).then(|| J::from(t.tracking)));
        put("leading", (t.leading != 0.0).then(|| J::from(t.leading)));
        put("kerning", t.kerning.then(|| J::from(true)));
        put("all_caps", t.all_caps.then(|| J::from(true)));
        put("faux_bold", t.faux_bold.then(|| J::from(true)));
        put("faux_italic", t.faux_italic.then(|| J::from(true)));
        put("box_width", (t.box_width != 0.0).then(|| J::from(t.box_width)));
        // A record is merged over the file's own, so a line inside it no build writes is kept.
        let old = |key: &str| base.and_then(|b| b.get("source_text")).and_then(|s| s.get(key)).and_then(J::as_object).cloned().unwrap_or_default();
        let merged = |key: &str, pairs: Vec<(&str, J)>| {
            let mut m = old(key);
            for (k, v) in pairs {
                m.insert(k.into(), v);
            }
            J::Object(m)
        };
        put("stroke", t.stroke.as_ref().map(|s| merged("stroke", vec![("color", J::from(s.color.to_vec())), ("width", J::from(s.width))])));
        put(
            "background",
            t.background.as_ref().map(|b| {
                merged(
                    "background",
                    vec![
                        ("color", J::from(b.color.to_vec())),
                        ("opacity", J::from(b.opacity)),
                        ("padding", J::from(b.padding)),
                        ("roundness", J::from(b.roundness)),
                    ],
                )
            }),
        );
        put(
            "shadow",
            t.shadow.as_ref().map(|s| {
                merged(
                    "shadow",
                    vec![
                        ("angle", J::from(s.angle)),
                        ("color", J::from(s.color.to_vec())),
                        ("distance", J::from(s.distance)),
                        ("opacity", J::from(s.opacity)),
                        ("softness", J::from(s.softness)),
                    ],
                )
            }),
        );
        owned.push(("source_text", J::Object(map)));
    }
    owned.push(("effects", J::Array(effects)));
    // D-57: written only when the layer has a parent, so a project that never had one is
    // written back without the key. A parent that was cleared writes null, because that is what
    // overrides the record the file held; leaving the pair out would merge the old one back in.
    match &layer.parent {
        Some(parent) => owned.push(("parent", J::from(parent.as_str()))),
        None if base.is_some_and(|b| b.get("parent").is_some()) => owned.push(("parent", J::Null)),
        None => {}
    }
    // D-58's depth, written the same way and for the same reason.
    match &layer.depth {
        Some(depth) => owned.push((
            "depth",
            property_json(base.and_then(|b| b.get("depth")), depth, 1.0),
        )),
        None if base.is_some_and(|b| b.get("depth").is_some()) => owned.push(("depth", J::Null)),
        None => {}
    }
    if layer.label != 0 || base.is_some_and(|b| b.get("label").is_some()) {
        owned.push(("label", J::from(layer.label)));
    }
    if layer.shy || base.is_some_and(|b| b.get("shy").is_some()) {
        owned.push(("shy", J::from(layer.shy)));
    }
    // D-84's record, written the way `parent` is.
    match &layer.timesheet {
        Some(t) => owned.push((
            "timesheet",
            serde_json::json!({"sheet": t.sheet, "column": t.column, "track": t.track}),
        )),
        None if base.is_some_and(|b| b.get("timesheet").is_some()) => {
            owned.push(("timesheet", J::Null))
        }
        None => {}
    }
    // D-84g: written only when there are some.
    if !layer.key_drawings.is_empty() {
        owned.push(("key_drawings", J::from(layer.key_drawings.clone())));
    }
    // D-188: written only when on.
    if layer.motion_blur {
        owned.push(("motion_blur", J::from(true)));
    }
    // D-216: each written only when it is not its default (FX-FBLEND-014, 024).
    if layer.time_stretch != 100.0 {
        owned.push(("time_stretch", num(layer.time_stretch)));
    }
    if layer.frame_blend {
        owned.push(("frame_blend", J::from("frame_mix")));
    }
    if layer.drawing_dissolve != 0 {
        owned.push(("drawing_dissolve", J::from(layer.drawing_dissolve)));
    }
    // D-323: written only when on, the way depth is (FX-TREMAP-001).
    match &layer.time_remap {
        Some(remap) => owned.push((
            "time_remap",
            property_json(base.and_then(|b| b.get("time_remap")), remap, 1.0),
        )),
        None if base.is_some_and(|b| b.get("time_remap").is_some()) => {
            owned.push(("time_remap", J::Null))
        }
        None => {}
    }
    let mut merged = merge(base, owned);
    if let Some(map) = merged.as_object_mut() {
        if layer.key_drawings.is_empty() {
            map.remove("key_drawings");
        }
        if !layer.motion_blur {
            map.remove("motion_blur");
        }
        if layer.time_stretch == 100.0 {
            map.remove("time_stretch");
        }
        if !layer.frame_blend {
            map.remove("frame_blend");
        }
        if layer.drawing_dissolve == 0 {
            map.remove("drawing_dissolve");
        }
        if layer.time_remap.is_none() {
            map.remove("time_remap");
        }
    }
    merged
}

/// The effect record this instance was read from, so that keys this build does not know about
/// -- the fixture's `opaque_unknown_data`, a `contract_version` a later schema adds -- survive a
/// round trip. Matched on `instance_id`, which document 07 requires be stable and which is why
/// reordering a stack does not lose anything.
fn effect_base<'a>(base: Option<&'a J>, instance_id: &str) -> Option<&'a J> {
    base?
        .get("effects")?
        .as_array()?
        .iter()
        .find(|e| e.get("instance_id").and_then(J::as_str) == Some(instance_id))
}

/// One effect instance as JSON.
///
/// An unsupported effect writes back only its identity and enabled flag; its parameters come
/// through the merge untouched, because this build does not know what they mean and rewriting
/// them from a model it never parsed them into is how a record gets quietly damaged.
fn effect_json(base: Option<&J>, instance: &crate::effects::EffectInstance) -> J {
    use crate::effects::Effect;
    let mut owned = vec![
        ("instance_id", J::from(instance.instance_id.as_str())),
        ("type_id", J::from(instance.type_id())),
        ("enabled", J::from(instance.enabled)),
    ];
    // D-202: the Mix only when it is not 100 or has keys; an unknown effect's stays as written.
    let unknown = matches!(instance.effect, Effect::Unsupported { .. });
    let plain_mix = !unknown && instance.mix == 100.0 && !instance.tracks.contains_key("mix");
    if !unknown && !plain_mix {
        owned.push(("mix", match instance.tracks.get("mix") {
            Some(track) => track_json(track, num(instance.mix)),
            None => num(instance.mix),
        }));
    }
    let mut params = base
        .and_then(|b| b.get("parameters"))
        .and_then(J::as_object)
        .cloned()
        .unwrap_or_else(Map::new);
    match &instance.effect {
        Effect::Exposure { stops, offset, gamma, bypass } => {
            params.insert("stops".into(), num(*stops));
            // D-335: each written only when not its default or the file had it.
            for (key, value, absent) in [("offset", *offset, 0.0), ("gamma", *gamma, 1.0)] {
                if value != absent || instance.tracks.contains_key(key) || params.contains_key(key) {
                    params.insert(key.into(), num(value));
                }
            }
            if bypass != "off" || params.contains_key("bypass") {
                params.insert("bypass".into(), J::from(bypass.as_str()));
            }
        }
        Effect::GaussianBlur { sigma_px, edges, dimensions, units } => {
            params.insert("sigma_px".into(), num(*sigma_px));
            put_edges(&mut params, edges);
            // D-303: written only when not both or when the file had it.
            if dimensions != "both" || params.contains_key("dimensions") {
                params.insert("dimensions".into(), J::from(dimensions.as_str()));
            }
            // D-321: likewise, sigma being what a file without it means.
            if units != "sigma" || params.contains_key("units") {
                params.insert("units".into(), J::from(units.as_str()));
            }
        }
        Effect::Tint { color, amount } => {
            params.insert(
                "color".into(),
                J::Array(color.iter().map(|c| num(*c)).collect()),
            );
            params.insert("amount".into(), num(*amount));
        }
        Effect::LineSmooth {
            softness,
            threshold,
        } => {
            params.insert("softness".into(), num(*softness));
            params.insert("threshold".into(), num(*threshold));
        }
        Effect::SelectiveColorBlur {
            blur,
            colors,
            tolerance,
        } => {
            params.insert("blur".into(), num(*blur));
            params.insert(
                "colors".into(),
                J::Array(colors.iter().map(|c| J::from(c.as_str())).collect()),
            );
            // D-88: a tolerance of 0 is not written, so a file from before it saves the same.
            if *tolerance != 0.0 || instance.tracks.contains_key("tolerance") {
                params.insert("tolerance".into(), num(*tolerance));
            } else {
                params.remove("tolerance");
            }
        }
        Effect::Glow {
            based_on,
            threshold,
            colors,
            tolerance,
            radius,
            intensity,
            operation,
            tint,
            units,
        } => {
            params.insert("based_on".into(), J::from(based_on.as_str()));
            params.insert("threshold".into(), num(*threshold));
            params.insert(
                "colors".into(),
                J::Array(colors.iter().map(|c| J::from(c.as_str())).collect()),
            );
            params.insert("tolerance".into(), num(*tolerance));
            params.insert("radius".into(), num(*radius));
            params.insert("intensity".into(), num(*intensity));
            params.insert("operation".into(), J::from(operation.as_str()));
            params.insert("tint".into(), J::from(tint.as_str()));
            // D-322: written only when not classic, what a file without it means, or when the
            // file had it.
            if units != "classic" || params.contains_key("units") {
                params.insert("units".into(), J::from(units.as_str()));
            }
            // D-353: a Glow that was physical and is now classic says so.
            if params.contains_key("falloff") {
                params.insert("falloff".into(), J::from("classic"));
            }
        }
        Effect::LineRecolor {
            colors,
            tolerance,
            new_color,
        } => {
            params.insert(
                "colors".into(),
                J::Array(colors.iter().map(|c| J::from(c.as_str())).collect()),
            );
            params.insert("tolerance".into(), num(*tolerance));
            params.insert("new_color".into(), J::from(new_color.as_str()));
        }
        Effect::DirectionalBlur {
            direction,
            length,
            edges,
        } => {
            params.insert("direction".into(), num(*direction));
            params.insert("length".into(), num(*length));
            put_edges(&mut params, edges);
        }
        Effect::SelectColor {
            colors,
            tolerance,
            keep,
        } => {
            params.insert(
                "colors".into(),
                J::Array(colors.iter().map(|c| J::from(c.as_str())).collect()),
            );
            params.insert("tolerance".into(), num(*tolerance));
            params.insert("keep".into(), J::from(keep.as_str()));
        }
        Effect::LineWidth {
            width,
            based_on,
            colors,
            tolerance,
        } => {
            params.insert("width".into(), num(*width));
            params.insert("based_on".into(), J::from(based_on.as_str()));
            params.insert(
                "colors".into(),
                J::Array(colors.iter().map(|c| J::from(c.as_str())).collect()),
            );
            params.insert("tolerance".into(), num(*tolerance));
        }
        Effect::RadialBlur {
            kind,
            amount,
            center,
            edges,
        } => {
            params.insert("type".into(), J::from(kind.as_str()));
            params.insert("amount".into(), num(*amount));
            params.insert(
                "center".into(),
                J::Array(center.iter().map(|c| num(*c)).collect()),
            );
            put_edges(&mut params, edges);
        }
        Effect::Bloom {
            threshold,
            radius,
            intensity,
            streaks,
            length,
            angle,
        } => {
            params.insert("threshold".into(), num(*threshold));
            params.insert("radius".into(), num(*radius));
            params.insert("intensity".into(), num(*intensity));
            params.insert("streaks".into(), J::from(streaks.as_str()));
            params.insert("length".into(), num(*length));
            params.insert("angle".into(), num(*angle));
        }
        Effect::ColorKey {
            colors,
            tolerance,
            softness,
            match_by,
        } => {
            params.insert(
                "colors".into(),
                J::Array(colors.iter().map(|c| J::from(c.as_str())).collect()),
            );
            params.insert("tolerance".into(), num(*tolerance));
            params.insert("softness".into(), num(*softness));
            params.insert("match".into(), J::from(match_by.as_str()));
        }
        Effect::Curves {
            master,
            red,
            green,
            blue,
            alpha,
        } => {
            // D-302: alpha written only when it bends or the file had it, so a file that never
            // changed it saves as before.
            let alpha = (!crate::grade::is_straight(alpha) || params.contains_key("alpha")).then_some(("alpha", alpha));
            for (name, points) in [("master", master), ("red", red), ("green", green), ("blue", blue)].into_iter().chain(alpha) {
                params.insert(
                    name.into(),
                    J::Array(
                        points
                            .iter()
                            .map(|p| J::Array(p.iter().map(|v| num(*v)).collect()))
                            .collect(),
                    ),
                );
            }
        }
        Effect::Levels {
            input_black,
            input_white,
            gamma,
            output_black,
            output_white,
        } => {
            params.insert("input_black".into(), num(*input_black));
            params.insert("input_white".into(), num(*input_white));
            params.insert("gamma".into(), num(*gamma));
            params.insert("output_black".into(), num(*output_black));
            params.insert("output_white".into(), num(*output_white));
        }
        Effect::HueSaturation {
            hue,
            saturation,
            lightness,
            ranges,
        } => {
            params.insert("hue".into(), num(*hue));
            params.insert("saturation".into(), num(*saturation));
            params.insert("lightness".into(), num(*lightness));
            // D-307: a range is written only once it is moved or keyed, or is in the file already.
            for (key, r) in crate::effects::HUE_RANGES.into_iter().zip(ranges) {
                if *r != [0.0; 3] || instance.tracks.contains_key(key) || params.contains_key(key) {
                    params.insert(key.into(), J::Array(r.iter().map(|v| num(*v)).collect()));
                }
            }
        }
        Effect::Gradient {
            shape,
            start,
            end,
            start_color,
            end_color,
            start_opacity,
            end_opacity,
            blend,
        } => {
            params.insert("shape".into(), J::from(shape.as_str()));
            params.insert("start".into(), J::Array(start.iter().map(|c| num(*c)).collect()));
            params.insert("end".into(), J::Array(end.iter().map(|c| num(*c)).collect()));
            params.insert("start_color".into(), J::from(start_color.as_str()));
            params.insert("end_color".into(), J::from(end_color.as_str()));
            params.insert("start_opacity".into(), num(*start_opacity));
            params.insert("end_opacity".into(), num(*end_opacity));
            params.insert("blend".into(), J::from(blend.as_str()));
        }
        Effect::DropShadow {
            color,
            opacity,
            direction,
            distance,
            softness,
        } => {
            params.insert("color".into(), J::from(color.as_str()));
            params.insert("opacity".into(), num(*opacity));
            params.insert("direction".into(), num(*direction));
            params.insert("distance".into(), num(*distance));
            params.insert("softness".into(), num(*softness));
        }
        Effect::LensBlur {
            radius,
            edges,
            iris,
            roundness,
            rotation,
            aspect,
            highlight_gain,
            highlight_threshold,
            layer,
            fit,
            channel,
            focal_distance,
            invert,
            ..
        } => {
            params.insert("radius".into(), num(*radius));
            params.insert("edges".into(), J::from(edges.as_str()));
            // D-121: a setting at its start, without keys, is written only if the file had it,
            // so a file from before it saves the same.
            for (key, value, start) in [
                ("iris", J::from(iris.as_str()), iris == "circle"),
                ("roundness", num(*roundness), *roundness == 0.0),
                ("rotation", num(*rotation), *rotation == 0.0),
                ("aspect", num(*aspect), *aspect == 1.0),
                ("highlight_gain", num(*highlight_gain), *highlight_gain == 0.0),
                ("highlight_threshold", num(*highlight_threshold), *highlight_threshold == 100.0),
                // D-359: the blur map, the same way.
                ("layer", layer.clone(), layer.as_str() == Some("")),
                ("fit", J::from(fit.as_str()), fit == "center"),
                ("channel", J::from(channel.as_str()), channel == "luminance"),
                ("focal_distance", num(*focal_distance), *focal_distance == 0.0),
                ("invert", J::from(invert.as_str()), invert == "off"),
            ] {
                if !start || instance.tracks.contains_key(key) || params.contains_key(key) {
                    params.insert(key.into(), value);
                }
            }
        }
        Effect::RimLight {
            color,
            direction,
            width,
            softness,
            intensity,
            blend,
        } => {
            params.insert("color".into(), J::from(color.as_str()));
            params.insert("direction".into(), num(*direction));
            params.insert("width".into(), num(*width));
            params.insert("softness".into(), num(*softness));
            params.insert("intensity".into(), num(*intensity));
            params.insert("blend".into(), J::from(blend.as_str()));
        }
        Effect::Outline {
            color,
            width,
            softness,
            opacity,
        } => {
            params.insert("color".into(), J::from(color.as_str()));
            params.insert("width".into(), num(*width));
            params.insert("softness".into(), num(*softness));
            params.insert("opacity".into(), num(*opacity));
        }
        Effect::Noise {
            amount,
            mode,
            seed,
            animate,
            ..
        } => {
            params.insert("amount".into(), num(*amount));
            params.insert("mode".into(), J::from(mode.as_str()));
            params.insert("seed".into(), num(*seed));
            params.insert("animate".into(), J::from(animate.as_str()));
        }
        Effect::ChromaticAberration { amount, center } => {
            params.insert("amount".into(), num(*amount));
            params.insert(
                "center".into(),
                J::Array(center.iter().map(|c| num(*c)).collect()),
            );
        }
        Effect::DistanceGradation {
            color,
            width,
            opacity,
            invert,
            blend,
        } => {
            params.insert("color".into(), J::from(color.as_str()));
            params.insert("width".into(), num(*width));
            params.insert("opacity".into(), num(*opacity));
            params.insert("invert".into(), J::from(invert.as_str()));
            params.insert("blend".into(), J::from(blend.as_str()));
        }
        Effect::LightRays {
            center,
            length,
            threshold,
            intensity,
            color,
        } => {
            params.insert(
                "center".into(),
                J::Array(center.iter().map(|c| num(*c)).collect()),
            );
            params.insert("length".into(), num(*length));
            params.insert("threshold".into(), num(*threshold));
            params.insert("intensity".into(), num(*intensity));
            params.insert("color".into(), J::from(color.as_str()));
        }
        Effect::ExposureFlicker {
            amount, hold, seed, ..
        } => {
            params.insert("amount".into(), num(*amount));
            params.insert("hold".into(), num(*hold));
            params.insert("seed".into(), num(*seed));
        }
        Effect::Vignette {
            amount,
            color,
            size,
            roundness,
            softness,
            center,
        } => {
            params.insert("amount".into(), num(*amount));
            params.insert("color".into(), J::from(color.as_str()));
            params.insert("size".into(), num(*size));
            params.insert("roundness".into(), num(*roundness));
            params.insert("softness".into(), num(*softness));
            params.insert(
                "center".into(),
                J::Array(center.iter().map(|c| num(*c)).collect()),
            );
        }
        Effect::TurbulentDisplace {
            amount,
            size,
            complexity,
            evolution,
            speed,
            seed,
            edges,
            displacement,
            pinning,
            units,
            ..
        } => {
            params.insert("amount".into(), num(*amount));
            params.insert("size".into(), num(*size));
            params.insert("complexity".into(), num(*complexity));
            params.insert("evolution".into(), num(*evolution));
            params.insert("speed".into(), num(*speed));
            params.insert("seed".into(), num(*seed));
            params.insert("edges".into(), J::from(edges.as_str()));
            // D-306: as D-303's, written only when changed or in the file already.
            if displacement != "turbulent" || params.contains_key("displacement") {
                params.insert("displacement".into(), J::from(displacement.as_str()));
            }
            if pinning != "none" || params.contains_key("pinning") {
                params.insert("pinning".into(), J::from(pinning.as_str()));
            }
            // D-328: as D-322's, written only when not classic or in the file already.
            if units != "classic" || params.contains_key("units") {
                params.insert("units".into(), J::from(units.as_str()));
            }
        }
        Effect::FractalNoise {
            size,
            complexity,
            contrast,
            brightness,
            evolution,
            speed,
            seed,
            dark_color,
            light_color,
            opacity,
            blend,
            fractal_type,
            noise_type,
            invert,
            offset,
            scale_width,
            scale_height,
            cycle,
            ..
        } => {
            // D-299: as D-121, each at its start, without keys, written only if the file had
            // it, so a file that never changed them saves as before.
            for (key, value, start) in [
                ("fractal_type", J::from(fractal_type.as_str()), fractal_type == "basic"),
                ("noise_type", J::from(noise_type.as_str()), noise_type == "smooth"),
                ("invert", J::from(invert.as_str()), invert == "off"),
                ("offset", J::Array(offset.iter().map(|v| num(*v)).collect()), *offset == [0.0, 0.0]),
                ("scale_width", num(*scale_width), *scale_width == 100.0),
                ("scale_height", num(*scale_height), *scale_height == 100.0),
                ("cycle", num(*cycle), *cycle == 0.0),
            ] {
                if !start || instance.tracks.contains_key(key) || params.contains_key(key) {
                    params.insert(key.into(), value);
                }
            }
            params.insert("size".into(), num(*size));
            params.insert("complexity".into(), num(*complexity));
            params.insert("contrast".into(), num(*contrast));
            params.insert("brightness".into(), num(*brightness));
            params.insert("evolution".into(), num(*evolution));
            params.insert("speed".into(), num(*speed));
            params.insert("seed".into(), num(*seed));
            params.insert("dark_color".into(), J::from(dark_color.as_str()));
            params.insert("light_color".into(), J::from(light_color.as_str()));
            params.insert("opacity".into(), num(*opacity));
            params.insert("blend".into(), J::from(blend.as_str()));
        }
        Effect::GradientMap {
            shadow_color,
            midtone_color,
            highlight_color,
            midpoint,
            amount,
        } => {
            params.insert("shadow_color".into(), J::from(shadow_color.as_str()));
            params.insert("midtone_color".into(), J::from(midtone_color.as_str()));
            params.insert("highlight_color".into(), J::from(highlight_color.as_str()));
            params.insert("midpoint".into(), num(*midpoint));
            params.insert("amount".into(), num(*amount));
        }
        Effect::ColorBalance {
            shadows,
            midtones,
            highlights,
            preserve_luminosity,
        } => {
            for (name, tone) in [("shadows", shadows), ("midtones", midtones), ("highlights", highlights)] {
                params.insert(name.into(), J::Array(tone.iter().map(|v| num(*v)).collect()));
            }
            // D-295: written only when it is not "off" or the file had it, so a file that never
            // turned it on saves as before.
            if preserve_luminosity != "off" || params.contains_key("preserve_luminosity") {
                params.insert("preserve_luminosity".into(), J::from(preserve_luminosity.as_str()));
            }
        }
        Effect::Offset { shift } => {
            params.insert("shift".into(), J::Array(shift.iter().map(|v| num(*v)).collect()));
        }
        Effect::LightWrap {
            width,
            intensity,
            blend,
        } => {
            params.insert("width".into(), num(*width));
            params.insert("intensity".into(), num(*intensity));
            params.insert("blend".into(), J::from(blend.as_str()));
        }
        Effect::Invert { channel, amount } => {
            params.insert("channel".into(), J::from(channel.as_str()));
            params.insert("amount".into(), num(*amount));
        }
        Effect::BrightnessContrast { brightness, contrast } => {
            params.insert("brightness".into(), num(*brightness));
            params.insert("contrast".into(), num(*contrast));
        }
        Effect::BlackWhite { reds, yellows, greens, cyans, blues, magentas } => {
            params.insert("reds".into(), num(*reds));
            params.insert("yellows".into(), num(*yellows));
            params.insert("greens".into(), num(*greens));
            params.insert("cyans".into(), num(*cyans));
            params.insert("blues".into(), num(*blues));
            params.insert("magentas".into(), num(*magentas));
        }
        Effect::Posterize { levels } => {
            params.insert("levels".into(), num(*levels));
        }
        Effect::Threshold { level } => {
            params.insert("level".into(), num(*level));
        }
        Effect::ChannelMixer {
            red,
            green,
            blue,
            monochrome,
        } => {
            for (name, row) in [("red", red), ("green", green), ("blue", blue)] {
                params.insert(name.into(), J::Array(row.iter().map(|v| num(*v)).collect()));
            }
            params.insert("monochrome".into(), J::from(monochrome.as_str()));
        }
        Effect::Vibrance { vibrance, saturation } => {
            params.insert("vibrance".into(), num(*vibrance));
            params.insert("saturation".into(), num(*saturation));
        }
        Effect::LeaveColor {
            color,
            tolerance,
            softness,
            amount,
        } => {
            params.insert("color".into(), J::from(color.as_str()));
            params.insert("tolerance".into(), num(*tolerance));
            params.insert("softness".into(), num(*softness));
            params.insert("amount".into(), num(*amount));
        }
        Effect::Solarize { threshold } => {
            params.insert("threshold".into(), num(*threshold));
        }
        Effect::Halftone {
            size,
            angle,
            ink,
            paper,
            amount,
        } => {
            params.insert("size".into(), num(*size));
            params.insert("angle".into(), num(*angle));
            params.insert("ink".into(), J::from(ink.as_str()));
            params.insert("paper".into(), J::from(paper.as_str()));
            params.insert("amount".into(), num(*amount));
        }
        Effect::Mosaic { size } => {
            params.insert("size".into(), num(*size));
        }
        Effect::Emboss {
            direction,
            relief,
            contrast,
            mode,
        } => {
            params.insert("direction".into(), num(*direction));
            params.insert("relief".into(), num(*relief));
            params.insert("contrast".into(), num(*contrast));
            params.insert("mode".into(), J::from(mode.as_str()));
        }
        Effect::FindEdges { invert, amount } => {
            params.insert("invert".into(), J::from(invert.as_str()));
            params.insert("amount".into(), num(*amount));
        }
        Effect::Sharpen { amount, radius, threshold } => {
            params.insert("amount".into(), num(*amount));
            params.insert("radius".into(), num(*radius));
            // D-317: as D-310, written only if moved, keyed or already in the file.
            if *threshold != 0.0 || instance.tracks.contains_key("threshold") || params.contains_key("threshold") {
                params.insert("threshold".into(), num(*threshold));
            }
        }
        Effect::Diffusion { radius, amount, blend } => {
            params.insert("radius".into(), num(*radius));
            params.insert("amount".into(), num(*amount));
            params.insert("blend".into(), J::from(blend.as_str()));
        }
        Effect::WaveWarp {
            shape,
            height,
            width,
            direction,
            speed,
            phase,
            edges,
            ..
        } => {
            params.insert("shape".into(), J::from(shape.as_str()));
            params.insert("height".into(), num(*height));
            params.insert("width".into(), num(*width));
            params.insert("direction".into(), num(*direction));
            params.insert("speed".into(), num(*speed));
            params.insert("phase".into(), num(*phase));
            params.insert("edges".into(), J::from(edges.as_str()));
        }
        Effect::Ripple {
            center,
            amplitude,
            wavelength,
            speed,
            phase,
            fade,
            ..
        } => {
            params.insert(
                "center".into(),
                J::Array(center.iter().map(|c| num(*c)).collect()),
            );
            params.insert("amplitude".into(), num(*amplitude));
            params.insert("wavelength".into(), num(*wavelength));
            params.insert("speed".into(), num(*speed));
            params.insert("phase".into(), num(*phase));
            params.insert("fade".into(), num(*fade));
        }
        Effect::Twirl {
            angle,
            radius,
            center,
        } => {
            params.insert("angle".into(), num(*angle));
            params.insert("radius".into(), num(*radius));
            params.insert(
                "center".into(),
                J::Array(center.iter().map(|c| num(*c)).collect()),
            );
        }
        Effect::Bulge {
            center,
            radius,
            height,
            vertical_radius,
            taper_radius,
        } => {
            params.insert(
                "center".into(),
                J::Array(center.iter().map(|c| num(*c)).collect()),
            );
            params.insert("radius".into(), num(*radius));
            params.insert("height".into(), num(*height));
            // D-310: as D-304, written only if moved, keyed or already in the file.
            for (key, value) in [("vertical_radius", *vertical_radius), ("taper_radius", *taper_radius)] {
                if value != 0.0 || instance.tracks.contains_key(key) || params.contains_key(key) {
                    params.insert(key.into(), num(value));
                }
            }
        }
        Effect::Mirror { center, angle } => {
            params.insert(
                "center".into(),
                J::Array(center.iter().map(|c| num(*c)).collect()),
            );
            params.insert("angle".into(), num(*angle));
        }
        Effect::MotionTile {
            output_width,
            output_height,
            mirror,
            tile_center,
            tile_width,
            tile_height,
        } => {
            params.insert("output_width".into(), num(*output_width));
            params.insert("output_height".into(), num(*output_height));
            params.insert("mirror".into(), J::from(mirror.as_str()));
            // D-304: as D-121, each at its start, without keys, written only if the file had
            // it, so a file that never changed them saves as before.
            for (key, value, start) in [
                ("tile_center", J::Array(tile_center.iter().map(|v| num(*v)).collect()), *tile_center == crate::layer_fx::PLAIN_TILE),
                ("tile_width", num(*tile_width), *tile_width == 100.0),
                ("tile_height", num(*tile_height), *tile_height == 100.0),
            ] {
                if !start || instance.tracks.contains_key(key) || params.contains_key(key) {
                    params.insert(key.into(), value);
                }
            }
        }
        Effect::LinearWipe {
            completion,
            angle,
            feather,
        } => {
            params.insert("completion".into(), num(*completion));
            params.insert("angle".into(), num(*angle));
            params.insert("feather".into(), num(*feather));
        }
        Effect::RadialWipe {
            completion,
            start_angle,
            center,
            wipe,
            feather,
        } => {
            params.insert("completion".into(), num(*completion));
            params.insert("start_angle".into(), num(*start_angle));
            params.insert(
                "center".into(),
                J::Array(center.iter().map(|c| num(*c)).collect()),
            );
            params.insert("wipe".into(), J::from(wipe.as_str()));
            params.insert("feather".into(), num(*feather));
        }
        Effect::VenetianBlinds {
            completion,
            angle,
            width,
            feather,
        } => {
            params.insert("completion".into(), num(*completion));
            params.insert("angle".into(), num(*angle));
            params.insert("width".into(), num(*width));
            params.insert("feather".into(), num(*feather));
        }
        Effect::IrisWipe {
            completion,
            center,
            feather,
            invert,
        } => {
            params.insert("completion".into(), num(*completion));
            params.insert(
                "center".into(),
                J::Array(center.iter().map(|c| num(*c)).collect()),
            );
            params.insert("feather".into(), num(*feather));
            params.insert("invert".into(), J::from(invert.as_str()));
        }
        Effect::SimpleChoker { choke } => {
            params.insert("choke".into(), num(*choke));
        }
        Effect::SpeedLines {
            center,
            color,
            count,
            thickness,
            inner,
            inner_jitter,
            angle_jitter,
            seed,
            hold,
            opacity,
            ..
        } => {
            params.insert(
                "center".into(),
                J::Array(center.iter().map(|c| num(*c)).collect()),
            );
            params.insert("color".into(), J::from(color.as_str()));
            params.insert("count".into(), num(*count));
            params.insert("thickness".into(), num(*thickness));
            params.insert("inner".into(), num(*inner));
            params.insert("inner_jitter".into(), num(*inner_jitter));
            params.insert("angle_jitter".into(), num(*angle_jitter));
            params.insert("seed".into(), num(*seed));
            params.insert("hold".into(), num(*hold));
            params.insert("opacity".into(), num(*opacity));
        }
        Effect::CrossGlare {
            threshold,
            length,
            points,
            angle,
            intensity,
            color,
        } => {
            params.insert("threshold".into(), num(*threshold));
            params.insert("length".into(), num(*length));
            params.insert("points".into(), num(*points));
            params.insert("angle".into(), num(*angle));
            params.insert("intensity".into(), num(*intensity));
            params.insert("color".into(), J::from(color.as_str()));
        }
        Effect::CameraShake {
            amount,
            rotation,
            hold,
            seed,
            ..
        } => {
            params.insert("amount".into(), num(*amount));
            params.insert("rotation".into(), num(*rotation));
            params.insert("hold".into(), num(*hold));
            params.insert("seed".into(), num(*seed));
        }
        Effect::Rain {
            color,
            density,
            spacing,
            length,
            width,
            direction,
            speed,
            seed,
            opacity,
            ..
        } => {
            params.insert("color".into(), J::from(color.as_str()));
            params.insert("density".into(), num(*density));
            params.insert("spacing".into(), num(*spacing));
            params.insert("length".into(), num(*length));
            params.insert("width".into(), num(*width));
            params.insert("direction".into(), num(*direction));
            params.insert("speed".into(), num(*speed));
            params.insert("seed".into(), num(*seed));
            params.insert("opacity".into(), num(*opacity));
        }
        Effect::ColorLookup { lut, .. } => {
            params.insert("lut".into(), J::from(lut.as_str()));
        }
        Effect::LineBlur {
            length,
            strength,
            lines_only,
        } => {
            params.insert("length".into(), num(*length));
            params.insert("strength".into(), num(*strength));
            params.insert("lines_only".into(), J::from(lines_only.as_str()));
        }
        Effect::HsvKey {
            hue,
            saturation,
            value,
            hue_range,
            saturation_range,
            value_range,
            invert,
        } => {
            params.insert("hue".into(), num(*hue));
            params.insert("saturation".into(), num(*saturation));
            params.insert("value".into(), num(*value));
            params.insert("hue_range".into(), num(*hue_range));
            params.insert("saturation_range".into(), num(*saturation_range));
            params.insert("value_range".into(), num(*value_range));
            params.insert("invert".into(), J::from(invert.as_str()));
        }
        Effect::Paraffin {
            color,
            direction,
            spread,
            opacity,
            blend,
        } => {
            params.insert("color".into(), J::from(color.as_str()));
            params.insert("direction".into(), num(*direction));
            params.insert("spread".into(), num(*spread));
            params.insert("opacity".into(), num(*opacity));
            params.insert("blend".into(), J::from(blend.as_str()));
        }
        Effect::KiraKira {
            threshold,
            spacing,
            density,
            size,
            shape,
            angle,
            twinkle,
            period,
            seed,
            opacity,
            color,
            ..
        } => {
            params.insert("threshold".into(), num(*threshold));
            params.insert("spacing".into(), num(*spacing));
            params.insert("density".into(), num(*density));
            params.insert("size".into(), num(*size));
            params.insert("shape".into(), J::from(shape.as_str()));
            params.insert("angle".into(), num(*angle));
            params.insert("twinkle".into(), num(*twinkle));
            params.insert("period".into(), num(*period));
            params.insert("seed".into(), num(*seed));
            params.insert("opacity".into(), num(*opacity));
            params.insert("color".into(), J::from(color.as_str()));
        }
        Effect::LightningBolt {
            start,
            end,
            jagged,
            detail,
            branches,
            width,
            glow,
            opacity,
            hold,
            seed,
            color,
            glow_color,
            composite,
            kind,
            turbulence,
            decay,
            conductivity,
            obstacle,
            path,
            core,
            forks,
            ..
        } => {
            // D-300: written only when off or the file had it, so a file that never changed it
            // saves as before.
            if composite != "on" || params.contains_key("composite") {
                params.insert("composite".into(), J::from(composite.as_str()));
            }
            // D-324: as D-300 and D-310, written only if moved, keyed or already in the file.
            if kind != "direction" || params.contains_key("kind") {
                params.insert("kind".into(), J::from(kind.as_str()));
            }
            // D-329: the same.
            if path != "split" || params.contains_key("path") {
                params.insert("path".into(), J::from(path.as_str()));
            }
            // D-334: the same.
            if core != "hard" || params.contains_key("core") {
                params.insert("core".into(), J::from(core.as_str()));
            }
            // D-338: the same.
            if forks != "short" || params.contains_key("forks") {
                params.insert("forks".into(), J::from(forks.as_str()));
            }
            for (key, value) in [("turbulence", *turbulence), ("decay", *decay), ("conductivity", *conductivity), ("obstacle", *obstacle)] {
                if value != 0.0 || instance.tracks.contains_key(key) || params.contains_key(key) {
                    params.insert(key.into(), num(value));
                }
            }
            params.insert("start".into(), J::Array(start.iter().map(|c| num(*c)).collect()));
            params.insert("end".into(), J::Array(end.iter().map(|c| num(*c)).collect()));
            params.insert("jagged".into(), num(*jagged));
            params.insert("detail".into(), num(*detail));
            params.insert("branches".into(), num(*branches));
            params.insert("width".into(), num(*width));
            params.insert("glow".into(), num(*glow));
            params.insert("opacity".into(), num(*opacity));
            params.insert("hold".into(), num(*hold));
            params.insert("seed".into(), num(*seed));
            params.insert("color".into(), J::from(color.as_str()));
            params.insert("glow_color".into(), J::from(glow_color.as_str()));
        }
        Effect::CompoundBlur { layer, fit, max_blur, invert, edges, .. } => {
            params.insert("layer".into(), layer.clone());
            params.insert("fit".into(), J::from(fit.as_str()));
            params.insert("max_blur".into(), num(*max_blur));
            params.insert("invert".into(), J::from(invert.as_str()));
            params.insert("edges".into(), J::from(edges.as_str()));
        }
        Effect::DisplacementMap { layer, fit, horizontal, max_horizontal, vertical, max_vertical, wrap, expand, .. } => {
            params.insert("layer".into(), layer.clone());
            params.insert("fit".into(), J::from(fit.as_str()));
            params.insert("horizontal".into(), J::from(horizontal.as_str()));
            params.insert("max_horizontal".into(), num(*max_horizontal));
            params.insert("vertical".into(), J::from(vertical.as_str()));
            params.insert("max_vertical".into(), num(*max_vertical));
            params.insert("wrap".into(), J::from(wrap.as_str()));
            // D-315: as D-303's, written only when changed or in the file already.
            if expand != "off" || params.contains_key("expand") {
                params.insert("expand".into(), J::from(expand.as_str()));
            }
        }
        Effect::GradientWipe { layer, fit, completion, softness, invert, .. } => {
            params.insert("layer".into(), layer.clone());
            params.insert("fit".into(), J::from(fit.as_str()));
            params.insert("completion".into(), num(*completion));
            params.insert("softness".into(), num(*softness));
            params.insert("invert".into(), J::from(invert.as_str()));
        }
        Effect::Echo { echo_time, echoes, intensity, decay, operator, .. } => {
            params.insert("echo_time".into(), num(*echo_time));
            params.insert("echoes".into(), num(*echoes));
            params.insert("intensity".into(), num(*intensity));
            params.insert("decay".into(), num(*decay));
            params.insert("operator".into(), J::from(operator.as_str()));
        }
        Effect::PosterizeTime { frame_rate } => {
            params.insert("frame_rate".into(), num(*frame_rate));
        }
        Effect::ChangeToColor {
            from,
            to,
            change,
            change_by,
            hue_tolerance,
            lightness_tolerance,
            saturation_tolerance,
            softness,
            view_matte,
        } => {
            params.insert("from".into(), J::from(from.as_str()));
            params.insert("to".into(), J::from(to.as_str()));
            params.insert("change".into(), J::from(change.as_str()));
            params.insert("change_by".into(), J::from(change_by.as_str()));
            params.insert("hue_tolerance".into(), num(*hue_tolerance));
            params.insert("lightness_tolerance".into(), num(*lightness_tolerance));
            params.insert("saturation_tolerance".into(), num(*saturation_tolerance));
            params.insert("softness".into(), num(*softness));
            params.insert("view_matte".into(), J::from(view_matte.as_str()));
        }
        Effect::CornerPin { upper_left, upper_right, lower_left, lower_right } => {
            for (name, v) in [
                ("upper_left", upper_left),
                ("upper_right", upper_right),
                ("lower_left", lower_left),
                ("lower_right", lower_right),
            ] {
                params.insert(name.into(), J::Array(v.iter().map(|c| num(*c)).collect()));
            }
        }
        Effect::LightSweep {
            center,
            direction,
            shape,
            width,
            sweep_intensity,
            edge_intensity,
            edge_thickness,
            light_color,
            light_reception,
        } => {
            params.insert("center".into(), J::Array(center.iter().map(|c| num(*c)).collect()));
            params.insert("direction".into(), num(*direction));
            params.insert("shape".into(), J::from(shape.as_str()));
            params.insert("width".into(), num(*width));
            params.insert("sweep_intensity".into(), num(*sweep_intensity));
            params.insert("edge_intensity".into(), num(*edge_intensity));
            params.insert("edge_thickness".into(), num(*edge_thickness));
            params.insert("light_color".into(), J::from(light_color.as_str()));
            params.insert("light_reception".into(), J::from(light_reception.as_str()));
        }
        Effect::RadioWaves {
            producer_point,
            sides,
            interval,
            expansion,
            orientation,
            direction,
            velocity,
            spin,
            lifespan,
            opacity,
            fade_in_time,
            fade_out_time,
            start_width,
            end_width,
            profile,
            color,
            ..
        } => {
            params.insert("producer_point".into(), J::Array(producer_point.iter().map(|c| num(*c)).collect()));
            params.insert("sides".into(), num(*sides));
            params.insert("interval".into(), num(*interval));
            params.insert("expansion".into(), num(*expansion));
            params.insert("orientation".into(), num(*orientation));
            params.insert("direction".into(), num(*direction));
            params.insert("velocity".into(), num(*velocity));
            params.insert("spin".into(), num(*spin));
            params.insert("lifespan".into(), num(*lifespan));
            params.insert("opacity".into(), num(*opacity));
            params.insert("fade_in_time".into(), num(*fade_in_time));
            params.insert("fade_out_time".into(), num(*fade_out_time));
            params.insert("start_width".into(), num(*start_width));
            params.insert("end_width".into(), num(*end_width));
            params.insert("profile".into(), J::from(profile.as_str()));
            params.insert("color".into(), J::from(color.as_str()));
        }
        Effect::PolarCoordinates { interpolation, conversion, shape } => {
            params.insert("interpolation".into(), num(*interpolation));
            params.insert("conversion".into(), J::from(conversion.as_str()));
            // D-320: as D-303's, written only when a circle or in the file already.
            if shape != "ellipse" || params.contains_key("shape") {
                params.insert("shape".into(), J::from(shape.as_str()));
            }
        }
        Effect::Median { radius, operate_on_alpha } => {
            params.insert("radius".into(), num(*radius));
            params.insert("operate_on_alpha".into(), J::from(operate_on_alpha.as_str()));
        }
        Effect::SmartBlur { radius, threshold } => {
            params.insert("radius".into(), num(*radius));
            params.insert("threshold".into(), num(*threshold));
        }
        Effect::BilateralBlur { radius, threshold, colorize } => {
            params.insert("radius".into(), num(*radius));
            params.insert("threshold".into(), num(*threshold));
            params.insert("colorize".into(), J::from(colorize.as_str()));
        }
        Effect::Snowfall {
            color,
            density,
            spacing,
            size,
            depth,
            speed,
            wind,
            wiggle,
            period,
            seed,
            opacity,
            ..
        } => {
            params.insert("color".into(), J::from(color.as_str()));
            params.insert("density".into(), num(*density));
            params.insert("spacing".into(), num(*spacing));
            params.insert("size".into(), num(*size));
            params.insert("depth".into(), num(*depth));
            params.insert("speed".into(), num(*speed));
            params.insert("wind".into(), num(*wind));
            params.insert("wiggle".into(), num(*wiggle));
            params.insert("period".into(), num(*period));
            params.insert("seed".into(), num(*seed));
            params.insert("opacity".into(), num(*opacity));
        }
        Effect::Kaleidoscope {
            segments,
            rotation,
            size,
            center,
            mode,
        } => {
            params.insert("segments".into(), num(*segments));
            params.insert("rotation".into(), num(*rotation));
            params.insert("size".into(), num(*size));
            params.insert("center".into(), J::Array(center.iter().map(|c| num(*c)).collect()));
            params.insert("mode".into(), J::from(mode.as_str()));
        }
        Effect::RoughenEdges {
            edge_type,
            edge_color,
            border,
            size,
            complexity,
            evolution,
            speed,
            seed,
            ..
        } => {
            params.insert("edge_type".into(), J::from(edge_type.as_str()));
            params.insert("edge_color".into(), J::from(edge_color.as_str()));
            params.insert("border".into(), num(*border));
            params.insert("size".into(), num(*size));
            params.insert("complexity".into(), num(*complexity));
            params.insert("evolution".into(), num(*evolution));
            params.insert("speed".into(), num(*speed));
            params.insert("seed".into(), num(*seed));
        }
        Effect::Beam {
            start,
            end,
            length,
            time,
            start_thickness,
            end_thickness,
            softness,
            inside_color,
            outside_color,
            composite,
        } => {
            params.insert("start".into(), J::Array(start.iter().map(|c| num(*c)).collect()));
            params.insert("end".into(), J::Array(end.iter().map(|c| num(*c)).collect()));
            params.insert("length".into(), num(*length));
            params.insert("time".into(), num(*time));
            params.insert("start_thickness".into(), num(*start_thickness));
            params.insert("end_thickness".into(), num(*end_thickness));
            params.insert("softness".into(), num(*softness));
            params.insert("inside_color".into(), J::from(inside_color.as_str()));
            params.insert("outside_color".into(), J::from(outside_color.as_str()));
            params.insert("composite".into(), J::from(composite.as_str()));
        }
        Effect::FourColorGradient {
            point_1,
            point_2,
            point_3,
            point_4,
            color_1,
            color_2,
            color_3,
            color_4,
            blend,
            opacity,
            blending_mode,
        } => {
            for (name, p) in [("point_1", point_1), ("point_2", point_2), ("point_3", point_3), ("point_4", point_4)] {
                params.insert(name.into(), J::Array(p.iter().map(|c| num(*c)).collect()));
            }
            for (name, c) in [("color_1", color_1), ("color_2", color_2), ("color_3", color_3), ("color_4", color_4)] {
                params.insert(name.into(), J::from(c.as_str()));
            }
            params.insert("blend".into(), num(*blend));
            params.insert("opacity".into(), num(*opacity));
            params.insert("blending_mode".into(), J::from(blending_mode.as_str()));
        }
        Effect::CellPattern {
            pattern,
            invert,
            contrast,
            disperse,
            size,
            evolution,
            seed,
            dark_color,
            light_color,
            opacity,
            blend,
        } => {
            for (name, w) in [("pattern", pattern), ("invert", invert), ("dark_color", dark_color), ("light_color", light_color), ("blend", blend)] {
                params.insert(name.into(), J::from(w.as_str()));
            }
            for (name, v) in [("contrast", contrast), ("disperse", disperse), ("size", size), ("evolution", evolution), ("seed", seed), ("opacity", opacity)] {
                params.insert(name.into(), num(*v));
            }
        }
        Effect::OpticsCompensation {
            field_of_view,
            reverse,
            orientation,
            center,
        } => {
            params.insert("field_of_view".into(), num(*field_of_view));
            params.insert("reverse".into(), J::from(reverse.as_str()));
            params.insert("orientation".into(), J::from(orientation.as_str()));
            params.insert("center".into(), J::Array(center.iter().map(|c| num(*c)).collect()));
        }
        Effect::RadialShadow {
            color,
            opacity,
            light,
            distance,
            softness,
            render,
            color_influence,
            shadow_only,
        } => {
            params.insert("color".into(), J::from(color.as_str()));
            params.insert("opacity".into(), num(*opacity));
            params.insert("light".into(), J::Array(light.iter().map(|c| num(*c)).collect()));
            params.insert("distance".into(), num(*distance));
            params.insert("softness".into(), num(*softness));
            params.insert("render".into(), J::from(render.as_str()));
            params.insert("color_influence".into(), num(*color_influence));
            params.insert("shadow_only".into(), J::from(shadow_only.as_str()));
        }
        Effect::Extract {
            channel,
            black_point,
            white_point,
            black_softness,
            white_softness,
            invert,
        } => {
            params.insert("channel".into(), J::from(channel.as_str()));
            params.insert("black_point".into(), num(*black_point));
            params.insert("white_point".into(), num(*white_point));
            params.insert("black_softness".into(), num(*black_softness));
            params.insert("white_softness".into(), num(*white_softness));
            params.insert("invert".into(), J::from(invert.as_str()));
        }
        Effect::BevelAlpha {
            edge_thickness,
            light_angle,
            light_color,
            light_intensity,
        }
        | Effect::BevelEdges {
            edge_thickness,
            light_angle,
            light_color,
            light_intensity,
        } => {
            params.insert("edge_thickness".into(), num(*edge_thickness));
            params.insert("light_angle".into(), num(*light_angle));
            params.insert("light_color".into(), J::from(light_color.as_str()));
            params.insert("light_intensity".into(), num(*light_intensity));
        }
        Effect::BlockDissolve {
            completion,
            block_width,
            block_height,
            feather,
        } => {
            params.insert("completion".into(), num(*completion));
            params.insert("block_width".into(), num(*block_width));
            params.insert("block_height".into(), num(*block_height));
            params.insert("feather".into(), num(*feather));
        }
        Effect::ShiftChannels {
            take_alpha,
            take_red,
            take_green,
            take_blue,
        } => {
            params.insert("take_alpha".into(), J::from(take_alpha.as_str()));
            params.insert("take_red".into(), J::from(take_red.as_str()));
            params.insert("take_green".into(), J::from(take_green.as_str()));
            params.insert("take_blue".into(), J::from(take_blue.as_str()));
        }
        Effect::SolidComposite { source_opacity, color, opacity, blend } => {
            params.insert("source_opacity".into(), num(*source_opacity));
            params.insert("color".into(), J::from(color.as_str()));
            params.insert("opacity".into(), num(*opacity));
            params.insert("blend".into(), J::from(blend.as_str()));
        }
        Effect::ChannelBlur {
            red_blurriness,
            green_blurriness,
            blue_blurriness,
            alpha_blurriness,
            edges,
            dimensions,
        } => {
            params.insert("red_blurriness".into(), num(*red_blurriness));
            params.insert("green_blurriness".into(), num(*green_blurriness));
            params.insert("blue_blurriness".into(), num(*blue_blurriness));
            params.insert("alpha_blurriness".into(), num(*alpha_blurriness));
            put_edges(&mut params, edges);
            params.insert("dimensions".into(), J::from(dimensions.as_str()));
        }
        Effect::FastBoxBlur { radius, iterations, edges, dimensions } => {
            params.insert("radius".into(), num(*radius));
            params.insert("iterations".into(), num(*iterations));
            put_edges(&mut params, edges);
            if dimensions != "both" || params.contains_key("dimensions") {
                params.insert("dimensions".into(), J::from(dimensions.as_str()));
            }
        }
        Effect::Colorama {
            get_phase,
            layer,
            fit,
            phase_shift,
            cycle_repetitions,
            stops,
            color_1,
            color_2,
            color_3,
            color_4,
            color_5,
            blend_with_original,
            ..
        } => {
            params.insert("get_phase".into(), J::from(get_phase.as_str()));
            params.insert("layer".into(), layer.clone());
            params.insert("fit".into(), J::from(fit.as_str()));
            params.insert("phase_shift".into(), num(*phase_shift));
            params.insert("cycle_repetitions".into(), num(*cycle_repetitions));
            params.insert("stops".into(), num(*stops));
            for (key, c) in [("color_1", color_1), ("color_2", color_2), ("color_3", color_3), ("color_4", color_4), ("color_5", color_5)] {
                params.insert(key.into(), J::from(c.as_str()));
            }
            params.insert("blend_with_original".into(), num(*blend_with_original));
        }
        Effect::Glass {
            layer,
            fit,
            property,
            softness,
            height,
            displacement,
            light_angle,
            light_color,
            light_intensity,
            ..
        } => {
            params.insert("layer".into(), layer.clone());
            params.insert("fit".into(), J::from(fit.as_str()));
            params.insert("property".into(), J::from(property.as_str()));
            params.insert("softness".into(), num(*softness));
            params.insert("height".into(), num(*height));
            params.insert("displacement".into(), num(*displacement));
            params.insert("light_angle".into(), num(*light_angle));
            params.insert("light_color".into(), J::from(light_color.as_str()));
            params.insert("light_intensity".into(), num(*light_intensity));
        }
        Effect::VectorBlur { kind, amount, angle_offset, ridge_smoothness, layer, fit, property, map_softness, .. } => {
            params.insert("type".into(), J::from(kind.as_str()));
            params.insert("amount".into(), num(*amount));
            params.insert("angle_offset".into(), num(*angle_offset));
            params.insert("ridge_smoothness".into(), num(*ridge_smoothness));
            params.insert("layer".into(), layer.clone());
            params.insert("fit".into(), J::from(fit.as_str()));
            params.insert("property".into(), J::from(property.as_str()));
            params.insert("map_softness".into(), num(*map_softness));
        }
        Effect::MomentMap { max_time, resolution, layer, fit, .. } => {
            params.insert("max_time".into(), num(*max_time));
            params.insert("resolution".into(), num(*resolution));
            params.insert("layer".into(), layer.clone());
            params.insert("fit".into(), J::from(fit.as_str()));
        }
        // D-348: the pass read for a frame is never saved; it is read from the file again.
        Effect::PassExtract { pass, black_point, white_point, invert, clamp, channel, .. } => {
            params.insert("pass".into(), J::from(pass.as_str()));
            params.insert("black_point".into(), num(*black_point));
            params.insert("white_point".into(), num(*white_point));
            params.insert("invert".into(), J::from(invert.as_str()));
            params.insert("clamp".into(), J::from(clamp.as_str()));
            // D-349: the channel names, left out of the file while empty.
            if !channel.is_empty() || params.contains_key("channel") {
                params.insert("channel".into(), J::from(channel.as_str()));
            }
        }
        Effect::DepthKey { depth, feather, invert, .. } => {
            params.insert("depth".into(), num(*depth));
            params.insert("feather".into(), num(*feather));
            params.insert("invert".into(), J::from(invert.as_str()));
        }
        Effect::IdKey { aux_channel, id, feather, invert, .. } => {
            params.insert("aux_channel".into(), J::from(aux_channel.as_str()));
            params.insert("id".into(), num(*id));
            params.insert("feather".into(), num(*feather));
            params.insert("invert".into(), J::from(invert.as_str()));
        }
        Effect::TextAnimator {
            position, scale, rotation, opacity, fill, color, tracking, start, end, offset, amount, based_on, shape, smoothness, ease_high, ease_low,
        } => {
            let list = |v: &[f64]| J::Array(v.iter().map(|x| num(*x)).collect());
            params.insert("position".into(), list(position));
            params.insert("scale".into(), list(scale));
            params.insert("rotation".into(), num(*rotation));
            params.insert("opacity".into(), num(*opacity));
            params.insert("fill".into(), J::from(fill.as_str()));
            params.insert("color".into(), list(color));
            params.insert("tracking".into(), num(*tracking));
            params.insert("start".into(), num(*start));
            params.insert("end".into(), num(*end));
            params.insert("offset".into(), num(*offset));
            params.insert("amount".into(), num(*amount));
            params.insert("based_on".into(), J::from(based_on.as_str()));
            params.insert("shape".into(), J::from(shape.as_str()));
            params.insert("smoothness".into(), num(*smoothness));
            params.insert("ease_high".into(), num(*ease_high));
            params.insert("ease_low".into(), num(*ease_low));
        }
        // D-351: the frames a smoothing added up are never saved; they are read again.
        Effect::AutoTone { kind, temporal_smoothing, scene_detect, black_clip, white_clip, snap_neutral_midtones, .. } => {
            params.insert("temporal_smoothing".into(), num(*temporal_smoothing));
            params.insert("scene_detect".into(), J::from(scene_detect.as_str()));
            params.insert("black_clip".into(), num(*black_clip));
            params.insert("white_clip".into(), num(*white_clip));
            if *kind == "color" {
                params.insert("snap_neutral_midtones".into(), J::from(snap_neutral_midtones.as_str()));
            }
        }
        Effect::SpreadTones { equalize, amount } => {
            params.insert("equalize".into(), J::from(equalize.as_str()));
            params.insert("amount".into(), num(*amount));
        }
        Effect::MatteChoker {
            geometric_softness_1,
            choke_1,
            gray_level_softness_1,
            geometric_softness_2,
            choke_2,
            gray_level_softness_2,
            iterations,
        } => {
            params.insert("geometric_softness_1".into(), num(*geometric_softness_1));
            params.insert("choke_1".into(), num(*choke_1));
            params.insert("gray_level_softness_1".into(), num(*gray_level_softness_1));
            params.insert("geometric_softness_2".into(), num(*geometric_softness_2));
            params.insert("choke_2".into(), num(*choke_2));
            params.insert("gray_level_softness_2".into(), num(*gray_level_softness_2));
            params.insert("iterations".into(), num(*iterations));
        }
        // D-352: Refine Hard Matte has no edge radius and never saves one.
        Effect::RefineMatte {
            kind,
            edge_radius,
            view_edge_region,
            feather,
            contrast,
            shift_edge,
            decontaminate,
            decontamination_amount,
            decontamination_radius,
            view_decontamination_map,
        } => {
            if *kind == "soft" {
                params.insert("edge_radius".into(), num(*edge_radius));
                params.insert("view_edge_region".into(), J::from(view_edge_region.as_str()));
            }
            params.insert("feather".into(), num(*feather));
            params.insert("contrast".into(), num(*contrast));
            params.insert("shift_edge".into(), num(*shift_edge));
            params.insert("decontaminate".into(), J::from(decontaminate.as_str()));
            params.insert("decontamination_amount".into(), num(*decontamination_amount));
            params.insert("decontamination_radius".into(), num(*decontamination_radius));
            params.insert("view_decontamination_map".into(), J::from(view_decontamination_map.as_str()));
        }
        // D-356: the paths are found each frame and never saved.
        Effect::Stroke { mask, all_masks, stroke_sequentially, color, brush_size, brush_hardness, opacity, start, end, spacing, paint_style, source, .. } => {
            params.insert("mask".into(), num(*mask));
            params.insert("all_masks".into(), J::from(all_masks.as_str()));
            params.insert("stroke_sequentially".into(), J::from(stroke_sequentially.as_str()));
            params.insert("color".into(), J::from(color.as_str()));
            params.insert("brush_size".into(), num(*brush_size));
            params.insert("brush_hardness".into(), num(*brush_hardness));
            params.insert("opacity".into(), num(*opacity));
            params.insert("start".into(), num(*start));
            params.insert("end".into(), num(*end));
            params.insert("spacing".into(), num(*spacing));
            params.insert("paint_style".into(), J::from(paint_style.as_str()));
            // D-357: as D-315's expand, written only when changed or in the file already.
            if source != "masks" || params.contains_key("source") {
                params.insert("source".into(), J::from(source.as_str()));
            }
        }
        Effect::SoftGlow {
            falloff,
            threshold_mode,
            threshold,
            threshold_smooth,
            saturation_bias,
            radius,
            exposure,
            aspect_ratio,
            aspect_angle,
            operation,
            source_opacity,
            unmult,
        } => {
            params.insert("falloff".into(), J::from(falloff.as_str()));
            params.insert("threshold_mode".into(), J::from(threshold_mode.as_str()));
            params.insert("threshold".into(), num(*threshold));
            params.insert("threshold_smooth".into(), num(*threshold_smooth));
            params.insert("saturation_bias".into(), num(*saturation_bias));
            params.insert("radius".into(), num(*radius));
            params.insert("exposure".into(), num(*exposure));
            params.insert("aspect_ratio".into(), num(*aspect_ratio));
            params.insert("aspect_angle".into(), num(*aspect_angle));
            params.insert("operation".into(), J::from(operation.as_str()));
            params.insert("source_opacity".into(), num(*source_opacity));
            params.insert("unmult".into(), J::from(unmult.as_str()));
        }
        Effect::Unsupported { .. } => {}
    }
    // D-68: a setting with keys is a property record whose base is the plain value just
    // written; a setting without any stays that plain value.
    for (name, track) in &instance.tracks {
        let Some(plain) = params.get(name).cloned() else {
            continue;
        };
        params.insert(name.clone(), track_json(track, plain));
    }
    if !matches!(instance.effect, Effect::Unsupported { .. }) || base.is_some() {
        owned.push(("parameters", J::Object(params)));
    }
    let mut record = merge(base, owned);
    if plain_mix {
        if let Some(map) = record.as_object_mut() {
            map.remove("mix");
        }
    }
    record
}

/// D-68: a keyed setting of one or more channels as one property record, whose base is `plain`
/// and whose keys hold one number, or a list of them where there are several channels.
fn track_json(track: &[Property], plain: J) -> J {
    let mut channels: Vec<J> = track.iter().map(|p| property_json(None, p, 1.0)).collect();
    let mut record = channels.remove(0);
    if let Some(keys) = record["keyframes"].as_array_mut() {
        for (i, key) in keys.iter_mut().enumerate() {
            if !channels.is_empty() {
                let mut all = vec![key["value"].clone()];
                all.extend(channels.iter().map(|c| c["keyframes"][i]["value"].clone()));
                key["value"] = J::Array(all);
            }
        }
    }
    record["base"] = plain;
    record
}

fn composition_json(base: Option<&J>, composition: &Composition) -> J {
    let layers: Vec<J> = composition
        .layers_in_order()
        .map(|layer| layer_json(by_id(base, "layers", layer.id.as_str()), layer))
        .collect();
    let mut rate = Map::new();
    rate.insert(
        "numerator".into(),
        J::from(composition.frame_rate.numerator()),
    );
    rate.insert(
        "denominator".into(),
        J::from(composition.frame_rate.denominator()),
    );
    let mut owned = vec![
        ("id", J::from(composition.id.as_str())),
        ("name", J::from(composition.name.as_str())),
        ("width", J::from(composition.width)),
        ("height", J::from(composition.height)),
        ("pixel_aspect_ratio", J::from(1)),
        ("frame_rate", J::Object(rate)),
        ("start_frame", J::from(composition.start_frame)),
        ("duration_frames", J::from(composition.duration_frames)),
        (
            "layer_order",
            J::Array(
                composition
                    .layer_order()
                    .iter()
                    .map(|id| J::from(id.as_str()))
                    .collect(),
            ),
        ),
        ("layers", J::Array(layers)),
    ];
    // W-24: written only when the model has them, so a file without them stays without them.
    if let Some((start, end)) = composition.work_area {
        let mut area = Map::new();
        area.insert("start_frame".into(), J::from(start));
        area.insert("end_frame_exclusive".into(), J::from(end));
        owned.push(("work_area", J::Object(area)));
    }
    // D-58: written only when the composition names a camera of its own. A file that says
    // nothing is drawn through the default lens the build knows and is written back still
    // saying nothing, so opening and saving a project made before the camera existed does
    // not put a camera into it.
    if let Some(camera) = &composition.camera {
        let held = base.and_then(|b| b.get("camera"));
        let mut props: Vec<(&str, J)> = Vec::new();
        for prop in [
            crate::model::CameraProp::Position,
            crate::model::CameraProp::Depth,
            crate::model::CameraProp::Zoom,
        ] {
            props.push((
                prop.as_str(),
                property_json(
                    held.and_then(|c| c.get(prop.as_str())),
                    camera.get(prop),
                    1.0,
                ),
            ));
        }
        // D-171, written as a layer's parent is: null when cleared, so the old one is not
        // merged back in.
        match &camera.parent {
            Some(parent) => props.push(("parent", J::from(parent.as_str()))),
            None if held.is_some_and(|c| c.get("parent").is_some()) => {
                props.push(("parent", J::Null))
            }
            None => {}
        }
        owned.push(("camera", merge(held, props)));
    }
    if !composition.markers.is_empty() || base.is_some_and(|b| b.get("markers").is_some()) {
        let markers = composition
            .markers
            .iter()
            .map(|m| {
                let mut marker = Map::new();
                marker.insert("frame".into(), J::from(m.frame));
                marker.insert("name".into(), J::from(m.name.as_str()));
                J::Object(marker)
            })
            .collect();
        owned.push(("markers", J::Array(markers)));
    }
    if !composition.sheet_text.is_empty() || base.is_some_and(|b| b.get("sheet_text").is_some()) {
        let columns = composition
            .sheet_text
            .iter()
            .map(|c| {
                let entries: Vec<J> = c
                    .entries
                    .iter()
                    .map(|e| {
                        serde_json::json!({"start_frame": e.start_frame,
                            "end_frame_exclusive": e.end_frame_exclusive, "text": e.text})
                    })
                    .collect();
                serde_json::json!({"kind": c.kind, "name": c.name, "track": c.track,
                    "entries": entries})
            })
            .collect();
        owned.push(("sheet_text", J::Array(columns)));
    }
    // D-84f: written only when one of the four is, or the file's own object holds keys this
    // build does not know, which are kept.
    let details = &composition.sheet_details;
    let four = [
        ("episode", &details.episode),
        ("scene", &details.scene),
        ("cut", &details.cut),
        ("animator", &details.animator),
    ];
    let held = base.and_then(|b| b.get("sheet_details"));
    // D-253: the status is written only when set or when the file had it, so a title block
    // saved before it gains no empty line.
    let status = !details.status.is_empty() || held.is_some_and(|h| h.get("status").is_some());
    let unknown = held
        .and_then(J::as_object)
        .is_some_and(|o| o.keys().any(|k| k != "status" && !four.iter().any(|(name, _)| name == k)));
    let written = four.iter().any(|(_, text)| !text.is_empty()) || !details.status.is_empty();
    // D-254: as a layer's label.
    if composition.label != 0 || base.is_some_and(|b| b.get("label").is_some()) {
        owned.push(("label", J::from(composition.label)));
    }
    let mut merged = merge(base, owned);
    if written || unknown || status {
        let mut lines: Vec<(&str, J)> =
            four.iter().map(|(name, text)| (*name, J::from(text.as_str()))).collect();
        if status {
            lines.push(("status", J::from(details.status.as_str())));
        }
        let object = merge(held, lines);
        merged["sheet_details"] = object;
    } else if let Some(map) = merged.as_object_mut() {
        map.remove("sheet_details");
    }
    // D-188: written only when it differs from the shutter a composition starts with.
    let shutter = composition.motion_blur;
    if shutter == crate::model::MotionBlur::default() {
        if let Some(map) = merged.as_object_mut() {
            map.remove("motion_blur");
        }
    } else {
        merged["motion_blur"] = serde_json::json!({
            "enabled": shutter.enabled,
            "shutter_angle": num(shutter.shutter_angle),
            "shutter_phase": num(shutter.shutter_phase),
            "samples": shutter.samples,
        });
    }
    // D-216: written only when on (FX-FBLEND-024).
    if composition.frame_blending {
        merged["frame_blending"] = J::from(true);
    } else if let Some(map) = merged.as_object_mut() {
        map.remove("frame_blending");
    }
    // D-311: written only when set.
    match composition.background_color {
        Some(c) => merged["background_color"] = J::Array(c.iter().map(|v| num(*v)).collect()),
        None => {
            if let Some(map) = merged.as_object_mut() {
                map.remove("background_color");
            }
        }
    }
    // D-319: written only when on.
    if composition.float_depth {
        merged["float_depth"] = J::from(true);
    } else if let Some(map) = merged.as_object_mut() {
        map.remove("float_depth");
    }
    // D-330: written only when on.
    if composition.eight_bpc {
        merged["eight_bpc"] = J::from(true);
    } else if let Some(map) = merged.as_object_mut() {
        map.remove("eight_bpc");
    }
    // D-333: written only when on.
    if composition.ae_32bpc {
        merged["ae_32bpc"] = J::from(true);
    } else if let Some(map) = merged.as_object_mut() {
        map.remove("ae_32bpc");
    }
    // D-261: written only when there are some or the file had the key.
    if !composition.sketches.is_empty() || base.is_some_and(|b| b.get("sketches").is_some()) {
        merged["sketches"] = J::Array(composition.sketches.iter().map(sketch_json).collect());
    }
    merged
}

/// D-261: a sketch layer, over the lines in it this build does not know.
fn sketch_json(sketch: &crate::model::SketchLayer) -> J {
    let mut map = sketch.rest.clone();
    map.insert("id".into(), J::from(sketch.id.as_str()));
    map.insert("name".into(), J::from(sketch.name.as_str()));
    map.insert("visible".into(), J::from(sketch.visible));
    map.insert("whole_cut".into(), J::from(sketch.whole_cut));
    // D-283: written only when they differ from a plain layer.
    if sketch.opacity != 1.0 {
        map.insert("opacity".into(), num(sketch.opacity));
    }
    if sketch.locked {
        map.insert("locked".into(), J::from(true));
    }
    let strokes = sketch.strokes.iter().map(|stroke| {
        let mut map = stroke.rest.clone();
        if let Some(frame) = stroke.frame {
            map.insert("frame".into(), J::from(frame));
        }
        map.insert("tool".into(), J::from(stroke.tool.as_str()));
        map.insert("size".into(), num(stroke.size));
        map.insert("colour".into(), J::from(stroke.colour.as_str()));
        map.insert(
            "points".into(),
            J::Array(stroke.points.iter().map(|[x, y]| J::Array(vec![num(*x), num(*y)])).collect()),
        );
        if !stroke.pressure.is_empty() {
            map.insert("pressure".into(), J::Array(stroke.pressure.iter().map(|p| num(*p)).collect()));
        }
        // D-286, D-287: written only when on.
        if stroke.pressure_opacity {
            map.insert("pressure_opacity".into(), J::from(true));
        }
        if stroke.filled {
            map.insert("filled".into(), J::from(true));
        }
        J::Object(map)
    });
    map.insert("strokes".into(), J::Array(strokes.collect()));
    J::Object(map)
}

/// D-261: one entry of a composition's `sketches`. Lines it does not know go into `rest`.
fn parse_sketch(v: &J, pointer: &str) -> Result<crate::model::SketchLayer, Diagnostic> {
    let mut rest = as_object(v, pointer)?.clone();
    for key in ["id", "name", "visible", "whole_cut", "opacity", "locked", "strokes"] {
        rest.remove(key);
    }
    // D-283: a plain layer when absent.
    let opacity = match v.get("opacity") {
        None => 1.0,
        Some(o) => match as_f64(o, &format!("{pointer}/opacity"))? {
            o if (0.0..=1.0).contains(&o) => o,
            _ => return Err(invalid(&format!("{pointer}/opacity"), "an opacity from 0 to 1")),
        },
    };
    let locked = v.get("locked").map(|l| as_bool(l, &format!("{pointer}/locked"))).transpose()?.unwrap_or(false);
    let mut sketch = crate::model::SketchLayer {
        id: as_id(field(v, pointer, "id")?, &format!("{pointer}/id"))?,
        name: as_str(field(v, pointer, "name")?, &format!("{pointer}/name"))?.to_string(),
        visible: as_bool(field(v, pointer, "visible")?, &format!("{pointer}/visible"))?,
        whole_cut: as_bool(field(v, pointer, "whole_cut")?, &format!("{pointer}/whole_cut"))?,
        opacity,
        locked,
        strokes: Vec::new(),
        rest,
    };
    let at = format!("{pointer}/strokes");
    for (i, s) in as_array(field(v, pointer, "strokes")?, &at)?.iter().enumerate() {
        let here = format!("{at}/{i}");
        let mut rest = as_object(s, &here)?.clone();
        for key in ["frame", "tool", "size", "colour", "points", "pressure", "pressure_opacity", "filled"] {
            rest.remove(key);
        }
        let mut points = Vec::new();
        let points_at = format!("{here}/points");
        for (j, p) in as_array(field(s, &here, "points")?, &points_at)?.iter().enumerate() {
            let point_at = format!("{points_at}/{j}");
            match as_array(p, &point_at)?.as_slice() {
                [x, y] => points.push([as_f64(x, &point_at)?, as_f64(y, &point_at)?]),
                _ => return Err(invalid(&point_at, "a point, as two numbers")),
            }
        }
        // D-271: one pressure from 0 to 1 per point, when the stroke was drawn with a pen.
        let mut pressure = Vec::new();
        if let Some(p) = s.get("pressure") {
            let pressure_at = format!("{here}/pressure");
            for (j, v) in as_array(p, &pressure_at)?.iter().enumerate() {
                let p = as_f64(v, &format!("{pressure_at}/{j}"))?;
                if !(0.0..=1.0).contains(&p) {
                    return Err(invalid(&format!("{pressure_at}/{j}"), "a pressure from 0 to 1"));
                }
                pressure.push(p);
            }
            if pressure.len() != points.len() {
                return Err(invalid(&pressure_at, "one pressure for each point"));
            }
        }
        sketch.strokes.push(crate::model::Stroke {
            frame: s.get("frame").map(|f| as_i32(f, &format!("{here}/frame"))).transpose()?,
            tool: as_enum(field(s, &here, "tool")?, &format!("{here}/tool"), &crate::model::SKETCH_TOOLS)?.to_string(),
            size: as_f64(field(s, &here, "size")?, &format!("{here}/size"))?,
            colour: as_str(field(s, &here, "colour")?, &format!("{here}/colour"))?.to_string(),
            points,
            pressure,
            pressure_opacity: s.get("pressure_opacity").map(|b| as_bool(b, &format!("{here}/pressure_opacity"))).transpose()?.unwrap_or(false),
            filled: s.get("filled").map(|b| as_bool(b, &format!("{here}/filled"))).transpose()?.unwrap_or(false),
            rest,
        });
    }
    Ok(sketch)
}

/// The project as the text that would be written to disk.
pub fn to_json(project: &Project, preserved: &Preserved) -> String {
    let base = preserved.root_object().map(|_| &preserved.root);
    let assets: Vec<J> = project
        .assets
        .iter()
        .map(|a| asset_json(by_id(base, "assets", a.id.as_str()), a))
        .collect();
    let compositions: Vec<J> = project
        .compositions
        .iter()
        .map(|c| composition_json(by_id(base, "compositions", c.id.as_str()), c))
        .collect();
    let mut colors = base
        .and_then(|b| b.get("color_settings"))
        .and_then(J::as_object)
        .cloned()
        .unwrap_or_else(Map::new);
    colors.insert("working_space".into(), J::from("linear-srgb"));
    colors.insert("alpha_mode".into(), J::from("premultiplied"));
    let root = merge(
        base,
        vec![
            ("schema_version", J::from(SCHEMA_VERSION)),
            ("project_id", J::from(project.id.as_str())),
            ("color_settings", J::Object(colors)),
            ("assets", J::Array(assets)),
            ("compositions", J::Array(compositions)),
        ],
    );
    let mut text = String::new();
    write_value(&mut text, &root, 0);
    text.push('\n');
    text
}

// ---------------------------------------------------------------------------------------
// JSON to model
// ---------------------------------------------------------------------------------------

fn invalid(pointer: &str, expected: &str) -> Diagnostic {
    Diagnostic::new(
        DiagnosticId::ProjectSchemaInvalid,
        Severity::Error,
        "This project file cannot be opened, because part of it does not match the project \
         format.",
        format!("At {pointer}: expected {expected}."),
    )
    .with_remediation(
        "The project was not opened and nothing on disk was changed. The file may have been \
         edited by hand or written by a different program.",
    )
}

fn field<'a>(parent: &'a J, pointer: &str, key: &str) -> Result<&'a J, Diagnostic> {
    parent
        .get(key)
        .ok_or_else(|| invalid(&format!("{pointer}/{key}"), "this field to be present"))
}

fn as_object<'a>(v: &'a J, pointer: &str) -> Result<&'a Map<String, J>, Diagnostic> {
    v.as_object().ok_or_else(|| invalid(pointer, "an object"))
}

fn as_array<'a>(v: &'a J, pointer: &str) -> Result<&'a Vec<J>, Diagnostic> {
    v.as_array().ok_or_else(|| invalid(pointer, "an array"))
}

fn as_str<'a>(v: &'a J, pointer: &str) -> Result<&'a str, Diagnostic> {
    v.as_str().ok_or_else(|| invalid(pointer, "a string"))
}

fn as_id(v: &J, pointer: &str) -> Result<Id, Diagnostic> {
    let text = as_str(v, pointer)?;
    if text.is_empty() {
        return Err(invalid(pointer, "a non-empty identifier"));
    }
    Ok(Id::new(text))
}

fn as_bool(v: &J, pointer: &str) -> Result<bool, Diagnostic> {
    v.as_bool().ok_or_else(|| invalid(pointer, "true or false"))
}

fn as_i32(v: &J, pointer: &str) -> Result<i32, Diagnostic> {
    let n = v.as_i64().ok_or_else(|| invalid(pointer, "an integer"))?;
    i32::try_from(n).map_err(|_| invalid(pointer, "an integer a frame number can hold"))
}

fn as_u32(v: &J, pointer: &str) -> Result<u32, Diagnostic> {
    let n = v
        .as_i64()
        .ok_or_else(|| invalid(pointer, "a whole number that is not negative"))?;
    u32::try_from(n).map_err(|_| invalid(pointer, "a whole number that is not negative"))
}

/// One named number out of an effect's parameter map.
///
/// A missing map or a missing key is a schema fault, not a default: document 19 requires that
/// "effect parameter types match the registered effect schema", and inventing a value for a
/// parameter the file does not carry would render a picture nobody asked for.
fn effect_number(params: Option<&J>, key: &str, at: &str) -> Result<f64, Diagnostic> {
    let params = effect_params(params, at)?;
    let at = format!("{at}/parameters");
    as_f64(field(params, &at, key)?, &format!("{at}/{key}"))
}

/// D-121: a setting added after its effect; a file without it is from before it, and means
/// `start`.
fn effect_number_or(params: Option<&J>, key: &str, at: &str, start: f64) -> Result<f64, Diagnostic> {
    match params.and_then(|p| p.get(key)) {
        Some(_) => effect_number(params, key, at),
        None => Ok(start),
    }
}

/// A setting of `N` numbers: the tint colour, three linear RGB numbers in document 21's order,
/// or D-95's centre, x then y. `what` names it for the fault.
fn effect_array<const N: usize>(
    params: Option<&J>,
    key: &str,
    what: &str,
    at: &str,
) -> Result<[f64; N], Diagnostic> {
    let params = effect_params(params, at)?;
    let at = format!("{at}/parameters/{key}");
    let list = as_array(field(params, &at, key)?, &at)?;
    if list.len() != N {
        return Err(invalid(
            &at,
            &format!("{what}, and this one has {} numbers", list.len()),
        ));
    }
    let mut out = [0.0; N];
    for (i, v) in list.iter().enumerate() {
        out[i] = as_f64(v, &format!("{at}/{i}"))?;
    }
    Ok(out)
}

/// D-87: the chosen colours, a list of strings, kept as written but in small letters. Whether
/// each is a colour is the effect's own check (D-46), not the file's shape.
/// D-89: a setting that is a word, kept as written so a wrong one is reported, not lost.
/// D-109: a blur's `edges`, as written. A file without it is from before it, and means
/// transparent.
fn effect_edges(params: Option<&J>, at: &str) -> Result<String, Diagnostic> {
    match params.and_then(|p| p.get("edges")) {
        Some(_) => effect_word(params, "edges", at),
        None => Ok("transparent".into()),
    }
}

/// D-109: `transparent` is not written, so a file from before it saves the same.
fn put_edges(params: &mut Map<String, J>, edges: &str) {
    if edges == "transparent" {
        params.remove("edges");
    } else {
        params.insert("edges".into(), J::from(edges));
    }
}

fn effect_word(params: Option<&J>, key: &str, at: &str) -> Result<String, Diagnostic> {
    let params = effect_params(params, at)?;
    let at = format!("{at}/parameters/{key}");
    Ok(as_str(field(params, &at, key)?, &at)?.to_string())
}

/// D-299: a word added after its effect, as [`effect_number_or`] is a number.
fn effect_word_or(params: Option<&J>, key: &str, at: &str, start: &str) -> Result<String, Diagnostic> {
    match params.and_then(|p| p.get(key)) {
        Some(_) => effect_word(params, key, at),
        None => Ok(start.to_string()),
    }
}

fn effect_colors(params: Option<&J>, at: &str) -> Result<Vec<String>, Diagnostic> {
    let params = effect_params(params, at)?;
    let at = format!("{at}/parameters/colors");
    as_array(field(params, &at, "colors")?, &at)?
        .iter()
        .enumerate()
        .map(|(i, c)| Ok(as_str(c, &format!("{at}/{i}"))?.to_ascii_lowercase()))
        .collect()
}

/// D-111: a curve, a list of points each a list of numbers. How many of each is the effect's
/// check, so a file's wrong count is kept and reported; anything but numbers is refused.
fn effect_points(params: Option<&J>, key: &str, at: &str) -> Result<Vec<Vec<f64>>, Diagnostic> {
    let params = effect_params(params, at)?;
    let at = format!("{at}/parameters/{key}");
    as_array(field(params, &at, key)?, &at)?
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let at = format!("{at}/{i}");
            as_array(p, &at)?
                .iter()
                .enumerate()
                .map(|(j, v)| as_f64(v, &format!("{at}/{j}")))
                .collect()
        })
        .collect()
}

/// D-130: numbers of any count, so a wrong count is kept and reported, not a refused file.
fn effect_list(params: Option<&J>, key: &str, at: &str) -> Result<Vec<f64>, Diagnostic> {
    let params = effect_params(params, at)?;
    let at = format!("{at}/parameters/{key}");
    as_array(field(params, &at, key)?, &at)?
        .iter()
        .enumerate()
        .map(|(i, v)| as_f64(v, &format!("{at}/{i}")))
        .collect()
}

/// D-68: a setting written as a property record, `{"base", "keyframes"}`, in place of a plain
/// value. Handed back are the parameters with every such record replaced by its base, which is
/// what the readers above take, and the keys of each setting that has any. A colour's record
/// holds three numbers wherever a number's holds one, and is read as three properties of one
/// number that share their frames and eases.
type Tracks = std::collections::BTreeMap<String, Vec<Property>>;
fn effect_tracks(params: Option<&J>, at: &str) -> Result<(Option<J>, Tracks), Diagnostic> {
    let mut tracks = Tracks::new();
    let Some(J::Object(map)) = params else {
        return Ok((None, tracks));
    };
    let mut plain = map.clone();
    // D-332: any setting written as a record is read as one. A list of names kept here fell
    // behind the settings that can be keyed, and a file holding keys on one left out (Fractal
    // Noise's offset, Bulge's vertical radius and twenty more) was saved and then would not
    // open. Keys on a setting the effect does not have are dropped below, as P-17 has it.
    for (name, record) in map.iter().filter(|(_, v)| v.get("keyframes").is_some()) {
        let name = name.as_str();
        let at = format!("{at}/parameters/{name}");
        let (count, what) = match name {
            "color" => (3, "a linear RGB triple"),
            "shadows" | "midtones" | "highlights" => (3, "three numbers, red, green and blue"),
            "reds_hsl" | "yellows_hsl" | "greens_hsl" | "cyans_hsl" | "blues_hsl" | "magentas_hsl" => {
                (3, "three numbers, hue, saturation and lightness")
            }
            "red" | "green" | "blue" => (4, "four numbers, from red, green, blue and a constant"),
            // D-350: a text animator's Start and End are one number each, a gradient's points.
            "start" | "end" if !record.get("base").is_some_and(J::is_array) => (1, "one number"),
            "center" | "start" | "end" | "shift" | "upper_left" | "upper_right" | "lower_left"
            | "lower_right" | "producer_point" | "point_1" | "point_2" | "point_3" | "point_4" | "light"
            | "tile_center" => {
                (2, "two numbers, x then y")
            }
            // D-335: Exposure's offset is one number, Fractal Noise's a point.
            "offset" if record.get("base").is_some_and(J::is_array) => {
                (2, "two numbers, x then y")
            }
            // Any other setting holds as many numbers as its base.
            _ => (record.get("base").and_then(J::as_array).map_or(1, Vec::len), "as many numbers as its base"),
        };
        // D-291: a number setting may carry D-59's expression; a colour or a point takes keys
        // only.
        let expression = match record.get("expression") {
            Some(_) if count > 1 => {
                return Err(invalid(
                    &at,
                    "no expression: a colour or a point takes keys only",
                ))
            }
            Some(e) => Some(parse_expression(e, &format!("{at}/expression"))?),
            None => None,
        };
        let (base, mut track) = channel_track(record, &at, count, what)?;
        plain.insert(name.into(), base);
        track[0].set_expression(expression);
        if track[0].is_animated() || track[0].expression().is_some() {
            tracks.insert(name.into(), track);
        }
    }
    Ok((Some(J::Object(plain)), tracks))
}

/// D-68's record of `count` numbers read as that many one-number properties sharing their frames
/// and eases, with its base as written. `what` says what a value of the wrong count should be.
fn channel_track(record: &J, at: &str, count: usize, what: &str) -> Result<(J, Vec<Property>), Diagnostic> {
    let base = field(record, at, "base")?;
    let keys = as_array(field(record, at, "keyframes")?, &format!("{at}/keyframes"))?;
    let mut track = Vec::new();
    for c in 0..count {
        // One channel's record: the same keys, each holding that channel's number.
        let pick = |v: &J, at: &str| -> Result<J, Diagnostic> {
            if count == 1 {
                return Ok(v.clone());
            }
            match v.as_array() {
                Some(list) if list.len() == count => Ok(list[c].clone()),
                _ => Err(invalid(at, what)),
            }
        };
        let mut channel_keys = Vec::new();
        for (i, key) in keys.iter().enumerate() {
            let at = format!("{at}/keyframes/{i}");
            as_object(key, &at)?;
            let mut channel_key = key.clone();
            channel_key["value"] = pick(field(key, &at, "value")?, &at)?;
            channel_keys.push(channel_key);
        }
        let mut channel = Map::new();
        channel.insert("base".into(), pick(base, &format!("{at}/base"))?);
        channel.insert("keyframes".into(), J::Array(channel_keys));
        track.push(parse_property(&J::Object(channel), at, "scalar", false, false, 1.0)?);
    }
    Ok((base.clone(), track))
}

fn effect_params<'a>(params: Option<&'a J>, at: &str) -> Result<&'a J, Diagnostic> {
    let params =
        params.ok_or_else(|| invalid(&format!("{at}/parameters"), "this field to be present"))?;
    as_object(params, &format!("{at}/parameters"))?;
    Ok(params)
}

fn as_f64(v: &J, pointer: &str) -> Result<f64, Diagnostic> {
    let n = v.as_f64().ok_or_else(|| invalid(pointer, "a number"))?;
    if !n.is_finite() {
        return Err(invalid(pointer, "a finite number"));
    }
    Ok(n)
}

fn as_enum<'a>(v: &'a J, pointer: &str, allowed: &[&str]) -> Result<&'a str, Diagnostic> {
    let text = as_str(v, pointer)?;
    if allowed.contains(&text) {
        Ok(text)
    } else {
        Err(invalid(pointer, &format!("one of {}", allowed.join(", "))))
    }
}

fn parse_value(v: &J, pointer: &str, kind: &str, factor: f64) -> Result<Value, Diagnostic> {
    match kind {
        "scalar" => Ok(Value::Scalar(as_f64(v, pointer)? / factor)),
        _ => {
            let pair = as_array(v, pointer)?;
            if pair.len() != 2 {
                return Err(invalid(pointer, "a pair of numbers"));
            }
            Ok(Value::Vec2(
                as_f64(&pair[0], &format!("{pointer}/0"))? / factor,
                as_f64(&pair[1], &format!("{pointer}/1"))? / factor,
            ))
        }
    }
}

/// `kind` is the value shape document 19 gives the property and `spatial_allowed` whether
/// D-53 lets a keyframe of it carry a motion path. Two arguments rather than a `Prop`,
/// because `Prop` is the layer transform's five and D-58 adds properties that are not
/// among them: a layer's depth, and the camera's place, depth and zoom.
fn parse_property(
    v: &J,
    pointer: &str,
    kind: &'static str,
    spatial_allowed: bool,
    separable: bool,
    factor: f64,
) -> Result<Property, Diagnostic> {
    as_object(v, pointer)?;
    // D-69: `{"x", "y"}` in place of `{"base", "keyframes"}`, on a layer's position only. Each
    // half is a number's property, so path handles, roving or a pair inside one is refused by
    // the same reading that refuses them on a rotation.
    if spatial_allowed && separable && v.get("x").is_some() {
        let half = |name: &str| {
            parse_property(
                field(v, pointer, name)?,
                &format!("{pointer}/{name}"),
                "scalar",
                false,
                false,
                factor,
            )
        };
        let (x, y) = (half("x")?, half("y")?);
        let mut property = Property::constant(Value::Vec2(
            x.base().as_scalar().unwrap_or(0.0),
            y.base().as_scalar().unwrap_or(0.0),
        ));
        property.set_split(Some((x, y)));
        return Ok(property);
    }
    let mut property = Property::constant(parse_value(
        field(v, pointer, "base")?,
        &format!("{pointer}/base"),
        kind,
        factor,
    )?);
    let keys = as_array(
        field(v, pointer, "keyframes")?,
        &format!("{pointer}/keyframes"),
    )?;
    let mut seen: Vec<i32> = Vec::new();
    for (i, key) in keys.iter().enumerate() {
        let at = format!("{pointer}/keyframes/{i}");
        as_object(key, &at)?;
        let frame = as_i32(field(key, &at, "frame")?, &format!("{at}/frame"))?;
        if seen.contains(&frame) {
            return Err(invalid(
                &at,
                &format!("at most one keyframe at frame {frame}"),
            ));
        }
        seen.push(frame);
        let value = parse_value(
            field(key, &at, "value")?,
            &format!("{at}/value"),
            kind,
            factor,
        )?;
        let interp = match as_enum(
            field(key, &at, "interp")?,
            &format!("{at}/interp"),
            &["hold", "linear", "ease"],
        )? {
            "hold" => Interp::Hold,
            "ease" => parse_ease(field(key, &at, "ease")?, &format!("{at}/ease"))?,
            _ => Interp::Linear,
        };
        // D-53: handles on anything but position are diagnosed, not dropped. Document 19 says
        // the field appears only there, so a file carrying one elsewhere is a file that does
        // not say a thing, and document 28's rule is that such a file is refused rather than
        // quietly repaired.
        let spatial = match key.get("spatial") {
            None => None,
            Some(handles) => {
                let at = format!("{at}/spatial");
                if !spatial_allowed {
                    return Err(invalid(
                        &at,
                        "no spatial: the motion path belongs to position keyframes and to no \
                         other property",
                    ));
                }
                Some(four_numbers(handles, &at, "in_x in_y out_x out_y")?)
            }
        };
        let key_kind = match key.get("kind") {
            None => crate::model::Kind::Bezier,
            Some(k) => crate::model::Kind::named(as_enum(
                k,
                &format!("{at}/kind"),
                &["bezier", "continuous", "auto"],
            )?)
            .expect("one of the three"),
        };
        let roving = match key.get("roving") {
            None => false,
            Some(r) => {
                let at = format!("{at}/roving");
                let on = r.as_bool().ok_or_else(|| invalid(&at, "true or false"))?;
                if on && (!spatial_allowed || i == 0 || i + 1 == keys.len()) {
                    return Err(invalid(
                        &at,
                        "no roving: only a position key that is neither first nor last roves",
                    ));
                }
                on
            }
        };
        property.set_keyframe(Keyframe {
            frame,
            value,
            interp,
            spatial,
            kind: key_kind,
            roving,
        });
    }
    if let Some(e) = v.get("expression") {
        property.set_expression(Some(parse_expression(e, &format!("{pointer}/expression"))?));
    }
    Ok(property)
}

/// D-59: exactly a `text` and an `enabled`. The text is kept as written, even one that does not
/// read: that is diagnosed when a frame is evaluated, not by refusing the file.
fn parse_expression(v: &J, pointer: &str) -> Result<Expression, Diagnostic> {
    let m = as_object(v, pointer)?;
    let shape = "an object holding exactly a text and an enabled";
    if m.len() != 2 {
        return Err(invalid(pointer, shape));
    }
    let text = field(v, pointer, "text")?
        .as_str()
        .ok_or_else(|| invalid(&format!("{pointer}/text"), "a string"))?;
    let enabled = field(v, pointer, "enabled")?
        .as_bool()
        .ok_or_else(|| invalid(&format!("{pointer}/enabled"), "true or false"))?;
    Ok(Expression {
        text: text.to_string(),
        enabled,
    })
}

/// Document 19's four numbers, `[x1, y1, x2, y2]`, of a keyframe whose `interp` is `ease`.
///
/// The two `x` are refused outside 0 to 1 rather than clamped. They are positions in time inside
/// the segment, and a handle outside it folds the curve back on itself, which would give one frame
/// two values; document 28's rule is that what cannot be understood is diagnosed and not quietly
/// repaired. The two `y` are deliberately unbounded - a `y` past 0 or 1 is an overshoot, the curve
/// going beyond its destination and coming back, and an animator means that one.
fn parse_ease(v: &J, pointer: &str) -> Result<Interp, Diagnostic> {
    let n = four_numbers(v, pointer, "x1 y1 x2 y2")?;
    for i in [0, 2] {
        if !(0.0..=1.0).contains(&n[i]) {
            return Err(invalid(
                &format!("{pointer}/{i}"),
                "a number from 0 to 1: an ease handle is a position in time inside its own segment",
            ));
        }
    }
    Ok(Interp::Ease {
        x1: n[0],
        y1: n[1],
        x2: n[2],
        y2: n[3],
    })
}

/// An array of exactly four finite numbers, which both `ease` and `spatial` are. `names` is how
/// the diagnostic spells them when the count is wrong.
fn four_numbers(v: &J, pointer: &str, names: &str) -> Result<[f64; 4], Diagnostic> {
    let four = as_array(v, pointer)?;
    if four.len() != 4 {
        return Err(invalid(
            pointer,
            &format!("four numbers, {names}, and this one has {}", four.len()),
        ));
    }
    let mut n = [0.0f64; 4];
    for (i, slot) in n.iter_mut().enumerate() {
        *slot = as_f64(&four[i], &format!("{pointer}/{i}"))?;
    }
    Ok(n)
}

fn parse_interpretation(v: &J, pointer: &str) -> Result<Interpretation, Diagnostic> {
    as_object(v, pointer)?;
    let color_space = match as_enum(
        field(v, pointer, "color_space")?,
        &format!("{pointer}/color_space"),
        &["srgb", "linear-srgb"],
    )? {
        "srgb" => ColorSpace::Srgb,
        _ => ColorSpace::LinearLight,
    };
    let alpha = match as_enum(
        field(v, pointer, "alpha")?,
        &format!("{pointer}/alpha"),
        &["straight", "premultiplied"],
    )? {
        "straight" => AlphaMode::Straight,
        _ => AlphaMode::Premultiplied,
    };
    Ok(Interpretation { color_space, alpha })
}

fn parse_asset(v: &J, pointer: &str) -> Result<Asset, Diagnostic> {
    as_object(v, pointer)?;
    let kind = match as_enum(
        field(v, pointer, "kind")?,
        &format!("{pointer}/kind"),
        &["still", "image_sequence", "audio", "lut"],
    )? {
        "still" => AssetKind::Still,
        "audio" => AssetKind::Audio,
        "lut" => AssetKind::Lut,
        _ => AssetKind::ImageSequence,
    };
    let mut frames = BTreeMap::new();
    if let Some(list) = v.get("frames") {
        let at = format!("{pointer}/frames");
        for (key, file) in as_object(list, &at)? {
            let number: u32 = key
                .parse()
                .map_err(|_| invalid(&format!("{at}/{key}"), "a drawing number as the key"))?;
            frames.insert(number, as_str(file, &format!("{at}/{key}"))?.to_string());
        }
    }
    Ok(Asset {
        id: as_id(field(v, pointer, "id")?, &format!("{pointer}/id"))?,
        kind,
        name: as_str(field(v, pointer, "name")?, &format!("{pointer}/name"))?.to_string(),
        path: match v.get("path") {
            Some(p) => Some(as_str(p, &format!("{pointer}/path"))?.to_string()),
            None => None,
        },
        pattern: match v.get("pattern") {
            Some(p) => Some(as_str(p, &format!("{pointer}/pattern"))?.to_string()),
            None => None,
        },
        frames,
        // D-71: a sound file has no colour to interpret, so none is asked for. The model
        // holds the usual pair and nothing reads it. D-182: nor has a lookup file.
        interpretation: match v.get("interpretation") {
            None if matches!(kind, AssetKind::Audio | AssetKind::Lut) => Interpretation {
                color_space: ColorSpace::Srgb,
                alpha: AlphaMode::Straight,
            },
            _ => parse_interpretation(
                field(v, pointer, "interpretation")?,
                &format!("{pointer}/interpretation"),
            )?,
        },
        redistribute: match v.get("redistribute") {
            Some(r) => as_bool(r, &format!("{pointer}/redistribute"))?,
            None => true,
        },
        label: parse_label(v, pointer)?,
    })
}

/// One of document 19's effect instances, read by the rules a layer's `effects` list is read by.
/// W-31: pasted effects and presets are read here too, so that they obey the file's rules.
fn parse_effect(
    effect: &J,
    at: &str,
    name: &str,
    id: &Id,
    warnings: &mut Vec<Diagnostic>,
) -> Result<crate::effects::EffectInstance, Diagnostic> {
    as_object(effect, &at)?;
    let type_id =
        as_str(field(effect, &at, "type_id")?, &format!("{at}/type_id"))?.to_string();
    let instance_id = as_id(
        field(effect, &at, "instance_id")?,
        &format!("{at}/instance_id"),
    )?;
    let enabled = match effect.get("enabled") {
        None | Some(J::Null) => true,
        Some(b) => as_bool(b, &format!("{at}/enabled"))?,
    };
    // D-202: the Mix, a plain number or a setting's property record; absent, 100.
    let (mix, mix_track) = match effect.get("mix") {
        None => (100.0, None),
        Some(record) if record.is_object() => {
            let at = format!("{at}/mix");
            if record.get("expression").is_some() {
                return Err(invalid(&at, "no expression: the Mix takes keys only"));
            }
            let (base, track) = channel_track(record, &at, 1, "")?;
            let keyed = track[0].is_animated().then_some(track);
            (as_f64(&base, &format!("{at}/base"))?, keyed)
        }
        Some(v) => (as_f64(v, &format!("{at}/mix"))?, None),
    };
    let known = [
        crate::effects::EXPOSURE,
        crate::effects::GAUSSIAN_BLUR,
        crate::effects::TINT,
        crate::effects::LINE_SMOOTH,
        crate::effects::SELECTIVE_COLOR_BLUR,
        crate::effects::GLOW,
        crate::effects::LINE_RECOLOR,
        crate::effects::DIRECTIONAL_BLUR,
        crate::effects::SELECT_COLOR,
        crate::effects::LINE_WIDTH,
        crate::effects::RADIAL_BLUR,
        crate::effects::BLOOM,
        crate::effects::COLOR_KEY,
        crate::effects::CURVES,
        crate::effects::LEVELS,
        crate::effects::HUE_SATURATION,
        crate::effects::GRADIENT,
        crate::effects::DROP_SHADOW,
        crate::effects::LENS_BLUR,
        crate::effects::RIM_LIGHT,
        crate::effects::OUTLINE,
        crate::effects::NOISE,
        crate::effects::CHROMATIC_ABERRATION,
        crate::effects::DISTANCE_GRADATION,
        crate::effects::LIGHT_RAYS,
        crate::effects::EXPOSURE_FLICKER,
        crate::effects::VIGNETTE,
        crate::effects::TURBULENT_DISPLACE,
        crate::effects::FRACTAL_NOISE,
        crate::effects::GRADIENT_MAP,
        crate::effects::COLOR_BALANCE,
        crate::effects::OFFSET,
        crate::effects::LIGHT_WRAP,
        crate::effects::INVERT,
        crate::effects::BRIGHTNESS_CONTRAST,
        crate::effects::BLACK_WHITE,
        crate::effects::POSTERIZE,
        crate::effects::THRESHOLD,
        crate::effects::CHANNEL_MIXER,
        crate::effects::VIBRANCE,
        crate::effects::LEAVE_COLOR,
        crate::effects::SOLARIZE,
        crate::effects::HALFTONE,
        crate::effects::MOSAIC,
        crate::effects::EMBOSS,
        crate::effects::FIND_EDGES,
        crate::effects::SHARPEN,
        crate::effects::DIFFUSION,
        crate::effects::WAVE_WARP,
        crate::effects::RIPPLE,
        crate::effects::TWIRL,
        crate::effects::BULGE,
        crate::effects::MIRROR,
        crate::effects::MOTION_TILE,
        crate::effects::LINEAR_WIPE,
        crate::effects::RADIAL_WIPE,
        crate::effects::VENETIAN_BLINDS,
        crate::effects::IRIS_WIPE,
        crate::effects::SIMPLE_CHOKER,
        crate::effects::SPEED_LINES,
        crate::effects::CROSS_GLARE,
        crate::effects::CAMERA_SHAKE,
        crate::effects::RAIN,
        crate::effects::COLOR_LOOKUP,
        crate::effects::LINE_BLUR,
        crate::effects::HSV_KEY,
        crate::effects::PARAFFIN,
        crate::effects::KIRA_KIRA,
        crate::effects::LIGHTNING_BOLT,
        crate::effects::COMPOUND_BLUR,
        crate::effects::DISPLACEMENT_MAP,
        crate::effects::GRADIENT_WIPE,
        crate::effects::ECHO,
        crate::effects::POSTERIZE_TIME,
        crate::effects::CHANGE_TO_COLOR,
        crate::effects::CORNER_PIN,
        crate::effects::LIGHT_SWEEP,
        crate::effects::RADIO_WAVES,
        crate::effects::POLAR_COORDINATES,
        crate::effects::MEDIAN,
        crate::effects::SMART_BLUR,
        crate::effects::BILATERAL_BLUR,
        crate::effects::SNOWFALL,
        crate::effects::KALEIDOSCOPE,
        crate::effects::ROUGHEN_EDGES,
        crate::effects::BEAM,
        crate::effects::FOUR_COLOR_GRADIENT,
        crate::effects::CELL_PATTERN,
        crate::effects::OPTICS_COMPENSATION,
        crate::effects::RADIAL_SHADOW,
        crate::effects::EXTRACT,
        crate::effects::BEVEL_ALPHA,
        crate::effects::BEVEL_EDGES,
        crate::effects::BLOCK_DISSOLVE,
        crate::effects::SHIFT_CHANNELS,
        crate::effects::SOLID_COMPOSITE,
        crate::effects::CHANNEL_BLUR,
        crate::effects::FAST_BOX_BLUR,
        crate::effects::COLORAMA,
        crate::effects::GLASS,
        crate::effects::VECTOR_BLUR,
        crate::effects::MOMENT_MAP,
        crate::effects::PASS_EXTRACT,
        crate::effects::DEPTH_KEY,
        crate::effects::ID_KEY,
        crate::effects::TEXT_ANIMATOR,
        crate::effects::STRETCH_LEVELS,
        crate::effects::STRETCH_CONTRAST,
        crate::effects::STRETCH_COLOR,
        crate::effects::SPREAD_TONES,
        crate::effects::MATTE_CHOKER,
        crate::effects::REFINE_HARD_MATTE,
        crate::effects::STROKE,
        crate::effects::REFINE_SOFT_MATTE,
    ]
    .contains(&type_id.as_str());
    let (plain, tracks) = if known {
        effect_tracks(effect.get("parameters"), &at)?
    } else {
        (None, std::collections::BTreeMap::new())
    };
    let params = plain.as_ref().or(effect.get("parameters"));
    let parsed = match type_id.as_str() {
        crate::effects::EXPOSURE => Some(crate::effects::Effect::Exposure {
            stops: effect_number(params, "stops", &at)?,
            // D-335: a file from before has no offset, gamma 1 and no bypass.
            offset: effect_number_or(params, "offset", &at, 0.0)?,
            gamma: effect_number_or(params, "gamma", &at, 1.0)?,
            bypass: effect_word_or(params, "bypass", &at, "off")?,
        }),
        crate::effects::GAUSSIAN_BLUR => Some(crate::effects::Effect::GaussianBlur {
            sigma_px: effect_number(params, "sigma_px", &at)?,
            edges: effect_edges(params, &at)?,
            // D-303: a file from before it blurs both ways, as it did.
            dimensions: effect_word_or(params, "dimensions", &at, "both")?,
            // D-321: a file from before it means sigma.
            units: effect_word_or(params, "units", &at, "sigma")?,
        }),
        crate::effects::TINT => Some(crate::effects::Effect::Tint {
            color: effect_array(params, "color", "a linear RGB triple", &at)?,
            amount: effect_number(params, "amount", &at)?,
        }),
        crate::effects::LINE_SMOOTH => Some(crate::effects::Effect::LineSmooth {
            softness: effect_number(params, "softness", &at)?,
            threshold: effect_number(params, "threshold", &at)?,
        }),
        crate::effects::SELECTIVE_COLOR_BLUR => {
            Some(crate::effects::Effect::SelectiveColorBlur {
                blur: effect_number(params, "blur", &at)?,
                colors: effect_colors(params, &at)?,
                // D-88: a file without it is from before it, and means exact.
                tolerance: match params.and_then(|p| p.get("tolerance")) {
                    Some(_) => effect_number(params, "tolerance", &at)?,
                    None => 0.0,
                },
            })
        }
        // D-353: any falloff but classic is Soft Physical Glow, which reports a wrong one.
        crate::effects::GLOW if effect_word_or(params, "falloff", &at, "classic")? != "classic" => {
            Some(crate::effects::Effect::SoftGlow {
                falloff: effect_word(params, "falloff", &at)?,
                threshold_mode: effect_word(params, "threshold_mode", &at)?,
                threshold: effect_number(params, "threshold", &at)?,
                threshold_smooth: effect_number(params, "threshold_smooth", &at)?,
                saturation_bias: effect_number(params, "saturation_bias", &at)?,
                radius: effect_number(params, "radius", &at)?,
                exposure: effect_number(params, "exposure", &at)?,
                aspect_ratio: effect_number(params, "aspect_ratio", &at)?,
                aspect_angle: effect_number(params, "aspect_angle", &at)?,
                operation: effect_word(params, "operation", &at)?,
                source_opacity: effect_number(params, "source_opacity", &at)?,
                unmult: effect_word(params, "unmult", &at)?,
            })
        }
        crate::effects::GLOW => Some(crate::effects::Effect::Glow {
            based_on: effect_word(params, "based_on", &at)?,
            threshold: effect_number(params, "threshold", &at)?,
            colors: effect_colors(params, &at)?,
            tolerance: effect_number(params, "tolerance", &at)?,
            radius: effect_number(params, "radius", &at)?,
            intensity: effect_number(params, "intensity", &at)?,
            operation: effect_word(params, "operation", &at)?,
            // D-89: a colour is read in small letters, as D-87's are.
            tint: effect_word(params, "tint", &at)?.to_ascii_lowercase(),
            // D-322: a file from before it is classic.
            units: effect_word_or(params, "units", &at, "classic")?,
        }),
        crate::effects::LINE_RECOLOR => Some(crate::effects::Effect::LineRecolor {
            colors: effect_colors(params, &at)?,
            tolerance: effect_number(params, "tolerance", &at)?,
            new_color: effect_word(params, "new_color", &at)?.to_ascii_lowercase(),
        }),
        crate::effects::DIRECTIONAL_BLUR => Some(crate::effects::Effect::DirectionalBlur {
            direction: effect_number(params, "direction", &at)?,
            length: effect_number(params, "length", &at)?,
            edges: effect_edges(params, &at)?,
        }),
        // D-93: keep is read as written; "Chosen" is not the word, and is reported.
        crate::effects::SELECT_COLOR => Some(crate::effects::Effect::SelectColor {
            colors: effect_colors(params, &at)?,
            tolerance: effect_number(params, "tolerance", &at)?,
            keep: effect_word(params, "keep", &at)?,
        }),
        crate::effects::LINE_WIDTH => Some(crate::effects::Effect::LineWidth {
            width: effect_number(params, "width", &at)?,
            based_on: effect_word(params, "based_on", &at)?,
            colors: effect_colors(params, &at)?,
            tolerance: effect_number(params, "tolerance", &at)?,
        }),
        crate::effects::RADIAL_BLUR => Some(crate::effects::Effect::RadialBlur {
            kind: effect_word(params, "type", &at)?,
            amount: effect_number(params, "amount", &at)?,
            center: effect_array(params, "center", "two numbers, x then y", &at)?,
            edges: effect_edges(params, &at)?,
        }),
        crate::effects::BLOOM => Some(crate::effects::Effect::Bloom {
            threshold: effect_number(params, "threshold", &at)?,
            radius: effect_number(params, "radius", &at)?,
            intensity: effect_number(params, "intensity", &at)?,
            streaks: effect_word(params, "streaks", &at)?,
            length: effect_number(params, "length", &at)?,
            angle: effect_number(params, "angle", &at)?,
        }),
        // D-97: match is read as written; "RGB" is not the word, and is reported.
        crate::effects::COLOR_KEY => Some(crate::effects::Effect::ColorKey {
            colors: effect_colors(params, &at)?,
            tolerance: effect_number(params, "tolerance", &at)?,
            softness: effect_number(params, "softness", &at)?,
            match_by: effect_word(params, "match", &at)?,
        }),
        // D-111: a point of the wrong count is kept and reported, as a wrong word is.
        crate::effects::CURVES => Some(crate::effects::Effect::Curves {
            master: effect_points(params, "master", &at)?,
            red: effect_points(params, "red", &at)?,
            green: effect_points(params, "green", &at)?,
            blue: effect_points(params, "blue", &at)?,
            // D-302: a file from before it, or one that never bent it, draws as it did.
            alpha: match params.and_then(|p| p.get("alpha")) {
                Some(_) => effect_points(params, "alpha", &at)?,
                None => vec![vec![0.0, 0.0], vec![255.0, 255.0]],
            },
        }),
        crate::effects::LEVELS => Some(crate::effects::Effect::Levels {
            input_black: effect_number(params, "input_black", &at)?,
            input_white: effect_number(params, "input_white", &at)?,
            gamma: effect_number(params, "gamma", &at)?,
            output_black: effect_number(params, "output_black", &at)?,
            output_white: effect_number(params, "output_white", &at)?,
        }),
        crate::effects::HUE_SATURATION => Some(crate::effects::Effect::HueSaturation {
            hue: effect_number(params, "hue", &at)?,
            saturation: effect_number(params, "saturation", &at)?,
            lightness: effect_number(params, "lightness", &at)?,
            // D-307: a file from before the ranges leaves them all at 0, as it drew.
            ranges: {
                let mut ranges = [[0.0; 3]; 6];
                for (r, key) in ranges.iter_mut().zip(crate::effects::HUE_RANGES) {
                    if params.and_then(|p| p.get(key)).is_some() {
                        *r = effect_array(params, key, "three numbers, hue, saturation and lightness", &at)?;
                    }
                }
                ranges
            },
        }),
        // D-114: the colours are read in small letters, as a new colour is.
        crate::effects::GRADIENT => Some(crate::effects::Effect::Gradient {
            shape: effect_word(params, "shape", &at)?,
            start: effect_array(params, "start", "two numbers, x then y", &at)?,
            end: effect_array(params, "end", "two numbers, x then y", &at)?,
            start_color: effect_word(params, "start_color", &at)?.to_ascii_lowercase(),
            end_color: effect_word(params, "end_color", &at)?.to_ascii_lowercase(),
            start_opacity: effect_number(params, "start_opacity", &at)?,
            end_opacity: effect_number(params, "end_opacity", &at)?,
            blend: effect_word(params, "blend", &at)?,
        }),
        crate::effects::DROP_SHADOW => Some(crate::effects::Effect::DropShadow {
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
            opacity: effect_number(params, "opacity", &at)?,
            direction: effect_number(params, "direction", &at)?,
            distance: effect_number(params, "distance", &at)?,
            softness: effect_number(params, "softness", &at)?,
        }),
        crate::effects::LENS_BLUR => Some(crate::effects::Effect::LensBlur {
            radius: effect_number(params, "radius", &at)?,
            edges: effect_word(params, "edges", &at)?,
            iris: match params.and_then(|p| p.get("iris")) {
                Some(_) => effect_word(params, "iris", &at)?,
                None => "circle".into(),
            },
            roundness: effect_number_or(params, "roundness", &at, 0.0)?,
            rotation: effect_number_or(params, "rotation", &at, 0.0)?,
            aspect: effect_number_or(params, "aspect", &at, 1.0)?,
            highlight_gain: effect_number_or(params, "highlight_gain", &at, 0.0)?,
            highlight_threshold: effect_number_or(params, "highlight_threshold", &at, 100.0)?,
            // D-359: the blur map, read at its start values from a file without it; the layer
            // kept as written, as Compound Blur's is.
            layer: params.and_then(|p| p.get("layer")).cloned().unwrap_or_else(|| J::from("")),
            fit: effect_word_or(params, "fit", &at, "center")?,
            channel: effect_word_or(params, "channel", &at, "luminance")?,
            focal_distance: effect_number_or(params, "focal_distance", &at, 0.0)?,
            invert: effect_word_or(params, "invert", &at, "off")?,
            map: None,
        }),
        crate::effects::RIM_LIGHT => Some(crate::effects::Effect::RimLight {
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
            direction: effect_number(params, "direction", &at)?,
            width: effect_number(params, "width", &at)?,
            softness: effect_number(params, "softness", &at)?,
            intensity: effect_number(params, "intensity", &at)?,
            blend: effect_word(params, "blend", &at)?,
        }),
        crate::effects::OUTLINE => Some(crate::effects::Effect::Outline {
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
            width: effect_number(params, "width", &at)?,
            softness: effect_number(params, "softness", &at)?,
            opacity: effect_number(params, "opacity", &at)?,
        }),
        crate::effects::NOISE => Some(crate::effects::Effect::Noise {
            amount: effect_number(params, "amount", &at)?,
            mode: effect_word(params, "mode", &at)?,
            seed: effect_number(params, "seed", &at)?,
            animate: effect_word(params, "animate", &at)?,
            frame: 0,
        }),
        crate::effects::CHROMATIC_ABERRATION => {
            Some(crate::effects::Effect::ChromaticAberration {
                amount: effect_number(params, "amount", &at)?,
                center: effect_array(params, "center", "two numbers, x then y", &at)?,
            })
        }
        // D-123: the colour is read in small letters, as a new colour is.
        crate::effects::DISTANCE_GRADATION => {
            Some(crate::effects::Effect::DistanceGradation {
                color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
                width: effect_number(params, "width", &at)?,
                opacity: effect_number(params, "opacity", &at)?,
                invert: effect_word(params, "invert", &at)?,
                blend: effect_word(params, "blend", &at)?,
            })
        }
        // D-124: the colour is read in small letters, as a new colour is.
        crate::effects::LIGHT_RAYS => Some(crate::effects::Effect::LightRays {
            center: effect_array(params, "center", "two numbers, x then y", &at)?,
            length: effect_number(params, "length", &at)?,
            threshold: effect_number(params, "threshold", &at)?,
            intensity: effect_number(params, "intensity", &at)?,
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
        }),
        crate::effects::EXPOSURE_FLICKER => {
            Some(crate::effects::Effect::ExposureFlicker {
                amount: effect_number(params, "amount", &at)?,
                hold: effect_number(params, "hold", &at)?,
                seed: effect_number(params, "seed", &at)?,
                frame: 0,
            })
        }
        // D-126: the colour is read in small letters, as a new colour is.
        crate::effects::VIGNETTE => Some(crate::effects::Effect::Vignette {
            amount: effect_number(params, "amount", &at)?,
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
            size: effect_number(params, "size", &at)?,
            roundness: effect_number(params, "roundness", &at)?,
            softness: effect_number(params, "softness", &at)?,
            center: effect_array(params, "center", "two numbers, x then y", &at)?,
        }),
        crate::effects::TURBULENT_DISPLACE => {
            Some(crate::effects::Effect::TurbulentDisplace {
                amount: effect_number(params, "amount", &at)?,
                size: effect_number(params, "size", &at)?,
                complexity: effect_number(params, "complexity", &at)?,
                evolution: effect_number(params, "evolution", &at)?,
                speed: effect_number(params, "speed", &at)?,
                seed: effect_number(params, "seed", &at)?,
                edges: effect_word(params, "edges", &at)?,
                frame: 0,
                displacement: effect_word_or(params, "displacement", &at, "turbulent")?,
                pinning: effect_word_or(params, "pinning", &at, "none")?,
                units: effect_word_or(params, "units", &at, "classic")?,
            })
        }
        // D-128: the colours are read in small letters, as a new colour is.
        crate::effects::FRACTAL_NOISE => Some(crate::effects::Effect::FractalNoise {
            size: effect_number(params, "size", &at)?,
            complexity: effect_number(params, "complexity", &at)?,
            contrast: effect_number(params, "contrast", &at)?,
            brightness: effect_number(params, "brightness", &at)?,
            evolution: effect_number(params, "evolution", &at)?,
            speed: effect_number(params, "speed", &at)?,
            seed: effect_number(params, "seed", &at)?,
            dark_color: effect_word(params, "dark_color", &at)?.to_ascii_lowercase(),
            light_color: effect_word(params, "light_color", &at)?.to_ascii_lowercase(),
            opacity: effect_number(params, "opacity", &at)?,
            blend: effect_word(params, "blend", &at)?,
            // D-299: a file from before these, or one that never changed them, draws as it did.
            fractal_type: effect_word_or(params, "fractal_type", &at, "basic")?,
            noise_type: effect_word_or(params, "noise_type", &at, "smooth")?,
            invert: effect_word_or(params, "invert", &at, "off")?,
            offset: match params.and_then(|p| p.get("offset")) {
                Some(_) => effect_array(params, "offset", "two numbers, x then y", &at)?,
                None => [0.0, 0.0],
            },
            scale_width: effect_number_or(params, "scale_width", &at, 100.0)?,
            scale_height: effect_number_or(params, "scale_height", &at, 100.0)?,
            cycle: effect_number_or(params, "cycle", &at, 0.0)?,
            frame: 0,
            float: false,
        }),
        // D-129: the colours are read in small letters, as a new colour is.
        crate::effects::GRADIENT_MAP => Some(crate::effects::Effect::GradientMap {
            shadow_color: effect_word(params, "shadow_color", &at)?.to_ascii_lowercase(),
            midtone_color: effect_word(params, "midtone_color", &at)?
                .to_ascii_lowercase(),
            highlight_color: effect_word(params, "highlight_color", &at)?
                .to_ascii_lowercase(),
            midpoint: effect_number(params, "midpoint", &at)?,
            amount: effect_number(params, "amount", &at)?,
        }),
        crate::effects::COLOR_BALANCE => Some(crate::effects::Effect::ColorBalance {
            shadows: effect_list(params, "shadows", &at)?,
            midtones: effect_list(params, "midtones", &at)?,
            highlights: effect_list(params, "highlights", &at)?,
            // D-295: a file from before Preserve Luminosity is "off", as it drew.
            preserve_luminosity: match params.and_then(|p| p.get("preserve_luminosity")) {
                Some(_) => effect_word(params, "preserve_luminosity", &at)?,
                None => "off".to_string(),
            },
        }),
        crate::effects::OFFSET => Some(crate::effects::Effect::Offset {
            shift: effect_array(params, "shift", "two numbers, x then y", &at)?,
        }),
        crate::effects::LIGHT_WRAP => Some(crate::effects::Effect::LightWrap {
            width: effect_number(params, "width", &at)?,
            intensity: effect_number(params, "intensity", &at)?,
            blend: effect_word(params, "blend", &at)?,
        }),
        crate::effects::INVERT => Some(crate::effects::Effect::Invert {
            channel: effect_word(params, "channel", &at)?,
            amount: effect_number(params, "amount", &at)?,
        }),
        crate::effects::BRIGHTNESS_CONTRAST => Some(crate::effects::Effect::BrightnessContrast {
            brightness: effect_number(params, "brightness", &at)?,
            contrast: effect_number(params, "contrast", &at)?,
        }),
        crate::effects::BLACK_WHITE => Some(crate::effects::Effect::BlackWhite {
            reds: effect_number(params, "reds", &at)?,
            yellows: effect_number(params, "yellows", &at)?,
            greens: effect_number(params, "greens", &at)?,
            cyans: effect_number(params, "cyans", &at)?,
            blues: effect_number(params, "blues", &at)?,
            magentas: effect_number(params, "magentas", &at)?,
        }),
        crate::effects::POSTERIZE => Some(crate::effects::Effect::Posterize {
            levels: effect_number(params, "levels", &at)?,
        }),
        crate::effects::THRESHOLD => Some(crate::effects::Effect::Threshold {
            level: effect_number(params, "level", &at)?,
        }),
        crate::effects::CHANNEL_MIXER => Some(crate::effects::Effect::ChannelMixer {
            red: effect_list(params, "red", &at)?,
            green: effect_list(params, "green", &at)?,
            blue: effect_list(params, "blue", &at)?,
            monochrome: effect_word(params, "monochrome", &at)?,
        }),
        crate::effects::VIBRANCE => Some(crate::effects::Effect::Vibrance {
            vibrance: effect_number(params, "vibrance", &at)?,
            saturation: effect_number(params, "saturation", &at)?,
        }),
        crate::effects::LEAVE_COLOR => Some(crate::effects::Effect::LeaveColor {
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
            tolerance: effect_number(params, "tolerance", &at)?,
            softness: effect_number(params, "softness", &at)?,
            amount: effect_number(params, "amount", &at)?,
        }),
        crate::effects::SOLARIZE => Some(crate::effects::Effect::Solarize {
            threshold: effect_number(params, "threshold", &at)?,
        }),
        crate::effects::HALFTONE => Some(crate::effects::Effect::Halftone {
            size: effect_number(params, "size", &at)?,
            angle: effect_number(params, "angle", &at)?,
            ink: effect_word(params, "ink", &at)?.to_ascii_lowercase(),
            paper: effect_word(params, "paper", &at)?.to_ascii_lowercase(),
            amount: effect_number(params, "amount", &at)?,
        }),
        crate::effects::MOSAIC => Some(crate::effects::Effect::Mosaic {
            size: effect_number(params, "size", &at)?,
        }),
        crate::effects::EMBOSS => Some(crate::effects::Effect::Emboss {
            direction: effect_number(params, "direction", &at)?,
            relief: effect_number(params, "relief", &at)?,
            contrast: effect_number(params, "contrast", &at)?,
            mode: effect_word(params, "mode", &at)?,
        }),
        crate::effects::FIND_EDGES => Some(crate::effects::Effect::FindEdges {
            invert: effect_word(params, "invert", &at)?,
            amount: effect_number(params, "amount", &at)?,
        }),
        crate::effects::SHARPEN => Some(crate::effects::Effect::Sharpen {
            amount: effect_number(params, "amount", &at)?,
            radius: effect_number(params, "radius", &at)?,
            threshold: effect_number_or(params, "threshold", &at, 0.0)?,
        }),
        crate::effects::DIFFUSION => Some(crate::effects::Effect::Diffusion {
            radius: effect_number(params, "radius", &at)?,
            amount: effect_number(params, "amount", &at)?,
            blend: effect_word(params, "blend", &at)?,
        }),
        crate::effects::WAVE_WARP => Some(crate::effects::Effect::WaveWarp {
            shape: effect_word(params, "shape", &at)?,
            height: effect_number(params, "height", &at)?,
            width: effect_number(params, "width", &at)?,
            direction: effect_number(params, "direction", &at)?,
            speed: effect_number(params, "speed", &at)?,
            phase: effect_number(params, "phase", &at)?,
            edges: effect_word(params, "edges", &at)?,
            frame: 0,
        }),
        crate::effects::RIPPLE => Some(crate::effects::Effect::Ripple {
            center: effect_array(params, "center", "two numbers, x then y", &at)?,
            amplitude: effect_number(params, "amplitude", &at)?,
            wavelength: effect_number(params, "wavelength", &at)?,
            speed: effect_number(params, "speed", &at)?,
            phase: effect_number(params, "phase", &at)?,
            fade: effect_number(params, "fade", &at)?,
            frame: 0,
        }),
        crate::effects::TWIRL => Some(crate::effects::Effect::Twirl {
            angle: effect_number(params, "angle", &at)?,
            radius: effect_number(params, "radius", &at)?,
            center: effect_array(params, "center", "two numbers, x then y", &at)?,
        }),
        crate::effects::BULGE => Some(crate::effects::Effect::Bulge {
            center: effect_array(params, "center", "two numbers, x then y", &at)?,
            radius: effect_number(params, "radius", &at)?,
            height: effect_number(params, "height", &at)?,
            vertical_radius: effect_number_or(params, "vertical_radius", &at, 0.0)?,
            taper_radius: effect_number_or(params, "taper_radius", &at, 0.0)?,
        }),
        crate::effects::MIRROR => Some(crate::effects::Effect::Mirror {
            center: effect_array(params, "center", "two numbers, x then y", &at)?,
            angle: effect_number(params, "angle", &at)?,
        }),
        crate::effects::MOTION_TILE => Some(crate::effects::Effect::MotionTile {
            output_width: effect_number(params, "output_width", &at)?,
            output_height: effect_number(params, "output_height", &at)?,
            mirror: effect_word(params, "mirror", &at)?,
            // D-304: a file from before these tiles at the drawing's own size, as it did.
            tile_center: match params.and_then(|p| p.get("tile_center")) {
                Some(_) => effect_array(params, "tile_center", "two numbers, x then y", &at)?,
                None => crate::layer_fx::PLAIN_TILE,
            },
            tile_width: effect_number_or(params, "tile_width", &at, 100.0)?,
            tile_height: effect_number_or(params, "tile_height", &at, 100.0)?,
        }),
        crate::effects::LINEAR_WIPE => Some(crate::effects::Effect::LinearWipe {
            completion: effect_number(params, "completion", &at)?,
            angle: effect_number(params, "angle", &at)?,
            feather: effect_number(params, "feather", &at)?,
        }),
        crate::effects::RADIAL_WIPE => Some(crate::effects::Effect::RadialWipe {
            completion: effect_number(params, "completion", &at)?,
            start_angle: effect_number(params, "start_angle", &at)?,
            center: effect_array(params, "center", "two numbers, x then y", &at)?,
            wipe: effect_word(params, "wipe", &at)?,
            feather: effect_number(params, "feather", &at)?,
        }),
        crate::effects::VENETIAN_BLINDS => Some(crate::effects::Effect::VenetianBlinds {
            completion: effect_number(params, "completion", &at)?,
            angle: effect_number(params, "angle", &at)?,
            width: effect_number(params, "width", &at)?,
            feather: effect_number(params, "feather", &at)?,
        }),
        crate::effects::IRIS_WIPE => Some(crate::effects::Effect::IrisWipe {
            completion: effect_number(params, "completion", &at)?,
            center: effect_array(params, "center", "two numbers, x then y", &at)?,
            feather: effect_number(params, "feather", &at)?,
            invert: effect_word(params, "invert", &at)?,
        }),
        crate::effects::SIMPLE_CHOKER => Some(crate::effects::Effect::SimpleChoker {
            choke: effect_number(params, "choke", &at)?,
        }),
        crate::effects::SPEED_LINES => Some(crate::effects::Effect::SpeedLines {
            center: effect_array(params, "center", "two numbers, x then y", &at)?,
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
            count: effect_number(params, "count", &at)?,
            thickness: effect_number(params, "thickness", &at)?,
            inner: effect_number(params, "inner", &at)?,
            inner_jitter: effect_number(params, "inner_jitter", &at)?,
            angle_jitter: effect_number(params, "angle_jitter", &at)?,
            seed: effect_number(params, "seed", &at)?,
            hold: effect_number(params, "hold", &at)?,
            opacity: effect_number(params, "opacity", &at)?,
            frame: 0,
        }),
        crate::effects::CROSS_GLARE => Some(crate::effects::Effect::CrossGlare {
            threshold: effect_number(params, "threshold", &at)?,
            length: effect_number(params, "length", &at)?,
            points: effect_number(params, "points", &at)?,
            angle: effect_number(params, "angle", &at)?,
            intensity: effect_number(params, "intensity", &at)?,
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
        }),
        crate::effects::CAMERA_SHAKE => Some(crate::effects::Effect::CameraShake {
            amount: effect_number(params, "amount", &at)?,
            rotation: effect_number(params, "rotation", &at)?,
            hold: effect_number(params, "hold", &at)?,
            seed: effect_number(params, "seed", &at)?,
            frame: 0,
        }),
        crate::effects::RAIN => Some(crate::effects::Effect::Rain {
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
            density: effect_number(params, "density", &at)?,
            spacing: effect_number(params, "spacing", &at)?,
            length: effect_number(params, "length", &at)?,
            width: effect_number(params, "width", &at)?,
            direction: effect_number(params, "direction", &at)?,
            speed: effect_number(params, "speed", &at)?,
            seed: effect_number(params, "seed", &at)?,
            opacity: effect_number(params, "opacity", &at)?,
            frame: 0,
        }),
        crate::effects::COLOR_LOOKUP => Some(crate::effects::Effect::ColorLookup {
            lut: effect_word(params, "lut", &at)?,
            table: None,
        }),
        crate::effects::LINE_BLUR => Some(crate::effects::Effect::LineBlur {
            length: effect_number(params, "length", &at)?,
            strength: effect_number(params, "strength", &at)?,
            lines_only: effect_word(params, "lines_only", &at)?,
        }),
        crate::effects::HSV_KEY => Some(crate::effects::Effect::HsvKey {
            hue: effect_number(params, "hue", &at)?,
            saturation: effect_number(params, "saturation", &at)?,
            value: effect_number(params, "value", &at)?,
            hue_range: effect_number(params, "hue_range", &at)?,
            saturation_range: effect_number(params, "saturation_range", &at)?,
            value_range: effect_number(params, "value_range", &at)?,
            invert: effect_word(params, "invert", &at)?,
        }),
        // D-185: the colour is read in small letters, as a new colour is.
        crate::effects::PARAFFIN => Some(crate::effects::Effect::Paraffin {
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
            direction: effect_number(params, "direction", &at)?,
            spread: effect_number(params, "spread", &at)?,
            opacity: effect_number(params, "opacity", &at)?,
            blend: effect_word(params, "blend", &at)?,
        }),
        // D-186: the colour is read in small letters, as a new colour is.
        crate::effects::KIRA_KIRA => Some(crate::effects::Effect::KiraKira {
            threshold: effect_number(params, "threshold", &at)?,
            spacing: effect_number(params, "spacing", &at)?,
            density: effect_number(params, "density", &at)?,
            size: effect_number(params, "size", &at)?,
            shape: effect_word(params, "shape", &at)?,
            angle: effect_number(params, "angle", &at)?,
            twinkle: effect_number(params, "twinkle", &at)?,
            period: effect_number(params, "period", &at)?,
            seed: effect_number(params, "seed", &at)?,
            opacity: effect_number(params, "opacity", &at)?,
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
            frame: 0,
        }),
        // D-190: the colours are read in small letters, as a new colour is.
        crate::effects::LIGHTNING_BOLT => Some(crate::effects::Effect::LightningBolt {
            start: effect_array(params, "start", "two numbers, x then y", &at)?,
            end: effect_array(params, "end", "two numbers, x then y", &at)?,
            jagged: effect_number(params, "jagged", &at)?,
            detail: effect_number(params, "detail", &at)?,
            branches: effect_number(params, "branches", &at)?,
            width: effect_number(params, "width", &at)?,
            glow: effect_number(params, "glow", &at)?,
            opacity: effect_number(params, "opacity", &at)?,
            hold: effect_number(params, "hold", &at)?,
            seed: effect_number(params, "seed", &at)?,
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
            glow_color: effect_word(params, "glow_color", &at)?.to_ascii_lowercase(),
            composite: effect_word_or(params, "composite", &at, "on")?,
            // D-324: a file from before it is D-190's bolt.
            kind: effect_word_or(params, "kind", &at, "direction")?,
            turbulence: effect_number_or(params, "turbulence", &at, 0.0)?,
            decay: effect_number_or(params, "decay", &at, 0.0)?,
            conductivity: effect_number_or(params, "conductivity", &at, 0.0)?,
            obstacle: effect_number_or(params, "obstacle", &at, 0.0)?,
            // D-329: a file from before it stops at an obstacle, as D-324's.
            path: effect_word_or(params, "path", &at, "split")?,
            // D-334: a file from before it has the hard core.
            core: effect_word_or(params, "core", &at, "hard")?,
            // D-338: a file from before it has the short forks.
            forks: effect_word_or(params, "forks", &at, "short")?,
            frame: 0,
        }),
        // D-191: the layer is kept as written, a word or not; a setting check says which.
        crate::effects::COMPOUND_BLUR => Some(crate::effects::Effect::CompoundBlur {
            layer: field(effect_params(params, &at)?, &format!("{at}/parameters"), "layer")?.clone(),
            fit: effect_word(params, "fit", &at)?,
            max_blur: effect_number(params, "max_blur", &at)?,
            invert: effect_word(params, "invert", &at)?,
            edges: effect_word(params, "edges", &at)?,
            map: None,
        }),
        // D-193: the layer is kept as written, as Compound Blur's is.
        crate::effects::DISPLACEMENT_MAP => Some(crate::effects::Effect::DisplacementMap {
            layer: field(effect_params(params, &at)?, &format!("{at}/parameters"), "layer")?.clone(),
            fit: effect_word(params, "fit", &at)?,
            horizontal: effect_word(params, "horizontal", &at)?,
            max_horizontal: effect_number(params, "max_horizontal", &at)?,
            vertical: effect_word(params, "vertical", &at)?,
            max_vertical: effect_number(params, "max_vertical", &at)?,
            wrap: effect_word(params, "wrap", &at)?,
            expand: effect_word_or(params, "expand", &at, "off")?,
            map: None,
        }),
        // D-194: the layer is kept as written, as Compound Blur's is.
        crate::effects::GRADIENT_WIPE => Some(crate::effects::Effect::GradientWipe {
            layer: field(effect_params(params, &at)?, &format!("{at}/parameters"), "layer")?.clone(),
            fit: effect_word(params, "fit", &at)?,
            completion: effect_number(params, "completion", &at)?,
            softness: effect_number(params, "softness", &at)?,
            invert: effect_word(params, "invert", &at)?,
            map: None,
        }),
        crate::effects::ECHO => Some(crate::effects::Effect::Echo {
            echo_time: effect_number(params, "echo_time", &at)?,
            echoes: effect_number(params, "echoes", &at)?,
            intensity: effect_number(params, "intensity", &at)?,
            decay: effect_number(params, "decay", &at)?,
            operator: effect_word(params, "operator", &at)?,
            picture: None,
        }),
        crate::effects::POSTERIZE_TIME => Some(crate::effects::Effect::PosterizeTime {
            frame_rate: effect_number(params, "frame_rate", &at)?,
        }),
        crate::effects::CHANGE_TO_COLOR => Some(crate::effects::Effect::ChangeToColor {
            from: effect_word(params, "from", &at)?.to_ascii_lowercase(),
            to: effect_word(params, "to", &at)?.to_ascii_lowercase(),
            change: effect_word(params, "change", &at)?,
            change_by: effect_word(params, "change_by", &at)?,
            hue_tolerance: effect_number(params, "hue_tolerance", &at)?,
            lightness_tolerance: effect_number(params, "lightness_tolerance", &at)?,
            saturation_tolerance: effect_number(params, "saturation_tolerance", &at)?,
            softness: effect_number(params, "softness", &at)?,
            view_matte: effect_word(params, "view_matte", &at)?,
        }),
        crate::effects::CORNER_PIN => Some(crate::effects::Effect::CornerPin {
            upper_left: effect_array(params, "upper_left", "two numbers, x then y", &at)?,
            upper_right: effect_array(params, "upper_right", "two numbers, x then y", &at)?,
            lower_left: effect_array(params, "lower_left", "two numbers, x then y", &at)?,
            lower_right: effect_array(params, "lower_right", "two numbers, x then y", &at)?,
        }),
        crate::effects::LIGHT_SWEEP => Some(crate::effects::Effect::LightSweep {
            center: effect_array(params, "center", "two numbers, x then y", &at)?,
            direction: effect_number(params, "direction", &at)?,
            shape: effect_word(params, "shape", &at)?,
            width: effect_number(params, "width", &at)?,
            sweep_intensity: effect_number(params, "sweep_intensity", &at)?,
            edge_intensity: effect_number(params, "edge_intensity", &at)?,
            edge_thickness: effect_number(params, "edge_thickness", &at)?,
            light_color: effect_word(params, "light_color", &at)?.to_ascii_lowercase(),
            light_reception: effect_word(params, "light_reception", &at)?,
        }),
        crate::effects::RADIO_WAVES => Some(crate::effects::Effect::RadioWaves {
            producer_point: effect_array(params, "producer_point", "two numbers, x then y", &at)?,
            sides: effect_number(params, "sides", &at)?,
            interval: effect_number(params, "interval", &at)?,
            expansion: effect_number(params, "expansion", &at)?,
            orientation: effect_number(params, "orientation", &at)?,
            direction: effect_number(params, "direction", &at)?,
            velocity: effect_number(params, "velocity", &at)?,
            spin: effect_number(params, "spin", &at)?,
            lifespan: effect_number(params, "lifespan", &at)?,
            opacity: effect_number(params, "opacity", &at)?,
            fade_in_time: effect_number(params, "fade_in_time", &at)?,
            fade_out_time: effect_number(params, "fade_out_time", &at)?,
            start_width: effect_number(params, "start_width", &at)?,
            end_width: effect_number(params, "end_width", &at)?,
            profile: effect_word(params, "profile", &at)?,
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
            frame: 0,
        }),
        crate::effects::POLAR_COORDINATES => Some(crate::effects::Effect::PolarCoordinates {
            interpolation: effect_number(params, "interpolation", &at)?,
            conversion: effect_word(params, "conversion", &at)?,
            shape: effect_word_or(params, "shape", &at, "ellipse")?,
        }),
        crate::effects::MEDIAN => Some(crate::effects::Effect::Median {
            radius: effect_number(params, "radius", &at)?,
            operate_on_alpha: effect_word(params, "operate_on_alpha", &at)?,
        }),
        crate::effects::SMART_BLUR => Some(crate::effects::Effect::SmartBlur {
            radius: effect_number(params, "radius", &at)?,
            threshold: effect_number(params, "threshold", &at)?,
        }),
        crate::effects::BILATERAL_BLUR => Some(crate::effects::Effect::BilateralBlur {
            radius: effect_number(params, "radius", &at)?,
            threshold: effect_number(params, "threshold", &at)?,
            colorize: effect_word(params, "colorize", &at)?,
        }),
        crate::effects::SNOWFALL => Some(crate::effects::Effect::Snowfall {
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
            density: effect_number(params, "density", &at)?,
            spacing: effect_number(params, "spacing", &at)?,
            size: effect_number(params, "size", &at)?,
            depth: effect_number(params, "depth", &at)?,
            speed: effect_number(params, "speed", &at)?,
            wind: effect_number(params, "wind", &at)?,
            wiggle: effect_number(params, "wiggle", &at)?,
            period: effect_number(params, "period", &at)?,
            seed: effect_number(params, "seed", &at)?,
            opacity: effect_number(params, "opacity", &at)?,
            frame: 0,
        }),
        crate::effects::KALEIDOSCOPE => Some(crate::effects::Effect::Kaleidoscope {
            segments: effect_number(params, "segments", &at)?,
            rotation: effect_number(params, "rotation", &at)?,
            size: effect_number(params, "size", &at)?,
            center: effect_array(params, "center", "two numbers, x then y", &at)?,
            mode: effect_word(params, "mode", &at)?,
        }),
        crate::effects::ROUGHEN_EDGES => Some(crate::effects::Effect::RoughenEdges {
            edge_type: effect_word(params, "edge_type", &at)?,
            edge_color: effect_word(params, "edge_color", &at)?.to_ascii_lowercase(),
            border: effect_number(params, "border", &at)?,
            size: effect_number(params, "size", &at)?,
            complexity: effect_number(params, "complexity", &at)?,
            evolution: effect_number(params, "evolution", &at)?,
            speed: effect_number(params, "speed", &at)?,
            seed: effect_number(params, "seed", &at)?,
            frame: 0,
        }),
        crate::effects::BEAM => Some(crate::effects::Effect::Beam {
            start: effect_array(params, "start", "two numbers, x then y", &at)?,
            end: effect_array(params, "end", "two numbers, x then y", &at)?,
            length: effect_number(params, "length", &at)?,
            time: effect_number(params, "time", &at)?,
            start_thickness: effect_number(params, "start_thickness", &at)?,
            end_thickness: effect_number(params, "end_thickness", &at)?,
            softness: effect_number(params, "softness", &at)?,
            inside_color: effect_word(params, "inside_color", &at)?.to_ascii_lowercase(),
            outside_color: effect_word(params, "outside_color", &at)?.to_ascii_lowercase(),
            composite: effect_word(params, "composite", &at)?,
        }),
        crate::effects::FOUR_COLOR_GRADIENT => Some(crate::effects::Effect::FourColorGradient {
            point_1: effect_array(params, "point_1", "two numbers, x then y", &at)?,
            point_2: effect_array(params, "point_2", "two numbers, x then y", &at)?,
            point_3: effect_array(params, "point_3", "two numbers, x then y", &at)?,
            point_4: effect_array(params, "point_4", "two numbers, x then y", &at)?,
            color_1: effect_word(params, "color_1", &at)?.to_ascii_lowercase(),
            color_2: effect_word(params, "color_2", &at)?.to_ascii_lowercase(),
            color_3: effect_word(params, "color_3", &at)?.to_ascii_lowercase(),
            color_4: effect_word(params, "color_4", &at)?.to_ascii_lowercase(),
            blend: effect_number(params, "blend", &at)?,
            opacity: effect_number(params, "opacity", &at)?,
            blending_mode: effect_word(params, "blending_mode", &at)?,
        }),
        crate::effects::CELL_PATTERN => Some(crate::effects::Effect::CellPattern {
            pattern: effect_word(params, "pattern", &at)?,
            invert: effect_word(params, "invert", &at)?,
            contrast: effect_number(params, "contrast", &at)?,
            disperse: effect_number(params, "disperse", &at)?,
            size: effect_number(params, "size", &at)?,
            evolution: effect_number(params, "evolution", &at)?,
            seed: effect_number(params, "seed", &at)?,
            dark_color: effect_word(params, "dark_color", &at)?.to_ascii_lowercase(),
            light_color: effect_word(params, "light_color", &at)?.to_ascii_lowercase(),
            opacity: effect_number(params, "opacity", &at)?,
            blend: effect_word(params, "blend", &at)?,
        }),
        crate::effects::OPTICS_COMPENSATION => Some(crate::effects::Effect::OpticsCompensation {
            field_of_view: effect_number(params, "field_of_view", &at)?,
            reverse: effect_word(params, "reverse", &at)?,
            orientation: effect_word(params, "orientation", &at)?,
            center: effect_array(params, "center", "two numbers, x then y", &at)?,
        }),
        crate::effects::RADIAL_SHADOW => Some(crate::effects::Effect::RadialShadow {
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
            opacity: effect_number(params, "opacity", &at)?,
            light: effect_array(params, "light", "two numbers, x then y", &at)?,
            distance: effect_number(params, "distance", &at)?,
            softness: effect_number(params, "softness", &at)?,
            render: effect_word(params, "render", &at)?,
            color_influence: effect_number(params, "color_influence", &at)?,
            shadow_only: effect_word(params, "shadow_only", &at)?,
        }),
        crate::effects::EXTRACT => Some(crate::effects::Effect::Extract {
            channel: effect_word(params, "channel", &at)?,
            black_point: effect_number(params, "black_point", &at)?,
            white_point: effect_number(params, "white_point", &at)?,
            black_softness: effect_number(params, "black_softness", &at)?,
            white_softness: effect_number(params, "white_softness", &at)?,
            invert: effect_word(params, "invert", &at)?,
        }),
        crate::effects::BEVEL_ALPHA => Some(crate::effects::Effect::BevelAlpha {
            edge_thickness: effect_number(params, "edge_thickness", &at)?,
            light_angle: effect_number(params, "light_angle", &at)?,
            light_color: effect_word(params, "light_color", &at)?.to_ascii_lowercase(),
            light_intensity: effect_number(params, "light_intensity", &at)?,
        }),
        crate::effects::BEVEL_EDGES => Some(crate::effects::Effect::BevelEdges {
            edge_thickness: effect_number(params, "edge_thickness", &at)?,
            light_angle: effect_number(params, "light_angle", &at)?,
            light_color: effect_word(params, "light_color", &at)?.to_ascii_lowercase(),
            light_intensity: effect_number(params, "light_intensity", &at)?,
        }),
        crate::effects::BLOCK_DISSOLVE => Some(crate::effects::Effect::BlockDissolve {
            completion: effect_number(params, "completion", &at)?,
            block_width: effect_number(params, "block_width", &at)?,
            block_height: effect_number(params, "block_height", &at)?,
            feather: effect_number(params, "feather", &at)?,
        }),
        crate::effects::SHIFT_CHANNELS => Some(crate::effects::Effect::ShiftChannels {
            take_alpha: effect_word(params, "take_alpha", &at)?,
            take_red: effect_word(params, "take_red", &at)?,
            take_green: effect_word(params, "take_green", &at)?,
            take_blue: effect_word(params, "take_blue", &at)?,
        }),
        crate::effects::SOLID_COMPOSITE => Some(crate::effects::Effect::SolidComposite {
            source_opacity: effect_number(params, "source_opacity", &at)?,
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
            opacity: effect_number(params, "opacity", &at)?,
            blend: effect_word(params, "blend", &at)?,
        }),
        crate::effects::CHANNEL_BLUR => Some(crate::effects::Effect::ChannelBlur {
            red_blurriness: effect_number(params, "red_blurriness", &at)?,
            green_blurriness: effect_number(params, "green_blurriness", &at)?,
            blue_blurriness: effect_number(params, "blue_blurriness", &at)?,
            alpha_blurriness: effect_number(params, "alpha_blurriness", &at)?,
            edges: effect_edges(params, &at)?,
            dimensions: effect_word(params, "dimensions", &at)?,
        }),
        crate::effects::FAST_BOX_BLUR => Some(crate::effects::Effect::FastBoxBlur {
            radius: effect_number(params, "radius", &at)?,
            iterations: effect_number(params, "iterations", &at)?,
            edges: effect_edges(params, &at)?,
            dimensions: effect_word_or(params, "dimensions", &at, "both")?,
        }),
        // D-316: the layer is kept as written, as Compound Blur's is.
        crate::effects::COLORAMA => Some(crate::effects::Effect::Colorama {
            get_phase: effect_word(params, "get_phase", &at)?,
            layer: field(effect_params(params, &at)?, &format!("{at}/parameters"), "layer")?.clone(),
            fit: effect_word(params, "fit", &at)?,
            phase_shift: effect_number(params, "phase_shift", &at)?,
            cycle_repetitions: effect_number(params, "cycle_repetitions", &at)?,
            stops: effect_number(params, "stops", &at)?,
            color_1: effect_word(params, "color_1", &at)?.to_ascii_lowercase(),
            color_2: effect_word(params, "color_2", &at)?.to_ascii_lowercase(),
            color_3: effect_word(params, "color_3", &at)?.to_ascii_lowercase(),
            color_4: effect_word(params, "color_4", &at)?.to_ascii_lowercase(),
            color_5: effect_word(params, "color_5", &at)?.to_ascii_lowercase(),
            blend_with_original: effect_number(params, "blend_with_original", &at)?,
            map: None,
        }),
        // D-317: the layer is kept as written, as Compound Blur's is.
        crate::effects::GLASS => Some(crate::effects::Effect::Glass {
            layer: field(effect_params(params, &at)?, &format!("{at}/parameters"), "layer")?.clone(),
            fit: effect_word(params, "fit", &at)?,
            property: effect_word(params, "property", &at)?,
            softness: effect_number(params, "softness", &at)?,
            height: effect_number(params, "height", &at)?,
            displacement: effect_number(params, "displacement", &at)?,
            light_angle: effect_number(params, "light_angle", &at)?,
            light_color: effect_word(params, "light_color", &at)?.to_ascii_lowercase(),
            light_intensity: effect_number(params, "light_intensity", &at)?,
            map: None,
        }),
        // D-336: the layer is kept as written, as CC Glass's is.
        crate::effects::VECTOR_BLUR => Some(crate::effects::Effect::VectorBlur {
            kind: effect_word(params, "type", &at)?,
            amount: effect_number(params, "amount", &at)?,
            angle_offset: effect_number(params, "angle_offset", &at)?,
            ridge_smoothness: effect_number(params, "ridge_smoothness", &at)?,
            layer: field(effect_params(params, &at)?, &format!("{at}/parameters"), "layer")?.clone(),
            fit: effect_word(params, "fit", &at)?,
            property: effect_word(params, "property", &at)?,
            map_softness: effect_number(params, "map_softness", &at)?,
            map: None,
        }),
        // D-347: the layer is kept as written, as CC Vector Blur's is.
        crate::effects::MOMENT_MAP => Some(crate::effects::Effect::MomentMap {
            max_time: effect_number(params, "max_time", &at)?,
            resolution: effect_number(params, "resolution", &at)?,
            layer: field(effect_params(params, &at)?, &format!("{at}/parameters"), "layer")?.clone(),
            fit: effect_word(params, "fit", &at)?,
            map: None,
            picture: None,
        }),
        // D-348: the words kept as written, so one outside the contract is refused by name.
        crate::effects::PASS_EXTRACT => Some(crate::effects::Effect::PassExtract {
            pass: effect_word(params, "pass", &at)?,
            black_point: effect_number(params, "black_point", &at)?,
            white_point: effect_number(params, "white_point", &at)?,
            invert: effect_word(params, "invert", &at)?,
            clamp: effect_word(params, "clamp", &at)?,
            channel: effect_word_or(params, "channel", &at, "")?,
            channels: None,
        }),
        crate::effects::DEPTH_KEY => Some(crate::effects::Effect::DepthKey {
            depth: effect_number(params, "depth", &at)?,
            feather: effect_number(params, "feather", &at)?,
            invert: effect_word(params, "invert", &at)?,
            channels: None,
        }),
        crate::effects::ID_KEY => Some(crate::effects::Effect::IdKey {
            aux_channel: effect_word(params, "aux_channel", &at)?,
            id: effect_number(params, "id", &at)?,
            feather: effect_number(params, "feather", &at)?,
            invert: effect_word(params, "invert", &at)?,
            channels: None,
        }),
        // D-350: the words kept as written, so one outside the contract is refused by name.
        crate::effects::TEXT_ANIMATOR => Some(crate::effects::Effect::TextAnimator {
            position: effect_array(params, "position", "two numbers, x then y", &at)?,
            scale: effect_array(params, "scale", "two numbers, x then y", &at)?,
            rotation: effect_number(params, "rotation", &at)?,
            opacity: effect_number(params, "opacity", &at)?,
            fill: effect_word(params, "fill", &at)?,
            color: effect_array(params, "color", "a linear RGB triple", &at)?,
            tracking: effect_number(params, "tracking", &at)?,
            start: effect_number(params, "start", &at)?,
            end: effect_number(params, "end", &at)?,
            offset: effect_number(params, "offset", &at)?,
            amount: effect_number(params, "amount", &at)?,
            based_on: effect_word(params, "based_on", &at)?,
            shape: effect_word(params, "shape", &at)?,
            smoothness: effect_number(params, "smoothness", &at)?,
            ease_high: effect_number(params, "ease_high", &at)?,
            ease_low: effect_number(params, "ease_low", &at)?,
        }),
        crate::effects::STRETCH_LEVELS | crate::effects::STRETCH_CONTRAST | crate::effects::STRETCH_COLOR => {
            let color = type_id == crate::effects::STRETCH_COLOR;
            Some(crate::effects::Effect::AutoTone {
                kind: if color {
                    "color"
                } else if type_id == crate::effects::STRETCH_LEVELS {
                    "levels"
                } else {
                    "contrast"
                },
                temporal_smoothing: effect_number(params, "temporal_smoothing", &at)?,
                scene_detect: effect_word(params, "scene_detect", &at)?,
                black_clip: effect_number(params, "black_clip", &at)?,
                white_clip: effect_number(params, "white_clip", &at)?,
                // Stretch Color's alone; the other two have none and never save one.
                snap_neutral_midtones: if color { effect_word(params, "snap_neutral_midtones", &at)? } else { "off".to_string() },
                stats: None,
            })
        }
        crate::effects::SPREAD_TONES => Some(crate::effects::Effect::SpreadTones {
            equalize: effect_word(params, "equalize", &at)?,
            amount: effect_number(params, "amount", &at)?,
        }),
        crate::effects::MATTE_CHOKER => Some(crate::effects::Effect::MatteChoker {
            geometric_softness_1: effect_number(params, "geometric_softness_1", &at)?,
            choke_1: effect_number(params, "choke_1", &at)?,
            gray_level_softness_1: effect_number(params, "gray_level_softness_1", &at)?,
            geometric_softness_2: effect_number(params, "geometric_softness_2", &at)?,
            choke_2: effect_number(params, "choke_2", &at)?,
            gray_level_softness_2: effect_number(params, "gray_level_softness_2", &at)?,
            iterations: effect_number(params, "iterations", &at)?,
        }),
        crate::effects::REFINE_HARD_MATTE | crate::effects::REFINE_SOFT_MATTE => {
            let soft = type_id == crate::effects::REFINE_SOFT_MATTE;
            Some(crate::effects::Effect::RefineMatte {
                kind: if soft { "soft" } else { "hard" },
                // Refine Soft Matte's alone; Refine Hard Matte has none and never saves one.
                edge_radius: if soft { effect_number(params, "edge_radius", &at)? } else { 0.0 },
                view_edge_region: if soft { effect_word(params, "view_edge_region", &at)? } else { "off".to_string() },
                feather: effect_number(params, "feather", &at)?,
                contrast: effect_number(params, "contrast", &at)?,
                shift_edge: effect_number(params, "shift_edge", &at)?,
                decontaminate: effect_word(params, "decontaminate", &at)?,
                decontamination_amount: effect_number(params, "decontamination_amount", &at)?,
                decontamination_radius: effect_number(params, "decontamination_radius", &at)?,
                view_decontamination_map: effect_word(params, "view_decontamination_map", &at)?,
            })
        }
        crate::effects::STROKE => Some(crate::effects::Effect::Stroke {
            mask: effect_number(params, "mask", &at)?,
            all_masks: effect_word(params, "all_masks", &at)?,
            stroke_sequentially: effect_word(params, "stroke_sequentially", &at)?,
            color: effect_word(params, "color", &at)?.to_ascii_lowercase(),
            brush_size: effect_number(params, "brush_size", &at)?,
            brush_hardness: effect_number(params, "brush_hardness", &at)?,
            opacity: effect_number(params, "opacity", &at)?,
            start: effect_number(params, "start", &at)?,
            end: effect_number(params, "end", &at)?,
            spacing: effect_number(params, "spacing", &at)?,
            paint_style: effect_word(params, "paint_style", &at)?,
            source: effect_word_or(params, "source", &at, "masks")?,
            paths: None,
        }),
        _ => None,
    };
    // P-17: keys on a setting this effect does not have are not its keys. The record
    // stays in the file exactly as written, as any setting it does not have does;
    // taken as keys, saving wrote it back inside itself and the file would not reopen.
    let mut tracks = tracks;
    if let Some(e) = &parsed {
        tracks.retain(|name, _| e.arity(name).is_some());
    }
    if let Some(track) = mix_track {
        tracks.insert("mix".into(), track);
    }
    let effect_value = match parsed {
        Some(e) => {
            // Document 28: a parameter outside its contract is reported, and the record
            // is kept as written. It is not repaired here -- a repaired file would open
            // clean the next time and quietly render something nobody chose.
            let whole = crate::effects::EffectInstance {
                instance_id: instance_id.clone(),
                enabled,
                effect: e.clone(),
                tracks: tracks.clone(),
                mix,
            };
            if let Some(why) = whole.fault() {
                warnings.push(
                    Diagnostic::new(
                        DiagnosticId::EffectParameterInvalid,
                        Severity::Warning,
                        format!(
                            "The layer \"{name}\" has a {type_id} whose settings this \
                             build cannot use."
                        ),
                        format!("{why} The effect is kept and bypassed."),
                    )
                    .with_remediation(
                        "Set the parameter to a value inside its range, or remove the \
                         effect.",
                    ),
                );
            }
            e
        }
        None => {
            warnings.push(
                Diagnostic::new(
                    DiagnosticId::EffectUnsupported,
                    Severity::Warning,
                    format!(
                        "The layer \"{name}\" uses the effect \"{type_id}\", which this \
                         build does not have."
                    ),
                    format!(
                        "Effect instance {instance_id} of type {type_id} on layer {id} is \
                         kept in the project exactly as it was and is bypassed when \
                          rendering."
                    ),
                )
                .with_remediation(
                    "Nothing was lost. Saving this project writes the effect back \
                     unchanged, but any frame rendered here is missing what it would \
                      have done.",
                ),
            );
            crate::effects::Effect::Unsupported { type_id }
        }
    };
    Ok(crate::effects::EffectInstance {
        instance_id,
        enabled,
        effect: effect_value,
        tracks,
        mix,
    })
}

fn parse_layer(v: &J, pointer: &str, warnings: &mut Vec<Diagnostic>) -> Result<Layer, Diagnostic> {
    as_object(v, pointer)?;
    let kind = match as_enum(
        field(v, pointer, "kind")?,
        &format!("{pointer}/kind"),
        &["raster", "adjustment", "composition", "audio", "solid", "shape", "null", "text"],
    )? {
        "text" => LayerKind::Text,
        "null" => LayerKind::Null,
        "solid" => LayerKind::Solid,
        "shape" => LayerKind::Shape,
        "audio" => LayerKind::Audio,
        "adjustment" => LayerKind::Adjustment,
        "composition" => LayerKind::Composition,
        _ => LayerKind::Raster,
    };
    let id = as_id(field(v, pointer, "id")?, &format!("{pointer}/id"))?;
    // D-188: the motion-blur switch, on a layer that draws (FX-MB-039 to 041).
    let motion_blur = match v.get("motion_blur") {
        None => false,
        Some(_) if matches!(kind, LayerKind::Null | LayerKind::Adjustment | LayerKind::Audio) => {
            return Err(invalid(
                &format!("{pointer}/motion_blur"),
                "no motion_blur on a null, adjustment or audio layer, which has no picture of \
                 its own to blur (D-188)",
            ))
        }
        Some(b) => as_bool(b, &format!("{pointer}/motion_blur"))?,
    };
    // D-216: a stretch and Frame Mix on a raster or composition layer, the dissolve on a raster
    // one (FX-FBLEND-050 to 063).
    let timed = matches!(kind, LayerKind::Raster | LayerKind::Composition);
    let only = |key: &str, on: bool, which: &str| -> Result<Option<(&J, String)>, Diagnostic> {
        let at = format!("{pointer}/{key}");
        match v.get(key) {
            None => Ok(None),
            Some(_) if !on => Err(invalid(&at, &format!("no {key} on this kind of layer; only {which} carries one (D-216)"))),
            Some(x) => Ok(Some((x, at))),
        }
    };
    let time_stretch = match only("time_stretch", timed, "a raster or composition layer")? {
        None => 100.0,
        Some((x, at)) => {
            let s = as_f64(x, &at)?;
            if !(1.0..=10000.0).contains(&s) {
                return Err(invalid(&at, "a stretch from 1 to 10000 per cent (D-216)"));
            }
            s
        }
    };
    let frame_blend = match only("frame_blend", timed, "a raster or composition layer")? {
        None => false,
        Some((x, at)) => {
            as_enum(x, &at, &["frame_mix"])?;
            true
        }
    };
    let drawing_dissolve = match only("drawing_dissolve", kind == LayerKind::Raster, "a raster layer")? {
        None => 0,
        Some((x, at)) => match as_u32(x, &at)? {
            d @ 0..=100 => d,
            _ => return Err(invalid(&at, "a whole number of frames from 0 to 100 (D-216)")),
        },
    };
    // D-323: a Time Remap on a raster or composition layer, one number keyed, no expression
    // (FX-TREMAP-050 to 055).
    let time_remap = match only("time_remap", timed, "a raster or composition layer")? {
        None => None,
        Some((x, at)) => {
            let remap = parse_property(x, &at, "scalar", false, false, 1.0)?;
            if remap.expression().is_some() {
                return Err(invalid(&at, "no expression on a Time Remap, which is keys only (D-323)"));
            }
            Some(remap)
        }
    };
    if kind == LayerKind::Audio {
        return parse_audio_layer(v, pointer, id);
    }
    // D-66: an adjustment layer has no drawing. A file that gives one an asset is not a file
    // this build can read faithfully, so it is refused rather than drawn with the asset ignored.
    let asset_id = if kind != LayerKind::Raster {
        if v.get("asset_id").is_some() {
            return Err(invalid(
                &format!("{pointer}/asset_id"),
                "no asset_id on an adjustment, composition, solid, shape or null layer, which \
                 has no drawing of its own (D-66, D-67, D-74, D-78, D-82)",
            ));
        }
        Id::new("")
    } else {
        as_id(
            field(v, pointer, "asset_id")?,
            &format!("{pointer}/asset_id"),
        )?
    };
    // D-67: a composition layer names its composition, and no other kind names one.
    let composition_id = if kind == LayerKind::Composition {
        if v.get("exposure_spans").is_some() {
            return Err(invalid(
                &format!("{pointer}/exposure_spans"),
                "no exposure_spans on a composition layer, which has no drawings (D-67)",
            ));
        }
        Some(as_id(
            field(v, pointer, "composition_id")?,
            &format!("{pointer}/composition_id"),
        )?)
    } else if v.get("composition_id").is_some() {
        return Err(invalid(
            &format!("{pointer}/composition_id"),
            "no composition_id on a layer whose kind is not composition (D-67)",
        ));
    } else {
        None
    };
    // D-74: a solid's drawing is its `solid` record, and it has no exposures or source offset.
    let solid = if kind == LayerKind::Solid {
        for key in ["exposure_spans", "source_offset_frames"] {
            if v.get(key).is_some() {
                return Err(invalid(
                    &format!("{pointer}/{key}"),
                    &format!(
                        "no {key} on a solid layer, which has one drawing of one colour (D-74)"
                    ),
                ));
            }
        }
        Some(parse_solid(
            field(v, pointer, "solid")?,
            &format!("{pointer}/solid"),
        )?)
    } else if v.get("solid").is_some() {
        return Err(invalid(
            &format!("{pointer}/solid"),
            "no solid record on a layer whose kind is not solid (D-74)",
        ));
    } else {
        None
    };
    // D-78: a shape layer's drawing is its `shapes` list, and like a solid it has no exposures
    // and no source offset -- there is no footage behind it to offset into.
    let mut shapes: Vec<crate::shape::Shape> = Vec::new();
    if kind == LayerKind::Shape {
        for key in ["exposure_spans", "source_offset_frames"] {
            if v.get(key).is_some() {
                return Err(invalid(
                    &format!("{pointer}/{key}"),
                    &format!(
                        "no {key} on a shape layer, whose drawing is the shapes it carries (D-78)"
                    ),
                ));
            }
        }
        let at = format!("{pointer}/shapes");
        for (i, s) in as_array(field(v, pointer, "shapes")?, &at)?.iter().enumerate() {
            shapes.push(parse_shape(s, &format!("{at}/{i}"), i)?);
        }
    } else if v.get("shapes").is_some() {
        return Err(invalid(
            &format!("{pointer}/shapes"),
            "no shapes on a layer whose kind is not shape (D-78)",
        ));
    }
    // D-263: a text layer's drawing is its `source_text` record, with no exposures or offset.
    let text = if kind == LayerKind::Text {
        for key in ["exposure_spans", "source_offset_frames"] {
            if v.get(key).is_some() {
                return Err(invalid(
                    &format!("{pointer}/{key}"),
                    &format!("no {key} on a text layer, whose drawing is its words (D-263)"),
                ));
            }
        }
        Some(parse_text(
            field(v, pointer, "source_text")?,
            &format!("{pointer}/source_text"),
        )?)
    } else if v.get("source_text").is_some() {
        return Err(invalid(
            &format!("{pointer}/source_text"),
            "no source_text on a layer whose kind is not text (D-263)",
        ));
    } else {
        None
    };
    // D-82: a null has no drawing at all, so nothing about one.
    if kind == LayerKind::Null {
        for key in ["exposure_spans", "source_offset_frames"] {
            if v.get(key).is_some() {
                return Err(invalid(
                    &format!("{pointer}/{key}"),
                    &format!("no {key} on a null layer, which is never drawn (D-82)"),
                ));
            }
        }
    }
    let source_offset_frames = match v.get("source_offset_frames") {
        None if matches!(
            kind,
            LayerKind::Adjustment
                | LayerKind::Solid
                | LayerKind::Shape
                | LayerKind::Null
                | LayerKind::Text
        ) =>
        {
            0
        }
        _ => as_i32(
            field(v, pointer, "source_offset_frames")?,
            &format!("{pointer}/source_offset_frames"),
        )?,
    };
    let name = as_str(field(v, pointer, "name")?, &format!("{pointer}/name"))?.to_string();
    let in_frame = as_i32(
        field(v, pointer, "in_frame")?,
        &format!("{pointer}/in_frame"),
    )?;
    let out_frame = as_i32(
        field(v, pointer, "out_frame")?,
        &format!("{pointer}/out_frame"),
    )?;
    if in_frame >= out_frame {
        return Err(invalid(
            pointer,
            "in_frame to be before out_frame, which document 19 requires of every layer",
        ));
    }
    let transform_at = format!("{pointer}/transform");
    let transform_json = field(v, pointer, "transform")?;
    as_object(transform_json, &transform_at)?;
    let mut transform = crate::model::Transform::default();
    for prop in [
        Prop::Anchor,
        Prop::Position,
        Prop::Scale,
        Prop::Rotation,
        Prop::Opacity,
    ] {
        let at = format!("{transform_at}/{prop}");
        // D-22: the file stores scale as a percentage and the model as a factor.
        let factor = if prop == Prop::Scale { 100.0 } else { 1.0 };
        // The loop names document 19's five, so this answers for every one of them. The
        // `else` is for a `Prop` a transform does not hold, which is B-13d's depth, and that
        // is read from the layer's own `depth` field further down.
        let Some(slot) = transform.get_mut(prop) else {
            continue;
        };
        *slot = parse_property(
            field(transform_json, &transform_at, prop.as_str())?,
            &at,
            prop.kind(),
            prop == Prop::Position,
            true,
            factor,
        )?;
    }

    let mut exposure_spans = Vec::new();
    if let Some(list) = v.get("exposure_spans") {
        let at = format!("{pointer}/exposure_spans");
        for (i, span) in as_array(list, &at)?.iter().enumerate() {
            let at = format!("{at}/{i}");
            as_object(span, &at)?;
            exposure_spans.push(ExposureSpan {
                start_frame: as_i32(
                    field(span, &at, "start_frame")?,
                    &format!("{at}/start_frame"),
                )?,
                end_frame_exclusive: as_i32(
                    field(span, &at, "end_frame_exclusive")?,
                    &format!("{at}/end_frame_exclusive"),
                )?,
                drawing_number: as_u32(
                    field(span, &at, "drawing_number")?,
                    &format!("{at}/drawing_number"),
                )?,
            });
        }
        // Document 20's invariants: spans are ordered, disjoint and non-empty. Checked by the
        // same code the exposure evaluator uses rather than by a second copy of the rule.
        ExposureMap::new(exposure_spans.clone())
            .map_err(|e| invalid(&at, &format!("exposure spans that {e}")))?;
    }

    // D-57. An unresolved parent is not refused here: document 28 makes it a warning raised
    // once the whole composition is known, beside the matte's, so that the reference survives.
    let parent = match v.get("parent") {
        None | Some(J::Null) => None,
        Some(p) => Some(as_id(p, &format!("{pointer}/parent"))?),
    };

    // D-84. Absent on every layer that was not made from a timesheet column.
    let timesheet = match v.get("timesheet") {
        None | Some(J::Null) => None,
        Some(t) => {
            let at = format!("{pointer}/timesheet");
            let text = |key: &str| {
                t.get(key)
                    .and_then(J::as_str)
                    .map(str::to_string)
                    .ok_or_else(|| {
                        invalid(&format!("{at}/{key}"), "a timesheet record's text (D-84)")
                    })
            };
            Some(crate::model::Timesheet {
                sheet: text("sheet")?,
                column: text("column")?,
                track: t.get("track").and_then(J::as_i64).ok_or_else(|| {
                    invalid(
                        &format!("{at}/track"),
                        "a timesheet column's whole number (D-84)",
                    )
                })?,
            })
        }
    };

    // D-84g. Absent on a layer with no key drawings.
    let key_drawings = match v.get("key_drawings") {
        None => Vec::new(),
        Some(list) => {
            let at = format!("{pointer}/key_drawings");
            as_array(list, &at)?
                .iter()
                .enumerate()
                .map(|(i, n)| as_u32(n, &format!("{at}/{i}")))
                .collect::<Result<Vec<_>, _>>()?
        }
    };

    // D-58. Absent means the layer sits on the depth-0 plane, which is what every project
    // written before the camera existed means and why those files still open unchanged. A
    // property and not a plain number, because a depth is keyed like any other number: a
    // plane that pushes in is an ordinary shot.
    let depth = match v.get("depth") {
        None | Some(J::Null) => None,
        Some(d) => Some(parse_property(
            d,
            &format!("{pointer}/depth"),
            "scalar",
            false,
            false,
            1.0,
        )?),
    };

    let matte = match v.get("matte") {
        None | Some(J::Null) => None,
        Some(m) => {
            let at = format!("{pointer}/matte");
            as_object(m, &at)?;
            let words = MatteMode::ALL.map(MatteMode::as_str);
            let mode = as_enum(field(m, &at, "mode")?, &format!("{at}/mode"), &words)?;
            Some(MatteReference {
                mode: MatteMode::parse(mode).expect("as_enum kept it to the four words"),
                layer_id: as_id(field(m, &at, "layer_id")?, &format!("{at}/layer_id"))?,
                // D-42's flag. Absent means false, so a project written before it existed keeps
                // rendering the way it did: the matte layer stays in the visible stack.
                matte_only: match m.get("matte_only") {
                    None | Some(J::Null) => false,
                    Some(b) => as_bool(b, &format!("{at}/matte_only"))?,
                },
            })
        }
    };

    // B-06 unparked masks, and D-77 made them a list. A file holding the old single `mask` key
    // is read as one mask, mode Add at full opacity with no feather, no expansion and no
    // handles, named "Mask 1" -- and one holding both keys is refused, because there is no
    // honest way to choose between them (FX-MSK-020).
    // A null on either side is not a record, and this build writes `"mask": null` itself to put
    // out the old key, so only two present records are the pair that cannot be read.
    let present = |k: &str| v.get(k).is_some_and(|m| !m.is_null());
    if present("mask") && present("masks") {
        return Err(invalid(
            &format!("{pointer}/masks"),
            "either the old `mask` key or D-77's `masks` list, not both: there is no way to tell \
             which of the two the person drew",
        ));
    }
    let mut masks: Vec<crate::mask::Mask> = Vec::new();
    if let Some(old) = v.get("mask").filter(|m| !m.is_null()) {
        let at = format!("{pointer}/mask");
        as_object(old, &at)?;
        let mut mask = crate::mask::Mask::polygon(parse_vertices(old, &at)?);
        mask.enabled = match old.get("enabled") {
            None => true,
            Some(e) => as_bool(e, &format!("{at}/enabled"))?,
        };
        mask.inverted = match old.get("inverted") {
            None => false,
            Some(e) => as_bool(e, &format!("{at}/inverted"))?,
        };
        masks.push(mask);
    }
    if let Some(list) = v.get("masks").filter(|m| !m.is_null()) {
        let at = format!("{pointer}/masks");
        for (i, m) in as_array(list, &at)?.iter().enumerate() {
            masks.push(parse_mask(m, &format!("{at}/{i}"), i)?);
        }
    }
    // A mask is kept in the model whatever its shape, so that saving writes back what was read.
    // What an unusable one loses is the picture, not the record, and the reason is said out loud
    // here rather than discovered as a blank layer.
    for mask in &masks {
        if !mask.enabled {
            // Nothing to say. A mask switched off is a mask switched off.
        } else if !mask.has_enough_points() && !mask.points.is_empty() {
            warnings.push(
                Diagnostic::new(
                    DiagnosticId::MaskInvalidOutline,
                    Severity::Warning,
                    format!(
                        "The mask \"{}\" on layer \"{name}\" has {} vertices, which is not \
                         enough to enclose anything.",
                        mask.name,
                        mask.points.len()
                    ),
                    format!(
                        "Document 19: a polygon mask is a closed ordered list of vertices, \
                         and fewer than three of them have no interior. The mask on layer \
                         {id} is kept in the project exactly as it was and takes no part in \
                         rendering, so that layer draws without it."
                    ),
                )
                .with_remediation(
                    "Nothing was lost. Saving this project writes the mask back unchanged.",
                ),
            );
        } else if mask.has_enough_points() && !crate::mask::is_simple(&mask.vertices()) {
            warnings.push(
                Diagnostic::new(
                    DiagnosticId::MaskInvalidOutline,
                    Severity::Warning,
                    format!(
                        "The mask \"{}\" on layer \"{name}\" crosses itself, which this \
                         build does not draw.",
                        mask.name
                    ),
                    format!(
                        "Document 19: self-intersection is unsupported in G1 and must be \
                         rejected, never normalized silently, because a normalized polygon \
                         is a different shape from the one that was drawn. The mask on \
                         layer {id} is kept in the project exactly as it was and takes no \
                         part in rendering, so that layer draws without it."
                    ),
                )
                .with_remediation(
                    "Nothing was lost. Saving this project writes the mask back unchanged. \
                     Redraw it so that no edge crosses another to have it drawn.",
                ),
            );
        }
    }

    // D-78: a shape of fewer than two points has nothing to fill and nothing to stroke between.
    // Kept, as an unusable mask is kept, so that saving writes it back -- and said out loud here
    // rather than discovered as a layer that draws nothing. A shape that crosses itself is not
    // this case: the even-odd rule says what it fills, so it is drawn.
    for shape in &shapes {
        if shape.enabled && !shape.has_enough_points() {
            warnings.push(
                Diagnostic::new(
                    DiagnosticId::ShapeInvalidOutline,
                    Severity::Warning,
                    format!(
                        "The shape \"{}\" on layer \"{name}\" has {} point(s), which is not \
                         enough to draw.",
                        shape.name,
                        shape.points.len()
                    ),
                    format!(
                        "D-78: a shape is filled inside its path and stroked along it, and \
                         neither means anything with fewer than two points. The shape on layer \
                         {id} is kept in the project exactly as it was and draws nothing, so \
                         that layer draws without it."
                    ),
                )
                .with_remediation(
                    "Nothing was lost. Saving this project writes the shape back unchanged. \
                     Add points to it to have it drawn.",
                ),
            );
        }
    }

    // Document 19's ordered effect instances. B-07 implements three of them; anything else is
    // read as `Unsupported`, which is a record this build keeps and never draws.
    let mut effects = Vec::new();
    if let Some(effects_json) = v.get("effects") {
        let at = format!("{pointer}/effects");
        for (i, effect) in as_array(effects_json, &at)?.iter().enumerate() {
            effects.push(parse_effect(effect, &format!("{at}/{i}"), &name, &id, warnings)?);
        }
    }

    let blend_mode = BlendMode::from_str(as_enum(
        field(v, pointer, "blend_mode")?,
        &format!("{pointer}/blend_mode"),
        &BlendMode::ALL.map(BlendMode::as_str),
    )?)
    .expect("as_enum allowed only these");
    // D-82: a null has no picture, so nothing that works on one (FX-NULL-025 to 028).
    if kind == LayerKind::Null {
        let carried = [
            ("masks", !masks.is_empty()),
            ("effects", !effects.is_empty()),
            ("matte", matte.is_some()),
            ("blend_mode", blend_mode != BlendMode::Normal),
        ];
        if let Some((key, _)) = carried.iter().find(|(_, has)| *has) {
            return Err(invalid(
                &format!("{pointer}/{key}"),
                "no mask, effect or matte on a null layer, and blend mode normal, because it \
                 is never drawn (D-82)",
            ));
        }
    }

    Ok(Layer {
        id,
        name,
        kind,
        asset_id,
        composition_id,
        enabled: as_bool(field(v, pointer, "enabled")?, &format!("{pointer}/enabled"))?,
        locked: as_bool(field(v, pointer, "locked")?, &format!("{pointer}/locked"))?,
        in_frame,
        out_frame,
        source_offset_frames,
        transform,
        exposure_spans,
        masks,
        matte,
        parent,
        depth,
        effects,
        shy: match v.get("shy") {
            None => false,
            Some(shy) => as_bool(shy, &format!("{pointer}/shy"))?,
        },
        label: parse_label(v, pointer)?,
        blend_mode,
        gain_db: 0.0,
        solid,
        shapes,
        text,
        timesheet,
        key_drawings,
        motion_blur,
        time_stretch,
        frame_blend,
        drawing_dissolve,
        time_remap,
    })
}

/// The old single `mask` key's vertex list, which D-77 reads as a path of corners.
fn parse_vertices(m: &J, at: &str) -> Result<Vec<(f64, f64)>, Diagnostic> {
    let at_v = format!("{at}/vertices");
    let mut vertices = Vec::new();
    for (i, vertex) in as_array(field(m, at, "vertices")?, &at_v)?.iter().enumerate() {
        let at_i = format!("{at_v}/{i}");
        let pair = as_array(vertex, &at_i)?;
        if pair.len() != 2 {
            return Err(invalid(
                &at_i,
                "a vertex that is not a pair of numbers. Document 19: a polygon mask is \
                 an ordered list of vec2 vertices",
            ));
        }
        vertices.push((
            as_f64(&pair[0], &format!("{at_i}/0"))?,
            as_f64(&pair[1], &format!("{at_i}/1"))?,
        ));
    }
    Ok(vertices)
}

/// A pair of numbers: a point, or one of its two handles, which D-53's convention makes an
/// offset in pixels from the point rather than a position.
fn parse_pair(v: &J, at: &str) -> Result<(f64, f64), Diagnostic> {
    let pair = as_array(v, at)?;
    if pair.len() != 2 {
        return Err(invalid(
            at,
            "a pair of numbers: D-77 writes a mask point and each of its two handles as [x, y]",
        ));
    }
    Ok((
        as_f64(&pair[0], &format!("{at}/0"))?,
        as_f64(&pair[1], &format!("{at}/1"))?,
    ))
}

/// The points of one path, which is a `base` and, from B-24d, a key.
fn parse_mask_points(v: &J, at: &str) -> Result<Vec<crate::mask::MaskPoint>, Diagnostic> {
    as_object(v, at)?;
    let at_p = format!("{at}/points");
    let mut points = Vec::new();
    for (i, point) in as_array(field(v, at, "points")?, &at_p)?.iter().enumerate() {
        let at_i = format!("{at_p}/{i}");
        as_object(point, &at_i)?;
        points.push(crate::mask::MaskPoint {
            point: parse_pair(field(point, &at_i, "point")?, &format!("{at_i}/point"))?,
            in_handle: match point.get("in") {
                None => (0.0, 0.0),
                Some(h) => parse_pair(h, &format!("{at_i}/in"))?,
            },
            out_handle: match point.get("out") {
                None => (0.0, 0.0),
                Some(h) => parse_pair(h, &format!("{at_i}/out"))?,
            },
        });
    }
    Ok(points)
}

/// D-77's `path` record: a `base` outline and, from B-24d, its keys in frame order.
///
/// A mask's path and a shape's path are the same record by D-78, so they are the same reader.
/// The rule a key must keep is D-77's: it holds a whole outline of the same length as the base,
/// because a path is interpolated point by point and two outlines of different lengths cannot be
/// paired.
fn parse_path(
    path: &J,
    path_at: &str,
) -> Result<(Vec<crate::mask::MaskPoint>, Vec<crate::mask::MaskKey>), Diagnostic> {
    as_object(path, path_at)?;
    let points = parse_mask_points(field(path, path_at, "base")?, &format!("{path_at}/base"))?;
    let keys_at = format!("{path_at}/keyframes");
    let mut keys: Vec<crate::mask::MaskKey> = Vec::new();
    if let Some(list) = path.get("keyframes").filter(|k| !k.is_null()) {
        for (i, key) in as_array(list, &keys_at)?.iter().enumerate() {
            let at_i = format!("{keys_at}/{i}");
            as_object(key, &at_i)?;
            let held = parse_mask_points(field(key, &at_i, "value")?, &format!("{at_i}/value"))?;
            if held.len() != points.len() {
                return Err(invalid(
                    &at_i,
                    "a key holding the same number of points as the path's base: D-77 \
                     interpolates a path point by point, and there is no honest way to \
                     interpolate between outlines of different lengths",
                ));
            }
            let frame = as_i32(field(key, &at_i, "frame")?, &format!("{at_i}/frame"))?;
            if keys.iter().any(|k| k.frame == frame) {
                return Err(invalid(
                    &at_i,
                    &format!("at most one keyframe at frame {frame}"),
                ));
            }
            let interp = match as_enum(
                field(key, &at_i, "interp")?,
                &format!("{at_i}/interp"),
                &["hold", "linear", "ease"],
            )? {
                "hold" => Interp::Hold,
                "ease" => parse_ease(field(key, &at_i, "ease")?, &format!("{at_i}/ease"))?,
                _ => Interp::Linear,
            };
            keys.push(crate::mask::MaskKey {
                frame,
                points: held,
                interp,
            });
        }
        // Written in order or not, they are held in order: every reader of them, here and in
        // `points_at`, asks which segment a frame is in and nothing else.
        keys.sort_by_key(|k| k.frame);
    }
    Ok((points, keys))
}

/// D-78's shape record: a path that may be open, with an optional fill and an optional stroke.
///
/// Everything outside D-78's ranges refuses the file (FX-SHP-024 to 029), for D-77's reason: a
/// clamped colour or opacity is a picture nobody chose. What is *kept* and diagnosed instead is a
/// path of fewer than two points, which the caller warns about.
fn parse_shape(v: &J, at: &str, index: usize) -> Result<crate::shape::Shape, Diagnostic> {
    as_object(v, at)?;
    let (points, keys) = parse_path(field(v, at, "path")?, &format!("{at}/path"))?;
    // D-170: a colour, an opacity or a width written as a property record is keyed. Its keys are
    // taken out here and its base left in its place, which is what is read below, as an effect's
    // keyed settings are (D-68).
    let mut tracks = std::collections::BTreeMap::new();
    let mut plain = v.clone();
    for paint in ["fill", "stroke"] {
        let Some(J::Object(p)) = plain.get_mut(paint) else {
            continue;
        };
        for (key, name) in [("color", "color"), ("opacity", "opacity"), ("width_px", "width")] {
            let Some(record) = p.get(key).filter(|r| r.is_object()) else {
                continue;
            };
            let at = format!("{at}/{paint}/{key}");
            if record.get("expression").is_some() {
                return Err(invalid(&at, "no expression: a shape's colour, opacity and width take keys only (D-170)"));
            }
            let count = if key == "color" { 3 } else { 1 };
            let (base, track) = channel_track(record, &at, count, "three numbers from 0 to 1 (D-78)")?;
            if track[0].is_animated() {
                tracks.insert(format!("{paint}_{name}"), track);
            }
            p.insert(key.into(), base);
        }
    }
    let v = &plain;
    let (join, miter_limit, cap) = match v.get("stroke").filter(|s| s.is_object()) {
        None => (crate::shape::Join::Round, 4.0, crate::shape::Cap::Round),
        Some(s) => {
            let at = format!("{at}/stroke");
            let word = |key: &str, names: &[&str]| -> Result<Option<String>, Diagnostic> {
                s.get(key)
                    .map(|w| as_enum(w, &format!("{at}/{key}"), names).map(str::to_string))
                    .transpose()
            };
            (
                word("join", &crate::shape::Join::NAMES)?
                    .map_or(crate::shape::Join::Round, |w| crate::shape::Join::named(&w).expect("named")),
                match s.get("miter_limit") {
                    None => 4.0,
                    Some(m) => as_f64(m, &format!("{at}/miter_limit"))?,
                },
                word("cap", &crate::shape::Cap::NAMES)?
                    .map_or(crate::shape::Cap::Round, |w| crate::shape::Cap::named(&w).expect("named")),
            )
        }
    };
    // A colour, an opacity and, for a stroke, a width. Read for both and ranged once afterwards
    // by `Shape::problem`, which is the same sentence the commands refuse with.
    type Paint = ([f64; 3], f64, f64, Option<crate::shape::Gradient>);
    let paint = |key: &str| -> Result<Option<Paint>, Diagnostic> {
        let Some(p) = v.get(key).filter(|p| !p.is_null()) else {
            return Ok(None);
        };
        let at = format!("{at}/{key}");
        as_object(p, &at)?;
        let at_c = format!("{at}/color");
        let color = as_array(field(p, &at, "color")?, &at_c)?;
        if color.len() != 3 {
            return Err(invalid(&at_c, "three numbers from 0 to 1 (D-78)"));
        }
        let mut rgb = [0.0; 3];
        for (i, c) in color.iter().enumerate() {
            rgb[i] = as_f64(c, &format!("{at_c}/{i}"))?;
        }
        let opacity = as_f64(field(p, &at, "opacity")?, &format!("{at}/opacity"))?;
        let width_px = match p.get("width_px") {
            None => 0.0,
            Some(w) => as_f64(w, &format!("{at}/width_px"))?,
        };
        let gradient = match p.get("gradient") {
            None => None,
            Some(g) => Some(parse_gradient(g, &format!("{at}/gradient"))?),
        };
        Ok(Some((rgb, opacity, width_px, gradient)))
    };
    let shape = crate::shape::Shape {
        name: match v.get("name") {
            None => format!("Shape {}", index + 1),
            Some(n) => as_str(n, &format!("{at}/name"))?.to_string(),
        },
        enabled: match v.get("enabled") {
            None => true,
            Some(e) => as_bool(e, &format!("{at}/enabled"))?,
        },
        closed: match v.get("closed") {
            None => true,
            Some(c) => as_bool(c, &format!("{at}/closed"))?,
        },
        points,
        keys,
        fill: paint("fill")?.map(|(color, opacity, _, gradient)| crate::shape::Fill {
            color,
            opacity,
            gradient,
        }),
        stroke: paint("stroke")?.map(|(color, opacity, width_px, gradient)| crate::shape::Stroke {
            color,
            opacity,
            width_px,
            gradient,
            join,
            miter_limit,
            cap,
        }),
        tracks,
        trim: match v.get("trim").filter(|t| !t.is_null()) {
            None => None,
            Some(t) => {
                // D-169: three one-number properties; their ranges are `Shape::problem`'s.
                let at = format!("{at}/trim");
                as_object(t, &at)?;
                let number = |key: &str| {
                    parse_property(field(t, &at, key)?, &format!("{at}/{key}"), "scalar", false, false, 1.0)
                };
                Some(crate::shape::Trim {
                    start: number("start")?,
                    end: number("end")?,
                    offset: number("offset")?,
                })
            }
        },
    };
    match shape.problem() {
        Some(p) => Err(invalid(at, &p)),
        None => Ok(shape),
    }
}

/// D-168's gradient record. Its shape is read here; its ranges, and that its points carry no
/// expression and no motion path, are `Shape::problem`'s, which the commands share.
fn parse_gradient(v: &J, at: &str) -> Result<crate::shape::Gradient, Diagnostic> {
    as_object(v, at)?;
    let kind = as_enum(field(v, at, "type")?, &format!("{at}/type"), &["linear", "radial"])?;
    let point = |key: &str| {
        let here = format!("{at}/{key}");
        parse_property(field(v, at, key)?, &here, "vec2", false, false, 1.0)
    };
    let at_s = format!("{at}/stops");
    let stops = as_array(field(v, at, "stops")?, &at_s)?
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let at = format!("{at_s}/{i}");
            as_object(s, &at)?;
            let at_c = format!("{at}/color");
            let color = as_array(field(s, &at, "color")?, &at_c)?;
            if color.len() != 3 {
                return Err(invalid(&at_c, "three numbers from 0 to 1 (D-168)"));
            }
            let mut rgb = [0.0; 3];
            for (i, c) in color.iter().enumerate() {
                rgb[i] = as_f64(c, &format!("{at_c}/{i}"))?;
            }
            Ok(crate::shape::Stop {
                offset: as_f64(field(s, &at, "offset")?, &format!("{at}/offset"))?,
                color: rgb,
                opacity: as_f64(field(s, &at, "opacity")?, &format!("{at}/opacity"))?,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(crate::shape::Gradient {
        kind: crate::shape::GradientKind::named(kind).expect("one of the two"),
        start: point("start")?,
        end: point("end")?,
        stops,
    })
}

/// D-77's mask record. `index` numbers the unnamed ones, as the window numbers them.
///
/// Everything outside D-77's ranges refuses the file rather than being clamped: a clamped
/// opacity is a picture nobody asked for, and document 28 would rather say no than guess.
/// What is *kept* and diagnosed instead is an outline that cannot be drawn, which the caller
/// warns about.
fn parse_mask(v: &J, at: &str, index: usize) -> Result<crate::mask::Mask, Diagnostic> {
    as_object(v, at)?;
    let mode_at = format!("{at}/mode");
    let mode = match v.get("mode") {
        None => crate::mask::MaskMode::Add,
        Some(m) => match crate::mask::MaskMode::from_str(as_str(m, &mode_at)?) {
            Some(mode) => mode,
            None => {
                return Err(invalid(
                    &mode_at,
                    "one of D-77's mask modes: add, subtract, intersect, difference or none",
                ))
            }
        },
    };
    // D-298: a number written as a property record is keyed. Its keys go to the track and its
    // base is read as the plain number, as a shape's style is (D-170).
    let mut tracks = std::collections::BTreeMap::new();
    let mut number = |key: &str, default: f64| -> Result<f64, Diagnostic> {
        let at = format!("{at}/{key}");
        match v.get(key) {
            None => Ok(default),
            Some(record) if record.is_object() => {
                if record.get("expression").is_some() {
                    return Err(invalid(&at, "no expression: a mask's numbers take keys only (D-298)"));
                }
                let (base, mut track) = channel_track(record, &at, 1, "a number")?;
                if track[0].is_animated() {
                    let name = key.trim_end_matches("_px");
                    tracks.insert(name.to_string(), track.remove(0));
                }
                as_f64(&base, &format!("{at}/base"))
            }
            Some(n) => as_f64(n, &at),
        }
    };
    let opacity = number("opacity", 1.0)?;
    if !(0.0..=1.0).contains(&opacity) {
        return Err(invalid(&format!("{at}/opacity"), "an opacity from 0 to 1 (D-77)"));
    }
    let feather_px = number("feather_px", 0.0)?;
    if !(feather_px >= 0.0) {
        return Err(invalid(
            &format!("{at}/feather_px"),
            "a feather of 0 pixels or more (D-77)",
        ));
    }
    let expansion_px = number("expansion_px", 0.0)?;
    if !(expansion_px.abs() <= crate::mask::MAX_EXPANSION) {
        return Err(invalid(
            &format!("{at}/expansion_px"),
            "an expansion from -8192 to 8192 pixels (D-77)",
        ));
    }
    let (points, keys) = parse_path(field(v, at, "path")?, &format!("{at}/path"))?;
    let name = match v.get("name") {
        None => format!("Mask {}", index + 1),
        Some(n) => as_str(n, &format!("{at}/name"))?.to_string(),
    };
    Ok(crate::mask::Mask {
        keys,
        name,
        enabled: match v.get("enabled") {
            None => true,
            Some(e) => as_bool(e, &format!("{at}/enabled"))?,
        },
        inverted: match v.get("inverted") {
            None => false,
            Some(e) => as_bool(e, &format!("{at}/inverted"))?,
        },
        mode,
        opacity,
        feather_px,
        expansion_px,
        points,
        tracks,
    })
    .and_then(|m| match m.out_of_range() {
        // D-298: the plain numbers were held to D-77's ranges above; this is every key's.
        Some(what) => Err(invalid(at, &format!("a mask inside D-77's ranges on every key, not {what}"))),
        None => Ok(m),
    })
}

/// D-74's `solid` record: a colour of three numbers from 0 to 1, and a whole width and height
/// from 1 to 8192. Anything else refuses the file (FX-SOL-023 to 028).
fn parse_solid(v: &J, pointer: &str) -> Result<crate::model::Solid, Diagnostic> {
    as_object(v, pointer)?;
    let at = format!("{pointer}/color");
    let color = as_array(field(v, pointer, "color")?, &at)?;
    if color.len() != 3 {
        return Err(invalid(&at, "three numbers from 0 to 1 (D-74)"));
    }
    let mut rgb = [0.0; 3];
    for (i, c) in color.iter().enumerate() {
        rgb[i] = as_f64(c, &format!("{at}/{i}"))?;
    }
    let solid = crate::model::Solid {
        color: rgb,
        width: as_u32(field(v, pointer, "width")?, &format!("{pointer}/width"))?,
        height: as_u32(field(v, pointer, "height")?, &format!("{pointer}/height"))?,
    };
    match solid.problem() {
        Some(p) => Err(invalid(pointer, &p)),
        None => Ok(solid),
    }
}

/// D-263's `source_text` record: words, a font's file name, a size, a colour, a place and an
/// alignment, all present and inside `Text::problem`'s ranges, or the file is refused. D-264's
/// settings may follow, each optional and each its default when absent.
fn parse_text(v: &J, pointer: &str) -> Result<crate::text::Text, Diagnostic> {
    as_object(v, pointer)?;
    fn numbers(v: &J, pointer: &str, key: &str, n: usize) -> Result<Vec<f64>, Diagnostic> {
        let at = format!("{pointer}/{key}");
        let list = as_array(field(v, pointer, key)?, &at)?;
        if list.len() != n {
            return Err(invalid(&at, &format!("{n} numbers (D-263)")));
        }
        list.iter().enumerate().map(|(i, x)| as_f64(x, &format!("{at}/{i}"))).collect()
    }
    let colour = |v: &J, pointer: &str| numbers(v, pointer, "color", 3).map(|c| [c[0], c[1], c[2]]);
    let number = |v: &J, pointer: &str, key: &str| as_f64(field(v, pointer, key)?, &format!("{pointer}/{key}"));
    let (color, at) = (numbers(v, pointer, "color", 3)?, numbers(v, pointer, "at", 2)?);
    let optional = |key: &str, default: f64| v.get(key).map_or(Ok(default), |x| as_f64(x, &format!("{pointer}/{key}")));
    let flag = |key: &str| v.get(key).map_or(Ok(false), |x| as_bool(x, &format!("{pointer}/{key}")));
    let record = |key: &str| -> Result<Option<(&J, String)>, Diagnostic> {
        match v.get(key) {
            None => Ok(None),
            Some(r) => {
                let at = format!("{pointer}/{key}");
                as_object(r, &at)?;
                Ok(Some((r, at)))
            }
        }
    };
    let stroke = match record("stroke")? {
        None => None,
        Some((r, at)) => Some(crate::text::Stroke { color: colour(r, &at)?, width: number(r, &at, "width")? }),
    };
    let background = match record("background")? {
        None => None,
        Some((r, at)) => Some(crate::text::Background {
            color: colour(r, &at)?,
            opacity: number(r, &at, "opacity")?,
            padding: number(r, &at, "padding")?,
            roundness: number(r, &at, "roundness")?,
        }),
    };
    let shadow = match record("shadow")? {
        None => None,
        Some((r, at)) => Some(crate::text::Shadow {
            color: colour(r, &at)?,
            opacity: number(r, &at, "opacity")?,
            angle: number(r, &at, "angle")?,
            distance: number(r, &at, "distance")?,
            softness: number(r, &at, "softness")?,
        }),
    };
    let align_at = format!("{pointer}/align");
    let text = crate::text::Text {
        text: as_str(field(v, pointer, "text")?, &format!("{pointer}/text"))?.to_string(),
        font: as_str(field(v, pointer, "font")?, &format!("{pointer}/font"))?.to_string(),
        size: as_f64(field(v, pointer, "size")?, &format!("{pointer}/size"))?,
        color: [color[0], color[1], color[2]],
        at: [at[0], at[1]],
        align: crate::text::Align::parse(as_enum(
            field(v, pointer, "align")?,
            &align_at,
            &["left", "center", "right", "justify"],
        )?)
        .expect("as_enum allows only the four"),
        tracking: optional("tracking", 0.0)?,
        leading: optional("leading", 0.0)?,
        kerning: flag("kerning")?,
        all_caps: flag("all_caps")?,
        faux_bold: flag("faux_bold")?,
        faux_italic: flag("faux_italic")?,
        box_width: optional("box_width", 0.0)?,
        stroke,
        background,
        shadow,
    };
    match text.problem() {
        Some(p) => Err(invalid(pointer, &p)),
        None => Ok(text),
    }
}

fn parse_label(v: &J, pointer: &str) -> Result<u8, Diagnostic> {
    match v.get("label") {
        None => Ok(0),
        Some(label) => match as_u32(label, &format!("{pointer}/label"))? {
            n @ 0..=8 => Ok(n as u8),
            _ => Err(invalid(
                &format!("{pointer}/label"),
                "a label colour from 0, none, to 8",
            )),
        },
    }
}

/// D-71: an audio layer is heard and not seen. A file that gives it anything about a picture
/// is not a file this build can read faithfully, so it is refused (FX-AUD-030 to 035).
/// D-188: a composition's shutter, all four fields present and in range (FX-MB-030 to 038).
fn parse_motion_blur(v: &J, pointer: &str) -> Result<crate::model::MotionBlur, Diagnostic> {
    const FIELDS: [&str; 4] = ["enabled", "shutter_angle", "shutter_phase", "samples"];
    if let Some(unknown) = as_object(v, pointer)?.keys().find(|k| !FIELDS.contains(&k.as_str())) {
        return Err(invalid(
            &format!("{pointer}/{unknown}"),
            "only enabled, shutter_angle, shutter_phase and samples (D-188)",
        ));
    }
    let number = |key: &str, low: f64, high: f64| {
        let at = format!("{pointer}/{key}");
        let n = as_f64(field(v, pointer, key)?, &at)?;
        if (low..=high).contains(&n) {
            Ok(n)
        } else {
            Err(invalid(&at, &format!("a number from {low} to {high} (D-188)")))
        }
    };
    let at = format!("{pointer}/samples");
    let samples = as_u32(field(v, pointer, "samples")?, &at)?;
    if !(2..=64).contains(&samples) {
        return Err(invalid(&at, "a whole number of samples from 2 to 64 (D-188)"));
    }
    Ok(crate::model::MotionBlur {
        enabled: as_bool(field(v, pointer, "enabled")?, &format!("{pointer}/enabled"))?,
        shutter_angle: number("shutter_angle", 0.0, 720.0)?,
        shutter_phase: number("shutter_phase", -360.0, 360.0)?,
        samples,
    })
}

fn parse_audio_layer(v: &J, pointer: &str, id: Id) -> Result<Layer, Diagnostic> {
    for key in [
        "transform",
        "exposure_spans",
        "mask",
        "masks",
        "matte",
        "blend_mode",
        "effects",
        "parent",
        "depth",
        "composition_id",
        "shapes",
    ] {
        if v.get(key).is_some() {
            return Err(invalid(
                &format!("{pointer}/{key}"),
                &format!("no {key} on an audio layer, which draws nothing (D-71)"),
            ));
        }
    }
    let in_frame = as_i32(
        field(v, pointer, "in_frame")?,
        &format!("{pointer}/in_frame"),
    )?;
    let out_frame = as_i32(
        field(v, pointer, "out_frame")?,
        &format!("{pointer}/out_frame"),
    )?;
    if in_frame >= out_frame {
        return Err(invalid(
            pointer,
            "in_frame to be before out_frame, which document 19 requires of every layer",
        ));
    }
    let gain_db = match v.get("gain_db") {
        None => 0.0,
        Some(g) => as_f64(g, &format!("{pointer}/gain_db"))?,
    };
    if !(-96.0..=12.0).contains(&gain_db) {
        return Err(invalid(
            &format!("{pointer}/gain_db"),
            "a level from -96 to +12 decibels (D-71)",
        ));
    }
    let mut layer = Layer::audio(
        id,
        as_str(field(v, pointer, "name")?, &format!("{pointer}/name"))?,
        as_id(
            field(v, pointer, "asset_id")?,
            &format!("{pointer}/asset_id"),
        )?,
        in_frame,
        out_frame,
    );
    layer.enabled = as_bool(field(v, pointer, "enabled")?, &format!("{pointer}/enabled"))?;
    layer.locked = as_bool(field(v, pointer, "locked")?, &format!("{pointer}/locked"))?;
    layer.source_offset_frames = as_i32(
        field(v, pointer, "source_offset_frames")?,
        &format!("{pointer}/source_offset_frames"),
    )?;
    layer.gain_db = gain_db;
    if let Some(shy) = v.get("shy") {
        layer.shy = as_bool(shy, &format!("{pointer}/shy"))?;
    }
    layer.label = parse_label(v, pointer)?;
    Ok(layer)
}

fn parse_composition(
    v: &J,
    pointer: &str,
    warnings: &mut Vec<Diagnostic>,
) -> Result<Composition, Diagnostic> {
    as_object(v, pointer)?;
    let ratio = field(v, pointer, "pixel_aspect_ratio")?;
    if as_f64(ratio, &format!("{pointer}/pixel_aspect_ratio"))? != 1.0 {
        return Err(invalid(
            &format!("{pointer}/pixel_aspect_ratio"),
            "1, because document 07 says G1 supports square pixels only and rejects \
             unsupported ratios explicitly rather than approximating them",
        ));
    }
    let rate_at = format!("{pointer}/frame_rate");
    let rate_json = field(v, pointer, "frame_rate")?;
    as_object(rate_json, &rate_at)?;
    let rate = FrameRate::new(
        as_u32(
            field(rate_json, &rate_at, "numerator")?,
            &format!("{rate_at}/numerator"),
        )?,
        as_u32(
            field(rate_json, &rate_at, "denominator")?,
            &format!("{rate_at}/denominator"),
        )?,
    )
    .map_err(|e| invalid(&rate_at, &format!("a frame rate that {e}")))?;

    let width = as_u32(field(v, pointer, "width")?, &format!("{pointer}/width"))?;
    let height = as_u32(field(v, pointer, "height")?, &format!("{pointer}/height"))?;
    if width == 0 || height == 0 {
        return Err(invalid(pointer, "a width and height of at least one pixel"));
    }
    // P-25: the limits a new or changed composition is held to (B-12d), held to on loading too.
    // A file is not a way round them: a size past them was a frame buffer too big to allocate,
    // which ended the program rather than saying why.
    use crate::command::{LARGEST_PIXELS, LARGEST_SIDE, LONGEST_COMPOSITION};
    if width > LARGEST_SIDE || height > LARGEST_SIDE || width as u64 * height as u64 > LARGEST_PIXELS {
        return Err(invalid(
            pointer,
            &format!(
                "a size this build will make, no side past {LARGEST_SIDE} and no more than \
                 {LARGEST_PIXELS} pixels in all; it is {width}x{height}"
            ),
        ));
    }
    let duration = as_u32(
        field(v, pointer, "duration_frames")?,
        &format!("{pointer}/duration_frames"),
    )?;
    if duration == 0 {
        return Err(invalid(
            &format!("{pointer}/duration_frames"),
            "a duration of at least one frame",
        ));
    }
    if duration > LONGEST_COMPOSITION {
        return Err(invalid(
            &format!("{pointer}/duration_frames"),
            &format!("a length of no more than {LONGEST_COMPOSITION} frames; it is {duration}"),
        ));
    }
    let mut composition = Composition::new(
        as_id(field(v, pointer, "id")?, &format!("{pointer}/id"))?,
        as_str(field(v, pointer, "name")?, &format!("{pointer}/name"))?,
        width,
        height,
        rate,
        as_i32(
            field(v, pointer, "start_frame")?,
            &format!("{pointer}/start_frame"),
        )?,
        duration,
    );

    // W-24: document 19's work area, which until now rode along unread. One outside its
    // composition or of no frames has no answer for what to play, so it is refused like a
    // duration of zero rather than quietly widened.
    if let Some(area) = v.get("work_area") {
        let at = format!("{pointer}/work_area");
        as_object(area, &at)?;
        let start = as_i32(
            field(area, &at, "start_frame")?,
            &format!("{at}/start_frame"),
        )?;
        let end = as_i32(
            field(area, &at, "end_frame_exclusive")?,
            &format!("{at}/end_frame_exclusive"),
        )?;
        let (first, past) = (
            composition.start_frame,
            composition.start_frame + duration as i32,
        );
        if start < first || end > past || start >= end {
            return Err(invalid(
                &at,
                &format!(
                    "a work area of at least one frame inside the composition's frames {first} \
                     to {past}; it is {start} to {end}"
                ),
            ));
        }
        composition.work_area = Some((start, end));
    }

    // D-58. Read into the default camera rather than over a blank one, so a file naming
    // only a zoom keeps the default's place and depth instead of being given nought for
    // both -- a file that says less means less, not worse.
    if let Some(cam) = v.get("camera") {
        let at = format!("{pointer}/camera");
        as_object(cam, &at)?;
        let mut camera = crate::model::Camera::default_for(width, height);
        for prop in [
            crate::model::CameraProp::Position,
            crate::model::CameraProp::Depth,
            crate::model::CameraProp::Zoom,
        ] {
            if let Some(value) = cam.get(prop.as_str()) {
                let here = format!("{at}/{}", prop.as_str());
                *camera.get_mut(prop) =
                    parse_property(value, &here, prop.kind(), false, false, 1.0)?;
            }
        }
        // D-171. Whether it names a layer is asked once the layers are known, below.
        camera.parent = match cam.get("parent") {
            None | Some(J::Null) => None,
            Some(p) => Some(as_id(p, &format!("{at}/parent"))?),
        };
        // D-58 refuses a zoom that is not more than nought, at every frame it is keyed
        // to: such a camera has nothing in front of it, so there is no picture to be had
        // and nothing sensible to draw instead of one.
        for value in std::iter::once(camera.zoom.base())
            .chain(camera.zoom.keyframes().iter().map(|k| k.value))
        {
            if !value.as_scalar().is_some_and(|z| z > 0.0) {
                return Err(invalid(
                    &format!("{at}/zoom"),
                    "a zoom of more than nought; a camera without one has nothing in \
                     front of it to draw",
                ));
            }
        }
        composition.camera = Some(camera);
    }
    if let Some(shutter) = v.get("motion_blur") {
        composition.motion_blur = parse_motion_blur(shutter, &format!("{pointer}/motion_blur"))?;
    }
    composition.label = parse_label(v, pointer)?;
    // D-216 (FX-FBLEND-056).
    if let Some(on) = v.get("frame_blending") {
        composition.frame_blending = as_bool(on, &format!("{pointer}/frame_blending"))?;
    }
    // D-311: three numbers from 0 to 1, or the file is refused.
    if let Some(c) = v.get("background_color") {
        let at = format!("{pointer}/background_color");
        let list = as_array(c, &at)?;
        let mut rgb = [0.0; 3];
        if list.len() != 3 {
            return Err(invalid(&at, "three numbers from 0 to 1 (D-311)"));
        }
        for (i, x) in list.iter().enumerate() {
            rgb[i] = as_f64(x, &format!("{at}/{i}"))?;
            if !(0.0..=1.0).contains(&rgb[i]) {
                return Err(invalid(&at, "three numbers from 0 to 1 (D-311)"));
            }
        }
        composition.background_color = Some(rgb);
    }
    // D-319.
    if let Some(on) = v.get("float_depth") {
        composition.float_depth = as_bool(on, &format!("{pointer}/float_depth"))?;
    }
    // D-330: not both.
    if let Some(on) = v.get("eight_bpc") {
        let at = format!("{pointer}/eight_bpc");
        composition.eight_bpc = as_bool(on, &at)?;
        if composition.eight_bpc && composition.float_depth {
            return Err(invalid(&at, "false when float_depth is true (D-330)"));
        }
    }
    // D-333: only with Float.
    if let Some(on) = v.get("ae_32bpc") {
        let at = format!("{pointer}/ae_32bpc");
        composition.ae_32bpc = as_bool(on, &at)?;
        if composition.ae_32bpc && !composition.float_depth {
            return Err(invalid(&at, "false unless float_depth is true (D-333)"));
        }
    }
    if let Some(sketches) = v.get("sketches") {
        let at = format!("{pointer}/sketches");
        for (i, sketch) in as_array(sketches, &at)?.iter().enumerate() {
            composition.sketches.push(parse_sketch(sketch, &format!("{at}/{i}"))?);
        }
    }
    if let Some(markers) = v.get("markers") {
        let at = format!("{pointer}/markers");
        for (i, marker) in as_array(markers, &at)?.iter().enumerate() {
            let here = format!("{at}/{i}");
            as_object(marker, &here)?;
            composition.markers.push(crate::model::Marker {
                frame: as_i32(field(marker, &here, "frame")?, &format!("{here}/frame"))?,
                name: as_str(field(marker, &here, "name")?, &format!("{here}/name"))?.to_string(),
            });
        }
    }
    // D-84c. A kind this build does not know is kept as written.
    if let Some(columns) = v.get("sheet_text") {
        let at = format!("{pointer}/sheet_text");
        for (i, column) in as_array(columns, &at)?.iter().enumerate() {
            let here = format!("{at}/{i}");
            as_object(column, &here)?;
            let text = |key: &str| {
                as_str(field(column, &here, key)?, &format!("{here}/{key}")).map(str::to_string)
            };
            let mut entries = Vec::new();
            let entries_at = format!("{here}/entries");
            for (j, entry) in as_array(field(column, &here, "entries")?, &entries_at)?
                .iter()
                .enumerate()
            {
                let there = format!("{entries_at}/{j}");
                as_object(entry, &there)?;
                let lines_at = format!("{there}/text");
                let lines = as_array(field(entry, &there, "text")?, &lines_at)?
                    .iter()
                    .enumerate()
                    .map(|(k, s)| as_str(s, &format!("{lines_at}/{k}")).map(str::to_string))
                    .collect::<Result<Vec<_>, _>>()?;
                let start_frame = as_i32(
                    field(entry, &there, "start_frame")?,
                    &format!("{there}/start_frame"),
                )?;
                let end_frame_exclusive = as_i32(
                    field(entry, &there, "end_frame_exclusive")?,
                    &format!("{there}/end_frame_exclusive"),
                )?;
                if end_frame_exclusive <= start_frame {
                    return Err(invalid(
                        &there,
                        "an entry that ends after it starts (D-84c)",
                    ));
                }
                entries.push(crate::model::SheetTextEntry {
                    start_frame,
                    end_frame_exclusive,
                    text: lines,
                });
            }
            composition.sheet_text.push(crate::model::SheetText {
                kind: text("kind")?,
                name: text("name")?,
                track: field(column, &here, "track")?.as_i64().ok_or_else(|| {
                    invalid(&format!("{here}/track"), "a timesheet column's whole number (D-84c)")
                })?,
                entries,
            });
        }
    }

    // D-84f. A key this build does not know stays in the file, and is written back.
    if let Some(details) = v.get("sheet_details") {
        let at = format!("{pointer}/sheet_details");
        as_object(details, &at)?;
        let text = |key: &str| -> Result<String, _> {
            match details.get(key) {
                None => Ok(String::new()),
                Some(s) => as_str(s, &format!("{at}/{key}")).map(str::to_string),
            }
        };
        composition.sheet_details = crate::model::SheetDetails {
            episode: text("episode")?,
            scene: text("scene")?,
            cut: text("cut")?,
            animator: text("animator")?,
            status: text("status")?,
        };
    }

    let order_at = format!("{pointer}/layer_order");
    let mut order = Vec::new();
    for (i, id) in as_array(field(v, pointer, "layer_order")?, &order_at)?
        .iter()
        .enumerate()
    {
        let id = as_id(id, &format!("{order_at}/{i}"))?;
        if order.contains(&id) {
            return Err(invalid(
                &format!("{order_at}/{i}"),
                &format!("each layer to appear once in the order; {id} appears twice"),
            ));
        }
        order.push(id);
    }

    let layers_at = format!("{pointer}/layers");
    let mut layers: Vec<Layer> = Vec::new();
    for (i, layer) in as_array(field(v, pointer, "layers")?, &layers_at)?
        .iter()
        .enumerate()
    {
        let layer = parse_layer(layer, &format!("{layers_at}/{i}"), warnings)?;
        if layers.iter().any(|l| l.id == layer.id) {
            return Err(invalid(
                &format!("{layers_at}/{i}"),
                &format!("a layer ID that is not already used; {} is", layer.id),
            ));
        }
        layers.push(layer);
    }

    // Document 19: "Layer order is composition order. Index is not identity." The order array
    // and the layer array are two statements of the same set, and a file where they disagree
    // has no single answer for what to draw.
    if order.len() != layers.len() {
        return Err(invalid(
            pointer,
            &format!(
                "layer_order and layers to name the same layers; the order has {} and the list \
                 has {}",
                order.len(),
                layers.len()
            ),
        ));
    }
    for id in &order {
        if !layers.iter().any(|l| &l.id == id) {
            return Err(invalid(
                &order_at,
                &format!("every ID in the order to be a layer in this composition; {id} is not"),
            ));
        }
    }

    for (index, id) in order.iter().enumerate() {
        let layer = layers
            .iter()
            .find(|l| &l.id == id)
            .expect("checked above")
            .clone();
        composition.insert_layer(layer, index);
    }

    for id in &order {
        if composition.parent_cycle_from(id) {
            return Err(Diagnostic::new(
                DiagnosticId::ParentCycle,
                Severity::Error,
                "This project cannot be opened, because two layers are parented to each other.",
                format!(
                    "A parent cycle was reached from layer {id} in composition {}. D-57 \
                     requires the parent graph to be acyclic.",
                    composition.id
                ),
            )
            .with_remediation(
                "The project was not opened and nothing on disk was changed. One of the parent \
                 references has to be cleared before it can open.",
            ));
        }
    }
    for id in &order {
        if composition.matte_cycle_from(id) {
            return Err(Diagnostic::new(
                DiagnosticId::MatteCycle,
                Severity::Error,
                "This project cannot be opened, because two layers use each other as a matte.",
                format!(
                    "A matte reference cycle was reached from layer {id} in composition {}. \
                     Document 19 requires the dependency graph to be acyclic.",
                    composition.id
                ),
            )
            .with_remediation(
                "The project was not opened and nothing on disk was changed. One of the matte \
                 references has to be cleared before it can open.",
            ));
        }
    }
    for id in &order {
        if composition.effect_layer_cycle_from(id) {
            return Err(Diagnostic::new(
                DiagnosticId::EffectLayerCycle,
                Severity::Error,
                "This project cannot be opened, because layers' effects read each other in a circle.",
                format!(
                    "The effects' layer settings lead from layer {id} back round to it in \
                     composition {}. D-189 requires them not to.",
                    composition.id
                ),
            )
            .with_remediation(
                "The project was not opened and nothing on disk was changed. One of the layer \
                 settings has to be cleared before it can open.",
            ));
        }
    }
    // D-171: the camera's parent, with the layer's rules.
    if let Some(parent) = composition.camera.as_ref().and_then(|c| c.parent.as_ref()) {
        match composition.layer(parent) {
            Some(l) if l.kind == crate::model::LayerKind::Audio => {
                return Err(invalid(
                    &format!("/compositions/{}/camera/parent", composition.id),
                    &format!("a parent that is not the audio layer {parent} (D-71)"),
                ));
            }
            Some(_) => {}
            None => warnings.push(
                Diagnostic::new(
                    DiagnosticId::ParentReferenceMissing,
                    Severity::Warning,
                    "The camera is parented to a layer that is not in this composition."
                        .to_string(),
                    format!(
                        "The camera of composition {} names parent {parent}, which no layer \
                         matches. The reference is kept and the camera stands where it would \
                         with no parent.",
                        composition.id
                    ),
                )
                .with_remediation(
                    "Choose a parent in the camera's panel, or clear it, to say which it is \
                     meant to be.",
                ),
            ),
        }
    }
    for layer in composition.layers_in_order() {
        if let Some(parent) = &layer.parent {
            if composition.layer(parent).is_none() {
                warnings.push(
                    Diagnostic::new(
                        DiagnosticId::ParentReferenceMissing,
                        Severity::Warning,
                        format!(
                            "The layer \"{}\" is parented to a layer that is not in this \
                             composition.",
                            layer.name
                        ),
                        format!(
                            "Layer {} names parent {}, which no layer in composition {} \
                             matches. The reference is kept and the layer is drawn where it \
                             would be with no parent.",
                            layer.id, parent, composition.id
                        ),
                    )
                    .with_remediation(
                        "Choose a parent in the layer's panel, or clear it, to say which it is \
                         meant to be.",
                    ),
                );
            }
        }
        if let Some(matte) = &layer.matte {
            if composition.layer(&matte.layer_id).is_none() {
                warnings.push(
                    Diagnostic::new(
                        DiagnosticId::MatteReferenceMissing,
                        Severity::Warning,
                        format!(
                            "The layer \"{}\" uses a matte layer that is not in this \
                             composition.",
                            layer.name
                        ),
                        format!(
                            "Layer {} refers to matte layer {}, which no layer in composition \
                             {} matches.",
                            layer.id, matte.layer_id, composition.id
                        ),
                    )
                    .with_remediation(
                        "The reference is kept as it is. Point it at a layer that exists, or \
                         clear it.",
                    ),
                );
            }
        }
    }

    Ok(composition)
}

/// Parse project text into a model, keeping everything this build does not understand.
pub fn load_str(text: &str) -> Result<Loaded, Diagnostic> {
    let root: J = serde_json::from_str(text).map_err(|e| {
        Diagnostic::new(
            DiagnosticId::ProjectSchemaInvalid,
            Severity::Error,
            "This project file cannot be opened, because it is not readable as a project file.",
            format!("The file is not valid JSON: {e}"),
        )
        .with_remediation("The project was not opened and nothing on disk was changed.")
    })?;
    as_object(&root, "")?;

    let version = field(&root, "", "schema_version")?
        .as_i64()
        .ok_or_else(|| invalid("/schema_version", "a whole number"))?;
    if version > SCHEMA_VERSION {
        return Err(Diagnostic::new(
            DiagnosticId::ProjectSchemaNewer,
            Severity::Error,
            "This project was saved by a newer version of the application and cannot be opened \
             here.",
            format!(
                "The file says schema_version {version}; this build understands \
                 {SCHEMA_VERSION}."
            ),
        )
        .with_remediation(
            "The project was not opened and nothing on disk was changed. Opening it in the \
             newer version is the only safe way to read it; guessing at what the newer format \
             means would change the work.",
        ));
    }
    if version != SCHEMA_VERSION {
        return Err(invalid(
            "/schema_version",
            &format!("{SCHEMA_VERSION}, the only project version that has ever existed"),
        ));
    }

    let colors_at = "/color_settings";
    let colors = field(&root, "", "color_settings")?;
    as_object(colors, colors_at)?;
    as_enum(
        field(colors, colors_at, "working_space")?,
        "/color_settings/working_space",
        &["linear-srgb"],
    )?;
    as_enum(
        field(colors, colors_at, "alpha_mode")?,
        "/color_settings/alpha_mode",
        &["premultiplied"],
    )?;

    let mut project = Project::new(as_id(field(&root, "", "project_id")?, "/project_id")?);
    let mut warnings = Vec::new();

    for (i, asset) in as_array(field(&root, "", "assets")?, "/assets")?
        .iter()
        .enumerate()
    {
        let asset = parse_asset(asset, &format!("/assets/{i}"))?;
        if project.assets.iter().any(|a| a.id == asset.id) {
            return Err(invalid(
                &format!("/assets/{i}"),
                &format!("an asset ID that is not already used; {} is", asset.id),
            ));
        }
        project.assets.push(asset);
    }

    for (i, composition) in as_array(field(&root, "", "compositions")?, "/compositions")?
        .iter()
        .enumerate()
    {
        let composition =
            parse_composition(composition, &format!("/compositions/{i}"), &mut warnings)?;
        if project.compositions.iter().any(|c| c.id == composition.id) {
            return Err(invalid(
                &format!("/compositions/{i}"),
                &format!(
                    "a composition ID that is not already used; {} is",
                    composition.id
                ),
            ));
        }
        project.compositions.push(composition);
    }

    // Document 19: a layer names an asset by ID. A layer pointing at nothing is a broken
    // reference, not a missing file, and no amount of relinking fixes it.
    for composition in &project.compositions {
        for layer in composition.layers_in_order() {
            // D-71: a sound is heard and a drawing is seen, and neither layer takes the other's
            // file (FX-AUD-033). Nor does anything ride on, or cut out by, a layer with no place.
            let asset = project.assets.iter().find(|a| a.id == layer.asset_id);
            let is_sound = |a: &Asset| a.kind == AssetKind::Audio;
            if asset.is_some_and(|a| is_sound(a) != (layer.kind == LayerKind::Audio)) {
                return Err(invalid(
                    &format!("/compositions/{}/layers", composition.id),
                    &format!(
                        "layer {} to name a sound file if it is an audio layer and a drawing if \
                         it is not (D-71); it names {}",
                        layer.id, layer.asset_id
                    ),
                ));
            }
            // D-182: a lookup file is read by a Color Lookup, and no layer shows one.
            if asset.is_some_and(|a| a.kind == AssetKind::Lut) {
                return Err(invalid(
                    &format!("/compositions/{}/layers", composition.id),
                    &format!(
                        "layer {} to name a drawing, not the colour lookup file {}; no layer \
                         shows one (D-182)",
                        layer.id, layer.asset_id
                    ),
                ));
            }
            let rides = [
                layer.parent.as_ref(),
                layer.matte.as_ref().map(|m| &m.layer_id),
            ];
            for other in rides.into_iter().flatten() {
                if composition
                    .layer(other)
                    .is_some_and(|l| l.kind == LayerKind::Audio)
                {
                    return Err(invalid(
                        &format!("/compositions/{}/layers", composition.id),
                        &format!(
                            "layer {} not to have the audio layer {other} as its parent or matte \
                             (D-71)",
                            layer.id
                        ),
                    ));
                }
            }
            // D-82: a null has no picture to cut anything out by (FX-NULL-029).
            if let Some(matte) = &layer.matte {
                if composition
                    .layer(&matte.layer_id)
                    .is_some_and(|l| l.kind == LayerKind::Null)
                {
                    return Err(invalid(
                        &format!("/compositions/{}/layers", composition.id),
                        &format!(
                            "layer {} not to have the null layer {} as its matte (D-82)",
                            layer.id, matte.layer_id
                        ),
                    ));
                }
            }
            if !layer.has_no_drawing() && !project.assets.iter().any(|a| a.id == layer.asset_id) {
                return Err(invalid(
                    &format!("/compositions/{}/layers", composition.id),
                    &format!(
                        "layer {} to name an asset this project has; it names {}, which no \
                         asset record matches",
                        layer.id, layer.asset_id
                    ),
                ));
            }
        }
    }

    // D-67: the composition graph is acyclic, and a composition layer naming a composition
    // that is not here keeps its reference and draws nothing.
    for composition in &project.compositions {
        for layer in composition.layers_in_order() {
            let Some(inner) = &layer.composition_id else {
                continue;
            };
            if project.composition_reaches(inner, &composition.id) {
                return Err(Diagnostic::new(
                    DiagnosticId::CompositionCycle,
                    Severity::Error,
                    "This project cannot be opened, because a composition is shown inside itself.",
                    format!(
                        "Layer {} of composition {} shows composition {inner}, which leads back \
                         to {}. D-67 requires the composition graph to be acyclic.",
                        layer.id, composition.id, composition.id
                    ),
                )
                .with_remediation(
                    "The project was not opened and nothing on disk was changed. One of the \
                     composition layers has to be removed before it can open.",
                ));
            }
            if project.composition(inner).is_none() {
                warnings.push(
                    Diagnostic::new(
                        DiagnosticId::CompositionReferenceMissing,
                        Severity::Warning,
                        format!(
                            "The layer \"{}\" shows a composition that is not in this project.",
                            layer.name
                        ),
                        format!(
                            "Layer {} of composition {} names composition {inner}, which no \
                             composition matches. The reference is kept and the layer draws \
                             nothing.",
                            layer.id, composition.id
                        ),
                    )
                    .with_remediation(
                        "Delete the layer, or put the composition it shows back in the project.",
                    ),
                );
            }
        }
    }

    // D-182: a Color Lookup naming no lookup file of this project keeps the name as written, and
    // says so (FX-LUT-010, 011). A missing or unreadable file is said by the asset loop above
    // and at the frame.
    for composition in &project.compositions {
        for layer in composition.layers_in_order() {
            for instance in &layer.effects {
                if let Some(lut) = crate::lut::dangling(&project, &instance.effect) {
                    warnings.push(crate::lut::not_a_lookup_file(&layer.name, lut));
                }
            }
        }
    }

    // D-189: a layer setting naming no layer of its composition is kept as written, and said
    // here as each frame says it (FX-CBLUR-015).
    for composition in &project.compositions {
        for layer in composition.layers_in_order() {
            for instance in &layer.effects {
                if let Some((named, _)) = instance.effect.layer_setting() {
                    if !named.is_empty() && composition.layer(&crate::model::Id::new(named)).is_none() {
                        warnings.push(crate::layer_map::missing(&layer.name, named, "every frame is drawn"));
                    }
                }
            }
        }
    }

    Ok(Loaded {
        document: Document::new(project),
        preserved: Preserved { root },
        warnings,
    })
}

/// W-31: effects copied off a layer or kept as a preset, a JSON list written the way a layer's
/// `effects` are, read by the same rules.
///
/// Refused whole rather than read in part (document 28). A pasted effect has no place in a file
/// for anything this build does not understand to be preserved in, so an effect this build does
/// not have, or anything in one that saving would not write back, would be lost without a word.
/// Settings out of range are read as a file's are: kept, and the effect is not drawn.
pub fn read_effects(text: &str) -> Result<Vec<crate::effects::EffectInstance>, Diagnostic> {
    let v: J = serde_json::from_str(text)
        .map_err(|e| invalid("/", &format!("a list of effects written as JSON ({e})")))?;
    effects_whole(
        &v,
        "",
        "pasted",
        "Nothing was pasted and nothing was changed. Paste it in the build that copied it.",
    )
}

/// `read_effects`'s rules on a list found at `at`, refusals worded as what was `done` to it and
/// what `remedy` says (D-180: a preset file's effects are read by them too).
fn effects_whole(
    v: &J,
    at: &str,
    done: &str,
    remedy: &str,
) -> Result<Vec<crate::effects::EffectInstance>, Diagnostic> {
    let mut ignored = Vec::new();
    let mut out = Vec::new();
    for (i, one) in as_array(v, if at.is_empty() { "/" } else { at })?.iter().enumerate() {
        let at = format!("{at}/{i}");
        let instance = parse_effect(one, &at, "pasted", &Id::new("pasted"), &mut ignored)?;
        let type_id = instance.type_id().to_string();
        let refuse = |why: String| {
            Diagnostic::new(
                DiagnosticId::EffectUnsupported,
                Severity::Error,
                format!("The effect {type_id} cannot be {done}: {why}"),
                format!("{type_id}: {why}"),
            )
            .with_remediation(remedy)
        };
        if let crate::effects::Effect::Unsupported { .. } = instance.effect {
            return Err(refuse("this build does not have it.".to_string()));
        }
        // What saving writes, against the same with the pasted text underneath: anything only the
        // second has is something this build would drop.
        let kept = effect_json(Some(one), &instance);
        let written = effect_json(None, &instance);
        if kept != written {
            let keys = |j: &J, sub: Option<&str>| -> Vec<String> {
                let j = sub.map_or(Some(j), |s| j.get(s));
                j.and_then(J::as_object)
                    .map(|m| m.keys().cloned().collect())
                    .unwrap_or_default()
            };
            let lost: Vec<String> = keys(&kept, None)
                .into_iter()
                .filter(|k| written.get(k).is_none())
                .chain(
                    keys(&kept, Some("parameters"))
                        .into_iter()
                        .filter(|k| written["parameters"].get(k).is_none()),
                )
                .collect();
            return Err(refuse(format!(
                "it carries {}, which this build does not understand and would not keep.",
                if lost.is_empty() { "settings".to_string() } else { lost.join(", ") }
            )));
        }
        out.push(instance);
    }
    Ok(out)
}

/// D-180: the one preset file version there has been.
pub const PRESET_FILE_VERSION: i64 = 0;

/// D-181: the starter presets the program comes with, as the file
/// `tools/starter_presets_reference.py` writes them. Changing one is a specification decision made
/// there, not here.
pub const STARTER_PRESETS: &str = include_str!("../Fixtures/starter_presets/starter.fxpreset");

/// D-181: the sentence the Effects panel shows for each starter preset, in the file's order.
pub const STARTER_ABOUT: [&str; 9] = [
    "A gentle glow off the brightest parts of the picture. Best on an adjustment layer.",
    "Day painted as night: cooler, darker, the corners fall away. Best on an adjustment layer.",
    "Warm evening light falling from the top of the frame. Best on an adjustment layer.",
    "A hard, flat shadow down and to the right, in a deep violet rather than black. For a character layer.",
    "A warm light catching the upper right edge. For a character layer.",
    "Sepia, grain, dark corners and a slight flicker. Best on an adjustment layer.",
    "The frame of a hit: glints on the brights, colour fringes and a shake. Best on an adjustment layer, for a few frames.",
    "A soft, bright haze with the colour lifted. Best on an adjustment layer.",
    "Focus lines rushing in to the middle of the frame. Best on an adjustment layer.",
];

/// D-180: one preset read from a preset file.
#[derive(Clone, Debug, PartialEq)]
pub struct Preset {
    pub name: String,
    pub effects: Vec<crate::effects::EffectInstance>,
}

/// D-180: a preset file, read whole or refused whole (document 19's "Effect preset files").
///
/// Its effects are read by `read_effects`'s rules, so an effect or a setting this build would not
/// keep refuses the file, and so does anything else the format does not name.
pub fn read_presets(text: &str) -> Result<Vec<Preset>, Diagnostic> {
    presets_whole(text, "imported", "Nothing was imported, and the presets you have are unchanged.")
}

/// D-180: the text of a preset file holding `presets`, a JSON list of `{name, effects}` as the
/// window keeps them. Nothing is written that would not read back whole.
pub fn write_presets(presets: &str) -> Result<String, Diagnostic> {
    let remedy = "Nothing was written.";
    let list: J = serde_json::from_str(presets).map_err(|e| {
        Diagnostic::new(
            DiagnosticId::PresetFileInvalid,
            Severity::Error,
            "These presets cannot be exported: the window sent them garbled.",
            e.to_string(),
        )
        .with_remediation(remedy)
    })?;
    let file = serde_json::json!({ "preset_file_version": PRESET_FILE_VERSION, "presets": list });
    let text = serde_json::to_string_pretty(&file).expect("a JSON value writes") + "\n";
    presets_whole(&text, "exported", remedy)?;
    Ok(text)
}

fn presets_whole(text: &str, done: &str, remedy: &str) -> Result<Vec<Preset>, Diagnostic> {
    let refuse = |said: String, detail: String| {
        Diagnostic::new(
            DiagnosticId::PresetFileInvalid,
            Severity::Error,
            format!("This preset file cannot be {done}: {said}"),
            detail,
        )
        .with_remediation(remedy)
    };
    let root: J = serde_json::from_str(text)
        .map_err(|e| refuse("it is not JSON text, so it is not a preset file.".into(), e.to_string()))?;
    let Some(top) = root.as_object() else {
        return Err(refuse("it is not a preset file.".into(), "The file is not a JSON object.".into()));
    };
    if top.contains_key("schema_version") {
        return Err(refuse(
            "it is a project file, not a preset file. Open it with Open instead.".into(),
            "The file has schema_version, which projects have.".into(),
        ));
    }
    match top.get("preset_file_version").and_then(J::as_i64) {
        Some(PRESET_FILE_VERSION) => {}
        Some(v) if v > PRESET_FILE_VERSION => {
            return Err(refuse(
                "it was written by a newer version of this program, whose presets this one does not read."
                    .into(),
                format!("preset_file_version {v}; this build reads {PRESET_FILE_VERSION}."),
            ))
        }
        _ => {
            return Err(refuse(
                "it does not say it is a preset file of a version this build reads.".into(),
                format!("preset_file_version is missing or not {PRESET_FILE_VERSION}."),
            ))
        }
    }
    let lost = |keys: &serde_json::Map<String, J>, known: &[&str]| {
        keys.keys().find(|k| !known.contains(&k.as_str())).cloned()
    };
    if let Some(extra) = lost(top, &["preset_file_version", "presets"]) {
        return Err(refuse(
            format!("it holds \"{extra}\", which a preset file does not have and this build would lose."),
            format!("At /{extra}."),
        ));
    }
    let list = match top.get("presets").and_then(J::as_array) {
        Some(list) if !list.is_empty() => list,
        _ => return Err(refuse("it holds no presets.".into(), "At /presets: expected a list of at least one.".into())),
    };
    let mut out: Vec<Preset> = Vec::new();
    for (i, one) in list.iter().enumerate() {
        let at = format!("/presets/{i}");
        let which = i + 1;
        let Some(preset) = one.as_object() else {
            return Err(refuse(format!("preset {which} is not written as a preset."), format!("At {at}.")));
        };
        if let Some(extra) = lost(preset, &["name", "effects"]) {
            return Err(refuse(
                format!("preset {which} holds \"{extra}\", which a preset does not have and this build would lose."),
                format!("At {at}/{extra}."),
            ));
        }
        let name = match preset.get("name").and_then(J::as_str) {
            Some(name) if !name.trim().is_empty() => name.to_string(),
            _ => return Err(refuse(format!("preset {which} has no name."), format!("At {at}/name."))),
        };
        if out.iter().any(|p| p.name == name) {
            return Err(refuse(format!("two presets are named \"{name}\"."), format!("At {at}/name.")));
        }
        let effects = match preset.get("effects") {
            Some(list) if list.as_array().is_some_and(|l| !l.is_empty()) => list,
            _ => return Err(refuse(format!("the preset \"{name}\" has no effects."), format!("At {at}/effects."))),
        };
        let effects = effects_whole(effects, &format!("{at}/effects"), done, remedy).map_err(|d| {
            if d.id == DiagnosticId::ProjectSchemaInvalid {
                refuse(format!("an effect in the preset \"{name}\" is not written as a project writes one."), d.detail)
            } else {
                d
            }
        })?;
        out.push(Preset { name, effects });
    }
    Ok(out)
}

/// Open a project file.
///
/// Adds `MEDIA_MISSING` for every file an asset names that is not on disk, resolved relative
/// to the project file's own directory. Document 28: "preserve reference; render transparent
/// placeholder + warning" — the reference is not repaired and not removed.
pub fn load(path: &Path) -> Result<Loaded, Diagnostic> {
    let text = fs::read_to_string(path).map_err(|e| {
        Diagnostic::new(
            DiagnosticId::ProjectSchemaInvalid,
            Severity::Error,
            "This project file could not be read.",
            format!("Reading {} failed: {e}", path.display()),
        )
        .with_remediation("Nothing on disk was changed.")
    })?;
    let mut loaded = load_str(&text)?;
    let root = path.parent().unwrap_or(Path::new("."));
    for asset in &loaded.document.project().assets {
        let absent: Vec<&str> = asset
            .files()
            .into_iter()
            .filter(|relative| !root.join(relative).exists())
            .collect();
        if absent.is_empty() {
            continue;
        }
        loaded.warnings.push(
            Diagnostic::new(
                DiagnosticId::MediaMissing,
                Severity::Warning,
                format!(
                    "{} of the files for \"{}\" are not where the project expects them.",
                    absent.len(),
                    asset.name
                ),
                format!(
                    "Asset {} names {} relative to {}.",
                    asset.id,
                    absent.join(", "),
                    root.display()
                ),
            )
            .with_remediation(
                "The reference is kept as it is. Relink the asset to point it at the files, \
                 or put them back. Frames that cannot be found render as nothing rather than \
                 as a guess.",
            ),
        );
    }
    Ok(loaded)
}

// ---------------------------------------------------------------------------------------
// Saving
// ---------------------------------------------------------------------------------------

fn save_failed(path: &Path, detail: String) -> Diagnostic {
    Diagnostic::new(
        DiagnosticId::ProjectSaveFailed,
        Severity::Error,
        "The project could not be saved.",
        format!("Writing {} failed: {detail}", path.display()),
    )
    .with_remediation(
        "The last version that saved successfully is still on disk exactly as it was, and \
         your unsaved changes are still open. Try a different location, or free some space.",
    )
}

/// Where the temporary sibling for `path` goes. ADR-008: a sibling, so the replacement stays
/// on one filesystem and can be atomic.
fn temp_sibling(path: &Path) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(".saving.tmp");
    path.with_file_name(name)
}

fn write_all_limited(file: &mut File, bytes: &[u8], limit: Option<u64>) -> io::Result<()> {
    let allowed = limit.map_or(bytes.len(), |l| (l as usize).min(bytes.len()));
    file.write_all(&bytes[..allowed])?;
    if allowed < bytes.len() {
        return Err(io::Error::other(format!(
            "no space left on device after {allowed} of {} bytes",
            bytes.len()
        )));
    }
    Ok(())
}

/// Save the project, replacing the file at `path` only once the new one is completely written.
///
/// On success the document is marked saved, which is what makes document 26's dirty rule work:
/// "If the current document state becomes byte/semantic-equivalent to the last successful save
/// revision, dirty becomes false." On any failure the document is left dirty and the file at
/// `path` is untouched.
pub fn save(path: &Path, document: &mut Document, preserved: &Preserved) -> Result<(), Diagnostic> {
    save_limited(path, document, preserved, None)
}

/// [`save`], with an injected write failure after `byte_limit` bytes.
///
/// This exists for document 25's FX-IO-002, "disk-full/write failure reports
/// `PROJECT_SAVE_FAILED` and does not truncate the previous valid save", and FX-IO-001,
/// "interrupted replacement retains last valid project". A real full disk cannot be arranged
/// inside a test, and a save path that is only ever exercised when it succeeds is a save path
/// nobody has checked. `None` is an ordinary save and is what [`save`] passes.
pub fn save_limited(
    path: &Path,
    document: &mut Document,
    preserved: &Preserved,
    byte_limit: Option<u64>,
) -> Result<(), Diagnostic> {
    let text = to_json(document.project(), preserved);

    // Document 07: "Validate before writing." The check is that the text just produced loads
    // back as a project; a file that cannot be reopened must never reach the disk under the
    // name of one that could.
    if let Err(e) = load_str(&text) {
        return Err(save_failed(
            path,
            format!(
                "the project was not written because it did not pass validation: {}",
                e.detail
            ),
        ));
    }

    let temp = temp_sibling(path);
    let outcome = (|| -> io::Result<()> {
        let mut file = File::create(&temp)?;
        write_all_limited(&mut file, text.as_bytes(), byte_limit)?;
        file.flush()?;
        file.sync_all()?;
        drop(file);
        fs::rename(&temp, path)
    })();

    if let Err(e) = outcome {
        // The partial temporary file is the only thing that could have been created, and it
        // is not the project. Removing it is a courtesy; failing to remove it is not a second
        // error to report over the first.
        let _ = fs::remove_file(&temp);
        return Err(save_failed(path, e.to_string()));
    }

    document.mark_saved();
    Ok(())
}

// ---------------------------------------------------------------------------------------
// Autosave and recovery
// ---------------------------------------------------------------------------------------

/// `shot.json` slot 2 is `shot.autosave-2.json`, beside the project.
///
/// Document 07: "Autosaves use separate recovery files and must not overwrite the last manual
/// save." A separate name rather than a separate directory, so a recovery file cannot be left
/// behind somewhere the person who owns the project never looks.
pub fn autosave_path(project_path: &Path, slot: usize) -> PathBuf {
    let stem = project_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    project_path.with_file_name(format!("{stem}.autosave-{slot}.json"))
}

/// One autosave found beside a project.
#[derive(Clone, Debug)]
pub struct RecoveryCandidate {
    pub path: PathBuf,
    pub slot: usize,
    pub modified: SystemTime,
}

/// Every autosave beside `project_path`, newest first.
///
/// "Newest" is the file system's modification time, which is only an ordering because
/// `autosave` makes it one: see `stamp_newest`. The slot is the tie-break of last resort, for
/// snapshots this build did not write.
pub fn recovery_candidates(project_path: &Path) -> Vec<RecoveryCandidate> {
    let mut found: Vec<RecoveryCandidate> = (0..AUTOSAVE_SLOTS)
        .filter_map(|slot| {
            let path = autosave_path(project_path, slot);
            let modified = fs::metadata(&path).and_then(|m| m.modified()).ok()?;
            Some(RecoveryCandidate {
                path,
                slot,
                modified,
            })
        })
        .collect();
    found.sort_by(|a, b| b.modified.cmp(&a.modified).then(a.slot.cmp(&b.slot)));
    found
}

/// Document 28: `PROJECT_RECOVERY_AVAILABLE`, "show timestamp/path choice".
pub fn recovery_diagnostic(candidates: &[RecoveryCandidate]) -> Option<Diagnostic> {
    let newest = candidates.first()?;
    Some(
        Diagnostic::new(
            DiagnosticId::ProjectRecoveryAvailable,
            Severity::Info,
            format!(
                "There {} {} recovery {} for this project, from work that was not saved.",
                if candidates.len() == 1 { "is" } else { "are" },
                candidates.len(),
                if candidates.len() == 1 {
                    "snapshot"
                } else {
                    "snapshots"
                }
            ),
            format!(
                "The newest is {} in slot {}.",
                newest.path.display(),
                newest.slot
            ),
        )
        .with_remediation(
            "Opening a snapshot does not replace the saved project. The saved project is \
             still on disk exactly as it was.",
        ),
    )
}

/// Write a recovery snapshot beside the project, into the oldest of the rotating slots.
///
/// Takes `&Document` rather than `&mut Document` on purpose. Document 26: "Autosave does not
/// clear user-facing dirty state and does not replace the canonical manual-save path." A
/// function that cannot reach `mark_saved` cannot accidentally clear it.
pub fn autosave(
    project_path: &Path,
    document: &Document,
    preserved: &Preserved,
) -> Result<PathBuf, Diagnostic> {
    let mut oldest: Option<(usize, SystemTime)> = None;
    let mut slot = None;
    for candidate in 0..AUTOSAVE_SLOTS {
        let path = autosave_path(project_path, candidate);
        match fs::metadata(&path).and_then(|m| m.modified()) {
            Err(_) => {
                slot = Some(candidate);
                break;
            }
            Ok(modified) => {
                if oldest.is_none_or(|(_, t)| modified < t) {
                    oldest = Some((candidate, modified));
                }
            }
        }
    }
    let slot = slot.unwrap_or_else(|| oldest.expect("AUTOSAVE_SLOTS is not zero").0);
    let path = autosave_path(project_path, slot);

    let text = to_json(document.project(), preserved);
    let temp = temp_sibling(&path);
    let outcome = (|| -> io::Result<()> {
        let mut file = File::create(&temp)?;
        file.write_all(text.as_bytes())?;
        file.flush()?;
        file.sync_all()?;
        drop(file);
        fs::rename(&temp, &path)
    })();
    match outcome {
        Ok(()) => {
            // Document 07 promises recovery opens the newest snapshot, and both the rotation
            // above and `recovery_candidates` read "newest" off the file system's modification
            // time. Windows records that time coarsely enough that two snapshots written in the
            // same millisecond tie, and a tie is exactly where the promise breaks: the eighth
            // snapshot of `verification/B-09_recovery_table.md` tied with the seventh in one run
            // of five and the older of the two was offered. So the writer states the order
            // rather than hoping the clock ticked between two writes.
            let _ = stamp_newest(project_path, &path);
            Ok(path)
        }
        Err(e) => {
            let _ = fs::remove_file(&temp);
            Err(save_failed(&path, e.to_string()))
        }
    }
}

/// Give the snapshot just written a modification time strictly later than every other snapshot
/// beside it, so that "newest" is an ordering and not a coincidence of clock granularity.
///
/// Best effort on purpose. A file system that refuses the change leaves the ordering exactly as
/// coarse as it was and no worse, which is why the caller discards the result: a snapshot that
/// was written is worth more than a timestamp that was not adjusted.
fn stamp_newest(project_path: &Path, path: &Path) -> io::Result<()> {
    let others = (0..AUTOSAVE_SLOTS)
        .map(|slot| autosave_path(project_path, slot))
        .filter(|p| p != path)
        .filter_map(|p| fs::metadata(p).and_then(|m| m.modified()).ok())
        .max();
    let now = SystemTime::now();
    let when = match others {
        Some(other) if other >= now => other + Duration::from_millis(1),
        _ => now,
    };
    File::options().write(true).open(path)?.set_modified(when)
}

// ---------------------------------------------------------------------------------------
// Relink
// ---------------------------------------------------------------------------------------

/// What relinking an asset to a chosen set of files would do, before it is done.
///
/// Document 07: "Show candidate sequence, dimensions, range and interpretation before relink."
/// So the preview is a value the caller can display, and applying it is a separate step.
pub struct RelinkCandidate {
    pub asset_id: Id,
    /// The record as it would become. Layer references are untouched: this replaces one asset
    /// record and nothing else, which is how document 07's "preserve layer IDs and effects"
    /// is met — there is no code path here that can reach a layer.
    pub asset: Asset,
    pub pattern: String,
    pub range: Option<(u32, u32)>,
    pub missing: Vec<u32>,
    pub width: u32,
    pub height: u32,
    pub interpretation: Interpretation,
    /// Everything B-03's importer said about the chosen files.
    pub diagnostics: Vec<Diagnostic>,
}

/// Normalise a media path for storage: relative to the project's directory when it is under
/// it, with forward slashes, so a project and its media tree move together.
///
/// Public because importing media happens in the window rather than here — B-12a's
/// `media.import` builds an asset record out of what B-03's importer found — and the rule for
/// how a path is written into a project belongs to the format, not to whoever is building the
/// record.
pub fn stored_path(project_dir: &Path, file: &Path) -> String {
    let relative = file.strip_prefix(project_dir).unwrap_or(file);
    // Separator by separator rather than component by component: a path that is not under the
    // project keeps its root, and this platform spells a root with the separator being replaced.
    relative.to_string_lossy().replace('\\', "/")
}

/// The same project with every media path rewritten for a file in `to` rather than in `from`.
///
/// Stored paths are relative to the project file, so a project written into another directory
/// keeps pointing at the same drawings only if its paths are rewritten on the way. Each one is
/// resolved against `from` and stored again by [`stored_path`]: relative where the drawing is
/// under `to`, whole where it is not. The owner's B-13e playtest found the cost of not doing
/// this: Save As into another folder reopened on a blank canvas.
pub fn rebased(project: &Project, from: &Path, to: &Path) -> Project {
    let mut project = project.clone();
    let moved = |stored: &mut String| *stored = stored_path(to, &from.join(stored.as_str()));
    for asset in &mut project.assets {
        asset.path.iter_mut().for_each(moved);
        asset.frames.values_mut().for_each(moved);
    }
    project
}

/// Work out what relinking `asset_id` to `files` would produce.
///
/// The files are the user's selection, never a directory this scanned on its own. Document 07:
/// "Search only user-selected locations."
pub fn relink_candidate(
    project: &Project,
    asset_id: &Id,
    files: &[PathBuf],
    project_dir: &Path,
) -> Result<RelinkCandidate, Diagnostic> {
    let existing = project
        .assets
        .iter()
        .find(|a| &a.id == asset_id)
        .ok_or_else(|| {
            Diagnostic::new(
                DiagnosticId::CommandTargetMissing,
                Severity::Error,
                "That media is not in this project.",
                format!("No asset record has the ID {asset_id}."),
            )
            .with_remediation("Nothing was changed.")
        })?;

    let result = media::import_sequence(files);
    let Some(sequence): Option<SequenceAsset> = result.asset else {
        return Err(Diagnostic::new(
            DiagnosticId::MediaMissing,
            Severity::Error,
            format!(
                "None of the chosen files can stand in for \"{}\".",
                existing.name
            ),
            format!(
                "{} file(s) were chosen and none of them formed a usable image sequence.",
                files.len()
            ),
        )
        .with_remediation(
            "The project still points at the files it pointed at before. Choose the folder \
             the drawings are actually in.",
        ));
    };

    // D-62: relinking to files of the other format takes that format's interpretation, since
    // an EXR and a PNG cannot share one; otherwise the record's own carries over.
    let old_file = existing
        .files()
        .first()
        .map(|f| f.to_string())
        .unwrap_or_default();
    let interpretation =
        if crate::exr_io::is_exr(sequence.pattern()) == crate::exr_io::is_exr(&old_file) {
            existing.interpretation
        } else {
            Interpretation::for_file(sequence.pattern())
        };
    let frames: BTreeMap<u32, String> = sequence
        .frames()
        .iter()
        .map(|(n, p)| (*n, stored_path(project_dir, p)))
        .collect();
    let asset = Asset {
        id: existing.id.clone(),
        kind: AssetKind::ImageSequence,
        name: existing.name.clone(),
        path: None,
        pattern: Some(sequence.pattern().to_string()),
        frames,
        // Document 07: "Preserve layer IDs and effects." Interpretation is a property of the
        // media the user chose, not of the record being replaced, but nothing in the new files
        // states it, so the record's own interpretation carries over rather than being reset
        // to a default that would silently change how the pixels are read.
        interpretation,
        redistribute: existing.redistribute,
        label: existing.label,
    };

    Ok(RelinkCandidate {
        asset_id: asset_id.clone(),
        pattern: sequence.pattern().to_string(),
        range: sequence.range(),
        missing: sequence.missing(),
        width: sequence.width(),
        height: sequence.height(),
        interpretation,
        diagnostics: result.diagnostics,
        asset,
    })
}

/// The command that applies a relink. Document 02: "Undo restores the prior reference."
pub fn relink_command(candidate: &RelinkCandidate) -> Command {
    Command::RelinkAsset {
        asset: Box::new(candidate.asset.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// P-25: a file is held to the size and length a new composition is; one inside them opens.
    #[test]
    fn a_file_is_held_to_the_composition_limits() {
        let fixture = include_str!("../Fixtures/projects/minimal_project.json");
        let with = |from: &str, to: &str| load_str(&fixture.replace(from, to));
        assert!(with("\"width\": 1920", "\"width\": 16384").is_ok());
        assert!(with("\"duration_frames\": 24", "\"duration_frames\": 10000").is_ok());
        assert!(with("\"width\": 1920", "\"width\": 16385").is_err());
        assert!(with("\"height\": 1080", "\"height\": 100000").is_err());
        assert!(with("\"duration_frames\": 24", "\"duration_frames\": 10001").is_err());
        let both = fixture.replace("\"width\": 1920", "\"width\": 16384").replace("\"height\": 1080", "\"height\": 16384");
        assert!(load_str(&both).is_err(), "16384 x 16384 is past the pixel limit");
    }
}
