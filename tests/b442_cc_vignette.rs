//! B-442: D-562, After Effects' CC Vignette under `core.cc_vignette`: Amount, Angle of View,
//! Center and Pin Highlights, a second effect beside D-126's Vignette.
//!
//! Writes `verification/D-562_cc_vignette_table.md` and draws pictures into
//! `verification/D-562 pictures/`.
//!
//! Every expected pixel is `Fixtures/cc_vignette/expected_cc_vignette.json`, written by
//! `tools/cc_vignette_reference.py` before this code existed and printed in document 25 as
//! FX-CCVIG-001 to 025. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, keys, median, on_card, reference, repo, set, time_passes, town, Table, MAIN, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// A CC Vignette with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"amount": 100, "angle_of_view": 90, "center": [50, 50], "pin_highlights": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.cc_vignette", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=25 {
        let file = format!("fx_ccvig_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::CcVignette { .. }));
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
        let project = reference(|id| json!([fx(&format!("b442-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, CC Vignette {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::CcVignette { .. }));
                t.row(
                    &format!("the reference shot, CC Vignette {what} on three layers, frame {frame}, {}", quality.label()),
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

/// Covered pixels of `before` and `after` side by side.
fn pairs<'a>(before: &'a [u8], after: &'a [u8]) -> impl Iterator<Item = (&'a [u8], &'a [u8])> {
    before.chunks_exact(4).zip(after.chunks_exact(4)).filter(|(b, _)| b[3] > 0)
}

/// The sum of the three channels at `(x, y)` of a straight 8-bit picture `w` wide.
fn light(p: &[u8], w: usize, x: usize, y: usize) -> u32 {
    let i = 4 * (y * w + x);
    p[i..i + 3].iter().map(|v| *v as u32).sum()
}

#[test]
fn b442_cc_vignette() {
    let mut t = Table::new(
        "cc_vignette",
        "# D-562: CC Vignette\n\nB-442: `core.cc_vignette` takes After Effects' CC Vignette controls, \
         Amount, Angle of View, Center and Pin Highlights: a second effect beside D-126's Vignette, \
         the light falling off as a lens's does (cos^4 of the angle it reaches the film at), \
         lightening with a negative amount, bright pixels kept by Pin Highlights. Every expected \
         pixel is `Fixtures/cc_vignette/expected_cc_vignette.json`, written by \
         `tools/cc_vignette_reference.py` before this code existed and printed in document 25 as \
         FX-CCVIG-001 to 025. Tolerance 2e-5.\n",
    );

    t.heading("FX-CCVIG-001 to 025 (document 25)");
    t.fixtures_numbered("expected_cc_vignette.json", 1..=25);

    t.heading("The file");
    let files: Vec<String> = (1..=25).map(|n| format!("fx_ccvig_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_ccvig_012.json");
    t.row(
        "fx_ccvig_012.json is saved with its four settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 4
            && saved["amount"] == -100.0
            && saved["angle_of_view"] == 90.0
            && saved["center"] == json!([50, 50])
            && saved["pin_highlights"] == 100.0,
    );
    for (file, want) in [
        ("fx_ccvig_019.json", "amount"),
        ("fx_ccvig_020.json", "amount"),
        ("fx_ccvig_021.json", "angle_of_view"),
        ("fx_ccvig_022.json", "angle_of_view"),
        ("fx_ccvig_023.json", "pin_highlights"),
        ("fx_ccvig_024.json", "center"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        let says = why.contains(want) || why.contains(&want.replace('_', " "));
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, says);
    }
    t.shape_refused("fx_ccvig_001.json", "a CC Vignette whose amount is a word", r#"{"amount": "lots", "angle_of_view": 90, "center": [50, 50], "pin_highlights": 0}"#);
    t.shape_refused("fx_ccvig_001.json", "a CC Vignette without its angle of view", r#"{"amount": 100, "center": [50, 50], "pin_highlights": 0}"#);
    t.shape_refused("fx_ccvig_001.json", "a CC Vignette whose centre is one number", r#"{"amount": 100, "angle_of_view": 90, "center": 50, "pin_highlights": 0}"#);

    t.heading("Commands");
    let mut document = t.load("fx_ccvig_001.json").document;
    let over = changed(&t, "fx_ccvig_001.json", |e| if let Effect::CcVignette { amount, .. } = e { *amount = 101.0 });
    let wide = changed(&t, "fx_ccvig_001.json", |e| if let Effect::CcVignette { angle_of_view, .. } = e { *angle_of_view = 180.0 });
    t.refused(
        &mut document,
        vec![
            ("amount 101", set(over)),
            ("angle of view 180", set(wide)),
            ("pin highlights keyed to 150", keys("pin_highlights", &[(0, &[0.0]), (4, &[150.0])])),
            ("centre keyed to 50, -1001", keys("center", &[(0, &[50.0, 50.0]), (4, &[50.0, -1001.0])])),
        ],
    );
    let bright = changed(&t, "fx_ccvig_001.json", |e| {
        if let Effect::CcVignette { amount, pin_highlights, .. } = e {
            *amount = -40.0;
            *pin_highlights = 60.0;
        }
    });
    t.taken(
        &mut document,
        "fx_ccvig_001.json",
        vec![
            ("amount -40, pin highlights 60", set(bright)),
            ("angle of view keyed from 0 to 179", keys("angle_of_view", &[(0, &[0.0]), (4, &[179.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_ccvig_001.json", 0),
        ("fx_ccvig_004.json", 0),
        ("fx_ccvig_007.json", 0),
        ("fx_ccvig_009.json", 0),
        ("fx_ccvig_012.json", 0),
        ("fx_ccvig_015.json", 2),
        ("fx_ccvig_017.json", 3),
        ("fx_ccvig_018.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Amount 0 (003), angle of view 0 (006) and refused (019 to 025) leave nothing for the card.
    let none: Vec<u32> = [3, 6].into_iter().chain(19..=25).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            ("amount -60, angle of view 150, pin highlights 50", json!({"amount": -60, "angle_of_view": 150, "pin_highlights": 50}), 3),
            ("angle of view 120 about 20, 80", json!({"angle_of_view": 120, "center": [20, 80]}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-562 pictures/`");
    let dir = repo("verification/D-562 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    for (what, p) in [("amount 0", json!({"amount": 0})), ("angle of view 0", json!({"angle_of_view": 0}))] {
        let (same, said) = street(p);
        t.row(
            &format!("{what} changes nothing: the street byte for byte; draws cleanly"),
            &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
            said.is_empty() && same == before,
        );
    }

    let (w, h) = TOWN;
    let corner = |p: &[u8]| light(p, w, 1, 1);
    let far = |p: &[u8]| light(p, w, w - 2, h - 2);
    let middle = |p: &[u8]| light(p, w, w / 2, h / 2);
    let (added, said) = street(json!({}));
    write("2_as_added.png", TOWN, &added);
    t.row(
        "2_as_added.png, as added (amount 100, angle of view 90): both far corners darker than the street's, the middle within 3 levels a channel; draws cleanly",
        &format!("{said:?}, corners {} and {} against {} and {}, middle {} against {}", corner(&added), far(&added), corner(&before), far(&before), middle(&added), middle(&before)),
        said.is_empty() && corner(&added) < corner(&before) && far(&added) < far(&before) && middle(&before).abs_diff(middle(&added)) <= 9,
    );

    let (lighter, said) = street(json!({"amount": -100}));
    write("3_amount_minus_100.png", TOWN, &lighter);
    let none_darker = pairs(&before, &lighter).all(|(b, a)| a[..3].iter().zip(&b[..3]).all(|(x, y)| u16::from(*x) + 1 >= u16::from(*y)));
    t.row(
        "3_amount_minus_100.png, amount -100: the corners lighter than the street's and no pixel darker; draws cleanly",
        &format!("{said:?}, corners {} and {} against {} and {}, none darker: {none_darker}", corner(&lighter), far(&lighter), corner(&before), far(&before)),
        said.is_empty() && corner(&lighter) > corner(&before) && far(&lighter) > far(&before) && none_darker,
    );

    let (widest, said) = street(json!({"angle_of_view": 150}));
    write("4_angle_of_view_150.png", TOWN, &widest);
    t.row(
        "4_angle_of_view_150.png, a wide lens (angle of view 150): the corners darker still than 2_as_added.png's; draws cleanly",
        &format!("{said:?}, corners {} and {} against 2's {} and {}", corner(&widest), far(&widest), corner(&added), far(&added)),
        said.is_empty() && corner(&widest) < corner(&added) && far(&widest) < far(&added),
    );

    let (pinned, said) = street(json!({"pin_highlights": 100}));
    write("5_pin_highlights_100.png", TOWN, &pinned);
    let kept = pairs(&added, &pinned).all(|(a, p)| p[..3].iter().zip(&a[..3]).all(|(x, y)| u16::from(*x) + 1 >= u16::from(*y)));
    t.row(
        "5_pin_highlights_100.png, as added with Pin Highlights 100: no pixel darker than in 2_as_added.png, and the picture nearer the street; draws cleanly",
        &format!("{said:?}, none darker than 2: {kept}, largest difference from the street {} against 2's {}", distance(&pinned, &before).0, distance(&added, &before).0),
        said.is_empty() && kept && pinned != added,
    );

    let (aside, said) = street(json!({"center": [0, 0], "angle_of_view": 120}));
    write("6_centre_top_left.png", TOWN, &aside);
    t.row(
        "6_centre_top_left.png, the centre at the top left corner, angle of view 120: the top left as the street, the bottom right darkest; draws cleanly",
        &format!("{said:?}, top left {} against {}, bottom right {} against {}", corner(&aside), corner(&before), far(&aside), far(&before)),
        said.is_empty() && corner(&before).abs_diff(corner(&aside)) <= 3 && far(&aside) < far(&added),
    );

    t.finish("D-562_cc_vignette_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B442_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-442: a measurement, run deliberately with --release --ignored"]
fn b442_cc_vignette_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B442_CPU").is_ok();
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
        ("Noise, then CC Vignette as added", Some(json!({}))),
        ("Noise, then CC Vignette, amount -60, angle of view 150, pin highlights 50", Some(json!({"amount": -60, "angle_of_view": 150, "pin_highlights": 50}))),
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
        let (first, times) = time_passes(&mut gpu, cpu, passes, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B442_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-442_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
