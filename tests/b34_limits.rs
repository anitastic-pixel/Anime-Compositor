//! B-34: the limits of the blur and exposure, against D-90.
//!
//! Writes `verification/B-34_limits_table.md`.
//!
//! Every expected pixel is `Fixtures/limits/expected_limits.json`, written by
//! `tools/limits_reference.py` before this code existed and printed in document 25 as
//! FX-LIMIT-001 to 010; D-318's wider exposure is `expected_limits_d318.json`, FX-LIMIT-011 to
//! 016, and supersedes 004, 006 and 007. The tolerance is relative, 1e-4 of the expected value's own size, and
//! exact where the expected value is 0. Nothing here is a snapshot of a run.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{plan_frame, render_frame};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::WorkingBuffer;

const MAIN: &str = "comp-main";

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/limits")
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

fn render(document: &Document, frame: i32) -> WorkingBuffer {
    let mut log = FrameLog::new(8);
    render_frame(
        document.project(),
        &Id::new(MAIN),
        frame,
        &root(),
        64,
        &mut log,
    )
    .unwrap_or_else(|d| panic!("frame {frame} renders: {}", d.message))
}

/// The largest difference as a share of the expected value, or infinity when an expected 0 is
/// anything but 0.
fn largest_share(buffer: &WorkingBuffer, expected: &J) -> f64 {
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
        .map(|(&a, &b)| match b {
            0.0 if a == 0.0 => 0.0,
            0.0 => f64::INFINITY,
            _ => (a as f64 - b).abs() / b.abs(),
        })
        .fold(0.0, f64::max)
}

/// Two files hold the same thing. A number is compared as a number, because the reference
/// writes `0` where the build writes `0.0`.
fn same_json(a: &J, b: &J) -> bool {
    match (a, b) {
        (J::Number(x), J::Number(y)) => x.as_f64() == y.as_f64(),
        (J::Array(x), J::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(u, v)| same_json(u, v))
        }
        (J::Object(x), J::Object(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(k, u)| y.get(k).is_some_and(|v| same_json(u, v)))
        }
        _ => a == b,
    }
}

fn load(file: &str) -> persist::Loaded {
    persist::load(&root().join(file)).unwrap_or_else(|d| panic!("{file} opens: {}", d.message))
}

fn effect_of(document: &Document) -> Effect {
    let comp = document.project().composition(&Id::new(MAIN)).unwrap();
    comp.layer(&Id::new("adj")).unwrap().effects[0].effect.clone()
}

