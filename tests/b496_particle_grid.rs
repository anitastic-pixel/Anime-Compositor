//! B-496: D-616, Particle Grid, the particle engine's fifth effect (P0-6, PLUGINS.md's parked
//! pick), after what Trapcode Form is for: particles that stay put, in a grid of points or on
//! layered spheres in 3D, turned and moved as one, each point nudged its own way, the whole
//! twisted and rippled by a noise field that can drift and change, coloured, sized and pushed
//! towards the camera by other layers; drawn as Particle Emitter's soft or glowing spheres, stars
//! or faded clouds. Red Giant publishes no formula and no settings when added: the rules, those
//! settings and the name are ours, stated in `tools/particle_grid_reference.py`.
//!
//! Writes `verification/D-616_particle_grid_table.md` and draws pictures into
//! `verification/D-616 pictures/`.
//!
//! Every expected pixel is `Fixtures/particle_grid/expected_particle_grid.json` and every
//! expected point `Fixtures/particle_grid/particles_particle_grid.json`, both written by the tool
//! before this code existed and printed in document 25 as FX-PGRID-001 to 060. Tolerances 2e-5
//! and 1e-7. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::sync::Arc;

use effect_table::{repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, PARTICLE_GRID_COLORS, PARTICLE_GRID_LAYERS, PARTICLE_GRID_SETTINGS, PARTICLE_GRID_WORDS};
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

/// The points in `file` at `frame`, each [j, x, y, z, size, opacity], given what compose gives
/// the effect: the clock, the keyed settings, and each layer read through D-189's map at the
/// frame, stretched to the fixtures' 16 by 10 layer.
fn alive(t: &Table, file: &str, frame: i32) -> Vec<[f64; 6]> {
    let d = t.load(file).document;
    let project = d.project();
    let (comp_id, holder) = (Id::new(MAIN), Id::new("art"));
    let comp = project.composition(&comp_id).unwrap();
    let layer = comp.layer(&holder).unwrap();
    let instance = &layer.effects[0];
    let fps = comp.frame_rate.numerator() as f64 / comp.frame_rate.denominator() as f64;
    let mut e = instance.at(frame).effect;
    let Effect::ParticleGrid { layers, clock, history, .. } = &mut e else { unreachable!() };
    *clock = [(frame - layer.in_frame) as f64, fps];
    if !instance.tracks.is_empty() {
        *history = Some(Arc::new((layer.in_frame..=frame + 1).map(|f| particles::grid_numbers_row(&instance.at(f).effect)).collect()));
    }
    let mut log = FrameLog::new(3);
    let maps: Vec<_> = layers
        .iter()
        .map(|named| compose::layer_map(project, &comp_id, &holder, named.as_str().unwrap(), "stretch", frame, &t.root, PreviewQuality::Full, &mut log))
        .collect();
    let maps: [_; PARTICLE_GRID_LAYERS.len()] = std::array::from_fn(|k| maps[k].as_ref());
    particles::grid_alive(&e, maps, 16.0, 10.0)
}

/// The street still (480 by 270) with `effects` on it at `frame`, over a second copy of the
/// street with nothing on it (`under`, the pictures' map), drawn on the processor, straight
/// 8-bit, with what it warned of.
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

/// A Particle Grid's settings, every one `p` leaves out as it is added.
fn parameters(p: &J) -> J {
    let mut all = json!({"form_center": [50, 50]});
    for (name, _, _, start) in PARTICLE_GRID_SETTINGS {
        all[name] = J::from(start);
    }
    for (name, _, start) in PARTICLE_GRID_WORDS {
        all[name] = J::from(start);
    }
    for name in PARTICLE_GRID_LAYERS {
        all[name] = J::from("");
    }
    for (name, start) in PARTICLE_GRID_COLORS {
        all[name] = J::from(start);
    }
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    all
}

fn fx(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.particle_grid", "enabled": true, "parameters": parameters(p)})
}

