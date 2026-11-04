//! B-931: D-1051, Lightning Bolt's Core Opacity and Glow Opacity, from wave 12's tutorial
//! uGxruFKgt6U.
//!
//! After Effects' Advanced Lightning has a Core Opacity and a Glow Opacity of their own; D-190's
//! bolt had one Opacity for both. Every expected pixel is
//! `Fixtures/lightning_opacity/expected_lightning_opacity.json`, written by
//! `tools/lightning_opacity_reference.py` before this code existed.

mod effect_table;

use std::fs;

use effect_table::{both, changed, keys, on_card, set, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::{card_takes, render_frame};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::PreviewQuality;
use anime_compositor::{persist, png_out, OutputDepth};

const SIZE: (usize, usize) = (480, 270);

/// `effect` with Core Opacity `a` and Glow Opacity `b`.
fn opacities(a: f64, b: f64) -> impl FnOnce(&mut Effect) {
    move |e| {
        if let Effect::LightningBolt {
            core_opacity,
            glow_opacity,
            ..
        } = e
        {
            *core_opacity = a;
            *glow_opacity = b;
        }
    }
}

/// A dark purple sky with `parameters`' bolt on it, drawn at frame 0, in 8-bit sRGB.
fn picture(dir: &std::path::Path, parameters: Option<J>) -> (Vec<u8>, Vec<String>) {
    let (w, h) = SIZE;
    let mut sky = Vec::new();
    for y in 0..h {
        let t = y as f64 / h as f64;
        for _ in 0..w {
            sky.extend([(30.0 + 20.0 * t) as u8, 16, (50.0 + 40.0 * t) as u8, 255]);
        }
    }
    png_out::write_rgba(&dir.join("sky.png"), w, h, OutputDepth::Eight, &[], &sky).unwrap();
    let t = |v: J| json!({"base": v, "keyframes": []});
    let effects = parameters.map_or(json!([]), |p| {
        json!([{"instance_id": "fx-0-0", "type_id": "core.lightning_bolt", "enabled": true, "parameters": p}])
    });
    let project = json!({
        "schema_version": 0, "project_id": "proj-b931-picture",
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
    let drawn = render_frame(
        loaded.document.project(),
        &Id::new(MAIN),
        0,
        dir,
        64,
        &mut log,
    )
    .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b931_lightning_opacity() {
    let mut t = Table::new(
        "lightning_opacity",
        "# B-931: Lightning Bolt's Core Opacity and Glow Opacity\n\nD-1051, from wave 12's \
         tutorial uGxruFKgt6U. Every expected pixel is \
         `Fixtures/lightning_opacity/expected_lightning_opacity.json`, written by \
         `tools/lightning_opacity_reference.py` before this code existed and printed in document \
         25 as FX-LOPAC-001 to 010. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5. \
         D-190's, D-324's and D-334's own cases are checked again, unchanged, by B-126, B-203 and \
         B-215.\n",
    );

    t.heading("FX-LOPAC-001 to 010 (document 25)");
    t.fixtures("expected_lightning_opacity.json");

    t.heading("The file");
    let files: Vec<String> = (1..=10).map(|n| format!("fx_lopac_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let old = persist::load(&effect_table::repo(
        "Fixtures/lightning_bolt/fx_bolt_001.json",
    ))
    .unwrap();
    let saved = effect_table::saved(&old);
    let params = &saved["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    t.row(
        "fx_bolt_001.json, a file from before D-1051, is saved without either opacity",
        &format!(
            "{:?} {:?}",
            params.get("core_opacity"),
            params.get("glow_opacity")
        ),
        params.get("core_opacity").is_none() && params.get("glow_opacity").is_none(),
    );
    let wrong = t.load("fx_lopac_009.json");
    let why = wrong
        .document
        .project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layer(&Id::new("art"))
        .unwrap()
        .effects[0]
        .effect
        .why_invalid();
    t.row(
        "fx_lopac_009.json's core opacity of 101 is named in a sentence",
        &why,
        why == "Lightning Bolt's core opacity runs from 0 to 100, and this is 101.",
    );

    t.heading("Commands");
    let mut document = t.load("fx_lopac_005.json").document;
    t.refused(
        &mut document,
        vec![
            (
                "core opacity 101",
                set(changed(&t, "fx_lopac_005.json", opacities(101.0, 100.0))),
            ),
            (
                "glow opacity -1",
                set(changed(&t, "fx_lopac_005.json", opacities(100.0, -1.0))),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_lopac_005.json",
        vec![
            (
                "core 75 and glow 50,",
                set(changed(&t, "fx_lopac_005.json", opacities(75.0, 50.0))),
            ),
            (
                "core opacity keyed 0 to 100,",
                keys("core_opacity", &[(0, &[0.0]), (4, &[100.0])]),
            ),
        ],
    );

    t.heading("The graphics card");
    // D-1106 (B-986): the card draws both opacities, so every bolt is handed to it.
    for (file, takes) in [
        ("fx_lopac_005.json", true),
        ("fx_lopac_006.json", true),
        ("fx_lopac_008.json", true),
    ] {
        let d = t.load(file).document;
        let instance = &d
            .project()
            .composition(&Id::new(MAIN))
            .unwrap()
            .layer(&Id::new("art"))
            .unwrap()
            .effects[0];
        let got = card_takes(instance, 16 * 10);
        t.row(
            &format!(
                "{file}: {}",
                if file == "fx_lopac_005.json" {
                    "both at 100, handed to the card as before"
                } else {
                    "an opacity below 100, handed to the card too (D-1106)"
                }
            ),
            &format!("handed: {got}"),
            got == takes,
        );
    }

    t.heading("The card against the processor (D-1106): FX-LOPAC-001 to 010, every frame, Full and Draft");
    let mut gpu = Gpu::new().expect("a graphics card");
    let comp = Id::new(MAIN);
    for file in &files {
        let project = persist::load(&t.root.join(file))
            .unwrap()
            .document
            .project()
            .clone();
        let c = project.composition(&comp).unwrap();
        let frames = c.start_frame..c.start_frame + c.duration_frames as i32;
        let (mut largest, mut refused, mut agree, mut card, mut count) = (0, false, true, 0, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in frames.clone() {
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| {
                    matches!(e, Effect::LightningBolt { .. })
                });
                count += 1;
            }
        }
        // 009 and 010 are left out with a warning, so nothing of theirs is on the card.
        let drawn = !matches!(file.as_str(), "fx_lopac_009.json" | "fx_lopac_010.json");
        t.row(
            &format!("{file}, {count} frames"),
            &format!("largest difference {largest} of 255; bolt on the card in {card} of {count}; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && card == if drawn { count } else { 0 },
        );
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_lopac_006.json", 0), ("fx_lopac_008.json", 2)]);

    t.heading("Pictures: the tutorial's retro bolt, in `verification/D-1051 pictures/`");
    let dir = effect_table::repo("verification/D-1051 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = SIZE;
    // The tutorial's Core Radius 2.4 is Width 4.8; Glow Radius 50 is Glow 50; pink glow.
    let bolt = |core: f64, glow: f64| {
        Some(
            json!({"start": [30.0, 0.0], "end": [70.0, 100.0], "jagged": 40, "detail": 6,
            "branches": 25, "width": 4.8, "glow": 50, "opacity": 100, "hold": 2, "seed": 0,
            "color": "#ffffff", "glow_color": "#ff4fd8", "turbulence": 1, "decay": 0.3,
            "core_opacity": core, "glow_opacity": glow}),
        )
    };
    let (before, _) = picture(&dir, None);
    let mut shots = Vec::new();
    for (name, what, core, glow) in [
        ("1_full", "core and glow at 100, the bolt before D-1051: a wide, strong pink glow", 100.0, 100.0),
        ("2_tutorial", "the tutorial's core 75 and glow 50: the same bolt, the glow faint, the core a little see-through", 75.0, 50.0),
        ("3_glow_only", "core 0, glow 100: no white core, the pink glow alone", 0.0, 100.0),
    ] {
        let (bytes, said) = picture(&dir, bolt(core, glow));
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let changed = (0..w * h).filter(|&i| bytes[i * 4..i * 4 + 4] != before[i * 4..i * 4 + 4]).count();
        t.row(
            &format!("{name}.png, {what}, draws cleanly and changes some pixels"),
            &format!("{said:?}, {changed} changed"),
            said.is_empty() && changed > 0,
        );
        shots.push(bytes);
    }
    let white = |b: &[u8]| b.chunks(4).filter(|p| p[..3] == [255, 255, 255]).count();
    let dimmer = (0..w * h).all(|i| (0..3).all(|c| shots[1][i * 4 + c] <= shots[0][i * 4 + c]));
    t.row(
        "the tutorial's settings are never brighter than full, and core 0 leaves no pure white",
        &format!(
            "pure white: full {}, tutorial {}, glow only {}",
            white(&shots[0]),
            white(&shots[1]),
            white(&shots[2])
        ),
        dimmer && white(&shots[2]) == 0 && white(&shots[0]) > 0,
    );

    t.finish("B-931_lightning_opacity_table.md");
}
