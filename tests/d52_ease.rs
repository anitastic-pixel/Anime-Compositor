//! D-52 (ease half) / R-03 / document 20: the eased keyframe segment, checked against a solver
//! that is not this one.
//!
//! D-52 asked for a curve to be named and a fixture to pin one eased segment to exact values
//! **before any code was written**, and that is what this file checks the code against. The curve
//! is a cubic Bezier of the value against time, stored as the two inner control points of a curve
//! whose ends are pinned at (0,0) and (1,1) - the four numbers CSS writes as `cubic-bezier()`.
//!
//! Every expected number below is transcribed from document 25, which got them from
//! `tools/ease_reference.py`. That file solves document 20's equation by bisection, in Python,
//! written from the document; `src/model.rs` solves it by Newton's method with a bisection
//! fallback, in Rust. **The two were written from the specification and not from each other**,
//! which is the only reason a match between them means anything. A number in the table below that
//! drifts is either a change to the build's solver or a change to the document, and both are
//! things someone should have to notice.
//!
//! Writes `verification/D-52_ease_table.md`.
//!
//! # What this does not cover
//!
//! A motion path. An eased position travels the straight line between its two keys, faster and
//! slower along it, and FX-EASE-003 is the row that says so: both components of the pair are the
//! same fraction of their own journey at every frame. A curve *through* the keys in space is the
//! third decision D-52 named and is still open.

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

/// Document 25's `[1/3, 1/3, 2/3, 2/3]`: the curve that is exactly linear.
const LINEAR_AS_BEZIER: Interp = Interp::Ease {
    x1: 0.3333333333333333,
    y1: 0.3333333333333333,
    x2: 0.6666666666666666,
    y2: 0.6666666666666666,
};
/// Document 25's `[0, 0, 0.58, 1]`: leaves fast, arrives slowly, and is not symmetric.
const EASE_OUT_ONLY: Interp = Interp::Ease {
    x1: 0.0,
    y1: 0.0,
    x2: 0.58,
    y2: 1.0,
};
/// Document 25's `[0.42, 0, 0.58, 1]`: the one whose `x` handles are not a third and two thirds,
/// so `x(t)` is not `t` and the solve is real work rather than free.
const EASE_IN_OUT: Interp = Interp::Ease {
    x1: 0.42,
    y1: 0.0,
    x2: 0.58,
    y2: 1.0,
};

/// One row of document 25's ease tables: a segment, the frames sampled, and what is expected.
struct Case {
    id: &'static str,
    purpose: &'static str,
    interp: Interp,
    from: (i32, Value),
    to: (i32, Value),
    /// Frame, then the expected value at it. Transcribed from document 25.
    expected: &'static [(i32, Value)],
    tolerance: f64,
}

fn s(v: f64) -> Value {
    Value::Scalar(v)
}
fn v2(x: f64, y: f64) -> Value {
    Value::Vec2(x, y)
}

