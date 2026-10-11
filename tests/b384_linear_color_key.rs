//! B-384: D-504, after After Effects' Linear Color Key (Keying): each pixel compared with a key
//! colour by its RGB, its hue or its chroma, the closer ones made more transparent, with a
//! tolerance and a softness. Adobe describes the effect but publishes no formula and no defaults
//! we found: the rule and the settings when added are ours, stated in
//! `tools/linear_color_key_reference.py`.
//!
//! Writes `verification/D-504_linear_color_key_table.md` and draws pictures into
//! `verification/D-504 pictures/`.
//!
//! Every expected pixel is `Fixtures/linear_color_key/expected_linear_color_key.json`, written by
//! the tool before this code existed and printed in document 25 as FX-LCKEY-001 to 018.
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

/// The Linear Color Keys the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::LinearColorKey { .. })))
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

/// A Linear Color Key with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"view": "final", "key_color": "#0000ff", "match_colors": "rgb", "matching_tolerance": 10, "matching_softness": 10});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.linear_color_key", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=18 {
        let file = format!("fx_lckey_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b384-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Linear Color Key {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Linear Color Key {what} on three layers, frame {frame}, {}", quality.label()),
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
/// take on more and more of the screen's green (the spill), and a blue shirt below. Color
/// Difference Key's (B-339).
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
fn b384_linear_color_key() {
    let mut t = Table::new(
        "linear_color_key",
        "# D-504: Linear Color Key\n\nB-384, after After Effects' Linear Color Key (Keying): each \
         pixel compared with a key colour by its RGB, its hue or its chroma, the closer ones made \
         more transparent. Adobe describes the effect and names its controls but publishes no \
         formula and no defaults we found, so the rule and the settings when added are ours, \
         stated in `tools/linear_color_key_reference.py`: a distance from the key from 0 to 100, \
         less the matching tolerance, over the matching softness, is the share of the pixel kept. \
         Key Operation's Keep Colors is not built. Every expected pixel is \
         `Fixtures/linear_color_key/expected_linear_color_key.json`, written by the tool before \
         this code existed and printed in document 25 as FX-LCKEY-001 to 018. Tolerance 2e-5.\n",
    );

    t.heading("FX-LCKEY-001 to 018 (document 25)");
    t.fixtures_numbered("expected_linear_color_key.json", 1..=18);

    t.heading("The file");
    let files: Vec<String> = (1..=18).map(|n| format!("fx_lckey_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_lckey_004.json");
    t.row(
        "fx_lckey_004.json is saved with its five settings, view final, key colour #28b43c, match colors hue and tolerance 20",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 5
            && saved["view"] == "final"
            && saved["key_color"] == "#28b43c"
            && saved["match_colors"] == "hue"
            && saved["matching_tolerance"] == 20.0,
    );
    for (file, want) in [
        ("fx_lckey_014.json", "matching tolerance"),
        ("fx_lckey_015.json", "matching softness"),
        ("fx_lckey_016.json", "view"),
        ("fx_lckey_017.json", "match colors"),
        ("fx_lckey_018.json", "key colour"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    t.shape_refused(
        "fx_lckey_001.json",
        "a Linear Color Key whose tolerance is a word",
        r##"{"view": "final", "key_color": "#0000ff", "match_colors": "rgb", "matching_tolerance": "close", "matching_softness": 10}"##,
    );
    t.shape_refused(
        "fx_lckey_001.json",
        "a Linear Color Key without its key colour",
        r##"{"view": "final", "match_colors": "rgb", "matching_tolerance": 10, "matching_softness": 10}"##,
    );
    t.shape_refused(
        "fx_lckey_001.json",
        "a Linear Color Key without its softness",
        r##"{"view": "final", "key_color": "#0000ff", "match_colors": "rgb", "matching_tolerance": 10}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_lckey_001.json").document;
    let over = changed(&t, "fx_lckey_001.json", |e| if let Effect::LinearColorKey { matching_tolerance, .. } = e { *matching_tolerance = 101.0 });
    let word = changed(&t, "fx_lckey_001.json", |e| if let Effect::LinearColorKey { match_colors, .. } = e { *match_colors = "lab".into() });
    t.refused(
        &mut document,
        vec![
            ("matching tolerance 101", set(over)),
            ("match colors \"lab\"", set(word)),
            ("matching softness keyed to 300", keys("matching_softness", &[(0, &[0.0]), (4, &[300.0])])),
        ],
    );
    let hue = changed(&t, "fx_lckey_001.json", |e| {
        if let Effect::LinearColorKey { key_color, match_colors, .. } = e {
            *key_color = "#28b43c".into();
            *match_colors = "hue".into();
        }
    });
    t.taken(
        &mut document,
        "fx_lckey_001.json",
        vec![
            ("match colors hue on the green screen's colour", set(hue)),
            ("matching tolerance keyed from 0 to 40", keys("matching_tolerance", &[(0, &[0.0]), (4, &[40.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_lckey_002.json", 0),
        ("fx_lckey_004.json", 0),
        ("fx_lckey_005.json", 0),
        ("fx_lckey_009.json", 0),
        ("fx_lckey_011.json", 2),
        ("fx_lckey_013.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // The refused ones (014 to 018) leave nothing for the card.
    let none: Vec<u32> = (14..=18).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("RGB, key white, tolerance 60, softness 20", json!({"key_color": "#ffffff", "matching_tolerance": 60, "matching_softness": 20}), 3),
            ("hue, key orange, tolerance 30, softness 20", json!({"key_color": "#ff8000", "match_colors": "hue", "matching_tolerance": 30, "matching_softness": 20}), 3),
            ("chroma, key grey, tolerance 10, softness 0, the matte", json!({"view": "matte", "key_color": "#808080", "match_colors": "chroma", "matching_softness": 0}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-504 pictures/`");
    let dir = repo("verification/D-504 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("portrait.png", TOWN, &portrait());
    let shot = |p: J| picture(&dir, "portrait.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "portrait.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the green-screen shot with no effect (a face whose edge is spilled green, a blue shirt); draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = shot(json!({}));
    t.row(
        "as added (pure blue, RGB, tolerance 10) nothing in the shot is within 10% of pure blue, so nothing is keyed: the shot byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );

    // The places to look: the screen, the face's middle (no spill), a spilled pixel near the
    // face's top edge, (156, 175, 113), the shirt.
    let (screen, face, edge, shirt) = ((20, 20), (240, 110), (240, 28), (240, 250));
    let cover = |b: &[u8], p: (usize, usize)| at(b, p.0, p.1)[3];
    let colour_kept = |after: &[u8]| after.chunks_exact(4).zip(before.chunks_exact(4)).filter(|(a, _)| a[3] >= 16).all(|(a, b)| (0..3).all(|c| a[c].abs_diff(b[c]) <= 1));

    let (after, said) = shot(json!({"key_color": "#28b43c"}));
    write("2_rgb.png", TOWN, &after);
    let (s, f, e, sh) = (cover(&after, screen), cover(&after, face), cover(&after, edge), cover(&after, shirt));
    t.row(
        "2_rgb.png, RGB on the screen's colour, tolerance 10, softness 10: the screen see-through, the face, the spilled pixel (45% off in red) and the shirt fully covered; colour kept wherever anything is left; draws cleanly",
        &format!("{said:?}, screen {s}, face {f}, spilled {e}, shirt {sh}, colour kept {}", colour_kept(&after)),
        said.is_empty() && s == 0 && f == 255 && e == 255 && sh == 255 && colour_kept(&after),
    );

    // By hand, from the tool: the spilled pixel's hue is 50.2 degrees from the screen's, a
    // distance of 27.88; (27.88 - 20) / 10 x 255 = 201.
    let hue = json!({"key_color": "#28b43c", "match_colors": "hue", "matching_tolerance": 20});
    let (after, said) = shot(hue.clone());
    write("3_hue.png", TOWN, &after);
    let (s, f, e, sh) = (cover(&after, screen), cover(&after, face), cover(&after, edge), cover(&after, shirt));
    t.row(
        "3_hue.png, hue on the screen's colour, tolerance 20, softness 10: the screen see-through, the face and the shirt whole, the spilled green edge fading (covering 201 by hand at the spilled pixel); draws cleanly",
        &format!("{said:?}, screen {s}, face {f}, spilled {e}, shirt {sh}"),
        said.is_empty() && s == 0 && f == 255 && sh == 255 && e.abs_diff(201) <= 2,
    );

    let mut matte = hue.clone();
    matte["view"] = J::from("matte");
    let (after, said) = shot(matte);
    write("4_hue_matte.png", TOWN, &after);
    let grey = |p: (usize, usize)| at(&after, p.0, p.1).to_vec();
    t.row(
        "4_hue_matte.png, the same as Matte Only: the screen black, the face and the shirt white, all fully covered; draws cleanly",
        &format!("{said:?}, screen {:?}, face {:?}, shirt {:?}", grey(screen), grey(face), grey(shirt)),
        said.is_empty() && grey(screen) == [0, 0, 0, 255] && grey(face) == [255, 255, 255, 255] && grey(shirt) == [255, 255, 255, 255],
    );

    // By hand on single colours, keyed on the screen's colour: the screen half lit (20, 90, 30)
    // is 35.29 off in RGB, so kept whole, but has the screen's chromaticity, so chroma keys it
    // out; (40, 180, 90) is 11.76 off in RGB, (11.76 - 10) / 10 x 255 = 45.
    for (file, name, c, p, want) in [
        ("shade.png", "the screen half lit (20, 90, 30), in RGB", [20u8, 90, 30], json!({"key_color": "#28b43c"}), 255u8),
        ("shade.png", "the screen half lit (20, 90, 30), in chroma", [20, 90, 30], json!({"key_color": "#28b43c", "match_colors": "chroma"}), 0),
        ("bluer.png", "the screen a little bluer (40, 180, 90), in RGB", [40, 180, 90], json!({"key_color": "#28b43c"}), 45),
    ] {
        write(file, (4, 4), &[c[0], c[1], c[2], 255].repeat(16));
        let (after, said) = picture(&dir, file, (4, 4), json!([fx("fx-0-0", &p)]));
        t.row(
            &format!("by hand: {name} keyed on the screen's colour, tolerance 10, softness 10, keeps covering {want} of 255; draws cleanly"),
            &format!("{said:?}, covering {}", after[3]),
            said.is_empty() && after[3].abs_diff(want) <= 2,
        );
    }

    t.finish("D-504_linear_color_key_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B384_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-384: a measurement, run deliberately with --release --ignored"]
fn b384_linear_color_key_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B384_CPU").is_ok();
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
        ("Noise, then Linear Color Key as added (RGB)", Some(json!({}))),
        ("Noise, then Linear Color Key, hue, key orange, tolerance 30", Some(json!({"key_color": "#ff8000", "match_colors": "hue", "matching_tolerance": 30, "matching_softness": 20}))),
        ("Noise, then Linear Color Key, chroma, the matte", Some(json!({"view": "matte", "key_color": "#808080", "match_colors": "chroma"}))),
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
    let out = std::env::var("B384_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-384_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
