//! B-214: D-333, the 32 bpc (After Effects) working depth.
//!
//! Writes `verification/D-333_ae_32bpc_table.md`.
//!
//! Every expected pixel is `Fixtures/ae_32bpc/expected_ae_32bpc.json`, written by
//! `tools/ae_32bpc_reference.py` before this code existed and printed in document 25 as
//! FX-AE32-001 to 006. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws P-26 tutorial 2's Lightning Diff copy after step F, from this program's own
//! frame 20 of the bolt, in Float and in 32 bpc (After Effects), into `verification/D-333 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{largest_difference, repo, Table};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

fn set(doc: &mut Document, float_depth: bool, ae_32bpc: bool) -> Result<(), String> {
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
        ae_32bpc,
    })
    .map(|_| ())
    .map_err(|d| d.message)
}

fn saved(doc: &Document) -> J {
    let text = persist::to_json(doc.project(), &Default::default());
    serde_json::from_str::<J>(&text).unwrap()["compositions"][0].clone()
}

fn file(t: &Table, name: &str) -> J {
    serde_json::from_str(&fs::read_to_string(t.root.join(name)).unwrap()).unwrap()
}

/// `depth` on a composition record: "Display", "Float" or "AE".
fn depth(comp: &mut J, depth: &str) {
    let c = comp.as_object_mut().unwrap();
    c.remove("float_depth");
    c.remove("ae_32bpc");
    if depth != "Display" {
        c.insert("float_depth".into(), json!(true));
    }
    if depth == "AE" {
        c.insert("ae_32bpc".into(), json!(true));
    }
}

/// `name`'s composition, renamed `comp-inner`, inside a new `comp-main` of the same size as its
/// only layer, unmoved; `outer` and `inner` each composition's depth.
fn nested(t: &Table, name: &str, outer: &str, inner: &str) -> Document {
    let mut project = file(t, name);
    let mut inside = project["compositions"][0].clone();
    inside["id"] = json!("comp-inner");
    inside["name"] = json!("Inner");
    depth(&mut inside, inner);
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
    depth(&mut top, outer);
    project["compositions"] = json!([top, inside]);
    persist::load_str(&project.to_string())
        .unwrap_or_else(|d| panic!("the nested file opens: {}", d.message))
        .document
}

