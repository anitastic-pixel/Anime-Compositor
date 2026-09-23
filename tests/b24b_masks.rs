//! B-24b: masks in the core, against D-77.
//!
//! Writes `verification/B-24b_mask_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/masks/expected_masks.json`, written by `tools/mask_reference.py`, which flattens
//! each outline, samples it on ADR-016's 4x4 grid and renders each six-by-two frame from
//! document 21 — a second implementation, written from D-77 rather than from `src/mask.rs`.
//! Document 25 prints the same numbers as FX-MSK-001 to 019; FX-MSK-020 to 030 are files a build
//! must refuse whole. The tolerance is the catalogue's, 1e-6. Nothing here is a snapshot of a
//! run.
//!
//! FX-MSK-001 is the load-bearing one: it is B-06's rectangle written D-77's way, and every
//! number in it is a number this build gave before D-77 existed.
//!
//! # What is deliberately not here
//!
//! The window — the pen tool, its G shortcut, the mask rows in the layer panel — is B-24c, and
//! a path that moves is B-24d, whose own evidence is `verification/B-24d_mask_key_table.md`.
//! The section this test used to carry, where a keyed path was kept, drawn at its base and
//! diagnosed, went with it on 2026-09-20: the build draws a path moving now.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::mask::{Mask, MaskMode, MaskPoint};
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::WorkingBuffer;

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

