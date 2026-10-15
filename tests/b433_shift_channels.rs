//! B-433: D-553, a fixture folder for `core.shift_channels` (D-305, B-190), after After Effects'
//! Shift Channels (Channel). No code changes: the rule is document 21's, pinned here against
//! numbers worked a second way.
//!
//! Writes `verification/D-553_shift_channels_table.md` and draws pictures into
//! `verification/D-553 pictures/`.
//!
//! Every expected pixel is `Fixtures/shift_channels/expected_shift_channels.json`, written by
//! `tools/shift_channels_reference.py` before this code existed and printed in document 25 as
//! FX-SHIFT-001 to 020. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, median, on_card, reference, repo, set, time_passes, town, Table, MAIN, TOWN,
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

/// A Shift Channels with the settings `p`, every one `p` leaves out at its start.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"take_alpha": "alpha", "take_red": "red", "take_green": "green", "take_blue": "blue"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.shift_channels", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=20 {
        let file = format!("fx_shift_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::ShiftChannels { .. }));
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
/// and Draft, on the card against the processor; 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J)]) {
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
        let project = reference(|id| json!([fx(&format!("b433-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Shift Channels {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::ShiftChannels { .. }));
                t.row(
                    &format!("the reference shot, Shift Channels {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
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

#[test]
fn b433_shift_channels() {
    let mut t = Table::new(
        "shift_channels",
        "# D-553: Shift Channels, its fixture folder\n\nB-433: `core.shift_channels` (D-305, built in \
         B-190 from P-26's Colorful Glitch tutorial) takes each of a drawing's alpha, red, green \
         and blue from a channel or a measure of its colour, after After Effects' Shift Channels \
         (Channel). B-190 pinned it on hand-worked solids; this unit gives it a fixture folder. \
         Every expected pixel is `Fixtures/shift_channels/expected_shift_channels.json`, written \
         by `tools/shift_channels_reference.py` before this code existed and printed in document \
         25 as FX-SHIFT-001 to 020. No code changed. Tolerance 2e-5.\n",
    );

    t.heading("FX-SHIFT-001 to 020 (document 25)");
    t.fixtures_numbered("expected_shift_channels.json", 1..=20);

    t.heading("The file");
    let files: Vec<String> = (1..=20).map(|n| format!("fx_shift_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_shift_015.json");
    t.row(
        "fx_shift_015.json is saved with its four settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 4
            && saved["take_alpha"] == "luminance"
            && saved["take_red"] == "hue"
            && saved["take_green"] == "saturation"
            && saved["take_blue"] == "lightness",
    );
    for (file, want) in [
        ("fx_shift_017.json", "takes red from"),
        ("fx_shift_018.json", "takes alpha from"),
        ("fx_shift_019.json", "takes green from"),
        ("fx_shift_020.json", "takes blue from"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence saying Shift Channels {want} ..."), &why, why.contains(want));
    }
    t.shape_refused("fx_shift_001.json", "a Shift Channels whose take red is a number", r#"{"take_alpha": "alpha", "take_red": 1, "take_green": "green", "take_blue": "blue"}"#);
    t.shape_refused("fx_shift_001.json", "a Shift Channels without its take blue", r#"{"take_alpha": "alpha", "take_red": "red", "take_green": "green"}"#);

    t.heading("Commands");
    let mut document = t.load("fx_shift_001.json").document;
    let word = |what: &'static str, w: &'static str| {
        changed(&t, "fx_shift_001.json", move |e| {
            if let Effect::ShiftChannels { take_alpha, take_red, take_green, take_blue } = e {
                *match what {
                    "alpha" => take_alpha,
                    "red" => take_red,
                    "green" => take_green,
                    _ => take_blue,
                } = w.into();
            }
        })
    };
    let (full, yes, luminance, hue) = (word("red", "Full"), word("alpha", "yes"), word("alpha", "luminance"), word("blue", "hue"));
    t.refused(&mut document, vec![("red from Full", set(full)), ("alpha from yes", set(yes))]);
    t.taken(&mut document, "fx_shift_001.json", vec![("alpha from luminance", set(luminance)), ("blue from hue", set(hue))]);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_shift_002.json", 0),
        ("fx_shift_005.json", 0),
        ("fx_shift_009.json", 0),
        ("fx_shift_010.json", 0),
        ("fx_shift_015.json", 0),
        ("fx_shift_016.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // 001 holds the settings as they start, which change nothing and leave nothing for the card;
    // refused (017 to 020) leave nothing either.
    let none: Vec<u32> = std::iter::once(1).chain(17..=20).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("alpha from luminance", json!({"take_alpha": "luminance"})),
            ("red and blue swapped", json!({"take_red": "blue", "take_blue": "red"})),
            ("red from hue, green from saturation, blue from lightness, alpha full", json!({"take_alpha": "full", "take_red": "hue", "take_green": "saturation", "take_blue": "lightness"})),
        ],
    );

    t.heading("Pictures: in `verification/D-553 pictures/`");
    let dir = repo("verification/D-553 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = street(json!({}));
    t.row(
        "the settings as they start change nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );

    let (glitch, said) = street(json!({"take_alpha": "luminance"}));
    write("2_alpha_from_luminance.png", TOWN, &glitch);
    let fainter = pairs(&before, &glitch).all(|(_, a)| a[3] < 255 || a[..3].iter().all(|v| *v >= 250));
    let darkest_gone = pairs(&before, &glitch).filter(|(b, _)| b[..3].iter().all(|v| *v <= 8)).all(|(_, a)| a[3] <= 8);
    t.row(
        "2_alpha_from_luminance.png, alpha from luminance (the Colorful Glitch use): only white (or within 5 levels of it) stays solid, and the darkest parts (every channel 8 or below) keep at most 8 of 255; draws cleanly",
        &format!("{said:?}, only white solid: {fainter}, the darkest gone: {darkest_gone}"),
        said.is_empty() && fainter && darkest_gone && glitch != before,
    );

    let (swap, said) = street(json!({"take_red": "blue", "take_blue": "red"}));
    write("3_red_blue_swapped.png", TOWN, &swap);
    let swapped = pairs(&before, &swap)
        .filter(|(b, _)| b[3] == 255)
        .all(|(b, a)| a[0].abs_diff(b[2]) <= 1 && a[1].abs_diff(b[1]) <= 1 && a[2].abs_diff(b[0]) <= 1 && a[3] == 255);
    t.row(
        "3_red_blue_swapped.png, red from blue and blue from red: every solid pixel's red and blue traded, its green kept, within 1 level; draws cleanly",
        &format!("{said:?}, traded: {swapped}"),
        said.is_empty() && swapped && swap != before,
    );

    let (hues, said) = street(json!({"take_red": "hue", "take_green": "hue", "take_blue": "hue"}));
    write("4_hue_as_grey.png", TOWN, &hues);
    let grey = pairs(&before, &hues).all(|(_, a)| a[0] == a[1] && a[1] == a[2]);
    let greys_black = pairs(&before, &hues).filter(|(b, _)| b[3] == 255 && b[0] == b[1] && b[1] == b[2]).all(|(_, a)| a[..3] == [0, 0, 0]);
    t.row(
        "4_hue_as_grey.png, red, green and blue from hue: the street in greys, light where its colour is far round the wheel, and every grey of the street black; draws cleanly",
        &format!("{said:?}, all grey: {grey}, the street's greys black: {greys_black}"),
        said.is_empty() && grey && greys_black,
    );

    let (solid, said) = street(json!({"take_alpha": "full"}));
    write("5_alpha_full.png", TOWN, &solid);
    let all_solid = solid.chunks_exact(4).all(|p| p[3] == 255);
    let kept = pairs(&before, &solid).filter(|(b, _)| b[3] == 255).all(|(b, a)| b == a);
    t.row(
        "5_alpha_full.png, alpha full: every pixel solid, the street's solid pixels as they were; draws cleanly",
        &format!("{said:?}, every pixel solid: {all_solid}, solid pixels kept: {kept}"),
        said.is_empty() && all_solid && kept,
    );

    t.finish("D-553_shift_channels_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B433_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-433: a measurement, run deliberately with --release --ignored"]
fn b433_shift_channels_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B433_CPU").is_ok();
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
        ("Noise, then Shift Channels, alpha from luminance", Some(json!({"take_alpha": "luminance"}))),
        ("Noise, then Shift Channels, red from hue, green from saturation, blue from lightness, alpha from luminance", Some(json!({"take_alpha": "luminance", "take_red": "hue", "take_green": "saturation", "take_blue": "lightness"}))),
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
    let out = std::env::var("B433_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-433_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
