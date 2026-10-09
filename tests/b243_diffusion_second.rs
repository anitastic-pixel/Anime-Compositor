//! B-243: D-364, Diffusion's second pass: the same glow laid on again in Soft Light or Overlay.
//!
//! Writes `verification/D-364_diffusion_second_table.md`.
//!
//! Every expected pixel is `Fixtures/diffusion/expected_diffusion_second.json`, written by
//! `tools/diffusion_second_reference.py`, printed in document 25 as FX-DIFFUSE-019 to 032.
//! Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a street before and after, into `verification/D-364 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, set, town, Table, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

fn diffusion(radius: f64, amount: f64, blend: &str, second_amount: f64, second_blend: &str) -> Effect {
    Effect::Diffusion { radius, amount, blend: blend.into(), second_amount, second_blend: second_blend.into() }
}

/// The preview of each file, on the card and on the CPU, within 1 level of 255.
fn preview_rows(t: &mut Table, files: &[&str]) {
    let mut gpu = match Gpu::new() {
        Err(why) => return t.row("The preview draws these files", &format!("NOT RUN: no usable card ({why})"), false),
        Ok(gpu) => gpu,
    };
    for name in files {
        let project = t.load(name).document.project().clone();
        let comp = Id::new(effect_table::MAIN);
        let mut cache = CelCache::viewer();
        let mut log = FrameLog::new(3);
        let cpu = preview::preview_frame_cached(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache).unwrap().to_srgb8_straight();
        let mut log = FrameLog::new(3);
        let (card, ..) = preview::preview_frame_srgb8(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).unwrap();
        let on_cpu = log.finish().iter().any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str());
        let worst = cpu.iter().zip(&card).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
        t.row(
            &format!("{name}: the preview draws the same picture as the CPU, within 1 level of 255"),
            &format!("{}, largest difference {worst} of 255", if on_cpu { "fell back to the CPU" } else { "no fallback" }),
            worst <= 1 && !on_cpu,
        );
    }
}

