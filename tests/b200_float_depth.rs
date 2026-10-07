//! B-200: D-319, a composition's working depth, Display or Float (After Effects' 8 and 32 bpc).
//!
//! Writes `verification/D-319_float_depth_table.md`.
//!
//! Every expected pixel is `Fixtures/float_depth/expected_float_depth.json`, written by
//! `tools/float_depth_reference.py` before this code existed and printed in document 25 as
//! FX-BLEND-ADDF-001 to 003, FX-BLEND-SCRF-001 to 003 and FX-FNOISE-HDR-001 to 003. Tolerance
//! 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{largest_difference, repo, Table};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use serde_json::{json, Value as J};

fn set(doc: &mut Document, float_depth: bool) {
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
        float_depth,
        eight_bpc: false,
        ae_32bpc: false,
    })
    .unwrap_or_else(|d| panic!("{}", d.message));
}

fn saved(doc: &Document) -> J {
    let text = persist::to_json(doc.project(), &Default::default());
    serde_json::from_str::<J>(&text).unwrap()["compositions"][0].clone()
}

fn file(t: &Table, name: &str) -> J {
    serde_json::from_str(&std::fs::read_to_string(t.root.join(name)).unwrap()).unwrap()
}

/// `name`'s composition, renamed `comp-inner`, inside a new `comp-main` of the same size as
/// its only layer, unmoved; `outer` and `inner` the two depths.
fn nested(t: &Table, name: &str, outer: bool, inner: bool) -> Document {
    let mut project = file(t, name);
    let mut inside = project["compositions"][0].clone();
    inside["id"] = json!("comp-inner");
    inside["name"] = json!("Inner");
    let o = inside.as_object_mut().unwrap();
    o.remove("float_depth");
    if inner {
        o.insert("float_depth".into(), json!(true));
    }
    let mut layer = inside["layers"][0].clone();
    let l = layer.as_object_mut().unwrap();
    l.remove("asset_id");
    l.remove("exposure_spans");
    l.remove("effects");
    l.insert("kind".into(), json!("composition"));
    l.insert("composition_id".into(), json!("comp-inner"));
    l.insert("id".into(), json!("inner"));
    l.insert("name".into(), json!("Inner"));
    let mut top = inside.clone();
    top["id"] = json!("comp-main");
    top["name"] = json!("Main");
    top["layer_order"] = json!(["inner"]);
    top["layers"] = json!([layer]);
    top.as_object_mut().unwrap().remove("float_depth");
    if outer {
        top["float_depth"] = json!(true);
    }
    project["compositions"] = json!([top, inside]);
    persist::load_str(&project.to_string())
        .unwrap_or_else(|d| panic!("the nested file opens: {}", d.message))
        .document
}

