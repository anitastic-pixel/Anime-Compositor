//! B-256 to B-258: D-377 Bend It, after CycoreFX's CC Bend It; D-378 Bender, after CycoreFX's CC
//! Bender; D-379 Blobbylize, after CycoreFX's CC Blobbylize ("Distort" in
//! `docs/effects/EFFECTS.md`).
//!
//! Writes `verification/D-377_bend_it_table.md`, `verification/D-378_bender_table.md` and
//! `verification/D-379_blobbylize_table.md`.
//!
//! Every expected pixel is `Fixtures/bend_it/expected_bend_it.json`,
//! `Fixtures/bender/expected_bender.json` or `Fixtures/blobbylize/expected_blobbylize.json`,
//! written by the matching `tools/*_reference.py` before this code existed and printed in
//! document 25 as FX-BENDIT-001 to 023, FX-BENDER-001 to 023 and FX-BLOB-001 to 027.
//! Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Each also draws the street into `verification/D-377 pictures/` (and D-378, D-379).

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, set, town, Table, MAIN, TOWN};
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

fn bend_it(bend: f64, start: [f64; 2], end: [f64; 2], prestart: &str, distort: &str) -> Effect {
    Effect::BendIt { bend, start, end, render_prestart: prestart.into(), distort: distort.into() }
}

fn bender(amount: f64, style: &str, adjust: &str, top: [f64; 2], base: [f64; 2]) -> Effect {
    Effect::Bender { amount, style: style.into(), adjust_to_distance: adjust.into(), top, base }
}

/// Blobbylize as it starts, with `change` made to it.
fn blob(change: impl FnOnce(&mut Effect)) -> Effect {
    let mut e = Effect::Blobbylize {
        layer: J::from(""),
        fit: "stretch".into(),
        property: "alpha".into(),
        softness: 10.0,
        cut_away: 0.0,
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
        map: None,
    };
    change(&mut e);
    e
}

/// Set the `holder` layer's Blobbylize; its fixtures' drawing is `holder`, not `art`.
fn set_holder(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: Id::new(MAIN), layer_id: Id::new("holder"), instance_id: Id::new("fx-1"), effect }
}

/// Key a setting of the `holder` layer's Blobbylize, linearly.
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
fn refused_holder(t: &mut Table, document: &mut anime_compositor::command::Document, commands: Vec<(&str, Command)>) {
    let held = |d: &anime_compositor::command::Document| format!("{:?}", d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0]);
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

/// An effect of `type_id` with the settings `p`; for the three here, every setting `p` leaves
/// out is the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.bend_it" => json!({"bend": 45, "start": [50, 100], "end": [50, 0], "render_prestart": "none", "distort": "legal"}),
        "core.bender" => json!({"amount": 20, "style": "bend", "adjust_to_distance": "off", "top": [50, 0], "base": [50, 100]}),
        "core.blobbylize" => json!({
            "layer": "", "fit": "stretch", "property": "alpha", "softness": 10, "cut_away": 0, "light_intensity": 100,
            "light_color": "#ffffff", "light_type": "distant", "light_height": 100, "light_position": [30, 30],
            "light_direction": -45, "ambient": 25, "diffuse": 75, "specular": 50, "roughness": 0.05, "metal": 100}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b256-{id}"), p)]));
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

/// `effects` on the street, drawn, straight 8-bit, with what it warned of.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
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
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// The pictures' folder, with the street written into it, and a writer for more.
fn pictures(t: &mut Table, d: &str) -> (std::path::PathBuf, impl Fn(&str, &[u8])) {
    t.heading(&format!("Pictures: the street, in `verification/{d} pictures/`"));
    let dir = repo(&format!("verification/{d} pictures"));
    fs::create_dir_all(&dir).unwrap();
    let out = dir.clone();
    let write = move |name: &str, bytes: &[u8]| png_out::write_rgba(&out.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    (dir, write)
}

/// The street drawn plain and with each of `shots` of `type_id`, written as numbered pictures; a
/// row each that it draws cleanly and changes what it says it changes.
fn street(t: &mut Table, d: &str, type_id: &str, shots: &[(&str, J, &str, fn(&[u8], &[u8]) -> bool)]) {
    let (dir, write) = pictures(t, d);
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    for (i, (name, p, what, check)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = picture(&dir, json!([fx(type_id, "fx-0-0", p)]));
        write(&file, &bytes);
        t.row(
            &format!("{file}, {what}; draws cleanly"),
            &format!("{said:?}, {} pixels changed", distance(&bytes, &before).1),
            said.is_empty() && check(&bytes, &before),
        );
    }
}

fn changed(a: &[u8], b: &[u8]) -> bool {
    distance(a, b).1 > 0
}

fn effect_of(d: &anime_compositor::command::Document, layer: &str) -> Effect {
    d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new(layer)).unwrap().effects[0].effect.clone()
}

fn why(t: &mut Table, layer: &str, cases: &[(&str, &str)]) {
    for (file, want) in cases {
        let why = effect_of(&t.load(file).document, layer).why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == *want);
    }
}

