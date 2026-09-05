//! B-08a: assembling and rendering one composition frame from a saved project.
//!
//! Writes `verification/B-08a_compose_table.md` and the frames under
//! `verification/B-08a_frames/`.
//!
//! # What this proves that nothing before it could
//!
//! Every earlier test built a render by hand. This one builds a project of the reference shot
//! through the model's own commands, writes it out as a project file, reads that text back, and
//! renders frames from what came back. Nothing between the file and the picture is test
//! scaffolding.
//!
//! # Where the expected values come from
//!
//! Two sources, both fixtures, neither of them a run of the code under test (ADR-009):
//!
//! - `Fixtures/reference_shot/exposure_sheet.json`, which the fixture README calls "the
//!   authority on timing". Layer 4's irregular exposures are read from it. Layers 1 to 3 follow
//!   the cadence document 22 fixes for them — held, on ones, on twos — and the drawing at a
//!   frame is worked out here rather than looked up.
//! - `Fixtures/reference_shot/generate_cels.py`, the fixture's own record of how the cels were
//!   drawn, which fixes where each drawing puts its marks:
//!
//!   ```text
//!   layer 2, drawing i: a red blob, centre (960 + 520 cos(2*pi*i/24), 540 + 300 sin(...)), r=130
//!   layer 3, drawing i: a yellow bar, x in [160i, 160i+159], y in [180, 419]
//!                       and a cyan bar, x in [1920-160i-160, 1920-160i-1], y in [780, 979]
//!   layer 4, drawing i: a green square at 50% alpha, centre
//!                       (960 + 380 sin(2*pi*i/20), 540 - 120 cos(...)), half-width 200
//!   ```
//!
//! Layer 3 is what most of the table samples, because its edge is binary: a pixel is either the
//! bar or nothing, so "which drawing is on screen" is a fact about alpha rather than a judgement
//! about a soft edge. Every sampled coordinate below is at least forty pixels inside the mark it
//! belongs to, so nothing here depends on how a rasteriser rounds an edge.
//!
//! Colours are checked on the red, blue and alpha channels only. Those three are exact in the
//! working space — 1, 0 and 1 for the yellow bar, whatever the transfer function does to green —
//! and B-02's table is where the sRGB curve is checked to six places.
//!
//! # What is deliberately not here
//!
//! No viewer, no transport, no playback and no work area: that is the rest of B-08 and it needs
//! decisions this build has not been given. No export either; that is T-08.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{plan_frame, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::media::import_sequence;
use anime_compositor::model::{
    Asset, AssetKind, Composition, Id, Interp, Interpretation, Layer, Project, Prop, Value,
};
use anime_compositor::persist::{self, Preserved};
use anime_compositor::render::FramePlan;
use anime_compositor::time::{ExposureMap, ExposureSpan, FrameRate};
use anime_compositor::{trace, AlphaMode, ColorSpace, WorkingBuffer};

const COMP: &str = "comp-reference-shot";

// ---------------------------------------------------------------------------------------
// Reporting
// ---------------------------------------------------------------------------------------

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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/reference_shot")
}

fn id(text: &str) -> Id {
    Id::new(text)
}

/// Red, blue and alpha of one pixel, to six places. Green is left to B-02's table.
fn rba(p: [f32; 4]) -> String {
    format!("R {:.6}, B {:.6}, A {:.6}", p[0], p[2], p[3])
}

fn alpha(p: [f32; 4]) -> String {
    format!("{:.6}", p[3])
}

/// The pixel a plan's `n`th layer holds, in the working space, before any transform.
fn source_pixel(plan: &FramePlan, index: usize, x: usize, y: usize) -> [f32; 4] {
    plan.layers[index].source.pixel(x, y)
}

