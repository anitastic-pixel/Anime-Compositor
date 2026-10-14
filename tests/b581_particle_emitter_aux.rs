//! B-581: D-711, Particle Emitter's second system (EFFECTS.md row 367, P0-6): particles thrown by
//! particles, after what Trapcode Particular's auxiliary system is for: a trail of particles
//! dropped as each particle goes (Aux Emit Continuously), or sparks thrown where it bounces on the
//! floor (At Bounce), each thrown one with its own life, speed, size and opacity fading to its
//! at-death levels, coloured as its thrower was or its own colour. Red Giant publishes no formula
//! and no settings when added: the rules are ours, stated in
//! `tools/particle_emitter_aux_reference.py`.
//!
//! Writes `verification/D-711_particle_emitter_aux_table.md` and draws pictures into
//! `verification/D-711 pictures/`.
//!
//! Every expected pixel is `Fixtures/particle_emitter_aux/expected_particle_emitter_aux.json` and
//! every expected particle `Fixtures/particle_emitter_aux/particles_particle_emitter_aux.json`,
//! both written by the tool before this code existed, FX-PEAUX-001 to 030. Tolerances 2e-5 and
//! 1e-7. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::sync::Arc;

use effect_table::{median, repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{Effect, PARTICLE_EMITTER_COLORS, PARTICLE_EMITTER_SETTINGS, PARTICLE_EMITTER_WORDS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{particles, persist, png_out, OutputDepth};

fn differing(a: &[u8], b: &[u8]) -> usize {
    a.chunks_exact(4).zip(b.chunks_exact(4)).filter(|(p, q)| p != q).count()
}

fn fault_of(t: &Table, file: &str) -> String {
    let d = t.load(file).document;
    d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].fault().unwrap_or_default()
}

/// The particles alive in `file` at `frame`, throwers then aux particles, each [k, birth, x, y,
/// z, vx, vy, vz], given the clock and keyed settings as compose gives them, on the 16 by 10 layer.
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
/// street, drawn on the processor, straight 8-bit, with what it warned of.
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

const AUX: [&str; 13] = [
    "aux_particles_per_second",
    "aux_particles_per_bounce",
    "aux_life",
    "aux_velocity",
    "aux_inherit_velocity",
    "aux_size",
    "aux_size_at_death",
    "aux_opacity",
    "aux_opacity_at_death",
    "aux_emit",
    "aux_particle_type",
    "aux_set_color",
    "aux_color",
];

#[test]
fn b581_particle_emitter_aux() {
    let mut t = Table::new(
        "particle_emitter_aux",
        "# D-711: Particle Emitter's second system\n\nB-581 (P0-6, EFFECTS.md row 367): particles \
         thrown by particles, after what Trapcode Particular's auxiliary system is for. Aux Emit \
         Continuously drops a trail as each particle goes; At Bounce throws sparks where it \
         bounces on the floor. Each thrown particle has its own life, speed (every way, plus a \
         share of its thrower's), size and opacity going straight to their at-death levels, \
         coloured as its thrower was when thrown or its own colour, drawn as any of the five \
         kinds with the particles. Red Giant publishes no formula and no settings when added, so \
         the rules are ours, stated in `tools/particle_emitter_aux_reference.py`. A file written \
         before reads the second system as added, off, and is saved without it. Every expected \
         pixel is `Fixtures/particle_emitter_aux/expected_particle_emitter_aux.json` and every \
         expected particle `Fixtures/particle_emitter_aux/particles_particle_emitter_aux.json`, \
         written by the tool before this code existed, FX-PEAUX-001 to 030. Tolerances 2e-5 and \
         1e-7.\n",
    );

    t.heading("FX-PEAUX-001 to 030");
    t.fixtures("expected_particle_emitter_aux.json");

    t.heading("The particles themselves: throwers and thrown, where each is and how fast it goes");
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("particles_particle_emitter_aux.json")).unwrap()).unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    for (name, case) in expected["cases"].as_object().unwrap() {
        let n: u32 = name.rsplit('-').next().unwrap().parse().unwrap();
        let file = format!("fx_peaux_{n:03}.json");
        let frame = case["frame"].as_i64().unwrap() as i32;
        let mut want: Vec<Vec<f64>> = case["particles"].as_array().unwrap().iter().map(|p| p.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()).collect();
        let mut got = alive(&t, &file, frame);
        want.sort_by(|a, b| a[0].total_cmp(&b[0]));
        got.sort_by(|a, b| a[0].total_cmp(&b[0]));
        let thrown = want.iter().filter(|p| p[0] < 0.0).count();
        let largest = got.iter().zip(&want).flat_map(|(g, w)| g.iter().zip(w).map(|(a, b)| (a - b).abs())).fold(0.0, f64::max);
        t.row(
            &format!("{name} frame {frame}: the {} particles alive ({thrown} thrown), each its number, birth, place and speed", want.len()),
            &format!("{} alive, largest difference {largest:.1e}", got.len()),
            got.len() == want.len() && largest <= tolerance,
        );
    }

    t.heading("The file");
    let files: Vec<String> = (1..=30).map(|n| format!("fx_peaux_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_peaux_010.json");
    t.row(
        "fx_peaux_010.json is saved with its 71 settings (the 58 before and the second system's 13), aux emit at_bounce",
        &format!("{} settings; aux emit {}, aux particles/bounce {}", saved.as_object().unwrap().len(), saved["aux_emit"], saved["aux_particles_per_bounce"]),
        saved.as_object().unwrap().len() == 71 && saved["aux_emit"] == "at_bounce" && saved["aux_particles_per_bounce"] == 4.0,
    );
    let older = t.load("fx_peaux_002.json");
    let mut older_json = effect_table::saved(&older);
    let params = &mut older_json["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    for name in AUX {
        params.as_object_mut().unwrap().remove(name);
    }
    let reread = persist::load_str(&older_json.to_string()).expect("a D-615 file reads");
    let back = effect_table::saved(&reread);
    let p = &back["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    let e = reread.document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    let as_added = matches!(&e, Effect::ParticleEmitter { values, words, colors, .. }
        if values[45..].iter().zip(&PARTICLE_EMITTER_SETTINGS[45..]).all(|(v, s)| *v == s.3) && words[6] == "off" && words[7] == "sphere" && words[8] == "from_main" && colors[6] == "#ffffff");
    t.row(
        "a file written before D-711, without the second system, reads it as added (off) and is saved without it",
        &format!("as added: {as_added}; saved with {} settings", p.as_object().unwrap().len()),
        as_added && p.as_object().unwrap().len() == 58,
    );
    for (n, want) in [
        (23, "aux emit"),
        (24, "aux particle type"),
        (25, "aux set color"),
        (26, "aux colour"),
        (27, "aux particles per second"),
        (28, "aux life"),
        (29, "aux inherit velocity"),
        (30, "aux opacity at death"),
    ] {
        let file = format!("fx_peaux_{n:03}.json");
        let why = fault_of(&t, &file);
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }
    t.shape_refused("fx_peaux_002.json", "a Particle Emitter whose aux life is a word", &parameters(&json!({"aux_life": "long"})).to_string());

    t.heading("Tiles");
    t.tiles(&[("fx_peaux_004.json", 20), ("fx_peaux_010.json", 24), ("fx_peaux_022.json", 12)]);

    t.heading("Pictures (verification/D-711 pictures)");
    let out = repo("verification/D-711 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0).0;
    for (name, p, frame) in [
        (
            "1_comet_trails",
            json!({"emitter_position": [10, 50], "direction": "directional", "emitter_rotation_z": 80, "direction_spread": 25, "velocity": 180, "velocity_random": 30,
                "velocity_from_motion": 0, "particles_per_second": 6, "life": 3, "particle_type": "glow_sphere", "size": 14, "color": "#ffe0a0", "transfer_mode": "add",
                "aux_emit": "continuously", "aux_particles_per_second": 60, "aux_life": 0.8, "aux_velocity": 15, "aux_size": 5, "aux_particle_type": "glow_sphere"}),
            48,
        ),
        (
            "2_sparks_at_the_floor",
            json!({"emitter_position": [50, 20], "velocity": 120, "velocity_from_motion": 0, "gravity": 500, "floor_position": 85, "floor_action": "bounce",
                "particles_per_second": 40, "life": 2.5, "size": 6, "color": "#a0d0ff",
                "aux_emit": "at_bounce", "aux_particles_per_bounce": 12, "aux_life": 0.5, "aux_velocity": 150, "aux_size": 3,
                "aux_particle_type": "streak", "aux_set_color": "own", "aux_color": "#ffb040"}),
            48,
        ),
        (
            "3_fireworks_fading_trails",
            json!({"emitter_position": [50, 45], "velocity": 200, "velocity_random": 20, "velocity_from_motion": 0, "gravity": 60, "particles_per_second": 120, "life": 1.2,
                "size": 4, "set_color": "over_life", "transfer_mode": "add",
                "aux_emit": "continuously", "aux_particles_per_second": 30, "aux_life": 0.6, "aux_velocity": 0, "aux_size": 3, "aux_size_at_death": 20, "aux_opacity": 70}),
            30,
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

    t.heading("On the card (D-711)");
    let mut gpu = Gpu::new().expect("a graphics card");
    let mine: fn(&Effect) -> bool = |e| matches!(e, Effect::ParticleEmitter { .. });
    t.card_fixtures(&mut gpu, "expected_particle_emitter_aux.json", &(23..=30).collect::<Vec<_>>(), mine);
    let shots: Vec<(&str, Project, usize)> = [
        ("trails of glowing dots, added", json!({"particles_per_second": 30, "transfer_mode": "add", "aux_emit": "continuously", "aux_particles_per_second": 20, "aux_particle_type": "glow_sphere", "aux_size": 4})),
        (
            "sparks thrown at a floor, own colour, streaks",
            json!({"particles_per_second": 60, "gravity": 300, "floor_position": 80, "floor_action": "bounce", "aux_emit": "at_bounce", "aux_velocity": 100,
                "aux_particle_type": "streak", "aux_set_color": "own", "aux_color": "#ff8000"}),
        ),
    ]
    .into_iter()
    .map(|(what, p)| (what, reference(json!([fx("card", &p)])), 1))
    .collect();
    t.card_reference(&mut gpu, &reference(json!([])), &shots, mine);
    t.finish("D-711_particle_emitter_aux_table.md");
}

/// The reference shot (1920 by 1080) with `stack` on its first layer.
fn reference(stack: J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    j["compositions"][0]["layers"][0]["effects"] = stack;
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// The frame times, B-495's way: the reference shot with Particle Emitter on its first layer,
/// frames 0, 60, 120 and 239 each drawn with empty caches, three times, the median; by the card
/// and by the processor.
#[test]
#[ignore = "B-581: a measurement, run deliberately with --release --ignored"]
fn b581_particle_emitter_aux_timing() {
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: the card and the processor\n\n\
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
        ("Particle Emitter as added (second system off)", Some(json!({}))),
        ("as added, each trailing 30 a second living 1 s", Some(json!({"aux_emit": "continuously"}))),
        (
            "200 a second bouncing on a floor, 10 sparks a bounce",
            Some(json!({"particles_per_second": 200, "gravity": 400, "floor_position": 90, "floor_action": "bounce", "aux_emit": "at_bounce", "aux_velocity": 100})),
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
    let out = std::env::var("B581_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-581_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
