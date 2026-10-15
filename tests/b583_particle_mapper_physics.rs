//! B-583: D-713, Particle Playground's further mapper properties (P0-6, EFFECTS.md row 364),
//! after what After Effects' property mappers can set: X and Y Scale, Angular Velocity, Torque,
//! Lifespan and Kinetic and Static Friction. Adobe publishes no formula: the rules are ours,
//! stated in `tools/particle_mapper_physics_reference.py`.
//!
//! Writes `verification/D-713_particle_mapper_physics_table.md` and draws pictures into
//! `verification/D-713 pictures/`.
//!
//! Every expected pixel is
//! `Fixtures/particle_mapper_physics/expected_particle_mapper_physics.json` and every expected
//! particle `Fixtures/particle_mapper_physics/particles_particle_mapper_physics.json`, both
//! written by the tool before this code existed and printed in document 25 as FX-PMPH-001 to 028.
//! Tolerances 2e-5 and 1e-7. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::sync::Arc;

use effect_table::{median, repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{
    Effect, PARTICLE_PLAYGROUND_COLORS, PARTICLE_PLAYGROUND_LAYERS, PARTICLE_PLAYGROUND_SETTINGS,
    PARTICLE_PLAYGROUND_WORDS,
};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, playground, png_out, OutputDepth};

/// The pixels that differ between two straight 8-bit pictures.
fn differing(a: &[u8], b: &[u8]) -> usize {
    a.chunks_exact(4)
        .zip(b.chunks_exact(4))
        .filter(|(p, q)| p != q)
        .count()
}

/// Why the `holder` layer's effect in `file` is left out, its keys too.
fn fault_of(t: &Table, file: &str) -> String {
    let d = t.load(file).document;
    d.project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layer(&Id::new("holder"))
        .unwrap()
        .effects[0]
        .fault()
        .unwrap_or_default()
}

