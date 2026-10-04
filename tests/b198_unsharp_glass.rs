//! B-198: D-317, Unsharp Mask's Threshold on Sharpen, and CC Glass, reduced.
//!
//! P-26, tutorial 1 (Shockwave) bends the picture behind its ring with After Effects' CC Glass
//! and crisps it with Unsharp Mask. Worked on small buffers, premultiplied linear light.

use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{apply_stack, Effect, EffectInstance};
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::WorkingBuffer;

/// Document 21's sRGB decode and encode.
fn lin(v: f64) -> f64 {
    if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
}
fn enc(l: f64) -> f64 {
    if l <= 0.0031308 { 12.92 * l } else { 1.055 * l.powf(1.0 / 2.4) - 0.055 }
}

fn run(b: &WorkingBuffer, effect: Effect) -> WorkingBuffer {
    let mut b = b.clone();
    apply_stack(&mut b, &[EffectInstance::new(Id::new("b198"), effect)], |at, _, why| panic!("bypassed at {at}: {why:?}"));
    b
}

fn sharpen(threshold: f64) -> Effect {
    Effect::Sharpen { amount: 100.0, radius: 2.0, threshold }
}

/// An opaque 9 by 3 picture of encoded colours between 0.3 and 0.7, so nothing clamps.
fn picture() -> WorkingBuffer {
    let mut b = WorkingBuffer::transparent(9, 3);
    for (i, px) in b.data_mut().chunks_exact_mut(4).enumerate() {
        let v = [0.3, 0.32, 0.7, 0.68, 0.31, 0.5, 0.52, 0.69, 0.3][i % 9] + 0.01 * (i / 9) as f64;
        px.copy_from_slice(&[lin(v) as f32, lin(0.5) as f32, lin(1.0 - v) as f32, 1.0]);
    }
    b
}

#[test]
fn a_threshold_leaves_the_small_differences_alone() {
    let b = picture();
    let none = run(&b, sharpen(0.0));
    assert_ne!(none.data(), b.data(), "it sharpens");
    // At amount 100 a sharpened channel is e + (e - eb), so e - eb is the change it made.
    let t = 0.1;
    let some = run(&b, sharpen(t * 255.0));
    let (mut kept, mut moved) = (0, 0);
    for i in 0..b.data().len() {
        if i % 4 == 3 {
            continue;
        }
        let e = enc(b.data()[i] as f64);
        let d = enc(none.data()[i] as f64) - e;
        if d.abs() < t {
            assert_eq!(some.data()[i], b.data()[i], "sample {i}: a difference of {d} is under the threshold, kept");
            kept += 1;
        } else {
            assert_eq!(some.data()[i], none.data()[i], "sample {i}: a difference of {d} is sharpened as before");
            moved += 1;
        }
    }
    assert!(kept > 0 && moved > 0, "both kinds are checked: {kept} kept, {moved} moved");
    assert_eq!(run(&b, sharpen(255.0)).data(), b.data(), "the most threshold keeps everything");
}

#[allow(clippy::too_many_arguments)]
fn glass(layer: &str, property: &str, softness: f64, height: f64, displacement: f64, angle: f64, color: &str, intensity: f64) -> Effect {
    Effect::Glass {
        layer: J::from(layer),
        fit: "stretch".into(),
        property: property.into(),
        softness,
        height,
        displacement,
        light_angle: angle,
        light_color: color.into(),
        light_intensity: intensity,
        map: None,
    }
}

/// A 16 by 16 drawing: a soft-edged blue disc on clear.
fn disc() -> WorkingBuffer {
    let mut b = WorkingBuffer::transparent(16, 16);
    for (i, px) in b.data_mut().chunks_exact_mut(4).enumerate() {
        let (x, y) = ((i % 16) as f64 - 7.5, (i / 16) as f64 - 7.5);
        let a = (6.0 - (x * x + y * y).sqrt()).clamp(0.0, 1.0) as f32;
        px.copy_from_slice(&[0.1 * a, 0.2 * a, 0.6 * a, a]);
    }
    b
}

#[test]
fn glass_with_no_displacement_is_bevel_alpha_and_with_no_height_nothing() {
    let b = disc();
    let bevel = Effect::BevelAlpha { edge_thickness: 4.0, light_angle: 30.0, light_color: "#ffcc88".into(), light_intensity: 0.7 };
    let lit = run(&b, glass("", "alpha", 4.0, 100.0, 0.0, 30.0, "#ffcc88", 70.0));
    assert_ne!(lit.data(), b.data(), "it lights the edge");
    assert_eq!(lit.data(), run(&b, bevel).data(), "height 100, no displacement: Bevel Alpha at thickness 4");
    assert_eq!(run(&b, glass("", "alpha", 4.0, 0.0, 100.0, 30.0, "#ffffff", 100.0)).data(), b.data(), "height 0");
    assert_eq!(run(&b, glass("", "alpha", 4.0, 50.0, 0.0, 30.0, "#ffffff", 0.0)).data(), b.data(), "no push, no light");
}

