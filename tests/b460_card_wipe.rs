//! B-460: D-580, after After Effects' Card Wipe (Transition): the layer cut into a grid of cards
//! that flip over one after another to show the Back Layer, in its own fake 3D. Adobe's page names
//! the controls and says what each does in a sentence but publishes no formula and no settings
//! when added: the rule and those settings are ours, stated in `tools/card_wipe_reference.py`.
//! The composition camera and lights are not used.
//!
//! Writes `verification/D-580_card_wipe_table.md` and draws pictures into
//! `verification/D-580 pictures/`.
//!
//! Every expected pixel is `Fixtures/card_wipe/expected_card_wipe.json`, written by the tool
//! before this code existed and printed in document 25 as FX-CW-001 to 056. Tolerance 2e-5.
//! Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{repo, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
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

/// The Card Wipes the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::CardWipe { .. })))
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

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=56 {
        let file = format!("fx_cw_{n:03}.json");
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

/// The reference shot with each setting on its first three layers, its background `layer-4`
/// the back (and, where named, the gradient), frames 0, 100 and 239 at Full and Draft, on the
/// card against the processor; `want` of 3 on the card each frame.
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
        let project = reference(|id| json!([fx(&format!("b460-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Card Wipe {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Card Wipe {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// FX-CW-004's project at the street's size in `dir`, the left street the holder, the right
/// street the back and the fixture's ramp the gradient, the holder's effect set to `p`; drawn
/// on the processor, straight 8-bit, with what it warned of.
fn picture(dir: &Path, p: &J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/card_wipe/fx_cw_004.json")).unwrap()).unwrap();
    for (asset, path) in [("asset-shot", "left.png"), ("asset-back", "right.png"), ("asset-ramp", "ramp.png")] {
        let a = project["assets"].as_array_mut().unwrap().iter_mut().find(|a| a["id"] == asset).unwrap();
        a["path"] = J::from(path);
    }
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let holder = comp["layers"].as_array_mut().unwrap().iter_mut().find(|l| l["id"] == "holder").unwrap();
    holder["effects"] = if p.is_null() { json!([]) } else { json!([fx("pic", p)]) };
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// The `holder` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

/// The pixel at (x, y) of a picture at the street's size.
fn at(bytes: &[u8], p: (usize, usize)) -> [u8; 4] {
    let i = (p.1 * TOWN.0 + p.0) * 4;
    [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
}

/// A Card Wipe with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"transition_completion": 0, "transition_width": 50, "back_layer": "",
        "rows_columns": "independent", "rows": 8, "columns": 8, "card_scale": 1, "flip_axis": "x",
        "flip_direction": "positive", "flip_order": "left_to_right", "gradient_layer": "",
        "timing_randomness": 0, "random_seed": 0, "x_rotation": 0, "y_rotation": 0, "z_rotation": 0,
        "xy_position": [50, 50], "z_position": 1, "focal_length": 50, "light_type": "distant",
        "light_intensity": 1, "light_color": "#ffffff", "light_position": [50, 50], "light_depth": 1,
        "ambient_light": 0.25, "diffuse_reflection": 0.75, "specular_reflection": 0,
        "highlight_sharpness": 5});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.card_wipe", "enabled": true, "parameters": all})
}

#[test]
fn b460_card_wipe() {
    let mut t = Table::new(
        "card_wipe",
        "# D-580: Card Wipe\n\nB-460, after After Effects' Card Wipe (Transition): the layer cut \
         into a grid of cards that flip over one after another to show the Back Layer, in its own \
         fake 3D. Adobe's page names the controls and says what each does in a sentence but \
         publishes no formula and no settings when added, so the rule and those settings are ours, \
         stated in `tools/card_wipe_reference.py`: each card turns about its own middle when the \
         sweep (the flip order, shuffled by Timing Randomness) reaches it, over Transition Width; \
         a camera inside the effect (X, Y and Z Rotation, X, Y Position, Z Position in focal \
         lengths, Focal Length on a 36 mm frame) sends one ray a pixel and the nearest card it \
         meets is drawn, its front the picture and its back the Back Layer (clear when none), lit \
         by a distant or point light with ambient, diffuse and a Blinn-Phong highlight. The \
         composition camera and lights are not used. Every expected pixel is \
         `Fixtures/card_wipe/expected_card_wipe.json`, written by the tool before this code \
         existed and printed in document 25 as FX-CW-001 to 056. Tolerance 2e-5.\n",
    );

    t.heading("FX-CW-001 to 056 (document 25)");
    t.fixtures_numbered("expected_card_wipe.json", 1..=56);

    t.heading("The settings");
    let plain = changed(&t, "fx_cw_004.json", |_| {});
    let mut draft = plain.clone();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview leaves every setting as it is, since each is a share of the drawing, a count or an angle",
        &format!("{draft:?}"),
        draft == plain,
    );

    t.heading("The file");
    let files: Vec<String> = (1..=56).map(|n| format!("fx_cw_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_cw_004.json");
    t.row(
        "fx_cw_004.json is saved with its twenty-eight settings, the back layer, completion 50, X, Y Position [50, 50]",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 28
            && saved["back_layer"] == "back"
            && saved["transition_completion"] == json!(50)
            && saved["xy_position"] == json!([50, 50]),
    );
    let params = t.saved_parameters("fx_cw_054.json");
    t.row(
        "fx_cw_054.json, its back layer written as the number 3, is saved as the number 3",
        &params["back_layer"].to_string(),
        params["back_layer"] == json!(3),
    );
    let params = t.saved_parameters("fx_cw_004.json");
    t.row(
        "the back and gradient pictures the effect reads are never saved",
        &format!("{:?} {:?}", params.get("back_map"), params.get("gradient_map")),
        params.get("back_map").is_none() && params.get("gradient_map").is_none(),
    );
    for (file, want) in [
        ("fx_cw_042.json", "flip axis"),
        ("fx_cw_043.json", "flip order"),
        ("fx_cw_044.json", "rows"),
        ("fx_cw_045.json", "columns"),
        ("fx_cw_046.json", "card scale"),
        ("fx_cw_047.json", "card scale"),
        ("fx_cw_048.json", "timing randomness"),
        ("fx_cw_049.json", "z position"),
        ("fx_cw_050.json", "focal length"),
        ("fx_cw_051.json", "light type"),
        ("fx_cw_052.json", "light colour"),
        ("fx_cw_053.json", "highlight sharpness"),
        ("fx_cw_054.json", "back layer"),
        ("fx_cw_055.json", "rows & columns"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    let mut good = fx("x", &json!({}))["parameters"].clone();
    good["rows"] = json!("many");
    t.shape_refused("fx_cw_001.json", "a Card Wipe whose rows is a word", &good.to_string());
    let mut good = fx("x", &json!({}))["parameters"].clone();
    good["xy_position"] = json!(50);
    t.shape_refused("fx_cw_001.json", "a Card Wipe whose X, Y Position is one number", &good.to_string());
    let mut good = fx("x", &json!({}))["parameters"].clone();
    good.as_object_mut().unwrap().remove("flip_order");
    t.shape_refused("fx_cw_001.json", "a Card Wipe with no flip order", &good.to_string());
    let mut good = fx("x", &json!({}))["parameters"].clone();
    good.as_object_mut().unwrap().remove("back_layer");
    t.shape_refused("fx_cw_001.json", "a Card Wipe with no back layer", &good.to_string());

    t.heading("On the card (D-580)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &(42..=56).collect::<Vec<_>>());
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("completion 40, the background layer-4 the back", json!({"transition_completion": 40, "back_layer": "layer-4"}), 3),
            (
                "completion 60 by the gradient layer-4, random axis and direction, shuffled, the camera turned, a point light with a highlight",
                json!({"transition_completion": 60, "back_layer": "layer-4", "flip_order": "gradient", "gradient_layer": "layer-4",
                    "flip_axis": "random", "flip_direction": "random", "timing_randomness": 0.5, "random_seed": 12,
                    "x_rotation": -20, "y_rotation": 30, "z_rotation": 10, "light_type": "point", "light_depth": 0.5,
                    "specular_reflection": 1, "highlight_sharpness": 20}),
                3,
            ),
            ("card scale 0.7, 20 rows by 30 columns, no back, completion 50", json!({"transition_completion": 50, "card_scale": 0.7, "rows": 20, "columns": 30}), 3),
            (
                "Transition Completion keyed 0 to 100",
                json!({"back_layer": "layer-4", "transition_completion": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 100, "interp": "linear"}]}}),
                3,
            ),
        ],
    );

    t.heading("Pictures (verification/D-580 pictures)");
    let dir = std::env::temp_dir().join("b460_pictures");
    fs::create_dir_all(&dir).unwrap();
    fs::copy(repo("verification/D-506 pictures/1_left.png"), dir.join("left.png")).unwrap();
    fs::copy(repo("verification/D-506 pictures/2_right.png"), dir.join("right.png")).unwrap();
    fs::copy(repo("Fixtures/card_wipe/media/ramp.png"), dir.join("ramp.png")).unwrap();
    let out = repo("verification/D-580 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(&dir, &J::Null).0;
    for (name, p) in [
        ("1_halfway_left_to_right", json!({"transition_completion": 50, "back_layer": "back"})),
        ("2_halfway_turned_camera_lit", json!({"transition_completion": 50, "back_layer": "back", "y_rotation": 25, "x_rotation": -15, "specular_reflection": 1, "highlight_sharpness": 20})),
        ("3_gradient_order", json!({"transition_completion": 50, "back_layer": "back", "flip_order": "gradient", "gradient_layer": "ramp"})),
        ("4_shuffled_random_flips", json!({"transition_completion": 50, "back_layer": "back", "timing_randomness": 1, "random_seed": 7, "flip_axis": "random", "flip_direction": "random"})),
        ("5_small_cards_no_back", json!({"transition_completion": 40, "card_scale": 0.7, "rows": 12, "columns": 20})),
        ("6_venetian_blinds", json!({"transition_completion": 50, "back_layer": "back", "rows": 10, "columns": 1, "transition_width": 100})),
        ("7_all_turned_over", json!({"transition_completion": 100, "back_layer": "back"})),
    ] {
        let (bytes, said) = picture(&dir, &p);
        png_out::write_rgba(&out.join(format!("{name}.png")), TOWN.0, TOWN.1, OutputDepth::Eight, &[], &bytes).unwrap();
        let middle = at(&bytes, (210, 118));
        t.row(
            &format!("{name}.png: drawn without a warning, the street changed"),
            &format!("{} pixels changed; pixel (210, 118) {middle:?}; warnings {said:?}", distance(&bytes, &street).1),
            said.is_empty() && distance(&bytes, &street).1 > 0,
        );
    }
    t.finish("D-580_card_wipe_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect
/// with the background `layer-4` its back, every eighth frame asked for as the viewer asks,
/// whole. The first loop starts with empty caches and its 30 frames' median is "first"; the
/// median of the loops after it is "again". With `B460_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-460: a measurement, run deliberately with --release --ignored"]
fn b460_card_wipe_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B460_CPU").is_ok();
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
        ("Noise, then Card Wipe, completion 50, 8 by 8", Some(json!({"transition_completion": 50, "back_layer": "layer-4"}))),
        ("Noise, then Card Wipe, camera turned, point light", Some(json!({"transition_completion": 50, "back_layer": "layer-4", "y_rotation": 30, "x_rotation": -20, "light_type": "point", "specular_reflection": 1}))),
        ("Noise, then Card Wipe, 100 by 100 cards", Some(json!({"transition_completion": 50, "back_layer": "layer-4", "rows": 100, "columns": 100}))),
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
    let out = std::env::var("B460_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-460_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
