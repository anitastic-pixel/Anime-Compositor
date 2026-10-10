//! B-281: D-402, After Effects' Tritone under `core.tritone`: Highlights, Midtones, Shadows and
//! Blend With Original, a second name over Gradient Map's engine (as D-383 and D-394).
//!
//! Writes `verification/D-402_tritone_table.md` and draws pictures into `verification/D-402 pictures/`.
//!
//! Every expected pixel is `Fixtures/tritone/expected_tritone.json`, written by
//! `tools/tritone_reference.py` before this code existed and printed in document 25 as
//! FX-TRITONE-001 to 015; Gradient Map's own FX-GRADMAP-001 to 023 rerun unchanged. Tolerance
//! 2e-5. Nothing here is a snapshot of a run.

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

/// The Tritones the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::Tritone { .. })))
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

/// A Tritone with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"highlights": "#ffffff", "midtones": "#8c7355", "shadows": "#000000", "blend_with_original": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.tritone", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=15 {
        let file = format!("fx_tritone_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b281-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Tritone {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Tritone {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor, straight
/// 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), effects: J) -> (Vec<u8>, Vec<String>) {
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

/// Covered pixels of `before` and `after` side by side.
fn pairs<'a>(before: &'a [u8], after: &'a [u8]) -> impl Iterator<Item = (&'a [u8], &'a [u8])> {
    before.chunks_exact(4).zip(after.chunks_exact(4)).filter(|(b, _)| b[3] > 0)
}

