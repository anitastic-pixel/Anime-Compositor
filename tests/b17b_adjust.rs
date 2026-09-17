//! B-17b: adjustment layers in the core, against D-66.
//!
//! Writes `verification/B-17b_adjust_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/adjust/expected_adjust.json`, written by `tools/adjust_reference.py`, which
//! renders each six-by-two frame pixel by pixel from document 21 and blurs with the
//! two-dimensional kernel summed directly, where this build runs two one-dimensional passes.
//! Document 25 prints the same numbers as FX-ADJ-001 to 013. The tolerance is the catalogue's,
//! 1e-6, because the build works in 32-bit floats. Nothing here is a snapshot of a run.
//!
//! # What this proves
//!
//! That `out = B + c*(E(B) - B)` is what an adjustment layer does to the frame beneath it, with
//! `c` its coverage through mask, transform and matte times its opacity; that layers above it
//! are not adjusted; that two adjustment layers stack in order; that a switched-off effect, a
//! layer behind everything in depth and a layer outside its in and out frames change nothing;
//! that the frame is the same frame whatever tile size it is cut into; that the kind survives a
//! file without an asset, and a file that gives it one, or another blend mode, is refused; that
//! the blend list refuses one; that the draft preview scales its blur; and that the trace names
//! the adjusted frame.
//!
//! # What is deliberately not here
//!
//! The window: the button, Ctrl+Alt+Y, the row on the timeline and the inspector are B-17c.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{plan_frame, render_frame};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::model::{BlendMode, Id, Layer, LayerKind, Value};
use anime_compositor::persist;
use anime_compositor::preview::{scale_plan, PreviewQuality};
use anime_compositor::render::render_without_culling;
use anime_compositor::trace::{render_traced, TraceRequest};
use anime_compositor::WorkingBuffer;

const COMP: &str = "comp-main";

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/adjust")
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

/// The largest difference from the expected pixels, over every sample.
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

fn diagnostics_at(document: &Document, frame: i32) -> Vec<DiagnosticId> {
    let mut log = FrameLog::new(8);
    let _ = plan_frame(document.project(), &Id::new(COMP), frame, &root(), &mut log);
    let mut ids: Vec<DiagnosticId> = log.finish().into_iter().map(|d| d.id).collect();
    ids.dedup();
    ids
}

fn layer_json(text: &str, id: &str, edit: impl FnOnce(&mut J)) -> String {
    let mut root: J = serde_json::from_str(text).unwrap();
    let layer = root["compositions"][0]["layers"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|l| l["id"] == id)
        .unwrap();
    edit(layer);
    serde_json::to_string_pretty(&root).unwrap()
}

