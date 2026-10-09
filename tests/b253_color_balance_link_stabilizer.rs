//! B-253 to B-255: D-374 Color Balance (HLS), D-375 Color Link and D-376 Color Stabilizer, after
//! After Effects' effects of those names ("Color Correction" in `docs/effects/EFFECTS.md`).
//!
//! Writes `verification/D-374_color_balance_hls_table.md`, `verification/D-375_color_link_table.md`
//! and `verification/D-376_color_stabilizer_table.md`.
//!
//! Every expected pixel is `Fixtures/color_balance_hls/expected_color_balance_hls.json`,
//! `Fixtures/color_link/expected_color_link.json` or
//! `Fixtures/color_stabilizer/expected_color_stabilizer.json`, written by the matching
//! `tools/*_reference.py` before this code existed and printed in document 25 as FX-HLSBAL-001
//! to 014, FX-CLINK-001 to 025 and FX-CSTAB-001 to 016. Tolerance 2e-5. Nothing here is a
//! snapshot of a run.
//!
//! Each also draws pictures into `verification/D-374 pictures/` (and D-375, D-376).

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, set, town, Table, MAIN, TOWN};
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

fn hls(hue: f64, lightness: f64, saturation: f64) -> Effect {
    Effect::ColorBalanceHls { hue, lightness, saturation }
}

fn link(layer: &str, sample: &str, clip: f64, stencil: &str, opacity: f64, blending_mode: &str) -> Effect {
    Effect::ColorLink {
        layer: J::from(layer),
        sample: sample.into(),
        clip,
        stencil: stencil.into(),
        opacity,
        blending_mode: blending_mode.into(),
        map: None,
    }
}

fn stabilizer(stabilize: &str, reference_frame: f64, points: [[f64; 2]; 3], sample_size: f64) -> Effect {
    Effect::ColorStabilizer {
        stabilize: stabilize.into(),
        reference_frame,
        black_point: points[0],
        mid_point: points[1],
        white_point: points[2],
        sample_size,
        reference: None,
    }
}

const POINTS: [[f64; 2]; 3] = [[25.0, 50.0], [50.0, 50.0], [75.0, 50.0]];

/// Set the `holder` layer's effect `instance` (the Color Link and Color Stabilizer files).
fn set_holder(instance: &str, effect: Effect) -> Command {
    Command::SetEffectParameters { composition: Id::new(MAIN), layer_id: Id::new("holder"), instance_id: Id::new(instance), effect }
}

fn keys_holder(instance: &str, setting: &str, values: &[(i32, &[f64])]) -> Command {
    Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new(instance),
        setting: setting.to_string(),
        keys: values.iter().map(|&(frame, v)| EffectKey { frame, value: v.to_vec(), interp: Interp::Linear }).collect(),
    }
}

fn holder_effect(d: &Document) -> Effect {
    d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].effect.clone()
}

