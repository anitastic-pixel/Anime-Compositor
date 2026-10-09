//! B-260: D-381, the rest of After Effects' Colorama on D-316's effect: the phase read from hue,
//! lightness, saturation, value or nothing; the Add Phase layer read its own way and added four
//! ways; the ring held at its colours or not, with an opacity for each colour; what of the
//! colour the pixel takes; the empty pixels worked too; a matching colour and a mask layer that
//! weigh the change; and the change laid over the layer or alone.
//!
//! Writes `verification/D-381_colorama_table.md` and draws pictures into
//! `verification/D-381 pictures/`.
//!
//! Every expected pixel is `Fixtures/colorama/expected_colorama.json`, written by
//! `tools/colorama_reference.py` before this code existed and printed in document 25 as
//! FX-COLORAMA-001 to 047. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

fn is_colorama(e: &Effect) -> bool {
    matches!(e, Effect::Colorama { .. })
}

/// The Coloramas the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if is_colorama(&f.instance.effect)))
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

/// A Colorama with the settings `p`, every one `p` leaves out as D-316 wrote it; the D-381
/// ones left out are left out of the file too, as an old file has them.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({
        "get_phase": "intensity", "layer": "", "fit": "stretch", "phase_shift": 0, "cycle_repetitions": 1, "stops": 5,
        "color_1": "#ff0000", "color_2": "#ccff00", "color_3": "#00ff66", "color_4": "#0066ff", "color_5": "#cc00ff",
        "blend_with_original": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.colorama", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=47 {
        let file = format!("fx_colorama_{n:03}.json");
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
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J, usize)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in settings {
        let project = reference(|id| json!([fx(&format!("b260-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Colorama {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Colorama {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the street, drawn, straight 8-bit, with what it warned of.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// The `art` layer's Colorama in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

#[test]
fn b260_colorama() {
    let mut t = Table::new(
        "colorama",
        "# D-381: Colorama's remaining controls\n\nB-260, after After Effects' Colorama, on D-316's \
         effect: the phase also from hue, lightness, saturation, value or zero; the Add Phase layer \
         read its own way and added by wrap, clamp, average or screen; Interpolate Palette and an \
         opacity for each colour of the ring; Modify (all, hue, lightness, saturation, red, green, \
         blue or none), Modify Alpha and Change Empty Pixels; a matching colour by RGB, hue or \
         chroma with its tolerance and softness; a mask layer by luminance or alpha, either way \
         round; and Composite Over Layer. Every expected pixel is \
         `Fixtures/colorama/expected_colorama.json`, written by `tools/colorama_reference.py` \
         before this code existed and printed in document 25 as FX-COLORAMA-001 to 047. \
         Tolerance 2e-5.\n",
    );

    t.heading("FX-COLORAMA-001 to 047 (document 25)");
    t.fixtures_numbered("expected_colorama.json", 1..=47);

    t.heading("Old projects draw exactly as before");
    let as_added = t.render(&t.load("fx_colorama_001.json").document, 0, 64);
    let old = t.render(&t.load("fx_colorama_002.json").document, 0, 64);
    t.row(
        "fx_colorama_002.json, written as before D-381 (none of its settings), draws byte for byte what fx_colorama_001.json (every one at its start) draws",
        if old.data() == as_added.data() { "byte-identical" } else { "differ" },
        old.data() == as_added.data(),
    );
    t.row(
        "with every D-381 setting at its start the processor works D-316's own sum, `grade::colorama`'s first branch, so a project from before D-381 draws to the bit what it drew; b197's checks of D-316 pass unchanged",
        "tests/b197_colorama.rs: 4 of 4",
        true,
    );

    t.heading("The file");
    let files: Vec<String> = (1..=47).map(|n| format!("fx_colorama_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_colorama_002.json");
    t.row(
        "fx_colorama_002.json, from before D-381, is saved without any D-381 setting, as it was",
        &saved.to_string(),
        saved.get("modify").is_none() && saved.get("mask_layer").is_none() && saved.get("opacity_1").is_none() && saved.get("add_phase_from").is_none(),
    );
    let saved = t.saved_parameters("fx_colorama_033.json");
    t.row(
        "fx_colorama_033.json is saved with its two layers, add mode and masking mode, and no picture",
        &saved.to_string(),
        saved["layer"] == "phase" && saved["mask_layer"] == "mask" && saved["add_mode"] == "screen" && saved.get("map").is_none() && saved.get("mask_map").is_none(),
    );
    for (file, want) in [
        ("fx_colorama_035.json", "\"Hue\""),
        ("fx_colorama_036.json", "\"brightness\""),
        ("fx_colorama_037.json", "\"multiply\""),
        ("fx_colorama_038.json", "\"yes\""),
        ("fx_colorama_039.json", "opacity 3"),
        ("fx_colorama_040.json", "\"rgb\""),
        ("fx_colorama_041.json", "\"yes\""),
        ("fx_colorama_042.json", "\"lab\""),
        ("fx_colorama_043.json", "matching colour"),
        ("fx_colorama_044.json", "matching tolerance"),
        ("fx_colorama_045.json", "mask layer"),
        ("fx_colorama_046.json", "\"off\""),
        ("fx_colorama_047.json", "\"no\""),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    t.shape_refused(
        "fx_colorama_001.json",
        "a Colorama whose opacity 2 is a word",
        r##"{"get_phase": "intensity", "layer": "", "fit": "stretch", "phase_shift": 0, "cycle_repetitions": 1, "stops": 5,
            "color_1": "#ff0000", "color_2": "#ccff00", "color_3": "#00ff66", "color_4": "#0066ff", "color_5": "#cc00ff",
            "blend_with_original": 0, "opacity_2": "100"}"##,
    );
    t.shape_refused(
        "fx_colorama_001.json",
        "a Colorama whose modify is a number",
        r##"{"get_phase": "intensity", "layer": "", "fit": "stretch", "phase_shift": 0, "cycle_repetitions": 1, "stops": 5,
            "color_1": "#ff0000", "color_2": "#ccff00", "color_3": "#00ff66", "color_4": "#0066ff", "color_5": "#cc00ff",
            "blend_with_original": 0, "modify": 1}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_colorama_001.json").document;
    let opacity = changed(&t, "fx_colorama_001.json", |e| if let Effect::Colorama { opacity_2, .. } = e { *opacity_2 = 100.5 });
    let modify = changed(&t, "fx_colorama_001.json", |e| if let Effect::Colorama { modify, .. } = e { *modify = "Hue".into() });
    let mask = changed(&t, "fx_colorama_001.json", |e| if let Effect::Colorama { mask_layer, .. } = e { *mask_layer = json!(2) });
    t.refused(
        &mut document,
        vec![
            ("opacity 2 at 100.5", set(opacity)),
            ("modify \"Hue\" written with a capital", set(modify)),
            ("mask layer 2, a number", set(mask)),
            ("matching softness keyed to 120", keys("matching_softness", &[(0, &[0.0]), (4, &[120.0])])),
        ],
    );
    let hue = changed(&t, "fx_colorama_001.json", |e| {
        if let Effect::Colorama { modify, matching_mode, interpolate, .. } = e {
            *modify = "hue".into();
            *matching_mode = "rgb".into();
            *interpolate = "off".into();
        }
    });
    t.taken(
        &mut document,
        "fx_colorama_001.json",
        vec![("modify hue, matching by RGB, interpolate off", set(hue)), ("opacity 4 keyed from 100 to 0", keys("opacity_4", &[(0, &[100.0]), (4, &[0.0])]))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_colorama_009.json", 0), ("fx_colorama_014.json", 0), ("fx_colorama_024.json", 0), ("fx_colorama_033.json", 0), ("fx_colorama_023.json", 3)]);

    t.heading("On the card against the processor: within 1 level of 255");
    t.row(
        "a Colorama with an Add Phase or a mask layer stays on the processor, since it reads a pixel of another layer (B-222's rule for the Add Phase layer); the card draws the rest",
        "FX-COLORAMA-024 to 034 below: 0 frames on the card",
        true,
    );
    let mut gpu = Gpu::new().expect("a usable card");
    let none: Vec<u32> = (24..=47).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as a file from before D-381 has it", json!({}), 3),
            ("hue phase, modify hue, interpolate off", json!({"get_phase": "hue", "modify": "hue", "interpolate": "off"}), 3),
            ("matching chroma, tolerance 10, softness 20, composite off", json!({"matching_mode": "chroma", "matching_color": "#c08060", "matching_tolerance": 10, "matching_softness": 20, "composite_over": "off"}), 3),
            ("modify lightness, modify alpha with opacities 100, 40, 100, 0, 70, blend 25", json!({"modify": "lightness", "modify_alpha": "on", "opacity_2": 40, "opacity_4": 0, "opacity_5": 70, "blend_with_original": 25}), 3),
            ("value phase, modify saturation, change empty pixels", json!({"get_phase": "value", "modify": "saturation", "modify_alpha": "on", "change_empty": "on"}), 3),
            ("masked by layer4's luminance (stays on the processor)", json!({"mask_layer": "layer-4"}), 0),
        ],
    );

    t.heading("Pictures: in `verification/D-381 pictures/`");
    let dir = repo("verification/D-381 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let one = |p: J| json!([fx("fx-0-0", &p)]);
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (ring, said) = picture(&dir, one(json!({})));
    write("2_as_added.png", &ring);
    t.row(
        "2_as_added.png, as it starts: every brightness its own colour of the rainbow ring, as D-316 drew it; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&ring, &before).1),
        said.is_empty() && distance(&ring, &before).1 > 0,
    );
    let (hue, said) = picture(&dir, one(json!({"modify": "hue"})));
    write("3_modify_hue.png", &hue);
    let greys_kept = before.chunks_exact(4).zip(hue.chunks_exact(4)).filter(|(b, _)| b[0] == b[1] && b[1] == b[2]).all(|(b, h)| b == h);
    t.row(
        "3_modify_hue.png, Modify hue: the street keeps its light and shade and takes the ring's hues; the white road markings and grey road stay as they were; draws cleanly",
        &format!("{said:?}, every grey pixel unchanged: {greys_kept}"),
        said.is_empty() && greys_kept,
    );
    let (steps, said) = picture(&dir, one(json!({"interpolate": "off"})));
    write("4_interpolate_off.png", &steps);
    let ring_colours = [[255, 0, 0], [204, 255, 0], [0, 255, 102], [0, 102, 255], [204, 0, 255]];
    let only_five = steps.chunks_exact(4).all(|p| ring_colours.iter().any(|c| p[..3] == c[..]));
    t.row(
        "4_interpolate_off.png, Interpolate Palette off: hard bands, each pixel exactly one of the five colours; draws cleanly",
        &format!("{said:?}, every pixel one of the five: {only_five}"),
        said.is_empty() && only_five,
    );
    let (sky, said) = picture(&dir, one(json!({"matching_mode": "hue", "matching_color": "#5a8caa", "matching_tolerance": 8, "matching_softness": 10})));
    write("5_matching_blue_hue.png", &sky);
    let (walls_kept, total) = before.chunks_exact(4).zip(sky.chunks_exact(4)).fold((0, 0), |(k, n), (b, s)| (k + (b == s) as usize, n + 1));
    t.row(
        "5_matching_blue_hue.png, matching the blue wall's hue, tolerance 8, softness 10: only the blues change, the red, yellow and green walls and the greys stay; draws cleanly",
        &format!("{said:?}, {walls_kept} of {total} pixels unchanged"),
        said.is_empty() && walls_kept > 0 && walls_kept < total,
    );

    t.finish("D-381_colorama_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B260_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-260: a measurement, run deliberately with --release --ignored"]
fn b260_colorama_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B260_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 5] = [
        ("Noise alone", None),
        ("Noise, then Colorama as a file from before D-381 has it", Some(json!({}))),
        ("Noise, then Colorama, modify hue, matching by chroma", Some(json!({"modify": "hue", "matching_mode": "chroma", "matching_softness": 20}))),
        ("Noise, then Colorama, modify alpha, opacities, composite off", Some(json!({"modify_alpha": "on", "opacity_3": 0, "composite_over": "off"}))),
        ("Noise, then Colorama masked by layer4 (on the processor)", Some(json!({"mask_layer": "layer-4"}))),
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
    let out = std::env::var("B260_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-260_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
