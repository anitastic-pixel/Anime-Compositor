//! B-512: D-632, after After Effects' Card Dance (Simulation): the layer cut into a grid of
//! cards, each moved, turned and scaled by a gradient layer under its centre, seen by the effect's
//! own camera or the composition's and lit. Adobe's help names the controls and says what several
//! do but publishes no formula: the rule, the units and every setting when added are ours, stated
//! in `tools/card_dance_reference.py`.
//!
//! Writes `verification/D-632_card_dance_table.md` and draws pictures into
//! `verification/D-632 pictures/`.
//!
//! Every expected pixel is `Fixtures/card_dance/expected_card_dance.json`, written by the tool
//! before this code existed and printed in document 25 as FX-CD-001 to 064. Tolerance 2e-5.
//! Nothing here is a snapshot of a run.

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
    serde_json::from_str(&fs::read_to_string(repo(&format!("Fixtures/card_dance/{file}"))).unwrap()).unwrap()
}

/// FX-CD-001's project at the street's size in `dir` (the street the holder's drawing, the
/// fixtures' map layers as they are), the holder's Card Dance settings changed by `p`; frame 0
/// drawn on the processor, straight 8-bit, with what it warned of.
fn picture(dir: &Path, p: &J) -> (Vec<u8>, Vec<String>) {
    let mut project = read("fx_cd_001.json");
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
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b512_card_dance() {
    let mut t = Table::new(
        "card_dance",
        "# D-632: Card Dance\n\nB-512, after After Effects' Card Dance (Simulation), under the \
         coordinator's approval of the D-630 design for the owner's /loop request. The layer cut \
         into Rows and Columns of cards, each moved, turned and scaled by the red, green, blue, \
         intensity or alpha of one of two Gradient Layers (or the layer itself) under its centre, \
         times a Multiplier plus an Offset, in the Rotation Order and the Transformation Order; \
         the backs from a Back Layer; seen by the effect's own camera or the composition camera \
         and lit by Card Wipe's light. Adobe's help publishes no formula, so the rule, the units \
         and every setting when added are ours, stated in `tools/card_dance_reference.py`. Every \
         expected pixel is `Fixtures/card_dance/expected_card_dance.json`, written by the tool \
         before this code existed and printed in document 25 as FX-CD-001 to 064. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-CD-001 to 064 (document 25)");
    t.fixtures_numbered("expected_card_dance.json", 1..=64);

    t.heading("The settings");
    let plain = effect(&t, "fx_cd_001.json");
    t.row("Card Dance never grows the layer's bounds", &plain.bounds_expansion().to_string(), plain.bounds_expansion() == 0);
    let mut draft = plain.clone();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes no setting, each a share of the layer, a count or a word (the composition camera, worked out each frame, is halved)",
        &format!("{draft:?}"),
        draft == plain,
    );
    for (file, want) in [
        ("fx_cd_049.json", "rotation order"),
        ("fx_cd_050.json", "transformation order"),
        ("fx_cd_051.json", "x position source"),
        ("fx_cd_053.json", "rows"),
        ("fx_cd_054.json", "columns"),
        ("fx_cd_055.json", "z position multiplier"),
        ("fx_cd_056.json", "x scale offset"),
        ("fx_cd_057.json", "y rotation offset"),
        ("fx_cd_058.json", "camera z position"),
        ("fx_cd_059.json", "focal length"),
        ("fx_cd_060.json", "light type"),
        ("fx_cd_061.json", "light colour"),
        ("fx_cd_062.json", "gradient layer 2"),
        ("fx_cd_063.json", "rows & columns"),
        ("fx_cd_064.json", "x position offset"),
    ] {
        // A setting keyed out of range is refused when the file is read, by the warning.
        let mut why = effect(&t, file).why_invalid();
        if why.is_empty() {
            why = t.load(file).warnings.iter().map(|d| format!("{} {}", d.message, d.detail)).collect::<Vec<_>>().join(" ");
        }
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }

    t.heading("The file");
    let files: Vec<String> = (1..=64).map(|n| format!("fx_cd_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_cd_040.json");
    t.row(
        "fx_cd_040.json is saved with its forty-eight settings, the back `back`, a point light, X Position Source \"none\", Camera X, Y Position [50, 50]",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 48
            && saved["back_layer"] == "back"
            && saved["gradient_layer_1"] == ""
            && saved["light_type"] == "point"
            && saved["x_position_source"] == "none"
            && saved["camera_xy_position"] == json!([50, 50]),
    );
    t.row(
        "the three pictures the effect is drawn with, and the composition camera it is given, are never saved",
        &format!("{:?} {:?} {:?} {:?}", saved.get("back_map"), saved.get("gradient_map_1"), saved.get("gradient_map_2"), saved.get("view")),
        ["back_map", "gradient_map_1", "gradient_map_2", "view", "sources", "multipliers", "offsets"].iter().all(|k| saved.get(*k).is_none()),
    );
    let good = read("fx_cd_002.json")["compositions"][0]["layers"][4]["effects"][0]["parameters"].clone();
    let mut p = good.clone();
    p["rows"] = json!("many");
    t.shape_refused("fx_cd_002.json", "a Card Dance whose rows is a word", &p.to_string());
    let mut p = good.clone();
    p["camera_xy_position"] = json!(50);
    t.shape_refused("fx_cd_002.json", "a Card Dance whose camera X, Y Position is a number, not a list", &p.to_string());
    let mut p = good.clone();
    p.as_object_mut().unwrap().remove("x_scale_source");
    t.shape_refused("fx_cd_002.json", "a Card Dance with no X Scale Source", &p.to_string());
    let mut p = good;
    p.as_object_mut().unwrap().remove("gradient_layer_2");
    t.shape_refused("fx_cd_002.json", "a Card Dance with no gradient layer 2", &p.to_string());

    t.heading("Pictures (verification/D-632 pictures)");
    let dir = std::env::temp_dir().join("b512_pictures");
    fs::create_dir_all(dir.join("media")).unwrap();
    png_out::write_rgba(&dir.join("town.png"), TOWN.0, TOWN.1, OutputDepth::Eight, &[], &effect_table::town()).unwrap();
    for m in ["back", "ramp", "colours"] {
        fs::copy(repo(&format!("Fixtures/card_dance/media/{m}.png")), dir.join(format!("media/{m}.png"))).unwrap();
    }
    let out = repo("verification/D-632 pictures");
    fs::create_dir_all(&out).unwrap();
    let grid = json!({"rows": 9, "columns": 16});
    let with = |extra: J| {
        let mut p = grid.clone();
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        p
    };
    for (name, p) in [
        ("1_as_added", json!({})),
        ("2_tilted_cards_backs", with(json!({"z_position_source": "none", "back_layer": "back", "x_rotation_offset": 40, "y_rotation_offset": -25, "z_rotation_offset": 10}))),
        ("3_turned_by_the_ramp", with(json!({"gradient_layer_1": "ramp", "z_position_source": "none", "y_rotation_source": "intensity_1", "y_rotation_multiplier": 80, "back_layer": "back"}))),
        ("4_scattered_by_colours", with(json!({"gradient_layer_1": "colours", "x_position_source": "red_1", "y_position_source": "green_1", "z_position_source": "blue_1", "x_position_multiplier": 0.6, "y_position_multiplier": 0.6, "z_position_multiplier": 1.5, "x_scale_offset": 0.8, "y_scale_offset": 0.8}))),
        ("5_seen_from_the_side", with(json!({"gradient_layer_1": "ramp", "z_position_multiplier": 2, "camera_y_rotation": 35, "camera_z_position": 1.6, "back_layer": "back"}))),
        ("6_composition_camera", with(json!({"gradient_layer_1": "ramp", "z_position_multiplier": 1.5, "camera_system": "comp_camera"}))),
        ("7_point_light_shine", with(json!({"z_position_source": "none", "x_rotation_offset": 30, "y_rotation_offset": -20, "light_type": "point", "light_position": [30, 30], "light_depth": 0.5, "specular_reflection": 1, "highlight_sharpness": 20}))),
    ] {
        let (bytes, said) = picture(&dir, &p);
        png_out::write_rgba(&out.join(format!("{name}.png")), TOWN.0, TOWN.1, OutputDepth::Eight, &[], &bytes).unwrap();
        let mut colours: Vec<[u8; 3]> = bytes.chunks_exact(4).map(|p| [p[0], p[1], p[2]]).collect();
        colours.sort();
        colours.dedup();
        let clear = bytes.chunks_exact(4).filter(|p| p[3] == 0).count();
        t.row(
            &format!("{name}.png: drawn without a warning, {} distinct colours", colours.len()),
            &format!("{} colours; {clear} clear pixels between and around the cards; warnings {said:?}", colours.len()),
            said.is_empty() && colours.len() > 100,
        );
    }
    t.finish("D-632_card_dance_table.md");
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

/// The frame times: the reference shot (1920 by 1080, 24 a second) with Card Dance on its first
/// layer, frames 0 and 120 each drawn on the processor with empty caches, three times; the median
/// of the three.
#[test]
#[ignore = "B-512: a measurement, run deliberately with --release --ignored"]
fn b512_card_dance_timing() {
    use std::fmt::Write as _;

    let mut s = format!(
        "- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: the processor (Card Dance has no card path)\n\n\
         | Shot | Frame 0 | Frame 120 |\n|---|---:|---:|\n",
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
    let base = read("fx_cd_001.json")["compositions"][0]["layers"][4]["effects"][0].clone();
    let with = |extra: J| {
        let mut e = base.clone();
        for (k, v) in extra.as_object().unwrap() {
            e["parameters"][k] = v.clone();
        }
        json!([e])
    };
    for (name, stack) in [
        ("no effect", json!([])),
        ("Card Dance as added (8 by 8 cards)", with(json!({}))),
        ("100 by 100 cards, each tilted", with(json!({"rows": 100, "columns": 100, "x_rotation_offset": 40, "y_rotation_offset": -25}))),
        ("1000 by 1000 cards (the ceiling)", with(json!({"rows": 1000, "columns": 1000}))),
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
    let out = std::env::var("B512_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-512_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
