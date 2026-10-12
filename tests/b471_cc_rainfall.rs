//! B-471: CC Rainfall in the core, against D-582.
//!
//! Writes `verification/D-582_cc_rainfall_table.md`.
//!
//! Every expected pixel is `Fixtures/cc_rainfall/expected_cc_rainfall.json`, written by
//! `tools/cc_rainfall_reference.py` before this code existed and printed in document 25 as
//! FX-RAIN3D-001 to 037. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also checks that an earlier effect that grows the layer leaves the drops where they were,
//! that the rain falls by the composition's clock, and draws rain over a street into
//! `verification/D-582 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{repo, Table, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::command::{Command, Document};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::model::{Id, Interp};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// CC Rainfall as added, changed by `f`.
fn rain(f: impl FnOnce(&mut Effect)) -> Effect {
    let mut e = Effect::CcRainfall {
        drops: 5000.0,
        size: 3.0,
        scene_depth: 5000.0,
        speed: 4000.0,
        wind: 0.0,
        wind_variation: 0.0,
        spread: 15.0,
        color: "#b4bec8".to_string(),
        opacity: 25.0,
        transfer_mode: "lighten".to_string(),
        composite_with_original: "on".to_string(),
        ground_level: 100.0,
        random_seed: 0.0,
        clock: [0.0, 0.0],
    };
    f(&mut e);
    e
}

fn main_comp() -> Id {
    Id::new(effect_table::MAIN)
}

/// Set the `art` layer's CC Rainfall.
fn set(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: main_comp(), layer_id: Id::new("art"), instance_id: Id::new("fx-1"), effect }
}

/// Key a setting of the `art` layer's CC Rainfall, linearly.
fn keys(setting: &str, values: &[(i32, f64)]) -> Command {
    Command::SetEffectKeys {
        composition: main_comp(),
        layer_id: Id::new("art"),
        instance_id: Id::new("fx-1"),
        setting: setting.to_string(),
        keys: values.iter().map(|&(frame, v)| EffectKey { frame, value: vec![v], interp: Interp::Linear }).collect(),
    }
}

/// The whole composition, to see that a refused command changed nothing.
fn everything(document: &Document) -> String {
    format!("{:?}", document.project().composition(&main_comp()).unwrap())
}

fn refused(t: &mut Table, document: &mut Document, commands: Vec<(&str, Command)>) {
    let held = everything(document);
    for (what, command) in commands {
        let refused = document.apply(command).err();
        let untouched = everything(document) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
}

fn read(file: &str) -> J {
    serde_json::from_str(&fs::read_to_string(repo(&format!("Fixtures/cc_rainfall/{file}"))).unwrap()).unwrap()
}

fn open(project: &J) -> Document {
    persist::load_str(&project.to_string()).expect("the project reads").document
}

/// The largest difference between two frames.
fn apart(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0, f32::max)
}

