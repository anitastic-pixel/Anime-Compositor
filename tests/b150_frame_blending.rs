//! B-150b: Time Stretch, Frame Mix and Drawing Dissolve in the core, against D-216 and ADR-020.
//!
//! Writes `verification/B-150_frame_blending_table.md`, and pictures in
//! `verification/B-150 pictures/`.
//!
//! Every expected number is `Fixtures/frame_blending/expected_frame_blending.json`, written by
//! `tools/frame_blending_reference.py` before this code existed and printed in document 25 as
//! FX-FBLEND-001 to 067. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;

use effect_table::{same_json, saved, Table, MAIN};
use serde_json::Value as J;

use anime_compositor::command::{Command, Document, Target};
use anime_compositor::compose::{plan_frame, render_frame};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Interp, Layer, Prop, Value};
use anime_compositor::{persist, png_out, OutputDepth};

fn json(t: &Table, file: &str) -> J {
    serde_json::from_str(&fs::read_to_string(t.root.join(file)).unwrap()).unwrap()
}

/// FX-FBLEND-012's project with its layer `bar` given these times and position keys at `keys`
/// (x the key's own frame), in a composition long enough for all of them.
fn timed(t: &Table, in_frame: i32, out_frame: i32, offset: i32, stretch: f64, keys: &[i32]) -> Document {
    let mut p = json(t, "fx_fblend_012.json");
    let comp = &mut p["compositions"][0];
    comp["duration_frames"] = J::from(100);
    comp["work_area"]["end_frame_exclusive"] = J::from(100);
    let layer = &mut comp["layers"][0];
    layer["in_frame"] = J::from(in_frame);
    layer["out_frame"] = J::from(out_frame);
    layer["source_offset_frames"] = J::from(offset);
    layer["time_stretch"] = J::from(stretch);
    layer["transform"]["position"]["keyframes"] = keys
        .iter()
        .map(|k| serde_json::json!({ "frame": k, "value": [k, 0], "interp": "linear" }))
        .collect();
    persist::load_str(&p.to_string()).expect("the made-up project reads").document
}

fn bar(document: &Document) -> &Layer {
    document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("bar")).unwrap()
}

fn stored(document: &Document) -> Vec<i32> {
    let layer = bar(document);
    layer.transform.get(Prop::Position).unwrap().keyframes().iter().map(|k| k.frame).collect()
}

fn layer_cmd(which: &str, value: f64) -> Command {
    let (composition, layer_id) = (Id::new(MAIN), Id::new("bar"));
    match which {
        "stretch" => Command::SetLayerTimeStretch { composition, layer_id, value },
        "dissolve" => Command::SetDrawingDissolve { composition, layer_id, frames: value as u32 },
        _ => Command::SetLayerFrameBlend { composition, layer_id, value: value != 0.0 },
    }
}

fn close(a: f64, b: &J, tolerance: f64) -> bool {
    (a - b.as_f64().unwrap()).abs() <= tolerance
}

/// The codes a frame's log holds, each once, in order.
fn codes(log: FrameLog) -> Vec<String> {
    let mut out: Vec<String> = log.finish().iter().map(|d| d.id.as_str().to_string()).collect();
    out.sort();
    out.dedup();
    out
}

const BALL: (usize, usize) = (48, 32);

/// A made-up drawing of a ball, the first of three on its way across: an orange disc with a
/// dark outline, `x` its middle, on nothing.
fn ball(x: f64, y: f64) -> Vec<u8> {
    let (w, h) = BALL;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for py in 0..h {
        for px in 0..w {
            let r = (px as f64 + 0.5 - x).hypot(py as f64 + 0.5 - y);
            bytes.extend(if r < 6.0 {
                [250, 150, 40, 255]
            } else if r < 8.0 {
                [60, 30, 20, 255]
            } else {
                [0, 0, 0, 0]
            });
        }
    }
    bytes
}

