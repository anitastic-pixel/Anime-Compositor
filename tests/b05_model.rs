//! T-03 (model half) / R-03, R-07 / B-05: layers, transforms, keyframes, undo and redo.
//!
//! Writes `verification/B-05_model_table.md` and the three project dumps document 15 asks for:
//! `B-05_project_before.json`, `_after.json` and `_undone.json`. The owner's check is that the
//! undone file is identical to the before file, and that the after file contains exactly the
//! edits that were made and nothing else.
//!
//! The checklist is document 26's own "Required tests" list, run item by item, plus document
//! 20's five keyframe evaluation rules and the layer operations R-03 names.
//!
//! The render half of T-03 (FX-XF-001 to 004: identity, integer translation, half-pixel
//! bilinear weights, rotation about a nonzero anchor) is not here. It needs a transform
//! renderer, which is B-05a. This test covers the model values those renders will read.

use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::command::{Command, Document};
use anime_compositor::inspect::project_json;
use anime_compositor::model::{
    Asset, BlendMode, Composition, Id, Interp, Layer, Project, Prop, Value,
};
use anime_compositor::time::{ExposureSpan, FrameRate};

struct Row {
    check: String,
    expected: String,
    actual: String,
}

impl Row {
    fn pass(&self) -> bool {
        self.expected == self.actual
    }
}

#[derive(Default)]
struct Report {
    rows: Vec<Row>,
}

impl Report {
    fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
        self.rows.push(Row {
            check: check.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
        });
    }
}

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn id(text: &str) -> Id {
    Id::new(text)
}

const COMP: &str = "comp-0000-0000-0000";

/// The reference shot as one asset and one layer per drawing folder, bottom to top.
fn reference_layers() -> Vec<(Asset, Layer)> {
    (1..=4)
        .map(|n| {
            let asset = Asset {
                id: id(&format!("asset-layer{n}")),
                name: format!("layer{n}"),
                pattern: format!("layer{n}_%03d.png"),
            };
            let mut layer = Layer::new(
                id(&format!("layer-{n}")),
                format!("layer{n}"),
                asset.id.clone(),
                0,
                240,
            );
            // Two short exposures per layer: enough to show in the dump without eighty rows
            // of JSON. B-04 already proves the full exposure sheet.
            layer.exposure_spans = vec![
                ExposureSpan {
                    start_frame: 0,
                    end_frame_exclusive: 3,
                    drawing_number: 0,
                },
                ExposureSpan {
                    start_frame: 3,
                    end_frame_exclusive: 6,
                    drawing_number: 1,
                },
            ];
            (asset, layer)
        })
        .collect()
}

/// The opening project, built through commands because the model refuses direct mutation.
fn build_start() -> Project {
    let mut project = Project::new(id("proj-b05-reference"));
    project.compositions.push(Composition::new(
        id(COMP),
        "reference shot",
        1920,
        1080,
        FrameRate::new(24, 1).unwrap(),
        0,
        240,
    ));
    let mut doc = Document::new(project);
    for (index, (asset, layer)) in reference_layers().into_iter().enumerate() {
        doc.apply_all(vec![
            Command::AddAsset { asset },
            Command::AddLayer {
                composition: id(COMP),
                layer: Box::new(layer),
                index,
            },
        ])
        .expect("opening layer");
    }
    doc.project().clone()
}

fn order_of(doc: &Document) -> String {
    doc.project()
        .composition(&id(COMP))
        .unwrap()
        .layers_in_order()
        .map(|l| l.name.clone())
        .collect::<Vec<_>>()
        .join(", ")
}

fn layer_named(doc: &Document, layer: &str) -> anime_compositor::model::Layer {
    doc.project()
        .composition(&id(COMP))
        .unwrap()
        .layer(&id(layer))
        .expect("layer present")
        .clone()
}

