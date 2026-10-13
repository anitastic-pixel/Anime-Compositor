//! B-513: D-633, after After Effects' Shatter (Simulation): the layer broken into pieces of a
//! pattern or a custom map, those a force reaches thrown, spun and pulled by gravity, drawn as
//! slabs seen by the effect's own camera or the composition's and lit. Adobe's help names the
//! controls and says what several do but publishes no formula: the rule, the units and most
//! settings when added are ours, stated in `tools/shatter_reference.py`.
//!
//! Writes `verification/D-633_shatter_table.md` and draws pictures into
//! `verification/D-633 pictures/`.
//!
//! Every expected pixel is `Fixtures/shatter/expected_shatter.json`, written by the tool before
//! this code existed and printed in document 25 as FX-SH-001 to 078. Tolerance 2e-5. Nothing
//! here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{repo, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// The `holder` layer's effect in `file`.
fn effect(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].effect.clone()
}

fn read(file: &str) -> J {
    serde_json::from_str(&fs::read_to_string(repo(&format!("Fixtures/shatter/{file}"))).unwrap()).unwrap()
}

/// FX-SH-001's project at the street's size in `dir` (the street the holder's drawing, the
/// fixtures' map layers as they are), the holder's Shatter settings changed by `p`; `frame` drawn
/// on the processor, straight 8-bit, with what it warned of.
fn picture(dir: &Path, p: &J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project = read("fx_sh_001.json");
    let a = project["assets"].as_array_mut().unwrap().iter_mut().find(|a| a["id"] == "asset-shot").unwrap();
    a["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let holder = comp["layers"].as_array_mut().unwrap().iter_mut().find(|l| l["id"] == "holder").unwrap();
    let parameters = &mut holder["effects"][0]["parameters"];
    for (k, v) in p.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b513_shatter() {
    let mut t = Table::new(
        "shatter",
        "# D-633: Shatter\n\nB-513, after After Effects' Shatter (Simulation), under the \
         coordinator's approval of the D-630 design for the owner's /loop request. The layer \
         broken into pieces of one of ten patterns, turned by Direction about the Origin, or of \
         a Custom Shatter Map; the pieces a force sphere reaches freed (a Gradient Layer and its \
         Shatter Threshold allowing), thrown from the force's centre with Randomness and Mass \
         Variance, slowed by Viscosity, pulled by Gravity, tumbling about the Tumble Axis, walked \
         four steps a frame from the layer's in point; drawn as slabs with front, side and back \
         textures in six modes, seen by the effect's own camera or the composition camera and lit \
         by Card Wipe's light. Adobe's help publishes no formula, so the rule, the units and most \
         settings when added are ours, stated in `tools/shatter_reference.py`. Every expected \
         pixel is `Fixtures/shatter/expected_shatter.json`, written by the tool before this code \
         existed and printed in document 25 as FX-SH-001 to 078. Tolerance 2e-5.\n",
    );

    t.heading("FX-SH-001 to 078 (document 25)");
    t.fixtures_numbered("expected_shatter.json", 1..=78);

    t.heading("The settings");
    let plain = effect(&t, "fx_sh_001.json");
    t.row("Shatter never grows the layer's bounds", &plain.bounds_expansion().to_string(), plain.bounds_expansion() == 0);
    let mut draft = plain.clone();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes no setting, each a share of the layer, a count or a word (the composition camera, worked out each frame, is halved)",
        &format!("{draft:?}"),
        draft == plain,
    );
    for (file, want) in [
        ("fx_sh_061.json", "not built yet (d-636)"),
        ("fx_sh_062.json", "pattern"),
        ("fx_sh_063.json", "not built yet (d-635)"),
        ("fx_sh_064.json", "render"),
        ("fx_sh_065.json", "tumble axis"),
        ("fx_sh_066.json", "front mode"),
        ("fx_sh_068.json", "repetitions"),
        ("fx_sh_069.json", "extrusion depth"),
        ("fx_sh_070.json", "force 1 radius"),
        ("fx_sh_071.json", "shatter threshold"),
        ("fx_sh_072.json", "gravity inclination"),
        ("fx_sh_073.json", "colour"),
        ("fx_sh_074.json", "custom shatter map"),
        ("fx_sh_075.json", "white tiles fixed"),
        ("fx_sh_077.json", "invert gradient"),
        ("fx_sh_078.json", "mass variance"),
    ] {
        // A setting keyed out of range is refused when the file is read, by the warning.
        let mut why = effect(&t, file).why_invalid();
        if why.is_empty() {
            why = t.load(file).warnings.iter().map(|d| format!("{} {}", d.message, d.detail)).collect::<Vec<_>>().join(" ");
        }
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }

    t.heading("The file");
    let files: Vec<String> = (1..=78).map(|n| format!("fx_sh_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_sh_055.json");
    t.row(
        "fx_sh_055.json is saved with its fifty-two settings, a point light at [10, 10], Tumble Axis \"y\", Repetitions 4, no gradient layer",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 52
            && saved["light_type"] == "point"
            && saved["light_position"] == json!([10, 10])
            && saved["tumble_axis"] == "y"
            && saved["repetitions"] == 4
            && saved["gradient_layer"] == "",
    );
    let never = ["custom_map", "gradient_map", "front_map", "side_map", "back_map", "seen", "history", "fps"];
    t.row(
        "the five pictures the effect is drawn with, the composition camera, and the numbers at each frame from the in point are never saved",
        &format!("{:?}", never.iter().map(|k| saved.get(*k)).collect::<Vec<_>>()),
        never.iter().all(|k| saved.get(*k).is_none()),
    );
    let good = read("fx_sh_002.json")["compositions"][0]["layers"][5]["effects"][0]["parameters"].clone();
    let mut p = good.clone();
    p["repetitions"] = json!("many");
    t.shape_refused("fx_sh_002.json", "a Shatter whose repetitions is a word", &p.to_string());
    let mut p = good.clone();
    p["force_1_position"] = json!(50);
    t.shape_refused("fx_sh_002.json", "a Shatter whose force 1 position is a number, not a list", &p.to_string());
    let mut p = good.clone();
    p.as_object_mut().unwrap().remove("tumble_axis");
    t.shape_refused("fx_sh_002.json", "a Shatter with no tumble axis", &p.to_string());
    let mut p = good;
    p.as_object_mut().unwrap().remove("side_layer");
    t.shape_refused("fx_sh_002.json", "a Shatter with no side layer", &p.to_string());

    t.heading("Pictures (verification/D-633 pictures)");
    let dir = std::env::temp_dir().join("b513_pictures");
    fs::create_dir_all(dir.join("media")).unwrap();
    png_out::write_rgba(&dir.join("town.png"), TOWN.0, TOWN.1, OutputDepth::Eight, &[], &effect_table::town()).unwrap();
    for m in ["back", "ramp", "colours", "cracks"] {
        fs::copy(repo(&format!("Fixtures/shatter/media/{m}.png")), dir.join(format!("media/{m}.png"))).unwrap();
    }
    let out = repo("verification/D-633 pictures");
    fs::create_dir_all(&out).unwrap();
    for (name, p, frame) in [
        ("1_as_added_frame_2", json!({}), 2),
        ("2_hexagons_burst", json!({"pattern": "hexagons", "repetitions": 16, "force_1_strength": 1.5, "force_1_radius": 0.3}), 3),
        ("3_glass_two_forces", json!({"pattern": "glass", "repetitions": 14, "force_1_position": [25, 50], "force_1_radius": 0.2, "force_1_strength": 2, "force_2_position": [80, 40], "force_2_radius": 0.2, "force_2_strength": -1.5}), 3),
        ("4_custom_cracks", json!({"pattern": "custom", "custom_shatter_map": "cracks", "force_1_radius": 10, "force_1_strength": 0.5, "gravity": 1}), 4),
        ("5_ramp_threshold", json!({"gradient_layer": "ramp", "shatter_threshold": 50, "force_1_radius": 10, "force_1_strength": 0.5, "gravity": 1}), 4),
        ("6_red_sides_seen_from_the_side", json!({"side_mode": "color", "color": "#ff4040", "extrusion_depth": 0.3, "tumble_axis": "y", "rotation_speed": 0.5, "camera_y_rotation": 30, "back_layer": "back"}), 3),
        ("7_composition_camera", json!({"camera_system": "comp_camera", "force_1_strength": 1, "gravity": 1}), 3),
        ("8_point_light_shine", json!({"tumble_axis": "y", "rotation_speed": 0.5, "light_type": "point", "light_position": [20, 20], "light_depth": 0.3, "specular_reflection": 1, "highlight_sharpness": 10}), 3),
    ] {
        let (bytes, said) = picture(&dir, &p, frame);
        png_out::write_rgba(&out.join(format!("{name}.png")), TOWN.0, TOWN.1, OutputDepth::Eight, &[], &bytes).unwrap();
        let mut colours: Vec<[u8; 3]> = bytes.chunks_exact(4).map(|p| [p[0], p[1], p[2]]).collect();
        colours.sort();
        colours.dedup();
        let clear = bytes.chunks_exact(4).filter(|p| p[3] == 0).count();
        t.row(
            &format!("{name}.png (frame {frame}): drawn without a warning, {} distinct colours", colours.len()),
            &format!("{} colours; {clear} clear pixels where pieces left; warnings {said:?}", colours.len()),
            said.is_empty() && colours.len() > 100,
        );
    }
    t.finish("D-633_shatter_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The reference shot (1920 by 1080) with `stack` on its first layer.
fn reference(stack: J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    j["compositions"][0]["layers"][0]["effects"] = stack;
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// The frame times: the reference shot (1920 by 1080, 24 a second) with Shatter on its first
/// layer, frames 0 and 120 each drawn on the processor with empty caches, three times; the median
/// of the three. Frame 120 walks 480 steps from the in point.
#[test]
#[ignore = "B-513: a measurement, run deliberately with --release --ignored"]
fn b513_shatter_timing() {
    use std::fmt::Write as _;

    let mut s = format!(
        "- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: the processor (Shatter has no card path)\n\n\
         | Shot | Frame 0 | Frame 120 |\n|---|---:|---:|\n",
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
    let base = read("fx_sh_001.json")["compositions"][0]["layers"][5]["effects"][0].clone();
    let with = |extra: J| {
        let mut e = base.clone();
        for (k, v) in extra.as_object().unwrap() {
            e["parameters"][k] = v.clone();
        }
        json!([e])
    };
    for (name, stack) in [
        ("no effect", json!([])),
        ("Shatter as added (bricks 10 across)", with(json!({}))),
        ("glass 50 across, every piece freed", with(json!({"pattern": "glass", "repetitions": 50, "force_1_radius": 10}))),
        ("force swept across over frames 0 to 120 (the release walked every step)", with(json!({"force_1_position": {"base": [0, 50], "keyframes": [{"frame": 0, "value": [0, 50], "interp": "linear"}, {"frame": 120, "value": [100, 50], "interp": "linear"}]}, "force_1_radius": 0.1}))),
        ("1000 across (the ceiling)", with(json!({"repetitions": 1000}))),
    ] {
        let project = reference(stack);
        let _ = write!(s, "| {name} |");
        for frame in [0, 120] {
            let times: Vec<f64> = (0..3)
                .map(|_| {
                    let mut log = FrameLog::new(3);
                    let t = std::time::Instant::now();
                    drop(
                        preview::preview_frame_cached(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                            .expect("CPU frame"),
                    );
                    t.elapsed().as_secs_f64() * 1000.0
                })
                .collect();
            let _ = write!(s, " {:.1} ms |", median(times));
        }
        s.push('\n');
    }
    let out = std::env::var("B513_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-513_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
