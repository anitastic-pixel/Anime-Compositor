//! B-26b: null layers in the core, against D-82.
//!
//! Writes `verification/B-26b_null_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/null/expected_null.json`, written by `tools/null_reference.py`, which carries the
//! drawing through the parent chain one step at a time and moves by whole pixels only.
//! Document 25 prints the same numbers as FX-NULL-001 to 007; FX-NULL-020 to 029 are files a
//! build must refuse whole. The tolerance is the catalogue's, 1e-6. Nothing here is a snapshot
//! of a run.
//!
//! # What is deliberately not here
//!
//! The window: New Null in the Layer menu, Ctrl+Alt+Shift+Y and the outline on the picture are
//! B-26c.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{Effect, EffectInstance};
use anime_compositor::model::{BlendMode, Id, Layer, LayerKind, Value};
use anime_compositor::persist;
use anime_compositor::WorkingBuffer;

const COMP: &str = "comp-main";

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/null")
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

fn layer_of(document: &Document, id: &str) -> Layer {
    document
        .project()
        .composition(&Id::new(COMP))
        .unwrap()
        .layer(&Id::new(id))
        .unwrap()
        .clone()
}

/// A command's refusal as its diagnostic, with the project checked unchanged.
fn refused(document: &mut Document, command: Command) -> (Option<DiagnosticId>, bool) {
    let before = document.project().clone();
    let got = document.apply(command).err().map(|d| d.id);
    (got, document.project() == &before)
}

