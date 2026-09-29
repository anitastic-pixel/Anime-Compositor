//! B-148: Bevel Alpha and Bevel Edges in the core, against D-213.
//!
//! Writes `verification/B-148_bevel_table.md`, and pictures in `verification/B-148 pictures/`.
//!
//! Every expected pixel is `Fixtures/bevel/expected_bevel.json`, written by
//! `tools/bevel_reference.py` before this code existed and printed in document 25 as
//! FX-BEVEL-001 to 026. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// Bevel Alpha with its edge thickness, light angle, light colour and light intensity.
fn alpha(thickness: f64, angle: f64, color: &str, intensity: f64) -> Effect {
    Effect::BevelAlpha {
        edge_thickness: thickness,
        light_angle: angle,
        light_color: color.to_string(),
        light_intensity: intensity,
    }
}

/// Bevel Edges with its edge thickness, light angle, light colour and light intensity.
fn edges(thickness: f64, angle: f64, color: &str, intensity: f64) -> Effect {
    Effect::BevelEdges {
        edge_thickness: thickness,
        light_angle: angle,
        light_color: color.to_string(),
        light_intensity: intensity,
    }
}

const PLATE: (usize, usize) = (160, 100);

/// A made-up badge: a red disc with a yellow diamond in it, on nothing.
fn badge() -> Vec<u8> {
    let (w, h) = PLATE;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (fx, fy) = (x as f64 + 0.5, y as f64 + 0.5);
            let c = if (fx - 80.0).hypot(fy - 50.0) >= 38.0 {
                [0, 0, 0, 0]
            } else if (fx - 80.0).abs() + (fy - 50.0).abs() < 16.0 {
                [250, 210, 40, 255]
            } else {
                [200, 40, 50, 255]
            };
            bytes.extend(c);
        }
    }
    bytes
}

/// A made-up panel filling the whole layer: blue, with a pale stripe across the middle.
fn panel() -> Vec<u8> {
    let (w, h) = PLATE;
    (0..w * h)
        .flat_map(|i| if (44..56).contains(&(i / w)) { [230, 230, 240, 255] } else { [60, 110, 200, 255] })
        .collect()
}

/// `art` as the composition's one layer, with `effects`, drawn, straight 8-bit; and the same
/// enlarged three times.
fn picture(dir: &Path, art: &str, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/bevel/fx_bevel_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from(art);
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
            bytes[i..i + 4].to_vec()
        })
        .collect();
    (bytes, big, said)
}

/// One bevel of `type_id` as a layer's `effects`, from its settings as the file writes them.
fn one(type_id: &str, parameters: J) -> J {
    let thickness = if type_id == "core.bevel_alpha" { 2.0 } else { 0.1 };
    let mut p = serde_json::json!({
        "edge_thickness": thickness, "light_angle": -60, "light_color": "#ffffff", "light_intensity": 0.4,
    });
    for (k, v) in parameters.as_object().unwrap() {
        p[k] = v.clone();
    }
    serde_json::json!([{ "instance_id": "fx-0-0", "type_id": type_id, "enabled": true, "parameters": p }])
}

