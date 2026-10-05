//! D-253 (a cut status) and D-254 (label colours on compositions and footage).
//!
//! Writes `verification/D-253_D-254_roundtrip_table.md`.
//!
//! The owner accepted both on 2026-10-02 with the same three rows each:
//!
//! 1. A project with a status, and with labels on a composition and a footage item, saves them and
//!    reads them back.
//! 2. Today's projects open and save unchanged, byte for byte. Every project under `Fixtures/`
//!    that opens is saved, and the saved text of all of them is hashed. The hash below was taken
//!    on the build before either change (e395d8b).
//! 3. A project carrying the new lines keeps them when opened and saved, which today's build does
//!    too, because it keeps lines it does not know.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::command::Command;
use anime_compositor::persist::Preserved;
use anime_compositor::{persist, sha256};

/// Taken on e395d8b, before either change: how many fixture projects open, and the SHA-256 of
/// their saved text, one after another in path order.
const FIXTURES_OPENED: usize = 2297;
const FIXTURES_SAVED: &str = "aeadb2e51777a56efb28e3113b652eaa65c03d10cf2ba66b8d4798f0c86dae2d";

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn projects(dir: &Path, found: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            projects(&path, found);
        } else if path.extension().is_some_and(|e| e == "json") {
            found.push(path);
        }
    }
}

/// The reference shot with a status in its title block and a label on its composition and on
/// its first footage item, written as text, the way a later build would write them.
fn with_new_lines() -> String {
    let mut root: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(repo("verification/B-08a_project.json")).unwrap()).unwrap();
    root["compositions"][0]["sheet_details"] = serde_json::json!({ "cut": "012", "status": "retake" });
    root["compositions"][0]["label"] = serde_json::json!(5);
    root["assets"][0]["label"] = serde_json::json!(2);
    root.to_string()
}

#[test]
fn d253_d254_status_and_labels_round_trip() {
    let mut rows: Vec<(String, String, String)> = Vec::new();

    // Row 1, through the commands the page sends.
    let mut doc = persist::load(&repo("verification/B-08a_project.json")).unwrap_or_else(|d| panic!("{}", d.message)).document;
    let comp = doc.project().compositions[0].clone();
    let asset = doc.project().assets[0].id.clone();
    let mut details = comp.sheet_details.clone();
    details.status = "retake".into();
    doc.apply(Command::SetCompositionSettings {
        composition: comp.id.clone(),
        name: comp.name.clone(),
        width: comp.width,
        height: comp.height,
        frame_rate: comp.frame_rate,
        duration_frames: comp.duration_frames,
        sheet_details: details,
        background_color: None,
    })
    .unwrap_or_else(|d| panic!("{}", d.message));
    doc.apply(Command::SetItemLabel { item: comp.id.clone(), label: 5 }).unwrap_or_else(|d| panic!("{}", d.message));
    doc.apply(Command::SetItemLabel { item: asset.clone(), label: 2 }).unwrap_or_else(|d| panic!("{}", d.message));
    rows.push((
        "There is no label colour 9".into(),
        "refused".into(),
        if doc.apply(Command::SetItemLabel { item: asset, label: 9 }).is_err() { "refused" } else { "taken" }.into(),
    ));
    let back = persist::load_str(&persist::to_json(doc.project(), &Preserved::none())).unwrap_or_else(|d| panic!("{}", d.message));
    let back = back.document.project();
    rows.push(("A status set in the app reads back after save and open".into(), "retake".into(), back.compositions[0].sheet_details.status.clone()));
    rows.push(("A composition's label reads back".into(), "5".into(), back.compositions[0].label.to_string()));
    rows.push(("A footage item's label reads back".into(), "2".into(), back.assets[0].label.to_string()));
    let fresh = persist::load(&repo("verification/B-08a_project.json")).unwrap_or_else(|d| panic!("{}", d.message));
    let fresh = fresh.document.project();
    rows.push((
        "A project without them reads as Not started, with no labels".into(),
        "\"\" 0 0".into(),
        format!("{:?} {} {}", fresh.compositions[0].sheet_details.status, fresh.compositions[0].label, fresh.assets[0].label),
    ));

    // Row 2.
    let mut found = Vec::new();
    projects(&repo("Fixtures"), &mut found);
    found.sort();
    // The files that were there on e395d8b; fixtures added since (D-318 on) are new behaviour.
    let pinned = include_str!("fixtures_e395d8b.txt");
    found.retain(|p| {
        let rel = p.strip_prefix(repo("")).unwrap().to_string_lossy().replace(std::path::MAIN_SEPARATOR, "/");
        pinned.lines().any(|l| l == rel)
    });
    let mut saved = String::new();
    let mut opened = 0;
    for path in &found {
        let Ok(text) = fs::read_to_string(path) else { continue };
        if let Ok(loaded) = persist::load_str(&text) {
            opened += 1;
            saved.push_str(&persist::to_json(loaded.document.project(), &loaded.preserved));
        }
    }
    rows.push(("Fixture projects that open".into(), FIXTURES_OPENED.to_string(), opened.to_string()));
    rows.push((
        "Their saved text, all of it, is byte for byte as before (SHA-256)".into(),
        FIXTURES_SAVED.into(),
        sha256::hex(saved.as_bytes()),
    ));

    // Row 3.
    let kept = persist::load_str(&with_new_lines()).unwrap_or_else(|d| panic!("{}", d.message));
    let written = persist::to_json(kept.document.project(), &kept.preserved);
    let again: serde_json::Value = serde_json::from_str(&written).unwrap();
    rows.push((
        "A status written by a later build is kept through open and save".into(),
        "\"retake\"".into(),
        again["compositions"][0]["sheet_details"]["status"].to_string(),
    ));
    rows.push((
        "A composition's label is kept through open and save".into(),
        "5".into(),
        again["compositions"][0]["label"].to_string(),
    ));
    rows.push((
        "A footage item's label is kept through open and save".into(),
        "2".into(),
        again["assets"][0]["label"].to_string(),
    ));

    let mut md = String::from("# D-253 and D-254: a cut status and labels, through save and open\n\nWritten by `tests/d253_d254_roundtrip.rs`. The fixture hash was taken on the build before either change (e395d8b).\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for (check, e, a) in &rows {
        let _ = writeln!(md, "| {check} | {e} | {a} | {} |", if e == a { "pass" } else { "FAIL" });
    }
    fs::write(repo("verification/D-253_D-254_roundtrip_table.md"), md).unwrap();
    let failed: Vec<_> = rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(failed.is_empty(), "{failed:?}");
}