#[test]
fn b281_tritone() {
    let mut t = Table::new(
        "tritone",
        "# D-402: Tritone\n\nB-281: `core.tritone` takes After Effects' Highlights, Midtones, \
         Shadows and Blend With Original, a second name over Gradient Map's engine (as D-383 and \
         D-394): Gradient Map with its midpoint at the middle and its amount one hundred less the \
         blend. Every expected pixel is `Fixtures/tritone/expected_tritone.json`, written by \
         `tools/tritone_reference.py` before this code existed and printed in document 25 as \
         FX-TRITONE-001 to 015; Gradient Map's FX-GRADMAP-001 to 023 rerun unchanged. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-TRITONE-001 to 015 (document 25)");
    t.fixtures_numbered("expected_tritone.json", 1..=15);

    t.heading("Gradient Map, the engine under the second name: its fixtures, unchanged");
    let root = t.root.clone();
    t.root = repo("Fixtures/gradient_map");
    t.fixtures_numbered("expected_gradient_map.json", 1..=23);
    t.root = root;

    t.heading("The file");
    // 006, its colours in capitals, is saved in small letters (the row after).
    let files: Vec<String> = (1..=15).filter(|n| *n != 6).map(|n| format!("fx_tritone_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let caps = t.saved_parameters("fx_tritone_006.json");
    t.row(
        "fx_tritone_006.json, its colours written in capitals, is saved in small letters, as Gradient Map's are",
        &format!("{} {} {}", caps["highlights"], caps["midtones"], caps["shadows"]),
        caps["highlights"] == "#fdf3a7" && caps["midtones"] == "#c0392b" && caps["shadows"] == "#1a2a6c",
    );
    let saved = t.saved_parameters("fx_tritone_007.json");
    t.row(
        "fx_tritone_007.json is saved with its four settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 4
            && saved["highlights"] == "#fdf3a7"
            && saved["midtones"] == "#c0392b"
            && saved["shadows"] == "#1a2a6c"
            && saved["blend_with_original"] == 70.0,
    );
    for (file, want) in [
        ("fx_tritone_010.json", "blend_with_original"),
        ("fx_tritone_011.json", "blend_with_original"),
        ("fx_tritone_013.json", "Highlights"),
        ("fx_tritone_014.json", "Midtones"),
        ("fx_tritone_015.json", "Shadows"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        let says = why.contains(want) || why.contains(&want.replace('_', " "));
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, says);
    }
    t.shape_refused("fx_tritone_001.json", "a Tritone whose blend is a word", r##"{"highlights": "#ffffff", "midtones": "#8c7355", "shadows": "#000000", "blend_with_original": "none"}"##);
    t.shape_refused("fx_tritone_001.json", "a Tritone without its Midtones", r##"{"highlights": "#ffffff", "shadows": "#000000", "blend_with_original": 0}"##);
    t.shape_refused("fx_tritone_001.json", "a Tritone whose Shadows is a number", r##"{"highlights": "#ffffff", "midtones": "#8c7355", "shadows": 0, "blend_with_original": 0}"##);

    t.heading("Commands");
    let mut document = t.load("fx_tritone_001.json").document;
    let over = changed(&t, "fx_tritone_001.json", |e| if let Effect::Tritone { blend_with_original, .. } = e { *blend_with_original = 101.0 });
    let bad = changed(&t, "fx_tritone_001.json", |e| if let Effect::Tritone { midtones, .. } = e { *midtones = "#fff".into() });
    t.refused(
        &mut document,
        vec![
            ("blend 101", set(over)),
            ("Midtones #fff", set(bad)),
            ("blend keyed to 150", keys("blend_with_original", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    let night = changed(&t, "fx_tritone_001.json", |e| {
        if let Effect::Tritone { highlights, midtones, shadows, .. } = e {
            *highlights = "#fdf3a7".into();
            *midtones = "#c0392b".into();
            *shadows = "#1a2a6c".into();
        }
    });
    t.taken(
        &mut document,
        "fx_tritone_001.json",
        vec![
            ("night to sunset", set(night)),
            ("blend keyed from 0 to 100", keys("blend_with_original", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_tritone_001.json", 0),
        ("fx_tritone_004.json", 0),
        ("fx_tritone_007.json", 0),
        ("fx_tritone_008.json", 2),
        ("fx_tritone_009.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Blend 100 (002) and refused (010 to 015) leave nothing for the card.
    let none: Vec<u32> = std::iter::once(2).chain(10..=15).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            ("night to sunset at blend 70", json!({"highlights": "#fdf3a7", "midtones": "#c0392b", "shadows": "#1a2a6c", "blend_with_original": 70}), 3),
            ("one colour three times", json!({"highlights": "#6450a0", "midtones": "#6450a0", "shadows": "#6450a0"}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-402 pictures/`");
    let dir = repo("verification/D-402 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = street(json!({"blend_with_original": 100}));
    t.row(
        "blend with original 100 changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );

    let luma = |p: &[u8]| 0.2126 * p[0] as f64 + 0.7152 * p[1] as f64 + 0.0722 * p[2] as f64;
    let (sepia, said) = street(json!({}));
    write("2_as_added.png", TOWN, &sepia);
    // Sepia: every middling pixel warmer than it is blue (red above blue).
    let warm = pairs(&before, &sepia).filter(|(b, _)| (60.0..200.0).contains(&luma(b))).all(|(_, a)| a[0] > a[2]);
    t.row(
        "2_as_added.png, as added (white, a sepia brown, black): the street in browns, every middling pixel redder than it is blue; draws cleanly",
        &format!("{said:?}, all warm: {warm}"),
        said.is_empty() && warm && sepia != before,
    );

    let (night, said) = street(json!({"highlights": "#fdf3a7", "midtones": "#c0392b", "shadows": "#1a2a6c"}));
    write("3_night_to_sunset.png", TOWN, &night);
    // The darkest parts go navy (blue above red) and the lightest pale yellow (red above blue).
    let dark_blue = pairs(&before, &night).filter(|(b, _)| luma(b) < 40.0).all(|(_, a)| a[2] > a[0]);
    let light_yellow = pairs(&before, &night).filter(|(b, _)| luma(b) > 215.0).all(|(_, a)| a[0] > a[2]);
    t.row(
        "3_night_to_sunset.png, shadows navy (#1a2a6c), midtones red (#c0392b), highlights pale yellow (#fdf3a7): the dark parts bluer than red, the light parts redder than blue; draws cleanly",
        &format!("{said:?}, dark parts blue: {dark_blue}, light parts yellow: {light_yellow}"),
        said.is_empty() && dark_blue && light_yellow,
    );

    let (part, said) = street(json!({"highlights": "#fdf3a7", "midtones": "#c0392b", "shadows": "#1a2a6c", "blend_with_original": 70}));
    write("4_blend_70.png", TOWN, &part);
    t.row(
        "4_blend_70.png, the same three at blend 70: the street with a light wash of them, nearer the street than 3 is; draws cleanly",
        &format!("{said:?}, largest difference from the street {} against 3's {}", distance(&part, &before).0, distance(&night, &before).0),
        said.is_empty() && distance(&part, &before).0 < distance(&night, &before).0,
    );

    // The second name: Gradient Map with the same three colours, midpoint 50 and amount one
    // hundred less the blend, draws the same street byte for byte.
    for (what, p) in [
        ("as added", json!({})),
        ("night to sunset", json!({"highlights": "#fdf3a7", "midtones": "#c0392b", "shadows": "#1a2a6c"})),
        ("night to sunset at blend 70", json!({"highlights": "#fdf3a7", "midtones": "#c0392b", "shadows": "#1a2a6c", "blend_with_original": 70})),
        ("one colour three times at blend 25", json!({"highlights": "#6450a0", "midtones": "#6450a0", "shadows": "#6450a0", "blend_with_original": 25})),
    ] {
        let tri = fx("fx-0-0", &p);
        let q = &tri["parameters"];
        let blend = q["blend_with_original"].as_f64().unwrap();
        let gm = json!({"instance_id": "fx-0-0", "type_id": "core.gradient_map", "enabled": true, "parameters": {
            "shadow_color": q["shadows"], "midtone_color": q["midtones"], "highlight_color": q["highlights"],
            "midpoint": 50, "amount": 100.0 - blend}});
        let (a, said0) = street(p.clone());
        let (b, said1) = picture(&dir, "town.png", TOWN, json!([gm]));
        if what == "night to sunset" {
            write("5_gradient_map_same.png", TOWN, &b);
        }
        t.row(
            &format!("Tritone {what} against Gradient Map with the same colours, midpoint 50 and amount {}: the same street byte for byte; both draw cleanly", 100.0 - blend),
            &format!("{said0:?} {said1:?}, {} pixels differ", distance(&a, &b).1),
            said0.is_empty() && said1.is_empty() && a == b && a != before,
        );
    }

    t.finish("D-402_tritone_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B281_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-281: a measurement, run deliberately with --release --ignored"]
fn b281_tritone_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B281_CPU").is_ok();
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
        ("Noise, then Tritone as added (white, sepia, black, blend 0)", Some(json!({}))),
        ("Noise, then Tritone, night to sunset at blend 70", Some(json!({"highlights": "#fdf3a7", "midtones": "#c0392b", "shadows": "#1a2a6c", "blend_with_original": 70}))),
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
    let out = std::env::var("B281_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-281_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
