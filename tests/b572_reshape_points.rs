//! B-572: D-702, Reshape's Correspondence Points and Interpolation, the gaps D-606 left against
//! After Effects' Reshape: which places on the source and destination outlines answer each
//! other, and Percent smoothed between its keys.
//!
//! Writes `verification/D-702_reshape_points_table.md` and draws pictures into
//! `verification/D-702 pictures/`.
//!
//! Every expected pixel is `Fixtures/reshape_points/expected_reshape_points.json`, written by
//! `tools/reshape_points_reference.py` before this code existed and printed in document 25 as
//! FX-RESHP-001 to 015. Tolerance 2e-5. Nothing here is a snapshot of a run. D-606's own
//! fixtures stand unchanged and are B-485's.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{distance, keys, repo, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

fn mine(e: &Effect) -> bool {
    matches!(e, Effect::Reshape { .. })
}

/// A rectangle from (x0, y0) to (x1, y1), corner points only.
fn rect([x0, y0, x1, y1]: [f64; 4]) -> J {
    let points: Vec<J> = [[x0, y0], [x1, y0], [x1, y1], [x0, y1]].iter().map(|p| json!({"point": p, "in": [0, 0], "out": [0, 0]})).collect();
    json!({ "points": points })
}

/// Masks that cut nothing (mode None), for Reshape to choose from.
fn masks(rects: &[[f64; 4]]) -> J {
    J::Array(
        rects
            .iter()
            .enumerate()
            .map(|(m, &r)| json!({"name": format!("Mask {}", m + 1), "enabled": true, "inverted": false, "mode": "none", "opacity": 1.0,
                "feather_px": 0.0, "expansion_px": 0.0, "path": {"base": rect(r), "keyframes": []}}))
            .collect(),
    )
}

