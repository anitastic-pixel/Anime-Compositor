//! B-210: D-330, the 8 bpc (After Effects) working depth.
//!
//! Writes `verification/D-330_eight_bpc_table.md`.
//!
//! Every expected pixel is `Fixtures/eight_bpc/expected_eight_bpc.json`, written by
//! `tools/eight_bpc_reference.py` before this code existed and printed in document 25 as
//! FX-8BPC-001 to 005. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws P-26 tutorial 2's reflection, three blue tips through Fast Box Blur and
//! Solid Composite on black and Exposure +17.33, in Display, Float and 8 bpc, into `verification/D-330 pictures/`, over black.

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

fn set(doc: &mut Document, float_depth: bool, eight_bpc: bool) -> Result<(), String> {
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
        eight_bpc,
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

/// `name`'s composition, renamed `comp-inner`, inside a new `comp-main` of the same size as its
/// only layer, unmoved; `outer` and `inner` whether each is 8 bpc.
fn nested(t: &Table, name: &str, outer: bool, inner: bool) -> Document {
    let mut project = file(t, name);
    let mut inside = project["compositions"][0].clone();
    inside["id"] = json!("comp-inner");
    inside["name"] = json!("Inner");
    inside.as_object_mut().unwrap().remove("eight_bpc");
    if inner {
        inside["eight_bpc"] = json!(true);
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
    top.as_object_mut().unwrap().remove("eight_bpc");
    if outer {
        top["eight_bpc"] = json!(true);
    }
    project["compositions"] = json!([top, inside]);
    persist::load_str(&project.to_string())
        .unwrap_or_else(|d| panic!("the nested file opens: {}", d.message))
        .document
}

/// Half of tutorial 2's frame, 960 by 540, holding the bolt's three tips, 5 by 3 pixels each in
/// the tutorial's blue #096bf1, clear round them.
const PLATE: (usize, usize) = (960, 540);

fn tips() -> Vec<u8> {
    let (w, h) = PLATE;
    (0..w * h)
        .flat_map(|i| {
            let (x, y) = (i % w, i / w);
            let tip = (378..381).contains(&y) && [260, 300, 340].iter().any(|c| x + 2 >= *c && x <= c + 2);
            if tip { [0x09, 0x6b, 0xf1, 255] } else { [0; 4] }
        })
        .collect()
}

/// The tips through the Lightning Diff copy's Fast Box Blurs (horizontal 75.2, vertical 87.3,
/// halved for the half-size plate, 3 passes each), Solid Composite on black and Exposure +17.33,
/// 8-bit; and how many pixels show.
fn reflection(dir: &Path, depth: &str) -> (Vec<u8>, usize, Vec<String>) {
    let mut project = file_at("Fixtures/eight_bpc/fx_8bpc_001.json");
    project["assets"][0]["path"] = J::from("tips.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(PLATE.0);
    comp["height"] = J::from(PLATE.1);
    let c = comp.as_object_mut().unwrap();
    c.remove("eight_bpc");
    match depth {
        "8 bpc" => {
            c.insert("eight_bpc".into(), json!(true));
        }
        "Float" => {
            c.insert("float_depth".into(), json!(true));
        }
        _ => {}
    }
    let layer = &mut comp["layers"][0];
    let middle = json!([PLATE.0 as f64 / 2.0, PLATE.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    let boxed = |id: &str, radius: f64, dimensions: &str| {
        json!({ "instance_id": id, "type_id": "core.fast_box_blur", "enabled": true,
                "parameters": { "radius": radius, "iterations": 3, "dimensions": dimensions } })
    };
    layer["effects"] = json!([
        boxed("fx-0-0", 37.6, "horizontal"),
        boxed("fx-0-1", 43.65, "vertical"),
        { "instance_id": "fx-0-2", "type_id": "core.solid_composite", "enabled": true,
          "parameters": { "source_opacity": 100, "color": "#000000", "opacity": 100, "blend": "normal" } },
        { "instance_id": "fx-0-3", "type_id": "core.exposure", "enabled": true, "parameters": { "stops": 17.33 } },
    ]);
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    let over = |c: f32| (anime_compositor::color::linear_to_srgb(c.clamp(0.0, 1.0)) * 255.0).round() as u8;
    let black: Vec<u8> = frame.data().chunks(4).flat_map(|p| [over(p[0]), over(p[1]), over(p[2]), 255]).collect();
    let lit = black.chunks(4).filter(|p| p[..3].iter().any(|&v| v > 0)).count();
    (black, lit, said)
}

fn file_at(rel: &str) -> J {
    serde_json::from_str(&fs::read_to_string(repo(rel)).unwrap()).unwrap()
}

#[test]
fn b210_eight_bpc() {
    let mut t = Table::new(
        "eight_bpc",
        "# D-330: 8 bpc (After Effects) working depth\n\nFrom P-26: tutorial 2 is drawn in After \
         Effects' 8 bpc until its step F, and its reflection blurs the bolt's tips wide and lifts \
         them 17 stops with Exposure. In 8 bpc After Effects blurs display colours and keeps each \
         layer's pixels as whole numbers 0 to 255 between effects, so the blur's faint edge is 0 \
         before Exposure sees it. The working depth gains a third choice, 8 bpc: after every \
         effect each pixel is rounded to 8 bits, and the four blurs average display colours. \
         Every expected pixel is `Fixtures/eight_bpc/expected_eight_bpc.json`, written by \
         `tools/eight_bpc_reference.py` before the build had 8 bpc. Tolerance 2e-5.\n",
    );

    t.heading("FX-8BPC-001 to 005 (document 25)");
    t.fixtures("expected_eight_bpc.json");
    t.round_trips(&["fx_8bpc_001.json", "fx_8bpc_004.json", "fx_8bpc_005.json"]);
    let expected = file(&t, "expected_eight_bpc.json");
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
    let mut doc = t.load("fx_8bpc_004.json").document;
    set(&mut doc, false, false).unwrap();
    let off = saved(&doc).get("eight_bpc").is_none();
    t.row("Set to Display, no eight_bpc line is written", if off { "no line" } else { "a line" }, off);
    let d = largest_difference(&t.render(&doc, 0, 64), &expected["cases"]["FX-8BPC-004"]["frames"]["0"]);
    t.row("And FX-8BPC-004's blur is no longer rounded", &format!("largest difference {d:.1e}"), d > 1e-3);
    set(&mut doc, false, true).unwrap();
    let line = saved(&doc).get("eight_bpc").cloned();
    t.row("Set to 8 bpc through the command, it is written as eight_bpc: true", &format!("{line:?}"), line == Some(json!(true)));
    let d = largest_difference(&t.render(&doc, 0, 64), &expected["cases"]["FX-8BPC-004"]["frames"]["0"]);
    t.row("And draws FX-8BPC-004's frame again", &format!("largest difference {d:.1e}"), d <= 2e-5);
    let back = persist::load_str(&persist::to_json(doc.project(), &Default::default())).unwrap();
    let on = back.document.project().compositions[0].eight_bpc;
    t.row("And is read back as 8 bpc", if on { "8 bpc" } else { "not" }, on);
    doc.undo().expect("one step to undo");
    let off = !doc.project().compositions[0].eight_bpc && saved(&doc).get("eight_bpc").is_none();
    t.row("Undo puts it back to Display, and nothing is written", if off { "Display, no line" } else { "not put back" }, off);
    let refused = set(&mut doc, true, true);
    t.row("Float and 8 bpc both at once is refused by the command", &format!("{refused:?}"), refused.is_err());

    t.heading("A composition inside another follows the outermost one");
    let one = &expected["cases"]["FX-8BPC-001"]["frames"]["0"];
    for (outer, inner, same) in [(true, false, true), (false, true, false), (true, true, true)] {
        let doc = nested(&t, "fx_8bpc_001.json", outer, inner);
        let d = largest_difference(&t.render(&doc, 0, 64), one);
        let depth = |e: bool| if e { "8 bpc" } else { "Display" };
        t.row(
            &format!(
                "FX-8BPC-001's layer inside a composition: outer {}, inner {}, {}",
                depth(outer),
                depth(inner),
                if same { "draws FX-8BPC-001's frame" } else { "is not rounded" }
            ),
            &format!("largest difference {d:.1e}"),
            if same { d <= 2e-5 } else { d > 1e-3 },
        );
    }

    t.heading("The viewer");
    let project = t.load("fx_8bpc_001.json").document.project().clone();
    let comp = Id::new("comp-main");
    let mut cache = CelCache::viewer();
    for pass in ["first", "second"] {
        let mut log = FrameLog::new(3);
        let frame = preview::preview_frame_cached(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache).unwrap();
        let d = largest_difference(&frame, one);
        t.row(&format!("FX-8BPC-001 in the viewer, {pass} time (its effects are not kept between frames)"), &format!("largest difference {d:.1e}"), d <= 2e-5);
    }
    match Gpu::new() {
        Err(why) => t.row("The preview card draws an 8 bpc frame", &format!("NOT RUN: no usable card ({why})"), false),
        Ok(mut gpu) => {
            for name in ["fx_8bpc_001.json", "fx_8bpc_002.json", "fx_8bpc_004.json"] {
                let project = t.load(name).document.project().clone();
                let mut log = FrameLog::new(3);
                let cpu = preview::preview_frame_cached(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache).unwrap().to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (card, ..) = preview::preview_frame_srgb8(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).unwrap();
                let on_cpu = log.finish().iter().any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str());
                let worst = cpu.iter().zip(&card).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
                t.row(
                    &format!("{name}: the preview draws the CPU's picture, within 1 level of 255 (the effects all run on the CPU)"),
                    &format!("{}, largest difference {worst} of 255", if on_cpu { "CPU" } else { "card" }),
                    worst <= 1,
                );
            }
        }
    }

    t.heading("Pictures: tutorial 2's reflection, in `verification/D-330 pictures/`, over black");
    let dir = repo("verification/D-330 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    write("tips.png", &tips());
    let mut lit = Vec::new();
    for (name, depth, what) in [
        ("built_1_display.png", "Display", "Display, as before: the blur's faint edge lit wide"),
        ("built_2_float.png", "Float", "Float: the same wide light"),
        ("built_3_eight_bpc.png", "8 bpc", "8 bpc: the faint edge rounds to nothing, one small spot where the tips are"),
    ] {
        let (bytes, n, said) = reflection(&dir, depth);
        write(name, &bytes);
        lit.push(n);
        t.row(&format!("{name}, {what}; draws cleanly"), &format!("{said:?}, {n} pixels show"), said.is_empty());
    }
    t.row(
        "In 8 bpc under a tenth as many pixels show as in Display",
        &format!("Display {}, Float {}, 8 bpc {}", lit[0], lit[1], lit[2]),
        lit[2] * 10 < lit[0] && lit[2] > 0,
    );

    t.heading("Files from before D-330");
    let shot = persist::load(&repo("verification/B-08a_project.json")).unwrap();
    let text = persist::to_json(shot.document.project(), &shot.preserved);
    let none = !text.contains("eight_bpc") && shot.document.project().compositions.iter().all(|c| !c.eight_bpc);
    t.row("The reference shot is not 8 bpc in any composition and saves no eight_bpc line", if none { "no line" } else { "differs" }, none);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_8bpc_001.json", 0), ("fx_8bpc_002.json", 0)]);

    t.finish("D-330_eight_bpc_table.md");
}
