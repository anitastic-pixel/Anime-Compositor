//! B-193: D-310, Bulge's Vertical Radius and Taper Radius.
//!
//! P-26, tutorial 1 (Shockwave) swells an oval, 240 wide and taller than it is wide, and
//! softens its edge with After Effects' Taper Radius; here a bulge was always a circle with
//! D-152's edge. Worked on a 64x64 buffer whose red is a pixel's x / 64 and green its y / 64,
//! so a bilinear read gives back where it was read from: each output pixel says which point of
//! the drawing it shows. The buffer is 63 across, so the centre, 31.5, is pixel 31's middle.
//! - At 0 both follow D-152 exactly, so a file that never set them draws as before.
//! - Radius 20 and Vertical Radius 10: a pixel 4 below the centre is at n = 0.4 of the oval's
//!   reach, t = 0.6, and reads from 4 (1 - t^2 / 2) below; one 12 below is outside and stays;
//!   one 12 across is inside.
//! - Taper Radius 5 on Radius 20: a pixel 17 out is 3 from the edge, s = 0.6, so the swell
//!   there is smoothstep(0.6) = 0.648 of the height; one 10 out, 10 from the edge, is as before.

#![recursion_limit = "256"]

use serde_json::{json, Value as J};

use anime_compositor::effects::{apply_stack, Effect, EffectInstance};
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::WorkingBuffer;

// Odd, so the centre, 50 per cent, is pixel 31's middle, 31.5.
const N: usize = 63;

fn bulge(radius: f64, height: f64, vertical_radius: f64, taper_radius: f64) -> Effect {
    Effect::Bulge { center: [50.0, 50.0], radius, height, vertical_radius, taper_radius }
}

/// Where each pixel of the bulged buffer was read from, (x, y).
fn read_from(effect: Effect) -> impl Fn(usize, usize) -> (f64, f64) {
    let mut b = WorkingBuffer::transparent(N, N);
    let data = b.data_mut();
    for y in 0..N {
        for x in 0..N {
            let i = (y * N + x) * 4;
            data[i] = (x as f32 + 0.5) / N as f32;
            data[i + 1] = (y as f32 + 0.5) / N as f32;
            data[i + 3] = 1.0;
        }
    }
    apply_stack(&mut b, &[EffectInstance::new(Id::new("b193"), effect)], |at, _, why| panic!("bypassed at {at}: {why:?}"));
    move |x, y| {
        let p = b.pixel(x, y);
        (p[0] as f64 * N as f64, p[1] as f64 * N as f64)
    }
}

fn near(got: (f64, f64), want: (f64, f64), what: &str) {
    assert!((got.0 - want.0).abs() < 1e-3 && (got.1 - want.1).abs() < 1e-3, "{what}: read from {got:?}, not {want:?}");
}

#[test]
fn at_zero_both_draw_as_before() {
    let before = read_from(bulge(20.0, 1.5, 0.0, 0.0));
    let same = read_from(bulge(20.0, 1.5, 20.0, 0.0));
    for y in 0..N {
        for x in 0..N {
            assert_eq!(before(x, y), same(x, y), "a vertical radius equal to the radius is the circle, at {x}, {y}");
        }
    }
    // D-152 by hand: 4 right of the centre, t = 1 - 4 / 20.
    let t: f64 = 1.0 - 4.0 / 20.0;
    near(before(35, 31), (31.5 + 4.0 * (1.0 - 1.5 * t * t / 2.0), 31.5), "D-152's swell");
}

#[test]
fn the_vertical_radius_makes_an_oval() {
    let f = read_from(bulge(20.0, 1.0, 10.0, 0.0));
    let t: f64 = 1.0 - 0.4;
    near(f(31, 35), (31.5, 31.5 + 4.0 * (1.0 - t * t / 2.0)), "4 below, inside the oval");
    near(f(31, 43), (31.5, 43.5), "12 below, outside it");
    let t: f64 = 1.0 - 12.0 / 20.0;
    near(f(43, 31), (31.5 + 12.0 * (1.0 - t * t / 2.0), 31.5), "12 across, inside it");
}

#[test]
fn the_taper_radius_fades_the_edge() {
    let plain = read_from(bulge(20.0, 2.0, 0.0, 0.0));
    let f = read_from(bulge(20.0, 2.0, 0.0, 5.0));
    let t: f64 = 1.0 - 17.0 / 20.0;
    near(f(48, 31), (31.5 + 17.0 * (1.0 - 0.648 * 2.0 * t * t / 2.0), 31.5), "3 from the edge, 0.648 of the swell");
    near(f(41, 31), plain(41, 31), "10 from the edge, as before");
    near(f(31, 31), plain(31, 31), "the middle, as before");
}

#[test]
fn the_draft_halves_both() {
    let mut e = bulge(50.0, 1.0, 80.0, 30.0);
    e.scale_distances(|d| d * 0.5);
    assert_eq!(e, bulge(25.0, 1.0, 40.0, 15.0));
}

#[test]
fn the_file_keeps_them_only_when_set() {
    let saved = |extra: J| {
        let mut p = json!({"center": [50, 50], "radius": 20, "height": 1});
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        let t = |v: J| json!({"base": v, "keyframes": []});
        let text = json!({
            "schema_version": 0, "project_id": "proj-b193",
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [],
            "compositions": [{
                "id": "main", "name": "main", "width": 8, "height": 8, "pixel_aspect_ratio": 1,
                "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
                "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
                "layer_order": ["art"],
                "layers": [{
                    "id": "art", "kind": "solid", "name": "art", "enabled": true, "locked": false,
                    "in_frame": 0, "out_frame": 1,
                    "solid": {"color": [1, 1, 1], "width": 8, "height": 8},
                    "transform": {
                        "anchor": t(json!([4, 4])), "position": t(json!([4, 4])),
                        "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                    },
                    "masks": [], "matte": null, "blend_mode": "normal",
                    "effects": [{"instance_id": "b", "type_id": "core.bulge", "enabled": true, "parameters": p}]
                }]
            }]
        })
        .to_string();
        let loaded = persist::load_str(&text).expect("the shot reads");
        let saved: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
        saved["compositions"][0]["layers"][0]["effects"][0]["parameters"].clone()
    };
    let p = saved(json!({}));
    assert!(p.get("vertical_radius").is_none() && p.get("taper_radius").is_none(), "unset, the file saves as before");
    let p = saved(json!({"vertical_radius": 80}));
    assert_eq!(p["vertical_radius"], 80, "a set one is kept");
    assert!(p.get("taper_radius").is_none(), "one never set stays unwritten");
    assert_eq!(saved(json!({"taper_radius": 0}))["taper_radius"], 0, "a file that wrote it keeps it");
}