#[test]
fn b05_model_and_undo() {
    let mut report = Report::default();
    let mut doc = Document::new(build_start());
    let before_json = project_json(doc.project());

    report.check("opening document: revision", 0, doc.revision());
    report.check("opening document: undo depth", 0, doc.undo_depth());
    report.check("opening document: dirty", false, doc.is_dirty());
    report.check(
        "opening document: layer order is bottom to top",
        "layer1, layer2, layer3, layer4",
        order_of(&doc),
    );

    // -- R-03: rename, hide, lock ---------------------------------------------------------------
    doc.apply(Command::RenameLayer {
        composition: id(COMP),
        layer_id: id("layer-2"),
        name: "sakura".to_string(),
    })
    .expect("rename");
    report.check(
        "rename: the new name is in the model",
        "sakura",
        layer_named(&doc, "layer-2").name,
    );
    report.check(
        "rename: the layer ID did not change, because names are not identity",
        "layer-2",
        layer_named(&doc, "layer-2").id.to_string(),
    );
    report.check("rename: document is now dirty", true, doc.is_dirty());

    doc.apply(Command::SetLayerEnabled {
        composition: id(COMP),
        layer_id: id("layer-3"),
        value: false,
    })
    .expect("hide");
    report.check(
        "hide: layer 3 is disabled",
        false,
        layer_named(&doc, "layer-3").enabled,
    );

    // -- Document 26: a locked layer refuses edits, but can still be unlocked --------------------
    doc.apply(Command::SetLayerLocked {
        composition: id(COMP),
        layer_id: id("layer-1"),
        value: true,
    })
    .expect("lock");
    let revision_before_reject = doc.revision();
    let undo_before_reject = doc.undo_depth();
    let locked_edit = doc.apply(Command::RenameLayer {
        composition: id(COMP),
        layer_id: id("layer-1"),
        name: "background".to_string(),
    });
    report.check(
        "locked layer: the edit is rejected",
        "COMMAND_LAYER_LOCKED",
        match &locked_edit {
            Err(d) => d.id.to_string(),
            Ok(_) => "applied".to_string(),
        },
    );
    report.check(
        "rejected command: revision unchanged",
        revision_before_reject,
        doc.revision(),
    );
    report.check(
        "rejected command: undo stack unchanged",
        undo_before_reject,
        doc.undo_depth(),
    );
    report.check(
        "rejected command: the model is untouched",
        "layer1",
        layer_named(&doc, "layer-1").name,
    );
    report.check(
        "locked layer: unlocking is still allowed",
        "ok",
        match doc.apply(Command::SetLayerLocked {
            composition: id(COMP),
            layer_id: id("layer-1"),
            value: false,
        }) {
            Ok(_) => "ok".to_string(),
            Err(d) => d.id.to_string(),
        },
    );

    // -- Document 26: layer reorder restores exact order and references --------------------------
    let order_before_move = order_of(&doc);
    let layer4_before_move = layer_named(&doc, "layer-4");
    doc.apply(Command::ReorderLayer {
        composition: id(COMP),
        layer_id: id("layer-1"),
        to_index: 3,
    })
    .expect("reorder");
    report.check(
        "reorder: layer 1 moved to the top of the stack",
        "sakura, layer3, layer4, layer1",
        order_of(&doc),
    );
    report.check(
        "reorder: an untouched layer is bit-identical afterwards",
        format!("{layer4_before_move:?}"),
        format!("{:?}", layer_named(&doc, "layer-4")),
    );
    doc.undo();
    report.check(
        "reorder undone: the exact prior order is back",
        order_before_move,
        order_of(&doc),
    );
    doc.redo();
    report.check(
        "reorder redone: the moved order is back",
        "sakura, layer3, layer4, layer1",
        order_of(&doc),
    );

    // -- Document 26: scalar property edit, undo, redo exact value -------------------------------
    doc.apply(Command::SetPropertyBase {
        composition: id(COMP),
        layer_id: id("layer-3"),
        prop: Prop::Rotation,
        value: Value::Scalar(12.5),
    })
    .expect("rotation");
    report.check(
        "scalar edit: rotation is 12.5 degrees",
        "12.5",
        layer_named(&doc, "layer-3").transform.rotation.base(),
    );
    doc.undo();
    report.check(
        "scalar edit undone: the exact prior value returns",
        "0",
        layer_named(&doc, "layer-3").transform.rotation.base(),
    );
    doc.redo();
    report.check(
        "scalar edit redone: the exact value returns",
        "12.5",
        layer_named(&doc, "layer-3").transform.rotation.base(),
    );

    // -- Document 20: opacity is clamped at command validation, scale may be negative ------------
    doc.apply(Command::SetPropertyBase {
        composition: id(COMP),
        layer_id: id("layer-3"),
        prop: Prop::Opacity,
        value: Value::Scalar(1.4),
    })
    .expect("opacity");
    report.check(
        "opacity: a value above 1 is clamped, not rejected",
        "1",
        layer_named(&doc, "layer-3").transform.opacity.base(),
    );
    doc.apply(Command::SetPropertyBase {
        composition: id(COMP),
        layer_id: id("layer-4"),
        prop: Prop::Scale,
        value: Value::Vec2(-1.0, 1.0),
    })
    .expect("mirror");
    report.check(
        "scale: a negative value is accepted, because it mirrors",
        "(-1, 1)",
        layer_named(&doc, "layer-4").transform.scale.base(),
    );
    report.check(
        "property type: a scalar cannot be written to position",
        "COMMAND_INVALID_VALUE",
        match doc.apply(Command::SetPropertyBase {
            composition: id(COMP),
            layer_id: id("layer-4"),
            prop: Prop::Position,
            value: Value::Scalar(3.0),
        }) {
            Err(d) => d.id.to_string(),
            Ok(_) => "applied".to_string(),
        },
    );
    report.check(
        "unknown layer: the command is rejected",
        "COMMAND_TARGET_MISSING",
        match doc.apply(Command::RenameLayer {
            composition: id(COMP),
            layer_id: id("layer-99"),
            name: "ghost".to_string(),
        }) {
            Err(d) => d.id.to_string(),
            Ok(_) => "applied".to_string(),
        },
    );

    // -- Document 20's five keyframe rules -------------------------------------------------------
    // The first keyframe deliberately differs from the base value (0, 0). If it matched, the
    // "before the first keyframe" and "inside a hold segment" rows below would pass just as
    // happily against an implementation that ignored the keyframes and returned the base.
    for (frame, value, interp) in [
        (0, Value::Vec2(20.0, 0.0), Interp::Hold),
        (12, Value::Vec2(100.0, 0.0), Interp::Linear),
        (24, Value::Vec2(100.0, 60.0), Interp::Linear),
    ] {
        doc.apply(Command::SetKeyframe {
            composition: id(COMP),
            layer_id: id("layer-2"),
            prop: Prop::Position,
            frame,
            value,
            interp,
        })
        .expect("keyframe");
    }
    let position = layer_named(&doc, "layer-2").transform.position;
    let sample = |f: i32| position.value_at(f).to_string();
    report.check(
        "keyframes: three were stored",
        3,
        position.keyframes().len(),
    );
    report.check(
        "rule, before the first keyframe (frame -5)",
        "(20, 0)",
        sample(-5),
    );
    report.check("rule, exactly on the first keyframe", "(20, 0)", sample(0));
    report.check(
        "rule, inside a hold segment (frame 6)",
        "(20, 0)",
        sample(6),
    );
    report.check("rule, on the second keyframe", "(100, 0)", sample(12));
    report.check(
        "rule, halfway along a linear segment (frame 18)",
        "(100, 30)",
        sample(18),
    );
    report.check(
        "rule, one third along a linear segment (frame 16)",
        "(100, 20)",
        sample(16),
    );
    report.check("rule, on the last keyframe", "(100, 60)", sample(24));
    report.check(
        "rule, after the last keyframe (frame 300)",
        "(100, 60)",
        sample(300),
    );
    report.check(
        "an unkeyframed property still returns its base value",
        "(0, 0)",
        layer_named(&doc, "layer-2")
            .transform
            .anchor
            .value_at(18)
            .to_string(),
    );
    report.check(
        "setting a keyframe where one exists replaces it rather than duplicating",
        "3",
        {
            doc.apply(Command::SetKeyframe {
                composition: id(COMP),
                layer_id: id("layer-2"),
                prop: Prop::Position,
                frame: 12,
                value: Value::Vec2(100.0, 0.0),
                interp: Interp::Linear,
            })
            .expect("replace");
            layer_named(&doc, "layer-2")
                .transform
                .position
                .keyframes()
                .len()
                .to_string()
        },
    );
    report.check(
        "removing a keyframe that is not there is rejected",
        "COMMAND_TARGET_MISSING",
        match doc.apply(Command::RemoveKeyframe {
            composition: id(COMP),
            layer_id: id("layer-2"),
            prop: Prop::Position,
            frame: 7,
        }) {
            Err(d) => d.id.to_string(),
            Ok(_) => "applied".to_string(),
        },
    );

    // -- Document 26: a drag of 100 intermediate values produces one undo item --------------------
    let undo_before_drag = doc.undo_depth();
    doc.begin_drag().expect("begin");
    for step in 1..=100 {
        doc.update_drag(Command::SetPropertyBase {
            composition: id(COMP),
            layer_id: id("layer-4"),
            prop: Prop::Position,
            value: Value::Vec2(step as f64, 0.0),
        })
        .expect("drag step");
    }
    doc.end_drag();
    report.check(
        "drag: 100 intermediate values produced one history record",
        undo_before_drag + 1,
        doc.undo_depth(),
    );
    report.check(
        "drag: the final value is the one that landed",
        "(100, 0)",
        layer_named(&doc, "layer-4").transform.position.base(),
    );
    doc.undo();
    report.check(
        "drag undone: the value from before the drag returns in one step",
        "(0, 0)",
        layer_named(&doc, "layer-4").transform.position.base(),
    );
    doc.redo();
    report.check(
        "drag redone: the final value returns",
        "(100, 0)",
        layer_named(&doc, "layer-4").transform.position.base(),
    );
    doc.begin_drag().expect("begin");
    let undo_before_cancel = doc.undo_depth();
    doc.update_drag(Command::SetPropertyBase {
        composition: id(COMP),
        layer_id: id("layer-4"),
        prop: Prop::Position,
        value: Value::Vec2(999.0, 0.0),
    })
    .expect("drag step");
    doc.cancel_drag();
    report.check(
        "drag cancelled: no history record and the value is restored",
        format!("{undo_before_cancel}, (100, 0)"),
        format!(
            "{}, {}",
            doc.undo_depth(),
            layer_named(&doc, "layer-4").transform.position.base()
        ),
    );

    // -- Document 26: import media plus create a layer is all-or-nothing --------------------------
    let revision_before_batch = doc.revision();
    let new_asset = Asset {
        id: id("asset-effects"),
        name: "effects".to_string(),
        pattern: "fx_%03d.png".to_string(),
    };
    let bad_batch = doc.apply_all(vec![
        Command::AddAsset {
            asset: new_asset.clone(),
        },
        Command::AddLayer {
            composition: id(COMP),
            layer: Box::new(Layer::new(
                id("layer-fx"),
                "fx",
                id("asset-effects"),
                100,
                50, // out before in: document 19's invariant rejects this
            )),
            index: 4,
        },
    ]);
    report.check(
        "transaction: an invalid second command rejects the whole batch",
        "COMMAND_INVALID_VALUE",
        match &bad_batch {
            Err(d) => d.id.to_string(),
            Ok(_) => "applied".to_string(),
        },
    );
    report.check(
        "transaction rejected: the asset from the first command was not added either",
        4,
        doc.project().assets.len(),
    );
    report.check(
        "transaction rejected: revision unchanged",
        revision_before_batch,
        doc.revision(),
    );
    let undo_before_batch = doc.undo_depth();
    doc.apply_all(vec![
        Command::AddAsset { asset: new_asset },
        Command::AddLayer {
            composition: id(COMP),
            layer: Box::new(Layer::new(
                id("layer-fx"),
                "fx",
                id("asset-effects"),
                0,
                240,
            )),
            index: 4,
        },
    ])
    .expect("valid batch");
    report.check(
        "transaction: import plus create layer is one history record",
        undo_before_batch + 1,
        doc.undo_depth(),
    );
    report.check(
        "transaction: both parts landed",
        "5 assets, 5 layers",
        format!(
            "{} assets, {} layers",
            doc.project().assets.len(),
            doc.project().composition(&id(COMP)).unwrap().len()
        ),
    );
    doc.undo();
    report.check(
        "transaction undone: both parts are gone in one step",
        "4 assets, 4 layers",
        format!(
            "{} assets, {} layers",
            doc.project().assets.len(),
            doc.project().composition(&id(COMP)).unwrap().len()
        ),
    );
    doc.redo();

    // -- Document 26: deleting and recovering a matte preserves dependent records -----------------
    doc.apply(Command::SetMatte {
        composition: id(COMP),
        layer_id: id("layer-fx"),
        matte: Some(id("layer-3")),
    })
    .expect("set matte");
    report.check(
        "matte: layer 3 now has a dependent",
        "layer-fx",
        doc.project()
            .composition(&id(COMP))
            .unwrap()
            .dependents_of(&id("layer-3"))
            .iter()
            .map(Id::to_string)
            .collect::<Vec<_>>()
            .join(", "),
    );
    report.check(
        "matte: a cycle is rejected",
        "MATTE_CYCLE",
        match doc.apply(Command::SetMatte {
            composition: id(COMP),
            layer_id: id("layer-3"),
            matte: Some(id("layer-fx")),
        }) {
            Err(d) => d.id.to_string(),
            Ok(_) => "applied".to_string(),
        },
    );
    report.check(
        "matte: a reference to a layer that is not there is rejected",
        "MATTE_REFERENCE_MISSING",
        match doc.apply(Command::SetMatte {
            composition: id(COMP),
            layer_id: id("layer-fx"),
            matte: Some(id("layer-99")),
        }) {
            Err(d) => d.id.to_string(),
            Ok(_) => "applied".to_string(),
        },
    );
    let fx_before_delete = layer_named(&doc, "layer-fx");
    doc.apply(Command::RemoveLayer {
        composition: id(COMP),
        layer_id: id("layer-3"),
    })
    .expect("delete the matte source");
    report.check(
        "matte: the dependent layer still records the reference after its target is deleted",
        "layer-3",
        layer_named(&doc, "layer-fx")
            .matte
            .map(|m| m.layer_id.to_string())
            .unwrap_or_else(|| "dropped".to_string()),
    );
    doc.undo();
    report.check(
        "matte: undoing the delete restores the target and the dependent exactly",
        format!("{fx_before_delete:?}"),
        format!("{:?}", layer_named(&doc, "layer-fx")),
    );
    report.check(
        "matte: the restored target is back in its original position",
        "sakura, layer3, layer4, layer1, fx",
        order_of(&doc),
    );

    // -- Document 26: save, edit, undo back to the saved state clears dirty -----------------------
    doc.mark_saved();
    report.check("after save: dirty", false, doc.is_dirty());
    doc.apply(Command::RenameLayer {
        composition: id(COMP),
        layer_id: id("layer-4"),
        name: "temporary".to_string(),
    })
    .expect("edit after save");
    report.check("after an edit following save: dirty", true, doc.is_dirty());
    doc.undo();
    report.check(
        "after undoing back to the saved state: dirty is false again",
        false,
        doc.is_dirty(),
    );
    report.check(
        "and redo is still available even though the document is clean",
        1,
        doc.redo_depth(),
    );

    // -- The three JSON dumps --------------------------------------------------------------------
    let after_json = project_json(doc.project());
    let edit_count = doc.undo_depth();
    // One record is already redoable: the rename that the dirty-state check undid.
    let redo_already = doc.redo_depth();
    while doc.undo().is_some() {}
    let undone_json = project_json(doc.project());

    report.check(
        "undoing every command returns the project to its opening state, byte for byte",
        "identical",
        if undone_json == before_json {
            "identical".to_string()
        } else {
            first_difference(&before_json, &undone_json)
        },
    );
    report.check("after undoing everything: undo depth", 0, doc.undo_depth());
    report.check(
        "after undoing everything: every undone command is redoable",
        edit_count + redo_already,
        doc.redo_depth(),
    );
    report.check("a new command clears the redo stack", 0, {
        doc.apply(Command::SetLayerEnabled {
            composition: id(COMP),
            layer_id: id("layer-1"),
            value: false,
        })
        .expect("new edit");
        doc.redo_depth()
    });

    fs::write(repo("verification/B-05_project_before.json"), &before_json).expect("write");
    fs::write(repo("verification/B-05_project_after.json"), &after_json).expect("write");
    fs::write(repo("verification/B-05_project_undone.json"), &undone_json).expect("write");
    write_artifact(&report, &before_json, &after_json);

    let failed: Vec<&Row> = report.rows.iter().filter(|r| !r.pass()).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed, first: {} expected {:?} got {:?}",
        failed.len(),
        report.rows.len(),
        failed[0].check,
        failed[0].expected,
        failed[0].actual
    );
}

