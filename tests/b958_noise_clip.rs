//! B-958: D-1078, Noise's Clip Result Values (lHQvi94dpvM 4:20 on, f80 4:00 off; G16 of the
//! wave 12 report). FX-NOISE-019 to 022 are run by b62; this draws a black-to-white ramp with a
//! strong grain, clipped and wrapped, for the eye, and checks the card hands a wrap to the CPU.

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
const W: usize = 240;
const H: usize = 60;

/// A white solid `W` by `H` ramped black to white left to right, then Noise `noise`.
fn shot(noise: &J) -> String {
    let t = |v: J| json!({"base": v, "keyframes": []});
    json!({
        "schema_version": 0, "project_id": "proj-b958",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": W, "height": H, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "solid", "name": "art", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 1,
                "solid": {"color": [1, 1, 1], "width": W, "height": H},
                "transform": {
                    "anchor": t(json!([W / 2, H / 2])), "position": t(json!([W / 2, H / 2])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "masks": [], "matte": null, "blend_mode": "normal",
                "effects": [
                    {"instance_id": "ramp", "type_id": "core.gradient", "enabled": true,
                     "parameters": {"shape": "linear", "start": [0, 50], "end": [100, 50],
                                    "start_color": "#000000", "end_color": "#ffffff",
                                    "start_opacity": 100, "end_opacity": 100, "blend": "normal"}},
                    {"instance_id": "grain", "type_id": "core.noise", "enabled": true, "parameters": noise}
                ]
            }]
        }]
    })
    .to_string()
}

fn noise(amount: f64, clip: Option<&str>) -> J {
    let mut j = json!({"amount": amount, "mode": "mono", "seed": 0, "animate": "off"});
    if let Some(c) = clip {
        j["clip"] = json!(c);
    }
    j
}

fn draw(n: &J) -> WorkingBuffer {
    let loaded = persist::load_str(&shot(n)).expect("the shot reads");
    let mut log = FrameLog::new(8);
    render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 256, &mut log)
        .expect("the frame draws")
}

/// The red channel's sRGB level, 0 to 255, at each pixel.
fn levels(b: &WorkingBuffer) -> Vec<u8> {
    b.to_srgb8_straight().chunks_exact(4).map(|p| p[0]).collect()
}

#[test]
fn b958_noise_clip() {
    let mut t = Table::new(
        "noise",
        "# D-1078: Noise, Clip Result Values\n\nB-958. A 240 by 60 ramp, black at the left to \
         white at the right, with a mono Noise of amount 40 (each channel moves up to 0.2 through \
         the sRGB curve). Clip on holds a channel inside black and white, as Noise always did; \
         Clip off wraps one that would pass white round to dark and one that would pass black \
         round to light, as After Effects' 8-bit Noise does.\n",
    );
    let before = draw(&noise(40.0, None));
    let on = draw(&noise(40.0, Some("on")));
    let off = draw(&noise(40.0, Some("off")));
    let (n, f) = (levels(&on), levels(&off));

    t.heading("Clip on is Noise as it was");
    t.row(
        "a Noise with no clip word and one with Clip on draw the same, bit for bit",
        if before.data() == on.data() { "the same" } else { "differs" },
        before.data() == on.data(),
    );

    t.heading("Clip off wraps only near black and white");
    let middle = (0..H).flat_map(|y| (80..160).map(move |x| y * W + x));
    let same_middle = middle.clone().all(|i| f[i] == n[i]);
    t.row(
        "in the middle third of the ramp, where the grain cannot pass black or white, every pixel is Clip on's",
        &format!("{} of {} the same", middle.clone().filter(|&i| f[i] == n[i]).count(), middle.count()),
        same_middle,
    );
    let right = (0..H).flat_map(|y| (W - 12..W).map(move |x| y * W + x));
    let clipped_white = right.clone().filter(|&i| n[i] == 255).count();
    let wrapped_dark = right.clone().filter(|&i| f[i] < 128).count();
    t.row(
        "at the white end, Clip on holds many pixels at exactly white (255)",
        &format!("{clipped_white} of {}", right.clone().count()),
        clipped_white > 100,
    );
    t.row(
        "and Clip off turns those round to dark (below 128) instead: the speckle of dark dots on white",
        &format!("{wrapped_dark} of {}", right.count()),
        wrapped_dark > 100 && wrapped_dark <= clipped_white,
    );
    let left: Vec<usize> = (0..H).flat_map(|y| (0..12).map(move |x| y * W + x)).collect();
    let clipped_black = left.iter().filter(|&&i| n[i] == 0).count();
    let wrapped_light = left.iter().filter(|&&i| f[i] > 128).count();
    t.row(
        "at the black end, Clip on holds many at exactly black (0), and Clip off turns them round to light (above 128)",
        &format!("{clipped_black} held, {wrapped_light} light, of {}", left.len()),
        clipped_black > 100 && wrapped_light > 100 && wrapped_light <= clipped_black,
    );
    let ramp = draw(&noise(0.0, None));
    let r = levels(&ramp);
    let row = H / 2 * W;
    t.row(
        "under the grain is the ramp: amount 0, the middle row's level at the left, middle and right",
        &format!("{}, {}, {}", r[row], r[row + W / 2], r[row + W - 1]),
        r[row] < 10 && r[row + W - 1] > 245 && (100..160).contains(&r[row + W / 2]),
    );

    t.heading("On the card");
    let card = match Gpu::new() {
        Ok(mut gpu) => {
            let loaded = persist::load_str(&shot(&noise(40.0, Some("off")))).unwrap();
            let mut log = FrameLog::new(8);
            let mut cache = CelCache::viewer();
            let (px, _, _) = preview::preview_frame_srgb8(
                loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."),
                PreviewQuality::Full, 256, &mut log, &mut cache, &mut gpu,
            )
            .expect("the frame draws");
            let worst = px.chunks_exact(4).zip(&f).map(|(p, &w)| (p[0] as i32 - w as i32).abs()).max().unwrap();
            (format!("largest difference {worst} level(s)"), worst <= 1)
        }
        Err(_) => ("no usable card here".to_string(), true),
    };
    t.row("the viewer with the card draws Clip off as the processor does (it hands a wrap to the CPU)", &card.0, card.1);

    picture(&on, &off);
    t.row(
        "clip.png: the ramp with Noise 40, Clip on above, Clip off below",
        "drawn",
        true,
    );
    t.finish("B-958_noise_clip_table.md");
}

/// The two shots one above the other, a dark grey gap between.
fn picture(on: &WorkingBuffer, off: &WorkingBuffer) {
    let gap = 8;
    let oh = 2 * H + gap;
    let mut out = WorkingBuffer::opaque(W, oh);
    for (n, b) in [on, off].iter().enumerate() {
        for y in 0..H {
            for x in 0..W {
                let p = b.pixel(x, y);
                let o = ((n * (H + gap) + y) * W + x) * 4;
                out.data_mut()[o..o + 4].copy_from_slice(&p);
            }
        }
    }
    for y in H..H + gap {
        for x in 0..W {
            let o = (y * W + x) * 4;
            out.data_mut()[o..o + 3].copy_from_slice(&[0.2, 0.05, 0.05]);
        }
    }
    let dir = repo("verification/B-958 pictures");
    std::fs::create_dir_all(&dir).unwrap();
    png_out::write_rgba(&dir.join("clip.png"), W, oh, OutputDepth::Eight, &[], &out.to_srgb8_straight()).unwrap();
}