/// Percent keyed 0 at frame 0, 100 at 2 and 40 at 4, as FX-RESHP-007.
fn up_and_back() -> J {
    json!({"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 2, "value": 100, "interp": "linear"}, {"frame": 4, "value": 40, "interp": "linear"}]})
}

/// A Reshape from mask 1 to mask 2 with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"source_mask": 1, "destination_mask": 2, "boundary_mask": 0, "percent": 100, "elasticity": "normal"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.reshape", "enabled": true, "parameters": all})
}

/// The reference shot (1920 by 1080) with B-485's three masks (a square, the same square moved
/// 200 right, a boundary about both) and `stack` on its first three layers.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        let d = 40.0 * i as f64;
        layers[i]["masks"] = masks(&[[760.0 + d, 340.0, 1160.0 + d, 740.0], [960.0 + d, 340.0, 1360.0 + d, 740.0], [400.0, 100.0, 1520.0, 980.0]]);
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// The street's masks: 1 a square about the middle, 2 the same square 60 right.
const STREET: [[f64; 4]; 2] = [[190.0, 85.0, 290.0, 185.0], [250.0, 85.0, 350.0, 185.0]];

/// `effects` on the street (in `dir`), with [`STREET`]'s masks, drawn on the processor at
/// `frame`, straight 8-bit, with what it warned of.
fn picture(dir: &Path, effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["masks"] = masks(&STREET);
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// The `art` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

#[test]
fn b572_reshape_points() {
    let mut t = Table::new(
        "reshape_points",
        "# D-702: Reshape's Correspondence Points and Interpolation\n\nB-572: `core.reshape` gains \
         `correspondence_points` (pairs of per cent places along the source and destination \
         outlines that answer each other; none is D-606's matching) and `interpolation` (Discrete, \
         Linear or Smooth: Percent between two of its keys worked as a blend of the fields at the \
         keys' Percents). Every expected pixel is `Fixtures/reshape_points/expected_reshape_points.json`, \
         written by `tools/reshape_points_reference.py` before this code existed and printed in \
         document 25 as FX-RESHP-001 to 015. Tolerance 2e-5.\n",
    );

    t.heading("FX-RESHP-001 to 015 (document 25)");
    t.fixtures_numbered("expected_reshape_points.json", 1..=15);

    t.heading("The file");
    let files: Vec<String> = (1..=15).map(|n| format!("fx_reshp_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_reshp_003.json");
    t.row(
        "fx_reshp_003.json is saved with its seven settings, the pairs one flat list, the interpolation a word",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 7 && saved["correspondence_points"] == json!([0, 0, 50, 25]) && saved["interpolation"] == "discrete",
    );
    for (file, want) in [
        ("fx_reshp_012.json", "correspondence points are none, or pairs"),
        ("fx_reshp_013.json", "correspondence points"),
        ("fx_reshp_014.json", "at most 32 pairs"),
        ("fx_reshp_015.json", "interpolation is discrete, linear or smooth"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Reshape and its {want}"), &why, why.contains("Reshape") && why.contains(want));
    }
    let full = fx("x", &json!({}))["parameters"].to_string();
    t.shape_refused("fx_reshp_001.json", "a Reshape whose interpolation is a number", &full.replace("\"elasticity\":\"normal\"", "\"elasticity\":\"normal\",\"interpolation\":2"));
    t.shape_refused("fx_reshp_001.json", "a Reshape whose correspondence points are one number", &full.replace("\"elasticity\":\"normal\"", "\"elasticity\":\"normal\",\"correspondence_points\":5"));

    t.heading("Commands");
    let mut document = t.load("fx_reshp_003.json").document;
    let odd = changed(&t, "fx_reshp_003.json", |e| if let Effect::Reshape { correspondence_points, .. } = e { correspondence_points.truncate(3) });
    let cubic = changed(&t, "fx_reshp_003.json", |e| if let Effect::Reshape { interpolation, .. } = e { *interpolation = "cubic".into() });
    t.refused(
        &mut document,
        vec![
            ("three numbers for the correspondence points", set(odd)),
            ("interpolation cubic", set(cubic)),
            ("the pairs keyed to end at 101", keys("correspondence_points", &[(0, &[0.0, 0.0, 50.0, 25.0]), (4, &[0.0, 0.0, 50.0, 101.0])])),
        ],
    );
    let smooth = changed(&t, "fx_reshp_003.json", |e| if let Effect::Reshape { interpolation, .. } = e { *interpolation = "smooth".into() });
    t.taken(
        &mut document,
        "fx_reshp_003.json",
        vec![
            ("interpolation smooth", set(smooth)),
            ("the pairs keyed from (0, 0), (50, 25) to (0, 25), (50, 50)", keys("correspondence_points", &[(0, &[0.0, 0.0, 50.0, 25.0]), (4, &[0.0, 25.0, 50.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_reshp_003.json", 0), ("fx_reshp_004.json", 0), ("fx_reshp_007.json", 1), ("fx_reshp_007.json", 3)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // The refused (012 to 015) are left out, so nothing is on the card; 005 and 006 rest at
    // frame 0 only.
    t.card_fixtures(&mut gpu, "expected_reshape_points.json", &[12, 13, 14, 15], mine);
    let shots: Vec<(&str, Project, usize)> = [
        ("Reshape, the square to the square moved right, pairs (0, 0) and (50, 25)", json!({"correspondence_points": [0, 0, 50, 25]})),
        ("Reshape, Percent keyed 20 to 100 and back to 40, Smooth", json!({"percent": {"base": 0, "keyframes": [{"frame": 0, "value": 20, "interp": "linear"}, {"frame": 120, "value": 100, "interp": "linear"}, {"frame": 239, "value": 40, "interp": "linear"}]}, "interpolation": "smooth"})),
    ]
    .into_iter()
    .map(|(what, p)| (what, reference(|id| json!([fx(&format!("b572-{id}"), &p)])), 3))
    .collect();
    t.card_reference(&mut gpu, &reference(|_| json!([])), &shots, mine);

    t.heading("Pictures: in `verification/D-702 pictures/`");
    let dir = repo("verification/D-702 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (w, h) = TOWN;
    let street = |p: J, frame: i32| picture(&dir, json!([fx("fx-0-0", &p)]), frame);
    let (before, said) = picture(&dir, json!([]), 0);
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect (its two masks cut nothing); draws cleanly", &format!("{said:?}"), said.is_empty());

    let (plain, said) = street(json!({}), 0);
    write("2_matched_from_the_start.png", &plain);
    let (paired, said_paired) = street(json!({"correspondence_points": [0, 0]}), 0);
    t.row(
        "2_matched_from_the_start.png, the square pulled 60 right with no pairs; one pair (0, 0) draws it byte for byte; both draw cleanly",
        &format!("{said:?} {said_paired:?}, {} pixels differ from the street, {} between the two", distance(&plain, &before).1, distance(&plain, &paired).1),
        said.is_empty() && said_paired.is_empty() && distance(&plain, &before).1 > w * h / 100 && plain == paired,
    );

    let (turned, said) = street(json!({"correspondence_points": [0, 25]}), 0);
    write("3_turned_a_quarter.png", &turned);
    t.row(
        "3_turned_a_quarter.png, the same with the pair (0, 25): the source's top left corner answers the destination's top right, so the middle swirls a quarter turn on its way; differs from 2; draws cleanly",
        &format!("{said:?}, {} pixels differ from 2", distance(&turned, &plain).1),
        said.is_empty() && distance(&turned, &plain).1 > w * h / 100,
    );

    let keyed = |i: &str| json!({"percent": up_and_back(), "interpolation": i});
    let (discrete, said_d) = street(keyed("discrete"), 3);
    let (linear, said_l) = street(keyed("linear"), 3);
    let (smooth, said_s) = street(keyed("smooth"), 3);
    write("4_discrete_frame_3.png", &discrete);
    write("5_linear_frame_3.png", &linear);
    write("6_smooth_frame_3.png", &smooth);
    t.row(
        "4_discrete_frame_3.png, 5_linear_frame_3.png and 6_smooth_frame_3.png: Percent keyed 0, 100, 40 at frames 0, 2, 4, drawn at frame 3 Discrete (one pull of 70 per cent), Linear (the pulls of 100 and 40 per cent, half each) and Smooth (the curve through the keys): Linear differs from Discrete and Smooth from Linear; all draw cleanly",
        &format!(
            "{said_d:?} {said_l:?} {said_s:?}; Linear against Discrete {} pixels, Smooth against Linear {}",
            distance(&linear, &discrete).1,
            distance(&smooth, &linear).1
        ),
        said_d.is_empty() && said_l.is_empty() && said_s.is_empty() && distance(&linear, &discrete).1 > 0 && distance(&smooth, &linear).1 > 0,
    );

    t.finish("D-702_reshape_points_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way, as B-485's: the reference shot with a Noise that changes every
/// frame on its first three layers (with their three masks), then the effect, every eighth frame
/// asked for whole. With `B572_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-572: a measurement, run deliberately with --release --ignored"]
fn b572_reshape_points_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B572_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        ("Noise, then Reshape with pairs (0, 0) and (50, 25)", Some(json!({"correspondence_points": [0, 0, 50, 25]}))),
        (
            "Noise, then Reshape, Percent keyed 20 to 100 and back to 40, Smooth",
            Some(json!({"percent": {"base": 0, "keyframes": [{"frame": 0, "value": 20, "interp": "linear"}, {"frame": 120, "value": 100, "interp": "linear"}, {"frame": 239, "value": 40, "interp": "linear"}]}, "interpolation": "smooth"})),
        ),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}c"), p));
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
    let out = std::env::var("B572_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-572_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