/// FX-RAIN3D-001's project at the street's size in `dir`, its CC Rainfall's settings changed by
/// `p` (none: no effect); drawn at Full on the processor, straight 8-bit, with what it said.
fn picture(dir: &Path, p: Option<J>, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project = read("fx_rain3d_001.json");
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    match p {
        None => layer["effects"] = json!([]),
        Some(p) => {
            for (k, v) in p.as_object().unwrap() {
                layer["effects"][0]["parameters"][k] = v.clone();
            }
        }
    }
    let document = open(&project);
    let mut log = FrameLog::new(3);
    let drawn = preview::preview_frame(document.project(), &main_comp(), frame, dir, PreviewQuality::Full, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b471_cc_rainfall() {
    let mut t = Table::new(
        "cc_rainfall",
        "# B-471: CC Rainfall\n\nD-582 (b), accepted by the owner on 2026-10-12: \"go with your \
         recommendations on all six\". Every expected pixel is \
         `Fixtures/cc_rainfall/expected_cc_rainfall.json`, written by \
         `tools/cc_rainfall_reference.py` before this code existed and printed in document 25 as \
         FX-RAIN3D-001 to 037. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5. \
         `core.rain` (D-163) is a separate effect and is not changed.\n",
    );

    t.heading("FX-RAIN3D-001 to 037 (document 25)");
    t.fixtures("expected_cc_rainfall.json");

    t.heading("How far it reaches");
    let got = rain(|_| {}).bounds_expansion();
    t.row("CC Rainfall never grows the layer's bounds", &got.to_string(), got == 0);
    let mut draft = rain(|_| {});
    draft.scale_distances(|d| d * 0.5);
    let want = rain(|e| {
        if let Effect::CcRainfall { size, scene_depth, speed, wind, .. } = e {
            (*size, *scene_depth, *speed, *wind) = (1.5, 2500.0, 2000.0, 0.0);
        }
    });
    t.row(
        "a half-size draft preview halves Size, Scene Depth, Speed and Wind, the distances, and \
         nothing else",
        &format!("{draft:?}"),
        draft == want,
    );

    t.heading("Where the drops are");
    let alone = t.render(&t.load("fx_rain3d_017.json").document, 1, 64);
    let mut grown = read("fx_rain3d_017.json");
    let effects = grown["compositions"][0]["layers"][0]["effects"].as_array_mut().unwrap();
    effects.insert(0, json!({"instance_id": "fx-0", "type_id": "core.gaussian_blur", "enabled": true, "parameters": {"sigma_px": 6}}));
    let after_blur = t.render(&open(&grown), 1, 64);
    let d = apart(alone.data(), after_blur.data());
    t.row(
        "FX-RAIN3D-017 (the rain alone) with a Gaussian Blur of 6 before it, which grows the layer: \
         the drops stay where they were in the drawing (within 1e-6)",
        &format!("largest difference {d:e}"),
        d <= 1e-6,
    );
    let at_one = t.render(&t.load("fx_rain3d_002.json").document, 1, 64);
    let mut later = read("fx_rain3d_002.json");
    later["compositions"][0]["layers"][0]["source_offset_frames"] = J::from(3);
    let d = apart(at_one.data(), t.render(&open(&later), 1, 64).data());
    t.row(
        "FX-RAIN3D-002 with the layer's source 3 frames on: frame 1 is the same, since the rain \
         falls by the composition's frame",
        &format!("largest difference {d:e}"),
        d == 0.0,
    );

    t.heading("The file");
    // FX-RAIN3D-022 is saved in small letters, checked below, as fx_rain_015 is for Rain.
    let files: Vec<String> = (1..=37).filter(|&n| n != 22).map(|n| format!("fx_rain3d_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_rain3d_001.json");
    t.row("the clock the effect falls by is never saved", &format!("{:?}", params.get("clock")), params.get("clock").is_none());
    let capitals = t.saved_parameters("fx_rain3d_022.json");
    t.row(
        "fx_rain3d_022.json, its colour written in capitals, is saved in small letters, as Rain's is",
        &capitals["color"].to_string(),
        capitals["color"] == "#c8d8ff",
    );
    for (what, parameters) in [
        ("no `random_seed` at all", r##"{"drops": 5000, "size": 3, "scene_depth": 5000, "speed": 4000, "wind": 0, "wind_variation": 0, "spread": 15, "color": "#b4bec8", "opacity": 25, "transfer_mode": "lighten", "composite_with_original": "on", "ground_level": 100}"##),
        ("no `color` at all", r##"{"drops": 5000, "size": 3, "scene_depth": 5000, "speed": 4000, "wind": 0, "wind_variation": 0, "spread": 15, "opacity": 25, "transfer_mode": "lighten", "composite_with_original": "on", "ground_level": 100, "random_seed": 0}"##),
        ("a drop count that is a word", r##"{"drops": "many", "size": 3, "scene_depth": 5000, "speed": 4000, "wind": 0, "wind_variation": 0, "spread": 15, "color": "#b4bec8", "opacity": 25, "transfer_mode": "lighten", "composite_with_original": "on", "ground_level": 100, "random_seed": 0}"##),
        ("a transfer mode that is a number", r##"{"drops": 5000, "size": 3, "scene_depth": 5000, "speed": 4000, "wind": 0, "wind_variation": 0, "spread": 15, "color": "#b4bec8", "opacity": 25, "transfer_mode": 1, "composite_with_original": "on", "ground_level": 100, "random_seed": 0}"##),
    ] {
        t.shape_refused("fx_rain3d_001.json", what, parameters);
    }

    t.heading("Commands");
    let mut document = t.load("fx_rain3d_001.json").document;
    refused(
        &mut t,
        &mut document,
        vec![
            ("Drops 50001", set(rain(|e| if let Effect::CcRainfall { drops, .. } = e { *drops = 50001.0 }))),
            ("Size -1", set(rain(|e| if let Effect::CcRainfall { size, .. } = e { *size = -1.0 }))),
            ("Spread 91", set(rain(|e| if let Effect::CcRainfall { spread, .. } = e { *spread = 91.0 }))),
            ("Transfer Mode \"screen\"", set(rain(|e| if let Effect::CcRainfall { transfer_mode, .. } = e { *transfer_mode = "screen".into() }))),
            ("Composite With Original \"maybe\"", set(rain(|e| if let Effect::CcRainfall { composite_with_original, .. } = e { *composite_with_original = "maybe".into() }))),
            ("Color \"blue\"", set(rain(|e| if let Effect::CcRainfall { color, .. } = e { *color = "blue".into() }))),
            ("Wind keyed to 6000 at frame 4", keys("wind", &[(0, 0.0), (4, 6000.0)])),
        ],
    );
    t.taken(
        &mut document,
        "fx_rain3d_001.json",
        vec![
            (
                "Composite, alone, wind -300 with variation 40, Color #ffffff",
                set(rain(|e| {
                    if let Effect::CcRainfall { transfer_mode, composite_with_original, wind, wind_variation, color, .. } = e {
                        (*transfer_mode, *composite_with_original, *wind, *wind_variation, *color) =
                            ("composite".into(), "off".into(), -300.0, 40.0, "#ffffff".into());
                    }
                })),
            ),
            ("Drops keyed from 0 at frame 0 to 20000 at frame 4", keys("drops", &[(0, 0.0), (4, 20000.0)])),
        ],
    );

    t.heading("Pictures: rain over a street, in `verification/D-582 pictures/`");
    let dir = std::env::temp_dir().join("b471_pictures");
    fs::create_dir_all(&dir).unwrap();
    png_out::write_rgba(&dir.join("town.png"), TOWN.0, TOWN.1, OutputDepth::Eight, &[], &effect_table::town()).unwrap();
    let out = repo("verification/D-582 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(&dir, None, 2).0;
    let changed = |a: &[u8]| a.chunks_exact(4).zip(street.chunks_exact(4)).filter(|(p, q)| p != q).count();
    let lighter = |a: &[u8]| {
        a.chunks_exact(4).zip(street.chunks_exact(4)).all(|(p, q)| (0..3).all(|c| p[c] >= q[c]))
    };
    for (name, p, frame, what) in [
        ("1_before", None, 2, "the street with no effect"),
        ("2_as_added", Some(json!({})), 2, "as added: 5000 fine pale drops through a deep volume, Lighten, so nothing darker"),
        ("3_storm", Some(json!({"drops": 8000, "size": 4, "speed": 6000, "wind": 1500, "wind_variation": 40, "opacity": 45, "random_seed": 4})), 2, "a storm: more, bigger, faster drops blown to the right"),
        ("4_near_drops", Some(json!({"drops": 300, "scene_depth": 400, "size": 6, "speed": 2400, "spread": 30, "opacity": 60, "color": "#e0e8ff", "random_seed": 9})), 2, "a few big near drops, each its own tilt"),
        ("5_ground_level_70", Some(json!({"drops": 8000, "ground_level": 70, "opacity": 40, "size": 4})), 2, "Ground Level 70: no rain on the road"),
        ("6_dark_rain_composite", Some(json!({"drops": 4000, "size": 4, "color": "#202838", "opacity": 60, "transfer_mode": "composite"})), 2, "dark drops with Composite: they darken the street"),
        ("7_rain_alone", Some(json!({"drops": 8000, "size": 4, "opacity": 60, "composite_with_original": "off"})), 2, "the rain alone, without the street"),
    ] {
        let (bytes, said) = picture(&dir, p.clone(), frame);
        png_out::write_rgba(&out.join(format!("{name}.png")), TOWN.0, TOWN.1, OutputDepth::Eight, &[], &bytes).unwrap();
        let n = changed(&bytes);
        let road_untouched = bytes[210 * TOWN.0 * 4..] == street[210 * TOWN.0 * 4..];
        let shape = match name {
            "1_before" => n == 0,
            "2_as_added" | "3_storm" | "4_near_drops" => n > 0 && lighter(&bytes),
            // Ground level 70 of 270 rows: nothing from row 189 down.
            "5_ground_level_70" => n > 0 && road_untouched && bytes[189 * TOWN.0 * 4..] == street[189 * TOWN.0 * 4..],
            "6_dark_rain_composite" => n > 0 && !lighter(&bytes),
            _ => bytes.chunks_exact(4).any(|p| p[3] == 0) && bytes.chunks_exact(4).any(|p| p[3] > 0),
        };
        t.row(
            &format!("{name}.png, frame {frame}, {what}: drawn without a warning"),
            &format!("{said:?}; {n} pixels differ from the street"),
            said.is_empty() && shape,
        );
    }

    t.heading("Played frame after frame with the viewer's cache (D-528)");
    for file in ["fx_rain3d_021.json", "fx_rain3d_002.json"] {
        let played = t.load(file).document;
        let mut cache = anime_compositor::cache::CelCache::viewer();
        let same = (0..5)
            .filter(|&frame| {
                let mut log = FrameLog::new(3);
                let warm = preview::preview_frame_cached(played.project(), &main_comp(), frame, &t.root, PreviewQuality::Full, 64, &mut log, &mut cache)
                    .expect("the frame draws");
                let mut log = FrameLog::new(3);
                let alone = preview::preview_frame(played.project(), &main_comp(), frame, &t.root, PreviewQuality::Full, 64, &mut log)
                    .expect("the frame draws");
                warm.data() == alone.data()
            })
            .count();
        t.row(
            &format!("{file}, frames 0 to 4 asked for in order through one viewer's cache, as playing \
                      asks: each is byte-identical to the same frame drawn alone with no cache, as an \
                      export draws it (the drawing is still, the rain is not)"),
            &format!("{same} of 5 byte-identical; {} bytes kept for effects", cache.effect_held_bytes()),
            same == 5,
        );
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_rain3d_001.json", 2), ("fx_rain3d_009.json", 1), ("fx_rain3d_018.json", 1), ("fx_rain3d_021.json", 3)]);

    t.finish("D-582_cc_rainfall_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-460's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so no finished frame is kept, then CC
/// Rainfall on the first of them; every eighth frame of 240 asked for whole at Full as the viewer
/// asks, Draw on: GPU. The first loop starts with empty caches and its 30 frames' median is
/// "first"; the median of the seven loops after it is "again".
#[test]
#[ignore = "B-471: a measurement, run deliberately with --release --ignored"]
fn b471_cc_rainfall_timing() {
    use anime_compositor::cache::CelCache;
    use anime_compositor::compose::DEFAULT_TILE_SIZE;
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
    let base = read("fx_rain3d_001.json")["compositions"][0]["layers"][0]["effects"][0]["parameters"].clone();
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone", None),
        ("Noise, then CC Rainfall as added (5000 drops)", Some(json!({}))),
        ("Noise, then CC Rainfall, 20000 drops, size 4", Some(json!({"drops": 20000, "size": 4}))),
        ("Noise, then CC Rainfall, 5000 near drops (depth 500), size 6", Some(json!({"scene_depth": 500, "size": 6}))),
    ];
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    for (name, p) in shots {
        let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
        for (i, id) in ["a", "b", "c"].iter().enumerate() {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let (0, Some(p)) = (i, &p) {
                let mut all = base.clone();
                for (k, v) in p.as_object().unwrap() {
                    all[k] = v.clone();
                }
                v.push(json!({"instance_id": "rain", "type_id": "core.cc_rainfall", "enabled": true, "parameters": all}));
            }
            j["compositions"][0]["layers"][i]["effects"] = J::Array(v);
        }
        let project = persist::load_str(&j.to_string()).expect("the reference shot reads").document.project().clone();
        let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
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
    let out = std::env::var("B471_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-471_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
