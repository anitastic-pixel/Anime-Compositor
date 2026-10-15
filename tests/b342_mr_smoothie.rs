//! B-342: D-462, After Effects' CC Mr. Smoothie under `core.mr_smoothie`: Flow Layer, Property,
//! Smoothness, Sample A, Sample B, Phase and Color Loop.
//!
//! Writes `verification/D-462_mr_smoothie_table.md` and draws pictures into
//! `verification/D-462 pictures/`.
//!
//! Every expected pixel is `Fixtures/mr_smoothie/expected_mr_smoothie.json`, written by
//! `tools/mr_smoothie_reference.py` before this code existed and printed in document 25 as
//! FX-SMOOTHIE-001 to 026. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, distance, keys, median, on_card, reference, repo, set, town, Table, MAIN, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// A Mr. Smoothie with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"layer": "", "fit": "stretch", "property": "luminance", "smoothness": 5, "sample_a": [25, 50], "sample_b": [75, 50], "phase": 0, "color_loop": "aba"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.mr_smoothie", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=26 {
        let file = format!("fx_smoothie_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::MrSmoothie { .. }));
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
        let project = reference(|id| json!([fx(&format!("b342-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Mr. Smoothie {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::MrSmoothie { .. }));
                t.row(
                    &format!("the reference shot, Mr. Smoothie {what} on three layers, frame {frame}, {}", quality.label()),
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
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

/// Pixel (x, y) of a picture `w` wide.
fn at(p: &[u8], w: usize, x: usize, y: usize) -> &[u8] {
    &p[(y * w + x) * 4..(y * w + x) * 4 + 4]
}

/// How many pixels of `p` and `q` are within one level of each other, out of those of `p`.
fn alike(p: &[u8], q: &[u8]) -> usize {
    p.chunks_exact(4).zip(q.chunks_exact(4)).filter(|(a, b)| a.iter().zip(*b).all(|(x, y)| x.abs_diff(*y) <= 1)).count()
}

/// `command` aimed at the `holder` layer's effect `fx-1`; the fixtures' drawing is `holder`, not `art`.
fn holder(mut command: Command) -> Command {
    if let Command::SetEffectParameters { layer_id, instance_id, .. } | Command::SetEffectKeys { layer_id, instance_id, .. } = &mut command {
        *layer_id = Id::new("holder");
        *instance_id = Id::new("fx-1");
    }
    command
}

#[test]
fn b342_mr_smoothie() {
    let mut t = Table::new(
        "mr_smoothie",
        "# D-462: Mr. Smoothie\n\nB-342: `core.mr_smoothie` takes After Effects' CC Mr. Smoothie controls \
         (Flow Layer, Property, Smoothness, Sample A, Sample B, Phase, Color Loop). Every expected pixel is \
         `Fixtures/mr_smoothie/expected_mr_smoothie.json`, written by `tools/mr_smoothie_reference.py` \
         before this code existed and printed in document 25 as FX-SMOOTHIE-001 to 026. \
         Tolerance 2e-5.\n",
    );

    t.heading("FX-SMOOTHIE-001 to 026 (document 25)");
    t.fixtures_numbered("expected_mr_smoothie.json", 1..=26);

    t.heading("The file");
    let files: Vec<String> = (1..=26).map(|n| format!("fx_smoothie_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_smoothie_011.json");
    t.row(
        "fx_smoothie_011.json is saved with its eight settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 8
            && saved["layer"] == "photo"
            && saved["fit"] == "stretch"
            && saved["property"] == "saturation"
            && saved["smoothness"] == 2.0
            && saved["sample_a"] == json!([25, 50])
            && saved["sample_b"] == json!([75, 50])
            && saved["phase"] == 0.0
            && saved["color_loop"] == "bab",
    );
    for (file, want) in [
        ("fx_smoothie_019.json", "smoothness"),
        ("fx_smoothie_020.json", "property"),
        ("fx_smoothie_021.json", "color loop"),
        ("fx_smoothie_022.json", "flow layer"),
        ("fx_smoothie_023.json", "sample b"),
        ("fx_smoothie_024.json", "phase"),
        ("fx_smoothie_025.json", "fit"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Mr. Smoothie and its {want}"), &why, why.contains("Mr. Smoothie") && why.contains(want));
    }
    let full = r#"{"layer": "", "fit": "stretch", "property": "luminance", "smoothness": 5, "sample_a": [25, 50], "sample_b": [75, 50], "phase": 0, "color_loop": "aba"}"#;
    t.shape_refused("fx_smoothie_001.json", "a Mr. Smoothie whose smoothness is a word", &full.replace("\"smoothness\": 5", "\"smoothness\": \"soft\""));
    t.shape_refused("fx_smoothie_001.json", "a Mr. Smoothie without its color loop", &full.replace(", \"color_loop\": \"aba\"", ""));
    t.shape_refused("fx_smoothie_001.json", "a Mr. Smoothie whose sample A is one number", &full.replace("[25, 50]", "25"));
    t.shape_refused("fx_smoothie_001.json", "a Mr. Smoothie whose property is a number", &full.replace("\"luminance\"", "4"));

    t.heading("Commands");
    let mut document = t.load("fx_smoothie_001.json").document;
    let over = changed(&t, "fx_smoothie_001.json", |e| if let Effect::MrSmoothie { smoothness, .. } = e { *smoothness = 101.0 });
    let far = changed(&t, "fx_smoothie_001.json", |e| if let Effect::MrSmoothie { sample_b, .. } = e { *sample_b = [50.0, -1001.0] });
    let word = changed(&t, "fx_smoothie_001.json", |e| if let Effect::MrSmoothie { color_loop, .. } = e { *color_loop = "abab".into() });
    let effect_of = |d: &Document| d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].clone();
    let held = format!("{:?}", effect_of(&document));
    for (what, command) in [
        ("smoothness 101", set(over)),
        ("sample B 50, -1001", set(far)),
        ("color loop abab", set(word)),
        ("smoothness keyed to 150", keys("smoothness", &[(0, &[5.0]), (4, &[150.0])])),
    ] {
        let refused = document.apply(holder(command)).err();
        let untouched = format!("{:?}", effect_of(&document)) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
    let moved = changed(&t, "fx_smoothie_001.json", |e| {
        if let Effect::MrSmoothie { layer, fit, property, smoothness, sample_a, sample_b, phase, color_loop, .. } = e {
            *layer = json!("ramp");
            *fit = "tile".into();
            *property = "hue".into();
            *smoothness = 3.0;
            *sample_a = [10.0, 20.0];
            *sample_b = [90.0, 80.0];
            *phase = 45.0;
            *color_loop = "bab".into();
        }
    });
    t.taken(
        &mut document,
        "fx_smoothie_001.json",
        vec![
            ("the ramp tiled, Hue, smoothness 3, samples 10, 20 and 90, 80, phase 45, BAB", holder(set(moved))),
            ("sample A keyed from 0, 50 to 100, 50", holder(keys("sample_a", &[(0, &[0.0, 50.0]), (4, &[100.0, 50.0])]))),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_smoothie_001.json", 0),
        ("fx_smoothie_002.json", 0),
        ("fx_smoothie_009.json", 0),
        ("fx_smoothie_012.json", 0),
        ("fx_smoothie_013.json", 0),
        ("fx_smoothie_015.json", 2),
        ("fx_smoothie_016.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Refused ones (019 to 026) leave nothing for the card.
    let none: Vec<u32> = (19..=26).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            (
                "layer4 as the flow, Hue, smoothness 20, samples 10, 20 and 90, 80, phase 90, BAB",
                json!({"layer": "layer-4", "property": "hue", "smoothness": 20, "sample_a": [10, 20], "sample_b": [90, 80], "phase": 90, "color_loop": "bab"}),
                3,
            ),
            ("Alpha, smoothness 0, AB, phase 45", json!({"property": "alpha", "smoothness": 0, "color_loop": "ab", "phase": 45}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-462 pictures/`");
    let dir = repo("verification/D-462 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]), 0);
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]), 0);
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (w, h) = TOWN;
    let all = w * h;
    let coverings = |p: &[u8]| p.chunks_exact(4).map(|q| q[3]).collect::<Vec<_>>();

    let (added, said) = street(json!({}));
    write("2_as_added.png", TOWN, &added);
    t.row(
        "2_as_added.png, as added (its own luminance, smoothness 5, ABA): the street repainted in bands of the colours between a quarter and three quarters across its middle, its covering exactly as before; draws cleanly",
        &format!("{said:?}, changed {}, coverings the same {}", added != before, coverings(&added) == coverings(&before)),
        said.is_empty() && added != before && coverings(&added) == coverings(&before),
    );

    let (hue, said) = street(json!({"property": "hue", "smoothness": 10, "color_loop": "bab"}));
    write("3_hue_bab.png", TOWN, &hue);
    t.row(
        "3_hue_bab.png, Hue, smoothness 10, BAB: not 2_as_added.png, the covering kept; draws cleanly",
        &format!("{said:?}, different {}, coverings the same {}", hue != added, coverings(&hue) == coverings(&before)),
        said.is_empty() && hue != added && coverings(&hue) == coverings(&before),
    );

    let (turned, said) = street(json!({"phase": 180}));
    write("4_phase_180.png", TOWN, &turned);
    let (round, said2) = street(json!({"phase": 360}));
    t.row(
        "4_phase_180.png, as added with Phase 180: the bands' colours moved half way round, not 2_as_added.png; Phase 360 brings back 2_as_added.png within 1 level at almost every pixel; draws cleanly",
        &format!("{said:?} {said2:?}, different {}, Phase 360 within 1 level at {} of {all}", turned != added, alike(&round, &added)),
        said.is_empty() && said2.is_empty() && turned != added && alike(&round, &added) * 100 >= all * 99,
    );

    let (one, said) = street(json!({"sample_a": [50, 50], "sample_b": [50, 50]}));
    write("5_one_point.png", TOWN, &one);
    let first = at(&one, w, 0, 0).to_vec();
    let same = one.chunks_exact(4).filter(|q| q.iter().zip(&first).all(|(x, y)| x.abs_diff(*y) <= 1)).count();
    t.row(
        "5_one_point.png, both samples at the middle: every pixel the one colour found there (within 1 level); draws cleanly",
        &format!("{said:?}, {same} of {all} pixels the colour {first:?}"),
        said.is_empty() && same == all,
    );

    let (soft, said) = street(json!({"smoothness": 40}));
    write("6_smoothness_40.png", TOWN, &soft);
    t.row(
        "6_smoothness_40.png, smoothness 40: wider, smoother bands than 2_as_added.png, the covering kept; draws cleanly",
        &format!("{said:?}, different {}, coverings the same {}", soft != added, coverings(&soft) == coverings(&before)),
        said.is_empty() && soft != added && coverings(&soft) == coverings(&before),
    );

    t.finish("D-462_mr_smoothie_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B342_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-342: a measurement, run deliberately with --release --ignored"]
fn b342_mr_smoothie_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B342_CPU").is_ok();
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
        ("Noise, then Mr. Smoothie as added", Some(json!({}))),
        ("the same, Hue, smoothness 30, BAB, phase 90", Some(json!({"property": "hue", "smoothness": 30, "color_loop": "bab", "phase": 90}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}b"), p));
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
    let out = std::env::var("B342_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-342_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
