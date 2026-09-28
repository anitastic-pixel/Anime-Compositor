//! B-110b: shape styles over time, and line joins and caps, against D-170.
//!
//! Writes `verification/B-110_shape_style_table.md`.
//!
//! The expected frames are `Fixtures/shape_styles/expected_shape_styles.json`, written by
//! `tools/shape_style_reference.py` as B-110a from D-169's rule and not from this build, and
//! printed in document 25 as FX-SHP-100 to 117; FX-SHP-120 to 127 are files a build must refuse
//! whole. Every still case is drawn in `verification/B-110a proposal/style_cases.png` and the
//! moving one in `verification/B-110a proposal/style_moving_cases.png`. Tolerance 1e-6.

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
    repo("Fixtures/shape_styles")
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
fn b110_shape_styles() {
    let expected: J = serde_json::from_str(
        &fs::read_to_string(root().join("expected_shape_styles.json")).unwrap(),
    )
    .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();

    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-110b: shape styles over time, and line joins and caps\n\nD-170, proposed on 2026-09-27. Every \
         expected pixel is `Fixtures/shape_styles/expected_shape_styles.json`, written by \
         `tools/shape_style_reference.py` as B-110a before this code existed and printed in \
         document 25 as FX-SHP-100 to 117; every still case is drawn in `verification/B-110a \
         proposal/style_cases.png` and the moving ones in `verification/B-110a \
         proposal/style_moving_cases.png`. The answer is the largest difference over every sample, \
         against the catalogue's tolerance of 1e-6. FX-SHP-120 to 127 are files the build must \
         refuse whole.\n",
    );

    t.heading("FX-SHP-100 to 117, rendered whole (document 25)");
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

    t.heading("FX-SHP-120 to 127, refused whole (D-170)");
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
    // Joins, caps and a limit; a keyed width; a keyed colour and opacity; a held colour.
    for file in ["fx_shp_104.json", "fx_shp_115.json", "fx_shp_116.json", "fx_shp_117.json"] {
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
    // Put back to round and round with the limit at 4, the three words leave the file, so a
    // stroke nobody changed is written as D-78 wrote it.
    let mut opened = load("fx_shp_104.json");
    let base = fs::read_to_string(root().join("fx_shp_104.json")).unwrap();
    let layer = opened.project().compositions[0].layers_in_order().next().unwrap().clone();
    let mut shapes = layer.shapes.clone();
    let stroke = shapes[0].stroke.as_mut().unwrap();
    stroke.join = anime_compositor::shape::Join::Round;
    stroke.cap = anime_compositor::shape::Cap::Round;
    stroke.miter_limit = 4.0;
    opened
        .apply(anime_compositor::command::Command::SetShapes {
            composition: Id::new(COMP),
            layer_id: layer.id.clone(),
            shapes,
        })
        .unwrap();
    let preserved = persist::load_str(&base).unwrap().preserved;
    let written: J =
        serde_json::from_str(&persist::to_json(opened.project(), &preserved)).unwrap();
    let stroke = &written["compositions"][0]["layers"][0]["shapes"][0]["stroke"];
    let words: Vec<&str> = ["join", "miter_limit", "cap"]
        .into_iter()
        .filter(|k| stroke.get(*k).is_some())
        .collect();
    t.row(
        "a stroke put back to round, round and 4 is saved without the three words",
        &format!("left in the file: {words:?}"),
        words.is_empty(),
    );

    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-110_shape_style_table.md"), &t.out).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-110_shape_style_table.md");
}
