//! B-24d: a mask's path set moving, against D-77.
//!
//! Writes `verification/B-24d_mask_key_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/masks/expected_masks.json`, written by `tools/mask_reference.py`, which interpolates
//! each path point by point by document 20's rules and then renders the frame the way B-24b's
//! fixtures are rendered — a second implementation, written from D-77 and document 20 rather than
//! from `src/mask.rs`. Document 25 prints the same numbers as FX-MSK-031 to 035, and
//! `verification/B-24d keys/mask_key_cases.png` draws all five, five frames each. The tolerance is
//! the catalogue's, 1e-6.
//!
//! # What is deliberately not here
//!
//! The window — the stopwatch on a mask row, the key track, a dragged point writing to the key at
//! the current frame — is checked by B-24d's own window table. Every mask that stands still is
//! B-24b's table and is unchanged by this unit.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::mask::{Mask, MaskKey};
use anime_compositor::model::{Id, Interp};
use anime_compositor::persist;

const COMP: &str = "comp-main";

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/masks")
}

struct Table {
    out: String,
    checks: usize,
    passed: usize,
}

impl Table {
    fn row(&mut self, what: &str, built: &str, ok: bool) {
        self.checks += 1;
        self.passed += ok as usize;
        let verdict = if ok { "yes" } else { "**NO**" };
        self.out
            .push_str(&format!("| {what} | {built} | {verdict} |\n"));
    }

    fn heading(&mut self, text: &str) {
        self.out.push_str(&format!(
            "\n## {text}\n\n| Check | The build's answer | Matches |\n| --- | --- | --- |\n"
        ));
    }
}

fn load(file: &str) -> Document {
    persist::load(&root().join(file))
        .unwrap_or_else(|d| panic!("{file} opens: {} {}", d.message, d.detail))
        .document
}

fn masks_of(document: &Document, layer: &str) -> Vec<Mask> {
    document
        .project()
        .composition(&Id::new(COMP))
        .unwrap()
        .layer(&Id::new(layer))
        .unwrap()
        .masks
        .clone()
}

/// The largest difference between the built frame and the fixture's, sample by sample.
fn difference(document: &Document, frame: i32, expected: &J) -> (f64, Vec<DiagnosticId>) {
    let mut log = FrameLog::new(8);
    let buffer = render_frame(
        document.project(),
        &Id::new(COMP),
        frame,
        &root(),
        64,
        &mut log,
    )
    .unwrap_or_else(|d| panic!("frame {frame} renders: {}", d.message));
    let want: Vec<f64> = expected
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|p| p.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()))
        .collect();
    assert_eq!(want.len(), buffer.data().len(), "the extents agree");
    let d = buffer
        .data()
        .iter()
        .zip(&want)
        .map(|(a, b)| (*a as f64 - b).abs())
        .fold(0.0, f64::max);
    (d, log.ids_at(frame))
}

