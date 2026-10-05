//! B-136: Polar Coordinates in the core, against D-201.
//!
//! Writes `verification/B-136_polar_coordinates_table.md`.
//!
//! Every expected pixel is `Fixtures/polar_coordinates/expected_polar_coordinates.json`, written
//! by `tools/polar_coordinates_reference.py` before this code existed and printed in document 25
//! as FX-POLAR-001 to 015. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws speed lines and a target and bends them into `verification/B-136 pictures/`,
//! three times enlarged, over a grey check where they are clear.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, set, Table};
use serde_json::Value as J;

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

fn polar(interpolation: f64, conversion: &str) -> Effect {
    Effect::PolarCoordinates { interpolation, conversion: conversion.to_string(), shape: "ellipse".to_string() }
}

const PLATE: (usize, usize) = (160, 100);
const LINE: [u8; 3] = [30, 26, 36];
/// The target's bands from the middle out, each a fifth of the way to the drawing's edge.
const BANDS: [[u8; 3]; 5] = [[255, 208, 64], [200, 40, 40], [244, 236, 216], [58, 111, 216], [30, 26, 36]];

/// Upright streaks, none in the top third, each widening to its foot, clear between: the speed
/// lines of a drawn action shot, sixteen samples a pixel so their edges are soft.
fn speed_lines() -> Vec<u8> {
    let (w, h) = (PLATE.0 as f64, PLATE.1 as f64);
    let mut seed = 7u32;
    let mut rnd = |lo: f64, hi: f64| {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        lo + (hi - lo) * (seed >> 8) as f64 / (1u32 << 24) as f64
    };
    let mut streaks = Vec::new();
    let mut x = 0.0;
    while x < w {
        streaks.push((x, rnd(0.34, 0.62) * h, rnd(0.5, 2.25)));
        x += rnd(3.5, 9.0);
    }
    let mut bytes = Vec::new();
    for y in 0..PLATE.1 {
        for x in 0..PLATE.0 {
            let inside = (0..16)
                .filter(|s| {
                    let (sx, sy) = (x as f64 + (s % 4) as f64 / 4.0 + 0.125, y as f64 + (s / 4) as f64 / 4.0 + 0.125);
                    streaks.iter().any(|&(cx, top, half)| sy > top && (sx - cx).abs() <= half * (sy - top) / (h - top))
                })
                .count();
            bytes.extend([LINE[0], LINE[1], LINE[2], (inside * 255 / 16) as u8]);
        }
    }
    bytes
}

/// Rings the drawing's shape, one band a fifth of the way out each, clear beyond its edge.
fn target() -> Vec<u8> {
    let (hw, hh) = (PLATE.0 as f64 / 2.0, PLATE.1 as f64 / 2.0);
    let mut bytes = Vec::new();
    for y in 0..PLATE.1 {
        for x in 0..PLATE.0 {
            let r = ((x as f64 + 0.5 - hw) / hw).hypot((y as f64 + 0.5 - hh) / hh);
            match BANDS.get((r * 5.0) as usize) {
                Some(c) => bytes.extend([c[0], c[1], c[2], 255]),
                None => bytes.extend([0, 0, 0, 0]),
            }
        }
    }
    bytes
}

