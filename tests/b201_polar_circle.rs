//! B-201: D-320, Polar Coordinates' shape, the ellipse of D-201 or After Effects' circle.
//!
//! Writes `verification/D-320_polar_circle_table.md`.
//!
//! Every expected pixel is `Fixtures/polar_coordinates/expected_polar_circle.json`, written by
//! `tools/polar_circle_reference.py` before this code existed and printed in document 25 as
//! FX-POLAR-016 to 022. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also bends a wide plate of speed lines and a band both ways into
//! `verification/D-320 pictures/`, three times enlarged, over a grey check where they are clear.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{set, Table};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

fn polar(conversion: &str, shape: &str) -> Effect {
    Effect::PolarCoordinates { interpolation: 100.0, conversion: conversion.to_string(), shape: shape.to_string() }
}

/// A 16:9 plate, as tutorial 1's.
const PLATE: (usize, usize) = (192, 108);
const LINE: [u8; 3] = [30, 26, 36];
const BAND: [u8; 3] = [58, 111, 216];

/// Upright streaks in the lower half, three pixels wide every twelve, and across them a blue band
/// from 60% to 70% of the way down, which Rect to Polar turns into a ring.
fn plate() -> Vec<u8> {
    let (w, h) = PLATE;
    let mut bytes = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let band = (h * 6 / 10..h * 7 / 10).contains(&y);
            let streak = y >= h / 2 && x % 12 < 3;
            bytes.extend(match (band, streak) {
                (true, _) => [BAND[0], BAND[1], BAND[2], 255],
                (false, true) => [LINE[0], LINE[1], LINE[2], 255],
                _ => [0, 0, 0, 0],
            });
        }
    }
    bytes
}

fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/polar_coordinates/fx_polar_016.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("plate.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(PLATE.0);
    comp["height"] = J::from(PLATE.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([PLATE.0 as f64 / 2.0, PLATE.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    let bytes = frame.to_srgb8_straight();
    let big: Vec<u8> = (0..PLATE.1 * 3)
        .flat_map(|y| (0..PLATE.0 * 3).map(move |x| (x, y)))
        .flat_map(|(x, y)| {
            let i = (y / 3 * PLATE.0 + x / 3) * 4;
            let check = if (x / 12 + y / 12) % 2 == 0 { 96.0 } else { 128.0 };
            let a = bytes[i + 3] as f64 / 255.0;
            let over = |c: u8| (c as f64 * a + check * (1.0 - a)).round() as u8;
            [over(bytes[i]), over(bytes[i + 1]), over(bytes[i + 2]), 255]
        })
        .collect();
    (bytes, big, said)
}

fn bent(shape: Option<&str>) -> J {
    let mut parameters = json!({ "interpolation": 100, "conversion": "rect_to_polar" });
    if let Some(shape) = shape {
        parameters["shape"] = J::from(shape);
    }
    json!([{ "instance_id": "fx-0-0", "type_id": "core.polar_coordinates", "enabled": true, "parameters": parameters }])
}

#[test]
fn b201_polar_circle() {
    let mut t = Table::new(
        "polar_coordinates",
        "# D-320: Polar Coordinates, ellipse or circle\n\nFrom D-308, approved by the owner on \
         2026-10-04 after P-26's tutorial 1. Polar Coordinates has a Shape: Ellipse, D-201's \
         rule, which bends the layer into the ellipse touching its sides, or Circle, After \
         Effects' look, round the middle with half the shorter side as its radius. A file \
         without a shape is the ellipse and draws as before; a new one is a circle. Every \
         expected pixel is `Fixtures/polar_coordinates/expected_polar_circle.json`, written by \
         `tools/polar_circle_reference.py` before the build had the circle, printed in document \
         25 as FX-POLAR-016 to 022. Tolerance 2e-5.\n",
    );

    t.heading("FX-POLAR-016 to 022 (document 25)");
    t.fixtures("expected_polar_circle.json");

    t.heading("The file");
    t.round_trips(&["fx_polar_016.json", "fx_polar_017.json", "fx_polar_019.json", "fx_polar_021.json", "fx_polar_022.json"]);
    for file in ["fx_polar_001.json", "fx_polar_003.json", "fx_polar_011.json"] {
        let saved = t.saved_parameters(file);
        t.row(
            &format!("{file}, from before D-320, has no shape and is saved without one"),
            &saved.to_string(),
            saved.get("shape").is_none(),
        );
    }
    let saved = t.saved_parameters("fx_polar_016.json");
    t.row("A circle is saved as shape: circle", &saved.to_string(), saved["shape"] == "circle");

    t.heading("Commands");
    let mut document = t.load("fx_polar_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("shape \"square\"", set(polar("rect_to_polar", "square"))),
            ("shape \"Circle\", written in capitals", set(polar("rect_to_polar", "Circle"))),
        ],
    );
    t.taken(
        &mut document,
        "fx_polar_001.json",
        vec![("FX-POLAR-001 set to a circle,", set(polar("rect_to_polar", "circle")))],
    );
    let mut document = t.load("fx_polar_001.json").document;
    document.apply(set(polar("rect_to_polar", "circle"))).unwrap();
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_polar_circle.json")).unwrap()).unwrap();
    let d = effect_table::largest_difference(&t.render(&document, 0, 64), &expected["cases"]["FX-POLAR-016"]["frames"]["0"]);
    t.row("FX-POLAR-001 set to a circle by the command draws FX-POLAR-016's frame", &format!("largest difference {d:.1e}"), d <= 2e-5);

    t.heading("The preview card");
    match Gpu::new() {
        Err(why) => t.row("The card draws the circle", &format!("NOT RUN: no usable card ({why})"), false),
        Ok(mut gpu) => {
            for name in ["fx_polar_016.json", "fx_polar_017.json", "fx_polar_019.json", "fx_polar_020.json", "fx_polar_001.json", "fx_polar_003.json"] {
                let project = t.load(name).document.project().clone();
                let comp = Id::new("comp-main");
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let cpu = preview::preview_frame_cached(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache).unwrap().to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (card, ..) = preview::preview_frame_srgb8(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).unwrap();
                let on_cpu = log.finish().iter().any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str());
                let worst = cpu.iter().zip(&card).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
                t.row(
                    &format!("{name}: drawn on the card, within 1 level of 255 of the CPU"),
                    &format!("{}, largest difference {worst} of 255", if on_cpu { "CPU" } else { "card" }),
                    !on_cpu && worst <= 1,
                );
            }
        }
    }

    t.heading("Pictures: a wide plate bent both ways, in `verification/D-320 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/D-320 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8], scale: usize| {
        png_out::write_rgba(&dir.join(name), w * scale, h * scale, OutputDepth::Eight, &[], bytes).unwrap()
    };
    write("plate.png", &plate(), 1);
    let (_, big, said) = picture(&dir, json!([]));
    write("1_plate.png", &big, 3);
    t.row("1_plate.png, the plate with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());
    // How far out from the middle the band's ring starts, straight right and straight down.
    let blue = |bytes: &[u8], x: usize, y: usize| {
        let i = (y * w + x) * 4;
        bytes[i + 3] > 128 && bytes[i + 2] as i32 > bytes[i] as i32 + 60
    };
    let reach = |bytes: &[u8]| {
        let right = (w / 2..w).find(|&x| blue(bytes, x, h / 2)).map(|x| x - w / 2);
        let down = (h / 2..h).find(|&y| blue(bytes, w / 2, y)).map(|y| y - h / 2);
        (right, down)
    };
    for (name, shape, what) in [
        ("2_ellipse_old_file.png", None, "a file from before D-320, no shape: the ellipse as before, the ring stretched to the frame's sides, so wider than tall"),
        ("3_circle_new.png", Some("circle"), "shape circle, as a new Polar Coordinates is: the ring round, as far out across as down, and the frame's ends beyond the circle clear"),
    ] {
        let (bytes, big, said) = picture(&dir, bent(shape));
        write(name, &big, 3);
        let (right, down) = reach(&bytes);
        let ends_clear = (0..h).all(|y| bytes[(y * w + 2) * 4 + 3] == 0 && bytes[(y * w + w - 3) * 4 + 3] == 0);
        let ok = match (shape, right, down) {
            (None, Some(r), Some(d)) => r * 10 >= d * 16 && !ends_clear,
            (Some(_), Some(r), Some(d)) => r.abs_diff(d) <= 1 && ends_clear,
            _ => false,
        };
        t.row(
            &format!("{name}, {what}; draws cleanly"),
            &format!(
                "{said:?}, the ring starts {} pixels right of the middle and {} down; the ends clear: {ends_clear}",
                right.map_or("no".to_string(), |r| r.to_string()),
                down.map_or("no".to_string(), |d| d.to_string())
            ),
            said.is_empty() && ok,
        );
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_polar_016.json", 0), ("fx_polar_017.json", 0), ("fx_polar_019.json", 0)]);

    t.finish("D-320_polar_circle_table.md");
}
