//! B-13b: parenting, against D-57.
//!
//! Writes `verification/B-13b_parenting_table.md`.
//!
//! # Where the expected values come from
//!
//! `Markdown/25_Test_Fixture_Catalog.md`, FX-PARENT-001 to 008, every number of which is
//! printed by `tools/parent_reference.py`. That script carries a point through document 21's
//! four steps one at a time -- subtract the anchor, scale, rotate clockwise, add the position --
//! and then hands the result to the parent. It never builds a matrix; this build multiplies
//! them. An agreement between the two is two answers and not one answer twice (ADR-009).
//!
//! Nothing here is a snapshot of a run. Every expected number below was written into document 25
//! and accepted by the owner before any of the code under test existed.
//!
//! # What this proves
//!
//! That `M_world(L) = M_world(parent(L)) * M(L)` is what draws: the chain is read through
//! `compose::world_transform`, which is the map the renderer builds at step 4. That only the
//! transform is inherited, not opacity or whether the parent is switched on or in range. That
//! keeping place does what document 21 says in the case where it can be exact, and does the
//! same wrong thing every time in the case where it cannot. That a loop is refused without
//! changing the project. And that a parent survives a file, including one that points at a
//! layer that is not there.
//!
//! # What is deliberately not here
//!
//! The panel. Choosing a parent from the layer's inspector, and deleting a parent from the
//! layer list, are the window's half of B-13b and are checked in `app/src/main.rs`'s own tests.

use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::command::{parent_keep_place_is_exact, Command, Document};
use anime_compositor::compose::world_transform;
use anime_compositor::diagnostics::DiagnosticId;
use anime_compositor::model::{Asset, Composition, Id, Interp, Layer, Project, Prop, Value};
use anime_compositor::persist;
use anime_compositor::time::FrameRate;

const COMP: &str = "comp-parenting";
const ASSET: &str = "asset-unused";

// ---------------------------------------------------------------------------------------
// Reporting
// ---------------------------------------------------------------------------------------

#[derive(Default)]
struct Report {
    rows: Vec<(String, String, String)>,
}

impl Report {
    fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
        self.rows
            .push((check.to_string(), expected.to_string(), actual.to_string()));
    }

    /// A pair of composition pixels, to nine places. Document 25 sets the tolerance at 1e-9,
    /// "because a rotation of 90 degrees is not exactly a quarter turn in floating point".
    fn point(&mut self, check: &str, expected: (f64, f64), actual: (f64, f64)) {
        self.check(check, place(expected), place(actual));
    }
}

fn place(p: (f64, f64)) -> String {
    format!("{:.9}, {:.9}", p.0, p.1)
}

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

// ---------------------------------------------------------------------------------------
// Building a composition of bare layers
// ---------------------------------------------------------------------------------------

/// A project with one empty composition, 1920x1080 at 24, forty frames long.
fn project() -> Document {
    let mut document = Document::new(Project::new(Id::new("proj-parenting-test")));
    // One asset, shared by every layer below. No case here asks what a layer draws, only where
    // its pixels would land, but a layer must name media the project holds.
    document
        .apply(Command::AddAsset {
            asset: Asset::sequence(Id::new(ASSET), "unused", "unused_%03d.png"),
        })
        .expect("the asset is added");
    document
        .apply(Command::AddComposition {
            composition: Box::new(Composition::new(
                Id::new(COMP),
                "Parenting",
                1920,
                1080,
                FrameRate::new(24, 1).expect("24 fps"),
                0,
                40,
            )),
        })
        .expect("the composition is added");
    document
}

/// A layer with no drawing behind it. Every case here asks where a layer's pixels would land,
/// which is a question about its transform and does not need the pixels.
fn layer(document: &mut Document, id: &str, index: usize) -> Id {
    let layer_id = Id::new(id);
    document
        .apply(Command::AddLayer {
            composition: Id::new(COMP),
            layer: Box::new(Layer::new(
                layer_id.clone(),
                id,
                Id::new(ASSET),
                0,
                40,
            )),
            index,
        })
        .unwrap_or_else(|d| panic!("adding {id}: {}", d.message));
    layer_id
}

fn set(document: &mut Document, layer_id: &Id, prop: Prop, value: Value) {
    document
        .apply(Command::SetPropertyBase {
            composition: Id::new(COMP),
            layer_id: layer_id.clone(),
            prop,
            value,
        })
        .unwrap_or_else(|d| panic!("setting {prop:?}: {}", d.message));
}

