//! B-111b: a camera that rides a layer, in the core, against D-171.
//!
//! Writes `verification/B-111_camera_rig_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/camera_rig/expected_camera_rig.json`, written by `tools/camera_rig_reference.py`,
//! which carries the camera's point through the parent chain with `parent_reference.py` and
//! does D-58's two lines by hand. Document 25 prints the same numbers as FX-RIG-001 to 031.
//! Nothing here is a snapshot of a run.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{camera_at, render_frame, screen_transform};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::model::{Id, Value};
use anime_compositor::persist;
use anime_compositor::WorkingBuffer;

const COMP: &str = "comp-main";

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/camera_rig")
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

fn render(document: &Document, frame: i32, tile: usize) -> WorkingBuffer {
    let mut log = FrameLog::new(8);
    render_frame(
        document.project(),
        &Id::new(COMP),
        frame,
        &root(),
        tile,
        &mut log,
    )
    .unwrap_or_else(|d| panic!("frame {frame} renders: {}", d.message))
}

fn largest_difference(buffer: &WorkingBuffer, expected: &J) -> f64 {
    let expected: Vec<f64> = expected
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|p| p.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()))
        .collect();
    assert_eq!(expected.len(), buffer.data().len(), "the extents agree");
    buffer
        .data()
        .iter()
        .zip(&expected)
        .map(|(a, b)| (*a as f64 - b).abs())
        .fold(0.0, f64::max)
}

/// A fixture's project with its composition changed by `change`, opened from memory.
fn edited(file: &str, change: impl Fn(&mut serde_json::Map<String, J>)) -> Document {
    let mut json: J = serde_json::from_str(&fs::read_to_string(root().join(file)).unwrap()).unwrap();
    change(json["compositions"][0].as_object_mut().unwrap());
    persist::load_str(&json.to_string())
        .unwrap_or_else(|d| panic!("{file}, changed, opens: {} {}", d.message, d.detail))
        .document
}

fn pair(v: &J) -> (f64, f64) {
    (v[0].as_f64().unwrap(), v[1].as_f64().unwrap())
}

