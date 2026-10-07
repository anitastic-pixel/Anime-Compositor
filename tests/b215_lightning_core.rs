//! B-215: D-334, Lightning Bolt's soft core, from P-26's tutorial 2.
//!
//! After Effects' Advanced Lightning draws its core bright in the middle and fading to its edge;
//! D-190's core covered its width evenly, so tutorial 2's bolt read as a flat ribbon. Every
//! expected pixel is `Fixtures/lightning_core/expected_lightning_core.json`, written by
//! `tools/lightning_core_reference.py` before this code existed.

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

/// `base` with the core edge `word`.
fn cored(base: &Effect, word: &str) -> Effect {
    let mut e = base.clone();
    if let Effect::LightningBolt { core, .. } = &mut e {
        *core = word.to_string();
    }
    e
}

const SIZE: (usize, usize) = (480, 270);

/// A night sky, darker at the top, with `parameters`' bolt on it, drawn at frame 0.
fn picture(dir: &Path, parameters: Option<J>) -> (Vec<u8>, Vec<String>) {
    let (w, h) = SIZE;
    let mut sky = Vec::new();
    for y in 0..h {
        let t = y as f64 / h as f64;
        for _ in 0..w {
            sky.extend([(26.0 + 30.0 * t) as u8, (28.0 + 22.0 * t) as u8, (58.0 + 40.0 * t) as u8, 255]);
        }
    }
    png_out::write_rgba(&dir.join("sky.png"), w, h, OutputDepth::Eight, &[], &sky).unwrap();
    let t = |v: J| json!({"base": v, "keyframes": []});
    let effects = parameters.map_or(json!([]), |p| {
        json!([{"instance_id": "fx-0-0", "type_id": "core.lightning_bolt", "enabled": true, "parameters": p}])
    });
    let project = json!({
        "schema_version": 0, "project_id": "proj-b215-picture",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-sky", "kind": "still", "name": "sky", "path": "sky.png",
            "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 24,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 24},
            "layer_order": ["sky"],
            "layers": [{
                "id": "sky", "kind": "raster", "name": "sky", "asset_id": "asset-sky", "enabled": true,
                "locked": false, "in_frame": 0, "out_frame": 24, "source_offset_frames": 0,
                "transform": {
                    "anchor": t(json!([w as f64 / 2.0, h as f64 / 2.0])), "position": t(json!([w as f64 / 2.0, h as f64 / 2.0])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal", "effects": effects
            }]
        }]
    });
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b215_lightning_core() {
    let mut t = Table::new(
        "lightning_core",
        "# B-215: Lightning Bolt's soft core\n\nD-334, from P-26's tutorial 2. Every expected pixel \
         is `Fixtures/lightning_core/expected_lightning_core.json`, written by \
         `tools/lightning_core_reference.py` before this code existed and printed in document 25 \
         as FX-LCORE-001 to 007. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5. \
         D-190's and D-324's own cases are checked again, unchanged, by B-126 and B-203.\n",
    );

    t.heading("FX-LCORE-001 to 007 (document 25)");
    t.fixtures("expected_lightning_core.json");

    t.heading("The file");
    let files: Vec<String> = (1..=7).map(|n| format!("fx_lcore_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let old = persist::load(&effect_table::repo("Fixtures/lightning_bolt/fx_bolt_001.json")).unwrap();
    let saved = effect_table::saved(&old);
    let params = &saved["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    t.row(
        "fx_bolt_001.json, a file from before D-334, is saved without the word core",
        &format!("{:?}", params.get("core")),
        params.get("core").is_none(),
    );
    let mut document = t.load("fx_lcore_001.json").document;
    let base = document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    let wrong = t.load("fx_lcore_007.json");
    let why = wrong.document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.why_invalid();
    t.row(
        "fx_lcore_007.json's core \"fuzzy\" is named in a sentence",
        &why,
        why == "Lightning Bolt's core edge is \"hard\" or \"soft\", and this is \"fuzzy\".",
    );

    t.heading("Commands");
    t.refused(&mut document, vec![("core \"fuzzy\"", set(cored(&base, "fuzzy")))]);
    t.taken(
        &mut document,
        "fx_lcore_001.json",
        vec![("core hard,", set(cored(&base, "hard"))), ("core soft,", set(cored(&base, "soft")))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_lcore_001.json", 0), ("fx_lcore_003.json", 2)]);

    t.heading("Pictures: a night sky, in `verification/D-334 pictures/`");
    let dir = effect_table::repo("verification/D-334 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = SIZE;
    let bolt = |core: &str| {
        Some(json!({"start": [30.0, 0.0], "end": [60.0, 100.0], "jagged": 40, "detail": 6, "branches": 30,
            "width": 4, "glow": 6, "opacity": 100, "hold": 2, "seed": 0, "color": "#ffffff",
            "glow_color": "#6e8cff", "core": core}))
    };
    let (before, _) = picture(&dir, None);
    let mut shots = Vec::new();
    for (name, what, core) in [
        ("1_hard", "core Hard, D-190's: the core one flat white ribbon with a hard edge", "hard"),
        ("2_soft", "core Soft: the core white in the middle, fading into the glow at its edge", "soft"),
    ] {
        let (bytes, said) = picture(&dir, bolt(core));
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let changed = (0..w * h).filter(|&i| bytes[i * 4..i * 4 + 4] != before[i * 4..i * 4 + 4]).count();
        t.row(&format!("{name}.png, {what}, draws cleanly and changes some pixels"), &format!("{said:?}, {changed} changed"), said.is_empty() && changed > 0);
        shots.push(bytes);
    }
    let white = |b: &[u8]| b.chunks(4).filter(|p| p[..3] == [255, 255, 255]).count();
    let darker = (0..w * h).all(|i| (0..3).all(|c| shots[1][i * 4 + c] <= shots[0][i * 4 + c]));
    t.row(
        "the soft core is never brighter than the hard one, and has fewer pure white pixels",
        &format!("pure white: hard {}, soft {}", white(&shots[0]), white(&shots[1])),
        darker && white(&shots[1]) < white(&shots[0]),
    );

    t.finish("D-334_lightning_core_table.md");
}
