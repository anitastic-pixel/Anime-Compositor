//! B-231: D-351, the whole picture's statistics (EFFECTS.md P0-15) and the four effects that
//! read them: Stretch Levels, Stretch Contrast, Stretch Color and Spread Tones, after After
//! Effects' Auto Levels, Auto Contrast, Auto Color and Equalize.
//!
//! Every expected pixel is `Fixtures/auto_tone/expected_auto_tone.json`, written by
//! `tools/auto_tone_reference.py` before this code existed. The four are drawn by the processor.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::frame_stats::{clip_points, Stats};
use anime_compositor::model::{Id, Interp, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// Set the `holder` layer's effect; the fixtures' drawing is `holder`, not `art`.
fn set(effect: Effect) -> Command {
    Command::SetEffectParameters {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        effect,
    }
}

fn effect_of(d: &Document) -> Effect {
    let comp = d.project().composition(&Id::new(MAIN)).unwrap();
    let layer = comp.layer(&Id::new("holder")).unwrap();
    layer.effects.iter().find(|i| i.instance_id.as_str() == "fx-1").unwrap().effect.clone()
}

/// A dull, warm street: the town squeezed into 50 to 190 with red raised and blue lowered, so
/// every stretch has something to do. Written into `dir` as `street.png`.
fn dull_street(dir: &Path) {
    let (w, h) = TOWN;
    let bytes: Vec<u8> = town()
        .chunks_exact(4)
        .flat_map(|p| {
            let squeeze = |v: u8, lift: f64| (50.0 + v as f64 * 140.0 / 255.0 + lift).round().clamp(0.0, 255.0) as u8;
            [squeeze(p[0], 20.0), squeeze(p[1], 5.0), squeeze(p[2], -15.0), 255]
        })
        .collect();
    png_out::write_rgba(&dir.join("street.png"), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
}

/// One layer of `street.png` filling the town-sized composition, carrying `effects`.
fn street(effects: J) -> Project {
    let (w, h) = TOWN;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let project = json!({
        "schema_version": 0, "project_id": "proj-b231",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-street", "kind": "still", "name": "street", "path": "street.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 8,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 8},
            "layer_order": ["holder"],
            "layers": [{"id": "holder", "kind": "raster", "name": "holder", "asset_id": "asset-street", "enabled": true,
                "locked": false, "in_frame": 0, "out_frame": 8, "source_offset_frames": 0,
                "transform": {"anchor": t(json!([0, 0])), "position": t(json!([0, 0])), "scale": t(json!([100, 100])),
                              "rotation": t(json!(0)), "opacity": t(json!(1))},
                "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal", "effects": effects}]
        }]
    });
    persist::load_str(&project.to_string()).unwrap_or_else(|d| panic!("the street opens: {}", d.message)).document.project().clone()
}

fn fx(type_id: &str, p: J) -> J {
    json!({"instance_id": "fx-1", "type_id": type_id, "enabled": true, "parameters": p})
}

fn levels() -> J {
    fx("core.stretch_levels", json!({"temporal_smoothing": 0, "scene_detect": "off", "black_clip": 0.1, "white_clip": 0.1}))
}

