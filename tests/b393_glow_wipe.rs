//! B-393: D-513, after CycoreFX's CC Light Wipe (Transition): the layer wiped away from the
//! Center behind a band of light, two walls, a circle or a square. The CycoreFX manual names the
//! controls but publishes no formula and no settings when added (beyond Color from Source off):
//! the rule and those settings are ours, stated in `tools/glow_wipe_reference.py`.
//!
//! Writes `verification/D-513_glow_wipe_table.md` and draws pictures into
//! `verification/D-513 pictures/`.
//!
//! Every expected pixel is `Fixtures/glow_wipe/expected_glow_wipe.json`, written by the tool
//! before this code existed and printed in document 25 as FX-GLOW-001 to 028. Tolerance 2e-5.
//! Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, changed, distance, median, on_card, reference, repo, time_passes, Table, MAIN, TOWN};
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
    for n in 1..=28 {
        let file = format!("fx_glow_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::GlowWipe { .. }));
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
        let project = reference(|id| json!([fx(&format!("b393-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Glow Wipe {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::GlowWipe { .. }));
                t.row(
                    &format!("the reference shot, Glow Wipe {what} on three layers, frame {frame}, {}", quality.label()),
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


/// The pixel at (x, y) of a picture at the street's size.
fn at(bytes: &[u8], p: (usize, usize)) -> [u8; 4] {
    let i = (p.1 * TOWN.0 + p.0) * 4;
    [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
}


/// A Glow Wipe with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"completion": 0, "center": [50, 50], "intensity": 100, "direction": 0,
        "shape": "radial", "color_from_source": "off", "color": "#ffffff", "reverse": "off"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.glow_wipe", "enabled": true, "parameters": all})
}

#[test]
fn b393_glow_wipe() {
    let mut t = Table::new(
        "glow_wipe",
        "# D-513: Glow Wipe\n\nB-393, after CycoreFX's CC Light Wipe (Transition): the layer \
         wiped away from the Center behind a band of light. The CycoreFX manual names the \
         controls but publishes no formula and no settings when added (beyond Color from Source \
         off), so the rule and those settings are ours, stated in \
         `tools/glow_wipe_reference.py`: an edge moving out from the Center by the shape's \
         distance (two walls, a circle or a square, the walls and the square turned by the \
         Direction), the layer gone inside it (outside it, reversed), and a band of light a tenth \
         of the range wide around it, the Color or the layer's own colour, added on top. Every \
         expected pixel is `Fixtures/glow_wipe/expected_glow_wipe.json`, written by the tool \
         before this code existed and printed in document 25 as FX-GLOW-001 to 028. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-GLOW-001 to 028 (document 25)");
    t.fixtures_numbered("expected_glow_wipe.json", 1..=28);

    t.heading("The file");
    let files: Vec<String> = (1..=28).map(|n| format!("fx_glow_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_glow_016.json");
    t.row(
        "fx_glow_016.json is saved with its eight settings, doors, Color from Source on, reversed, direction -135, Center [60, 40], completion 55",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 8
            && saved["shape"] == "doors"
            && saved["color_from_source"] == "on"
            && saved["reverse"] == "on"
            && saved["direction"] == json!(-135)
            && saved["center"] == json!([60, 40])
            && saved["completion"] == json!(55),
    );
    for (file, want) in [
        ("fx_glow_017.json", "completion"),
        ("fx_glow_018.json", "completion"),
        ("fx_glow_019.json", "center"),
        ("fx_glow_020.json", "intensity"),
        ("fx_glow_021.json", "intensity"),
        ("fx_glow_022.json", "direction"),
        ("fx_glow_023.json", "shape"),
        ("fx_glow_024.json", "shape"),
        ("fx_glow_025.json", "color from source"),
        ("fx_glow_026.json", "colour"),
        ("fx_glow_027.json", "reverse"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    t.shape_refused(
        "fx_glow_001.json",
        "a Glow Wipe whose intensity is a word",
        r##"{"completion": 0, "center": [50, 50], "intensity": "bright", "direction": 0, "shape": "radial", "color_from_source": "off", "color": "#ffffff", "reverse": "off"}"##,
    );
    t.shape_refused(
        "fx_glow_001.json",
        "a Glow Wipe whose Center is one number",
        r##"{"completion": 0, "center": 50, "intensity": 100, "direction": 0, "shape": "radial", "color_from_source": "off", "color": "#ffffff", "reverse": "off"}"##,
    );

    t.heading("On the card (D-513)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[1, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("completion 50 (as added otherwise)", json!({"completion": 50}), 3),
            ("completion 40, doors, direction 30, #ff8000 at 250", json!({"completion": 40, "shape": "doors", "direction": 30, "color": "#ff8000", "intensity": 250}), 3),
            ("completion 30, rectangular, reversed, Color from Source, Center [20, 70]", json!({"completion": 30, "shape": "rectangular", "reverse": "on", "color_from_source": "on", "center": [20, 70]}), 3),
            ("completion keyed 10 to 90", json!({"completion": {"base": 10, "keyframes": [{"frame": 0, "value": 10, "interp": "linear"}, {"frame": 239, "value": 90, "interp": "linear"}]}}), 3),
        ],
    );

    t.heading("Pictures (verification/D-513 pictures)");
    let dir = repo("verification/D-506 pictures");
    let out = repo("verification/D-513 pictures");
    fs::create_dir_all(&out).unwrap();
    let (street, _) = picture(&dir, "1_left.png", (480, 270), json!([]));
    png_out::write_rgba(&out.join("0_the_street.png"), 480, 270, OutputDepth::Eight, &[], &street).unwrap();
    // name, settings, a pixel that must be gone and one that must be as the street's, far from
    // the band of light.
    for (name, p, gone, kept) in [
        ("1_circle", json!({"completion": 40}), (240, 135), (2, 2)),
        ("2_doors", json!({"completion": 40, "shape": "doors"}), (240, 135), (2, 135)),
        ("3_square_turned", json!({"completion": 40, "shape": "rectangular", "direction": 30}), (240, 135), (2, 2)),
        ("5_reversed_own_colour", json!({"completion": 40, "reverse": "on", "color_from_source": "on"}), (2, 2), (240, 135)),
        ("6_done", json!({"completion": 100}), (2, 2), (240, 135)),
    ] {
        let (bytes, said) = picture(&dir, "1_left.png", (480, 270), json!([fx("pic", &p)]));
        png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
        let (g, k) = (at(&bytes, gone), at(&bytes, kept));
        let done = name == "6_done";
        t.row(
            &format!("{name}.png: drawn without a warning, the pixel at {gone:?} gone, {}", if done { "every pixel gone".to_string() } else { format!("the one at {kept:?} as the street's") }),
            &format!("{} pixels changed; at {gone:?} {g:?}; at {kept:?} {k:?}; warnings {said:?}", distance(&bytes, &street).1),
            said.is_empty() && g[3] == 0 && if done { bytes.chunks_exact(4).all(|p| p[3] == 0) } else { k == at(&street, kept) },
        );
    }
    // The circle's edge at completion 40 is 104.6 pixels from the middle (0.38 of the corner's
    // 275.4), the band 27.5 wide: (344, 135) is just inside it, cleared and lit almost fully.
    let p = json!({"completion": 40, "color": "#ff8000", "intensity": 300});
    let (bytes, said) = picture(&dir, "1_left.png", (480, 270), json!([fx("pic", &p)]));
    png_out::write_rgba(&out.join("4_orange_light.png"), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
    let (lit, gone) = (at(&bytes, (344, 135)), at(&bytes, (240, 135)));
    t.row(
        "4_orange_light.png: drawn without a warning, the middle gone, the pixel at (344, 135) on the edge lit orange: covered, red full, green less, no blue",
        &format!("{} pixels changed; at (344, 135) {lit:?}; at (240, 135) {gone:?}; warnings {said:?}", distance(&bytes, &street).1),
        said.is_empty() && gone[3] == 0 && lit[3] == 255 && lit[0] == 255 && lit[1] < 255 && lit[1] > 0 && lit[2] == 0,
    );
    t.finish("D-513_glow_wipe_table.md");
}


/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B393_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-393: a measurement, run deliberately with --release --ignored"]
fn b393_glow_wipe_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B393_CPU").is_ok();
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
        ("Noise, then Glow Wipe, completion 50 (as added otherwise)", Some(json!({"completion": 50}))),
        ("Noise, then Glow Wipe, completion 50, doors, direction 30, Color from Source", Some(json!({"completion": 50, "shape": "doors", "direction": 30, "color_from_source": "on"}))),
        ("Noise, then Glow Wipe, completion 50, rectangular, reversed, #ff8000 at 300", Some(json!({"completion": 50, "shape": "rectangular", "reverse": "on", "color": "#ff8000", "intensity": 300}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}u"), p));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
        let (first, times) = time_passes(&mut gpu, cpu, passes, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B393_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-393_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
