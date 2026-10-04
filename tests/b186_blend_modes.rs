//! B-186: D-301, the Overlay, Soft Light, Stencil Alpha and Stencil Luma blend modes.
//!
//! P-26, tutorials 2 (Advanced Electric) and 3 (Colorful Glitch) put layers in Overlay, Soft
//! Light and Stencil modes; here a layer had normal, multiply, screen or add only.
//! Worked by hand on a 16x16 opaque backdrop, with values encoded as document 21's sRGB:
//! - Overlay and Soft Light run on the encoded colours, so a 50% grey changes nothing.
//! - Backdrop encoded (0.25, 0.75, 0.5) under top (0.8, 0.3, 0.5):
//!   Overlay is (2 x 0.25 x 0.8, 1 - 2 x 0.25 x 0.7, 0.5) = (0.4, 0.65, 0.5);
//!   Soft Light is (0.25 + 0.6 x (0.5 - 0.25), 0.75 - 0.4 x 0.75 x 0.25, 0.5) = (0.4, 0.675, 0.5),
//!   where 0.5 is W3C's D(0.25) = ((16 x 0.25 - 12) x 0.25 + 4) x 0.25.
//! - An 8x8 Stencil Alpha at 50% opacity in the top left quarter keeps half of the backdrop there
//!   and nothing anywhere else.
//! - An 8x8 opaque Stencil Luma of linear grey 0.2 keeps the backdrop times 0.2 encoded,
//!   0.484529, there and nothing anywhere else.
//! Tiles of 4 so that P-05's culling skips tiles the stencil does not reach, if it would.

use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::model::Id;
use anime_compositor::persist;

const MAIN: &str = "comp-main";

fn enc(c: f64) -> f64 {
    if c <= 0.0031308 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 }
}

fn lin(c: f64) -> f64 {
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

fn solid(id: &str, color: [f64; 3], size: u32, opacity: f64, blend: &str) -> J {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let h = size as f64 / 2.0;
    json!({
        "id": id, "kind": "solid", "name": id, "enabled": true, "locked": false,
        "in_frame": 0, "out_frame": 1,
        "solid": {"color": color, "width": size, "height": size},
        "transform": {
            "anchor": t(json!([h, h])), "position": t(json!([h, h])),
            "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(opacity))
        },
        "masks": [], "matte": null, "blend_mode": blend, "effects": []
    })
}

fn shot(layers: Vec<J>) -> String {
    let order: Vec<J> = layers.iter().map(|l| l["id"].clone()).collect();
    json!({
        "schema_version": 0, "project_id": "proj-b186",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": 16, "height": 16, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": order, "layers": layers
        }]
    })
    .to_string()
}

fn frame(layers: Vec<J>) -> Vec<[f32; 4]> {
    let loaded = persist::load_str(&shot(layers)).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let f = render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 4, &mut log)
        .expect("the frame draws");
    (0..16).flat_map(|y| (0..16).map(move |x| (x, y))).map(|(x, y)| f.pixel(x, y)).collect()
}

/// The backdrop, encoded (0.25, 0.75, 0.5), stored linear.
fn backdrop() -> J {
    solid("back", [lin(0.25), lin(0.75), lin(0.5)], 16, 1.0, "normal")
}

#[test]
fn overlay_and_soft_light_are_the_encoded_sums_worked_by_hand() {
    let grey = [lin(0.5); 3];
    let alone = frame(vec![backdrop()]);
    for mode in ["overlay", "soft_light"] {
        let neutral = frame(vec![backdrop(), solid("top", grey, 16, 1.0, mode)]);
        for (i, (a, b)) in alone.iter().zip(&neutral).enumerate() {
            for c in 0..4 {
                assert!((a[c] - b[c]).abs() < 1e-5, "{mode}, pixel {i}: 50% grey changes nothing: {a:?} {b:?}");
            }
        }
    }
    let top = [lin(0.8), lin(0.3), lin(0.5)];
    for (mode, want) in [("overlay", [0.4, 0.65, 0.5]), ("soft_light", [0.4, 0.675, 0.5])] {
        let got = frame(vec![backdrop(), solid("top", top, 16, 1.0, mode)]);
        for (i, p) in got.iter().enumerate() {
            for c in 0..3 {
                assert!((enc(p[c] as f64) - want[c]).abs() < 1e-5, "{mode}, pixel {i}, channel {c}: {} against {}", enc(p[c] as f64), want[c]);
            }
            assert_eq!(p[3], 1.0, "{mode}, pixel {i}: opaque");
        }
    }
}

#[test]
fn a_stencil_keeps_the_backdrop_only_where_it_is() {
    let alone = frame(vec![backdrop()]);
    let cases = [
        ("stencil_alpha", solid("top", [1.0, 0.0, 0.0], 8, 0.5, "stencil_alpha"), 0.5),
        ("stencil_luma", solid("top", [0.2, 0.2, 0.2], 8, 1.0, "stencil_luma"), enc(0.2)),
    ];
    for (mode, top, k) in cases {
        let got = frame(vec![backdrop(), top]);
        for (i, (a, b)) in alone.iter().zip(&got).enumerate() {
            let inside = i % 16 < 8 && i / 16 < 8;
            for c in 0..4 {
                let want = if inside { a[c] as f64 * k } else { 0.0 };
                assert!((b[c] as f64 - want).abs() < 1e-6, "{mode}, pixel {i}, channel {c}: {} against {want}", b[c]);
            }
        }
    }
    assert!((enc(0.2) - 0.484529).abs() < 1e-6, "the luma worked by hand");
}

/// The card does not draw these four, so a frame with one is drawn on the CPU and says so, and
/// the viewer shows the same picture as the export.
#[test]
fn the_card_hands_them_to_the_cpu() {
    let Ok(mut gpu) = Gpu::new() else {
        eprintln!("no usable card; nothing to check");
        return;
    };
    for mode in ["multiply", "overlay", "soft_light", "stencil_alpha", "stencil_luma"] {
        let loaded = persist::load_str(&shot(vec![backdrop(), solid("top", [1.0; 3], 8, 1.0, mode)])).expect("the shot reads");
        let mut log = FrameLog::new(8);
        let mut cache = CelCache::viewer();
        preview::preview_frame_srgb8(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), PreviewQuality::Full, 4, &mut log, &mut cache, &mut gpu)
            .expect("the frame draws");
        let on_cpu = log.finish().iter().any(|d| d.id == DiagnosticId::GpuPreviewOnCpu);
        assert_eq!(on_cpu, mode != "multiply", "{mode}: drawn on the CPU only if the card cannot");
    }
}

#[test]
fn the_file_keeps_each_mode_and_refuses_an_unknown_one() {
    for mode in ["normal", "multiply", "screen", "add", "overlay", "soft_light", "stencil_alpha", "stencil_luma"] {
        let loaded = persist::load_str(&shot(vec![backdrop(), solid("top", [1.0; 3], 8, 1.0, mode)])).expect("the shot reads");
        let saved: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
        assert_eq!(saved["compositions"][0]["layers"][1]["blend_mode"], mode, "{mode} is kept");
    }
    let wrong = persist::load_str(&shot(vec![backdrop(), solid("top", [1.0; 3], 8, 1.0, "dodge")]));
    let why = format!("{:?}", wrong.err().expect("an unknown mode is refused"));
    assert!(why.contains("layers/1/blend_mode: expected one of normal, multiply, screen, add, overlay, soft_light, stencil_alpha, stencil_luma."), "the reason names where and what is allowed: {why}");
}