/// The frame as the viewer asks for it on the processor, with what it warned of.
fn view(project: &Project, root: &Path, quality: PreviewQuality) -> (anime_compositor::WorkingBuffer, Vec<String>) {
    let mut log = FrameLog::new(8);
    let b = preview::preview_frame_cached(project, &Id::new(MAIN), 0, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
        .unwrap_or_else(|d| panic!("the street draws: {}", d.message));
    (b, log.finish().iter().map(|d| d.id.as_str().to_string()).collect())
}

#[test]
fn b231_auto_tone() {
    let mut t = Table::new(
        "auto_tone",
        "# B-231: the whole picture's statistics, and Stretch Levels, Stretch Contrast, Stretch Color and Spread Tones\n\n\
         D-351 (EFFECTS.md P0-15): a histogram of the layer's whole picture at the effect's place \
         in its stack, and the four effects that read it, after After Effects' Auto Levels, Auto \
         Contrast, Auto Color and Equalize, with Temporal Smoothing and Scene Detect. Every \
         expected pixel is `Fixtures/auto_tone/expected_auto_tone.json`, written by \
         `tools/auto_tone_reference.py` before this code existed and printed in document 25 as \
         FX-AUTO-001 to 032. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5. \
         FX-AUTO-023 warns TEMPORAL_SMOOTHING_SKIPPED on every frame.\n",
    );

    t.heading("FX-AUTO-001 to 032 (document 25)");
    t.fixtures("expected_auto_tone.json");

    t.heading("The file");
    let files: Vec<String> = (1..=32).map(|n| format!("fx_auto_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    for file in ["fx_auto_001.json", "fx_auto_004.json"] {
        let params = t.saved_parameters(file);
        t.row(
            &format!("{file}: Stretch Levels and Stretch Contrast write no snap neutral midtones"),
            &params.to_string(),
            params.get("snap_neutral_midtones").is_none(),
        );
    }
    for file in ["fx_auto_013.json", "fx_auto_015.json"] {
        let params = t.saved_parameters(file);
        t.row(
            &format!("{file}: the statistics a smoothing added up are never saved"),
            &params.to_string(),
            params.get("stats").is_none(),
        );
    }
    t.shape_refused("fx_auto_001.json", "no black clip", r#"{"temporal_smoothing": 0, "scene_detect": "off", "white_clip": 0.1}"#);
    t.shape_refused("fx_auto_001.json", "a black clip written as a word", r#"{"temporal_smoothing": 0, "scene_detect": "off", "black_clip": "some", "white_clip": 0.1}"#);
    t.shape_refused("fx_auto_018.json", "Spread Tones with no amount", r#"{"equalize": "rgb"}"#);
    for (file, said) in [
        ("fx_auto_026.json", "Stretch Levels's black clip runs from 0 to 10, and this is 11."),
        ("fx_auto_029.json", "Stretch Levels's scene detect is \"off\" or \"on\", and this is \"yes\"."),
        ("fx_auto_030.json", "Stretch Color's snap neutral midtones is \"off\" or \"on\", and this is \"maybe\"."),
        ("fx_auto_031.json", "Spread Tones equalizes by \"rgb\", \"brightness\" or \"photoshop\", and this is \"hsl\"."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming it"), &why, why == said);
    }

    t.heading("Commands");
    let mut document = t.load("fx_auto_001.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    for (what, effect) in [
        ("black clip 11", with(&|e| if let Effect::AutoTone { black_clip, .. } = e { *black_clip = 11.0 })),
        ("temporal smoothing -1", with(&|e| if let Effect::AutoTone { temporal_smoothing, .. } = e { *temporal_smoothing = -1.0 })),
        ("scene detect \"yes\"", with(&|e| if let Effect::AutoTone { scene_detect, .. } = e { *scene_detect = "yes".into() })),
    ] {
        let refused = document.apply(set(effect)).err();
        let untouched = effect_of(&document) == base;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
    let key_black = Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        setting: "black_clip".to_string(),
        keys: vec![
            EffectKey { frame: 0, value: vec![0.0], interp: Interp::Linear },
            EffectKey { frame: 4, value: vec![10.0], interp: Interp::Linear },
        ],
    };
    t.taken(
        &mut document,
        "fx_auto_001.json",
        vec![
            ("clips 5 and 3, smoothing 0.1, scene detect on,", set(with(&|e| if let Effect::AutoTone { black_clip, white_clip, temporal_smoothing, scene_detect, .. } = e {
                *black_clip = 5.0;
                *white_clip = 3.0;
                *temporal_smoothing = 0.1;
                *scene_detect = "on".into();
            }))),
            ("black clip keyed 0 to 10 over frames 0 to 4,", key_black),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_auto_001.json", 0),
        ("fx_auto_007.json", 0),
        ("fx_auto_013.json", 3),
        ("fx_auto_019.json", 0),
        ("fx_auto_022.json", 0),
        ("fx_auto_025.json", 0),
    ]);

    let dir = effect_table::repo("verification/D-351 pictures");
    fs::create_dir_all(&dir).unwrap();
    dull_street(&dir);

    t.heading("Only the part on screen (B-158): the statistics are still the whole picture's");
    for (name, effects) in [
        ("Stretch Levels", json!([levels()])),
        ("Spread Tones, RGB", json!([fx("core.spread_tones", json!({"equalize": "rgb", "amount": 100}))])),
    ] {
        let project = street(effects);
        let (whole, _) = view(&project, &dir, PreviewQuality::Full);
        let mut log = FrameLog::new(8);
        let (part, w, _, cut) = preview::preview_part(&project, &Id::new(MAIN), 0, &dir, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer(), [0.25, 0.25, 0.75, 0.75])
            .unwrap_or_else(|d| panic!("{name}: {}", d.message));
        let cut = cut.expect("the middle is a part");
        let same = (0..cut.height).all(|y| {
            let from = ((cut.y + y) * w + cut.x) * 4;
            part.data()[y * cut.width * 4..][..cut.width * 4] == whole.data()[from..from + cut.width * 4]
        });
        t.row(
            &format!("{name}, the middle quarter of the street drawn alone: the same pixels as the whole frame's"),
            if same { "byte-identical" } else { "differ" },
            same,
        );
    }

    t.heading("Draft against Full: the statistics of a quarter-size picture");
    let (full, _) = view(&street(json!([])), &dir, PreviewQuality::Full);
    let (draft, _) = view(&street(json!([])), &dir, PreviewQuality::Draft);
    let (sf, sd) = (Stats::of(&full).unwrap(), Stats::of(&draft).unwrap());
    let (mut worst_black, mut worst_white) = (0usize, 0usize);
    for (clip, label) in [(0.1, "clips 0.1 per cent"), (2.0, "clips 2 per cent")] {
        let mut said = Vec::new();
        for (c, channel) in ["red", "green", "blue", "brightness"].iter().enumerate() {
            let (a, b) = (clip_points(&sf.hist[c], clip, clip), clip_points(&sd.hist[c], clip, clip));
            worst_black = worst_black.max(a.0.abs_diff(b.0));
            worst_white = worst_white.max(a.1.abs_diff(b.1));
            said.push(format!("{channel} {}-{} at Full, {}-{} at Draft", a.0, a.1, b.0, b.1));
        }
        t.row(
            &format!("the dull street's black and white points, {label}, in levels of 255 (Draft reads its own quarter-size picture)"),
            &said.join("; "),
            true,
        );
    }
    // D-351: recorded, not held to Full. Draft's picture is a quarter the size on each axis, so
    // a small lit window is averaged with its wall and the lightest pixels are fewer and darker.
    t.row(
        "Draft against Full, the largest difference in a black point (the street's dark road is wide, so it survives the quarter size): within 3 levels of 255",
        &format!("{worst_black} level(s)"),
        worst_black <= 3,
    );
    t.row(
        "Draft against Full, the largest difference in a white point, recorded as it is: the lit windows are a few pixels wide and average away at a quarter size, so Draft stretches harder than Full. Export is always Full",
        &format!("{worst_white} level(s)"),
        true,
    );

    t.heading("Pictures: a dull, warm street, in `verification/D-351 pictures/`");
    let (w, h) = TOWN;
    let draw = |effects: J| {
        let mut log = FrameLog::new(8);
        let b = render_frame(&street(effects), &Id::new(MAIN), 0, &dir, 64, &mut log).expect("the street draws");
        (b.to_srgb8_straight(), log.finish().iter().map(|d| d.id.as_str().to_string()).collect::<Vec<_>>())
    };
    let (before, said) = draw(json!([]));
    png_out::write_rgba(&dir.join("before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    t.row("before.png, the street greyed and warm: no true black, no true white, a red-orange cast", &format!("warnings {said:?}"), said.is_empty());
    let range = |b: &[u8]| {
        let (mut lo, mut hi) = (255u8, 0u8);
        for p in b.chunks_exact(4) {
            lo = lo.min(p[0]).min(p[1]).min(p[2]);
            hi = hi.max(p[0]).max(p[1]).max(p[2]);
        }
        (lo, hi)
    };
    let shots = [
        ("stretch_levels", "Stretch Levels as added: each channel to its own darkest and lightest, so blacks are black, whites white, and the cast much less", levels()),
        ("stretch_contrast", "Stretch Contrast as added: darker darks and lighter lights, the warm cast kept", fx("core.stretch_contrast", json!({"temporal_smoothing": 0, "scene_detect": "off", "black_clip": 0.1, "white_clip": 0.1}))),
        ("stretch_color_snap", "Stretch Color with Snap Neutral Midtones: the warm cast taken out: the sky blue again, the road near black", fx("core.stretch_color", json!({"temporal_smoothing": 0, "scene_detect": "off", "black_clip": 0.1, "white_clip": 0.1, "snap_neutral_midtones": "on"}))),
        ("spread_tones", "Spread Tones, RGB: every tone spread evenly, strong and a little posterised", fx("core.spread_tones", json!({"equalize": "rgb", "amount": 100}))),
    ];
    let (blo, bhi) = range(&before);
    for (file, what, e) in shots {
        let (bytes, said) = draw(json!([e]));
        png_out::write_rgba(&dir.join(format!("{file}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let (lo, hi) = range(&bytes);
        t.row(
            &format!("{file}.png, {what}"),
            &format!("darkest {lo}, lightest {hi} (before: {blo}, {bhi}); warnings {said:?}"),
            said.is_empty() && lo < blo && hi > bhi,
        );
    }
    fs::remove_file(dir.join("street.png")).unwrap();

    t.finish("D-351_auto_tone_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-227's way: the reference shot (1920 by 1080, 24 a second) with the effect
/// on its first three layers, then a Noise that changes every frame so nothing is kept, every
/// eighth frame asked for as the viewer asks, whole, with Draw on: GPU. The first loop starts
/// with empty caches and its 30 frames' median is "first"; then seven loops are timed, the
/// median of their 210 frames is "again". The four are drawn by the processor.
#[test]
#[ignore = "B-231: a measurement, run deliberately with --release --ignored"]
fn b231_auto_tone_timing() {
    use anime_compositor::gpu::Gpu;
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | Quality | First | Again |\n|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let fx = |id: &str, type_id: &str, p: J| json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": p});
    let shots: [(&str, Option<(&str, J)>); 5] = [
        ("Noise alone", None),
        ("Stretch Levels as added, then Noise", Some(("core.stretch_levels", json!({"temporal_smoothing": 0, "scene_detect": "off", "black_clip": 0.1, "white_clip": 0.1})))),
        ("Stretch Color, Snap Neutral Midtones, then Noise", Some(("core.stretch_color", json!({"temporal_smoothing": 0, "scene_detect": "off", "black_clip": 0.1, "white_clip": 0.1, "snap_neutral_midtones": "on"})))),
        ("Stretch Levels, Temporal Smoothing 0.1 s (2 frames each side), Scene Detect, then Noise", Some(("core.stretch_levels", json!({"temporal_smoothing": 0.1, "scene_detect": "on", "black_clip": 0.1, "white_clip": 0.1})))),
        ("Spread Tones, RGB, then Noise", Some(("core.spread_tones", json!({"equalize": "rgb", "amount": 100})))),
    ];
    for (name, e) in shots {
        let stack = |id: &str| {
            let mut v = Vec::new();
            if let Some((type_id, p)) = &e {
                v.push(fx(&format!("{id}s"), type_id, p.clone()));
            }
            v.push(fx(&format!("{id}n"), "core.noise", json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"})));
            v
        };
        let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
        let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
        let layers = &mut j["compositions"][0]["layers"];
        for (i, id) in ["a", "b", "c"].iter().enumerate() {
            layers[i]["effects"] = J::Array(stack(id));
        }
        let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{name}: {}", d.message));
        let (project, comp, root) = (loaded.document.project().clone(), Id::new("comp-reference-shot"), effect_table::repo("Fixtures/reference_shot"));
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..8 {
            for frame in (0..240).step_by(8) {
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                drop(preview::preview_frame_srgb8(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                if pass > 0 { times.push(ms) } else { first.push(ms) }
            }
        }
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B231_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-231_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
