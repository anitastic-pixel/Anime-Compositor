//! B-23b: solid layers in the core, against D-74.
//!
//! Writes `verification/B-23b_solid_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/solid/expected_solid.json`, written by `tools/solid_reference.py`, which renders
//! each six-by-two frame pixel by pixel from document 21 and moves by whole pixels only.
//! Document 25 prints the same numbers as FX-SOL-001 to 008; FX-SOL-020 to 029 are files a
//! build must refuse whole. The tolerance is the catalogue's, 1e-6. Nothing here is a snapshot
//! of a run.
//!
//! # What is deliberately not here
//!
//! The window: New Solid in the Layer menu, Ctrl+Y and the colour and size in the layer's panel
//! are B-23c.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::model::{Id, Interp, Keyframe, Layer, LayerKind, Solid, Value};
use anime_compositor::persist;
use anime_compositor::time::ExposureSpan;
use anime_compositor::WorkingBuffer;

const COMP: &str = "comp-main";

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/solid")
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

/// The keys of the layer whose `"id"` is `id`, in the order the text spells them. serde_json's
/// map sorts its keys, so the order is read from the pretty-printed text itself.
fn keys_in_order(text: &str, id: &str) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    let indent_of = |l: &str| l.len() - l.trim_start().len();
    let at = lines
        .iter()
        .position(|l| l.trim() == format!("\"id\": \"{id}\","))
        .unwrap();
    let indent = indent_of(lines[at]);
    // Back to the layer's opening brace, then forward to its closing one.
    let start = (0..at)
        .rev()
        .find(|&i| indent_of(lines[i]) < indent)
        .unwrap()
        + 1;
    lines[start..]
        .iter()
        .take_while(|l| indent_of(l) >= indent)
        .filter(|l| indent_of(l) == indent && l.trim_start().starts_with('"'))
        .map(|l| l.trim_start()[1..].split('"').next().unwrap().to_string())
        .collect()
}

fn solid_of(document: &Document, id: &str) -> Layer {
    document
        .project()
        .composition(&Id::new(COMP))
        .unwrap()
        .layer(&Id::new(id))
        .unwrap()
        .clone()
}

