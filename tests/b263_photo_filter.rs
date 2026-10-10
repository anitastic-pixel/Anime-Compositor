//! B-263: D-384, after After Effects' Photo Filter: the picture as if shot through a coloured
//! glass filter, picked from six presets or given as a colour, its Density saying how strongly
//! it tints, Preserve Luminosity keeping each pixel's brightness.
//!
//! Writes `verification/D-384_photo_filter_table.md` and draws pictures into
//! `verification/D-384 pictures/`.
//!
//! Every expected pixel is `Fixtures/photo_filter/expected_photo_filter.json`, written by
//! `tools/photo_filter_reference.py` before this code existed and printed in document 25 as
//! FX-PFILT-001 to 020. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

fn is_pf(e: &Effect) -> bool {
    matches!(e, Effect::PhotoFilter { .. })
}

/// The Photo Filters the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if is_pf(&f.instance.effect)))
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

/// A Photo Filter with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"filter": "warming_85", "color": "#ec8a00", "density": 25, "preserve_luminosity": "on"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.photo_filter", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=20 {
        let file = format!("fx_pfilt_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b263-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Photo Filter {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Photo Filter {what} on three layers, frame {frame}, {}", quality.label()),
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

/// The `art` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

/// The mean of red less blue over the covered pixels, in levels.
fn warmth(p: &[u8]) -> f64 {
    let px: Vec<&[u8]> = p.chunks_exact(4).filter(|q| q[3] > 0).collect();
    px.iter().map(|q| q[0] as f64 - q[2] as f64).sum::<f64>() / px.len().max(1) as f64
}

#[test]
fn b263_photo_filter() {
    let mut t = Table::new(
        "photo_filter",
        "# D-384: Photo Filter\n\nB-263, after After Effects' Photo Filter: the picture as if shot \
         through a coloured glass filter, picked from six presets or given as a colour, its Density \
         saying how strongly it tints, Preserve Luminosity keeping each pixel's brightness. Every \
         expected pixel is `Fixtures/photo_filter/expected_photo_filter.json`, written by \
         `tools/photo_filter_reference.py` before this code existed and printed in document 25 as \
         FX-PFILT-001 to 020. Tolerance 2e-5.\n",
    );

    t.heading("FX-PFILT-001 to 020 (document 25)");
    t.fixtures_numbered("expected_photo_filter.json", 1..=20);

    t.heading("The file");
    let files: Vec<String> = (1..=20).map(|n| format!("fx_pfilt_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_pfilt_011.json");
    t.row(
        "fx_pfilt_011.json is saved with its four settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 4
            && saved["filter"] == "custom"
            && saved["color"] == "#ff00ff"
            && saved["density"] == 50.0
            && saved["preserve_luminosity"] == "off",
    );
    for (file, want) in [
        ("fx_pfilt_015.json", "density"),
        ("fx_pfilt_016.json", "density"),
        ("fx_pfilt_017.json", "filter"),
        ("fx_pfilt_018.json", "filter"),
        ("fx_pfilt_019.json", "colour"),
        ("fx_pfilt_020.json", "preserve luminosity"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    t.shape_refused(
        "fx_pfilt_001.json",
        "a Photo Filter whose density is a word",
        r##"{"filter": "warming_85", "color": "#ec8a00", "density": "25", "preserve_luminosity": "on"}"##,
    );
    t.shape_refused(
        "fx_pfilt_001.json",
        "a Photo Filter without its filter",
        r##"{"color": "#ec8a00", "density": 25, "preserve_luminosity": "on"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_pfilt_001.json").document;
    let dense = changed(&t, "fx_pfilt_001.json", |e| if let Effect::PhotoFilter { density, .. } = e { *density = 101.0 });
    let word = changed(&t, "fx_pfilt_001.json", |e| if let Effect::PhotoFilter { filter, .. } = e { *filter = "warm".into() });
    t.refused(
        &mut document,
        vec![
            ("density 101", set(dense)),
            ("filter \"warm\"", set(word)),
            ("density keyed to 150", keys("density", &[(0, &[25.0]), (4, &[150.0])])),
        ],
    );
    let cool = changed(&t, "fx_pfilt_001.json", |e| {
        if let Effect::PhotoFilter { filter, density, .. } = e {
            *filter = "cooling_80".into();
            *density = 80.0;
        }
    });
    t.taken(
        &mut document,
        "fx_pfilt_001.json",
        vec![("Cooling Filter (80) at density 80", set(cool)), ("density keyed from 0 to 100", keys("density", &[(0, &[0.0]), (4, &[100.0])]))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_pfilt_001.json", 0), ("fx_pfilt_004.json", 0), ("fx_pfilt_009.json", 0), ("fx_pfilt_013.json", 2), ("fx_pfilt_014.json", 3)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Density 0 (005) and refused (015 to 020) leave nothing for the card.
    let none: Vec<u32> = std::iter::once(5).chain(15..=20).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added, Warming Filter (85) at 25", json!({}), 3),
            ("Cooling Filter (80) at 70, luminosity kept", json!({"filter": "cooling_80", "density": 70}), 3),
            ("custom #ff00ff at 50, luminosity off", json!({"filter": "custom", "color": "#ff00ff", "density": 50, "preserve_luminosity": "off"}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-384 pictures/`");
    let dir = repo("verification/D-384 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let one = |p: J| json!([fx("fx-0-0", &p)]);
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = picture(&dir, one(json!({"density": 0})));
    t.row(
        "density 0 changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );
    for (name, p, warmer) in [
        ("2_warming_85.png, as added, Warming Filter (85) at 25: the street warmer", json!({}), true),
        ("3_cooling_80.png, Cooling Filter (80) at 60: the street cooler", json!({"filter": "cooling_80", "density": 60}), false),
        ("4_sepia.png, Sepia at 80: the street brown", json!({"filter": "sepia", "density": 80}), true),
        ("5_underwater.png, Underwater at 70: the street green-blue, the reds pulled down", json!({"filter": "underwater", "density": 70}), false),
    ] {
        let (after, said) = picture(&dir, one(p));
        write(name.split(',').next().unwrap(), &after);
        let (w0, w1) = (warmth(&before), warmth(&after));
        t.row(
            &format!("{name}; draws cleanly"),
            &format!("{said:?}, red less blue {w0:.1} before, {w1:.1} after"),
            said.is_empty() && (w1 > w0) == warmer && w1 != w0,
        );
    }
    let (dark, said) = picture(&dir, one(json!({"filter": "custom", "color": "#3366cc", "density": 60, "preserve_luminosity": "off"})));
    write("6_custom_blue_unkept.png", &dark);
    let darker = before.chunks_exact(4).zip(dark.chunks_exact(4)).all(|(b, d)| (0..3).all(|c| d[c] <= b[c]));
    t.row(
        "6_custom_blue_unkept.png, a custom blue #3366cc at 60, luminosity off: the street bluer and darker, no channel brighter; draws cleanly",
        &format!("{said:?}, no channel brighter: {darker}"),
        said.is_empty() && darker && dark != before,
    );

    t.finish("D-384_photo_filter_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B263_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-263: a measurement, run deliberately with --release --ignored"]
fn b263_photo_filter_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B263_CPU").is_ok();
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
        ("Noise, then Photo Filter as added (Warming (85), 25, luminosity kept)", Some(json!({}))),
        ("Noise, then Photo Filter, custom colour at 60, luminosity off", Some(json!({"filter": "custom", "color": "#3366cc", "density": 60, "preserve_luminosity": "off"}))),
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
    let out = std::env::var("B263_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-263_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
