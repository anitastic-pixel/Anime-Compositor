//! B-550: D-671, Bubbles, after CycoreFX's CC Bubbles (Simulation): the layer turned into
//! bubbles that rise or fall from the in point and wobble, each showing the picture through
//! itself, turned over (Inverse) or upright (World), shaded one of five ways. CycoreFX publishes
//! no formula and no settings when added: the rule and those settings are ours, stated in
//! `tools/bubbles_reference.py`.
//!
//! Writes `verification/D-671_bubbles_table.md` and draws pictures into
//! `verification/D-671 pictures/`.
//!
//! Every expected pixel is `Fixtures/bubbles/expected_bubbles.json`, written by the tool before
//! this code existed and printed in document 25 as FX-BUBBLE-001 to 034. Tolerance 2e-5. Nothing
//! here is a snapshot of a run.

mod effect_table;

use std::fs;

use effect_table::{both, distance, median, on_card, reference, repo, time_passes, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, BUBBLES_SETTINGS, BUBBLES_WORDS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// Every fixture file on the card against the processor, at each frame its case draws, at Full
/// and Draft. The files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    let expected: J =
        serde_json::from_str(&fs::read_to_string(t.root.join("expected_bubbles.json")).unwrap())
            .unwrap();
    for (name, case) in expected["cases"].as_object().unwrap() {
        let n: u32 = name.rsplit('-').next().unwrap().parse().unwrap();
        let file = case["project"].as_str().unwrap();
        let frames: Vec<i32> = case["frames"]
            .as_object()
            .unwrap()
            .keys()
            .map(|f| f.parse().unwrap())
            .collect();
        let project = persist::load(&t.root.join(file))
            .unwrap()
            .document
            .project()
            .clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for &frame in &frames {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| {
                    matches!(e, Effect::Bubbles { .. })
                });
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
        preview::preview_frame_cached(
            project,
            &ref_comp,
            100,
            &ref_root,
            PreviewQuality::Full,
            DEFAULT_TILE_SIZE,
            &mut log,
            &mut CelCache::viewer(),
        )
        .expect("the reference shot draws")
        .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in settings {
        let project = reference(|id| json!([fx(&format!("b550-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Bubbles {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| {
                    matches!(e, Effect::Bubbles { .. })
                });
                t.row(
                    &format!("the reference shot, Bubbles {what} on three layers, frame {frame}, {}", quality.label()),
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
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap(),
    )
    .unwrap();
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
    let frame = render_frame(
        loaded.document.project(),
        &Id::new(MAIN),
        frame,
        &repo("verification/D-506 pictures"),
        64,
        &mut log,
    )
    .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (frame.to_srgb8_straight(), said)
}

/// The `art` layer's effect in `file`.
fn effect_of(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    d.project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layer(&Id::new("art"))
        .unwrap()
        .effects[0]
        .effect
        .clone()
}

/// A Bubbles' settings, every one `p` leaves out as it is added.
fn parameters(p: &J) -> J {
    let mut all = json!({});
    for (name, _, _, start) in BUBBLES_SETTINGS {
        all[name] = J::from(start);
    }
    for (name, _, start) in BUBBLES_WORDS {
        all[name] = J::from(start);
    }
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    all
}

fn fx(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.bubbles", "enabled": true, "parameters": parameters(p)})
}

#[test]
fn b550_bubbles() {
    let mut t = Table::new(
        "bubbles",
        "# D-671: Bubbles\n\nB-550, after CycoreFX's CC Bubbles (Simulation): the layer turned into \
         bubbles that rise or fall from the in point and wobble, each showing the picture through \
         itself, turned over (Inverse) or upright (World), shaded one of five ways. Nothing is \
         carried from frame to frame: every frame works each bubble out from the in point \
         (document 20). CycoreFX publishes no formula and no settings when added, so the rule and \
         those settings are ours, stated in `tools/bubbles_reference.py`. Every expected pixel is \
         `Fixtures/bubbles/expected_bubbles.json`, written by the tool before this code existed \
         and printed in document 25 as FX-BUBBLE-001 to 034. Tolerance 2e-5.\n",
    );

    t.heading("FX-BUBBLE-001 to 034 (document 25)");
    t.fixtures("expected_bubbles.json");

    t.heading("The file");
    let files: Vec<String> = (1..=34).map(|n| format!("fx_bubble_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_bubble_022.json");
    t.row(
        "fx_bubble_022.json is saved with its 8 settings, 6 numbers and 2 words, and no clock or history",
        &format!("{} settings; amount {}, reflection {}, shading {}", saved.as_object().unwrap().len(), saved["amount"], saved["reflection"], saved["shading"]),
        saved.as_object().unwrap().len() == 8 && saved["amount"] == 40 && saved["reflection"] == "world" && saved["shading"] == "lighten",
    );
    for (n, want) in [
        (25, "amount"),
        (26, "amount"),
        (27, "speed"),
        (28, "wobble amplitude"),
        (29, "wobble frequency"),
        (30, "size"),
        (31, "random seed"),
        (32, "reflection"),
        (33, "shading"),
    ] {
        let file = format!("fx_bubble_{n:03}.json");
        let why = effect_of(&t, &file).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming {want}"),
            &why,
            why.to_lowercase().contains(want),
        );
    }
    t.shape_refused(
        "fx_bubble_001.json",
        "a Bubbles whose amount is a word",
        &parameters(&json!({"amount": "many"})).to_string(),
    );
    t.shape_refused(
        "fx_bubble_001.json",
        "a Bubbles whose shading is a number",
        &parameters(&json!({"shading": 3})).to_string(),
    );
    t.shape_refused("fx_bubble_001.json", "a Bubbles without its reflection", &{
        let mut p = parameters(&json!({}));
        p.as_object_mut().unwrap().remove("reflection");
        p.to_string()
    });

    t.heading("Tiles");
    t.tiles(&[
        ("fx_bubble_002.json", 12),
        ("fx_bubble_022.json", 10),
        ("fx_bubble_024.json", 0),
        ("fx_bubble_017.json", 8),
    ]);

    t.heading("On the card (D-671)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[25, 26, 27, 28, 29, 30, 31, 32, 33, 34]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            (
                "forty big bubbles, World, Lighten",
                json!({"amount": 40, "size": 6, "reflection": "world", "shading": "lighten", "speed": 1.5, "wobble_amplitude": 12}),
                3,
            ),
            (
                "speed keyed 0 to 3, None shading, 300 bubbles",
                json!({"amount": 300, "size": 2, "shading": "none",
                    "speed": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 3, "interp": "linear"}]}}),
                3,
            ),
        ],
    );

    t.heading("Pictures (verification/D-671 pictures)");
    let out = repo("verification/D-671 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0, None).0;
    for (name, p, frame) in [
        ("1_as_added_frame_24", json!({}), 24),
        (
            "2_soap_bubbles",
            json!({"amount": 30, "size": 4, "speed": 1, "wobble_amplitude": 8}),
            24,
        ),
        (
            "3_world_lighten",
            json!({"amount": 30, "size": 4, "speed": 1, "reflection": "world", "shading": "lighten"}),
            24,
        ),
        (
            "4_darken_falling",
            json!({"amount": 60, "size": 3, "speed": -1.5, "shading": "darken"}),
            24,
        ),
        (
            "5_fade_outwards",
            json!({"amount": 30, "size": 4, "shading": "fade_outwards"}),
            24,
        ),
        (
            "6_one_fisheye",
            json!({"amount": 1, "size": 12, "shading": "none", "speed": 0, "random_seed": 29}),
            0,
        ),
    ] {
        let (bytes, said) = picture(json!([fx("pic", &p)]), frame, None);
        png_out::write_rgba(
            &out.join(format!("{name}.png")),
            480,
            270,
            OutputDepth::Eight,
            &[],
            &bytes,
        )
        .unwrap();
        t.row(
            &format!("{name}.png, frame {frame}: drawn without a warning, the street changed"),
            &format!(
                "{} pixels changed; warnings {said:?}",
                distance(&bytes, &street).1
            ),
            said.is_empty() && distance(&bytes, &street).1 > 0,
        );
    }
    t.finish("D-671_bubbles_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B550_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-550: a measurement, run deliberately with --release --ignored"]
fn b550_bubbles_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B550_CPU").is_ok();
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
        ("Noise, then Bubbles as added", Some(json!({}))),
        (
            "Noise, then Bubbles, 1000 of size 3, World, Lighten",
            Some(json!({"amount": 1000, "size": 3, "reflection": "world", "shading": "lighten"})),
        ),
        (
            "Noise, then Bubbles, speed keyed 0 to 3 (the settings read back to the in point)",
            Some(
                json!({"speed": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 3, "interp": "linear"}]}}),
            ),
        ),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![
                json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}}),
            ];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}u"), p));
            }
            J::Array(v)
        });
        let (comp, root) = (
            Id::new("comp-reference-shot"),
            repo("Fixtures/reference_shot"),
        );
        let (first, times) = time_passes(
            &mut gpu,
            cpu,
            passes,
            &project,
            &comp,
            &root,
            PreviewQuality::Full,
        );
        let _ = writeln!(
            s,
            "| {name} | Full | {:.1} | {:.1} |",
            median(first),
            median(times)
        );
    }
    let out = std::env::var("B550_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-550_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
