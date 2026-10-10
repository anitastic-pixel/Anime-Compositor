//! B-280: D-401, After Effects' Tint under `core.tint`: Map Black To, Map White To and Amount
//! to Tint, with Swap Colors in the panel; a Tint saved before it is read and drawn as it was.
//!
//! Writes `verification/D-401_tint_table.md` and draws pictures into `verification/D-401 pictures/`.
//!
//! Every expected pixel is `Fixtures/tint/expected_tint.json`, written by
//! `tools/tint_reference.py` before this code existed and printed in document 25 as FX-TINT-001
//! to 016; the older Tint's are FX-ADJ-008 to 010 and FX-FXK-004, 005, 008 and 009, from before
//! D-401. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The Tints the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::TintMap { .. })))
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

/// A Tint with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"map_black_to": "#000000", "map_white_to": "#ffffff", "amount_to_tint": 100});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.tint", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=16 {
        let file = format!("fx_tint_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b280-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Tint {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Tint {what} on three layers, frame {frame}, {}", quality.label()),
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

/// The `art` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

/// Covered pixels of `before` and `after` side by side.
fn pairs<'a>(before: &'a [u8], after: &'a [u8]) -> impl Iterator<Item = (&'a [u8], &'a [u8])> {
    before.chunks_exact(4).zip(after.chunks_exact(4)).filter(|(b, _)| b[3] > 0)
}

