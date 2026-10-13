//! B-495: D-615, Particle Emitter, the particle engine's fourth effect (P0-6, PLUGINS.md pick
//! #20), after what Trapcode Particular is for: Particle Space's engine (D-610) with particles
//! born from a point, a box, a ball or a grid of points in 3D, thrown out every way, one way, two
//! ways, round a disc, or straight out or in, each with its own life, size and opacity changing
//! over that life, coloured once or along a gradient, pushed by gravity, wind and swirling air,
//! and bouncing, sliding or sticking on a floor; drawn as soft or glowing spheres, stars, faded
//! clouds or streaks. Red Giant publishes no formula and no settings when added: the rules, those
//! settings and the name are ours, stated in `tools/particle_emitter_reference.py`.
//!
//! Writes `verification/D-615_particle_emitter_table.md` and draws pictures into
//! `verification/D-615 pictures/`.
//!
//! Every expected pixel is `Fixtures/particle_emitter/expected_particle_emitter.json` and every
//! expected particle `Fixtures/particle_emitter/particles_particle_emitter.json`, both written by
//! the tool before this code existed and printed in document 25 as FX-PEMIT-001 to 063.
//! Tolerances 2e-5 and 1e-7. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::sync::Arc;

use effect_table::{repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{Effect, PARTICLE_EMITTER_COLORS, PARTICLE_EMITTER_SETTINGS, PARTICLE_EMITTER_WORDS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{particles, persist, png_out, OutputDepth};

/// The pixels that differ between two straight 8-bit pictures.
fn differing(a: &[u8], b: &[u8]) -> usize {
    a.chunks_exact(4).zip(b.chunks_exact(4)).filter(|(p, q)| p != q).count()
}

/// Why the `art` layer's effect in `file` is left out, its keys too.
fn fault_of(t: &Table, file: &str) -> String {
    let d = t.load(file).document;
    d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].fault().unwrap_or_default()
}

/// The particles alive in `file` at `frame`, each [k, birth, x, y, z, vx, vy, vz], given the
/// clock and the keyed settings as compose gives them, on the fixtures' 16 by 10 layer.
fn alive(t: &Table, file: &str, frame: i32) -> Vec<[f64; 8]> {
    let d = t.load(file).document;
    let comp = d.project().composition(&Id::new(MAIN)).unwrap();
    let layer = comp.layer(&Id::new("art")).unwrap();
    let instance = &layer.effects[0];
    let fps = comp.frame_rate.numerator() as f64 / comp.frame_rate.denominator() as f64;
    let mut e = instance.at(frame).effect;
    if let Effect::ParticleEmitter { clock, history, .. } = &mut e {
        *clock = [(frame - layer.in_frame) as f64, fps];
        if !instance.tracks.is_empty() {
            *history = Some(Arc::new((layer.in_frame..=frame + 1).map(|f| particles::emitter_numbers_row(&instance.at(f).effect)).collect()));
        }
    }
    particles::emitter_alive(&e, 16.0, 10.0)
}

/// The street still (480 by 270) with `effects` on it at `frame`, over a second copy of the
/// street with nothing on it, drawn on the processor, straight 8-bit, with what it warned of.
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

/// A Particle Emitter's settings, every one `p` leaves out as it is added.
fn parameters(p: &J) -> J {
    let mut all = json!({"emitter_position": [50, 50]});
    for (name, _, _, start) in PARTICLE_EMITTER_SETTINGS {
        all[name] = J::from(start);
    }
    for (name, _, start) in PARTICLE_EMITTER_WORDS {
        all[name] = J::from(start);
    }
    for (name, start) in PARTICLE_EMITTER_COLORS {
        all[name] = J::from(start);
    }
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    all
}

fn fx(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.particle_emitter", "enabled": true, "parameters": parameters(p)})
}

#[test]
fn b495_particle_emitter() {
    let mut t = Table::new(
        "particle_emitter",
        "# D-615: Particle Emitter\n\nB-495, the particle engine's fourth effect (P0-6, PLUGINS.md \
         pick #20), after what Trapcode Particular is for: Particle Space's engine (D-610) with \
         particles born from a point, a box, a ball or a grid of points in 3D, thrown out every \
         way, one way, two ways, round a disc, or straight out or in, each with its own life, \
         size and opacity changing over that life, coloured once or along a gradient, pushed by \
         gravity, wind and swirling air, and bouncing, sliding or sticking on a floor; drawn as \
         soft or glowing spheres, stars, faded clouds or streaks over a clear layer, through the \
         composition camera when there is one. Nothing is carried from frame to frame: every \
         frame works each particle out from its birth (document 20). The particles are worked \
         out on the processor and drawn on the card too (D-617), within 1 level of the \
         processor. Red Giant publishes no formula and no settings when added, so the rules, those \
         settings and the name are ours, stated in `tools/particle_emitter_reference.py`. Every \
         expected pixel is `Fixtures/particle_emitter/expected_particle_emitter.json` and every \
         expected particle `Fixtures/particle_emitter/particles_particle_emitter.json`, written \
         by the tool before this code existed and printed in document 25 as FX-PEMIT-001 to 063. \
         Tolerances 2e-5 and 1e-7.\n",
    );

    t.heading("FX-PEMIT-001 to 063 (document 25)");
    t.fixtures("expected_particle_emitter.json");

    t.heading("The particles themselves: where each is and how fast it goes");
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("particles_particle_emitter.json")).unwrap()).unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    for (name, case) in expected["cases"].as_object().unwrap() {
        let n: u32 = name.rsplit('-').next().unwrap().parse().unwrap();
        let file = format!("fx_pemit_{n:03}.json");
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
    let files: Vec<String> = (1..=63).map(|n| format!("fx_pemit_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_pemit_051.json");
    let count = saved.as_object().unwrap().len();
    t.row(
        "fx_pemit_051.json is saved with its 58 settings (45 numbers, the position, 6 words and 6 colours) and no clock, camera or history",
        &format!("{count} settings; emitter position {}, particle type {}, set color {}, transfer mode {}", saved["emitter_position"], saved["particle_type"], saved["set_color"], saved["transfer_mode"]),
        count == 58
            && saved["emitter_position"].as_array().is_some_and(|a| a.len() == 2)
            && saved["particle_type"] == "glow_sphere"
            && saved["set_color"] == "over_life"
            && saved["transfer_mode"] == "add",
    );
    for (n, want) in [
        (52, "particles per second"),
        (53, "life"),
        (54, "size over life 3"),
        (55, "opacity over life 2"),
        (56, "emitter type"),
        (57, "direction"),
        (58, "particle type"),
        (59, "set color"),
        (60, "transfer mode"),
        (61, "floor action"),
        (62, "gradient colour 2"),
        (63, "grid x"),
    ] {
        let file = format!("fx_pemit_{n:03}.json");
        let why = fault_of(&t, &file);
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }
    t.shape_refused("fx_pemit_001.json", "a Particle Emitter whose velocity is a word", &parameters(&json!({"velocity": "fast"})).to_string());
    t.shape_refused("fx_pemit_001.json", "a Particle Emitter whose particle type is a number", &parameters(&json!({"particle_type": 2})).to_string());
    t.shape_refused("fx_pemit_001.json", "a Particle Emitter without its fifth gradient colour", &{
        let mut p = parameters(&json!({}));
        p.as_object_mut().unwrap().remove("gradient_color_5");
        p.to_string()
    });

    t.heading("Tiles");
    t.tiles(&[("fx_pemit_016.json", 20), ("fx_pemit_022.json", 20), ("fx_pemit_039.json", 24), ("fx_pemit_044.json", 20)]);

    t.heading("Pictures (verification/D-615 pictures)");
    let out = repo("verification/D-615 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0).0;
    for (name, p, frame) in [
        ("1_as_added_frame_24", json!({}), 24),
        (
            "2_fountain",
            json!({"emitter_position": [50, 95], "direction": "directional", "direction_spread": 12, "velocity": 320, "velocity_random": 15, "gravity": 380,
                "particles_per_second": 240, "life": 2, "particle_type": "glow_sphere", "size": 6, "set_color": "over_life", "transfer_mode": "add",
                "gradient_color_1": "#ffffff", "gradient_color_2": "#a0e0ff", "gradient_color_3": "#4090ff", "gradient_color_4": "#2040c0", "gradient_color_5": "#101040"}),
            48,
        ),
        (
            "3_sparks_on_a_floor",
            json!({"emitter_position": [50, 30], "velocity": 220, "velocity_random": 40, "gravity": 500, "floor_position": 90, "floor_action": "bounce",
                "bounce": 55, "bounce_random": 20, "particles_per_second": 300, "life": 2.5, "size": 3, "set_color": "random_from_gradient", "transfer_mode": "add"}),
            48,
        ),
        (
            "4_snow",
            json!({"emitter_type": "box", "emitter_position": [50, -5], "emitter_size_x": 600, "emitter_size_y": 10, "emitter_size_z": 200, "velocity": 20,
                "gravity": 30, "wind_x": 20, "turbulence": 40, "turbulence_scale": 80, "particles_per_second": 80, "life": 6, "size": 4, "size_random": 50,
                "sphere_feather": 100, "opacity_random": 40}),
            72,
        ),
        (
            "5_streaks_outwards",
            json!({"emitter_type": "sphere", "emitter_size_x": 100, "emitter_size_y": 100, "emitter_size_z": 100, "direction": "outwards", "velocity": 400,
                "particles_per_second": 400, "life": 1, "particle_type": "streak", "size": 2, "color": "#a0e0ff", "transfer_mode": "add"}),
            24,
        ),
        (
            "6_grid_of_cloudlets",
            json!({"emitter_type": "grid", "emitter_size_x": 400, "emitter_size_y": 200, "grid_x": 8, "grid_y": 5, "velocity": 10, "particles_per_second": 40,
                "life": 3, "particle_type": "cloudlet", "size": 30, "opacity": 50, "color": "#ffd0a0"}),
            36,
        ),
        (
            "7_stars_turned",
            json!({"emitter_type": "box", "emitter_size_x": 300, "emitter_size_y": 150, "emitter_rotation_z": 30, "velocity": 30, "particles_per_second": 30,
                "life": 3, "particle_type": "star", "size": 12, "opacity_over_life_1": 0, "opacity_over_life_5": 0, "color": "#fff0a0", "transfer_mode": "screen"}),
            48,
        ),
    ] {
        let (bytes, said) = picture(json!([fx("pic", &p)]), frame);
        png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
        t.row(
            &format!("{name}.png, frame {frame}: drawn without a warning, the street changed"),
            &format!("{} pixels changed; warnings {said:?}", differing(&bytes, &street)),
            said.is_empty() && differing(&bytes, &street) > 0,
        );
    }
    t.heading("On the card (D-617)");
    let mut gpu = Gpu::new().expect("a graphics card");
    let mine: fn(&Effect) -> bool = |e| matches!(e, Effect::ParticleEmitter { .. });
    t.card_fixtures(&mut gpu, "expected_particle_emitter.json", &(52..=63).collect::<Vec<_>>(), mine);
    let shots: Vec<(&str, Project, usize)> = [
        ("Particle Emitter as added", json!({})),
        ("glowing spheres added, turbulence", json!({"particles_per_second": 300, "particle_type": "glow_sphere", "size": 10, "transfer_mode": "add", "turbulence": 60})),
        ("stars screened from a box", json!({"emitter_type": "box", "emitter_size_x": 600, "particles_per_second": 200, "particle_type": "star", "size": 14, "transfer_mode": "screen"})),
        ("cloudlets bouncing on a floor", json!({"particles_per_second": 200, "particle_type": "cloudlet", "size": 20, "gravity": 300, "floor_position": 80, "floor_action": "bounce"})),
        ("streaks from a ball", json!({"emitter_type": "sphere", "particles_per_second": 400, "particle_type": "streak", "velocity": 300, "size": 3})),
    ]
    .into_iter()
    .map(|(what, p)| (what, reference(json!([fx("card", &p)])), 1))
    .collect();
    t.card_reference(&mut gpu, &reference(json!([])), &shots, mine);
    t.finish("D-615_particle_emitter_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The reference shot (1920 by 1080) with `stack` on its first layer.
fn reference(stack: J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    j["compositions"][0]["layers"][0]["effects"] = stack;
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// The frame times, B-493's way: the reference shot (1920 by 1080, 24 a second) with Particle
/// Emitter on its first layer, frames 0, 60, 120 and 239 each drawn with empty caches, three
/// times; the median of the three. A later frame steps every particle further from its birth, so
/// it costs more. D-617: each shot drawn by the card and by the processor, the card after one
/// frame unmeasured so its passes are built.
#[test]
#[ignore = "B-495: a measurement, run deliberately with --release --ignored"]
fn b495_particle_emitter_timing() {
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: the card and the processor (D-617)\n\n\
         | Shot | Drawn by | Frame 0 | Frame 60 | Frame 120 | Frame 239 |\n|---|---|---:|---:|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
    let shots: [(&str, Option<J>); 4] = [
        ("no effect", None),
        ("Particle Emitter as added", Some(json!({}))),
        ("1000 glowing spheres a second, added", Some(json!({"particles_per_second": 1000, "particle_type": "glow_sphere", "size": 8, "transfer_mode": "add"}))),
        (
            "2000 sparks a second living 3 s, turbulence, bouncing on a floor",
            Some(json!({"particles_per_second": 2000, "gravity": 400, "turbulence": 100, "floor_position": 90, "floor_action": "bounce", "size": 3})),
        ),
    ];
    for (name, p) in shots {
        let project = reference(p.map_or(json!([]), |p| json!([fx("timing", &p)])));
        let mut card = |frame| {
            let mut log = FrameLog::new(3);
            gpu.forget();
            let t = std::time::Instant::now();
            drop(
                preview::preview_frame_srgb8(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer(), &mut gpu)
                    .expect("GPU frame"),
            );
            let ms = t.elapsed().as_secs_f64() * 1000.0;
            assert!(!log.finish().iter().any(|d| d.id == DiagnosticId::GpuPreviewOnCpu), "the card refused {name}");
            ms
        };
        card(0);
        let _ = write!(s, "| {name} | card |");
        for frame in [0, 60, 120, 239] {
            let _ = write!(s, " {:.1} ms |", median((0..3).map(|_| card(frame)).collect()));
        }
        let _ = write!(s, "\n| {name} | processor |");
        for frame in [0, 60, 120, 239] {
            let times: Vec<f64> = (0..3)
                .map(|_| {
                    let mut log = FrameLog::new(3);
                    let t = std::time::Instant::now();
                    drop(
                        preview::preview_frame_cached(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                            .expect("CPU frame"),
                    );
                    t.elapsed().as_secs_f64() * 1000.0
                })
                .collect();
            let _ = write!(s, " {:.1} ms |", median(times));
        }
        s.push('\n');
    }
    let out = std::env::var("B495_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-495_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
