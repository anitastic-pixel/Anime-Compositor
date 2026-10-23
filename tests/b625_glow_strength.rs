//! B-625: D-746, Glow's strength and fall-off in After Effects units, fitted to After Effects'
//! own frames (audit row 12).
//!
//! Every FX-GLDISP pixel is `Fixtures/glow_display/expected_glow_display.json`, rewritten by
//! `tools/glow_display_reference.py` in D-746's commit. The two pictures below are rebuilt from
//! Wave 1's tutorial stills at a quarter size, with After Effects' own levels written in.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

const SIZE: (usize, usize) = (480, 270);

/// One opaque picture, `paint(x, y)` its 8-bit colour, saved as `name` in `dir`.
fn picture(dir: &Path, name: &str, paint: impl Fn(f64, f64) -> [u8; 3]) {
    let (w, h) = SIZE;
    let mut px = vec![255u8; w * h * 4];
    for y in 0..h {
        for x in 0..w {
            px[(y * w + x) * 4..][..3].copy_from_slice(&paint(x as f64 + 0.5, y as f64 + 0.5));
        }
    }
    png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], &px).unwrap();
}

/// `name` at 8 bpc with one Glow of `glow`'s parameters (After Effects units); the frame as
/// 8-bit straight pixels, and the warnings.
fn draw(dir: &Path, name: &str, glow: J) -> (Vec<u8>, Vec<String>) {
    let (w, h) = SIZE;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let mut parameters = json!({"based_on": "bright", "colors": [], "tolerance": 0,
        "operation": "add", "tint": "", "units": "after_effects"});
    for (k, v) in glow.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    let project = json!({
        "schema_version": 0, "project_id": "proj-b625",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-art", "kind": "still", "name": "art", "path": name,
            "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 24,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 24},
            "layer_order": ["art"], "eight_bpc": true,
            "layers": [{
                "id": "art", "kind": "raster", "name": "art", "asset_id": "asset-art", "enabled": true,
                "locked": false, "in_frame": 0, "out_frame": 24, "source_offset_frames": 0,
                "transform": {
                    "anchor": t(json!([w as f64 / 2.0, h as f64 / 2.0])), "position": t(json!([w as f64 / 2.0, h as f64 / 2.0])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal",
                "effects": [{"instance_id": "fx-glow", "type_id": "core.glow", "enabled": true,
                    "parameters": parameters}]
            }]
        }]
    });
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (drawn.to_srgb8_straight(), said)
}

fn at(b: &[u8], x: usize, y: usize) -> [u8; 3] {
    let i = (y * SIZE.0 + x) * 4;
    [b[i], b[i + 1], b[i + 2]]
}

#[test]
fn b625_glow_strength() {
    let mut t = Table::new(
        "glow_display",
        "# B-625: Glow's strength and fall-off in After Effects units\n\nD-746, audit row 12 \
         (https://claude.ai/artifact/A63KdY44N4HfUHzUnLQhTp). In 8 bpc and 32 bpc (After \
         Effects) a Glow in After Effects units gives each glowing channel min(1, intensity x \
         (t + 16 (1 - t)) / 3 x its value), spreads it at sigma 0.4 x radius and lays it on at \
         strength 1. Every FX-GLDISP pixel is `Fixtures/glow_display/expected_glow_display.json`, \
         rewritten by `tools/glow_display_reference.py`; the tolerance is 2e-5. The pictures are \
         Wave 1's tutorial frames at a quarter size, After Effects' levels measured from its stills.\n",
    );

    t.heading("FX-GLDISP-001 to 008 (document 25), D-746's values");
    t.fixtures("expected_glow_display.json");

    let dir = effect_table::repo("verification/B-625 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = SIZE;
    let (cx, cy) = (w as f64 / 2.0, h as f64 / 2.0);

    t.heading("Valeri Visuals, case 2: a white disc on black, threshold 0, radius 400, intensity 2 (a quarter size)");
    picture(&dir, "disc.png", |x, y| {
        if (x - cx).hypot(y - cy) <= 40.5 {
            [255; 3]
        } else {
            [0; 3]
        }
    });
    let (disc, said) = draw(
        &dir,
        "disc.png",
        json!({"threshold": 0, "radius": 100, "intensity": 2}),
    );
    png_out::write_rgba(&dir.join("1_valeri_disc.png"), w, h, OutputDepth::Eight, &[], &disc).unwrap();
    t.row("1_valeri_disc.png draws cleanly", &format!("{said:?}"), said.is_empty());
    // After Effects' levels at these distances from the disc's middle, in its 1920-wide frame
    // (Composite Original None, so the light alone), from `t0120` of Wave 1's stills.
    for (d, ae) in [
        (180.0, 63.4),
        (200.0, 55.7),
        (250.0, 39.3),
        (300.0, 25.7),
        (350.0, 16.1),
        (400.0, 9.0),
        (500.0, 1.7),
    ] {
        let x = (cx + d / 4.0) as usize;
        let ours = at(&disc, x, h / 2)[0] as f64;
        t.row(
            &format!("{d} pixels from the middle: After Effects {ae}, within 6 levels"),
            &format!("{ours}"),
            (ours - ae).abs() <= 6.0,
        );
    }

    t.heading("School of Motion's square: blue (57, 67, 179) on grey, threshold 60, radius 10, intensity 1");
    picture(&dir, "square.png", |x, y| {
        if (x - cx).abs() <= 50.0 && (y - cy).abs() <= 50.0 {
            [57, 67, 179]
        } else {
            [128; 3]
        }
    });
    let (square, said) = draw(
        &dir,
        "square.png",
        json!({"threshold": 60, "radius": 10, "intensity": 1}),
    );
    png_out::write_rgba(&dir.join("2_square.png"), w, h, OutputDepth::Eight, &[], &square).unwrap();
    t.row("2_square.png draws cleanly", &format!("{said:?}"), said.is_empty());
    let mid = at(&square, w / 2, h / 2);
    t.row(
        "the middle: only blue is past the threshold, so only blue glows; After Effects shows (56, 70, 255), red and green within 4",
        &format!("{mid:?}"),
        mid[2] == 255 && (mid[0] as i32 - 56).abs() <= 4 && (mid[1] as i32 - 70).abs() <= 4,
    );
    let grey = at(&square, w / 2 + 52, h / 2);
    t.row(
        "2 pixels outside the square the grey gains blue only",
        &format!("{grey:?}"),
        grey[0] == 128 && grey[1] == 128 && grey[2] > 128,
    );

    t.heading("Intensity 0 changes nothing (no dark ring is added)");
    let (none, _) = draw(
        &dir,
        "square.png",
        json!({"threshold": 60, "radius": 10, "intensity": 0}),
    );
    let edge = at(&none, w / 2 + 52, h / 2);
    t.row(
        "2 pixels outside the square, intensity 0",
        &format!("{edge:?}"),
        edge == [128; 3] && at(&none, w / 2, h / 2) == [57, 67, 179],
    );
    for f in ["disc.png", "square.png"] {
        fs::remove_file(dir.join(f)).unwrap();
    }

    t.finish("B-625_glow_strength_table.md");
}
