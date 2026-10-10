//! B-275: D-396, after After Effects' Selective Color (Photoshop's adjustment): the cyan,
//! magenta, yellow and black turned up or down in one family of colours at a time (reds,
//! yellows, greens, cyans, blues, magentas, whites, neutrals, blacks), relative or absolute.
//!
//! Writes `verification/D-396_selective_color_table.md` and draws pictures into
//! `verification/D-396 pictures/`.
//!
//! Every expected pixel is `Fixtures/selective_color/expected_selective_color.json`, written by
//! `tools/selective_color_reference.py` before this code existed and printed in document 25 as
//! FX-SELC-001 to 023. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{Effect, SELECTIVE_COLOR_FAMILIES};
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

/// The Selective Colors the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::SelectiveColor { .. })))
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

/// A Selective Color with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"method": "relative"});
    for name in SELECTIVE_COLOR_FAMILIES {
        all[name] = json!([0, 0, 0, 0]);
    }
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.selective_color", "enabled": true, "parameters": all})
}

/// The nine families of the reference's ALL case, then the method.
fn all_nine(method: &str) -> J {
    json!({
        "method": method,
        "reds": [-20, 10, 30, 5], "yellows": [15, -10, -40, 0], "greens": [30, -20, 10, -10],
        "cyans": [-30, 20, 0, 15], "blues": [20, 30, -20, 0], "magentas": [-10, -30, 20, 10],
        "whites": [0, 0, 10, -10], "neutrals": [5, -5, -15, 5], "blacks": [0, 10, 0, 20]
    })
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=23 {
        let file = format!("fx_selc_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b275-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Selective Color {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Selective Color {what} on three layers, frame {frame}, {}", quality.label()),
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
fn b275_selective_color() {
    let mut t = Table::new(
        "selective_color",
        "# D-396: Selective Color\n\nB-275, after After Effects' Selective Color (Photoshop's \
         adjustment): the cyan, magenta, yellow and black turned up or down in one family of \
         colours at a time (reds, yellows, greens, cyans, blues, magentas, whites, neutrals, \
         blacks), relative or absolute, by Clement Boesch's reverse-engineering of Photoshop's \
         (FFmpeg's selectivecolor), in fractions. Every expected pixel is \
         `Fixtures/selective_color/expected_selective_color.json`, written by \
         `tools/selective_color_reference.py` before this code existed and printed in document 25 \
         as FX-SELC-001 to 023. Tolerance 2e-5.\n",
    );

    t.heading("FX-SELC-001 to 023 (document 25)");
    t.fixtures_numbered("expected_selective_color.json", 1..=23);

    t.heading("The file");
    let files: Vec<String> = (1..=23).map(|n| format!("fx_selc_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_selc_013.json");
    t.row(
        "fx_selc_013.json is saved with its ten settings, the method and nine families of four",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 10
            && saved["method"] == "absolute"
            && saved["reds"].as_array().is_some_and(|a| a.iter().map(|v| v.as_f64()).eq([40.0, -60.0, 10.0, 20.0].map(Some)))
            && SELECTIVE_COLOR_FAMILIES.iter().all(|n| saved[n].as_array().is_some_and(|a| a.len() == 4)),
    );
    for (file, want) in [
        ("fx_selc_019.json", "reds"),
        ("fx_selc_020.json", "blacks"),
        ("fx_selc_021.json", "greens"),
        ("fx_selc_022.json", "method"),
        ("fx_selc_023.json", "method"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    let rest: String = SELECTIVE_COLOR_FAMILIES[1..].iter().map(|n| format!(r#", "{n}": [0, 0, 0, 0]"#)).collect();
    t.shape_refused(
        "fx_selc_001.json",
        "a Selective Color whose reds are words",
        &format!(r#"{{"method": "relative", "reds": "0, 0, 0, 0"{rest}}}"#),
    );
    t.shape_refused("fx_selc_001.json", "a Selective Color without its reds", &format!(r#"{{"method": "relative"{rest}}}"#));
    t.shape_refused(
        "fx_selc_001.json",
        "a Selective Color without its method",
        &format!(r#"{{"reds": [0, 0, 0, 0]{rest}}}"#),
    );

    t.heading("Commands");
    let mut document = t.load("fx_selc_001.json").document;
    let over = changed(&t, "fx_selc_001.json", |e| if let Effect::SelectiveColor { families, .. } = e { families[0][0] = 101.0 });
    let word = changed(&t, "fx_selc_001.json", |e| if let Effect::SelectiveColor { method, .. } = e { *method = "percentage".into() });
    let three = changed(&t, "fx_selc_001.json", |e| if let Effect::SelectiveColor { families, .. } = e { families[2] = vec![0.0; 3] });
    t.refused(
        &mut document,
        vec![
            ("reds cyan 101", set(over)),
            ("method \"percentage\"", set(word)),
            ("greens of three numbers", set(three)),
            ("reds keyed to cyan 150", keys("reds", &[(0, &[0.0, 0.0, 0.0, 0.0]), (4, &[150.0, 0.0, 0.0, 0.0])])),
        ],
    );
    let blue = changed(&t, "fx_selc_001.json", |e| {
        if let Effect::SelectiveColor { method, families } = e {
            *method = "absolute".into();
            families[4] = vec![0.0, 0.0, -100.0, 0.0];
        }
    });
    t.taken(
        &mut document,
        "fx_selc_001.json",
        vec![
            ("absolute, blues yellow -100", set(blue)),
            ("reds keyed from 0 to cyan 100", keys("reds", &[(0, &[0.0, 0.0, 0.0, 0.0]), (4, &[100.0, 0.0, 0.0, 0.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_selc_001.json", 0),
        ("fx_selc_013.json", 0),
        ("fx_selc_015.json", 0),
        ("fx_selc_016.json", 0),
        ("fx_selc_017.json", 2),
        ("fx_selc_018.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // As added (001, every amount 0) and refused (019 to 023) leave nothing for the card.
    let none: Vec<u32> = std::iter::once(1).chain(19..=23).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("all nine families, relative", all_nine("relative"), 3),
            ("all nine families, absolute", all_nine("absolute"), 3),
            ("reds cyan +100, absolute", json!({"method": "absolute", "reds": [100, 0, 0, 0]}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-396 pictures/`");
    let dir = repo("verification/D-396 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = street(json!({}));
    t.row(
        "as added (relative, every amount 0) changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );

    // Reds, cyan +100, absolute: only pixels whose red is the largest change, their red falls.
    let (after, said) = street(json!({"method": "absolute", "reds": [100, 0, 0, 0]}));
    write("2_reds_cyan.png", TOWN, &after);
    let others = pairs(&before, &after).filter(|(b, _)| b[0] < b[1].max(b[2])).all(|(b, a)| b == a);
    let falls = pairs(&before, &after).all(|(b, a)| a[0] <= b[0]);
    let moved = pairs(&before, &after).filter(|(b, a)| a[0] < b[0]).count();
    t.row(
        "2_reds_cyan.png, reds cyan +100, absolute: the red walls and skin lose red; pixels whose red is not the largest are untouched, no red rises; draws cleanly",
        &format!("{said:?}, others untouched: {others}, no red rises: {falls}, {moved} pixels lost red"),
        said.is_empty() && others && falls && moved > 0,
    );

    // Blues, yellow -100, absolute: only pixels whose blue is the largest change, their blue rises.
    let (after, said) = street(json!({"method": "absolute", "blues": [0, 0, -100, 0]}));
    write("3_blues_bluer.png", TOWN, &after);
    let others = pairs(&before, &after).filter(|(b, _)| b[2] < b[0].max(b[1])).all(|(b, a)| b == a);
    let rises = pairs(&before, &after).all(|(b, a)| a[2] >= b[2]);
    let moved = pairs(&before, &after).filter(|(b, a)| a[2] > b[2]).count();
    t.row(
        "3_blues_bluer.png, blues yellow -100, absolute: the sky bluer; pixels whose blue is not the largest are untouched, no blue falls; draws cleanly",
        &format!("{said:?}, others untouched: {others}, no blue falls: {rises}, {moved} pixels bluer"),
        said.is_empty() && others && rises && moved > 0,
    );

    // Blacks, black -50, relative: shadows lifted; nothing whose largest is at least half moves.
    let (after, said) = street(json!({"blacks": [0, 0, 0, -50]}));
    write("4_blacks_lifted.png", TOWN, &after);
    let bright = pairs(&before, &after).filter(|(b, _)| b[0].max(b[1]).max(b[2]) >= 128).all(|(b, a)| b == a);
    let lifts = pairs(&before, &after).all(|(b, a)| (0..3).all(|c| a[c] >= b[c]));
    let moved = pairs(&before, &after).filter(|(b, a)| a != b).count();
    t.row(
        "4_blacks_lifted.png, blacks black -50, relative: the shadows lifted; pixels whose largest channel is 128 or more untouched, nothing darker; draws cleanly",
        &format!("{said:?}, bright untouched: {bright}, nothing darker: {lifts}, {moved} pixels lifted"),
        said.is_empty() && bright && lifts && moved > 0,
    );

    // Whites, black +100, absolute: the lights darkened; nothing whose smallest is below half moves.
    let (after, said) = street(json!({"method": "absolute", "whites": [0, 0, 0, 100]}));
    write("5_whites_darker.png", TOWN, &after);
    let dark = pairs(&before, &after).filter(|(b, _)| b[0].min(b[1]).min(b[2]) <= 127).all(|(b, a)| b == a);
    let darkens = pairs(&before, &after).all(|(b, a)| (0..3).all(|c| a[c] <= b[c]));
    let moved = pairs(&before, &after).filter(|(b, a)| a != b).count();
    t.row(
        "5_whites_darker.png, whites black +100, absolute: the lightest pixels darkened; pixels whose smallest channel is 127 or less untouched, nothing brighter; draws cleanly",
        &format!("{said:?}, darker pixels untouched: {dark}, nothing brighter: {darkens}, {moved} pixels darkened"),
        said.is_empty() && dark && darkens && moved > 0,
    );

    let (after, said) = street(all_nine("relative"));
    write("6_all_nine.png", TOWN, &after);
    t.row(
        "6_all_nine.png, all nine families at once, relative: the street regraded; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&after, &before).1),
        said.is_empty() && after != before,
    );

    // Boesch's Photoshop measurements on (180, 100, 50), absolute, reds only, within 1 level.
    let orange: Vec<u8> = [180u8, 100, 50, 255].repeat(16);
    write("photoshop_orange.png", (4, 4), &orange);
    for (reds, channels, want) in [
        ([60, 0, 0, 0], 0..1, [132, 0, 0]),
        ([0, -60, 0, 0], 1..2, [0, 148, 0]),
        ([0, 0, -70, 0], 2..3, [0, 0, 106]),
        ([100, 100, 100, 0], 0..3, [124, 69, 34]),
        ([-100, -100, -100, 0], 0..3, [204, 149, 114]),
        ([40, -60, 10, -40], 0..1, [193, 0, 0]),
        ([40, -60, 10, 20], 0..1, [126, 0, 0]),
        ([40, -60, 10, -80], 2..3, [0, 0, 112]),
        ([40, -60, 10, -10], 2..3, [0, 0, 51]),
    ] {
        let (after, said) = picture(&dir, "photoshop_orange.png", (4, 4), json!([fx("fx-0-0", &json!({"method": "absolute", "reds": reds}))]));
        let got = &after[..3];
        let near = channels.clone().all(|c| got[c].abs_diff(want[c]) <= 1) && after.chunks_exact(4).all(|p| p == &after[..4]);
        let names = ["red", "green", "blue"];
        t.row(
            &format!(
                "Photoshop, measured by Boesch: (180, 100, 50) with reds {reds:?}, absolute, gives {} within 1 level; draws cleanly",
                channels.clone().map(|c| format!("{} {}", names[c], want[c])).collect::<Vec<_>>().join(", ")
            ),
            &format!("{said:?}, ({}, {}, {})", got[0], got[1], got[2]),
            said.is_empty() && near,
        );
    }

    t.finish("D-396_selective_color_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B275_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-275: a measurement, run deliberately with --release --ignored"]
fn b275_selective_color_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B275_CPU").is_ok();
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
        ("Noise, then Selective Color, reds cyan +100, absolute", Some(json!({"method": "absolute", "reds": [100, 0, 0, 0]}))),
        ("Noise, then Selective Color, all nine families, relative", Some(all_nine("relative"))),
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
    let out = std::env::var("B275_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-275_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
