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
use std::time::SystemTime;

use serde_json::{Map, Value as J};

use crate::command::{Command, Document};
use crate::diagnostics::{Diagnostic, DiagnosticId, Severity};
use crate::media::{self, SequenceAsset};
use crate::model::{
    Asset, AssetKind, BlendMode, Composition, Id, Interp, Interpretation, Keyframe, Layer,
    MatteReference, Project, Prop, Property, Value,
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
    "path",
    "pattern",
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
    "layer_order",
    "layers",
    "asset_id",
    "enabled",
    "locked",
    "in_frame",
    "out_frame",
    "source_offset_frames",
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
    "inverted",
    "vertices",
    "matte",
    "layer_id",
    "mode",
    "blend_mode",
    "effects",
    "instance_id",
    "type_id",
    "parameters",
];

/// An effect record is the one place a flat list is not enough: it spells `enabled` after
/// `type_id`, while a layer spells it near the top, so the two cannot share a ranking. Effect
/// records are recognised by `instance_id`, which nothing else in the schema has.
const EFFECT_KEY_ORDER: &[&str] = &["instance_id", "type_id", "enabled", "parameters"];

fn order_for(map: &Map<String, J>) -> &'static [&'static str] {
    if map.contains_key("instance_id") {
        EFFECT_KEY_ORDER
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
    let keyframes: Vec<J> = property
        .keyframes()
        .iter()
        .map(|k| {
            let mut m = Map::new();
            m.insert("frame".into(), J::from(k.frame));
            m.insert("value".into(), value_json(k.value, factor));
            m.insert("interp".into(), J::from(k.interp.as_str()));
            J::Object(m)
        })
        .collect();
    merge(
        base,
        vec![
            ("base", value_json(property.base(), factor)),
            ("keyframes", J::Array(keyframes)),
        ],
    )
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
    match &asset.path {
        Some(p) => owned.push(("path", J::from(p.as_str()))),
        None => owned.push(("path", J::Null)),
    }
    match &asset.pattern {
        Some(p) => owned.push(("pattern", J::from(p.as_str()))),
        None => owned.push(("pattern", J::Null)),
    }
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
            J::Object(map)
        }
        None => J::Null,
    };
    let mut owned = vec![
        ("id", J::from(layer.id.as_str())),
        ("kind", J::from("raster")),
        ("name", J::from(layer.name.as_str())),
        ("asset_id", J::from(layer.asset_id.as_str())),
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
    // Effects are B-06. An effect stack read from a file is preserved verbatim by the merge;
    // a layer created here genuinely has none, and the schema requires the key.
    if base.and_then(|b| b.get("effects")).is_none() {
        owned.push(("effects", J::Array(Vec::new())));
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
    merge(
        base,
        vec![
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
        ],
    )
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

fn parse_property(v: &J, pointer: &str, kind: &str, factor: f64) -> Result<Property, Diagnostic> {
    as_object(v, pointer)?;
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
            &["hold", "linear"],
        )? {
            "hold" => Interp::Hold,
            _ => Interp::Linear,
        };
        property.set_keyframe(Keyframe {
            frame,
            value,
            interp,
        });
    }
    Ok(property)
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
        &["still", "image_sequence"],
    )? {
        "still" => AssetKind::Still,
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
        interpretation: parse_interpretation(
            field(v, pointer, "interpretation")?,
            &format!("{pointer}/interpretation"),
        )?,
    })
}

