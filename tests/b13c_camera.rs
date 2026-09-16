//! B-13c: the camera and depth, against D-58.
//!
//! Writes `verification/B-13c_camera_table.md`.
//!
//! # Where the expected values come from
//!
//! `Markdown/25_Test_Fixture_Catalog.md`, FX-CAM-001 to 011, every number of which is printed by
//! `tools/camera_reference.py`. That script reuses `tools/parent_reference.py` to walk a point
//! through document 21's four steps one at a time, and then does D-58's two lines by hand: the
//! scale `zoom / (world depth - camera depth)`, and the point measured from the camera and
//! scaled about the centre of the frame. It never builds a matrix; this build folds the
//! projection into the layer's transform and multiplies matrices. An agreement between the two
//! is two answers and not one answer twice (ADR-009).
//!
//! Nothing here is a snapshot of a run. Every expected number below was written into document 25
//! before any of the code under test existed.
//!
//! # What this proves
//!
//! That `screen = centre + (p_comp - camera_position) * zoom / (world depth - camera depth)` is
//! what draws, read through `compose::screen_transform`, which is the map the renderer builds at
//! document 21's step 4. That a projection which is the identity is **left out rather than
//! applied**, so every fixture written before D-58 lands exactly on its expected value and not
//! within a tolerance of it. That depth rides the parent chain additively. That a zoom and a
//! dolly are different moves. That what is drawn first is a question with a different answer on
//! different frames. That a plane level with the camera or behind it reports
//! `CAMERA_PLANE_BEHIND` and draws nothing. And that a camera and a depth survive a file.
//!
//! # The one thing no command can do yet
//!
//! Neither a layer's `depth` nor any of the camera's three properties has a command that
//! **keys** it: `SetKeyframe` names one of document 21's five transform properties and D-58 did
//! not add a sixth. The build reads such keys perfectly well - the camera fixture carries a
//! keyed camera - so the three animated cases here go in through the file, which is the only
//! door open to them. That is a gap in the window, not in the contract, and it is written into
//! the artifact rather than worked around quietly.
//!
//! # What is deliberately not here
//!
//! The panel. Setting a depth from the layer's inspector, and the camera's own controls, are the
//! window's half of B-13c and are checked in `app/src/main.rs`'s own tests.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use anime_compositor::command::{Command, Document, Target};
use anime_compositor::compose::{plan_frame, screen_transform, world_depth, world_transform};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::media::import_sequence;
use anime_compositor::model::{
    Asset, AssetKind, Camera, CameraProp, Composition, Id, Interpretation, Layer, Project, Prop,
    Value,
};
use anime_compositor::persist::{self, Preserved};
use anime_compositor::time::{ExposureMap, FrameRate};
use serde_json::json;

const COMP: &str = "comp-camera";
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

    /// A pair of composition pixels, to nine places, which is document 25's tolerance.
    fn point(&mut self, check: &str, expected: (f64, f64), actual: (f64, f64)) {
        self.check(check, place(expected), place(actual));
    }

    /// One number to nine places.
    fn number(&mut self, check: &str, expected: f64, actual: f64) {
        self.check(check, num(expected), num(actual));
    }
}

fn place(p: (f64, f64)) -> String {
    format!("{:.9}, {:.9}", p.0, p.1)
}

fn num(v: f64) -> String {
    format!("{v:.9}")
}

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

// ---------------------------------------------------------------------------------------
// Building a composition of bare layers
// ---------------------------------------------------------------------------------------

/// A project with one empty composition, 1920x1080 at 24, forty-nine frames long, which is
/// FX-CAM-003's track from frame 0 to frame 48 inclusive.
fn project() -> Document {
    project_of(1920, 1080)
}

fn project_of(width: u32, height: u32) -> Document {
    let mut document = Document::new(Project::new(Id::new("proj-camera-test")));
    // One asset, shared by every layer below. Most cases here ask where a layer's pixels would
    // land, which does not need the pixels; the cases that ask which layers are drawn, and in
    // what order, use the reference shot further down instead.
    document
        .apply(Command::AddAsset {
            asset: Asset::sequence(Id::new(ASSET), "unused", "unused_%03d.png"),
        })
        .expect("the asset is added");
    document
        .apply(Command::AddComposition {
            composition: Box::new(Composition::new(
                Id::new(COMP),
                "Camera",
                width,
                height,
                FrameRate::new(24, 1).expect("24 fps"),
                0,
                49,
            )),
        })
        .expect("the composition is added");
    document
}

fn layer(document: &mut Document, id: &str, index: usize) -> Id {
    let layer_id = Id::new(id);
    document
        .apply(Command::AddLayer {
            composition: Id::new(COMP),
            layer: Box::new(Layer::new(layer_id.clone(), id, Id::new(ASSET), 0, 49)),
            index,
        })
        .unwrap_or_else(|d| panic!("adding {id}: {}", d.message));
    layer_id
}