#[test]
fn b200_float_depth() {
    let mut t = Table::new(
        "float_depth",
        "# D-319: Float working depth\n\n\
         Written by `tests/b200_float_depth.rs`. A composition's working depth is Display (0 to \
         1, every file before D-319) or Float (past white), as After Effects' 8 and 32 bpc. In \
         Float, Add is not held to 1, Screen keeps the brighter where both are past white \
         (Nuke's rule), and Fractal Noise goes past white. A composition inside another draws \
         in the outermost one's depth. Every expected pixel is \
         `Fixtures/float_depth/expected_float_depth.json`, written by \
         `tools/float_depth_reference.py` before the build had Float. Tolerance 2e-5.\n",
    );

    t.heading("The fixtures");
    t.fixtures("expected_float_depth.json");
    t.round_trips(&["fx_blend_addf_001.json", "fx_blend_addf_002.json"]);

    t.heading("Saved, read back and undone");
    let mut doc = t.load("fx_blend_addf_002.json").document;
    let none = saved(&doc).get("float_depth").is_none();
    t.row("A file without the setting is Display and saves no float_depth line", if none { "no line" } else { "a line" }, none && !doc.project().compositions[0].float_depth);
    set(&mut doc, true);
    let line = saved(&doc).get("float_depth").cloned();
    t.row("Set to Float, it is written as float_depth: true", &format!("{line:?}"), line == Some(json!(true)));
    let back = persist::load_str(&persist::to_json(doc.project(), &Default::default())).unwrap();
    let on = back.document.project().compositions[0].float_depth;
    t.row("And read back as Float", if on { "Float" } else { "Display" }, on);
    let frame = t.render(&doc, 0, 64);
    let expected = file(&t, "expected_float_depth.json");
    let d = largest_difference(&frame, &expected["cases"]["FX-BLEND-ADDF-001"]["frames"]["0"]);
    t.row("FX-BLEND-ADDF-002 set to Float through the command draws FX-BLEND-ADDF-001's frame", &format!("largest difference {d:.1e}"), d <= 2e-5);
    doc.undo().expect("one step to undo");
    let off = !doc.project().compositions[0].float_depth && saved(&doc).get("float_depth").is_none();
    t.row("Undo puts it back to Display, and nothing is written", if off { "Display, no line" } else { "not put back" }, off);
    let d = largest_difference(&t.render(&doc, 0, 64), &expected["cases"]["FX-BLEND-ADDF-002"]["frames"]["0"]);
    t.row("And the frame is FX-BLEND-ADDF-002's again", &format!("largest difference {d:.1e}"), d <= 2e-5);
    set(&mut doc, true);
    set(&mut doc, false);
    let off = saved(&doc).get("float_depth").is_none();
    t.row("Set back to Display, no float_depth line is written", if off { "no line" } else { "a line" }, off);

    let mut bad = file(&t, "fx_blend_addf_001.json");
    for word in [json!("yes"), json!(1), json!(null)] {
        bad["compositions"][0]["float_depth"] = word.clone();
        let refused = persist::load_str(&bad.to_string()).err().map(|d| d.id.as_str().to_string());
        t.row(&format!("A file with float_depth {word} is refused, not guessed"), &format!("{refused:?}"), refused.as_deref() == Some("PROJECT_SCHEMA_INVALID"));
    }

    t.heading("A composition inside another follows the outermost one");
    for (outer, inner, case) in [(true, false, "FX-BLEND-ADDF-001"), (false, true, "FX-BLEND-ADDF-002"), (true, true, "FX-BLEND-ADDF-001")] {
        let doc = nested(&t, "fx_blend_addf_002.json", outer, inner);
        let d = largest_difference(&t.render(&doc, 0, 64), &expected["cases"][case]["frames"]["0"]);
        let depth = |f: bool| if f { "Float" } else { "Display" };
        t.row(
            &format!("FX-BLEND-ADDF's layers inside a composition: outer {}, inner {}, draws {case}'s frame", depth(outer), depth(inner)),
            &format!("largest difference {d:.1e}"),
            d <= 2e-5,
        );
    }

    t.heading("The preview card");
    match Gpu::new() {
        Err(why) => t.row("The card hands a Float frame to the CPU", &format!("NOT RUN: no usable card ({why})"), false),
        Ok(mut gpu) => {
            for (name, float) in [("fx_blend_addf_001.json", true), ("fx_blend_addf_002.json", false), ("fx_fnoise_hdr_001.json", true)] {
                let project = t.load(name).document.project().clone();
                let comp = Id::new("comp-main");
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let cpu = preview::preview_frame_cached(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache).unwrap().to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (card, ..) = preview::preview_frame_srgb8(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).unwrap();
                let said: Vec<String> = log.finish().iter().map(|d| d.id.as_str().to_string()).collect();
                let on_cpu = said.iter().any(|s| s == DiagnosticId::GpuPreviewOnCpu.as_str());
                let worst = cpu.iter().zip(&card).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
                let ok = if float { on_cpu && worst == 0 } else { !on_cpu && worst <= 1 };
                t.row(
                    &format!("{name}: {}", if float { "Float, drawn by the CPU, the CPU's picture exactly, and said as GPU_PREVIEW_ON_CPU" } else { "Display, drawn on the card as before, within 1 level of 255" }),
                    &format!("{}, largest difference {worst} of 255", if on_cpu { "CPU" } else { "card" }),
                    ok,
                );
            }
        }
    }

    t.heading("Files from before D-319");
    let shot = persist::load(&repo("verification/B-08a_project.json")).unwrap();
    let text = persist::to_json(shot.document.project(), &shot.preserved);
    let none = !text.contains("float_depth") && shot.document.project().compositions.iter().all(|c| !c.float_depth);
    t.row("The reference shot is Display in every composition and saves no float_depth line", if none { "Display, no line" } else { "differs" }, none);

    t.finish("D-319_float_depth_table.md");
}
