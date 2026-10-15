//! B-510: D-630, after After Effects' Wave World (Simulation): water on a grid, rippled by up to
//! two producers and shaped by a ground layer, drawn in place of the layer as a grey height map.
//! Adobe's help names the controls and says what each does but publishes no formula: the rule,
//! the units and the settings when added are ours, stated in `tools/wave_world_reference.py`.
//!
//! Writes `verification/D-630_wave_world_table.md` and draws pictures into
//! `verification/D-630 pictures/`.
//!
//! Every expected pixel is `Fixtures/wave_world/expected_wave_world.json`, written by the tool
//! before this code existed and printed in document 25 as FX-WW-001 to 048. Tolerance 2e-5.
//! Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{median, repo, Table, MAIN, TOWN};
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
    d.project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layer(&Id::new("holder"))
        .unwrap()
        .effects[0]
        .effect
        .clone()
}

fn read(file: &str) -> J {
    serde_json::from_str(&fs::read_to_string(repo(&format!("Fixtures/wave_world/{file}"))).unwrap())
        .unwrap()
}

/// The largest difference between two frames.
fn apart(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b)
        .map(|(x, y)| (x - y).abs())
        .fold(0.0, f32::max)
}

/// FX-WW-018's project at the street's size in `dir`, the logo the ground, the holder's Wave
/// World settings changed by `p`; frame `frame` drawn on the processor, straight 8-bit, with
/// what it warned of.
fn picture(dir: &Path, p: &J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project = read("fx_ww_018.json");
    for (asset, path) in [("asset-shot", "town.png"), ("asset-logo", "logo.png")] {
        let a = project["assets"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|a| a["id"] == asset)
            .unwrap();
        a["path"] = J::from(path);
    }
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    for l in comp["layers"].as_array_mut().unwrap() {
        if l["id"] == "logo" {
            l["transform"]["scale"]["base"] = json!([1500, 1500]);
        }
    }
    let holder = comp["layers"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|l| l["id"] == "holder")
        .unwrap();
    let parameters = &mut holder["effects"][0]["parameters"];
    for (k, v) in p.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(
        loaded.document.project(),
        &Id::new(MAIN),
        frame,
        dir,
        64,
        &mut log,
    )
    .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b510_wave_world() {
    let mut t = Table::new(
        "wave_world",
        "# D-630: Wave World\n\nB-510, after After Effects' Wave World (Simulation), under the \
         coordinator's approval of the D-630 design for the owner's /loop request. Water on a \
         grid of Grid Resolution cells across, rippled by up to two producers (a ring or a line, \
         each beating at its Frequency) and shaped by a ground layer, whose brightness raises the \
         floor by Steepness under water of Height; Wave Speed, Damping and Reflect Edges set how \
         the waves cross, die and meet the edge. The water runs from the layer's in point (after \
         the Pre-roll) to the frame drawn, through every frame's settings, and is drawn in place \
         of the layer as a grey height map by Brightness, Contrast and Gamma, dry land level or \
         clear. Adobe's help publishes no formula, so the rule, the units and the settings when \
         added are ours, stated in `tools/wave_world_reference.py`. Every expected pixel is \
         `Fixtures/wave_world/expected_wave_world.json`, written by the tool before this code \
         existed and printed in document 25 as FX-WW-001 to 048. Tolerance 2e-5.\n",
    );

    t.heading("FX-WW-001 to 048 (document 25)");
    t.fixtures_numbered("expected_wave_world.json", 1..=48);

    t.heading("The history");
    let document = t.load("fx_ww_002.json").document;
    let cold = t.render(&document, 40, 64);
    for f in [0, 12, 24, 39] {
        t.render(&document, f, 64);
    }
    let warm = t.render(&document, 40, 64);
    let d = apart(cold.data(), warm.data());
    t.row(
        "FX-WW-002's frame 40 drawn first, then again after frames 0, 12, 24 and 39: the same, since every frame runs the water from the in point",
        &format!("largest difference {d:e}"),
        d == 0.0,
    );
    let mut later = read("fx_ww_002.json");
    later["compositions"][0]["layers"][2]["source_offset_frames"] = J::from(5);
    let moved = persist::load_str(&later.to_string()).unwrap().document;
    let d = apart(
        t.render(&document, 12, 64).data(),
        t.render(&moved, 12, 64).data(),
    );
    t.row(
        "FX-WW-002 with the holder's source 5 frames on: frame 12 is the same, since the water runs by the composition's frames from the in point",
        &format!("largest difference {d:e}"),
        d == 0.0,
    );

    t.heading("The settings");
    let plain = effect(&t, "fx_ww_002.json");
    t.row(
        "Wave World never grows the layer's bounds",
        &plain.bounds_expansion().to_string(),
        plain.bounds_expansion() == 0,
    );
    let mut draft = plain.clone();
    draft.scale_distances(|d| d * 0.5);
    let mut want = plain.clone();
    if let Effect::WaveWorld { grid_scale, .. } = &mut want {
        *grid_scale = 0.5;
    }
    t.row(
        "a half-size draft preview changes no setting, each a share of the layer, a count or a time; it halves only the grid when Grid Res Downsamples is on",
        &format!("{draft:?}"),
        draft == want,
    );
    for (file, want) in [
        ("fx_ww_035.json", "grid resolution"),
        ("fx_ww_036.json", "grid resolution"),
        ("fx_ww_037.json", "wave speed"),
        ("fx_ww_038.json", "damping"),
        ("fx_ww_039.json", "pre roll"),
        ("fx_ww_040.json", "gamma"),
        ("fx_ww_041.json", "transparency"),
        ("fx_ww_042.json", "reflect edges"),
        ("fx_ww_043.json", "render dry areas"),
        ("fx_ww_044.json", "grid res downsamples"),
        ("fx_ww_045.json", "producer 1 type"),
        ("fx_ww_046.json", "ground layer"),
        ("fx_ww_048.json", "producer 2 position"),
    ] {
        let why = effect(&t, file).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming {want}"),
            &why,
            why.to_lowercase().contains(want),
        );
    }

    t.heading("The file");
    let files: Vec<String> = (1..=48).map(|n| format!("fx_ww_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_ww_018.json");
    t.row(
        "fx_ww_018.json is saved with its thirty-one settings, the ground layer `logo`, Producer 1 Position [20, 30]",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 31 && saved["ground_layer"] == "logo" && saved["producer_1_position"] == json!([20, 30]),
    );
    t.row(
        "the ground picture, the history, the frame rate and the draft's grid scale the effect is drawn with are never saved",
        &format!("{:?} {:?} {:?} {:?}", saved.get("ground_map"), saved.get("history"), saved.get("fps"), saved.get("grid_scale")),
        ["ground_map", "history", "fps", "grid_scale"].iter().all(|k| saved.get(*k).is_none()),
    );
    let one = t.saved_parameters("fx_ww_048.json");
    t.row(
        "fx_ww_048.json, its Producer 2 Position one number in a list, is saved as it was written",
        &one["producer_2_position"].to_string(),
        one["producer_2_position"] == json!([10]),
    );
    let good =
        read("fx_ww_002.json")["compositions"][0]["layers"][2]["effects"][0]["parameters"].clone();
    let mut p = good.clone();
    p["grid_resolution"] = json!("many");
    t.shape_refused(
        "fx_ww_002.json",
        "a Wave World whose grid resolution is a word",
        &p.to_string(),
    );
    let mut p = good.clone();
    p["producer_1_position"] = json!(50);
    t.shape_refused(
        "fx_ww_002.json",
        "a Wave World whose Producer 1 Position is a number, not a list",
        &p.to_string(),
    );
    let mut p = good.clone();
    p.as_object_mut().unwrap().remove("reflect_edges");
    t.shape_refused(
        "fx_ww_002.json",
        "a Wave World with no reflect edges",
        &p.to_string(),
    );
    let mut p = good;
    p.as_object_mut().unwrap().remove("ground_layer");
    t.shape_refused(
        "fx_ww_002.json",
        "a Wave World with no ground layer",
        &p.to_string(),
    );

    t.heading("Pictures (verification/D-630 pictures)");
    let dir = std::env::temp_dir().join("b510_pictures");
    fs::create_dir_all(&dir).unwrap();
    png_out::write_rgba(
        &dir.join("town.png"),
        TOWN.0,
        TOWN.1,
        OutputDepth::Eight,
        &[],
        &effect_table::town(),
    )
    .unwrap();
    fs::copy(
        repo("Fixtures/wave_world/media/logo.png"),
        dir.join("logo.png"),
    )
    .unwrap();
    let out = repo("verification/D-630 pictures");
    fs::create_dir_all(&out).unwrap();
    // The settings as added (open water, a ring a tenth wide in the middle beating once a
    // second), on a 60-cell grid; each picture changes some.
    let open_water = json!({"ground_layer": "", "grid_resolution": 60, "contrast": 0.5, "steepness": 0.25,
        "producer_1_position": [50, 50], "producer_1_height": 0.1, "producer_1_width": 0.1, "producer_1_frequency": 1});
    let with = |extra: J| {
        let mut p = open_water.clone();
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        p
    };
    let island = |extra: J| {
        let mut p = with(
            json!({"ground_layer": "logo", "steepness": 0.6, "contrast": 3, "wave_speed": 0.5, "producer_1_position": [25, 50], "producer_1_amplitude": 1.5,
            "producer_1_height": 0.06, "producer_1_width": 0.06, "producer_1_frequency": 2}),
        );
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        p
    };
    for (name, p, frame) in [
        ("1_as_added_ring", with(json!({})), 36),
        (
            "2_two_producers_crossing",
            with(
                json!({"contrast": 2, "producer_1_position": [30, 50], "producer_2_position": [70, 50], "producer_2_amplitude": 0.5, "producer_2_frequency": 2.5}),
            ),
            36,
        ),
        (
            "3_line_reflect_edges",
            with(
                json!({"contrast": 2, "producer_1_type": "line", "producer_1_height": 0.5, "producer_1_width": 0.04, "producer_1_angle": 30, "producer_1_frequency": 2, "reflect_edges": "all"}),
            ),
            48,
        ),
        ("4_shore_round_an_island", island(json!({})), 48),
        (
            "5_island_clear",
            island(json!({"render_dry_areas": "transparent"})),
            48,
        ),
        (
            "6_shallows_fading",
            island(json!({"steepness": 0.3, "transparency": 0.8})),
            48,
        ),
    ] {
        let (bytes, said) = picture(&dir, &p, frame);
        png_out::write_rgba(
            &out.join(format!("{name}.png")),
            TOWN.0,
            TOWN.1,
            OutputDepth::Eight,
            &[],
            &bytes,
        )
        .unwrap();
        let mut greys: Vec<u8> = bytes
            .chunks_exact(4)
            .filter(|p| p[3] > 0)
            .map(|p| p[0])
            .collect();
        greys.sort();
        greys.dedup();
        let clear = bytes.chunks_exact(4).filter(|p| p[3] == 0).count();
        let shape = match name {
            "5_island_clear" => clear > 0,
            _ => clear == 0,
        };
        t.row(
            &format!("{name}.png, frame {frame}: drawn without a warning, {} distinct greys (ripples, not flat)", greys.len()),
            &format!("{} greys; {clear} clear pixels; warnings {said:?}", greys.len()),
            said.is_empty() && greys.len() > 8 && shape,
        );
    }
    t.finish("D-630_wave_world_table.md");
}

/// The reference shot (1920 by 1080) with `stack` on its first layer.
fn reference(stack: J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json"))
        .expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    j["compositions"][0]["layers"][0]["effects"] = stack;
    persist::load_str(&j.to_string())
        .unwrap_or_else(|d| panic!("the reference shot: {}", d.message))
        .document
        .project()
        .clone()
}

/// The frame times: the reference shot (1920 by 1080, 24 a second) with Wave World on its first
/// layer, frames 0, 60, 120 and 239 each drawn on the processor with empty caches, three times;
/// the median of the three. A later frame runs more history, so it costs more.
#[test]
#[ignore = "B-510: a measurement, run deliberately with --release --ignored"]
fn b510_wave_world_timing() {
    use std::fmt::Write as _;

    let mut s = format!(
        "- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: the processor (Wave World has no card path)\n\n\
         | Shot | Frame 0 | Frame 60 | Frame 120 | Frame 239 |\n|---|---:|---:|---:|---:|\n",
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let (comp, root) = (
        Id::new("comp-reference-shot"),
        repo("Fixtures/reference_shot"),
    );
    for (name, grid) in [
        ("no effect", None),
        ("Wave World, Grid 60 (as added)", Some(60)),
        ("Wave World, Grid 145", Some(145)),
    ] {
        let stack = match grid {
            None => json!([]),
            Some(g) => {
                let mut p =
                    read("fx_ww_001.json")["compositions"][0]["layers"][2]["effects"][0].clone();
                p["parameters"]["grid_resolution"] = J::from(g);
                json!([p])
            }
        };
        let project = reference(stack);
        let _ = write!(s, "| {name} |");
        for frame in [0, 60, 120, 239] {
            let times: Vec<f64> = (0..3)
                .map(|_| {
                    let mut log = FrameLog::new(3);
                    let t = std::time::Instant::now();
                    drop(
                        preview::preview_frame_cached(
                            &project,
                            &comp,
                            frame,
                            &root,
                            PreviewQuality::Full,
                            DEFAULT_TILE_SIZE,
                            &mut log,
                            &mut CelCache::viewer(),
                        )
                        .expect("CPU frame"),
                    );
                    t.elapsed().as_secs_f64() * 1000.0
                })
                .collect();
            let _ = write!(s, " {:.1} ms |", median(times));
        }
        s.push('\n');
    }
    let out = std::env::var("B510_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-510_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
