//! B-203: D-324, Lightning Bolt's Advanced Lightning extras, from D-308 after P-26.
//!
//! Tutorial 2 (Advanced Electric) uses After Effects' Advanced Lightning, whose lightning types,
//! turbulence, decay, Conductivity State and Alpha Obstacle D-190's bolt did not have; its bolt
//! ends on the ground because the ground is an obstacle. Every expected pixel is
//! `Fixtures/lightning_extras/expected_lightning_extras.json`, written by
//! `tools/lightning_extras_reference.py` before this code existed.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, set, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

const NEW: [&str; 5] = ["kind", "turbulence", "decay", "conductivity", "obstacle"];

/// `base` with the lightning type `kind` and `[turbulence, decay, conductivity, obstacle]`.
fn extras(base: &Effect, kind: &str, n: [f64; 4]) -> Effect {
    let mut e = base.clone();
    if let Effect::LightningBolt { kind: k, turbulence, decay, conductivity, obstacle, .. } = &mut e {
        *k = kind.to_string();
        [*turbulence, *decay, *conductivity, *obstacle] = n;
    }
    e
}

const SIZE: (usize, usize) = (480, 270);

/// The top of the hill at column x, as B-126's night scene has it.
fn hill(x: usize) -> f64 {
    212.0 - 10.0 * ((x as f64 + 0.5) / 60.0).sin()
}

/// B-126's night scene in two drawings: the sky with its clouds, and the hill alone on clear.
fn drawings() -> (Vec<u8>, Vec<u8>) {
    let (w, h) = SIZE;
    let clouds = [(40.0, 20.0, 90.0, 30.0), (150.0, 10.0, 110.0, 36.0), (260.0, 26.0, 100.0, 28.0), (380.0, 14.0, 120.0, 34.0)];
    let (mut sky, mut ground) = (Vec::new(), Vec::new());
    for y in 0..h {
        for x in 0..w {
            let (fx, fy) = (x as f64 + 0.5, y as f64 + 0.5);
            let t = y as f64 / h as f64;
            let mut c = [(26.0 + 30.0 * t) as u8, (28.0 + 22.0 * t) as u8, (58.0 + 40.0 * t) as u8];
            if clouds.iter().any(|&(cx, cy, rx, ry)| ((fx - cx) / rx).powi(2) + ((fy - cy) / ry).powi(2) <= 1.0) {
                c = [58, 58, 78];
            }
            sky.extend([c[0], c[1], c[2], 255]);
            ground.extend(if fy >= hill(x) { [14, 16, 28, 255] } else { [0, 0, 0, 0] });
        }
    }
    (sky, ground)
}

