//! B-910: D-1031, Particle Space's Tri, Quad and Motion Polygon, after the purpose of CC
//! Particle World's flat particle types; the rules ours, stated in
//! `tools/particle_shapes_reference.py`.
//!
//! Writes `verification/B-910_particle_shapes_table.md` and draws pictures into
//! `verification/B-910 pictures/`.
//!
//! Every expected pixel is `Fixtures/particle_shapes/expected_particle_shapes.json`, written by
//! the tool before this code existed and printed in document 25 as FX-PSHAPE-001 to 018.
//! Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;

use effect_table::{both, distance, on_card, repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{
    Effect, PARTICLE_SPACE_COLORS, PARTICLE_SPACE_SETTINGS, PARTICLE_SPACE_WORDS,
};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::PreviewQuality;
use anime_compositor::{persist, png_out, OutputDepth};

/// `effects` on the street still (480 by 270) at `frame`, drawn on the processor, straight 8-bit,
/// with what it warned of.
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

/// A Particle Space's settings, every one `p` leaves out as it is added.
fn parameters(p: &J) -> J {
    let mut all = json!({"texture_layer": ""});
    for (name, _, _, start) in PARTICLE_SPACE_SETTINGS {
        all[name] = J::from(start);
    }
    for (name, _, start) in PARTICLE_SPACE_WORDS {
        all[name] = J::from(start);
    }
    for (name, start) in PARTICLE_SPACE_COLORS {
        all[name] = J::from(start);
    }
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    all
}

#[test]
fn b910_particle_shapes() {
    let mut t = Table::new(
        "particle_shapes",
        "# D-1031: Particle Space's Tri, Quad and Motion Polygon\n\nB-910 (wave 12, gap 8): three \
         more particle types, after the purpose of CC Particle World's flat shapes: Quad (D-611's \
         Textured Square with no texture), Tri (an equilateral triangle of side 2R, turned by \
         Initial Rotation and Rotation Speed) and Motion Polygon (a triangle from the particle back \
         to where a Line's tail is drawn, 2R wide at its base). Never lit, never textured. The \
         rules are ours, stated in `tools/particle_shapes_reference.py`. The card hands the three \
         back to the processor, so every frame below is drawn there. Every expected pixel is \
         `Fixtures/particle_shapes/expected_particle_shapes.json`, written by the tool before \
         this code existed and printed in document 25 as FX-PSHAPE-001 to 018. Tolerance 2e-5.\n",
    );

    t.heading("FX-PSHAPE-001 to 018 (document 25)");
    t.fixtures("expected_particle_shapes.json");

    t.heading("The file");
    let files: Vec<String> = (1..=18).map(|n| format!("fx_pshape_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_pshape_007.json");
    t.row(
        "fx_pshape_007.json is saved with the particle type motion_polygon",
        &format!("particle type {}", saved["particle_type"]),
        saved["particle_type"] == "motion_polygon",
    );
    for n in [17, 18] {
        let file = format!("fx_pshape_{n:03}.json");
        let d = t.load(&file).document;
        let why = d
            .project()
            .composition(&Id::new(MAIN))
            .unwrap()
            .layer(&Id::new("art"))
            .unwrap()
            .effects[0]
            .effect
            .why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming particle type"),
            &why,
            why.to_lowercase().contains("particle type"),
        );
    }

    t.heading("The card hands them back (D-1031)");
    let mut gpu = Gpu::new().expect("a graphics card");
    let comp = Id::new(MAIN);
    let expected: J = serde_json::from_str(
        &fs::read_to_string(t.root.join("expected_particle_shapes.json")).unwrap(),
    )
    .unwrap();
    for (_, case) in expected["cases"].as_object().unwrap() {
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
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| {
                    matches!(e, Effect::ParticleSpace { .. })
                });
            }
        }
        t.row(
            &format!("{file}, frames {frames:?} at Full and Draft, through the card's path"),
            &format!("largest difference {largest} of 255; on the card in {card} of {} frames; refused by the card: {refused}; the same warnings: {agree}", 2 * frames.len()),
            largest <= 1 && !refused && agree && card == 0,
        );
    }

    t.heading("Pictures (verification/B-910 pictures)");
    let out = repo("verification/B-910 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0).0;
    for (name, p, frame) in [
        (
            "1_tris_turning",
            json!({"particle_type": "tri", "birth_rate": 1, "longevity": 2, "velocity": 0.4, "gravity": 0.1,
                "birth_size": 6, "death_size": 3, "initial_rotation": 360, "rotation_speed": 90, "composite_on_original": "on"}),
            48,
        ),
        (
            "2_quads_confetti",
            json!({"particle_type": "quad", "birth_rate": 1, "longevity": 2, "velocity": 0.5, "gravity": 0.3,
                "birth_size": 4, "death_size": 4, "initial_rotation": 360, "rotation_speed": 180,
                "color_map": "custom", "composite_on_original": "on"}),
            48,
        ),
        (
            "3_motion_polygon_sparks",
            json!({"particle_type": "motion_polygon", "birth_rate": 2, "longevity": 1.5, "velocity": 1.2, "gravity": 0.5,
                "birth_size": 3, "death_size": 1, "birth_color": "#ffffff", "death_color": "#ff8800",
                "transfer_mode": "add", "composite_on_original": "on"}),
            48,
        ),
    ] {
        let fx = json!({"instance_id": "pic", "type_id": "core.particle_space", "enabled": true, "parameters": parameters(&p)});
        let (bytes, said) = picture(json!([fx]), frame);
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
    t.finish("B-910_particle_shapes_table.md");
}
