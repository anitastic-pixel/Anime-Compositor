//! B-18b: precompositions in the core, against D-67.
//!
//! Writes `verification/B-18b_precomp_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/precomp/expected_precomp.json`, written by `tools/precomp_reference.py` before
//! this code existed. It renders each six-by-two frame pixel by pixel from document 21, and a
//! composition layer's picture by rendering the inner composition the same way first. Document
//! 25 prints the same numbers as FX-PRE-001 to 015. The tolerance is the catalogue's, 1e-6.
//! Nothing here is a snapshot of a run.
//!
//! # What is deliberately not here
//!
//! The window: the Project panel's button, Pre-compose and Ctrl+Shift+C, the row's mark and
//! double-click to open are B-18c.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::cache::CelCache;
use anime_compositor::command::{precompose, Command, Document};
use anime_compositor::compose::{plan_frame, plan_frame_at, render_frame};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::model::{BlendMode, CameraProp, Id, Layer, LayerKind, Property, Value};
use anime_compositor::persist;
use anime_compositor::preview::{preview_frame, PreviewQuality};
use anime_compositor::render::render_without_culling;
use anime_compositor::trace::{render_traced, TraceRequest};
use anime_compositor::WorkingBuffer;

const MAIN: &str = "comp-main";
const INNER: &str = "comp-inner";

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/precomp")
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

    fn same(&mut self, what: &str, a: &WorkingBuffer, b: &WorkingBuffer) {
        let ok = a.data() == b.data();
        self.row(what, if ok { "byte-identical" } else { "differ" }, ok);
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

fn render_of(document: &Document, comp: &str, frame: i32, tile: usize) -> WorkingBuffer {
    let mut log = FrameLog::new(8);
    render_frame(
        document.project(),
        &Id::new(comp),
        frame,
        &root(),
        tile,
        &mut log,
    )
    .unwrap_or_else(|d| panic!("frame {frame} renders: {}", d.message))
}

fn render(document: &Document, frame: i32, tile: usize) -> WorkingBuffer {
    render_of(document, MAIN, frame, tile)
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

fn names(document: &Document, comp: &str) -> String {
    document
        .project()
        .composition(&Id::new(comp))
        .map(|c| {
            c.layers_in_order()
                .map(|l| format!("{} ({})", l.name, l.kind.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_else(|| "(no such composition)".to_string())
}

#[test]
fn b18b_precomp() {
    let expected: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/precomp/expected_precomp.json")).unwrap(),
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
        "# B-18b: precompositions\n\nD-67, accepted by the owner on 2026-09-18. Every expected \
         pixel is `Fixtures/precomp/expected_precomp.json`, written by \
         `tools/precomp_reference.py` before this code existed and printed in document 25 as \
         FX-PRE-001 to 015. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 1e-6.\n",
    );

    // -----------------------------------------------------------------------------------
    t.heading("FX-PRE-001 to 015 (document 25)");
    for (name, case) in cases {
        let file = case["project"].as_str().unwrap();
        let says = case["says"].as_str().unwrap();
        if let Some(rejected) = case.get("rejected").and_then(J::as_str) {
            let refused = persist::load(&root().join(file))
                .err()
                .map(|d| d.id.as_str());
            t.row(
                &format!("{name}: {says}"),
                &format!("refused to open: {refused:?}"),
                refused == Some(rejected),
            );
            continue;
        }
        let loaded = persist::load(&root().join(file))
            .unwrap_or_else(|d| panic!("{file} opens: {}", d.message));
        for (frame, pixels) in case["frames"].as_object().unwrap() {
            let frame: i32 = frame.parse().unwrap();
            let d = largest_difference(&render(&loaded.document, frame, 64), pixels);
            t.row(
                &format!("{name} frame {frame}: {says}"),
                &format!("largest difference {d:.1e}"),
                d <= tolerance,
            );
        }
        if let Some(warning) = case.get("warning").and_then(J::as_str) {
            let on_open: Vec<&str> = loaded.warnings.iter().map(|d| d.id.as_str()).collect();
            t.row(
                &format!("{name}: opening it says so, once"),
                &format!("{on_open:?}"),
                on_open == [warning],
            );
            let mut log = FrameLog::new(8);
            let _ = plan_frame(
                loaded.document.project(),
                &Id::new(MAIN),
                0,
                &root(),
                &mut log,
            );
            let at_frame: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
            t.row(
                &format!("{name}: and so does the frame, so an export is marked"),
                &format!("{at_frame:?}"),
                at_frame == [warning],
            );
        }
    }

    // -----------------------------------------------------------------------------------
    t.heading("Tiled against untiled: the same frame whatever it is cut into");
    for (name, case) in cases {
        if case.get("rejected").is_some() {
            continue;
        }
        let document = load(case["project"].as_str().unwrap());
        for frame in case["frames"].as_object().unwrap().keys() {
            let frame: i32 = frame.parse().unwrap();
            let whole = render(&document, frame, 64);
            let by_ones = render(&document, frame, 1);
            let by_fours = render(&document, frame, 4);
            let mut log = FrameLog::new(8);
            let plan =
                plan_frame(document.project(), &Id::new(MAIN), frame, &root(), &mut log).unwrap();
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
    t.heading("A composition layer against a drawn layer of the same picture");
    // FX-PRE-005: Main is BG with Inner over it, and Inner is the 2 by 2 drawing and nothing
    // else. A drawn layer of that drawing, placed the same, has to be the same frame.
    for mode in [
        BlendMode::Normal,
        BlendMode::Multiply,
        BlendMode::Screen,
        BlendMode::Add,
    ] {
        let mut nested = load("fx_pre_005.json");
        nested
            .apply(Command::SetBlendMode {
                composition: Id::new(MAIN),
                layer_id: Id::new("pre"),
                mode,
            })
            .expect("a composition layer takes any blend mode");
        let mut drawn = load("fx_pre_005.json");
        let mut layer = Layer::new(Id::new("drawn"), "Drawn", Id::new("asset-small"), 0, 3);
        layer.transform.anchor = Property::constant(Value::Vec2(1.0, 1.0));
        layer.transform.position = Property::constant(Value::Vec2(3.0, 1.0));
        layer.blend_mode = mode;
        drawn
            .apply_all(vec![
                Command::RemoveLayer {
                    composition: Id::new(MAIN),
                    layer_id: Id::new("pre"),
                },
                Command::AddLayer {
                    composition: Id::new(MAIN),
                    layer: Box::new(layer),
                    index: 1,
                },
            ])
            .expect("the drawn twin is built");
        t.same(
            &format!(
                "blend mode {}: the composition layer and the drawn layer",
                mode.as_str()
            ),
            &render(&nested, 0, 64),
            &render(&drawn, 0, 64),
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("Inner on its own against its picture inside Main, with a camera");
    let mut document = load("fx_pre_002.json");
    let without = render(&document, 0, 64);
    document
        .apply(Command::SetCameraProperty {
            composition: Id::new(INNER),
            prop: CameraProp::Position,
            value: Value::Vec2(4.0, 1.0),
        })
        .expect("Inner's camera moves");
    // Main's layer keeps its blur, so it is taken off to leave Inner's picture as it is.
    document
        .apply(Command::RemoveEffect {
            composition: Id::new(MAIN),
            layer_id: Id::new("pre"),
            instance_id: Id::new("fx-0-0"),
        })
        .expect("the blur comes off");
    let alone = render_of(&document, INNER, 0, 64);
    let inside = render(&document, 0, 64);
    t.same(
        "Inner's camera moved one pixel right: Inner rendered alone, and Main showing it",
        &alone,
        &inside,
    );
    t.row(
        "and the camera took part: that frame is not the frame without it",
        if inside.data() != without.data() {
            "different"
        } else {
            "the same"
        },
        inside.data() != without.data(),
    );

    // -----------------------------------------------------------------------------------
    t.heading("What goes wrong inside is said about the frame that was asked for");
    let document = load("fx_pre_006.json");
    let mut log = FrameLog::new(8);
    let nowhere = repo("target/b18b_no_media_here");
    let _ = plan_frame(document.project(), &Id::new(MAIN), 0, &nowhere, &mut log);
    let ids = log.ids_at(0);
    t.row(
        "FX-PRE-006 with its drawings moved away: frame 0 of Main shows frame 1 of Inner, and \
         MEDIA_MISSING is recorded against frame 0",
        &format!("{:?}", ids.iter().map(|i| i.as_str()).collect::<Vec<_>>()),
        ids == [DiagnosticId::MediaMissing],
    );

    // -----------------------------------------------------------------------------------
    t.heading("The file (document 19)");
    let document = load("fx_pre_001.json");
    let pre = document
        .project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layer(&Id::new("pre"))
        .unwrap();
    t.row(
        "fx_pre_001.json's `pre` reads as kind composition, showing comp-inner",
        &format!(
            "{}, showing {:?}",
            pre.kind.as_str(),
            pre.composition_id.as_ref().map(|c| c.as_str())
        ),
        pre.kind == LayerKind::Composition
            && pre.composition_id.as_ref().map(|c| c.as_str()) == Some(INNER),
    );
    let written = persist::to_json(document.project(), &persist::Preserved::default());
    let written_json: J = serde_json::from_str(&written).unwrap();
    let written_pre = &written_json["compositions"][0]["layers"][0];
    let keys: Vec<&str> = [
        "composition_id",
        "source_offset_frames",
        "asset_id",
        "exposure_spans",
    ]
    .into_iter()
    .filter(|k| written_pre.get(k).is_some())
    .collect();
    t.row(
        "written back: composition_id and source_offset_frames, and no asset_id or exposure_spans",
        &format!("kind {}, keys present: {keys:?}", written_pre["kind"]),
        written_pre["kind"] == "composition" && keys == ["composition_id", "source_offset_frames"],
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
    let gone = persist::load(&root().join("fx_pre_014.json")).unwrap();
    let kept = persist::to_json(gone.document.project(), &gone.preserved).contains("comp-gone");
    t.row(
        "FX-PRE-014 written back still names the composition that is not there",
        if kept {
            "comp-gone is kept"
        } else {
            "comp-gone was dropped"
        },
        kept,
    );
    let text = fs::read_to_string(root().join("fx_pre_001.json")).unwrap();
    for (what, edited) in [
        (
            "a composition layer given an asset_id",
            layer_json(&text, "pre", |l| l["asset_id"] = J::from("asset-bg")),
        ),
        (
            "a composition layer given exposure_spans",
            layer_json(&text, "pre", |l| l["exposure_spans"] = J::Array(Vec::new())),
        ),
        (
            "a composition layer with no composition_id",
            layer_json(&text, "pre", |l| {
                l.as_object_mut().unwrap().remove("composition_id");
            }),
        ),
    ] {
        let refused = persist::load_str(&edited).err().map(|d| d.id);
        t.row(
            &format!("{what}: PROJECT_SCHEMA_INVALID"),
            &format!("{refused:?}"),
            refused == Some(DiagnosticId::ProjectSchemaInvalid),
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("Commands (document 24)");
    let mut document = load("fx_pre_005.json");
    let project = document.project().clone();
    let main = project.composition(&Id::new(MAIN)).unwrap();
    let inner = project.composition(&Id::new(INNER)).unwrap();
    let new_layer = Layer::composition(Id::new("pre-2"), "Inner again", inner, 6, 2, 0, 3);
    let centre = (
        new_layer.transform.anchor.value_at(0),
        new_layer.transform.position.value_at(0),
    );
    let added = document
        .apply(Command::AddLayer {
            composition: Id::new(MAIN),
            layer: Box::new(new_layer),
            index: 2,
        })
        .is_ok();
    t.row(
        "layer.add_composition: a 2 by 2 composition into a 6 by 2 one has its anchor at its own \
         centre and its position at the outer centre",
        &format!(
            "added {added}, anchor {:?}, position {:?}",
            centre.0, centre.1
        ),
        added && centre.0 == Value::Vec2(1.0, 1.0) && centre.1 == Value::Vec2(3.0, 1.0),
    );
    let mut ghost = Layer::composition(Id::new("pre-3"), "Ghost", inner, 6, 2, 0, 3);
    ghost.composition_id = Some(Id::new("comp-nowhere"));
    let refused = document
        .apply(Command::AddLayer {
            composition: Id::new(MAIN),
            layer: Box::new(ghost),
            index: 0,
        })
        .err()
        .map(|d| d.id);
    t.row(
        "a composition layer naming a composition that is not there: COMMAND_TARGET_MISSING",
        &format!("{refused:?}"),
        refused == Some(DiagnosticId::CommandTargetMissing),
    );
    for (what, into, shown) in [
        (
            "Main shown inside Inner, which Main already shows",
            INNER,
            main,
        ),
        ("Main shown inside itself", MAIN, main),
    ] {
        let refused = document
            .apply(Command::AddLayer {
                composition: Id::new(into),
                layer: Box::new(Layer::composition(
                    Id::new("loop"),
                    "Loop",
                    shown,
                    6,
                    2,
                    0,
                    3,
                )),
                index: 0,
            })
            .err();
        t.row(
            &format!("{what}: COMPOSITION_CYCLE"),
            &refused.as_ref().map_or("accepted".to_string(), |d| {
                format!("{}: {}", d.id.as_str(), d.message)
            }),
            refused.map(|d| d.id) == Some(DiagnosticId::CompositionCycle),
        );
    }
    let refused = document
        .apply(Command::RemoveComposition {
            composition: Id::new(INNER),
        })
        .err();
    t.row(
        "composition.delete on Inner while Main shows it: refused, naming Main",
        &refused
            .as_ref()
            .map_or("accepted".to_string(), |d| d.message.clone()),
        refused.as_ref().is_some_and(|d| {
            d.id == DiagnosticId::CommandInvalidValue && d.message.contains("Main")
        }),
    );
    let mut freed = load("fx_pre_005.json");
    let deleted = freed
        .apply_all(vec![
            Command::RemoveLayer {
                composition: Id::new(MAIN),
                layer_id: Id::new("pre"),
            },
            Command::RemoveComposition {
                composition: Id::new(INNER),
            },
        ])
        .is_ok();
    t.row(
        "and once that layer is deleted, Inner can be",
        if deleted { "accepted" } else { "refused" },
        deleted,
    );

    // layer.precompose. FX-PRE-012's Main is a matte drawing and a composition layer using it.
    let mut document = load("fx_pre_012.json");
    let before = render(&document, 0, 64);
    let original = document.project().clone();
    let alone = precompose(
        document.project(),
        &Id::new(MAIN),
        &[Id::new("pre")],
        Id::new("comp-precomp-1"),
        Id::new("precomp-1"),
        "Precomp 1",
    )
    .err();
    t.row(
        "layer.precompose on a layer without the matte it uses: COMMAND_INVALID_VALUE, with a sentence",
        &alone
            .as_ref()
            .map_or("accepted".to_string(), |d| d.message.clone()),
        alone.map(|d| d.id) == Some(DiagnosticId::CommandInvalidValue),
    );
    let commands = precompose(
        document.project(),
        &Id::new(MAIN),
        &[Id::new("pre"), Id::new("matte")],
        Id::new("comp-precomp-1"),
        Id::new("precomp-1"),
        "Precomp 1",
    )
    .expect("both together are closed");
    let depth = document.undo_depth();
    document
        .apply_all(commands)
        .expect("the transaction applies");
    t.row(
        "on both together: Main holds one composition layer and Precomp 1 holds the two",
        &format!(
            "Main: {}; Precomp 1: {}",
            names(&document, MAIN),
            names(&document, "comp-precomp-1")
        ),
        names(&document, MAIN) == "Precomp 1 (composition)"
            && document
                .project()
                .composition(&Id::new("comp-precomp-1"))
                .is_some_and(|c| c.len() == 2),
    );
    t.same(
        "the frame is the frame it was",
        &before,
        &render(&document, 0, 64),
    );
    document.undo();
    t.row(
        "it is one undo, and Undo puts the project back exactly",
        &format!(
            "{} undo record(s) added; {}",
            depth + 1 - document.undo_depth(),
            if document.project() == &original {
                "equal"
            } else {
                "different"
            }
        ),
        document.undo_depth() == depth && document.project() == &original,
    );
    // FX-PRE-010's Main is a composition layer with an adjustment layer above it.
    let mut document = load("fx_pre_010.json");
    let before = render(&document, 0, 64);
    let commands = precompose(
        document.project(),
        &Id::new(MAIN),
        &[Id::new("pre")],
        Id::new("comp-precomp-1"),
        Id::new("precomp-1"),
        "Precomp 1",
    )
    .expect("one layer with no ties");
    document.apply_all(commands).unwrap();
    let order = names(&document, MAIN);
    t.row(
        "pre-composing the lower of two layers leaves the new layer in its place, beneath the other",
        &order,
        order.starts_with("Precomp 1 (composition), "),
    );
    t.same(
        "and the frame, now two compositions deep, is the frame it was",
        &before,
        &render(&document, 0, 64),
    );

    // -----------------------------------------------------------------------------------
    t.heading("The draft preview (D-33, D-67)");
    let document = load("fx_pre_001.json");
    let source_size = |quality: PreviewQuality| {
        let mut log = FrameLog::new(8);
        let plan = plan_frame_at(
            document.project(),
            &Id::new(MAIN),
            0,
            &root(),
            quality,
            &mut log,
            &mut CelCache::none(),
        )
        .unwrap();
        (
            plan.layers[0].source.width(),
            plan.layers[0].source.height(),
        )
    };
    t.row(
        "FX-PRE-001's inner picture is rendered 6 by 2 for a full frame and 2 by 1 for a draft",
        &format!(
            "full {:?}, draft {:?}",
            source_size(PreviewQuality::Full),
            source_size(PreviewQuality::Draft)
        ),
        source_size(PreviewQuality::Full) == (6, 2) && source_size(PreviewQuality::Draft) == (2, 1),
    );
    let draft_of = |comp: &str| {
        let mut log = FrameLog::new(8);
        preview_frame(
            document.project(),
            &Id::new(comp),
            0,
            &root(),
            PreviewQuality::Draft,
            64,
            &mut log,
        )
        .unwrap()
    };
    let draft = draft_of(MAIN);
    t.row(
        "the draft frame of Main is 2 by 1",
        &format!("{} by {}", draft.width(), draft.height()),
        (draft.width(), draft.height()) == (2, 1),
    );
    // Main shows Inner whole and unmoved, so a draft of one is a draft of the other.
    t.same(
        "and it is the draft frame of Inner on its own",
        &draft,
        &draft_of(INNER),
    );

    // -----------------------------------------------------------------------------------
    t.heading("The trace (ADR-012)");
    let document = load("fx_pre_006.json");
    let mut log = FrameLog::new(8);
    let plan = plan_frame(document.project(), &Id::new(MAIN), 0, &root(), &mut log).unwrap();
    let dir = repo("target/b18b_trace");
    if dir.exists() {
        fs::remove_dir_all(&dir).unwrap();
    }
    let (traced, _) = render_traced(
        &plan,
        64,
        &TraceRequest {
            dir: dir.clone(),
            frame: 0,
        },
    )
    .expect("trace render");
    t.same(
        "the frame render_traced returns is the frame render returns",
        &traced,
        &render(&document, 0, 64),
    );
    let manifest = fs::read_to_string(dir.join("frame_00000/manifest.md")).unwrap();
    let named: Vec<&str> = manifest
        .lines()
        .filter(|l| l.contains("frame 1 of composition `comp-inner`"))
        .collect();
    t.row(
        "FX-PRE-006 frame 0: the manifest names the layer's decode image as frame 1 of comp-inner, once",
        &format!("{} line(s)", named.len()),
        named.len() == 1 && named[0].contains("decode"),
    );

    // -----------------------------------------------------------------------------------
    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-18b_precomp_table.md"), &t.out).unwrap();
    assert_eq!(
        t.passed, t.checks,
        "see verification/B-18b_precomp_table.md"
    );
}