#[test]
fn b23b_solid() {
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root().join("expected_solid.json")).unwrap())
            .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();

    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-23b: solid layers\n\nD-74, accepted by the owner on 2026-09-19. Every expected \
         pixel is `Fixtures/solid/expected_solid.json`, written by `tools/solid_reference.py` \
         before this code existed and printed in document 25 as FX-SOL-001 to 008. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 1e-6. FX-SOL-020 to 029 are files the build \
         must refuse whole.\n",
    );

    // -----------------------------------------------------------------------------------
    t.heading("FX-SOL-001 to 008, rendered whole (document 25)");
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
    t.heading("FX-SOL-020 to 029, refused whole (D-74)");
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
    let document = load("fx_sol_006.json");
    let written = persist::to_json(document.project(), &persist::Preserved::default());
    let written_json: J = serde_json::from_str(&written).unwrap();
    let layer = written_json["compositions"][0]["layers"]
        .as_array()
        .unwrap()
        .iter()
        .find(|l| l["id"] == "matte")
        .unwrap();
    let keys = keys_in_order(&written, "matte");
    let file_text = fs::read_to_string(root().join("fx_sol_006.json")).unwrap();
    let file: J = serde_json::from_str(&file_text).unwrap();
    let file_keys = keys_in_order(&file_text, "matte");
    t.row(
        "fx_sol_006.json's solid written back: the fixture's keys in the fixture's order, no asset_id, exposures or source offset",
        &keys.join(", "),
        keys == file_keys,
    );
    t.row(
        "and its record written back",
        &layer["solid"].to_string(),
        layer["solid"] == file["compositions"][0]["layers"][0]["solid"],
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
    let mut document = load("fx_sol_001.json");
    let blue = Solid {
        color: [0.2, 0.5, 0.8],
        width: 6,
        height: 2,
    };
    let new_layer = Layer::solid(Id::new("solid-2"), "Solid 2", blue, 6, 2, 0, 3);
    let added = document
        .apply(Command::AddLayer {
            composition: Id::new(COMP),
            layer: Box::new(new_layer.clone()),
            index: 1,
        })
        .is_ok();
    let (anchor, position) = (
        new_layer.transform.anchor.value_at(0),
        new_layer.transform.position.value_at(0),
    );
    t.row(
        "layer.add_solid: a new solid needs no asset; its anchor is its own centre and its position the composition's",
        &format!("added {added}, anchor {anchor:?}, position {position:?}"),
        added && anchor == Value::Vec2(3.0, 1.0) && position == Value::Vec2(3.0, 1.0),
    );
    let mut too_big = new_layer.clone();
    too_big.id = Id::new("solid-3");
    too_big.solid = Some(Solid {
        width: 8193,
        ..blue
    });
    let refused = document
        .apply(Command::AddLayer {
            composition: Id::new(COMP),
            layer: Box::new(too_big),
            index: 0,
        })
        .err()
        .map(|d| (d.id, d.message));
    t.row(
        "layer.add_solid 8193 wide: COMMAND_INVALID_VALUE, in a sentence",
        &format!("{refused:?}"),
        refused.is_some_and(|(id, _)| id == DiagnosticId::CommandInvalidValue),
    );

    // Two anchor keys, so the rule is seen on keys as well as on the base.
    let mut document = load("fx_sol_001.json");
    let set = |document: &mut Document, solid: Solid| {
        document
            .apply(Command::SetSolid {
                composition: Id::new(COMP),
                layer_id: Id::new("solid"),
                solid,
            })
            .map(|_| ())
            .map_err(|d| (d.id, d.message))
    };
    let _ = set(&mut document, blue);
    let before = document.project().clone();
    let red = Solid {
        color: [1.0, 0.0, 0.0],
        width: 12,
        height: 8,
    };
    let applied = set(&mut document, red);
    let layer = solid_of(&document, "solid");
    t.row(
        "solid.set red, 12 by 8, on the 6 by 2 solid: its record, and its anchor at the same fraction (3, 1) to (6, 4)",
        &format!(
            "{applied:?}; {:?}, anchor {:?}",
            layer.solid,
            layer.transform.anchor.base()
        ),
        applied.is_ok()
            && layer.solid == Some(red)
            && layer.transform.anchor.base() == Value::Vec2(6.0, 4.0),
    );
    let one_step = document.undo().is_some() && document.project() == &before;
    t.row(
        "and one undo gives back the file as it was",
        if one_step {
            "the same project"
        } else {
            "different"
        },
        one_step,
    );
    for (what, bad) in [
        (
            "colour 1.5",
            Solid {
                color: [1.5, 0.0, 0.0],
                ..blue
            },
        ),
        ("width 0", Solid { width: 0, ..blue }),
        (
            "height 8193",
            Solid {
                height: 8193,
                ..blue
            },
        ),
    ] {
        let refused = set(&mut document, bad);
        t.row(
            &format!("solid.set {what}: COMMAND_INVALID_VALUE, in a sentence, and nothing changes"),
            &format!("{refused:?}"),
            matches!(&refused, Err((DiagnosticId::CommandInvalidValue, _)))
                && document.project() == &before,
        );
    }
    let mut on_drawing = load("fx_sol_003.json");
    let refused = on_drawing
        .apply(Command::SetSolid {
            composition: Id::new(COMP),
            layer_id: Id::new("bg"),
            solid: blue,
        })
        .err()
        .map(|d| d.id);
    t.row(
        "solid.set on a drawing: COMMAND_INVALID_VALUE",
        &format!("{refused:?}"),
        refused == Some(DiagnosticId::CommandInvalidValue),
    );
    let refused = document
        .apply(Command::SetExposureSpans {
            composition: Id::new(COMP),
            layer_id: Id::new("solid"),
            spans: vec![ExposureSpan {
                start_frame: 0,
                end_frame_exclusive: 3,
                drawing_number: 1,
            }],
        })
        .err()
        .map(|d| d.id);
    t.row(
        "exposures set on a solid, which has none: COMMAND_INVALID_VALUE rather than dropped on save",
        &format!("{refused:?}"),
        refused == Some(DiagnosticId::CommandInvalidValue),
    );

    // The anchor keys, scaled with the base.
    let mut keyed = load("fx_sol_001.json");
    keyed
        .apply(Command::SetKeyframe {
            composition: Id::new(COMP),
            target: anime_compositor::command::Target::Layer(Id::new("solid")),
            prop: anime_compositor::model::Prop::Anchor,
            frame: 2,
            value: Value::Vec2(0.0, 2.0),
            interp: Interp::Linear,
            spatial: None,
        })
        .expect("an anchor key");
    set(
        &mut keyed,
        Solid {
            width: 12,
            height: 4,
            ..blue
        },
    )
    .unwrap();
    let keys: Vec<Keyframe> = solid_of(&keyed, "solid")
        .transform
        .anchor
        .keyframes()
        .to_vec();
    let values: Vec<Value> = keys.iter().map(|k| k.value).collect();
    t.row(
        "a keyed anchor: every key moves to the same fraction too, (0, 2) to (0, 4)",
        &format!("{values:?}"),
        values.contains(&Value::Vec2(0.0, 4.0)),
    );
    t.row(
        "the kind reads back as solid",
        solid_of(&keyed, "solid").kind.as_str(),
        solid_of(&keyed, "solid").kind == LayerKind::Solid,
    );

    // -----------------------------------------------------------------------------------
    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-23b_solid_table.md"), &t.out).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-23b_solid_table.md");
}
