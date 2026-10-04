//! B-180: D-295, Color Balance's Preserve Luminosity.
//!
//! P-26, tutorial 5: shadows pushed +100 blue turned a black backing solid mid-blue, so Screen
//! flooded the whole frame. After Effects keeps each pixel's brightness when the box is ticked.
//! Three solids, each worked by hand in display values with luma 0.2126 R + 0.7152 G + 0.0722 B:
//!
//! - black, shadows blue +100: off pushes blue to 0.5; on puts the luma back to 0, so black.
//! - grey 0.5, midtones red +100: off gives (1, 0.5, 0.5), luma 0.6063; on moves every channel
//!   by 0.5 - 0.6063 = -0.1063 to (0.8937, 0.3937, 0.3937), which fits, so it stays.
//! - white, highlights blue -100: off gives (1, 1, 0.5); on moves to luma 1, past 1 in red and
//!   green, and is pulled all the way in to white.
//!
//! The CPU is checked at 1e-5 of display value; the card at one level of 255, D-165's rule.

use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::preview::{preview_frame_srgb8, PreviewQuality};

const MAIN: &str = "comp-main";

fn to_linear(e: f64) -> f64 {
    if e <= 0.04045 { e / 12.92 } else { ((e + 0.055) / 1.055).powf(2.4) }
}

fn to_srgb(l: f64) -> f64 {
    if l <= 0.0031308 { l * 12.92 } else { 1.055 * l.powf(1.0 / 2.4) - 0.055 }
}

/// A 4x4 solid of display grey `e` at `x`, with one Color Balance.
fn solid(id: &str, e: f64, x: f64, tones: [[f64; 3]; 3], keep: Option<&str>) -> J {
    let g = to_linear(e);
    let mut parameters = json!({"shadows": tones[0], "midtones": tones[1], "highlights": tones[2]});
    if let Some(keep) = keep {
        parameters["preserve_luminosity"] = json!(keep);
    }
    json!({
        "id": id, "kind": "solid", "name": id, "enabled": true, "locked": false,
        "in_frame": 0, "out_frame": 1,
        "solid": {"color": [g, g, g], "width": 4, "height": 4},
        "transform": {
            "anchor": {"base": [2, 2], "keyframes": []},
            "position": {"base": [x, 2], "keyframes": []},
            "scale": {"base": [100, 100], "keyframes": []},
            "rotation": {"base": 0, "keyframes": []},
            "opacity": {"base": 1, "keyframes": []}
        },
        "mask": null, "matte": null, "blend_mode": "normal",
        "effects": [{"instance_id": format!("{id}-cb"), "type_id": "core.color_balance", "enabled": true, "parameters": parameters}]
    })
}

fn shot(keep: Option<&str>) -> String {
    let none = [0.0; 3];
    let layers = vec![
        solid("black", 0.0, 2.0, [[0.0, 0.0, 100.0], none, none], keep),
        solid("grey", 0.5, 6.0, [none, [100.0, 0.0, 0.0], none], keep),
        solid("white", 1.0, 10.0, [none, none, [0.0, 0.0, -100.0]], keep),
    ];
    json!({
        "schema_version": 0, "project_id": "proj-b180",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": 12, "height": 4, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["black", "grey", "white"], "layers": layers
        }]
    })
    .to_string()
}

/// The display colour at the middle of each solid, black then grey then white.
const OFF: [[f64; 3]; 3] = [[0.0, 0.0, 0.5], [1.0, 0.5, 0.5], [1.0, 1.0, 0.5]];
const ON: [[f64; 3]; 3] = [[0.0, 0.0, 0.0], [0.8937, 0.3937, 0.3937], [1.0, 1.0, 1.0]];

fn cpu(keep: Option<&str>) -> Vec<[f64; 3]> {
    let loaded = persist::load_str(&shot(keep)).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 64, &mut log).expect("the frame draws");
    assert!(log.finish().is_empty(), "the shot draws without a word");
    [2, 6, 10]
        .iter()
        .map(|&x| {
            let p = frame.pixel(x, 2);
            [0, 1, 2].map(|c| to_srgb(p[c] as f64 / p[3] as f64))
        })
        .collect()
}

#[test]
fn each_solid_lands_where_it_was_worked_by_hand() {
    for (keep, want) in [(None, OFF), (Some("off"), OFF), (Some("on"), ON)] {
        let got = cpu(keep);
        for (g, w) in got.iter().zip(want) {
            assert!((0..3).all(|c| (g[c] - w[c]).abs() < 1e-5), "{keep:?}: {g:?} against {w:?}");
        }
    }
}

#[test]
fn the_card_draws_it_as_the_cpu_does() {
    let Ok(mut gpu) = Gpu::new() else {
        eprintln!("NOT RUN: no usable card");
        return;
    };
    let loaded = persist::load_str(&shot(Some("on"))).unwrap();
    let mut log = FrameLog::new(8);
    let mut cache = CelCache::with_budget(1 << 24);
    let (px, w, _) = preview_frame_srgb8(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
        .expect("the card draws");
    for (x, want) in [2usize, 6, 10].into_iter().zip(ON) {
        let at = (2 * w + x) * 4;
        for c in 0..3 {
            let level = (want[c] * 255.0).round() as i32;
            assert!((px[at + c] as i32 - level).abs() <= 1, "x {x} channel {c}: {} against {level}", px[at + c]);
        }
    }
}

#[test]
fn the_file_keeps_it_only_when_it_is_on_and_refuses_a_wrong_word() {
    let saved = |keep| {
        let loaded = persist::load_str(&shot(keep)).unwrap();
        persist::to_json(loaded.document.project(), &loaded.preserved)
    };
    assert!(!saved(None).contains("preserve_luminosity"), "a file from before it saves as before");
    let on: J = serde_json::from_str(&saved(Some("on"))).unwrap();
    assert_eq!(on["compositions"][0]["layers"][0]["effects"][0]["parameters"]["preserve_luminosity"], "on");
    let loaded = persist::load_str(&shot(Some("maybe"))).unwrap();
    let mut log = FrameLog::new(8);
    let _ = render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 64, &mut log);
    let said = format!("{:?}", log.finish());
    assert!(said.contains("preserve luminosity is \\\"off\\\" or \\\"on\\\""), "{said}");
}
