//! B-493: D-613, Particle Playground, the particle engine's third effect (P0-6), after what After
//! Effects' Particle Playground (Simulation) is for: dots shot from a Cannon or laid out on a
//! Grid, a layer cut into flying pieces by the Layer Exploder, any of them split again by the
//! Particle Exploder, shown as a layer by the Layer Map, pulled by Gravity and kept inside a mask
//! by the Wall, each force taking only the particles its Affects choose. Adobe publishes no
//! formula: the rules and the settings when added are ours, stated in
//! `tools/particle_playground_reference.py`.
//!
//! Writes `verification/D-613_particle_playground_table.md` and draws pictures into
//! `verification/D-613 pictures/`.
//!
//! Every expected pixel is `Fixtures/particle_playground/expected_particle_playground.json` and
//! every expected particle `Fixtures/particle_playground/particles_particle_playground.json`,
//! both written by the tool before this code existed and printed in document 25 as FX-PPG-001 to
//! 060. Tolerances 2e-5 and 1e-7. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::sync::Arc;

use effect_table::{repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{
    Effect, PARTICLE_PLAYGROUND_COLORS, PARTICLE_PLAYGROUND_LAYERS, PARTICLE_PLAYGROUND_SETTINGS, PARTICLE_PLAYGROUND_WORDS,
};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, playground, png_out, OutputDepth};

/// The pixels that differ between two straight 8-bit pictures.
fn differing(a: &[u8], b: &[u8]) -> usize {
    a.chunks_exact(4).zip(b.chunks_exact(4)).filter(|(p, q)| p != q).count()
}

/// Why the `holder` layer's effect in `file` is left out, its keys too.
fn fault_of(t: &Table, file: &str) -> String {
    let d = t.load(file).document;
    d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].fault().unwrap_or_default()
}

/// Every particle in `file` at `frame`, each [code, a, b, born, x, y, vx, vy], given what compose
/// gives the effect: the clock, the keyed settings, each layer read through D-189's map (the
/// Explode Layer at the in point, the others at the frame) and the mask Wall Mask names.
fn listed(t: &Table, file: &str, frame: i32) -> Vec<[f64; 8]> {
    let d = t.load(file).document;
    let project = d.project();
    let (comp_id, holder) = (Id::new(MAIN), Id::new("holder"));
    let comp = project.composition(&comp_id).unwrap();
    let layer = comp.layer(&holder).unwrap();
    let instance = &layer.effects[0];
    let fps = comp.frame_rate.numerator() as f64 / comp.frame_rate.denominator() as f64;
    let mut e = instance.at(frame).effect;
    let Effect::ParticlePlayground { values, layers, clock, history, .. } = &mut e else { unreachable!() };
    *clock = [(frame - layer.in_frame) as f64, fps];
    if !instance.tracks.is_empty() {
        *history = Some(Arc::new((layer.in_frame..=frame + 1).map(|f| playground::numbers(&instance.at(f).effect)).collect()));
    }
    let mut log = FrameLog::new(3);
    let maps: Vec<_> = layers
        .iter()
        .enumerate()
        .map(|(k, named)| {
            let at = if k == 0 { layer.in_frame } else { frame };
            compose::layer_map(project, &comp_id, &holder, named.as_str().unwrap(), "center", at, &t.root, PreviewQuality::Full, &mut log)
        })
        .collect();
    let n = values[playground::WALL_MASK].floor();
    let wall: Option<Vec<[f64; 2]>> = (n >= 1.0)
        .then(|| layer.masks.get(n as usize - 1).map(|m| m.at(frame)))
        .flatten()
        .filter(|m| m.enabled && m.points.len() >= 3)
        .map(|m| m.outline().into_iter().map(|(x, y)| [x, y]).collect());
    let maps: [_; PARTICLE_PLAYGROUND_LAYERS.len()] = std::array::from_fn(|k| maps[k].as_ref());
    playground::listed(&e, maps, wall.as_deref(), (16.0, 10.0))
}

