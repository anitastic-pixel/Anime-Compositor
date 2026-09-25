//! B-28b: a cut imported from its XDTS timesheet, in the core, against D-84.
//!
//! Writes `verification/B-28b_timesheet_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/xdts/expected_xdts.json`, written by `tools/xdts_reference.py`, which reads the same
//! folders its own way. Document 25 prints the same timing as FX-XDTS-001 to 028 and 040;
//! FX-XDTS-030 to 036 are folders nothing is imported from. Nothing here is a snapshot of a run.
//!
//! # What is deliberately not here
//!
//! The window: the Import Cut button, its notes and the record in the inspector are B-28c.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value as J};

use anime_compositor::command::Document;
use anime_compositor::model::{Id, Project, SheetText};
use anime_compositor::persist;
use anime_compositor::timesheet::{self, Column, Note};

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/xdts")
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

/// A note as the reference writes one: its ID and its facts in one object.
fn as_json(note: &Note) -> J {
    let mut facts = note.facts.clone();
    facts.insert("id".into(), J::from(note.id.as_str()));
    J::Object(facts)
}

/// Notes compared in any order: the reference and this build may say them in different ones.
fn sorted(notes: impl IntoIterator<Item = J>) -> Vec<String> {
    let mut all: Vec<String> = notes.into_iter().map(|n| n.to_string()).collect();
    all.sort();
    all
}

/// A column as the reference writes a layer.
fn layer_json(folder: &Path, c: &Column) -> J {
    let spans: Vec<J> = c
        .spans
        .iter()
        .map(|s| json!([s.start_frame, s.end_frame_exclusive, s.drawing_number]))
        .collect();
    let drawings: serde_json::Map<String, J> = c
        .drawings
        .iter()
        .map(|(n, p)| (n.to_string(), J::from(persist::stored_path(folder, p))))
        .collect();
    json!({
        "name": c.name,
        "track": c.timesheet.track,
        "spans": spans,
        "drawings": drawings,
        "timesheet": {"sheet": c.timesheet.sheet, "column": c.timesheet.column,
                      "track": c.timesheet.track},
    })
}

/// A column's frames as document 25 prints them: its drawing on each frame, x for none.
/// A text column as the reference writes it: entries as [start, end, text].
fn text_json(c: &SheetText) -> J {
    json!({"kind": c.kind, "name": c.name, "track": c.track,
        "entries": c.entries.iter()
            .map(|e| json!([e.start_frame, e.end_frame_exclusive, e.text]))
            .collect::<Vec<_>>()})
}

fn sheet_line(c: &Column, duration: u32) -> String {
    let mut shown = vec!["x".to_string(); duration as usize];
    for s in &c.spans {
        for f in s.start_frame..s.end_frame_exclusive {
            shown[f as usize] = s.drawing_number.to_string();
        }
    }
    shown.join(" ")
}