fn key(document: &mut Document, layer_id: &Id, prop: Prop, frame: i32, value: Value) {
    document
        .apply(Command::SetKeyframe {
            composition: Id::new(COMP),
            layer_id: layer_id.clone(),
            prop,
            frame,
            value,
            interp: Interp::Linear,
            spatial: None,
        })
        .unwrap_or_else(|d| panic!("keying {prop:?} at {frame}: {}", d.message));
}

fn set_parent(document: &mut Document, layer_id: &Id, parent: Option<&Id>, frame: i32) -> String {
    let command = Command::SetParent {
        composition: Id::new(COMP),
        layer_id: layer_id.clone(),
        parent: parent.cloned(),
        frame,
        keep_place: true,
    };
    match document.apply(command) {
        Ok(record) => record.label.clone(),
        Err(d) => format!("{}: {}", d.id, d.message),
    }
}

/// Where a point of the layer's own space lands in the composition, through the chain.
fn at(document: &Document, layer_id: &Id, frame: i32, point: (f64, f64)) -> (f64, f64) {
    let comp = document
        .project()
        .composition(&Id::new(COMP))
        .expect("the composition is there");
    world_transform(comp, layer_id, frame).apply(point.0, point.1)
}

/// The layer's own held values, in the units document 25 writes them in: scale as a percentage,
/// which is the file and UI boundary D-22 puts the divide by 100 at.
fn held(document: &Document, layer_id: &Id) -> (String, String, String, String) {
    let layer = document
        .project()
        .composition(&Id::new(COMP))
        .and_then(|c| c.layer(layer_id))
        .expect("the layer is there");
    let pair = |v: Value, by: f64| {
        let (x, y) = v.as_vec2().expect("a vec2 property");
        place((x * by, y * by))
    };
    (
        pair(layer.transform.anchor.base(), 1.0),
        pair(layer.transform.position.base(), 1.0),
        pair(layer.transform.scale.base(), 100.0),
        format!(
            "{:.9}",
            layer
                .transform
                .rotation
                .base()
                .as_scalar()
                .expect("a scalar property")
        ),
    )
}

const CORNERS: [(f64, f64); 4] = [(0.0, 0.0), (100.0, 0.0), (0.0, 100.0), (100.0, 100.0)];

// ---------------------------------------------------------------------------------------
// The test
// ---------------------------------------------------------------------------------------