fn cases() -> Vec<Case> {
    vec![
        Case {
            id: "FX-EASE-001",
            purpose: "the curve that is linear evaluates as linear",
            interp: LINEAR_AS_BEZIER,
            from: (0, s(0.0)),
            to: (24, s(120.0)),
            expected: &[
                (0, Value::Scalar(0.0)),
                (1, Value::Scalar(4.999999999999999)),
                (6, Value::Scalar(30.0)),
                (12, Value::Scalar(60.0)),
                (18, Value::Scalar(90.0)),
                (23, Value::Scalar(115.0)),
                (24, Value::Scalar(120.0)),
            ],
            tolerance: 1e-9,
        },
        Case {
            id: "FX-EASE-002",
            purpose: "easy ease on a scalar, and its exact midpoint",
            interp: Interp::EASY,
            from: (0, s(0.0)),
            to: (24, s(100.0)),
            expected: &[
                (0, Value::Scalar(0.0)),
                (3, Value::Scalar(4.296875)),
                (6, Value::Scalar(15.625)),
                (12, Value::Scalar(50.0)),
                (18, Value::Scalar(84.375)),
                (21, Value::Scalar(95.703125)),
                (24, Value::Scalar(100.0)),
            ],
            tolerance: 1e-6,
        },
        Case {
            id: "FX-EASE-003",
            purpose: "one timing curve drives both components of a pair",
            interp: Interp::EASY,
            from: (0, v2(0.0, 200.0)),
            to: (12, v2(300.0, -40.0)),
            expected: &[
                (0, Value::Vec2(0.0, 200.0)),
                (3, Value::Vec2(46.875, 162.5)),
                (6, Value::Vec2(150.0, 80.0)),
                (9, Value::Vec2(253.125, -2.5)),
                (12, Value::Vec2(300.0, -40.0)),
            ],
            tolerance: 1e-6,
        },
        Case {
            id: "FX-EASE-004",
            purpose: "an asymmetric curve is not evaluated as a symmetric one",
            interp: EASE_OUT_ONLY,
            from: (10, s(0.0)),
            to: (34, s(1.0)),
            expected: &[
                (10, Value::Scalar(0.0)),
                (16, Value::Scalar(0.37813813082510966)),
                (22, Value::Scalar(0.6846431874274606)),
                (28, Value::Scalar(0.9065353492811752)),
                (34, Value::Scalar(1.0)),
            ],
            tolerance: 1e-6,
        },
        Case {
            id: "FX-EASE-005",
            purpose: "a curve whose solve is not free, because x(t) is not t",
            interp: EASE_IN_OUT,
            from: (0, s(0.0)),
            to: (20, s(1.0)),
            expected: &[
                (0, Value::Scalar(0.0)),
                (4, Value::Scalar(0.08165985626589747)),
                (8, Value::Scalar(0.3318838700976461)),
                (10, Value::Scalar(0.5)),
                (12, Value::Scalar(0.6681161299023539)),
                (16, Value::Scalar(0.9183401437341024)),
                (20, Value::Scalar(1.0)),
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

/// The property a case's two keyframes were set on, read back out of the document.
fn evaluated(case: &Case) -> Property {
    let pair = matches!(case.from.1, Value::Vec2(..));
    let prop = if pair { Prop::Position } else { Prop::Rotation };
    let mut doc = document();
    for (frame, value, interp) in [
        (case.from.0, case.from.1, case.interp),
        // The mode on the last keyframe never runs: document 19 says it belongs to the segment
        // that starts there, and there is no segment after the last key. Hold is set here to make
        // that visible - if it were ever consulted, every case's final row would be wrong.
        (case.to.0, case.to.1, Interp::Hold),
    ] {
        doc.apply(Command::SetKeyframe {
            composition: id(COMP),
            layer_id: id("l"),
            prop,
            frame,
            value,
            interp,
        })
        .expect("key");
    }
    let layer = doc
        .project()
        .composition(&id(COMP))
        .unwrap()
        .layer(&id("l"))
        .unwrap();
    if pair {
        layer.transform.position.clone()
    } else {
        layer.transform.rotation.clone()
    }
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
fn d52_eased_segments_match_the_reference_solver() {
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
                 of {d} against a tolerance of {}. The reference solver is \
                 `tools/ease_reference.py`; running it prints the table this row came from.",
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

    // FX-EASE-003's real claim, stated as its own check rather than left to be read off the
    // table: both components of the pair are the same fraction of their own journey at every
    // frame, which is what "an ease in time, not a path in space" means.
    let three = &cases()[2];
    let prop = evaluated(three);
    for &(frame, _) in three.expected {
        let (Value::Vec2(x, y), Value::Vec2(x0, y0), Value::Vec2(x1, y1)) =
            (prop.value_at(frame), three.from.1, three.to.1)
        else {
            panic!("FX-EASE-003 is a pair");
        };
        let fx = (x - x0) / (x1 - x0);
        let fy = (y - y0) / (y1 - y0);
        assert!(
            (fx - fy).abs() <= 1e-12,
            "FX-EASE-003 frame {frame}: the layer is {fx} of the way along in x and {fy} in y, so \
             it has left the straight line between its keys. One curve drives both components."
        );
        checks += 1;
    }
    let _ = writeln!(
        rows,
        "| FX-EASE-003 | all | x and y the same fraction of their own journey | same to 1e-12 | \
         - | 1e-12 | pass |"
    );

    // A keyframe's own value is returned exactly at its own frame, whatever its ease says. This
    // is document 20's third rule firing before its sixth, and it is why the first and last row
    // of every case above is exact rather than merely close.
    for case in cases() {
        let prop = evaluated(&case);
        for (frame, want) in [case.from, case.to] {
            assert!(
                gap(want, prop.value_at(frame)) == 0.0,
                "{} frame {frame}: a keyframe's own value must come back exactly",
                case.id
            );
            checks += 1;
        }
    }

    let mut purposes = String::new();
    for case in cases() {
        let _ = writeln!(purposes, "- **{}** - {}.", case.id, case.purpose);
    }

    let page = format!(
        "# D-52: the eased segment, against a solver that is not this one\n\n\
         Written by `d52_eased_segments_match_the_reference_solver` in `tests/d52_ease.rs`, \
         which runs on every build. **{checks} of {checks} checks pass.**\n\n\
         D-52 asked for the curve to be named and a fixture to pin one eased segment to exact \
         values *before any code was written*. The curve is a cubic Bezier of the value against \
         time, written as the two inner control points of a curve whose ends are pinned at (0,0) \
         and (1,1) - the four numbers CSS writes as `cubic-bezier()` and After Effects draws as \
         two handles in its value graph.\n\n\
         Every number in the **expected** column is transcribed from document 25, which got it \
         from `tools/ease_reference.py`. That file solves the equation by bisection, in Python, \
         written from document 20. This build solves it by Newton's method with a bisection \
         fallback, in Rust. The two were written from the specification and not from each other, \
         which is what makes an agreement between them worth having. The largest disagreement \
         anywhere in this table is **{worst:.3e}**.\n\n\
         What each case is for:\n\n{purposes}\n\
         | Case | Frame | Expected | This build | Difference | Tolerance | Result |\n\
         |---|---|---|---|---|---|---|\n{rows}\n\
         ## How to read a failure here\n\n\
         A row that fails is not necessarily a broken solver. It is a disagreement between two \
         solvers, and either one of them may be the one that moved. Running \
         `python tools/ease_reference.py` prints the Python side's table; document 25 holds what \
         it printed when the fixtures were set. If the Python and the document still agree and \
         this page does not, the build moved.\n\n\
         ## What this page does not show\n\n\
         A motion path. An eased position travels the straight line between its two keys, faster \
         and slower along it - FX-EASE-003's extra row is that claim checked, not assumed. A \
         curve *through* the keys in space is the third decision D-52 named, and it is still \
         open.\n\n\
         It also shows nothing about the window. Whether an artist sets an ease by dragging a \
         graph or by pressing one preset button is a separate unit; this page is about the \
         numbers those controls would produce.\n",
    );
    fs::write(repo("verification/D-52_ease_table.md"), page).expect("write the artifact");
}

/// The four numbers survive a save and a load, in order.
#[test]
fn an_ease_round_trips_through_the_save_format() {
    let case = Case {
        id: "round trip",
        purpose: "",
        interp: EASE_IN_OUT,
        from: (0, s(0.0)),
        to: (20, s(1.0)),
        expected: &[],
        tolerance: 0.0,
    };
    let mut doc = document();
    for (frame, value, interp) in [(0, s(0.0), case.interp), (20, s(1.0), Interp::Linear)] {
        doc.apply(Command::SetKeyframe {
            composition: id(COMP),
            layer_id: id("l"),
            prop: Prop::Rotation,
            frame,
            value,
            interp,
        })
        .expect("key");
    }
    let text = persist::to_json(doc.project(), &Preserved::default());
    assert!(
        text.contains("\"ease\""),
        "a keyframe whose interp is ease must write its four numbers"
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
        .rotation
        .clone();
    assert_eq!(
        back.keyframes()[0].interp,
        case.interp,
        "the curve that came back is not the curve that went in"
    );
    assert_eq!(
        back.keyframes()[1].interp,
        Interp::Linear,
        "a linear keyframe must not acquire a curve"
    );
}

/// Document 19: an `x` handle outside its own segment is diagnosed, not clamped.
#[test]
fn an_ease_handle_outside_its_segment_is_refused() {
    for (broken, why) in [
        (
            json!([1.4, 0.0, 0.58, 1.0]),
            "x1 past the end of the segment",
        ),
        (json!([0.42, 0.0, -0.1, 1.0]), "x2 before the start of it"),
        (
            json!([0.42, 0.0, 0.58]),
            "three numbers where four are required",
        ),
        (json!("easy"), "a name where the four numbers belong"),
        (J::Null, "the four numbers missing entirely"),
    ] {
        let text = saved_with_ease(broken);
        let d = persist::load_str(&text)
            .err()
            .unwrap_or_else(|| panic!("a file with {why} loaded without complaint"));
        assert!(
            d.detail.contains("/ease"),
            "the diagnostic for {why} must point at the field: {}",
            d.detail
        );
    }
}

/// Document 19: a `y` handle outside 0 and 1 is an overshoot, which is a thing an animator means,
/// so it is kept rather than refused or clamped.
#[test]
fn an_overshooting_ease_is_kept_and_overshoots() {
    let text = saved_with_ease(json!([0.42, -0.6, 0.58, 1.6]));
    let loaded =
        persist::load_str(&text).unwrap_or_else(|d| panic!("overshoot refused: {}", d.message));
    let rot = loaded
        .document
        .project()
        .composition(&id(COMP))
        .unwrap()
        .layer(&id("l"))
        .unwrap()
        .transform
        .rotation
        .clone();
    let mut below = false;
    let mut above = false;
    for frame in 0..=20 {
        let Value::Scalar(v) = rot.value_at(frame) else {
            unreachable!()
        };
        below |= v < -0.001;
        above |= v > 1.001;
    }
    assert!(
        below && above,
        "an ease with y1 below zero and y2 above one must dip below its start and pass its end \
         before settling; if it does not, the y handles are being clamped somewhere"
    );
}

/// The one-layer document above, saved, with the first keyframe's `ease` replaced by `ease`.
///
/// The replacement is made in the parsed JSON rather than in the text, so that this stays true
/// whichever way the writer decides to indent. `J::Null` removes the field instead of setting it.
fn saved_with_ease(ease: J) -> String {
    let mut doc = document();
    for (frame, value, interp) in [(0, s(0.0), EASE_IN_OUT), (20, s(1.0), Interp::Linear)] {
        doc.apply(Command::SetKeyframe {
            composition: id(COMP),
            layer_id: id("l"),
            prop: Prop::Rotation,
            frame,
            value,
            interp,
        })
        .expect("key");
    }
    let text = persist::to_json(doc.project(), &Preserved::default());
    let mut root: J = serde_json::from_str(&text).expect("what was just written is JSON");
    let key = &mut root["compositions"][0]["layers"][0]["transform"]["rotation"]["keyframes"][0];
    assert_eq!(
        key["interp"], "ease",
        "the keyframe to break is the eased one"
    );
    let slot = key.as_object_mut().expect("a keyframe is an object");
    match ease {
        J::Null => {
            slot.remove("ease");
        }
        other => {
            slot.insert("ease".into(), other);
        }
    }
    root.to_string()
}