/// The sky, and over it the hill carrying `parameters`' bolt, drawn at `frame`.
fn picture(dir: &Path, parameters: Option<J>, frame: i32) -> (Vec<u8>, Vec<String>) {
    let (w, h) = SIZE;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let layer = |id: &str, effects: J| {
        json!({
            "id": id, "kind": "raster", "name": id, "asset_id": format!("asset-{id}"), "enabled": true,
            "locked": false, "in_frame": 0, "out_frame": 24, "source_offset_frames": 0,
            "transform": {
                "anchor": t(json!([w as f64 / 2.0, h as f64 / 2.0])), "position": t(json!([w as f64 / 2.0, h as f64 / 2.0])),
                "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
            },
            "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal", "effects": effects
        })
    };
    let asset = |id: &str| {
        json!({"id": format!("asset-{id}"), "kind": "still", "name": id, "path": format!("{id}.png"),
            "interpretation": {"color_space": "srgb", "alpha": "straight"}})
    };
    let effects = parameters.map_or(json!([]), |p| {
        json!([{"instance_id": "fx-0-0", "type_id": "core.lightning_bolt", "enabled": true, "parameters": p}])
    });
    let project = json!({
        "schema_version": 0, "project_id": "proj-b203-picture",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [asset("sky"), asset("ground")],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 24,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 24},
            "layer_order": ["sky", "ground"],
            "layers": [layer("sky", json!([])), layer("ground", effects)]
        }]
    });
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b203_lightning_extras() {
    let mut t = Table::new(
        "lightning_extras",
        "# B-203: Lightning Bolt's Advanced Lightning extras\n\nD-324, from D-308, accepted by the \
         owner on 2026-10-04. Every expected pixel is \
         `Fixtures/lightning_extras/expected_lightning_extras.json`, written by \
         `tools/lightning_extras_reference.py` before this code existed and printed in document 25 \
         as FX-LIGHTX-001 to 024. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5. \
         D-190's own FX-BOLT-001 to 032 are checked again, unchanged, by B-126.\n",
    );

    t.heading("FX-LIGHTX-001 to 024 (document 25)");
    t.fixtures("expected_lightning_extras.json");

    t.heading("The file");
    let files: Vec<String> = (1..=24).map(|n| format!("fx_lightx_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let old = persist::load(&effect_table::repo("Fixtures/lightning_bolt/fx_bolt_001.json")).unwrap();
    let saved = effect_table::saved(&old);
    let params = &saved["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    let written: Vec<&str> = NEW.iter().copied().filter(|k| params.get(k).is_some()).collect();
    t.row(
        "fx_bolt_001.json, a file from before D-324, is saved with none of the five new settings",
        &format!("{written:?}"),
        written.is_empty(),
    );
    let mut document = t.load("fx_lightx_001.json").document;
    let base = document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    let wrong = t.load("fx_lightx_020.json");
    let why = wrong.document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.why_invalid();
    t.row(
        "fx_lightx_020.json's lightning type \"sideways\" is named in a sentence",
        &why,
        why == "Lightning Bolt's lightning type is \"direction\", \"strike\", \"breaking\", \"bouncy\", \"omni\", \"anywhere\", \"vertical\" or \"two_way\", and this is \"sideways\".",
    );
    t.shape_refused(
        "fx_lightx_001.json",
        "a turbulence that is a word",
        r##"{"start": [40, 0], "end": [60, 100], "jagged": 40, "detail": 6, "branches": 30, "width": 3, "glow": 24,
            "opacity": 100, "hold": 2, "seed": 0, "color": "#ffffff", "glow_color": "#6e8cff", "turbulence": "lots"}"##,
    );
    t.shape_refused(
        "fx_lightx_001.json",
        "a lightning type that is a number",
        r##"{"start": [40, 0], "end": [60, 100], "jagged": 40, "detail": 6, "branches": 30, "width": 3, "glow": 24,
            "opacity": 100, "hold": 2, "seed": 0, "color": "#ffffff", "glow_color": "#6e8cff", "kind": 3}"##,
    );

    t.heading("How far it reaches");
    let mut draft = extras(&base, "strike", [50.0, 50.0, 3.0, 50.0]);
    let full = draft.clone();
    draft.scale_distances(|d| d * 0.5);
    let mut halved = full.clone();
    if let Effect::LightningBolt { width, glow, .. } = &mut halved {
        (*width, *glow) = (*width / 2.0, *glow / 2.0);
    }
    t.row(
        "a half-size draft preview halves the width and the glow, and none of the five new settings",
        &format!("{draft:?}"),
        draft == halved,
    );
    let grows = extras(&base, "omni", [100.0, 100.0, 10000.0, 100.0]).bounds_expansion();
    t.row("every new setting at its top, it never grows the drawing's bounds", &grows.to_string(), grows == 0);

    t.heading("Commands");
    t.refused(
        &mut document,
        vec![
            ("lightning type \"sideways\"", set(extras(&base, "sideways", [0.0; 4]))),
            ("turbulence 101", set(extras(&base, "direction", [101.0, 0.0, 0.0, 0.0]))),
            ("decay -1", set(extras(&base, "direction", [0.0, -1.0, 0.0, 0.0]))),
            ("conductivity 10001", set(extras(&base, "direction", [0.0, 0.0, 10001.0, 0.0]))),
            // D-329 took -100 to 0 (B-211), so the bottom is now -100.
            ("Alpha Obstacle -101", set(extras(&base, "direction", [0.0, 0.0, 0.0, -101.0]))),
            ("Alpha Obstacle keyed to 150", keys("obstacle", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    let mut taken: Vec<(&str, anime_compositor::command::Command)> = ["strike", "breaking", "bouncy", "omni", "anywhere", "vertical", "two_way"]
        .iter()
        .map(|k| ("a lightning type,", set(extras(&base, k, [0.0; 4]))))
        .collect();
    taken.push(("every new number at its top,", set(extras(&base, "direction", [100.0, 100.0, 10000.0, 100.0]))));
    taken.push(("conductivity keyed from 0 to 3", keys("conductivity", &[(0, &[0.0]), (4, &[3.0])])));
    taken.push(("Alpha Obstacle keyed from 0 to 100", keys("obstacle", &[(0, &[0.0]), (4, &[100.0])])));
    t.taken(&mut document, "fx_lightx_001.json", taken);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_lightx_004.json", 0), ("fx_lightx_006.json", 0), ("fx_lightx_012.json", 0), ("fx_lightx_019.json", 2)]);

    t.heading("Pictures: B-126's night scene, its hill a drawing of its own, in `verification/D-324 pictures/`");
    let dir = effect_table::repo("verification/D-324 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = SIZE;
    let (sky, ground) = drawings();
    png_out::write_rgba(&dir.join("sky.png"), w, h, OutputDepth::Eight, &[], &sky).unwrap();
    png_out::write_rgba(&dir.join("ground.png"), w, h, OutputDepth::Eight, &[], &ground).unwrap();
    let (before, said) = picture(&dir, None, 0);
    png_out::write_rgba(&dir.join("before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    t.row(
        "before.png, the sky with the hill over it, draws cleanly, the sky at the top and the hill at the bottom",
        &format!("{said:?}, top {:?}, bottom {:?}", &before[..4], &before[before.len() - 4..]),
        said.is_empty() && before[..4] == sky[..4] && before[before.len() - 4..] == [14, 16, 28, 255],
    );
    // From the cloud to below the hill, at a quarter of the width and the glow, as B-126's.
    let bolt = |kind: &str, extra: [(&str, f64); 4], end: [f64; 2]| {
        let mut p = json!({"start": [30.0, 14.0], "end": end, "jagged": 40, "detail": 6, "branches": 30,
            "width": 0.75, "glow": 6, "opacity": 100, "hold": 2, "seed": 0, "color": "#ffffff",
            "glow_color": "#6e8cff", "kind": kind});
        for (k, v) in extra {
            p[k] = json!(v);
        }
        Some(p)
    };
    let none = [("turbulence", 0.0), ("decay", 0.0), ("conductivity", 0.0), ("obstacle", 0.0)];
    let with = |k: &str, v: f64| none.map(|(n, x)| (n, if n == k { v } else { x }));
    let down = [36.0, 100.0];
    let bottom = |b: &[u8]| (0..w).filter(|&x| b[((h - 1) * w + x) * 4..][..4] != before[((h - 1) * w + x) * 4..][..4]).count();
    let mut shots: Vec<(&str, Vec<u8>)> = Vec::new();
    for (name, what, params) in [
        ("1_no_obstacle", "type Direction, Alpha Obstacle 0: the bolt runs on through the hill to the bottom edge", bolt("direction", none, down)),
        ("2_obstacle_50", "Alpha Obstacle 50: the bolt ends where it reaches the hill", bolt("direction", with("obstacle", 50.0), down)),
        ("3_strike", "Strike: the forks reach on toward the end point", bolt("strike", none, down)),
        ("4_breaking", "Breaking: the forks as bright as the bolt where they leave it", bolt("breaking", none, down)),
        ("5_bouncy", "Bouncy: three bolts between the two points", bolt("bouncy", none, down)),
        ("6_omni", "Omni: six bolts from the start, 60 degrees apart", bolt("omni", none, [30.0, 60.0])),
        ("7_anywhere", "Anywhere: one bolt from the start to somewhere within reach", bolt("anywhere", none, [30.0, 60.0])),
        ("8_vertical", "Vertical: straight down to the bottom edge, the end point unused", bolt("vertical", none, [90.0, 10.0])),
        ("9_two_way", "Two-Way Striking: one bolt from each end, meeting in the middle", bolt("two_way", none, down)),
        ("10_turbulence_100", "turbulence 100: rougher, with more forks", bolt("direction", with("turbulence", 100.0), down)),
        ("11_decay_100", "decay 100: thinning to nothing at its end", bolt("direction", with("decay", 100.0), down)),
        ("12_conductivity_0_5", "conductivity 0.5: picture 1's bolt half way to another shape", bolt("direction", with("conductivity", 0.5), down)),
    ] {
        let (bytes, said) = picture(&dir, params, 0);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let changed = (0..w * h).filter(|&i| bytes[i * 4..i * 4 + 4] != before[i * 4..i * 4 + 4]).count();
        t.row(
            &format!("{name}.png, {what}, draws cleanly and changes some pixels"),
            &format!("{said:?}, {changed} changed, {} of the bottom row", bottom(&bytes)),
            said.is_empty() && changed > 0,
        );
        shots.push((name, bytes));
    }
    let lowest = |b: &[u8]| (0..h).rev().find(|&y| (0..w).any(|x| b[(y * w + x) * 4..][..4] != before[(y * w + x) * 4..][..4])).unwrap_or(0);
    let (free, stopped) = (&shots[0].1, &shots[1].1);
    t.row(
        "with no obstacle the bolt reaches the bottom row; with Alpha Obstacle 50 its lowest lit pixel is \
         within its glow (6 pixels) of the hill's top, and the bottom row is untouched",
        &format!("bottom row {} and {}, lowest lit row {}, the hill at most row {:.0}", bottom(free), bottom(stopped), lowest(stopped), (0..w).map(hill).fold(0.0, f64::max)),
        bottom(free) > 0 && bottom(stopped) == 0 && (lowest(stopped) as f64) < (0..w).map(hill).fold(0.0, f64::max) + 7.0,
    );
    t.row(
        "each type draws a picture of its own: no two of pictures 1 and 3 to 12 are the same",
        &format!("{} pictures", shots.len()),
        shots.iter().enumerate().filter(|(i, _)| *i != 1).all(|(i, a)| shots.iter().skip(i + 1).filter(|(n, _)| *n != "2_obstacle_50").all(|b| a.1 != b.1)),
    );

    t.finish("B-203_lightning_extras_table.md");
}
