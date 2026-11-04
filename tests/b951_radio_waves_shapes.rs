//! B-951: D-1071, Radio Waves' Curve Size, Star, Star Depth and Frequency (f80Aj131YJs 1:40 to
//! 3:40, G7 of the wave 12 report). Worked by hand: one white wave on a black 200-pixel solid,
//! its producer in the middle (100, 100), a square (Sides 4) turned 45 degrees so its sides
//! stand straight, 60 pixels from the middle to a corner and so 42.43 to a side's middle, 2
//! pixels wide. A pixel is read at its centre, so pixel 142 is 42.5 from the middle.

#![recursion_limit = "256"]

mod effect_table;

use effect_table::{repo, Table};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth, WorkingBuffer};

const MAIN: &str = "comp-main";

/// A `side`-pixel solid of `color` at 24 frames a second, Radio Waves with `waves` over the
/// one-wave square above, drawn at `frame`.
fn shot(side: usize, color: [f64; 3], frame: i32, waves: &J) -> String {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let mut parameters = json!({
        "producer_point": [50, 50], "sides": 4, "interval": 1000, "expansion": 6,
        "orientation": 45, "direction": 0, "velocity": 0, "spin": 0, "lifespan": 1000,
        "opacity": 100, "fade_in_time": 0, "fade_out_time": 0, "start_width": 2, "end_width": 2,
        "profile": "square", "color": "#ffffff"
    });
    for (k, v) in waves.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    let n = frame + 1;
    json!({
        "schema_version": 0, "project_id": "proj-b951",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": side, "height": side, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": n, "work_area": {"start_frame": 0, "end_frame_exclusive": n},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "solid", "name": "art", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": n,
                "solid": {"color": color, "width": side, "height": side},
                "transform": {
                    "anchor": t(json!([side / 2, side / 2])), "position": t(json!([side / 2, side / 2])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "masks": [], "matte": null, "blend_mode": "normal",
                "effects": [
                    {"instance_id": "waves", "type_id": "core.radio_waves", "enabled": true, "parameters": parameters}
                ]
            }]
        }]
    })
    .to_string()
}

fn draw(side: usize, color: [f64; 3], frame: i32, waves: &J) -> WorkingBuffer {
    let loaded = persist::load_str(&shot(side, color, frame, waves)).expect("the shot reads");
    let mut log = FrameLog::new(8);
    render_frame(loaded.document.project(), &Id::new(MAIN), frame, std::path::Path::new("."), 256, &mut log)
        .expect("the frame draws")
}

/// How lit each pixel `(x, y)` is at frame 10 (the wave 60 to a corner): 0 black, 1 white.
fn lit(waves: &J, frame: i32, at: &[(usize, usize)]) -> Vec<f64> {
    let f = draw(200, [0.0; 3], frame, waves);
    at.iter().map(|&(x, y)| (f.pixel(x, y)[0] as f64 * 100.0).round() / 100.0).collect()
}

fn show(v: &[f64]) -> String {
    v.iter().map(|a| format!("{a}")).collect::<Vec<_>>().join(" ")
}

/// Each of `v` lit (above 0.5) where `want` is 1, and dark (below 0.01) where it is 0.
fn is(v: &[f64], want: &[u8]) -> bool {
    v.iter().zip(want).all(|(a, w)| if *w == 1 { *a > 0.5 } else { *a < 0.01 })
}

/// A side's middle, the square's corner, a point on the diagonal 43.1 out (a circle touching
/// the middles), one 51.6 out (Curve Size 0.5's corner), and one on the side 21.5 out.
const AT: [(usize, usize); 5] = [(142, 100), (142, 142), (130, 130), (136, 136), (121, 100)];

