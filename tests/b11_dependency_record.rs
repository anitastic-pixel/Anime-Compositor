//! B-11's first piece: the dependency and licence record is checked against the build it claims
//! to describe.
//!
//! Writes `verification/B-11_record_table.md`.
//!
//! # What this is for
//!
//! `docs/DEPENDENCIES.md` and the archived licence texts under `Licenses/` are
//! the artifact. A record like that is worth exactly as much as its agreement with the build,
//! and a hand-maintained one stops agreeing the first time somebody adds a crate. Document 10
//! is explicit about the failure it wants avoided: "Generate a software bill of materials from
//! the final build inputs rather than a guessed list."
//!
//! So this asks cargo what the build resolves and requires that it and the record say the same
//! thing in both directions: no crate the build uses missing from the record, no crate in the
//! record the build does not use, the same version for each, and an archived licence directory
//! for every one of them.
//!
//! # Why it asks cargo rather than reading `Cargo.lock` alone
//!
//! It used to read `Cargo.lock`, and that was right while every crate in the graph was portable.
//! The shell changed it. `Cargo.lock` is the union over every platform cargo could ever resolve
//! for, so it now lists the GTK, Cocoa and Android stacks — 435 crates, of which 264 compile on
//! the only platform this project supports. A record naming the other 171 would describe a
//! program nobody has, and archiving their licence texts would claim a distribution carries code
//! it does not.
//!
//! So the resolved set comes from `cargo metadata --filter-platform`, and `Cargo.lock` is still
//! read and still checked: every crate the build resolves must appear in the committed lock file
//! at the same version. The lock file remains the reproducibility record; it is simply no longer
//! the same question as "what is in this build".
//!
//! # Where the expected values come from
//!
//! The workspace manifests, which are written by hand and name four dependencies: `png`, `rayon`
//! and `serde_json` for the core, `tauri` for the shell, with `tauri-build` as its build-time
//! half. Everything else in the graph arrived underneath one of those. The count is not a
//! hand-derived value and is not asserted as one — the two-directional agreement is what is
//! checked, and it holds at any count.
//!
//! # What this deliberately does not do
//!
//! It does not read a licence and decide anything. Document 10 reserves that: "legal
//! conclusions requiring professional judgment should be recorded by the appropriate reviewer".
//! It checks that the text was archived and that the record names a licence, and stops there.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// The only platform this project builds for: ADR-001, and CI runs `windows-latest`. Naming it
/// here rather than asking for the host triple is deliberate — the record describes the supported
/// build, and it should say the same thing wherever this test is run from.
const PLATFORM: &str = "x86_64-pc-windows-msvc";

// ---------------------------------------------------------------------------------------
// Reporting
// ---------------------------------------------------------------------------------------

struct Row {
    check: String,
    expected: String,
    actual: String,
}

impl Row {
    fn pass(&self) -> bool {
        self.expected == self.actual
    }
}

#[derive(Default)]
struct Report {
    rows: Vec<Row>,
}