fn files(stem: &str, count: u32) -> Vec<String> {
    (1..=count).map(|n| format!("{stem}_{n:03}.json")).collect()
}

// --- Bend It ---------------------------------------------------------------------------------

fn is_bend_it(e: &Effect) -> bool {
    matches!(e, Effect::BendIt { .. })
}

#[test]
fn b256_bend_it() {
    let mut t = Table::new(
        "bend_it",
        "# D-377: Bend It\n\nB-256, after CycoreFX's CC Bend It: the strip of the layer along a \
         bar from Start to End curled into an arc that turns Bend degrees over the bar's length, \
         as a bar of rubber bends; before the Start the drawing can be left out, stay, carry the \
         bend on or mirror it, and past the End it can be left out or run on straight. The \
         formulas are this program's own. Every expected pixel is \
         `Fixtures/bend_it/expected_bend_it.json`, written by `tools/bend_it_reference.py` before \
         this code existed and printed in document 25 as FX-BENDIT-001 to 023. Tolerance 2e-5.\n",
    );

    t.heading("FX-BENDIT-001 to 023 (document 25)");
    t.fixtures("expected_bend_it.json");

    t.heading("The file");
    let all = files("fx_bendit", 23);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_bendit_017.json");
    t.row(
        "fx_bendit_017.json is saved with its bend, start, end, render prestart and distort",
        &saved.to_string(),
        saved["bend"] == -150 && saved["start"] == json!([50, 80]) && saved["end"] == json!([50, 20]) && saved["render_prestart"] == "static" && saved["distort"] == "extended",
    );
    why(
        &mut t,
        "art",
        &[
            ("fx_bendit_018.json", "Bend It's bend runs from -360 to 360, and this is 361."),
            ("fx_bendit_019.json", "Bend It's bend runs from -360 to 360, and this is -361."),
            ("fx_bendit_020.json", "Bend It's render prestart is none, static, bend or mirror, and this is \"Bend\"."),
            ("fx_bendit_021.json", "Bend It's distort is legal or extended, and this is \"extend\"."),
            ("fx_bendit_022.json", "Bend It's start runs from -1000 to 1000, and this is 1001."),
        ],
    );
    t.shape_refused("fx_bendit_001.json", "a Bend It with no `bend`", r#"{"start": [50, 100], "end": [50, 0], "render_prestart": "none", "distort": "legal"}"#);
    t.shape_refused("fx_bendit_001.json", "a Bend It whose start is a word", r#"{"bend": 45, "start": "50, 100", "end": [50, 0], "render_prestart": "none", "distort": "legal"}"#);

    t.heading("Commands");
    let mut document = t.load("fx_bendit_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("bend 360.5", set(bend_it(360.5, [50.0, 100.0], [50.0, 0.0], "none", "legal"))),
            ("start 50, 1000.5", set(bend_it(45.0, [50.0, 1000.5], [50.0, 0.0], "none", "legal"))),
            ("render prestart \"Mirror\", written with a capital", set(bend_it(45.0, [50.0, 100.0], [50.0, 0.0], "Mirror", "legal"))),
            ("distort \"extend\"", set(bend_it(45.0, [50.0, 100.0], [50.0, 0.0], "none", "extend"))),
            ("bend keyed to 400", keys("bend", &[(0, &[45.0]), (4, &[400.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_bendit_001.json",
        vec![
            ("bend -90 from 20, 80 to 70, 10, mirror, extended,", set(bend_it(-90.0, [20.0, 80.0], [70.0, 10.0], "mirror", "extended"))),
            ("bend keyed from 0 to 90", keys("bend", &[(0, &[0.0]), (4, &[90.0])])),
            ("end keyed from 50, 0 to 50, 50", keys("end", &[(0, &[50.0, 0.0]), (4, &[50.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_bendit_001.json", 0), ("fx_bendit_005.json", 0), ("fx_bendit_007.json", 0), ("fx_bendit_008.json", 0), ("fx_bendit_009.json", 0), ("fx_bendit_010.json", 0), ("fx_bendit_017.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_bendit", 23, &[12, 18, 19, 20, 21, 22, 23], is_bend_it);
    card_reference(
        &mut t,
        &mut gpu,
        "Bend It",
        "core.bend_it",
        &[
            ("as it starts (45, up the middle)", json!({})),
            ("-120 from 20, 90 to 80, 10, bend before, extended", json!({"bend": -120, "start": [20, 90], "end": [80, 10], "render_prestart": "bend", "distort": "extended"})),
            ("300 across the middle, mirror", json!({"bend": 300, "start": [10, 50], "end": [90, 50], "render_prestart": "mirror"})),
            ("75 on the upper half, static", json!({"bend": 75, "start": [50, 50], "end": [50, 0], "render_prestart": "static"})),
        ],
        is_bend_it,
    );

    street(
        &mut t,
        "D-377",
        "core.bend_it",
        &[
            ("as_added", json!({}), "as it starts (45 up the middle): the street leans and curls to the right as it rises", changed),
            ("bend_0", json!({"bend": 0}), "bend 0: the straight bar, nothing changes", |a, b| distance(a, b).0 == 0),
            ("curl_180_static", json!({"bend": 180, "start": [50, 70], "end": [50, 10], "render_prestart": "static"}), "180 from 70 per cent up to 10, static: the upper street rolls over into a half circle, the lower part stays", changed),
            ("banner_across_mirror", json!({"bend": -90, "start": [50, 50], "end": [100, 50], "render_prestart": "mirror"}), "-90 from the middle to the right edge, mirror: the street curls up at both sides like a banner", changed),
        ],
    );

    t.finish("D-377_bend_it_table.md");
}

// --- Bender ----------------------------------------------------------------------------------

fn is_bender(e: &Effect) -> bool {
    matches!(e, Effect::Bender { .. })
}

#[test]
fn b257_bender() {
    let mut t = Table::new(
        "bender",
        "# D-378: Bender\n\nB-257, after CycoreFX's CC Bender: the layer pushed sideways from an \
         axis between Base and Top, the push growing along the axis in one of four shapes: a bend \
         (straight on past the Top), a smooth sway in the middle (Marilyn), a sharp kink (Sharp) \
         or a smooth step (Boxer). The formulas are this program's own. Every expected pixel is \
         `Fixtures/bender/expected_bender.json`, written by `tools/bender_reference.py` before \
         this code existed and printed in document 25 as FX-BENDER-001 to 023. Tolerance 2e-5.\n",
    );

    t.heading("FX-BENDER-001 to 023 (document 25)");
    t.fixtures("expected_bender.json");

    t.heading("The file");
    let all = files("fx_bender", 23);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_bender_011.json");
    t.row(
        "fx_bender_011.json is saved with its amount, style, adjust to distance, top and base",
        &saved.to_string(),
        saved["amount"] == 3 && saved["style"] == "boxer" && saved["adjust_to_distance"] == "off" && saved["top"] == json!([100, 50]) && saved["base"] == json!([0, 50]),
    );
    why(
        &mut t,
        "art",
        &[
            ("fx_bender_018.json", "Bender's amount runs from -1000 to 1000, and this is 1001."),
            ("fx_bender_019.json", "Bender's amount runs from -1000 to 1000, and this is -1001."),
            ("fx_bender_020.json", "Bender's style is bend, marilyn, sharp or boxer, and this is \"Bend\"."),
            ("fx_bender_021.json", "Bender's adjust to distance is \"on\" or \"off\", and this is \"yes\"."),
            ("fx_bender_022.json", "Bender's top runs from -1000 to 1000, and this is 1001."),
        ],
    );
    t.shape_refused("fx_bender_001.json", "a Bender with no `style`", r#"{"amount": 20, "adjust_to_distance": "off", "top": [50, 0], "base": [50, 100]}"#);
    t.shape_refused("fx_bender_001.json", "a Bender whose amount is a word", r#"{"amount": "20", "style": "bend", "adjust_to_distance": "off", "top": [50, 0], "base": [50, 100]}"#);

    t.heading("Commands");
    let mut document = t.load("fx_bender_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("amount 1000.5", set(bender(1000.5, "bend", "off", [50.0, 0.0], [50.0, 100.0]))),
            ("base 50, -1000.5", set(bender(20.0, "bend", "off", [50.0, 0.0], [50.0, -1000.5]))),
            ("style \"wave\"", set(bender(20.0, "wave", "off", [50.0, 0.0], [50.0, 100.0]))),
            ("adjust to distance \"On\", written with a capital", set(bender(20.0, "bend", "On", [50.0, 0.0], [50.0, 100.0]))),
            ("amount keyed to 2000", keys("amount", &[(0, &[20.0]), (4, &[2000.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_bender_001.json",
        vec![
            ("marilyn 35 per cent of a slanted axis", set(bender(35.0, "marilyn", "on", [40.0, 10.0], [60.0, 90.0]))),
            ("amount keyed from 0 to 8", keys("amount", &[(0, &[0.0]), (4, &[8.0])])),
            ("top keyed from 50, 0 to 50, 50", keys("top", &[(0, &[50.0, 0.0]), (4, &[50.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_bender_001.json", 0), ("fx_bender_005.json", 0), ("fx_bender_006.json", 0), ("fx_bender_009.json", 0), ("fx_bender_011.json", 0), ("fx_bender_016.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_bender", 23, &[2, 12, 18, 19, 20, 21, 22, 23], is_bender);
    card_reference(
        &mut t,
        &mut gpu,
        "Bender",
        "core.bender",
        &[
            ("as it starts (bend 20, up the middle)", json!({})),
            ("marilyn -60 on the middle half", json!({"amount": -60, "style": "marilyn", "top": [50, 25], "base": [50, 75]})),
            ("sharp 15 per cent of a slanted axis", json!({"amount": 15, "style": "sharp", "adjust_to_distance": "on", "top": [30, 10], "base": [70, 90]})),
            ("boxer 40 across, left to right", json!({"amount": 40, "style": "boxer", "top": [90, 50], "base": [10, 50]})),
        ],
        is_bender,
    );

    street(
        &mut t,
        "D-378",
        "core.bender",
        &[
            ("as_added", json!({}), "as it starts (bend 20 up the middle): the upper street swept 20 pixels right, the bottom row still", changed),
            ("amount_0", json!({"amount": 0}), "amount 0: nothing changes", |a, b| distance(a, b).0 == 0),
            ("marilyn_40", json!({"amount": 40, "style": "marilyn"}), "marilyn 40: the middle of the street bulges 40 pixels right, top and bottom still", changed),
            ("boxer_across", json!({"amount": -30, "style": "boxer", "top": [100, 50], "base": [0, 50]}), "boxer -30 across: the street steps smoothly up 30 pixels from left to right", changed),
        ],
    );

    t.finish("D-378_bender_table.md");
}

// --- Blobbylize ------------------------------------------------------------------------------

fn is_blob(e: &Effect) -> bool {
    matches!(e, Effect::Blobbylize { .. })
}

#[test]
fn b258_blobbylize() {
    let mut t = Table::new(
        "blobbylize",
        "# D-379: Blobbylize\n\nB-258, after CycoreFX's CC Blobbylize: a blob map, the layer's own \
         or another layer's red, green, blue, alpha, luminance or lightness, softened and cut, \
         becomes the layer's covering and a surface lit by a distant or point light, ambient, \
         diffuse and a shine whose colour leans to the layer's by Metal. The formulas are this \
         program's own. Every expected pixel is `Fixtures/blobbylize/expected_blobbylize.json`, \
         written by `tools/blobbylize_reference.py` before this code existed and printed in \
         document 25 as FX-BLOB-001 to 027. Tolerance 2e-5.\n",
    );

    t.heading("FX-BLOB-001 to 027 (document 25)");
    t.fixtures("expected_blobbylize.json");

    t.heading("The file");
    let all = files("fx_blob", 27);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_blob_008.json");
    t.row(
        "fx_blob_008.json is saved with its layer, fit, property, softness, light and surface",
        &saved.to_string(),
        saved["layer"] == "" && saved["fit"] == "stretch" && saved["light_type"] == "point" && saved["light_position"] == json!([25, 30]) && saved["light_height"] == 10 && saved["roughness"] == 0.05 && saved["metal"] == 100,
    );
    let saved = t.saved_parameters("fx_blob_012.json");
    t.row("fx_blob_012.json is saved with its blob layer, dots", &saved.to_string(), saved["layer"] == "dots" && saved["softness"] == 2);
    why(
        &mut t,
        "holder",
        &[
            ("fx_blob_020.json", "Blobbylize's softness runs from 0 to 100, and this is 101."),
            ("fx_blob_021.json", "Blobbylize's cut away runs from 0 to 100, and this is -1."),
            ("fx_blob_022.json", "Blobbylize's property is red, green, blue, alpha, luminance or lightness, and this is \"hue\"."),
            ("fx_blob_023.json", "Blobbylize's light type is distant or point, and this is \"spot\"."),
            ("fx_blob_024.json", "Blobbylize's roughness runs from 0.001 to 1, and this is 0."),
            ("fx_blob_025.json", "Blobbylize's blob layer is the name of a layer of this composition, and this is 3."),
            ("fx_blob_026.json", "Blobbylize's light position runs from -1000 to 1000, and this is 1001."),
        ],
    );
    t.shape_refused(
        "fx_blob_001.json",
        "a Blobbylize with no `light_color`",
        r#"{"layer": "", "fit": "stretch", "property": "alpha", "softness": 10, "cut_away": 0, "light_intensity": 100, "light_type": "distant", "light_height": 100, "light_position": [30, 30], "light_direction": -45, "ambient": 25, "diffuse": 75, "specular": 50, "roughness": 0.05, "metal": 100}"#,
    );
    t.shape_refused(
        "fx_blob_001.json",
        "a Blobbylize whose light position is one number",
        r##"{"layer": "", "fit": "stretch", "property": "alpha", "softness": 10, "cut_away": 0, "light_intensity": 100, "light_color": "#ffffff", "light_type": "distant", "light_height": 100, "light_position": 30, "light_direction": -45, "ambient": 25, "diffuse": 75, "specular": 50, "roughness": 0.05, "metal": 100}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_blob_001.json").document;
    refused_holder(
        &mut t,
        &mut document,
        vec![
            ("softness 100.5", set_holder(blob(|e| if let Effect::Blobbylize { softness, .. } = e { *softness = 100.5 }))),
            ("roughness 0", set_holder(blob(|e| if let Effect::Blobbylize { roughness, .. } = e { *roughness = 0.0 }))),
            ("light intensity 400.5", set_holder(blob(|e| if let Effect::Blobbylize { light_intensity, .. } = e { *light_intensity = 400.5 }))),
            ("property \"saturation\"", set_holder(blob(|e| if let Effect::Blobbylize { property, .. } = e { *property = "saturation".into() }))),
            ("light colour \"#fff\"", set_holder(blob(|e| if let Effect::Blobbylize { light_color, .. } = e { *light_color = "#fff".into() }))),
            ("fit \"fill\"", set_holder(blob(|e| if let Effect::Blobbylize { fit, .. } = e { *fit = "fill".into() }))),
            ("metal keyed to 150", keys_holder("metal", &[(0, &[100.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_blob_001.json",
        vec![
            (
                "the dots as the blob by luminance, a warm point light",
                set_holder(blob(|e| {
                    if let Effect::Blobbylize { layer, property, light_type, light_color, light_height, .. } = e {
                        *layer = J::from("dots");
                        *property = "luminance".into();
                        *light_type = "point".into();
                        *light_color = "#ffeecc".into();
                        *light_height = 40.0;
                    }
                })),
            ),
            ("softness keyed from 0 to 8", keys_holder("softness", &[(0, &[0.0]), (4, &[8.0])])),
            ("light position keyed from 0, 0 to 100, 100", keys_holder("light_position", &[(0, &[0.0, 0.0]), (4, &[100.0, 100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_blob_001.json", 0), ("fx_blob_004.json", 0), ("fx_blob_008.json", 0), ("fx_blob_010.json", 0), ("fx_blob_012.json", 0), ("fx_blob_014.json", 0), ("fx_blob_017.json", 0), ("fx_blob_019.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_blob", 27, &[20, 21, 22, 23, 24, 25, 26, 27], is_blob);
    card_reference(
        &mut t,
        &mut gpu,
        "Blobbylize",
        "core.blobbylize",
        &[
            ("as it starts (its own alpha, softness 10)", json!({})),
            ("luminance, softness 6, cut 20, a warm point light", json!({"property": "luminance", "softness": 6, "cut_away": 20, "light_type": "point", "light_position": [25, 75], "light_height": 40, "light_color": "#ffeecc", "light_intensity": 150, "specular": 80, "roughness": 0.2, "metal": 40})),
            ("layer-4's lightness, stretched, as the blob", json!({"layer": "layer-4", "property": "lightness", "softness": 12, "light_direction": 135})),
            ("layer-4's red, tiled, rough and dull", json!({"layer": "layer-4", "fit": "tile", "property": "red", "softness": 3, "cut_away": 50, "roughness": 1, "metal": 0, "ambient": 50, "diffuse": 30})),
            ("layer-4's alpha, centred, a sharp shine", json!({"layer": "layer-4", "fit": "center", "softness": 20, "roughness": 0.005, "specular": 100, "light_height": -30})),
        ],
        is_blob,
    );

    street(
        &mut t,
        "D-379",
        "core.blobbylize",
        &[
            ("as_added", json!({}), "as it starts (its own alpha, softness 10): the solid street lit flat, its edges rounding off into a soft rim", changed),
            ("luminance_shiny", json!({"property": "luminance", "softness": 6}), "luminance, softness 6: the lit windows and markings stand up as shiny bumps", changed),
            (
                "luminance_cut_50",
                json!({"property": "luminance", "softness": 4, "cut_away": 50, "light_type": "point", "light_position": [70, 30], "light_height": 60, "light_color": "#ffd8a0"}),
                "luminance, cut away 50, a warm point light: the dark parts melt away, the bright parts left as warm lit blobs",
                |a, b| changed(a, b) && a.chunks_exact(4).any(|p| p[3] == 0),
            ),
        ],
    );

    t.finish("D-379_blobbylize_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B256_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-256: a measurement, run deliberately with --release --ignored"]
fn b256_distort_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B256_CPU").is_ok();
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
    let shots: [(&str, Option<(&str, J)>); 4] = [
        ("Noise alone", None),
        ("Noise, then Bend It as it starts (45, up the middle)", Some(("core.bend_it", json!({})))),
        ("Noise, then Bender as it starts (bend 20, up the middle)", Some(("core.bender", json!({})))),
        ("Noise, then Blobbylize as it starts (its own alpha, softness 10)", Some(("core.blobbylize", json!({})))),
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
    let out = std::env::var("B256_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-256_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
