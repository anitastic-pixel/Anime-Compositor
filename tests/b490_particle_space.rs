//! B-490: D-610, Particle Space, the particle engine's first effect (P0-6), after the purpose of
//! CycoreFX's CC Particle World (Simulation): particles born from a producer, thrown by one of
//! eleven animations, pulled by gravity, wind and turbulence, landing on a floor, and drawn in
//! full 3D as lines, faded or shaded spheres, discs or bubbles, through the composition camera
//! when the file has one and through the effect's own camera when not. CycoreFX publishes no
//! formula and no settings when added: the rules and those settings are ours, stated in
//! `tools/particle_space_reference.py`.
//!
//! Writes `verification/D-610_particle_space_table.md` and draws pictures into
//! `verification/D-610 pictures/`.
//!
//! Every expected pixel is `Fixtures/particle_space/expected_particle_space.json` and every
//! expected particle `Fixtures/particle_space/particles_particle_space.json`, both written by the
//! tool before this code existed and printed in document 25 as FX-PSPACE-001 to 071. Tolerances
//! 2e-5 and 1e-7. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::sync::Arc;

use effect_table::{both, distance, median, on_card, reference, repo, time_passes, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, PARTICLE_SPACE_COLORS, PARTICLE_SPACE_SETTINGS, PARTICLE_SPACE_WORDS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{particles, persist, png_out, OutputDepth};

/// Every fixture file on the card against the processor, at each frame its case draws, at Full
/// and Draft. The files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_particle_space.json")).unwrap()).unwrap();
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
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::ParticleSpace { .. }));
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
        let project = reference(|id| json!([fx(&format!("b490-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Particle Space {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::ParticleSpace { .. }));
                t.row(
                    &format!("the reference shot, Particle Space {what} on three layers, frame {frame}, {}", quality.label()),
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

/// The `art` layer's effect in `file`.
fn effect_of(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone()
}

/// A Particle Space's settings, every one `p` leaves out as it is added.
fn parameters(p: &J) -> J {
    let mut all = json!({});
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

fn fx(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.particle_space", "enabled": true, "parameters": parameters(p)})
}

/// The particles alive in `file` at `frame`, each [k, birth, x, y, z, vx, vy, vz], given the
/// clock and the keyed settings as compose gives them.
fn alive(t: &Table, file: &str, frame: i32) -> Vec<[f64; 8]> {
    let d = t.load(file).document;
    let comp = d.project().composition(&Id::new(MAIN)).unwrap();
    let layer = comp.layer(&Id::new("art")).unwrap();
    let instance = &layer.effects[0];
    let fps = comp.frame_rate.numerator() as f64 / comp.frame_rate.denominator() as f64;
    let mut e = instance.at(frame).effect;
    if let Effect::ParticleSpace { clock, history, .. } = &mut e {
        *clock = [(frame - layer.in_frame) as f64, fps];
        if !instance.tracks.is_empty() {
            *history = Some(Arc::new(
                (layer.in_frame..=frame + 1)
                    .map(|f| match instance.at(f).effect {
                        Effect::ParticleSpace { values, .. } => values,
                        _ => unreachable!(),
                    })
                    .collect(),
            ));
        }
    }
    particles::alive(&e)
}

#[test]
fn b490_particle_space() {
    let mut t = Table::new(
        "particle_space",
        "# D-610: Particle Space\n\nB-490, the particle engine's first effect (P0-6), after the \
         purpose of CycoreFX's CC Particle World (Simulation): particles born from a producer, \
         thrown by one of eleven animations, pulled by gravity, wind and turbulence, landing on a \
         floor, and drawn in full 3D as lines, faded or shaded spheres, discs or bubbles, through \
         the composition camera when the file has one and the effect's own camera when not. \
         Nothing is carried from frame to frame: every frame works each particle out from its \
         birth (document 20). CycoreFX publishes no formula and no settings when added, so the \
         rules and those settings are ours, stated in `tools/particle_space_reference.py`. Every \
         expected pixel is `Fixtures/particle_space/expected_particle_space.json` and every \
         expected particle `Fixtures/particle_space/particles_particle_space.json`, written by \
         the tool before this code existed and printed in document 25 as FX-PSPACE-001 to 071. \
         Tolerances 2e-5 and 1e-7.\n",
    );

    t.heading("FX-PSPACE-001 to 071 (document 25)");
    t.fixtures("expected_particle_space.json");

    t.heading("The particles themselves: where each is and how fast it goes");
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("particles_particle_space.json")).unwrap()).unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    for (name, case) in expected["cases"].as_object().unwrap() {
        let n: u32 = name.rsplit('-').next().unwrap().parse().unwrap();
        let file = format!("fx_pspace_{n:03}.json");
        let frame = case["frame"].as_i64().unwrap() as i32;
        let mut want: Vec<Vec<f64>> = case["particles"].as_array().unwrap().iter().map(|p| p.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()).collect();
        let mut got = alive(&t, &file, frame);
        want.sort_by(|a, b| a[0].total_cmp(&b[0]));
        got.sort_by(|a, b| a[0].total_cmp(&b[0]));
        let largest = got.iter().zip(&want).flat_map(|(g, w)| g.iter().zip(w).map(|(a, b)| (a - b).abs())).fold(0.0, f64::max);
        t.row(
            &format!("{name} frame {frame}: the {} particles alive, each its number, birth, place and speed", want.len()),
            &format!("{} alive, largest difference {largest:.1e}", got.len()),
            got.len() == want.len() && largest <= tolerance,
        );
    }

    t.heading("The file");
    let files: Vec<String> = (1..=71).map(|n| format!("fx_pspace_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_pspace_047.json");
    t.row(
        "fx_pspace_047.json is saved with its 59 settings, 44 numbers, 7 words and 8 colours, and no clock, camera or history",
        &format!("{} settings; birth rate {}, particle type {}, camera FOV {}", saved.as_object().unwrap().len(), saved["birth_rate"], saved["particle_type"], saved["camera_fov"]),
        saved.as_object().unwrap().len() == 59 && saved["birth_rate"] == 0.2 && saved["particle_type"] == "shaded_sphere" && saved["camera_fov"] == 20,
    );
    for (n, want) in [
        (59, "birth rate"),
        (60, "birth rate"),
        (61, "longevity"),
        (62, "turbulence scale"),
        (63, "camera fov"),
        (64, "animation"),
        (65, "animation"),
        (66, "particle type"),
        (67, "floor action"),
        (68, "birth colour"),
        (69, "composite on original"),
        (70, "transfer mode"),
    ] {
        let file = format!("fx_pspace_{n:03}.json");
        let why = effect_of(&t, &file).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }
    t.shape_refused("fx_pspace_001.json", "a Particle Space whose birth rate is a word", &parameters(&json!({"birth_rate": "many"})).to_string());
    t.shape_refused("fx_pspace_001.json", "a Particle Space whose animation is a number", &parameters(&json!({"animation": 3})).to_string());
    t.shape_refused("fx_pspace_001.json", "a Particle Space without its depth cue colour", &{
        let mut p = parameters(&json!({}));
        p.as_object_mut().unwrap().remove("depth_cue_color");
        p.to_string()
    });

    t.heading("Tiles");
    t.tiles(&[("fx_pspace_024.json", 24), ("fx_pspace_023.json", 20), ("fx_pspace_046.json", 20), ("fx_pspace_058.json", 12)]);

    t.heading("On the card (D-610)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &(59..=70).collect::<Vec<_>>());
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            (
                "shaded spheres in a vortex, fog, over the drawing",
                json!({"animation": "vortex", "particle_type": "shaded_sphere", "birth_size": 0.6, "death_size": 0.3, "depth_cue": "fog", "composite_on_original": "on", "birth_rate": 5}),
                3,
            ),
            (
                "bubbles bouncing on the floor, screened, birth rate keyed 0 to 4",
                json!({"particle_type": "bubble", "floor_action": "bounce", "birth_size": 0.8, "death_size": 0.8, "transfer_mode": "screen", "composite_on_original": "on",
                    "birth_rate": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 4, "interp": "linear"}]}}),
                3,
            ),
        ],
    );

    t.heading("Pictures (verification/D-610 pictures)");
    let out = repo("verification/D-610 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0, None).0;
    let over = |p: J| {
        let mut p = p;
        p["composite_on_original"] = J::from("on");
        p
    };
    let dolly = json!({"position": {"base": [240, 135], "keyframes": []}, "depth": {"base": -200, "keyframes": []}, "zoom": {"base": 200, "keyframes": []}});
    for (name, p, frame, camera) in [
        ("1_as_added_frame_24", json!({}), 24, None),
        (
            "2_fountain",
            over(json!({"animation": "direction_axis", "extra_angle": 20, "velocity": 1.4, "gravity": 1.2, "birth_rate": 8, "longevity": 2, "producer_y": 0.2,
                "particle_type": "faded_sphere", "birth_size": 1.5, "death_size": 0.8, "birth_color": "#a0d8ff", "death_color": "#2050ff", "transfer_mode": "add"})),
            48,
            None,
        ),
        (
            "3_fire",
            over(json!({"animation": "fire", "birth_rate": 10, "longevity": 1.2, "producer_y": 0.2, "radius_x": 0.08, "velocity": 0.3, "gravity": 0.3,
                "particle_type": "faded_sphere", "birth_size": 6, "death_size": 1, "transfer_mode": "add"})),
            48,
            None,
        ),
        (
            "4_vortex",
            over(json!({"animation": "vortex", "birth_rate": 6, "longevity": 3, "radius_x": 0.3, "radius_z": 0.3, "velocity": 0.3, "gravity": -0.1,
                "particle_type": "shaded_sphere", "birth_size": 2, "death_size": 2, "depth_cue": "fade", "camera_rotation_x": -25})),
            72,
            None,
        ),
        (
            "5_glue_floor",
            over(json!({"birth_rate": 4, "longevity": 4, "velocity": 0.6, "gravity": 1.0, "floor_action": "glue", "floor_position": 0.3,
                "particle_type": "disc", "birth_size": 1.5, "death_size": 1.5, "camera_rotation_x": -30})),
            72,
            None,
        ),
        (
            "6_composition_camera_dollied_in",
            over(json!({"birth_rate": 3, "longevity": 3, "velocity": 0.2, "gravity": 0, "radius_x": 0.4, "radius_y": 0.25, "radius_z": 0.6,
                "particle_type": "shaded_sphere", "birth_size": 2.5, "death_size": 2.5})),
            48,
            Some(dolly.clone()),
        ),
    ] {
        let (bytes, said) = picture(json!([fx("pic", &p)]), frame, camera);
        png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
        t.row(
            &format!("{name}.png, frame {frame}: drawn without a warning, the street changed"),
            &format!("{} pixels changed; warnings {said:?}", distance(&bytes, &street).1),
            said.is_empty() && distance(&bytes, &street).1 > 0,
        );
    }
    t.finish("D-610_particle_space_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B490_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-490: a measurement, run deliberately with --release --ignored"]
fn b490_particle_space_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B490_CPU").is_ok();
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
        ("Noise, then Particle Space as added", Some(json!({}))),
        ("Noise, then Particle Space, 1000 a second, shaded spheres, over the drawing", Some(json!({"birth_rate": 10, "particle_type": "shaded_sphere", "birth_size": 0.5, "composite_on_original": "on"}))),
        ("Noise, then Particle Space, 2000 a second living 3 s, vortex, faded spheres, added", Some(json!({"birth_rate": 20, "longevity": 3, "animation": "vortex", "particle_type": "faded_sphere", "birth_size": 0.4, "transfer_mode": "add", "composite_on_original": "on"}))),
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
    let out = std::env::var("B490_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-490_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
