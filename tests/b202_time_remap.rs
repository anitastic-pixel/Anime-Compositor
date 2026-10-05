//! B-202: D-323, Time Remapping, against ADR-021 and document 20's "Time remapping".
//!
//! Writes `verification/D-323_time_remap_table.md`, and pictures in
//! `verification/D-323 pictures/`.
//!
//! Every expected number is `Fixtures/time_remap/expected_time_remap.json`, written by
//! `tools/time_remap_reference.py` before this code existed and printed in document 25 as
//! FX-TREMAP-001 to 055. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;

use effect_table::{same_json, saved, Table, MAIN};
use serde_json::Value as J;

use anime_compositor::command::{Command, Document, Target, TimeRemap};
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Expression, Id, Interp, Layer, Prop, Value};
use anime_compositor::{persist, png_out, OutputDepth};

fn json(t: &Table, file: &str) -> J {
    serde_json::from_str(&fs::read_to_string(t.root.join(file)).unwrap()).unwrap()
}

/// FX-TREMAP-010's drawn layer `bar` given these times and this remap (`Null` for none), in a
/// composition long enough for all of them.
fn timed(t: &Table, in_frame: i64, out_frame: i64, offset: i64, stretch: f64, remap: &J) -> Document {
    let mut p = json(t, "fx_tremap_010.json");
    let comp = &mut p["compositions"][0];
    comp["duration_frames"] = J::from(1000);
    comp["work_area"]["end_frame_exclusive"] = J::from(1000);
    let layer = &mut comp["layers"][0];
    layer["in_frame"] = J::from(in_frame);
    layer["out_frame"] = J::from(out_frame);
    layer["source_offset_frames"] = J::from(offset);
    layer["time_stretch"] = J::from(stretch);
    match remap {
        J::Null => {
            layer.as_object_mut().unwrap().remove("time_remap");
        }
        keys => layer["time_remap"] = serde_json::json!({ "base": 0, "keyframes": keys }),
    }
    persist::load_str(&p.to_string()).expect("the made-up project reads").document
}

fn bar(document: &Document) -> &Layer {
    document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("bar")).unwrap()
}

/// The remap's keys as the file writes them: frame, value, interpolation.
fn remap_keys(document: &Document) -> Vec<(i32, f64, String)> {
    bar(document).time_remap.as_ref().map_or(Vec::new(), |r| {
        r.keyframes()
            .iter()
            .map(|k| (k.frame, k.value.as_scalar().unwrap(), format!("{:?}", k.interp).to_lowercase()))
            .collect()
    })
}

fn want_keys(keys: &J) -> Vec<(i32, f64, String)> {
    keys.as_array()
        .unwrap()
        .iter()
        .map(|k| (k["frame"].as_i64().unwrap() as i32, k["value"].as_f64().unwrap(), k["interp"].as_str().unwrap().to_string()))
        .collect()
}

