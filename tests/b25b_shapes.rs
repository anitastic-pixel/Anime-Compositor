//! B-25b: shape layers in the core, against D-78.
//!
//! Writes `verification/B-25b_shape_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/shapes/expected_shapes.json`, written by `tools/shape_reference.py` as B-25a, before
//! any of `src/shape.rs` existed — a second implementation, written from D-78 rather than from
//! this build. Document 25 prints the same numbers as FX-SHP-001 to 017; FX-SHP-020 to 030 are
//! files a build must refuse whole. The tolerance is the catalogue's, 1e-6. Nothing here is a
//! snapshot of a run.
//!
//! Every still case is drawn in `verification/B-25a proposal/shape_cases.png` and the moving one,
//! frame by frame, in `verification/B-25a proposal/shape_moving_case.png`, so a reader who does
//! not read code can check the same seventeen pictures the table checks.
//!
//! # What is deliberately not here
//!
//! The window — a shape layer made from a menu, the tools that draw one, the panel rows for a
//! fill and a stroke — is B-25c, and so are the commands, which live beside the mask commands in
//! `app/src/main.rs`. This table is the core: the model, the rasterizer, step 1, the file and the
//! diagnostic.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::mask::MaskPoint;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::shape::{Fill, Shape, Stroke};
use anime_compositor::WorkingBuffer;

const COMP: &str = "comp-main";

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/shapes")
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

fn load(file: &str) -> anime_compositor::command::Document {
    persist::load(&root().join(file))
        .unwrap_or_else(|d| panic!("{file} opens: {} {}", d.message, d.detail))
        .document
}

