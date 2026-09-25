//! B-31b: selective colour blur in the core, against D-87.
//!
//! Writes `verification/B-31b_selective_blur_table.md`.
//!
//! Every expected pixel is `Fixtures/selblur/expected_selblur.json`, written by
//! `tools/selblur_reference.py` before this code existed and printed in document 25 as
//! FX-SELBLUR-001 to 024. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Not here: Selective Colour Blur in the Effects panel, which is B-31c. `effect.add` putting it
//! at the top of the stack is the window's command and is checked by the window's tests.

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
    repo("Fixtures/selblur")
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

fn saved(loaded: &persist::Loaded) -> J {
    serde_json::from_str(&persist::to_json(
        loaded.document.project(),
        &loaded.preserved,
    ))
    .unwrap()
}

/// The `art` layer's selective colour blur, and how many keys its blur has.
fn settings(document: &Document) -> String {
    let comp = document.project().composition(&Id::new(MAIN)).unwrap();
    let effect = &comp.layer(&Id::new("art")).unwrap().effects[0];
    format!("{:?}, keys {}", effect.effect, effect.keys("blur").len())
}

#[test]
fn b31_selective_blur() {
    let expected: J = serde_json::from_str(
        &fs::read_to_string(root().join("expected_selblur.json")).unwrap(),
    )
    .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-31b: selective colour blur\n\nD-87, accepted by the owner on 2026-09-25. Every \
         expected pixel is `Fixtures/selblur/expected_selblur.json`, written by \
         `tools/selblur_reference.py` before this code existed and printed in document 25 as \
         FX-SELBLUR-001 to 024. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    // -----------------------------------------------------------------------------------
    t.heading("FX-SELBLUR-001 to 024 (document 25)");
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
        "fx_selblur_001.json",
        "fx_selblur_009.json",
        "fx_selblur_020.json",
        "fx_selblur_023.json",
        "fx_selblur_024.json",
    ] {
        let original: J =
            serde_json::from_str(&fs::read_to_string(root().join(file)).unwrap()).unwrap();
        let same = same_json(&saved(&load(file)), &original);
        t.row(
            &format!("{file} opened and saved holds what it held, out-of-range values and keys included"),
            if same { "the same" } else { "differs" },
            same,
        );
    }
    let colors = &saved(&load("fx_selblur_015.json"))["compositions"][0]["layers"][0]["effects"]
        [0]["parameters"]["colors"];
    t.row(
        "fx_selblur_015.json, written in capitals, is saved in small letters (D-87)",
        &colors.to_string(),
        colors == &serde_json::json!(["#f6d6be", "#dba08e"]),
    );
    for (what, parameters) in [
        ("no `colors` at all", r#"{"blur": 6}"#),
        ("`colors` that is not a list", r##"{"blur": 6, "colors": "#f6d6be"}"##),
        ("a colour that is a number", r#"{"blur": 6, "colors": [16176830]}"#),
    ] {
        let text = fs::read_to_string(root().join("fx_selblur_001.json")).unwrap();
        let mut file: J = serde_json::from_str(&text).unwrap();
        file["compositions"][0]["layers"][0]["effects"][0]["parameters"] =
            serde_json::from_str(parameters).unwrap();
        let path = std::env::temp_dir().join("b31_selblur_shape.json");
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
    let mut document = load("fx_selblur_001.json").document;
    let held = settings(&document);
    let set = |blur: f64, colors: &[&str]| Command::SetEffectParameters {
        composition: Id::new(MAIN),
        layer_id: Id::new("art"),
        instance_id: Id::new("fx-0-0"),
        effect: Effect::SelectiveColorBlur {
            blur,
            colors: colors.iter().map(|c| c.to_string()).collect(),
        },
    };
    let keys = |values: &[(i32, f64)]| Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("art"),
        instance_id: Id::new("fx-0-0"),
        setting: "blur".to_string(),
        keys: values
            .iter()
            .map(|&(frame, v)| EffectKey {
                frame,
                value: vec![v],
                interp: Interp::Linear,
            })
            .collect(),
    };
    let skin = ["#f6d6be", "#dba08e"];
    let nine = ["#000001", "#000002", "#000003", "#000004", "#000005", "#000006", "#000007",
        "#000008", "#000009"];
    for (what, command) in [
        ("blur 201", set(201.0, &skin)),
        ("blur -1", set(-1.0, &skin)),
        ("nine colours", set(6.0, &nine)),
        ("the colour \"#12345\"", set(6.0, &["#12345"])),
        ("blur keyed to 250", keys(&[(0, 0.0), (4, 250.0)])),
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
        ("blur 200 with eight colours", set(200.0, &nine[..8])),
        ("blur 0 with no colour", set(0.0, &[])),
    ] {
        let taken = document.apply(command).is_ok();
        t.row(
            &format!("{what}, the ends of the ranges, are taken"),
            if taken { "taken" } else { "refused" },
            taken,
        );
    }
    let before = render(&load("fx_selblur_001.json").document, 0, 64);
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
    for file in ["fx_selblur_005.json", "fx_selblur_013.json"] {
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
    t.heading("The draft preview (D-66)");
    // An adjustment layer's stack runs on the quarter-size frame, so its blur, a distance in
    // pixels, shrinks with it as a Gaussian blur's sigma does. FX-ADJ-007 lends the layer.
    let adjust = persist::load(&repo("Fixtures/adjust/fx_adj_007.json")).unwrap().document;
    let mut log = FrameLog::new(8);
    let mut plan =
        plan_frame(adjust.project(), &Id::new(MAIN), 0, &repo("Fixtures/adjust"), &mut log)
            .unwrap();
    for layer in &mut plan.layers {
        for instance in layer.adjust.iter_mut().flatten() {
            instance.effect = Effect::SelectiveColorBlur {
                blur: 12.0,
                colors: vec!["#f6d6be".to_string()],
            };
        }
    }
    let blurs: Vec<f64> = scale_plan(plan, PreviewQuality::Draft)
        .layers
        .iter()
        .flat_map(|l| l.adjust.iter().flatten())
        .filter_map(|i| match i.effect {
            Effect::SelectiveColorBlur { blur, .. } => Some(blur),
            _ => None,
        })
        .collect();
    t.row(
        "an adjustment layer's blur 12 is blur 3 on the quarter-size draft frame",
        &format!("{blurs:?}"),
        blurs == [3.0],
    );

    // -----------------------------------------------------------------------------------
    t.heading("The packaged build carries F's Plugins' licence");
    let package = fs::read_to_string(repo("tools/package.ps1")).unwrap();
    let licence =
        fs::read_to_string(repo("docs/third_party/F-s-PluginsProjects-LICENSE.txt")).unwrap();
    let port = fs::read_to_string(repo("src/selective_blur.rs")).unwrap();
    t.row(
        "`tools/package.ps1` puts `docs/third_party/F-s-PluginsProjects-LICENSE.txt` in the package",
        if package.contains("F-s-PluginsProjects-LICENSE.txt") {
            "it does"
        } else {
            "it does not"
        },
        package.contains("F-s-PluginsProjects-LICENSE.txt"),
    );
    let noticed = port.contains("Copyright (c) 2019 bryful")
        && licence.contains("Copyright (c) 2019 bryful");
    t.row(
        "the port and the licence file carry F's Plugins' copyright notice",
        if noticed { "both" } else { "not both" },
        noticed,
    );

    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-31b_selective_blur_table.md"), &t.out).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-31b_selective_blur_table.md");
}
