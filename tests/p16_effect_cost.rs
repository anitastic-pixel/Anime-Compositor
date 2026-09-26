//! P-16: what each effect costs on one 1920x1080 cel, and the fingerprint of what it drew.
//!
//! Ignored, because it is a timer: `cargo test --release --test p16_effect_cost -- --ignored
//! --nocapture` prints one table row per case. Nothing is asserted about time (document 12). The
//! SHA-256 of every result is printed beside its time, so a run before a change and a run after
//! it show whether the change moved a single bit of any effect's output, which is the claim
//! `verification/P-16_effect_cost.md` rests on.

use std::time::Instant;

use anime_compositor::color::srgb_to_linear;
use anime_compositor::effects::{apply_stack, Effect, EffectInstance};
use anime_compositor::model::Id;
use anime_compositor::WorkingBuffer;

const W: usize = 1920;
const H: usize = 1080;

/// Flat 8-bit colours as a drawing program paints them, premultiplied into working values.
fn paint(data: &mut [f32], i: usize, rgb: [u8; 3], a: f32) {
    for c in 0..3 {
        data[i + c] = srgb_to_linear(rgb[c] as f32 / 255.0) * a;
    }
    data[i + 3] = a;
}

/// A character cel: a figure in three flat colours with a dark line round it and a soft edge
/// on one side, on nothing. About a fifth of the frame shows.
fn character() -> WorkingBuffer {
    let mut b = WorkingBuffer::transparent(W, H);
    let d = b.data_mut();
    for y in 0..H {
        for x in 0..W {
            let (dx, dy) = (x as f32 - 960.0, y as f32 - 540.0);
            let r = (dx * dx + dy * dy).sqrt();
            let i = (y * W + x) * 4;
            if r < 380.0 {
                let rgb = match (x / 97 + y / 83) % 3 {
                    0 => [0xf6, 0xd6, 0xbe],
                    1 => [0x50, 0x23, 0x8c],
                    _ => [0xff, 0xe6, 0xfa],
                };
                paint(d, i, rgb, 1.0);
            } else if r < 390.0 {
                paint(d, i, [0x0f, 0x0c, 0x14], if dx > 0.0 { (390.0 - r) / 10.0 } else { 1.0 });
            }
        }
    }
    b
}

/// A background plate: opaque everywhere, a gradient sky over flat ground.
fn background() -> WorkingBuffer {
    let mut b = WorkingBuffer::transparent(W, H);
    let d = b.data_mut();
    for y in 0..H {
        for x in 0..W {
            let i = (y * W + x) * 4;
            let rgb = if y > 700 {
                [0x3c, 0xb4, 0x28]
            } else {
                [(40 + y / 5) as u8, (120 + y / 8) as u8, 0xe6 - (x / 40) as u8]
            };
            paint(d, i, rgb, 1.0);
        }
    }
    b
}

fn glow(radius: f64, based_on: &str) -> Effect {
    Effect::Glow {
        based_on: based_on.to_string(),
        threshold: 60.0,
        colors: vec!["#50238c".to_string()],
        tolerance: 0.0,
        radius,
        intensity: 1.0,
        operation: "add".to_string(),
        tint: String::new(),
    }
}

