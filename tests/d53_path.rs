//! D-53 / R-03 / document 20: the motion path, checked against an evaluator that is not this one.
//!
//! D-53 is the third of the three decisions D-52 named and the last of them closed: a curve
//! *through* the position keys in space. Like the ease it was a fixture before it was code, and
//! this file checks the code against that fixture. The curve is a cubic Bezier in composition
//! pixels, one handle either side of each position key, stored as offsets from the key in
//! document 19's `spatial` array `[in_x, in_y, out_x, out_y]`. The parameter along it is the
//! fraction of the way through the segment *after* the ease - not arc length - so there is nothing
//! to solve, and the ease and the path compose in one order: when, then where.
//!
//! Every expected number below is transcribed from document 25, which got them from
//! `tools/path_reference.py`. That file evaluates document 20's cubic by de Casteljau - repeated
//! linear interpolation - in Python, written from the document; `src/model.rs` expands the
//! polynomial, in Rust. **The two were written from the specification and not from each other**,
//! which is the only reason a match between them means anything.
//!
//! Writes `verification/D-53_path_table.md`.
//!
//! # What this does not cover
//!
//! The window. No handle can yet be taken hold of on the canvas; a path reaches a project through
//! its file or through a command, and this file is about the numbers a handle would produce once
//! it can be dragged.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::command::{Command, Document};
use anime_compositor::model::{
    Asset, Composition, Id, Interp, Layer, Project, Prop, Property, Value,
};
use anime_compositor::persist::{self, Preserved};
use anime_compositor::time::FrameRate;
use serde_json::{json, Value as J};

const COMP: &str = "comp-0000-0000-0000";

/// Document 25's corner: leaves heading right, arrives heading down.
const CORNER_OUT: Option<[f64; 4]> = Some([0.0, 0.0, 160.0, 0.0]);
const CORNER_IN: Option<[f64; 4]> = Some([0.0, -160.0, 0.0, 0.0]);

/// One row of document 25's path tables: a segment, the frames sampled, and what is expected.
struct Case {
    id: &'static str,
    purpose: &'static str,
    /// The mode of the first key, which is the mode of the segment.
    interp: Interp,
    /// Frame, value, `spatial` of the first key.
    from: (i32, Value, Option<[f64; 4]>),
    /// Frame, value, `spatial` of the second key.
    to: (i32, Value, Option<[f64; 4]>),
    /// Frame, then the expected point at it. Transcribed from document 25.
    expected: &'static [(i32, Value)],
    tolerance: f64,
}

fn v2(x: f64, y: f64) -> Value {
    Value::Vec2(x, y)
}

/// FX-PATH-001's table, which two cases share: the handles at the thirds and no handles at all
/// are the same line, and document 25 says both must agree with it.
const THIRDS: &[(i32, Value)] = &[
    (0, Value::Vec2(0.0, 0.0)),
    (1, Value::Vec2(9.999999999999998, 4.999999999999999)),
    (6, Value::Vec2(60.0, 30.0)),
    (12, Value::Vec2(120.0, 60.0)),
    (18, Value::Vec2(180.0, 90.0)),
    (23, Value::Vec2(230.00000000000006, 115.00000000000003)),
    (24, Value::Vec2(240.0, 120.0)),
];

