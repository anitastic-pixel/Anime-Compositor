//! B-554: D-675, Polygon Shards, after CycoreFX's CC Pixel Polly (Simulation): the layer cut
//! along a grid into triangles or squares thrown away from a force centre in 3D, falling and
//! spinning, seen through the composition camera or the effect's own eye. CycoreFX publishes no
//! formula and no settings when added: the rule and those settings are ours, stated in
//! `tools/polygon_shards_reference.py`.
//!
//! Writes `verification/D-675_polygon_shards_table.md` and draws pictures into `verification/D-675
//! pictures/`.
//!
//! Every expected pixel is `Fixtures/polygon_shards/expected_polygon_shards.json`, written by the
//! tool before this code existed and printed in document 25 as FX-POLYGON-SHARDS-001 to 036.
//! Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;

use effect_table::{both, distance, median, on_card, reference, repo, time_passes, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, POLYGON_SHARDS_SETTINGS, POLYGON_SHARDS_WORDS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// Every fixture file on the card against the processor, at each frame its case draws, at Full
/// and Draft. The files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_polygon_shards.json")).unwrap()).unwrap();
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
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::PolygonShards { .. }));
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
        let project = reference(|id| json!([fx(&format!("b554-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Polygon Shards {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::PolygonShards { .. }));
                t.row(
                    &format!("the reference shot, Polygon Shards {what} on three layers, frame {frame}, {}", quality.label()),
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

/// A Polygon Shards' settings, every one `p` leaves out as it is added.
fn parameters(p: &J) -> J {
    let mut all = json!({});
    for (name, _, _, start) in POLYGON_SHARDS_SETTINGS {
        all[name] = J::from(start);
    }
    for (name, _, start) in POLYGON_SHARDS_WORDS {
        all[name] = J::from(start);
    }
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    all
}

fn fx(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.polygon_shards", "enabled": true, "parameters": parameters(p)})
}

#[test]
fn b554_polygon_shards() {
    let mut t = Table::new(
        "polygon_shards",
        "# D-675: Polygon Shards\n\nB-554, after CycoreFX's CC Pixel Polly (Simulation): the \
         layer cut along a grid into triangles or squares from the start time, each thrown away \
         from the force centre in 3D, falling and spinning, seen through the composition camera \
         or the effect's own eye, drawn far to near; before the start time the layer as it is. \
         Nothing is carried from frame to frame: every shard's flight is worked from the start \
         time with every setting as it is at the frame drawn (document 20). CycoreFX publishes \
         no formula and no settings when added, so the rule and those settings are ours, stated \
         in `tools/polygon_shards_reference.py`. Every expected pixel is \
         `Fixtures/polygon_shards/expected_polygon_shards.json`, written by the tool before this \
         code existed and printed in document 25 as FX-POLYGON-SHARDS-001 to 036. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-POLYGON-SHARDS-001 to 036 (document 25)");
    t.fixtures("expected_polygon_shards.json");

    t.heading("The file");
    let files: Vec<String> = (1..=36).map(|n| format!("fx_polygon_shards_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_polygon_shards_023.json");
    t.row(
        "fx_polygon_shards_023.json is saved with its 11 settings, nine numbers and two words, and no clock or camera",
        &format!(
            "{} settings; grid spacing {}, force {}, object {}, depth sort {}",
            saved.as_object().unwrap().len(),
            saved["grid_spacing"],
            saved["force"],
            saved["object"],
            saved["depth_sort"]
        ),
        saved.as_object().unwrap().len() == 11
            && saved["grid_spacing"] == 8
            && saved["force"] == 300
            && saved["object"] == "textured_polygon"
            && saved["depth_sort"] == "on",
    );
    for (n, want) in [
        (26, "force"),
        (27, "gravity"),
        (28, "spinning"),
        (29, "force center x"),
        (30, "direction randomness"),
        (31, "speed randomness"),
        (32, "grid spacing"),
        (33, "start time"),
        (34, "object"),
        (35, "depth sort"),
        (36, "force"),
    ] {
        let file = format!("fx_polygon_shards_{n:03}.json");
        let why = fault_of(&t, &file);
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }
    t.shape_refused("fx_polygon_shards_001.json", "a Polygon Shards whose force is a word", &parameters(&json!({"force": "hard"})).to_string());
    t.shape_refused("fx_polygon_shards_001.json", "a Polygon Shards whose object is a number", &parameters(&json!({"object": 1})).to_string());
    t.shape_refused("fx_polygon_shards_001.json", "a Polygon Shards without its depth sort", &{
        let mut p = parameters(&json!({}));
        p.as_object_mut().unwrap().remove("depth_sort");
        p.to_string()
    });

    t.heading("Tiles");
    t.tiles(&[("fx_polygon_shards_002.json", 12), ("fx_polygon_shards_013.json", 6), ("fx_polygon_shards_019.json", 6), ("fx_polygon_shards_024.json", 12)]);

    t.heading("On the card (D-675)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            (
                "small flat triangles tumbling from the top left",
                json!({"object": "polygon", "grid_spacing": 12, "force_center_x": 0, "force_center_y": 0, "spinning": 720}),
                3,
            ),
            ("force keyed 0 to 600, textured squares in grid order", json!({"object": "textured_square", "depth_sort": "off", "force": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 600, "interp": "linear"}]}}), 3),
        ],
    );

    t.heading("Pictures (verification/D-675 pictures)");
    let out = repo("verification/D-675 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0, None).0;
    for (name, p, frame) in [
        ("1_as_added_frame_24", json!({}), 24),
        ("2_just_cracked_frame_3", json!({}), 3),
        ("3_flat_squares", json!({"object": "square", "grid_spacing": 30}), 24),
        ("4_from_the_top_left", json!({"force_center_x": 0, "force_center_y": 0, "force": 500}), 24),
        ("5_small_tumbling_pieces", json!({"grid_spacing": 8, "spinning": 720, "direction_randomness": 100}), 24),
        ("6_depth_sort_off", json!({"depth_sort": "off", "spinning": 360}), 24),
    ] {
        let (bytes, said) = picture(json!([fx("pic", &p)]), frame, None);
        png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
        t.row(
            &format!("{name}.png, frame {frame}: drawn without a warning, the street changed"),
            &format!("{} pixels changed; warnings {said:?}", distance(&bytes, &street).1),
            said.is_empty() && distance(&bytes, &street).1 > 0,
        );
    }
    t.finish("D-675_polygon_shards_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B554_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-554: a measurement, run deliberately with --release --ignored"]
fn b554_polygon_shards_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B554_CPU").is_ok();
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
        ("Noise, then Polygon Shards as added (the Noise on the processor, as the shards begin the card's run)", Some(json!({}))),
        ("Noise, then Polygon Shards, flat triangles on a grid of 8 pixels", Some(json!({"object": "polygon", "grid_spacing": 8}))),
        ("Noise, then Polygon Shards, force keyed 0 to 600", Some(json!({"force": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 600, "interp": "linear"}]}}))),
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
    let out = std::env::var("B554_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-554_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
