//! B-328: D-448, After Effects' Cartoon under `core.cartoon`: Render, Detail Radius, Detail
//! Threshold, Shading Steps, Shading Smoothness, Edge Threshold, Edge Width, Edge Softness, Edge
//! Opacity, Edge Enhancement, Edge Black Level and Edge Contrast.
//!
//! Writes `verification/D-448_cartoon_table.md` and draws pictures into
//! `verification/D-448 pictures/`.
//!
//! Every expected pixel is `Fixtures/cartoon/expected_cartoon.json`, written by
//! `tools/cartoon_reference.py` before this code existed and printed in document 25 as
//! FX-CARTOON-001 to 032. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        if p[3] == 0 && q[3] == 0 {
            continue;
        }
        let d = p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
        largest = largest.max(d);
        pixels += (d > 0) as usize;
    }
    (largest, pixels)
}

fn said(log: FrameLog) -> String {
    let mut ids: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
    ids.sort();
    ids.dedup();
    ids.join(", ")
}

/// The Cartoons the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::Cartoon { .. })))
        .count()
}

/// The processor and the card, each drawing the frame the page receives: the largest
/// difference, the pixels differing, whether the card refused the frame, and each one's warnings.
fn both(gpu: &mut Gpu, project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> ((u8, usize), bool, String, String) {
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let c = preview::preview_frame_cached(project, comp, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
        .unwrap_or_else(|d| panic!("frame {frame} on the CPU: {}", d.message));
    let said_cpu = said(log);
    let mut log = FrameLog::new(3);
    let (g, ..) = preview::preview_frame_srgb8(project, comp, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu)
        .unwrap_or_else(|d| panic!("frame {frame} on the GPU: {}", d.message));
    let said_gpu = said(log);
    let refused = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
    (distance(&c.to_srgb8_straight(), &g), refused, said_cpu, said_gpu)
}

/// The reference shot (1920 by 1080) with `stack` on its first three layers.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// A Cartoon with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"render": "fill_and_edges", "detail_radius": 8, "detail_threshold": 10, "shading_steps": 8,
        "shading_smoothness": 40, "edge_threshold": 1.6, "edge_width": 1.5, "edge_softness": 60, "edge_opacity": 100,
        "edge_enhancement": 0, "edge_black_level": 0, "edge_contrast": 0.5});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.cartoon", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=32 {
        let file = format!("fx_cartoon_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality);
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor; `want` of 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, cases: &[(&str, J, usize)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in cases {
        let project = reference(|id| json!([fx(&format!("b328-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Cartoon {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Cartoon {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor at `frame`,
/// straight 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(size.0);
    comp["height"] = J::from(size.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([size.0 as f64 / 2.0, size.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// The `art` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

/// How many neighbouring pixel pairs differ by more than `step` levels in red, across and down.
fn edges(p: &[u8], (w, h): (usize, usize), step: u8) -> usize {
    let r = |x: usize, y: usize| p[(y * w + x) * 4];
    let mut n = 0;
    for y in 0..h - 1 {
        for x in 0..w - 1 {
            n += (r(x, y).abs_diff(r(x + 1, y)) > step) as usize + (r(x, y).abs_diff(r(x, y + 1)) > step) as usize;
        }
    }
    n
}

/// Pixels whose three colours are all at most `top` (dark), and all at least `bottom` (light).
fn dark(p: &[u8], top: u8) -> usize {
    p.chunks_exact(4).filter(|q| q[..3].iter().all(|&c| c <= top)).count()
}
fn light(p: &[u8], bottom: u8) -> usize {
    p.chunks_exact(4).filter(|q| q[..3].iter().all(|&c| c >= bottom)).count()
}

#[test]
fn b328_cartoon() {
    let mut t = Table::new(
        "cartoon",
        "# D-448: Cartoon\n\nB-328: `core.cartoon` takes After Effects' Cartoon controls (Render, \
         Detail Radius, Detail Threshold, Shading Steps, Shading Smoothness, Edge Threshold, Edge \
         Width, Edge Softness, Edge Opacity, Edge Enhancement, Edge Black Level, Edge Contrast). \
         Every expected pixel is `Fixtures/cartoon/expected_cartoon.json`, written by \
         `tools/cartoon_reference.py` before this code existed and printed in document 25 as \
         FX-CARTOON-001 to 032. Tolerance 2e-5.\n",
    );

    t.heading("FX-CARTOON-001 to 032 (document 25)");
    t.fixtures_numbered("expected_cartoon.json", 1..=32);

    t.heading("The file");
    let files: Vec<String> = (1..=32).map(|n| format!("fx_cartoon_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_cartoon_023.json");
    t.row(
        "fx_cartoon_023.json is saved with its twelve settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 12 && saved["detail_radius"] == 3.0 && saved["edge_contrast"] == 0.7 && saved["render"] == "fill_and_edges",
    );
    for (file, want) in [
        ("fx_cartoon_024.json", "render"),
        ("fx_cartoon_025.json", "detail radius"),
        ("fx_cartoon_026.json", "detail threshold"),
        ("fx_cartoon_027.json", "shading steps"),
        ("fx_cartoon_028.json", "shading steps"),
        ("fx_cartoon_029.json", "edge width"),
        ("fx_cartoon_030.json", "edge enhancement"),
        ("fx_cartoon_031.json", "edge black level"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Cartoon and its {want}"), &why, why.contains("Cartoon") && why.contains(want));
    }
    let full = r#"{"render": "fill_and_edges", "detail_radius": 8, "detail_threshold": 10, "shading_steps": 8, "shading_smoothness": 40, "edge_threshold": 1.6, "edge_width": 1.5, "edge_softness": 60, "edge_opacity": 100, "edge_enhancement": 0, "edge_black_level": 0, "edge_contrast": 0.5}"#;
    t.shape_refused("fx_cartoon_001.json", "a Cartoon whose detail radius is a word", &full.replace("\"detail_radius\": 8", "\"detail_radius\": \"wide\""));
    t.shape_refused("fx_cartoon_001.json", "a Cartoon without its edge width", &full.replace("\"edge_width\": 1.5, ", ""));
    t.shape_refused("fx_cartoon_001.json", "a Cartoon whose render is a number", &full.replace("\"fill_and_edges\"", "2"));

    t.heading("Commands");
    let mut document = t.load("fx_cartoon_001.json").document;
    let one = changed(&t, "fx_cartoon_001.json", |e| if let Effect::Cartoon { shading_steps, .. } = e { *shading_steps = 1.0 });
    let bad = changed(&t, "fx_cartoon_001.json", |e| if let Effect::Cartoon { render, .. } = e { *render = "Fill".into() });
    t.refused(
        &mut document,
        vec![
            ("shading steps 1", set(one)),
            ("render Fill (a capital)", set(bad)),
            ("edge opacity keyed to 150", keys("edge_opacity", &[(0, &[100.0]), (4, &[150.0])])),
        ],
    );
    let edged = changed(&t, "fx_cartoon_001.json", |e| {
        if let Effect::Cartoon { render, edge_width, .. } = e {
            *render = "edges".into();
            *edge_width = 4.0;
        }
    });
    t.taken(
        &mut document,
        "fx_cartoon_001.json",
        vec![
            ("Edges, edge width 4", set(edged)),
            ("detail radius keyed from 0 to 20", keys("detail_radius", &[(0, &[0.0]), (4, &[20.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_cartoon_001.json", 0),
        ("fx_cartoon_010.json", 0),
        ("fx_cartoon_014.json", 0),
        ("fx_cartoon_020.json", 3),
        ("fx_cartoon_022.json", 0),
        ("fx_cartoon_023.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Refused (024 to 032) leave nothing for the card.
    let none: Vec<u32> = (24..=32).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            ("Edges, width 4, softness 0, enhancement 60, black level 0.3", json!({"render": "edges", "edge_width": 4, "edge_softness": 0, "edge_enhancement": 60, "edge_black_level": 0.3}), 3),
            ("Fill, radius 0, steps 3, smoothness 0", json!({"render": "fill", "detail_radius": 0, "shading_steps": 3, "shading_smoothness": 0}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-448 pictures/`");
    let dir = repo("verification/D-448 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]), 0);
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]), 0);
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());

    let (added, said) = street(json!({}));
    write("2_as_added.png", TOWN, &added);
    t.row(
        "2_as_added.png, as added: the street in flat smoothed colours with dark lines along its edges, so more near-black pixels than the street; draws cleanly",
        &format!("{said:?}, pixels changed {}, near-black (all three at most 40): street {}, cartoon {}", distance(&added, &before).1, dark(&before, 40), dark(&added, 40)),
        said.is_empty() && dark(&added, 40) > dark(&before, 40),
    );

    let (fill, said) = street(json!({"render": "fill"}));
    write("3_fill.png", TOWN, &fill);
    let (no_ink, said1) = street(json!({"edge_opacity": 0}));
    t.row(
        "3_fill.png, Render Fill: the smoothed, stepped colours with no lines, fewer near-black pixels than 2_as_added.png; Edge Opacity 0 draws the same picture byte for byte; both draw cleanly",
        &format!("{said:?} {said1:?}, near-black: as added {}, fill {}; opacity 0 the same: {}", dark(&added, 40), dark(&fill, 40), no_ink == fill),
        said.is_empty() && said1.is_empty() && dark(&fill, 40) < dark(&added, 40) && no_ink == fill,
    );

    let (lines, said) = street(json!({"render": "edges"}));
    write("4_edges.png", TOWN, &lines);
    let n = TOWN.0 * TOWN.1;
    t.row(
        "4_edges.png, Render Edges: dark lines on white paper, so most pixels near white and some near black; draws cleanly",
        &format!("{said:?}, near-white (all three at least 215) {} and near-black {} of {n}", light(&lines, 215), dark(&lines, 40)),
        said.is_empty() && light(&lines, 215) * 2 > n && dark(&lines, 40) > 0,
    );

    let (thick, said) = street(json!({"render": "edges", "edge_width": 5, "edge_softness": 0}));
    write("5_thick_edges.png", TOWN, &thick);
    t.row(
        "5_thick_edges.png, Edges, width 5, softness 0: thicker lines, more near-black pixels than 4_edges.png; draws cleanly",
        &format!("{said:?}, near-black: width 1.5 {}, width 5 {}", dark(&lines, 40), dark(&thick, 40)),
        said.is_empty() && dark(&thick, 40) > dark(&lines, 40),
    );

    let (smooth, said) = street(json!({"render": "fill", "detail_radius": 20, "detail_threshold": 60}));
    write("6_smoother.png", TOWN, &smooth);
    t.row(
        "6_smoother.png, Fill, detail radius 20, threshold 60: flatter areas, fewer edges between neighbours than 3_fill.png; draws cleanly",
        &format!("{said:?}, neighbour pairs differing by more than 8 in red: radius 8 {}, radius 20 {}", edges(&fill, TOWN, 8), edges(&smooth, TOWN, 8)),
        said.is_empty() && edges(&smooth, TOWN, 8) < edges(&fill, TOWN, 8),
    );

    let (white_ink, said) = street(json!({"render": "edges", "edge_black_level": 1}));
    write("7_black_level_1.png", TOWN, &white_ink);
    t.row(
        "7_black_level_1.png, Edges, black level 1: the lines white and the paper black, so most pixels near black; draws cleanly",
        &format!("{said:?}, near-black {} of {n}", dark(&white_ink, 40)),
        said.is_empty() && dark(&white_ink, 40) * 2 > n,
    );

    t.finish("D-448_cartoon_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B328_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-328: a measurement, run deliberately with --release --ignored"]
fn b328_cartoon_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B328_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        ("Noise, then Cartoon as added", Some(json!({}))),
        ("Noise, then Cartoon detail radius 30, edge width 8, enhancement 50 (the widest reach)", Some(json!({"detail_radius": 30, "edge_width": 8, "edge_enhancement": 50}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}c"), p));
            }
            J::Array(v)
        });
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
            }
        }
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B328_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-328_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
