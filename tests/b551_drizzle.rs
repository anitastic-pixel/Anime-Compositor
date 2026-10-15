//! B-551: D-672, Drizzle, after CycoreFX's CC Drizzle (Simulation): raindrops on a pond from
//! the in point, each a ring of ripples that spreads and dies away, bending the layer and catching
//! the light. CycoreFX publishes no formula and no settings when added: the rule and those
//! settings are ours, stated in `tools/drizzle_reference.py`.
//!
//! Writes `verification/D-672_drizzle_table.md` and draws pictures into
//! `verification/D-672 pictures/`.
//!
//! Every expected pixel is `Fixtures/drizzle/expected_drizzle.json`, written by the tool before
//! this code existed and printed in document 25 as FX-DRIZZLE-001 to 036. Tolerance 2e-5. Nothing
//! here is a snapshot of a run.

mod effect_table;

use std::fs;

use effect_table::{both, distance, median, on_card, reference, repo, time_passes, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, DRIZZLE_SETTINGS, DRIZZLE_WORDS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// Every fixture file on the card against the processor, at each frame its case draws, at Full
/// and Draft. The files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_drizzle.json")).unwrap()).unwrap();
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
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::Drizzle { .. }));
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
        let project = reference(|id| json!([fx(&format!("b551-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Drizzle {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::Drizzle { .. }));
                t.row(
                    &format!("the reference shot, Drizzle {what} on three layers, frame {frame}, {}", quality.label()),
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

/// A Drizzle's settings, every one `p` leaves out as it is added.
fn parameters(p: &J) -> J {
    let mut all = json!({});
    for (name, _, _, start) in DRIZZLE_SETTINGS {
        all[name] = J::from(start);
    }
    for (name, _, start) in DRIZZLE_WORDS {
        all[name] = J::from(start);
    }
    all["light_color"] = J::from("#ffffff");
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    all
}

fn fx(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.drizzle", "enabled": true, "parameters": parameters(p)})
}

#[test]
fn b551_drizzle() {
    let mut t = Table::new(
        "drizzle",
        "# D-672: Drizzle\n\nB-551, after CycoreFX's CC Drizzle (Simulation): raindrops on a pond \
         from the in point, each a ring of ripples that spreads and dies away, bending the layer \
         and catching the light by Blobbylize's rule. Nothing is carried from frame to frame: \
         every frame counts the drops from the in point (document 20). CycoreFX publishes no \
         formula and no settings when added, so the rule and those settings are ours, stated in \
         `tools/drizzle_reference.py`. Every expected pixel is \
         `Fixtures/drizzle/expected_drizzle.json`, written by the tool before this code existed \
         and printed in document 25 as FX-DRIZZLE-001 to 036. Tolerance 2e-5.\n",
    );

    t.heading("FX-DRIZZLE-001 to 036 (document 25)");
    t.fixtures("expected_drizzle.json");

    t.heading("The file");
    let files: Vec<String> = (1..=36).map(|n| format!("fx_drizzle_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_drizzle_019.json");
    t.row(
        "fx_drizzle_019.json is saved with its 18 settings, 16 numbers, a word and a colour, and no clock or history",
        &format!(
            "{} settings; specular {}, roughness {}, light type {}, light colour {}",
            saved.as_object().unwrap().len(),
            saved["specular"],
            saved["roughness"],
            saved["light_type"],
            saved["light_color"]
        ),
        saved.as_object().unwrap().len() == 18
            && saved["specular"] == 100
            && saved["roughness"] == 0.5
            && saved["light_type"] == "distant"
            && saved["light_color"] == "#ff8000",
    );
    for (n, want) in [
        (25, "drip rate"),
        (26, "drip rate"),
        (27, "longevity"),
        (28, "rippling"),
        (29, "displacement"),
        (30, "ripple height"),
        (31, "spreading"),
        (32, "roughness"),
        (33, "light type"),
        (34, "light colour"),
        (35, "light position x"),
        (36, "drip rate"),
    ] {
        let file = format!("fx_drizzle_{n:03}.json");
        let why = fault_of(&t, &file);
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }
    t.shape_refused("fx_drizzle_001.json", "a Drizzle whose drip rate is a word", &parameters(&json!({"drip_rate": "lots"})).to_string());
    t.shape_refused("fx_drizzle_001.json", "a Drizzle whose light type is a number", &parameters(&json!({"light_type": 1})).to_string());
    t.shape_refused("fx_drizzle_001.json", "a Drizzle without its light colour", &{
        let mut p = parameters(&json!({}));
        p.as_object_mut().unwrap().remove("light_color");
        p.to_string()
    });

    t.heading("Tiles");
    t.tiles(&[("fx_drizzle_002.json", 14), ("fx_drizzle_018.json", 14), ("fx_drizzle_024.json", 25), ("fx_drizzle_015.json", 26)]);

    t.heading("On the card (D-672)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            (
                "thirty drops a second, Rippling 720, Spreading 150, a point light at the middle",
                json!({"drip_rate": 30, "rippling": 720, "spreading": 150, "light_type": "point",
                    "light_position_x": 50, "light_position_y": 50, "light_height": 200}),
                3,
            ),
            (
                "drip rate keyed 0 to 40, an orange shine",
                json!({"light_color": "#ff8000", "specular": 100, "roughness": 0.3, "metal": 0,
                    "drip_rate": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 40, "interp": "linear"}]}}),
                3,
            ),
        ],
    );

    t.heading("Pictures (verification/D-672 pictures)");
    let out = repo("verification/D-672 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0, None).0;
    for (name, p, frame) in [
        ("1_as_added_frame_24", json!({}), 24),
        ("2_downpour", json!({"drip_rate": 30, "rippling": 720, "spreading": 100}), 48),
        ("3_point_light", json!({"light_type": "point", "light_position_x": 50, "light_position_y": 50, "light_height": 60}), 24),
        ("4_orange_shine", json!({"light_color": "#ff8000", "specular": 100, "roughness": 0.3, "metal": 0}), 24),
        ("5_big_slow_ripples", json!({"drip_rate": 2, "longevity": 3, "spreading": 300, "displacement": 150}), 48),
        ("6_bent_unlit", json!({"drip_rate": 4, "spreading": 200, "displacement": 200, "ambient": 100, "diffuse": 0, "specular": 0}), 24),
    ] {
        let (bytes, said) = picture(json!([fx("pic", &p)]), frame, None);
        png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
        t.row(
            &format!("{name}.png, frame {frame}: drawn without a warning, the street changed"),
            &format!("{} pixels changed; warnings {said:?}", distance(&bytes, &street).1),
            said.is_empty() && distance(&bytes, &street).1 > 0,
        );
    }
    t.finish("D-672_drizzle_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B551_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-551: a measurement, run deliberately with --release --ignored"]
fn b551_drizzle_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B551_CPU").is_ok();
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
        ("Noise, then Drizzle as added", Some(json!({}))),
        (
            "Noise, then Drizzle, thirty drops a second, Rippling 720, Spreading 150, a point light",
            Some(json!({"drip_rate": 30, "rippling": 720, "spreading": 150, "light_type": "point", "light_height": 200})),
        ),
        (
            "Noise, then Drizzle, drip rate keyed 0 to 40 (the settings read back to the in point)",
            Some(json!({"drip_rate": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 40, "interp": "linear"}]}})),
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
        let (first, times) = time_passes(&mut gpu, cpu, passes, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B551_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-551_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
