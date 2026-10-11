//! B-441: D-561, After Effects' CC Glass under `core.glass` with its full control set: Bump Map,
//! Property, Softness, Height, Displacement, the light (Intensity, Color, Type, Height, Position,
//! Direction), Ambient, Diffuse, Specular, Roughness and Metal.
//!
//! Writes `verification/D-561_glass_table.md` and draws pictures into
//! `verification/D-561 pictures/`.
//!
//! Every expected pixel is `Fixtures/glass/expected_glass.json`, written by
//! `tools/glass_reference.py` before this code existed and printed in document 25 as
//! FX-GLASS-001 to 027. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        if p[3] == 0 && q[3] == 0 {
            continue;
        }
        let d = p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
        largest = largest.max(d);
        pixels += (d > 0) as usize;
    }
    (largest, pixels)
}

fn said(log: FrameLog) -> String {
    let mut ids: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
    ids.sort();
    ids.dedup();
    ids.join(", ")
}

/// The CC Glasses the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::Glass { .. })))
        .count()
}

/// The processor and the card, each drawing the frame the page receives: the largest
/// difference, the pixels differing, whether the card refused the frame, and each one's warnings.
fn both(gpu: &mut Gpu, project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> ((u8, usize), bool, String, String) {
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let c = preview::preview_frame_cached(project, comp, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
        .unwrap_or_else(|d| panic!("frame {frame} on the CPU: {}", d.message));
    let said_cpu = said(log);
    let mut log = FrameLog::new(3);
    let (g, ..) = preview::preview_frame_srgb8(project, comp, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu)
        .unwrap_or_else(|d| panic!("frame {frame} on the GPU: {}", d.message));
    let said_gpu = said(log);
    let refused = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
    (distance(&c.to_srgb8_straight(), &g), refused, said_cpu, said_gpu)
}

/// The reference shot (1920 by 1080) with `stack` on its first three layers.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// A CC Glass with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"layer": "", "fit": "stretch", "property": "alpha", "softness": 10, "height": 50, "displacement": 100,
        "light_angle": -45, "light_color": "#ffffff", "light_intensity": 100, "light_type": "distant", "light_height": 25,
        "light_position": [50, 50], "ambient": 75, "diffuse": 90, "specular": 25, "roughness": 0.1, "metal": 100});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.glass", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=27 {
        let file = format!("fx_glass_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality);
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor; `want` of 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, cases: &[(&str, J, usize)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in cases {
        let project = reference(|id| json!([fx(&format!("b441-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, CC Glass {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, CC Glass {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor at `frame`,
/// straight 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(size.0);
    comp["height"] = J::from(size.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([size.0 as f64 / 2.0, size.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// The `art` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

/// Pixel (x, y) of a picture `w` wide.
fn at(p: &[u8], w: usize, x: usize, y: usize) -> &[u8] {
    &p[(y * w + x) * 4..(y * w + x) * 4 + 4]
}

/// `command` aimed at the `holder` layer's effect `fx-1`; the fixtures' drawing is `holder`, not `art`.
fn holder(mut command: Command) -> Command {
    if let Command::SetEffectParameters { layer_id, instance_id, .. } | Command::SetEffectKeys { layer_id, instance_id, .. } = &mut command {
        *layer_id = Id::new("holder");
        *instance_id = Id::new("fx-1");
    }
    command
}

#[test]
fn b441_glass() {
    let mut t = Table::new(
        "glass",
        "# D-561: CC Glass, its full control set\n\nB-441: `core.glass` takes the rest of After Effects' CC Glass \
         controls (Light Type, Light Height, Light Position, Ambient, Diffuse, Specular, Roughness, Metal) \
         beside D-317's. A file without the light type keeps D-317's light. Every expected pixel is \
         `Fixtures/glass/expected_glass.json`, written by `tools/glass_reference.py` before this code existed \
         and printed in document 25 as FX-GLASS-001 to 027. Tolerance 2e-5.\n",
    );

    t.heading("FX-GLASS-001 to 027 (document 25)");
    t.fixtures_numbered("expected_glass.json", 1..=27);

    t.heading("The file");
    let files: Vec<String> = (1..=27).map(|n| format!("fx_glass_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_glass_013.json");
    t.row(
        "fx_glass_013.json is saved with its seventeen settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 17
            && saved["layer"] == "ramp"
            && saved["fit"] == "stretch"
            && saved["property"] == "luminance"
            && saved["softness"] == 0.0
            && saved["height"] == 100.0
            && saved["displacement"] == 3.0
            && saved["light_angle"] == -45.0
            && saved["light_color"] == "#ffffff"
            && saved["light_intensity"] == 100.0
            && saved["light_type"] == "distant"
            && saved["light_height"] == 25.0
            && saved["light_position"] == json!([50, 50])
            && saved["ambient"] == 75.0
            && saved["diffuse"] == 90.0
            && saved["specular"] == 25.0
            && saved["roughness"] == 0.1
            && saved["metal"] == 100.0,
    );
    let old = t.saved_parameters("fx_glass_002.json");
    t.row(
        "fx_glass_002.json, written before D-561, is saved with its nine settings as read, no light type added",
        &old.to_string(),
        old.as_object().unwrap().len() == 9 && old.get("light_type").is_none(),
    );
    for (file, want) in [
        ("fx_glass_021.json", "light type"),
        ("fx_glass_022.json", "roughness"),
        ("fx_glass_023.json", "ambient"),
        ("fx_glass_024.json", "light height"),
        ("fx_glass_025.json", "specular"),
        ("fx_glass_026.json", "light position"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming CC Glass and its {want}"), &why, why.contains("CC Glass") && why.contains(want));
    }
    let full = r##"{"layer": "", "fit": "stretch", "property": "alpha", "softness": 10, "height": 50, "displacement": 100, "light_angle": -45, "light_color": "#ffffff", "light_intensity": 100, "light_type": "distant", "light_height": 25, "light_position": [50, 50], "ambient": 75, "diffuse": 90, "specular": 25, "roughness": 0.1, "metal": 100}"##;
    t.shape_refused("fx_glass_001.json", "a CC Glass whose ambient is a word", &full.replace("\"ambient\": 75", "\"ambient\": \"some\""));
    t.shape_refused("fx_glass_001.json", "a CC Glass with a light type but without its metal", &full.replace(", \"metal\": 100", ""));
    t.shape_refused("fx_glass_001.json", "a CC Glass whose light position is one number", &full.replace("[50, 50]", "50"));
    t.shape_refused("fx_glass_001.json", "a CC Glass whose light type is a number", &full.replace("\"light_type\": \"distant\"", "\"light_type\": 2"));

    t.heading("Commands");
    let mut document = t.load("fx_glass_001.json").document;
    let over = changed(&t, "fx_glass_001.json", |e| if let Effect::Glass { ambient, .. } = e { *ambient = 101.0 });
    let far = changed(&t, "fx_glass_001.json", |e| if let Effect::Glass { light_position, .. } = e { *light_position = [50.0, -1001.0] });
    let word = changed(&t, "fx_glass_001.json", |e| if let Effect::Glass { light_type, .. } = e { *light_type = "spot".into() });
    let effect_of = |d: &Document| d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].clone();
    let held = format!("{:?}", effect_of(&document));
    for (what, command) in [
        ("ambient 101", set(over)),
        ("light position 50, -1001", set(far)),
        ("light type spot", set(word)),
        ("metal keyed to 150", keys("metal", &[(0, &[0.0]), (4, &[150.0])])),
    ] {
        let refused = document.apply(holder(command)).err();
        let untouched = format!("{:?}", effect_of(&document)) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
    let moved = changed(&t, "fx_glass_001.json", |e| {
        if let Effect::Glass { layer, property, softness, light_type, light_height, light_color, specular, roughness, metal, .. } = e {
            *layer = json!("ramp");
            *property = "luminance".into();
            *softness = 3.0;
            *light_type = "point".into();
            *light_height = 60.0;
            *light_color = "#ffcc88".into();
            *specular = 70.0;
            *roughness = 0.4;
            *metal = 0.0;
        }
    });
    let classic = changed(&t, "fx_glass_001.json", |e| if let Effect::Glass { light_type, .. } = e { *light_type = "classic".into() });
    t.taken(
        &mut document,
        "fx_glass_001.json",
        vec![
            ("the ramp, Luminance, softness 3, an orange point light 60 up, specular 70, roughness 0.4, metal 0", holder(set(moved))),
            ("the classic light", holder(set(classic))),
            ("light position keyed from 0, 0 to 100, 100", holder(keys("light_position", &[(0, &[0.0, 0.0]), (4, &[100.0, 100.0])]))),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_glass_001.json", 0),
        ("fx_glass_002.json", 0),
        ("fx_glass_008.json", 0),
        ("fx_glass_013.json", 0),
        ("fx_glass_014.json", 0),
        ("fx_glass_016.json", 0),
        ("fx_glass_018.json", 2),
        ("fx_glass_019.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Refused ones (021 to 027) leave nothing for the card.
    let none: Vec<u32> = (21..=27).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            ("the classic light, displacement 20, intensity 70", json!({"light_type": "classic", "displacement": 20, "light_intensity": 70}), 3),
            (
                "layer4 as the bump, Luminance, softness 20, height -60, a point light, specular 80, roughness 0.3, metal 0",
                json!({"layer": "layer-4", "property": "luminance", "softness": 20, "height": -60, "light_type": "point", "light_position": [20, 70], "light_height": 60, "specular": 80, "roughness": 0.3, "metal": 0}),
                3,
            ),
        ],
    );

    t.heading("Pictures: in `verification/D-561 pictures/`");
    let dir = repo("verification/D-561 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]), 0);
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]), 0);
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (w, h) = TOWN;
    let middle = |p: &[u8]| at(p, w, w / 2, h / 2).to_vec();

    let (added, said) = street(json!({}));
    write("2_as_added.png", TOWN, &added);
    t.row(
        "2_as_added.png, as added (its own covering as the bump, softness 10, height 50, displacement 100, a white distant light): the picture's edge bent like thick glass and lit from the top left; draws cleanly",
        &format!("{said:?}, changed {}", added != before),
        said.is_empty() && added != before,
    );

    let (classic, said) = street(json!({"light_type": "classic"}));
    write("3_classic_light.png", TOWN, &classic);
    t.row(
        "3_classic_light.png, the same with the classic light (as older projects open): the bend of 2 with Bevel Alpha's light instead, so it differs from 2_as_added.png; draws cleanly",
        &format!("{said:?}, different {}", classic != added),
        said.is_empty() && classic != added,
    );

    let relief = json!({"property": "luminance", "softness": 4, "height": 60, "displacement": 10, "specular": 80, "roughness": 0.3});
    let mut metal_p = relief.clone();
    metal_p["light_color"] = json!("#ffd080");
    let mut plastic_p = metal_p.clone();
    plastic_p["metal"] = json!(0);
    let (metal, said) = street(metal_p);
    let (orange, said2) = street(plastic_p);
    write("4_luminance_metal.png", TOWN, &metal);
    write("5_luminance_orange_light.png", TOWN, &orange);
    t.row(
        "4_luminance_metal.png and 5_luminance_orange_light.png, the street's lightness as glass ripples under an orange light with Metal 100 and 0: the shine takes the street's own colours in 4 and the light's orange in 5, so they differ; draw cleanly",
        &format!("{said:?} {said2:?}, different {}", metal != orange),
        said.is_empty() && said2.is_empty() && metal != orange,
    );

    let (point, said) = street(json!({"light_type": "point", "light_position": [25, 30], "light_height": 80, "specular": 60}));
    write("6_point_light.png", TOWN, &point);
    t.row(
        "6_point_light.png, a point light at 25, 30 per cent, 80 pixels up: brightest toward the top left, not 2_as_added.png; draws cleanly",
        &format!("{said:?}, different {}", point != added),
        said.is_empty() && point != added,
    );

    let (flat, said) = street(json!({"displacement": 0, "ambient": 100, "diffuse": 0, "specular": 0}));
    write("7_flat.png", TOWN, &flat);
    t.row(
        "7_flat.png, Displacement 0, Ambient 100, Diffuse 0, Specular 0: the street unchanged within 1 level at its middle pixel; draws cleanly",
        &format!("{said:?}, middle {:?} against {:?}", middle(&flat), middle(&before)),
        said.is_empty() && middle(&flat).iter().zip(middle(&before)).all(|(a, b)| a.abs_diff(b) <= 1),
    );

    t.finish("D-561_glass_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B441_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-441: a measurement, run deliberately with --release --ignored"]
fn b441_glass_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B441_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        ("Noise, then CC Glass as added", Some(json!({}))),
        ("the same, Luminance, softness 30, a point light, metal 0", Some(json!({"property": "luminance", "softness": 30, "light_type": "point", "metal": 0}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}b"), p));
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
    let out = std::env::var("B441_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-441_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