fn render(
    document: &anime_compositor::command::Document,
    frame: i32,
    tile: usize,
) -> (WorkingBuffer, Vec<DiagnosticId>) {
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

fn shapes_of(document: &anime_compositor::command::Document, layer: &str) -> Vec<Shape> {
    document
        .project()
        .composition(&Id::new(COMP))
        .unwrap()
        .layer(&Id::new(layer))
        .unwrap()
        .shapes
        .clone()
}

#[test]
fn b25b_shapes() {
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root().join("expected_shapes.json")).unwrap())
            .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();

    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-25b: shape layers in the core\n\nD-78, accepted by the owner on 2026-09-20. Every \
         expected pixel is `Fixtures/shapes/expected_shapes.json`, written by \
         `tools/shape_reference.py` as B-25a before this code existed and printed in document 25 \
         as FX-SHP-001 to 017; every still case is drawn in `verification/B-25a \
         proposal/shape_cases.png` and the moving one in `verification/B-25a \
         proposal/shape_moving_case.png`. The build's frame is compared sample by sample; the \
         answer is the largest difference over all of them, against the catalogue's tolerance of \
         1e-6. FX-SHP-020 to 030 are files the build must refuse whole.\n\nThe window and the \
         commands are B-25c and are not in this table.\n",
    );

    // -----------------------------------------------------------------------------------
    t.heading("FX-SHP-001 to 017, rendered whole (document 25)");
    let mut cases: Vec<(&String, &J)> = expected["cases"].as_object().unwrap().iter().collect();
    cases.sort_by_key(|(name, _)| name.as_str());
    for (name, case) in cases {
        let document = load(case["project"].as_str().unwrap());
        let mut frames: Vec<(&String, &J)> = case["frames"].as_object().unwrap().iter().collect();
        frames.sort_by_key(|(f, _)| f.parse::<i32>().unwrap());
        for (frame, pixels) in frames {
            let frame: i32 = frame.parse().unwrap();
            let (built, ids) = render(&document, frame, 64);
            let d = largest_difference(&built, pixels);
            // The tile size must not change one bit of the answer: a shape is drawn into the
            // composition's own space, and a picture cut into tiles is the same picture.
            let tiled = render(&document, frame, 1).0.data() == built.data();
            // A case with a diagnostic must raise it on every frame, and one without must raise
            // none: a shape quietly left out is exactly what document 28 forbids.
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
    t.heading("FX-SHP-020 to 030, refused whole (D-78)");
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
    // The curved one and the keyed one together carry every part of the record: handles, an open
    // flag, a fill, a stroke and a path that moves.
    for file in ["fx_shp_004.json", "fx_shp_011.json", "fx_shp_017.json"] {
        let opened = load(file);
        let written = persist::to_json(opened.project(), &persist::Preserved::default());
        let again = persist::load_str(&written)
            .unwrap_or_else(|d| panic!("what this build wrote, it opens: {}", d.message));
        t.row(
            &format!("{file} survives a save and a load, to the last bit"),
            if again.document.project() == opened.project() {
                "equal"
            } else {
                "different"
            },
            again.document.project() == opened.project(),
        );
    }
    let opened = load("fx_shp_011.json");
    let written: J =
        serde_json::from_str(&persist::to_json(opened.project(), &persist::Preserved::default()))
            .unwrap();
    let layer = &written["compositions"][0]["layers"][0];
    t.row(
        "a saved shape layer says `shape` and carries a `shapes` list",
        &format!("kind {}, shapes {}", layer["kind"], layer["shapes"].is_array()),
        layer["kind"] == "shape" && layer["shapes"].is_array(),
    );
    t.row(
        "and carries no asset, no exposures and no source offset, having no footage behind it",
        &format!(
            "asset_id {}, exposure_spans {}, source_offset_frames {}",
            layer.get("asset_id").map_or("absent", |_| "present"),
            layer.get("exposure_spans").map_or("absent", |_| "present"),
            layer
                .get("source_offset_frames")
                .map_or("absent", |_| "present"),
        ),
        layer.get("asset_id").is_none()
            && layer.get("exposure_spans").is_none()
            && layer.get("source_offset_frames").is_none(),
    );
    t.row(
        "and its shape holds its path as a base of points with both handles, as a mask's does",
        &layer["shapes"][0]["path"]["base"]["points"][0].to_string(),
        layer["shapes"][0]["path"]["base"]["points"][0]
            .get("in")
            .is_some()
            && layer["shapes"][0]["path"]["base"]["points"][0]
                .get("out")
                .is_some(),
    );
    t.row(
        "and its `closed` is written out, so an open path stays open",
        &layer["shapes"][0]["closed"].to_string(),
        layer["shapes"][0]["closed"] == false,
    );
    let keyed = shapes_of(&load("fx_shp_017.json"), "shapes");
    t.row(
        "fx_shp_017.json's keyed path reads back as two keys on the shape itself",
        &format!(
            "{} keys, at frames {:?}",
            keyed[0].keys.len(),
            keyed[0].keys.iter().map(|k| k.frame).collect::<Vec<_>>()
        ),
        keyed[0].keys.len() == 2 && keyed[0].keys.iter().map(|k| k.frame).eq([0, 4]),
    );

    // -----------------------------------------------------------------------------------
    t.heading("The rule itself (D-78)");
    let square = |closed: bool| Shape {
        closed,
        ..Shape::filled(
            "s",
            vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)],
            Fill {
                color: [1.0, 1.0, 1.0],
                opacity: 1.0,
            },
        )
    };
    // The one difference open makes: the same four corners either way, and the segment back to
    // the start there or not. A stroke is where that shows -- a fill closes an open path anyway.
    let open = square(false);
    let closed = square(true);
    t.row(
        "a path of four corners hands the sampler its four vertices whether it is closed or open",
        &format!(
            "closed {:?}, open {:?}",
            closed.outline().len(),
            open.outline().len()
        ),
        closed.outline() == vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]
            && open.outline() == closed.outline(),
    );
    let on_the_left_edge = |closed: bool| {
        anime_compositor::mask::distance_to_path(&open.outline(), closed, 0.0, 2.0)
    };
    t.row(
        "and closing it is what puts the fourth side there: the left edge is on the path of a \
         closed square and two pixels off an open one",
        &format!(
            "closed {:.3}, open {:.3}",
            on_the_left_edge(true),
            on_the_left_edge(false)
        ),
        on_the_left_edge(true) == 0.0 && (on_the_left_edge(false) - 2.0).abs() < 1e-12,
    );
    // The stroke's definition, which is the choice D-78 asked the owner to look at: every point
    // within half the width of the path, so a cap is a half circle and nothing enumerates it.
    let line = Shape {
        closed: false,
        points: vec![MaskPoint::corner(1.0, 1.0), MaskPoint::corner(3.0, 1.0)],
        stroke: Some(Stroke {
            color: [1.0, 1.0, 1.0],
            opacity: 1.0,
            width_px: 2.0,
        }),
        ..Shape::default()
    };
    let poly = line.outline();
    let d = |x: f64, y: f64| anime_compositor::mask::distance_to_path(&poly, false, x, y);
    t.row(
        "a stroke two pixels wide reaches exactly one pixel past the end of an open path, in a circle",
        &format!(
            "at the end {:.3}, one right of it {:.3}, one diagonal from it {:.3}",
            d(3.0, 1.0),
            d(4.0, 1.0),
            d(3.0 + 0.6, 1.0 + 0.8)
        ),
        (d(4.0, 1.0) - 1.0).abs() < 1e-12 && (d(3.6, 1.8) - 1.0).abs() < 1e-12,
    );
    // Where a shape and a mask part company, in one line: the same bowtie, two answers.
    let bowtie = Shape::filled(
        "bowtie",
        vec![(0.0, 0.0), (4.0, 0.0), (0.0, 4.0), (4.0, 4.0)],
        Fill {
            color: [1.0, 1.0, 1.0],
            opacity: 1.0,
        },
    );
    let mask_says = anime_compositor::mask::is_simple(&[
        (0.0, 0.0),
        (4.0, 0.0),
        (0.0, 4.0),
        (4.0, 4.0),
    ]);
    t.row(
        "a path that crosses itself is drawn as a shape though the same outline is refused as a mask",
        &format!(
            "shape draws: {}, mask simple: {mask_says}",
            bowtie.is_renderable()
        ),
        bowtie.is_renderable() && !mask_says,
    );
    // The one thing that *is* diagnosed, and the one thing that is not.
    let lone = Shape {
        points: vec![MaskPoint::corner(1.0, 1.0)],
        ..bowtie.clone()
    };
    t.row(
        "a shape of one point draws nothing and is the SHAPE_INVALID_OUTLINE case",
        &format!(
            "renderable {}, catalogued {}",
            lone.is_renderable(),
            DiagnosticId::ShapeInvalidOutline.in_catalog()
        ),
        !lone.is_renderable()
            && DiagnosticId::ShapeInvalidOutline.as_str() == "SHAPE_INVALID_OUTLINE"
            && DiagnosticId::ShapeInvalidOutline.in_catalog(),
    );
    let undecided = Shape {
        fill: None,
        ..bowtie.clone()
    };
    t.row(
        "a shape with neither a fill nor a stroke draws nothing and says nothing: not a fault",
        &format!("problem {:?}", undecided.problem()),
        undecided.problem().is_none() && undecided.is_renderable(),
    );
    // D-78's ranges, each in the same sentence the loader refuses with.
    for (what, bad) in [
        (
            "a fill colour above 1",
            Shape::filled(
                "s",
                vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)],
                Fill {
                    color: [1.5, 0.0, 0.0],
                    opacity: 1.0,
                },
            ),
        ),
        (
            "a fill opacity below 0",
            Shape::filled(
                "s",
                vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)],
                Fill {
                    color: [1.0, 0.0, 0.0],
                    opacity: -0.5,
                },
            ),
        ),
        (
            "a stroke width of 0",
            Shape {
                stroke: Some(Stroke {
                    color: [1.0, 0.0, 0.0],
                    opacity: 1.0,
                    width_px: 0.0,
                }),
                ..bowtie.clone()
            },
        ),
        (
            "a stroke width past 8192",
            Shape {
                stroke: Some(Stroke {
                    color: [1.0, 0.0, 0.0],
                    opacity: 1.0,
                    width_px: 8193.0,
                }),
                ..bowtie.clone()
            },
        ),
    ] {
        let said = bad.problem();
        t.row(
            &format!("{what} is a problem the build can say in a sentence"),
            &format!("{said:?}"),
            said.is_some(),
        );
    }
    // A shape layer has no size of its own: its space is the composition's, which is what makes
    // a shape's coordinates and the frame's the same numbers.
    let layer = anime_compositor::model::Layer::shape(
        Id::new("s"),
        "shapes",
        vec![closed.clone()],
        6,
        2,
        0,
        4,
    );
    let drawn = anime_compositor::shape::draw(&layer.shapes, 6, 2);
    t.row(
        "a shape layer's picture is the composition's size, and it starts transparent black",
        &format!(
            "{} by {}; the far corner {:?}",
            drawn.width(),
            drawn.height(),
            &drawn.data()[drawn.data().len() - 4..]
        ),
        drawn.width() == 6 && drawn.height() == 2 && drawn.data()[drawn.data().len() - 1] == 0.0,
    );

    // -----------------------------------------------------------------------------------
    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-25b_shape_table.md"), &t.out).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-25b_shape_table.md");
}
