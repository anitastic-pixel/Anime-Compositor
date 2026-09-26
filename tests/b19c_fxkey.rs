//! B-19c: keyframed effect settings in the core, against D-68.
//!
//! Writes `verification/B-19c_fxkey_table.md`.
//!
//! Every expected pixel is `Fixtures/fxkey/expected_fxkey.json`, written by
//! `tools/fxkey_reference.py` before this code existed and printed in document 25 as FX-FXK-001
//! to 009. Tolerance 1e-6. Nothing here is a snapshot of a run.
//!
//! Not here: the stopwatch, the keys on the timeline and the setting in the graph are B-19d.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{plan_frame, render_frame};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::EffectKey;
use anime_compositor::model::{Id, Interp};
use anime_compositor::persist;
use anime_compositor::WorkingBuffer;

const MAIN: &str = "comp-main";

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/fxkey")
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

/// The keys of one setting of the `adj` layer's first effect, as "frame:value" words.
fn keys_of(document: &Document, setting: &str) -> String {
    let comp = document.project().composition(&Id::new(MAIN)).unwrap();
    let keys = comp.layer(&Id::new("adj")).unwrap().effects[0].keys(setting);
    if keys.is_empty() {
        return "no keys".to_string();
    }
    keys.iter()
        .map(|k| format!("{}:{:?}", k.frame, k.value))
        .collect::<Vec<_>>()
        .join(" ")
}

fn set_keys(setting: &str, keys: Vec<EffectKey>) -> Command {
    Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("adj"),
        instance_id: Id::new("fx-1-0"),
        setting: setting.to_string(),
        keys,
    }
}

fn key(frame: i32, value: f64) -> EffectKey {
    EffectKey {
        frame,
        value: vec![value],
        interp: Interp::Linear,
    }
}

