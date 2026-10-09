//! B-244 to B-246: D-365 Broadcast Safe, after After Effects' Broadcast Colors; D-366 Color
//! Neutralizer, after CycoreFX's CC Color Neutralizer; D-367 Color Offset, after CycoreFX's CC
//! Color Offset ("Color Correction" in `docs/effects/EFFECTS.md`).
//!
//! Writes `verification/D-365_broadcast_safe_table.md`, `verification/D-366_color_neutralizer_table.md`
//! and `verification/D-367_color_offset_table.md`.
//!
//! Every expected pixel is `Fixtures/broadcast_safe/expected_broadcast_safe.json`,
//! `Fixtures/color_neutralizer/expected_color_neutralizer.json` or
//! `Fixtures/color_offset/expected_color_offset.json`, written by the matching
//! `tools/*_reference.py` before this code existed and printed in document 25 as FX-BCAST-001 to
//! 015, FX-NEUTRAL-001 to 018 and FX-COFFSET-001 to 016. Tolerance 2e-5. Nothing here is a
//! snapshot of a run.
//!
//! Each also draws the street into `verification/D-365 pictures/` (and D-366, D-367).

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

fn bcast(locale: &str, method: &str, max_amplitude: f64) -> Effect {
    Effect::BroadcastSafe { locale: locale.into(), method: method.into(), max_amplitude }
}

fn neutral(unbalance: [&str; 3], numbers: [[f64; 3]; 3], pinning: f64, black_point: f64, white_point: f64) -> Effect {
    Effect::ColorNeutralizer {
        shadows_unbalance: unbalance[0].into(),
        midtones_unbalance: unbalance[1].into(),
        highlights_unbalance: unbalance[2].into(),
        shadows: numbers[0].to_vec(),
        midtones: numbers[1].to_vec(),
        highlights: numbers[2].to_vec(),
        pinning,
        black_point,
        white_point,
    }
}

fn offset(red_phase: f64, green_phase: f64, blue_phase: f64, overflow: &str) -> Effect {
    Effect::ColorOffset { red_phase, green_phase, blue_phase, overflow: overflow.into() }
}

const GREY: [&str; 3] = ["#000000", "#808080", "#ffffff"];
const NONE: [[f64; 3]; 3] = [[0.0; 3]; 3];

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
        "core.broadcast_safe" => json!({"locale": "ntsc", "method": "reduce_luminance", "max_amplitude": 110}),
        "core.color_neutralizer" => json!({
            "shadows_unbalance": "#000000", "midtones_unbalance": "#808080", "highlights_unbalance": "#ffffff",
            "shadows": [0, 0, 0], "midtones": [0, 0, 0], "highlights": [0, 0, 0],
            "pinning": 0, "black_point": 0, "white_point": 255}),
        "core.color_offset" => json!({"red_phase": 0, "green_phase": 0, "blue_phase": 0, "overflow": "wrap"}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b244-{id}"), p)]));
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

fn ours(e: &Effect) -> bool {
    matches!(e, Effect::BroadcastSafe { .. } | Effect::ColorNeutralizer { .. } | Effect::ColorOffset { .. })
}

/// All three in one run with Levels between, on the reference shot: the card fuses the colour
/// run into one pass, and it still agrees with the processor.
fn card_fused(t: &mut Table, gpu: &mut Gpu) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let project = reference(|id| {
        json!([
            fx("core.color_neutralizer", &format!("{id}n"), &json!({"shadows_unbalance": "#3c2d1e", "midtones_unbalance": "#968064", "highlights_unbalance": "#f0e6c8", "pinning": 30})),
            fx("core.levels", &format!("{id}l"), &json!({"input_black": 20, "input_white": 230, "gamma": 1.2, "output_black": 10, "output_white": 245})),
            fx("core.color_offset", &format!("{id}o"), &json!({"red_phase": 40, "blue_phase": -60, "overflow": "polarize"})),
            fx("core.broadcast_safe", &format!("{id}b"), &json!({"max_amplitude": 100})),
        ])
    });
    for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
        for frame in [0, 100, 239] {
            let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
            let card = on_card(&project, &ref_comp, &ref_root, frame, quality, ours);
            t.row(
                &format!(
                    "the reference shot, Color Neutralizer, Levels, Color Offset (polarize) and Broadcast Safe (100 IRE) in one colour run on three layers, frame {frame}, {}",
                    quality.label()
                ),
                &format!("largest difference {} of 255, {} pixels differ; {card} of 9 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                d.0 <= 1 && !r && a == b && card == 9,
            );
        }
    }
}

