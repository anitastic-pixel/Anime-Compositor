//! B-397: D-517, Twister under `core.twister`, after CycoreFX's CC Twister (Transition), under
//! our own name: the layer twists over round a line through the Center, a strip at a time, like a
//! ribbon, to show another layer on its back. CycoreFX names the controls but publishes no
//! formula and no defaults: the rule and the settings when added are ours, stated in
//! `tools/twister_reference.py`.
//!
//! Writes `verification/D-517_twister_table.md` and draws pictures into
//! `verification/D-517 pictures/`.
//!
//! Every expected pixel is `Fixtures/twister/expected_twister.json`, written by the tool before
//! this code existed and printed in document 25 as FX-TWIST-001 to 030. Tolerance 2e-5. Nothing
//! here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{repo, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::Command;
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

fn is_ours(c: &render::OnCard) -> bool {
    matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::Twister { .. }))
}

/// The Twisters the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers.iter().flat_map(|l| &l.on_card).filter(|c| is_ours(c.unmixed())).count()
        // An adjustment layer's run is made from its stack when the card draws it.
        + plan.layers
            .iter()
            .filter_map(|l| l.adjust.as_ref())
            .filter_map(|stack| compose::adjust_run(stack, (plan.width, plan.height)))
            .flatten()
            .filter(|c| is_ours(c.unmixed()))
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

