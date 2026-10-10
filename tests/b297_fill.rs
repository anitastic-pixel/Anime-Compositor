//! B-297: D-418 Fill, after After Effects' Fill ("Generate" in `docs/effects/EFFECTS.md`): the
//! layer, or the masks chosen, filled with one colour, feathered, inverted if asked, the layer's
//! alpha kept.
//!
//! Writes `verification/D-418_fill_table.md`.
//!
//! Every expected pixel is `Fixtures/fill/expected_fill.json`, written by
//! `tools/fill_reference.py` before this code existed and printed in document 25 as
//! FX-FILL-001 to 044. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-418 pictures/`.

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

fn fill(mask: f64, [horizontal_feather, vertical_feather, opacity]: [f64; 3], words: [&str; 3]) -> Effect {
    let [all_masks, color, invert] = words.map(str::to_string);
    Effect::Fill { mask, all_masks, color, invert, horizontal_feather, vertical_feather, opacity, paths: None }
}

const PLAIN: [&str; 3] = ["off", "#ff0000", "off"];

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

/// A circle as a closed path of four curves.
fn circle(((cx, cy), r): ((f64, f64), f64)) -> J {
    let k = 0.552_284_749_8 * r;
    json!({"points": [
        {"point": [cx, cy - r], "in": [-k, 0], "out": [k, 0]},
        {"point": [cx + r, cy], "in": [0, -k], "out": [0, k]},
        {"point": [cx, cy + r], "in": [k, 0], "out": [-k, 0]},
        {"point": [cx - r, cy], "in": [0, k], "out": [0, -k]},
    ]})
}

/// Masks that cut nothing (mode None), for Fill to choose from.
fn masks(circles: &[((f64, f64), f64)]) -> J {
    J::Array(
        circles
            .iter()
            .enumerate()
            .map(|(m, &c)| json!({"name": format!("Mask {}", m + 1), "enabled": true, "inverted": false, "mode": "none", "opacity": 1.0,
                "feather_px": 0.0, "expansion_px": 0.0, "path": {"base": circle(c), "keyframes": []}}))
            .collect(),
    )
}

/// The reference shot (1920 by 1080) with three masks (mode None) and `stack` on its first three layers.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        let d = 40.0 * i as f64;
        layers[i]["masks"] = masks(&[((960.0 + d, 540.0), 400.0), ((560.0 + d, 400.0), 220.0), ((1400.0 + d, 620.0), 300.0)]);
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// An effect of `type_id` with the settings `p`; for the ones here, every setting `p` leaves
/// out is the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.fill" => json!({"mask": 0, "all_masks": "off", "color": "#ff0000", "invert": "off", "horizontal_feather": 0, "vertical_feather": 0, "opacity": 100}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b297-{id}"), p)]));
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

/// `effects` on the street, drawn, straight 8-bit, with what it warned of. The street layer has
/// two masks that cut nothing: a circle about the middle and one to the upper left.
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
    layer["masks"] = masks(&[((240.0, 135.0), 80.0), ((110.0, 80.0), 50.0)]);
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// The street drawn plain and with each of `shots` of `type_id`, written as numbered pictures
/// into `verification/{d} pictures/`; a row each that it draws cleanly and changes what it says
/// it changes.
fn street(t: &mut Table, d: &str, type_id: &str, shots: &[(&str, J, &str, fn(&[u8], &[u8]) -> bool)]) {
    t.heading(&format!("Pictures: the street, in `verification/{d} pictures/`"));
    let dir = repo(&format!("verification/{d} pictures"));
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect (its two masks cut nothing); draws cleanly", &format!("{said:?}"), said.is_empty());
    for (i, (name, p, what, check)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = picture(&dir, json!([fx(type_id, "fx-0-0", p)]));
        write(&file, &bytes);
        t.row(
            &format!("{file}, {what}; draws cleanly"),
            &format!("{said:?}, {} pixels changed of {}", distance(&bytes, &before).1, TOWN.0 * TOWN.1),
            said.is_empty() && check(&bytes, &before),
        );
    }
}

fn why(t: &mut Table, cases: &[(&str, &str)]) {
    for (file, want) in cases {
        let doc = t.load(file).document;
        let why = doc.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == *want);
    }
}

fn files(stem: &str, count: u32) -> Vec<String> {
    (1..=count).map(|n| format!("{stem}_{n:03}.json")).collect()
}

// --- Fill -----------------------------------------------------------------------------------

fn is_fill(e: &Effect) -> bool {
    matches!(e, Effect::Fill { .. })
}

