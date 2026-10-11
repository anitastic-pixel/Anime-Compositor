//! B-339: D-459, after After Effects' Color Difference Key (Keying): a matte from a key colour
//! as two partial mattes, A and B, each with its own levels, screened together and taken through
//! levels of its own as the layer's covering. Adobe describes the effect but publishes no
//! formula: the rule is ours, after Robert Ihringer's reverse engineering, stated in
//! `tools/color_difference_key_reference.py`.
//!
//! Writes `verification/D-459_color_difference_key_table.md` and draws pictures into
//! `verification/D-459 pictures/`.
//!
//! Every expected pixel is `Fixtures/color_difference_key/expected_color_difference_key.json`,
//! written by the tool before this code existed and printed in document 25 as FX-CDK-001 to 026.
//! Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, set, Table, MAIN, TOWN};
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

/// The Color Difference Keys the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::ColorDifferenceKey { .. })))
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

/// A Color Difference Key with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"view": "final", "key_color": "#0000ff",
        "partial_a_in_black": 0, "partial_a_in_white": 255, "partial_a_gamma": 1, "partial_a_out_black": 0, "partial_a_out_white": 255,
        "partial_b_in_black": 0, "partial_b_in_white": 255, "partial_b_gamma": 1, "partial_b_out_black": 0, "partial_b_out_white": 255,
        "matte_in_black": 0, "matte_in_white": 255, "matte_gamma": 1});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.color_difference_key", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=26 {
        let file = format!("fx_cdk_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b339-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Color Difference Key {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Color Difference Key {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor, straight
/// 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), effects: J) -> (Vec<u8>, Vec<String>) {
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
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

const SCREEN: [u8; 3] = [40, 180, 60];
const SHIRT: [u8; 3] = [40, 60, 160];

/// A green-screen shot at the street's size: a lit green screen, a face whose outer 14 pixels
/// take on more and more of the screen's green (the spill), and a blue shirt below.
fn portrait() -> Vec<u8> {
    let (w, h) = TOWN;
    let skin = [214.0, 172.0, 140.0];
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (dx, dy) = (x as f64 - 240.0, y as f64 - 110.0);
            let r = (dx * dx / (70.0 * 70.0) + dy * dy / (90.0 * 90.0)).sqrt();
            let c = if y >= 200 && (130..350).contains(&x) {
                SHIRT
            } else if r < 1.0 {
                // The spill, from none 14 pixels in to 60% of the screen at the edge.
                let spill = (1.0 - (1.0 - r) * 70.0 / 14.0).clamp(0.0, 1.0) * 0.6;
                std::array::from_fn(|c| (skin[c] + (SCREEN[c] as f64 - skin[c]) * spill).round() as u8)
            } else {
                SCREEN
            };
            bytes.extend([c[0], c[1], c[2], 255]);
        }
    }
    bytes
}

/// The `art` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

/// The pixel at (x, y) of a picture at the street's size.
fn at(bytes: &[u8], x: usize, y: usize) -> &[u8] {
    let i = (y * TOWN.0 + x) * 4;
    &bytes[i..i + 4]
}

