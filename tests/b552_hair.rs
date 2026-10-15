//! B-552: D-673, Hair, after CycoreFX's CC Hair (Simulation): strands grown where the layer is
//! opaque, 3D curves seen straight on, hanging under their weight, coloured from the hair colour
//! and the picture at their roots and lit as hair is. CycoreFX publishes no formula and no
//! settings when added: the rule and those settings are ours, stated in
//! `tools/hair_reference.py`.
//!
//! Writes `verification/D-673_hair_table.md` and draws pictures into `verification/D-673
//! pictures/`.
//!
//! Every expected pixel is `Fixtures/hair/expected_hair.json`, written by the tool before this
//! code existed and printed in document 25 as FX-HAIR-001 to 032. Tolerance 2e-5. Nothing here is
//! a snapshot of a run.

mod effect_table;

use std::fs;

use effect_table::{both, distance, median, on_card, reference, repo, time_passes, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, HAIR_SETTINGS, HAIR_WORDS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// Every fixture file on the card against the processor, at each frame its case draws, at Full
/// and Draft. The files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    let expected: J =
        serde_json::from_str(&fs::read_to_string(t.root.join("expected_hair.json")).unwrap())
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
                    matches!(e, Effect::Hair { .. })
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
        let project = reference(|id| json!([fx(&format!("b552-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Hair {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| {
                    matches!(e, Effect::Hair { .. })
                });
                t.row(
                    &format!("the reference shot, Hair {what} on three layers, frame {frame}, {}", quality.label()),
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

/// Why the `art` layer's effect in `file` is left out of every frame, its keys included, or
/// nothing.
fn fault_of(t: &Table, file: &str) -> String {
    let d = t.load(file).document;
    d.project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layer(&Id::new("art"))
        .unwrap()
        .effects[0]
        .fault()
        .unwrap_or_default()
}

/// A Hair's settings, every one `p` leaves out as it is added.
fn parameters(p: &J) -> J {
    let mut all = json!({});
    for (name, _, _, start) in HAIR_SETTINGS {
        all[name] = J::from(start);
    }
    for (name, _, start) in HAIR_WORDS {
        all[name] = J::from(start);
    }
    all["hair_color"] = J::from("#7a4e2d");
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    all
}

fn fx(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.hair", "enabled": true, "parameters": parameters(p)})
}

#[test]
fn b552_hair() {
    let mut t = Table::new(
        "hair",
        "# D-673: Hair\n\nB-552, after CycoreFX's CC Hair (Simulation): strands grown where the \
         layer is opaque, 3D curves seen straight on, hanging under their weight, coloured from \
         the hair colour and the picture at their roots and lit as hair is, brightest across the \
         light. Nothing moves by itself: a frame is the settings as they are at that frame \
         (document 20). CycoreFX publishes no formula and no settings when added, so the rule \
         and those settings are ours, stated in `tools/hair_reference.py`. Every expected pixel \
         is `Fixtures/hair/expected_hair.json`, written by the tool before this code existed and \
         printed in document 25 as FX-HAIR-001 to 032. Tolerance 2e-5.\n",
    );

    t.heading("FX-HAIR-001 to 032 (document 25)");
    t.fixtures("expected_hair.json");

    t.heading("The file");
    let files: Vec<String> = (1..=32).map(|n| format!("fx_hair_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_hair_017.json");
    t.row(
        "fx_hair_017.json is saved with its 15 settings, 13 numbers, a word and a colour",
        &format!(
            "{} settings; density {}, colour inheritance {}, constant mass {}, hair colour {}",
            saved.as_object().unwrap().len(),
            saved["density"],
            saved["color_inheritance"],
            saved["constant_mass"],
            saved["hair_color"]
        ),
        saved.as_object().unwrap().len() == 15
            && saved["density"] == 0.5
            && saved["color_inheritance"] == 0
            && saved["constant_mass"] == "off"
            && saved["hair_color"] == "#2050ff",
    );
    for (n, want) in [
        (21, "length"),
        (22, "length"),
        (23, "thickness"),
        (24, "weight"),
        (25, "density"),
        (26, "color inheritance"),
        (27, "opacity"),
        (28, "roughness"),
        (29, "constant mass"),
        (30, "hair colour"),
        (31, "light direction"),
        (32, "weight"),
    ] {
        let file = format!("fx_hair_{n:03}.json");
        let why = fault_of(&t, &file);
        t.row(
            &format!("{file} is refused in a sentence naming {want}"),
            &why,
            why.to_lowercase().contains(want),
        );
    }
    t.shape_refused(
        "fx_hair_001.json",
        "a Hair whose length is a word",
        &parameters(&json!({"length": "long"})).to_string(),
    );
    t.shape_refused(
        "fx_hair_001.json",
        "a Hair whose constant mass is a number",
        &parameters(&json!({"constant_mass": 1})).to_string(),
    );
    t.shape_refused("fx_hair_001.json", "a Hair without its hair colour", &{
        let mut p = parameters(&json!({}));
        p.as_object_mut().unwrap().remove("hair_color");
        p.to_string()
    });

    t.heading("Tiles");
    t.tiles(&[
        ("fx_hair_001.json", 3),
        ("fx_hair_002.json", 3),
        ("fx_hair_014.json", 3),
        ("fx_hair_019.json", 3),
    ]);

    t.heading("On the card (D-673)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(
        &mut t,
        &mut gpu,
        &[21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32],
    );
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            (
                "a long fur: length 200, thickness 6, density 60",
                json!({"length": 200, "thickness": 6, "density": 60}),
                3,
            ),
            (
                "weight keyed -100 to 200, constant mass, blue",
                json!({"length": 150, "thickness": 5, "constant_mass": "on", "hair_color": "#2050ff", "color_inheritance": 0,
                    "weight": {"base": 0, "keyframes": [{"frame": 0, "value": -100, "interp": "linear"}, {"frame": 239, "value": 200, "interp": "linear"}]}}),
                3,
            ),
        ],
    );

    t.heading("Pictures (verification/D-673 pictures)");
    let out = repo("verification/D-673 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0, None).0;
    for (name, p) in [
        ("1_as_added", json!({})),
        (
            "2_long_fur",
            json!({"length": 200, "thickness": 6, "density": 60}),
        ),
        (
            "3_curled_up",
            json!({"length": 150, "thickness": 5, "density": 80, "weight": -80}),
        ),
        (
            "4_blue_hair",
            json!({"length": 120, "thickness": 6, "density": 40, "hair_color": "#2050ff", "color_inheritance": 0}),
        ),
        (
            "5_shiny",
            json!({"length": 200, "thickness": 8, "density": 30, "specular": 100, "roughness": 0.05}),
        ),
        (
            "6_constant_mass",
            json!({"length": 250, "thickness": 5, "density": 20, "weight": 150, "constant_mass": "on"}),
        ),
    ] {
        let (bytes, said) = picture(json!([fx("pic", &p)]), 0, None);
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
            &format!("{name}.png: drawn without a warning, the street changed"),
            &format!(
                "{} pixels changed; warnings {said:?}",
                distance(&bytes, &street).1
            ),
            said.is_empty() && distance(&bytes, &street).1 > 0,
        );
    }
    t.finish("D-673_hair_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B552_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-552: a measurement, run deliberately with --release --ignored"]
fn b552_hair_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B552_CPU").is_ok();
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
        ("Noise, then Hair as added", Some(json!({}))),
        (
            "Noise, then Hair, a long fur: length 200, thickness 6, density 60",
            Some(json!({"length": 200, "thickness": 6, "density": 60})),
        ),
        (
            "Noise, then Hair, weight keyed -100 to 200, constant mass",
            Some(
                json!({"constant_mass": "on", "weight": {"base": 0, "keyframes": [{"frame": 0, "value": -100, "interp": "linear"}, {"frame": 239, "value": 200, "interp": "linear"}]}}),
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
    let out = std::env::var("B552_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-552_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
