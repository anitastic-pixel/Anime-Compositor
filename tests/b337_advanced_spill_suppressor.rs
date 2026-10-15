//! B-337: D-457, after After Effects' Advanced Spill Suppressor (Keying): the green or blue a
//! screen throws onto a keyed subject taken back out. Standard cuts green or blue, whichever
//! leads, to the larger other channel; Ultra works from a key colour, with a hue window
//! (Tolerance), a cut (Spill Range), colour given back (Spill Color Correction), greying
//! (Desaturate) and brightness put back (Luma Correction). Adobe publishes no formula: the rule
//! is ours, stated in `tools/advanced_spill_suppressor_reference.py`.
//!
//! Writes `verification/D-457_advanced_spill_suppressor_table.md` and draws pictures into
//! `verification/D-457 pictures/`.
//!
//! Every expected pixel is
//! `Fixtures/advanced_spill_suppressor/expected_advanced_spill_suppressor.json`, written by the
//! tool before this code existed and printed in document 25 as FX-SPILL-001 to 028. Tolerance
//! 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, keys, median, on_card, reference, repo, set, time_passes, town, Table,
    MAIN, TOWN,
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

/// An Advanced Spill Suppressor with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"method": "standard", "suppression": 100, "key_color": "#00ff00", "tolerance": 50,
        "desaturate": 25, "spill_range": 50, "spill_color_correction": 50, "luma_correction": 50});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.advanced_spill_suppressor", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=28 {
        let file = format!("fx_spill_{n:03}.json");
        let project = persist::load(&t.root.join(&file))
            .unwrap()
            .document
            .project()
            .clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| {
                    matches!(e, Effect::AdvancedSpillSuppressor { .. })
                });
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
        preview::preview_frame_cached(
            project,
            &ref_comp,
            100,
            &ref_root,
            PreviewQuality::Full,
            DEFAULT_TILE_SIZE,
            &mut log,
            &mut CelCache::viewer(),
        )
        .expect("the reference shot draws")
        .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in settings {
        let project = reference(|id| json!([fx(&format!("b337-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Advanced Spill Suppressor {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| {
                    matches!(e, Effect::AdvancedSpillSuppressor { .. })
                });
                t.row(
                    &format!("the reference shot, Advanced Spill Suppressor {what} on three layers, frame {frame}, {}", quality.label()),
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
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap(),
    )
    .unwrap();
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
    let frame = render_frame(
        loaded.document.project(),
        &Id::new(MAIN),
        0,
        dir,
        64,
        &mut log,
    )
    .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
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
                std::array::from_fn(|c| {
                    (skin[c] + (SCREEN[c] as f64 - skin[c]) * spill).round() as u8
                })
            } else {
                SCREEN
            };
            bytes.extend([c[0], c[1], c[2], 255]);
        }
    }
    bytes
}

