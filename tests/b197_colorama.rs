//! B-197: D-316, Colorama, reduced.
//!
//! P-26, tutorial 3 (Colorful Glitch) repaints a picture's brightnesses with After Effects'
//! Colorama; here there was none. Worked on small buffers by hand, premultiplied linear light.
//! A pixel's phase is read from its encoded (sRGB) colour; the ring's colours are encoded too.

use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{apply_stack, Effect, EffectInstance};
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::WorkingBuffer;

/// Document 21's sRGB decode.
fn lin(v: f32) -> f32 {
    if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
}

/// One pixel per encoded straight colour, at the given alpha, premultiplied.
fn row(px: &[([f32; 3], f32)]) -> WorkingBuffer {
    let mut b = WorkingBuffer::transparent(px.len(), 1);
    for (x, (c, a)) in px.iter().enumerate() {
        b.data_mut()[x * 4..][..4].copy_from_slice(&[lin(c[0]) * a, lin(c[1]) * a, lin(c[2]) * a, *a]);
    }
    b
}

fn grey(v: f32) -> ([f32; 3], f32) {
    ([v, v, v], 1.0)
}

fn run(b: &WorkingBuffer, effect: Effect) -> WorkingBuffer {
    let mut b = b.clone();
    apply_stack(&mut b, &[EffectInstance::new(Id::new("b197"), effect)], |at, _, why| panic!("bypassed at {at}: {why:?}"));
    b
}

fn colorama(get: &str, (shift, reps): (f64, f64), stops: f64, ring: [&str; 5], blend: f64) -> Effect {
    Effect::Colorama {
        get_phase: get.into(),
        layer: J::from(""),
        fit: "stretch".into(),
        phase_shift: shift,
        cycle_repetitions: reps,
        stops,
        color_1: ring[0].into(),
        color_2: ring[1].into(),
        color_3: ring[2].into(),
        color_4: ring[3].into(),
        color_5: ring[4].into(),
        blend_with_original: blend,
        map: None,
        add_phase_from: get.into(),
        add_mode: "wrap".into(),
        interpolate: "on".into(),
        opacity_1: 100.0,
        opacity_2: 100.0,
        opacity_3: 100.0,
        opacity_4: 100.0,
        opacity_5: 100.0,
        modify: "all".into(),
        modify_alpha: "off".into(),
        change_empty: "off".into(),
        matching_mode: "off".into(),
        matching_color: "#ffffff".into(),
        matching_tolerance: 15.0,
        matching_softness: 0.0,
        mask_layer: J::from(""),
        masking_mode: "luminance".into(),
        composite_over: "on".into(),
        mask_map: None,
    }
}

/// Black to white and back: the ring of two colours.
const BW: [&str; 5] = ["#000000", "#ffffff", "#ff0000", "#00ff00", "#0000ff"];

fn near(got: [f32; 4], want: [f32; 4], what: &str) {
    for c in 0..4 {
        assert!((got[c] - want[c]).abs() < 1e-4, "{what}: {got:?} against {want:?}");
    }
}

fn g(v: f32) -> [f32; 4] {
    [lin(v), lin(v), lin(v), 1.0]
}

#[test]
fn the_ring_is_read_by_phase_shift_and_repetitions() {
    let b = row(&[grey(0.1), grey(0.25), grey(0.75), grey(0.0)]);
    // Two colours, black then white, running back into black: phase p lands 2p round it.
    let out = run(&b, colorama("intensity", (0., 1.), 2., BW, 0.));
    near(out.pixel(0, 0), g(0.2), "0.1 is a fifth of the way from black to white");
    near(out.pixel(1, 0), g(0.5), "0.25 is half way from black to white");
    near(out.pixel(2, 0), g(0.5), "0.75 is half way from white back to black");
    near(out.pixel(3, 0), g(0.0), "black is the first colour");
    // Half a turn: 0.1 reads as 0.6, a fifth of the way from white back to black.
    near(run(&b, colorama("intensity", (180., 1.), 2., BW, 0.)).pixel(0, 0), g(0.8), "phase shift 180");
    near(run(&b, colorama("intensity", (-180., 1.), 2., BW, 0.)).pixel(0, 0), g(0.8), "and -180 the same");
    near(run(&b, colorama("intensity", (360., 1.), 2., BW, 0.)).pixel(0, 0), g(0.2), "a whole turn changes nothing");
    near(run(&b, colorama("intensity", (0., 2.), 2., BW, 0.)).pixel(0, 0), g(0.4), "twice round");
    near(run(&b, colorama("intensity", (0., 0.), 2., BW, 0.)).pixel(1, 0), g(0.0), "no repetitions is all the first colour");
    // Blend: half way back toward the pixel's own colour, in linear light.
    let half = (lin(0.2) + lin(0.1)) / 2.0;
    near(run(&b, colorama("intensity", (0., 1.), 2., BW, 50.)).pixel(0, 0), [half, half, half, 1.0], "blend 50");
    let all = run(&b, colorama("intensity", (0., 1.), 2., BW, 100.));
    for x in 0..4 {
        near(all.pixel(x, 0), b.pixel(x, 0), "blend 100 is the original");
    }
}