/// Every particle in `file` at `frame`, each [code, a, b, born, x, y, vx, vy], given what compose
/// gives the effect, as B-494's test: the clock, the keyed settings, each layer read through
/// D-189's map (the Explode Layer at the in point, the others at the frame) and the mask Wall
/// Mask names.
fn listed(t: &Table, file: &str, frame: i32) -> Vec<[f64; 8]> {
    let d = t.load(file).document;
    let project = d.project();
    let (comp_id, holder) = (Id::new(MAIN), Id::new("holder"));
    let comp = project.composition(&comp_id).unwrap();
    let layer = comp.layer(&holder).unwrap();
    let instance = &layer.effects[0];
    let fps = comp.frame_rate.numerator() as f64 / comp.frame_rate.denominator() as f64;
    let mut e = instance.at(frame).effect;
    let Effect::ParticlePlayground {
        values,
        layers,
        clock,
        history,
        ..
    } = &mut e
    else {
        unreachable!()
    };
    *clock = [(frame - layer.in_frame) as f64, fps];
    if !instance.tracks.is_empty() {
        *history = Some(Arc::new(
            (layer.in_frame..=frame + 1)
                .map(|f| playground::numbers(&instance.at(f).effect))
                .collect(),
        ));
    }
    let mut log = FrameLog::new(3);
    let maps: Vec<_> = layers
        .iter()
        .enumerate()
        .map(|(k, named)| {
            let at = if k == 0 { layer.in_frame } else { frame };
            compose::layer_map(
                project,
                &comp_id,
                &holder,
                named.as_str().unwrap(),
                "center",
                at,
                &t.root,
                PreviewQuality::Full,
                &mut log,
            )
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
/// street with nothing on it (`under`, the mappers' map), drawn on the processor, straight
/// 8-bit, with what it warned of. The layer is `art`.
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
    let mut under = layer.clone();
    layer["effects"] = effects;
    under["id"] = J::from("under");
    under["name"] = J::from("under");
    under["effects"] = json!([]);
    comp["layers"].as_array_mut().unwrap().push(under);
    comp["layer_order"]
        .as_array_mut()
        .unwrap()
        .insert(0, J::from("under"));
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
fn b583_particle_mapper_physics() {
    let mut t = Table::new(
        "particle_mapper_physics",
        "# D-713: Particle Playground's further mapper properties\n\nB-583 (P0-6, EFFECTS.md row \
         364): the property mappers can now set X and Y Scale (a particle drawn stretched along \
         its own axes), Angular Velocity and Torque (degrees a second, and a second a second), \
         Lifespan (seconds from its own birth), and Kinetic and Static Friction (pixels a second \
         a second: kinetic slows a moving particle, static holds a still one while the pull on it \
         is no more). Mass came later (D-715). Gradient Velocity, Character and Font Size are not built and are \
         refused by name. A file written before D-713 is the effect it was. The card draws within \
         1 level of the processor. Adobe publishes no formula, so the rules are ours, stated in \
         `tools/particle_mapper_physics_reference.py`. Every expected pixel is \
         `Fixtures/particle_mapper_physics/expected_particle_mapper_physics.json` and every \
         expected particle `Fixtures/particle_mapper_physics/particles_particle_mapper_physics.json`, \
         written by the tool before this code existed and printed in document 25 as FX-PMPH-001 \
         to 028. Tolerances 2e-5 and 1e-7.\n",
    );

    t.heading("FX-PMPH-001 to 028 (document 25)");
    t.fixtures("expected_particle_mapper_physics.json");

    t.heading("The particles themselves: where each is and how fast it goes");
    let expected: J = serde_json::from_str(
        &fs::read_to_string(t.root.join("particles_particle_mapper_physics.json")).unwrap(),
    )
    .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let key = |a: &[f64], b: &[f64]| {
        a[..4]
            .iter()
            .zip(&b[..4])
            .map(|(x, y)| x.total_cmp(y))
            .find(|o| o.is_ne())
            .unwrap_or(std::cmp::Ordering::Equal)
    };
    for (name, case) in expected["cases"].as_object().unwrap() {
        let n: u32 = name.rsplit('-').next().unwrap().parse().unwrap();
        let file = format!("fx_pmph_{n:03}.json");
        let frame = case["frame"].as_i64().unwrap() as i32;
        let mut want: Vec<Vec<f64>> = case["particles"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| {
                p.as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_f64().unwrap())
                    .collect()
            })
            .collect();
        let mut got = listed(&t, &file, frame);
        want.sort_by(|a, b| key(a, b));
        got.sort_by(|a, b| key(a, b));
        let largest = got
            .iter()
            .zip(&want)
            .flat_map(|(g, w)| g.iter().zip(w).map(|(a, b)| (a - b).abs()))
            .fold(0.0, f64::max);
        t.row(
            &format!("{name} frame {frame}: the {} particles, each its emitter, numbers, birth, place and speed", want.len()),
            &format!("{} listed, largest difference {largest:.1e}", got.len()),
            got.len() == want.len() && largest <= tolerance,
        );
    }

    t.heading("The file");
    let files: Vec<String> = (1..=28).map(|n| format!("fx_pmph_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    for (n, want) in [
        (26, "gradient_velocity"),
        (27, "character"),
        (28, "font_size"),
    ] {
        let file = format!("fx_pmph_{n:03}.json");
        let why = fault_of(&t, &file);
        t.row(
            &format!("{file} is refused in a sentence naming {want}"),
            &why,
            why.to_lowercase().contains(want),
        );
    }

    t.heading("Tiles");
    t.tiles(&[
        ("fx_pmph_002.json", 6),
        ("fx_pmph_012.json", 6),
        ("fx_pmph_017.json", 6),
        ("fx_pmph_010.json", 36),
    ]);

    t.heading("Pictures (verification/D-713 pictures)");
    let out = repo("verification/D-713 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0).0;
    let grid = json!({"particles_per_second": 0, "gravity_force": 0, "particles_across": 24, "particles_down": 14, "grid_width": 480,
        "grid_height": 270, "grid_particle_radius": 5, "grid_color": "#ffffff", "persistent_map_layer": "under"});
    let fountain = json!({"cannon_position": [50, 95], "particles_per_second": 80, "velocity": 260, "direction_random_spread": 50,
        "cannon_particle_radius": 3, "gravity_force": 200, "cannon_color": "#ffffff", "persistent_map_layer": "under", "ephemeral_map_layer": "under"});
    let with = |base: &J, more: J| {
        let mut p = base.clone();
        for (k, v) in more.as_object().unwrap() {
            p[k] = v.clone();
        }
        p
    };
    for (name, p, frame) in [
        (
            "1_dots_stretched_and_turned_by_the_street",
            with(
                &grid,
                json!({"persistent_map_red_to": "x_scale", "persistent_min_red": 0.5, "persistent_max_red": 3,
                "persistent_map_green_to": "angle", "persistent_min_green": 0, "persistent_max_green": 90}),
            ),
            0,
        ),
        (
            "2_fountain_short_lived_where_dark",
            with(
                &fountain,
                json!({"persistent_map_red_to": "lifespan", "persistent_min_red": 0.1, "persistent_max_red": 1.5}),
            ),
            48,
        ),
        (
            "3_fountain_with_kinetic_friction",
            with(
                &fountain,
                json!({"persistent_map_red_to": "kinetic_friction", "persistent_min_red": 150, "persistent_max_red": 150}),
            ),
            48,
        ),
        (
            "4_grid_held_except_on_the_dark_road",
            with(
                &grid,
                json!({"gravity_force": 300, "ephemeral_map_layer": "under", "ephemeral_map_red_to": "static_friction",
                "ephemeral_min_red": 0, "ephemeral_max_red": 1000}),
            ),
            24,
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
                differing(&bytes, &street)
            ),
            said.is_empty() && differing(&bytes, &street) > 0,
        );
    }
    t.heading("On the card");
    let mut gpu = Gpu::new().expect("a graphics card");
    let mine: fn(&Effect) -> bool = |e| matches!(e, Effect::ParticlePlayground { .. });
    t.card_fixtures(
        &mut gpu,
        "expected_particle_mapper_physics.json",
        &(26..=28).collect::<Vec<_>>(),
        mine,
    );
    t.finish("D-713_particle_mapper_physics_table.md");
}

/// The reference shot (1920 by 1080) with `stack` on its first layer.
fn reference(stack: J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json"))
        .expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    j["compositions"][0]["layers"][0]["effects"] = stack;
    persist::load_str(&j.to_string())
        .unwrap_or_else(|d| panic!("the reference shot: {}", d.message))
        .document
        .project()
        .clone()
}

/// The frame times, B-510's way, as B-494's: the reference shot (1920 by 1080, 24 a second)
/// with Particle Playground on its first layer, frames 0, 60, 120 and 239 each drawn on the
/// processor with empty caches, three times; the median of the three.
#[test]
#[ignore = "B-583: a measurement, run deliberately with --release --ignored"]
fn b583_particle_mapper_physics_timing() {
    use std::fmt::Write as _;

    let mut s = format!(
        "- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: the processor (Particle Playground has no card path)\n\n\
         | Shot | Frame 0 | Frame 60 | Frame 120 | Frame 239 |\n|---|---:|---:|---:|---:|\n",
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let (comp, root) = (
        Id::new("comp-reference-shot"),
        repo("Fixtures/reference_shot"),
    );
    let shots: [(&str, Option<J>); 3] = [
        ("no effect", None),
        ("Particle Playground as added (no mapper)", Some(json!({}))),
        (
            "as added, both mappers reading the layer: red to X Scale and green to Lifespan, kept; blue to Static Friction, for the moment",
            Some(json!({"persistent_map_layer": "a", "persistent_map_red_to": "x_scale", "persistent_max_red": 3,
                "persistent_map_green_to": "lifespan", "persistent_max_green": 2, "ephemeral_map_layer": "a",
                "ephemeral_map_blue_to": "static_friction", "ephemeral_max_blue": 100})),
        ),
    ];
    for (name, p) in shots {
        let project = reference(p.map_or(json!([]), |p| json!([fx("timing", &p)])));
        let _ = write!(s, "| {name} |");
        for frame in [0, 60, 120, 239] {
            let times: Vec<f64> = (0..3)
                .map(|_| {
                    let mut log = FrameLog::new(3);
                    let t = std::time::Instant::now();
                    drop(
                        preview::preview_frame_cached(
                            &project,
                            &comp,
                            frame,
                            &root,
                            PreviewQuality::Full,
                            DEFAULT_TILE_SIZE,
                            &mut log,
                            &mut CelCache::viewer(),
                        )
                        .expect("CPU frame"),
                    );
                    t.elapsed().as_secs_f64() * 1000.0
                })
                .collect();
            let _ = write!(s, " {:.1} ms |", median(times));
        }
        s.push('\n');
    }
    let out = std::env::var("B583_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-583_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