fn set(document: &mut Document, layer_id: &Id, prop: Prop, value: Value) {
    document
        .apply(Command::SetPropertyBase {
            composition: Id::new(COMP),
            target: Target::Layer(layer_id.clone()),
            prop,
            value,
        })
        .unwrap_or_else(|d| panic!("setting {prop:?}: {}", d.message));
}

fn set_depth(document: &mut Document, layer_id: &Id, value: f64) {
    document
        .apply(Command::SetDepth {
            composition: Id::new(COMP),
            layer_id: layer_id.clone(),
            value,
        })
        .unwrap_or_else(|d| panic!("setting the depth of {layer_id}: {}", d.message));
}

fn set_camera(document: &mut Document, prop: CameraProp, value: Value) {
    document
        .apply(Command::SetCameraProperty {
            composition: Id::new(COMP),
            prop,
            value,
        })
        .unwrap_or_else(|d| panic!("setting a camera property: {}", d.message));
}

/// The camera every case but FX-CAM-001 and FX-CAM-011 names for itself: a 36 mm lens, depth
/// -1920 and zoom 1920, which puts whole pixels in the table and can be checked by hand.
fn camera_36mm(document: &mut Document) {
    set_camera(document, CameraProp::Depth, Value::Scalar(-1920.0));
    set_camera(document, CameraProp::Zoom, Value::Scalar(1920.0));
}

fn composition(document: &Document) -> &Composition {
    document
        .project()
        .composition(&Id::new(COMP))
        .expect("the composition is there")
}

/// Where a point of the layer's own space lands in composition space, before the camera.
fn at(document: &Document, layer_id: &Id, frame: i32, point: (f64, f64)) -> (f64, f64) {
    world_transform(composition(document), layer_id, frame).apply(point.0, point.1)
}

/// Where that point lands on screen, the camera included. `None` is the layer that is level with
/// the camera or behind it.
fn screen(
    document: &Document,
    layer_id: &Id,
    frame: i32,
    point: (f64, f64),
) -> Option<(f64, f64)> {
    screen_transform(composition(document), layer_id, frame).map(|t| t.apply(point.0, point.1))
}

fn drawn(document: &Document, layer_id: &Id, frame: i32, point: (f64, f64)) -> (f64, f64) {
    screen(document, layer_id, frame, point).expect("the layer is in front of the camera")
}

/// The size the plane is drawn at, measured off the screen rather than recomputed: a hundred
/// pixels of the layer, divided by the distance they cover on screen. A distance rather than a
/// difference in x, so FX-CAM-007's parent, which is turned through ninety degrees, reads the
/// same size as an upright one.
fn drawn_at(document: &Document, layer_id: &Id, frame: i32) -> f64 {
    let origin = drawn(document, layer_id, frame, (0.0, 0.0));
    let across = drawn(document, layer_id, frame, (100.0, 0.0));
    (across.0 - origin.0).hypot(across.1 - origin.1) / 100.0
}

// ---------------------------------------------------------------------------------------
// Keyframes, which no command can set
// ---------------------------------------------------------------------------------------

/// The same document with keyframes written into it through its own file format.
///
/// See the module note: nothing keys a depth or a camera property yet, and this is the door that
/// is open. The project is written out by this build, edited as JSON, and read back by this
/// build, so the keys under test are the keys a hand-written project file would carry.
fn with_keys(document: &Document, edit: impl FnOnce(&mut serde_json::Value)) -> Document {
    let text = persist::to_json(document.project(), &Preserved::none());
    let mut root: serde_json::Value =
        serde_json::from_str(&text).expect("what this build writes is JSON");
    edit(&mut root);
    let edited = serde_json::to_string_pretty(&root).expect("the edited project serialises");
    persist::load_str(&edited)
        .unwrap_or_else(|d| panic!("the keyed project did not open: {} - {}", d.message, d.detail))
        .document
}

/// Linear keys, in document 19's shape.
fn keys(values: &[(i32, serde_json::Value)]) -> serde_json::Value {
    serde_json::Value::Array(
        values
            .iter()
            .map(|(frame, value)| json!({ "frame": frame, "value": value, "interp": "linear" }))
            .collect(),
    )
}

fn camera_json<'a>(root: &'a mut serde_json::Value, prop: &str) -> &'a mut serde_json::Value {
    &mut root["compositions"][0]["camera"][prop]["keyframes"]
}

fn layer_json<'a>(root: &'a mut serde_json::Value, id: &str) -> &'a mut serde_json::Value {
    root["compositions"][0]["layers"]
        .as_array_mut()
        .expect("the composition has layers")
        .iter_mut()
        .find(|l| l["id"] == id)
        .unwrap_or_else(|| panic!("{id} is in the file this build wrote"))
}

// ---------------------------------------------------------------------------------------
// Building a composition of layers that actually draw
// ---------------------------------------------------------------------------------------