fn first_difference(a: &str, b: &str) -> String {
    for (n, (x, y)) in a.lines().zip(b.lines()).enumerate() {
        if x != y {
            return format!("line {}: {x:?} vs {y:?}", n + 1);
        }
    }
    format!("line counts {} vs {}", a.lines().count(), b.lines().count())
}

fn write_artifact(report: &Report, before: &str, after: &str) {
    let passed = report.rows.iter().filter(|r| r.pass()).count();
    let mut out = String::new();
    out.push_str(&format!(
        "# B-05 model, commands and undo\n\n\
         Test T-03 (model half), requirements R-03 and R-07. Produced by `tests/b05_model.rs`. \
         **{passed} of {} checks pass.**\n\n\
         ## What to check by eye\n\n\
         Three project dumps sit beside this file:\n\n\
         - `B-05_project_before.json` — the reference shot as four layers, before any edit.\n\
         - `B-05_project_after.json` — after every edit listed in the table below.\n\
         - `B-05_project_undone.json` — after pressing undo until nothing is left to undo.\n\n\
         The before and undone files should be identical. Any difference at all means undo did \
         not restore the model exactly, which is the whole of R-07. Diffing them is the check; \
         the test also makes it and reports it as a row below, but the files are there so the \
         claim can be verified without trusting the test.\n\n\
         The after file should differ from the before file only in the edits that were made. \
         An edit that appears there and is not in the table is a bug even if every check passes.\n\n\
         These dumps are an inspection view, not the save format. Persistence, schema \
         versioning and migration are B-09. The shape follows \
         `Schemas/project-v0.schema.json` so the two can be compared when B-09 arrives, but \
         nothing reads these files back.\n\n\
         The before file is {} lines and the after file is {} lines.\n\n",
        report.rows.len(),
        before.lines().count(),
        after.lines().count(),
    ));

    out.push_str("## Checks\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for r in &report.rows {
        out.push_str(&format!(
            "| {} | `{}` | `{}` | {} |\n",
            r.check,
            r.expected,
            r.actual,
            if r.pass() { "PASS" } else { "**FAIL**" }
        ));
    }

    out.push_str(
        "\n## Document 26's required tests, item by item\n\n\
         | Document 26 asks for | Covered by |\n|---|---|\n\
         | scalar property edit, undo, redo exact value | the three `scalar edit` rows |\n\
         | layer reorder restores exact order and references | the four `reorder` rows, \
           including one asserting an untouched layer is bit-identical after the move |\n\
         | deleting/recovering a matte preserves dependent records | the five `matte` rows |\n\
         | import/create-layer transaction is all-or-nothing | the six `transaction` rows |\n\
         | drag of 100 intermediate values produces one undo item | the five `drag` rows |\n\
         | rejected command produces no revision/history change | the three `rejected command` \
           rows, plus every row naming a diagnostic ID |\n\
         | save -> edit -> undo back to saved state clears dirty | the four `after save` rows |\n\
         | undo/redo after project reopen is empty | not run: reopening is B-09 |\n\n",
    );

    out.push_str(
        "## Not run by this test\n\n\
         - The render half of T-03: FX-XF-001 identity, FX-XF-002 integer translation, \
           FX-XF-003 half-pixel bilinear weights and FX-XF-004 rotation about a nonzero anchor. \
           Those need a transform renderer, which is B-05a. This test covers the model values \
           that renderer will read, and the keyframe rows above are the animation those \
           fixtures will sample.\n\
         - Save and reopen, and therefore document 26's \"undo/redo after project reopen is \
           empty\". That is B-09 and T-07.\n\
         - Cache invalidation domains, which document 26 requires every committed command to \
           report to document 27. There is no cache yet; it is B-07.\n\
         - Effects and masks. Effects are B-06; masks are parked to G1-rest with R-04 under \
           D-12. A layer therefore serialises `\"effects\": []`, which is accurate rather than \
           a placeholder, and carries no mask field at all.\n\
         - Colour4 and boolean properties, which document 19 lists. G1 needs them once effects \
           have colour parameters, which is B-06.\n",
    );

    fs::write(repo("verification/B-05_model_table.md"), out).expect("write artifact");
}

