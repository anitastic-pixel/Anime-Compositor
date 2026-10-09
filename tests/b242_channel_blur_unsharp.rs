//! B-242: D-363, fixtures for Channel Blur (D-312/D-313) and Unsharp Mask's threshold (D-317).
//!
//! Writes `verification/D-363_channel_blur_unsharp_table.md`.
//!
//! Every expected pixel is `Fixtures/channel_blur/expected_channel_blur.json`, written by
//! `tools/channel_blur_reference.py`, or `Fixtures/sharpen/expected_sharpen_threshold.json`,
//! written by `tools/unsharp_threshold_reference.py`, printed in document 25 as FX-CHBLUR-001
//! to 014 and FX-SHARPEN-019 to 030. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a street with each, into `verification/D-363 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, set, town, Table, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

fn channel_blur(rgba: [f64; 4], edges: &str, dimensions: &str) -> Effect {
    Effect::ChannelBlur {
        red_blurriness: rgba[0],
        green_blurriness: rgba[1],
        blue_blurriness: rgba[2],
        alpha_blurriness: rgba[3],
        edges: edges.into(),
        dimensions: dimensions.into(),
        units: "sigma".into(),
    }
}

fn sharpen(amount: f64, radius: f64, threshold: f64) -> Effect {
    Effect::Sharpen { amount, radius, threshold }
}

/// The preview of each file, on the card and on the CPU, within 1 level of 255.
fn preview_rows(t: &mut Table, files: &[&str]) {
    let mut gpu = match Gpu::new() {
        Err(why) => return t.row("The preview draws these files", &format!("NOT RUN: no usable card ({why})"), false),
        Ok(gpu) => gpu,
    };
    for name in files {
        let project = t.load(name).document.project().clone();
        let comp = Id::new(effect_table::MAIN);
        let mut cache = CelCache::viewer();
        let mut log = FrameLog::new(3);
        let cpu = preview::preview_frame_cached(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache).unwrap().to_srgb8_straight();
        let mut log = FrameLog::new(3);
        let (card, ..) = preview::preview_frame_srgb8(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).unwrap();
        let on_cpu = log.finish().iter().any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str());
        let worst = cpu.iter().zip(&card).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
        t.row(
            &format!("{name}: the preview draws the same picture as the CPU, within 1 level of 255"),
            &format!("{}, largest difference {worst} of 255", if on_cpu { "fell back to the CPU" } else { "no fallback" }),
            worst <= 1 && !on_cpu,
        );
    }
}

/// The street with a fine grain of up to 6 levels laid over it, for the threshold pictures.
fn grainy_town() -> Vec<u8> {
    let mut bytes = town();
    for (i, p) in bytes.chunks_mut(4).enumerate() {
        let n = ((i * 2654435761) >> 7) % 13;
        for c in &mut p[..3] {
            *c = (*c as i32 + n as i32 - 6).clamp(0, 255) as u8;
        }
    }
    bytes
}

