//! B-750 (D-870): in 8 bpc (After Effects) the card draws Exposure, Drop Shadow, Fill, Solid
//! Composite and Bloom as the CPU does. Glow and Echo stay the CPU's.
//!
//! Every fixture project holding one of the five, each composition set to 8 bpc, drawn by the
//! CPU and by the card at its first and middle frames, Full and Draft: as saved, with each of
//! the five at Mix 50, and with every Exposure's Bypass Linear Light Conversion on. The frames
//! the card draws are held to wave 8's bar; a frame the card refuses must be the CPU's exactly.
//!
//! Writes `verification/B-750_eight_card_effects_table.md`.

use anime_compositor::gpu::Gpu;
use serde_json::Value;

mod effect_table;
mod eight_walk;
use effect_table::Table;

const FIVE: [(&str, &str); 5] = [
    ("\"core.exposure\"", "Exposure"),
    ("\"core.drop_shadow\"", "Drop Shadow"),
    ("\"core.fill\"", "Fill"),
    ("\"core.solid_composite\"", "Solid Composite"),
    ("\"core.bloom\"", "Bloom"),
];

/// Every effect instance in the project.
fn each(j: &mut Value, f: &mut impl FnMut(&mut Value)) {
    if j.get("type_id").is_some() && j.get("parameters").is_some() {
        f(j);
    }
    match j {
        Value::Object(o) => o.values_mut().for_each(|v| each(v, f)),
        Value::Array(a) => a.iter_mut().for_each(|v| each(v, f)),
        _ => {}
    }
}

fn of_five(i: &Value) -> bool {
    let t = i["type_id"].to_string();
    FIVE.iter().any(|(id, _)| *id == t)
}

#[test]
fn b750_eight_card_effects() {
    let mut gpu = Gpu::new().expect("a usable card");
    let pick = |t: &str| FIVE.iter().any(|(id, _)| t.contains(id));
    let groups = |t: &str| {
        FIVE.iter()
            .filter(|(id, _)| t.contains(id))
            .map(|(_, n)| n.to_string())
            .collect::<Vec<_>>()
    };
    let mut t = Table::new(
        "effect_mix",
        &format!(
            "# B-750 (D-870): Exposure, Drop Shadow, Fill, Solid Composite and Bloom in 8 bpc on the card\n\n\
             Card: {}. Every fixture project holding one of the five, each composition set to \
             8 bpc (After Effects), drawn by the CPU and by the card at its first and middle \
             frames, Full and Draft. The bar: within 1 level of 255 on at least 99.9% of pixels \
             and within 2 everywhere. A frame the card refuses must be the CPU's exactly. Each \
             row counts every frame of a fixture holding the effect; \"on the card\" counts those \
             whose plan leaves that effect to the card.\n",
            gpu.about()
        ),
    );
    let as_saved = |_: &mut Value| {};
    let mix50 = |j: &mut Value| {
        each(j, &mut |i| {
            if of_five(i) {
                i["mix"] = Value::from(50);
            }
        })
    };
    let bypass = |j: &mut Value| {
        each(j, &mut |i| {
            if i["type_id"] == "core.exposure" {
                i["parameters"]["bypass"] = Value::from("on");
            }
        })
    };
    let mut worst = Vec::new();
    for (label, edit) in [
        ("as saved", &as_saved as &dyn Fn(&mut Value)),
        ("each of the five at Mix 50", &mix50),
        ("every Exposure with Bypass on", &bypass),
    ] {
        let w = eight_walk::walk(&mut gpu, pick, edit, groups);
        t.heading(&format!("The frames the card drew, {label}"));
        for (_, n) in FIVE {
            if label.starts_with("every Exposure") && n != "Exposure" {
                continue;
            }
            let s = w.drawn.get(n).cloned().unwrap_or_default();
            t.row(n, &s.text(true), s.on_card > 0 && s.ok());
        }
        t.heading(&format!("The frames the card refused, {label}"));
        for (why, (n, exact)) in &w.refused {
            t.row(why, &format!("{n} frames, {exact} of them the CPU's exactly"), n == exact);
        }
        worst.extend(w.worst.iter().map(|s| format!("{label}: {s}")));
    }
    t.heading("Frames with a pixel over 1 level");
    let text = if worst.is_empty() { "none".to_string() } else { worst.join("; ") };
    t.row("all three", &text, true);
    t.finish("B-750_eight_card_effects_table.md");
}
