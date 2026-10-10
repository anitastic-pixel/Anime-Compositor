//! B-291: D-412, Map Chromatic Displacement (PLUGINS.md pick #19): `core.displacement_map` takes
//! an amount for each colour and a spectrum, so light bent through water or glass parts into its
//! colours. A file without the settings moves every colour alike, as before.
//!
//! Writes `verification/D-412_map_chromatic_table.md` and draws pictures into
//! `verification/D-412 pictures/`.
//!
//! Every expected pixel is `Fixtures/map_chromatic/expected_map_chromatic.json`, written by
//! `tools/map_chromatic_reference.py` before this code existed and printed in document 25 as
//! FX-MAPCHROMA-001 to 019. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{repo, same_json, saved, town, Table, MAIN, TOWN};
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

fn is_dmap(c: &render::OnCard) -> bool {
    matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::DisplacementMap { .. }))
}

/// The Displacement Maps the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers.iter().flat_map(|l| &l.on_card).filter(|c| is_dmap(c.unmixed())).count()
        + plan.layers
            .iter()
            .filter_map(|l| l.adjust.as_ref())
            .filter_map(|stack| compose::adjust_run(stack, (plan.width, plan.height)))
            .flatten()
            .filter(|c| is_dmap(c.unmixed()))
            .count()
}

/// The processor and the card, each drawing the frame the page receives.
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

/// The reference shot (1920 by 1080) with `stack` on its first three layers; its fourth layer
/// is the map, as B-224 has it.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// A Displacement Map reading `layer-4`, red across and green down 12 pixels, with `p` laid over it.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"layer": "layer-4", "fit": "stretch", "horizontal": "red", "max_horizontal": 12, "vertical": "green",
        "max_vertical": 12, "wrap": "off", "red_amount": 50, "green_amount": 100, "blue_amount": 150, "spectrum": 3});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.displacement_map", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=19 {
        let file = format!("fx_mapchroma_{n:03}.json");
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

/// The reference shot with each setting on its first three layers, on the card against the
/// processor, all three on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project, frame: i32| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, frame, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    for (what, p) in settings {
        let project = reference(|id| json!([fx(&format!("b291-{id}"), p)]));
        let alike = reference(|id| json!([fx(&format!("b291-{id}"), &{
            let mut q = p.clone();
            for k in ["red_amount", "green_amount", "blue_amount"] {
                q[k] = J::from(100);
            }
            q
        })]));
        let parted = distance(&cpu(&project, 102), &cpu(&alike, 102)).1;
        t.row(
            &format!("the reference shot, Displacement Map {what}: the processor's frame 102 differs from the same displacement with every colour at 100, so the comparisons below test the colours parting"),
            &format!("{parted} pixels changed"),
            parted > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 1, 100, 102, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Displacement Map {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// A map of waves, B-128's: red in bands down the picture, green in waves across it.
fn waves() -> Vec<u8> {
    let (w, h) = TOWN;
    let tau = std::f64::consts::TAU;
    let byte = |v: f64| (255.0 * v).round() as u8;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (x, y) = (x as f64, y as f64);
            let r = 0.5 + 0.5 * (tau * y / 48.0).sin();
            let g = 0.5 + 0.5 * (tau * x / 70.0 + y / 40.0).sin();
            bytes.extend([byte(r), byte(g), 128, 255]);
        }
    }
    bytes
}

/// B-128's glass ball: inside it red and green point back to its middle; outside it the map is
/// clear.
fn ball() -> Vec<u8> {
    let (w, h) = TOWN;
    let (cx, cy, r) = (480.0 * 0.62, 270.0 * 0.55, 270.0 * 0.38);
    let byte = |v: f64| (255.0 * v.clamp(0.0, 1.0)).round() as u8;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (px, py) = (x as f64 + 0.5, y as f64 + 0.5);
            let inside = (px - cx).hypot(py - cy) <= r;
            bytes.extend([byte(0.5 - 0.5 * (px - cx) / r), byte(0.5 - 0.5 * (py - cy) / r), 128, if inside { 255 } else { 0 }]);
        }
    }
    bytes
}

