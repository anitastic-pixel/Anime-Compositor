//! B-33b: glow in the core, against D-89.
//!
//! Writes `verification/B-33b_glow_table.md`.
//!
//! Every expected pixel is `Fixtures/glow/expected_glow.json`, written by
//! `tools/glow_reference.py` before this code existed and printed in document 25 as FX-GLOW-001
//! to 033. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Not here: Glow in the Effects panel, which is B-33c.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{plan_frame, render_frame};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::model::{Id, Interp};
use anime_compositor::persist;
use anime_compositor::preview::{scale_plan, PreviewQuality};
use anime_compositor::WorkingBuffer;

const MAIN: &str = "comp-main";

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/glow")
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

/// A glow's settings, the fixtures' defaults unless changed.
struct Glow {
    based_on: &'static str,
    threshold: f64,
    colors: &'static [&'static str],
    tolerance: f64,
    radius: f64,
    intensity: f64,
    operation: &'static str,
    tint: &'static str,
}

const PLAIN: Glow = Glow {
    based_on: "bright",
    threshold: 60.0,
    colors: &[],
    tolerance: 0.0,
    radius: 4.0,
    intensity: 1.0,
    operation: "add",
    tint: "",
};

impl Glow {
    fn effect(&self) -> Effect {
        Effect::Glow {
            based_on: self.based_on.to_string(),
            threshold: self.threshold,
            colors: self.colors.iter().map(|c| c.to_string()).collect(),
            tolerance: self.tolerance,
            radius: self.radius,
            intensity: self.intensity,
            operation: self.operation.to_string(),
            tint: self.tint.to_string(),
        }
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

fn saved(loaded: &persist::Loaded) -> J {
    serde_json::from_str(&persist::to_json(
        loaded.document.project(),
        &loaded.preserved,
    ))
    .unwrap()
}

/// The `art` layer's glow, and how many keys its radius has.
fn settings(document: &Document) -> String {
    let comp = document.project().composition(&Id::new(MAIN)).unwrap();
    let effect = &comp.layer(&Id::new("art")).unwrap().effects[0];
    format!("{:?}, keys {}", effect.effect, effect.keys("radius").len())
}

#[test]
fn b33_glow() {
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root().join("expected_glow.json")).unwrap())
            .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-33b: glow\n\nD-89, accepted by the owner on 2026-09-25. Every expected pixel is \
         `Fixtures/glow/expected_glow.json`, written by `tools/glow_reference.py` before this \
         code existed and printed in document 25 as FX-GLOW-001 to 033. The build's frame is \
         compared sample by sample; the answer is the largest difference over all of them, \
         against the catalogue's tolerance of 2e-5.\n",
    );

    // -----------------------------------------------------------------------------------
    t.heading("FX-GLOW-001 to 033 (document 25)");
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
    t.heading("How far the light reaches");
    for (radius, reach) in [(4.0, 4), (2.5, 3), (0.0, 0), (500.0, 500)] {
        let got = Glow { radius, ..PLAIN }.effect().bounds_expansion();
        t.row(
            &format!("radius {radius} grows the drawing's bounds by {reach} pixels on each side"),
            &got.to_string(),
            got == reach,
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("The file");
    for file in [
        "fx_glow_001.json",
        "fx_glow_007.json",
        "fx_glow_013.json",
        "fx_glow_016.json",
        "fx_glow_017.json",
        "fx_glow_022.json",
        "fx_glow_024.json",
        "fx_glow_029.json",
        "fx_glow_030.json",
        "fx_glow_031.json",
        "fx_glow_032.json",
        "fx_glow_033.json",
    ] {
        let original: J =
            serde_json::from_str(&fs::read_to_string(root().join(file)).unwrap()).unwrap();
        let same = same_json(&saved(&load(file)), &original);
        t.row(
            &format!(
                "{file} opened and saved holds what it held, out-of-range values, wrong words \
                 and keys included"
            ),
            if same { "the same" } else { "differs" },
            same,
        );
    }
    let params =
        saved(&load("fx_glow_021.json"))["compositions"][0]["layers"][0]["effects"][0]
            ["parameters"]
            .clone();
    let small = (&params["colors"], &params["tint"]);
    t.row(
        "fx_glow_021.json, written in capitals, is saved in small letters",
        &format!("{} and {}", small.0, small.1),
        small == (&serde_json::json!(["#3c286e"]), &serde_json::json!("#ff4000")),
    );
    for (what, parameters) in [
        (
            "no `based_on` at all",
            r#"{"threshold": 60, "colors": [], "tolerance": 0, "radius": 4, "intensity": 1, "operation": "add", "tint": ""}"#,
        ),
        (
            "a tint that is a number",
            r#"{"based_on": "bright", "threshold": 60, "colors": [], "tolerance": 0, "radius": 4, "intensity": 1, "operation": "add", "tint": 16711680}"#,
        ),
        (
            "a radius that is a word",
            r#"{"based_on": "bright", "threshold": 60, "colors": [], "tolerance": 0, "radius": "big", "intensity": 1, "operation": "add", "tint": ""}"#,
        ),
    ] {
        let text = fs::read_to_string(root().join("fx_glow_001.json")).unwrap();
        let mut file: J = serde_json::from_str(&text).unwrap();
        file["compositions"][0]["layers"][0]["effects"][0]["parameters"] =
            serde_json::from_str(parameters).unwrap();
        let path = std::env::temp_dir().join("b33_glow_shape.json");
        fs::write(&path, file.to_string()).unwrap();
        let refused = persist::load(&path).err();
        t.row(
            &format!("a file with {what} is refused as a fault in its shape"),
            &refused
                .as_ref()
                .map_or("opened".to_string(), |d| d.message.clone()),
            refused.is_some(),
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("Commands");
    let mut document = load("fx_glow_001.json").document;
    let held = settings(&document);
    let set = |glow: Glow| Command::SetEffectParameters {
        composition: Id::new(MAIN),
        layer_id: Id::new("art"),
        instance_id: Id::new("fx-0-0"),
        effect: glow.effect(),
    };
    let keys = |setting: &str, values: &[(i32, f64)]| Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("art"),
        instance_id: Id::new("fx-0-0"),
        setting: setting.to_string(),
        keys: values
            .iter()
            .map(|&(frame, v)| EffectKey {
                frame,
                value: vec![v],
                interp: Interp::Linear,
            })
            .collect(),
    };
    let nine: &[&str] = &[
        "#000001", "#000002", "#000003", "#000004", "#000005", "#000006", "#000007", "#000008",
        "#000009",
    ];
    for (what, command) in [
        ("radius 501", set(Glow { radius: 501.0, ..PLAIN })),
        ("radius -1", set(Glow { radius: -1.0, ..PLAIN })),
        ("threshold 101", set(Glow { threshold: 101.0, ..PLAIN })),
        ("intensity 11", set(Glow { intensity: 11.0, ..PLAIN })),
        ("intensity -1", set(Glow { intensity: -1.0, ..PLAIN })),
        ("tolerance 256", set(Glow { tolerance: 256.0, ..PLAIN })),
        ("nine colours", set(Glow { colors: nine, ..PLAIN })),
        ("the colour \"#12345\"", set(Glow { colors: &["#12345"], ..PLAIN })),
        ("the tint \"orange\"", set(Glow { tint: "orange", ..PLAIN })),
        ("operation \"multiply\"", set(Glow { operation: "multiply", ..PLAIN })),
        ("based on \"dark\"", set(Glow { based_on: "dark", ..PLAIN })),
        ("radius keyed to 600", keys("radius", &[(0, 0.0), (4, 600.0)])),
        ("intensity keyed to 20", keys("intensity", &[(0, 0.0), (4, 20.0)])),
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
        (
            "radius 500, threshold 100, intensity 10 and tolerance 255 with eight colours",
            set(Glow {
                radius: 500.0,
                threshold: 100.0,
                intensity: 10.0,
                tolerance: 255.0,
                colors: &nine[..8],
                ..PLAIN
            }),
        ),
        (
            "radius 0, threshold 0, intensity 0 and tolerance 0 with no colour",
            set(Glow {
                radius: 0.0,
                threshold: 0.0,
                intensity: 0.0,
                ..PLAIN
            }),
        ),
        (
            "chosen colours, Screen and a tint",
            set(Glow {
                based_on: "colors",
                colors: &["#3c286e"],
                operation: "screen",
                tint: "#ff4000",
                ..PLAIN
            }),
        ),
        ("radius keyed from 0 to 8", keys("radius", &[(0, 0.0), (4, 8.0)])),
    ] {
        let taken = document.apply(command).is_ok();
        t.row(
            &format!("{what}, the ends of the ranges and the other choices, are taken"),
            if taken { "taken" } else { "refused" },
            taken,
        );
    }
    let before = render(&load("fx_glow_001.json").document, 0, 64);
    for _ in 0..4 {
        document.undo();
    }
    let same = render(&document, 0, 64).data() == before.data();
    t.row(
        "undo four times: frame 0 is the frame it was",
        if same { "byte-identical" } else { "differ" },
        same,
    );

    // -----------------------------------------------------------------------------------
    t.heading("The frame does not depend on how it is cut up");
    for (file, frame) in [("fx_glow_014.json", 0), ("fx_glow_019.json", 3)] {
        let d = load(file).document;
        let whole = render(&d, frame, 64);
        let cut = render(&d, frame, 1);
        t.row(
            &format!("{file} frame {frame} in tiles of 1 and of 64"),
            if whole.data() == cut.data() {
                "byte-identical"
            } else {
                "differ"
            },
            whole.data() == cut.data(),
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("The draft preview (D-66)");
    // An adjustment layer's stack runs on the quarter-size frame, so the radius, a distance in
    // pixels, shrinks with it as a Gaussian blur's sigma does. FX-ADJ-007 lends the layer.
    let adjust = persist::load(&repo("Fixtures/adjust/fx_adj_007.json"))
        .unwrap()
        .document;
    let draft = |radius: f64| -> Vec<(f64, f64, f64)> {
        let mut log = FrameLog::new(8);
        let mut plan = plan_frame(
            adjust.project(),
            &Id::new(MAIN),
            0,
            &repo("Fixtures/adjust"),
            &mut log,
        )
        .unwrap();
        for layer in &mut plan.layers {
            for instance in layer.adjust.iter_mut().flatten() {
                instance.effect = Glow {
                    radius,
                    threshold: 60.0,
                    intensity: 2.0,
                    ..PLAIN
                }
                .effect();
            }
        }
        scale_plan(plan, PreviewQuality::Draft)
            .layers
            .iter()
            .flat_map(|l| l.adjust.iter().flatten())
            .filter_map(|i| match i.effect {
                Effect::Glow {
                    radius,
                    threshold,
                    intensity,
                    ..
                } => Some((radius, threshold, intensity)),
                _ => None,
            })
            .collect()
    };
    let scaled = draft(12.0);
    t.row(
        "an adjustment layer's radius 12 is radius 3 on the quarter-size draft frame, and its \
         threshold 60 and intensity 2 stay as they are",
        &format!("{scaled:?}"),
        scaled == [(3.0, 60.0, 2.0)],
    );
    // The audit of 2026-09-25: a radius out of range was scaled back inside it, so the draft
    // drew a glow the export leaves out.
    let scaled = draft(1200.0);
    t.row(
        "an adjustment layer's radius 1200, out of range, stays 1200 on the draft frame, so the \
         draft leaves it out as the export does",
        &format!("{scaled:?}"),
        scaled == [(1200.0, 60.0, 2.0)],
    );

    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-33b_glow_table.md"), &t.out).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-33b_glow_table.md");
}