/// `plate` as the composition's one layer, with `effects`, drawn, straight 8-bit; and the same
/// enlarged three times over a grey check where it is clear.
fn picture(dir: &Path, plate: &str, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/polar_coordinates/fx_polar_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from(plate);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(PLATE.0);
    comp["height"] = J::from(PLATE.1);
    let layer = &mut comp["layers"][0];
    let middle = serde_json::json!([PLATE.0 as f64 / 2.0, PLATE.1 as f64 / 2.0]);
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

/// Polar Coordinates effects, in order, as a layer's `effects`.
fn stack(steps: &[(f64, &str)]) -> J {
    J::Array(
        steps
            .iter()
            .enumerate()
            .map(|(i, (interpolation, conversion))| {
                serde_json::json!({
                    "instance_id": format!("fx-0-{i}"), "type_id": "core.polar_coordinates", "enabled": true,
                    "parameters": { "interpolation": interpolation, "conversion": conversion },
                })
            })
            .collect(),
    )
}

#[test]
fn b136_polar_coordinates() {
    let mut t = Table::new(
        "polar_coordinates",
        "# B-136: Polar Coordinates\n\nD-201, accepted on 2026-09-28 with the After Effects \
         picks (A11). Every expected pixel is \
         `Fixtures/polar_coordinates/expected_polar_coordinates.json`, written by \
         `tools/polar_coordinates_reference.py` before this code existed and printed in document \
         25 as FX-POLAR-001 to 015. The build's frame is compared sample by sample; the answer \
         is the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-POLAR-001 to 015 (document 25)");
    t.fixtures("expected_polar_coordinates.json");

    t.heading("How far it reaches");
    let got = polar(100.0, "rect_to_polar").bounds_expansion();
    t.row("it never grows the layer: it declares no growth", &got.to_string(), got == 0);
    let mut draft = polar(60.0, "polar_to_rect");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview keeps Interpolation, which is a share, not a distance",
        &format!("{draft:?}"),
        draft == polar(60.0, "polar_to_rect"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_polar_002.json",
        "fx_polar_003.json",
        "fx_polar_006.json",
        "fx_polar_007.json",
        "fx_polar_009.json",
        "fx_polar_011.json",
        "fx_polar_012.json",
        "fx_polar_013.json",
        "fx_polar_014.json",
        "fx_polar_015.json",
    ]);
    t.shape_refused("fx_polar_001.json", "no `conversion` at all", r#"{"interpolation": 100}"#);
    t.shape_refused("fx_polar_001.json", "no `interpolation` at all", r#"{"conversion": "rect_to_polar"}"#);
    t.shape_refused(
        "fx_polar_001.json",
        "a conversion that is a number",
        r#"{"interpolation": 100, "conversion": 1}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_polar_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("Interpolation 100.5", set(polar(100.5, "rect_to_polar"))),
            ("Interpolation -0.5", set(polar(-0.5, "rect_to_polar"))),
            ("conversion \"sideways\"", set(polar(100.0, "sideways"))),
            ("conversion \"Rect_To_Polar\", written in capitals", set(polar(100.0, "Rect_To_Polar"))),
            ("Interpolation keyed to 150", keys("interpolation", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_polar_001.json",
        vec![
            ("Interpolation 0,", set(polar(0.0, "rect_to_polar"))),
            ("Polar to Rect at 100,", set(polar(100.0, "polar_to_rect"))),
            ("Interpolation keyed from 0 to 100", keys("interpolation", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("Pictures: speed lines and a target, in `verification/B-136 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-136 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8], scale: usize| {
        png_out::write_rgba(&dir.join(name), w * scale, h * scale, OutputDepth::Eight, &[], bytes).unwrap()
    };
    write("speed_lines.png", &speed_lines(), 1);
    write("target.png", &target(), 1);
    let px = |bytes: &[u8], (x, y): (usize, usize)| {
        let i = (y * w + x) * 4;
        [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
    };
    let near = |a: [u8; 4], b: [u8; 4]| a.iter().zip(b).all(|(&u, v)| u.abs_diff(v) <= 1);
    // How far each pixel's middle is from the drawing's, its edge at 1.
    let r = |(x, y): (usize, usize)| {
        let (hw, hh) = (w as f64 / 2.0, h as f64 / 2.0);
        ((x as f64 + 0.5 - hw) / hw).hypot((y as f64 + 0.5 - hh) / hh)
    };
    let every = || (0..h).flat_map(move |y| (0..w).map(move |x| (x, y)));

    let (lines, big, said) = picture(&dir, "speed_lines.png", stack(&[]));
    write("before_lines.png", &big, 3);
    t.row("before_lines.png, the speed lines with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());

    let (focus, big, said) = picture(&dir, "speed_lines.png", stack(&[(100.0, "rect_to_polar")]));
    write("focus_lines.png", &big, 3);
    let ring: Vec<[u8; 4]> = every().filter(|&p| (0.9..0.97).contains(&r(p))).map(|p| px(&focus, p)).collect();
    let (dark, clear) = (ring.iter().filter(|p| p[3] > 128).count(), ring.iter().filter(|p| p[3] == 0).count());
    t.row(
        "focus_lines.png, Rect to Polar at 100: the streaks turned into spokes pointing in to the \
         middle, which the lines never reach, as their top third was clear, and the corners clear, \
         as the drawing's edge is its outer ring; of the pixels near the edge a tenth at least \
         dark and a tenth at least clear; draws cleanly",
        &format!(
            "{said:?}, middle {:?}, corners {:?}, near the edge {dark} dark and {clear} clear of {}",
            px(&focus, (80, 50)),
            [(0, 0), (159, 0), (0, 99), (159, 99)].map(|p| px(&focus, p)[3]),
            ring.len()
        ),
        said.is_empty()
            && px(&focus, (80, 50))[3] == 0
            && [(0, 0), (159, 0), (0, 99), (159, 99)].iter().all(|&p| px(&focus, p)[3] == 0)
            && dark * 10 >= ring.len()
            && clear * 10 >= ring.len(),
    );

    let (half, big, said) = picture(&dir, "speed_lines.png", stack(&[(50.0, "rect_to_polar")]));
    write("half_way.png", &big, 3);
    t.row(
        "half_way.png, Rect to Polar at 50: between the two, neither the speed lines nor the focus \
         lines; draws cleanly",
        &format!("{said:?}, same as the speed lines: {}, as the focus lines: {}", half == lines, half == focus),
        said.is_empty() && half != lines && half != focus,
    );

    let (back, big, said) = picture(&dir, "speed_lines.png", stack(&[(100.0, "rect_to_polar"), (100.0, "polar_to_rect")]));
    write("back_again.png", &big, 3);
    let kept = every().filter(|&p| px(&back, p)[3].abs_diff(px(&lines, p)[3]) <= 32).count();
    t.row(
        "back_again.png, Rect to Polar then a second effect, Polar to Rect: the speed lines back, \
         a little softer from being drawn twice; at least three pixels in four within 32 of 255 \
         of their covering in before_lines.png; draws cleanly",
        &format!("{said:?}, {kept} of {}", w * h),
        said.is_empty() && kept * 4 >= w * h * 3,
    );

    let (_, big, said) = picture(&dir, "target.png", stack(&[]));
    write("before_target.png", &big, 3);
    t.row("before_target.png, the target with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());

    let (open, big, said) = picture(&dir, "target.png", stack(&[(100.0, "polar_to_rect")]));
    write("target_unrolled.png", &big, 3);
    let rows = [9, 29, 49, 69, 89];
    let flat = rows.iter().zip(BANDS).all(|(&y, c)| (0..w).all(|x| near(px(&open, (x, y)), [c[0], c[1], c[2], 255])));
    t.row(
        "target_unrolled.png, Polar to Rect at 100: the rings unrolled into stripes, the middle at \
         the top and the outer ring at the foot; rows 9, 29, 49, 69 and 89, one in each band, \
         each its band's colour all the way across; draws cleanly",
        &format!("{said:?}, {:?}, every one flat: {flat}", rows.map(|y| px(&open, (80, y)))),
        said.is_empty() && flat,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_polar_001.json", 0), ("fx_polar_006.json", 2), ("fx_polar_009.json", 0), ("fx_polar_011.json", 0)]);

    t.finish("B-136_polar_coordinates_table.md");
}
