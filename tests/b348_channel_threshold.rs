//! B-348: D-468, CycoreFX's CC Threshold under `core.channel_threshold`: Threshold, Channel, Invert
//! and Blend With Original, a second name over Threshold's rule (as D-383 and D-394).
//!
//! Writes `verification/D-468_channel_threshold_table.md` and draws pictures into
//! `verification/D-468 pictures/`.
//!
//! Every expected pixel is `Fixtures/channel_threshold/expected_channel_threshold.json`, written by
//! `tools/channel_threshold_reference.py` before this code existed and printed in document 25 as
//! FX-CHTHRESH-001 to 023. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// A Channel Threshold with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"threshold": 128, "channel": "luminance", "invert": "off", "blend_with_original": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.channel_threshold", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=23 {
        let file = format!("fx_chthresh_{n:03}.json");
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
                    matches!(e, Effect::ChannelThreshold { .. })
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
        let project = reference(|id| json!([fx(&format!("b348-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Channel Threshold {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| {
                    matches!(e, Effect::ChannelThreshold { .. })
                });
                t.row(
                    &format!("the reference shot, Channel Threshold {what} on three layers, frame {frame}, {}", quality.label()),
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

/// Covered pixels of `before` and `after` side by side.
fn pairs<'a>(before: &'a [u8], after: &'a [u8]) -> impl Iterator<Item = (&'a [u8], &'a [u8])> {
    before
        .chunks_exact(4)
        .zip(after.chunks_exact(4))
        .filter(|(b, _)| b[3] > 0)
}

#[test]
fn b348_channel_threshold() {
    let mut t = Table::new(
        "channel_threshold",
        "# D-468: Channel Threshold\n\nB-348: `core.channel_threshold` takes CycoreFX's CC Threshold \
         controls, Threshold, Channel, Invert and Blend With Original, under our own name: a second \
         name over Threshold's rule (as D-383 and D-394), Threshold's decision on the luminance, on \
         each of red, green and blue, on the saturation or on the covering, turned over by Invert \
         and blended back in linear light. Every expected pixel is \
         `Fixtures/channel_threshold/expected_channel_threshold.json`, written by \
         `tools/channel_threshold_reference.py` before this code existed and printed in document 25 \
         as FX-CHTHRESH-001 to 023; Threshold's FX-THRESH-001 to 018 rerun unchanged (019 is in dispute, D-164). Tolerance \
         2e-5.\n",
    );

    t.heading("FX-CHTHRESH-001 to 023 (document 25)");
    t.fixtures_numbered("expected_channel_threshold.json", 1..=23);

    t.heading("Threshold, the rule under the second name: its fixtures, unchanged");
    let root = t.root.clone();
    t.root = repo("Fixtures/threshold");
    t.fixtures_numbered("expected_threshold.json", 1..=18);
    t.root = root;

    t.heading("The file");
    let files: Vec<String> = (1..=23)
        .map(|n| format!("fx_chthresh_{n:03}.json"))
        .collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_chthresh_013.json");
    t.row(
        "fx_chthresh_013.json is saved with its four settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 4
            && saved["threshold"] == 128.0
            && saved["channel"] == "rgb"
            && saved["invert"] == "on"
            && saved["blend_with_original"] == 25.0,
    );
    for (file, want) in [
        ("fx_chthresh_018.json", "threshold"),
        ("fx_chthresh_019.json", "threshold"),
        ("fx_chthresh_020.json", "channel"),
        ("fx_chthresh_021.json", "invert"),
        ("fx_chthresh_022.json", "blend_with_original"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        let says = why.contains(want) || why.contains(&want.replace('_', " "));
        t.row(
            &format!("{file} is refused in a sentence naming {want}"),
            &why,
            says,
        );
    }
    t.shape_refused("fx_chthresh_001.json", "a Channel Threshold whose threshold is a word", r#"{"threshold": "half", "channel": "luminance", "invert": "off", "blend_with_original": 0}"#);
    t.shape_refused(
        "fx_chthresh_001.json",
        "a Channel Threshold without its channel",
        r#"{"threshold": 128, "invert": "off", "blend_with_original": 0}"#,
    );
    t.shape_refused(
        "fx_chthresh_001.json",
        "a Channel Threshold whose invert is a number",
        r#"{"threshold": 128, "channel": "luminance", "invert": 1, "blend_with_original": 0}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_chthresh_001.json").document;
    let over = changed(&t, "fx_chthresh_001.json", |e| {
        if let Effect::ChannelThreshold { threshold, .. } = e {
            *threshold = 256.0
        }
    });
    let bad = changed(&t, "fx_chthresh_001.json", |e| {
        if let Effect::ChannelThreshold { channel, .. } = e {
            *channel = "red".into()
        }
    });
    t.refused(
        &mut document,
        vec![
            ("threshold 256", set(over)),
            ("channel red", set(bad)),
            (
                "threshold keyed to 300",
                keys("threshold", &[(0, &[128.0]), (4, &[300.0])]),
            ),
            (
                "blend with original keyed to 150",
                keys("blend_with_original", &[(0, &[0.0]), (4, &[150.0])]),
            ),
        ],
    );
    let rgb = changed(&t, "fx_chthresh_001.json", |e| {
        if let Effect::ChannelThreshold {
            channel, invert, ..
        } = e
        {
            *channel = "rgb".into();
            *invert = "on".into();
        }
    });
    t.taken(
        &mut document,
        "fx_chthresh_001.json",
        vec![
            ("RGB, turned over", set(rgb)),
            (
                "threshold keyed from 0 to 255",
                keys("threshold", &[(0, &[0.0]), (4, &[255.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_chthresh_001.json", 0),
        ("fx_chthresh_004.json", 0),
        ("fx_chthresh_006.json", 0),
        ("fx_chthresh_008.json", 0),
        ("fx_chthresh_013.json", 0),
        ("fx_chthresh_015.json", 2),
        ("fx_chthresh_017.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Blend 100 (012) and refused (018 to 023) leave nothing for the card.
    let none: Vec<u32> = std::iter::once(12).chain(18..=23).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            (
                "RGB at 200, turned over, blend 30",
                json!({"threshold": 200, "channel": "rgb", "invert": "on", "blend_with_original": 30}),
                3,
            ),
            (
                "saturation at 100",
                json!({"threshold": 100, "channel": "saturation"}),
                3,
            ),
        ],
    );

    t.heading("Pictures: in `verification/D-468 pictures/`");
    let dir = repo("verification/D-468 pictures");
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
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row(
        "1_before.png, the street with no effect; draws cleanly",
        &format!("{said:?}"),
        said.is_empty(),
    );
    let (same, said) = street(json!({"blend_with_original": 100}));
    t.row(
        "blend with original 100 changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );

    let solid = |b: &[u8]| b[3] == 255;
    let full = |v: u8| v == 0 || v == 255;
    let (plain, said) = street(json!({}));
    write("2_as_added.png", TOWN, &plain);
    let bw = pairs(&before, &plain)
        .filter(|(b, _)| solid(b))
        .all(|(_, a)| full(a[0]) && a[0] == a[1] && a[1] == a[2]);
    t.row(
        "2_as_added.png, as added (luminance at 128): every solid pixel pure black or pure white; draws cleanly",
        &format!("{said:?}, all black or white: {bw}"),
        said.is_empty() && bw && plain != before,
    );

    let (rgb, said) = street(json!({"channel": "rgb"}));
    write("3_rgb.png", TOWN, &rgb);
    let eight = pairs(&before, &rgb)
        .filter(|(b, _)| solid(b))
        .all(|(_, a)| a[..3].iter().all(|v| full(*v)));
    let coloured = pairs(&before, &rgb)
        .filter(|(b, _)| solid(b))
        .any(|(_, a)| a[0] != a[1] || a[1] != a[2]);
    t.row(
        "3_rgb.png, RGB at 128: every channel of every solid pixel full or nothing, some pixels pure colours; draws cleanly",
        &format!("{said:?}, every channel full or nothing: {eight}, some coloured: {coloured}"),
        said.is_empty() && eight && coloured,
    );

    let (sat, said) = street(json!({"channel": "saturation"}));
    write("4_saturation.png", TOWN, &sat);
    let grey = |b: &[u8]| b[0] == b[1] && b[1] == b[2];
    let greys_black = pairs(&before, &sat)
        .filter(|(b, _)| solid(b) && grey(b))
        .all(|(_, a)| a[..3] == [0, 0, 0]);
    let bw = pairs(&before, &sat)
        .filter(|(b, _)| solid(b))
        .all(|(_, a)| full(a[0]) && a[0] == a[1] && a[1] == a[2]);
    t.row(
        "4_saturation.png, saturation at 128: black or white, every grey of the street black; draws cleanly",
        &format!("{said:?}, all black or white: {bw}, greys black: {greys_black}"),
        said.is_empty() && bw && greys_black,
    );

    let (over, said) = street(json!({"invert": "on"}));
    write("5_turned_over.png", TOWN, &over);
    let flipped = pairs(&plain, &over)
        .filter(|(b, _)| solid(b))
        .all(|(p, q)| p[..3].iter().zip(&q[..3]).all(|(x, y)| *x == 255 - *y));
    t.row(
        "5_turned_over.png, as added with Invert on: 2 turned over, white where 2 is black and black where it is white; draws cleanly",
        &format!("{said:?}, every solid pixel turned over: {flipped}"),
        said.is_empty() && flipped,
    );

    let (half, said) = street(json!({"blend_with_original": 50}));
    write("6_blend_50.png", TOWN, &half);
    t.row(
        "6_blend_50.png, as added at blend 50: nearer the street than 2 is; draws cleanly",
        &format!(
            "{said:?}, largest difference from the street {} against 2's {}",
            distance(&half, &before).0,
            distance(&plain, &before).0
        ),
        said.is_empty() && distance(&half, &before).0 < distance(&plain, &before).0,
    );

    // The second name: Channel Threshold on luminance with nothing turned over or blended back
    // draws Threshold's street byte for byte.
    for level in [50, 128, 200] {
        let th = json!({"instance_id": "fx-0-0", "type_id": "core.threshold", "enabled": true, "parameters": {"level": level}});
        let (a, said0) = street(json!({"threshold": level}));
        let (b, said1) = picture(&dir, "town.png", TOWN, json!([th]));
        t.row(
            &format!("Channel Threshold on luminance at {level} against Threshold at {level}: the same street byte for byte; both draw cleanly"),
            &format!("{said0:?} {said1:?}, {} pixels differ", distance(&a, &b).1),
            said0.is_empty() && said1.is_empty() && a == b && a != before,
        );
    }

    t.finish("D-468_channel_threshold_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B348_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-348: a measurement, run deliberately with --release --ignored"]
fn b348_channel_threshold_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B348_CPU").is_ok();
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
            "Noise, then Channel Threshold as added (luminance at 128)",
            Some(json!({})),
        ),
        (
            "Noise, then Channel Threshold, RGB at 200 turned over at blend 30",
            Some(
                json!({"threshold": 200, "channel": "rgb", "invert": "on", "blend_with_original": 30}),
            ),
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
    let out = std::env::var("B348_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-348_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