/// A Twister with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"completion": 0, "backside": "", "shading": "on", "center": [50, 50], "axis": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.twister", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` leave nothing for the card.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=30 {
        let file = format!("fx_twist_{n:03}.json");
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
/// and Draft, on the card against the processor; 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J)]) {
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
        let project = reference(|id| json!([fx(&format!("b397-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Twister {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Twister {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the layer showing `1_street.png` (from `dir`), with a hidden layer `reveal`
/// showing `2_reveal.png` beneath it, drawn on the processor, straight 8-bit, with what it
/// warned of.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("1_street.png");
    let mut asset = project["assets"][0].clone();
    asset["id"] = J::from("asset-reveal");
    asset["name"] = J::from("reveal");
    asset["path"] = J::from("2_reveal.png");
    project["assets"].as_array_mut().unwrap().push(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    let layer = &mut comp["layers"][0];
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let mut reveal = layer.clone();
    reveal["id"] = J::from("reveal");
    reveal["name"] = J::from("reveal");
    reveal["asset_id"] = J::from("asset-reveal");
    reveal["enabled"] = J::from(false);
    reveal["effects"] = json!([]);
    comp["layers"].as_array_mut().unwrap().insert(0, reveal);
    comp["layer_order"].as_array_mut().unwrap().push(J::from("reveal"));
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// Set the `holder` layer's first effect (the fixtures' own names).
fn set_holder(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: Id::new(MAIN), layer_id: Id::new("holder"), instance_id: Id::new("fx-1"), effect }
}

/// Key a setting of the `holder` layer's first effect, linearly.
fn keys_holder(setting: &str, values: &[(i32, f64)]) -> Command {
    Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        setting: setting.to_string(),
        keys: values.iter().map(|&(frame, v)| EffectKey { frame, value: vec![v], interp: Interp::Linear }).collect(),
    }
}

#[test]
fn b397_twister() {
    let mut t = Table::new(
        "twister",
        "# D-517: Twister\n\nB-397: `core.twister`, after CycoreFX's CC Twister (Transition), \
         under our own name: the layer twists over round a line through the Center, along the \
         Axis, a strip at a time like a ribbon, showing the Backside (D-189's layer setting, \
         centred on the drawing; nothing when none is chosen) where a strip has turned over. \
         CycoreFX names the controls but publishes no formula and no defaults, so the rule and \
         the settings when added are ours, stated in `tools/twister_reference.py`: across the \
         drawing a strip at t (0 where the twist starts, 1 at the far side) is turned by \
         pi clamp(2 Completion / 100 - t, 0, 1); each point reads the picture squeezed toward \
         the line by the cosine of that turn, the front while it faces the viewer and the back \
         after, and Shading multiplies the colour by that cosine's size. Every expected pixel is \
         `Fixtures/twister/expected_twister.json`, written by the tool before this code existed \
         and printed in document 25 as FX-TWIST-001 to 030. Tolerance 2e-5.\n",
    );

    t.heading("FX-TWIST-001 to 030 (document 25)");
    t.fixtures_numbered("expected_twister.json", 1..=30);

    t.heading("Files whose layers read each other (document 25)");
    let expected: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/twister/expected_twister.json")).unwrap()).unwrap();
    for (file, case) in expected["loads"].as_object().unwrap() {
        let says = case["says"].as_str().unwrap();
        let id = case["refused"].as_str().unwrap();
        let got = persist::load(&t.root.join(file)).err();
        t.row(
            &format!("{file}: {says}"),
            &got.as_ref().map_or("opened".to_string(), |d| format!("{} {}", d.id.as_str(), d.message)),
            got.is_some_and(|d| d.id.as_str() == id),
        );
    }

    t.heading("The file");
    let files: Vec<String> = (1..=30).map(|n| format!("fx_twist_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_twist_008.json");
    t.row(
        "fx_twist_008.json is saved with its five settings, the back as written",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 5
            && saved["backside"] == "paint"
            && saved["shading"] == "on"
            && saved["completion"] == 45
            && saved["center"] == json!([40, 60])
            && saved["axis"] == 30,
    );
    for (file, want) in [
        ("fx_twist_022.json", "completion"),
        ("fx_twist_023.json", "completion"),
        ("fx_twist_024.json", "axis"),
        ("fx_twist_025.json", "axis"),
        ("fx_twist_026.json", "center"),
        ("fx_twist_027.json", "center"),
        ("fx_twist_028.json", "shading"),
        ("fx_twist_029.json", "backside"),
    ] {
        let why = holder_effect(&t, file).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    let whole = r#""backside": "", "shading": "on", "center": [50, 50], "axis": 0"#;
    t.shape_refused("fx_twist_001.json", "a Twister whose completion is a word", &format!(r#"{{{whole}, "completion": "half"}}"#));
    t.shape_refused("fx_twist_001.json", "a Twister without its completion", &format!(r#"{{{whole}}}"#));
    t.shape_refused(
        "fx_twist_001.json",
        "a Twister without its backside",
        r#"{"completion": 0, "shading": "on", "center": [50, 50], "axis": 0}"#,
    );
    t.shape_refused(
        "fx_twist_001.json",
        "a Twister whose Center is one number",
        r#"{"completion": 0, "backside": "", "shading": "on", "center": 50, "axis": 0}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_twist_008.json").document;
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = holder_effect(&t, "fx_twist_008.json");
        f(&mut e);
        e
    };
    let over = with(&|e| if let Effect::Twister { completion, .. } = e { *completion = 101.0 });
    let word = with(&|e| if let Effect::Twister { shading, .. } = e { *shading = "maybe".to_string() });
    let bare = with(&|e| if let Effect::Twister { completion, backside, .. } = e {
        *completion = 70.0;
        *backside = J::from("");
    });
    // Nothing is a distance: a draft changes nothing.
    let mut draft = with(&|_| {});
    draft.scale_distances(|d| d * 0.5);
    let same = draft == with(&|_| {});
    refused(
        &mut t,
        &mut document,
        vec![
            ("completion 101", set_holder(over)),
            ("shading \"maybe\"", set_holder(word)),
            ("axis keyed to 4000", keys_holder("axis", &[(0, 0.0), (4, 4000.0)])),
        ],
    );
    taken(
        &mut t,
        &mut document,
        "fx_twist_008.json",
        vec![
            ("completion 70 with nothing on the back", set_holder(bare)),
            ("completion keyed from 0 to 100", keys_holder("completion", &[(0, 0.0), (4, 100.0)])),
        ],
    );
    t.row("a half-size draft preview changes no setting (none is a distance)", &format!("{draft:?}"), same);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_twist_003.json", 0),
        ("fx_twist_004.json", 0),
        ("fx_twist_005.json", 0),
        ("fx_twist_006.json", 0),
        ("fx_twist_007.json", 0),
        ("fx_twist_008.json", 0),
        ("fx_twist_009.json", 0),
        ("fx_twist_011.json", 0),
        ("fx_twist_012.json", 0),
        ("fx_twist_013.json", 2),
        ("fx_twist_018.json", 0),
        ("fx_twist_019.json", 0),
        ("fx_twist_020.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Completion 0 (001), a back not in the composition (016) and refused (022 to 030) leave
    // nothing for the card.
    let none: Vec<u32> = [1, 16].into_iter().chain(22..=30).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("completion 40, layer 4 on the back (as added otherwise)", json!({"completion": 40, "backside": "layer-4"})),
            ("completion 60, axis 90, shading off, nothing on the back", json!({"completion": 60, "axis": 90, "shading": "off"})),
            ("completion 30, axis -30, Center [30, 60], layer 4 on the back", json!({"completion": 30, "axis": -30, "center": [30, 60], "backside": "layer-4"})),
            ("completion 100, layer 4 on the back", json!({"completion": 100, "backside": "layer-4"})),
        ],
    );

    t.heading("Pictures: in `verification/D-517 pictures/`");
    let dir = repo("verification/D-517 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    // The street, and on its back, the street in its opposite colours.
    let street = town();
    let opposite: Vec<u8> = street.chunks_exact(4).flat_map(|p| [255 - p[0], 255 - p[1], 255 - p[2], p[3]]).collect();
    write("1_street.png", &street);
    write("2_reveal.png", &opposite);
    let px = |v: &[u8], i: usize| v[i * 4..i * 4 + 4].to_vec();
    // The left and right halves: pixels changed in each.
    let halves = |v: &[u8]| {
        let mut n = [0, 0];
        for i in 0..w * h {
            if px(v, i) != px(&street, i) {
                n[(i % w >= w / 2) as usize] += 1;
            }
        }
        n
    };

    let (begun, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 25, "backside": "reveal"}))]));
    write("3_begun.png", &begun);
    let n = halves(&begun);
    t.row(
        "3_begun.png, completion 25: the left half twisting toward edge on and darkening, squeezed toward the middle row; the right half untouched; draws cleanly",
        &format!("{said:?}, pixels changed left {} right {}", n[0], n[1]),
        said.is_empty() && n[0] > w * h / 4 && n[1] == 0,
    );

    let (half, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 50, "backside": "reveal"}))]));
    write("4_half_over.png", &half);
    let n = halves(&half);
    t.row(
        "4_half_over.png, completion 50: the left half turned over to the opposite colours, the right half starting to turn, both squeezed toward the middle row near the centre line; draws cleanly",
        &format!("{said:?}, pixels changed left {} right {}", n[0], n[1]),
        said.is_empty() && n[0] > w * h / 4 && n[1] > 0,
    );

    let (down, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 50, "backside": "reveal", "axis": 90, "shading": "off"}))]));
    write("5_down_unshaded.png", &down);
    t.row(
        "5_down_unshaded.png, completion 50, axis 90, shading off: the twist runs downward, the top half turned over, no darkening; draws cleanly",
        &format!("{said:?}, {} pixels differ from 4", distance(&down, &half).1),
        said.is_empty() && distance(&down, &half).1 > 0,
    );

    let (done, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 100, "backside": "reveal"}))]));
    write("6_done.png", &done);
    let d = distance(&done, &opposite);
    t.row(
        "6_done.png, completion 100: turned over entirely, the opposite colours; draws cleanly",
        &format!("{said:?}, largest difference {} of 255 from the back, {} pixels", d.0, d.1),
        said.is_empty() && d.0 <= 1,
    );

    let (gone, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 50, "backside": "gone"}))]));
    t.row(
        "a backside not in the composition: the street exactly as it was, with the warning",
        &format!("{said:?}, {} pixels changed", distance(&gone, &street).1),
        gone == street && said.iter().any(|s| s.starts_with("EFFECT_LAYER_MISSING")),
    );

    t.finish("D-517_twister_table.md");
}

