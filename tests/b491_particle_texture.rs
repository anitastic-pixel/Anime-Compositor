//! B-491: D-611, Particle Space's textured particles (P0-6): three more particle types, a square,
//! a disc and a faded disc, that turn (Rotation Speed, Initial Rotation) and show a layer of the
//! composition (Texture Layer, D-189's layer setting, read whole at its own size), taken down
//! through its own smaller copies as the particle shrinks. The kinds after the purpose of
//! CycoreFX's CC Particle World textured types; CycoreFX publishes no formula and no settings when
//! added, so the rules and those settings are ours, stated in
//! `tools/particle_texture_reference.py`.
//!
//! Writes `verification/D-611_particle_texture_table.md` and draws pictures into
//! `verification/D-611 pictures/`.
//!
//! Every expected pixel is `Fixtures/particle_texture/expected_particle_texture.json`, written by
//! the tool before this code existed and printed in document 25 as FX-PTEX-001 to 030. Tolerance
//! 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, distance, median, reference, repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, PARTICLE_SPACE_COLORS, PARTICLE_SPACE_SETTINGS, PARTICLE_SPACE_WORDS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

/// The Particle Spaces the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::ParticleSpace { .. })))
        .count()
}

/// Every fixture file on the card against the processor, at each frame its case draws, at Full
/// and Draft. The files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_particle_texture.json")).unwrap()).unwrap();
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
        let project = reference(|id| json!([fx(&format!("b491-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Particle Space {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
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
/// with what it warned of. A switched-off layer `street` holds the same still, for a texture.
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
    let mut street = layer.clone();
    layer["effects"] = effects;
    street["id"] = J::from("street");
    street["name"] = J::from("street");
    street["enabled"] = J::from(false);
    comp["layers"].as_array_mut().unwrap().push(street);
    comp["layer_order"].as_array_mut().unwrap().push(J::from("street"));
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

fn fx(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.particle_space", "enabled": true, "parameters": parameters(p)})
}

#[test]
fn b491_particle_texture() {
    let mut t = Table::new(
        "particle_texture",
        "# D-611: Particle Space's textured particles\n\nB-491 (P0-6): three more particle types, \
         Textured Square, Textured Disc and Textured Faded Disc, that turn (Rotation Speed, \
         Initial Rotation) and show a layer of the composition (Texture Layer, read whole at its \
         own size), taken down through its own smaller copies as the particle shrinks. After the \
         purpose of CycoreFX's CC Particle World textured types; CycoreFX publishes no formula \
         and no settings when added, so the rules and those settings are ours, stated in \
         `tools/particle_texture_reference.py`. A texture layer that is not in the composition \
         leaves the effect out with a warning (D-189); one with no picture at the frame draws no \
         textured particles. Every expected pixel is \
         `Fixtures/particle_texture/expected_particle_texture.json`, written by the tool before \
         this code existed and printed in document 25 as FX-PTEX-001 to 030. Tolerance 2e-5.\n",
    );

    t.heading("FX-PTEX-001 to 030 (document 25)");
    t.fixtures("expected_particle_texture.json");

    t.heading("The file");
    let files: Vec<String> = (1..=30).map(|n| format!("fx_ptex_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_ptex_004.json");
    t.row(
        "fx_ptex_004.json is saved with its 62 settings, the texture layer dot, particle type textured_square, and not the picture it reads",
        &format!("{} settings; texture layer {}, particle type {}, texture {:?}", saved.as_object().unwrap().len(), saved["texture_layer"], saved["particle_type"], saved.get("texture")),
        saved.as_object().unwrap().len() == 62 && saved["texture_layer"] == "dot" && saved["particle_type"] == "textured_square" && saved.get("texture").is_none(),
    );
    let older = t.load("fx_ptex_001.json");
    let mut older_json = effect_table::saved(&older);
    let params = &mut older_json["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    for name in ["rotation_speed", "initial_rotation", "texture_layer"] {
        params.as_object_mut().unwrap().remove(name);
    }
    let reread = persist::load_str(&older_json.to_string()).expect("a D-610 file reads");
    let back = effect_table::saved(&reread);
    let p = &back["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    let e = reread.document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    let as_added = matches!(&e, Effect::ParticleSpace { values, texture_layer, .. } if values[44] == 0.0 && values[45] == 0.0 && texture_layer == "");
    t.row(
        "a file written before D-611, without rotation speed, initial rotation or texture layer, reads them as added and is saved without them",
        &format!("as added: {as_added}; saved with {} settings", p.as_object().unwrap().len()),
        as_added && p.as_object().unwrap().len() == 59,
    );
    for (n, want) in [(26, "particle type"), (27, "rotation speed"), (28, "initial rotation"), (29, "initial rotation"), (30, "texture layer")] {
        let file = format!("fx_ptex_{n:03}.json");
        let why = effect_of(&t, &file).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }
    t.shape_refused("fx_ptex_004.json", "a Particle Space whose rotation speed is a word", &parameters(&json!({"rotation_speed": "fast"})).to_string());

    t.heading("Tiles");
    t.tiles(&[("fx_ptex_011.json", 20), ("fx_ptex_012.json", 20), ("fx_ptex_023.json", 24)]);

    t.heading("On the card (D-611)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[20, 26, 27, 28, 29, 30]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("textured squares, no texture layer", json!({"particle_type": "textured_square", "birth_size": 0.8, "death_size": 0.8, "initial_rotation": 360}), 3),
            (
                "textured discs showing the background layer-4, turning, over the drawing",
                json!({"particle_type": "textured_disc", "texture_layer": "layer-4", "birth_size": 1.5, "death_size": 0.5, "rotation_speed": 90, "initial_rotation": 360,
                    "composite_on_original": "on", "birth_rate": 3}),
                3,
            ),
            (
                "textured faded discs of layer-4, fogged, Rotation Speed keyed -360 to 360",
                json!({"particle_type": "textured_faded_disc", "texture_layer": "layer-4", "birth_size": 2, "death_size": 2, "depth_cue": "fog", "transfer_mode": "screen",
                    "rotation_speed": {"base": 0, "keyframes": [{"frame": 0, "value": -360, "interp": "linear"}, {"frame": 239, "value": 360, "interp": "linear"}]}}),
                3,
            ),
        ],
    );

    t.heading("Pictures (verification/D-611 pictures)");
    let out = repo("verification/D-611 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0).0;
    let over = |p: J| {
        let mut p = p;
        p["composite_on_original"] = J::from("on");
        p
    };
    for (name, p, frame) in [
        ("1_squares_no_texture", over(json!({"particle_type": "textured_square", "birth_size": 2, "death_size": 1, "initial_rotation": 360, "rotation_speed": 120})), 48),
        (
            "2_street_squares_turning",
            over(json!({"particle_type": "textured_square", "texture_layer": "street", "birth_rate": 2, "longevity": 3, "birth_size": 8, "death_size": 3,
                "initial_rotation": 360, "rotation_speed": 60, "velocity": 0.3, "gravity": 0.1})),
            48,
        ),
        (
            "3_street_discs_fountain",
            over(json!({"particle_type": "textured_disc", "texture_layer": "street", "animation": "direction_axis", "extra_angle": 25, "velocity": 1.2, "gravity": 1.0,
                "birth_rate": 3, "longevity": 2, "producer_y": 0.2, "birth_size": 5, "death_size": 2, "rotation_speed": 180, "initial_rotation": 360})),
            48,
        ),
        (
            "4_street_faded_discs_fog",
            over(json!({"particle_type": "textured_faded_disc", "texture_layer": "street", "animation": "vortex", "birth_rate": 2, "longevity": 3, "radius_x": 0.3,
                "radius_z": 0.3, "velocity": 0.3, "birth_size": 6, "death_size": 6, "depth_cue": "fog", "depth_cue_distance": 2, "camera_rotation_x": -25,
                "rotation_speed": 45})),
            72,
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
    let (bytes, said) = picture(json!([fx("pic", &over(json!({"particle_type": "textured_disc", "texture_layer": "gone"})))]), 24);
    t.row(
        "the street with Particle Space's texture layer `gone`, not in the composition: the effect left out, the street unchanged, the warning said",
        &format!("{} pixels changed; warnings {said:?}", distance(&bytes, &street).1),
        distance(&bytes, &street).1 == 0 && said.iter().any(|s| s.starts_with("EFFECT_LAYER_MISSING")),
    );
    t.finish("D-611_particle_texture_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B491_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-491: a measurement, run deliberately with --release --ignored"]
fn b491_particle_texture_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B491_CPU").is_ok();
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
        ("Noise, then Particle Space, textured squares, no texture", Some(json!({"particle_type": "textured_square", "birth_size": 0.8, "initial_rotation": 360}))),
        (
            "Noise, then Particle Space, 1000 a second, textured discs of layer-4 (1920 by 1080), turning, over the drawing",
            Some(json!({"birth_rate": 10, "particle_type": "textured_disc", "texture_layer": "layer-4", "birth_size": 1, "rotation_speed": 90, "composite_on_original": "on"})),
        ),
        (
            "Noise, then Particle Space, 2000 a second living 3 s, vortex, textured faded discs of layer-4, added",
            Some(json!({"birth_rate": 20, "longevity": 3, "animation": "vortex", "particle_type": "textured_faded_disc", "texture_layer": "layer-4", "birth_size": 0.6,
                "transfer_mode": "add", "composite_on_original": "on"})),
        ),
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
    let out = std::env::var("B491_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-491_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
