//! B-930: dashed strokes on shape layers, against D-1050.
//!
//! Writes `verification/B-930_shape_dash_table.md`.
//!
//! The expected frames are `Fixtures/shape_dashes/expected_shape_dashes.json`, written by
//! `tools/shape_dash_reference.py` with D-1050 from its rule and not from this build, and
//! printed in document 25 as FX-SHP-130 to 140; FX-SHP-141 to 146 are files a build must refuse
//! whole. Every still case is drawn in `verification/D-1050 pictures/dash_cases.png` and the
//! moving one in `verification/D-1050 pictures/dash_moving_case.png`. Tolerance 1e-6.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::WorkingBuffer;

const COMP: &str = "comp-main";

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/shape_dashes")
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

#[test]
fn b930_shape_dashes() {
    let expected: J = serde_json::from_str(
        &fs::read_to_string(root().join("expected_shape_dashes.json")).unwrap(),
    )
    .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();

    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-930: dashed strokes on shape layers\n\nD-1050, accepted on 2026-10-27. Every \
         expected pixel is `Fixtures/shape_dashes/expected_shape_dashes.json`, written by \
         `tools/shape_dash_reference.py` before this code existed and printed in \
         document 25 as FX-SHP-130 to 140; every still case is drawn in `verification/D-1050 \
         pictures/dash_cases.png` and the moving one in `verification/D-1050 \
         pictures/dash_moving_case.png`. The answer is the largest difference over every sample, \
         against the catalogue's tolerance of 1e-6. FX-SHP-141 to 146 are files the build must \
         refuse whole.\n",
    );

    t.heading("FX-SHP-130 to 140, rendered whole (document 25)");
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
            let tiled = render(&document, frame, 1).0.data() == built.data();
            t.row(
                &format!("{name} frame {frame}: {}", case["says"].as_str().unwrap()),
                &format!(
                    "largest difference {d:.1e}; tiles of 1 {}; said {}",
                    if tiled { "byte-identical" } else { "differ" },
                    if ids.is_empty() {
                        "nothing".to_string()
                    } else {
                        format!("{ids:?}")
                    }
                ),
                d <= tolerance && tiled && ids.is_empty(),
            );
        }
    }

    t.heading("FX-SHP-141 to 146, refused whole (D-1050)");
    let mut refused: Vec<(&String, &J)> = expected["refused"].as_object().unwrap().iter().collect();
    refused.sort_by_key(|(name, _)| name.as_str());
    for (name, case) in refused {
        let got = persist::load(&root().join(case["project"].as_str().unwrap())).err();
        t.row(
            &format!("{name}: {}", case["says"].as_str().unwrap()),
            &got.as_ref().map_or("opened".to_string(), |d| {
                format!("{:?}: {}", d.id, d.detail)
            }),
            got.map(|d| d.id) == Some(DiagnosticId::ProjectSchemaInvalid),
        );
    }

    t.heading("The file (document 19)");
    // A keyed offset, and dashes on a trimmed line: between them every part of the record.
    for file in ["fx_shp_138.json", "fx_shp_140.json"] {
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
    // Taken off, the dashes leave the file, rather than the old ones being kept from the base.
    let mut opened = load("fx_shp_131.json");
    let base = fs::read_to_string(root().join("fx_shp_131.json")).unwrap();
    let mut shapes = opened.project().compositions[0]
        .layers_in_order()
        .next()
        .unwrap()
        .shapes
        .clone();
    shapes[0].dashes = None;
    let layer_id = opened.project().compositions[0]
        .layers_in_order()
        .next()
        .unwrap()
        .id
        .clone();
    opened
        .apply(anime_compositor::command::Command::SetShapes {
            composition: Id::new(COMP),
            layer_id,
            shapes,
            groups: None,
        })
        .unwrap();
    let preserved = persist::load_str(&base).unwrap().preserved;
    let written: J = serde_json::from_str(&persist::to_json(opened.project(), &preserved)).unwrap();
    let shape = &written["compositions"][0]["layers"][0]["shapes"][0];
    t.row(
        "dashes taken off is gone from the saved shape",
        &format!(
            "dashes {}",
            shape.get("dashes").map_or("absent".to_string(), J::to_string)
        ),
        shape.get("dashes").is_none(),
    );

    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-930_shape_dash_table.md"), &t.out).unwrap();
    assert_eq!(
        t.passed, t.checks,
        "see verification/B-930_shape_dash_table.md"
    );
}