fn cases() -> Vec<Case> {
    vec![
        Case {
            id: "FX-PATH-001",
            purpose: "the handles that make a straight line make a straight line",
            interp: Interp::Linear,
            from: (0, v2(0.0, 0.0), Some([0.0, 0.0, 80.0, 40.0])),
            to: (24, v2(240.0, 120.0), Some([-80.0, -40.0, 0.0, 0.0])),
            expected: THIRDS,
            tolerance: 1e-9,
        },
        Case {
            id: "FX-PATH-001 (absent)",
            purpose: "the same two keys with no `spatial` at all, which document 19 says is that \
                      same line",
            interp: Interp::Linear,
            from: (0, v2(0.0, 0.0), None),
            to: (24, v2(240.0, 120.0), None),
            expected: THIRDS,
            tolerance: 1e-9,
        },
        Case {
            id: "FX-PATH-002",
            purpose: "one curved segment under linear timing; its midpoint is (180, 60) where the \
                      straight line would be at (120, 120)",
            interp: Interp::Linear,
            from: (0, v2(0.0, 0.0), CORNER_OUT),
            to: (24, v2(240.0, 240.0), CORNER_IN),
            expected: &[
                (0, Value::Vec2(0.0, 0.0)),
                (6, Value::Vec2(105.0, 15.0)),
                (12, Value::Vec2(180.0, 60.0)),
                (18, Value::Vec2(225.0, 135.0)),
                (24, Value::Vec2(240.0, 240.0)),
            ],
            tolerance: 1e-6,
        },
        Case {
            id: "FX-PATH-003",
            purpose: "the same curved segment with easy ease on it: the ease says when, the path \
                      says where, and frame 12 is still (180, 60)",
            interp: Interp::EASY,
            from: (0, v2(0.0, 0.0), CORNER_OUT),
            to: (24, v2(240.0, 240.0), CORNER_IN),
            expected: &[
                (0, Value::Vec2(0.0, 0.0)),
                (6, Value::Vec2(69.140625, 5.859375)),
                (12, Value::Vec2(180.0, 60.0)),
                (18, Value::Vec2(234.140625, 170.859375)),
                (24, Value::Vec2(240.0, 240.0)),
            ],
            tolerance: 1e-6,
        },
        Case {
            id: "FX-PATH-004",
            purpose: "a handle longer than its own segment, which is legal in space: the layer \
                      sets off backwards and overshoots its destination",
            interp: Interp::Linear,
            from: (0, v2(0.0, 0.0), Some([0.0, 0.0, -150.0, 90.0])),
            to: (20, v2(200.0, 0.0), Some([150.0, 90.0, 0.0, 0.0])),
            expected: &[
                (0, Value::Vec2(0.0, 0.0)),
                (5, Value::Vec2(-10.9375, 50.625)),
                (10, Value::Vec2(100.0, 67.5)),
                (15, Value::Vec2(210.9375, 50.625)),
                (20, Value::Vec2(200.0, 0.0)),
            ],
            tolerance: 1e-6,
        },
    ]
}

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn id(text: &str) -> Id {
    Id::new(text)
}

/// A one-layer document whose composition is long enough for every case above.
fn document() -> Document {
    let mut project = Project::new(id("p"));
    project
        .assets
        .push(Asset::sequence(id("a"), "a", "a_%03d.png"));
    project.compositions.push(Composition::new(
        id(COMP),
        "c",
        4,
        4,
        FrameRate::new(24, 1).unwrap(),
        0,
        40,
    ));
    let mut doc = Document::new(project);
    doc.apply(Command::AddLayer {
        composition: id(COMP),
        layer: Box::new(Layer::new(id("l"), "l", id("a"), 0, 40)),
        index: 0,
    })
    .expect("layer");
    doc
}

/// The property a set of keys was put on, read back out of the document.
fn keyed(prop: Prop, keys: &[(i32, Value, Interp, Option<[f64; 4]>)]) -> Property {
    let mut doc = document();
    for &(frame, value, interp, spatial) in keys {
        doc.apply(Command::SetKeyframe {
            composition: id(COMP),
            layer_id: id("l"),
            prop,
            frame,
            value,
            interp,
            spatial,
        })
        .expect("key");
    }
    let layer = doc
        .project()
        .composition(&id(COMP))
        .unwrap()
        .layer(&id("l"))
        .unwrap();
    layer
        .transform
        .get(prop)
        .expect("d53 asks only about position")
        .clone()
}

/// A case's two position keys, with the mode of the segment on the first.
fn evaluated(case: &Case) -> Property {
    keyed(
        Prop::Position,
        &[
            (case.from.0, case.from.1, case.interp, case.from.2),
            // The mode on the last keyframe never runs: document 19 says it belongs to the
            // segment that starts there, and there is no segment after the last key. Hold is set
            // here to make that visible - if it were ever consulted, every case's final row would
            // be wrong.
            (case.to.0, case.to.1, Interp::Hold, case.to.2),
        ],
    )
}

/// The largest difference between two values, component by component.
fn gap(a: Value, b: Value) -> f64 {
    match (a, b) {
        (Value::Scalar(x), Value::Scalar(y)) => (x - y).abs(),
        (Value::Vec2(ax, ay), Value::Vec2(bx, by)) => (ax - bx).abs().max((ay - by).abs()),
        _ => f64::INFINITY,
    }
}