fn render(document: &Document, frame: i32, tile: usize) -> (WorkingBuffer, Vec<DiagnosticId>) {
    let mut log = FrameLog::new(8);
    let buffer = render_frame(
        document.project(),
        &Id::new(COMP),
        frame,
        &root(),
        tile,
        &mut log,
    )
    .unwrap_or_else(|d| panic!("frame {frame} renders: {}", d.message));
    let ids = log.ids_at(frame);
    (buffer, ids)
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

#[test]
fn b24b_masks() {
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
        "# B-24b: masks in the core\n\nD-77, accepted by the owner on 2026-09-19. Every expected \
         pixel is `Fixtures/masks/expected_masks.json`, written by `tools/mask_reference.py` \
         before this code existed and printed in document 25 as FX-MSK-001 to 019; every case is \
         drawn in `verification/B-24a proposal/mask_cases.png`. The build's frame is compared \
         sample by sample; the answer is the largest difference over all of them, against the \
         catalogue's tolerance of 1e-6. FX-MSK-020 to 030 are files the build must refuse \
         whole.\n\nFX-MSK-001 is the one that says nothing moved: it is the rectangle B-06 drew, \
         written D-77's way.\n",
    );

    // -----------------------------------------------------------------------------------
    t.heading("FX-MSK-001 to 019, rendered whole (document 25)");
    let mut cases: Vec<(&String, &J)> = expected["cases"].as_object().unwrap().iter().collect();
    cases.sort_by_key(|(name, _)| name.as_str());
    for (name, case) in cases {
        let document = load(case["project"].as_str().unwrap());
        for (frame, pixels) in case["frames"].as_object().unwrap() {
            let frame: i32 = frame.parse().unwrap();
            let (built, ids) = render(&document, frame, 64);
            let d = largest_difference(&built, pixels);
            let tiled = render(&document, frame, 1).0.data() == built.data();
            // A case with a diagnostic must raise it on every frame, and one without must raise
            // none: a mask quietly bypassed is exactly what document 28 forbids.
            let wanted = case.get("diagnostic").and_then(J::as_str);
            let said: Vec<&str> = ids.iter().map(|i| i.as_str()).collect();
            let diagnosed = match wanted {
                Some(id) => said == vec![id],
                None => said.is_empty(),
            };
            t.row(
                &format!("{name} frame {frame}: {}", case["says"].as_str().unwrap()),
                &format!(
                    "largest difference {d:.1e}; tiles of 1 {}; said {}",
                    if tiled { "byte-identical" } else { "differ" },
                    if said.is_empty() {
                        "nothing".to_string()
                    } else {
                        said.join(", ")
                    }
                ),
                d <= tolerance && tiled && diagnosed,
            );
        }
    }

    // -----------------------------------------------------------------------------------
    t.heading("FX-MSK-020 to 030, refused whole (D-77)");
    let mut refused: Vec<(&String, &J)> = expected["refused"].as_object().unwrap().iter().collect();
    refused.sort_by_key(|(name, _)| name.as_str());
    for (name, case) in refused {
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
    // FX-MSK-002 holds the old single `mask` key, which is the conversion D-77 states.
    let old = load("fx_msk_002.json");
    let converted = masks_of(&old, "cel");
    t.row(
        "fx_msk_002.json's old `mask` key reads as one Add mask at full opacity, no feather, no expansion, no handles, named Mask 1",
        &format!(
            "{} mask, name {:?}, mode {}, opacity {}, feather {}, expansion {}, handles all zero: {}",
            converted.len(),
            converted[0].name,
            converted[0].mode.as_str(),
            converted[0].opacity,
            converted[0].feather_px,
            converted[0].expansion_px,
            converted[0]
                .points
                .iter()
                .all(|p| p.in_handle == (0.0, 0.0) && p.out_handle == (0.0, 0.0))
        ),
        converted.len() == 1
            && converted[0].name == "Mask 1"
            && converted[0].mode == MaskMode::Add
            && converted[0].opacity == 1.0
            && converted[0].feather_px == 0.0
            && converted[0].expansion_px == 0.0
            && converted[0]
                .points
                .iter()
                .all(|p| p.in_handle == (0.0, 0.0) && p.out_handle == (0.0, 0.0)),
    );
    let written = persist::to_json(old.project(), &persist::Preserved::default());
    let written_json: J = serde_json::from_str(&written).unwrap();
    let layer = &written_json["compositions"][0]["layers"][0];
    t.row(
        "and saving it writes `masks` and no `mask` key at all",
        &format!(
            "masks: {}, mask: {}",
            layer["masks"].is_array(),
            layer.get("mask").map_or("absent", |_| "present")
        ),
        layer["masks"].is_array() && layer.get("mask").is_none(),
    );
    t.row(
        "the written mask holds its path as a base of points with both handles",
        &layer["masks"][0]["path"]["base"]["points"][0].to_string(),
        layer["masks"][0]["path"]["base"]["points"][0]
            .get("in")
            .is_some()
            && layer["masks"][0]["path"]["base"]["points"][0]
                .get("out")
                .is_some(),
    );
    let reopened = persist::load_str(&written).expect("what this build wrote, it opens");
    t.row(
        "and it opens again as the same project",
        if reopened.document.project() == old.project() {
            "equal"
        } else {
            "different"
        },
        reopened.document.project() == old.project(),
    );
    // A curve, which is the half of the file B-06 had no shape for.
    let curve = load("fx_msk_016.json");
    let written = persist::to_json(curve.project(), &persist::Preserved::default());
    let again = persist::load_str(&written).expect("a curved mask opens again");
    t.row(
        "fx_msk_016.json's handles survive a save and a load, to the last bit",
        &format!("{:?}", masks_of(&again.document, "cel")[0].points[0]),
        masks_of(&again.document, "cel") == masks_of(&curve, "cel"),
    );

    // -----------------------------------------------------------------------------------
    t.heading("Commands (document 24)");
    let mut document = load("fx_msk_001.json");
    let before = document.project().clone();
    let square = |x: f64| {
        Mask::polygon(vec![
            (x, 0.0),
            (x + 2.0, 0.0),
            (x + 2.0, 2.0),
            (x, 2.0),
        ])
    };
    let set = |document: &mut Document, masks: Vec<Mask>| {
        document
            .apply(Command::SetMasks {
                composition: Id::new(COMP),
                layer_id: Id::new("cel"),
                masks,
            })
            .map(|_| ())
            .map_err(|d| (d.id, d.message))
    };
    let mut second = square(3.0);
    second.mode = MaskMode::Subtract;
    let applied = set(&mut document, vec![square(0.0), second]);
    t.row(
        "mask.set with two masks: both land, in order, with their modes",
        &format!(
            "{applied:?}; {:?}",
            masks_of(&document, "cel")
                .iter()
                .map(|m| m.mode.as_str())
                .collect::<Vec<_>>()
        ),
        applied.is_ok()
            && masks_of(&document, "cel").len() == 2
            && masks_of(&document, "cel")[1].mode == MaskMode::Subtract,
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
    let name = document
        .apply(Command::SetMasks {
            composition: Id::new(COMP),
            layer_id: Id::new("cel"),
            masks: vec![square(0.0), square(3.0)],
        })
        .map(|c| c.label.clone())
        .unwrap_or_default();
    // B-24h: the step used to be named "Set 2 masks", after the command rather than the change.
    // It is now read off the list as it was: this one puts a second mask on a layer that had
    // one, and both are named "Mask 1", so there is no name to draw and it says so plainly.
    t.row(
        "and the step is named for what it did",
        &name,
        name == "Add a mask",
    );
    document.undo();
    for (what, bad) in [
        (
            "an opacity of 1.5",
            Mask {
                opacity: 1.5,
                ..square(0.0)
            },
        ),
        (
            "a feather of -1",
            Mask {
                feather_px: -1.0,
                ..square(0.0)
            },
        ),
        (
            "an expansion of 9000",
            Mask {
                expansion_px: 9000.0,
                ..square(0.0)
            },
        ),
    ] {
        let refused = set(&mut document, vec![bad]);
        t.row(
            &format!("mask.set with {what}: COMMAND_INVALID_VALUE, in a sentence, and nothing changes"),
            &format!("{refused:?}"),
            matches!(&refused, Err((DiagnosticId::CommandInvalidValue, _)))
                && document.project() == &before,
        );
    }
    let bowtie = Mask::polygon(vec![(0.0, 0.0), (4.0, 0.0), (0.0, 4.0), (4.0, 4.0)]);
    let refused = set(&mut document, vec![bowtie]);
    t.row(
        "mask.set with a mask that crosses itself: MASK_INVALID_OUTLINE, refused rather than normalized",
        &format!("{refused:?}"),
        matches!(&refused, Err((DiagnosticId::MaskInvalidOutline, _)))
            && document.project() == &before,
    );
    let cleared = set(&mut document, Vec::new());
    t.row(
        "mask.set with an empty list clears them, and the layer is whole again",
        &format!("{cleared:?}; {} masks", masks_of(&document, "cel").len()),
        cleared.is_ok() && masks_of(&document, "cel").is_empty(),
    );

    // -----------------------------------------------------------------------------------
    t.heading("The rule itself (D-77)");
    // A segment with no handles is one straight line, which is the whole of "nothing moved".
    let corners = Mask::polygon(vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]);
    t.row(
        "a mask of corners hands the sampler its four vertices and nothing else",
        &format!("{} points", corners.outline().len()),
        corners.outline() == corners.vertices(),
    );
    // A curve is cut into pieces of about two pixels, held between 16 and 512.
    let mut curved = corners.clone();
    curved.points[0].out_handle = (2.0, 0.0);
    curved.points[1].in_handle = (-2.0, 0.0);
    let pieces = curved.outline().len() - 3;
    t.row(
        "one curved segment four pixels long becomes the floor of 16 pieces, not fewer",
        &format!("{pieces} pieces"),
        pieces == 16,
    );
    let mut long = Mask::polygon(vec![(0.0, 0.0), (400.0, 0.0), (400.0, 400.0), (0.0, 400.0)]);
    long.points[0].out_handle = (200.0, 0.0);
    long.points[1].in_handle = (-200.0, 0.0);
    let pieces = long.outline().len() - 3;
    t.row(
        "a segment of 400 pixels becomes 200 pieces of about two pixels each",
        &format!("{pieces} pieces"),
        pieces == 200,
    );
    // The accumulator's starting value, which is the one rule a reader would not guess.
    let mut only = Mask::polygon(vec![(0.0, 0.0), (3.0, 0.0), (3.0, 2.0), (0.0, 2.0)]);
    only.mode = MaskMode::Subtract;
    let field = anime_compositor::mask::coverage(&[only], 6, 2).expect("one mask takes part");
    t.row(
        "a first mask in Subtract takes its shape away from the whole layer rather than from nothing",
        &format!("left {}, right {}", field[0], field[5]),
        field[0] == 0.0 && field[5] == 1.0,
    );
    let none = Mask {
        mode: MaskMode::None,
        ..Mask::polygon(vec![(0.0, 0.0), (3.0, 0.0), (3.0, 2.0)])
    };
    t.row(
        "a mask in mode none takes no part, so the layer is whole",
        match anime_compositor::mask::coverage(&[none], 6, 2) {
            Some(_) => "masked",
            None => "whole",
        },
        anime_compositor::mask::coverage(
            &[Mask {
                mode: MaskMode::None,
                ..Mask::polygon(vec![(0.0, 0.0), (3.0, 0.0), (3.0, 2.0)])
            }],
            6,
            2,
        )
        .is_none(),
    );
    // The handles are offsets from their point, not positions: D-53's convention, which a build
    // that read them as positions would fail here and nowhere else in this table.
    let mut moved = curved.clone();
    for p in &mut moved.points {
        p.point = (p.point.0 + 10.0, p.point.1);
    }
    let same_shape = moved
        .outline()
        .iter()
        .zip(curved.outline())
        .all(|(a, b)| (a.0 - b.0 - 10.0).abs() < 1e-12 && (a.1 - b.1).abs() < 1e-12);
    t.row(
        "moving every point ten pixels right carries the handles with it, unchanged",
        if same_shape {
            "the same curve, ten pixels right"
        } else {
            "a different curve"
        },
        same_shape,
    );
    t.row(
        "a point with no handles is a corner",
        &format!("{:?}", MaskPoint::corner(1.0, 2.0)),
        MaskPoint::corner(1.0, 2.0).in_handle == (0.0, 0.0),
    );

    // -----------------------------------------------------------------------------------
    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-24b_mask_table.md"), &t.out).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-24b_mask_table.md");
}