fn remap(value: TimeRemap) -> Command {
    Command::SetTimeRemap { composition: Id::new(MAIN), layer_id: Id::new("bar"), value }
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

const BALL: (usize, usize) = (72, 20);
const DRAWINGS: usize = 17;

/// A made-up drawing of a ball, one of seventeen on its way across: an orange disc with a dark
/// outline, `x` its middle, on nothing.
fn ball(x: f64) -> Vec<u8> {
    let (w, h) = BALL;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for py in 0..h {
        for px in 0..w {
            let r = (px as f64 + 0.5 - x).hypot(py as f64 + 0.5 - 10.0);
            bytes.extend(if r < 5.0 {
                [250, 150, 40, 255]
            } else if r < 7.0 {
                [60, 30, 20, 255]
            } else {
                [0, 0, 0, 0]
            });
        }
    }
    bytes
}

#[test]
fn b202_time_remap() {
    let mut t = Table::new(
        "time_remap",
        "# D-323: Time Remapping\n\nADR-021 and D-308, accepted by the owner on 2026-10-04: a drawn \
         or composition layer may key which of its source frames it shows, as After Effects' Time \
         Remap does. Enable Time Remapping (Layer menu, Ctrl+Alt+T) writes two keys that change no \
         frame; Freeze Frame on a composition layer is one hold key. Every expected number is \
         `Fixtures/time_remap/expected_time_remap.json`, written by `tools/time_remap_reference.py` \
         before this code existed and printed in document 25 as FX-TREMAP-001 to 055. Each frame is \
         8 by 1 pixels; the answer is the largest difference over all its samples, against the \
         catalogue's pixel tolerance of 1e-6.\n",
    );
    let expected = json(&t, "expected_time_remap.json");
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let pixel_tolerance = expected["pixel_tolerance"].as_f64().unwrap();

    t.heading("FX-TREMAP-001 to 009 and 030: which source time each frame shows");
    for (name, case) in expected["times"].as_object().unwrap() {
        let document = timed(
            &t,
            case["in_frame"].as_i64().unwrap(),
            1000,
            case["source_offset_frames"].as_i64().unwrap(),
            case["time_stretch"].as_f64().unwrap(),
            &case["time_remap"]["keyframes"],
        );
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

    t.heading("FX-TREMAP-010 to 019: the frames, and what each frame says");
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
    for (_, case) in expected["cases"].as_object().unwrap() {
        let file = case["project"].as_str().unwrap();
        let same = same_json(&saved(&t.load(file)), &json(&t, file));
        t.row(
            &format!("{file} opened and saved holds what it held, its Time Remap and keys included"),
            if same { "the same" } else { "differs" },
            same,
        );
    }
    let mut old = json(&t, "fx_tremap_010.json");
    old["compositions"][0]["layers"][0].as_object_mut().unwrap().remove("time_remap");
    let again = saved(&persist::load_str(&old.to_string()).expect("the old file reads"));
    let quiet = same_json(&again, &old);
    t.row(
        "a file with no Time Remap, as every file before D-323, is saved as it was, with no time_remap",
        if quiet { "the same" } else { "differs" },
        quiet,
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

    t.heading("FX-TREMAP-040 to 044: Enable Time Remapping changes no frame");
    for (name, case) in expected["on"].as_object().unwrap() {
        let get = |k: &str| case[k].as_i64().unwrap();
        let (i, o) = (get("in_frame"), get("out_frame"));
        let mut document = timed(&t, i, o, get("source_offset_frames"), case["time_stretch"].as_f64().unwrap(), &J::Null);
        let before = format!("{:?}", bar(&document));
        let times: Vec<Option<f64>> = (i as i32..o as i32).map(|n| bar(&document).source_time(n)).collect();
        let taken = document.apply(remap(TimeRemap::On)).is_ok();
        let keys = remap_keys(&document);
        let now: Vec<Option<f64>> = (i as i32..o as i32).map(|n| bar(&document).source_time(n)).collect();
        let kept = times.iter().zip(&now).all(|(a, b)| match (a, b) {
            (Some(a), Some(b)) => (a - b).abs() <= tolerance,
            _ => false,
        });
        document.undo();
        let back = format!("{:?}", bar(&document)) == before;
        t.row(
            &format!("{name}: {} Every frame shows the source time it showed. One undo takes it off.", case["says"].as_str().unwrap()),
            &format!("{}, keys {keys:?}, frames kept {kept}, undone {back}", if taken { "taken" } else { "refused" }),
            taken && keys == want_keys(&case["keys"]) && kept && back,
        );
    }
    let mut document = timed(&t, 0, 6, 0, 100.0, &J::Null);
    let pictures: Vec<_> = (0..6).map(|n| t.render(&document, n, 64)).collect();
    document.apply(remap(TimeRemap::On)).expect("on");
    let same = (0..6).all(|n| t.render(&document, n, 64).data() == pictures[n as usize].data());
    t.row(
        "FX-TREMAP-010's drawings, with Time Remapping turned on: all six frames byte-identical to before",
        if same { "byte-identical" } else { "differ" },
        same,
    );
    let off = document.apply(remap(TimeRemap::Off)).is_ok() && bar(&document).time_remap.is_none();
    // A file that held one, opened, turned off and saved.
    let mut loaded = t.load("fx_tremap_010.json");
    let off = off && loaded.document.apply(remap(TimeRemap::Off)).is_ok();
    let quiet = saved(&loaded)["compositions"][0]["layers"][0].get("time_remap").is_none();
    t.row(
        "turned off again: the remap and its keys are gone, and a file that held one is written without time_remap",
        &format!("taken {off}, written {}", !quiet),
        off && quiet,
    );

    t.heading("FX-TREMAP-045 to 047: Freeze Frame by Time Remap");
    for (name, case) in expected["freezes"].as_object().unwrap() {
        let get = |k: &str| case[k].as_i64().unwrap();
        let mut document = timed(
            &t,
            get("in_frame"),
            get("out_frame"),
            get("source_offset_frames"),
            case["time_stretch"].as_f64().unwrap(),
            &case["keys_before"],
        );
        let before = format!("{:?}", bar(&document));
        let n = get("playhead") as i32;
        let shown = bar(&document).source_time(n).unwrap();
        let taken = document.apply(remap(TimeRemap::Freeze(n))).is_ok();
        let keys = remap_keys(&document);
        let layer = bar(&document);
        let held = (get("in_frame") as i32..get("out_frame") as i32).all(|m| layer.source_time(m) == Some(shown));
        document.undo();
        let back = format!("{:?}", bar(&document)) == before;
        t.row(
            &format!("{name}: {} Every frame then shows the playhead's source time. One undo puts it back.", case["says"].as_str().unwrap()),
            &format!("{}, keys {keys:?}, all frames at {shown}: {held}, undone {back}", if taken { "taken" } else { "refused" }),
            taken && keys == want_keys(&case["keys"]) && held && back,
        );
    }

    t.heading("FX-TREMAP-048: the keys move with the layer");
    for (name, case) in expected["shifts"].as_object().unwrap() {
        let mut document = timed(&t, 0, 12, 0, 100.0, &case["keys_before"]);
        let by = case["by"].as_i64().unwrap() as i32;
        let taken = document
            .apply(Command::ShiftLayer { composition: Id::new(MAIN), layer_id: Id::new("bar"), in_frame: by })
            .is_ok();
        let keys = remap_keys(&document);
        t.row(
            &format!("{name}: {}", case["says"].as_str().unwrap()),
            &format!("{}, keys {keys:?}", if taken { "taken" } else { "refused" }),
            taken && keys == want_keys(&case["keys"]),
        );
    }
    let mut document = timed(&t, 0, 12, 0, 100.0, &expected["shifts"]["FX-TREMAP-048"]["keys_before"]);
    let picture = t.render(&document, 4, 64);
    let taken = document
        .apply(Command::TrimLayer { composition: Id::new(MAIN), layer_id: Id::new("bar"), in_frame: 2, out_frame: 12 })
        .is_ok();
    let same = t.render(&document, 4, 64).data() == picture.data();
    t.row(
        "the in point trimmed from 0 to 2: frame 4 still shows what it showed",
        &format!("{}, frame 4 {}", if taken { "taken" } else { "refused" }, if same { "byte-identical" } else { "differs" }),
        taken && same,
    );

    t.heading("What is refused, with a sentence, changing nothing");
    let mut document = timed(&t, 0, 6, 0, 100.0, &J::Null);
    let before = format!("{:?}", bar(&document));
    let target = || Target::Layer(Id::new("bar"));
    for (what, command) in [
        (
            "a Time Remap key while the remap is off",
            Command::SetKeyframe {
                composition: Id::new(MAIN),
                target: target(),
                prop: Prop::TimeRemap,
                frame: 2,
                value: Value::Scalar(4.0),
                interp: Interp::Linear,
                spatial: None,
            },
        ),
        (
            "a Time Remap value while the remap is off",
            Command::SetPropertyBase { composition: Id::new(MAIN), target: target(), prop: Prop::TimeRemap, value: Value::Scalar(4.0) },
        ),
        ("a freeze on frame 9, after the layer's out point", remap(TimeRemap::Freeze(9))),
    ] {
        let refused = document.apply(command).err();
        t.row(
            &format!("{what} is refused"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && format!("{:?}", bar(&document)) == before,
        );
    }
    document.apply(remap(TimeRemap::On)).expect("on");
    let before = format!("{:?}", bar(&document));
    let refused = document
        .apply(Command::SetExpression {
            composition: Id::new(MAIN),
            target: target(),
            prop: Prop::TimeRemap,
            expression: Some(Expression { text: "time * 24".into(), enabled: true }),
        })
        .err();
    t.row(
        "an expression on a Time Remap is refused (FX-TREMAP-054's rule, at the command)",
        &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
        refused.is_some() && format!("{:?}", bar(&document)) == before,
    );
    let mut p = json(&t, "fx_tremap_050.json");
    p["compositions"][0]["layers"][1].as_object_mut().unwrap().remove("time_remap");
    let mut solid = persist::load_str(&p.to_string()).expect("the solid's project reads").document;
    let card = Id::new("card");
    let refused = solid
        .apply(Command::SetTimeRemap { composition: Id::new(MAIN), layer_id: card, value: TimeRemap::On })
        .err();
    t.row(
        "Enable Time Remapping on a solid layer is refused",
        &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
        refused.is_some(),
    );
    let mut document = timed(&t, 0, 6, 0, 100.0, &J::Null);
    document.apply(remap(TimeRemap::On)).expect("on");
    let taken = document
        .apply(Command::SetKeyframe {
            composition: Id::new(MAIN),
            target: target(),
            prop: Prop::TimeRemap,
            frame: 3,
            value: Value::Scalar(1.0),
            interp: Interp::Linear,
            spatial: None,
        })
        .is_ok();
    let t3 = bar(&document).source_time(3);
    t.row(
        "with the remap on, a key of 1 at frame 3 is taken, and frame 3 shows source frame 1",
        &format!("{}, t {t3:?}", if taken { "taken" } else { "refused" }),
        taken && t3 == Some(1.0),
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_tremap_012.json", 3), ("fx_tremap_014.json", 3), ("fx_tremap_017.json", 5)]);

    t.heading("Pictures: a made-up ball in seventeen drawings, in `verification/D-323 pictures/`");
    let dir = effect_table::repo("verification/D-323 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = BALL;
    for i in 0..DRAWINGS {
        let path = dir.join(format!("ball_{:04}.png", i + 1));
        png_out::write_rgba(&path, w, h, OutputDepth::Eight, &[], &ball(6.0 + 3.5 * i as f64)).unwrap();
    }
    let template = json(&t, "fx_tremap_010.json");
    let frames = 12;
    let strip = |name: &str, keys: J| -> (Vec<f64>, Vec<String>) {
        let mut p = template.clone();
        p["assets"][0]["frames"] =
            (1..=DRAWINGS).map(|i| (i.to_string(), J::from(format!("ball_{i:04}.png")))).collect::<serde_json::Map<_, _>>().into();
        let comp = &mut p["compositions"][0];
        comp["width"] = J::from(w);
        comp["height"] = J::from(h);
        comp["duration_frames"] = J::from(frames);
        comp["work_area"]["end_frame_exclusive"] = J::from(frames);
        let layer = &mut comp["layers"][0];
        layer["out_frame"] = J::from(frames);
        layer["exposure_spans"] = (0..DRAWINGS)
            .map(|i| serde_json::json!({ "start_frame": i, "end_frame_exclusive": i + 1, "drawing_number": i + 1 }))
            .collect();
        match keys {
            J::Null => {
                layer.as_object_mut().unwrap().remove("time_remap");
            }
            keys => layer["time_remap"] = serde_json::json!({ "base": 0, "keyframes": keys }),
        }
        let document = persist::load_str(&p.to_string()).expect("the picture's project reads").document;
        let mut said = Vec::new();
        let mut shots = Vec::new();
        for n in 0..frames {
            let mut log = FrameLog::new(8);
            let frame = render_frame(document.project(), &Id::new(MAIN), n, &dir, 64, &mut log).expect("the picture draws");
            said.extend(log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)));
            shots.push(frame.to_srgb8_straight());
        }
        // One frame a row, twice the size, a grey line between frames.
        let (sw, sh) = (w * 2, (h * 2 + 2) * frames as usize);
        let mut sheet = vec![0u8; sw * sh * 4];
        for (i, shot) in shots.iter().enumerate() {
            for y in 0..h * 2 + 2 {
                for x in 0..sw {
                    let o = ((i * (h * 2 + 2) + y) * sw + x) * 4;
                    let px = if y >= h * 2 {
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
        // Which drawing each frame shows: from where its ball's orange is.
        let places = shots
            .iter()
            .map(|shot| {
                let (mut sum, mut count) = (0.0, 0);
                for (i, p) in shot.chunks(4).enumerate() {
                    if p[3] > 0 && p[0] > 150 {
                        sum += (i % w) as f64 + 0.5;
                        count += 1;
                    }
                }
                if count == 0 { -1.0 } else { ((sum / count as f64 - 6.0) / 3.5).round() }
            })
            .collect();
        (places, said)
    };
    let key = |frame: i32, value: i32, interp: &str| serde_json::json!({ "frame": frame, "value": value, "interp": interp });
    let (drawn, said) = strip("as_drawn.png", J::Null);
    t.row(
        "as_drawn.png, no remap: drawing 0, 1, 2 ... 11, one a frame",
        &format!("{said:?}, drawings {drawn:?}"),
        said.is_empty() && drawn == (0..12).map(f64::from).collect::<Vec<_>>(),
    );
    let (burst, said) = strip("burst_then_real_speed.png", J::Array(vec![key(0, 0, "linear"), key(4, 8, "linear"), key(12, 16, "linear")]));
    t.row(
        "burst_then_real_speed.png, tutorial 1's keys 0 at 0, 8 at 4, 16 at 12: two drawings a frame for four frames, then one a frame",
        &format!("{said:?}, drawings {burst:?}"),
        said.is_empty() && burst == [0.0, 2.0, 4.0, 6.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0],
    );
    let (back, said) = strip("backwards.png", J::Array(vec![key(0, 11, "linear"), key(11, 0, "linear")]));
    t.row(
        "backwards.png, keys 11 at 0 and 0 at 11: the ball goes right to left",
        &format!("{said:?}, drawings {back:?}"),
        said.is_empty() && back == (0..12).rev().map(f64::from).collect::<Vec<_>>(),
    );
    let (frozen, said) = strip("frozen_at_5.png", J::Array(vec![key(5, 5, "hold")]));
    t.row(
        "frozen_at_5.png, Freeze Frame at 5: drawing 5 on every frame",
        &format!("{said:?}, drawings {frozen:?}"),
        said.is_empty() && frozen.iter().all(|d| *d == 5.0),
    );

    t.finish("D-323_time_remap_table.md");
}