#[test]
fn b150_frame_blending() {
    let mut t = Table::new(
        "frame_blending",
        "# B-150: Time Stretch, Frame Mix and Drawing Dissolve\n\nD-216 and ADR-020, accepted on \
         2026-09-29 with the owner's answers: a stretched layer's keys stretch with it, as in After \
         Effects; the Drawing Dissolve works whatever the composition's switch says; speeding up \
         mixes only the two nearest frames; and the composition's switch is off until it is turned \
         on. Every expected number is `Fixtures/frame_blending/expected_frame_blending.json`, \
         written by `tools/frame_blending_reference.py` before this code existed and printed in \
         document 25 as FX-FBLEND-001 to 067. Each frame is 8 by 1 pixels; the answer is the \
         largest difference over all its samples, against the catalogue's pixel tolerance of \
         1e-6.\n",
    );
    let expected = json(&t, "expected_frame_blending.json");
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let pixel_tolerance = expected["pixel_tolerance"].as_f64().unwrap();

    t.heading("FX-FBLEND-001 to 008: where in its source a layer is, and where its keys are read");
    for (name, case) in expected["times"].as_object().unwrap() {
        let mut p = json(&t, "fx_fblend_012.json");
        let layer = &mut p["compositions"][0]["layers"][0];
        layer["in_frame"] = case["in_frame"].clone();
        layer["out_frame"] = J::from(1000);
        layer["source_offset_frames"] = case["source_offset_frames"].clone();
        layer["time_stretch"] = case["time_stretch"].clone();
        let document = persist::load_str(&p.to_string()).expect("the made-up project reads").document;
        let layer = bar(&document);
        for row in case["rows"].as_array().unwrap() {
            let n = row["n"].as_i64().unwrap() as i32;
            let time = layer.source_time(n).unwrap();
            let (f, w) = (time.floor(), time - time.floor());
            let u = layer.key_time(n as f64);
            t.row(
                &format!("{name} frame {n}: {}", case["says"].as_str().unwrap()),
                &format!("t {time}, f {f}, w {w}, u {u}"),
                close(time, &row["t"], tolerance)
                    && f as i64 == row["f"].as_i64().unwrap()
                    && close(w, &row["w"], tolerance)
                    && close(u, &row["u"], tolerance),
            );
        }
    }

    t.heading("FX-FBLEND-010 to 039: the frames, and what each frame says");
    for (name, case) in expected["cases"].as_object().unwrap() {
        let says = case["says"].as_str().unwrap();
        let file = case["project"].as_str().unwrap();
        let loaded = t.load(file);
        for (frame, pixels) in case["frames"].as_object().unwrap() {
            let n: i32 = frame.parse().unwrap();
            let mut log = FrameLog::new(64);
            let got = render_frame(loaded.document.project(), &Id::new(MAIN), n, &t.root, 64, &mut log)
                .unwrap_or_else(|d| panic!("{file} frame {n} renders: {}", d.message));
            let d = effect_table::largest_difference(&got, pixels);
            let said = codes(log);
            let mut want: Vec<String> = case["diagnostics"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|x| x["frame"].as_i64() == Some(n as i64))
                .map(|x| x["code"].as_str().unwrap().to_string())
                .collect();
            want.dedup();
            t.row(
                &format!("{name} frame {n}: {says}"),
                &format!("largest difference {d:.1e}, says {said:?}"),
                d <= pixel_tolerance && said == want,
            );
        }
        let on_open: Vec<&str> = loaded.warnings.iter().map(|d| d.id.as_str()).collect();
        t.row(&format!("{name}: opening it warns of nothing"), &format!("{on_open:?}"), on_open.is_empty());
    }

    t.heading("The file");
    let mut kept = Vec::new();
    for (_, case) in expected["cases"].as_object().unwrap() {
        let file = case["project"].as_str().unwrap();
        if file != "fx_fblend_024.json" {
            kept.push(file);
        }
    }
    // D-216: a field at its default reads as absent and is saved so (FX-FBLEND-024), so the file
    // is compared with any written at its default taken out.
    for file in kept {
        let mut original = json(&t, file);
        for comp in original["compositions"].as_array_mut().unwrap() {
            if comp.get("frame_blending") == Some(&J::from(false)) {
                comp.as_object_mut().unwrap().remove("frame_blending");
            }
            for layer in comp["layers"].as_array_mut().unwrap() {
                let layer = layer.as_object_mut().unwrap();
                if layer.get("time_stretch").and_then(J::as_f64) == Some(100.0) {
                    layer.remove("time_stretch");
                }
                if layer.get("drawing_dissolve").and_then(J::as_f64) == Some(0.0) {
                    layer.remove("drawing_dissolve");
                }
            }
        }
        let same = same_json(&saved(&t.load(file)), &original);
        t.row(
            &format!("{file} opened and saved holds what it held, a stretch of 100 or a switch written false left out"),
            if same { "the same" } else { "differs" },
            same,
        );
    }
    let again = saved(&t.load("fx_fblend_024.json"));
    let comp = &again["compositions"][0];
    let quiet = comp.get("frame_blending").is_none() && comp["layers"][0].get("time_stretch").is_none();
    t.row(
        "FX-FBLEND-024: a file that writes stretch 100 and the switch false is saved without either",
        &format!("{}", comp["layers"][0].get("time_stretch").is_some() || comp.get("frame_blending").is_some()),
        quiet,
    );
    let fourteen = same_json(&saved(&t.load("fx_fblend_014.json")), &json(&t, "fx_fblend_014.json"));
    t.row(
        "FX-FBLEND-014: a file written before D-216 is saved with none of the four fields",
        if fourteen { "the same" } else { "differs" },
        fourteen,
    );
    for (name, case) in expected["refused"].as_object().unwrap() {
        let refused = persist::load(&t.root.join(case["project"].as_str().unwrap())).err();
        let code = refused.as_ref().map(|d| d.id.as_str().to_string());
        t.row(
            &format!("{name}: {} Refused as {}", case["says"].as_str().unwrap(), case["code"].as_str().unwrap()),
            &refused.as_ref().map_or("opened".to_string(), |d| format!("{}: {}", d.id.as_str(), d.message)),
            code.as_deref() == case["code"].as_str(),
        );
    }

    t.heading("FX-FBLEND-040 to 044: the Time Stretch command");
    for (name, case) in expected["commands"].as_object().unwrap() {
        let get = |k: &str| case[k].as_f64().unwrap();
        let mut document = timed(&t, get("in_frame") as i32, get("out_frame") as i32, 0, get("from"), &[]);
        let before = format!("{:?}", bar(&document));
        let taken = document.apply(layer_cmd("stretch", get("to"))).is_ok();
        let out = bar(&document).out_frame;
        document.undo();
        let back = format!("{:?}", bar(&document)) == before;
        t.row(
            &format!("{name}: {} One undo puts it back.", case["says"].as_str().unwrap()),
            &format!("{}, out point {out}, undone {}", if taken { "taken" } else { "refused" }, back),
            taken && out as f64 == get("out_after") && back,
        );
    }
    let mut document = timed(&t, 0, 12, 0, 100.0, &[]);
    let before = format!("{:?}", bar(&document));
    for (what, command) in [
        ("a stretch of 0.5", layer_cmd("stretch", 0.5)),
        ("a stretch of 10001", layer_cmd("stretch", 10001.0)),
        ("a stretch of -100", layer_cmd("stretch", -100.0)),
        ("a stretch that is no number", layer_cmd("stretch", f64::NAN)),
        ("a drawing dissolve of 101 frames", layer_cmd("dissolve", 101.0)),
    ] {
        let refused = document.apply(command).err();
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && format!("{:?}", bar(&document)) == before,
        );
    }
    let mut p = json(&t, "fx_fblend_060.json");
    p["compositions"][0]["layers"][1].as_object_mut().unwrap().remove("time_stretch");
    let mut solid = persist::load_str(&p.to_string()).expect("the solid's project reads").document;
    for (what, command) in [
        ("a stretch on a solid layer", Command::SetLayerTimeStretch { composition: Id::new(MAIN), layer_id: Id::new("card"), value: 200.0 }),
        ("frame mix on a solid layer", Command::SetLayerFrameBlend { composition: Id::new(MAIN), layer_id: Id::new("card"), value: true }),
        ("a drawing dissolve on a solid layer", Command::SetDrawingDissolve { composition: Id::new(MAIN), layer_id: Id::new("card"), frames: 1 }),
    ] {
        let refused = solid.apply(command).err();
        t.row(
            &format!("{what} is refused with a sentence"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some(),
        );
    }
    let mut document = t.load("fx_fblend_011.json").document;
    let before = t.render(&document, 3, 64);
    let steps = [
        ("the layer's frame mix switched on", layer_cmd("mix", 1.0)),
        ("the composition's frame blending switched on", Command::SetFrameBlending { composition: Id::new(MAIN), value: true }),
    ];
    let n = steps.len() + 1;
    for (what, command) in steps {
        let taken = document.apply(command).is_ok();
        t.row(&format!("on FX-FBLEND-011, {what} is taken"), if taken { "taken" } else { "refused" }, taken);
    }
    let now = t.render(&document, 3, 64);
    let loaded_12 = t.load("fx_fblend_012.json").document;
    t.row(
        "with its frame mix and the composition's switch on, FX-FBLEND-011 frame 3 is no longer a drawing held but a mix, as FX-FBLEND-012 frame 3 is",
        if now.data() == before.data() { "the same" } else { "changed" },
        now.data() != before.data() && now.data() == t.render(&loaded_12, 3, 64).data(),
    );
    let taken = document.apply(layer_cmd("dissolve", 2.0)).is_ok();
    let dissolved = t.render(&document, 1, 64).data() != t.render(&loaded_12, 1, 64).data();
    t.row(
        "then a drawing dissolve of 2 frames is taken, and frame 1 changes with it",
        &format!("{}, frame 1 {}", if taken { "taken" } else { "refused" }, if dissolved { "changed" } else { "the same" }),
        taken && dissolved,
    );
    for _ in 0..n {
        document.undo();
    }
    let same = t.render(&document, 3, 64).data() == before.data();
    t.row(
        &format!("undo {n} times: frame 3 is the frame it was"),
        if same { "byte-identical" } else { "differ" },
        same,
    );

    t.heading("FX-FBLEND-045 to 049: where keys play on a stretched layer");
    for (name, case) in expected["keys"].as_object().unwrap() {
        let in_frame = case["in_frame"].as_i64().unwrap() as i32;
        let stretch = case["time_stretch"].as_f64().unwrap();
        let want: Vec<i32> = case["stored"].as_array().unwrap().iter().map(|k| k.as_i64().unwrap() as i32).collect();
        let document = match case.get("playhead").and_then(J::as_i64) {
            // A key set at the playhead: stored where the build puts it, by the command a key
            // takes.
            Some(n) => {
                let mut d = timed(&t, in_frame, 40, 0, stretch, &[]);
                let at = bar(&d).key_frame_at(n as i32);
                d.apply(Command::SetKeyframe {
                    composition: Id::new(MAIN),
                    target: Target::Layer(Id::new("bar")),
                    prop: Prop::Position,
                    frame: at,
                    value: Value::Vec2(1.0, 0.0),
                    interp: Interp::Linear,
                    spatial: None,
                })
                .expect("the key is taken");
                d
            }
            // Keys stored at 100, then the layer stretched by the command.
            None if in_frame == 0 && stretch == 200.0 => {
                let mut d = timed(&t, in_frame, 12, 0, 100.0, &want);
                d.apply(layer_cmd("stretch", stretch)).expect("the stretch is taken");
                d
            }
            None => timed(&t, in_frame, 40, 0, stretch, &want),
        };
        let layer = bar(&document);
        let keys = stored(&document);
        let played: Vec<f64> = keys.iter().map(|k| layer.played_at(*k)).collect();
        let reads_back = keys.iter().zip(&played).all(|(k, p)| (layer.key_time(*p) - *k as f64).abs() <= tolerance);
        let ok = keys == want
            && played.len() == case["played"].as_array().unwrap().len()
            && played.iter().zip(case["played"].as_array().unwrap()).all(|(p, e)| close(*p, e, tolerance))
            && reads_back;
        t.row(
            &format!("{name}: {}", case["says"].as_str().unwrap()),
            &format!("stored at {keys:?}, playing at {played:?}, each read back at its own frame: {reads_back}"),
            ok,
        );
    }

    t.heading("FX-FBLEND-064 to 067: trimming a stretched layer's in point");
    for (name, case) in expected["trims"].as_object().unwrap() {
        let get = |k: &str| case[k].as_f64().unwrap();
        let keys: Vec<i32> = case["keys"].as_array().unwrap().iter().map(|k| k.as_i64().unwrap() as i32).collect();
        let (in_frame, out_frame) = (get("in_frame") as i32, get("out_frame") as i32);
        let mut document = timed(&t, in_frame, out_frame, get("source_offset_frames") as i32, get("time_stretch"), &keys);
        let before = format!("{:?}", bar(&document));
        let played: Vec<f64> = keys.iter().map(|k| bar(&document).played_at(*k)).collect();
        let result = document.apply(Command::TrimLayer {
            composition: Id::new(MAIN),
            layer_id: Id::new("bar"),
            in_frame: get("new_in") as i32,
            out_frame,
        })
        .map(|_| ())
        .map_err(|d| d.message.clone());
        let layer = bar(&document);
        let now = stored(&document);
        let (built, ok) = match &case["after"] {
            J::Null => (
                result.clone().err().unwrap_or("taken".to_string()),
                result.is_err() && format!("{layer:?}") == before,
            ),
            after => {
                let want: Vec<i32> = after["keys"].as_array().unwrap().iter().map(|k| k.as_i64().unwrap() as i32).collect();
                let still: Vec<f64> = now.iter().map(|k| layer.played_at(*k)).collect();
                (
                    format!("offset {}, keys at {now:?}, playing at {still:?} (were {played:?})", layer.source_offset_frames),
                    result.is_ok()
                        && layer.source_offset_frames as i64 == after["source_offset_frames"].as_i64().unwrap()
                        && now == want
                        && still == played,
                )
            }
        };
        t.row(&format!("{name}: {}", case["says"].as_str().unwrap()), &built, ok);
    }

    t.heading("The graphics card leaves a mixed or dissolved frame to the CPU");
    for (file, n, mixed, says) in [
        ("fx_fblend_012.json", 1, true, "a frame half way between two frames"),
        ("fx_fblend_012.json", 2, false, "a whole frame of the same layer"),
        ("fx_fblend_011.json", 1, false, "a stretched frame held, not mixed"),
        ("fx_fblend_030.json", 1, true, "a dissolved frame"),
        ("fx_fblend_021.json", 1, true, "a composition layer between two of its frames"),
    ] {
        let loaded = t.load(file);
        let mut log = FrameLog::new(8);
        let plan = plan_frame(loaded.document.project(), &Id::new(MAIN), n, &t.root, &mut log).unwrap();
        let got = plan.layers.iter().any(|l| l.mixed);
        t.row(
            &format!("{file} frame {n}, {says}: marked for the CPU {mixed}"),
            &got.to_string(),
            got == mixed,
        );
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_fblend_012.json", 3),
        ("fx_fblend_015.json", 4),
        ("fx_fblend_021.json", 3),
        ("fx_fblend_038.json", 1),
    ]);

    t.heading("Pictures: a made-up ball in three drawings on twos, in `verification/B-150 pictures/`");
    let dir = effect_table::repo("verification/B-150 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = BALL;
    for (i, x) in [10.0, 24.0, 38.0].into_iter().enumerate() {
        let path = dir.join(format!("ball_{:04}.png", i + 1));
        png_out::write_rgba(&path, w, h, OutputDepth::Eight, &[], &ball(x, 16.0)).unwrap();
    }
    let template = json(&t, "fx_fblend_012.json");
    let strip = |name: &str, stretch: f64, mix: bool, dissolve: u32, frames: i32| -> (Vec<Vec<u8>>, Vec<String>) {
        let mut p = template.clone();
        p["assets"][0]["frames"] = serde_json::json!({ "1": "ball_0001.png", "2": "ball_0002.png", "3": "ball_0003.png" });
        let comp = &mut p["compositions"][0];
        comp["width"] = J::from(w);
        comp["height"] = J::from(h);
        let layer = &mut comp["layers"][0];
        layer["time_stretch"] = J::from(stretch);
        if mix {
            layer["frame_blend"] = J::from("frame_mix");
        } else {
            layer.as_object_mut().unwrap().remove("frame_blend");
        }
        layer["drawing_dissolve"] = J::from(dissolve);
        let document = persist::load_str(&p.to_string()).expect("the picture's project reads").document;
        let mut said = Vec::new();
        let mut shots = Vec::new();
        for n in 0..frames {
            let mut log = FrameLog::new(8);
            let frame = render_frame(document.project(), &Id::new(MAIN), n, &dir, 64, &mut log).expect("the picture draws");
            said.extend(log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)));
            shots.push(frame.to_srgb8_straight());
        }
        // Side by side, twice the size, a grey line between frames.
        let (sw, sh) = ((w * 2 + 2) * frames as usize, h * 2);
        let mut sheet = vec![0u8; sw * sh * 4];
        for (i, shot) in shots.iter().enumerate() {
            for y in 0..sh {
                for x in 0..w * 2 + 2 {
                    let o = (y * sw + i * (w * 2 + 2) + x) * 4;
                    let px = if x >= w * 2 {
                        [128, 128, 128, 255]
                    } else {
                        let s = ((y / 2) * w + x / 2) * 4;
                        [shot[s], shot[s + 1], shot[s + 2], shot[s + 3]]
                    };
                    sheet[o..o + 4].copy_from_slice(&px);
                }
            }
        }
        png_out::write_rgba(&dir.join(name), sw, sh, OutputDepth::Eight, &[], &sheet).unwrap();
        (shots, said)
    };
    // Where a frame's ball is: the middle of its orange pixels, and how many of them are faint.
    let seen = |shot: &Vec<u8>| -> (f64, usize) {
        let (mut sum, mut count, mut faint) = (0.0, 0, 0);
        for (i, p) in shot.chunks(4).enumerate() {
            if p[3] > 0 && p[0] > 150 {
                sum += (i % w) as f64;
                count += 1;
                faint += (p[3] < 250) as usize;
            }
        }
        (if count == 0 { -1.0 } else { sum / count as f64 }, faint)
    };
    let (plain, said) = strip("on_twos.png", 100.0, false, 0, 6);
    let places: Vec<(f64, usize)> = plain.iter().map(seen).collect();
    t.row(
        "on_twos.png, as drawn: each drawing held two frames, whole",
        &format!("{said:?}, ball at {places:?}"),
        said.is_empty() && places[0] == places[1] && places[1] != places[2] && places.iter().all(|p| p.1 == 0),
    );
    let (held, said) = strip("stretched_held.png", 200.0, false, 0, 12);
    let places: Vec<(f64, usize)> = held.iter().map(seen).collect();
    t.row(
        "stretched_held.png, stretched to 200% with no mixing: each drawing held four frames, whole",
        &format!("{said:?}, ball at {places:?}"),
        said.is_empty() && places[0] == places[3] && places[3] != places[4] && places.iter().all(|p| p.1 == 0),
    );
    let (mixed, said) = strip("stretched_mixed.png", 200.0, true, 0, 12);
    let places: Vec<(f64, usize)> = mixed.iter().map(seen).collect();
    t.row(
        "stretched_mixed.png, stretched to 200% with Frame Mix: frames 3 and 7 show both drawings faint, a ghosted in-between; the rest are whole",
        &format!("{said:?}, ball at {places:?}"),
        said.is_empty() && places[3].1 > 0 && places[7].1 > 0 && places[2].1 == 0 && places[4].1 == 0,
    );
    let (dissolved, said) = strip("dissolved.png", 100.0, false, 1, 6);
    let places: Vec<(f64, usize)> = dissolved.iter().map(seen).collect();
    t.row(
        "dissolved.png, Drawing Dissolve 1: the second frame of each hold is half this drawing and half the next; the last drawing holds",
        &format!("{said:?}, ball at {places:?}"),
        said.is_empty() && places[1].1 > 0 && places[3].1 > 0 && places[0].1 == 0 && places[5].1 == 0,
    );

    t.finish("B-150_frame_blending_table.md");
}