#[test]
fn b34_limits() {
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root().join("expected_limits.json")).unwrap())
            .unwrap();
    let tolerance = expected["relative_tolerance"].as_f64().unwrap();
    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-34: limits for the blur and exposure\n\nD-90, accepted by the owner on 2026-09-25 \
         (\"works; proceed with limits\"): a Gaussian blur's sigma runs from 0 to 500 and \
         exposure from -20 to 20 stops; D-318 (2026-10-04) widens exposure to -40 to 40. Every expected pixel is \
         `Fixtures/limits/expected_limits.json`, written by `tools/limits_reference.py` before \
         this code existed and printed in document 25 as FX-LIMIT-001 to 010. The build's frame \
         is compared sample by sample; the answer is the largest difference as a share of the \
         expected value, against the catalogue's 1e-4, and an expected 0 must be exactly 0.\n",
    );

    // -----------------------------------------------------------------------------------
    t.heading("FX-LIMIT-001 to 016 (document 25)");
    let mut cases: Vec<(String, J)> = Vec::new();
    for file in ["expected_limits.json", "expected_limits_d318.json"] {
        let e: J = serde_json::from_str(&fs::read_to_string(root().join(file)).unwrap()).unwrap();
        cases.extend(e["cases"].as_object().unwrap().iter().map(|(n, c)| (n.clone(), c.clone())));
    }
    for (name, case) in &cases {
        // D-318: exposure runs from -40 to 40 now. The three cases are kept as written.
        if ["FX-LIMIT-004", "FX-LIMIT-006", "FX-LIMIT-007"].contains(&name.as_str()) {
            let loaded = load(case["project"].as_str().unwrap());
            let warned = !loaded.warnings.is_empty();
            let red = render(&loaded.document, 2).data()[0];
            let (what, ok) = if name == "FX-LIMIT-004" {
                ("the ease past 20 is no longer held at 20: frame 2 is brighter than 2^20", red > 1.01 * 1048576.0)
            } else {
                ("its value is in range now: it opens with no warning and is drawn", !warned && red != 1.0)
            };
            t.row(&format!("{name}, superseded by D-318: {what}"), &format!("red {red:e}, warned {warned}"), ok);
            continue;
        }
        let says = case["says"].as_str().unwrap();
        let file = case["project"].as_str().unwrap();
        let loaded = load(file);
        for (frame, pixels) in case["frames"].as_object().unwrap() {
            let frame: i32 = frame.parse().unwrap();
            let d = largest_share(&render(&loaded.document, frame), pixels);
            t.row(
                &format!("{name} frame {frame}: {says}"),
                &format!("largest difference {d:.1e} of the value"),
                d <= tolerance,
            );
        }
        let on_open: Vec<&str> = loaded.warnings.iter().map(|d| d.id.as_str()).collect();
        let mut log = FrameLog::new(8);
        let _ = plan_frame(
            loaded.document.project(),
            &Id::new(MAIN),
            2,
            &root(),
            &mut log,
        );
        let at_frame: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
        let want: Vec<&str> = case
            .get("warning")
            .and_then(J::as_str)
            .into_iter()
            .collect();
        t.row(
            &format!("{name}: what opening it warns of, and what frame 2 warns of"),
            &format!("{on_open:?} and {at_frame:?}"),
            on_open == want && at_frame == want,
        );
        if case.get("warning").is_some() {
            // D-46: the value past the limit is kept as written, not repaired.
            let written: J = serde_json::from_str(&persist::to_json(
                loaded.document.project(),
                &loaded.preserved,
            ))
            .unwrap();
            let original: J =
                serde_json::from_str(&fs::read_to_string(root().join(file)).unwrap()).unwrap();
            let path = |j: &J| j["compositions"][0]["layers"][1]["effects"][0].clone();
            let kept = same_json(&path(&written), &path(&original));
            t.row(
                &format!("{name} saved again keeps the setting as written"),
                &path(&written)["parameters"].to_string(),
                kept,
            );
        }
    }

    // -----------------------------------------------------------------------------------
    t.heading("Commands (D-46)");
    let set = |effect: Effect| Command::SetEffectParameters {
        composition: Id::new(MAIN),
        layer_id: Id::new("adj"),
        instance_id: Id::new("fx-1-0"),
        effect,
    };
    for (file, value, sentence) in [
        (
            "fx_limit_003.json",
            Effect::GaussianBlur { sigma_px: 501.0, edges: "transparent".into(), dimensions: "both".into(), units: "sigma".into() },
            "A Gaussian blur's sigma runs from 0 to 500, and this is 501.",
        ),
        (
            "fx_limit_003.json",
            Effect::GaussianBlur { sigma_px: -1.0, edges: "transparent".into(), dimensions: "both".into(), units: "sigma".into() },
            "A Gaussian blur's sigma runs from 0 to 500, and this is -1.",
        ),
        (
            "fx_limit_001.json",
            Effect::exposure(41.0),
            "Exposure runs from -40 to 40 stops, and this is 41.",
        ),
        (
            "fx_limit_001.json",
            Effect::exposure(-41.0),
            "Exposure runs from -40 to 40 stops, and this is -41.",
        ),
        (
            "fx_limit_001.json",
            Effect::exposure(128.0),
            "Exposure runs from -40 to 40 stops, and this is 128.",
        ),
    ] {
        let mut document = load(file).document;
        let (held, revision, depth) = (effect_of(&document), document.revision(), 0);
        let refused = document.apply(set(value)).err();
        let message = refused.as_ref().map_or("taken".to_string(), |d| d.message.clone());
        let untouched = effect_of(&document) == held
            && document.revision() == revision
            && document.undo_depth() == depth;
        t.row(
            &format!("{file}: \"{sentence}\" and nothing changes"),
            &message,
            message == sentence && untouched,
        );
    }
    for (file, value) in [
        ("fx_limit_003.json", Effect::GaussianBlur { sigma_px: 0.0, edges: "transparent".into(), dimensions: "both".into(), units: "sigma".into() }),
        ("fx_limit_003.json", Effect::GaussianBlur { sigma_px: 500.0, edges: "transparent".into(), dimensions: "both".into(), units: "sigma".into() }),
        ("fx_limit_001.json", Effect::exposure(-20.0)),
        ("fx_limit_001.json", Effect::exposure(20.0)),
        ("fx_limit_001.json", Effect::exposure(21.0)),
        ("fx_limit_001.json", Effect::exposure(-40.0)),
        ("fx_limit_001.json", Effect::exposure(40.0)),
    ] {
        let mut document = load(file).document;
        let what = format!("{value:?}");
        let taken = document.apply(set(value.clone())).is_ok() && effect_of(&document) == value;
        t.row(
            &format!("{file}: {what}, on the limit, is taken"),
            if taken { "taken" } else { "refused" },
            taken,
        );
    }

    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-34_limits_table.md"), &t.out).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-34_limits_table.md");
}