fn shot_root() -> PathBuf {
    repo("Fixtures/reference_shot")
}

/// Layer 1 of the reference shot as an asset, with paths relative to the project the way
/// document 07 stores them.
///
/// One asset is enough for every case that needs real drawings: those cases ask which layers are
/// drawn and in what order, never what they hold, so they can all name the same cel.
fn shot_asset() -> Asset {
    let dir = shot_root().join("layer1");
    let files: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .map(|e| e.expect("directory entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "png"))
        .collect();
    let imported = import_sequence(&files)
        .asset
        .expect("layer1 imports as a sequence");
    let frames: BTreeMap<u32, String> = imported
        .frames()
        .iter()
        .map(|(number, path)| {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .expect("a UTF-8 file name");
            (*number, format!("layer1/{name}"))
        })
        .collect();
    Asset {
        id: Id::new("asset-layer1"),
        kind: AssetKind::ImageSequence,
        name: "layer1".to_string(),
        path: None,
        pattern: Some(imported.pattern().to_string()),
        frames,
        interpretation: Interpretation::default(),
        redistribute: true,
    }
}

fn media_project() -> Document {
    let mut document = Document::new(Project::new(Id::new("proj-camera-media")));
    document
        .apply(Command::AddAsset {
            asset: shot_asset(),
        })
        .expect("the asset is added");
    document
        .apply(Command::AddComposition {
            composition: Box::new(Composition::new(
                Id::new(COMP),
                "Camera",
                1920,
                1080,
                FrameRate::new(24, 1).expect("24 fps"),
                0,
                49,
            )),
        })
        .expect("the composition is added");
    document
}

/// A layer holding layer 1's one drawing for the whole composition.
///
/// Its transform is left alone on purpose: the reference shot's cels fill the frame from the
/// origin, so a layer that has not been moved covers the picture whatever the camera does to it,
/// and no case here asks where these layers land, only whether and in what order they are drawn.
fn media_layer(document: &mut Document, id: &str, index: usize) -> Id {
    let layer_id = Id::new(id);
    let mut layer = Layer::new(layer_id.clone(), id, Id::new("asset-layer1"), 0, 49);
    layer.exposure_spans = ExposureMap::from_lengths(&[(0, 49)])
        .expect("one drawing, held")
        .spans()
        .to_vec();
    document
        .apply(Command::AddLayer {
            composition: Id::new(COMP),
            layer: Box::new(layer),
            index,
        })
        .unwrap_or_else(|d| panic!("adding {id}: {}", d.message));
    layer_id
}

/// The layers of one planned frame, in the order they are drawn, first to last.
fn drawn_order(document: &Document, frame: i32) -> String {
    let mut log = FrameLog::new(8);
    let plan = plan_frame(
        document.project(),
        &Id::new(COMP),
        frame,
        &shot_root(),
        &mut log,
    )
    .unwrap_or_else(|d| panic!("frame {frame} plans: {}", d.message));
    plan.layers
        .iter()
        .map(|l| l.id.as_str().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// What planning one frame reported, as diagnostic identifiers.
fn diagnostics_at(document: &Document, frame: i32, root: &PathBuf) -> String {
    let mut log = FrameLog::new(8);
    let _ = plan_frame(document.project(), &Id::new(COMP), frame, root, &mut log);
    let mut ids: Vec<String> = log
        .finish()
        .into_iter()
        .map(|d| d.id.as_str().to_string())
        .collect();
    ids.dedup();
    ids.join(", ")
}

const SAMPLES: [(f64, f64); 3] = [(0.0, 0.0), (10.0, 0.0), (0.0, 10.0)];

// ---------------------------------------------------------------------------------------
// The test
// ---------------------------------------------------------------------------------------

#[test]
fn b13c_camera() {
    let mut report = Report::default();

    fx_cam_001(&mut report);
    fx_cam_002(&mut report);
    fx_cam_003(&mut report);
    fx_cam_004(&mut report);
    fx_cam_005(&mut report);
    fx_cam_006(&mut report);
    fx_cam_007(&mut report);
    fx_cam_008(&mut report);
    fx_cam_009(&mut report);
    fx_cam_010(&mut report);
    fx_cam_011(&mut report);

    write_artifact(&report);
    let failed: Vec<&(String, String, String)> =
        report.rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// FX-CAM-001: the camera nobody has touched changes nothing.
fn fx_cam_001(report: &mut Report) {
    let mut document = project();
    let l = layer(&mut document, "layer-a", 0);
    set(&mut document, &l, Prop::Anchor, Value::Vec2(50.0, 50.0));
    set(&mut document, &l, Prop::Position, Value::Vec2(400.0, 300.0));
    // D-22: the model holds scale as a factor, and 150 is what a file and a panel write.
    set(&mut document, &l, Prop::Scale, Value::Vec2(1.5, 1.5));
    set(&mut document, &l, Prop::Rotation, Value::Scalar(30.0));

    for (point, comp, screen_point) in [
        (
            SAMPLES[0],
            (372.5480947161671, 197.5480947161671),
            (372.54809471616704, 197.5480947161671),
        ),
        (
            SAMPLES[1],
            (385.5384757729337, 205.0480947161671),
            (385.5384757729337, 205.0480947161671),
        ),
        (
            SAMPLES[2],
            (365.0480947161671, 210.53847577293368),
            (365.04809471616704, 210.53847577293368),
        ),
    ] {
        report.point(
            &format!(
                "FX-CAM-001: the layer's ({}, {}) in composition space",
                point.0, point.1
            ),
            comp,
            at(&document, &l, 0, point),
        );
        report.point(
            &format!(
                "FX-CAM-001: and its ({}, {}) on screen, under the camera nobody has touched",
                point.0, point.1
            ),
            screen_point,
            drawn(&document, &l, 0, point),
        );
    }

    // The case exists for this row. Document 25: carrying a point out to the camera and back is
    // two roundings, so a build that applies an identity projection lands within a tolerance of
    // every pre-D-58 fixture instead of exactly on it.
    report.check(
        "FX-CAM-001: the projection is left out rather than applied, so the screen point is the \
         composition point bit for bit",
        true,
        SAMPLES
            .iter()
            .all(|p| screen(&document, &l, 0, *p) == Some(at(&document, &l, 0, *p))),
    );
    report.check(
        "FX-CAM-001: and the composition holds no camera record, because nobody touched one",
        true,
        composition(&document).camera.is_none(),
    );
}

/// FX-CAM-002: one plane, at four depths, seen by a 36 mm camera the file names for itself.
fn fx_cam_002(report: &mut Report) {
    for (depth, size, origin, across) in [
        (-960.0, 2.0, (960.0, 540.0), (1160.0, 540.0)),
        (0.0, 1.0, (960.0, 540.0), (1060.0, 540.0)),
        (
            960.0,
            0.6666666666666666,
            (960.0, 540.0),
            (1026.6666666666667, 540.0),
        ),
        (1920.0, 0.5, (960.0, 540.0), (1010.0, 540.0)),
    ] {
        let mut document = project();
        let l = layer(&mut document, "layer-a", 0);
        camera_36mm(&mut document);
        set(&mut document, &l, Prop::Position, Value::Vec2(960.0, 540.0));
        set_depth(&mut document, &l, depth);

        report.point(
            &format!("FX-CAM-002: at depth {depth}, the origin cannot move"),
            origin,
            drawn(&document, &l, 0, (0.0, 0.0)),
        );
        report.point(
            &format!("FX-CAM-002: at depth {depth}, the layer's (100, 0)"),
            across,
            drawn(&document, &l, 0, (100.0, 0.0)),
        );
        report.number(
            &format!("FX-CAM-002: and so it is drawn at"),
            size,
            drawn_at(&document, &l, 0),
        );
    }
}

/// FX-CAM-003: the sideways track, which is what the whole entry is for.
fn fx_cam_003(report: &mut Report) {
    let document = sideways_track();
    let planes = [
        ("near", Id::new("layer-near")),
        ("middle", Id::new("layer-middle")),
        ("far", Id::new("layer-far")),
    ];

    for (frame, near, middle, far) in [
        (0, 1160.0, 1110.0, 1060.0),
        (
            16,
            1026.6666666666667,
            1010.0,
            993.3333333333334,
        ),
        (
            32,
            893.3333333333335,
            910.0,
            926.6666666666667,
        ),
        (48, 760.0, 810.0, 860.0),
    ] {
        for ((name, id), expected) in planes.iter().zip([near, middle, far]) {
            report.number(
                &format!("FX-CAM-003: the {name} plane's origin x at frame {frame}"),
                expected,
                drawn(&document, id, frame, (0.0, 0.0)).0,
            );
        }
    }

    // The camera travelled 400 pixels; the planes did not travel the same distance as each other,
    // which is the whole of parallax.
    for ((name, id), travelled) in planes.iter().zip([400.0, 300.0, 200.0]) {
        let start = drawn(&document, id, 0, (0.0, 0.0)).0;
        let end = drawn(&document, id, 48, (0.0, 0.0)).0;
        report.number(
            &format!("FX-CAM-003: the {name} plane travelled, in pixels"),
            travelled,
            (end - start).abs(),
        );
    }
}

/// Three planes at depths 0, 640 and 1920, with the camera keyed sideways from (760, 540) to
/// (1160, 540) over forty-eight frames.
fn sideways_track() -> Document {
    let mut document = project();
    let near = layer(&mut document, "layer-near", 0);
    let middle = layer(&mut document, "layer-middle", 1);
    let far = layer(&mut document, "layer-far", 2);
    camera_36mm(&mut document);
    for id in [&near, &middle, &far] {
        set(&mut document, id, Prop::Position, Value::Vec2(960.0, 540.0));
    }
    // The near plane is left without a depth at all, as the fixture file leaves it: depth 0 and
    // no depth have to be the same thing.
    set_depth(&mut document, &middle, 640.0);
    set_depth(&mut document, &far, 1920.0);
    with_keys(&document, |root| {
        *camera_json(root, "position") = keys(&[
            (0, json!([760.0, 540.0])),
            (48, json!([1160.0, 540.0])),
        ]);
    })
}

/// FX-CAM-004: a zoom and a dolly are different moves.
fn fx_cam_004(report: &mut Report) {
    let mut base = project();
    let near = layer(&mut base, "layer-near", 0);
    let far = layer(&mut base, "layer-far", 1);
    camera_36mm(&mut base);
    for id in [&near, &far] {
        set(&mut base, id, Prop::Position, Value::Vec2(960.0, 540.0));
    }
    set_depth(&mut base, &far, 1920.0);

    let zoomed = with_keys(&base, |root| {
        *camera_json(root, "zoom") = keys(&[(0, json!(1920.0)), (48, json!(2880.0))]);
    });
    let dollied = with_keys(&base, |root| {
        *camera_json(root, "depth") = keys(&[(0, json!(-1920.0)), (48, json!(-960.0))]);
    });

    for (move_name, document, rows) in [
        (
            "zoom to 2880",
            &zoomed,
            [(0, 1060.0, 1010.0, 2.0), (48, 1110.0, 1035.0, 2.0)],
        ),
        (
            "dolly in 960",
            &dollied,
            [
                (0, 1060.0, 1010.0, 2.0),
                (48, 1160.0, 1026.6666666666667, 3.0),
            ],
        ),
    ] {
        for (frame, near_x, far_x, ratio) in rows {
            let near_at = drawn(document, &near, frame, (100.0, 0.0)).0;
            let far_at = drawn(document, &far, frame, (100.0, 0.0)).0;
            report.number(
                &format!("FX-CAM-004: {move_name}, the near plane's (100, 0) x at frame {frame}"),
                near_x,
                near_at,
            );
            report.number(
                &format!("FX-CAM-004: {move_name}, the far plane's (100, 0) x at frame {frame}"),
                far_x,
                far_at,
            );
            report.number(
                &format!("FX-CAM-004: {move_name}, near over far at frame {frame}"),
                ratio,
                (near_at - 960.0) / (far_at - 960.0),
            );
        }
    }
}

/// FX-CAM-005: what is drawn first.
fn fx_cam_005(report: &mut Report) {
    let mut document = media_project();
    let _a = media_layer(&mut document, "layer-a", 0);
    let b = media_layer(&mut document, "layer-b", 1);
    let c = media_layer(&mut document, "layer-c", 2);
    let d = media_layer(&mut document, "layer-d", 3);
    camera_36mm(&mut document);
    set_depth(&mut document, &b, 1920.0);
    set_depth(&mut document, &c, 1920.0);
    set_depth(&mut document, &d, -960.0);

    report.check(
        "FX-CAM-005: the stack is A, B, C, D at depths 0, 1920, 1920, -960, and they are drawn",
        "layer-b, layer-c, layer-a, layer-d",
        drawn_order(&document, 0),
    );
    // B and C share a depth and keep the order the composition gives them; D is last in the
    // stack and nearest the viewer, so depth overrode the stack without changing it.
    report.check(
        "FX-CAM-005: and the layer stack itself is untouched",
        "layer-a, layer-b, layer-c, layer-d",
        composition(&document)
            .layer_order()
            .iter()
            .map(|id| id.as_str().to_string())
            .collect::<Vec<_>>()
            .join(", "),
    );
}

/// FX-CAM-006: level with the camera, and behind it.
fn fx_cam_006(report: &mut Report) {
    for (depth, ahead, size) in [
        (-2000.0, -80.0, None),
        (-1920.0, 0.0, None),
        (-1919.0, 1.0, Some(1920.0)),
        (-960.0, 960.0, Some(2.0)),
    ] {
        let mut document = media_project();
        let l = media_layer(&mut document, "layer-a", 0);
        camera_36mm(&mut document);
        set_depth(&mut document, &l, depth);

        report.check(
            &format!("FX-CAM-006: at depth {depth}, in front of the camera by {ahead}, drawn at"),
            size.map(num).unwrap_or_else(|| "not drawn".to_string()),
            match screen(&document, &l, 0, (0.0, 0.0)) {
                None => "not drawn".to_string(),
                Some(_) => num(drawn_at(&document, &l, 0)),
            },
        );
        report.check(
            &format!("FX-CAM-006: at depth {depth}, planning the frame reports"),
            match size {
                None => "CAMERA_PLANE_BEHIND",
                Some(_) => "",
            },
            diagnostics_at(&document, 0, &shot_root()),
        );
        report.check(
            &format!("FX-CAM-006: at depth {depth}, the layer's own record is unchanged"),
            num(depth),
            num(world_depth(composition(&document), &l, 0)),
        );
    }
}

/// FX-CAM-007: a layer rides on its parent's plane.
fn fx_cam_007(report: &mut Report) {
    let mut document = project();
    let p = layer(&mut document, "layer-p", 0);
    let c = layer(&mut document, "layer-c", 1);
    camera_36mm(&mut document);
    set(&mut document, &p, Prop::Anchor, Value::Vec2(50.0, 50.0));
    set(&mut document, &p, Prop::Position, Value::Vec2(400.0, 300.0));
    set(&mut document, &p, Prop::Rotation, Value::Scalar(90.0));
    set(&mut document, &c, Prop::Position, Value::Vec2(100.0, 0.0));
    document
        .apply(Command::SetParent {
            composition: Id::new(COMP),
            layer_id: c.clone(),
            parent: Some(p.clone()),
            frame: 0,
            keep_place: false,
        })
        .expect("the parent is set");
    set_depth(&mut document, &p, 1920.0);

    report.check(
        "FX-CAM-007: the child has no depth of its own",
        true,
        composition(&document)
            .layer(&c)
            .expect("the child is there")
            .depth
            .is_none(),
    );
    for (name, id) in [("P", &p), ("C", &c)] {
        report.number(
            &format!("FX-CAM-007: {name}'s world depth"),
            1920.0,
            world_depth(composition(&document), id, 0),
        );
        report.number(
            &format!("FX-CAM-007: and {name} is drawn at"),
            0.5,
            drawn_at(&document, id, 0),
        );
    }

    // The composition column is FX-PARENT-001 unchanged, which is the check that D-58 adds a
    // step to document 21 rather than editing one.
    for (point, comp, screen_point) in [
        (SAMPLES[0], (450.0, 350.0), (705.0, 445.0)),
        (SAMPLES[1], (450.0, 360.0), (705.0, 450.0)),
        (SAMPLES[2], (440.0, 350.0), (700.0, 445.0)),
    ] {
        report.point(
            &format!(
                "FX-CAM-007: the child's ({}, {}) in composition space",
                point.0, point.1
            ),
            comp,
            at(&document, &c, 0, point),
        );
        report.point(
            &format!(
                "FX-CAM-007: and the child's ({}, {}) on screen",
                point.0, point.1
            ),
            screen_point,
            drawn(&document, &c, 0, point),
        );
    }
}

/// FX-CAM-008: the file.
fn fx_cam_008(report: &mut Report) {
    let path = repo("Fixtures/projects/camera_project.json");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading the fixture: {e}"));
    let loaded = match persist::load_str(&text) {
        Ok(loaded) => loaded,
        Err(d) => panic!("the camera fixture did not open: {} - {}", d.message, d.detail),
    };
    let written = persist::to_json(loaded.document.project(), &loaded.preserved);
    report.check(
        "FX-CAM-008: opening it and saving it reproduces the file byte for byte",
        "identical",
        match written == text {
            true => "identical",
            false => "different",
        },
    );

    let document = loaded.document;
    let comp = composition_named(&document, "comp-main");
    report.check(
        "FX-CAM-008: layer-near carries no depth field and did not gain one",
        true,
        comp.layer(&Id::new("layer-near"))
            .expect("layer-near is there")
            .depth
            .is_none(),
    );
    for (id, depth) in [("layer-middle", 640.0), ("layer-far", 1920.0)] {
        report.number(
            &format!("FX-CAM-008: {id}'s depth"),
            depth,
            world_depth(comp, &Id::new(id), 0),
        );
    }
    report.check(
        "FX-CAM-008: its layer_order is deliberately not the order the planes are drawn in",
        "layer-near, layer-middle, layer-far",
        comp.layer_order()
            .iter()
            .map(|id| id.as_str().to_string())
            .collect::<Vec<_>>()
            .join(", "),
    );
    report.check(
        "FX-CAM-008: and the camera it carries is keyed",
        true,
        comp.camera
            .as_ref()
            .is_some_and(|c| c.position.is_animated()),
    );

    // The same three numbers FX-CAM-003 pins, out of a file rather than out of a command.
    for (frame, near, middle, far) in [(0, 1160.0, 1110.0, 1060.0), (48, 760.0, 810.0, 860.0)] {
        for (name, id, expected) in [
            ("near", "layer-near", near),
            ("middle", "layer-middle", middle),
            ("far", "layer-far", far),
        ] {
            report.number(
                &format!("FX-CAM-008: the {name} plane's origin x at frame {frame}"),
                expected,
                screen_transform(comp, &Id::new(id), frame)
                    .expect("the plane is in front of the camera")
                    .apply(0.0, 0.0)
                    .0,
            );
        }
    }

    // Its drawings are not there, which document 25 says is expected of this fixture.
    let mut log = FrameLog::new(8);
    let _ = plan_frame(
        document.project(),
        &Id::new("comp-main"),
        0,
        &repo("Fixtures/projects"),
        &mut log,
    );
    report.check(
        "FX-CAM-008: planning a frame of it reports its absent drawings and nothing else",
        "MEDIA_MISSING",
        {
            let mut ids: Vec<String> = log
                .finish()
                .into_iter()
                .map(|d| d.id.as_str().to_string())
                .collect();
            ids.dedup();
            ids.join(", ")
        },
    );
}

fn composition_named<'a>(document: &'a Document, id: &str) -> &'a Composition {
    document
        .project()
        .composition(&Id::new(id))
        .expect("the composition is there")
}

/// FX-CAM-009: a matte is a plane too.
fn fx_cam_009(report: &mut Report) {
    let mut document = project();
    let a = layer(&mut document, "layer-a", 0);
    let m = layer(&mut document, "layer-m", 1);
    camera_36mm(&mut document);
    set_camera(&mut document, CameraProp::Position, Value::Vec2(760.0, 540.0));
    for id in [&a, &m] {
        set(&mut document, id, Prop::Position, Value::Vec2(960.0, 540.0));
    }
    set_depth(&mut document, &m, 1920.0);
    document
        .apply(Command::SetMatte {
            composition: Id::new(COMP),
            layer_id: a.clone(),
            matte: Some(m.clone()),
            matte_only: true,
        })
        .expect("the matte is set");

    let a_x = drawn(&document, &a, 0, (0.0, 0.0)).0;
    let m_x = drawn(&document, &m, 0, (0.0, 0.0)).0;
    report.number("FX-CAM-009: layer A's origin x, at depth 0", 1160.0, a_x);
    report.number("FX-CAM-009: its matte's origin x, at depth 1920", 1060.0, m_x);
    // Written down because it is correct and looks like a defect: a matte is meant to share its
    // layer's depth, and nothing in this contract forces it to.
    report.number(
        "FX-CAM-009: so the matte slides against the layer it shapes, in pixels",
        100.0,
        a_x - m_x,
    );
}

/// FX-CAM-010: a depth that is animated, and a draw order that changes because of it.
fn fx_cam_010(report: &mut Report) {
    let mut base = media_project();
    let _a = media_layer(&mut base, "layer-a", 0);
    let b = media_layer(&mut base, "layer-b", 1);
    camera_36mm(&mut base);
    set_depth(&mut base, &b, -960.0);
    let document = with_keys(&base, |root| {
        layer_json(root, "layer-b")["depth"]["keyframes"] =
            keys(&[(0, json!(-960.0)), (48, json!(1920.0))]);
    });

    for (frame, depth, size, order) in [
        (0, -960.0, 2.0, "layer-a, layer-b"),
        (24, 480.0, 0.8, "layer-b, layer-a"),
        (48, 1920.0, 0.5, "layer-b, layer-a"),
    ] {
        report.number(
            &format!("FX-CAM-010: B's depth at frame {frame}"),
            depth,
            world_depth(composition(&document), &b, frame),
        );
        report.number(
            &format!("FX-CAM-010: B is drawn at, at frame {frame}"),
            size,
            drawn_at(&document, &b, frame),
        );
        report.check(
            &format!("FX-CAM-010: and the layers are drawn, at frame {frame}"),
            order,
            drawn_order(&document, frame),
        );
    }
}

/// FX-CAM-011: what the default lens is, in the terms After Effects uses for it.
fn fx_cam_011(report: &mut Report) {
    for (width, height, zoom, angle, one_width_back) in [
        (
            1920u32,
            1080u32,
            2666.6666666666665,
            39.597752709049864,
            0.5813953488372093,
        ),
        (
            1280u32,
            720u32,
            1777.7777777777778,
            39.597752709049864,
            0.5813953488372093,
        ),
    ] {
        let held = Camera::default_for(width, height)
            .zoom
            .base()
            .as_scalar()
            .expect("the zoom is a scalar");
        report.number(
            &format!("FX-CAM-011: a 50 mm lens on a {width}-wide composition, zoom in pixels"),
            zoom,
            held,
        );
        report.number(
            &format!("FX-CAM-011: and its horizontal angle of view, in degrees"),
            angle,
            2.0 * (width as f64 / (2.0 * held)).atan().to_degrees(),
        );
        report.number(
            &format!("FX-CAM-011: and a plane one width back is drawn at, at {width} wide"),
            one_width_back,
            one_width_back_at(width, height, None),
        );
    }

    // The lens D-58 proposed before the After Effects question was asked, kept because the
    // difference between the two is the answer to it.
    report.number(
        "FX-CAM-011: the 36 mm lens' horizontal angle of view, in degrees",
        53.13010235415598,
        2.0 * (1920.0f64 / (2.0 * 1920.0)).atan().to_degrees(),
    );
    report.number(
        "FX-CAM-011: and a plane one width back under it is drawn at",
        0.5,
        one_width_back_at(1920, 1080, Some(1920.0)),
    );
}

/// A plane one composition width behind the camera plane, as this build draws it. `None` is the
/// default lens; a zoom names the 36 mm one.
fn one_width_back_at(width: u32, height: u32, zoom: Option<f64>) -> f64 {
    let mut document = project_of(width, height);
    let l = layer(&mut document, "layer-a", 0);
    if let Some(zoom) = zoom {
        set_camera(&mut document, CameraProp::Depth, Value::Scalar(-zoom));
        set_camera(&mut document, CameraProp::Zoom, Value::Scalar(zoom));
    }
    set(
        &mut document,
        &l,
        Prop::Position,
        Value::Vec2(width as f64 / 2.0, height as f64 / 2.0),
    );
    set_depth(&mut document, &l, width as f64);
    drawn_at(&document, &l, 0)
}

// ---------------------------------------------------------------------------------------
// The artifact
// ---------------------------------------------------------------------------------------

fn write_artifact(report: &Report) {
    let passed = report.rows.iter().filter(|(_, e, a)| e == a).count();
    let mut out = String::from("# The camera and depth against document 25\n\n");
    out.push_str(&format!(
        "Generated by `tests/b13c_camera.rs`. {passed} of {} checks pass.\n\n",
        report.rows.len()
    ));
    out.push_str(
        "D-58 gives a composition one camera and every layer a depth. Every expected value in \
         this table is FX-CAM-001 to 011 in `Markdown/25_Test_Fixture_Catalog.md`, printed by \
         `tools/camera_reference.py`, which walks a point through document 21's four steps one \
         at a time and then does D-58's two lines by hand. It never builds a matrix; this build \
         folds the projection into the layer's transform and multiplies matrices, so the two \
         columns below are two answers and not one answer twice.\n\n",
    );
    out.push_str(
        "Positions are composition pixels to nine places, which is document 25's tolerance. \
         \"Drawn at\" is a size, measured off the screen rather than recomputed: a hundred \
         pixels of the layer, divided by the pixels they cover.\n\n",
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
        "\n## The row worth reading twice\n\n\
         FX-CAM-001 asks whether a camera nobody has touched changes anything, and the answer \
         has to be *nothing at all*, bit for bit. Carrying a point out to the camera and back is \
         two roundings, so a build that applies an identity projection would land within a \
         tolerance of every transform fixture written before D-58 instead of exactly on it. This \
         build leaves the projection out rather than applying it, which is the row above saying \
         the screen point is the composition point bit for bit.\n",
    );
    out.push_str(
        "\n## What this build now says that it could not before\n\n\
         - **`CAMERA_PLANE_BEHIND`.** A layer level with the camera or behind it has no size, is \
         not drawn, and says so. The FX-CAM-006 rows above are that case at four depths, \
         including the one a pixel in front of the camera, which is drawn at 1920 times its size \
         and must not be clamped into an error.\n\
         - **Draw order is a question with a different answer on different frames.** FX-CAM-005 \
         shows depth overriding the layer stack without changing it, and FX-CAM-010 shows the \
         order changing between frame 0 and frame 24 because a depth is keyed. A build that \
         sorted once when a project opened would pass the first and fail the second.\n",
    );
    out.push_str(
        "\n## What this contract does not force, and should be seen\n\n\
         A matte is projected at its own depth before its alpha is sampled, so a matte on a \
         different plane from the layer it shapes slides against it - by 100 pixels in \
         FX-CAM-009, and by more as the camera moves further. This is correct under D-58 and it \
         looks like a defect. A matte is meant to share its layer's depth, and nothing here \
         makes it.\n",
    );
    out.push_str(
        "\n## The gap this table cannot close\n\n\
         Nothing keys a depth or a camera property from the window yet. `SetKeyframe` names one \
         of document 21's five transform properties and D-58 did not add a sixth, so the three \
         animated cases above - FX-CAM-003's tracking camera, FX-CAM-004's zoom and dolly, and \
         FX-CAM-010's animated depth - are built by writing the project out, adding the keys to \
         the file, and reading it back. The build renders them correctly; it is the editing \
         command that is missing, and a project file can carry keys this window cannot yet \
         make.\n",
    );
    out.push_str(
        "\n## What is not in this table\n\n\
         The panel. Setting a depth from the layer's inspector and the camera's own controls are \
         checked in `verification/B-13c_panel_table.md`, and walked by hand in \
         `verification/B-13c_camera_playtest.md`.\n",
    );

    let path = repo("verification/B-13c_camera_table.md");
    fs::write(&path, out).unwrap_or_else(|e| panic!("writing {}: {e}", path.display()));
}