/// `effects` on the picture `asset` in `dir`, drawn, straight 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/broadcast_safe/fx_bcast_001.json")).unwrap()).unwrap();
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
    t.heading(&format!("Pictures: the street, in `verification/{d} pictures/`"));
    let dir = repo(&format!("verification/{d} pictures"));
    fs::create_dir_all(&dir).unwrap();
    let out = dir.clone();
    let write = move |name: &str, bytes: &[u8]| png_out::write_rgba(&out.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    (dir, write)
}

/// Where the street first has the colour `c`.
fn first(bytes: &[u8], c: [u8; 3]) -> usize {
    bytes.chunks_exact(4).position(|p| p[..3] == c).expect("the street has that colour")
}

// --- Broadcast Safe --------------------------------------------------------------------------

fn is_bcast(e: &Effect) -> bool {
    matches!(e, Effect::BroadcastSafe { .. })
}

#[test]
fn b244_broadcast_safe() {
    let mut t = Table::new(
        "broadcast_safe",
        "# D-365: Broadcast Safe\n\nB-244, after After Effects' Broadcast Colors: a pixel whose \
         brightness and colour together make a video signal above the limit, in IRE, is darkened, \
         greyed, or keyed out, or only those pixels are kept. Every expected pixel is \
         `Fixtures/broadcast_safe/expected_broadcast_safe.json`, written by \
         `tools/broadcast_safe_reference.py` before this code existed and printed in document 25 \
         as FX-BCAST-001 to 015. Tolerance 2e-5.\n",
    );

    t.heading("FX-BCAST-001 to 015 (document 25)");
    t.fixtures("expected_broadcast_safe.json");

    t.heading("The file");
    let files: Vec<String> = (1..=15).map(|n| format!("fx_bcast_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_bcast_008.json");
    t.row(
        "fx_bcast_008.json is saved with its locale, method and limit",
        &saved.to_string(),
        saved["locale"] == "pal" && saved["method"] == "reduce_saturation" && saved["max_amplitude"] == 95,
    );
    let effect_of = |d: &anime_compositor::command::Document| {
        d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone()
    };
    for (file, want) in [
        ("fx_bcast_013.json", "Broadcast Safe's locale is \"ntsc\" or \"pal\", and this is \"secam\"."),
        ("fx_bcast_015.json", "Broadcast Safe's method is \"reduce_luminance\", \"reduce_saturation\", \"key_out_unsafe\" or \"key_out_safe\", and this is \"reduce\"."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == want);
    }
    t.shape_refused("fx_bcast_001.json", "a Broadcast Safe with no `method`", r#"{"locale": "ntsc", "max_amplitude": 110}"#);
    t.shape_refused("fx_bcast_001.json", "a Broadcast Safe whose locale is a number", r#"{"locale": 1, "method": "reduce_luminance", "max_amplitude": 110}"#);

    t.heading("Commands");
    let mut document = t.load("fx_bcast_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("limit 120.5", set(bcast("ntsc", "reduce_luminance", 120.5))),
            ("limit 89.5", set(bcast("ntsc", "reduce_luminance", 89.5))),
            ("locale \"Pal\", written with a capital", set(bcast("Pal", "reduce_luminance", 110.0))),
            ("method \"darken\"", set(bcast("ntsc", "darken", 110.0))),
            ("limit keyed to 130", keys("max_amplitude", &[(0, &[100.0]), (4, &[130.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_bcast_001.json",
        vec![
            ("PAL, key out safe, 100 IRE,", set(bcast("pal", "key_out_safe", 100.0))),
            ("limit keyed from 90 to 120", keys("max_amplitude", &[(0, &[90.0]), (4, &[120.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_bcast_001.json", 0), ("fx_bcast_002.json", 0), ("fx_bcast_004.json", 0), ("fx_bcast_009.json", 2), ("fx_bcast_010.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_bcast", 15, &[11, 12, 13, 14, 15], is_bcast);
    card_reference(
        &mut t,
        &mut gpu,
        "Broadcast Safe",
        "core.broadcast_safe",
        &[
            ("NTSC, reduce luminance, 90 IRE", json!({"locale": "ntsc", "method": "reduce_luminance", "max_amplitude": 90})),
            ("PAL, reduce saturation, 95 IRE", json!({"locale": "pal", "method": "reduce_saturation", "max_amplitude": 95})),
            ("NTSC, key out unsafe, 90 IRE", json!({"locale": "ntsc", "method": "key_out_unsafe", "max_amplitude": 90})),
            ("NTSC, key out safe, 100 IRE", json!({"locale": "ntsc", "method": "key_out_safe", "max_amplitude": 100})),
        ],
        is_bcast,
    );
    card_fused(&mut t, &mut gpu);

    let (dir, write) = pictures(&mut t, "D-365");
    let street = town();
    let window = first(&street, [255, 240, 170]);
    let marking = first(&street, [250, 250, 250]);
    let (before, said) = picture(&dir, "town.png", json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let one = |p: J| json!([fx("core.broadcast_safe", "fx-0-0", &p)]);
    let (as_added, said) = picture(&dir, "town.png", one(json!({})));
    write("2_as_added.png", &as_added);
    t.row(
        "2_as_added.png, as it starts (NTSC, reduce luminance, 110 IRE): every colour in the street is safe, so nothing changes; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&as_added, &before).1),
        said.is_empty() && distance(&as_added, &before).0 == 0,
    );
    let (dark, said) = picture(&dir, "town.png", one(json!({"max_amplitude": 100})));
    write("3_reduce_luminance_100.png", &dark);
    let px = |b: &[u8], i: usize| format!("{:?}", &b[i * 4..i * 4 + 3]);
    t.row(
        "3_reduce_luminance_100.png, NTSC at 100 IRE: the lit windows (about 107 IRE) are darkened, the white road markings (about 98) are left; draws cleanly",
        &format!("{said:?}, a window {} from {}, a marking {} from {}", px(&dark, window), px(&before, window), px(&dark, marking), px(&before, marking)),
        said.is_empty() && dark[window * 4 + 1] < before[window * 4 + 1] && dark[marking * 4..marking * 4 + 4] == before[marking * 4..marking * 4 + 4],
    );
    let (grey, said) = picture(&dir, "town.png", one(json!({"max_amplitude": 100, "method": "reduce_saturation"})));
    write("4_reduce_saturation_100.png", &grey);
    let spread = |b: &[u8], i: usize| b[i * 4..i * 4 + 3].iter().max().unwrap() - b[i * 4..i * 4 + 3].iter().min().unwrap();
    t.row(
        "4_reduce_saturation_100.png, the same made greyer instead: the windows lose colour; draws cleanly",
        &format!("{said:?}, a window {} from {}", px(&grey, window), px(&before, window)),
        said.is_empty() && spread(&grey, window) < spread(&before, window),
    );
    let (keyed, said) = picture(&dir, "town.png", one(json!({"max_amplitude": 100, "method": "key_out_safe"})));
    write("5_key_out_safe_100.png", &keyed);
    let left = keyed.chunks_exact(4).filter(|p| p[3] > 0).count();
    t.row(
        "5_key_out_safe_100.png, key out safe at 100 IRE: only the unsafe pixels are left, the lit windows; draws cleanly",
        &format!("{said:?}, a window's covering {}, a marking's {}, {left} pixels left", keyed[window * 4 + 3], keyed[marking * 4 + 3]),
        said.is_empty() && keyed[window * 4 + 3] == 255 && keyed[marking * 4 + 3] == 0 && left > 0,
    );

    t.finish("D-365_broadcast_safe_table.md");
}

// --- Color Neutralizer -----------------------------------------------------------------------

fn is_neutral(e: &Effect) -> bool {
    matches!(e, Effect::ColorNeutralizer { .. })
}

/// The street with a warm cast: red up 24, green up 8, blue down 24.
fn warm_town() -> Vec<u8> {
    let mut bytes = town();
    for p in bytes.chunks_mut(4) {
        for (c, by) in p[..3].iter_mut().zip([24, 8, -24]) {
            *c = (*c as i32 + by).clamp(0, 255) as u8;
        }
    }
    bytes
}

#[test]
fn b245_color_neutralizer() {
    let mut t = Table::new(
        "color_neutralizer",
        "# D-366: Color Neutralizer\n\nB-245, after CycoreFX's CC Color Neutralizer: the colours \
         picked as the shadows', midtones' and highlights' cast are each pulled to grey at their \
         own lightness, the corrections faded between black and white, with levels to add, \
         pinning and black and white points. Every expected pixel is \
         `Fixtures/color_neutralizer/expected_color_neutralizer.json`, written by \
         `tools/color_neutralizer_reference.py` before this code existed and printed in document \
         25 as FX-NEUTRAL-001 to 018. Tolerance 2e-5.\n",
    );

    t.heading("FX-NEUTRAL-001 to 018 (document 25)");
    t.fixtures("expected_color_neutralizer.json");

    t.heading("The file");
    let files: Vec<String> = (1..=18).map(|n| format!("fx_neutral_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_neutral_006.json");
    t.row(
        "fx_neutral_006.json is saved with its colours, its three lists of levels, pinning and points",
        &saved.to_string(),
        saved["shadows"] == json!([20, 0, -20]) && saved["highlights_unbalance"] == "#ffffff" && saved["white_point"] == 255,
    );
    let effect_of = |d: &anime_compositor::command::Document| {
        d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone()
    };
    for (file, want) in [("fx_neutral_013.json", "shadows unbalance"), ("fx_neutral_014.json", "midtones are three numbers")] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming the {want}"), &why, why.starts_with("Color Neutralizer's") && why.contains(want));
    }
    t.shape_refused("fx_neutral_001.json", "a Color Neutralizer whose shadows are a word", r#"{"shadows": "20, 0, 0"}"#);
    t.shape_refused("fx_neutral_001.json", "a Color Neutralizer whose midtones unbalance is a number", r#"{"midtones_unbalance": 128}"#);

    t.heading("Commands");
    let mut document = t.load("fx_neutral_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("shadows red 255.5", set(neutral(GREY, [[255.5, 0.0, 0.0], [0.0; 3], [0.0; 3]], 0.0, 0.0, 255.0))),
            ("pinning -0.5", set(neutral(GREY, NONE, -0.5, 0.0, 255.0))),
            ("white point 255.5", set(neutral(GREY, NONE, 0.0, 0.0, 255.5))),
            ("highlights unbalance \"white\"", set(neutral(["#000000", "#808080", "white"], NONE, 0.0, 0.0, 255.0))),
            ("black point keyed to 300", keys("black_point", &[(0, &[0.0]), (4, &[300.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_neutral_001.json",
        vec![
            (
                "the three warm colours, pinning 50,",
                set(neutral(["#3c2d1e", "#968064", "#f0e6c8"], NONE, 50.0, 0.0, 255.0)),
            ),
            ("midtones keyed from 0, 0, 0 to 10, 20, 30", keys("midtones", &[(0, &[0.0, 0.0, 0.0]), (4, &[10.0, 20.0, 30.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_neutral_005.json", 0), ("fx_neutral_006.json", 0), ("fx_neutral_009.json", 0), ("fx_neutral_011.json", 2), ("fx_neutral_012.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_neutral", 18, &[1, 13, 14, 15, 16, 17, 18], is_neutral);
    card_reference(
        &mut t,
        &mut gpu,
        "Color Neutralizer",
        "core.color_neutralizer",
        &[
            ("with the three warm colours", json!({"shadows_unbalance": "#3c2d1e", "midtones_unbalance": "#968064", "highlights_unbalance": "#f0e6c8"})),
            (
                "with levels only, pinning 40, points 30 and 220",
                json!({"shadows": [20, 0, -20], "midtones": [0, 10, 0], "highlights": [-30, 0, 30], "pinning": 40, "black_point": 30, "white_point": 220}),
            ),
            ("with the numbers and white point 100 below black point 120", json!({"shadows": [40, 0, -40], "highlights": [-40, 0, 40], "black_point": 120, "white_point": 100})),
        ],
        is_neutral,
    );

    let (dir, write) = pictures(&mut t, "D-366");
    write("warm.png", &warm_town());
    let warm = warm_town();
    let road = first(&town(), [60, 60, 66]);
    let (before, said) = picture(&dir, "warm.png", json!([]));
    write("1_before_warm.png", &before);
    t.row("1_before_warm.png, the street with a warm cast laid on it (red up 24, green up 8, blue down 24), no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let spread = |b: &[u8], i: usize| b[i * 4..i * 4 + 3].iter().max().unwrap() - b[i * 4..i * 4 + 3].iter().min().unwrap();
    let (fixed, said) = picture(
        &dir,
        "warm.png",
        json!([fx("core.color_neutralizer", "fx-0-0", &json!({"shadows_unbalance": "#54442a", "midtones_unbalance": "#54442a", "highlights_unbalance": "#ffffe2"}))]),
    );
    write("2_neutralized.png", &fixed);
    t.row(
        "2_neutralized.png, the warm road's colour #54442a picked as the shadows' and midtones' cast and the warm markings' #ffffe2 as the highlights': the road comes out grey again; draws cleanly",
        &format!(
            "{said:?}, the road {:?} from {:?} (spread {} from {})",
            &fixed[road * 4..road * 4 + 3],
            &warm[road * 4..road * 4 + 3],
            spread(&fixed, road),
            spread(&before, road)
        ),
        said.is_empty() && spread(&fixed, road) <= 2 && spread(&before, road) > 40,
    );
    let (pushed, said) = picture(&dir, "town.png", json!([fx("core.color_neutralizer", "fx-0-0", &json!({"shadows": [0, 0, 60], "highlights": [40, 20, 0]}))]));
    write("3_numbers_cool_shadows_warm_highlights.png", &pushed);
    t.row(
        "3_numbers_cool_shadows_warm_highlights.png, the plain street with blue 60 added to the shadows and red 40, green 20 to the highlights: the road turns blue; draws cleanly",
        &format!("{said:?}, the road {:?}", &pushed[road * 4..road * 4 + 3]),
        said.is_empty() && pushed[road * 4 + 2] > pushed[road * 4] + 20,
    );

    t.finish("D-366_color_neutralizer_table.md");
}

// --- Color Offset ----------------------------------------------------------------------------

fn is_offset(e: &Effect) -> bool {
    matches!(e, Effect::ColorOffset { .. })
}

#[test]
fn b246_color_offset() {
    let mut t = Table::new(
        "color_offset",
        "# D-367: Color Offset\n\nB-246, after CycoreFX's CC Color Offset: each colour channel \
         turned round by its own phase, what runs past white brought back by wrapping, folding \
         (solarize) or folding smoothly (polarize). Every expected pixel is \
         `Fixtures/color_offset/expected_color_offset.json`, written by \
         `tools/color_offset_reference.py` before this code existed and printed in document 25 as \
         FX-COFFSET-001 to 016. Tolerance 2e-5.\n",
    );

    t.heading("FX-COFFSET-001 to 016 (document 25)");
    t.fixtures("expected_color_offset.json");

    t.heading("The file");
    let files: Vec<String> = (1..=16).map(|n| format!("fx_coffset_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_coffset_010.json");
    t.row(
        "fx_coffset_010.json is saved with its three phases and overflow",
        &saved.to_string(),
        saved["red_phase"] == 45 && saved["green_phase"] == -135 && saved["blue_phase"] == 600 && saved["overflow"] == "polarize",
    );
    let effect_of = |d: &anime_compositor::command::Document| {
        d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone()
    };
    let why = effect_of(&t.load("fx_coffset_015.json").document).why_invalid();
    t.row(
        "fx_coffset_015.json is refused in a sentence",
        &why,
        why == "Color Offset's overflow is \"wrap\", \"solarize\" or \"polarize\", and this is \"mirror\".",
    );
    t.shape_refused("fx_coffset_001.json", "a Color Offset with no `blue_phase`", r#"{"red_phase": 0, "green_phase": 0, "overflow": "wrap"}"#);
    t.shape_refused("fx_coffset_001.json", "a Color Offset whose overflow is a number", r#"{"red_phase": 0, "green_phase": 0, "blue_phase": 0, "overflow": 2}"#);

    t.heading("Commands");
    let mut document = t.load("fx_coffset_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("red phase 3600.5", set(offset(3600.5, 0.0, 0.0, "wrap"))),
            ("green phase -3600.5", set(offset(0.0, -3600.5, 0.0, "wrap"))),
            ("overflow \"Polarize\", written with a capital", set(offset(0.0, 0.0, 0.0, "Polarize"))),
            ("blue phase keyed to 4000", keys("blue_phase", &[(0, &[0.0]), (4, &[4000.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_coffset_001.json",
        vec![
            ("red 90, green 180, blue 270, solarize,", set(offset(90.0, 180.0, 270.0, "solarize"))),
            ("green phase keyed from 0 to 720", keys("green_phase", &[(0, &[0.0]), (4, &[720.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_coffset_003.json", 0), ("fx_coffset_005.json", 0), ("fx_coffset_010.json", 0), ("fx_coffset_011.json", 2), ("fx_coffset_012.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_coffset", 16, &[1, 13, 14, 15, 16], is_offset);
    card_reference(
        &mut t,
        &mut gpu,
        "Color Offset",
        "core.color_offset",
        &[
            ("red 90, green 180, blue 270, wrap", json!({"red_phase": 90, "green_phase": 180, "blue_phase": 270})),
            ("all three 180, solarize", json!({"red_phase": 180, "green_phase": 180, "blue_phase": 180, "overflow": "solarize"})),
            ("red 45, green -135, blue 600, polarize", json!({"red_phase": 45, "green_phase": -135, "blue_phase": 600, "overflow": "polarize"})),
        ],
        is_offset,
    );

    let (dir, write) = pictures(&mut t, "D-367");
    let (before, said) = picture(&dir, "town.png", json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let one = |p: J| json!([fx("core.color_offset", "fx-0-0", &p)]);
    let mut drawn = Vec::new();
    for (name, p, what) in [
        ("2_wrap_red_120.png", json!({"red_phase": 120}), "red phase 120, wrap: reds past two thirds come round to dark"),
        ("3_solarize_all_180.png", json!({"red_phase": 180, "green_phase": 180, "blue_phase": 180, "overflow": "solarize"}), "all three 180, solarize: the darks lifted, the lights folded down"),
        ("4_polarize_all_360.png", json!({"red_phase": 360, "green_phase": 360, "blue_phase": 360, "overflow": "polarize"}), "all three 360, polarize: the street's negative"),
        ("5_polarize_mixed.png", json!({"red_phase": 60, "green_phase": -150, "blue_phase": 240, "overflow": "polarize"}), "red 60, green -150, blue 240, polarize: shifted psychedelic colours"),
    ] {
        let (bytes, said) = picture(&dir, "town.png", one(p));
        write(name, &bytes);
        t.row(&format!("{name}, {what}; draws cleanly"), &format!("{said:?}"), said.is_empty());
        drawn.push(bytes);
    }
    let negative: Vec<u8> = before.chunks_exact(4).flat_map(|p| [255 - p[0], 255 - p[1], 255 - p[2], p[3]]).collect();
    let d = distance(&drawn[2], &negative);
    t.row(
        "4_polarize_all_360.png is the street's negative, each channel 255 less its value, within 1 level",
        &format!("largest difference {} of 255", d.0),
        d.0 <= 1,
    );

    t.finish("D-367_color_offset_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B244_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-244: a measurement, run deliberately with --release --ignored"]
fn b244_colour_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B244_CPU").is_ok();
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
        ("Noise, then Broadcast Safe, NTSC, reduce luminance, 90 IRE", Some(("core.broadcast_safe", json!({"max_amplitude": 90})))),
        (
            "Noise, then Color Neutralizer with the three warm colours, pinning 30",
            Some(("core.color_neutralizer", json!({"shadows_unbalance": "#3c2d1e", "midtones_unbalance": "#968064", "highlights_unbalance": "#f0e6c8", "pinning": 30}))),
        ),
        ("Noise, then Color Offset, red 45, green -135, blue 600, polarize", Some(("core.color_offset", json!({"red_phase": 45, "green_phase": -135, "blue_phase": 600, "overflow": "polarize"})))),
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
    let out = std::env::var("B244_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-244_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