/// The street still (480 by 270) with `effects` on it at `frame`, over a second copy of the
/// street with nothing on it, its layer carrying `masks`, drawn on the processor, straight 8-bit,
/// with what it warned of. The layer is `art`.
fn picture(effects: J, masks: J, frame: i32) -> (Vec<u8>, Vec<String>) {
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
    layer["masks"] = masks;
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

/// A Particle Playground's settings, every one `p` leaves out as it is added.
fn parameters(p: &J) -> J {
    let mut all = json!({"cannon_position": [50, 50], "grid_position": [50, 50]});
    for (name, _, _, start) in PARTICLE_PLAYGROUND_SETTINGS {
        all[name] = J::from(start);
    }
    for (name, _, start) in PARTICLE_PLAYGROUND_WORDS {
        all[name] = J::from(start);
    }
    for name in PARTICLE_PLAYGROUND_LAYERS {
        all[name] = J::from("");
    }
    for (name, start) in PARTICLE_PLAYGROUND_COLORS {
        all[name] = J::from(start);
    }
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    all
}

fn fx(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.particle_playground", "enabled": true, "parameters": parameters(p)})
}

#[test]
fn b493_particle_playground() {
    let mut t = Table::new(
        "particle_playground",
        "# D-613: Particle Playground\n\nB-493, the particle engine's third effect (P0-6), after \
         what After Effects' Particle Playground (Simulation) is for: dots shot from a Cannon or \
         laid out on a Grid, a layer cut into flying pieces by the Layer Exploder, any of them \
         split again by the Particle Exploder, shown as a layer by the Layer Map, pulled by \
         Gravity and kept inside a mask by the Wall, each force taking only the particles its \
         Affects choose. Nothing is carried from frame to frame: every frame works each particle \
         out from its birth (document 20), so a force that would need the frame before (Repel, \
         and the maps and walls read as they move) is a gap, logged in the playtest. The \
         particles are worked out on the processor and drawn on the card too (D-619), within 1 \
         level of the processor. Adobe publishes no formula, so the rules and the settings when \
         added are ours, stated in `tools/particle_playground_reference.py`. Every expected \
         pixel is `Fixtures/particle_playground/expected_particle_playground.json` and every \
         expected particle `Fixtures/particle_playground/particles_particle_playground.json`, \
         written by the tool before this code existed and printed in document 25 as FX-PPG-001 \
         to 060. Tolerances 2e-5 and 1e-7.\n",
    );

    t.heading("FX-PPG-001 to 060 (document 25)");
    t.fixtures("expected_particle_playground.json");

    t.heading("The particles themselves: where each is and how fast it goes");
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("particles_particle_playground.json")).unwrap()).unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let key = |a: &[f64], b: &[f64]| a[..4].iter().zip(&b[..4]).map(|(x, y)| x.total_cmp(y)).find(|o| o.is_ne()).unwrap_or(std::cmp::Ordering::Equal);
    for (name, case) in expected["cases"].as_object().unwrap() {
        let n: u32 = name.rsplit('-').next().unwrap().parse().unwrap();
        let file = format!("fx_ppg_{n:03}.json");
        let frame = case["frame"].as_i64().unwrap() as i32;
        let mut want: Vec<Vec<f64>> = case["particles"].as_array().unwrap().iter().map(|p| p.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()).collect();
        let mut got = listed(&t, &file, frame);
        want.sort_by(|a, b| key(a, b));
        got.sort_by(|a, b| key(a, b));
        let largest = got.iter().zip(&want).flat_map(|(g, w)| g.iter().zip(w).map(|(a, b)| (a - b).abs())).fold(0.0, f64::max);
        t.row(
            &format!("{name} frame {frame}: the {} particles, each its emitter, numbers, birth, place and speed", want.len()),
            &format!("{} listed, largest difference {largest:.1e}", got.len()),
            got.len() == want.len() && largest <= tolerance,
        );
    }

    t.heading("The file");
    let files: Vec<String> = (1..=60).map(|n| format!("fx_ppg_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_ppg_029.json");
    let count = saved.as_object().unwrap().len();
    t.row(
        "fx_ppg_029.json is saved with its 44 settings (29 numbers, 2 positions, 5 words, 6 layers and 2 colours) and no maps, wall, clock or history",
        &format!("{count} settings; cannon position {}, explode layer {}, auto orient {}", saved["cannon_position"], saved["explode_layer"], saved["auto_orient"]),
        count == 44 && saved["cannon_position"].as_array().is_some_and(|a| a.len() == 2) && saved["explode_layer"].is_string() && saved["auto_orient"] == "off",
    );
    for (n, want) in [
        (49, "particles per second"),
        (50, "direction random spread"),
        (51, "particles across"),
        (52, "explode radius"),
        (53, "gravity older younger than"),
        (54, "wall mask"),
        (55, "gravity particles from"),
        (56, "auto orient"),
        (57, "cannon colour"),
        (58, "explode layer"),
        (59, "cannon position"),
        (60, "velocity"),
    ] {
        let file = format!("fx_ppg_{n:03}.json");
        let why = fault_of(&t, &file);
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }
    t.shape_refused("fx_ppg_001.json", "a Particle Playground whose velocity is a word", &parameters(&json!({"velocity": "fast"})).to_string());
    t.shape_refused("fx_ppg_001.json", "a Particle Playground whose auto-orient is a number", &parameters(&json!({"auto_orient": 1})).to_string());
    t.shape_refused("fx_ppg_001.json", "a Particle Playground without its wall selection map", &{
        let mut p = parameters(&json!({}));
        p.as_object_mut().unwrap().remove("wall_selection_map");
        p.to_string()
    });

    t.heading("Tiles");
    t.tiles(&[("fx_ppg_006.json", 24), ("fx_ppg_022.json", 12), ("fx_ppg_029.json", 30), ("fx_ppg_039.json", 36)]);

    t.heading("Pictures (verification/D-613 pictures)");
    let out = repo("verification/D-613 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), json!([]), 0).0;
    let corner = |x: f64, y: f64| json!({"point": [x, y], "in": [0, 0], "out": [0, 0]});
    let box_mask = json!([{"name": "Mask 1", "enabled": true, "inverted": false, "mode": "none", "opacity": 1.0, "feather_px": 0.0, "expansion_px": 0.0,
        "path": {"base": {"points": [corner(140.0, 60.0), corner(340.0, 60.0), corner(340.0, 230.0), corner(140.0, 230.0)]}, "keyframes": []}}]);
    for (name, p, masks, frame) in [
        ("1_as_added_frame_24", json!({}), json!([]), 24),
        (
            "2_fountain_and_grid",
            json!({"cannon_position": [50, 95], "particles_per_second": 60, "velocity": 260, "direction_random_spread": 30, "cannon_particle_radius": 2,
                "gravity_force": 220, "cannon_color": "#a0d8ff", "particles_across": 12, "particles_down": 3, "grid_width": 400, "grid_height": 60,
                "grid_position": [50, 20], "grid_particle_radius": 3, "gravity_particles_from": "cannon"}),
            json!([]),
            48,
        ),
        (
            "3_layer_exploded",
            json!({"particles_per_second": 0, "explode_layer": "art", "explode_radius": 6, "explode_dispersion": 80, "gravity_force": 60, "auto_orient": "on"}),
            json!([]),
            24,
        ),
        (
            "4_fireworks",
            json!({"cannon_position": [50, 100], "particles_per_second": 3, "velocity": 240, "direction_random_spread": 40, "cannon_particle_radius": 3,
                "cannon_color": "#ffe080", "gravity_force": 120, "particle_exploder_radius": 0.75, "particle_exploder_dispersion": 90,
                "particle_exploder_older_younger_than": 0.8}),
            json!([]),
            72,
        ),
        (
            "5_wall_box",
            json!({"particles_per_second": 30, "velocity": 160, "direction_random_spread": 360, "cannon_particle_radius": 2.5, "gravity_force": 150,
                "wall_mask": 1, "cannon_color": "#80ff80"}),
            box_mask,
            72,
        ),
    ] {
        let (bytes, said) = picture(json!([fx("pic", &p)]), masks, frame);
        png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
        t.row(
            &format!("{name}.png, frame {frame}: drawn without a warning, the street changed"),
            &format!("{} pixels changed; warnings {said:?}", differing(&bytes, &street)),
            said.is_empty() && differing(&bytes, &street) > 0,
        );
    }
    t.heading("On the card (D-619)");
    let mut gpu = Gpu::new().expect("a graphics card");
    let mine: fn(&Effect) -> bool = |e| matches!(e, Effect::ParticlePlayground { .. });
    t.card_fixtures(&mut gpu, "expected_particle_playground.json", &(49..=60).collect::<Vec<_>>(), mine);
    let shots: Vec<(&str, Project, usize)> = [
        ("Particle Playground as added", json!({})),
        (
            "a fountain and a grid of dots, falling",
            json!({"cannon_position": [50, 95], "particles_per_second": 60, "velocity": 400, "direction_random_spread": 30, "cannon_particle_radius": 4,
                "gravity_force": 220, "cannon_color": "#a0d8ff", "particles_across": 20, "particles_down": 5, "grid_width": 1200, "grid_height": 200,
                "grid_position": [50, 20], "grid_particle_radius": 5, "gravity_particles_from": "cannon"}),
        ),
        (
            "layer 2 exploded into turning pieces, falling",
            json!({"particles_per_second": 0, "explode_layer": "layer-2", "explode_radius": 8, "explode_dispersion": 80, "gravity_force": 60, "auto_orient": "on"}),
        ),
        (
            "fireworks bursting into sparks",
            json!({"cannon_position": [50, 100], "particles_per_second": 3, "velocity": 600, "direction_random_spread": 40, "cannon_particle_radius": 6,
                "cannon_color": "#ffe080", "gravity_force": 200, "particle_exploder_radius": 0.75, "particle_exploder_dispersion": 200,
                "particle_exploder_older_younger_than": 0.8}),
        ),
        (
            "copies of layer 2 shot from the cannon",
            json!({"particles_per_second": 4, "velocity": 300, "direction_random_spread": 90, "layer_map": "layer-2", "auto_orient": "on"}),
        ),
    ]
    .into_iter()
    .map(|(what, p)| (what, reference(json!([fx("card", &p)])), 1))
    .collect();
    t.card_reference(&mut gpu, &reference(json!([])), &shots, mine);
    t.finish("D-613_particle_playground_table.md");
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

/// The frame times, B-510's way: the reference shot (1920 by 1080, 24 a second) with Particle
/// Playground on its first layer, frames 0, 60, 120 and 239 each drawn with empty caches, three
/// times; the median of the three. A later frame steps every particle further from its birth, so
/// it costs more. D-619: each shot drawn by the card and by the processor, the card after one
/// frame unmeasured so its passes are built.
#[test]
#[ignore = "B-493: a measurement, run deliberately with --release --ignored"]
fn b493_particle_playground_timing() {
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: the card and the processor (D-619)\n\n\
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
        ("Particle Playground as added", Some(json!({}))),
        ("fireworks: 10 shells a second, each bursting into 16 sparks", Some(json!({"particles_per_second": 10, "particle_exploder_radius": 0.5, "particle_exploder_older_younger_than": 0.5}))),
        ("the layer exploded into 16-pixel pieces, falling", Some(json!({"particles_per_second": 0, "explode_layer": "layer-2", "explode_radius": 8, "explode_dispersion": 80, "gravity_force": 60}))),
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
    let out = std::env::var("B493_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-493_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