#[test]
fn b148_bevel() {
    let mut t = Table::new(
        "bevel",
        "# B-148: Bevel Alpha and Bevel Edges\n\nD-213, accepted on 2026-09-28 with the After Effects \
         picks (B11): After Effects' own Bevel Alpha and Bevel Edges, as two effects; CC Glass is \
         left out. Every expected pixel is `Fixtures/bevel/expected_bevel.json`, written by \
         `tools/bevel_reference.py` before this code existed and printed in document 25 as \
         FX-BEVEL-001 to 026. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-BEVEL-001 to 026 (document 25)");
    t.fixtures_numbered("expected_bevel.json", 1..=26);

    t.heading("How far they reach");
    for (name, effect) in [("Bevel Alpha", alpha(8.0, -60.0, "#ffffff", 1.0)), ("Bevel Edges", edges(0.5, -60.0, "#ffffff", 1.0))] {
        let got = effect.bounds_expansion();
        t.row(&format!("{name} grows the drawing's bounds by nothing"), &got.to_string(), got == 0);
    }
    let mut draft = alpha(6.0, 45.0, "#2040a0", 0.7);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves Bevel Alpha's edge thickness, in pixels, and nothing else",
        &format!("{draft:?}"),
        draft == alpha(3.0, 45.0, "#2040a0", 0.7),
    );
    let mut draft = edges(0.25, 45.0, "#2040a0", 0.7);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview leaves Bevel Edges alone: its thickness is a share of the layer",
        &format!("{draft:?}"),
        draft == edges(0.25, 45.0, "#2040a0", 0.7),
    );

    t.heading("The file");
    t.round_trips(&(1..=26).map(|n| format!("fx_bevel_{n:03}.json")).collect::<Vec<_>>().iter().map(String::as_str).collect::<Vec<_>>());
    t.shape_refused(
        "fx_bevel_001.json",
        "no `light_color` at all",
        r#"{"edge_thickness": 2, "light_angle": -60, "light_intensity": 0.4}"#,
    );
    t.shape_refused(
        "fx_bevel_011.json",
        "a light intensity that is a list",
        r##"{"edge_thickness": 0.1, "light_angle": -60, "light_color": "#ffffff", "light_intensity": [0.4]}"##,
    );

    t.heading("Commands, Bevel Alpha");
    let mut document = t.load("fx_bevel_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("edge thickness -1", set(alpha(-1.0, -60.0, "#ffffff", 0.4))),
            ("edge thickness 201", set(alpha(201.0, -60.0, "#ffffff", 0.4))),
            ("light angle 3601", set(alpha(2.0, 3601.0, "#ffffff", 0.4))),
            ("light angle -3601", set(alpha(2.0, -3601.0, "#ffffff", 0.4))),
            ("light intensity 1.1", set(alpha(2.0, -60.0, "#ffffff", 1.1))),
            ("light colour \"white\"", set(alpha(2.0, -60.0, "white", 0.4))),
            ("edge thickness keyed to 300", keys("edge_thickness", &[(0, &[2.0]), (4, &[300.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_bevel_001.json",
        vec![
            ("every number at its bottom,", set(alpha(0.0, -3600.0, "#000000", 0.0))),
            ("every number at its top,", set(alpha(200.0, 3600.0, "#2040a0", 1.0))),
            ("light angle keyed from 0 to 360", keys("light_angle", &[(0, &[0.0]), (4, &[360.0])])),
            ("light intensity keyed from 0 to 1", keys("light_intensity", &[(0, &[0.0]), (4, &[1.0])])),
        ],
    );

    t.heading("Commands, Bevel Edges");
    let mut document = t.load("fx_bevel_011.json").document;
    t.refused(
        &mut document,
        vec![
            ("edge thickness 0.6", set(edges(0.6, -60.0, "#ffffff", 0.4))),
            ("edge thickness -0.01", set(edges(-0.01, -60.0, "#ffffff", 0.4))),
            ("light intensity -0.1", set(edges(0.1, -60.0, "#ffffff", -0.1))),
            ("light colour \"#fff\"", set(edges(0.1, -60.0, "#fff", 0.4))),
            ("edge thickness keyed to 0.8", keys("edge_thickness", &[(0, &[0.1]), (4, &[0.8])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_bevel_011.json",
        vec![
            ("edge thickness 0.5, its top,", set(edges(0.5, 3600.0, "#ffd070", 1.0))),
            ("edge thickness keyed from 0 to 0.5", keys("edge_thickness", &[(0, &[0.0]), (4, &[0.5])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_bevel_001.json", 0),
        ("fx_bevel_006.json", 0),
        ("fx_bevel_008.json", 2),
        ("fx_bevel_012.json", 0),
        ("fx_bevel_015.json", 0),
        ("fx_bevel_017.json", 2),
    ]);

    t.heading("Pictures: a made-up badge and panel, in `verification/B-148 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-148 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8], scale: usize| {
        png_out::write_rgba(&dir.join(name), w * scale, h * scale, OutputDepth::Eight, &[], bytes).unwrap()
    };
    let (badge_art, panel_art) = (badge(), panel());
    write("badge.png", &badge_art, 1);
    write("panel.png", &panel_art, 1);
    let px = |bytes: &[u8], (x, y): (usize, usize)| {
        let i = (y * w + x) * 4;
        [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
    };
    let sum = |p: [u8; 4]| p[..3].iter().map(|&c| c as i32).sum::<i32>();
    // How much lighter (above 0) or darker (below 0) each spot is than in the drawing.
    let change = |art: &[u8], bytes: &[u8], spots: &[(usize, usize)]| -> Vec<i32> {
        spots.iter().map(|&p| sum(px(bytes, p)) - sum(px(art, p))).collect()
    };
    let same_covering = |art: &[u8], bytes: &[u8]| art.chunks(4).zip(bytes.chunks(4)).all(|(a, b)| a[3] == b[3]);

    // On the disc's rim toward the upper left, toward the lower right, and at the middle.
    let rim = [(48, 31), (111, 68), (80, 50)];
    let mut badge_sheet = |name: &str, settings: J, says: &str, want: fn(&[i32]) -> bool| {
        let (bytes, big, said) = picture(&dir, "badge.png", one("core.bevel_alpha", settings));
        write(name, &big, 3);
        let c = change(&badge_art, &bytes, &rim);
        t.row(
            &format!("{name}, {says}; the covering unchanged everywhere; draws cleanly"),
            &format!("{said:?}, upper-left rim {:+}, lower-right rim {:+}, middle {:+}", c[0], c[1], c[2]),
            said.is_empty() && same_covering(&badge_art, &bytes) && want(&c),
        );
    };
    badge_sheet(
        "badge_start.png",
        serde_json::json!({}),
        "Bevel Alpha as it starts: the rim lighter at the upper left, darker at the lower right, the middle as it was",
        |c| c[0] > 0 && c[1] < 0 && c[2] == 0,
    );
    badge_sheet(
        "badge_off.png",
        serde_json::json!({ "light_intensity": 0 }),
        "light intensity 0: the badge as it was",
        |c| c == [0, 0, 0],
    );
    badge_sheet(
        "badge_wide.png",
        serde_json::json!({ "edge_thickness": 8, "light_intensity": 0.8 }),
        "edge thickness 8 at intensity 0.8: a wider, stronger bevel, the middle still as it was",
        |c| c[0] > 0 && c[1] < 0 && c[2] == 0,
    );
    badge_sheet(
        "badge_low_light.png",
        serde_json::json!({ "light_angle": 120 }),
        "the light from the lower right at 120: the lower-right rim lighter and the upper-left darker",
        |c| c[0] < 0 && c[1] > 0 && c[2] == 0,
    );
    badge_sheet(
        "badge_blue.png",
        serde_json::json!({ "light_color": "#2040a0", "light_intensity": 1 }),
        "a blue light #2040a0 at intensity 1: the lit rim bluer, the far rim darker",
        |c| c[1] < 0 && c[2] == 0,
    );
    let (blue, _, _) = picture(&dir, "badge.png", one("core.bevel_alpha", serde_json::json!({ "light_color": "#2040a0", "light_intensity": 1 })));
    let (lit, was) = (px(&blue, rim[0]), px(&badge_art, rim[0]));
    t.row(
        "in badge_blue.png the lit rim has less red and more blue than the drawing there",
        &format!("drawing {was:?}, built {lit:?}"),
        lit[0] < was[0] && lit[2] > was[2],
    );

    // Just inside the panel's left, top, right and bottom sides, and its middle.
    let sides = [(2, 30), (80, 2), (157, 30), (80, 97), (80, 30)];
    let mut panel_sheet = |name: &str, settings: J, says: &str, want: fn(&[i32]) -> bool| {
        let (bytes, big, said) = picture(&dir, "panel.png", one("core.bevel_edges", settings));
        write(name, &big, 3);
        let c = change(&panel_art, &bytes, &sides);
        t.row(
            &format!("{name}, {says}; the covering unchanged everywhere; draws cleanly"),
            &format!(
                "{said:?}, left {:+}, top {:+}, right {:+}, bottom {:+}, middle {:+}",
                c[0], c[1], c[2], c[3], c[4]
            ),
            said.is_empty() && same_covering(&panel_art, &bytes) && want(&c),
        );
    };
    panel_sheet(
        "panel_start.png",
        serde_json::json!({}),
        "Bevel Edges as it starts, a tenth of the panel's height, 10 pixels: the left and top sides lighter, \
         the right and bottom darker, the left more than the top, the middle as it was",
        |c| c[0] > c[1] && c[1] > 0 && c[2] < c[3] && c[3] < 0 && c[4] == 0,
    );
    panel_sheet(
        "panel_below.png",
        serde_json::json!({ "light_angle": 180 }),
        "the light from below at 180: the bottom lighter, the top darker, the left and right as they were",
        |c| c[0] == 0 && c[1] < 0 && c[2] == 0 && c[3] > 0 && c[4] == 0,
    );
    panel_sheet(
        "panel_gold.png",
        serde_json::json!({ "edge_thickness": 0.25, "light_color": "#ffd070", "light_intensity": 0.9 }),
        "a quarter of the panel's height, 25 pixels, with a gold light #ffd070 at 0.9: a deep frame, \
         the middle as it was",
        |c| c[0] > 0 && c[1] > 0 && c[2] < 0 && c[3] < 0 && c[4] == 0,
    );
    panel_sheet(
        "panel_pyramid.png",
        serde_json::json!({ "edge_thickness": 0.5 }),
        "half the panel's height: every pixel on a side, so even the middle changes",
        |c| c[0] > 0 && c[2] < 0 && c[4] != 0,
    );

    t.finish("B-148_bevel_table.md");
}
