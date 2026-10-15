//! B-411: D-531, after CycoreFX's CC Environment (Perspective): an environment picture (a
//! Spherical Map, a Light Probe or a Vertical Cross) seen round a fixed eye, filling the layer's
//! shape, in its own fake 3D. The CycoreFX manual names the controls but publishes no formula and
//! no settings when added: the rule and those settings are ours, stated in
//! `tools/cc_environment_reference.py`. The composition camera is not used; the effect has its
//! own Tilt, Roll and Field of View beside Horizontal Pan.
//!
//! Writes `verification/D-531_cc_environment_table.md` and draws pictures into
//! `verification/D-531 pictures/`.
//!
//! Every expected pixel is `Fixtures/cc_environment/expected_cc_environment.json`, written by
//! the tool before this code existed and printed in document 25 as FX-ENV-001 to 030.
//! Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, distance, median, on_card, reference, repo, time_passes, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or have nothing to read.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=30 {
        let file = format!("fx_env_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::CcEnvironment { .. }));
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, its background `layer-4`
/// the environment, frames 0, 100 and 239 at Full and Draft, on the card against the processor;
/// `want` of 3 on the card each frame.
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
        let project = reference(|id| json!([fx(&format!("b411-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, CC Environment {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::CcEnvironment { .. }));
                t.row(
                    &format!("the reference shot, CC Environment {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// FX-ENV-002's project at the street's size in `dir`, the street the holder and the
/// spherical map, the fixture's probe and cross the others, the holder's effect set to `p`;
/// drawn on the processor, straight 8-bit, with what it warned of.
fn picture(dir: &Path, p: &J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/cc_environment/fx_env_002.json")).unwrap()).unwrap();
    for (asset, path) in [("asset-shot", "street.png"), ("asset-sphere", "street.png"), ("asset-probe", "probe.png"), ("asset-cross", "cross.png")] {
        let a = project["assets"].as_array_mut().unwrap().iter_mut().find(|a| a["id"] == asset).unwrap();
        a["path"] = J::from(path);
    }
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let holder = comp["layers"].as_array_mut().unwrap().iter_mut().find(|l| l["id"] == "holder").unwrap();
    holder["effects"] = json!([fx("pic", p)]);
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// The `holder` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

/// The pixel at (x, y) of a picture at the street's size.
fn at(bytes: &[u8], p: (usize, usize)) -> [u8; 4] {
    let i = (p.1 * TOWN.0 + p.0) * 4;
    [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
}

/// A CC Environment with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"environment": "", "mapping": "spherical", "horizontal_pan": 0, "tilt": 0, "roll": 0,
        "field_of_view": 40, "filter_environment": "on"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.cc_environment", "enabled": true, "parameters": all})
}

#[test]
fn b411_cc_environment() {
    let mut t = Table::new(
        "cc_environment",
        "# D-531: CC Environment\n\nB-411, after CycoreFX's CC Environment (Perspective): an \
         environment picture seen round a fixed eye, filling the layer's shape, in its own fake 3D. \
         The CycoreFX manual names the controls but publishes no formula and no settings when \
         added, so the rule and those settings are ours, stated in \
         `tools/cc_environment_reference.py`: the ray through each pixel, turned by Roll, Tilt and \
         Horizontal Pan, reads a Spherical Map (latitude and longitude), a Light Probe (angular) or \
         a Vertical Cross; Filter Environment averages four rays a pixel on a Spherical Map; the \
         colour is laid on the layer's alpha. The composition camera is not used. Every expected \
         pixel is `Fixtures/cc_environment/expected_cc_environment.json`, written by the tool \
         before this code existed and printed in document 25 as FX-ENV-001 to 030. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-ENV-001 to 030 (document 25)");
    t.fixtures_numbered("expected_cc_environment.json", 1..=30);

    t.heading("The settings");
    let plain = changed(&t, "fx_env_002.json", |_| {});
    let mut draft = plain.clone();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview leaves every setting as it is, since none is a distance",
        &format!("{draft:?}"),
        draft == plain,
    );

    t.heading("The file");
    let files: Vec<String> = (1..=30).map(|n| format!("fx_env_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_env_013.json");
    t.row(
        "fx_env_013.json is saved with its seven settings, the cross layer, mapping vertical_cross, field of view 90",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 7
            && saved["environment"] == "cross"
            && saved["mapping"] == "vertical_cross"
            && saved["field_of_view"] == json!(90),
    );
    let params = t.saved_parameters("fx_env_029.json");
    t.row(
        "fx_env_029.json, its environment written as the number 3, is saved as the number 3",
        &params["environment"].to_string(),
        params["environment"] == json!(3),
    );
    let params = t.saved_parameters("fx_env_002.json");
    t.row("the environment picture the effect reads is never saved", &format!("{:?}", params.get("map")), params.get("map").is_none());
    for (file, want) in [
        ("fx_env_022.json", "mapping"),
        ("fx_env_023.json", "filter environment"),
        ("fx_env_024.json", "horizontal pan"),
        ("fx_env_025.json", "tilt"),
        ("fx_env_026.json", "roll"),
        ("fx_env_027.json", "field of view"),
        ("fx_env_028.json", "field of view"),
        ("fx_env_029.json", "environment"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    for (what, parameters) in [
        ("a CC Environment whose field of view is a word", r#"{"environment": "sphere", "mapping": "spherical", "horizontal_pan": 0, "tilt": 0, "roll": 0, "field_of_view": "wide", "filter_environment": "on"}"#),
        ("a CC Environment with no mapping", r#"{"environment": "sphere", "horizontal_pan": 0, "tilt": 0, "roll": 0, "field_of_view": 40, "filter_environment": "on"}"#),
        ("a CC Environment with no environment", r#"{"mapping": "spherical", "horizontal_pan": 0, "tilt": 0, "roll": 0, "field_of_view": 40, "filter_environment": "on"}"#),
    ] {
        t.shape_refused("fx_env_002.json", what, parameters);
    }

    t.heading("On the card (D-531)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[1, 19, 22, 23, 24, 25, 26, 27, 28, 29, 30]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("spherical, the background layer-4 the map, field of view 90", json!({"environment": "layer-4", "field_of_view": 90}), 3),
            ("probe, turned and tilted, unfiltered", json!({"environment": "layer-4", "mapping": "probe", "horizontal_pan": 30, "tilt": -15, "roll": 10, "field_of_view": 60, "filter_environment": "off"}), 3),
            ("vertical cross, looking up and to the right", json!({"environment": "layer-4", "mapping": "vertical_cross", "horizontal_pan": 40, "tilt": 30, "field_of_view": 100}), 3),
            ("Horizontal Pan keyed one whole turn", json!({"environment": "layer-4", "horizontal_pan": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 360, "interp": "linear"}]}}), 3),
        ],
    );

    t.heading("Pictures (verification/D-531 pictures)");
    let dir = std::env::temp_dir().join("b411_pictures");
    fs::create_dir_all(&dir).unwrap();
    fs::copy(repo("verification/D-506 pictures/1_left.png"), dir.join("street.png")).unwrap();
    for f in ["probe.png", "cross.png"] {
        fs::copy(repo("Fixtures/cc_environment/media").join(f), dir.join(f)).unwrap();
    }
    let out = repo("verification/D-531 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(&dir, &json!({})).0;
    for (name, p) in [
        ("1_street_all_round", json!({"environment": "sphere", "field_of_view": 90})),
        ("2_turned_behind", json!({"environment": "sphere", "field_of_view": 90, "horizontal_pan": 180})),
        ("3_looking_up", json!({"environment": "sphere", "field_of_view": 90, "tilt": 45})),
        ("4_wide_looking_down", json!({"environment": "sphere", "field_of_view": 150, "tilt": -30, "roll": -20})),
        ("5_cross_corner", json!({"environment": "cross", "mapping": "vertical_cross", "field_of_view": 90, "horizontal_pan": 45})),
        ("6_probe", json!({"environment": "probe", "mapping": "probe", "field_of_view": 120})),
    ] {
        let (bytes, said) = picture(&dir, &p);
        png_out::write_rgba(&out.join(format!("{name}.png")), TOWN.0, TOWN.1, OutputDepth::Eight, &[], &bytes).unwrap();
        let middle = at(&bytes, (240, 135));
        t.row(
            &format!("{name}.png: drawn without a warning, the street changed, the middle drawn"),
            &format!("{} pixels changed; middle {middle:?}; warnings {said:?}", distance(&bytes, &street).1),
            said.is_empty() && distance(&bytes, &street).1 > 0 && middle[3] > 0,
        );
    }
    t.finish("D-531_cc_environment_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect
/// with the background `layer-4` its environment, every eighth frame asked for as the viewer
/// asks, whole. The first loop starts with empty caches and its 30 frames' median is "first";
/// the median of the loops after it is "again". With `B411_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-411: a measurement, run deliberately with --release --ignored"]
fn b411_cc_environment_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B411_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone", None),
        ("Noise, then CC Environment, spherical, filtered", Some(json!({"environment": "layer-4", "field_of_view": 90}))),
        ("Noise, then CC Environment, probe", Some(json!({"environment": "layer-4", "mapping": "probe", "field_of_view": 90}))),
        ("Noise, then CC Environment, vertical cross", Some(json!({"environment": "layer-4", "mapping": "vertical_cross", "field_of_view": 90}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}u"), p));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
        let (first, times) = time_passes(&mut gpu, cpu, passes, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B411_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-411_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
