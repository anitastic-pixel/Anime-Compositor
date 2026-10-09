//! B-259: D-380, Channel Blur in After Effects' Blurriness, as Gaussian Blur's Units (D-321).
//!
//! Writes `verification/D-380_channel_blur_units_table.md`.
//!
//! Every expected pixel is `Fixtures/channel_blur/expected_channel_blur_blurriness.json` or
//! `expected_channel_blur.json`, written by `tools/channel_blur_reference.py`, printed in
//! document 25 as FX-CHBLUR-001 to 024. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws the street each way, into `verification/D-380 pictures/`.

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

fn channel_blur(rgba: [f64; 4], units: &str) -> Effect {
    Effect::ChannelBlur {
        red_blurriness: rgba[0],
        green_blurriness: rgba[1],
        blue_blurriness: rgba[2],
        alpha_blurriness: rgba[3],
        edges: "transparent".into(),
        dimensions: "both".into(),
        units: units.into(),
    }
}

/// The street (`town.png` in `dir`) with `effect`, 8-bit straight.
fn picture(dir: &Path, effect: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J =
        serde_json::from_str(&fs::read_to_string(repo("Fixtures/channel_blur/fx_chblur_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
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

fn chblur_json(rgba: [f64; 4], units: &str) -> J {
    let mut parameters = json!({
        "red_blurriness": rgba[0], "green_blurriness": rgba[1], "blue_blurriness": rgba[2], "alpha_blurriness": rgba[3], "dimensions": "both",
    });
    if !units.is_empty() {
        parameters["units"] = J::from(units);
    }
    json!({ "instance_id": "fx-0-0", "type_id": "core.channel_blur", "enabled": true, "parameters": parameters })
}

#[test]
fn b259_channel_blur_units() {
    let mut t = Table::new(
        "channel_blur",
        "# D-380: Channel Blur in After Effects' Blurriness\n\nThe owner chose option (c) on \
         2026-10-09 (\"c\"): Channel Blur gains Units exactly as Gaussian Blur did under D-321. \
         Blurriness (After Effects), which a new Channel Blur takes, reads each of Red, Green, \
         Blue and Alpha Blurriness as After Effects does, sigma 0.3 times it, its kernel reaching \
         6.5 sigmas; Sigma (older projects), what a file without units means, is document 21's \
         rule as before, so a saved project draws as it did. Every expected pixel is \
         `Fixtures/channel_blur/expected_channel_blur_blurriness.json` (FX-CHBLUR-015 to 024) or \
         `expected_channel_blur.json` (FX-CHBLUR-001 to 014, unchanged), written by \
         `tools/channel_blur_reference.py` before the build had the units. Tolerance 2e-5.\n",
    );

    t.heading("FX-CHBLUR-001 to 014, the old file, unchanged (document 25)");
    t.fixtures("expected_channel_blur.json");
    t.heading("FX-CHBLUR-015 to 024, Blurriness (document 25)");
    t.fixtures("expected_channel_blur_blurriness.json");

    t.heading("The file");
    t.round_trips(&[
        "fx_chblur_001.json",
        "fx_chblur_015.json",
        "fx_chblur_016.json",
        "fx_chblur_021.json",
        "fx_chblur_022.json",
        "fx_chblur_023.json",
        "fx_chblur_024.json",
    ]);
    let saved = t.saved_parameters("fx_chblur_001.json");
    t.row("fx_chblur_001.json, a Channel Blur with no units as before D-380, is saved without units", &saved.to_string(), saved.get("units").is_none());
    let saved = t.saved_parameters("fx_chblur_015.json");
    t.row("A Channel Blur in Blurriness is saved as units: blurriness", &saved.to_string(), saved["units"] == "blurriness");

    t.heading("Commands");
    let mut document = t.load("fx_chblur_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("units \"pixels\"", set(channel_blur([3.0, 0.0, 0.0, 0.0], "pixels"))),
            ("units \"Blurriness\", written with a capital", set(channel_blur([3.0, 0.0, 0.0, 0.0], "Blurriness"))),
        ],
    );
    t.taken(&mut document, "fx_chblur_001.json", vec![("FX-CHBLUR-001 set to Blurriness 6/0/15/3,", set(channel_blur([6.0, 0.0, 15.0, 3.0], "blurriness")))]);
    let blurriness: J =
        serde_json::from_str(&fs::read_to_string(t.root.join("expected_channel_blur_blurriness.json")).unwrap()).unwrap();
    let mut document = t.load("fx_chblur_001.json").document;
    document.apply(set(channel_blur([6.0, 0.0, 15.0, 3.0], "blurriness"))).unwrap();
    let d = effect_table::largest_difference(&t.render(&document, 0, 64), &blurriness["cases"]["FX-CHBLUR-016"]["frames"]["0"]);
    t.row("FX-CHBLUR-001 set to Blurriness 6/0/15/3 by the command draws FX-CHBLUR-016's frame", &format!("largest difference {d:.1e}"), d <= 2e-5);
    let mut document = t.load("fx_chblur_015.json").document;
    document.apply(keys("red_blurriness", &[(0, &[0.0]), (4, &[20.0])])).unwrap();
    let d = effect_table::largest_difference(&t.render(&document, 2, 64), &blurriness["cases"]["FX-CHBLUR-022"]["frames"]["2"]);
    t.row("FX-CHBLUR-015 with Red keyed 0 to 20 by the command draws FX-CHBLUR-022's frame 2", &format!("largest difference {d:.1e}"), d <= 2e-5);

    t.heading("The preview card");
    match Gpu::new() {
        Err(why) => t.row("The card draws Blurriness", &format!("NOT RUN: no usable card ({why})"), false),
        Ok(mut gpu) => {
            for (name, frame) in [
                ("fx_chblur_001.json", 0),
                ("fx_chblur_015.json", 0),
                ("fx_chblur_016.json", 0),
                ("fx_chblur_017.json", 0),
                ("fx_chblur_018.json", 0),
                ("fx_chblur_019.json", 0),
                ("fx_chblur_020.json", 0),
                ("fx_chblur_022.json", 2),
            ] {
                let project = t.load(name).document.project().clone();
                let comp = Id::new(effect_table::MAIN);
                for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
                    let mut cache = CelCache::viewer();
                    let mut log = FrameLog::new(3);
                    let cpu = preview::preview_frame_cached(&project, &comp, frame, &t.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache).unwrap().to_srgb8_straight();
                    let mut log = FrameLog::new(3);
                    let (card, ..) = preview::preview_frame_srgb8(&project, &comp, frame, &t.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).unwrap();
                    let on_cpu = log.finish().iter().any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str());
                    let worst = cpu.iter().zip(&card).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
                    t.row(
                        &format!("{name} frame {frame} at {quality:?}: drawn on the card, within 1 level of 255 of the CPU"),
                        &format!("{}, largest difference {worst} of 255", if on_cpu { "CPU" } else { "card" }),
                        !on_cpu && worst <= 1,
                    );
                }
            }
        }
    }

    t.heading("Pictures: the street, in `verification/D-380 pictures/`");
    let dir = repo("verification/D-380 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let mut drawn = Vec::new();
    for (name, effect, what) in [
        ("built_1_sigma_red_20.png", chblur_json([20.0, 0.0, 0.0, 0.0], ""), "Red Blurriness 20 in an old file (Sigma): red smeared far past the houses"),
        ("built_2_blurriness_red_20.png", chblur_json([20.0, 0.0, 0.0, 0.0], "blurriness"), "Red Blurriness 20 in Blurriness, as After Effects reads it (sigma 6): a red fringe"),
        ("built_3_sigma_all_10.png", chblur_json([10.0; 4], ""), "all four at 10 in Sigma: very soft"),
        ("built_4_blurriness_all_10.png", chblur_json([10.0; 4], "blurriness"), "all four at 10 in Blurriness: softened, the windows still there"),
        (
            "built_5_gaussian_blurriness_10.png",
            json!({ "instance_id": "fx-0-0", "type_id": "core.gaussian_blur", "enabled": true, "parameters": { "sigma_px": 10.0, "units": "blurriness" } }),
            "Gaussian Blur at Blurriness 10, for comparison",
        ),
    ] {
        let (bytes, said) = picture(&dir, effect);
        write(name, &bytes);
        t.row(&format!("{name}, {what}; draws cleanly"), &format!("{said:?}"), said.is_empty());
        drawn.push(bytes);
    }
    let worst = |a: &[u8], b: &[u8]| a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
    let d = worst(&drawn[3], &drawn[4]);
    t.row("Channel Blur with all four at Blurriness 10 is Gaussian Blur at Blurriness 10, within 1 level", &format!("largest difference {d} of 255"), d <= 1);
    let d = worst(&drawn[1], &drawn[0]);
    t.row("Blurriness 20 is not Sigma 20 (the old file keeps the old look)", &format!("largest difference {d} of 255"), d > 20);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_chblur_016.json", 0), ("fx_chblur_017.json", 0), ("fx_chblur_019.json", 0), ("fx_chblur_022.json", 2)]);

    t.finish("D-380_channel_blur_units_table.md");
}