fn layer_ids(plan: &FramePlan) -> String {
    plan.layers
        .iter()
        .map(|l| l.id.as_str().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

// ---------------------------------------------------------------------------------------
// The project
// ---------------------------------------------------------------------------------------

/// Layer 4's exposures, read from the sheet the fixture README calls the authority on timing.
fn layer4_exposures() -> Vec<(u32, u32)> {
    let text =
        fs::read_to_string(root().join("exposure_sheet.json")).expect("read the exposure sheet");
    let sheet: serde_json::Value = serde_json::from_str(&text).expect("the sheet is JSON");
    let array = |key: &str| -> Vec<u32> {
        sheet
            .get(key)
            .unwrap_or_else(|| panic!("the sheet has no {key}"))
            .as_array()
            .unwrap_or_else(|| panic!("{key} is not an array"))
            .iter()
            .map(|n| n.as_u64().expect("{key} holds a non-integer") as u32)
            .collect()
    };
    let lengths = array("layer4_exposure_lengths");
    let drawings = array("layer4_exposure_drawing_ids");
    drawings.into_iter().zip(lengths).collect()
}

/// Document 22's cadences: layer 1 held for the whole shot, layer 2 on ones over 24 drawings,
/// layer 3 on twos over 12, layer 4 from the sheet.
fn spans(layer: u32) -> Vec<ExposureSpan> {
    let exposures: Vec<(u32, u32)> = match layer {
        1 => vec![(0, 240)],
        2 => (0..240).map(|f| (f % 24, 1)).collect(),
        3 => (0..120).map(|k| (k % 12, 2)).collect(),
        4 => layer4_exposures(),
        _ => unreachable!(),
    };
    ExposureMap::from_lengths(&exposures)
        .expect("the cadences are disjoint and in order")
        .spans()
        .to_vec()
}

/// The asset record an import of one layer's folder produces, with paths relative to the
/// project, which is how document 07 says they are stored.
fn asset_for(layer: u32) -> Asset {
    let dir = root().join(format!("layer{layer}"));
    let files: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .map(|e| e.expect("directory entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "png"))
        .collect();
    let imported = import_sequence(&files)
        .asset
        .unwrap_or_else(|| panic!("layer{layer} imports as a sequence"));
    let frames: BTreeMap<u32, String> = imported
        .frames()
        .iter()
        .map(|(number, path)| {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .expect("a UTF-8 file name");
            (*number, format!("layer{layer}/{name}"))
        })
        .collect();
    Asset {
        id: id(&format!("asset-layer{layer}")),
        kind: AssetKind::ImageSequence,
        name: format!("layer{layer}"),
        path: None,
        pattern: Some(imported.pattern().to_string()),
        frames,
        interpretation: Interpretation::default(),
    }
}

/// The reference shot as a project, built through commands because the model refuses direct
/// mutation, bottom layer first.
fn build_project() -> Project {
    let mut project = Project::new(id("proj-b08a-reference-shot"));
    project.compositions.push(Composition::new(
        id(COMP),
        "reference shot",
        1920,
        1080,
        FrameRate::new(24, 1).expect("24 fps"),
        0,
        240,
    ));
    let mut doc = Document::new(project);
    for n in 1..=4u32 {
        let asset = asset_for(n);
        let mut layer = Layer::new(
            id(&format!("layer-{n}")),
            format!("layer{n}"),
            asset.id.clone(),
            0,
            240,
        );
        layer.exposure_spans = spans(n);
        doc.apply_all(vec![
            Command::AddAsset { asset },
            Command::AddLayer {
                composition: id(COMP),
                layer: Box::new(layer),
                index: (n - 1) as usize,
            },
        ])
        .expect("the opening project is valid");
    }
    doc.project().clone()
}

fn plan_at(project: &Project, frame: i32) -> FramePlan {
    let mut log = FrameLog::new(8);
    plan_frame(project, &id(COMP), frame, &root(), &mut log)
        .unwrap_or_else(|d| panic!("frame {frame} plans: {}", d.message))
}

/// The diagnostics one frame produces, as `ID/subject` lines.
fn diagnostics_at(project: &Project, frame: i32, limit: usize) -> Vec<String> {
    let mut log = FrameLog::new(limit);
    let _ = plan_frame(project, &id(COMP), frame, &root(), &mut log);
    log.finish()
        .into_iter()
        .map(|d| format!("{}: {}", d.id.as_str(), d.message))
        .collect()
}

// ---------------------------------------------------------------------------------------
// The test
// ---------------------------------------------------------------------------------------

#[test]
fn b08a_assembles_and_renders_a_frame_from_a_project() {
    let mut report = Report::default();

    // ---- the project makes the round trip through its own file format ----------------------
    let built = build_project();
    let text = persist::to_json(&built, &Preserved::none());
    let loaded = persist::load_str(&text).expect("the project this build wrote, it can open");
    let project = loaded.document.project().clone();
    fs::write(repo("verification/B-08a_project.json"), &text).expect("write the project artifact");

    let comp = project
        .composition(&id(COMP))
        .expect("the composition survived the round trip");
    report.check(
        "the project read back from its own file has four layers",
        4,
        comp.len(),
    );
    report.check(
        "in composition order, bottom of the stack first",
        "layer1, layer2, layer3, layer4",
        comp.layers_in_order()
            .map(|l| l.name.clone())
            .collect::<Vec<_>>()
            .join(", "),
    );
    report.check(
        "layer 3's asset has no drawing 7, because that file is a deliberate defect of the shot",
        "0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11",
        project
            .assets
            .iter()
            .find(|a| a.id.as_str() == "asset-layer3")
            .expect("layer 3's asset")
            .frames
            .keys()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    );

    // ---- frame 0: every layer draws -------------------------------------------------------
    let f0 = plan_at(&project, 0);
    report.check("frame 0 plans four layers", 4, f0.layers.len());
    report.check(
        "and hands the renderer them bottom first, so layer 4 composites last",
        "layer-1, layer-2, layer-3, layer-4",
        layer_ids(&f0),
    );
    report.check(
        "at the composition's own extent",
        "1920x1080",
        format!("{}x{}", f0.width, f0.height),
    );
    report.check(
        "frame 0 reports nothing wrong",
        "",
        diagnostics_at(&project, 0, 8).join(" / "),
    );

    // Layer 3 at frame 0 is drawing 0 (on twos: 0 / 2 = 0), whose yellow bar occupies
    // x 0..159, y 180..419. (80, 300) is inside it; (400, 300) belongs to drawing 2.
    report.check(
        "layer 3 at frame 0 is drawing 0: its yellow bar is where drawing 0 puts it",
        "R 1.000000, B 0.000000, A 1.000000",
        rba(source_pixel(&f0, 2, 80, 300)),
    );
    report.check(
        "and drawing 2's position is empty, so this is not simply every bar at once",
        "0.000000",
        alpha(source_pixel(&f0, 2, 400, 300)),
    );
    // The cyan bar of drawing 0 runs x 1760..1919, y 780..979.
    report.check(
        "the same drawing's cyan bar is in the bottom right, at full alpha",
        "R 0.000000, B 1.000000, A 1.000000",
        rba(source_pixel(&f0, 2, 1840, 880)),
    );

    // Layer 4 at frame 0 is drawing 0: a square of half-width 200 centred on
    // (960 + 380 sin 0, 540 - 120 cos 0) = (960, 420), painted at 128/255 alpha.
    report.check(
        "layer 4 keeps its exactly-50% interior through the conversion to the working space",
        "0.501961",
        alpha(source_pixel(&f0, 3, 960, 420)),
    );
    report.check(
        "and is transparent outside the square it was painted in",
        "0.000000",
        alpha(source_pixel(&f0, 3, 960, 900)),
    );

    // ---- frame 100: the drawing changes with the frame -------------------------------------
    // On twos, frame 100 is drawing (100 / 2) % 12 = 2, whose bar is x 320..479.
    let f100 = plan_at(&project, 100);
    report.check(
        "layer 3 at frame 100 is drawing 2, one hundred frames of exposure later",
        "R 1.000000, B 0.000000, A 1.000000",
        rba(source_pixel(&f100, 2, 400, 300)),
    );
    report.check(
        "and drawing 0's position is now the empty one",
        "0.000000",
        alpha(source_pixel(&f100, 2, 80, 300)),
    );

    // ---- frame 165: the back-reference reaches the renderer ---------------------------------
    // The sheet's exposures run 3 frames each with a 5 at index 20 and a 1 at index 50, so
    // exposure 55 covers frames 165 to 167, and it re-exposes drawing 11 rather than 15.
    // Drawing 11's square is centred on (960 + 380 sin 198deg, 540 - 120 cos 198deg) =
    // (842.6, 654.1), so it covers (1000, 800). Drawing 15's is centred on (580, 540) and
    // does not: an implementation that assumed drawing numbers only increase reads 0 here.
    let f165 = plan_at(&project, 165);
    report.check(
        "frame 165 re-exposes layer 4's drawing 11, which an earlier exposure already used",
        "0.501961",
        alpha(source_pixel(&f165, 3, 1000, 800)),
    );
    report.check(
        "and it is not drawing 15, which is what counting exposures forward would have given",
        "0.000000",
        alpha(source_pixel(&f165, 3, 500, 400)),
    );

    // ---- frame 13: the Japanese filename ----------------------------------------------------
    // Layer 2 on ones: frame 13 is drawing 13, the one stored as `layer2_桜_013.png`. Its blob
    // is centred on (960 + 520 cos 195deg, 540 + 300 sin 195deg) = (457.7, 462.4).
    let f13 = plan_at(&project, 13);
    report.check(
        "frame 13 opens layer2_\u{685c}_013.png, whose name is not ASCII",
        "1.000000",
        alpha(source_pixel(&f13, 1, 458, 462)),
    );

    // ---- frames 14 and 15: the drawing that is deliberately absent --------------------------
    // On twos, frames 14 and 15 both expose layer 3's drawing 7, which the shot does not have.
    let f14 = plan_at(&project, 14);
    report.check("frame 14 plans three layers, not four", 3, f14.layers.len());
    report.check(
        "and it is layer 3 that is missing, the others untouched",
        "layer-1, layer-2, layer-4",
        layer_ids(&f14),
    );
    report.check(
        "the frame says why, naming the drawing rather than the file",
        "MEDIA_SEQUENCE_GAP: Frame 14 exposes drawing 7 of layer3_%03d.png, which is missing.",
        diagnostics_at(&project, 14, 8).join(" / "),
    );
    report.check(
        "no neighbouring drawing is substituted: layer 3 contributes nothing at all",
        "layer-1, layer-2, layer-4",
        layer_ids(&plan_at(&project, 15)),
    );

    // Document 26's rate limit, first exercised on a real shot: the same complaint about the
    // same layer at two frames is one full report and one summary, not two identical lines.
    let mut log = FrameLog::new(1);
    for frame in [14, 15] {
        let _ = plan_frame(&project, &id(COMP), frame, &root(), &mut log);
    }
    let limited = log.finish();
    report.check(
        "over frames 14 and 15 with a limit of one, the second is summarised, not repeated",
        2,
        limited.len(),
    );
    report.check(
        "and the summary names both frames and how many were held back",
        "Frames 14 to 15. 1 further identical warnings were not logged individually.",
        limited
            .last()
            .map(|d| d.detail.clone())
            .unwrap_or_else(|| "there was no summary".to_string()),
    );

    // ---- document 20 step 6: the properties are evaluated at the frame ----------------------
    let mut doc = Document::new(project.clone());
    doc.apply_all(vec![
        Command::SetKeyframe {
            composition: id(COMP),
            layer_id: id("layer-2"),
            prop: Prop::Opacity,
            frame: 0,
            value: Value::Scalar(0.0),
            interp: Interp::Linear,
        },
        Command::SetKeyframe {
            composition: id(COMP),
            layer_id: id("layer-2"),
            prop: Prop::Opacity,
            frame: 24,
            value: Value::Scalar(1.0),
            interp: Interp::Linear,
        },
        Command::SetPropertyBase {
            composition: id(COMP),
            layer_id: id("layer-3"),
            prop: Prop::Anchor,
            value: Value::Vec2(100.0, 50.0),
        },
        Command::SetPropertyBase {
            composition: id(COMP),
            layer_id: id("layer-3"),
            prop: Prop::Position,
            value: Value::Vec2(960.0, 540.0),
        },
        Command::SetPropertyBase {
            composition: id(COMP),
            layer_id: id("layer-3"),
            prop: Prop::Scale,
            value: Value::Vec2(2.0, 2.0),
        },
    ])
    .expect("the animated variant is valid");
    let animated = doc.project().clone();

    let opacity_at = |frame: i32| format!("{:.6}", plan_at(&animated, frame).layers[1].opacity);
    report.check(
        "a layer's opacity is read at the frame, not at zero: frame 0",
        "0.000000",
        opacity_at(0),
    );
    report.check(
        "halfway between the two keyframes, frame 12",
        "0.500000",
        opacity_at(12),
    );
    report.check(
        "and at the second keyframe, frame 24",
        "1.000000",
        opacity_at(24),
    );

    // Anchor (100, 50), position (960, 540), scale 2. The anchor lands on the position, and a
    // point 100 to the left of the anchor lands 200 to the left of it. Swapping anchor and
    // position, which is the easy mistake, sends the origin to (-1820, -1030) instead.
    let t = plan_at(&animated, 0).layers[2].transform;
    report.check(
        "the anchor point lands on the layer's position",
        "(960.0, 540.0)",
        format!("{:?}", t.apply(100.0, 50.0)),
    );
    report.check(
        "and the layer's origin lands anchor-distance away, scaled",
        "(760.0, 440.0)",
        format!("{:?}", t.apply(0.0, 0.0)),
    );

    // ---- the asset record decides how its pixels are read ------------------------------------
    //
    // `decode_png` tags what a PNG guarantees: sRGB, straight. The asset record is what says
    // otherwise, and if it is ignored a sequence rendered in linear light goes through the
    // transfer function a second time and comes out dark. Layer 3's yellow bar is (255, 220, 0)
    // in the file, so its green channel is 220/255 = 0.862745 before any conversion.
    let mut as_linear = project.clone();
    for asset in &mut as_linear.assets {
        if asset.id.as_str() == "asset-layer3" {
            asset.interpretation.color_space = ColorSpace::LinearLight;
        }
    }
    let green = |plan: &FramePlan| source_pixel(plan, 2, 80, 300)[1] as f64;
    let linear_tagged = green(&plan_at(&as_linear, 0));
    report.check(
        "an asset recorded as already linear is not put through the transfer function again",
        "0.862745",
        format!("{linear_tagged:.6}"),
    );
    report.check(
        "and the same drawing under the default sRGB tag is darker, because it is converted",
        "darker",
        match green(&f0) {
            g if g < linear_tagged => "darker",
            g if g > linear_tagged => "brighter",
            _ => "the same, so the tag changed nothing",
        },
    );

    let mut as_premultiplied = project.clone();
    for asset in &mut as_premultiplied.assets {
        if asset.id.as_str() == "asset-layer4" {
            asset.interpretation.alpha = AlphaMode::Premultiplied;
        }
    }
    // Layer 4's interior is (40, 255, 120) at 128/255 alpha. Read as straight it is multiplied
    // by that alpha on the way into the working space; read as premultiplied it is not, so it
    // stays brighter. Alpha itself is unchanged either way, which is why this row reads red.
    let red = |plan: &FramePlan| source_pixel(plan, 3, 960, 420)[0] as f64;
    report.check(
        "an asset recorded as premultiplied is not multiplied by its alpha a second time",
        "brighter",
        match red(&plan_at(&as_premultiplied, 0)) {
            r if r > red(&f0) => "brighter",
            r if r < red(&f0) => "darker",
            _ => "the same, so the tag changed nothing",
        },
    );

    // ---- what the caller got wrong, as opposed to the media ---------------------------------
    let mut log = FrameLog::new(8);
    report.check(
        "a frame past the end of the composition is refused, not clamped to the last one",
        "COMMAND_INVALID_VALUE",
        match plan_frame(&project, &id(COMP), 240, &root(), &mut log) {
            Err(d) => d.id.as_str().to_string(),
            Ok(_) => "the render was attempted anyway".to_string(),
        },
    );
    report.check(
        "and one before the start likewise",
        "COMMAND_INVALID_VALUE",
        match plan_frame(&project, &id(COMP), -1, &root(), &mut log) {
            Err(d) => d.id.as_str().to_string(),
            Ok(_) => "the render was attempted anyway".to_string(),
        },
    );
    report.check(
        "frame 239 is inside the shot and renders, so the refusal is not off by one",
        4,
        plan_at(&project, 239).layers.len(),
    );
    report.check(
        "a composition the project does not have is refused",
        "COMMAND_TARGET_MISSING",
        match plan_frame(&project, &id("comp-nope"), 0, &root(), &mut log) {
            Err(d) => d.id.as_str().to_string(),
            Ok(_) => "a frame was planned from nothing".to_string(),
        },
    );

    // ---- a hidden layer, a matte, a broken reference ----------------------------------------
    let mut doc = Document::new(project.clone());
    doc.apply(Command::SetLayerEnabled {
        composition: id(COMP),
        layer_id: id("layer-2"),
        value: false,
    })
    .expect("hiding a layer is valid");
    let hidden = doc.project().clone();
    report.check(
        "a layer switched off is left out of the frame",
        "layer-1, layer-3, layer-4",
        layer_ids(&plan_at(&hidden, 0)),
    );
    report.check(
        "and switching it off is not a fault, so nothing is reported",
        "",
        diagnostics_at(&hidden, 0, 8).join(" / "),
    );

    let mut doc = Document::new(project.clone());
    doc.apply(Command::SetMatte {
        composition: id(COMP),
        layer_id: id("layer-4"),
        matte: Some(id("layer-3")),
    })
    .expect("a matte naming a layer that exists is a valid command");
    let with_matte = doc.project().clone();
    report.check(
        "a track matte, which this build does not render, is reported rather than ignored",
        "PROJECT_FEATURE_UNSUPPORTED: Layer layer4 has a track matte, which this build does not render.",
        diagnostics_at(&with_matte, 0, 8).join(" / "),
    );
    report.check(
        "the layer still draws; what it loses is the matte, not itself",
        4,
        plan_at(&with_matte, 0).layers.len(),
    );

    // The layer keeps its reference and the asset record goes: document 28's case of a layer
    // pointing at something the project no longer holds.
    let mut orphan = project.clone();
    orphan.assets.retain(|a| a.id.as_str() != "asset-layer3");
    report.check(
        "a layer naming an asset the project does not have is reported and left out",
        "PROJECT_SCHEMA_INVALID: Layer layer3 names asset asset-layer3, which is not in the project.",
        diagnostics_at(&orphan, 0, 8).join(" / "),
    );

    let mut moved = project.clone();
    for asset in &mut moved.assets {
        if asset.id.as_str() == "asset-layer3" {
            asset
                .frames
                .insert(0, "layer3/layer3_000_moved_away.png".to_string());
        }
    }
    report.check(
        "a file the project points at that is not on disk is a different fault, named differently",
        "MEDIA_MISSING: layer3/layer3_000_moved_away.png is not where the project says it is.",
        diagnostics_at(&moved, 0, 8).join(" / "),
    );

    // ---- document 20 step 8: the composite ---------------------------------------------------
    let mut log = FrameLog::new(8);
    let frame0 = render_frame(&project, &id(COMP), 0, &root(), DEFAULT_TILE_SIZE, &mut log)
        .expect("frame 0 renders");
    report.check(
        "the rendered frame is the composition's size",
        "1920x1080",
        format!("{}x{}", frame0.width(), frame0.height()),
    );
    // Layer 3's yellow bar is opaque and sits above layer 1; layer 4's square is x 760..1160 and
    // layer 2's blob is off at (1480, 540), so neither reaches (80, 300).
    report.check(
        "where layer 3 is opaque, the composite is layer 3, because it is above layer 1",
        "R 1.000000, B 0.000000, A 1.000000",
        rba(frame0.pixel(80, 300)),
    );
    let smaller_tiles = render_frame(&project, &id(COMP), 0, &root(), 64, &mut log)
        .expect("frame 0 renders at another tile size");
    report.check(
        "tile size is a speed setting: 64 and 128 give the same frame, byte for byte",
        "identical",
        if smaller_tiles.to_srgb8_straight() == frame0.to_srgb8_straight() {
            "identical".to_string()
        } else {
            "the two tile sizes disagreed".to_string()
        },
    );

    // ---- the frames the owner looks at --------------------------------------------------------
    let dir = repo("verification/B-08a_frames");
    fs::create_dir_all(&dir).expect("make the frame folder");
    for (frame, note) in [
        (0, "every layer draws"),
        (
            14,
            "layer 3 exposes drawing 7, which the shot does not have",
        ),
        (100, "layer 3 has advanced to drawing 2"),
        (165, "layer 4 re-exposes drawing 11"),
    ] {
        let mut log = FrameLog::new(8);
        let rendered = render_frame(
            &project,
            &id(COMP),
            frame,
            &root(),
            DEFAULT_TILE_SIZE,
            &mut log,
        )
        .unwrap_or_else(|d| panic!("frame {frame} renders: {}", d.message));
        write_frame(&dir, frame, note, &rendered);
    }
    report.check(
        "four frames were written for the owner to look at",
        4,
        fs::read_dir(&dir).expect("read the frame folder").count(),
    );

    write_report(&report);
    let failed: Vec<&Row> = report.rows.iter().filter(|r| !r.pass()).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed, first: {} (expected {}, got {})",
        failed.len(),
        report.rows.len(),
        failed[0].check,
        failed[0].expected,
        failed[0].actual
    );
}

fn write_frame(dir: &Path, frame: i32, note: &str, buffer: &WorkingBuffer) {
    trace::write_png(
        &dir.join(format!("frame_{frame:03}.png")),
        buffer,
        &[
            ("Composition", "reference shot".to_string()),
            ("Frame", frame.to_string()),
            ("Note", note.to_string()),
            (
                "Source",
                "assembled from verification/B-08a_project.json by tests/b08a_compose.rs"
                    .to_string(),
            ),
        ],
    )
    .expect("write the frame");
}

fn write_report(report: &Report) {
    let passed = report.rows.iter().filter(|r| r.pass()).count();
    let mut out = String::new();
    out.push_str(&format!(
        "# B-08a — a frame assembled and rendered from a project file\n\n\
         **{passed} of {} checks passed.**\n\n\
         Produced by `tests/b08a_compose.rs`. Covers document 20's evaluation order at one \
         frame, steps 1 to 8.\n\n\
         ## What this is\n\n\
         Until now every picture this build produced was assembled by a test. This one is \
         assembled from a project. The reference shot is built through the application's own \
         commands, written out as `verification/B-08a_project.json`, read back from that text, \
         and rendered. The four frames in `verification/B-08a_frames/` came out of the end of \
         that chain.\n\n\
         ## What to look at\n\n\
         - **`B-08a_frames/frame_000.png`** — the shot at frame 0, all four layers. The yellow \
         bar is at the far left and the cyan bar at the bottom right; that is drawing 0 of \
         layer 3.\n\
         - **`B-08a_frames/frame_100.png`** — the same shot a hundred frames later. The yellow \
         bar has moved right by two bar-widths and the green square has moved. If these two \
         images were the same, the exposure sheet would not be reaching the renderer.\n\
         - **`B-08a_frames/frame_014.png`** — layer 3 is absent here, and that is correct. \
         Frame 14 asks for drawing 7, which the fixture deliberately does not contain. The \
         bars are gone and nothing has been substituted for them. A build that quietly held \
         drawing 6 for two more frames would look better and be wrong.\n\
         - **`B-08a_frames/frame_165.png`** — layer 4 re-exposes drawing 11 here, after \
         drawing 14. Cel work does this constantly; an implementation that assumed drawing \
         numbers only go up would show drawing 15 and this frame would be in the wrong place.\n\n\
         ## What a wrong result looks like\n\n\
         The table's expected values were worked out from the fixture's own record of how the \
         cels were drawn — where each drawing puts its bar, its blob and its square — before \
         the code was run. A layer resolving to the wrong drawing shows up as a bar of alpha \
         where the table expects nothing, which is why almost every row samples two points: \
         one where the mark should be and one where the previous or next drawing's mark would \
         be.\n\n\
         ## What is not here\n\n\
         There is no viewer, no playback and no export. This is the headless half of B-08: it \
         turns a project and a frame number into a picture, and stops there. Masks, effects \
         and track mattes are parked (document 23); a layer carrying a matte renders without \
         it and says so rather than pretending.\n\n\
         ## Checks\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        report.rows.len(),
    ));
    for r in &report.rows {
        out.push_str(&format!(
            "| {} | `{}` | `{}` | {} |\n",
            r.check,
            r.expected,
            r.actual,
            if r.pass() { "pass" } else { "**FAIL**" }
        ));
    }
    fs::write(repo("verification/B-08a_compose_table.md"), out).expect("write report");
}