#[test]
fn b28b_timesheet() {
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root().join("expected_xdts.json")).unwrap())
            .unwrap();
    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-28b: a cut imported from its XDTS timesheet\n\nD-84, accepted by the owner on \
         2026-09-24. Every expected value is `Fixtures/xdts/expected_xdts.json`, written by \
         `tools/xdts_reference.py` before this code existed and printed in document 25 as \
         FX-XDTS-001 to 040. For each cut the build's composition, its layers bottom first \
         (name, track, timing, drawings and timesheet record) and its notes are compared with \
         the reference's; the notes in any order. The timing is shown as document 25 prints \
         it: a column's drawing on each frame, x for none.\n",
    );
    let mut seen: BTreeMap<String, Note> = BTreeMap::new();

    // -----------------------------------------------------------------------------------
    t.heading("FX-XDTS-001 to 028 and 040, read whole (document 25)");
    for (name, case) in expected["cases"].as_object().unwrap() {
        let folder = root().join(case["folder"].as_str().unwrap());
        let cut = match timesheet::read_cut(&folder) {
            Ok(cut) => cut,
            Err(refused) => {
                t.row(name, &format!("refused: {:?}", refused.refusal), false);
                continue;
            }
        };
        let composition = json!({"name": cut.name, "duration": cut.duration,
            "frame_rate": timesheet::FRAME_RATE, "width": cut.width, "height": cut.height});
        let layers: Vec<J> = cut.columns.iter().map(|c| layer_json(&folder, c)).collect();
        let notes = sorted(cut.notes.iter().map(as_json));
        let text: Vec<J> = cut.text.iter().map(text_json).collect();
        let ok = composition == case["composition"]
            && J::from(layers) == case["layers"]
            && J::from(text) == case.get("text").cloned().unwrap_or(json!([]))
            && notes == sorted(case["notes"].as_array().unwrap().iter().cloned());
        let mut built = format!(
            "{} frames at {} by {}, named {}",
            cut.duration, cut.width, cut.height, cut.name
        );
        for c in cut.columns.iter().rev() {
            built.push_str(&format!(
                "<br>{}: `{}`",
                c.name,
                sheet_line(c, cut.duration)
            ));
        }
        for c in &cut.text {
            built.push_str(&format!(
                "<br>{} column {}: `{}`",
                c.kind,
                c.name,
                text_json(c)["entries"]
            ));
        }
        for n in &cut.notes {
            built.push_str(&format!("<br>note `{}`", as_json(n)));
            seen.entry(n.id.as_str().to_string())
                .or_insert_with(|| n.clone());
        }
        t.row(
            &format!("{name}: {}", case["says"].as_str().unwrap()),
            &built,
            ok,
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("FX-XDTS-030 to 036, nothing imported (D-84)");
    for (name, case) in expected["refused"].as_object().unwrap() {
        let folder = root().join(case["folder"].as_str().unwrap());
        let got = timesheet::read_cut(&folder);
        let (built, ok) = match got {
            Ok(cut) => (format!("imported {} layers", cut.columns.len()), false),
            Err(refused) => {
                let mut facts: serde_json::Map<String, J> = case
                    .as_object()
                    .unwrap()
                    .iter()
                    .filter(|(k, _)| !["says", "folder", "refused", "notes"].contains(&k.as_str()))
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                facts.insert("refused".into(), case["refused"].clone());
                let mut got_facts = refused.refusal.facts.clone();
                got_facts.insert("refused".into(), J::from(refused.refusal.id.as_str()));
                let notes = sorted(refused.notes.iter().map(as_json));
                let ok = got_facts == facts
                    && notes == sorted(case["notes"].as_array().unwrap().iter().cloned());
                let mut built = format!("refused: `{}`", J::Object(got_facts));
                for n in refused.notes.iter().chain([&refused.refusal]) {
                    built.push_str(&format!("<br>note `{}`", as_json(n)));
                    seen.entry(n.id.as_str().to_string())
                        .or_insert_with(|| n.clone());
                }
                (built, ok)
            }
        };
        t.row(
            &format!("{name}: {}", case["says"].as_str().unwrap()),
            &built,
            ok,
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("What a person reads for each note (documents 25 and 28)");
    let d25 = fs::read_to_string(repo("Markdown/25_Test_Fixture_Catalog.md")).unwrap();
    let severities: BTreeMap<String, String> = d25
        .lines()
        .filter(|l| l.starts_with("| TIMESHEET_"))
        .map(|l| {
            let cells: Vec<&str> = l.split(" | ").collect();
            (cells[0][2..].to_string(), cells[1].to_string())
        })
        .collect();
    t.row(
        "every one of document 25's sixteen TIMESHEET IDs is said by some fixture",
        &format!("{} of {} said", seen.len(), severities.len()),
        seen.len() == 16 && seen.keys().eq(severities.keys()),
    );
    for (id, n) in &seen {
        let d = n.diagnostic();
        let wanted = severities
            .get(id)
            .map_or("(not in document 25)", String::as_str);
        let mut said = format!("{} {}: {}", d.severity, d.id, d.message);
        if let Some(r) = &d.remediation {
            said.push_str(&format!(" {r}"));
        }
        t.row(
            &format!("{id}, which document 25 makes {wanted}"),
            &said,
            d.severity.to_string() == wanted && d.id.in_catalog(),
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("Into a project (document 24)");
    let cut = timesheet::read_cut(&root().join("fx_xdts_040")).unwrap();
    let mut document = Document::new(Project::new(Id::new("project")));
    let empty = document.project().clone();
    let (comp_id, commands) = timesheet::commands(&cut, document.project(), &root());
    let applied = document.apply_all(commands).is_ok();
    let project = document.project().clone();
    let comp = project.composition(&comp_id);
    let layers: Vec<String> = comp.map_or_else(Vec::new, |c| {
        c.layers_in_order()
            .map(|l| {
                format!(
                    "{} on {} ({} exposures, {:?})",
                    l.name,
                    l.asset_id,
                    l.exposure_spans.len(),
                    l.timesheet.as_ref().map(|s| (&s.sheet, &s.column, s.track))
                )
            })
            .collect()
    });
    let timing_kept = comp.is_some_and(|c| {
        c.layers_in_order()
            .zip(&cut.columns)
            .all(|(l, col)| l.exposure_spans == col.spans && l.name == col.name)
    });
    t.row(
        "FX-XDTS-040 added to an empty project: three drawing sequences, and a composition of 48 frames at 24 a second with layers A, B and C, bottom first, each timed as read and carrying its timesheet record",
        &format!(
            "{}; {} assets; {} {}; layers {}",
            if applied { "applied" } else { "refused" },
            project.assets.len(),
            comp_id,
            comp.map_or(String::new(), |c| format!(
                "\"{}\" {} frames at {:?}",
                c.name, c.duration_frames, c.frame_rate
            )),
            layers.join(", ")
        ),
        applied
            && project.assets.len() == 3
            && comp.is_some_and(|c| c.duration_frames == 48 && c.len() == 3)
            && timing_kept,
    );
    let stored = project
        .assets
        .first()
        .map(|a| a.frames.get(&1).cloned().unwrap_or_default());
    t.row(
        "drawing paths are stored relative to the project's folder",
        &format!("A's drawing 1 at {stored:?}"),
        stored.as_deref() == Some("fx_xdts_040/A/A_0001.png"),
    );
    let one_step = document.undo().is_some() && document.project() == &empty;
    t.row(
        "and one undo takes the whole import away",
        if one_step {
            "the empty project"
        } else {
            "different"
        },
        one_step,
    );
    let again = timesheet::commands(&cut, &project, &root());
    t.row(
        "importing it a second time counts new IDs past the first",
        &format!("{}", again.0),
        again.0 == Id::new("comp-2"),
    );

    // -----------------------------------------------------------------------------------
    t.heading("The file (document 19)");
    let written = persist::to_json(&project, &persist::Preserved::default());
    let written_json: J = serde_json::from_str(&written).unwrap();
    let first = &written_json["compositions"][0]["layers"][0];
    t.row(
        "an imported layer is written with its timesheet record",
        &format!("`\"timesheet\": {}`", first["timesheet"]),
        first["timesheet"] == json!({"sheet": "fx_xdts_040.xdts", "column": "A", "track": 0}),
    );
    let reopened = persist::load_str(&written).expect("what this build wrote, it opens");
    t.row(
        "and it opens again as the same project",
        if reopened.document.project() == &project {
            "equal"
        } else {
            "different"
        },
        reopened.document.project() == &project,
    );
    let layer_count = written.matches("\"timesheet\"").count();
    t.row(
        "the key is written once per imported layer, three times",
        &format!("{layer_count}"),
        layer_count == 3,
    );
    let text_written = &written_json["compositions"][0]["sheet_text"];
    t.row(
        "its dialogue and camera columns are written as `sheet_text` (D-84c)",
        &format!("`{text_written}`"),
        *text_written
            == json!([
                {"kind": "dialogue", "name": "Dialogue", "track": 0, "entries": [
                    {"start_frame": 0, "end_frame_exclusive": 16, "text": ["MIKA", "Over here!"]}]},
                {"kind": "camera", "name": "Camera", "track": 0, "entries": [
                    {"start_frame": 20, "end_frame_exclusive": 48, "text": ["FOLLOW"]}]}
            ]),
    );
    let mut other = project.clone();
    if let Some(c) = other.compositions.first_mut() {
        c.sheet_text.push(SheetText {
            kind: "sound".into(),
            name: "SE".into(),
            track: 0,
            entries: vec![],
        });
    }
    let other_written = persist::to_json(&other, &persist::Preserved::default());
    let kept = persist::load_str(&other_written).is_ok_and(|r| r.document.project() == &other);
    t.row(
        "a column of a kind this build does not know is kept and saved back",
        if kept { "kept" } else { "lost or refused" },
        kept,
    );
    let broken = written.replacen(
        "\"end_frame_exclusive\": 16",
        "\"end_frame_exclusive\": 0",
        1,
    );
    let refused_text = persist::load_str(&broken).err().map(|d| d.id.as_str());
    t.row(
        "a text entry that ends before it starts is refused: PROJECT_SCHEMA_INVALID",
        &format!("{refused_text:?}"),
        refused_text == Some("PROJECT_SCHEMA_INVALID"),
    );
    let without = persist::to_json(&empty, &persist::Preserved::default());
    t.row(
        "a project with no text columns is written without the key",
        if without.contains("sheet_text") {
            "written"
        } else {
            "absent"
        },
        !without.contains("sheet_text"),
    );
    let bad = written.replacen("\"track\": 0", "\"track\": \"bottom\"", 1);
    let refused = persist::load_str(&bad).err().map(|d| d.id.as_str());
    t.row(
        "a record whose track is not a whole number is refused: PROJECT_SCHEMA_INVALID",
        &format!("{refused:?}"),
        refused == Some("PROJECT_SCHEMA_INVALID"),
    );

    // -----------------------------------------------------------------------------------
    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-28b_timesheet_table.md"), &t.out).unwrap();
    assert_eq!(
        t.passed, t.checks,
        "see verification/B-28b_timesheet_table.md"
    );
}
