//! B-492: D-612, Particle Plane, the particle engine's flat emitter (P0-6), after the purpose of
//! CycoreFX's CC Particle Systems II (Simulation): Particle Space's engine (D-610) with everything
//! in the layer's own plane, particles born from a producer, thrown by one of eleven animations,
//! pulled down by gravity and slowed by resistance, drawn straight onto the layer as lines, faded
//! or shaded spheres, discs, bubbles or stars, faded by an Opacity Map. CycoreFX publishes no
//! formula and no settings when added: the rules, those settings and the name are ours, stated in
//! `tools/particle_plane_reference.py`.
//!
//! Writes `verification/D-612_particle_plane_table.md` and draws pictures into
//! `verification/D-612 pictures/`.
//!
//! Every expected pixel is `Fixtures/particle_plane/expected_particle_plane.json` and every
//! expected particle `Fixtures/particle_plane/particles_particle_plane.json`, both written by the
//! tool before this code existed and printed in document 25 as FX-PPLANE-001 to 058. Tolerances
//! 2e-5 and 1e-7. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::sync::Arc;

use effect_table::{both, distance, median, on_card, reference, repo, time_passes, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, PARTICLE_PLANE_COLORS, PARTICLE_PLANE_SETTINGS, PARTICLE_PLANE_WORDS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{particles, persist, png_out, OutputDepth};

/// Every fixture file on the card against the processor, at each frame its case draws, at Full
/// and Draft. The files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_particle_plane.json")).unwrap()).unwrap();
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
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::ParticlePlane { .. }));
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
        let project = reference(|id| json!([fx(&format!("b492-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Particle Plane {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::ParticlePlane { .. }));
                t.row(
                    &format!("the reference shot, Particle Plane {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the street still (480 by 270) at `frame`, over a second copy of the street with
/// nothing on it, drawn on the processor, straight 8-bit, with what it warned of.
fn picture(effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let size = (480, 270);
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
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
    let mut under = layer.clone();
    layer["effects"] = effects;
    under["id"] = J::from("under");
    under["name"] = J::from("under");
    under["effects"] = json!([]);
    comp["layers"].as_array_mut().unwrap().push(under);
    comp["layer_order"].as_array_mut().unwrap().insert(0, J::from("under"));
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

/// A Particle Plane's settings, every one `p` leaves out as it is added.
fn parameters(p: &J) -> J {
    let mut all = json!({});
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

/// The particles alive in `file` at `frame`, each [k, birth, x, y, z, vx, vy, vz], given the
/// clock and the keyed settings as compose gives them.
fn alive(t: &Table, file: &str, frame: i32) -> Vec<[f64; 8]> {
    let d = t.load(file).document;
    let comp = d.project().composition(&Id::new(MAIN)).unwrap();
    let layer = comp.layer(&Id::new("art")).unwrap();
    let instance = &layer.effects[0];
    let fps = comp.frame_rate.numerator() as f64 / comp.frame_rate.denominator() as f64;
    let mut e = instance.at(frame).effect;
    if let Effect::ParticlePlane { clock, history, .. } = &mut e {
        *clock = [(frame - layer.in_frame) as f64, fps];
        if !instance.tracks.is_empty() {
            *history = Some(Arc::new(
                (layer.in_frame..=frame + 1)
                    .map(|f| match instance.at(f).effect {
                        Effect::ParticlePlane { values, .. } => values,
                        _ => unreachable!(),
                    })
                    .collect(),
            ));
        }
    }
    particles::alive(&e)
}

#[test]
fn b492_particle_plane() {
    let mut t = Table::new(
        "particle_plane",
        "# D-612: Particle Plane\n\nB-492, the particle engine's flat emitter (P0-6), after the \
         purpose of CycoreFX's CC Particle Systems II (Simulation): Particle Space's engine (D-610) \
         with everything in the layer's own plane, particles born from a producer, thrown by one \
         of eleven animations, pulled down by gravity and slowed by resistance, drawn straight onto \
         the layer as lines, faded or shaded spheres, discs, bubbles or stars, faded by an Opacity \
         Map. Nothing is carried from frame to frame: every frame works each particle out from its \
         birth (document 20). CycoreFX publishes no formula and no settings when added, so the \
         rules, those settings and the name are ours, stated in \
         `tools/particle_plane_reference.py`. Every expected pixel is \
         `Fixtures/particle_plane/expected_particle_plane.json` and every expected particle \
         `Fixtures/particle_plane/particles_particle_plane.json`, written by the tool before this \
         code existed and printed in document 25 as FX-PPLANE-001 to 058. Tolerances 2e-5 and \
         1e-7.\n",
    );

    t.heading("FX-PPLANE-001 to 058 (document 25)");
    t.fixtures("expected_particle_plane.json");

    t.heading("The particles themselves: where each is and how fast it goes");
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("particles_particle_plane.json")).unwrap()).unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    for (name, case) in expected["cases"].as_object().unwrap() {
        let n: u32 = name.rsplit('-').next().unwrap().parse().unwrap();
        let file = format!("fx_pplane_{n:03}.json");
        let frame = case["frame"].as_i64().unwrap() as i32;
        let mut want: Vec<Vec<f64>> = case["particles"].as_array().unwrap().iter().map(|p| p.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()).collect();
        let mut got = alive(&t, &file, frame);
        want.sort_by(|a, b| a[0].total_cmp(&b[0]));
        got.sort_by(|a, b| a[0].total_cmp(&b[0]));
        let largest = got.iter().zip(&want).flat_map(|(g, w)| g.iter().zip(w).map(|(a, b)| (a - b).abs())).fold(0.0, f64::max);
        let flat = got.iter().all(|p| p[4] == 0.0 && p[7] == 0.0);
        t.row(
            &format!("{name} frame {frame}: the {} particles alive, each its number, birth, place and speed, and none off the plane", want.len()),
            &format!("{} alive, largest difference {largest:.1e}, all at depth 0 moving in the plane: {flat}", got.len()),
            got.len() == want.len() && largest <= tolerance && flat,
        );
    }

    t.heading("The file");
    let files: Vec<String> = (1..=58).map(|n| format!("fx_pplane_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_pplane_048.json");
    t.row(
        "fx_pplane_048.json is saved with its 35 settings, 23 numbers, 5 words and 7 colours, and no clock or history",
        &format!("{} settings; particle type {}, opacity map {}, transfer mode {}", saved.as_object().unwrap().len(), saved["particle_type"], saved["opacity_map"], saved["transfer_mode"]),
        saved.as_object().unwrap().len() == 35 && saved["particle_type"] == "star" && saved["opacity_map"] == "fade_out" && saved["transfer_mode"] == "add",
    );
    for (n, want) in [
        (49, "birth rate"),
        (50, "longevity"),
        (51, "birth size"),
        (52, "custom opacity 3"),
        (53, "animation"),
        (54, "animation"),
        (55, "particle type"),
        (56, "opacity map"),
        (57, "birth colour"),
        (58, "transfer mode"),
    ] {
        let file = format!("fx_pplane_{n:03}.json");
        let why = effect_of(&t, &file).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }
    t.shape_refused("fx_pplane_001.json", "a Particle Plane whose birth rate is a word", &parameters(&json!({"birth_rate": "many"})).to_string());
    t.shape_refused("fx_pplane_001.json", "a Particle Plane whose particle type is a number", &parameters(&json!({"particle_type": 5})).to_string());
    t.shape_refused("fx_pplane_001.json", "a Particle Plane without its fifth custom colour", &{
        let mut p = parameters(&json!({}));
        p.as_object_mut().unwrap().remove("custom_color_5");
        p.to_string()
    });

    t.heading("Tiles");
    t.tiles(&[("fx_pplane_025.json", 24), ("fx_pplane_006.json", 20), ("fx_pplane_048.json", 12), ("fx_pplane_034.json", 24)]);

    t.heading("On the card (D-612)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &(49..=58).collect::<Vec<_>>());
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            (
                "stars in a vortex, faded in and out, screened",
                json!({"animation": "vortex", "particle_type": "star", "birth_size": 1.2, "death_size": 0.6, "opacity_map": "fade_in_and_out", "transfer_mode": "screen", "birth_rate": 5}),
                3,
            ),
            (
                "bubbles from a fire, custom colours, birth rate keyed 0 to 4",
                json!({"animation": "fire", "particle_type": "bubble", "birth_size": 0.8, "death_size": 0.8, "color_map": "custom", "opacity_map": "custom", "custom_opacity_3": 30,
                    "birth_rate": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 4, "interp": "linear"}]}}),
                3,
            ),
        ],
    );

    t.heading("Pictures (verification/D-612 pictures)");
    let out = repo("verification/D-612 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0).0;
    for (name, p, frame) in [
        ("1_as_added_frame_24", json!({}), 24),
        (
            "2_fountain",
            json!({"animation": "direction", "extra_angle": 20, "velocity": 1.4, "gravity": 1.2, "birth_rate": 8, "longevity": 2, "producer_y": 0.2,
                "particle_type": "faded_sphere", "birth_size": 1.5, "death_size": 0.8, "birth_color": "#a0d8ff", "death_color": "#2050ff", "transfer_mode": "add"}),
            48,
        ),
        (
            "3_star_burst",
            json!({"birth_rate": 3, "longevity": 1.5, "velocity": 0.5, "gravity": 0.3, "particle_type": "star", "birth_size": 4, "death_size": 2,
                "opacity_map": "fade_out", "max_opacity": 100, "birth_color": "#fff6a0", "death_color": "#ff8a20", "transfer_mode": "add"}),
            30,
        ),
        (
            "4_vortex",
            json!({"animation": "vortex", "birth_rate": 6, "longevity": 3, "radius_x": 0.3, "radius_y": 0.2, "velocity": 0.3, "gravity": 0,
                "particle_type": "shaded_sphere", "birth_size": 2, "death_size": 2, "opacity_map": "fade_in_and_out", "max_opacity": 100}),
            72,
        ),
        (
            "5_fire",
            json!({"animation": "fire", "birth_rate": 10, "longevity": 1.2, "producer_y": 0.2, "radius_x": 0.08, "velocity": 0.3, "gravity": 0.3,
                "particle_type": "faded_sphere", "birth_size": 6, "death_size": 1, "transfer_mode": "add", "opacity_map": "fade_out"}),
            48,
        ),
        (
            "6_jet_sideways_lines",
            json!({"animation": "jet_sideways", "direction": 0, "birth_rate": 6, "longevity": 1, "velocity": 0.8, "gravity": 0.2, "birth_size": 1, "death_size": 0.5,
                "max_opacity": 100, "color_map": "custom"}),
            36,
        ),
    ] {
        let (bytes, said) = picture(json!([fx("pic", &p)]), frame);
        png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
        t.row(
            &format!("{name}.png, frame {frame}: drawn without a warning, the street changed"),
            &format!("{} pixels changed; warnings {said:?}", distance(&bytes, &street).1),
            said.is_empty() && distance(&bytes, &street).1 > 0,
        );
    }
    t.finish("D-612_particle_plane_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B492_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-492: a measurement, run deliberately with --release --ignored"]
fn b492_particle_plane_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B492_CPU").is_ok();
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
        ("Noise, then Particle Plane as added", Some(json!({}))),
        ("Noise, then Particle Plane, 1000 a second, stars, faded out", Some(json!({"birth_rate": 10, "particle_type": "star", "birth_size": 0.5, "opacity_map": "fade_out"}))),
        ("Noise, then Particle Plane, 2000 a second living 3 s, vortex, faded spheres, added", Some(json!({"birth_rate": 20, "longevity": 3, "animation": "vortex", "particle_type": "faded_sphere", "birth_size": 0.4, "transfer_mode": "add"}))),
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
    let out = std::env::var("B492_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-492_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