/// How far round the colour wheel the 8-bit colour `b` lies from pure green, in degrees; 0 for
/// a grey.
fn from_green(b: &[u8]) -> f64 {
    let [r, g, bl] = [b[0], b[1], b[2]].map(f64::from);
    let (mx, mn) = (r.max(g).max(bl), r.min(g).min(bl));
    if mx == mn {
        return 0.0;
    }
    let d = mx - mn;
    let h = if r == mx {
        (60.0 * (g - bl) / d).rem_euclid(360.0)
    } else if g == mx {
        60.0 * ((bl - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    ((h - 120.0 + 180.0).rem_euclid(360.0) - 180.0).abs()
}

/// Covered pixels of `before` and `after` side by side.
fn pairs<'a>(before: &'a [u8], after: &'a [u8]) -> impl Iterator<Item = (&'a [u8], &'a [u8])> {
    before
        .chunks_exact(4)
        .zip(after.chunks_exact(4))
        .filter(|(b, _)| b[3] > 0)
}

#[test]
fn b337_advanced_spill_suppressor() {
    let mut t = Table::new(
        "advanced_spill_suppressor",
        "# D-457: Advanced Spill Suppressor\n\nB-337, after After Effects' Advanced Spill \
         Suppressor (Keying): the green or blue a screen throws onto a keyed subject taken back \
         out. Adobe names the controls but publishes no formula, so the rule is ours, stated in \
         `tools/advanced_spill_suppressor_reference.py`: Standard cuts green or blue, whichever \
         leads, to the larger other channel; Ultra cuts the key's screen channel toward a blend of \
         the other two (Spill Range), weighted by a hue window (Tolerance), gives part back (Spill \
         Color Correction), greys by the share taken (Desaturate) and puts back lost brightness \
         (Luma Correction). Every expected pixel is \
         `Fixtures/advanced_spill_suppressor/expected_advanced_spill_suppressor.json`, written by \
         the tool before this code existed and printed in document 25 as FX-SPILL-001 to 028. \
         Tolerance 2e-5.\n",
    );

    t.heading("FX-SPILL-001 to 028 (document 25)");
    t.fixtures_numbered("expected_advanced_spill_suppressor.json", 1..=28);

    t.heading("The file");
    let files: Vec<String> = (1..=28).map(|n| format!("fx_spill_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_spill_013.json");
    t.row(
        "fx_spill_013.json is saved with its eight settings, the method Ultra and the key #3cbe50",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 8
            && saved["method"] == "ultra"
            && saved["key_color"] == "#3cbe50",
    );
    for (file, want) in [
        ("fx_spill_019.json", "suppression"),
        ("fx_spill_020.json", "suppression"),
        ("fx_spill_021.json", "tolerance"),
        ("fx_spill_022.json", "desaturate"),
        ("fx_spill_023.json", "spill range"),
        ("fx_spill_024.json", "spill color correction"),
        ("fx_spill_025.json", "luma correction"),
        ("fx_spill_026.json", "method"),
        ("fx_spill_027.json", "method"),
        ("fx_spill_028.json", "key colour"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming {want}"),
            &why,
            why.contains(want),
        );
    }
    let rest = r##""key_color": "#00ff00", "tolerance": 50, "desaturate": 25, "spill_range": 50, "spill_color_correction": 50, "luma_correction": 50"##;
    t.shape_refused(
        "fx_spill_001.json",
        "an Advanced Spill Suppressor whose suppression is a word",
        &format!(r#"{{"method": "standard", "suppression": "full", {rest}}}"#),
    );
    t.shape_refused(
        "fx_spill_001.json",
        "an Advanced Spill Suppressor without its method",
        &format!(r#"{{"suppression": 100, {rest}}}"#),
    );
    t.shape_refused(
        "fx_spill_001.json",
        "an Advanced Spill Suppressor without its key colour",
        r#"{"method": "standard", "suppression": 100, "tolerance": 50, "desaturate": 25, "spill_range": 50, "spill_color_correction": 50, "luma_correction": 50}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_spill_001.json").document;
    let over = changed(&t, "fx_spill_001.json", |e| {
        if let Effect::AdvancedSpillSuppressor { suppression, .. } = e {
            *suppression = 101.0
        }
    });
    let word = changed(&t, "fx_spill_001.json", |e| {
        if let Effect::AdvancedSpillSuppressor { method, .. } = e {
            *method = "auto".into()
        }
    });
    let colour = changed(&t, "fx_spill_001.json", |e| {
        if let Effect::AdvancedSpillSuppressor { key_color, .. } = e {
            *key_color = "green".into()
        }
    });
    t.refused(
        &mut document,
        vec![
            ("suppression 101", set(over)),
            ("method \"auto\"", set(word)),
            ("key colour \"green\"", set(colour)),
            (
                "tolerance keyed to 150",
                keys("tolerance", &[(0, &[50.0]), (4, &[150.0])]),
            ),
        ],
    );
    let blue = changed(&t, "fx_spill_001.json", |e| {
        if let Effect::AdvancedSpillSuppressor {
            method, key_color, ..
        } = e
        {
            *method = "ultra".into();
            *key_color = "#0000ff".into();
        }
    });
    t.taken(
        &mut document,
        "fx_spill_001.json",
        vec![
            ("Ultra with a blue key", set(blue)),
            (
                "suppression keyed from 0 to 100",
                keys("suppression", &[(0, &[0.0]), (4, &[100.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_spill_001.json", 0),
        ("fx_spill_004.json", 0),
        ("fx_spill_010.json", 0),
        ("fx_spill_014.json", 0),
        ("fx_spill_016.json", 2),
        ("fx_spill_018.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Suppression 0 (002), Ultra at tolerance 0 (006) and refused (019 to 028) leave nothing for
    // the card.
    let none: Vec<u32> = [2, 6].into_iter().chain(19..=28).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("Standard at 100, as added", json!({}), 3),
            ("Ultra as added, green key", json!({"method": "ultra"}), 3),
            (
                "Ultra, blue key, tolerance 80, desaturate 60, spill range 30, correction 90, luma 20",
                json!({"method": "ultra", "key_color": "#2040e0", "tolerance": 80, "desaturate": 60, "spill_range": 30, "spill_color_correction": 90, "luma_correction": 20}),
                3,
            ),
        ],
    );

    t.heading("Pictures: in `verification/D-457 pictures/`");
    let dir = repo("verification/D-457 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| {
        png_out::write_rgba(
            &dir.join(name),
            size.0,
            size.1,
            OutputDepth::Eight,
            &[],
            bytes,
        )
        .unwrap()
    };
    write("portrait.png", TOWN, &portrait());
    write("town.png", TOWN, &town());
    let shot = |p: J| picture(&dir, "portrait.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "portrait.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the green-screen shot with no effect (a face whose edge is spilled green, a blue shirt); draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = shot(json!({"suppression": 0}));
    t.row(
        "suppression 0 changes nothing: the shot byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );

    // Standard: pixels whose red leads are untouched, no green or blue lead is left, and the
    // blue shirt loses its blue too (Standard does not know which colour the screen was).
    let (after, said) = shot(json!({}));
    write("2_standard.png", TOWN, &after);
    let red_leads = pairs(&before, &after)
        .filter(|(b, _)| b[0] >= b[1] && b[0] >= b[2])
        .all(|(b, a)| b == a);
    let no_lead = pairs(&before, &after)
        .all(|(_, a)| a[1] <= a[0].max(a[2]) + 1 && a[2] <= a[0].max(a[1]) + 1);
    let skin_edge = pairs(&before, &after)
        .filter(|(b, a)| b[1] > b[0] && a[1] < b[1] && b[..3] != SCREEN)
        .count();
    let shirt_hit = pairs(&before, &after).any(|(b, a)| b[..3] == SHIRT && a[2] < b[2]);
    t.row(
        "2_standard.png, Standard at 100: the screen turns grey, the green edge of the face clears; pixels whose red leads (the face) untouched; no green or blue lead left anywhere; the blue shirt loses its blue too, Standard's known limit; draws cleanly",
        &format!("{said:?}, red-led untouched: {red_leads}, no lead left: {no_lead}, {skin_edge} edge pixels cleared, shirt hit: {shirt_hit}"),
        said.is_empty() && red_leads && no_lead && skin_edge > 0 && shirt_hit,
    );

    // Ultra with the green key: the shirt (about 110 degrees from green) untouched, green falls.
    let (after, said) = shot(json!({"method": "ultra"}));
    write("3_ultra_green_key.png", TOWN, &after);
    let shirt_kept = pairs(&before, &after)
        .filter(|(b, _)| b[..3] == SHIRT)
        .all(|(b, a)| b == a);
    let green_falls = pairs(&before, &after).all(|(b, a)| a[1] <= b[1]);
    let moved = pairs(&before, &after).filter(|(b, a)| a[1] < b[1]).count();
    t.row(
        "3_ultra_green_key.png, Ultra as added (green key): the screen and the face's green edge cleaned; the blue shirt untouched; no pixel's green rises; draws cleanly",
        &format!("{said:?}, shirt untouched: {shirt_kept}, no green rises: {green_falls}, {moved} pixels lost green"),
        said.is_empty() && shirt_kept && green_falls && moved > 0,
    );

    // Ultra, correction 100, desaturate 0, luma 0: what green loses goes to red and blue.
    let (after, said) = shot(
        json!({"method": "ultra", "spill_color_correction": 100, "desaturate": 0, "luma_correction": 0}),
    );
    write("4_ultra_correction.png", TOWN, &after);
    let given = pairs(&before, &after)
        .filter(|(b, a)| a[1] < b[1])
        .all(|(b, a)| a[0] >= b[0] && a[2] >= b[2]);
    t.row(
        "4_ultra_correction.png, Ultra with spill colour correction 100, no desaturate, no luma: every pixel that loses green gains red and blue, so the cleaned screen turns a muted pink; draws cleanly",
        &format!("{said:?}, red and blue never fall where green fell: {given}"),
        said.is_empty() && given,
    );

    // The street, with no screen in it: Standard greys its sky; Ultra with the green key leaves
    // every colour 90 degrees or more from green alone.
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (plain, _) = picture(&dir, "town.png", TOWN, json!([]));
    let (after, said) = street(json!({}));
    write("5_street_standard.png", TOWN, &after);
    let sky_cut = pairs(&plain, &after)
        .filter(|(b, _)| b[2] > b[1])
        .all(|(_, a)| a[2] <= a[1] + 1);
    t.row(
        "5_street_standard.png, the street with Standard at 100: a scene with no screen still loses every green or blue lead (the sky greys to its green); Standard is for keyed shots only; draws cleanly",
        &format!("{said:?}, sky's blue down to its green: {sky_cut}"),
        said.is_empty() && sky_cut,
    );
    let (after, said) = street(json!({"method": "ultra"}));
    write("6_street_ultra.png", TOWN, &after);
    let far = pairs(&plain, &after).filter(|(b, _)| from_green(b) >= 91.0);
    let (far_count, far_kept) = far.fold((0, true), |(n, k), (b, a)| (n + 1, k && b == a));
    t.row(
        "6_street_ultra.png, the street with Ultra as added (green key): every pixel 91 degrees or more round the colour wheel from green (most of the sky, the red and blue walls) untouched; draws cleanly",
        &format!("{said:?}, {far_count} such pixels, untouched: {far_kept}, {} pixels changed", distance(&after, &plain).1),
        said.is_empty() && far_count > 0 && far_kept,
    );

    // By hand, the rule on one colour: the green-spilled skin (200, 214, 160), Standard at 100,
    // gives (200, 200, 160); at 50 its green is 207.
    let skin: Vec<u8> = [200u8, 214, 160, 255].repeat(16);
    write("skin.png", (4, 4), &skin);
    for (suppression, want) in [(100, [200, 200, 160]), (50, [200, 207, 160])] {
        let (after, said) = picture(
            &dir,
            "skin.png",
            (4, 4),
            json!([fx("fx-0-0", &json!({"suppression": suppression}))]),
        );
        t.row(
            &format!("by hand: the skin (200, 214, 160), Standard at {suppression}, gives {want:?}; draws cleanly"),
            &format!("{said:?}, ({}, {}, {})", after[0], after[1], after[2]),
            said.is_empty() && (0..3).all(|c| after[c].abs_diff(want[c]) <= 1),
        );
    }

    t.finish("D-457_advanced_spill_suppressor_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B337_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-337: a measurement, run deliberately with --release --ignored"]
fn b337_advanced_spill_suppressor_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B337_CPU").is_ok();
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
        (
            "Noise, then Advanced Spill Suppressor, Standard",
            Some(json!({})),
        ),
        (
            "Noise, then Advanced Spill Suppressor, Ultra",
            Some(json!({"method": "ultra"})),
        ),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![
                json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}}),
            ];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}c"), p));
            }
            J::Array(v)
        });
        let (comp, root) = (
            Id::new("comp-reference-shot"),
            repo("Fixtures/reference_shot"),
        );
        let (first, times) = time_passes(
            &mut gpu,
            cpu,
            passes,
            &project,
            &comp,
            &root,
            PreviewQuality::Full,
        );
        let _ = writeln!(
            s,
            "| {name} | Full | {:.1} | {:.1} |",
            median(first),
            median(times)
        );
    }
    let out = std::env::var("B337_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-337_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
