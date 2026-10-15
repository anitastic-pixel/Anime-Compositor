//! B-584: D-714, Particle Plane's Textured Square (P0-6): Particle Space's Textured Square (D-611)
//! on the flat emitter, turned and textured as D-710's textured discs, by the owner's decision of
//! 2026-10-13 (FX-PPLANE-055 and FX-PPTEX-024 now refuse "cube"). After the purpose of CycoreFX's
//! CC Particle Systems II; CycoreFX publishes no formula, so the rule is ours, stated in
//! `tools/particle_plane_square_reference.py`.
//!
//! Writes `verification/D-714_particle_plane_square_table.md` and draws pictures into
//! `verification/D-714 pictures/`.
//!
//! Every expected pixel is `Fixtures/particle_plane_square/expected_particle_plane_square.json`,
//! written by the tool before this code existed, FX-PPSQ-001 to 018. Tolerance 2e-5. Nothing here
//! is a snapshot of a run.

mod effect_table;

use std::fs;

use effect_table::{both, distance, median, on_card, reference, repo, time_passes, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{
    Effect, PARTICLE_PLANE_COLORS, PARTICLE_PLANE_SETTINGS, PARTICLE_PLANE_WORDS,
};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// Every fixture file on the card against the processor, at each frame its case draws, at Full
/// and Draft. The files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    let expected: J = serde_json::from_str(
        &fs::read_to_string(t.root.join("expected_particle_plane_square.json")).unwrap(),
    )
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
                    matches!(e, Effect::ParticlePlane { .. })
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
        let project = reference(|id| json!([fx(&format!("b584-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Particle Plane {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| {
                    matches!(e, Effect::ParticlePlane { .. })
                });
                t.row(
                    &format!("the reference shot, Particle Plane {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the street still (480 by 270) at `frame`, drawn on the processor, straight 8-bit,
/// with what it warned of. A switched-off layer `street` holds the same still, for a texture.
fn picture(effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
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
    let layer = &mut comp["layers"][0];
    layer["out_frame"] = J::from(96);
    let middle = json!([size.0 as f64 / 2.0, size.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    let mut street = layer.clone();
    layer["effects"] = effects;
    street["id"] = J::from("street");
    street["name"] = J::from("street");
    street["enabled"] = J::from(false);
    comp["layers"].as_array_mut().unwrap().push(street);
    comp["layer_order"]
        .as_array_mut()
        .unwrap()
        .push(J::from("street"));
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

/// A Particle Plane's settings, every one `p` leaves out as it is added.
fn parameters(p: &J) -> J {
    let mut all = json!({"texture_layer": ""});
    for (name, _, _, start) in PARTICLE_PLANE_SETTINGS {
        all[name] = J::from(start);
    }
    for (name, _, start) in PARTICLE_PLANE_WORDS {
        all[name] = J::from(start);
    }
    for (name, start) in PARTICLE_PLANE_COLORS {
        all[name] = J::from(start);
    }
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    all
}

fn fx(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.particle_plane", "enabled": true, "parameters": parameters(p)})
}

#[test]
fn b584_particle_plane_square() {
    let mut t = Table::new(
        "particle_plane_square",
        "# D-714: Particle Plane's Textured Square\n\nB-584 (P0-6): Particle Space's Textured Square \
         (D-611) on the flat emitter, turned and textured as D-710's textured discs, by the owner's \
         decision of 2026-10-13 (FX-PPLANE-055 and FX-PPTEX-024 now refuse \"cube\"). After the \
         purpose of CycoreFX's CC Particle Systems II; CycoreFX publishes no formula, so the rule is \
         ours, stated in `tools/particle_plane_square_reference.py`. Every expected pixel is \
         `Fixtures/particle_plane_square/expected_particle_plane_square.json`, written by the tool \
         before this code existed, FX-PPSQ-001 to 018. Tolerance 2e-5.\n",
    );

    t.heading("FX-PPSQ-001 to 018");
    t.fixtures("expected_particle_plane_square.json");

    t.heading("The file");
    let files: Vec<String> = (1..=18).map(|n| format!("fx_ppsq_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_ppsq_002.json");
    t.row(
        "fx_ppsq_002.json is saved with its 39 settings, the texture layer dot and particle type textured_square",
        &format!("{} settings; texture layer {}, particle type {}", saved.as_object().unwrap().len(), saved["texture_layer"], saved["particle_type"]),
        saved.as_object().unwrap().len() == 39 && saved["texture_layer"] == "dot" && saved["particle_type"] == "textured_square",
    );
    for file in ["fx_ppsq_017.json", "fx_ppsq_018.json"] {
        let why = effect_of(&t, file).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming particle type and textured_square among its words"),
            &why,
            why.to_lowercase().contains("particle type") && why.contains("textured_square"),
        );
    }

    t.heading("Tiles");
    t.tiles(&[
        ("fx_ppsq_002.json", 20),
        ("fx_ppsq_013.json", 20),
        ("fx_ppsq_016.json", 20),
    ]);

    t.heading("On the card (D-714)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[12, 17, 18]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            (
                "textured squares, no texture layer, turning, over the drawing",
                json!({"particle_type": "textured_square", "birth_size": 0.8, "death_size": 0.8, "initial_rotation": 360, "composite_on_original": "on"}),
                3,
            ),
            (
                "textured squares showing the background layer-4, turning, Add",
                json!({"particle_type": "textured_square", "texture_layer": "layer-4", "birth_size": 1.5, "death_size": 0.5, "rotation_speed": 90, "initial_rotation": 360,
                    "transfer_mode": "add", "birth_rate": 3}),
                3,
            ),
        ],
    );

    t.heading("Pictures (verification/D-714 pictures)");
    let out = repo("verification/D-714 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0).0;
    for (name, p, frame) in [
        (
            "1_street_squares_fountain_over_the_street",
            json!({"particle_type": "textured_square", "texture_layer": "street", "animation": "direction", "extra_angle": 25, "velocity": 1.2, "gravity": 1.0,
                "birth_rate": 3, "longevity": 2, "producer_y": 0.2, "birth_size": 5, "death_size": 2, "rotation_speed": 180, "initial_rotation": 360, "composite_on_original": "on"}),
            48,
        ),
        (
            "2_street_squares_upright_on_empty",
            json!({"particle_type": "textured_square", "texture_layer": "street", "birth_rate": 2, "longevity": 3, "birth_size": 8, "death_size": 8,
                "velocity": 0.3, "gravity": 0.1}),
            48,
        ),
    ] {
        let (bytes, said) = picture(json!([fx("pic", &p)]), frame);
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
    t.finish("D-714_particle_plane_square_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B584_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-584: a measurement, run deliberately with --release --ignored"]
fn b584_particle_plane_square_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B584_CPU").is_ok();
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
        ("Noise, then Particle Plane, textured discs, no texture (D-710)", Some(json!({"particle_type": "textured_disc", "birth_size": 0.8, "initial_rotation": 360}))),
        ("Noise, then Particle Plane, textured squares, no texture", Some(json!({"particle_type": "textured_square", "birth_size": 0.8, "initial_rotation": 360}))),
        (
            "Noise, then Particle Plane, 1000 a second, textured squares of layer-4 (1920 by 1080), turning, over the drawing",
            Some(json!({"birth_rate": 10, "particle_type": "textured_square", "texture_layer": "layer-4", "birth_size": 1, "rotation_speed": 90, "composite_on_original": "on"})),
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
    let out = std::env::var("B584_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-584_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
