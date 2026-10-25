//! B-642: D-763, colours mix in display values in 8 bpc (After Effects).
//!
//! Writes `verification/B-642_display_mix_table.md`.
//!
//! Every expected pixel is `Fixtures/display_mix/expected_display_mix.json`, written by
//! `tools/display_mix_reference.py` before this code existed and printed in document 25 as
//! FX-MIX8-001 to 012: each scene in 8 bpc and the same scene in Display, whose frame is the
//! linear-light mix as before. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;

use effect_table::{largest_difference, repo, Table};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};

fn set_eight(doc: &mut Document, eight_bpc: bool) {
    let comp = doc.project().compositions[0].clone();
    doc.apply(Command::SetCompositionSettings {
        composition: comp.id.clone(),
        name: comp.name.clone(),
        width: comp.width,
        height: comp.height,
        frame_rate: comp.frame_rate,
        duration_frames: comp.duration_frames,
        sheet_details: comp.sheet_details.clone(),
        background_color: comp.background_color,
        float_depth: false,
        eight_bpc,
        ae_32bpc: false,
    })
    .unwrap_or_else(|d| panic!("the depth is set: {}", d.message));
}

/// `name`'s composition, renamed `comp-inner`, inside a new `comp-main` of the same size as its
/// only layer; `outer` and `inner` whether each is 8 bpc.
fn nested(t: &Table, name: &str, outer: bool, inner: bool) -> Document {
    let mut project: J =
        serde_json::from_str(&fs::read_to_string(t.root.join(name)).unwrap()).unwrap();
    let mut inside = project["compositions"][0].clone();
    inside["id"] = json!("comp-inner");
    inside.as_object_mut().unwrap().remove("eight_bpc");
    if inner {
        inside["eight_bpc"] = json!(true);
    }
    let (w, h) = (inside["width"].clone(), inside["height"].clone());
    let p = |v: J| json!({"base": v, "keyframes": []});
    let layer = json!({
        "id": "inner", "kind": "composition", "name": "Inner", "composition_id": "comp-inner",
        "enabled": true, "locked": false, "in_frame": 0, "out_frame": 1,
        "source_offset_frames": 0,
        "transform": {
            "anchor": p(json!([w.as_f64().unwrap() / 2.0, h.as_f64().unwrap() / 2.0])),
            "position": p(json!([w.as_f64().unwrap() / 2.0, h.as_f64().unwrap() / 2.0])),
            "scale": p(json!([100, 100])), "rotation": p(json!(0)), "opacity": p(json!(1)),
        },
        "masks": [], "matte": null, "blend_mode": "normal", "effects": [],
    });
    let mut top = inside.clone();
    top["id"] = json!("comp-main");
    top["layer_order"] = json!(["inner"]);
    top["layers"] = json!([layer]);
    top.as_object_mut().unwrap().remove("eight_bpc");
    if outer {
        top["eight_bpc"] = json!(true);
    }
    project["compositions"] = json!([top, inside]);
    persist::load_str(&project.to_string())
        .unwrap_or_else(|d| panic!("the nested file opens: {}", d.message))
        .document
}

