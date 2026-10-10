//! B-303: D-424 Glue Gun, our name for CycoreFX's CC Glue Gun ("Generate" in
//! `docs/effects/EFFECTS.md`): a glossy, blobby stroke squeezed out along the path the brush takes.
//!
//! Writes `verification/D-424_glue_gun_table.md`.
//!
//! Every expected pixel is `Fixtures/glue_gun/expected_glue_gun.json`, written by
//! `tools/glue_gun_reference.py` before this code existed and printed in document 25 as
//! FX-GLUE-001 to 027. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-424 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{repo, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Interp, Project};
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

/// The effects `pick` takes that the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality, pick: fn(&Effect) -> bool) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if pick(&f.instance.effect)))
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

/// An effect of `type_id` with the settings `p`; for the ones here, every setting `p` leaves
/// out is the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.glue_gun" => json!({
            "brush_position": [50, 50], "stroke_width": 20, "density": 5, "time_span": 1, "reflection": 50, "strength": 50,
            "paint_style": "plain", "wobble_width": 10, "wobble_height": 10, "wobble_speed": 1, "light_intensity": 100,
            "light_color": "#ffffff", "light_type": "distant", "light_height": 100, "light_position": [30, 30],
            "light_direction": -45, "ambient": 25, "diffuse": 75, "specular": 50, "roughness": 0.05, "metal": 100
        }),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` change nothing or are left out with a warning, so the card is not asked.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, stem: &str, count: u32, none: &[u32], pick: fn(&Effect) -> bool) {
    let comp = Id::new(MAIN);
    for n in 1..=count {
        let file = format!("{stem}_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, pick);
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor.
fn card_reference(t: &mut Table, gpu: &mut Gpu, name: &str, type_id: &str, settings: &[(&str, J)], pick: fn(&Effect) -> bool) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p) in settings {
        let project = reference(|id| json!([fx(type_id, &format!("b303-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, {name} {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, pick);
                t.row(
                    &format!("the reference shot, {name} {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

fn changed(a: &[u8], b: &[u8]) -> bool {
    distance(a, b).1 > 0
}

fn files(stem: &str, count: u32) -> Vec<String> {
    (1..=count).map(|n| format!("{stem}_{n:03}.json")).collect()
}

// --- Glue Gun --------------------------------------------------------------------------------

fn is_glue(e: &Effect) -> bool {
    matches!(e, Effect::GlueGun { .. })
}

/// A Glue Gun as a new one starts, changed by `change`.
fn glue(change: impl FnOnce(&mut Effect)) -> Effect {
    let mut e = Effect::GlueGun {
        brush_position: [50.0, 50.0],
        stroke_width: 20.0,
        density: 5.0,
        time_span: 1.0,
        reflection: 50.0,
        strength: 50.0,
        paint_style: "plain".into(),
        wobble_width: 10.0,
        wobble_height: 10.0,
        wobble_speed: 1.0,
        light_intensity: 100.0,
        light_color: "#ffffff".into(),
        light_type: "distant".into(),
        light_height: 100.0,
        light_position: [30.0, 30.0],
        light_direction: -45.0,
        ambient: 25.0,
        diffuse: 75.0,
        specular: 50.0,
        roughness: 0.05,
        metal: 100.0,
        trail: Vec::new(),
        clock: [0.0, 0.0],
    };
    change(&mut e);
    e
}

/// Set the `holder` layer's Glue Gun; its fixtures' drawing is `holder`, not `art`.
fn set_holder(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: Id::new(MAIN), layer_id: Id::new("holder"), instance_id: Id::new("fx-1"), effect }
}

/// Key a setting of the `holder` layer's Glue Gun, linearly.
fn keys_holder(setting: &str, values: &[(i32, &[f64])]) -> Command {
    Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        setting: setting.to_string(),
        keys: values.iter().map(|&(frame, v)| EffectKey { frame, value: v.to_vec(), interp: Interp::Linear }).collect(),
    }
}

/// Each command is refused with a sentence and leaves the `holder` layer's effect as it was.
fn refused_holder(t: &mut Table, document: &mut Document, commands: Vec<(&str, Command)>) {
    let held = |d: &Document| format!("{:?}", d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0]);
    let before = held(document);
    for (what, command) in commands {
        let refused = document.apply(command).err();
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && held(document) == before,
        );
    }
}

fn why_holder(t: &mut Table, cases: &[(&str, &str)]) {
    for (file, want) in cases {
        let doc = t.load(file).document;
        let why = doc.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].effect.why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == *want);
    }
}

/// The brush keyed from `from` at frame 0 to `to` at frame 4, as the file holds it.
fn sweep(from: [f64; 2], to: [f64; 2]) -> J {
    json!({"base": from, "keyframes": [{"frame": 0, "value": from, "interp": "linear"}, {"frame": 4, "value": to, "interp": "linear"}]})
}

/// `effects` on the street at frame 4, drawn, straight 8-bit, with what it warned of: the
/// stroke the brush laid over frames 0 to 4.
fn stroke_picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 4, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

#[test]
fn b303_glue_gun() {
    let mut t = Table::new(
        "glue_gun",
        "# D-424: Glue Gun\n\nB-303, after CycoreFX's CC Glue Gun: a glossy stroke squeezed out \
         along the path the brush takes. Each frame the brush's place now and at each frame of the \
         Time Span before (with Time Span 0, back to the layer's in point) is read as its keys are; \
         Density blobs a frame are laid along that path, wobbling with Wobbly; the blobs run into \
         one another by Strength, and the paint mirrors the layer by Reflection and is lit as \
         Blobbylize is. The manual gives no formula, ranges or defaults, so those are ours. Every \
         expected pixel is `Fixtures/glue_gun/expected_glue_gun.json`, written by \
         `tools/glue_gun_reference.py` before this code existed and printed in document 25 as \
         FX-GLUE-001 to 027. Tolerance 2e-5.\n",
    );

    t.heading("FX-GLUE-001 to 027 (document 25)");
    t.fixtures("expected_glue_gun.json");

    t.heading("How far it reaches");
    let got = glue(|_| {}).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing: paint past the layer's edge is cut", &got.to_string(), got == 0);
    let mut draft = glue(|e| {
        if let Effect::GlueGun { light_type, light_height, .. } = e {
            *light_type = "point".into();
            *light_height = 40.0;
        }
    });
    draft.scale_distances(|d| d * 0.5);
    let want = glue(|e| {
        if let Effect::GlueGun { stroke_width, wobble_width, wobble_height, light_type, light_height, .. } = e {
            *stroke_width = 10.0;
            *wobble_width = 5.0;
            *wobble_height = 5.0;
            *light_type = "point".into();
            *light_height = 20.0;
        }
    });
    t.row(
        "a half-size draft preview halves the stroke width, the wobble and a point light's height; the brush and the light's place are shares of the drawing",
        &format!("{draft:?}"),
        draft == want,
    );

    t.heading("The file");
    let all = files("fx_glue", 27);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_glue_010.json");
    t.row(
        "fx_glue_010.json is saved with all 21 settings, the brush's keys with them",
        &saved.to_string(),
        saved["brush_position"]["keyframes"].as_array().is_some_and(|k| k.len() == 2)
            && saved["stroke_width"] == 6
            && saved["density"] == 2
            && saved["time_span"] == 0
            && saved["reflection"] == 0
            && saved["light_color"] == "#ff8000"
            && saved["specular"] == 100
            && saved["roughness"] == 0.5
            && saved["metal"] == 0
            && saved["diffuse"] == 0
            && saved["light_height"] == 40
            && saved["paint_style"] == "plain"
            && saved["light_type"] == "distant"
            && saved.as_object().unwrap().len() == 21,
    );
    why_holder(
        &mut t,
        &[
            ("fx_glue_019.json", "Glue Gun's stroke width runs from 0 to 500, and this is 501."),
            ("fx_glue_020.json", "Glue Gun's density runs from 0 to 100, and this is -1."),
            ("fx_glue_021.json", "Glue Gun's time span runs from 0 to 100, and this is 101."),
            ("fx_glue_022.json", "Glue Gun's paint style is \"plain\" or \"wobbly\", and this is \"drippy\"."),
            ("fx_glue_023.json", "Glue Gun's light type is distant or point, and this is \"spot\"."),
            ("fx_glue_024.json", "Glue Gun's roughness runs from 0.001 to 1, and this is 0."),
            ("fx_glue_025.json", "Glue Gun's brush position runs from -1000 to 1000, and this is 1001."),
            ("fx_glue_026.json", "Glue Gun's light colour is written #rrggbb, and this is \"orange\"."),
        ],
    );
    let rest = r##""stroke_width": 20, "density": 5, "time_span": 1, "reflection": 50, "strength": 50, "paint_style": "plain", "wobble_width": 10, "wobble_height": 10, "wobble_speed": 1, "light_intensity": 100, "light_color": "#ffffff", "light_type": "distant", "light_height": 100, "light_position": [30, 30], "light_direction": -45, "ambient": 25, "diffuse": 75, "specular": 50, "roughness": 0.05, "metal": 100"##;
    t.shape_refused("fx_glue_001.json", "a Glue Gun with no `brush_position`", &format!("{{{rest}}}"));
    t.shape_refused("fx_glue_001.json", "a Glue Gun whose brush position is one number", &format!("{{\"brush_position\": 50, {rest}}}"));
    t.shape_refused("fx_glue_001.json", "a Glue Gun whose stroke width is a word", &format!("{{\"brush_position\": [50, 50], {}}}", rest.replace("\"stroke_width\": 20", "\"stroke_width\": \"20\"")));
    t.shape_refused("fx_glue_001.json", "a Glue Gun with no `metal`", &format!("{{\"brush_position\": [50, 50], {}}}", rest.replace(", \"metal\": 100", "")));

    t.heading("Commands");
    let mut document = t.load("fx_glue_001.json").document;
    refused_holder(
        &mut t,
        &mut document,
        vec![
            ("stroke width 500.5", set_holder(glue(|e| if let Effect::GlueGun { stroke_width, .. } = e { *stroke_width = 500.5 }))),
            ("density -0.5", set_holder(glue(|e| if let Effect::GlueGun { density, .. } = e { *density = -0.5 }))),
            ("time span 100.5", set_holder(glue(|e| if let Effect::GlueGun { time_span, .. } = e { *time_span = 100.5 }))),
            ("roughness 0", set_holder(glue(|e| if let Effect::GlueGun { roughness, .. } = e { *roughness = 0.0 }))),
            ("paint style \"drippy\"", set_holder(glue(|e| if let Effect::GlueGun { paint_style, .. } = e { *paint_style = "drippy".into() }))),
            ("light type \"spot\"", set_holder(glue(|e| if let Effect::GlueGun { light_type, .. } = e { *light_type = "spot".into() }))),
            ("light colour \"#fff\"", set_holder(glue(|e| if let Effect::GlueGun { light_color, .. } = e { *light_color = "#fff".into() }))),
            ("brush position keyed to 50, 1001", keys_holder("brush_position", &[(0, &[50.0, 50.0]), (4, &[50.0, 1001.0])])),
            ("wobble speed keyed to 150", keys_holder("wobble_speed", &[(0, &[1.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_glue_001.json",
        vec![
            (
                "the tops: width 500, density 100, time span 100, wobbly 1000 by 1000 at 100, a point light",
                set_holder(glue(|e| {
                    if let Effect::GlueGun { stroke_width, density, time_span, paint_style, wobble_width, wobble_height, wobble_speed, light_type, .. } = e {
                        *stroke_width = 500.0;
                        *density = 100.0;
                        *time_span = 100.0;
                        *paint_style = "wobbly".into();
                        *wobble_width = 1000.0;
                        *wobble_height = 1000.0;
                        *wobble_speed = 100.0;
                        *light_type = "point".into();
                    }
                })),
            ),
            (
                "the bottoms: width 0, density 0, time span 0, roughness 0.001",
                set_holder(glue(|e| {
                    if let Effect::GlueGun { stroke_width, density, time_span, roughness, .. } = e {
                        *stroke_width = 0.0;
                        *density = 0.0;
                        *time_span = 0.0;
                        *roughness = 0.001;
                    }
                })),
            ),
            ("brush position keyed from 20, 50 to 80, 50", keys_holder("brush_position", &[(0, &[20.0, 50.0]), (4, &[80.0, 50.0])])),
            ("stroke width keyed from 2 to 6", keys_holder("stroke_width", &[(0, &[2.0]), (4, &[6.0])])),
            ("light position keyed from 0, 0 to 100, 100", keys_holder("light_position", &[(0, &[0.0, 0.0]), (4, &[100.0, 100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_glue_001.json", 4),
        ("fx_glue_002.json", 4),
        ("fx_glue_007.json", 4),
        ("fx_glue_008.json", 6),
        ("fx_glue_009.json", 4),
        ("fx_glue_011.json", 4),
        ("fx_glue_014.json", 4),
        ("fx_glue_018.json", 4),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_glue", 27, &[12, 13, 19, 20, 21, 22, 23, 24, 25, 26, 27], is_glue);
    let across = json!({"base": [10, 50], "keyframes": [{"frame": 0, "value": [10, 50], "interp": "linear"}, {"frame": 239, "value": [90, 50], "interp": "linear"}]});
    card_reference(
        &mut t,
        &mut gpu,
        "Glue Gun",
        "core.glue_gun",
        &[
            ("as it starts (width 20, density 5, time span 1, the brush at the middle)", json!({})),
            ("the brush keyed across, width 60, density 10, time span 2", json!({"brush_position": across.clone(), "stroke_width": 60, "density": 10, "time_span": 2})),
            (
                "the brush keyed across, wobbly 30 by 20, a warm point light, reflection 100",
                json!({"brush_position": across.clone(), "stroke_width": 40, "paint_style": "wobbly", "wobble_width": 30, "wobble_height": 20, "wobble_speed": 2, "light_type": "point", "light_position": [40, 30], "light_height": 80, "light_color": "#ffd8a0", "reflection": 100}),
            ),
            (
                "the brush keyed across, strength 0, density 1, rough and dull, light from below",
                json!({"brush_position": across, "stroke_width": 50, "strength": 0, "density": 1, "roughness": 1, "metal": 0, "light_direction": 135}),
            ),
        ],
        is_glue,
    );

    t.heading("Pictures: the street at frame 4, in `verification/D-424 pictures/`");
    let dir = repo("verification/D-424 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = stroke_picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let shots: [(&str, J, &str); 5] = [
        ("as_added", json!({}), "as it starts, the brush never keyed: one round glossy blob in the middle"),
        ("sweep", json!({"brush_position": sweep([15.0, 70.0], [85.0, 30.0]), "time_span": 0, "density": 20}), "the brush keyed from bottom left to top right over frames 0 to 4, kept for ever, density 20: a shiny tube of paint across the street"),
        (
            "wobbly_point",
            json!({"brush_position": sweep([15.0, 50.0], [85.0, 50.0]), "time_span": 0, "density": 20, "paint_style": "wobbly", "wobble_width": 8, "wobble_height": 12, "wobble_speed": 3, "light_type": "point", "light_position": [50, 20], "light_height": 60}),
            "the same across the middle, wobbly 8 by 12, a point light above: a lumpy wobbling stroke lit from above",
        ),
        (
            "mirror_orange",
            json!({"brush_position": sweep([15.0, 30.0], [85.0, 70.0]), "time_span": 0, "density": 20, "stroke_width": 40, "reflection": 100, "light_color": "#ff8000", "metal": 0, "specular": 100, "roughness": 0.2}),
            "width 40, reflection 100, an orange shine: the street mirrored in a thick glassy stroke with an orange highlight",
        ),
        (
            "blobs",
            json!({"brush_position": sweep([15.0, 50.0], [85.0, 50.0]), "time_span": 0, "density": 1, "strength": 0, "stroke_width": 30}),
            "density 1, strength 0, width 30: five separate glossy beads, one a frame",
        ),
    ];
    for (i, (name, p, what)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = stroke_picture(&dir, json!([fx("core.glue_gun", "fx-0-0", p)]));
        write(&file, &bytes);
        t.row(
            &format!("{file}, {what}; draws cleanly"),
            &format!("{said:?}, {} pixels changed", distance(&bytes, &before).1),
            said.is_empty() && changed(&bytes, &before),
        );
    }

    t.finish("D-424_glue_gun_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B303_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-303: a measurement, run deliberately with --release --ignored"]
fn b303_glue_gun_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B303_CPU").is_ok();
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
    let across = json!({"base": [10, 50], "keyframes": [{"frame": 0, "value": [10, 50], "interp": "linear"}, {"frame": 239, "value": [90, 50], "interp": "linear"}]});
    let shots: [(&str, Option<(&str, J)>); 4] = [
        ("Noise alone", None),
        ("Noise, then Glue Gun as added (width 20, density 5, time span 1)", Some(("core.glue_gun", json!({})))),
        (
            "Noise, then Glue Gun, the brush keyed across, width 60, density 10, time span 2",
            Some(("core.glue_gun", json!({"brush_position": across.clone(), "stroke_width": 60, "density": 10, "time_span": 2}))),
        ),
        (
            "Noise, then Glue Gun, the brush keyed across, wobbly 30 by 20, a point light, width 40",
            Some(("core.glue_gun", json!({"brush_position": across, "stroke_width": 40, "paint_style": "wobbly", "wobble_width": 30, "wobble_height": 20, "wobble_speed": 2, "light_type": "point", "light_position": [40, 30], "light_height": 80}))),
        ),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some((type_id, p)) = &e {
                v.push(fx(type_id, &format!("{id}c"), p));
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
    let out = std::env::var("B303_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-303_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
