//! B-556: D-677, Star Burst, after CycoreFX's CC Star Burst (Simulation): the layer broken into
//! a grid of stars, each the picture's colour where it was, flying through space towards the
//! viewer and coming round again from far off, seen through the composition camera when the file
//! has one. CycoreFX publishes no formula and no settings when added: the rule and those
//! settings are ours, stated in `tools/star_burst_reference.py`.
//!
//! Writes `verification/D-677_star_burst_table.md` and draws pictures into `verification/D-677
//! pictures/`.
//!
//! Every expected pixel is `Fixtures/star_burst/expected_star_burst.json`, written by the tool
//! before this code existed and printed in document 25 as FX-SBURST-001 to 025. Tolerance 2e-5.
//! Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, distance, median, reference, repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, STAR_BURST_SETTINGS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

/// The Star Bursts the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::StarBurst { .. })))
        .count()
}

/// Every fixture file on the card against the processor, at each frame its case draws, at Full
/// and Draft. The files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_star_burst.json")).unwrap()).unwrap();
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
                card += on_card(&project, &comp, &t.root, frame, quality);
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
        let project = reference(|id| json!([fx(&format!("b556-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Star Burst {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Star Burst {what} on three layers, frame {frame}, {}", quality.label()),
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

/// A Star Burst's settings, every one `p` leaves out as it is added.
fn parameters(p: &J) -> J {
    let mut all = json!({});
    for (name, _, _, start) in STAR_BURST_SETTINGS {
        all[name] = J::from(start);
    }
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    all
}

fn fx(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.star_burst", "enabled": true, "parameters": parameters(p)})
}

#[test]
fn b556_star_burst() {
    let mut t = Table::new(
        "star_burst",
        "# D-677: Star Burst\n\nB-556, after CycoreFX's CC Star Burst (Simulation): the layer \
         broken into a grid of stars, Grid Spacing 270ths of its height apart, each the \
         picture's colour at its grid point; Scatter jumbles them across and through a field \
         three times the eye's distance deep, and the field flies towards the viewer by the \
         speed summed from the in point plus the phase, in picture heights, a star that passes \
         0.9 of the eye's distance in front coming round again behind. They are seen through the \
         composition camera when the file has one, else through an eye 1.5 picture heights off, \
         drawn as Particle Space's discs farthest first onto clear and mixed with the picture by \
         Blend w. Original. Nothing is carried from frame to frame (document 20). CycoreFX \
         publishes no formula and no settings when added, so the rule and those settings are \
         ours, stated in `tools/star_burst_reference.py`. Every expected pixel is \
         `Fixtures/star_burst/expected_star_burst.json`, written by the tool before this code \
         existed and printed in document 25 as FX-SBURST-001 to 025. Tolerance 2e-5.\n",
    );

    t.heading("FX-SBURST-001 to 025 (document 25)");
    t.fixtures("expected_star_burst.json");

    t.heading("The file");
    let files: Vec<String> = (1..=25).map(|n| format!("fx_sburst_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_sburst_007.json");
    t.row(
        "fx_sburst_007.json is saved with its 6 settings, and no clock, view or history",
        &format!(
            "{} settings; scatter {}, speed {}, phase {}, grid spacing {}, size {}, blend with original {}",
            saved.as_object().unwrap().len(),
            saved["scatter"],
            saved["speed"],
            saved["phase"],
            saved["grid_spacing"],
            saved["size"],
            saved["blend_with_original"]
        ),
        saved.as_object().unwrap().len() == 6
            && saved["scatter"] == 100
            && saved["speed"] == 1
            && saved["phase"] == 0
            && saved["grid_spacing"] == 20
            && saved["size"] == 300
            && saved["blend_with_original"] == 0,
    );
    for (n, want) in [(19, "scatter"), (20, "speed"), (21, "phase"), (22, "grid spacing"), (23, "size"), (24, "blend with original"), (25, "speed")] {
        let file = format!("fx_sburst_{n:03}.json");
        let why = fault_of(&t, &file);
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }
    t.shape_refused("fx_sburst_001.json", "a Star Burst whose scatter is a word", &parameters(&json!({"scatter": "lots"})).to_string());
    t.shape_refused("fx_sburst_001.json", "a Star Burst whose grid spacing is a list", &parameters(&json!({"grid_spacing": [4, 4]})).to_string());
    t.shape_refused("fx_sburst_001.json", "a Star Burst without its phase", &{
        let mut p = parameters(&json!({}));
        p.as_object_mut().unwrap().remove("phase");
        p.to_string()
    });

    t.heading("Tiles");
    t.tiles(&[("fx_sburst_002.json", 12), ("fx_sburst_011.json", 6), ("fx_sburst_014.json", 12), ("fx_sburst_018.json", 24)]);

    t.heading("On the card (D-677)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[19, 20, 21, 22, 23, 24, 25]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            ("Grid Spacing 20, Size 300, flying away at Speed -3", json!({"grid_spacing": 20, "size": 300, "speed": -3}), 3),
            (
                "Speed keyed 0 to 5, Blend w. Original 30",
                json!({"blend_with_original": 30, "speed": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 5, "interp": "linear"}]}}),
                3,
            ),
        ],
    );

    t.heading("Pictures (verification/D-677 pictures)");
    let out = repo("verification/D-677 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0, None).0;
    let side = json!({"position": {"base": [150, 135], "keyframes": []}, "depth": {"base": -405, "keyframes": []}, "zoom": {"base": 405, "keyframes": []}});
    for (name, p, frame, camera) in [
        ("1_as_added_frame_24", json!({}), 24, None),
        ("2_close_look_grid_20_frame_24", json!({"grid_spacing": 20}), 24, None),
        ("3_picture_as_dots", json!({"scatter": 0, "speed": 0, "grid_spacing": 8}), 24, None),
        ("4_dots_half_a_height_nearer", json!({"scatter": 0, "speed": 0, "phase": 0.5, "grid_spacing": 8}), 24, None),
        ("5_big_stars_size_400_frame_48", json!({"grid_spacing": 12, "size": 400}), 48, None),
        ("6_blend_with_original_50_frame_24", json!({"grid_spacing": 12, "blend_with_original": 50}), 24, None),
        ("7_composition_camera_from_the_left_frame_24", json!({"grid_spacing": 12}), 24, Some(side.clone())),
    ] {
        let (bytes, said) = picture(json!([fx("pic", &p)]), frame, camera);
        png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
        t.row(
            &format!("{name}.png, frame {frame}: drawn without a warning, the street changed"),
            &format!("{} pixels changed; warnings {said:?}", distance(&bytes, &street).1),
            said.is_empty() && distance(&bytes, &street).1 > 0,
        );
    }
    t.finish("D-677_star_burst_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B556_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-556: a measurement, run deliberately with --release --ignored"]
fn b556_star_burst_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B556_CPU").is_ok();
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
        ("Noise, then Star Burst as added", Some(json!({}))),
        ("Noise, then Star Burst, Grid Spacing 0.5 (about half a million stars)", Some(json!({"grid_spacing": 0.5}))),
        ("Noise, then Star Burst, Speed keyed 0 to 5, Blend w. Original 30", Some(json!({"blend_with_original": 30, "speed": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 5, "interp": "linear"}]}}))),
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
    let out = std::env::var("B556_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-556_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