#[test]
fn b339_color_difference_key() {
    let mut t = Table::new(
        "color_difference_key",
        "# D-459: Color Difference Key\n\nB-339, after After Effects' Color Difference Key \
         (Keying): a matte from a key colour as two partial mattes, A and B, each with its own \
         levels, screened together and taken through levels of its own as the layer's covering. \
         Adobe describes the effect but publishes no formula, so the rule is ours, after Robert \
         Ihringer's reverse engineering (bildfehler.de, 2007), stated in \
         `tools/color_difference_key_reference.py`: each pixel's hue against a window round the \
         key (partial B) and 60 degrees short of it (partial A), weighted by saturation and laid \
         over lightness as a hard light. Every expected pixel is \
         `Fixtures/color_difference_key/expected_color_difference_key.json`, written by the tool \
         before this code existed and printed in document 25 as FX-CDK-001 to 026. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-CDK-001 to 026 (document 25)");
    t.fixtures_numbered("expected_color_difference_key.json", 1..=26);

    t.heading("The file");
    let files: Vec<String> = (1..=26).map(|n| format!("fx_cdk_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_cdk_011.json");
    t.row(
        "fx_cdk_011.json is saved with its fifteen settings, the key #28b43c and partial B in white 128",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 15 && saved["key_color"] == "#28b43c" && saved["partial_b_in_white"] == 128.0,
    );
    for (file, want) in [
        ("fx_cdk_019.json", "partial a in black"),
        ("fx_cdk_020.json", "partial b out white"),
        ("fx_cdk_021.json", "partial a gamma"),
        ("fx_cdk_022.json", "matte gamma"),
        ("fx_cdk_023.json", "matte in white"),
        ("fx_cdk_024.json", "view"),
        ("fx_cdk_025.json", "view"),
        ("fx_cdk_026.json", "key colour"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    let numbers = r##""partial_a_in_black": 0, "partial_a_in_white": 255, "partial_a_gamma": 1, "partial_a_out_black": 0, "partial_a_out_white": 255, "partial_b_in_black": 0, "partial_b_in_white": 255, "partial_b_gamma": 1, "partial_b_out_black": 0, "partial_b_out_white": 255, "matte_in_black": 0, "matte_in_white": 255"##;
    t.shape_refused(
        "fx_cdk_001.json",
        "a Color Difference Key whose matte gamma is a word",
        &format!(r##"{{"view": "final", "key_color": "#0000ff", {numbers}, "matte_gamma": "soft"}}"##),
    );
    t.shape_refused("fx_cdk_001.json", "a Color Difference Key without its view", &format!(r##"{{"key_color": "#0000ff", {numbers}, "matte_gamma": 1}}"##));
    t.shape_refused("fx_cdk_001.json", "a Color Difference Key without its matte gamma", &format!(r##"{{"view": "final", "key_color": "#0000ff", {numbers}}}"##));

    t.heading("Commands");
    let mut document = t.load("fx_cdk_001.json").document;
    let over = changed(&t, "fx_cdk_001.json", |e| if let Effect::ColorDifferenceKey { partial_a, .. } = e { partial_a[2] = 11.0 });
    let word = changed(&t, "fx_cdk_001.json", |e| if let Effect::ColorDifferenceKey { view, .. } = e { *view = "all".into() });
    let colour = changed(&t, "fx_cdk_001.json", |e| if let Effect::ColorDifferenceKey { key_color, .. } = e { *key_color = "blue".into() });
    t.refused(
        &mut document,
        vec![
            ("partial A gamma 11", set(over)),
            ("view \"all\"", set(word)),
            ("key colour \"blue\"", set(colour)),
            ("matte in white keyed to 300", keys("matte_in_white", &[(0, &[255.0]), (4, &[300.0])])),
        ],
    );
    let green = changed(&t, "fx_cdk_001.json", |e| {
        if let Effect::ColorDifferenceKey { view, key_color, .. } = e {
            *view = "matte".into();
            *key_color = "#00ff00".into();
        }
    });
    t.taken(
        &mut document,
        "fx_cdk_001.json",
        vec![("view matte with a green key", set(green)), ("matte gamma keyed from 1 to 3", keys("matte_gamma", &[(0, &[1.0]), (4, &[3.0])]))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_cdk_001.json", 0),
        ("fx_cdk_004.json", 0),
        ("fx_cdk_008.json", 0),
        ("fx_cdk_009.json", 0),
        ("fx_cdk_016.json", 2),
        ("fx_cdk_018.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // View source (002) and refused (019 to 026) leave nothing for the card.
    let none: Vec<u32> = [2].into_iter().chain(19..=26).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added (blue key, final)", json!({}), 3),
            ("view matte, green key, matte gamma 0.6", json!({"view": "matte", "key_color": "#00ff00", "matte_gamma": 0.6}), 3),
            (
                "final, key #28b43c, every levels set",
                json!({"key_color": "#28b43c", "partial_a_in_black": 40, "partial_a_in_white": 200, "partial_a_gamma": 1.5, "partial_a_out_black": 10, "partial_a_out_white": 245,
                    "partial_b_in_black": 30, "partial_b_in_white": 180, "partial_b_gamma": 0.7, "partial_b_out_white": 250, "matte_in_black": 60, "matte_in_white": 220, "matte_gamma": 1.3}),
                3,
            ),
        ],
    );

    t.heading("Pictures: in `verification/D-459 pictures/`");
    let dir = repo("verification/D-459 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("portrait.png", TOWN, &portrait());
    let shot = |p: J| picture(&dir, "portrait.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "portrait.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the green-screen shot with no effect (a face whose edge is spilled green, a blue shirt); draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = shot(json!({"view": "source", "matte_in_black": 200}));
    t.row(
        "view source changes nothing, whatever the levels: the shot byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );

    // The places to look: the screen, the face's middle (no spill), its spilled edge, the shirt.
    let (screen, face, edge, shirt) = ((20, 20), (240, 110), (240, 22), (240, 250));
    let grey = |b: &[u8], p: (usize, usize)| at(b, p.0, p.1)[0];

    // By hand, from the tool: with the screen's own green as the key, the uncorrected matte is
    // 103 of 255 on the screen, 182 on the face and 226 on the shirt.
    let key = "#28b43c";
    let (after, said) = shot(json!({"view": "matte_uncorrected", "key_color": key}));
    write("2_matte_uncorrected.png", TOWN, &after);
    let (s, f, sh) = (grey(&after, screen), grey(&after, face), grey(&after, shirt));
    t.row(
        "2_matte_uncorrected.png, the matte as grey, key #28b43c (the screen's green), no levels: the screen darkest (103 by hand), the face lighter (182), the shirt lightest (226); the screen is not yet black, which is what the levels are for; draws cleanly",
        &format!("{said:?}, screen {s}, face {f}, shirt {sh}"),
        said.is_empty() && s.abs_diff(103) <= 1 && f.abs_diff(182) <= 1 && sh.abs_diff(226) <= 1,
    );

    // The matte's levels at 110 to 180: the screen black, the face and shirt white, the spilled
    // edge between.
    let tuned = json!({"key_color": key, "matte_in_black": 110, "matte_in_white": 180});
    let mut matte_view = tuned.clone();
    matte_view["view"] = J::from("matte");
    let (after, said) = shot(matte_view);
    write("3_matte.png", TOWN, &after);
    let (s, f, e, sh) = (grey(&after, screen), grey(&after, face), grey(&after, edge), grey(&after, shirt));
    t.row(
        "3_matte.png, the corrected matte with matte in black 110 and in white 180: the screen black, the face and the shirt white, the face's spilled green edge grey; draws cleanly",
        &format!("{said:?}, screen {s}, face {f}, edge {e}, shirt {sh}"),
        said.is_empty() && s == 0 && f == 255 && sh == 255 && e > 0 && e < 255,
    );

    let (after, said) = shot(tuned);
    write("4_final.png", TOWN, &after);
    let gone = at(&after, screen.0, screen.1)[3];
    let kept = [face, shirt].iter().all(|&(x, y)| at(&after, x, y) == at(&before, x, y));
    let colour_kept = after.chunks_exact(4).zip(before.chunks_exact(4)).filter(|(a, _)| a[3] >= 16).all(|(a, b)| (0..3).all(|c| a[c].abs_diff(b[c]) <= 1));
    let soft = at(&after, edge.0, edge.1)[3];
    t.row(
        "4_final.png, Final Output with the same levels: the screen see-through, the face and the shirt exactly as they were, the spilled edge part covered; wherever anything is left its colour is the shot's (the key takes covering, never colour, so the green spill stays for a spill suppressor); draws cleanly",
        &format!("{said:?}, screen covering {gone}, face and shirt untouched: {kept}, colour kept: {colour_kept}, edge covering {soft}"),
        said.is_empty() && gone == 0 && kept && colour_kept && soft > 0 && soft < 255,
    );

    // As added (a blue key, every levels plain), by hand on single colours: pure blue all gone,
    // the violet (128, 64, 255) half, green all kept.
    for (file, name, c, want) in [
        ("blue.png", "pure blue", [0u8, 0, 255], 0u8),
        ("violet.png", "the violet (128, 64, 255)", [128, 64, 255], 128),
        ("green.png", "pure green", [0, 255, 0], 255),
    ] {
        write(file, (4, 4), &[c[0], c[1], c[2], 255].repeat(16));
        let (after, said) = picture(&dir, file, (4, 4), json!([fx("fx-0-0", &json!({}))]));
        t.row(
            &format!("by hand: {name}, as added, keeps covering {want} of 255; draws cleanly"),
            &format!("{said:?}, covering {}", after[3]),
            said.is_empty() && after[3].abs_diff(want) <= 3,
        );
    }

    t.finish("D-459_color_difference_key_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B339_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-339: a measurement, run deliberately with --release --ignored"]
fn b339_color_difference_key_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B339_CPU").is_ok();
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
        ("Noise, then Color Difference Key, as added", Some(json!({}))),
        ("Noise, then Color Difference Key, every levels set", Some(json!({"key_color": "#28b43c", "partial_a_in_black": 40, "partial_b_gamma": 0.7, "matte_in_black": 60, "matte_in_white": 220, "matte_gamma": 1.3}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}c"), p));
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
    let out = std::env::var("B339_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-339_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