impl Report {
    fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
        self.rows.push(Row {
            check: check.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
        });
    }
}

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn read(rel: &str) -> String {
    let path = repo(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

// ---------------------------------------------------------------------------------------
// The two things being compared
// ---------------------------------------------------------------------------------------

/// Every crate the build resolves for `PLATFORM`, but this workspace's own: name to the versions
/// of it in the graph.
///
/// `--locked` is passed so that a stale `Cargo.lock` fails here loudly instead of being quietly
/// updated by the act of checking it.
fn resolved() -> BTreeMap<String, BTreeSet<String>> {
    let out = Command::new(env!("CARGO"))
        .args([
            "metadata",
            "--format-version",
            "1",
            "--locked",
            "--filter-platform",
            PLATFORM,
        ])
        .current_dir(repo(""))
        .output()
        .expect("run cargo metadata");
    assert!(
        out.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let meta: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("cargo metadata is not JSON");
    let mut found: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for p in meta["packages"].as_array().expect("packages") {
        // A package with no `source` is one of this workspace's own crates, not a dependency.
        if p["source"].is_null() {
            continue;
        }
        found
            .entry(p["name"].as_str().expect("name").to_string())
            .or_default()
            .insert(p["version"].as_str().expect("version").to_string());
    }
    found
}

/// Every `[[package]]` in `Cargo.lock` but this workspace's own: name to the versions of it the
/// build resolved. It is a set and not a single version because a graph can legitimately carry
/// two majors of the same crate at once, and this one does: `miniz_oxide` 0.9.1 and 0.8.9 are
/// both here, reached by different dependants.
fn locked() -> BTreeMap<String, BTreeSet<String>> {
    let lock = read("Cargo.lock");
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for block in lock.split("[[package]]").skip(1) {
        let field = |key: &str| -> Option<String> {
            block.lines().find_map(|line| {
                let line = line.trim();
                let head = format!("{key} = \"");
                line.strip_prefix(&head)
                    .and_then(|rest| rest.strip_suffix('"'))
                    .map(str::to_string)
            })
        };
        if let (Some(name), Some(version)) = (field("name"), field("version")) {
            if name != "anime_compositor" && name != "anime_compositor_app" {
                out.entry(name).or_default().insert(version);
            }
        }
    }
    out
}

/// Every row of the record's bill of materials. A row looks like
/// `| `png` | 0.18.1 | MIT OR Apache-2.0 | direct | linked | https://... | `hash…` |`.
struct Rows {
    /// Crate name to the versions the record lists for it.
    versions: BTreeMap<String, BTreeSet<String>>,
    /// Name and version to the licence and role that row declares.
    cells: BTreeMap<(String, String), (String, String)>,
}

fn recorded() -> Rows {
    let text = read("docs/DEPENDENCIES.md");
    let mut rows = Rows {
        versions: BTreeMap::new(),
        cells: BTreeMap::new(),
    };
    for line in text.lines() {
        let cells: Vec<&str> = line.trim().split('|').map(str::trim).collect();
        // A leading and trailing empty cell come from the outer pipes.
        if cells.len() != 9 || !cells[1].starts_with('`') {
            continue;
        }
        let name = cells[1].trim_matches('`').to_string();
        if name == "Crate" {
            continue;
        }
        let version = cells[2].to_string();
        rows.versions
            .entry(name.clone())
            .or_default()
            .insert(version.clone());
        rows.cells.insert(
            (name, version),
            (cells[3].to_string(), cells[4].to_string()),
        );
    }
    rows
}

fn joined(items: &[String]) -> String {
    if items.is_empty() {
        "none".to_string()
    } else {
        items.join(", ")
    }
}

// ---------------------------------------------------------------------------------------
// The test
// ---------------------------------------------------------------------------------------

#[test]
fn b11_the_dependency_record_describes_the_build_it_claims_to() {
    let mut report = Report::default();
    let build = resolved();
    let lock = locked();
    let record = recorded();

    // ---- the record parses at all ------------------------------------------------------------
    report.check(
        "the record's bill of materials is a table this check can read",
        "the table parses into rows: true",
        format!("the table parses into rows: {}", record.versions.len() > 1),
    );

    // ---- neither side has anything the other lacks -------------------------------------------
    let missing: Vec<String> = build
        .keys()
        .filter(|n| !record.versions.contains_key(*n))
        .cloned()
        .collect();
    report.check(
        "every crate the build resolves has a row in the record",
        "none missing from the record",
        if missing.is_empty() {
            "none missing from the record".to_string()
        } else {
            format!("missing from the record: {}", joined(&missing))
        },
    );

    let phantom: Vec<String> = record
        .versions
        .keys()
        .filter(|n| !build.contains_key(*n))
        .cloned()
        .collect();
    report.check(
        "the record names no crate the build does not use",
        "no rows without a crate",
        if phantom.is_empty() {
            "no rows without a crate".to_string()
        } else {
            format!("rows without a crate: {}", joined(&phantom))
        },
    );

    // A crate can appear at two versions at once, so this compares the whole set per crate. That
    // catches a stale version, a version the build no longer uses, and a second major that was
    // pulled in without anybody noticing, which is the one a single-version check would miss.
    let wrong: Vec<String> = build
        .iter()
        .filter_map(|(name, versions)| {
            let listed = record.versions.get(name)?;
            (listed != versions).then(|| {
                format!(
                    "{name}: record says {}, the build uses {}",
                    listed.iter().cloned().collect::<Vec<_>>().join(" and "),
                    versions.iter().cloned().collect::<Vec<_>>().join(" and ")
                )
            })
        })
        .collect();
    report.check(
        "every row carries the exact version the build resolved, not a range or an older one",
        "no version disagrees",
        if wrong.is_empty() {
            "no version disagrees".to_string()
        } else {
            joined(&wrong)
        },
    );

    // ---- document 10's "archive the exact license" -------------------------------------------
    let unarchived: Vec<String> = build
        .iter()
        .flat_map(|(name, versions)| versions.iter().map(move |v| (name, v)))
        .filter(|(name, version)| {
            let dir = repo("Licenses").join(format!("{name}-{version}"));
            !dir.is_dir()
                || fs::read_dir(&dir)
                    .map(|mut d| d.next().is_none())
                    .unwrap_or(true)
        })
        .map(|(name, version)| format!("{name}-{version}"))
        .collect();
    report.check(
        "every crate's own licence text is archived in this repository, not merely named",
        "every crate has an archived licence",
        if unarchived.is_empty() {
            "every crate has an archived licence".to_string()
        } else {
            format!("nothing archived for: {}", joined(&unarchived))
        },
    );

    // ---- the committed lock file still describes this build ----------------------------------
    // `Cargo.lock` is why a checkout months from now resolves what this one did. It holds more
    // than this build uses, because it covers every platform, but it must hold all of this build.
    let unlocked: Vec<String> = build
        .iter()
        .filter_map(|(name, versions)| {
            let locked = lock.get(name)?;
            versions
                .iter()
                .find(|v| !locked.contains(*v))
                .map(|v| format!("{name} {v}"))
                .or(None)
        })
        .chain(
            build
                .keys()
                .filter(|n| !lock.contains_key(*n))
                .map(|n| format!("{n}, not in the lock file at all")),
        )
        .collect();
    report.check(
        "every crate the build resolves is in the committed lock file at that version",
        "the lock file covers the build",
        if unlocked.is_empty() {
            "the lock file covers the build".to_string()
        } else {
            joined(&unlocked)
        },
    );

    let unlicensed: Vec<String> = record
        .cells
        .iter()
        .filter(|(_, (licence, _))| licence.is_empty() || licence.contains("none stated"))
        .map(|((name, version), _)| format!("{name}-{version}"))
        .collect();
    report.check(
        "every row names a licence",
        "every row names a licence",
        if unlicensed.is_empty() {
            "every row names a licence".to_string()
        } else {
            format!("no licence named for: {}", joined(&unlicensed))
        },
    );

    // ---- the ones chosen on purpose ----------------------------------------------------------
    // The workspace manifests are written by hand and name exactly these. `tauri-build` is
    // `tauri`'s build-time half and is asked for by name in `app/Cargo.toml`, so it counts as
    // direct even though nothing links it. `tauri-plugin-dialog` arrived with B-09's Open and
    // Save dialogs, and this line is how a dependency added quietly stops being quiet.
    let mut direct: Vec<String> = record
        .cells
        .iter()
        .filter(|(_, (_, role))| role == "direct")
        .map(|((name, _), _)| name.clone())
        .collect();
    direct.sort();
    // Two majors of `png` are in the graph at once, so the same name arrives twice. The question
    // here is which crates were asked for by name, not how many rows carry the mark.
    direct.dedup();
    report.check(
        "the record marks as direct exactly the dependencies the manifests ask for",
        "png, rayon, serde_json, tauri, tauri-build, tauri-plugin-dialog",
        joined(&direct),
    );

    // ---- this project's own licence, D-31 ------------------------------------------------------
    // The record checks everyone else's terms; this is the row that checks ours. Until D-31 was
    // decided the repository carried no licence at all, which put the code under exclusive
    // copyright - the safe state, but not the one ADR-010 describes.
    let manifest = read("Cargo.toml");
    let declared = manifest
        .lines()
        .find_map(|l| l.trim().strip_prefix("license = \""))
        .and_then(|r| r.strip_suffix('"'))
        .unwrap_or("no license field");
    report.check(
        "the crate declares the licence D-31 chose",
        "MIT OR Apache-2.0",
        declared,
    );

    // A declared licence with no text beside it is a claim, not a licence.
    let missing: Vec<String> = ["LICENSE-MIT", "LICENSE-APACHE"]
        .iter()
        .filter(|f| !repo(f).is_file())
        .map(|f| f.to_string())
        .collect();
    report.check(
        "both licence texts the declaration names are in the repository",
        "both present",
        if missing.is_empty() {
            "both present".to_string()
        } else {
            format!("missing: {}", joined(&missing))
        },
    );

    // ---- the comparison can fail -------------------------------------------------------------
    // If the two sides were being compared against themselves, a crate that exists in neither
    // would appear to be in both.
    report.check(
        "the comparison can fail: a crate that is in neither file is reported as in neither",
        "in the build: false, in the record: false",
        format!(
            "in the build: {}, in the record: {}",
            build.contains_key("ffmpeg-sys"),
            record.versions.contains_key("ffmpeg-sys")
        ),
    );

    write_report(&report, &build);
    let failed: Vec<&Row> = report.rows.iter().filter(|r| !r.pass()).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed, first: {} (expected {}, got {}); see verification/B-11_record_table.md",
        failed.len(),
        report.rows.len(),
        failed[0].check,
        failed[0].expected,
        failed[0].actual
    );
}

fn write_report(report: &Report, build: &BTreeMap<String, BTreeSet<String>>) {
    let passed = report.rows.iter().filter(|r| r.pass()).count();
    let mut out = String::new();
    out.push_str("# B-11 dependency record, checked against the build\n\n");
    out.push_str(
        "The artifact is `docs/DEPENDENCIES.md` and the archived licence texts under \
         `Licenses/`. This is the check that keeps them true: it asks cargo what the build \
         resolves for the one platform this project supports, and requires that the answer and \
         the record agree in both directions. Produced by \
         `tests/b11_dependency_record.rs`.\n\n",
    );
    out.push_str(&format!(
        "The build currently resolves **{} dependencies** beneath the four the workspace \
         manifests name. `Cargo.lock` lists more, because it covers every platform cargo could \
         resolve for; the count here is what compiles on `{PLATFORM}`.\n\n",
        build.values().map(BTreeSet::len).sum::<usize>()
    ));
    out.push_str(
        "This check reads no licence and decides nothing about one. Document 10 reserves that \
         for a reviewer, and there has not been one.\n\n",
    );
    out.push_str("## Checks\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for row in &report.rows {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            row.check,
            row.expected,
            row.actual,
            if row.pass() { "pass" } else { "**FAIL**" }
        ));
    }
    out.push_str(&format!(
        "\n**{passed} of {} checks pass.**\n",
        report.rows.len()
    ));
    let path = repo("verification/B-11_record_table.md");
    fs::write(&path, out).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
}
