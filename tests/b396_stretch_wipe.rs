//! B-396: D-516, after CycoreFX's CC Scale Wipe (Transition): everything past a line through
//! the Center stretched along the Direction, so moving the Center stretches the layer off the
//! screen or onto it. The CycoreFX manual names the controls but publishes no formula and no
//! settings when added: the rule and those settings are ours, stated in
//! `tools/stretch_wipe_reference.py`.
//!
//! Writes `verification/D-516_stretch_wipe_table.md` and draws pictures into
//! `verification/D-516 pictures/`.
//!
//! Every expected pixel is `Fixtures/stretch_wipe/expected_stretch_wipe.json`, written by the
//! tool before this code existed and printed in document 25 as FX-STRETCH-001 to 017. Tolerance
//! 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, median, on_card, reference, repo, time_passes, Table, MAIN, TOWN,
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

/// The reference shot (1920 by 1080) with `stack` on its first three layers.

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=17 {
        let file = format!("fx_stretch_{n:03}.json");
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
                    matches!(e, Effect::StretchWipe { .. })
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
        let project = reference(|id| json!([fx(&format!("b396-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Stretch Wipe {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| {
                    matches!(e, Effect::StretchWipe { .. })
                });
                t.row(
                    &format!("the reference shot, Stretch Wipe {what} on three layers, frame {frame}, {}", quality.label()),
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

/// The pixel at (x, y) of a picture at the street's size.
fn at(bytes: &[u8], p: (usize, usize)) -> [u8; 4] {
    let i = (p.1 * TOWN.0 + p.0) * 4;
    [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
}

/// A Stretch Wipe with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"scale": 100, "center": [50, 50], "direction": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.stretch_wipe", "enabled": true, "parameters": all})
}

#[test]
fn b396_stretch_wipe() {
    let mut t = Table::new(
        "stretch_wipe",
        "# D-516: Stretch Wipe\n\nB-396, after CycoreFX's CC Scale Wipe (Transition): \
         everything past a line through the Center stretched Scale times along the Direction, \
         so the line's own colours draw out in streaks and moving the Center stretches the layer \
         off the screen or onto it. The CycoreFX manual names the controls but publishes no \
         formula and no settings when added, so the rule and those settings are ours, stated in \
         `tools/stretch_wipe_reference.py`. Every expected pixel is \
         `Fixtures/stretch_wipe/expected_stretch_wipe.json`, written by the tool before this code \
         existed and printed in document 25 as FX-STRETCH-001 to 017. Tolerance 2e-5.\n",
    );

    t.heading("FX-STRETCH-001 to 017 (document 25)");
    t.fixtures_numbered("expected_stretch_wipe.json", 1..=17);

    t.heading("The file");
    let files: Vec<String> = (1..=17)
        .map(|n| format!("fx_stretch_{n:03}.json"))
        .collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_stretch_012.json");
    t.row(
        "fx_stretch_012.json is saved with its three settings, direction -135, Center [60, 40], scale 175",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 3
            && saved["direction"] == json!(-135)
            && saved["center"] == json!([60, 40])
            && saved["scale"] == json!(175),
    );
    for (file, want) in [
        ("fx_stretch_013.json", "scale"),
        ("fx_stretch_014.json", "scale"),
        ("fx_stretch_015.json", "center"),
        ("fx_stretch_016.json", "direction"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming {want}"),
            &why,
            why.contains(want),
        );
    }
    t.shape_refused(
        "fx_stretch_001.json",
        "a Stretch Wipe whose scale is a word",
        r##"{"scale": "big", "center": [50, 50], "direction": 0}"##,
    );
    t.shape_refused(
        "fx_stretch_001.json",
        "a Stretch Wipe whose Center is one number",
        r##"{"scale": 100, "center": 50, "direction": 0}"##,
    );

    t.heading("On the card (D-516)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[1, 13, 14, 15, 16, 17]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("scale 300 (as added otherwise)", json!({"scale": 300}), 3),
            (
                "scale 1000, direction 90",
                json!({"scale": 1000, "direction": 90}),
                3,
            ),
            (
                "scale 175, direction -135, Center [60, 40]",
                json!({"scale": 175, "direction": -135, "center": [60, 40]}),
                3,
            ),
            (
                "scale keyed 150 to 5000",
                json!({"scale": {"base": 150, "keyframes": [{"frame": 0, "value": 150, "interp": "linear"}, {"frame": 239, "value": 5000, "interp": "linear"}]}}),
                3,
            ),
        ],
    );

    t.heading("Pictures (verification/D-516 pictures)");
    let dir = repo("verification/D-506 pictures");
    let out = repo("verification/D-516 pictures");
    fs::create_dir_all(&out).unwrap();
    let (street, _) = picture(&dir, "1_left.png", (480, 270), json!([]));
    png_out::write_rgba(
        &out.join("0_the_street.png"),
        480,
        270,
        OutputDepth::Eight,
        &[],
        &street,
    )
    .unwrap();
    // name, settings, a pixel behind the line that must be as the street's.
    for (name, p, kept) in [
        ("1_stretched", json!({"scale": 300}), (100, 135)),
        ("2_down", json!({"scale": 200, "direction": 90}), (240, 2)),
        (
            "3_turned",
            json!({"scale": 175, "direction": -135, "center": [60, 40]}),
            (470, 268),
        ),
        (
            "4_off_the_left",
            json!({"scale": 150, "center": [-20, 50]}),
            (0, 0),
        ),
    ] {
        let (bytes, said) = picture(&dir, "1_left.png", (480, 270), json!([fx("pic", &p)]));
        png_out::write_rgba(
            &out.join(format!("{name}.png")),
            480,
            270,
            OutputDepth::Eight,
            &[],
            &bytes,
        )
        .unwrap();
        let k = at(&bytes, kept);
        let off = name == "4_off_the_left";
        let changed_px = distance(&bytes, &street).1;
        t.row(
            &format!(
                "{name}.png: drawn without a warning, {}",
                if off {
                    "every pixel stretched".to_string()
                } else {
                    format!("pixels changed, the one at {kept:?} behind the line as the street's")
                }
            ),
            &format!("{changed_px} pixels changed; at {kept:?} {k:?}; warnings {said:?}"),
            said.is_empty() && changed_px > 0 && (off || k == at(&street, kept)),
        );
    }
    // At 100000 the right half is the middle line drawn out: every pixel there is read within a
    // quarter pixel right of the line, so it lies between the street's two middle columns.
    let (bytes, said) = picture(
        &dir,
        "1_left.png",
        (480, 270),
        json!([fx("pic", &json!({"scale": 100000}))]),
    );
    png_out::write_rgba(
        &out.join("5_streaks.png"),
        480,
        270,
        OutputDepth::Eight,
        &[],
        &bytes,
    )
    .unwrap();
    let rows_same = (0..270).all(|y| {
        (241..480).all(|x| {
            let (l, r) = (at(&street, (239, y)), at(&street, (240, y)));
            at(&bytes, (x, y)).iter().enumerate().all(|(c, v)| {
                v.saturating_add(1) >= l[c].min(r[c]) && *v <= l[c].max(r[c]).saturating_add(1)
            })
        })
    });
    t.row(
        "5_streaks.png: drawn without a warning, scale 100000: along every row the right half lies between the street's two middle columns, within 1 level",
        &format!("{} pixels changed; every row from x 241 to 479 between columns 239 and 240: {rows_same}; warnings {said:?}", distance(&bytes, &street).1),
        said.is_empty() && rows_same,
    );
    t.finish("D-516_stretch_wipe_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B396_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-396: a measurement, run deliberately with --release --ignored"]
fn b396_stretch_wipe_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B396_CPU").is_ok();
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
        (
            "Noise, then Stretch Wipe, scale 300 (as added otherwise)",
            Some(json!({"scale": 300})),
        ),
        (
            "Noise, then Stretch Wipe, scale 100000, direction 90",
            Some(json!({"scale": 100000, "direction": 90})),
        ),
        (
            "Noise, then Stretch Wipe, scale 175, direction -135, Center [60, 40]",
            Some(json!({"scale": 175, "direction": -135, "center": [60, 40]})),
        ),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![
                json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}}),
            ];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}u"), p));
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
    let out = std::env::var("B396_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-396_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