/// The street as the holder and `map` as the switched-off `ramp` layer, with `effects` on the
/// street, drawn at frame 0, as B-128 draws it.
fn picture(dir: &Path, map: &str, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/displacement_map/fx_dmap_002.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    project["assets"][1]["path"] = J::from(map);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    for layer in comp["layers"].as_array_mut().unwrap() {
        if layer["id"] == "holder" {
            layer["effects"] = effects.clone();
        }
    }
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

/// The `holder` layer's Displacement Map in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

fn amounts(e: &mut Effect, r: f64, g: f64, b: f64, n: f64) {
    if let Effect::DisplacementMap { red_amount, green_amount, blue_amount, spectrum, .. } = e {
        (*red_amount, *green_amount, *blue_amount, *spectrum) = (r, g, b, n);
    }
}

fn set(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: Id::new(MAIN), layer_id: Id::new("holder"), instance_id: Id::new("fx-1"), effect }
}

fn keys(setting: &str, values: &[(i32, f64)]) -> Command {
    Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        setting: setting.to_string(),
        keys: values.iter().map(|&(frame, v)| EffectKey { frame, value: vec![v], interp: Interp::Linear }).collect(),
    }
}

/// `json` opened and drawn at `frame` on the processor, from the folder `root`.
fn draw(json: &J, root: &Path, frame: i32) -> Vec<f32> {
    let loaded = persist::load_str(&json.to_string()).expect("the file reads");
    let mut log = FrameLog::new(3);
    render_frame(loaded.document.project(), &Id::new(MAIN), frame, root, 64, &mut log).expect("the frame draws").data().to_vec()
}

