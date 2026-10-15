//! B-511: D-631, after After Effects' Caustics (Simulation): a bottom seen through a water
//! surface, bent by the waves, brightened and darkened where the waves gather and spread the
//! light, under a coloured, lit surface that can mirror a sky. Adobe's help names the controls and
//! says what several do but publishes no formula: the rule, the units and most settings when
//! added are ours, stated in `tools/caustics_reference.py`.
//!
//! Writes `verification/D-631_caustics_table.md` and draws pictures into
//! `verification/D-631 pictures/`.
//!
//! Every expected pixel is `Fixtures/caustics/expected_caustics.json`, written by the tool before
//! this code existed and printed in document 25 as FX-CA-001 to 038. Tolerance 2e-5. Nothing here
//! is a snapshot of a run.

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
    serde_json::from_str(&fs::read_to_string(repo(&format!("Fixtures/caustics/{file}"))).unwrap())
        .unwrap()
}

/// FX-CA-002's project at the street's size in `dir` (the street the holder's drawing, the
/// ripples redrawn at that size, the fixtures' floor and sky as they are), the holder's Caustics settings changed by `p`;
/// frame 0 drawn on the processor, straight 8-bit, with what it warned of.
fn picture(dir: &Path, p: &J) -> (Vec<u8>, Vec<String>) {
    let mut project = read("fx_ca_002.json");
    let a = project["assets"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|a| a["id"] == "asset-shot")
        .unwrap();
    a["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
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
        0,
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
fn b511_caustics() {
    let mut t = Table::new(
        "caustics",
        "# D-631: Caustics\n\nB-511, after After Effects' Caustics (Simulation), under the \
         coordinator's approval of the D-630 design for the owner's /loop request. A bottom (the \
         layer's own drawing, or a chosen layer placed by Scaling, Repeat Mode and If Layer Size \
         Differs, softened by Bottom Blur) seen through water whose waves come from a water \
         surface layer's brightness, by Wave Height and Smoothing, bent by Water Depth and the \
         Refractive Index, brightened and darkened by Caustics Strength where the waves gather and \
         spread the light, under a Surface Color at Surface Opacity that can mirror a placed sky \
         and is lit by Card Wipe's light. Adobe's help publishes no formula, so the rule, the units \
         and most settings when added are ours, stated in `tools/caustics_reference.py`. Every \
         expected pixel is `Fixtures/caustics/expected_caustics.json`, written by the tool before \
         this code existed and printed in document 25 as FX-CA-001 to 038. Tolerance 2e-5.\n",
    );

    t.heading("FX-CA-001 to 038 (document 25)");
    t.fixtures_numbered("expected_caustics.json", 1..=38);

    t.heading("The settings");
    let plain = effect(&t, "fx_ca_002.json");
    t.row(
        "Caustics never grows the layer's bounds",
        &plain.bounds_expansion().to_string(),
        plain.bounds_expansion() == 0,
    );
    let mut draft = plain.clone();
    draft.scale_distances(|d| d * 0.5);
    t.row("a half-size draft preview changes no setting, each a share of the layer, a count or a word", &format!("{draft:?}"), draft == plain);
    for (file, want) in [
        ("fx_ca_029.json", "refractive index"),
        ("fx_ca_030.json", "bottom scaling"),
        ("fx_ca_031.json", "bottom blur"),
        ("fx_ca_032.json", "bottom repeat mode"),
        ("fx_ca_033.json", "sky fit"),
        ("fx_ca_034.json", "surface colour"),
        ("fx_ca_035.json", "light type"),
        ("fx_ca_036.json", "water surface"),
        ("fx_ca_037.json", "light position"),
    ] {
        let why = effect(&t, file).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming {want}"),
            &why,
            why.to_lowercase().contains(want),
        );
    }

    t.heading("The file");
    let files: Vec<String> = (1..=38).map(|n| format!("fx_ca_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_ca_028.json");
    t.row(
        "fx_ca_028.json is saved with its twenty-eight settings, the water surface `ripples`, the sky `sky`, Light Position [15, 20]",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 28
            && saved["water_surface"] == "ripples"
            && saved["sky_layer"] == "sky"
            && saved["bottom_layer"] == ""
            && saved["light_position"] == json!([15, 20]),
    );
    t.row(
        "the three pictures the effect is drawn with are never saved",
        &format!(
            "{:?} {:?} {:?}",
            saved.get("bottom_map"),
            saved.get("surface_map"),
            saved.get("sky_map")
        ),
        ["bottom_map", "surface_map", "sky_map"]
            .iter()
            .all(|k| saved.get(*k).is_none()),
    );
    let one = t.saved_parameters("fx_ca_037.json");
    t.row(
        "fx_ca_037.json, its Light Position one number in a list, is saved as it was written",
        &one["light_position"].to_string(),
        one["light_position"] == json!([10]),
    );
    let good =
        read("fx_ca_002.json")["compositions"][0]["layers"][4]["effects"][0]["parameters"].clone();
    let mut p = good.clone();
    p["wave_height"] = json!("tall");
    t.shape_refused(
        "fx_ca_002.json",
        "a Caustics whose wave height is a word",
        &p.to_string(),
    );
    let mut p = good.clone();
    p["light_position"] = json!(50);
    t.shape_refused(
        "fx_ca_002.json",
        "a Caustics whose Light Position is a number, not a list",
        &p.to_string(),
    );
    let mut p = good.clone();
    p.as_object_mut().unwrap().remove("bottom_fit");
    t.shape_refused(
        "fx_ca_002.json",
        "a Caustics with no bottom fit",
        &p.to_string(),
    );
    let mut p = good;
    p.as_object_mut().unwrap().remove("sky_layer");
    t.shape_refused(
        "fx_ca_002.json",
        "a Caustics with no sky layer",
        &p.to_string(),
    );

    t.heading("Pictures (verification/D-631 pictures)");
    let dir = std::env::temp_dir().join("b511_pictures");
    fs::create_dir_all(dir.join("media")).unwrap();
    png_out::write_rgba(
        &dir.join("town.png"),
        TOWN.0,
        TOWN.1,
        OutputDepth::Eight,
        &[],
        &effect_table::town(),
    )
    .unwrap();
    for m in ["floor", "sky"] {
        fs::copy(
            repo(&format!("Fixtures/caustics/media/{m}.png")),
            dir.join(format!("media/{m}.png")),
        )
        .unwrap();
    }
    // The fixtures' ripples drawn again at the street's size, so the water is smooth and not
    // the 48 by 27 picture stretched.
    let tau = std::f64::consts::TAU;
    let k = TOWN.0 as f64 / 48.0;
    let ripples: Vec<u8> = (0..TOWN.0 * TOWN.1)
        .flat_map(|i| {
            let (x, y) = ((i % TOWN.0) as f64 / k, (i / TOWN.0) as f64 / k);
            let v = 0.5
                + 0.3 * (tau * (x + 0.5) / 16.0).sin() * (tau * (y + 0.5) / 13.0).cos()
                + 0.12 * (tau * (x + y + 1.0) / 9.0).sin();
            let g = (255.0 * v.clamp(0.0, 1.0)).round() as u8;
            [g, g, g, 255]
        })
        .collect();
    png_out::write_rgba(
        &dir.join("media/ripples.png"),
        TOWN.0,
        TOWN.1,
        OutputDepth::Eight,
        &[],
        &ripples,
    )
    .unwrap();
    let out = repo("verification/D-631 pictures");
    fs::create_dir_all(&out).unwrap();
    // The settings as added, with the ripples as the water surface; each picture changes some.
    let added = json!({"water_surface": "ripples", "wave_height": 0.2, "water_depth": 0.1});
    let with = |extra: J| {
        let mut p = added.clone();
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        p
    };
    let spot = json!({"light_type": "point", "light_position": [25, 30], "light_height": 0.8, "specular_reflection": 1, "highlight_sharpness": 20});
    let mut lit = with(json!({"wave_height": 0.5, "water_depth": 0.2, "surface_opacity": 0.5}));
    for (k, v) in spot.as_object().unwrap() {
        lit[k] = v.clone();
    }
    let mut all = lit.clone();
    for (k, v) in json!({"caustics_strength": 0.7, "sky_layer": "sky", "sky_intensity": 0.6, "bottom_blur": 4}).as_object().unwrap() {
        all[k] = v.clone();
    }
    for (name, p) in [
        ("1_as_added_with_water", with(json!({}))),
        (
            "2_deeper_waves",
            with(json!({"wave_height": 0.5, "water_depth": 0.2, "surface_opacity": 0.1})),
        ),
        (
            "3_caustics",
            with(
                json!({"wave_height": 0.5, "water_depth": 0.2, "surface_opacity": 0.1, "caustics_strength": 1}),
            ),
        ),
        (
            "4_floor_tiled",
            with(
                json!({"wave_height": 0.5, "water_depth": 0.2, "bottom_layer": "floor", "bottom_fit": "center", "bottom_scaling": 4, "surface_opacity": 0.1}),
            ),
        ),
        (
            "5_sky_mirrored",
            with(
                json!({"wave_height": 0.5, "water_depth": 0.2, "sky_layer": "sky", "sky_intensity": 1, "surface_opacity": 0.7}),
            ),
        ),
        ("6_point_light_highlight", lit),
        ("7_everything", all),
    ] {
        let (bytes, said) = picture(&dir, &p);
        png_out::write_rgba(
            &out.join(format!("{name}.png")),
            TOWN.0,
            TOWN.1,
            OutputDepth::Eight,
            &[],
            &bytes,
        )
        .unwrap();
        let mut colours: Vec<[u8; 3]> = bytes.chunks_exact(4).map(|p| [p[0], p[1], p[2]]).collect();
        colours.sort();
        colours.dedup();
        let clear = bytes.chunks_exact(4).filter(|p| p[3] == 0).count();
        t.row(
            &format!(
                "{name}.png: drawn without a warning, every pixel covered, {} distinct colours",
                colours.len()
            ),
            &format!(
                "{} colours; {clear} clear pixels; warnings {said:?}",
                colours.len()
            ),
            said.is_empty() && colours.len() > 100 && clear == 0,
        );
    }
    t.finish("D-631_caustics_table.md");
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

/// The frame times: the reference shot (1920 by 1080, 24 a second) with Caustics on its first
/// layer, frames 0 and 120 each drawn on the processor with empty caches, three times; the median
/// of the three.
#[test]
#[ignore = "B-511: a measurement, run deliberately with --release --ignored"]
fn b511_caustics_timing() {
    use std::fmt::Write as _;

    let mut s = format!(
        "- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: the processor (Caustics has no card path)\n\n\
         | Shot | Frame 0 | Frame 120 |\n|---|---:|---:|\n",
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let (comp, root) = (
        Id::new("comp-reference-shot"),
        repo("Fixtures/reference_shot"),
    );
    let base = read("fx_ca_001.json")["compositions"][0]["layers"][4]["effects"][0].clone();
    let with = |extra: J| {
        let mut e = base.clone();
        for (k, v) in extra.as_object().unwrap() {
            e["parameters"][k] = v.clone();
        }
        json!([e])
    };
    for (name, stack) in [
        ("no effect", json!([])),
        ("Caustics as added (no water surface)", with(json!({}))),
        (
            "Caustics, the layer its own water surface, caustics 1",
            with(json!({"water_surface": "layer-1", "caustics_strength": 1})),
        ),
        (
            "the same with Bottom Blur 50 (the ceiling)",
            with(json!({"water_surface": "layer-1", "caustics_strength": 1, "bottom_blur": 50})),
        ),
    ] {
        let project = reference(stack);
        let _ = write!(s, "| {name} |");
        for frame in [0, 120] {
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
    let out = std::env::var("B511_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-511_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
