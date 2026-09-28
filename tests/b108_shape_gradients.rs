//! B-108b: gradient fills and strokes on shape layers, against D-168.
//!
//! Writes `verification/B-108_shape_gradient_table.md`.
//!
//! The expected frames are `Fixtures/shape_gradients/expected_shape_gradients.json`, written by
//! `tools/shape_gradient_reference.py` as B-108a from D-168's rule and not from this build, and
//! printed in document 25 as FX-SHP-040 to 051; FX-SHP-060 to 069 are files a build must refuse
//! whole. Every still case is drawn in `verification/B-108a proposal/gradient_cases.png` and the
//! moving one in `verification/B-108a proposal/gradient_moving_case.png`. Tolerance 1e-6.

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
    repo("Fixtures/shape_gradients")
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
fn b108_shape_gradients() {
    let expected: J = serde_json::from_str(
        &fs::read_to_string(root().join("expected_shape_gradients.json")).unwrap(),
    )
    .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();

    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-108b: gradient fills and strokes on shape layers\n\nD-168, proposed on 2026-09-27. \
         Every expected pixel is `Fixtures/shape_gradients/expected_shape_gradients.json`, written \
         by `tools/shape_gradient_reference.py` as B-108a before this code existed and printed in \
         document 25 as FX-SHP-040 to 051; every still case is drawn in `verification/B-108a \
         proposal/gradient_cases.png` and the moving one in `verification/B-108a \
         proposal/gradient_moving_case.png`. The answer is the largest difference over every \
         sample, against the catalogue's tolerance of 1e-6. FX-SHP-060 to 069 are files the build \
         must refuse whole.\n",
    );

    t.heading("FX-SHP-040 to 051, rendered whole (document 25)");
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

    t.heading("FX-SHP-060 to 069, refused whole (D-168)");
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
    // A fill with keyed points, and a stroke: between them every part of the record.
    for file in ["fx_shp_044.json", "fx_shp_050.json", "fx_shp_051.json"] {
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
    let opened = load("fx_shp_048.json");
    let written: J =
        serde_json::from_str(&persist::to_json(opened.project(), &persist::Preserved::default()))
            .unwrap();
    let fill = &written["compositions"][0]["layers"][0]["shapes"][0]["fill"];
    t.row(
        "stops are saved in the order they were written, blue first, not re-sorted",
        &fill["gradient"]["stops"].to_string(),
        fill["gradient"]["stops"][0]["offset"] == 1.0,
    );
    t.row(
        "and the fill keeps its own flat colour beside the gradient, for when it is taken off",
        &fill["color"].to_string(),
        fill["color"].is_array(),
    );

    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-108_shape_gradient_table.md"), &t.out).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-108_shape_gradient_table.md");
}