#[test]
fn d53_path_segments_match_the_reference_evaluator() {
    let mut rows = String::new();
    let mut checks = 0usize;
    let mut worst = 0.0f64;

    for case in cases() {
        let prop = evaluated(&case);
        for &(frame, want) in case.expected {
            let got = prop.value_at(frame);
            let d = gap(want, got);
            assert!(
                d <= case.tolerance,
                "{} frame {frame}: document 25 says {want}, the build says {got}, a difference \
                 of {d} against a tolerance of {}. The reference evaluator is \
                 `tools/path_reference.py`; running it prints the table this row came from.",
                case.id,
                case.tolerance,
            );
            checks += 1;
            worst = worst.max(d);
            let _ = writeln!(
                rows,
                "| {} | {frame} | {want} | {got} | {d:.3e} | {:.0e} | pass |",
                case.id, case.tolerance,
            );
        }
    }

    // A keyframe's own value is returned exactly at its own frame, whatever its handles say.
    // Document 20's third rule fires before the path is consulted, which is why the first and
    // last row of every case above is exact rather than merely close - and why a handle that
    // overshoots, as FX-PATH-004's do, still lands on its key.
    for case in cases() {
        let prop = evaluated(&case);
        for (frame, want) in [(case.from.0, case.from.1), (case.to.0, case.to.1)] {
            assert!(
                gap(want, prop.value_at(frame)) == 0.0,
                "{} frame {frame}: a keyframe's own value must come back exactly",
                case.id
            );
            checks += 1;
        }
    }

    // A held segment returns the left value and the path has no say: document 20's rule for
    // hold fires before the curve is consulted. FX-PATH-002's corner, held, never leaves (0, 0).
    let corner = &cases()[2];
    let held = keyed(
        Prop::Position,
        &[
            (corner.from.0, corner.from.1, Interp::Hold, corner.from.2),
            (corner.to.0, corner.to.1, Interp::Hold, corner.to.2),
        ],
    );
    for &(frame, _) in corner.expected {
        let got = held.value_at(frame);
        let want = if frame >= corner.to.0 {
            corner.to.1
        } else {
            corner.from.1
        };
        assert!(
            gap(want, got) == 0.0,
            "FX-PATH-002 held, frame {frame}: a hold segment returns its left value whatever \
             handles it carries, and this one returned {got}"
        );
        checks += 1;
    }
    let _ = writeln!(
        rows,
        "| FX-PATH-002 held | all | the left key's value, whatever the handles say | the left \
         key's value | 0 | exact | pass |"
    );

    let mut purposes = String::new();
    for case in cases() {
        let _ = writeln!(purposes, "- **{}** - {}.", case.id, case.purpose);
    }

    let page = format!(
        "# D-53: the motion path, against an evaluator that is not this one\n\n\
         Written by `d53_path_segments_match_the_reference_evaluator` in `tests/d53_path.rs`, \
         which runs on every build. **{checks} of {checks} checks pass.**\n\n\
         D-53 asked for the curve to be named and a fixture to pin it to exact values *before any \
         code was written*. The curve is a cubic Bezier through the position keys in composition \
         pixels, one handle either side of each key, stored as offsets from the key - the handles \
         After Effects draws on the canvas. The fraction along it is the fraction of the way \
         through the segment *after* the ease, so the ease says when and the path says where; \
         FX-PATH-003 is the case with both on at once, and its frame 12 lands on the same point \
         as FX-PATH-002's because an ease never moves the path.\n\n\
         Every number in the **expected** column is transcribed from document 25, which got it \
         from `tools/path_reference.py`. That file evaluates the curve by de Casteljau - repeated \
         linear interpolation - in Python, written from document 20. This build expands the \
         polynomial, in Rust. The two were written from the specification and not from each \
         other, which is what makes an agreement between them worth having. The largest \
         disagreement anywhere in this table is **{worst:.3e}**.\n\n\
         What each case is for:\n\n{purposes}\n\
         | Case | Frame | Expected | This build | Difference | Tolerance | Result |\n\
         |---|---|---|---|---|---|---|\n{rows}\n\
         ## How to read a failure here\n\n\
         A row that fails is a disagreement between two evaluators, and either one may be the \
         one that moved. Running `python tools/path_reference.py` prints the Python side's \
         table; document 25 holds what it printed when the fixtures were set. If the Python and \
         the document still agree and this page does not, the build moved.\n\n\
         ## What this page does not show\n\n\
         The window. No handle can yet be taken hold of on the canvas; a path reaches a project \
         through its file or through a command, and this page is about the numbers a handle \
         would produce once it can be dragged.\n\n\
         Speed. D-53 chose the fraction of the segment over arc length and recorded the cost: on \
         a curved segment with uneven handles the layer's speed varies a little even under \
         linear timing. No row here measures speed, and FX-PATH-004's spacing is that cost \
         visible in the numbers rather than hidden.\n",
    );
    fs::write(repo("verification/D-53_path_table.md"), page).expect("write the artifact");
}

