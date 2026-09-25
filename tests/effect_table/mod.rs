//! What the tables of the 2026-09-25 batch of effects share (B-35 to B-41): each renders its
//! fixtures against the reference tool's numbers, opens and saves its files, refuses and takes
//! commands, and cuts a frame into tiles, and writes the answers as a table for the owner.
//!
//! Every item is `#[allow(dead_code)]` for the reason `tests/common/mod.rs` gives.
#![allow(dead_code)]

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

pub const MAIN: &str = "comp-main";

pub fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// A table of checks, each with the build's answer and whether it matches.
pub struct Table {
    pub root: PathBuf,
    out: String,
    checks: usize,
    passed: usize,
}

impl Table {
    /// `fixtures` is the folder under `Fixtures/`; `intro` the Markdown above the first table.
    pub fn new(fixtures: &str, intro: &str) -> Table {
        Table {
            root: repo(&format!("Fixtures/{fixtures}")),
            out: intro.to_string(),
            checks: 0,
            passed: 0,
        }
    }

    pub fn row(&mut self, what: &str, built: &str, ok: bool) {
        self.checks += 1;
        self.passed += ok as usize;
        let verdict = if ok { "yes" } else { "**NO**" };
        self.out
            .push_str(&format!("| {what} | {built} | {verdict} |\n"));
    }

    pub fn heading(&mut self, text: &str) {
        self.out.push_str(&format!(
            "\n## {text}\n\n| Check | The build's answer | Matches |\n| --- | --- | --- |\n"
        ));
    }

    /// Write the table to `verification/{file}` and fail the test on any row that does not match.
    pub fn finish(mut self, file: &str) {
        self.out.push_str(&format!(
            "\n## Result\n\n{} of {} checks pass.\n",
            self.passed, self.checks
        ));
        fs::write(repo(&format!("verification/{file}")), &self.out).unwrap();
        assert_eq!(self.passed, self.checks, "see verification/{file}");
    }

    pub fn load(&self, file: &str) -> persist::Loaded {
        persist::load(&self.root.join(file))
            .unwrap_or_else(|d| panic!("{file} opens: {}", d.message))
    }

    pub fn render(&self, document: &Document, frame: i32, tile: usize) -> WorkingBuffer {
        let mut log = FrameLog::new(8);
        render_frame(
            document.project(),
            &Id::new(MAIN),
            frame,
            &self.root,
            tile,
            &mut log,
        )
        .unwrap_or_else(|d| panic!("frame {frame} renders: {}", d.message))
    }

