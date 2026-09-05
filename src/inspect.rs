//! A deterministic JSON view of the model, for the owner to read.
//!
//! Document 15 asks B-05 for "a before, after and undone project JSON the owner can diff by
//! eye", and document 04 asks B-05 for a "minimal inspection surface only, no real interface".
//! This is that surface.
//!
//! **This is not the save format.** Persistence is B-09 (ADR-008, R-08), and it owns schema
//! versioning, atomic replacement, migration and unknown-record preservation. What is written
//! here is shaped to match `Schemas/project-v0.schema.json` so the two can be compared later,
//! but nothing reads it back, and no test here claims a round trip.
//!
//! Output is deterministic: keys are emitted in schema order, layers in composition order,
//! and floats print in Rust's shortest round-trip form, so two dumps of equal models are equal
//! byte for byte and `diff` says nothing when nothing changed.

use std::fmt::Write as _;

use crate::model::{Layer, Project, Property, Transform, Value};

/// The whole project, as pretty-printed JSON.
pub fn project_json(project: &Project) -> String {
    let mut s = String::new();
    s.push_str("{\n");
    let _ = writeln!(s, "  \"schema_version\": 0,");
    let _ = writeln!(s, "  \"project_id\": {},", quote(project.id.as_str()));
    s.push_str("  \"color_settings\": {\n");
    s.push_str("    \"working_space\": \"linear-srgb\",\n");
    s.push_str("    \"alpha_mode\": \"premultiplied\"\n");
    s.push_str("  },\n");

    s.push_str("  \"assets\": [");
    for (i, asset) in project.assets.iter().enumerate() {
        s.push_str(if i == 0 { "\n" } else { ",\n" });
        s.push_str("    {\n");
        let _ = writeln!(s, "      \"id\": {},", quote(asset.id.as_str()));
        s.push_str("      \"kind\": \"image_sequence\",\n");
        let _ = writeln!(s, "      \"name\": {},", quote(&asset.name));
        let _ = writeln!(s, "      \"pattern\": {},", quote(&asset.pattern));
        s.push_str(
            "      \"interpretation\": { \"color_space\": \"srgb\", \"alpha\": \"straight\" }\n",
        );
        s.push_str("    }");
    }
    s.push_str(if project.assets.is_empty() {
        "],\n"
    } else {
        "\n  ],\n"
    });

    s.push_str("  \"compositions\": [");
    for (i, comp) in project.compositions.iter().enumerate() {
        s.push_str(if i == 0 { "\n" } else { ",\n" });
        s.push_str("    {\n");
        let _ = writeln!(s, "      \"id\": {},", quote(comp.id.as_str()));
        let _ = writeln!(s, "      \"name\": {},", quote(&comp.name));
        let _ = writeln!(s, "      \"width\": {},", comp.width);
        let _ = writeln!(s, "      \"height\": {},", comp.height);
        s.push_str("      \"pixel_aspect_ratio\": 1,\n");
        let _ = writeln!(
            s,
            "      \"frame_rate\": {{ \"numerator\": {}, \"denominator\": {} }},",
            comp.frame_rate.numerator(),
            comp.frame_rate.denominator()
        );
        let _ = writeln!(s, "      \"start_frame\": {},", comp.start_frame);
        let _ = writeln!(s, "      \"duration_frames\": {},", comp.duration_frames);
        let order: Vec<String> = comp
            .layer_order()
            .iter()
            .map(|id| quote(id.as_str()))
            .collect();
        let _ = writeln!(s, "      \"layer_order\": [{}],", order.join(", "));
        s.push_str("      \"layers\": [");
        for (j, layer) in comp.layers_in_order().enumerate() {
            s.push_str(if j == 0 { "\n" } else { ",\n" });
            s.push_str(&layer_json(layer));
        }
        s.push_str(if comp.is_empty() {
            "]\n"
        } else {
            "\n      ]\n"
        });
        s.push_str("    }");
    }
    s.push_str(if project.compositions.is_empty() {
        "]\n"
    } else {
        "\n  ]\n"
    });
    s.push_str("}\n");
    s
}

