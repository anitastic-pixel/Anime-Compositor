//! B-195: D-312, Solid Composite, and D-313, Channel Blur.
//!
//! P-26, tutorials 2 (Energy Ball) and 3 (Colorful Glitch) use After Effects' Solid Composite
//! and Channel Blur; here there were neither. Worked on small buffers by hand, premultiplied
//! linear light, with `g` the linear value of #808080.

use serde_json::json;

use anime_compositor::effects::{apply_stack, Effect, EffectInstance};
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::WorkingBuffer;

fn buffer(w: usize, h: usize, px: impl Fn(usize, usize) -> [f32; 4]) -> WorkingBuffer {
    let mut b = WorkingBuffer::transparent(w, h);
    for y in 0..h {
        for x in 0..w {
            b.data_mut()[(y * w + x) * 4..][..4].copy_from_slice(&px(x, y));
        }
    }
    b
}

fn run(b: &WorkingBuffer, effect: Effect) -> WorkingBuffer {
    let mut b = b.clone();
    apply_stack(&mut b, &[EffectInstance::new(Id::new("b195"), effect)], |at, _, why| panic!("bypassed at {at}: {why:?}"));
    b
}

fn solid(source_opacity: f64, color: &str, opacity: f64, blend: &str) -> Effect {
    Effect::SolidComposite { source_opacity, color: color.into(), opacity, blend: blend.into() }
}

fn channels(s: [f64; 4], edges: &str, dimensions: &str) -> Effect {
    Effect::ChannelBlur {
        red_blurriness: s[0],
        green_blurriness: s[1],
        blue_blurriness: s[2],
        alpha_blurriness: s[3],
        edges: edges.into(),
        dimensions: dimensions.into(),
        units: "sigma".into(),
    }
}

fn gaussian(sigma_px: f64, edges: &str, dimensions: &str) -> Effect {
    Effect::GaussianBlur { sigma_px, edges: edges.into(), dimensions: dimensions.into(), units: "sigma".into() }
}

fn near(got: [f32; 4], want: [f32; 4], what: &str) {
    for c in 0..4 {
        assert!((got[c] - want[c]).abs() < 1e-5, "{what}: {got:?} against {want:?}");
    }
}

/// Document 21's sRGB decode.
fn lin(v: f32) -> f32 {
    if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
}

#[test]
fn solid_composite_lays_the_layer_on_the_colour() {
    // Opaque red and half-covering red.
    let b = buffer(2, 1, |x, _| if x == 0 { [1., 0., 0., 1.] } else { [0.5, 0., 0., 0.5] });
    let g = lin(128.0 / 255.0);

    let out = run(&b, solid(100., "#ffffff", 100., "normal"));
    near(out.pixel(0, 0), [1., 0., 0., 1.], "as it starts, opaque red stays");
    near(out.pixel(1, 0), [1., 0.5, 0.5, 1.], "half red over white");
    let out = run(&b, solid(0., "#ffffff", 100., "normal"));
    near(out.pixel(0, 0), [1., 1., 1., 1.], "the layer at 0% is only the colour");
    let out = run(&b, solid(100., "#808080", 0., "normal"));
    near(out.pixel(1, 0), [0.5, 0., 0., 0.5], "the colour at 0% leaves the layer as it was");
    let out = run(&b, solid(0., "#808080", 50., "normal"));
    near(out.pixel(0, 0), [g * 0.5, g * 0.5, g * 0.5, 0.5], "grey at half");
    near(run(&b, solid(50., "#ffffff", 100., "normal")).pixel(0, 0), [1., 0.5, 0.5, 1.], "red at half over white");

    near(run(&b, solid(100., "#808080", 100., "multiply")).pixel(0, 0), [g, 0., 0., 1.], "red multiplied by grey");
    near(run(&b, solid(100., "#808080", 100., "screen")).pixel(0, 0), [1., g, g, 1.], "red screened on grey");
    near(run(&b, solid(100., "#808080", 100., "add")).pixel(0, 0), [1., g, g, 1.], "red added to grey");
    assert_eq!(run(&b, solid(100., "#ffffff", 100., "normal")).width(), 2, "it does not grow the layer");

    for (bad, says) in [(solid(100., "#ffffff", 100., "overlay"), "blend"), (solid(100., "white", 100., "normal"), "colour"), (solid(101., "#ffffff", 100., "normal"), "source opacity")] {
        assert!(!bad.is_valid(), "{says} refused");
        assert!(bad.why_invalid().contains(says), "{}", bad.why_invalid());
    }
}

/// A 9 x 7 picture with colour and alpha that both vary.
fn varied() -> WorkingBuffer {
    buffer(9, 7, |x, y| {
        let a = if (2..7).contains(&x) && (1..6).contains(&y) { 1.0 } else { ((x * 3 + y * 5) % 7) as f32 / 7.0 };
        let c = [((x * 7 + y) % 5) as f32 / 4.0, ((x + y * 3) % 4) as f32 / 3.0, ((x * y) % 3) as f32 / 2.0];
        [c[0] * a, c[1] * a, c[2] * a, a]
    })
}