#[test]
fn b951_radio_waves_shapes() {
    let mut t = Table::new(
        "radio_waves",
        "# D-1071: Radio Waves, Curve Size, Star, Star Depth and Frequency\n\nB-951. One white wave \
         on a black 200-pixel solid at frame 10: a square (Sides 4) turned so its sides stand \
         straight, 60 pixels from the middle to a corner and 42.43 to a side's middle, 2 pixels \
         wide. Each row reads five pixels' red, 0 black to 1 white; a check passes when a pixel meant to be lit is above 0.5 and one meant to be black is below 0.01: a side's middle (142, 100); the \
         corner (142, 142); the diagonal 43.1 out (130, 130), where a circle touching the sides' \
         middles passes; the diagonal 51.6 out (136, 136), where Curve Size 0.5's round corner \
         passes (30 to the shrunk corner and 21.2 more); and (121, 100), 21.5 out towards a side, \
         where a star of Star Depth -0.5 has its inner corner.\n",
    );
    t.heading("The polygon, Curve Size and Star");
    for (waves, want, what) in [
        (json!({}), [1, 1, 0, 0, 0], "defaults: the sharp square, as before"),
        (json!({"curve_size": 1}), [1, 0, 1, 0, 0], "Curve Size 1: a circle through the sides' middles"),
        (json!({"curve_size": 0.5}), [1, 0, 0, 1, 0], "Curve Size 0.5: the corner rounded, the side's middle kept"),
        (json!({"star": "on", "star_depth": -0.5}), [0, 1, 0, 0, 1], "Star on, Star Depth -0.5: the corner kept, the side pulled in to 21.2"),
        (json!({"star": "on", "star_depth": 0}), [1, 1, 0, 0, 0], "Star on, Star Depth 0: the square's own outline"),
        (json!({"star": "off", "star_depth": -0.5}), [1, 1, 0, 0, 0], "Star off: Star Depth does nothing"),
    ] {
        let v = lit(&waves, 10, &AT);
        t.row(what, &show(&v), is(&v, &want));
    }

    t.heading("Frequency, waves a second (the composition runs at 24)");
    // At frame 20 the first wave is 120 to a corner, 84.9 to a side's middle, so pixel 184 at
    // 84.5; one sent at frame 12 is 48 to a corner and 33.9 to a side's middle, pixel 133.
    let at = [(184, 100), (133, 100)];
    for (waves, want, what) in [
        (json!({}), [1, 0], "Frequency 0: Interval 1000 sends only the first wave"),
        (json!({"frequency": 2}), [1, 1], "Frequency 2: a wave every 12 frames, so a second sent at frame 12"),
        (json!({"frequency": 0.5}), [1, 0], "Frequency 0.5: every 48 frames, so still one"),
        (json!({"frequency": 2, "interval": 5}), [1, 1], "Frequency 2 with Interval 5: the Frequency rules"),
    ] {
        let v = lit(&waves, 20, &at);
        t.row(what, &show(&v), is(&v, &want));
    }

    t.heading("On the card");
    for (waves, frame, (x, y), what) in [
        (json!({"curve_size": 1}), 10, (130, 130), "Curve Size 1, the circle's pixel (it hands the wave to the CPU)"),
        (json!({"star": "on", "star_depth": -0.5}), 10, (121, 100), "the star's inner corner (handed to the CPU)"),
        (json!({"frequency": 2}), 20, (133, 100), "Frequency 2's second wave (drawn on the card)"),
    ] {
        let (answer, ok) = match Gpu::new() {
            Ok(mut gpu) => {
                let loaded = persist::load_str(&shot(200, [0.0; 3], frame, &waves)).unwrap();
                let mut log = FrameLog::new(8);
                let mut cache = CelCache::viewer();
                let (px, w, _) = preview::preview_frame_srgb8(
                    loaded.document.project(), &Id::new(MAIN), frame, std::path::Path::new("."),
                    PreviewQuality::Full, 256, &mut log, &mut cache, &mut gpu,
                )
                .expect("the frame draws");
                let red = px[(y * w + x) * 4];
                (format!("red {red} of 255"), red > 200)
            }
            Err(_) => ("no usable card here".to_string(), true),
        };
        t.row(&format!("the viewer with the card lights {what}"), &answer, ok);
    }

    t.heading("Saved and refused");
    let saved = |waves: &J| {
        let loaded = persist::load_str(&shot(200, [0.0; 3], 10, waves)).unwrap();
        let j: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
        j["compositions"][0]["layers"][0]["effects"][0]["parameters"].clone()
    };
    let p = saved(&json!({}));
    t.row(
        "a file without them saves without them",
        "none of the four",
        ["curve_size", "star", "star_depth", "frequency"].iter().all(|k| p.get(k).is_none()),
    );
    let p = saved(&json!({"curve_size": 0.78, "star": "on", "star_depth": -0.5, "frequency": 1.9}));
    t.row(
        "set ones are written",
        &format!("curve {}, star {}, depth {}, frequency {}", p["curve_size"], p["star"], p["star_depth"], p["frequency"]),
        p["curve_size"] == 0.78 && p["star"] == "on" && p["star_depth"] == -0.5 && p["frequency"] == 1.9,
    );
    for (waves, what) in [
        (json!({"star": "yes"}), "an unknown Star word"),
        (json!({"curve_size": 2}), "a Curve Size above 1"),
        (json!({"frequency": -1}), "a Frequency below 0"),
    ] {
        let bad = persist::load_str(&shot(200, [0.0; 3], 10, &waves)).unwrap();
        let valid = bad.document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].is_valid();
        t.row(&format!("{what} switches the effect off"), &format!("runs: {valid}"), !valid);
    }

    picture();
    t.row(
        "shapes.png: the tutorial's look, white waves on orange, Frequency 4 at 24 frames a second: a sharp hexagon, Curve Size 0.78, and a five-point star at Star Depth -0.5",
        "drawn",
        true,
    );
    t.finish("B-951_radio_waves_shapes_table.md");
}

/// Three 320-pixel shots side by side at frame 60.
fn picture() {
    let s = 320;
    let base = json!({"sides": 6, "orientation": 0, "expansion": 4, "frequency": 4, "start_width": 4, "end_width": 4});
    let with = |extra: J| {
        let mut j = base.clone();
        for (k, v) in extra.as_object().unwrap() {
            j[k] = v.clone();
        }
        j
    };
    let shots = [
        with(json!({})),
        with(json!({"curve_size": 0.78})),
        with(json!({"sides": 5, "star": "on", "star_depth": -0.5})),
    ];
    let gap = 16;
    let ow = 3 * s + 2 * gap;
    let mut out = WorkingBuffer::opaque(ow, s);
    for (n, waves) in shots.iter().enumerate() {
        let b = draw(s, [1.0, 0.5, 0.0], 60, waves);
        for y in 0..s {
            for x in 0..s {
                let p = b.pixel(x, y);
                let o = (y * ow + n * (s + gap) + x) * 4;
                let d = out.data_mut();
                d[o..o + 4].copy_from_slice(&p);
            }
        }
    }
    let dir = repo("verification/B-951 pictures");
    std::fs::create_dir_all(&dir).unwrap();
    png_out::write_rgba(&dir.join("shapes.png"), ow, s, OutputDepth::Eight, &[], &out.to_srgb8_straight()).unwrap();
}