/// The street (from `town.png` or `grain.png` in `dir`) with `effect`, 8-bit straight.
fn picture(dir: &Path, asset: &str, effect: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J =
        serde_json::from_str(&fs::read_to_string(repo("Fixtures/channel_blur/fx_chblur_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = json!([effect]);
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

#[test]
fn b242_channel_blur_unsharp() {
    let mut t = Table::new(
        "channel_blur",
        "# D-363: Channel Blur and Unsharp Mask, checked against fixtures\n\nChannel Blur \
         (D-312/D-313) and Unsharp Mask's Threshold (D-317) had hand-worked tests only. After \
         Effects' Channel Blur has Red, Green, Blue and Alpha Blurriness, Repeat Edge Pixels and \
         Blur Dimensions, all built; its Unsharp Mask has Amount, Radius and Threshold, all \
         built. Every expected pixel is `Fixtures/channel_blur/expected_channel_blur.json` \
         (`tools/channel_blur_reference.py`) or `Fixtures/sharpen/expected_sharpen_threshold.json` \
         (`tools/unsharp_threshold_reference.py`), independent double-precision workings, \
         printed in document 25 as FX-CHBLUR-001 to 014 and FX-SHARPEN-019 to 030. Tolerance \
         2e-5.\n",
    );

    // --- Channel Blur ---------------------------------------------------------------------
    t.heading("FX-CHBLUR-001 to 014 (document 25)");
    t.fixtures("expected_channel_blur.json");

    t.heading("Channel Blur: the file");
    t.round_trips(&["fx_chblur_001.json", "fx_chblur_004.json", "fx_chblur_005.json", "fx_chblur_007.json", "fx_chblur_010.json", "fx_chblur_013.json"]);
    let saved = t.saved_parameters("fx_chblur_007.json");
    t.row(
        "Saved with its four blurrinesses, edges and dimensions",
        &saved.to_string(),
        saved["red_blurriness"] == 4 && saved["alpha_blurriness"] == 2 && saved["edges"] == "repeat" && saved["dimensions"] == "vertical",
    );

    t.heading("Channel Blur: commands");
    let mut document = t.load("fx_chblur_008.json").document;
    t.refused(
        &mut document,
        vec![
            ("red 501", set(channel_blur([501.0, 0.0, 0.0, 0.0], "transparent", "both"))),
            ("alpha -1", set(channel_blur([0.0, 0.0, 0.0, -1.0], "transparent", "both"))),
            ("Repeat Edge Pixels written \"Repeat\"", set(channel_blur([0.0; 4], "Repeat", "both"))),
            ("Blur Dimensions \"diagonal\"", set(channel_blur([0.0; 4], "transparent", "diagonal"))),
        ],
    );
    t.taken(
        &mut document,
        "fx_chblur_008.json",
        vec![("FX-CHBLUR-008 set to red 2, blue 5, alpha 1,", set(channel_blur([2.0, 0.0, 5.0, 1.0], "transparent", "both")))],
    );
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_channel_blur.json")).unwrap()).unwrap();
    document.apply(set(channel_blur([2.0, 0.0, 5.0, 1.0], "transparent", "both"))).unwrap();
    let d = effect_table::largest_difference(&t.render(&document, 0, 64), &expected["cases"]["FX-CHBLUR-004"]["frames"]["0"]);
    t.row("FX-CHBLUR-008 set to red 2, blue 5, alpha 1 by the command draws FX-CHBLUR-004's frame", &format!("largest difference {d:.1e}"), d <= 2e-5);
    let mut document = t.load("fx_chblur_008.json").document;
    document.apply(keys("red_blurriness", &[(0, &[0.0]), (4, &[6.0])])).unwrap();
    let d = effect_table::largest_difference(&t.render(&document, 2, 64), &expected["cases"]["FX-CHBLUR-010"]["frames"]["2"]);
    t.row("FX-CHBLUR-008 with red keyed 0 to 6 over frames 0 to 4 by the command draws FX-CHBLUR-010's frame 2", &format!("largest difference {d:.1e}"), d <= 2e-5);

    t.heading("Channel Blur: the preview");
    preview_rows(&mut t, &["fx_chblur_001.json", "fx_chblur_004.json", "fx_chblur_005.json", "fx_chblur_006.json", "fx_chblur_007.json", "fx_chblur_009.json"]);

    t.heading("Channel Blur: the frame does not depend on how it is cut up");
    t.tiles(&[("fx_chblur_004.json", 0), ("fx_chblur_007.json", 0)]);

    // --- Unsharp Mask's threshold -----------------------------------------------------------
    t.root = repo("Fixtures/sharpen");
    t.heading("FX-SHARPEN-019 to 030 (document 25)");
    t.fixtures("expected_sharpen_threshold.json");

    t.heading("Unsharp Mask: the file");
    t.round_trips(&["fx_sharpen_019.json", "fx_sharpen_022.json", "fx_sharpen_026.json", "fx_sharpen_028.json", "fx_sharpen_030.json"]);
    let saved = t.saved_parameters("fx_sharpen_022.json");
    t.row("Saved with its amount, radius and threshold", &saved.to_string(), saved["amount"] == 200 && saved["radius"] == 1 && saved["threshold"] == 110);

    t.heading("Unsharp Mask: commands");
    let mut document = t.load("fx_sharpen_019.json").document;
    t.refused(
        &mut document,
        vec![("threshold 256", set(sharpen(100.0, 1.0, 256.0))), ("threshold -1", set(sharpen(100.0, 1.0, -1.0)))],
    );
    t.taken(&mut document, "fx_sharpen_019.json", vec![("FX-SHARPEN-019 set to threshold 30,", set(sharpen(100.0, 1.0, 30.0)))]);
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_sharpen_threshold.json")).unwrap()).unwrap();
    document.apply(set(sharpen(100.0, 1.0, 30.0))).unwrap();
    let d = effect_table::largest_difference(&t.render(&document, 0, 64), &expected["cases"]["FX-SHARPEN-021"]["frames"]["0"]);
    t.row("FX-SHARPEN-019 set to threshold 30 by the command draws FX-SHARPEN-021's frame", &format!("largest difference {d:.1e}"), d <= 2e-5);

    t.heading("Unsharp Mask: the preview (a threshold keeps it on the processor, D-317)");
    preview_rows(&mut t, &["fx_sharpen_019.json", "fx_sharpen_020.json", "fx_sharpen_025.json", "fx_sharpen_027.json"]);

    t.heading("Unsharp Mask: the frame does not depend on how it is cut up");
    t.tiles(&[("fx_sharpen_020.json", 0), ("fx_sharpen_027.json", 0)]);

    // --- Pictures ---------------------------------------------------------------------------
    t.heading("Pictures: a street, in `verification/D-363 pictures/`");
    let dir = repo("verification/D-363 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    write("grain.png", &grainy_town());
    let chblur = |r: f64, g: f64, b: f64, a: f64| {
        json!({ "instance_id": "fx-0-0", "type_id": "core.channel_blur", "enabled": true,
                "parameters": { "red_blurriness": r, "green_blurriness": g, "blue_blurriness": b, "alpha_blurriness": a, "edges": "repeat", "dimensions": "both" } })
    };
    let unsharp = |threshold: f64| {
        json!({ "instance_id": "fx-0-0", "type_id": "core.sharpen", "enabled": true,
                "parameters": { "amount": 300, "radius": 2, "threshold": threshold } })
    };
    let mut drawn = Vec::new();
    for (name, asset, effect, what) in [
        ("1_channel_blur_red_6.png", "town.png", chblur(6.0, 0.0, 0.0, 0.0), "Channel Blur, red 6 only, repeat edges: red fringes, green and blue sharp"),
        ("2_channel_blur_blue_6.png", "town.png", chblur(0.0, 0.0, 6.0, 0.0), "Channel Blur, blue 6 only: blue fringes"),
        ("3_channel_blur_all_6.png", "town.png", chblur(6.0, 6.0, 6.0, 6.0), "Channel Blur, all four 6: a plain blur"),
        ("4_unsharp_threshold_0.png", "grain.png", unsharp(0.0), "the grainy street, Unsharp Mask amount 300, radius 2, threshold 0: the grain crisped into noise"),
        ("5_unsharp_threshold_16.png", "grain.png", unsharp(16.0), "the same with threshold 16: the grain left as it was, the windows and road markings crisped"),
    ] {
        let (bytes, said) = picture(&dir, asset, effect);
        write(name, &bytes);
        t.row(&format!("{name}, {what}; draws cleanly"), &format!("{said:?}"), said.is_empty());
        drawn.push(bytes);
    }
    // In the sky (row 20), where only the grain varies: threshold 16 leaves it as it was.
    let grain = grainy_town();
    let sky = |bytes: &[u8]| (0..w).map(|x| bytes[(20 * w + x) * 4].abs_diff(grain[(20 * w + x) * 4])).max().unwrap();
    t.row(
        "In the sky, threshold 0 pushes the grain apart; threshold 16 leaves it within 1 level",
        &format!("largest change along row 20: threshold 0 {} of 255, threshold 16 {}", sky(&drawn[3]), sky(&drawn[4])),
        sky(&drawn[3]) > 6 && sky(&drawn[4]) <= 1,
    );

    t.finish("D-363_channel_blur_unsharp_table.md");
}
