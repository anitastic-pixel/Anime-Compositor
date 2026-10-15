//! B-395: D-515, after CycoreFX's CC Radial ScaleWipe (Transition): a round hole opens at the
//! Center and pushes the picture out of its way (reversed, the outside closes in like an iris).
//! The CycoreFX manual says only "radial wipe where edge folds" and names the controls, with no
//! formula and no settings when added: the rule and those settings are ours, stated in
//! `tools/push_iris_reference.py`.
//!
//! Writes `verification/D-515_push_iris_table.md` and draws pictures into
//! `verification/D-515 pictures/`.
//!
//! Every expected pixel is `Fixtures/push_iris/expected_push_iris.json`, written by the tool
//! before this code existed and printed in document 25 as FX-PUSH-001 to 017. Tolerance 2e-5.
//! Nothing here is a snapshot of a run.

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

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=17 {
        let file = format!("fx_push_{n:03}.json");
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
                    matches!(e, Effect::PushIris { .. })
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
        let project = reference(|id| json!([fx(&format!("b395-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Push Iris {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| {
                    matches!(e, Effect::PushIris { .. })
                });
                t.row(
                    &format!("the reference shot, Push Iris {what} on three layers, frame {frame}, {}", quality.label()),
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

/// A Push Iris with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"completion": 0, "center": [50, 50], "reverse": "off"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.push_iris", "enabled": true, "parameters": all})
}

#[test]
fn b395_push_iris() {
    let mut t = Table::new(
        "push_iris",
        "# D-515: Push Iris\n\nB-395, after CycoreFX's CC Radial ScaleWipe (Transition): a \
         round hole opens at the Center and pushes the picture out of its way, its middle \
         stretched round the hole's edge; reversed, the outside closes in like an iris and the \
         picture is pulled toward the middle. The CycoreFX manual says only \"radial wipe where \
         edge folds\" and names the controls, so the rule and the settings when added are ours, \
         stated in `tools/push_iris_reference.py`. Every expected pixel is \
         `Fixtures/push_iris/expected_push_iris.json`, written by the tool before this code \
         existed and printed in document 25 as FX-PUSH-001 to 017. Tolerance 2e-5.\n",
    );

    t.heading("FX-PUSH-001 to 017 (document 25)");
    t.fixtures_numbered("expected_push_iris.json", 1..=17);

    t.heading("The file");
    let files: Vec<String> = (1..=17).map(|n| format!("fx_push_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_push_011.json");
    t.row(
        "fx_push_011.json is saved with its three settings, reversed, completion 65, Center [60, 40]",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 3
            && saved["reverse"] == "on"
            && saved["completion"] == json!(65)
            && saved["center"] == json!([60, 40]),
    );
    for (file, want) in [
        ("fx_push_012.json", "completion"),
        ("fx_push_013.json", "completion"),
        ("fx_push_014.json", "center"),
        ("fx_push_015.json", "reverse"),
        ("fx_push_016.json", "reverse"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming {want}"),
            &why,
            why.contains(want),
        );
    }
    t.shape_refused(
        "fx_push_001.json",
        "a Push Iris whose completion is a word",
        r##"{"completion": "half", "center": [50, 50], "reverse": "off"}"##,
    );
    t.shape_refused(
        "fx_push_001.json",
        "a Push Iris whose Center is one number",
        r##"{"completion": 0, "center": 50, "reverse": "off"}"##,
    );

    t.heading("On the card (D-515)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[1, 12, 13, 14, 15, 16, 17]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            (
                "completion 30 (as added otherwise)",
                json!({"completion": 30}),
                3,
            ),
            (
                "completion 50, reversed, Center [30, 60]",
                json!({"completion": 50, "reverse": "on", "center": [30, 60]}),
                3,
            ),
            (
                "completion 70, Center [120, 50]",
                json!({"completion": 70, "center": [120, 50]}),
                3,
            ),
            (
                "completion keyed 10 to 90",
                json!({"completion": {"base": 10, "keyframes": [{"frame": 0, "value": 10, "interp": "linear"}, {"frame": 239, "value": 90, "interp": "linear"}]}}),
                3,
            ),
        ],
    );

    t.heading("Pictures (verification/D-515 pictures)");
    let dir = repo("verification/D-506 pictures");
    let out = repo("verification/D-515 pictures");
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
    // name, settings, a pixel that must be clear and one that must be covered, the street pushed
    // or pulled into it (worked by the reference's rule at 480 by 270).
    for (name, p, gone, covered) in [
        ("1_hole", json!({"completion": 30}), (240, 135), (2, 2)),
        (
            "2_bigger_hole",
            json!({"completion": 60}),
            (240, 135),
            (2, 2),
        ),
        (
            "3_iris_closing",
            json!({"completion": 30, "reverse": "on"}),
            (2, 2),
            (240, 135),
        ),
        (
            "4_off_centre",
            json!({"completion": 40, "center": [20, 70]}),
            (96, 189),
            (470, 2),
        ),
        ("5_done", json!({"completion": 100}), (2, 2), (240, 135)),
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
        let (g, k) = (at(&bytes, gone), at(&bytes, covered));
        let done = name == "5_done";
        t.row(
            &format!(
                "{name}.png: drawn without a warning, the pixel at {gone:?} clear, {}",
                if done {
                    "every pixel clear".to_string()
                } else {
                    format!("the one at {covered:?} covered")
                }
            ),
            &format!(
                "{} pixels changed; at {gone:?} {g:?}; at {covered:?} {k:?}; warnings {said:?}",
                distance(&bytes, &street).1
            ),
            said.is_empty()
                && g[3] == 0
                && if done {
                    bytes.chunks_exact(4).all(|p| p[3] == 0)
                } else {
                    k[3] == 255
                },
        );
    }
    t.finish("D-515_push_iris_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B395_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-395: a measurement, run deliberately with --release --ignored"]
fn b395_push_iris_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B395_CPU").is_ok();
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
            "Noise, then Push Iris, completion 50 (as added otherwise)",
            Some(json!({"completion": 50})),
        ),
        (
            "Noise, then Push Iris, completion 50, reversed",
            Some(json!({"completion": 50, "reverse": "on"})),
        ),
        (
            "Noise, then Push Iris, completion 30, Center [20, 70]",
            Some(json!({"completion": 30, "center": [20, 70]})),
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
    let out = std::env::var("B395_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-395_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