#[test]
fn channel_blur_with_four_the_same_is_blur() {
    let b = varied();
    assert_eq!(run(&b, channels([0.; 4], "transparent", "both")).data(), b.data(), "no blur changes nothing");
    for (edges, dims) in [("transparent", "both"), ("repeat", "both"), ("transparent", "horizontal"), ("repeat", "vertical")] {
        let ours = run(&b, channels([2.5; 4], edges, dims));
        let blur = run(&b, gaussian(2.5, edges, dims));
        assert_eq!((ours.width(), ours.height()), (blur.width(), blur.height()), "{edges} {dims}: the same size");
        assert_eq!(ours.data(), blur.data(), "{edges} {dims}: the same bits");
    }
}

#[test]
fn channel_blur_blurs_each_channel_by_its_own() {
    // Opaque, red on the left four columns, green throughout.
    let b = buffer(8, 3, |x, _| [if x < 4 { 1. } else { 0. }, 0.5, 0., 1.]);
    let out = run(&b, channels([1.5, 0., 0., 0.], "repeat", "both"));
    for y in 0..3 {
        for x in 0..8 {
            let (p, q) = (out.pixel(x, y), b.pixel(x, y));
            assert_eq!([p[1], p[2], p[3]], [q[1], q[2], q[3]], "green, blue and alpha stay at ({x}, {y})");
        }
    }
    let r: Vec<f32> = (0..8).map(|x| out.pixel(x, 1)[0]).collect();
    assert!(r[3] < 1.0 && r[4] > 0.0, "red is soft across the line: {r:?}");
    assert!((r[3] + r[4] - 1.0).abs() < 1e-5, "and even about it: {r:?}");
    assert!(r.windows(2).all(|w| w[0] >= w[1]), "and falls off: {r:?}");

    // Only the alpha blurred, a drawing of one colour: its edges go soft in its own colour,
    // with no dark rim, so it is Blur's picture; the layer grows by Blur's reach.
    let square = buffer(6, 6, |x, y| if (1..5).contains(&x) && (1..5).contains(&y) { [0.8, 0.3, 0.1, 1.] } else { [0.; 4] });
    let ours = run(&square, channels([0., 0., 0., 3.], "transparent", "both"));
    let blur = run(&square, gaussian(3., "transparent", "both"));
    assert_eq!((ours.width(), ours.height()), (blur.width(), blur.height()), "grows as Blur");
    for (o, b) in ours.data().iter().zip(blur.data()) {
        assert!((o - b).abs() < 1e-5, "the same picture as Blur");
    }

    for (bad, says) in [(channels([0.; 4], "wrap", "both"), "edges"), (channels([0.; 4], "transparent", "diagonal"), "dimensions"), (channels([0., 501., 0., 0.], "transparent", "both"), "green blurriness")] {
        assert!(!bad.is_valid(), "{says} refused");
        assert!(bad.why_invalid().contains(says), "{}", bad.why_invalid());
    }

    let mut draft = channels([2., 4., 6., 8.], "transparent", "both");
    draft.scale_distances(|v| v / 2.0);
    assert_eq!(draft, channels([1., 2., 3., 4.], "transparent", "both"), "a half-size draft halves all four");
}

#[test]
fn both_are_saved_and_read_back() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("verification/B-08a_project.json");
    let mut file: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let solid = json!({"source_opacity": 80, "color": "#203040", "opacity": 60, "blend": "screen"});
    let chan = json!({"red_blurriness": 3, "green_blurriness": 0, "blue_blurriness": 8, "alpha_blurriness": 2, "edges": "repeat", "dimensions": "horizontal"});
    let layer = &mut file["compositions"][0]["layers"][0];
    layer["effects"] = json!([
        {"instance_id": "s", "type_id": "core.solid_composite", "enabled": true, "parameters": solid},
        {"instance_id": "c", "type_id": "core.channel_blur", "enabled": true, "parameters": chan},
    ]);
    let loaded = persist::load_str(&file.to_string()).unwrap_or_else(|d| panic!("{}", d.message));
    let effects = &loaded.document.project().compositions[0].layers_in_order().next().unwrap().effects;
    assert_eq!(effects[0].effect, self::solid(80., "#203040", 60., "screen"));
    assert_eq!(effects[1].effect, channels([3., 0., 8., 2.], "repeat", "horizontal"));
    let saved: serde_json::Value = serde_json::from_str(&persist::to_json(loaded.document.project(), &Default::default())).unwrap();
    let saved = &saved["compositions"][0]["layers"][0]["effects"];
    assert_eq!(saved[0]["parameters"], solid, "Solid Composite written as read");
    assert_eq!(saved[1]["parameters"], chan, "Channel Blur written as read");
}
