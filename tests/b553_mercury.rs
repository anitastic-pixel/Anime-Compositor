//! B-553: D-674, Mercury, after CycoreFX's CC Mr. Mercury (Simulation): the layer turned into
//! drops of liquid metal from the in point, born at a producer, thrown and falling, running
//! together as metaballs and lit by Drizzle's light. CycoreFX publishes no formula and no
//! settings when added: the rule and those settings are ours, stated in
//! `tools/mercury_reference.py`.
//!
//! Writes `verification/D-674_mercury_table.md` and draws pictures into `verification/D-674
//! pictures/`.
//!
//! Every expected pixel is `Fixtures/mercury/expected_mercury.json`, written by the tool before
//! this code existed and printed in document 25 as FX-MERCURY-001 to 038. Tolerance 2e-5. Nothing
//! here is a snapshot of a run.

mod effect_table;

use std::fs;

use effect_table::{both, distance, median, on_card, reference, repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, MERCURY_SETTINGS, MERCURY_WORDS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// Every fixture file on the card against the processor, at each frame its case draws, at Full
/// and Draft. The files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_mercury.json")).unwrap()).unwrap();
    for (name, case) in expected["cases"].as_object().unwrap() {
        let n: u32 = name.rsplit('-').next().unwrap().parse().unwrap();
        let file = case["project"].as_str().unwrap();
        let frames: Vec<i32> = case["frames"].as_object().unwrap().keys().map(|f| f.parse().unwrap()).collect();
        let project = persist::load(&t.root.join(file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for &frame in &frames {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::Mercury { .. }));
            }
        }
        t.row(
            &format!("{file}, frames {frames:?} at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of {} frames; refused by the card: {refused}; the same warnings: {agree}", 2 * frames.len()),
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
        let project = reference(|id| json!([fx(&format!("b553-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Mercury {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::Mercury { .. }));
                t.row(
                    &format!("the reference shot, Mercury {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the street still (480 by 270) at `frame`, drawn on the processor, straight 8-bit,
/// with what it warned of; `camera` the composition camera, if any.
fn picture(effects: J, frame: i32, camera: Option<J>) -> (Vec<u8>, Vec<String>) {
    let size = (480, 270);
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("1_left.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(size.0);
    comp["height"] = J::from(size.1);
    comp["duration_frames"] = J::from(96);
    comp["work_area"]["end_frame_exclusive"] = J::from(96);
    if let Some(c) = camera {
        comp["camera"] = c;
    }
    let layer = &mut comp["layers"][0];
    layer["out_frame"] = J::from(96);
    let middle = json!([size.0 as f64 / 2.0, size.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), frame, &repo("verification/D-506 pictures"), 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// Why the `art` layer's effect in `file` is left out of every frame, its keys included, or
/// nothing.
fn fault_of(t: &Table, file: &str) -> String {
    let d = t.load(file).document;
    d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].fault().unwrap_or_default()
}

/// A Mercury's settings, every one `p` leaves out as it is added.
fn parameters(p: &J) -> J {
    let mut all = json!({});
    for (name, _, _, start) in MERCURY_SETTINGS {
        all[name] = J::from(start);
    }
    for (name, _, start) in MERCURY_WORDS {
        all[name] = J::from(start);
    }
    all["light_color"] = J::from("#ffffff");
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    all
}

fn fx(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.mercury", "enabled": true, "parameters": parameters(p)})
}

#[test]
fn b553_mercury() {
    let mut t = Table::new(
        "mercury",
        "# D-674: Mercury\n\nB-553, after CycoreFX's CC Mr. Mercury (Simulation): the layer \
         turned into drops of liquid metal from the in point, each born at the producer with the \
         picture's colour there, thrown and falling, swelling in and shrinking out, running \
         together as metaballs and lit by Drizzle's light. Nothing is carried from frame to \
         frame: every frame counts the drops from the in point (document 20). CycoreFX publishes \
         no formula and no settings when added, so the rule and those settings are ours, stated \
         in `tools/mercury_reference.py`. Every expected pixel is \
         `Fixtures/mercury/expected_mercury.json`, written by the tool before this code existed \
         and printed in document 25 as FX-MERCURY-001 to 038. Tolerance 2e-5.\n",
    );

    t.heading("FX-MERCURY-001 to 038 (document 25)");
    t.fixtures("expected_mercury.json");

    t.heading("The file");
    let files: Vec<String> = (1..=38).map(|n| format!("fx_mercury_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_mercury_023.json");
    t.row(
        "fx_mercury_023.json is saved with its 28 settings, 24 numbers, three words and a colour, and no clock or history",
        &format!(
            "{} settings; light height {}, metal {}, light type {}, light colour {}",
            saved.as_object().unwrap().len(),
            saved["light_height"],
            saved["metal"],
            saved["light_type"],
            saved["light_color"]
        ),
        saved.as_object().unwrap().len() == 28
            && saved["light_height"] == 10
            && saved["metal"] == 0
            && saved["light_type"] == "point"
            && saved["light_color"] == "#ff8000",
    );
    for (n, want) in [
        (27, "birth rate"),
        (28, "longevity"),
        (29, "radius x"),
        (30, "velocity"),
        (31, "extra"),
        (32, "gravity"),
        (33, "blob influence"),
        (34, "death size"),
        (35, "animation"),
        (36, "influence map"),
        (37, "light colour"),
        (38, "gravity"),
    ] {
        let file = format!("fx_mercury_{n:03}.json");
        let why = fault_of(&t, &file);
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }
    t.shape_refused("fx_mercury_001.json", "a Mercury whose birth rate is a word", &parameters(&json!({"birth_rate": "lots"})).to_string());
    t.shape_refused("fx_mercury_001.json", "a Mercury whose animation is a number", &parameters(&json!({"animation": 1})).to_string());
    t.shape_refused("fx_mercury_001.json", "a Mercury without its light colour", &{
        let mut p = parameters(&json!({}));
        p.as_object_mut().unwrap().remove("light_color");
        p.to_string()
    });

    t.heading("Tiles");
    t.tiles(&[("fx_mercury_002.json", 24), ("fx_mercury_011.json", 24), ("fx_mercury_018.json", 12), ("fx_mercury_023.json", 24)]);

    t.heading("On the card (D-674)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            (
                "a vortex of big drops running together under an orange point light",
                json!({"animation": "vortex", "gravity": 0, "birth_size": 200, "blob_influence": 200, "light_type": "point",
                    "light_position_x": 30, "light_position_y": 30, "light_height": 300, "light_color": "#ff8000", "metal": 0}),
                3,
            ),
            (
                "birth rate keyed 0 to 40, thrown to the right",
                json!({"animation": "direction", "direction": 90, "resistance": 100,
                    "birth_rate": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 40, "interp": "linear"}]}}),
                3,
            ),
        ],
    );

    t.heading("Pictures (verification/D-674 pictures)");
    let out = repo("verification/D-674 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0, None).0;
    for (name, p, frame) in [
        ("1_as_added_frame_24", json!({}), 24),
        ("2_fountain", json!({"animation": "direction", "direction": 0, "velocity": 120, "gravity": 150, "extra": 30, "birth_rate": 40, "producer_y": 90}), 48),
        ("3_puddle", json!({"animation": "fixed", "blob_influence": 300, "birth_size": 200, "influence_map": "constant", "radius_x": 60, "radius_y": 30}), 48),
        ("4_vortex", json!({"animation": "vortex", "gravity": 0, "velocity": 80, "birth_rate": 40}), 48),
        ("5_orange_point_light", json!({"birth_size": 250, "light_type": "point", "light_position_x": 30, "light_position_y": 30, "light_height": 100, "light_color": "#ff8000", "metal": 0}), 24),
        ("6_unlit", json!({"birth_size": 250, "ambient": 100, "diffuse": 0, "specular": 0}), 24),
    ] {
        let (bytes, said) = picture(json!([fx("pic", &p)]), frame, None);
        png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
        t.row(
            &format!("{name}.png, frame {frame}: drawn without a warning, the street changed"),
            &format!("{} pixels changed; warnings {said:?}", distance(&bytes, &street).1),
            said.is_empty() && distance(&bytes, &street).1 > 0,
        );
    }
    t.finish("D-674_mercury_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B553_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-553: a measurement, run deliberately with --release --ignored"]
fn b553_mercury_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B553_CPU").is_ok();
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
        ("Noise, then Mercury as added", Some(json!({}))),
        (
            "Noise, then Mercury, forty big drops a second running together, a point light",
            Some(json!({"birth_rate": 40, "birth_size": 200, "blob_influence": 200, "light_type": "point", "light_height": 300})),
        ),
        (
            "Noise, then Mercury, birth rate keyed 0 to 40 (the settings read back to the in point)",
            Some(json!({"birth_rate": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 40, "interp": "linear"}]}})),
        ),
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
    let out = std::env::var("B553_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-553_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
