//! B-580: D-710, Particle Plane's textured particles (P0-6): Particle Space's Textured Disc and
//! Textured Faded Disc (D-611) on the flat emitter, with Texture Layer, Rotation Speed and Initial
//! Rotation, and Composite With Original, which draws the particles over the layer instead of on
//! an empty one. After the purpose of CycoreFX's CC Particle Systems II; CycoreFX publishes no
//! formula, so the rules are ours, stated in `tools/particle_plane_texture_reference.py`.
//! Textured Square came later, by the owner's decision of 2026-10-13 (D-714, B-584).
//!
//! Writes `verification/D-710_particle_plane_texture_table.md` and draws pictures into
//! `verification/D-710 pictures/`.
//!
//! Every expected pixel is `Fixtures/particle_plane_texture/expected_particle_plane_texture.json`,
//! written by the tool before this code existed, FX-PPTEX-001 to 029. Tolerance 2e-5. Nothing here
//! is a snapshot of a run.

mod effect_table;

use std::fs;

use effect_table::{both, distance, median, on_card, reference, repo, time_passes, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, PARTICLE_PLANE_COLORS, PARTICLE_PLANE_SETTINGS, PARTICLE_PLANE_WORDS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// Every fixture file on the card against the processor, at each frame its case draws, at Full
/// and Draft. The files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_particle_plane_texture.json")).unwrap()).unwrap();
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
        let project = reference(|id| json!([fx(&format!("b580-{id}"), p)]));
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
fn b580_particle_plane_texture() {
    let mut t = Table::new(
        "particle_plane_texture",
        "# D-710: Particle Plane's textured particles and Composite With Original\n\nB-580 (P0-6): \
         Particle Space's Textured Disc and Textured Faded Disc (D-611) on the flat emitter, with \
         Texture Layer, Rotation Speed and Initial Rotation, and Composite With Original, which \
         draws the particles over the layer instead of on an empty one. After the purpose of \
         CycoreFX's CC Particle Systems II; CycoreFX publishes no formula and no settings when \
         added, so the rules are ours, stated in `tools/particle_plane_texture_reference.py`. \
         Textured Square came later, by the owner's decision of 2026-10-13 (D-714, B-584). A \
         texture layer that is not in the composition leaves the effect out with a \
         warning (D-189). Every expected pixel is \
         `Fixtures/particle_plane_texture/expected_particle_plane_texture.json`, written by the \
         tool before this code existed, FX-PPTEX-001 to 029. Tolerance 2e-5.\n",
    );

    t.heading("FX-PPTEX-001 to 029");
    t.fixtures("expected_particle_plane_texture.json");

    t.heading("The file");
    let files: Vec<String> = (1..=29).map(|n| format!("fx_pptex_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_pptex_008.json");
    t.row(
        "fx_pptex_008.json is saved with its 39 settings, the texture layer dot, particle type textured_faded_disc, and not the picture it reads",
        &format!("{} settings; texture layer {}, particle type {}, texture {:?}", saved.as_object().unwrap().len(), saved["texture_layer"], saved["particle_type"], saved.get("texture")),
        saved.as_object().unwrap().len() == 39 && saved["texture_layer"] == "dot" && saved["particle_type"] == "textured_faded_disc" && saved.get("texture").is_none(),
    );
    let older = t.load("fx_pptex_005.json");
    let mut older_json = effect_table::saved(&older);
    let params = &mut older_json["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    for name in ["rotation_speed", "initial_rotation", "texture_layer", "composite_on_original"] {
        params.as_object_mut().unwrap().remove(name);
    }
    let reread = persist::load_str(&older_json.to_string()).expect("a D-612 file reads");
    let back = effect_table::saved(&reread);
    let p = &back["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    let e = reread.document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    let as_added = matches!(&e, Effect::ParticlePlane { values, words, texture_layer, .. }
        if values[23] == 0.0 && values[24] == 0.0 && words[5] == "off" && texture_layer == "");
    t.row(
        "a file written before D-710, without rotation speed, initial rotation, texture layer or composite with original, reads them as added and is saved without them",
        &format!("as added: {as_added}; saved with {} settings", p.as_object().unwrap().len()),
        as_added && p.as_object().unwrap().len() == 35,
    );
    for (n, want) in [
        (24, "particle type"),
        (25, "rotation speed"),
        (26, "initial rotation"),
        (27, "initial rotation"),
        (28, "composite on original"),
        (29, "texture layer"),
    ] {
        let file = format!("fx_pptex_{n:03}.json");
        let why = effect_of(&t, &file).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }
    t.shape_refused("fx_pptex_008.json", "a Particle Plane whose rotation speed is a word", &parameters(&json!({"rotation_speed": "fast"})).to_string());

    t.heading("Tiles");
    t.tiles(&[("fx_pptex_008.json", 20), ("fx_pptex_012.json", 20), ("fx_pptex_019.json", 20)]);

    t.heading("On the card (D-710)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[18, 24, 25, 26, 27, 28, 29]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("textured discs, no texture layer, over the drawing", json!({"particle_type": "textured_disc", "birth_size": 0.8, "death_size": 0.8, "composite_on_original": "on"}), 3),
            (
                "textured discs showing the background layer-4, turning, over the drawing",
                json!({"particle_type": "textured_disc", "texture_layer": "layer-4", "birth_size": 1.5, "death_size": 0.5, "rotation_speed": 90, "initial_rotation": 360,
                    "composite_on_original": "on", "birth_rate": 3}),
                3,
            ),
            (
                "textured faded discs of layer-4, screened, Rotation Speed keyed -360 to 360",
                json!({"particle_type": "textured_faded_disc", "texture_layer": "layer-4", "birth_size": 2, "death_size": 2, "transfer_mode": "screen",
                    "rotation_speed": {"base": 0, "keyframes": [{"frame": 0, "value": -360, "interp": "linear"}, {"frame": 239, "value": 360, "interp": "linear"}]}}),
                3,
            ),
        ],
    );

    t.heading("Pictures (verification/D-710 pictures)");
    let out = repo("verification/D-710 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(json!([]), 0).0;
    let over = |p: J| {
        let mut p = p;
        p["composite_on_original"] = J::from("on");
        p
    };
    for (name, p, frame) in [
        ("1_as_added_over_the_street", over(json!({})), 24),
        (
            "2_street_discs_fountain",
            over(json!({"particle_type": "textured_disc", "texture_layer": "street", "animation": "direction", "extra_angle": 25, "velocity": 1.2, "gravity": 1.0,
                "birth_rate": 3, "longevity": 2, "producer_y": 0.2, "birth_size": 5, "death_size": 2, "rotation_speed": 180, "initial_rotation": 360})),
            48,
        ),
        (
            "3_street_faded_discs_vortex",
            over(json!({"particle_type": "textured_faded_disc", "texture_layer": "street", "animation": "vortex", "birth_rate": 2, "longevity": 3, "radius_x": 0.3,
                "radius_y": 0.2, "velocity": 0.3, "gravity": 0, "birth_size": 6, "death_size": 6, "rotation_speed": 45})),
            72,
        ),
        (
            "4_street_discs_on_empty",
            json!({"particle_type": "textured_disc", "texture_layer": "street", "birth_rate": 2, "longevity": 3, "birth_size": 8, "death_size": 3,
                "initial_rotation": 360, "rotation_speed": 60, "velocity": 0.3, "gravity": 0.1}),
            48,
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
        "the street with Particle Plane's texture layer `gone`, not in the composition: the effect left out, the street unchanged, the warning said",
        &format!("{} pixels changed; warnings {said:?}", distance(&bytes, &street).1),
        distance(&bytes, &street).1 == 0 && said.iter().any(|s| s.starts_with("EFFECT_LAYER_MISSING")),
    );
    t.finish("D-710_particle_plane_texture_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B580_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-580: a measurement, run deliberately with --release --ignored"]
fn b580_particle_plane_texture_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B580_CPU").is_ok();
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
        ("Noise, then Particle Plane, textured discs, no texture", Some(json!({"particle_type": "textured_disc", "birth_size": 0.8, "initial_rotation": 360}))),
        (
            "Noise, then Particle Plane, 1000 a second, textured discs of layer-4 (1920 by 1080), turning, over the drawing",
            Some(json!({"birth_rate": 10, "particle_type": "textured_disc", "texture_layer": "layer-4", "birth_size": 1, "rotation_speed": 90, "composite_on_original": "on"})),
        ),
        (
            "Noise, then Particle Plane, 2000 a second living 3 s, vortex, textured faded discs of layer-4, added",
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
        let (first, times) = time_passes(&mut gpu, cpu, passes, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B580_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-580_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