#[test]
fn b291_map_chromatic() {
    let mut t = Table::new(
        "map_chromatic",
        "# D-412: Map Chromatic Displacement\n\nB-291: PLUGINS.md's pick #19, Prism Displacement and Red \
         Giant's Chromatic Displacement merged into `core.displacement_map`: Red, Green and Blue Amount \
         (`red_amount`, `green_amount`, `blue_amount`, -1000 to 1000 per cent of D-193's displacement) and \
         Spectrum (`spectrum`, 3 to 32 samples, taken whole), all keyable. The samples run from red \
         through green to blue, each read at its own share of the displacement; each colour is the \
         samples' mean weighted toward it, the covering the largest. 3 samples part three colours; more \
         give Red Giant's smooth rainbow. 100, 100, 100 and 3, what a file without them means, is \
         D-193's rule. Every expected pixel is `Fixtures/map_chromatic/expected_map_chromatic.json`, \
         written by `tools/map_chromatic_reference.py` before this code existed and printed in document \
         25 as FX-MAPCHROMA-001 to 019. Tolerance 2e-5.\n",
    );

    t.heading("FX-MAPCHROMA-001 to 019 (document 25)");
    t.fixtures_numbered("expected_map_chromatic.json", 1..=19);

    t.heading("Older projects: every colour moving alike, as before");
    let root = repo("Fixtures/displacement_map");
    for n in 1..=31 {
        let file = format!("fx_dmap_{n:03}.json");
        let text = fs::read_to_string(root.join(&file)).unwrap();
        let Ok(loaded) = persist::load_str(&text) else { continue };
        let old: J = serde_json::from_str(&text).unwrap();
        let kept = same_json(&saved(&loaded), &old);
        // Its twin with the amounts 100 and the spectrum 3 written draws the same frames.
        let mut twin = old.clone();
        let mut found = 0;
        for layer in twin["compositions"][0]["layers"].as_array_mut().unwrap() {
            for e in layer["effects"].as_array_mut().into_iter().flatten() {
                if e["type_id"] == "core.displacement_map" {
                    for (k, v) in [("red_amount", 100), ("green_amount", 100), ("blue_amount", 100), ("spectrum", 3)] {
                        e["parameters"][k] = J::from(v);
                    }
                    found += 1;
                }
            }
        }
        let same = found > 0 && [0, 4].iter().all(|&f| draw(&old, &root, f) == draw(&twin, &root, f));
        t.row(
            &format!("displacement_map/{file} is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Red, Green and Blue Amount 100 and Spectrum 3"),
            &format!("saved {}; {}", if kept { "the same" } else { "differently" }, if same { "bit-identical" } else { "differ" }),
            kept && same,
        );
    }

    t.heading("The file");
    let files: Vec<String> = (1..=19).map(|n| format!("fx_mapchroma_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let p = t.saved_parameters("fx_mapchroma_003.json");
    t.row(
        "fx_mapchroma_003.json is saved with Red 0, Green 100, Blue 200 and Spectrum 9",
        &p.to_string(),
        p["red_amount"] == 0.0 && p["green_amount"] == 100.0 && p["blue_amount"] == 200.0 && p["spectrum"] == 9.0,
    );
    for (file, what, want) in [
        ("fx_mapchroma_015.json", "Red Amount 1001", "red amount"),
        ("fx_mapchroma_016.json", "Blue Amount -1001", "blue amount"),
        ("fx_mapchroma_017.json", "Spectrum 2", "spectrum"),
        ("fx_mapchroma_018.json", "Spectrum 33", "spectrum"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        let says = why.contains(want) || why.contains(&want.replace(' ', "_"));
        t.row(&format!("{file} ({what}) is refused in a sentence naming {want}"), &why, says);
    }
    t.shape_refused(
        "fx_mapchroma_002.json",
        "a Displacement Map whose Spectrum is a word",
        r#"{"layer": "white", "fit": "stretch", "horizontal": "red", "max_horizontal": 2, "vertical": "green", "max_vertical": 2, "wrap": "off", "spectrum": "fine"}"#,
    );

    t.heading("How far it reaches");
    let mut draft = changed(&t, "fx_mapchroma_002.json", |_| {});
    draft.scale_distances(|d| d * 0.5);
    let kept = matches!(draft, Effect::DisplacementMap { max_horizontal, red_amount, green_amount, blue_amount, spectrum, .. }
        if max_horizontal == 1.0 && red_amount == 0.0 && green_amount == 100.0 && blue_amount == 200.0 && spectrum == 3.0);
    t.row("a half-size draft preview halves the maxima and leaves the amounts, shares and not distances, and the spectrum", &format!("{draft:?}"), kept);
    let grown = |r: f64, g: f64, b: f64| {
        changed(&t, "fx_mapchroma_002.json", |e| {
            amounts(e, r, g, b, 3.0);
            if let Effect::DisplacementMap { expand, max_horizontal, .. } = e {
                *expand = "on".into();
                *max_horizontal = 2.5;
            }
        })
        .bounds_expansion()
    };
    let (a, b, c) = (grown(100.0, 100.0, 100.0), grown(0.0, 100.0, 200.0), grown(-300.0, 50.0, 20.0));
    t.row(
        "with Expand Output and a maximum of 2.5, the drawing grows by 3 with every colour at 100, by 5 with Blue at 200, by 8 with Red at -300",
        &format!("{a}, {b}, {c}"),
        (a, b, c) == (3, 5, 8),
    );

    t.heading("Commands");
    let mut document = t.load("fx_mapchroma_002.json").document;
    let held = format!("{:?}", document.project().composition(&Id::new(MAIN)).unwrap());
    for (what, command) in [
        ("Red Amount 1001", set(changed(&t, "fx_mapchroma_002.json", |e| amounts(e, 1001.0, 100.0, 100.0, 3.0)))),
        ("Spectrum 2", set(changed(&t, "fx_mapchroma_002.json", |e| amounts(e, 0.0, 100.0, 200.0, 2.0)))),
        ("Green Amount keyed to 2000", keys("green_amount", &[(0, 100.0), (4, 2000.0)])),
    ] {
        let refused = document.apply(command).err();
        let untouched = format!("{:?}", document.project().composition(&Id::new(MAIN)).unwrap()) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
    let taken = [
        ("water: Red 50, Green 100, Blue 150, Spectrum 16", set(changed(&t, "fx_mapchroma_002.json", |e| amounts(e, 50.0, 100.0, 150.0, 16.0)))),
        ("Blue Amount keyed from 100 to 300", keys("blue_amount", &[(0, 100.0), (4, 300.0)])),
        ("Spectrum keyed from 3 to 32", keys("spectrum", &[(0, 3.0), (4, 32.0)])),
    ];
    let n = taken.len();
    for (what, command) in taken {
        let ok = document.apply(command).is_ok();
        t.row(&format!("{what} is taken"), if ok { "taken" } else { "refused" }, ok);
    }
    let before = t.render(&t.load("fx_mapchroma_002.json").document, 0, 64);
    for _ in 0..n {
        document.undo();
    }
    let same = t.render(&document, 0, 64).data() == before.data();
    t.row(&format!("undo {n} times: frame 0 is the frame it was"), if same { "byte-identical" } else { "differ" }, same);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_mapchroma_003.json", 0),
        ("fx_mapchroma_005.json", 0),
        ("fx_mapchroma_006.json", 0),
        ("fx_mapchroma_008.json", 0),
        ("fx_mapchroma_009.json", 0),
        ("fx_mapchroma_010.json", 0),
        ("fx_mapchroma_013.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, &[14, 15, 16, 17, 18, 19]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("red and green 12 pixels, water (Red 50, Green 100, Blue 150), Spectrum 3", json!({})),
            ("luminance -20 and alpha 8, wrapped, Red 0, Green 100, Blue 200, Spectrum 16", json!({"horizontal": "luminance", "max_horizontal": -20, "vertical": "alpha", "max_vertical": 8, "wrap": "on", "red_amount": 0, "blue_amount": 200, "spectrum": 16})),
            ("hue 15 and saturation -15, tiled, expanded, Red -100, Green 0, Blue 100, Spectrum 7", json!({"fit": "tile", "horizontal": "hue", "max_horizontal": 15, "vertical": "saturation", "max_vertical": -15, "expand": "on", "red_amount": -100, "green_amount": 0, "blue_amount": 100, "spectrum": 7})),
        ],
    );

    t.heading("Pictures: B-128's street and maps, in `verification/D-412 pictures/`");
    let dir = repo("verification/D-412 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    write("map_waves.png", &waves());
    write("map_ball.png", &ball());
    let dm = |most: f64, r: f64, g: f64, b: f64, n: f64| {
        json!([{"instance_id": "fx-1", "type_id": "core.displacement_map", "enabled": true, "parameters": {"layer": "ramp", "fit": "stretch",
            "horizontal": "red", "max_horizontal": most, "vertical": "green", "max_vertical": most, "wrap": "off",
            "red_amount": r, "green_amount": g, "blue_amount": b, "spectrum": n}}])
    };
    let shots = [
        ("1_waves_alike.png", "map_waves.png", dm(8.0, 100.0, 100.0, 100.0, 3.0), "waves, 8 pixels, every colour at 100: D-193's displacement as before"),
        ("2_waves_split.png", "map_waves.png", dm(8.0, 40.0, 100.0, 160.0, 3.0), "the same with Red 40, Green 100, Blue 160, Spectrum 3: three colours parted into hard fringes"),
        ("3_waves_spectrum.png", "map_waves.png", dm(8.0, 40.0, 100.0, 160.0, 16.0), "the same with Spectrum 16: the fringes a smooth rainbow, as through water"),
        ("4_ball_alike.png", "map_ball.png", dm(30.0, 100.0, 100.0, 100.0, 3.0), "the glass ball, 30 pixels, every colour at 100"),
        ("5_ball_prism.png", "map_ball.png", dm(30.0, 70.0, 100.0, 130.0, 12.0), "the glass ball with Red 70, Green 100, Blue 130, Spectrum 12: its rim parts into a rainbow, as glass does"),
    ];
    let mut drawn: Vec<Vec<u8>> = Vec::new();
    for (name, map, effects, what) in shots {
        let (p, s) = picture(&dir, map, effects);
        write(name, &p);
        let differs = drawn.last().map(|q| distance(&p, q).1);
        let ok = s.is_empty() && (name.starts_with('1') || name.starts_with('4') || differs.is_some_and(|d| d > 0));
        t.row(
            &format!("{name}, {what}; draws cleanly{}", if differs.is_some() && !name.starts_with('4') { ", and differs from the picture before" } else { "" }),
            &format!("{s:?}; {} pixels differ from the picture before", differs.unwrap_or(0)),
            ok,
        );
        drawn.push(p);
    }

    t.finish("D-412_map_chromatic_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, then the effect, every eighth frame
/// asked for as the viewer asks, whole. With `B291_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-291: a measurement, run deliberately with --release --ignored"]
fn b291_map_chromatic_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B291_CPU").is_ok();
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
        ("Noise, then Displacement Map 12 pixels, every colour at 100", Some(json!({"red_amount": 100, "blue_amount": 100}))),
        ("Noise, then Displacement Map 12 pixels, Red 50, Green 100, Blue 150, Spectrum 3", Some(json!({}))),
        ("Noise, then Displacement Map 12 pixels, Red 50, Green 100, Blue 150, Spectrum 16", Some(json!({"spectrum": 16}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}d"), p));
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
    let out = std::env::var("B291_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-291_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