    /// Every case of `expected`: each frame against the reference's numbers, and what opening
    /// the file and planning frame 4 warn of.
    pub fn fixtures(&mut self, expected: &str) {
        let expected: J =
            serde_json::from_str(&fs::read_to_string(self.root.join(expected)).unwrap()).unwrap();
        let tolerance = expected["tolerance"].as_f64().unwrap();
        for (name, case) in expected["cases"].as_object().unwrap() {
            let says = case["says"].as_str().unwrap();
            let loaded = self.load(case["project"].as_str().unwrap());
            for (frame, pixels) in case["frames"].as_object().unwrap() {
                let frame: i32 = frame.parse().unwrap();
                let d = largest_difference(&self.render(&loaded.document, frame, 64), pixels);
                self.row(
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
                &self.root,
                &mut log,
            );
            let at_frame: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
            let want: Vec<&str> = case
                .get("warning")
                .and_then(J::as_str)
                .into_iter()
                .collect();
            self.row(
                &format!("{name}: what opening it warns of, and what frame 4 warns of"),
                &format!("{on_open:?} and {at_frame:?}"),
                on_open == want && at_frame == want,
            );
        }
    }

    /// Each file opened and saved holds what it held.
    pub fn round_trips(&mut self, files: &[&str]) {
        for file in files {
            let original: J =
                serde_json::from_str(&fs::read_to_string(self.root.join(file)).unwrap()).unwrap();
            let same = same_json(&saved(&self.load(file)), &original);
            self.row(
                &format!(
                    "{file} opened and saved holds what it held, out-of-range values, wrong \
                     words and keys included"
                ),
                if same { "the same" } else { "differs" },
                same,
            );
        }
    }

    /// The parameters `file` is saved with.
    pub fn saved_parameters(&self, file: &str) -> J {
        saved(&self.load(file))["compositions"][0]["layers"][0]["effects"][0]["parameters"].clone()
    }

    /// `file` with its effect's parameters replaced by `parameters` is refused on opening as a
    /// fault in its shape.
    pub fn shape_refused(&mut self, file: &str, what: &str, parameters: &str) {
        let mut json: J =
            serde_json::from_str(&fs::read_to_string(self.root.join(file)).unwrap()).unwrap();
        json["compositions"][0]["layers"][0]["effects"][0]["parameters"] =
            serde_json::from_str(parameters).unwrap();
        let path = std::env::temp_dir().join(format!("effect_table_shape_{}.json", self.checks));
        fs::write(&path, json.to_string()).unwrap();
        let refused = persist::load(&path).err();
        self.row(
            &format!("a file with {what} is refused as a fault in its shape"),
            &refused
                .as_ref()
                .map_or("opened".to_string(), |d| d.message.clone()),
            refused.is_some(),
        );
    }

    /// Each command is refused with a sentence and leaves the effect as it was.
    pub fn refused(&mut self, document: &mut Document, commands: Vec<(&str, Command)>) {
        let held = settings(document);
        for (what, command) in commands {
            let refused = document.apply(command).err();
            let untouched = settings(document) == held;
            self.row(
                &format!("{what} is refused with a sentence, and nothing changes"),
                &refused
                    .as_ref()
                    .map_or("taken".to_string(), |d| d.message.clone()),
                refused.is_some() && untouched,
            );
        }
    }

    /// Each command is taken; then as many undos give back `file`'s frame 0 exactly.
    pub fn taken(&mut self, document: &mut Document, file: &str, commands: Vec<(&str, Command)>) {
        let n = commands.len();
        for (what, command) in commands {
            let taken = document.apply(command).is_ok();
            self.row(
                &format!("{what} is taken"),
                if taken { "taken" } else { "refused" },
                taken,
            );
        }
        let before = self.render(&self.load(file).document, 0, 64);
        for _ in 0..n {
            document.undo();
        }
        let same = self.render(document, 0, 64).data() == before.data();
        self.row(
            &format!("undo {n} times: frame 0 is the frame it was"),
            if same { "byte-identical" } else { "differ" },
            same,
        );
    }

    /// The frame is the same cut into tiles of 1 pixel and of 64.
    pub fn tiles(&mut self, cases: &[(&str, i32)]) {
        for &(file, frame) in cases {
            let d = self.load(file).document;
            let same = self.render(&d, frame, 64).data() == self.render(&d, frame, 1).data();
            self.row(
                &format!("{file} frame {frame} in tiles of 1 and of 64"),
                if same { "byte-identical" } else { "differ" },
                same,
            );
        }
    }
}

/// The largest difference between a frame and the reference's pixels.
pub fn largest_difference(buffer: &WorkingBuffer, expected: &J) -> f64 {
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
pub fn same_json(a: &J, b: &J) -> bool {
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

pub fn saved(loaded: &persist::Loaded) -> J {
    serde_json::from_str(&persist::to_json(
        loaded.document.project(),
        &loaded.preserved,
    ))
    .unwrap()
}

/// The `art` layer's first effect and every key it holds, to see that nothing changed.
pub fn settings(document: &Document) -> String {
    let comp = document.project().composition(&Id::new(MAIN)).unwrap();
    format!("{:?}", comp.layer(&Id::new("art")).unwrap().effects[0])
}

/// Set the `art` layer's first effect.
pub fn set(effect: Effect) -> Command {
    Command::SetEffectParameters {
        composition: Id::new(MAIN),
        layer_id: Id::new("art"),
        instance_id: Id::new("fx-0-0"),
        effect,
    }
}

/// Key a setting of the `art` layer's first effect, linearly; each value is one number or a
/// point's two.
pub fn keys(setting: &str, values: &[(i32, &[f64])]) -> Command {
    Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("art"),
        instance_id: Id::new("fx-0-0"),
        setting: setting.to_string(),
        keys: values
            .iter()
            .map(|&(frame, v)| EffectKey {
                frame,
                value: v.to_vec(),
                interp: Interp::Linear,
            })
            .collect(),
    }
}

/// Nine distinct colours, one more than a chosen-colour list takes.
pub const NINE: &[&str] = &[
    "#000001", "#000002", "#000003", "#000004", "#000005", "#000006", "#000007", "#000008",
    "#000009",
];

pub fn strings(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}