#[test]
fn b24d_mask_keys() {
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root().join("expected_masks.json")).unwrap())
            .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();

    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-24d: a mask's path set moving\n\nD-77 said it before anything was built: a path is \
         interpolated point by point, the point and both of its handles, between keys that hold \
         the same number of points, by document 20's curves. FX-MSK-031 to 035 are five cases of \
         that, five frames each, drawn in `verification/B-24d keys/mask_key_cases.png` and \
         printed in document 25. Every expected pixel is \
         `Fixtures/masks/expected_masks.json`, written by `tools/mask_reference.py` before this \
         code existed; the build's frame is compared sample by sample and the answer is the \
         largest difference over all of them, against the catalogue's tolerance of \
         1e-6.\n\nUntil today a file with keys on a path opened, was kept and was drawn at its \
         base with `PROJECT_FEATURE_UNSUPPORTED` said out loud. That fallback is gone: the \
         diagnostic is retired and the path moves.\n",
    );

    // -----------------------------------------------------------------------------------
    t.heading("The five moving cases, frame by frame (document 25)");
    let mut cases: Vec<(&String, &J)> = expected["cases"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(name, _)| name.as_str() >= "FX-MSK-031")
        .collect();
    cases.sort_by_key(|(name, _)| name.as_str());
    for (name, case) in &cases {
        let document = load(case["project"].as_str().unwrap());
        let mut frames: Vec<i32> = case["frames"]
            .as_object()
            .unwrap()
            .keys()
            .map(|f| f.parse().unwrap())
            .collect();
        frames.sort();
        t.out
            .push_str(&format!("| *{name}: {}* | | |\n", case["says"].as_str().unwrap()));
        for frame in frames {
            let (d, said) = difference(&document, frame, &case["frames"][frame.to_string()]);
            t.row(
                &format!("{name} frame {frame}"),
                &format!(
                    "largest difference {d:.1e}; said {}",
                    if said.is_empty() {
                        "nothing".to_string()
                    } else {
                        said.iter().map(|i| i.as_str()).collect::<Vec<_>>().join(", ")
                    }
                ),
                d <= tolerance && said.is_empty(),
            );
        }
    }

    // -----------------------------------------------------------------------------------
    t.heading("The shape at a frame, read out loud (document 20)");
    // FX-MSK-031 slides a three-wide rectangle from x=0 to x=3 over frames 0 to 4, linearly, so
    // the left edge is at 0, 0.75, 1.5, 2.25, 3. The whole outline moves with it; the first
    // point is enough to read the clock by.
    let slide = masks_of(&load("fx_msk_031.json"), "cel")[0].clone();
    let lefts: Vec<f64> = (0..5).map(|f| slide.points_at(f)[0].point.0).collect();
    t.row(
        "FX-MSK-031's left edge over frames 0 to 4, linear between keys at 0 and 4",
        &format!("{lefts:?}"),
        lefts == vec![0.0, 0.75, 1.5, 2.25, 3.0],
    );
    let hold = masks_of(&load("fx_msk_032.json"), "cel")[0].clone();
    let held: Vec<f64> = (0..5).map(|f| hold.points_at(f)[0].point.0).collect();
    t.row(
        "FX-MSK-032's first key holds, so the shape waits and then arrives whole",
        &format!("{held:?}"),
        held == vec![0.0, 0.0, 0.0, 0.0, 3.0],
    );
    let eased = masks_of(&load("fx_msk_033.json"), "cel")[0].clone();
    let easy: Vec<f64> = (0..5).map(|f| eased.points_at(f)[0].point.0).collect();
    t.row(
        "FX-MSK-033's easy ease is late at a quarter, level at the half and early at three quarters",
        &format!("{easy:?}"),
        easy[1] < lefts[1] && (easy[2] - lefts[2]).abs() < 1e-12 && easy[3] > lefts[3],
    );
    let inside = masks_of(&load("fx_msk_034.json"), "cel")[0].clone();
    let both_ends: Vec<f64> = (0..5).map(|f| inside.points_at(f)[0].point.0).collect();
    t.row(
        "FX-MSK-034 keys at 1 and 3 only: before the first key and after the last, the shape holds",
        &format!("{both_ends:?}"),
        both_ends[0] == both_ends[1] && both_ends[4] == both_ends[3],
    );
    let bulge = masks_of(&load("fx_msk_035.json"), "cel")[0].clone();
    let handles: Vec<f64> = (0..5).map(|f| bulge.points_at(f)[1].out_handle.0).collect();
    t.row(
        "FX-MSK-035 keys the handles, not the points, so the right edge bows out a step at a time",
        &format!("{handles:?}"),
        handles.windows(2).all(|w| w[1] > w[0]),
    );
    t.row(
        "a mask asked for its shape at a frame gives back a mask that stands still there",
        &format!("{} key(s)", slide.at(2).keys.len()),
        slide.at(2).keys.is_empty() && slide.at(2).points == slide.points_at(2),
    );
    let still = masks_of(&load("fx_msk_001.json"), "cel")[0].clone();
    t.row(
        "and a path with no keys gives back its base at every frame",
        &format!("{} key(s)", still.keys.len()),
        still.keys.is_empty()
            && (-5..5).all(|f| still.points_at(f) == still.points),
    );

    // -----------------------------------------------------------------------------------
    t.heading("The file (document 19)");
    let document = load("fx_msk_033.json");
    let written = persist::to_json(document.project(), &persist::Preserved::default());
    let again = persist::load_str(&written).expect("a keyed path opens again");
    t.row(
        "fx_msk_033.json's keys survive a save and a load, to the last bit",
        &format!(
            "{} key(s), frames {:?}",
            masks_of(&again.document, "cel")[0].keys.len(),
            masks_of(&again.document, "cel")[0]
                .keys
                .iter()
                .map(|k| k.frame)
                .collect::<Vec<_>>()
        ),
        masks_of(&again.document, "cel") == masks_of(&document, "cel"),
    );
    let written_json: J = serde_json::from_str(&written).unwrap();
    let key = &written_json["compositions"][0]["layers"][0]["masks"][0]["path"]["keyframes"][0];
    t.row(
        "a written key is a frame, a whole outline and its interpolation, document 19's way",
        &format!(
            "frame {}, {} point(s), interp {}, ease {}",
            key["frame"],
            key["value"]["points"].as_array().map_or(0, |a| a.len()),
            key["interp"],
            key.get("ease").map_or("absent", |_| "present")
        ),
        key["frame"] == 0
            && key["value"]["points"].as_array().map_or(0, |a| a.len()) == 4
            && key["interp"] == "ease"
            && key["ease"].as_array().map_or(0, |a| a.len()) == 4,
    );
    t.row(
        "and no file with keys on a path is diagnosed any more",
        &format!("{:?}", again.warnings.iter().map(|d| d.id).collect::<Vec<_>>()),
        again.warnings.is_empty(),
    );

    // -----------------------------------------------------------------------------------
    t.heading("Refusals (D-77)");
    // A key that holds a different number of points is not a shape this build can interpolate,
    // and D-77 says so: adding or removing a point is a command's problem, not a file's.
    let mut document = load("fx_msk_001.json");
    let base = masks_of(&document, "cel")[0].clone();
    let key = |frame: i32, points: Vec<_>| MaskKey {
        frame,
        points,
        interp: Interp::Linear,
    };
    let set = |document: &mut Document, masks: Vec<Mask>| {
        document
            .apply(Command::SetMasks {
                composition: Id::new(COMP),
                layer_id: Id::new("cel"),
                masks,
            })
            .map(|c| c.label.clone())
            .map_err(|d| (d.id, d.detail))
    };
    let mut short = base.clone();
    let mut fewer = base.points.clone();
    fewer.pop();
    short.keys = vec![key(0, base.points.clone()), key(4, fewer)];
    let refused = set(&mut document, vec![short]);
    t.row(
        "a key holding a different number of points is refused, with MASK_INVALID_OUTLINE",
        &match &refused {
            Ok(label) => format!("accepted: {label}"),
            Err((id, detail)) => format!("{id:?}: {detail}"),
        },
        refused.as_ref().err().map(|(id, _)| *id) == Some(DiagnosticId::MaskInvalidOutline),
    );
    let mut twice = base.clone();
    twice.keys = vec![key(2, base.points.clone()), key(2, base.points.clone())];
    let refused = set(&mut document, vec![twice]);
    t.row(
        "two keys at one frame are refused the same way",
        &match &refused {
            Ok(label) => format!("accepted: {label}"),
            Err((id, detail)) => format!("{id:?}: {detail}"),
        },
        refused.as_ref().err().map(|(id, _)| *id) == Some(DiagnosticId::MaskInvalidOutline),
    );
    let before = document.project().clone();
    let mut jumbled = base.clone();
    let moved: Vec<_> = base
        .points
        .iter()
        .map(|p| {
            let mut p = *p;
            p.point.0 += 3.0;
            p
        })
        .collect();
    jumbled.keys = vec![key(4, moved), key(0, base.points.clone())];
    let accepted = set(&mut document, vec![jumbled]);
    let kept = masks_of(&document, "cel")[0].keys.clone();
    t.row(
        "keys given out of order are accepted and kept in frame order",
        &format!(
            "{accepted:?}; frames {:?}",
            kept.iter().map(|k| k.frame).collect::<Vec<_>>()
        ),
        accepted.is_ok() && kept.iter().map(|k| k.frame).collect::<Vec<_>>() == vec![0, 4],
    );
    let one_step = document.undo().is_some() && document.project() == &before;
    t.row(
        "and one undo gives back the mask as it was",
        if one_step {
            "the same project"
        } else {
            "different"
        },
        one_step,
    );

    // -----------------------------------------------------------------------------------
    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-24d_mask_key_table.md"), &t.out).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-24d_mask_key_table.md");
}
