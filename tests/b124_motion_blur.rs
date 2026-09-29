//! B-124b: motion blur on the processor, against D-188 and ADR-019.
//!
//! Writes `verification/B-124b_motion_blur_table.md`.
//!
//! Every expected time, value, pixel and point is `Fixtures/motion_blur/expected_motion_blur.json`,
//! written by `tools/motion_blur_reference.py` before this code existed and printed in document 25
//! as FX-MB-001 to 050. Nothing here is a snapshot of a run. The pictures for the playtest are
//! drawn at the end into `verification/B-124 pictures/`.

mod effect_table;

use std::fs;

use effect_table::{largest_difference, same_json, saved, Table, MAIN};
use serde_json::Value as J;

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{self, plan_frame_for_card};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, MotionBlur, Prop};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{png_out, OutputDepth, WorkingBuffer};

fn shutter(v: &J) -> MotionBlur {
    MotionBlur {
        enabled: v["enabled"].as_bool().unwrap(),
        shutter_angle: v["shutter_angle"].as_f64().unwrap(),
        shutter_phase: v["shutter_phase"].as_f64().unwrap(),
        samples: v["samples"].as_u64().unwrap() as u32,
    }
}

fn numbers(v: &J) -> Vec<f64> {
    v.as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect()
}

/// The largest difference between two lists of numbers, or infinity when their lengths differ.
fn apart(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return f64::INFINITY;
    }
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0, f64::max)
}

/// FX-MB-011's file with the bar's switch set to `on`.
fn with_switch(t: &Table, file: &str, on: bool) -> Document {
    let mut document = t.load(file).document;
    document
        .apply(Command::SetLayerMotionBlur {
            composition: Id::new(MAIN),
            layer_id: Id::new("bar"),
            value: on,
        })
        .expect("the bar's switch");
    document
}

fn draft(document: &Document, root: &std::path::Path, frame: i32) -> WorkingBuffer {
    preview::preview_frame_cached(
        document.project(),
        &Id::new(MAIN),
        frame,
        root,
        PreviewQuality::Draft,
        64,
        &mut FrameLog::new(8),
        &mut CelCache::none(),
    )
    .expect("the draft frame draws")
}

/// The playtest's shot, 960 by 540: a dark ground, an orange card crossing it 60 pixels a frame
/// while it turns 30 degrees a frame, and a white bar turning 60 degrees a frame in place. Both
/// have their switch on; `bar_switch` turns the bar's off.
fn shot(shutter: J, bar_switch: bool) -> Document {
    let solid = |id: &str, color: [f64; 3], (w, h): (u32, u32), from: [f64; 2], to: [f64; 2], turn: f64, blur: bool| {
        serde_json::json!({
            "id": id, "kind": "solid", "name": id, "enabled": true, "locked": false,
            "in_frame": 0, "out_frame": 12,
            "solid": {"color": color, "width": w, "height": h},
            "transform": {
                "anchor": {"base": [w as f64 / 2.0, h as f64 / 2.0], "keyframes": []},
                "position": {"base": from, "keyframes": [
                    {"frame": 0, "value": from, "interp": "linear"},
                    {"frame": 12, "value": to, "interp": "linear"}]},
                "scale": {"base": [100, 100], "keyframes": []},
                "rotation": {"base": 0, "keyframes": [
                    {"frame": 0, "value": 0, "interp": "linear"},
                    {"frame": 12, "value": turn, "interp": "linear"}]},
                "opacity": {"base": 1, "keyframes": []}
            },
            "mask": null, "matte": null, "blend_mode": "normal", "effects": [], "motion_blur": blur
        })
    };
    let project = serde_json::json!({
        "schema_version": 0, "project_id": "proj-b124-shot",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": "shot", "width": 960, "height": 540, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 12, "work_area": {"start_frame": 0, "end_frame_exclusive": 12},
            "layer_order": ["ground", "card", "bar"],
            "layers": [
                solid("ground", [0.02, 0.03, 0.06], (960, 540), [480.0, 270.0], [480.0, 270.0], 0.0, false),
                solid("card", [1.0, 0.35, 0.05], (160, 100), [120.0, 250.0], [840.0, 250.0], 360.0, true),
                solid("bar", [1.0, 1.0, 1.0], (240, 16), [700.0, 400.0], [700.0, 400.0], 720.0, bar_switch),
            ],
            "motion_blur": shutter
        }]
    });
    persist::load_str(&project.to_string()).expect("the shot reads").document
}