/// The four numbers survive a save and a load, in order, and a key without them stays without.
#[test]
fn path_handles_round_trip_through_the_save_format() {
    let first = Some([1.5, -2.0, 160.0, 0.0]);
    let second = Some([0.0, -160.0, 3.0, 4.0]);
    let mut doc = document();
    for (frame, value, spatial) in [
        (0, v2(0.0, 0.0), first),
        (24, v2(240.0, 240.0), second),
        (30, v2(10.0, 10.0), None),
    ] {
        doc.apply(Command::SetKeyframe {
            composition: id(COMP),
            layer_id: id("l"),
            prop: Prop::Position,
            frame,
            value,
            interp: Interp::Linear,
            spatial,
        })
        .expect("key");
    }
    let text = persist::to_json(doc.project(), &Preserved::default());
    let root: J = serde_json::from_str(&text).expect("what was just written is JSON");
    let keys = &root["compositions"][0]["layers"][0]["transform"]["position"]["keyframes"];
    let written: Vec<f64> = keys[0]["spatial"]
        .as_array()
        .expect("a key with handles writes them")
        .iter()
        .map(|n| n.as_f64().expect("a number"))
        .collect();
    assert_eq!(
        written,
        vec![1.5, -2.0, 160.0, 0.0],
        "document 19's order is in_x, in_y, out_x, out_y"
    );
    assert!(
        keys[2].get("spatial").is_none(),
        "a key without handles must not acquire any"
    );

    let loaded = persist::load_str(&text).unwrap_or_else(|d| panic!("reload: {}", d.message));
    let back = loaded
        .document
        .project()
        .composition(&id(COMP))
        .unwrap()
        .layer(&id("l"))
        .unwrap()
        .transform
        .position
        .clone();
    let spatial: Vec<Option<[f64; 4]>> = back.keyframes().iter().map(|k| k.spatial).collect();
    assert_eq!(
        spatial,
        vec![first, second, None],
        "the handles that came back are not the handles that went in"
    );
}

/// Document 19: `spatial` appears only on `position`, and a file or a command that puts it
/// anywhere else is refused rather than quietly stripped.
#[test]
fn path_handles_on_anything_but_position_are_refused() {
    for (prop, broken, why) in [
        (
            Prop::Rotation,
            json!([0.0, 0.0, 10.0, 0.0]),
            "handles on rotation, which has no path",
        ),
        (
            Prop::Anchor,
            json!([0.0, 0.0, 10.0, 0.0]),
            "handles on anchor, which is a pair but is not position",
        ),
        (
            Prop::Position,
            json!([1.0, 2.0, 3.0]),
            "three numbers where four are required",
        ),
        (
            Prop::Position,
            json!("thirds"),
            "a word where the four numbers belong",
        ),
    ] {
        let text = saved_with_spatial(prop, broken);
        let d = persist::load_str(&text)
            .err()
            .unwrap_or_else(|| panic!("a file with {why} loaded without complaint"));
        assert!(
            d.detail.contains("/spatial"),
            "the diagnostic for {why} must point at the field: {}",
            d.detail
        );
    }

    let mut doc = document();
    let d = doc
        .apply(Command::SetKeyframe {
            composition: id(COMP),
            layer_id: id("l"),
            prop: Prop::Rotation,
            frame: 0,
            value: Value::Scalar(0.0),
            interp: Interp::Linear,
            spatial: Some([0.0, 0.0, 10.0, 0.0]),
        })
        .expect_err("a rotation keyframe with path handles must be refused");
    assert!(
        d.detail.contains("position"),
        "the refusal must say where handles belong, and this one says: {}",
        d.detail
    );
}

/// The one-layer document above with two keys on `prop`, saved, and the first key's `spatial`
/// set to `spatial` in the parsed JSON rather than in the text, so that this stays true whichever
/// way the writer decides to indent.
fn saved_with_spatial(prop: Prop, spatial: J) -> String {
    let (a, b) = if prop.kind() == "vec2" {
        (v2(0.0, 0.0), v2(240.0, 240.0))
    } else {
        (Value::Scalar(0.0), Value::Scalar(1.0))
    };
    let mut doc = document();
    for (frame, value) in [(0, a), (24, b)] {
        doc.apply(Command::SetKeyframe {
            composition: id(COMP),
            layer_id: id("l"),
            prop,
            frame,
            value,
            interp: Interp::Linear,
            spatial: None,
        })
        .expect("key");
    }
    let text = persist::to_json(doc.project(), &Preserved::default());
    let mut root: J = serde_json::from_str(&text).expect("what was just written is JSON");
    let name = prop.to_string();
    let layer = &mut root["compositions"][0]["layers"][0];
    let key = &mut layer["transform"][name.as_str()]["keyframes"][0];
    key.as_object_mut()
        .expect("a keyframe is an object")
        .insert("spatial".into(), spatial);
    root.to_string()
}