/// The effect in `file` (on `holder`, or the adjustment layer in 022).
fn holder_effect(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let layer = d.project().composition(&Id::new(MAIN)).unwrap().layers_in_order().find(|l| l.effects.iter().any(|e| matches!(e.effect, Effect::Twister { .. })));
    layer.expect("a layer with a Twister").effects[0].effect.clone()
}

/// The whole composition, to see that a refused command changed nothing.
fn everything(document: &anime_compositor::command::Document) -> String {
    format!("{:?}", document.project().composition(&Id::new(MAIN)).unwrap())
}

fn refused(t: &mut Table, document: &mut anime_compositor::command::Document, commands: Vec<(&str, Command)>) {
    let held = everything(document);
    for (what, command) in commands {
        let refused = document.apply(command).err();
        let untouched = everything(document) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
}

fn taken(t: &mut Table, document: &mut anime_compositor::command::Document, file: &str, commands: Vec<(&str, Command)>) {
    let n = commands.len();
    for (what, command) in commands {
        let taken = document.apply(command).is_ok();
        t.row(&format!("{what} is taken"), if taken { "taken" } else { "refused" }, taken);
    }
    let before = t.render(&t.load(file).document, 0, 64);
    for _ in 0..n {
        document.undo();
    }
    let same = t.render(document, 0, 64).data() == before.data();
    t.row(&format!("undo {n} times: frame 0 is the frame it was"), if same { "byte-identical" } else { "differ" }, same);
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B397_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-397: a measurement, run deliberately with --release --ignored"]
fn b397_twister_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B397_CPU").is_ok();
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
        ("Noise, then Twister, completion 50, layer 4 on the back (as added otherwise)", Some(json!({"completion": 50, "backside": "layer-4"}))),
        ("Noise, then Twister, completion 50, axis 30, shading off, nothing on the back", Some(json!({"completion": 50, "axis": 30, "shading": "off"}))),
        ("Noise, then Twister, completion 75, Center [30, 60], layer 4 on the back", Some(json!({"completion": 75, "center": [30, 60], "backside": "layer-4"}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}g"), p));
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
    let out = std::env::var("B397_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-397_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