#[test]
fn b111_camera_rig() {
    let expected: J = serde_json::from_str(
        &fs::read_to_string(root().join("expected_camera_rig.json")).unwrap(),
    )
    .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let pixel_tolerance = expected["pixel_tolerance"].as_f64().unwrap();
    let comp_id = Id::new(COMP);

    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-111b: a camera that rides a layer\n\nD-171, proposed on 2026-09-27. Every expected \
         number is `Fixtures/camera_rig/expected_camera_rig.json`, written by \
         `tools/camera_rig_reference.py` before this code existed and printed in document 25 as \
         FX-RIG-001 to 031. Frames are compared sample by sample to within 1e-6, as FX-NULL's \
         are, because a picture is decoded and drawn in 32-bit numbers; points to within 1e-9. \
         The answer given is the largest difference.\n",
    );

    // -----------------------------------------------------------------------------------
    t.heading("FX-RIG-001 to 005, rendered whole (document 25)");
    for (name, case) in expected["cases"].as_object().unwrap() {
        let document = load(case["project"].as_str().unwrap());
        for (frame, pixels) in case["frames"].as_object().unwrap() {
            let frame: i32 = frame.parse().unwrap();
            let d = largest_difference(&render(&document, frame, 64), pixels);
            let tiled = render(&document, frame, 1).data() == render(&document, frame, 64).data();
            t.row(
                &format!("{name} frame {frame}: {}", case["says"].as_str().unwrap()),
                &format!(
                    "largest difference {d:.1e}; tiles of 1 {}",
                    if tiled { "byte-identical" } else { "differ" }
                ),
                d <= pixel_tolerance && tiled,
            );
        }
    }
    let alone = render(&edited("fx_rig_001.json", |c| drop(c.remove("camera"))), 0, 64);
    let same = render(&load("fx_rig_001.json"), 0, 64).data() == alone.data();
    t.row(
        "FX-RIG-001 is not merely close: the frame is byte for byte the frame with no camera",
        if same { "identical" } else { "different" },
        same,
    );

    // -----------------------------------------------------------------------------------
    t.heading("FX-RIG-010 to 012, where the corners land (document 25)");
    for (name, case) in expected["points"].as_object().unwrap() {
        let document = load(case["project"].as_str().unwrap());
        let comp = document.project().composition(&comp_id).unwrap();
        let layer = Id::new(case["layer"].as_str().unwrap());
        for (frame, screen) in case["screen"].as_object().unwrap() {
            let frame: i32 = frame.parse().unwrap();
            let cam = camera_at(comp, frame).unwrap();
            let want = pair(&case["camera"]["position"]);
            let want_depth = case["camera"]["depth"].as_f64().unwrap();
            let mut d = (cam.position.0 - want.0)
                .abs()
                .max((cam.position.1 - want.1).abs())
                .max((cam.depth - want_depth).abs());
            let m = screen_transform(comp, &layer, frame).unwrap();
            let mut got = Vec::new();
            for (corner, on_screen) in case["corners"]
                .as_array()
                .unwrap()
                .iter()
                .zip(screen.as_array().unwrap())
            {
                let (x, y) = pair(corner);
                let (sx, sy) = m.apply(x, y);
                let (ex, ey) = pair(on_screen);
                d = d.max((sx - ex).abs()).max((sy - ey).abs());
                got.push(format!("({sx}, {sy})"));
            }
            t.row(
                &format!("{name} frame {frame}: {}", case["says"].as_str().unwrap()),
                &format!(
                    "camera at ({}, {}), depth {}; corners {}; largest difference {d:.1e}",
                    cam.position.0,
                    cam.position.1,
                    cam.depth,
                    got.join(", ")
                ),
                d <= tolerance,
            );
        }
    }

    // -----------------------------------------------------------------------------------
    t.heading("FX-RIG-020, opened with a warning (document 28)");
    for (name, case) in expected["warned"].as_object().unwrap() {
        let loaded = persist::load(&root().join(case["project"].as_str().unwrap()))
            .unwrap_or_else(|d| panic!("{name} opens: {}", d.message));
        let codes: Vec<String> = loaded
            .warnings
            .iter()
            .map(|w| format!("{:?}", w.id))
            .collect();
        let warned = loaded
            .warnings
            .iter()
            .any(|w| w.id == DiagnosticId::ParentReferenceMissing);
        let comp = loaded.document.project().composition(&comp_id).unwrap();
        let kept = comp.camera.as_ref().and_then(|c| c.parent.clone());
        let mut d = 0.0;
        for (frame, pixels) in case["frames"].as_object().unwrap() {
            let frame: i32 = frame.parse().unwrap();
            d = largest_difference(&render(&loaded.document, frame, 64), pixels);
        }
        let written = persist::to_json(loaded.document.project(), &loaded.preserved);
        let written_json: J = serde_json::from_str(&written).unwrap();
        let written_parent = written_json["compositions"][0]["camera"]["parent"].clone();
        t.row(
            &format!("{name}: {}", case["says"].as_str().unwrap()),
            &format!(
                "warnings {codes:?}; parent kept as {kept:?} and written back as {written_parent}; \
                 largest difference {d:.1e}"
            ),
            warned && kept == Some(Id::new("gone")) && written_parent == "gone" && d <= pixel_tolerance,
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("FX-RIG-030 and 031, refused whole (D-171)");
    for (name, case) in expected["refused"].as_object().unwrap() {
        let got = persist::load(&root().join(case["project"].as_str().unwrap())).err();
        t.row(
            &format!("{name}: {}", case["says"].as_str().unwrap()),
            &got.as_ref().map_or("opened".to_string(), |d| {
                format!("{:?}: {}", d.id, d.detail)
            }),
            got.map(|d| d.id) == Some(DiagnosticId::ProjectSchemaInvalid)
                && case["code"] == "PROJECT_SCHEMA_INVALID",
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("The file (document 19)");
    let document = load("fx_rig_004.json");
    let written = persist::to_json(document.project(), &persist::Preserved::default());
    let written_json: J = serde_json::from_str(&written).unwrap();
    let parent = written_json["compositions"][0]["camera"]["parent"].clone();
    t.row(
        "fx_rig_004.json's camera written back with its parent",
        &format!("parent {parent}"),
        parent == "rig",
    );
    let reopened = persist::load_str(&written).expect("what this build wrote, it opens");
    let equal = reopened.document.project() == document.project();
    t.row(
        "and it opens again as the same project",
        if equal { "equal" } else { "different" },
        equal,
    );

    // -----------------------------------------------------------------------------------
    t.heading("Commands (document 24)");
    // Keep place: the camera leaves the null it rides, and then rides it again, and neither
    // moves the picture.
    let mut document = load("fx_rig_003.json");
    let frames = |d: &Document| (0..3).map(|f| render(d, f, 64).data().to_vec()).collect::<Vec<_>>();
    let before = frames(&document);
    let before_project = document.project().clone();
    let off = document
        .apply(Command::SetCameraParent {
            composition: comp_id.clone(),
            parent: None,
            frame: 0,
            keep_place: true,
        })
        .is_ok();
    let cam = document
        .project()
        .composition(&comp_id)
        .unwrap()
        .camera
        .clone()
        .unwrap();
    let shot_0 = render(&document, 0, 64).data() == before[0].as_slice();
    t.row(
        "camera.set_parent to none, keeping place at frame 0: the camera stays over the middle, so frame 0 is as it was",
        &format!(
            "done {off}; position {:?}; frame 0 {}",
            cam.position.base(),
            if shot_0 { "unchanged" } else { "moved" }
        ),
        off && cam.parent.is_none() && shot_0,
    );
    let stays = render(&document, 2, 64).data() == before[0].as_slice();
    t.row(
        "and the null's slide no longer moves the shot: frame 2 is frame 0",
        if stays { "the same" } else { "different" },
        stays,
    );
    let back = document
        .apply(Command::SetCameraParent {
            composition: comp_id.clone(),
            parent: Some(Id::new("rig")),
            frame: 0,
            keep_place: true,
        })
        .is_ok();
    let again = frames(&document) == before;
    t.row(
        "camera.set_parent back to the null at frame 0: the slide comes back, every frame as it was",
        &format!("done {back}; frames {}", if again { "as before" } else { "different" }),
        back && again,
    );
    let undone = document.undo().is_some()
        && document.undo().is_some()
        && document.project() == &before_project;
    t.row(
        "and two undos give back the file as it was",
        if undone { "the same project" } else { "different" },
        undone,
    );

    // Keep place through a turned and grown parent, with the depth too.
    let mut document = load("fx_rig_010.json");
    let comp = document.project().composition(&comp_id).unwrap();
    let before = camera_at(comp, 0).unwrap();
    let off = document
        .apply(Command::SetCameraParent {
            composition: comp_id.clone(),
            parent: None,
            frame: 0,
            keep_place: true,
        })
        .is_ok();
    let comp = document.project().composition(&comp_id).unwrap();
    let after = camera_at(comp, 0).unwrap();
    let d = (after.position.0 - before.position.0)
        .abs()
        .max((after.position.1 - before.position.1).abs())
        .max((after.depth - before.depth).abs());
    t.row(
        "FX-RIG-010's camera taken off the turned null, keeping place: it stands where it stood",
        &format!(
            "done {off}; from ({}, {}) depth {} to ({}, {}) depth {}; largest difference {d:.1e}",
            before.position.0,
            before.position.1,
            before.depth,
            after.position.0,
            after.position.1,
            after.depth
        ),
        off && d <= tolerance,
    );
    let mut document = load("fx_rig_011.json");
    let comp = document.project().composition(&comp_id).unwrap();
    let before = camera_at(comp, 0).unwrap();
    let off = document
        .apply(Command::SetCameraParent {
            composition: comp_id.clone(),
            parent: None,
            frame: 0,
            keep_place: true,
        })
        .is_ok();
    let comp = document.project().composition(&comp_id).unwrap();
    let after = camera_at(comp, 0).unwrap();
    let depth = comp.camera.as_ref().unwrap().depth.base();
    t.row(
        "FX-RIG-011's camera taken off the null one lens length back: its own depth takes the null's",
        &format!("depth before {} and after {}; written {depth:?}", before.depth, after.depth),
        off && (after.depth - before.depth).abs() <= tolerance
            && depth == Value::Scalar(before.depth),
    );

    // FX-RIG-031's file with the camera's parent taken off, so it opens with its audio layer.
    let mut document = edited("fx_rig_031.json", |c| {
        drop(c["camera"].as_object_mut().unwrap().remove("parent"))
    });
    let refusals: Vec<(&str, Option<Id>, DiagnosticId)> = vec![
        (
            "a parent that is not in the composition: PARENT_REFERENCE_MISSING",
            Some(Id::new("gone")),
            DiagnosticId::ParentReferenceMissing,
        ),
        (
            "the audio layer: COMMAND_INVALID_VALUE",
            Some(Id::new("sound")),
            DiagnosticId::CommandInvalidValue,
        ),
    ];
    for (what, parent, code) in refusals {
        let before = document.project().clone();
        let got = document
            .apply(Command::SetCameraParent {
                composition: comp_id.clone(),
                parent,
                frame: 0,
                keep_place: true,
            })
            .err()
            .map(|d| d.id);
        t.row(
            &format!("camera.set_parent to {what}, and nothing changes"),
            &format!("{got:?}"),
            got == Some(code) && document.project() == &before,
        );
    }

    // -----------------------------------------------------------------------------------
    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-111_camera_rig_table.md"), &t.out).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-111_camera_rig_table.md");
}