fn cases() -> Vec<(&'static str, Effect)> {
    vec![
        ("Exposure +1", Effect::Exposure { stops: 1.0 }),
        ("Tint 50%", Effect::Tint { color: [1.0, 0.5, 0.25], amount: 0.5 }),
        ("Gaussian Blur 4", Effect::GaussianBlur { sigma_px: 4.0 }),
        ("Gaussian Blur 10", Effect::GaussianBlur { sigma_px: 10.0 }),
        ("Gaussian Blur 40", Effect::GaussianBlur { sigma_px: 40.0 }),
        ("Line Smooth", Effect::LineSmooth { softness: 50.0, threshold: 16.0 }),
        (
            "Selective Colour Blur 12",
            Effect::SelectiveColorBlur {
                blur: 12.0,
                colors: vec!["#50238c".into(), "#f6d6be".into(), "#3cb428".into()],
                tolerance: 0.0,
            },
        ),
        (
            "Selective Colour Blur 100",
            Effect::SelectiveColorBlur {
                blur: 100.0,
                colors: vec!["#50238c".into(), "#f6d6be".into(), "#3cb428".into()],
                tolerance: 0.0,
            },
        ),
        ("Glow 10", glow(10.0, "bright")),
        ("Glow 50", glow(50.0, "bright")),
        ("Glow 250", glow(250.0, "bright")),
        ("Glow 50, chosen colour", glow(50.0, "colors")),
        // P-17: the batch of seven, D-91 to D-97.
        (
            "Line Recolour",
            Effect::LineRecolor {
                colors: vec!["#0f0c14".into()],
                tolerance: 0.0,
                new_color: "#ff0000".into(),
            },
        ),
        ("Directional Blur 10", Effect::DirectionalBlur { direction: 30.0, length: 10.0 }),
        ("Directional Blur 100", Effect::DirectionalBlur { direction: 30.0, length: 100.0 }),
        (
            "Select Colour",
            Effect::SelectColor {
                colors: vec!["#50238c".into()],
                tolerance: 0.0,
                keep: "chosen".into(),
            },
        ),
        ("Line Width 3", line_width(3.0, "shape")),
        ("Line Width 10", line_width(10.0, "shape")),
        ("Line Width -3", line_width(-3.0, "shape")),
        ("Line Width 3, chosen colour", line_width(3.0, "colors")),
        ("Radial Blur spin 10", radial("spin", 10.0)),
        ("Radial Blur zoom 20", radial("zoom", 20.0)),
        ("Bloom 20", bloom("none")),
        ("Bloom 20, star 60", bloom("star")),
        ("Colour Key rgb", color_key("rgb")),
        ("Colour Key hue", color_key("hue")),
    ]
}

fn line_width(width: f64, based_on: &str) -> Effect {
    Effect::LineWidth {
        width,
        based_on: based_on.into(),
        colors: vec!["#0f0c14".into()],
        tolerance: 0.0,
    }
}

fn radial(kind: &str, amount: f64) -> Effect {
    Effect::RadialBlur { kind: kind.into(), amount, center: [50.0, 50.0] }
}

fn bloom(streaks: &str) -> Effect {
    Effect::Bloom {
        threshold: 80.0,
        radius: 20.0,
        intensity: 1.0,
        streaks: streaks.into(),
        length: 60.0,
        angle: 0.0,
    }
}

fn color_key(match_by: &str) -> Effect {
    Effect::ColorKey {
        colors: vec!["#3cb428".into()],
        tolerance: 20.0,
        softness: 20.0,
        match_by: match_by.into(),
    }
}

fn fingerprint(b: &WorkingBuffer) -> String {
    let bytes: Vec<u8> = b.data().iter().flat_map(|v| v.to_bits().to_le_bytes()).collect();
    anime_compositor::sha256::hex(&bytes)[..16].to_string()
}

#[test]
#[ignore]
fn p16_effect_cost() {
    println!("| Effect | Cel | Median ms | Fastest ms | Runs | Result SHA-256 (first 16) |");
    println!("|---|---|---|---|---|---|");
    for (cel_name, cel) in [("character", character()), ("background", background())] {
        for (name, effect) in cases() {
            let stack = [EffectInstance::new(Id::new("p16"), effect)];
            let mut times = Vec::new();
            let mut print = String::new();
            let started = Instant::now();
            while times.len() < 7 && (times.len() < 2 || started.elapsed().as_secs() < 20) {
                let mut b = cel.clone();
                let t = Instant::now();
                apply_stack(&mut b, &stack, |_, _, why| panic!("{name} bypassed: {why:?}"));
                times.push(t.elapsed().as_secs_f64() * 1000.0);
                print = fingerprint(&b);
            }
            let mut sorted = times.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            println!(
                "| {name} | {cel_name} | {:.1} | {:.1} | {} | `{print}` |",
                sorted[sorted.len() / 2],
                sorted[0],
                sorted.len()
            );
        }
    }
}
