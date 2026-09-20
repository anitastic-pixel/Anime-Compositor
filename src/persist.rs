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
    Layer, LayerKind, MatteReference, Project, Prop, Property, Value,
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
    "instance_id",
    "type_id",
    "parameters",
    "zoom",
    "expression",
];

/// An effect record is the one place a flat list is not enough: it spells `enabled` after
/// `type_id`, while a layer spells it near the top, so the two cannot share a ranking. Effect
/// records are recognised by `instance_id`, which nothing else in the schema has.
const EFFECT_KEY_ORDER: &[&str] = &["instance_id", "type_id", "enabled", "parameters"];

/// D-59's expression record is the other: `text`, then `enabled`. Recognised by `text`, which
/// nothing else in the schema has.
const EXPRESSION_KEY_ORDER: &[&str] = &["text", "enabled"];

fn order_for(map: &Map<String, J>) -> &'static [&'static str] {
    if map.contains_key("instance_id") {
        EFFECT_KEY_ORDER
    } else if map.contains_key("text") {
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
    if asset.kind == AssetKind::Audio {
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
            map.insert("mode".into(), J::from("alpha"));
            map.insert("matte_only".into(), J::from(m.matte_only));
            J::Object(map)
        }
        None => J::Null,
    };
    // Each mask merges over whatever the file held at the same place in the list, the way the
    // matte does, so a key this build does not know about inside a mask record survives the
    // round trip -- and so does a path's `keyframes`, which B-24d will build and this build only
    // carries. D-77: the `masks` list is written and the old single `mask` key never is, so a
    // file saved by this build and opened by one older than it has no mask rather than a
    // disagreeing pair.
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
            map.insert("opacity".into(), J::from(m.opacity));
            map.insert("feather_px".into(), J::from(m.feather_px));
            map.insert("expansion_px".into(), J::from(m.expansion_px));
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
            path_base.insert(
                "points".into(),
                J::Array(
                    m.points
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
                ),
            );
            path.insert("base".into(), J::Object(path_base));
            path.entry("keyframes".to_string())
                .or_insert_with(|| J::Array(Vec::new()));
            map.insert("path".into(), J::Object(path));
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
    // D-66: an adjustment layer has no drawing, so the three keys about one are not written.
    // D-74: nor are they for a solid, whose drawing is its `solid` record.
    if layer.is_adjustment() || layer.solid.is_some() {
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
    merge(base, owned)
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
    let mut params = base
        .and_then(|b| b.get("parameters"))
        .and_then(J::as_object)
        .cloned()
        .unwrap_or_else(Map::new);
    match &instance.effect {
        Effect::Exposure { stops } => {
            params.insert("stops".into(), num(*stops));
        }
        Effect::GaussianBlur { sigma_px } => {
            params.insert("sigma_px".into(), num(*sigma_px));
        }
        Effect::Tint { color, amount } => {
            params.insert(
                "color".into(),
                J::Array(color.iter().map(|c| num(*c)).collect()),
            );
            params.insert("amount".into(), num(*amount));
        }
        Effect::Unsupported { .. } => {}
    }
    // D-68: a setting with keys is a property record whose base is the plain value just
    // written; a setting without any stays that plain value.
    for (name, track) in &instance.tracks {
        let Some(plain) = params.get(name).cloned() else {
            continue;
        };
        let mut channels: Vec<J> = track.iter().map(|p| property_json(None, p, 1.0)).collect();
        let mut record = channels.remove(0);
        if let Some(keys) = record["keyframes"].as_array_mut() {
            for (i, key) in keys.iter_mut().enumerate() {
                if !channels.is_empty() {
                    let mut three = vec![key["value"].clone()];
                    three.extend(channels.iter().map(|c| c["keyframes"][i]["value"].clone()));
                    key["value"] = J::Array(three);
                }
            }
        }
        record["base"] = plain;
        params.insert(name.clone(), record);
    }
    if !matches!(instance.effect, Effect::Unsupported { .. }) || base.is_some() {
        owned.push(("parameters", J::Object(params)));
    }
    merge(base, owned)
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
    merge(base, owned)
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

/// The tint colour: three linear RGB numbers, in document 21's order.
fn effect_color(params: Option<&J>, at: &str) -> Result<[f64; 3], Diagnostic> {
    let params = effect_params(params, at)?;
    let at = format!("{at}/parameters/color");
    let color = as_array(field(params, &at, "color")?, &at)?;
    if color.len() != 3 {
        return Err(invalid(
            &at,
            &format!(
                "a linear RGB triple, and this one has {} numbers",
                color.len()
            ),
        ));
    }
    Ok([
        as_f64(&color[0], &format!("{at}/0"))?,
        as_f64(&color[1], &format!("{at}/1"))?,
        as_f64(&color[2], &format!("{at}/2"))?,
    ])
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
    for name in ["stops", "sigma_px", "color", "amount"] {
        let Some(record) = map.get(name).filter(|v| v.is_object()) else {
            continue;
        };
        let at = format!("{at}/parameters/{name}");
        if record.get("expression").is_some() {
            return Err(invalid(
                &at,
                "no expression: an effect's setting takes keys only",
            ));
        }
        let base = field(record, &at, "base")?;
        let keys = as_array(field(record, &at, "keyframes")?, &format!("{at}/keyframes"))?;
        let count = if name == "color" { 3 } else { 1 };
        let mut track = Vec::new();
        for c in 0..count {
            // One channel's record: the same keys, each holding that channel's number.
            let pick = |v: &J, at: &str| -> Result<J, Diagnostic> {
                if count == 1 {
                    return Ok(v.clone());
                }
                match v.as_array() {
                    Some(three) if three.len() == 3 => Ok(three[c].clone()),
                    _ => Err(invalid(at, "a linear RGB triple")),
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
            track.push(parse_property(
                &J::Object(channel),
                &at,
                "scalar",
                false,
                false,
                1.0,
            )?);
        }
        plain.insert(name.into(), base.clone());
        if track[0].is_animated() {
            tracks.insert(name.into(), track);
        }
    }
    Ok((Some(J::Object(plain)), tracks))
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
        &["still", "image_sequence", "audio"],
    )? {
        "still" => AssetKind::Still,
        "audio" => AssetKind::Audio,
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
        // holds the usual pair and nothing reads it.
        interpretation: match v.get("interpretation") {
            None if kind == AssetKind::Audio => Interpretation {
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
    })
}

fn parse_layer(v: &J, pointer: &str, warnings: &mut Vec<Diagnostic>) -> Result<Layer, Diagnostic> {
    as_object(v, pointer)?;
    let kind = match as_enum(
        field(v, pointer, "kind")?,
        &format!("{pointer}/kind"),
        &["raster", "adjustment", "composition", "audio", "solid"],
    )? {
        "solid" => LayerKind::Solid,
        "audio" => LayerKind::Audio,
        "adjustment" => LayerKind::Adjustment,
        "composition" => LayerKind::Composition,
        _ => LayerKind::Raster,
    };
    let id = as_id(field(v, pointer, "id")?, &format!("{pointer}/id"))?;
    if kind == LayerKind::Audio {
        return parse_audio_layer(v, pointer, id);
    }
    // D-66: an adjustment layer has no drawing. A file that gives one an asset is not a file
    // this build can read faithfully, so it is refused rather than drawn with the asset ignored.
    let asset_id = if kind != LayerKind::Raster {
        if v.get("asset_id").is_some() {
            return Err(invalid(
                &format!("{pointer}/asset_id"),
                "no asset_id on an adjustment, composition or solid layer, which has no drawing \
                 (D-66, D-67, D-74)",
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
    let source_offset_frames = match v.get("source_offset_frames") {
        None if matches!(kind, LayerKind::Adjustment | LayerKind::Solid) => 0,
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
            as_enum(field(m, &at, "mode")?, &format!("{at}/mode"), &["alpha"])?;
            Some(MatteReference {
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
            masks.push(parse_mask(m, &format!("{at}/{i}"), i, &name, warnings)?);
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

    // Document 19's ordered effect instances. B-07 implements three of them; anything else is
    // read as `Unsupported`, which is a record this build keeps and never draws.
    let mut effects = Vec::new();
    if let Some(effects_json) = v.get("effects") {
        let at = format!("{pointer}/effects");
        for (i, effect) in as_array(effects_json, &at)?.iter().enumerate() {
            let at = format!("{at}/{i}");
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
            let known = [
                crate::effects::EXPOSURE,
                crate::effects::GAUSSIAN_BLUR,
                crate::effects::TINT,
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
                }),
                crate::effects::GAUSSIAN_BLUR => Some(crate::effects::Effect::GaussianBlur {
                    sigma_px: effect_number(params, "sigma_px", &at)?,
                }),
                crate::effects::TINT => Some(crate::effects::Effect::Tint {
                    color: effect_color(params, &at)?,
                    amount: effect_number(params, "amount", &at)?,
                }),
                _ => None,
            };
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
                    };
                    if let Some(bad) = whole.invalid() {
                        warnings.push(
                            Diagnostic::new(
                                DiagnosticId::EffectParameterInvalid,
                                Severity::Warning,
                                format!(
                                    "The layer \"{name}\" has a {type_id} whose settings this \
                                     build cannot use."
                                ),
                                format!("{} The effect is kept and bypassed.", bad.why_invalid()),
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
            effects.push(crate::effects::EffectInstance {
                instance_id: instance_id.clone(),
                enabled,
                effect: effect_value,
                tracks,
            });
        }
    }

    let blend_mode = match as_enum(
        field(v, pointer, "blend_mode")?,
        &format!("{pointer}/blend_mode"),
        &["normal", "multiply", "screen", "add"],
    )? {
        "multiply" => BlendMode::Multiply,
        "screen" => BlendMode::Screen,
        "add" => BlendMode::Add,
        _ => BlendMode::Normal,
    };
    if kind == LayerKind::Adjustment && blend_mode != BlendMode::Normal {
        return Err(invalid(
            &format!("{pointer}/blend_mode"),
            "normal, the one blend mode an adjustment layer has (D-66)",
        ));
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

/// D-77's mask record. `index` numbers the unnamed ones, as the window numbers them.
///
/// Everything outside D-77's ranges refuses the file rather than being clamped: a clamped
/// opacity is a picture nobody asked for, and document 28 would rather say no than guess.
/// What is *kept* and diagnosed instead is an outline that cannot be drawn, which the caller
/// warns about, and a path with keyframes on it, which this build carries but does not yet
/// animate.
fn parse_mask(
    v: &J,
    at: &str,
    index: usize,
    layer_name: &str,
    warnings: &mut Vec<Diagnostic>,
) -> Result<crate::mask::Mask, Diagnostic> {
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
    let number = |key: &str, default: f64| -> Result<f64, Diagnostic> {
        match v.get(key) {
            None => Ok(default),
            Some(n) => as_f64(n, &format!("{at}/{key}")),
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
    let path_at = format!("{at}/path");
    let path = field(v, at, "path")?;
    as_object(path, &path_at)?;
    let points = parse_mask_points(
        field(path, &path_at, "base")?,
        &format!("{path_at}/base"),
    )?;
    let name = match v.get("name") {
        None => format!("Mask {}", index + 1),
        Some(n) => as_str(n, &format!("{at}/name"))?.to_string(),
    };
    // B-24d animates a path. Until it does, a file that carries keys on one is read, kept and
    // drawn at its base -- and said out loud, because document 28 forbids a silent fidelity
    // fallback and `export` marks a file incomplete on exactly this identifier.
    let keys_at = format!("{path_at}/keyframes");
    if let Some(keys) = path.get("keyframes").filter(|k| !k.is_null()) {
        let keys = as_array(keys, &keys_at)?;
        for (i, key) in keys.iter().enumerate() {
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
        }
        if !keys.is_empty() {
            warnings.push(
                Diagnostic::new(
                    DiagnosticId::ProjectFeatureUnsupported,
                    Severity::Warning,
                    format!(
                        "The mask \"{name}\" on layer \"{layer_name}\" has an animated \
                         outline, which this build does not draw yet."
                    ),
                    format!(
                        "D-77 defines an animated mask path and B-24d builds it. The {} keys \
                         are kept in the project exactly as they were; every frame is drawn \
                         from the path's base, so the outline does not move.",
                        keys.len()
                    ),
                )
                .with_remediation(
                    "Nothing was lost. Saving this project writes the keys back unchanged.",
                ),
            );
        }
    }
    Ok(crate::mask::Mask {
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

    Ok(Loaded {
        document: Document::new(project),
        preserved: Preserved { root },
        warnings,
    })
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