/// Each command on the `holder` file is refused with a sentence and leaves it as it was; then
/// each taken one is taken, and as many undos give back `file`'s frame 0 exactly.
fn holder_commands(t: &mut Table, file: &str, refused: Vec<(&str, Command)>, taken: Vec<(&str, Command)>) {
    let mut document = t.load(file).document;
    let held = format!("{:?}", document.project());
    for (what, command) in refused {
        let why = document.apply(command).err();
        let untouched = format!("{:?}", document.project()) == held;
        t.row(&format!("{what} is refused with a sentence, and nothing changes"), &why.as_ref().map_or("taken".into(), |d| d.message.clone()), why.is_some() && untouched);
    }
    let n = taken.len();
    for (what, command) in taken {
        let ok = document.apply(command).is_ok();
        t.row(&format!("{what} is taken"), if ok { "taken" } else { "refused" }, ok);
    }
    let before = t.render(&t.load(file).document, 0, 64);
    for _ in 0..n {
        document.undo();
    }
    let same = t.render(&document, 0, 64).data() == before.data();
    t.row(&format!("undo {n} times: frame 0 is the frame it was"), if same { "byte-identical" } else { "differ" }, same);
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
        "core.color_balance_hls" => json!({"hue": 0, "lightness": 0, "saturation": 0}),
        "core.color_link" => json!({"layer": "", "sample": "average", "clip": 5, "stencil": "off", "opacity": 100, "blending_mode": "normal"}),
        "core.color_stabilizer" => json!({
            "stabilize": "brightness", "reference_frame": 0, "black_point": [25, 50], "mid_point": [50, 50],
            "white_point": [75, 50], "sample_size": 5}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` change nothing, are left out with a warning, or stay on the
/// processor, so the card is not asked.
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
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor; `card` of 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, name: &str, type_id: &str, settings: &[(&str, J)], pick: fn(&Effect) -> bool, want: usize) {
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
        let project = reference(|id| json!([fx(type_id, &format!("b253-{id}"), p)]));
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
                    d.0 <= 1 && !r && a == b && card == want,
                );
            }
        }
    }
}

fn colour_run(e: &Effect) -> bool {
    matches!(e, Effect::ColorBalanceHls { .. } | Effect::ColorLink { .. })
}

/// Color Balance (HLS), Levels and Color Link in one run on the reference shot: the card fuses
/// the colour run into one pass, and it still agrees with the processor.
fn card_fused(t: &mut Table, gpu: &mut Gpu) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let project = reference(|id| {
        json!([
            fx("core.color_balance_hls", &format!("{id}h"), &json!({"hue": 150, "lightness": -10, "saturation": 25})),
            fx("core.levels", &format!("{id}l"), &json!({"input_black": 20, "input_white": 230, "gamma": 1.2, "output_black": 10, "output_white": 245})),
            fx("core.color_link", &format!("{id}c"), &json!({"layer": "layer-4", "sample": "median", "opacity": 40, "blending_mode": "overlay", "stencil": "on"})),
        ])
    });
    for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
        for frame in [0, 100, 239] {
            let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
            let card = on_card(&project, &ref_comp, &ref_root, frame, quality, colour_run);
            t.row(
                &format!("the reference shot, Color Balance (HLS) (150, -10, 25), Levels and Color Link (layer4's median, overlay 40) in one colour run on three layers, frame {frame}, {}", quality.label()),
                &format!("largest difference {} of 255, {} pixels differ; {card} of 6 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                d.0 <= 1 && !r && a == b && card == 6,
            );
        }
    }
}

/// `effects` on the picture `asset` in `dir`, drawn, straight 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from(asset);
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
    t.heading(&format!("Pictures: in `verification/{d} pictures/`"));
    let dir = repo(&format!("verification/{d} pictures"));
    fs::create_dir_all(&dir).unwrap();
    let out = dir.clone();
    let write = move |name: &str, bytes: &[u8]| png_out::write_rgba(&out.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    (dir, write)
}

fn px(b: &[u8], i: usize) -> [u8; 3] {
    [b[i * 4], b[i * 4 + 1], b[i * 4 + 2]]
}

/// Where the street first has the colour `c`.
fn first(bytes: &[u8], c: [u8; 3]) -> usize {
    bytes.chunks_exact(4).position(|p| p[..3] == c).expect("the street has that colour")
}

// --- Color Balance (HLS) ----------------------------------------------------------------------

fn is_hls(e: &Effect) -> bool {
    matches!(e, Effect::ColorBalanceHls { .. })
}

#[test]
fn b253_color_balance_hls() {
    let mut t = Table::new(
        "color_balance_hls",
        "# D-374: Color Balance (HLS)\n\nB-253, after After Effects' Color Balance (HLS): every colour \
         turned round the wheel by the hue, and its HLS lightness and saturation moved by the same \
         amounts everywhere, a grey keeping no saturation. Every expected pixel is \
         `Fixtures/color_balance_hls/expected_color_balance_hls.json`, written by \
         `tools/color_balance_hls_reference.py` before this code existed and printed in document 25 \
         as FX-HLSBAL-001 to 014. Tolerance 2e-5.\n",
    );

    t.heading("FX-HLSBAL-001 to 014 (document 25)");
    t.fixtures("expected_color_balance_hls.json");

    t.heading("The file");
    let files: Vec<String> = (1..=14).map(|n| format!("fx_hlsbal_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_hlsbal_009.json");
    t.row(
        "fx_hlsbal_009.json is saved with its hue, lightness and saturation",
        &saved.to_string(),
        saved["hue"] == 60 && saved["lightness"] == -20 && saved["saturation"] == 20,
    );
    for (file, want) in [
        ("fx_hlsbal_012.json", "hue"),
        ("fx_hlsbal_013.json", "lightness"),
        ("fx_hlsbal_014.json", "saturation"),
    ] {
        let d = t.load(file).document;
        let why = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.why_invalid();
        t.row(&format!("{file} is refused in a sentence naming its {want}"), &why, why.contains(want));
    }
    t.shape_refused("fx_hlsbal_001.json", "a Color Balance (HLS) with no `hue`", r#"{"lightness": 0, "saturation": 0}"#);
    t.shape_refused("fx_hlsbal_001.json", "a Color Balance (HLS) whose saturation is a word", r#"{"hue": 0, "lightness": 0, "saturation": "0"}"#);

    t.heading("Commands");
    let mut document = t.load("fx_hlsbal_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("hue 3600.5", set(hls(3600.5, 0.0, 0.0))),
            ("lightness -100.5", set(hls(0.0, -100.5, 0.0))),
            ("saturation keyed to 120", keys("saturation", &[(0, &[0.0]), (4, &[120.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_hlsbal_001.json",
        vec![("hue 60, lightness -20, saturation 20", set(hls(60.0, -20.0, 20.0))), ("hue keyed from 0 to 240", keys("hue", &[(0, &[0.0]), (4, &[240.0])]))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_hlsbal_002.json", 0), ("fx_hlsbal_005.json", 0), ("fx_hlsbal_009.json", 0), ("fx_hlsbal_011.json", 3)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_hlsbal", 14, &[1, 12, 13, 14], is_hls);
    card_reference(
        &mut t,
        &mut gpu,
        "Color Balance (HLS)",
        "core.color_balance_hls",
        &[
            ("hue 120", json!({"hue": 120})),
            ("hue -60, lightness 20, saturation 40", json!({"hue": -60, "lightness": 20, "saturation": 40})),
            ("saturation -100", json!({"saturation": -100})),
        ],
        is_hls,
        3,
    );
    card_fused(&mut t, &mut gpu);

    let (dir, write) = pictures(&mut t, "D-374");
    let street = town();
    let red = first(&street, [180, 90, 70]);
    let (before, said) = picture(&dir, "town.png", json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let one = |p: J| json!([fx("core.color_balance_hls", "fx-0-0", &p)]);
    let (as_added, said) = picture(&dir, "town.png", one(json!({})));
    write("2_as_added.png", &as_added);
    t.row(
        "2_as_added.png, as it starts (all three 0): nothing changes; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&as_added, &before).1),
        said.is_empty() && distance(&as_added, &before).0 == 0,
    );
    let (turned, said) = picture(&dir, "town.png", one(json!({"hue": 120})));
    write("3_hue_120.png", &turned);
    t.row(
        "3_hue_120.png, hue 120: every colour a third of the way round, the red-brown houses green; draws cleanly",
        &format!("{said:?}, a red wall {:?} from {:?}", px(&turned, red), px(&before, red)),
        said.is_empty() && turned[red * 4 + 1] > turned[red * 4],
    );
    let (grey, said) = picture(&dir, "town.png", one(json!({"saturation": -100})));
    write("4_saturation_-100.png", &grey);
    let greyed = grey.chunks_exact(4).all(|p| p[0].abs_diff(p[1]) <= 1 && p[1].abs_diff(p[2]) <= 1);
    t.row("4_saturation_-100.png, saturation -100: the street in greys; draws cleanly", &format!("{said:?}, every pixel grey: {greyed}"), said.is_empty() && greyed);
    let (dark, said) = picture(&dir, "town.png", one(json!({"lightness": -30})));
    write("5_lightness_-30.png", &dark);
    let darker = dark.chunks_exact(4).zip(before.chunks_exact(4)).all(|(d, b)| d[..3].iter().zip(&b[..3]).all(|(x, y)| x <= y));
    t.row("5_lightness_-30.png, lightness -30: every pixel darker or as dark; draws cleanly", &format!("{said:?}, every pixel darker or the same: {darker}"), said.is_empty() && darker);

    t.finish("D-374_color_balance_hls_table.md");
}

// --- Color Link ----------------------------------------------------------------------------------

fn is_link(e: &Effect) -> bool {
    matches!(e, Effect::ColorLink { .. })
}

#[test]
fn b254_color_link() {
    let mut t = Table::new(
        "color_link",
        "# D-375: Color Link\n\nB-254, after After Effects' Color Link: one colour read from a whole \
         layer's picture (its average, median, brightest, darkest, or each channel's highest or \
         lowest, a share of each end clipped), laid over the layer by a blending mode at an \
         opacity, only where it shows or over all of it. Every expected pixel is \
         `Fixtures/color_link/expected_color_link.json`, written by `tools/color_link_reference.py` \
         before this code existed and printed in document 25 as FX-CLINK-001 to 025. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-CLINK-001 to 025 (document 25)");
    t.fixtures_numbered("expected_color_link.json", 1..=25);

    t.heading("The file");
    let files: Vec<String> = (1..=25).map(|n| format!("fx_clink_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_clink_012.json");
    t.row(
        "fx_clink_012.json is saved with its layer, sample, clip, stencil, opacity and blending mode, and no picture",
        &saved.to_string(),
        saved["layer"] == "swatch" && saved["blending_mode"] == "multiply" && saved["opacity"] == 50 && saved["stencil"] == "on" && saved.get("map").is_none(),
    );
    for (file, want) in [
        ("fx_clink_022.json", "\"mean\""),
        ("fx_clink_023.json", "\"yes\""),
        ("fx_clink_024.json", "\"color_dodge\""),
        ("fx_clink_025.json", "\"Normal\""),
    ] {
        let why = holder_effect(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    t.shape_refused("fx_clink_002.json", "a Color Link with no `sample`", r#"{"layer": "swatch", "clip": 5, "stencil": "off", "opacity": 100, "blending_mode": "normal"}"#);
    t.shape_refused("fx_clink_002.json", "a Color Link whose opacity is a word", r#"{"layer": "swatch", "sample": "average", "clip": 5, "stencil": "off", "opacity": "100", "blending_mode": "normal"}"#);

    t.heading("Commands");
    holder_commands(
        &mut t,
        "fx_clink_002.json",
        vec![
            ("clip 49.5", set_holder("fx-0-0", link("swatch", "average", 49.5, "off", 100.0, "normal"))),
            ("opacity 100.5", set_holder("fx-0-0", link("swatch", "average", 5.0, "off", 100.5, "normal"))),
            ("sample \"Median\" written with a capital", set_holder("fx-0-0", link("swatch", "Median", 5.0, "off", 100.0, "normal"))),
            ("blending mode \"difference\"", set_holder("fx-0-0", link("swatch", "average", 5.0, "off", 100.0, "difference"))),
        ],
        vec![
            ("the median, stencil on, overlay at 60", set_holder("fx-0-0", link("swatch", "median", 5.0, "on", 60.0, "overlay"))),
            ("opacity keyed from 0 to 100", keys_holder("fx-0-0", "opacity", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_clink_001.json", 0), ("fx_clink_002.json", 3), ("fx_clink_009.json", 0), ("fx_clink_016.json", 0), ("fx_clink_018.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    t.row(
        "the layer's own picture (no source layer) stays on the processor, since the card does not read a layer's colour back yet",
        "FX-CLINK-001 below: 0 frames on the card",
        true,
    );
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_clink", 25, &[1, 19, 20, 21, 22, 23, 24, 25], is_link);
    card_reference(
        &mut t,
        &mut gpu,
        "Color Link",
        "core.color_link",
        &[
            ("layer4's average, normal at 50", json!({"layer": "layer-4", "opacity": 50})),
            ("layer4's brightest, multiply, stencil on", json!({"layer": "layer-4", "sample": "brightest", "blending_mode": "multiply", "stencil": "on"})),
            ("layer4's median, soft light at 80", json!({"layer": "layer-4", "sample": "median", "blending_mode": "soft_light", "opacity": 80})),
        ],
        is_link,
        3,
    );

    let (dir, write) = pictures(&mut t, "D-375");
    let (before, said) = picture(&dir, "town.png", json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let one = |p: J| json!([fx("core.color_link", "fx-0-0", &p)]);
    let (flat, said) = picture(&dir, "town.png", one(json!({})));
    write("2_as_added.png", &flat);
    let colour = px(&flat, 0);
    let one_colour = flat.chunks_exact(4).all(|p| p[..3] == colour);
    t.row(
        "2_as_added.png, as it starts (its own average, normal at 100): the whole street one flat colour, its clipped average; draws cleanly",
        &format!("{said:?}, every pixel {colour:?}: {one_colour}"),
        said.is_empty() && one_colour,
    );
    let (wash, said) = picture(&dir, "town.png", one(json!({"opacity": 50, "blending_mode": "overlay", "stencil": "on"})));
    write("3_overlay_50.png", &wash);
    t.row(
        "3_overlay_50.png, its own average laid over it by overlay at 50: the street keeps its light and dark under a wash of its own average colour; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&wash, &before).1),
        said.is_empty() && distance(&wash, &before).1 > 0,
    );
    let (bright, said) = picture(&dir, "town.png", one(json!({"sample": "brightest", "opacity": 40, "blending_mode": "multiply"})));
    write("4_brightest_multiply_40.png", &bright);
    let darker = bright.chunks_exact(4).zip(before.chunks_exact(4)).all(|(d, b)| d[..3].iter().zip(&b[..3]).all(|(x, y)| x <= y));
    t.row(
        "4_brightest_multiply_40.png, its brightest colour by multiply at 40: a little darker everywhere, tinted by the lit windows; draws cleanly",
        &format!("{said:?}, every pixel darker or the same: {darker}"),
        said.is_empty() && darker,
    );

    t.finish("D-375_color_link_table.md");
}

// --- Color Stabilizer -----------------------------------------------------------------------------

fn is_stabilizer(e: &Effect) -> bool {
    matches!(e, Effect::ColorStabilizer { .. })
}

/// Fixture `file`'s frame drawn, straight 8-bit, with its effects or without, grown 24 times so
/// the owner can see each pixel.
fn grown(t: &Table, file: &str, frame: i32, effects: bool) -> Vec<u8> {
    let mut j: J = serde_json::from_str(&fs::read_to_string(t.root.join(file)).unwrap()).unwrap();
    if !effects {
        for layer in j["compositions"][0]["layers"].as_array_mut().unwrap() {
            layer["effects"] = json!([]);
        }
    }
    let loaded = persist::load_str(&j.to_string()).unwrap();
    let small = t.render(&loaded.document, frame, 64).to_srgb8_straight();
    let (w, h, k) = (16, 10, 24);
    let mut out = vec![0u8; w * k * h * k * 4];
    for y in 0..h * k {
        for x in 0..w * k {
            let from = ((y / k) * w + x / k) * 4;
            out[(y * w * k + x) * 4..][..4].copy_from_slice(&small[from..from + 4]);
        }
    }
    out
}

#[test]
fn b255_color_stabilizer() {
    let mut t = Table::new(
        "color_stabilizer",
        "# D-376: Color Stabilizer\n\nB-255, after After Effects' Color Stabilizer: one, two or three \
         points sampled at a reference frame, and every frame corrected so the same points match \
         them, by brightness, levels or curves. Every expected pixel is \
         `Fixtures/color_stabilizer/expected_color_stabilizer.json`, written by \
         `tools/color_stabilizer_reference.py` before this code existed and printed in document 25 \
         as FX-CSTAB-001 to 016. Tolerance 2e-5.\n",
    );

    t.heading("FX-CSTAB-001 to 016 (document 25)");
    t.fixtures("expected_color_stabilizer.json");

    t.heading("The file");
    let files: Vec<String> = (1..=16).map(|n| format!("fx_cstab_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_cstab_008.json");
    t.row(
        "fx_cstab_008.json is saved with its mode, reference frame, three points and sample size, and no samples",
        &saved.to_string(),
        saved["stabilize"] == "levels" && saved["black_point"] == json!([10, 20]) && saved["white_point"] == json!([90, 80]) && saved.get("reference").is_none(),
    );
    for (file, want) in [("fx_cstab_013.json", "\"colour\""), ("fx_cstab_014.json", "\"Levels\"")] {
        let why = holder_effect(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    t.shape_refused(
        "fx_cstab_001.json",
        "a Color Stabilizer whose black point is one number",
        r#"{"stabilize": "brightness", "reference_frame": 0, "black_point": 25, "mid_point": [50, 50], "white_point": [75, 50], "sample_size": 5}"#,
    );
    t.shape_refused(
        "fx_cstab_001.json",
        "a Color Stabilizer with no `reference_frame`",
        r#"{"stabilize": "brightness", "black_point": [25, 50], "mid_point": [50, 50], "white_point": [75, 50], "sample_size": 5}"#,
    );

    t.heading("Commands");
    holder_commands(
        &mut t,
        "fx_cstab_001.json",
        vec![
            ("sample size 100.5", set_holder("fx-1", stabilizer("brightness", 0.0, POINTS, 100.5))),
            ("reference frame -0.5", set_holder("fx-1", stabilizer("brightness", -0.5, POINTS, 5.0))),
            ("stabilize \"color\"", set_holder("fx-1", stabilizer("color", 0.0, POINTS, 5.0))),
            ("the white point at x 1000.5", set_holder("fx-1", stabilizer("levels", 0.0, [POINTS[0], POINTS[1], [1000.5, 50.0]], 5.0))),
        ],
        vec![
            ("curves at reference frame 2", set_holder("fx-1", stabilizer("curves", 2.0, POINTS, 3.0))),
            ("the black point keyed from (25, 50) to (5, 50)", keys_holder("fx-1", "black_point", &[(0, &[25.0, 50.0]), (4, &[5.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_cstab_001.json", 1), ("fx_cstab_002.json", 2), ("fx_cstab_003.json", 3), ("fx_cstab_008.json", 2)]);

    t.heading("On the card against the processor: within 1 level of 255");
    t.row(
        "Color Stabilizer stays on the processor, since its reference frame is read there; the card draws the rest of the frame round it",
        "every file below: 0 frames on the card",
        true,
    );
    let mut gpu = Gpu::new().expect("a usable card");
    let all: Vec<u32> = (1..=16).collect();
    card_fixtures(&mut t, &mut gpu, "fx_cstab", 16, &all, is_stabilizer);
    // The reference shot's drawings hold still, so a Color Stabilizer alone finds nothing to
    // mend there; an Exposure Flicker before it gives it a flicker to take out.
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let flicker = |id: &str| fx("core.exposure_flicker", &format!("b253-{id}f"), &json!({"amount": 0.25, "hold": 1, "seed": 3}));
    let cpu100 = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let flickering = cpu100(&reference(|id| json!([flicker(id)])));
    for (what, p) in [("brightness, reference frame 0", json!({})), ("curves, reference frame 120", json!({"stabilize": "curves", "reference_frame": 120}))] {
        let project = reference(|id| json!([flicker(id), fx("core.color_stabilizer", &format!("b253-{id}s"), &p)]));
        let changed = distance(&cpu100(&project), &flickering).1;
        t.row(
            &format!("the reference shot, an Exposure Flicker then Color Stabilizer {what}: the processor's frame 100 differs from the flicker alone, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, an Exposure Flicker then Color Stabilizer {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; refused by the card: {r}; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && on_card(&project, &ref_comp, &ref_root, frame, quality, is_stabilizer) == 0,
                );
            }
        }
    }

    t.heading("Pictures: in `verification/D-376 pictures/`, each pixel grown 24 times");
    let dir = repo("verification/D-376 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), 16 * 24, 10 * 24, OutputDepth::Eight, &[], bytes).unwrap();
    let reference = grown(&t, "fx_cstab_002.json", 0, false);
    let cast = grown(&t, "fx_cstab_002.json", 2, false);
    let mended = grown(&t, "fx_cstab_002.json", 2, true);
    write("1_reference_frame_0.png", &reference);
    write("2_frame_2_before.png", &cast);
    write("3_frame_2_levels.png", &mended);
    let (was, now) = (distance(&cast, &reference), distance(&mended, &reference));
    t.row(
        "1_reference_frame_0.png, 2_frame_2_before.png and 3_frame_2_levels.png: frame 2's colour cast and flattened contrast (before) are taken out by Levels, so it looks like frame 0 again",
        &format!("frame 2 against frame 0: largest difference {} of 255 before, {} after", was.0, now.0),
        now.0 < was.0 && now.0 <= 3,
    );
    let lifted = grown(&t, "fx_cstab_001.json", 1, false);
    let back = grown(&t, "fx_cstab_001.json", 1, true);
    write("4_frame_1_before.png", &lifted);
    write("5_frame_1_brightness.png", &back);
    let (was, now) = (distance(&lifted, &reference), distance(&back, &reference));
    t.row(
        "4_frame_1_before.png and 5_frame_1_brightness.png: frame 1, lifted by 20, is brought back down to frame 0's brightness",
        &format!("frame 1 against frame 0: largest difference {} of 255 before, {} after", was.0, now.0),
        now.0 < was.0 && now.0 <= 1,
    );

    t.finish("D-376_color_stabilizer_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B253_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-253: a measurement, run deliberately with --release --ignored"]
fn b253_colour_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B253_CPU").is_ok();
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
    let shots: [(&str, Option<(&str, J)>); 5] = [
        ("Noise alone", None),
        ("Noise, then Color Balance (HLS), hue 120, saturation 30", Some(("core.color_balance_hls", json!({"hue": 120, "saturation": 30})))),
        ("Noise, then Color Link, layer4's average, overlay at 50", Some(("core.color_link", json!({"layer": "layer-4", "opacity": 50, "blending_mode": "overlay"})))),
        ("Noise, then Color Link, its own average, overlay at 50", Some(("core.color_link", json!({"opacity": 50, "blending_mode": "overlay"})))),
        ("Noise, then Color Stabilizer, levels to frame 0", Some(("core.color_stabilizer", json!({"stabilize": "levels"})))),
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
    let out = std::env::var("B253_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-253_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