#[test]
fn a_slope_bends_the_picture_by_the_worked_amount() {
    // One opaque row whose encoded red climbs 0.1 a pixel: with softness 0 the bump is that red,
    // its slope inside is 0.1 a pixel, times height 80 / 100 * 1.25 = 1, so a displacement of 10
    // reads one pixel to the right.
    let mut b = WorkingBuffer::transparent(8, 1);
    for (x, px) in b.data_mut().chunks_exact_mut(4).enumerate() {
        px.copy_from_slice(&[lin(0.1 * x as f64) as f32, lin(0.5) as f32, lin(0.5) as f32, 1.0]);
    }
    let out = run(&b, glass("", "red", 0.0, 80.0, 10.0, 0.0, "#ffffff", 0.0));
    let near = |got: [f32; 4], want: [f32; 4], what: &str| {
        assert!((0..4).all(|c| (got[c] - want[c]).abs() < 1e-6), "{what}: {got:?} against {want:?}");
    };
    for x in 1..7 {
        near(out.pixel(x, 0), b.pixel(x + 1, 0), &format!("pixel {x} shows pixel {}", x + 1));
    }
    // The first pixel's slope is (0.1 - 0) / 2, half a pixel: half way to the second.
    let half: Vec<f32> = (0..4).map(|c| (b.pixel(0, 0)[c] + b.pixel(1, 0)[c]) / 2.0).collect();
    near(out.pixel(0, 0), [half[0], half[1], half[2], half[3]], "pixel 0 half way to pixel 1");
    // The last's is (0 - 0.6) / 2, three pixels back.
    near(out.pixel(7, 0), b.pixel(4, 0), "pixel 7 shows pixel 4");
    // Turned round, the slope reads the other way.
    near(run(&b, glass("", "red", 0.0, -80.0, 10.0, 0.0, "#ffffff", 0.0)).pixel(3, 0), b.pixel(2, 0), "height below 0 dips");
}

#[test]
fn wrong_settings_are_refused_and_both_are_saved_as_read() {
    for (bad, says) in [
        (glass("", "hue", 1.0, 1.0, 1.0, 0.0, "#ffffff", 1.0), "intensity, luminance, red, green, blue or alpha"),
        (glass("", "alpha", 1.0, 1.0, 1.0, 0.0, "white", 1.0), "light colour"),
        (glass("", "alpha", 1.0, 101.0, 1.0, 0.0, "#ffffff", 1.0), "height"),
        (Effect::Sharpen { amount: 100.0, radius: 1.0, threshold: 256.0 }, "threshold"),
    ] {
        assert!(!bad.is_valid(), "{says} refused");
        assert!(bad.why_invalid().contains(says), "{}", bad.why_invalid());
    }

    let glass_params = json!({"layer": "map", "fit": "tile", "property": "luminance", "softness": 6, "height": -40,
        "displacement": 75, "light_angle": 30, "light_color": "#ffeecc", "light_intensity": 60});
    for params in [
        json!([{"instance_id": "fx-1", "type_id": "core.glass", "enabled": true, "parameters": glass_params}]),
        json!([{"instance_id": "fx-1", "type_id": "core.sharpen", "enabled": true, "parameters": {"amount": 100, "radius": 1, "threshold": 12}}]),
        json!([{"instance_id": "fx-1", "type_id": "core.sharpen", "enabled": true, "parameters": {"amount": 100, "radius": 1}}]),
    ] {
        let loaded = persist::load_str(&project(params.clone()).to_string()).unwrap_or_else(|d| panic!("{}", d.message));
        let saved: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
        assert_eq!(saved["compositions"][0]["layers"][1]["effects"][0]["parameters"], params[0]["parameters"], "written as read");
    }
}

/// A 4 by 1 composition: a hidden black "map" solid and a "card" carrying the effects.
fn project(effects: J) -> J {
    let solid = |id: &str, enabled: bool, color: f64, effects: J| {
        json!({
            "id": id, "kind": "solid", "name": id, "enabled": enabled, "locked": false,
            "in_frame": 0, "out_frame": 1,
            "transform": {
                "anchor": {"base": [0, 0], "keyframes": []},
                "position": {"base": [0, 0], "keyframes": []},
                "scale": {"base": [100, 100], "keyframes": []},
                "rotation": {"base": 0, "keyframes": []},
                "opacity": {"base": 1, "keyframes": []}
            },
            "mask": null, "matte": null, "blend_mode": "normal", "effects": effects,
            "solid": {"color": [color, color, color], "width": 4, "height": 1}
        })
    };
    json!({
        "schema_version": 0,
        "project_id": "proj-b198",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": "comp-main", "name": "comp-main", "width": 4, "height": 1, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 1,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["map", "card"],
            "layers": [solid("map", false, 0.0, json!([])), solid("card", true, 0.5, effects)]
        }]
    })
}

#[test]
fn another_layer_is_the_bump() {
    let draw = |layer: &str| {
        let params = json!({"layer": layer, "fit": "stretch", "property": "luminance", "softness": 0, "height": 100,
            "displacement": 0, "light_angle": 90, "light_color": "#ffffff", "light_intensity": 100});
        let effects = json!([{"instance_id": "fx-1", "type_id": "core.glass", "enabled": true, "parameters": params}]);
        let loaded = persist::load_str(&project(effects).to_string()).expect("the project reads");
        let mut log = FrameLog::new(3);
        let drawn = render_frame(loaded.document.project(), &Id::new("comp-main"), 0, &std::env::temp_dir(), 64, &mut log).expect("it draws");
        assert!(log.finish().is_empty(), "nothing to report");
        drawn.to_srgb8_straight()
    };
    let plain = {
        let loaded = persist::load_str(&project(json!([])).to_string()).expect("the project reads");
        let mut log = FrameLog::new(3);
        render_frame(loaded.document.project(), &Id::new("comp-main"), 0, &std::env::temp_dir(), 64, &mut log).expect("it draws").to_srgb8_straight()
    };
    // The card's own luminance falls off at its sides, so they are lit and shaded.
    assert_ne!(draw(""), plain, "the card's own bump");
    // The black map is flat, no bump at all: the card is as it was.
    assert_eq!(draw("map"), plain, "the black layer's bump");
}