/// Tutorial 2's Lightning Diff copy after step F, on this program's frame 20 of the bolt over
/// black (`bolt_f20.png`, 1920 by 1080): Linear Wipe 70% from the top, Fast Box Blur 101.2 across
/// and 121.3 down (3 passes each), Solid Composite on black, Exposure +20.49, Linear Wipe 69%
/// feathered 32.2; 8-bit over black, and how many pixels are brighter than a quarter (linear).
fn diff_copy(dir: &Path, how: &str) -> (Vec<u8>, usize, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/ae_32bpc/fx_ae32_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("bolt_f20.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(1920);
    comp["height"] = J::from(1080);
    depth(comp, how);
    let layer = &mut comp["layers"][0];
    let middle = json!([960.0, 540.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    let wipe = |id: &str, completion: f64, feather: f64| {
        json!({ "instance_id": id, "type_id": "core.linear_wipe", "enabled": true,
                "parameters": { "completion": completion, "angle": 180, "feather": feather } })
    };
    let boxed = |id: &str, radius: f64, dimensions: &str| {
        json!({ "instance_id": id, "type_id": "core.fast_box_blur", "enabled": true,
                "parameters": { "radius": radius, "iterations": 3, "dimensions": dimensions } })
    };
    layer["effects"] = json!([
        wipe("fx-0-0", 70.0, 0.0),
        boxed("fx-0-1", 101.2, "horizontal"),
        boxed("fx-0-2", 121.3, "vertical"),
        { "instance_id": "fx-0-3", "type_id": "core.solid_composite", "enabled": true,
          "parameters": { "source_opacity": 100, "color": "#000000", "opacity": 100, "blend": "normal" } },
        { "instance_id": "fx-0-4", "type_id": "core.exposure", "enabled": true, "parameters": { "stops": 20.49 } },
        wipe("fx-0-5", 69.0, 32.2),
    ]);
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    let bright = frame.data().chunks(4).filter(|p| p[..3].iter().any(|&v| v >= 0.25)).count();
    let over = |c: f32| (anime_compositor::color::linear_to_srgb(c.clamp(0.0, 1.0)) * 255.0).round() as u8;
    let black = frame.data().chunks(4).flat_map(|p| [over(p[0]), over(p[1]), over(p[2]), 255]).collect();
    (black, bright, said)
}

#[test]
fn b214_ae_32bpc() {
    let mut t = Table::new(
        "ae_32bpc",
        "# D-333: 32 bpc (After Effects) working depth\n\nFrom P-26: after its step F tutorial 2 \
         is in After Effects' 32 bpc, which by default has no linear working space. Its Lightning \
         Diff copy blurs the bolt's tips wide and lifts them 20 stops with Exposure; in Float this \
         program lit a block where the tutorial shows a soft pool. The working depth gains a \
         fourth choice, 32 bpc (After Effects): Float, except that the four blurs average display \
         colours and Exposure works through a 2.2 curve, which lifts faint light far less. Which \
         curve After Effects uses is not written anywhere found; the 2.2 curve is this program's \
         best reading. Every expected pixel is `Fixtures/ae_32bpc/expected_ae_32bpc.json`, written \
         by `tools/ae_32bpc_reference.py` before the build had this depth. Tolerance 2e-5.\n",
    );

    t.heading("FX-AE32-001 to 006 (document 25)");
    t.fixtures("expected_ae_32bpc.json");
    t.round_trips(&["fx_ae32_001.json", "fx_ae32_004.json", "fx_ae32_006.json"]);
    let expected = file(&t, "expected_ae_32bpc.json");
    for (name, load) in expected["loads"].as_object().unwrap() {
        let text = fs::read_to_string(t.root.join(name)).unwrap();
        let refused = persist::load_str(&text).err().map(|d| d.id.as_str().to_string());
        t.row(
            &format!("{name}: {}", load["says"].as_str().unwrap()),
            &format!("{refused:?}"),
            refused.as_deref() == load["refused"].as_str(),
        );
    }

    t.heading("Saved, read back and undone");
    let one = &expected["cases"]["FX-AE32-001"]["frames"]["0"];
    let mut doc = t.load("fx_ae32_001.json").document;
    set(&mut doc, true, false).unwrap();
    let off = saved(&doc).get("ae_32bpc").is_none();
    t.row("Set to Float, no ae_32bpc line is written", if off { "no line" } else { "a line" }, off);
    let d = largest_difference(&t.render(&doc, 0, 64), one);
    t.row("And FX-AE32-001 is drawn as Float draws it, not as expected here", &format!("largest difference {d:.1e}"), d > 1e-3);
    set(&mut doc, true, true).unwrap();
    let line = saved(&doc).get("ae_32bpc").cloned();
    t.row("Set to 32 bpc (After Effects) through the command, it is written as ae_32bpc: true", &format!("{line:?}"), line == Some(json!(true)));
    let d = largest_difference(&t.render(&doc, 0, 64), one);
    t.row("And draws FX-AE32-001's frame again", &format!("largest difference {d:.1e}"), d <= 2e-5);
    let back = persist::load_str(&persist::to_json(doc.project(), &Default::default())).unwrap();
    let c = &back.document.project().compositions[0];
    t.row("And is read back as 32 bpc (After Effects), Float on", &format!("float {}, ae {}", c.float_depth, c.ae_32bpc), c.ae_32bpc && c.float_depth);
    doc.undo().expect("one step to undo");
    let off = !doc.project().compositions[0].ae_32bpc && saved(&doc).get("ae_32bpc").is_none();
    t.row("Undo puts it back to Float, and nothing is written", if off { "Float, no line" } else { "not put back" }, off);
    let refused = set(&mut doc, false, true);
    t.row("32 bpc (After Effects) without Float is refused by the command", &format!("{refused:?}"), refused.is_err());

    t.heading("A composition inside another follows the outermost one");
    for (outer, inner, same) in [("AE", "Display", true), ("Display", "AE", false), ("Float", "AE", false), ("AE", "AE", true)] {
        let doc = nested(&t, "fx_ae32_001.json", outer, inner);
        let d = largest_difference(&t.render(&doc, 0, 64), one);
        t.row(
            &format!(
                "FX-AE32-001's layer inside a composition: outer {outer}, inner {inner}, {}",
                if same { "draws FX-AE32-001's frame" } else { "does not" }
            ),
            &format!("largest difference {d:.1e}"),
            if same { d <= 2e-5 } else { d > 1e-3 },
        );
    }

    t.heading("The viewer");
    let project = t.load("fx_ae32_001.json").document.project().clone();
    let comp = Id::new("comp-main");
    let mut cache = CelCache::viewer();
    for pass in ["first", "second"] {
        let mut log = FrameLog::new(3);
        let frame = preview::preview_frame_cached(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache).unwrap();
        let d = largest_difference(&frame, one);
        t.row(&format!("FX-AE32-001 in the viewer, {pass} time (its effects are not kept between frames)"), &format!("largest difference {d:.1e}"), d <= 2e-5);
    }
    match Gpu::new() {
        Err(why) => t.row("The preview card draws a 32 bpc (After Effects) frame", &format!("NOT RUN: no usable card ({why})"), false),
        Ok(mut gpu) => {
            for name in ["fx_ae32_001.json", "fx_ae32_002.json", "fx_ae32_005.json"] {
                let project = t.load(name).document.project().clone();
                let mut log = FrameLog::new(3);
                let cpu = preview::preview_frame_cached(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache).unwrap().to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (card, ..) = preview::preview_frame_srgb8(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).unwrap();
                let on_cpu = log.finish().iter().any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str());
                let worst = cpu.iter().zip(&card).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
                t.row(
                    &format!("{name}: the preview draws the CPU's picture, within 1 level of 255"),
                    &format!("{}, largest difference {worst} of 255", if on_cpu { "CPU" } else { "card" }),
                    worst <= 1,
                );
            }
        }
    }

    t.heading("Pictures: tutorial 2's Lightning Diff copy after step F, in `verification/D-333 pictures/`");
    let dir = repo("verification/D-333 pictures");
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), 1920, 1080, OutputDepth::Eight, &[], bytes).unwrap();
    let mut bright = Vec::new();
    for (name, how, what) in [
        ("built_1_float.png", "Float", "Float, as before: a block of light about 600 pixels wide"),
        ("built_2_ae_32bpc.png", "AE", "32 bpc (After Effects): a soft pool where the bolt lands"),
    ] {
        let (bytes, n, said) = diff_copy(&dir, how);
        write(name, &bytes);
        bright.push(n);
        t.row(&format!("{name}, {what}; draws cleanly"), &format!("{said:?}, {n} pixels brighter than a quarter"), said.is_empty());
    }
    t.row(
        "In 32 bpc (After Effects) under a quarter as many pixels are brighter than a quarter as in Float",
        &format!("Float {}, 32 bpc (After Effects) {}", bright[0], bright[1]),
        bright[1] * 4 < bright[0] && bright[1] > 0,
    );

    t.heading("Files from before D-333");
    let shot = persist::load(&repo("verification/B-08a_project.json")).unwrap();
    let text = persist::to_json(shot.document.project(), &shot.preserved);
    let none = !text.contains("ae_32bpc") && shot.document.project().compositions.iter().all(|c| !c.ae_32bpc);
    t.row("The reference shot is not 32 bpc (After Effects) in any composition and saves no ae_32bpc line", if none { "no line" } else { "differs" }, none);
    let float = t.load("../float_depth/fx_blend_addf_001.json");
    let none = !persist::to_json(float.document.project(), &float.preserved).contains("ae_32bpc");
    t.row("A Float file from D-319 stays plain Float and saves no ae_32bpc line", if none { "no line" } else { "a line" }, none);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_ae32_001.json", 0), ("fx_ae32_005.json", 0)]);

    t.finish("D-333_ae_32bpc_table.md");
}