/// Document 20 says the interpolation mode belongs to the segment starting at a keyframe. A
/// hold followed by a linear must therefore step, then ramp — not ramp, then step.
#[test]
fn interpolation_mode_belongs_to_the_segment_that_starts_at_it() {
    let mut project = Project::new(id("p"));
    let comp = Composition::new(id(COMP), "c", 4, 4, FrameRate::new(24, 1).unwrap(), 0, 10);
    project.assets.push(Asset {
        id: id("a"),
        name: "a".to_string(),
        pattern: "a_%03d.png".to_string(),
    });
    project.compositions.push(comp);
    let mut doc = Document::new(project);
    doc.apply(Command::AddLayer {
        composition: id(COMP),
        layer: Box::new(Layer::new(id("l"), "l", id("a"), 0, 10)),
        index: 0,
    })
    .expect("layer");
    for (frame, v, interp) in [
        (0, 0.0, Interp::Hold),
        (4, 10.0, Interp::Linear),
        (8, 20.0, Interp::Linear),
    ] {
        doc.apply(Command::SetKeyframe {
            composition: id(COMP),
            layer_id: id("l"),
            prop: Prop::Rotation,
            frame,
            value: Value::Scalar(v),
            interp,
        })
        .expect("key");
    }
    let rot = doc
        .project()
        .composition(&id(COMP))
        .unwrap()
        .layer(&id("l"))
        .unwrap()
        .transform
        .rotation
        .clone();
    let got: Vec<String> = (0..=8).map(|f| rot.value_at(f).to_string()).collect();
    assert_eq!(
        got,
        ["0", "0", "0", "0", "10", "12.5", "15", "17.5", "20"],
        "hold segment must step and linear segment must ramp"
    );
}

/// Blend mode round-trips through the inspection dump's spelling.
#[test]
fn blend_modes_serialise_as_the_schema_spells_them() {
    for (mode, text) in [
        (BlendMode::Normal, "normal"),
        (BlendMode::Multiply, "multiply"),
        (BlendMode::Screen, "screen"),
        (BlendMode::Add, "add"),
    ] {
        assert_eq!(mode.as_str(), text);
    }
}