#[test]
fn b17b_adjust() {
    let expected: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/adjust/expected_adjust.json")).unwrap(),
    )
    .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let cases = expected["cases"].as_object().unwrap();

    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-17b: adjustment layers\n\nD-66, accepted by the owner on 2026-09-17. Every \
         expected pixel is `Fixtures/adjust/expected_adjust.json`, written by \
         `tools/adjust_reference.py` before this code existed and printed in document 25 as \
         FX-ADJ-001 to 013. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 1e-6.\n",
    );

    // -----------------------------------------------------------------------------------
    t.heading("FX-ADJ-001 to 013, rendered whole (document 25)");
    for (name, case) in cases {
        let document = load(case["project"].as_str().unwrap());
        for (frame, pixels) in case["frames"].as_object().unwrap() {
            let frame: i32 = frame.parse().unwrap();
            let d = largest_difference(&render(&document, frame, 64), pixels);
            t.row(
                &format!("{name} frame {frame}: {}", case["says"].as_str().unwrap()),
                &format!("largest difference {d:.1e}"),
                d <= tolerance,
            );
        }
    }

    // -----------------------------------------------------------------------------------
    t.heading("Tiled against untiled: the same frame whatever it is cut into");
    for (name, case) in cases {
        let document = load(case["project"].as_str().unwrap());
        for frame in case["frames"].as_object().unwrap().keys() {
            let frame: i32 = frame.parse().unwrap();
            let whole = render(&document, frame, 64);
            let by_ones = render(&document, frame, 1);
            let by_fours = render(&document, frame, 4);
            let mut log = FrameLog::new(8);
            let plan =
                plan_frame(document.project(), &Id::new(COMP), frame, &root(), &mut log).unwrap();
            let unculled = render_without_culling(&plan, 1);
            let same = whole.data() == by_ones.data()
                && whole.data() == by_fours.data()
                && whole.data() == unculled.data();
            t.row(
                &format!("{name} frame {frame}: one tile, 12 tiles of 1, tiles of 4 and unculled"),
                if same { "byte-identical" } else { "differ" },
                same,
            );
        }
    }

    // -----------------------------------------------------------------------------------
    t.heading("Nothing to say and something to say");
    let quiet = load("fx_adj_011.json");
    let ids = diagnostics_at(&quiet, 0);
    t.row(
        "FX-ADJ-011, an effect switched off: no diagnostic (a bypass a person chose is not a fault)",
        &format!("{ids:?}"),
        ids.is_empty(),
    );
    let text = fs::read_to_string(root().join("fx_adj_001.json")).unwrap();
    let unsupported = layer_json(&text, "adj", |l| {
        l["effects"][0]["type_id"] = J::from("core.not_in_this_build");
    });
    let loaded = persist::load_str(&unsupported).expect("an unknown effect is kept, not refused");
    let ids = diagnostics_at(&loaded.document, 0);
    t.row(
        "an adjustment layer with an effect this build does not have: EFFECT_UNSUPPORTED at the frame",
        &format!("{ids:?}"),
        ids == [DiagnosticId::EffectUnsupported],
    );
    let frame_without = {
        let mut without = load("fx_adj_001.json");
        without
            .apply(Command::RemoveLayer {
                composition: Id::new(COMP),
                layer_id: Id::new("adj"),
            })
            .unwrap();
        render(&without, 0, 64)
    };
    let bypassed = render(&loaded.document, 0, 64);
    t.row(
        "and that frame is the frame beneath it, untouched",
        if bypassed.data() == frame_without.data() {
            "byte-identical to the frame with the layer removed"
        } else {
            "differs"
        },
        bypassed.data() == frame_without.data(),
    );

    // -----------------------------------------------------------------------------------
    t.heading("The file (document 19)");
    let document = load("fx_adj_006.json");
    let adj = document
        .project()
        .composition(&Id::new(COMP))
        .unwrap()
        .layer(&Id::new("adj"))
        .unwrap();
    t.row(
        "fx_adj_006.json's `adj` reads as kind adjustment, with its matte and effect",
        &format!(
            "{}, matte {:?}, {} effect(s)",
            adj.kind.as_str(),
            adj.matte.as_ref().map(|m| m.layer_id.as_str()),
            adj.effects.len()
        ),
        adj.kind == LayerKind::Adjustment && adj.matte.is_some() && adj.effects.len() == 1,
    );
    let written = persist::to_json(document.project(), &persist::Preserved::default());
    let written_json: J = serde_json::from_str(&written).unwrap();
    let written_adj = written_json["compositions"][0]["layers"]
        .as_array()
        .unwrap()
        .iter()
        .find(|l| l["id"] == "adj")
        .unwrap();
    let keys: Vec<&str> = ["kind", "asset_id", "source_offset_frames", "exposure_spans"]
        .into_iter()
        .filter(|k| written_adj.get(k).is_some())
        .collect();
    t.row(
        "written back: `kind` is adjustment and there is no asset_id, source_offset_frames or exposure_spans",
        &format!("kind {}, keys present: {keys:?}", written_adj["kind"]),
        written_adj["kind"] == "adjustment" && keys == ["kind"],
    );
    let reopened = persist::load_str(&written).expect("what this build wrote, it opens");
    t.row(
        "and it opens again as the same layer",
        if reopened.document.project() == document.project() {
            "equal"
        } else {
            "different"
        },
        reopened.document.project() == document.project(),
    );
    let with_asset = layer_json(&text, "adj", |l| {
        l["asset_id"] = J::from("asset-bg");
    });
    let refused = persist::load_str(&with_asset).err().map(|d| d.id);
    t.row(
        "an adjustment layer given an asset_id: PROJECT_SCHEMA_INVALID",
        &format!("{refused:?}"),
        refused == Some(DiagnosticId::ProjectSchemaInvalid),
    );
    let with_blend = layer_json(&text, "adj", |l| {
        l["blend_mode"] = J::from("multiply");
    });
    let refused = persist::load_str(&with_blend).err().map(|d| d.id);
    t.row(
        "an adjustment layer with blend_mode multiply: PROJECT_SCHEMA_INVALID",
        &format!("{refused:?}"),
        refused == Some(DiagnosticId::ProjectSchemaInvalid),
    );

    // -----------------------------------------------------------------------------------
    t.heading("Commands (document 24)");
    let mut document = load("fx_adj_001.json");
    let refused = document
        .apply(Command::SetBlendMode {
            composition: Id::new(COMP),
            layer_id: Id::new("adj"),
            mode: BlendMode::Multiply,
        })
        .err()
        .map(|d| d.id);
    t.row(
        "layer.set_blend_mode multiply on an adjustment layer: COMMAND_INVALID_VALUE",
        &format!("{refused:?}"),
        refused == Some(DiagnosticId::CommandInvalidValue),
    );
    let accepted = document
        .apply(Command::SetBlendMode {
            composition: Id::new(COMP),
            layer_id: Id::new("adj"),
            mode: BlendMode::Normal,
        })
        .is_ok();
    t.row(
        "layer.set_blend_mode normal on one: accepted",
        if accepted { "accepted" } else { "refused" },
        accepted,
    );
    let new_layer = Layer::adjustment(Id::new("adj-2"), "Adjustment 2", 6, 2, 0, 3);
    let added = document
        .apply(Command::AddLayer {
            composition: Id::new(COMP),
            layer: Box::new(new_layer.clone()),
            index: 2,
        })
        .is_ok();
    let centre = (
        new_layer.transform.anchor.value_at(0),
        new_layer.transform.position.value_at(0),
    );
    t.row(
        "a new adjustment layer needs no asset, and its anchor and position are the centre of the composition",
        &format!("added {added}, anchor {:?}, position {:?}", centre.0, centre.1),
        added && centre.0 == Value::Vec2(3.0, 1.0) && centre.1 == Value::Vec2(3.0, 1.0),
    );

    // -----------------------------------------------------------------------------------
    t.heading("The draft preview (D-33, D-66)");
    let document = load("fx_adj_007.json");
    let mut log = FrameLog::new(8);
    let plan = plan_frame(document.project(), &Id::new(COMP), 0, &root(), &mut log).unwrap();
    let draft = scale_plan(plan, PreviewQuality::Draft);
    let sigma = draft
        .layers
        .iter()
        .find(|l| l.id.as_str() == "adj")
        .and_then(|l| l.adjust.as_ref())
        .and_then(|stack| match stack[0].effect {
            Effect::GaussianBlur { sigma_px } => Some(sigma_px),
            _ => None,
        });
    t.row(
        "FX-ADJ-007's blur of sigma 1 is sigma 0.25 on the quarter-size frame",
        &format!("{sigma:?}"),
        sigma == Some(0.25),
    );

    // -----------------------------------------------------------------------------------
    t.heading("The trace (ADR-012)");
    let document = load("fx_adj_003.json");
    let mut log = FrameLog::new(8);
    let plan = plan_frame(document.project(), &Id::new(COMP), 0, &root(), &mut log).unwrap();
    let dir = repo("target/b17b_trace");
    if dir.exists() {
        fs::remove_dir_all(&dir).unwrap();
    }
    let (traced, written) = render_traced(
        &plan,
        64,
        &TraceRequest {
            dir: dir.clone(),
            frame: 0,
        },
    )
    .expect("trace render");
    let plain = render(&document, 0, 64);
    t.row(
        "the frame render_traced returns is the frame render returns",
        if traced.data() == plain.data() {
            "byte-identical"
        } else {
            "differs"
        },
        traced.data() == plain.data(),
    );
    let manifest = fs::read_to_string(dir.join("frame_00000/manifest.md")).unwrap();
    let named = manifest
        .lines()
        .filter(|l| l.contains("the adjusted frame"))
        .count();
    t.row(
        "the manifest names the adjustment layer's composite image as the adjusted frame, once",
        &format!("{named} line(s); {} images written", written.len()),
        named == 1,
    );

    // -----------------------------------------------------------------------------------
    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-17b_adjust_table.md"), &t.out).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-17b_adjust_table.md");
}