#[test]
fn b280_tint() {
    let mut t = Table::new(
        "tint",
        "# D-401: Tint with After Effects' settings\n\nB-280: `core.tint` takes After Effects' \
         Map Black To, Map White To and Amount to Tint; each pixel's lightness picks a colour \
         between the two, by Gradient Map's rule with its two ends only. A Tint saved before \
         D-401 (`color` and `amount`) is read and drawn by its own older rule, unchanged. Every \
         expected pixel is `Fixtures/tint/expected_tint.json`, written by \
         `tools/tint_reference.py` before this code existed and printed in document 25 as \
         FX-TINT-001 to 016; the older Tint's are FX-ADJ-008 to 010 and FX-FXK-004, 005, 008 \
         and 009, pinned long before D-401. Tolerance 2e-5.\n",
    );

    t.heading("FX-TINT-001 to 016 (document 25)");
    t.fixtures_numbered("expected_tint.json", 1..=16);

    t.heading("A Tint saved before D-401: its fixtures from before, unchanged");
    let root = t.root.clone();
    t.root = repo("Fixtures/adjust");
    t.fixtures_numbered("expected_adjust.json", 8..=10);
    t.round_trips(&["fx_adj_008.json", "fx_adj_009.json", "fx_adj_010.json"]);
    let older = t.load("fx_adj_008.json").document;
    let kinds: Vec<&str> = older.project().compositions.iter()
        .flat_map(|c| c.layers_in_order()).flat_map(|l| l.effects.iter())
        .map(|e| match e.effect { Effect::Tint { .. } => "the older Tint", Effect::TintMap { .. } => "the new Tint", _ => "other" })
        .collect();
    t.row(
        "fx_adj_008.json, a Tint with a colour and an amount, opens as the older Tint",
        &format!("{kinds:?}"),
        !kinds.is_empty() && kinds.iter().all(|k| *k == "the older Tint"),
    );
    t.root = repo("Fixtures/fxkey");
    t.fixtures_numbered("expected_fxkey.json", 4..=5);
    t.fixtures_numbered("expected_fxkey.json", 8..=9);
    t.round_trips(&["fx_fxk_004.json", "fx_fxk_005.json", "fx_fxk_008.json", "fx_fxk_009.json"]);
    t.root = root;

    t.heading("The file");
    // 008, its colours in capitals, is saved in small letters (the row after).
    let files: Vec<String> = (1..=16).filter(|n| *n != 8).map(|n| format!("fx_tint_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let caps = t.saved_parameters("fx_tint_008.json");
    t.row(
        "fx_tint_008.json, its colours written in capitals, is saved in small letters, as Gradient Map's are",
        &format!("{} {}", caps["map_black_to"], caps["map_white_to"]),
        caps["map_black_to"] == "#1a2a6c" && caps["map_white_to"] == "#fdbb2d",
    );
    let saved = t.saved_parameters("fx_tint_009.json");
    t.row(
        "fx_tint_009.json is saved with its three settings, the colours in small letters",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 3
            && saved["map_black_to"] == "#1a2a6c"
            && saved["map_white_to"] == "#fdbb2d"
            && saved["amount_to_tint"] == 30.0,
    );
    for (file, want) in [
        ("fx_tint_012.json", "amount_to_tint"),
        ("fx_tint_013.json", "amount_to_tint"),
        ("fx_tint_015.json", "Map Black To"),
        ("fx_tint_016.json", "Map White To"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        let says = why.contains(want) || why.contains(&want.replace('_', " "));
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, says);
    }
    t.shape_refused("fx_tint_001.json", "a Tint whose amount to tint is a word", r##"{"map_black_to": "#000000", "map_white_to": "#ffffff", "amount_to_tint": "all"}"##);
    t.shape_refused("fx_tint_001.json", "a Tint without its Map Black To", r##"{"map_white_to": "#ffffff", "amount_to_tint": 100}"##);
    t.shape_refused("fx_tint_001.json", "a Tint whose Map White To is a number", r##"{"map_black_to": "#000000", "map_white_to": 255, "amount_to_tint": 100}"##);

    t.heading("Commands");
    let mut document = t.load("fx_tint_001.json").document;
    let over = changed(&t, "fx_tint_001.json", |e| if let Effect::TintMap { amount_to_tint, .. } = e { *amount_to_tint = 101.0 });
    let bad = changed(&t, "fx_tint_001.json", |e| if let Effect::TintMap { map_white_to, .. } = e { *map_white_to = "#fff".into() });
    t.refused(
        &mut document,
        vec![
            ("amount to tint 101", set(over)),
            ("Map White To #fff", set(bad)),
            ("amount to tint keyed to 150", keys("amount_to_tint", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    let duo = changed(&t, "fx_tint_001.json", |e| {
        if let Effect::TintMap { map_black_to, map_white_to, .. } = e {
            *map_black_to = "#1a2a6c".into();
            *map_white_to = "#fdbb2d".into();
        }
    });
    t.taken(
        &mut document,
        "fx_tint_001.json",
        vec![
            ("navy to gold", set(duo)),
            ("amount to tint keyed from 0 to 100", keys("amount_to_tint", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_tint_001.json", 0),
        ("fx_tint_004.json", 0),
        ("fx_tint_009.json", 0),
        ("fx_tint_010.json", 2),
        ("fx_tint_011.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Amount 0 (002) and refused (012 to 016) leave nothing for the card.
    let none: Vec<u32> = std::iter::once(2).chain(12..=16).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            ("navy to gold at 30", json!({"map_black_to": "#1a2a6c", "map_white_to": "#fdbb2d", "amount_to_tint": 30}), 3),
            ("swapped, white to black", json!({"map_black_to": "#ffffff", "map_white_to": "#000000"}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-401 pictures/`");
    let dir = repo("verification/D-401 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));
    let older = |color: [f64; 3], amount: f64| {
        picture(&dir, "town.png", TOWN, json!([{"instance_id": "fx-0-0", "type_id": "core.tint", "enabled": true,
            "parameters": {"color": color, "amount": amount}}]))
    };
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = street(json!({"amount_to_tint": 0}));
    t.row(
        "amount to tint 0 changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );
    let grey = |p: &[u8]| p[0] == p[1] && p[1] == p[2];

    let (after, said) = street(json!({}));
    write("2_as_added.png", TOWN, &after);
    let coloured = after.chunks_exact(4).filter(|p| p[3] > 0 && !grey(p)).count();
    t.row(
        "2_as_added.png, as added (black to black, white to white, 100): the street in greys, no pixel coloured; draws cleanly",
        &format!("{said:?}, {coloured} coloured pixels"),
        said.is_empty() && coloured == 0 && after != before,
    );

    let (duo, said) = street(json!({"map_black_to": "#1a2a6c", "map_white_to": "#fdbb2d"}));
    write("3_navy_to_gold.png", TOWN, &duo);
    // Each pixel lies between the navy and the gold: its red at least the navy's and at most the gold's.
    let between = duo.chunks_exact(4).filter(|p| p[3] > 0).all(|p| (0x1a..=0xfd).contains(&p[0]) && (0x2a..=0xbb).contains(&p[1]) && (0x2d..=0x6c).contains(&p[2]));
    t.row(
        "3_navy_to_gold.png, navy (#1a2a6c) to gold (#fdbb2d): every pixel's red, green and blue between the navy's and the gold's; draws cleanly",
        &format!("{said:?}, all between: {between}"),
        said.is_empty() && between,
    );

    let (swapped, said) = street(json!({"map_black_to": "#fdbb2d", "map_white_to": "#1a2a6c"}));
    write("4_swapped.png", TOWN, &swapped);
    let luma = |p: &[u8]| 0.2126 * p[0] as f64 + 0.7152 * p[1] as f64 + 0.0722 * p[2] as f64;
    // The dark parts turn gold and the light parts navy: the street's lightness turned over.
    let turned = pairs(&before, &swapped).zip(pairs(&before, &duo)).filter(|((b, s), (_, d))| luma(b) < 60.0 && luma(s) > luma(d) || luma(b) > 200.0 && luma(s) < luma(d)).count();
    let ends = pairs(&before, &swapped).filter(|(b, _)| luma(b) < 60.0 || luma(b) > 200.0).count();
    t.row(
        "4_swapped.png, Swap Colors (gold to navy): the dark parts gold and the light parts navy, the other way from 3; draws cleanly",
        &format!("{said:?}, {turned} of the {ends} darkest and lightest pixels turned over"),
        said.is_empty() && ends > 0 && turned == ends,
    );

    let (part, said) = street(json!({"map_black_to": "#1a2a6c", "map_white_to": "#fdbb2d", "amount_to_tint": 30}));
    write("5_amount_30.png", TOWN, &part);
    t.row(
        "5_amount_30.png, navy to gold at amount 30: the street with a light wash of the two colours; draws cleanly",
        &format!("{said:?}, {} pixels changed, nearer the street than 3 is", distance(&part, &before).1),
        said.is_empty() && distance(&part, &before).0 < distance(&duo, &before).0,
    );

    // An older Tint, as a file saved before D-401 holds it, against the new Tint with one colour
    // at both ends: the same picture, within 1 level of 255 (the older rule works in single
    // precision).
    let lin = |v: f64| if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) };
    let (old, said0) = older([lin(0x64 as f64 / 255.0), lin(0x50 as f64 / 255.0), lin(0xa0 as f64 / 255.0)], 0.3);
    let (new, said1) = street(json!({"map_black_to": "#6450a0", "map_white_to": "#6450a0", "amount_to_tint": 30}));
    write("6_older_tint.png", TOWN, &old);
    t.row(
        "6_older_tint.png, an older Tint (colour #6450a0 in linear numbers, amount 0.3) against the new Tint with #6450a0 at both ends and amount 30: the same picture within 1 level; both draw cleanly",
        &format!("{said0:?} {said1:?}, largest difference {} of 255", distance(&old, &new).0),
        said0.is_empty() && said1.is_empty() && distance(&old, &new).0 <= 1 && old != before,
    );

    t.finish("D-401_tint_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B280_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-280: a measurement, run deliberately with --release --ignored"]
fn b280_tint_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B280_CPU").is_ok();
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
        ("Noise, then Tint as added (black to white, amount 100)", Some(json!({}))),
        ("Noise, then Tint, navy to gold at amount 30", Some(json!({"map_black_to": "#1a2a6c", "map_white_to": "#fdbb2d", "amount_to_tint": 30}))),
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
    let out = std::env::var("B280_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-280_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