#[test]
fn the_stops_the_phase_source_and_alpha() {
    // After Effects' start here: five hues; 0 is the first, 0.2 the second, 0.1 half between.
    let hues = ["#ff0000", "#ccff00", "#00ff66", "#0066ff", "#cc00ff"];
    let b = row(&[grey(0.0), grey(0.2), grey(0.1)]);
    let out = run(&b, colorama("intensity", (0., 1.), 5., hues, 0.));
    near(out.pixel(0, 0), [1., 0., 0., 1.], "the first colour");
    near(out.pixel(1, 0), [lin(0.8), 1., 0., 1.], "the second colour");
    near(out.pixel(2, 0), [lin(0.9), lin(0.5), 0., 1.], "half between, mixed in encoded values");
    // Three stops: 0.5 is half way round three, half between the second and the third.
    let out = run(&row(&[grey(0.5)]), colorama("intensity", (0., 1.), 3., hues, 0.));
    near(out.pixel(0, 0), [lin(0.4), lin(1.0), lin(0.2), 1.], "three stops");
    near(run(&row(&[grey(0.5)]), colorama("intensity", (0., 1.), 3.9, hues, 0.)).pixel(0, 0), out.pixel(0, 0), "taken whole");

    // The phase read from each source, on one colour at half alpha.
    let c = row(&[([0.2, 0.6, 0.9], 0.5)]);
    let luma = {
        let l = 0.2126 * lin(0.2) + 0.7152 * lin(0.6) + 0.0722 * lin(0.9);
        if l <= 0.0031308 { 12.92 * l } else { 1.055 * l.powf(1.0 / 2.4) - 0.055 }
    };
    for (get, p) in [("red", 0.2), ("green", 0.6), ("blue", 0.9), ("alpha", 0.5), ("intensity", (0.2 + 0.6 + 0.9) / 3.0), ("luminance", luma)] {
        // Two stops black and white: phase p is 2p round, a triangle.
        let v = if p < 0.5 { 2.0 * p } else { 2.0 - 2.0 * p };
        let got = run(&c, colorama(get, (0., 1.), 2., BW, 0.)).pixel(0, 0);
        near(got, [lin(v) * 0.5, lin(v) * 0.5, lin(v) * 0.5, 0.5], get);
    }
    // A clear pixel stays clear, and the effect never changes the size.
    let clear = WorkingBuffer::transparent(2, 2);
    let out = run(&clear, colorama("intensity", (90., 1.), 5., hues, 0.));
    assert_eq!(out.data(), clear.data(), "nothing to repaint");
}

#[test]
fn wrong_settings_are_refused_and_the_effect_is_saved_as_read() {
    for (bad, says) in [
        // D-381 adds hue, lightness, saturation, value and zero.
        (colorama("brightness", (0., 1.), 2., BW, 0.), "intensity, luminance, red, green, blue, alpha, hue"),
        (colorama("red", (0., 1.), 2., ["#000000", "white", "#000000", "#000000", "#000000"], 0.), "colour 2"),
        (colorama("red", (0., 1.), 6., BW, 0.), "stops"),
        (colorama("red", (0., 101.), 2., BW, 0.), "cycle repetitions"),
    ] {
        assert!(!bad.is_valid(), "{says} refused");
        assert!(bad.why_invalid().contains(says), "{}", bad.why_invalid());
    }

    let params = json!({"get_phase": "luminance", "layer": "map", "fit": "tile", "phase_shift": 90, "cycle_repetitions": 2.5,
        "stops": 3, "color_1": "#102030", "color_2": "#405060", "color_3": "#708090", "color_4": "#a0b0c0", "color_5": "#d0e0f0",
        "blend_with_original": 25});
    let loaded = persist::load_str(&project(params.clone(), 0.0).to_string()).unwrap_or_else(|d| panic!("{}", d.message));
    let saved: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
    assert_eq!(saved["compositions"][0]["layers"][1]["effects"][0]["parameters"], params, "written as read");
}

/// A 4 by 1 composition: a hidden white "map" solid and a black "card" with Colorama.
fn project(params: J, card: f64) -> J {
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
        "project_id": "proj-b197",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": "comp-main", "name": "comp-main", "width": 4, "height": 1, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 1,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["map", "card"],
            "layers": [
                solid("map", false, 1.0, json!([])),
                solid("card", true, card, json!([{"instance_id": "fx-1", "type_id": "core.colorama", "enabled": true, "parameters": params}])),
            ]
        }]
    })
}

#[test]
fn another_layer_adds_its_phase() {
    let draw = |layer: &str| {
        let params = json!({"get_phase": "intensity", "layer": layer, "fit": "stretch", "phase_shift": 0, "cycle_repetitions": 0.5,
            "stops": 2, "color_1": "#000000", "color_2": "#ffffff", "color_3": "#000000", "color_4": "#000000", "color_5": "#000000",
            "blend_with_original": 0});
        let loaded = persist::load_str(&project(params, 0.0).to_string()).expect("the project reads");
        let mut log = FrameLog::new(3);
        let drawn = render_frame(loaded.document.project(), &Id::new("comp-main"), 0, &std::env::temp_dir(), 64, &mut log).expect("it draws");
        assert!(log.finish().is_empty(), "nothing to report");
        drawn.to_srgb8_straight()[..4].to_vec()
    };
    // Black's own phase is 0; half a repetition of black to white is black.
    assert_eq!(draw(""), [0, 0, 0, 255], "no layer added");
    // The white map adds 1: half of 1 is half way round, the second colour, white.
    assert_eq!(draw("map"), [255, 255, 255, 255], "the white layer's phase added");
}
