//! B-218: D-338, Lightning Bolt's long forks, from P-26's tutorial 2.
//!
//! The tutorial turns Advanced Lightning's Decay down so its forks run on down beside the main
//! bolt until the ground stops them; D-190's forks are short twigs. Every expected pixel is
//! `Fixtures/lightning_forks/expected_lightning_forks.json`, written by
//! `tools/lightning_forks_reference.py` before this code existed.

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
        "schema_version": 0, "project_id": "proj-b218-picture",
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
fn b218_lightning_forks() {
    let mut t = Table::new(
        "lightning_forks",
        "# B-218: Lightning Bolt's long forks\n\nD-338, from P-26's tutorial 2. Every expected pixel \
         is `Fixtures/lightning_forks/expected_lightning_forks.json`, written by \
         `tools/lightning_forks_reference.py` before this code existed and printed in document 25 \
         as FX-LFORK-001 to 008. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5. \
         D-190's, D-324's, D-329's and D-334's own cases are checked again, unchanged, by B-126, \
         B-203, B-211 and B-215.\n",
    );

    t.heading("FX-LFORK-001 to 008 (document 25)");
    t.fixtures("expected_lightning_forks.json");

    t.heading("The file");
    let files: Vec<String> = (1..=8).map(|n| format!("fx_lfork_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let old = persist::load(&effect_table::repo("Fixtures/lightning_bolt/fx_bolt_001.json")).unwrap();
    let saved = effect_table::saved(&old);
    let params = &saved["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    t.row(
        "fx_bolt_001.json, a file from before D-338, is saved without the word forks",
        &format!("{:?}", params.get("forks")),
        params.get("forks").is_none(),
    );
    let mut document = t.load("fx_lfork_001.json").document;
    let base = document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    let wrong = t.load("fx_lfork_008.json");
    let why = wrong.document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.why_invalid();
    t.row(
        "fx_lfork_008.json's forks \"many\" is named in a sentence",
        &why,
        why == "Lightning Bolt's forks are \"short\" or \"long\", and this is \"many\".",
    );

    t.heading("Commands");
    t.refused(&mut document, vec![("forks \"many\"", set(forked(&base, "many")))]);
    t.taken(
        &mut document,
        "fx_lfork_001.json",
        vec![("forks short,", set(forked(&base, "short"))), ("forks long,", set(forked(&base, "long")))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_lfork_001.json", 0), ("fx_lfork_004.json", 1), ("fx_lfork_006.json", 2)]);

    t.heading("Pictures: a night sky over the ground, in `verification/D-338 pictures/`");
    let dir = effect_table::repo("verification/D-338 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = SIZE;
    let [sky, ground] = drawings();
    for (name, bytes) in [("sky", &sky), ("ground", &ground)] {
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    }
    // Tutorial 2's bolt: Direction, decay 0, branches 37, Alpha Obstacle 50, its end in the ground.
    let bolt = |forks: &str| {
        Some(json!({"start": [50.0, 0.0], "end": [50.0, 100.0], "jagged": 40, "detail": 6, "branches": 37,
            "width": 3, "glow": 8, "opacity": 100, "hold": 1, "seed": 0, "color": "#ffffff",
            "glow_color": "#6e8cff", "kind": "direction", "decay": 0, "obstacle": 50, "forks": forks}))
    };
    let (before, said) = picture(&dir, 0, None);
    png_out::write_rgba(&dir.join("before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    t.row("before.png, the sky over the ground, draws cleanly", &format!("{said:?}"), said.is_empty());
    let lit = |b: &[u8], x: usize, y: usize| b[(y * w + x) * 4..][..4] != before[(y * w + x) * 4..][..4];
    // Where a strand's core meets the ground: the columns just above it whose green, which the blue
    // glow hardly has, is 200 or more, in runs.
    let touches = |b: &[u8]| {
        let row: Vec<bool> = (0..w).map(|x| b[((GROUND - 1) * w + x) * 4 + 1] >= 200).collect();
        (0..w).filter(|&x| row[x] && (x == 0 || !row[x - 1])).count()
    };
    let deep = |b: &[u8]| (GROUND + 12..h).flat_map(|y| (0..w).map(move |x| (x, y))).filter(|&(x, y)| lit(b, x, y)).count();
    for frame in [0, 7] {
        let mut shots = Vec::new();
        for (forks, what) in [
            ("short", "forks Short, D-190's: the forks are short twigs off the main bolt"),
            ("long", "forks Long: the first forks run on down beside the main bolt to the ground"),
        ] {
            let name = format!("f{frame}_{forks}");
            let (bytes, said) = picture(&dir, frame, bolt(forks));
            png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
            let (touch, under) = (touches(&bytes), deep(&bytes));
            t.row(
                &format!("{name}.png, frame {frame}, {what}"),
                &format!("{said:?}, {touch} strands touch the ground, {under} pixels lit more than 12 into it"),
                said.is_empty() && under == 0 && touch >= 1,
            );
            shots.push(touch);
        }
        t.row(
            &format!("on frame {frame}, more strands reach the ground with long forks than with short"),
            &format!("short {}, long {}", shots[0], shots[1]),
            shots[1] > shots[0],
        );
    }

    t.finish("D-338_lightning_forks_table.md");
}