#[test]
fn b19c_fxkey() {
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root().join("expected_fxkey.json")).unwrap())
            .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-19c: keyframed effect settings\n\nD-68, accepted by the owner on 2026-09-18. Every \
         expected pixel is `Fixtures/fxkey/expected_fxkey.json`, written by \
         `tools/fxkey_reference.py` before this code existed and printed in document 25 as \
         FX-FXK-001 to 009. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 1e-6.\n",
    );

    // -----------------------------------------------------------------------------------
    t.heading("FX-FXK-001 to 009 (document 25)");
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
    }

    // -----------------------------------------------------------------------------------
    t.heading("The file");
    for file in ["fx_fxk_003.json", "fx_fxk_008.json", "fx_fxk_009.json"] {
        let loaded = load(file);
        let written: J = serde_json::from_str(&persist::to_json(
            loaded.document.project(),
            &loaded.preserved,
        ))
        .unwrap();
        let original: J =
            serde_json::from_str(&fs::read_to_string(root().join(file)).unwrap()).unwrap();
        t.row(
            &format!("{file} opened and saved holds what it held, keys and eases included"),
            if same_json(&written, &original) {
                "the same"
            } else {
                "differs"
            },
            same_json(&written, &original),
        );
    }
    let plain_file = repo("Fixtures/adjust/fx_adj_003.json");
    let plain = persist::load(&plain_file).expect("an adjustment fixture opens");
    let text = fs::read_to_string(&plain_file).unwrap();
    let written: J = serde_json::from_str(&persist::to_json(
        plain.document.project(),
        &plain.preserved,
    ))
    .unwrap();
    t.row(
        "a project made before D-68 saves with every setting still a plain number",
        if same_json(&written, &serde_json::from_str::<J>(&text).unwrap()) {
            "the same"
        } else {
            "differs"
        },
        same_json(&written, &serde_json::from_str::<J>(&text).unwrap()),
    );
    // P-17: exposure has no radius; keys on one used to be written back inside themselves.
    let text = fs::read_to_string(root().join("fx_fxk_001.json")).unwrap().replacen(
        "\"parameters\": {",
        "\"parameters\": {\"radius\": {\"base\": 5, \"keyframes\": [{\"frame\": 0, \"value\": 1, \
         \"interp\": \"linear\"}, {\"frame\": 4, \"value\": 9, \"interp\": \"linear\"}]}, ",
        1,
    );
    let loaded = persist::load_str(&text).expect("FX-FXK-001 with a keyed radius opens");
    let saved = persist::to_json(loaded.document.project(), &loaded.preserved);
    let same = same_json(
        &serde_json::from_str::<J>(&saved).unwrap(),
        &serde_json::from_str::<J>(&text).unwrap(),
    );
    let reopens = persist::load_str(&saved).is_ok();
    t.row(
        "FX-FXK-001 with keys on a radius, which exposure does not have, saves them as written \
         and opens again",
        &format!(
            "{}, {}",
            if same { "the same" } else { "differs" },
            if reopens { "opens" } else { "refused" }
        ),
        same && reopens,
    );
    for (what, from, to) in [
        (
            "path handles on a setting's key",
            "\"interp\": \"linear\"",
            "\"interp\": \"linear\", \"spatial\": [0,0,0,0]",
        ),
        ("two keys on one frame", "\"frame\": 3", "\"frame\": 1"),
    ] {
        let text = fs::read_to_string(root().join("fx_fxk_001.json")).unwrap();
        let refused = persist::load_str(&text.replacen(from, to, 1))
            .err()
            .map(|d| d.id.as_str());
        t.row(
            &format!("FX-FXK-001 with {what} is refused"),
            &format!("{refused:?}"),
            refused == Some("PROJECT_SCHEMA_INVALID"),
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("Commands and undo");
    let mut document = load("fx_fxk_001.json").document;
    let before = render(&document, 2, 64);
    t.row(
        "FX-FXK-001 as opened: the keys of stops",
        &keys_of(&document, "stops"),
        true,
    );
    document
        .apply(Command::ShiftLayer {
            composition: Id::new(MAIN),
            layer_id: Id::new("adj"),
            in_frame: 1,
        })
        .expect("the layer shifts");
    let shifted = keys_of(&document, "stops");
    t.row(
        "the layer moved one frame later: its effect's keys moved with it",
        &shifted,
        shifted == "2:[0.0] 4:[2.0]",
    );
    document.undo();
    let back = keys_of(&document, "stops");
    t.row("undo puts them back", &back, back == "1:[0.0] 3:[2.0]");

    document
        .apply(set_keys(
            "stops",
            vec![key(0, 0.0), key(2, 1.0), key(4, 3.0)],
        ))
        .expect("the keys are taken");
    let now = keys_of(&document, "stops");
    t.row(
        "the keys of stops replaced by three, as one entry to undo",
        &format!("{now}, undo depth {}", document.undo_depth()),
        now == "0:[0.0] 2:[1.0] 4:[3.0]" && document.undo_depth() == 1,
    );
    document.undo();
    let same = render(&document, 2, 64).data() == before.data();
    t.row(
        "undo: frame 2 is the frame it was",
        if same { "byte-identical" } else { "differ" },
        same,
    );
    document
        .apply(set_keys("stops", Vec::new()))
        .expect("no keys is allowed");
    let written = persist::to_json(document.project(), &persist::Preserved::default());
    let root_json: J = serde_json::from_str(&written).unwrap();
    let stops = &root_json["compositions"][0]["layers"][1]["effects"][0]["parameters"]["stops"];
    t.row(
        "no keys: the setting is constant again and is written as a plain number",
        &stops.to_string(),
        stops.is_number(),
    );

    let mut tint = load("fx_fxk_005.json").document;
    let held = keys_of(&tint, "amount");
    for (what, command) in [
        (
            "a tint amount keyed to 1.5",
            set_keys("amount", vec![key(0, 0.0), key(4, 1.5)]),
        ),
        (
            "two keys on one frame",
            set_keys("amount", vec![key(2, 0.0), key(2, 1.0)]),
        ),
        (
            "one number for a colour",
            set_keys("color", vec![key(0, 0.5)]),
        ),
        (
            "a setting a tint does not have",
            set_keys("stops", vec![key(0, 0.5)]),
        ),
    ] {
        let refused = tint.apply(command).err().map(|d| d.id.as_str());
        let untouched = keys_of(&tint, "amount") == held;
        t.row(
            &format!("{what} is refused and nothing changes"),
            &format!("{refused:?}"),
            refused.is_some() && untouched,
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("The frame does not depend on how it is cut up");
    let blur = load("fx_fxk_006.json").document;
    let whole = render(&blur, 2, 64);
    let cut = render(&blur, 2, 1);
    t.row(
        "FX-FXK-006 frame 2 in tiles of 1 and of 64",
        if whole.data() == cut.data() {
            "byte-identical"
        } else {
            "differ"
        },
        whole.data() == cut.data(),
    );

    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-19c_fxkey_table.md"), &t.out).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-19c_fxkey_table.md");
}