#[test]
fn b13b_parenting() {
    let mut report = Report::default();

    fx_parent_001(&mut report);
    fx_parent_002(&mut report);
    fx_parent_003(&mut report);
    fx_parent_004(&mut report);
    fx_parent_005(&mut report);
    fx_parent_006(&mut report);
    fx_parent_007(&mut report);
    fx_parent_008(&mut report);

    write_artifact(&report);
    let failed: Vec<&(String, String, String)> =
        report.rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// FX-PARENT-001: one parent, nothing animated.
fn fx_parent_001(report: &mut Report) {
    let mut document = project();
    let parent = layer(&mut document, "layer-p", 0);
    let child = layer(&mut document, "layer-c", 1);
    set(&mut document, &parent, Prop::Anchor, Value::Vec2(50.0, 50.0));
    set(
        &mut document,
        &parent,
        Prop::Position,
        Value::Vec2(400.0, 300.0),
    );
    set(&mut document, &parent, Prop::Rotation, Value::Scalar(90.0));
    set(
        &mut document,
        &child,
        Prop::Position,
        Value::Vec2(100.0, 0.0),
    );
    // The child is already where it is meant to be in the parent's space, so this is the one
    // case that asks for the parent without keeping place.
    document
        .apply(Command::SetParent {
            composition: Id::new(COMP),
            layer_id: child.clone(),
            parent: Some(parent.clone()),
            frame: 0,
            keep_place: false,
        })
        .expect("the parent is set");

    for (point, expected) in [
        ((0.0, 0.0), (450.0, 350.0)),
        ((10.0, 0.0), (450.0, 360.0)),
        ((0.0, 10.0), (440.0, 350.0)),
    ] {
        report.point(
            &format!("FX-PARENT-001: the child's ({}, {})", point.0, point.1),
            expected,
            at(&document, &child, 0, point),
        );
    }
}

/// FX-PARENT-002: a chain of three, with the middle one animated.
fn fx_parent_002(report: &mut Report) {
    let document = chain_of_three();
    let child = Id::new("layer-c");
    for (frame, origin, step) in [
        (0, (1110.0, 540.0), (1120.0, 540.0)),
        (
            6,
            (1106.1939766255643, 559.1341716182545),
            (1115.4327719506773, 562.9610059419053),
        ),
        (
            12,
            (1095.3553390593274, 575.3553390593274),
            (1102.4264068711927, 582.4264068711929),
        ),
        (24, (1060.0, 590.0), (1060.0, 600.0)),
    ] {
        report.point(
            &format!("FX-PARENT-002: the child's origin at frame {frame}"),
            origin,
            at(&document, &child, frame, (0.0, 0.0)),
        );
        report.point(
            &format!("FX-PARENT-002: the child's (20, 0) at frame {frame}"),
            step,
            at(&document, &child, frame, (20.0, 0.0)),
        );
    }
    // The child has no keys of its own and still moves, which is the point of the case.
    report.check(
        "FX-PARENT-002: the child holds no keyframes of its own",
        false,
        document
            .project()
            .composition(&Id::new(COMP))
            .and_then(|c| c.layer(&child))
            .expect("the child is there")
            .transform
            .position
            .is_animated(),
    );
}

fn chain_of_three() -> Document {
    let mut document = project();
    let grand = layer(&mut document, "layer-g", 0);
    let parent = layer(&mut document, "layer-p", 1);
    let child = layer(&mut document, "layer-c", 2);
    set(
        &mut document,
        &grand,
        Prop::Position,
        Value::Vec2(960.0, 540.0),
    );
    set(&mut document, &grand, Prop::Scale, Value::Vec2(0.5, 0.5));
    set(
        &mut document,
        &parent,
        Prop::Position,
        Value::Vec2(200.0, 0.0),
    );
    key(&mut document, &parent, Prop::Rotation, 0, Value::Scalar(0.0));
    key(
        &mut document,
        &parent,
        Prop::Rotation,
        24,
        Value::Scalar(90.0),
    );
    set(
        &mut document,
        &child,
        Prop::Position,
        Value::Vec2(100.0, 0.0),
    );
    for (layer_id, parent_id) in [(&parent, &grand), (&child, &parent)] {
        document
            .apply(Command::SetParent {
                composition: Id::new(COMP),
                layer_id: layer_id.clone(),
                parent: Some(parent_id.clone()),
                frame: 0,
                keep_place: false,
            })
            .expect("the parent is set");
    }
    document
}

/// FX-PARENT-003: what is not inherited.
///
/// The parent is switched off, half transparent, and out of range at the frame read. Its
/// transform still reaches the child; nothing else does.
fn fx_parent_003(report: &mut Report) {
    let mut document = project();
    let parent = layer(&mut document, "layer-p", 0);
    let child = layer(&mut document, "layer-c", 1);
    set(&mut document, &parent, Prop::Anchor, Value::Vec2(50.0, 50.0));
    set(
        &mut document,
        &parent,
        Prop::Position,
        Value::Vec2(400.0, 300.0),
    );
    set(&mut document, &parent, Prop::Rotation, Value::Scalar(90.0));
    set(&mut document, &parent, Prop::Opacity, Value::Scalar(0.5));
    set(
        &mut document,
        &child,
        Prop::Position,
        Value::Vec2(100.0, 0.0),
    );
    document
        .apply(Command::SetParent {
            composition: Id::new(COMP),
            layer_id: child.clone(),
            parent: Some(parent.clone()),
            frame: 0,
            keep_place: false,
        })
        .expect("the parent is set");
    document
        .apply(Command::SetLayerEnabled {
            composition: Id::new(COMP),
            layer_id: parent.clone(),
            value: false,
        })
        .expect("the parent is switched off");
    document
        .apply(Command::TrimLayer {
            composition: Id::new(COMP),
            layer_id: parent.clone(),
            in_frame: 0,
            out_frame: 24,
        })
        .expect("the parent is trimmed");

    // Frame 30: past the parent's out point, with the parent switched off.
    for (point, expected) in [
        ((0.0, 0.0), (450.0, 350.0)),
        ((10.0, 0.0), (450.0, 360.0)),
        ((0.0, 10.0), (440.0, 350.0)),
    ] {
        report.point(
            &format!(
                "FX-PARENT-003: at frame 30, off and out of range, the child's ({}, {})",
                point.0, point.1
            ),
            expected,
            at(&document, &child, 30, point),
        );
    }
    report.check(
        "FX-PARENT-003: the child's own opacity is untouched by the parent's",
        "1.000000000",
        format!(
            "{:.9}",
            document
                .project()
                .composition(&Id::new(COMP))
                .and_then(|c| c.layer(&child))
                .expect("the child is there")
                .transform
                .opacity
                .value_at(30)
                .as_scalar()
                .expect("a scalar property")
        ),
    );
}

/// FX-PARENT-004: loops.
fn fx_parent_004(report: &mut Report) {
    let mut document = project();
    let a = layer(&mut document, "layer-a", 0);
    let b = layer(&mut document, "layer-b", 1);

    report.check(
        "FX-PARENT-004: parenting A to B is allowed",
        "Set parent to layer-b",
        set_parent(&mut document, &a, Some(&b), 0),
    );

    let before = document.project().clone();
    report.check(
        "FX-PARENT-004: parenting B to A closes a loop and is refused",
        "PARENT_CYCLE: That parent would make two layers ride on each other.",
        set_parent(&mut document, &b, Some(&a), 0),
    );
    report.check(
        "FX-PARENT-004: and a layer cannot be its own parent",
        "PARENT_CYCLE: That parent would make two layers ride on each other.",
        set_parent(&mut document, &a, Some(&a), 0),
    );
    report.check(
        "FX-PARENT-004: neither refusal changed the project",
        true,
        *document.project() == before,
    );
}

/// FX-PARENT-005: keeping place, in the exact case.
fn fx_parent_005(report: &mut Report) {
    let (mut document, parent, child) = keep_place_case(
        ((50.0, 50.0), (400.0, 300.0), (2.0, 2.0), 90.0),
        ((10.0, 20.0), (500.0, 400.0), (1.0, 1.0), 30.0),
    );
    let before: Vec<(f64, f64)> = CORNERS.iter().map(|c| at(&document, &child, 0, *c)).collect();

    let comp = document
        .project()
        .composition(&Id::new(COMP))
        .expect("the composition is there");
    report.check(
        "FX-PARENT-005: the build says it can keep every point of the layer",
        true,
        parent_keep_place_is_exact(comp, &child, Some(&parent), 0),
    );

    set_parent(&mut document, &child, Some(&parent), 0);
    let (anchor, position, scale, rotation) = held(&document, &child);
    report.check("FX-PARENT-005: the new anchor", place((10.0, 20.0)), anchor);
    report.check(
        "FX-PARENT-005: the new position",
        place((100.0, 0.0)),
        position,
    );
    report.check("FX-PARENT-005: the new scale", place((50.0, 50.0)), scale);
    report.check("FX-PARENT-005: the new rotation", "-60.000000000", rotation);

    for (corner, was) in CORNERS.iter().zip(before) {
        report.point(
            &format!(
                "FX-PARENT-005: the corner ({}, {}) has not moved",
                corner.0, corner.1
            ),
            was,
            at(&document, &child, 0, *corner),
        );
    }
}

/// FX-PARENT-006: keeping place, in the case that cannot be exact.
fn fx_parent_006(report: &mut Report) {
    let (mut document, parent, child) = keep_place_case(
        ((0.0, 0.0), (300.0, 200.0), (2.0, 1.0), 0.0),
        ((0.0, 0.0), (500.0, 400.0), (1.0, 1.0), 45.0),
    );

    let comp = document
        .project()
        .composition(&Id::new(COMP))
        .expect("the composition is there");
    report.check(
        "FX-PARENT-006: the build says it cannot keep the layer's shape",
        false,
        parent_keep_place_is_exact(comp, &child, Some(&parent), 0),
    );

    set_parent(&mut document, &child, Some(&parent), 0);
    let (_, position, scale, rotation) = held(&document, &child);
    report.check(
        "FX-PARENT-006: the new position",
        place((100.0, 200.0)),
        position,
    );
    report.check("FX-PARENT-006: the new scale", place((50.0, 100.0)), scale);
    report.check("FX-PARENT-006: the new rotation", "45.000000000", rotation);

    // The anchor corner lands where it was; the others do not, and these are the values the
    // build must land on every time it cannot be right.
    for (corner, expected) in CORNERS.iter().zip([
        (500.0, 400.0),
        (570.7106781186548, 435.3553390593274),
        (358.5786437626905, 470.71067811865476),
        (429.28932188134524, 506.06601717798213),
    ]) {
        report.point(
            &format!(
                "FX-PARENT-006: the corner ({}, {}) lands where document 25 pins it",
                corner.0, corner.1
            ),
            expected,
            at(&document, &child, 0, *corner),
        );
    }
}

/// A parent and an unparented child, each with the four values document 25 gives it.
fn keep_place_case(
    parent_values: ((f64, f64), (f64, f64), (f64, f64), f64),
    child_values: ((f64, f64), (f64, f64), (f64, f64), f64),
) -> (Document, Id, Id) {
    let mut document = project();
    let parent = layer(&mut document, "layer-p", 0);
    let child = layer(&mut document, "layer-c", 1);
    for (id, (anchor, position, scale, rotation)) in
        [(&parent, parent_values), (&child, child_values)]
    {
        set(&mut document, id, Prop::Anchor, Value::Vec2(anchor.0, anchor.1));
        set(
            &mut document,
            id,
            Prop::Position,
            Value::Vec2(position.0, position.1),
        );
        set(&mut document, id, Prop::Scale, Value::Vec2(scale.0, scale.1));
        set(&mut document, id, Prop::Rotation, Value::Scalar(rotation));
    }
    (document, parent, child)
}

/// FX-PARENT-007: the file.
fn fx_parent_007(report: &mut Report) {
    let path = repo("Fixtures/projects/parenting_project.json");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading the fixture: {e}"));
    let loaded = match persist::load_str(&text) {
        Ok(loaded) => loaded,
        Err(d) => panic!("the parenting fixture did not open: {} - {}", d.message, d.detail),
    };
    let missing: Vec<&anime_compositor::diagnostics::Diagnostic> = loaded
        .warnings
        .iter()
        .filter(|w| w.id == DiagnosticId::ParentReferenceMissing)
        .collect();
    report.check(
        "FX-PARENT-007: opening it reports one unresolved parent",
        1,
        missing.len(),
    );
    report.check(
        "FX-PARENT-007: and its words name the layer that has no parent to ride on",
        "The layer \"Orphan\" is parented to a layer that is not in this composition.",
        missing
            .first()
            .map(|d| d.message.clone())
            .unwrap_or_else(|| "nothing was reported".to_string()),
    );

    let project = loaded.document.project();
    let comp = project
        .compositions
        .first()
        .expect("the fixture has a composition");
    for (frame, origin) in [
        (0, (1110.0, 540.0)),
        (6, (1106.1939766255643, 559.1341716182545)),
        (12, (1095.3553390593274, 575.3553390593274)),
        (24, (1060.0, 590.0)),
    ] {
        report.point(
            &format!("FX-PARENT-007: layer-child's origin at frame {frame}"),
            origin,
            world_transform(comp, &Id::new("layer-child"), frame).apply(0.0, 0.0),
        );
    }
    // A layer whose parent is not there draws where it would with no parent at all.
    report.point(
        "FX-PARENT-007: layer-orphan draws as if it had no parent",
        (10.0, 10.0),
        world_transform(comp, &Id::new("layer-orphan"), 0).apply(0.0, 0.0),
    );
    report.check(
        "FX-PARENT-007: and the reference it cannot resolve is kept, not cleared",
        "layer-gone",
        comp.layer(&Id::new("layer-orphan"))
            .and_then(|l| l.parent.clone())
            .map(|p| p.as_str().to_string())
            .unwrap_or_else(|| "it was cleared".to_string()),
    );
}

/// FX-PARENT-008: deleting a parent.
///
/// The child lets go where it stands, as one entry to undo. This is the one place parenting
/// does not follow the matte, whose reference is left dangling on purpose: a dangling parent
/// would move the layer, and a layer deleted by mistake would take its children with it.
fn fx_parent_008(report: &mut Report) {
    let (mut document, parent, child) = keep_place_case(
        ((50.0, 50.0), (400.0, 300.0), (2.0, 2.0), 90.0),
        ((10.0, 20.0), (500.0, 400.0), (1.0, 1.0), 30.0),
    );
    set_parent(&mut document, &child, Some(&parent), 0);

    document
        .apply_all(vec![
            Command::SetParent {
                composition: Id::new(COMP),
                layer_id: child.clone(),
                parent: None,
                frame: 0,
                keep_place: true,
            },
            Command::RemoveLayer {
                composition: Id::new(COMP),
                layer_id: parent.clone(),
            },
        ])
        .expect("the parent is deleted");

    let (anchor, position, scale, rotation) = held(&document, &child);
    report.check("FX-PARENT-008: the anchor", place((10.0, 20.0)), anchor);
    report.check("FX-PARENT-008: the position", place((500.0, 400.0)), position);
    report.check("FX-PARENT-008: the scale", place((100.0, 100.0)), scale);
    report.check("FX-PARENT-008: the rotation", "30.000000000", rotation);
    report.check(
        "FX-PARENT-008: the child has no parent",
        true,
        document
            .project()
            .composition(&Id::new(COMP))
            .and_then(|c| c.layer(&child))
            .expect("the child is there")
            .parent
            .is_none(),
    );

    document.undo().expect("one entry to undo");
    let (_, position, scale, rotation) = held(&document, &child);
    report.check(
        "FX-PARENT-008: one undo puts the parent back",
        true,
        document
            .project()
            .composition(&Id::new(COMP))
            .and_then(|c| c.layer(&parent))
            .is_some(),
    );
    report.check(
        "FX-PARENT-008: and the child's FX-PARENT-005 values with it",
        format!("{}, {}, {}", place((100.0, 0.0)), place((50.0, 50.0)), "-60.000000000"),
        format!("{position}, {scale}, {rotation}"),
    );
}

// ---------------------------------------------------------------------------------------
// The artifact
// ---------------------------------------------------------------------------------------

fn write_artifact(report: &Report) {
    let passed = report.rows.iter().filter(|(_, e, a)| e == a).count();
    let mut out = String::from("# Parenting against document 25\n\n");
    out.push_str(&format!(
        "Generated by `tests/b13b_parenting.rs`. {passed} of {} checks pass.\n\n",
        report.rows.len()
    ));
    out.push_str(
        "D-57 lets a layer ride on another layer's transform. Every expected value in this \
         table is FX-PARENT-001 to 008 in `Markdown/25_Test_Fixture_Catalog.md`, printed by \
         `tools/parent_reference.py`, which walks a point through document 21's four steps one \
         at a time and never builds a matrix. This build multiplies matrices, so the two \
         columns below are two answers and not one answer twice.\n\n",
    );
    out.push_str(
        "Positions and corners are composition pixels to nine places, which is document 25's \
         tolerance. Scale is written as a percentage, the way a file and a panel write it; the \
         model holds it as a factor (D-22).\n\n",
    );
    out.push_str("| Check | Expected | Actual | |\n|---|---|---|---|\n");
    for (check, expected, actual) in &report.rows {
        out.push_str(&format!(
            "| {check} | `{expected}` | `{actual}` | {} |\n",
            match expected == actual {
                true => "pass",
                false => "**FAIL**",
            }
        ));
    }
    out.push_str(
        "\n## The two things this build now says that it could not before\n\n\
         Document 28 gained both by D-57, and this is the table its words are read from.\n\n\
         - **`PARENT_CYCLE`.** \"That parent would make two layers ride on each other.\" Refused \
         the way a matte loop is: the command changes nothing, which is the FX-PARENT-004 row \
         above comparing the whole project before and after.\n\
         - **`PARENT_REFERENCE_MISSING`.** \"The layer ... is parented to a layer that is not in \
         this composition.\" A warning, not a refusal: the reference is kept in the file and the \
         layer draws where it would with no parent, which is the FX-PARENT-007 rows above.\n",
    );
    out.push_str(
        "\n## What is not in this table\n\n\
         The panel. Choosing a parent from the layer's inspector, and what the window says when \
         a layer cannot keep its shape exactly, are checked in \
         `verification/B-13b_panel_table.md`, and walked by hand in \
         `verification/B-13b_parent_playtest.md`. The camera is not here at all: B-13c is a \
         separate contract and parenting does not imply it.\n",
    );

    let path = repo("verification/B-13b_parenting_table.md");
    fs::write(&path, out).unwrap_or_else(|e| panic!("writing {}: {e}", path.display()));
}