#[test]
fn b26b_null() {
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root().join("expected_null.json")).unwrap())
            .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();

    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-26b: null layers\n\nD-82, accepted by the owner on 2026-09-23. Every expected \
         pixel is `Fixtures/null/expected_null.json`, written by `tools/null_reference.py` \
         before this code existed and printed in document 25 as FX-NULL-001 to 007. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 1e-6. FX-NULL-020 to 029 are files the build \
         must refuse whole.\n",
    );

    // -----------------------------------------------------------------------------------
    t.heading("FX-NULL-001 to 007, rendered whole (document 25)");
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
                d <= tolerance && tiled,
            );
        }
    }

    // -----------------------------------------------------------------------------------
    t.heading("FX-NULL-020 to 029, refused whole (D-82)");
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
    let document = load("fx_null_007.json");
    let written = persist::to_json(document.project(), &persist::Preserved::default());
    let written_json: J = serde_json::from_str(&written).unwrap();
    let layers = written_json["compositions"][0]["layers"].as_array().unwrap();
    let null = layers.iter().find(|l| l["id"] == "null").unwrap();
    let drawn_keys = ["asset_id", "exposure_spans", "source_offset_frames", "solid", "shapes"];
    let present: Vec<&str> = drawn_keys
        .iter()
        .copied()
        .filter(|k| null.get(*k).is_some())
        .collect();
    t.row(
        "fx_null_007.json's inner null written back as kind null, with its parent, and none of asset_id, exposures, source offset, solid or shapes",
        &format!(
            "kind {}, parent {}, drawing keys present {present:?}",
            null["kind"], null["parent"]
        ),
        null["kind"] == "null" && null["parent"] == "outer" && present.is_empty(),
    );
    let reopened = persist::load_str(&written).expect("what this build wrote, it opens");
    t.row(
        "and it opens again as the same project",
        if reopened.document.project() == document.project() {
            "equal"
        } else {
            "different"
        },
        reopened.document.project() == document.project(),
    );

    // -----------------------------------------------------------------------------------
    t.heading("Commands (document 24)");
    let mut document = load("fx_null_002.json");
    let new_null = Layer::null(Id::new("null-2"), "Null 1", 6, 2, 0, 3);
    let added = document
        .apply(Command::AddLayer {
            composition: Id::new(COMP),
            layer: Box::new(new_null.clone()),
            index: 2,
        })
        .is_ok();
    let (anchor, position) = (
        new_null.transform.anchor.value_at(0),
        new_null.transform.position.value_at(0),
    );
    t.row(
        "layer.add_null: a new null needs no asset; its anchor is the middle of its 100 by 100 outline and its position the composition's centre",
        &format!("added {added}, anchor {anchor:?}, position {position:?}"),
        added && anchor == Value::Vec2(50.0, 50.0) && position == Value::Vec2(3.0, 1.0),
    );
    let bg_alone = load("fx_null_002.json");
    let same = render(&document, 0, 64).data() == render(&bg_alone, 0, 64).data();
    t.row(
        "and the frame with a second null added is the frame without it, byte for byte",
        if same { "identical" } else { "different" },
        same,
    );
    let mut dressed = new_null.clone();
    dressed.id = Id::new("null-3");
    dressed.blend_mode = BlendMode::Multiply;
    let (got, unchanged) = refused(
        &mut document,
        Command::AddLayer {
            composition: Id::new(COMP),
            layer: Box::new(dressed),
            index: 0,
        },
    );
    t.row(
        "layer.add of a null carrying a blend mode: COMMAND_INVALID_VALUE, and nothing changes",
        &format!("{got:?}"),
        got == Some(DiagnosticId::CommandInvalidValue) && unchanged,
    );

    let on_null = |layer: &str| (Id::new(COMP), Id::new(layer));
    let (c, l) = on_null("null");
    let attempts: Vec<(&str, Command)> = vec![
        (
            "a mask",
            Command::SetMasks {
                composition: c.clone(),
                layer_id: l.clone(),
                masks: vec![anime_compositor::mask::Mask::polygon(vec![
                    (0.0, 0.0),
                    (3.0, 0.0),
                    (3.0, 2.0),
                ])],
            },
        ),
        (
            "an effect",
            Command::AddEffect {
                composition: c.clone(),
                layer_id: l.clone(),
                effect: EffectInstance::new(Id::new("fx-1"), Effect::Exposure { stops: 1.0 }),
                index: None,
            },
        ),
        (
            "blend mode multiply",
            Command::SetBlendMode {
                composition: c.clone(),
                layer_id: l.clone(),
                mode: BlendMode::Multiply,
            },
        ),
        (
            "a matte of its own",
            Command::SetMatte {
                composition: c.clone(),
                layer_id: l.clone(),
                matte: Some(Id::new("bg")),
                matte_only: false,
            },
        ),
        (
            "exposures",
            Command::SetExposureSpans {
                composition: c.clone(),
                layer_id: l.clone(),
                spans: vec![],
            },
        ),
    ];
    for (what, command) in attempts {
        let (got, unchanged) = refused(&mut document, command);
        t.row(
            &format!("giving the null {what}: COMMAND_INVALID_VALUE, in a sentence, and nothing changes"),
            &format!("{got:?}"),
            got == Some(DiagnosticId::CommandInvalidValue) && unchanged,
        );
    }
    let (got, unchanged) = refused(
        &mut document,
        Command::SetMatte {
            composition: Id::new(COMP),
            layer_id: Id::new("bg"),
            matte: Some(Id::new("null")),
            matte_only: false,
        },
    );
    t.row(
        "the drawing given the null as its matte: COMMAND_INVALID_VALUE, and nothing changes",
        &format!("{got:?}"),
        got == Some(DiagnosticId::CommandInvalidValue) && unchanged,
    );
    let renamed = document
        .apply(Command::RenameLayer {
            composition: Id::new(COMP),
            layer_id: Id::new("null"),
            name: "Head rig".into(),
        })
        .is_ok();
    t.row(
        "and what a null does have still works: renaming it",
        &format!("{renamed}"),
        renamed && layer_of(&document, "null").name == "Head rig",
    );

    // Parenting to a null keeps the drawing in place, and moving the null then carries it.
    let mut document = load("fx_null_002.json");
    let before = render(&document, 0, 64).data().to_vec();
    let before_project = document.project().clone();
    let parented = document
        .apply(Command::SetParent {
            composition: Id::new(COMP),
            layer_id: Id::new("bg"),
            parent: Some(Id::new("null")),
            frame: 0,
            keep_place: true,
        })
        .is_ok();
    let kept = render(&document, 0, 64).data() == before.as_slice();
    t.row(
        "parent.set: the drawing parented to the null at (52, 50) stays exactly where it was",
        &format!(
            "parented {parented}; frame {}",
            if kept { "unchanged" } else { "moved" }
        ),
        parented && kept,
    );
    let one_step = document.undo().is_some() && document.project() == &before_project;
    t.row(
        "and one undo gives back the file as it was",
        if one_step { "the same project" } else { "different" },
        one_step,
    );
    t.row(
        "the kind reads back as null",
        layer_of(&document, "null").kind.as_str(),
        layer_of(&document, "null").kind == LayerKind::Null,
    );

    // -----------------------------------------------------------------------------------
    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-26b_null_table.md"), &t.out).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-26b_null_table.md");
}