#[test]
fn b642_display_mix() {
    let mut t = Table::new(
        "display_mix",
        "# B-642: colours mix in display values in 8 bpc (After Effects) (D-763)\n\n\
         Written by `tests/b642_display_mix.rs`. Every expected pixel is from \
         `tools/display_mix_reference.py`, which never runs the build's code.\n",
    );
    let expected: J = serde_json::from_str(
        &fs::read_to_string(t.root.join("expected_display_mix.json")).unwrap(),
    )
    .unwrap();
    let frame = |fx: &str| expected["cases"][fx]["frames"]["0"].clone();

    t.heading("FX-MIX8-001 to 012: each scene in 8 bpc, and the same scene in Display");
    t.fixtures_warned_at("expected_display_mix.json", 1..=12, 0);
    t.tiles(&[
        ("fx_mix8_001.json", 0),
        ("fx_mix8_007.json", 0),
        ("fx_mix8_009.json", 0),
        ("fx_mix8_011.json", 0),
    ]);

    t.heading("Display is unchanged: each 8 bpc scene set to Display draws the Display frame");
    for (eight, display) in [
        ("FX-MIX8-001", "FX-MIX8-002"),
        ("FX-MIX8-003", "FX-MIX8-004"),
        ("FX-MIX8-005", "FX-MIX8-006"),
        ("FX-MIX8-007", "FX-MIX8-008"),
        ("FX-MIX8-009", "FX-MIX8-010"),
        ("FX-MIX8-011", "FX-MIX8-012"),
    ] {
        let name = expected["cases"][eight]["project"].as_str().unwrap();
        let mut doc = t.load(name).document;
        set_eight(&mut doc, false);
        let d = largest_difference(&t.render(&doc, 0, 64), &frame(display));
        t.row(
            &format!("{eight} set to Display draws {display}'s frame, the linear mix"),
            &format!("largest difference {d:.1e}"),
            d <= 2e-5,
        );
        set_eight(&mut doc, true);
        let d = largest_difference(&t.render(&doc, 0, 64), &frame(eight));
        t.row(
            &format!("And set back to 8 bpc, {eight}'s frame again"),
            &format!("largest difference {d:.1e}"),
            d <= 2e-5,
        );
    }

    t.heading("A composition inside another follows the outermost one");
    for (outer, inner, want) in [
        (true, false, "FX-MIX8-001"),
        (false, true, "FX-MIX8-002"),
        (true, true, "FX-MIX8-001"),
    ] {
        let doc = nested(&t, "fx_mix8_001.json", outer, inner);
        let d = largest_difference(&t.render(&doc, 0, 64), &frame(want));
        let depth = |e: bool| if e { "8 bpc" } else { "Display" };
        t.row(
            &format!(
                "FX-MIX8-001's Screen inside a composition: outer {}, inner {}, draws {want}'s frame",
                depth(outer),
                depth(inner)
            ),
            &format!("largest difference {d:.1e}"),
            d <= 2e-5,
        );
    }

    t.heading("Old 8 bpc projects open as they are and draw their D-330 frames (one layer, nothing beneath)");
    let old_root = repo("Fixtures/eight_bpc");
    let old: J = serde_json::from_str(
        &fs::read_to_string(old_root.join("expected_eight_bpc.json")).unwrap(),
    )
    .unwrap();
    for (fx, case) in old["cases"].as_object().unwrap() {
        let file = case["project"].as_str().unwrap();
        let loaded = persist::load(&old_root.join(file))
            .unwrap_or_else(|d| panic!("{file} opens: {}", d.message));
        let mut log = FrameLog::new(8);
        let drawn = render_frame(
            loaded.document.project(),
            &Id::new("comp-main"),
            0,
            &old_root,
            64,
            &mut log,
        )
        .unwrap();
        let d = largest_difference(&drawn, &case["frames"]["0"]);
        t.row(
            &format!("{fx} ({file}), saved before D-763 with no new line"),
            &format!(
                "opens with {} warnings, largest difference {d:.1e}",
                loaded.warnings.len()
            ),
            loaded.warnings.is_empty() && d <= 2e-5,
        );
    }

    t.heading("The viewer, and the card");
    let project = t.load("fx_mix8_007.json").document.project().clone();
    let comp = Id::new("comp-main");
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let cpu = preview::preview_frame_cached(
        &project,
        &comp,
        0,
        &t.root,
        PreviewQuality::Full,
        DEFAULT_TILE_SIZE,
        &mut log,
        &mut cache,
    )
    .unwrap();
    let d = largest_difference(&cpu, &frame("FX-MIX8-007"));
    t.row(
        "FX-MIX8-007 (Drop Shadow 50% over white) in the viewer",
        &format!("largest difference {d:.1e}"),
        d <= 2e-5,
    );
    match Gpu::new() {
        Err(why) => t.row(
            "The preview card draws an 8 bpc frame (B-688)",
            &format!("NOT RUN: no usable card ({why})"),
            false,
        ),
        Ok(mut gpu) => {
            for name in ["fx_mix8_001.json", "fx_mix8_007.json", "fx_mix8_002.json"] {
                let project = t.load(name).document.project().clone();
                let mut log = FrameLog::new(3);
                let cpu = preview::preview_frame_cached(
                    &project,
                    &comp,
                    0,
                    &t.root,
                    PreviewQuality::Full,
                    DEFAULT_TILE_SIZE,
                    &mut log,
                    &mut cache,
                )
                .unwrap()
                .to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (card, ..) = preview::preview_frame_srgb8(
                    &project,
                    &comp,
                    0,
                    &t.root,
                    PreviewQuality::Full,
                    DEFAULT_TILE_SIZE,
                    &mut log,
                    &mut cache,
                    &mut gpu,
                )
                .unwrap();
                let on_cpu = log
                    .finish()
                    .iter()
                    .any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str());
                let worst = cpu
                    .iter()
                    .zip(&card)
                    .map(|(a, b)| a.abs_diff(*b))
                    .max()
                    .unwrap_or(0);
                let eight = project.compositions[0].eight_bpc;
                t.row(
                    &format!(
                        "{name} ({}): the preview draws the CPU's picture within 1 level of 255, {}",
                        if eight { "8 bpc" } else { "Display" },
                        if eight { "on the card (B-688)" } else { "the card free to draw it" }
                    ),
                    &format!(
                        "{}, largest difference {worst} of 255",
                        if on_cpu { "CPU" } else { "card" }
                    ),
                    worst <= 1 && (!eight || !on_cpu),
                );
            }
        }
    }

    t.finish("B-642_display_mix_table.md");
}
