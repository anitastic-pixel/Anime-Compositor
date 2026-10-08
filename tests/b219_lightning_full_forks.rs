//! B-219: D-339, Lightning Bolt's full-width long forks, from P-26's tutorial 2.
//!
//! The tutorial's strands are nearly as thick as the main bolt; D-338's long forks start at half
//! its weight. Every expected pixel is `Fixtures/lightning_full_forks/expected_lightning_full_forks.json`,
//! written by `tools/lightning_full_forks_reference.py` before this code existed.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{set, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

/// `base` with the forks `word`.
fn forked(base: &Effect, word: &str) -> Effect {
    let mut e = base.clone();
    if let Effect::LightningBolt { forks, .. } = &mut e {
        *forks = word.to_string();
    }
    e
}

const SIZE: (usize, usize) = (480, 270);
/// Where the ground starts, from the top.
const GROUND: usize = 210;

/// A night sky, and the ground under it as a drawing of its own, empty above `GROUND`.
fn drawings() -> [Vec<u8>; 2] {
    let (w, h) = SIZE;
    let (mut sky, mut ground) = (Vec::new(), Vec::new());
    for y in 0..h {
        let t = y as f64 / h as f64;
        for _ in 0..w {
            sky.extend([(26.0 + 30.0 * t) as u8, (28.0 + 22.0 * t) as u8, (58.0 + 40.0 * t) as u8, 255]);
            ground.extend(if y >= GROUND { [52, 40, 34, 255] } else { [0; 4] });
        }
    }
    [sky, ground]
}

/// The sky, and over it the ground carrying `parameters`' bolt, at frame `frame`.
fn picture(dir: &Path, frame: i32, parameters: Option<J>) -> (Vec<u8>, Vec<String>) {
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
        "schema_version": 0, "project_id": "proj-b219-picture",
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
fn b219_lightning_full_forks() {
    let mut t = Table::new(
        "lightning_full_forks",
        "# B-219: Lightning Bolt's full-width long forks\n\nD-339, from P-26's tutorial 2. Every \
         expected pixel is `Fixtures/lightning_full_forks/expected_lightning_full_forks.json`, \
         written by `tools/lightning_full_forks_reference.py` before this code existed and printed \
         in document 25 as FX-LFULL-001 to 006. The build's frame is compared sample by sample; \
         the answer is the largest difference over all of them, against the catalogue's tolerance \
         of 2e-5. D-338's own cases are checked again, unchanged, by B-218.\n",
    );

    t.heading("FX-LFULL-001 to 006 (document 25)");
    t.fixtures("expected_lightning_full_forks.json");

    t.heading("The file");
    let files: Vec<String> = (1..=6).map(|n| format!("fx_lfull_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let mut document = t.load("fx_lfull_001.json").document;
    let base = document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    let wrong = t.load("fx_lfull_006.json");
    let why = wrong.document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.why_invalid();
    t.row(
        "fx_lfull_006.json's forks \"Full\" is named in a sentence",
        &why,
        why == "Lightning Bolt's forks are \"short\", \"long\" or \"full\", and this is \"Full\".",
    );

    t.heading("Commands");
    t.refused(&mut document, vec![("forks \"Full\"", set(forked(&base, "Full")))]);
    t.taken(
        &mut document,
        "fx_lfull_001.json",
        vec![("forks long,", set(forked(&base, "long"))), ("forks full,", set(forked(&base, "full")))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_lfull_001.json", 0), ("fx_lfull_003.json", 1), ("fx_lfull_004.json", 2)]);

    t.heading("Pictures: a night sky over the ground, in `verification/D-339 pictures/`");
    let dir = effect_table::repo("verification/D-339 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = SIZE;
    let [sky, ground] = drawings();
    for (name, bytes) in [("sky", &sky), ("ground", &ground)] {
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    }
    // Tutorial 2's bolt, as B-218's pictures.
    let bolt = |forks: &str| {
        Some(json!({"start": [50.0, 0.0], "end": [50.0, 100.0], "jagged": 40, "detail": 6, "branches": 37,
            "width": 3, "glow": 8, "opacity": 100, "hold": 1, "seed": 0, "color": "#ffffff",
            "glow_color": "#6e8cff", "kind": "direction", "decay": 0, "obstacle": 50, "forks": forks}))
    };
    let (before, said) = picture(&dir, 0, None);
    t.row("the sky over the ground draws cleanly", &format!("{said:?}"), said.is_empty());
    let lit = |b: &[u8], x: usize, y: usize| b[(y * w + x) * 4..][..4] != before[(y * w + x) * 4..][..4];
    // How much of the sky the bolts' white cores cover: pixels whose green, which the blue glow
    // hardly has, is 200 or more.
    let white = |b: &[u8]| (0..w * GROUND).filter(|i| b[i * 4 + 1] >= 200).count();
    let deep = |b: &[u8]| (GROUND + 12..h).flat_map(|y| (0..w).map(move |x| (x, y))).filter(|&(x, y)| lit(b, x, y)).count();
    for frame in [0, 7] {
        let mut shots = Vec::new();
        for (forks, what) in [
            ("long", "forks Long, D-338's: the strands start at half the main bolt's width"),
            ("full", "forks Long, full width: the strands start as wide as the main bolt"),
        ] {
            let name = format!("f{frame}_{forks}");
            let (bytes, said) = picture(&dir, frame, bolt(forks));
            png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
            let (core, under) = (white(&bytes), deep(&bytes));
            t.row(
                &format!("{name}.png, frame {frame}, {what}"),
                &format!("{said:?}, {core} white pixels in the sky, {under} pixels lit more than 12 into the ground"),
                said.is_empty() && under == 0,
            );
            shots.push(core);
        }
        t.row(
            &format!("on frame {frame}, the full-width strands cover more of the sky in white than the long ones"),
            &format!("long {}, full {}", shots[0], shots[1]),
            shots[1] > shots[0],
        );
    }

    t.finish("D-339_lightning_full_forks_table.md");
}
