//! B-484: D-605, After Effects' Liquify under `core.liquify`: one of six brushes (Warp, the two
//! Twirls, Pucker, Bloat, Shift Pixels) dragged along a typed stroke in dabs, drawn through
//! D-601's backward field.
//!
//! Writes `verification/D-605_liquify_table.md` and draws pictures into `verification/D-605 pictures/`.
//!
//! Every expected pixel is `Fixtures/liquify/expected_liquify.json`, written by
//! `tools/liquify_reference.py` before this code existed and printed in document 25 as
//! FX-LIQUIFY-001 to 021. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The Liquifies the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::Liquify { .. })))
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

/// A Liquify with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"tool": "warp", "brush_size": 64, "brush_pressure": 50, "stroke": [], "distortion_percentage": 100, "distortion_mesh_offset": [0, 0]});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.liquify", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` change nothing or are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=21 {
        let file = format!("fx_liquify_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b484-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Liquify {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Liquify {what} on three layers, frame {frame}, {}", quality.label()),
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

#[test]
fn b484_liquify() {
    let mut t = Table::new(
        "liquify",
        "# D-605: Liquify\n\nB-484: `core.liquify` takes After Effects' Liquify controls (Tool, \
         Brush Size, Brush Pressure, Distortion Percentage, Distortion Mesh Offset) and a stroke \
         typed as points, drags the brush along it in dabs, and draws the drawing through D-601's \
         backward field. Every expected pixel is `Fixtures/liquify/expected_liquify.json`, written \
         by `tools/liquify_reference.py` before this code existed and printed in document 25 as \
         FX-LIQUIFY-001 to 021. Tolerance 2e-5.\n",
    );

    t.heading("FX-LIQUIFY-001 to 021 (document 25)");
    t.fixtures_numbered("expected_liquify.json", 1..=21);

    t.heading("The file");
    let files: Vec<String> = (1..=21).map(|n| format!("fx_liquify_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_liquify_001.json");
    t.row(
        "fx_liquify_001.json is saved with its six settings, the tool as a word and the stroke as an empty list",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 6
            && saved["tool"] == "warp"
            && saved["brush_size"] == 64.0
            && saved["brush_pressure"] == 50.0
            && saved["stroke"] == json!([])
            && saved["distortion_percentage"] == 100.0
            && saved["distortion_mesh_offset"].as_array().is_some_and(|a| a.len() == 2 && a[0] == 0.0 && a[1] == 0.0),
    );
    for (file, want) in [
        ("fx_liquify_016.json", "tool"),
        ("fx_liquify_017.json", "brush size"),
        ("fx_liquify_018.json", "brush pressure"),
        ("fx_liquify_019.json", "stroke"),
        ("fx_liquify_020.json", "distortion percentage"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Liquify and its {want}"), &why, why.contains("Liquify") && why.contains(want));
    }
    let full = fx("x", &json!({}))["parameters"].to_string();
    t.shape_refused("fx_liquify_001.json", "a Liquify whose tool is a number", &full.replace("\"tool\":\"warp\"", "\"tool\":3"));
    t.shape_refused("fx_liquify_001.json", "a Liquify whose stroke is a word", &full.replace("\"stroke\":[]", "\"stroke\":\"across\""));
    t.shape_refused("fx_liquify_001.json", "a Liquify whose offset is one number", &full.replace("\"distortion_mesh_offset\":[0,0]", "\"distortion_mesh_offset\":[0]"));
    t.shape_refused("fx_liquify_001.json", "a Liquify without its brush size", &full.replace("\"brush_size\":64,", ""));

    t.heading("Commands");
    let mut document = t.load("fx_liquify_002.json").document;
    let freeze = changed(&t, "fx_liquify_002.json", |e| if let Effect::Liquify { tool, .. } = e { *tool = "freeze".into() });
    let none = changed(&t, "fx_liquify_002.json", |e| if let Effect::Liquify { brush_size, .. } = e { *brush_size = 0.0 });
    let half = changed(&t, "fx_liquify_002.json", |e| if let Effect::Liquify { stroke, .. } = e { *stroke = vec![25.0, 50.0, 75.0] });
    t.refused(
        &mut document,
        vec![
            ("tool freeze", set(freeze)),
            ("brush size 0", set(none)),
            ("a stroke of three numbers", set(half)),
            ("the stroke keyed to end at 300 per cent across", keys("stroke", &[(0, &[25.0, 50.0, 75.0, 50.0]), (4, &[25.0, 50.0, 300.0, 50.0])])),
        ],
    );
    let twirl = changed(&t, "fx_liquify_002.json", |e| {
        if let Effect::Liquify { tool, brush_size, stroke, .. } = e {
            *tool = "twirl_clockwise".into();
            *brush_size = 30.0;
            *stroke = vec![50.0, 50.0];
        }
    });
    t.taken(
        &mut document,
        "fx_liquify_002.json",
        vec![
            ("Twirl Clockwise, brush 30, one dab in the middle", set(twirl)),
            ("distortion percentage keyed from 0 to 200", keys("distortion_percentage", &[(0, &[0.0]), (4, &[200.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_liquify_002.json", 0),
        ("fx_liquify_003.json", 0),
        ("fx_liquify_010.json", 0),
        ("fx_liquify_012.json", 2),
        ("fx_liquify_015.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // 001 (no stroke) changes nothing, so nothing is left to the card; the refused (016 to 021)
    // neither; 011 changes nothing at frame 0 only, so it is on the card in 8 of 10.
    let none: Vec<u32> = [1].into_iter().chain(16..=21).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("Warp, brush 200, stroked across the middle", json!({"brush_size": 200, "stroke": [25, 50, 75, 50]}), 3),
            ("Pucker, brush 400, pressure 70, one dab in the middle", json!({"tool": "pucker", "brush_size": 400, "brush_pressure": 70, "stroke": [50, 50]}), 3),
            (
                "Twirl Clockwise, brush 600, 150 per cent, offset 20 by 10",
                json!({"tool": "twirl_clockwise", "brush_size": 600, "brush_pressure": 100, "stroke": [40, 50], "distortion_percentage": 150, "distortion_mesh_offset": [20, 10]}),
                3,
            ),
        ],
    );

    t.heading("Pictures: in `verification/D-605 pictures/`");
    let dir = repo("verification/D-605 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let (w, h) = TOWN;
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]), 0);
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]), 0);
    write("1_before.png", TOWN, &before);
    let open = |p: &[u8]| (0..w * h).filter(|&i| p[i * 4 + 3] == 0).count();
    t.row("1_before.png, the street with no effect, every pixel covered; draws cleanly", &format!("{said:?}, {} clear pixels", open(&before)), said.is_empty() && open(&before) == 0);

    let (added, said) = street(json!({}));
    t.row(
        "As added, with nothing painted: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels differ", distance(&added, &before).1),
        said.is_empty() && added == before,
    );

    let corner = |p: &[u8]| p[..4] == before[..4];
    let across = json!({"brush_size": 120, "stroke": [20, 50, 80, 50]});
    let (pushed, said) = street(across.clone());
    write("2_warp.png", TOWN, &pushed);
    t.row(
        "2_warp.png, Warp, brush 120, dragged across the middle from a fifth to four fifths: the street under the stroke smeared right, the corners untouched; draws cleanly",
        &format!("{said:?}, {} pixels differ from the street, top left corner untouched: {}", distance(&pushed, &before).1, corner(&pushed)),
        said.is_empty() && distance(&pushed, &before).1 > w * h / 100 && corner(&pushed),
    );

    let dab = |tool: &str| street(json!({"tool": tool, "brush_size": 200, "brush_pressure": 100, "stroke": [50, 50]}));
    let (clockwise, said) = dab("twirl_clockwise");
    write("3_twirl_clockwise.png", TOWN, &clockwise);
    t.row(
        "3_twirl_clockwise.png, Twirl Clockwise, brush 200, pressure 100, one dab in the middle: the middle of the street turned; draws cleanly",
        &format!("{said:?}, {} pixels differ from the street, top left corner untouched: {}", distance(&clockwise, &before).1, corner(&clockwise)),
        said.is_empty() && distance(&clockwise, &before).1 > w * h / 100 && corner(&clockwise),
    );
    let (counter, said) = dab("twirl_counterclockwise");
    write("4_twirl_counterclockwise.png", TOWN, &counter);
    t.row(
        "4_twirl_counterclockwise.png, the same dab turning the other way, so it differs from 3_twirl_clockwise.png; draws cleanly",
        &format!("{said:?}, {} pixels differ from 3_twirl_clockwise.png", distance(&counter, &clockwise).1),
        said.is_empty() && distance(&counter, &clockwise).1 > w * h / 100,
    );
    let (pucker, said) = dab("pucker");
    write("5_pucker.png", TOWN, &pucker);
    t.row(
        "5_pucker.png, Pucker, the same dab: the middle of the street shrunk toward the dab; draws cleanly",
        &format!("{said:?}, {} pixels differ from the street", distance(&pucker, &before).1),
        said.is_empty() && distance(&pucker, &before).1 > w * h / 100 && corner(&pucker),
    );
    let (bloat, said) = dab("bloat");
    write("6_bloat.png", TOWN, &bloat);
    t.row(
        "6_bloat.png, Bloat, the same dab: the middle swollen out instead, so it differs from 5_pucker.png; draws cleanly",
        &format!("{said:?}, {} pixels differ from 5_pucker.png", distance(&bloat, &pucker).1),
        said.is_empty() && distance(&bloat, &pucker).1 > w * h / 100 && corner(&bloat),
    );
    let mut sideways = across.clone();
    sideways["tool"] = json!("shift_pixels");
    let (shifted, said) = street(sideways);
    write("7_shift_pixels.png", TOWN, &shifted);
    t.row(
        "7_shift_pixels.png, Shift Pixels along 2_warp.png's stroke: the street under it moved square to the stroke, so it differs from 2_warp.png; draws cleanly",
        &format!("{said:?}, {} pixels differ from 2_warp.png", distance(&shifted, &pushed).1),
        said.is_empty() && distance(&shifted, &pushed).1 > w * h / 100 && corner(&shifted),
    );

    t.finish("D-605_liquify_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B484_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-484: a measurement, run deliberately with --release --ignored"]
fn b484_liquify_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B484_CPU").is_ok();
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
        ("Noise, then Liquify Warp, brush 200, stroked across the middle", Some(json!({"brush_size": 200, "stroke": [25, 50, 75, 50]}))),
        ("Noise, then Liquify Twirl Clockwise, brush 600, one dab", Some(json!({"tool": "twirl_clockwise", "brush_size": 600, "brush_pressure": 100, "stroke": [40, 50]}))),
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
    let out = std::env::var("B484_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-484_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