#[test]
fn b124_motion_blur() {
    let mut t = Table::new(
        "motion_blur",
        "# B-124b: motion blur, on the processor\n\nD-188 and ADR-019, accepted on 2026-09-28 by \
         the owner's words \"I approve of motion blur plan\". Every expected time, value, pixel \
         and point is `Fixtures/motion_blur/expected_motion_blur.json`, written by \
         `tools/motion_blur_reference.py` before this code existed and printed in document 25 as \
         FX-MB-001 to 050. Times and values are held to 1e-12, points to 1e-9 and pixels to \
         1e-6; the answer is the largest difference.\n",
    );
    let expected: J = serde_json::from_str(
        &fs::read_to_string(t.root.join("expected_motion_blur.json")).unwrap(),
    )
    .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let point_tolerance = expected["point_tolerance"].as_f64().unwrap();
    let pixel_tolerance = expected["pixel_tolerance"].as_f64().unwrap();

    t.heading("FX-MB-001 to 005: the moments inside a frame's shutter");
    for (name, case) in expected["times"].as_object().unwrap() {
        let frame = case["frame"].as_i64().unwrap() as i32;
        let got = shutter(&case["motion_blur"]).times(frame);
        let d = apart(&got, &numbers(&case["times"]));
        t.row(
            &format!("{name}: {}", case["says"].as_str().unwrap()),
            &format!("{} moments, largest difference {d:.1e}", got.len()),
            d <= tolerance,
        );
    }

    t.heading("FX-MB-006 to 009: a property read between frames");
    let template: J =
        serde_json::from_str(&fs::read_to_string(t.root.join("fx_mb_011.json")).unwrap()).unwrap();
    for (name, case) in expected["values"].as_object().unwrap() {
        // The property is put on the bar, as its rotation when it is a number and as its
        // position when it is a pair, and read back through the file format.
        let two = case["property"]["base"].is_array();
        let (key, prop) = if two { ("position", Prop::Position) } else { ("rotation", Prop::Rotation) };
        let mut file = template.clone();
        file["compositions"][0]["layers"][0]["transform"][key] = case["property"].clone();
        let loaded = persist::load_str(&file.to_string()).expect("the property reads");
        let comp = loaded.document.project().composition(&Id::new(MAIN)).unwrap();
        let property = comp.layer(&Id::new("bar")).unwrap().transform.get(prop).unwrap();
        let mut got = Vec::new();
        let mut want = Vec::new();
        for (time, value) in numbers(&case["times"]).into_iter().zip(case["values"].as_array().unwrap()) {
            let v = property.value_at_time(time);
            if two {
                let (x, y) = v.as_vec2().unwrap();
                got.extend([x, y]);
                want.extend(numbers(value));
            } else {
                got.push(v.as_scalar().unwrap());
                want.push(value.as_f64().unwrap());
            }
        }
        let d = apart(&got, &want);
        t.row(
            &format!("{name}: {}", case["says"].as_str().unwrap()),
            &format!("largest difference {d:.1e}"),
            d <= tolerance,
        );
    }

    t.heading("FX-MB-010 to 028: pictures (20 by 1 pixels, 24 frames a second)");
    for (name, case) in expected["cases"].as_object().unwrap() {
        let loaded = t.load(case["project"].as_str().unwrap());
        for (frame, pixels) in case["frames"].as_object().unwrap() {
            let frame: i32 = frame.parse().unwrap();
            let d = largest_difference(&t.render(&loaded.document, frame, 64), pixels);
            t.row(
                &format!("{name} frame {frame}: {}", case["says"].as_str().unwrap()),
                &format!("largest difference {d:.1e}"),
                d <= pixel_tolerance,
            );
        }
    }

    t.heading("What the pictures claim beyond their numbers");
    let on = t.render(&t.load("fx_mb_010.json").document, 1, 64);
    let off = t.render(&with_switch(&t, "fx_mb_010.json", false), 1, 64);
    let same = on.data().iter().zip(off.data()).all(|(a, b)| a.to_bits() == b.to_bits());
    t.row(
        "FX-MB-010: the still bar with its switch on is the picture with it off, bit for bit",
        if same { "the same bits" } else { "differs" },
        same,
    );
    let original: J =
        serde_json::from_str(&fs::read_to_string(t.root.join("fx_mb_014.json")).unwrap()).unwrap();
    let kept = saved(&t.load("fx_mb_014.json"));
    let neither = kept["compositions"][0].get("motion_blur").is_none()
        && kept["compositions"][0]["layers"][0].get("motion_blur").is_none();
    t.row(
        "FX-MB-014: the file from before motion blur, saved again, holds what it held and neither field",
        if same_json(&kept, &original) && neither { "the same, neither field" } else { "differs" },
        same_json(&kept, &original) && neither,
    );
    for file in ["fx_mb_011.json", "fx_mb_016.json", "fx_mb_025.json"] {
        let original: J =
            serde_json::from_str(&fs::read_to_string(t.root.join(file)).unwrap()).unwrap();
        let same = same_json(&saved(&t.load(file)), &original);
        t.row(
            &format!("{file} opened and saved holds what it held, both switches included"),
            if same { "the same" } else { "differs" },
            same,
        );
    }
    let blurred = |file: &str| {
        let loaded = t.load(file);
        let plan = plan_frame_for_card(
            loaded.document.project(),
            &Id::new(MAIN),
            1,
            &t.root,
            PreviewQuality::Full,
            &mut FrameLog::new(8),
            &mut CelCache::none(),
        )
        .unwrap();
        plan.layers.iter().any(|l| l.motion_blur)
    };
    let marks = ["fx_mb_011.json", "fx_mb_025.json", "fx_mb_010.json"].map(blurred);
    t.row(
        "FX-MB-011's frame is marked for the processor, which the card leaves to the CPU with GPU_PREVIEW_ON_CPU",
        &format!("{}", marks[0]),
        marks[0],
    );
    t.row(
        "FX-MB-025's frame, whose matte alone is blurred, is marked too",
        &format!("{}", marks[1]),
        marks[1],
    );
    t.row(
        "FX-MB-010's still bar, drawn once, is not marked: the card may draw it",
        &format!("{}", marks[2]),
        !marks[2],
    );

    // Draft uses the same moments: FX-MB-011 at Draft is the average of four sharp Draft
    // frames, one with the bar standing still where each moment puts it.
    let moving = t.load("fx_mb_011.json").document;
    let comp = moving.project().composition(&Id::new(MAIN)).unwrap();
    let times = comp.motion_blur.times(1);
    let position = comp.layer(&Id::new("bar")).unwrap().transform.get(Prop::Position).unwrap();
    let mut sum: Vec<f64> = Vec::new();
    for &time in &times {
        let (x, y) = position.value_at_time(time).as_vec2().unwrap();
        let mut file = template.clone();
        file["compositions"][0]["layers"][0]["transform"]["position"] =
            serde_json::json!({"base": [x, y], "keyframes": []});
        let still = persist::load_str(&file.to_string()).unwrap().document;
        let one = draft(&still, &t.root, 1);
        sum.resize(one.data().len(), 0.0);
        for (s, v) in sum.iter_mut().zip(one.data()) {
            *s += *v as f64;
        }
    }
    let average: Vec<f64> = sum.iter().map(|s| s / times.len() as f64).collect();
    let got: Vec<f64> = draft(&moving, &t.root, 1).data().iter().map(|v| *v as f64).collect();
    let d = apart(&got, &average);
    t.row(
        "Draft uses the same four moments: FX-MB-011 at Draft is the average of the bar drawn still at each",
        &format!("largest difference {d:.1e}"),
        d <= pixel_tolerance,
    );

    t.heading("FX-MB-050: where a corner lands at each moment (1920 by 1080)");
    let points = &expected["points"]["FX-MB-050"];
    let loaded = t.load(points["project"].as_str().unwrap());
    let comp = loaded.document.project().composition(&Id::new(MAIN)).unwrap();
    let parent = comp.layer(&Id::new("P")).unwrap();
    let child = comp.layer(&Id::new("C")).unwrap();
    for row in points["rows"].as_array().unwrap() {
        let frame = row["frame"].as_i64().unwrap() as i32;
        let time = row["time"].as_f64().unwrap();
        let listed = comp.motion_blur.times(frame).iter().any(|&m| (m - time).abs() <= tolerance);
        let rotation = parent.transform.get(Prop::Rotation).unwrap().value_at_time(time).as_scalar().unwrap();
        let (px, py) = child.transform.get(Prop::Position).unwrap().value_at_time(time).as_vec2().unwrap();
        let depth = compose::camera_at_time(comp, frame, time).unwrap().depth;
        let screen = compose::screen_transform_at(comp, &Id::new("C"), frame, time).unwrap();
        let (ax, ay) = screen.apply(0.0, 0.0);
        let (bx, by) = screen.apply(100.0, 0.0);
        let values = apart(
            &[rotation, px, py, depth],
            &[
                row["parent rotation"].as_f64().unwrap(),
                row["child position"][0].as_f64().unwrap(),
                row["child position"][1].as_f64().unwrap(),
                row["camera depth"].as_f64().unwrap(),
            ],
        );
        let corners = apart(
            &[ax, ay, bx, by],
            &[numbers(&row["(0, 0) on screen"]), numbers(&row["(100, 0) on screen"])].concat(),
        );
        t.row(
            &format!("frame {frame}, moment {time}: a moment of the frame; the parent's rotation, the child's position and the camera's depth; both corners on screen"),
            &format!("{listed}; {values:.1e}; {corners:.1e}"),
            listed && values <= tolerance && corners <= point_tolerance,
        );
    }

    t.heading("FX-MB-030 to 041: files that are refused");
    for (name, case) in expected["refused"].as_object().unwrap() {
        let got = persist::load(&t.root.join(case["project"].as_str().unwrap()));
        let code = got.as_ref().err().map(|d| d.id.as_str().to_string()).unwrap_or("opened".into());
        t.row(
            &format!("{name}: {}", case["says"].as_str().unwrap()),
            &code,
            code == case["code"].as_str().unwrap(),
        );
    }

    t.heading("The two commands");
    let mut document = t.load("fx_mb_011.json").document;
    let set = |mb: MotionBlur| Command::SetMotionBlur { composition: Id::new(MAIN), motion_blur: mb };
    let as_added = MotionBlur { enabled: true, ..MotionBlur::default() };
    for (what, mb) in [
        ("an angle above 720", MotionBlur { shutter_angle: 720.5, ..as_added }),
        ("an angle below 0", MotionBlur { shutter_angle: -1.0, ..as_added }),
        ("a phase below -360", MotionBlur { shutter_phase: -361.0, ..as_added }),
        ("one sample", MotionBlur { samples: 1, ..as_added }),
        ("sixty-five samples", MotionBlur { samples: 65, ..as_added }),
        ("an angle that is not a number", MotionBlur { shutter_angle: f64::NAN, ..as_added }),
    ] {
        let refused = document.apply(set(mb)).is_err();
        t.row(&format!("the composition's shutter with {what} is refused"), &format!("{refused}"), refused);
    }
    let taken = document.apply(set(as_added)).is_ok();
    let now = document.project().composition(&Id::new(MAIN)).unwrap().motion_blur;
    let undone = document.undo().is_some()
        && document.project().composition(&Id::new(MAIN)).unwrap().motion_blur.samples == 4;
    t.row(
        "shutter 180, phase -90, 16 samples is taken, and undo puts back the file's 4",
        &format!("taken {taken}, {} samples, undone {undone}", now.samples),
        taken && now.samples == 16 && undone,
    );
    let null = t.load("fx_mb_019.json").document.project().composition(&Id::new(MAIN)).unwrap()
        .layers_in_order().find(|l| l.kind == anime_compositor::model::LayerKind::Null)
        .map(|l| l.id.clone()).unwrap();
    let mut on_null = t.load("fx_mb_019.json").document;
    let refused = on_null
        .apply(Command::SetLayerMotionBlur { composition: Id::new(MAIN), layer_id: null, value: true })
        .is_err();
    t.row("the switch on a null is refused", &format!("{refused}"), refused);
    let off = with_switch(&t, "fx_mb_011.json", false);
    let sharp = largest_difference(
        &t.render(&off, 1, 64),
        &expected["cases"]["FX-MB-013"]["frames"]["1"],
    );
    t.row(
        "switching FX-MB-011's bar off draws FX-MB-013's sharp picture",
        &format!("largest difference {sharp:.1e}"),
        sharp <= pixel_tolerance,
    );

    t.heading("Pictures for the playtest, frame 5 of a 960 by 540 shot, in `verification/B-124 pictures/`");
    let dir = effect_table::repo("verification/B-124 pictures");
    fs::create_dir_all(&dir).unwrap();
    let shutter = |on: bool, angle: f64, phase: f64, samples: u32| {
        serde_json::json!({"enabled": on, "shutter_angle": angle, "shutter_phase": phase, "samples": samples})
    };
    let root = t.root.clone();
    let draw = |document: &Document, quality: PreviewQuality| {
        let mut log = FrameLog::new(8);
        let drawn = preview::preview_frame_cached(
            document.project(), &Id::new(MAIN), 5, &root, quality, 64, &mut log, &mut CelCache::none(),
        )
        .expect("the shot draws");
        let said: Vec<String> = log.finish().iter().map(|d| d.id.as_str().to_string()).collect();
        (drawn, said)
    };
    let (sharp, _) = draw(&shot(shutter(false, 180.0, -90.0, 16), true), PreviewQuality::Full);
    // On frame 5 the card is centred at (420, 250), and (360, 250) is just inside its left corner.
    // The bar turns about (700, 400) and stands at 300 degrees; (731, 305) is 100 pixels out at 288
    // degrees, which the sharp bar misses and the blurred one sweeps. Nothing crosses (20, 20).
    let at = |b: &WorkingBuffer, (x, y): (usize, usize)| b.data()[(y * 960 + x) * 4..(y * 960 + x) * 4 + 4].to_vec();
    for (name, what, document, quality) in [
        ("sharp", "the composition's switch off: both sharp", shot(shutter(false, 180.0, -90.0, 16), true), PreviewQuality::Full),
        ("as_added", "switched on as added, shutter 180, phase -90, 16 samples", shot(shutter(true, 180.0, -90.0, 16), true), PreviewQuality::Full),
        ("shutter_360", "shutter 360, phase -180, 32 samples: a whole frame of travel", shot(shutter(true, 360.0, -180.0, 32), true), PreviewQuality::Full),
        ("four_samples", "shutter 180 with only 4 samples: four copies can be seen", shot(shutter(true, 180.0, -90.0, 4), true), PreviewQuality::Full),
        ("bar_switch_off", "as added with the bar's own switch off: the card blurs, the bar is sharp", shot(shutter(true, 180.0, -90.0, 16), false), PreviewQuality::Full),
        ("draft", "as added at Draft, a quarter of the size each way", shot(shutter(true, 180.0, -90.0, 16), true), PreviewQuality::Draft),
    ] {
        let (drawn, said) = draw(&document, quality);
        let (w, h) = (drawn.width(), drawn.height());
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &drawn.to_srgb8_straight()).unwrap();
        if quality == PreviewQuality::Draft {
            t.row(&format!("{name}.png, {what}, draws cleanly at {w} by {h}"), &format!("{said:?}"), said.is_empty() && (w, h) == (240, 135));
            continue;
        }
        let card_moved = at(&drawn, (360, 250)) != at(&sharp, (360, 250));
        let bar_moved = at(&drawn, (731, 305)) != at(&sharp, (731, 305));
        let ground = at(&drawn, (20, 20)) == at(&sharp, (20, 20));
        let blurred = name != "sharp";
        t.row(
            &format!("{name}.png, {what}: draws cleanly; a point near the card's corner and one in the bar's sweep change only where blurred; the ground is untouched"),
            &format!("{said:?}; card {card_moved}, bar {bar_moved}, ground {ground}"),
            said.is_empty() && card_moved == blurred && bar_moved == (blurred && name != "bar_switch_off") && ground,
        );
    }

    t.finish("B-124b_motion_blur_table.md");
}