fn parse_layer(v: &J, pointer: &str, warnings: &mut Vec<Diagnostic>) -> Result<Layer, Diagnostic> {
    as_object(v, pointer)?;
    as_enum(
        field(v, pointer, "kind")?,
        &format!("{pointer}/kind"),
        &["raster"],
    )?;
    let id = as_id(field(v, pointer, "id")?, &format!("{pointer}/id"))?;
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
        *transform.get_mut(prop) = parse_property(
            field(transform_json, &transform_at, prop.as_str())?,
            &at,
            prop.kind(),
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

    let matte = match v.get("matte") {
        None | Some(J::Null) => None,
        Some(m) => {
            let at = format!("{pointer}/matte");
            as_object(m, &at)?;
            as_enum(field(m, &at, "mode")?, &format!("{at}/mode"), &["alpha"])?;
            Some(MatteReference {
                layer_id: as_id(field(m, &at, "layer_id")?, &format!("{at}/layer_id"))?,
            })
        }
    };

    if let Some(mask) = v.get("mask") {
        if !mask.is_null() {
            warnings.push(
                Diagnostic::new(
                    DiagnosticId::ProjectFeatureUnsupported,
                    Severity::Warning,
                    format!("The layer \"{name}\" has a mask, which this build cannot draw."),
                    format!(
                        "Masks are parked with requirement R-04 in document 23. The mask record \
                         on layer {id} is kept in the project exactly as it was and takes no \
                         part in rendering."
                    ),
                )
                .with_remediation(
                    "Nothing was lost. Saving this project writes the mask back unchanged.",
                ),
            );
        }
    }

    if let Some(effects) = v.get("effects") {
        let at = format!("{pointer}/effects");
        for (i, effect) in as_array(effects, &at)?.iter().enumerate() {
            let at = format!("{at}/{i}");
            as_object(effect, &at)?;
            let type_id = as_str(field(effect, &at, "type_id")?, &format!("{at}/type_id"))?;
            let instance_id = as_str(
                field(effect, &at, "instance_id")?,
                &format!("{at}/instance_id"),
            )?;
            warnings.push(
                Diagnostic::new(
                    DiagnosticId::EffectUnsupported,
                    Severity::Warning,
                    format!(
                        "The layer \"{name}\" uses the effect \"{type_id}\", which this build \
                         does not have."
                    ),
                    format!(
                        "Effect instance {instance_id} of type {type_id} on layer {id} is kept \
                         in the project exactly as it was and is bypassed when rendering."
                    ),
                )
                .with_remediation(
                    "Nothing was lost. Saving this project writes the effect back unchanged, \
                     but any frame rendered here is missing what it would have done.",
                ),
            );
        }
    }

    Ok(Layer {
        id,
        name,
        asset_id: as_id(
            field(v, pointer, "asset_id")?,
            &format!("{pointer}/asset_id"),
        )?,
        enabled: as_bool(field(v, pointer, "enabled")?, &format!("{pointer}/enabled"))?,
        locked: as_bool(field(v, pointer, "locked")?, &format!("{pointer}/locked"))?,
        in_frame,
        out_frame,
        source_offset_frames: as_i32(
            field(v, pointer, "source_offset_frames")?,
            &format!("{pointer}/source_offset_frames"),
        )?,
        transform,
        exposure_spans,
        matte,
        blend_mode: match as_enum(
            field(v, pointer, "blend_mode")?,
            &format!("{pointer}/blend_mode"),
            &["normal", "multiply", "screen", "add"],
        )? {
            "multiply" => BlendMode::Multiply,
            "screen" => BlendMode::Screen,
            "add" => BlendMode::Add,
            _ => BlendMode::Normal,
        },
    })
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
            if !project.assets.iter().any(|a| a.id == layer.asset_id) {
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
        Ok(()) => Ok(path),
        Err(e) => {
            let _ = fs::remove_file(&temp);
            Err(save_failed(&path, e.to_string()))
        }
    }
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
fn stored_path(project_dir: &Path, file: &Path) -> String {
    let relative = file.strip_prefix(project_dir).unwrap_or(file);
    relative
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
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
        interpretation: existing.interpretation,
    };

    Ok(RelinkCandidate {
        asset_id: asset_id.clone(),
        pattern: sequence.pattern().to_string(),
        range: sequence.range(),
        missing: sequence.missing(),
        width: sequence.width(),
        height: sequence.height(),
        interpretation: existing.interpretation,
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