/// The street (`town.png` in `dir`) with `effect`, 8-bit straight.
fn picture(dir: &Path, effect: Option<J>) -> (Vec<u8>, Vec<String>) {
    let mut project: J =
        serde_json::from_str(&fs::read_to_string(repo("Fixtures/diffusion/fx_diffuse_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = json!(effect.into_iter().collect::<Vec<_>>());
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// The largest difference and the pixels differing between two 8-bit pictures.
fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    let worst = a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
    (worst, a.chunks(4).zip(b.chunks(4)).filter(|(p, q)| p != q).count())
}

#[test]
fn b243_diffusion_second() {
    let mut t = Table::new(
        "diffusion",
        "# D-364: Diffusion's second pass\n\nAnime compositing's usual diffusion is two blurred \
         copies: one in Lighten or Screen, one in Soft Light or Overlay at about 30 to 50 per \
         cent. Diffusion (D-148) was the first; D-364 adds the second to the same effect, \
         Second Blend (Soft Light or Overlay) and Second Amount (0 to 100, starting at 0, off), \
         sharing the one blur. Every expected pixel is \
         `Fixtures/diffusion/expected_diffusion_second.json`, written by \
         `tools/diffusion_second_reference.py` before this code existed, printed in document 25 \
         as FX-DIFFUSE-019 to 032. Tolerance 2e-5. FX-DIFFUSE-001 to 018 are still checked, \
         unchanged, by `tests/b91_diffusion.rs`.\n",
    );

    t.heading("FX-DIFFUSE-019 to 032 (document 25)");
    t.fixtures("expected_diffusion_second.json");

    t.heading("The file");
    t.round_trips(&["fx_diffuse_019.json", "fx_diffuse_021.json", "fx_diffuse_026.json", "fx_diffuse_027.json", "fx_diffuse_028.json"]);
    let saved = t.saved_parameters("fx_diffuse_021.json");
    t.row(
        "Saved with its second amount and second blend",
        &saved.to_string(),
        saved["second_amount"] == 30 && saved["second_blend"] == "overlay",
    );
    let saved = t.saved_parameters("fx_diffuse_001.json");
    t.row(
        "A file from before D-364 saves exactly as it was, with no second_amount or second_blend",
        &saved.to_string(),
        saved.get("second_amount").is_none() && saved.get("second_blend").is_none(),
    );
    let got = diffusion(500.0, 100.0, "screen", 100.0, "overlay").bounds_expansion();
    t.row("it still grows the drawing's bounds by nothing", &got.to_string(), got == 0);

    t.heading("Commands");
    let mut document = t.load("fx_diffuse_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("second amount 101", set(diffusion(10.0, 50.0, "lighten", 101.0, "soft_light"))),
            ("second amount -1", set(diffusion(10.0, 50.0, "lighten", -1.0, "soft_light"))),
            ("second blend \"screen\"", set(diffusion(10.0, 50.0, "lighten", 50.0, "screen"))),
            ("second amount keyed to 150", keys("second_amount", &[(0, &[50.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_diffuse_001.json",
        vec![
            ("second amount 100 in overlay, the top,", set(diffusion(10.0, 50.0, "lighten", 100.0, "overlay"))),
            ("second amount keyed from 0 to 100", keys("second_amount", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_diffusion_second.json")).unwrap()).unwrap();
    let mut document = t.load("fx_diffuse_001.json").document;
    document.apply(set(diffusion(10.0, 50.0, "lighten", 50.0, "soft_light"))).unwrap();
    let d = effect_table::largest_difference(&t.render(&document, 0, 64), &expected["cases"]["FX-DIFFUSE-019"]["frames"]["0"]);
    t.row("FX-DIFFUSE-001 set to lighten 50 then soft light 50 by the command draws FX-DIFFUSE-019's frame", &format!("largest difference {d:.1e}"), d <= 2e-5);

    t.heading("The preview, on the card");
    preview_rows(
        &mut t,
        &[
            "fx_diffuse_019.json",
            "fx_diffuse_020.json",
            "fx_diffuse_021.json",
            "fx_diffuse_022.json",
            "fx_diffuse_023.json",
            "fx_diffuse_024.json",
            "fx_diffuse_025.json",
            "fx_diffuse_026.json",
            "fx_diffuse_027.json",
            "fx_diffuse_028.json",
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_diffuse_019.json", 0), ("fx_diffuse_021.json", 0), ("fx_diffuse_028.json", 3)]);

    t.heading("Pictures: a street, in `verification/D-364 pictures/`");
    let dir = repo("verification/D-364 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let fx = |second_amount: f64, second_blend: &str| {
        Some(json!({ "instance_id": "fx-0-0", "type_id": "core.diffusion", "enabled": true,
                     "parameters": { "radius": 30, "amount": 50, "blend": "lighten",
                                     "second_amount": second_amount, "second_blend": second_blend } }))
    };
    let mut drawn = Vec::new();
    for (name, effect, what) in [
        ("1_before.png", None, "the street, no effect"),
        ("2_lighten_50.png", fx(0.0, "soft_light"), "Diffusion radius 30, lighten 50, second pass off: the glow as before D-364"),
        ("3_lighten_50_soft_light_50.png", fx(50.0, "soft_light"), "the same, then soft light 50: the two-layer stack"),
        ("4_lighten_50_overlay_50.png", fx(50.0, "overlay"), "the same, then overlay 50: stronger contrast"),
    ] {
        let (bytes, said) = picture(&dir, effect);
        t.row(&format!("{name}, {what}; draws cleanly"), &format!("{said:?}"), said.is_empty());
        write(name, &bytes);
        drawn.push(bytes);
    }
    let (soft, overlay) = (distance(&drawn[1], &drawn[2]), distance(&drawn[1], &drawn[3]));
    t.row(
        "The second pass changes the lighten-only picture, overlay more than soft light",
        &format!("soft light: largest change {} of 255 over {} pixels; overlay: {} over {}", soft.0, soft.1, overlay.0, overlay.1),
        soft.0 > 0 && overlay.0 > soft.0,
    );

    t.finish("D-364_diffusion_second_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then Diffusion, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B243_CPU` set, the processor draws instead; `B243_OUT` names the file written.
#[test]
#[ignore = "B-243: a measurement, run deliberately with --release --ignored"]
fn b243_diffusion_second_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B243_CPU").is_ok();
    let passes = if cpu { 3 } else { 8 };
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: {}\n- Loops: {passes}\n\n\
         | Shot | Quality | First | Again |\n|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
        if cpu { "the processor" } else { "the card" },
    );
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone", None),
        ("Noise, then Diffusion radius 30, lighten 50 (second pass off, as before D-364)", Some(json!({"radius": 30, "amount": 50, "blend": "lighten"}))),
        ("Noise, then Diffusion radius 30, lighten 50, soft light 50", Some(json!({"radius": 30, "amount": 50, "blend": "lighten", "second_amount": 50}))),
        ("Noise, then Diffusion radius 30, lighten 50, overlay 50", Some(json!({"radius": 30, "amount": 50, "blend": "lighten", "second_amount": 50, "second_blend": "overlay"}))),
    ];
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    for (name, e) in shots {
        let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
        for (i, id) in ["a", "b", "c"].iter().enumerate() {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true, "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(json!({"instance_id": format!("{id}d"), "type_id": "core.diffusion", "enabled": true, "parameters": p.clone()}));
            }
            j["compositions"][0]["layers"][i]["effects"] = J::Array(v);
        }
        let project = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone();
        let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..passes {
            for frame in (0..240).step_by(8) {
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                if cpu {
                    drop(preview::preview_frame_cached(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache).expect("CPU frame"));
                } else {
                    drop(preview::preview_frame_srgb8(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                }
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                if pass > 0 { times.push(ms) } else { first.push(ms) }
                if !cpu {
                    assert!(
                        !log.finish().iter().any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str()),
                        "{name}: the card refused frame {frame}"
                    );
                }
            }
        }
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B243_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-243_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