/// The share of pixels that changed, and every pixel's alpha as it was.
fn alpha_kept(a: &[u8], b: &[u8]) -> (f64, bool) {
    let share = distance(a, b).1 as f64 / (a.len() / 4) as f64;
    (share, a.chunks_exact(4).zip(b.chunks_exact(4)).all(|(p, q)| p[3] == q[3]))
}

/// Nearly every pixel changed, alpha kept.
fn all_over(a: &[u8], b: &[u8]) -> bool {
    let (share, kept) = alpha_kept(a, b);
    share > 0.95 && kept
}

/// Some pixels changed but not most, alpha kept.
fn in_part(a: &[u8], b: &[u8]) -> bool {
    let (share, kept) = alpha_kept(a, b);
    share > 0.0 && share < 0.6 && kept
}

/// Most pixels changed but not all, alpha kept.
fn mostly(a: &[u8], b: &[u8]) -> bool {
    let (share, kept) = alpha_kept(a, b);
    share > 0.6 && share < 0.98 && kept
}

#[test]
fn b297_fill() {
    let mut t = Table::new(
        "fill",
        "# D-418: Fill\n\nB-297, after After Effects' Fill: the masks chosen as Path Stroke's (Fill Mask \
         n, its floor taken, of the masks switched on with two points or more, or every one with All \
         Masks; 0 for the whole layer), each covered by ADR-016's 4 by 4 even-odd rule and joined as a \
         screen, feathered by a Gaussian of sigma = feather / 2 each way, turned over by Invert, then \
         P (1 - k) + C P.a k with k = covering x Opacity: the layer's alpha is kept. A mask asked for \
         and missing leaves the layer as it is with a warning each frame. The layer never grows. Every \
         expected pixel is `Fixtures/fill/expected_fill.json`, written by `tools/fill_reference.py` \
         before this code existed and printed in document 25 as FX-FILL-001 to 044. Tolerance 2e-5.\n",
    );

    t.heading("FX-FILL-001 to 044 (document 25)");
    t.fixtures_numbered("expected_fill.json", 1..=44);

    t.heading("The file");
    // FX-FILL-017's colour is written in capitals and saved in small letters, as every colour is.
    let all: Vec<String> = files("fx_fill", 44).into_iter().filter(|f| f != "fx_fill_017.json").collect();
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_fill_017.json");
    t.row(
        "fx_fill_017.json is saved with its colour in small letters, its words and numbers as written, no paths",
        &saved.to_string(),
        saved["color"] == "#3080ff" && saved["mask"] == 1 && saved["all_masks"] == "off" && saved["invert"] == "off"
            && saved["horizontal_feather"] == 0 && saved["vertical_feather"] == 0 && saved["opacity"] == 100 && saved.get("paths").is_none(),
    );
    why(
        &mut t,
        &[
            ("fx_fill_036.json", "Fill's mask runs from 0 to 1000, and this is -1."),
            ("fx_fill_037.json", "Fill's mask runs from 0 to 1000, and this is 1001."),
            ("fx_fill_038.json", "Fill's horizontal feather runs from 0 to 1000, and this is -1."),
            ("fx_fill_039.json", "Fill's vertical feather runs from 0 to 1000, and this is 1001."),
            ("fx_fill_040.json", "Fill's opacity runs from 0 to 100, and this is 101."),
            ("fx_fill_041.json", "Fill's colour is written #rrggbb, and this is \"#12345\"."),
            ("fx_fill_042.json", "Fill's invert is \"off\" or \"on\", and this is \"yes\"."),
            ("fx_fill_043.json", "Fill's all masks is \"off\" or \"on\", and this is \"maybe\"."),
        ],
    );
    t.shape_refused("fx_fill_001.json", "a Fill with no `opacity`", r##"{"mask": 0, "all_masks": "off", "color": "#ff0000", "invert": "off", "horizontal_feather": 0, "vertical_feather": 0}"##);
    t.shape_refused("fx_fill_001.json", "a Fill with a mask in words", r##"{"mask": "first", "all_masks": "off", "color": "#ff0000", "invert": "off", "horizontal_feather": 0, "vertical_feather": 0, "opacity": 100}"##);
    t.shape_refused("fx_fill_001.json", "a Fill whose invert is true", r##"{"mask": 0, "all_masks": "off", "color": "#ff0000", "invert": true, "horizontal_feather": 0, "vertical_feather": 0, "opacity": 100}"##);

    t.heading("Commands");
    let mut document = t.load("fx_fill_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("mask -1", set(fill(-1.0, [0.0, 0.0, 100.0], PLAIN))),
            ("vertical feather 1001", set(fill(1.0, [0.0, 1001.0, 100.0], PLAIN))),
            ("all masks \"maybe\"", set(fill(1.0, [0.0, 0.0, 100.0], ["maybe", "#ff0000", "off"]))),
            ("invert \"yes\"", set(fill(1.0, [0.0, 0.0, 100.0], ["off", "#ff0000", "yes"]))),
            ("colour \"red\"", set(fill(1.0, [0.0, 0.0, 100.0], ["off", "red", "off"]))),
            ("opacity keyed to 101", keys("opacity", &[(0, &[100.0]), (4, &[101.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_fill_001.json",
        vec![
            ("mask 2, All Masks, blue, inverted, feathers 3 and 6, opacity 70", set(fill(2.0, [3.0, 6.0, 70.0], ["on", "#3080ff", "on"]))),
            ("opacity keyed from 0 to 100", keys("opacity", &[(0, &[0.0]), (4, &[100.0])])),
            ("horizontal feather keyed from 0 to 8", keys("horizontal_feather", &[(0, &[0.0]), (4, &[8.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_fill_002.json", 0), ("fx_fill_005.json", 0), ("fx_fill_007.json", 0), ("fx_fill_012.json", 0), ("fx_fill_018.json", 2), ("fx_fill_022.json", 0), ("fx_fill_028.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // FX-FILL-024 (the whole layer, inverted) and 025 (opacity 0) change nothing; 031 to 035 have
    // no mask to fill and 036 to 044 are refused, so the card is not asked.
    let none: Vec<u32> = [24, 25].into_iter().chain(31..=44).collect();
    card_fixtures(&mut t, &mut gpu, "fx_fill", 44, &none, is_fill);
    card_reference(
        &mut t,
        &mut gpu,
        "Fill",
        "core.fill",
        &[
            ("as added (the whole layer, red)", json!({})),
            ("the whole layer, blue at 50 per cent", json!({"color": "#3080ff", "opacity": 50})),
            ("mask 1 (a circle of radius 400), orange", json!({"mask": 1, "color": "#ff8800"})),
            ("mask 2, inverted, feathers 30 and 10, opacity 80", json!({"mask": 2, "invert": "on", "horizontal_feather": 30, "vertical_feather": 10, "opacity": 80})),
            ("All Masks, feathers 60, violet", json!({"all_masks": "on", "horizontal_feather": 60, "vertical_feather": 60, "color": "#6450a0"})),
            ("mask 3, vertical feather 200, green at 60 per cent", json!({"mask": 3, "vertical_feather": 200, "color": "#20c040", "opacity": 60})),
        ],
        is_fill,
    );

    street(
        &mut t,
        "D-418",
        "core.fill",
        &[
            ("as_added", json!({}), "as added: the whole street red, its clear pixels left clear", all_over),
            ("mask_orange", json!({"mask": 1, "color": "#ff8800"}), "mask 1, orange: a hard orange disc in the middle, the rest the street", in_part),
            ("mask_feathered", json!({"mask": 1, "horizontal_feather": 24, "vertical_feather": 24, "color": "#ff8800"}), "mask 1, feathers 24, orange: the disc with a soft edge", in_part),
            ("inverted_dark", json!({"mask": 1, "invert": "on", "color": "#000000", "opacity": 60}), "mask 1 inverted, black at 60 per cent: everything outside the disc darkened, the disc left as the street", mostly),
            ("all_masks_blue", json!({"all_masks": "on", "horizontal_feather": 8, "color": "#3080ff", "opacity": 70}), "All Masks, horizontal feather 8, blue at 70 per cent: both discs blue, softened left and right", in_part),
        ],
    );

    t.finish("D-418_fill_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B297_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-297: a measurement, run deliberately with --release --ignored"]
fn b297_fill_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B297_CPU").is_ok();
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
        ("Noise, then Fill as added (the whole layer)", Some(("core.fill", json!({})))),
        ("Noise, then Fill, mask 1, feathers 30 and 10, inverted, opacity 80", Some(("core.fill", json!({"mask": 1, "invert": "on", "horizontal_feather": 30, "vertical_feather": 10, "opacity": 80})))),
        ("Noise, then Fill, All Masks, feathers 60", Some(("core.fill", json!({"all_masks": "on", "horizontal_feather": 60, "vertical_feather": 60, "color": "#6450a0"})))),
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
    let out = std::env::var("B297_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-297_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
