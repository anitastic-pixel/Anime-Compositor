//! B-30b: line smoothing in the core, against D-86.
//!
//! Writes `verification/B-30b_line_smooth_table.md`.
//!
//! Every expected pixel is `Fixtures/smooth/expected_smooth.json`, written by
//! `tools/smooth_reference.py` before this code existed and printed in document 25 as
//! FX-SMOOTH-001 to 022. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Not here: Line Smoothing in the Effects panel, and `effect.add` putting it at the top of the
//! stack, which is the window's command and is checked by the window's tests in B-30c.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{plan_frame, render_frame};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::model::{Id, Interp};
use anime_compositor::persist;
use anime_compositor::WorkingBuffer;

const MAIN: &str = "comp-main";

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/smooth")
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

fn render(document: &Document, frame: i32, tile: usize) -> WorkingBuffer {
    let mut log = FrameLog::new(8);
    render_frame(
        document.project(),
        &Id::new(MAIN),
        frame,
        &root(),
        tile,
        &mut log,
    )
    .unwrap_or_else(|d| panic!("frame {frame} renders: {}", d.message))
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

/// The `art` layer's smoothing, as the file would write it.
fn settings(document: &Document) -> String {
    let comp = document.project().composition(&Id::new(MAIN)).unwrap();
    let effect = &comp.layer(&Id::new("art")).unwrap().effects[0];
    format!(
        "{:?} {:?}, keys {}",
        effect.effect,
        effect.keys("softness").len(),
        effect.keys("threshold").len()
    )
}

#[test]
fn b30_line_smooth() {
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root().join("expected_smooth.json")).unwrap())
            .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-30b: line smoothing\n\nD-86, accepted by the owner on 2026-09-25. Every expected \
         pixel is `Fixtures/smooth/expected_smooth.json`, written by `tools/smooth_reference.py` \
         before this code existed and printed in document 25 as FX-SMOOTH-001 to 022. The \
         build's frame is compared sample by sample; the answer is the largest difference over \
         all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    // -----------------------------------------------------------------------------------
    t.heading("FX-SMOOTH-001 to 022 (document 25)");
    for (name, case) in expected["cases"].as_object().unwrap() {
        let says = case["says"].as_str().unwrap();
        let loaded = load(case["project"].as_str().unwrap());
        for (frame, pixels) in case["frames"].as_object().unwrap() {
            let frame: i32 = frame.parse().unwrap();
            let d = largest_difference(&render(&loaded.document, frame, 64), pixels);
            t.row(
                &format!("{name} frame {frame}: {says}"),
                &format!("largest difference {d:.1e}"),
                d <= tolerance,
            );
        }
        let on_open: Vec<&str> = loaded.warnings.iter().map(|d| d.id.as_str()).collect();
        let mut log = FrameLog::new(8);
        let _ = plan_frame(
            loaded.document.project(),
            &Id::new(MAIN),
            4,
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
            &format!("{name}: what opening it warns of, and what frame 4 warns of"),
            &format!("{on_open:?} and {at_frame:?}"),
            on_open == want && at_frame == want,
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("The file");
    for file in [
        "fx_smooth_001.json",
        "fx_smooth_012.json",
        "fx_smooth_020.json",
        "fx_smooth_022.json",
    ] {
        let loaded = load(file);
        let written: J = serde_json::from_str(&persist::to_json(
            loaded.document.project(),
            &loaded.preserved,
        ))
        .unwrap();
        let original: J =
            serde_json::from_str(&fs::read_to_string(root().join(file)).unwrap()).unwrap();
        t.row(
            &format!("{file} opened and saved holds what it held, out-of-range values and keys included"),
            if same_json(&written, &original) {
                "the same"
            } else {
                "differs"
            },
            same_json(&written, &original),
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("Commands");
    let mut document = load("fx_smooth_001.json").document;
    let held = settings(&document);
    let set = |softness: f64, threshold: f64| Command::SetEffectParameters {
        composition: Id::new(MAIN),
        layer_id: Id::new("art"),
        instance_id: Id::new("fx-0-0"),
        effect: Effect::LineSmooth {
            softness,
            threshold,
        },
    };
    let keys = |values: &[(i32, f64)]| Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("art"),
        instance_id: Id::new("fx-0-0"),
        setting: "softness".to_string(),
        keys: values
            .iter()
            .map(|&(frame, v)| EffectKey {
                frame,
                value: vec![v],
                interp: Interp::Linear,
            })
            .collect(),
    };
    for (what, command) in [
        ("softness 101", set(101.0, 10.0)),
        ("softness -1", set(-1.0, 10.0)),
        ("threshold 256", set(50.0, 256.0)),
        ("threshold -1", set(50.0, -1.0)),
        ("softness keyed to 120", keys(&[(0, 0.0), (4, 120.0)])),
    ] {
        let refused = document.apply(command).err();
        let untouched = settings(&document) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused
                .as_ref()
                .map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
    for (what, command) in [
        ("softness 100 and threshold 255", set(100.0, 255.0)),
        ("softness 0 and threshold 0", set(0.0, 0.0)),
    ] {
        let taken = document.apply(command).is_ok();
        t.row(
            &format!("{what}, the ends of the ranges, are taken"),
            if taken { "taken" } else { "refused" },
            taken,
        );
    }
    let before = render(&load("fx_smooth_001.json").document, 0, 64);
    document.undo();
    document.undo();
    let same = render(&document, 0, 64).data() == before.data();
    t.row(
        "undo twice: frame 0 is the frame it was",
        if same { "byte-identical" } else { "differ" },
        same,
    );

    // -----------------------------------------------------------------------------------
    t.heading("The frame does not depend on how it is cut up");
    for file in ["fx_smooth_004.json", "fx_smooth_013.json"] {
        let d = load(file).document;
        let whole = render(&d, 3, 64);
        let cut = render(&d, 3, 1);
        t.row(
            &format!("{file} frame 3 in tiles of 1 and of 64"),
            if whole.data() == cut.data() {
                "byte-identical"
            } else {
                "differ"
            },
            whole.data() == cut.data(),
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("The packaged build carries OpenToonz's licence");
    let package = fs::read_to_string(repo("tools/package.ps1")).unwrap();
    let licence = fs::read_to_string(repo("docs/third_party/OpenToonz-LICENSE.txt")).unwrap();
    let port = fs::read_to_string(repo("src/line_smooth.rs")).unwrap();
    t.row(
        "`tools/package.ps1` puts `docs/third_party/OpenToonz-LICENSE.txt` in the package",
        if package.contains("OpenToonz-LICENSE.txt") {
            "it does"
        } else {
            "it does not"
        },
        package.contains("OpenToonz-LICENSE.txt"),
    );
    let noticed = port.contains("Copyright (c) 2016 - 2026, DWANGO Co., Ltd.")
        && licence.contains("Copyright (c) 2016 - 2026, DWANGO Co., Ltd.");
    t.row(
        "the port and the licence file carry OpenToonz's copyright notice",
        if noticed { "both" } else { "not both" },
        noticed,
    );

    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-30b_line_smooth_table.md"), &t.out).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-30b_line_smooth_table.md");
}