#[test]
fn b496_particle_grid() {
    let mut t = Table::new(
        "particle_grid",
        "# D-616: Particle Grid\n\nB-496, the particle engine's fifth effect (P0-6, PLUGINS.md's \
         parked pick), after what Trapcode Form is for: particles that stay put, in a grid of \
         points or on layered spheres in 3D, turned and moved as one, each point nudged its own \
         way, the whole twisted and rippled by a noise field that can drift and change, \
         coloured, sized and pushed towards the camera by other layers; drawn as soft or glowing \
         spheres, stars or faded clouds over a clear layer, through the composition camera when \
         there is one. Nothing is carried from frame to frame: the drift is the sum of the Flow \
         settings over the frames from the in point, each worked from the settings alone \
         (document 20). Drawn on the processor only. Red Giant publishes no formula and no \
         settings when added, so the rules, those settings and the name are ours, stated in \
         `tools/particle_grid_reference.py`. Every expected pixel is \
         `Fixtures/particle_grid/expected_particle_grid.json` and every expected point \
         `Fixtures/particle_grid/particles_particle_grid.json`, written by the tool before this \
         code existed and printed in document 25 as FX-PGRID-001 to 060. Tolerances 2e-5 and \
         1e-7.\n",
    );

    t.heading("FX-PGRID-001 to 060 (document 25)");
    t.fixtures("expected_particle_grid.json");

    t.heading("The points themselves: where each is, its size and its opacity");
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("particles_particle_grid.json")).unwrap()).unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    for (name, case) in expected["cases"].as_object().unwrap() {
        let n: u32 = name.rsplit('-').next().unwrap().parse().unwrap();
        let file = format!("fx_pgrid_{n:03}.json");
        let frame = case["frame"].as_i64().unwrap() as i32;
        let mut want: Vec<Vec<f64>> = case["particles"].as_array().unwrap().iter().map(|p| p.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()).collect();
        let mut got = alive(&t, &file, frame);
        want.sort_by(|a, b| a[0].total_cmp(&b[0]));
        got.sort_by(|a, b| a[0].total_cmp(&b[0]));
        let largest = got.iter().zip(&want).flat_map(|(g, w)| g.iter().zip(w).map(|(a, b)| (a - b).abs())).fold(0.0, f64::max);
        t.row(
            &format!("{name} frame {frame}: the {} points drawn, each its number, place, size and opacity", want.len()),
            &format!("{} drawn, largest difference {largest:.1e}", got.len()),
            got.len() == want.len() && largest <= tolerance,
        );
    }

    t.heading("The file");
    let files: Vec<String> = (1..=60).map(|n| format!("fx_pgrid_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_pgrid_049.json");
    let count = saved.as_object().unwrap().len();
    t.row(
        "fx_pgrid_049.json is saved with its 36 settings (28 numbers, the centre, 3 words, 3 layers and the colour) and no maps, clock, camera or history",
        &format!("{count} settings; form center {}, particle type {}, transfer mode {}, color map {}", saved["form_center"], saved["particle_type"], saved["transfer_mode"], saved["color_map"]),
        count == 36
            && saved["form_center"].as_array().is_some_and(|a| a.len() == 2)
            && saved["particle_type"] == "glow_sphere"
            && saved["transfer_mode"] == "add"
            && saved["color_map"] == "ramp",
    );
    for (n, want) in [
        (50, "particles in x"),
        (51, "particles in z"),
        (52, "size"),
        (53, "opacity"),
        (54, "fractal scale"),
        (55, "base form"),
        (56, "base form"),
        (57, "particle type"),
        (58, "transfer mode"),
        (59, "colour"),
        (60, "flow evolution"),
    ] {
        let file = format!("fx_pgrid_{n:03}.json");
        let why = fault_of(&t, &file);
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }
    t.shape_refused("fx_pgrid_001.json", "a Particle Grid whose size is a word", &parameters(&json!({"size": "big"})).to_string());
    t.shape_refused("fx_pgrid_001.json", "a Particle Grid whose base form is a number", &parameters(&json!({"base_form": 1})).to_string());
    t.shape_refused("fx_pgrid_001.json", "a Particle Grid without its displace map", &{
        let mut p = parameters(&json!({}));
        p.as_object_mut().unwrap().remove("displace_map");
        p.to_string()
    });

    t.heading("Tiles");
    t.tiles(&[("fx_pgrid_027.json", 0), ("fx_pgrid_034.json", 0), ("fx_pgrid_039.json", 0), ("fx_pgrid_040.json", 0)]);

    t.heading("Pictures (verification/D-616 pictures)");
    let out = repo("verification/D-616 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0).0;
    for (name, p, frame) in [
        ("1_as_added", json!({}), 0),
        (
            "2_rippling_field",
            json!({"form_size_x": 520, "form_size_y": 300, "form_size_z": 200, "particles_in_x": 60, "particles_in_y": 35, "form_rotation_x": 60,
                "size": 3, "displace": 40, "fractal_scale": 120, "flow_x": 40, "flow_evolution": 0.4, "affect_size": 80, "particle_type": "glow_sphere",
                "transfer_mode": "add", "color": "#80c0ff"}),
            48,
        ),
        (
            "3_layered_ball_turning",
            json!({"base_form": "sphere_layered", "form_size_x": 220, "form_size_y": 220, "form_size_z": 220, "particles_in_x": 40, "particles_in_y": 20,
                "particles_in_z": 3, "form_rotation_x": 20, "form_rotation_y": 35, "size": 4, "color": "#ffd080"}),
            0,
        ),
        (
            "4_coloured_by_the_street",
            json!({"form_size_x": 480, "form_size_y": 270, "particles_in_x": 80, "particles_in_y": 45, "size": 5, "sphere_feather": 80, "color_map": "under"}),
            0,
        ),
        (
            "5_pushed_by_the_street",
            json!({"form_size_x": 480, "form_size_y": 270, "particles_in_x": 80, "particles_in_y": 45, "form_rotation_x": 50, "size": 3, "displace_map": "under",
                "map_displace": 120, "color_map": "under"}),
            0,
        ),
        (
            "6_twisted_stars",
            json!({"form_size_x": 360, "form_size_y": 200, "particles_in_x": 12, "particles_in_y": 7, "twist": 180, "disperse": 8, "particle_type": "star",
                "size": 14, "transfer_mode": "screen", "color": "#fff0a0"}),
            0,
        ),
        (
            "7_cloudlets",
            json!({"form_size_x": 400, "form_size_y": 220, "particles_in_x": 10, "particles_in_y": 6, "particle_type": "cloudlet", "size": 50,
                "size_random": 40, "opacity": 50, "opacity_random": 30, "random_seed": 3, "color": "#ffd0a0"}),
            0,
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
    t.finish("D-616_particle_grid_table.md");
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
/// Grid on its first layer, frames 0, 60, 120 and 239 each drawn on the processor with empty
/// caches, three times; the median of the three. A grid's points cost the same at every frame;
/// only the Flow sums grow with the frame, and only when a setting is keyed.
#[test]
#[ignore = "B-496: a measurement, run deliberately with --release --ignored"]
fn b496_particle_grid_timing() {
    use std::fmt::Write as _;

    let mut s = format!(
        "- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: the processor (Particle Grid has no card path)\n\n\
         | Shot | Frame 0 | Frame 60 | Frame 120 | Frame 239 |\n|---|---:|---:|---:|---:|\n",
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
    let shots: [(&str, Option<J>); 4] = [
        ("no effect", None),
        ("Particle Grid as added (1271 points)", Some(json!({}))),
        (
            "100 by 100 glowing points, rippling and drifting, added",
            Some(json!({"particles_in_x": 100, "particles_in_y": 100, "displace": 40, "flow_x": 40, "flow_evolution": 0.5, "particle_type": "glow_sphere",
                "transfer_mode": "add"})),
        ),
        (
            "200 by 100 by 5 points tipped, coloured and pushed by a layer",
            Some(json!({"particles_in_x": 200, "particles_in_y": 100, "particles_in_z": 5, "form_size_z": 400, "form_rotation_x": 50, "size": 3,
                "color_map": "layer-2", "displace_map": "layer-2"})),
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
    let out = std::env::var("B496_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-496_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