fn layer_json(layer: &Layer) -> String {
    let mut s = String::new();
    s.push_str("        {\n");
    let _ = writeln!(s, "          \"id\": {},", quote(layer.id.as_str()));
    s.push_str("          \"kind\": \"raster\",\n");
    let _ = writeln!(s, "          \"name\": {},", quote(&layer.name));
    let _ = writeln!(
        s,
        "          \"asset_id\": {},",
        quote(layer.asset_id.as_str())
    );
    let _ = writeln!(s, "          \"enabled\": {},", layer.enabled);
    let _ = writeln!(s, "          \"locked\": {},", layer.locked);
    let _ = writeln!(s, "          \"in_frame\": {},", layer.in_frame);
    let _ = writeln!(s, "          \"out_frame\": {},", layer.out_frame);
    let _ = writeln!(
        s,
        "          \"source_offset_frames\": {},",
        layer.source_offset_frames
    );
    s.push_str(&transform_json(&layer.transform));
    if !layer.exposure_spans.is_empty() {
        s.push_str("          \"exposure_spans\": [\n");
        let spans: Vec<String> = layer
            .exposure_spans
            .iter()
            .map(|sp| {
                format!(
                    "            {{ \"start_frame\": {}, \"end_frame_exclusive\": {}, \"drawing_number\": {} }}",
                    sp.start_frame, sp.end_frame_exclusive, sp.drawing_number
                )
            })
            .collect();
        s.push_str(&spans.join(",\n"));
        s.push_str("\n          ],\n");
    }
    match &layer.matte {
        Some(m) => {
            let _ = writeln!(
                s,
                "          \"matte\": {{ \"layer_id\": {}, \"mode\": \"alpha\" }},",
                quote(m.layer_id.as_str())
            );
        }
        None => s.push_str("          \"matte\": null,\n"),
    }
    let _ = writeln!(
        s,
        "          \"blend_mode\": {},",
        quote(layer.blend_mode.as_str())
    );
    // Effects are B-06. A layer with no effect stack genuinely has none, so an empty array is
    // accurate rather than a placeholder.
    s.push_str("          \"effects\": []\n");
    s.push_str("        }");
    s
}

fn transform_json(t: &Transform) -> String {
    let mut s = String::new();
    s.push_str("          \"transform\": {\n");
    let parts = [
        ("anchor", &t.anchor),
        ("position", &t.position),
        ("scale", &t.scale),
        ("rotation", &t.rotation),
        ("opacity", &t.opacity),
    ];
    for (i, (name, prop)) in parts.iter().enumerate() {
        let comma = if i + 1 == parts.len() { "" } else { "," };
        let _ = writeln!(
            s,
            "            {}: {}{comma}",
            quote(name),
            property_json(prop)
        );
    }
    s.push_str("          },\n");
    s
}

fn property_json(p: &Property) -> String {
    let keys: Vec<String> = p
        .keyframes()
        .iter()
        .map(|k| {
            format!(
                "{{ \"frame\": {}, \"value\": {}, \"interp\": {} }}",
                k.frame,
                value_json(k.value),
                quote(k.interp.as_str())
            )
        })
        .collect();
    if keys.is_empty() {
        format!(
            "{{ \"base\": {}, \"keyframes\": [] }}",
            value_json(p.base())
        )
    } else {
        format!(
            "{{ \"base\": {}, \"keyframes\": [{}] }}",
            value_json(p.base()),
            keys.join(", ")
        )
    }
}

fn value_json(v: Value) -> String {
    match v {
        Value::Scalar(n) => number(n),
        Value::Vec2(x, y) => format!("[{}, {}]", number(x), number(y)),
    }
}

/// JSON has one number type, so `1.0` and `1` are the same value. Printing the shorter form
/// keeps the owner's diff about what actually changed.
fn number(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{n}")
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
