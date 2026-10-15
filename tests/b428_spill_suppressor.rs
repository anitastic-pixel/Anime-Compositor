//! B-428: D-548, After Effects' Spill Suppressor (Keying, Obsolete) under
//! `core.spill_suppressor`: Color To Suppress and Suppression, a second name over Screen Key's
//! despill (D-540) with its matte turned off, as D-383 and D-394.
//!
//! Writes `verification/D-548_spill_suppressor_table.md` and draws pictures into
//! `verification/D-548 pictures/`.
//!
//! Every expected pixel is `Fixtures/spill_suppressor/expected_spill_suppressor.json`, written
//! by `tools/spill_suppressor_reference.py` before this code existed and printed in document 25
//! as FX-SSUP-001 to 015. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// A Spill Suppressor with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"color_to_suppress": "#0000ff", "suppression": 100});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.spill_suppressor", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=15 {
        let file = format!("fx_ssup_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::SpillSuppressor { .. }));
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
        let project = reference(|id| json!([fx(&format!("b428-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Spill Suppressor {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::SpillSuppressor { .. }));
                t.row(
                    &format!("the reference shot, Spill Suppressor {what} on three layers, frame {frame}, {}", quality.label()),
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

/// Solid pixels of `before` and `after` side by side.
fn solid<'a>(before: &'a [u8], after: &'a [u8]) -> impl Iterator<Item = (&'a [u8], &'a [u8])> {
    before.chunks_exact(4).zip(after.chunks_exact(4)).filter(|(b, _)| b[3] == 255)
}

/// By hand, in encoded 8-bit values: the leading channel `s` less `amount` of how far it stands
/// above the mean of the other two.
fn by_hand(p: &[u8], s: usize, amount: f64) -> f64 {
    let o: Vec<f64> = (0..3).filter(|&i| i != s).map(|i| p[i] as f64).collect();
    let v = p[s] as f64;
    v - amount * (v - (o[0] + o[1]) / 2.0).max(0.0)
}

#[test]
fn b428_spill_suppressor() {
    let mut t = Table::new(
        "spill_suppressor",
        "# D-548: Spill Suppressor\n\nB-428: `core.spill_suppressor` takes After Effects' \
         Spill Suppressor (Keying, Obsolete) controls, Color To Suppress and Suppression, as a \
         second name over Screen Key's despill (D-540, as D-383 and D-394) with its matte turned \
         off: the colour's leading channel is held toward the mean of the other two by the \
         suppression, every covering kept. There is no Color Accuracy (one careful rule for every \
         colour, as D-459). Every expected pixel is \
         `Fixtures/spill_suppressor/expected_spill_suppressor.json`, written by \
         `tools/spill_suppressor_reference.py` before this code existed and printed in document \
         25 as FX-SSUP-001 to 015; Screen Key's FX-SKEY-001 to 024 rerun unchanged. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-SSUP-001 to 015 (document 25)");
    t.fixtures_numbered("expected_spill_suppressor.json", 1..=15);

    t.heading("Screen Key, the engine under the second name: its fixtures, unchanged");
    let root = t.root.clone();
    t.root = repo("Fixtures/screen_key");
    t.fixtures_numbered("expected_screen_key.json", 1..=24);
    t.root = root;

    t.heading("The file");
    let files: Vec<String> = (1..=10).map(|n| format!("fx_ssup_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_ssup_002.json");
    t.row(
        "fx_ssup_002.json is saved with its two settings, colour to suppress #00ff00 and suppression 100",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 2 && saved["color_to_suppress"] == "#00ff00" && saved["suppression"] == 100.0,
    );
    for (file, want) in [
        ("fx_ssup_011.json", "suppression"),
        ("fx_ssup_012.json", "suppression"),
        ("fx_ssup_014.json", "color to suppress"),
        ("fx_ssup_015.json", "color to suppress"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    t.shape_refused("fx_ssup_001.json", "a Spill Suppressor whose suppression is a word", r##"{"color_to_suppress": "#0000ff", "suppression": "all"}"##);
    t.shape_refused("fx_ssup_001.json", "a Spill Suppressor without its colour", r#"{"suppression": 100}"#);
    t.shape_refused("fx_ssup_001.json", "a Spill Suppressor whose colour is a number", r#"{"color_to_suppress": 255, "suppression": 100}"#);

    t.heading("Commands");
    let mut document = t.load("fx_ssup_001.json").document;
    let over = changed(&t, "fx_ssup_001.json", |e| if let Effect::SpillSuppressor { suppression, .. } = e { *suppression = 101.0 });
    let word = changed(&t, "fx_ssup_001.json", |e| if let Effect::SpillSuppressor { color_to_suppress, .. } = e { *color_to_suppress = "green".into() });
    t.refused(
        &mut document,
        vec![
            ("suppression 101", set(over)),
            ("colour to suppress \"green\"", set(word)),
            ("suppression keyed to 150", keys("suppression", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    let picked = changed(&t, "fx_ssup_001.json", |e| {
        if let Effect::SpillSuppressor { color_to_suppress, suppression } = e {
            *color_to_suppress = "#28b43c".into();
            *suppression = 40.0;
        }
    });
    t.taken(
        &mut document,
        "fx_ssup_001.json",
        vec![
            ("a green screen's colour picked, suppression 40", set(picked)),
            ("suppression keyed from 0 to 100", keys("suppression", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_ssup_001.json", 0),
        ("fx_ssup_002.json", 0),
        ("fx_ssup_003.json", 0),
        ("fx_ssup_006.json", 0),
        ("fx_ssup_009.json", 2),
        ("fx_ssup_010.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Refused (011 to 015) leave nothing for the card.
    let none: Vec<u32> = (11..=15).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added (blue, 100)", json!({}), 3),
            ("green screen colour #28b43c, suppression 60", json!({"color_to_suppress": "#28b43c", "suppression": 60}), 3),
            ("red, suppression 30", json!({"color_to_suppress": "#ff0000", "suppression": 30}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-548 pictures/`");
    let dir = repo("verification/D-548 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());

    let (same, said) = street(json!({"suppression": 0}));
    t.row(
        "suppression 0 changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );

    // Each solid pixel, by hand: the leading channel `s` held toward the mean of the other two
    // by `amount`, the other two untouched; the worst miss in 8-bit levels and how many pixels
    // lost some of `s`.
    let check = |after: &[u8], s: usize, amount: f64| {
        let (mut worst, mut lost) = (0.0f64, 0);
        for (b, a) in solid(&before, after) {
            for i in 0..3 {
                let want = if i == s { by_hand(b, s, amount) } else { b[i] as f64 };
                worst = worst.max((a[i] as f64 - want).abs());
            }
            lost += (a[s] < b[s]) as usize;
        }
        (worst, lost)
    };
    for (name, p, s, amount, what) in [
        ("2_as_added.png", json!({}), 2, 1.0, "as added (blue, 100): every solid pixel's blue held no higher than the mean of its red and green, red and green untouched"),
        ("3_green.png", json!({"color_to_suppress": "#28b43c"}), 1, 1.0, "a green screen's colour picked (#28b43c, green leads): every solid pixel's green held no higher than the mean of its red and blue, red and blue untouched"),
        ("4_green_50.png", json!({"color_to_suppress": "#28b43c", "suppression": 50}), 1, 0.5, "the same at suppression 50: green brought half way down, red and blue untouched"),
        ("5_red.png", json!({"color_to_suppress": "#ff0000"}), 0, 1.0, "red: every solid pixel's red held no higher than the mean of its green and blue"),
    ] {
        let (after, said) = street(p);
        write(name, TOWN, &after);
        let (worst, lost) = check(&after, s, amount);
        t.row(
            &format!("{name}, {what}, each pixel within 1 level of the rule worked by hand; draws cleanly"),
            &format!("{said:?}, worst miss {worst:.2} levels, {lost} pixels lose some"),
            said.is_empty() && worst <= 1.0 && lost > 0,
        );
    }

    // The second name: Screen Key with gain 0 (so every covering kept), balance 50, clip 0 to
    // 100, despill bias 50 and despill the suppression draws the same street byte for byte.
    for (colour, amount) in [("#0000ff", 100), ("#28b43c", 60), ("#ff0000", 30)] {
        let key = json!({"instance_id": "fx-0-0", "type_id": "core.screen_key", "enabled": true, "parameters": {
            "view": "final", "screen_color": colour, "screen_gain": 0, "screen_balance": 50, "clip_black": 0, "clip_white": 100,
            "despill_bias": 50, "despill": amount, "shrink_grow": 0, "softness": 0, "edge_refine": 0}});
        let (a, said0) = street(json!({"color_to_suppress": colour, "suppression": amount}));
        let (b, said1) = picture(&dir, "town.png", TOWN, json!([key]));
        t.row(
            &format!("Spill Suppressor {colour} at {amount} against Screen Key with gain 0, balance 50, despill bias 50 and despill {amount}: the same street byte for byte; both draw cleanly"),
            &format!("{said0:?} {said1:?}, {} pixels differ", distance(&a, &b).1),
            said0.is_empty() && said1.is_empty() && a == b && a != before,
        );
    }

    t.finish("D-548_spill_suppressor_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B428_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-428: a measurement, run deliberately with --release --ignored"]
fn b428_spill_suppressor_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B428_CPU").is_ok();
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
        ("Noise, then Spill Suppressor as added (blue, 100)", Some(json!({}))),
        ("Noise, then Spill Suppressor #28b43c at 60", Some(json!({"color_to_suppress": "#28b43c", "suppression": 60}))),
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
    let out = std::env::var("B428_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-428_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
