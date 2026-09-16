//! Collect and package (D-61): one folder that opens from anywhere, with a manifest of every
//! file's size and SHA-256, and a check of a package against its manifest.
//!
//! `tools/package_reference.py` is the same rules written a second way, in Python; its output
//! under `Fixtures/packaging/` is what `tests/b15b_package.rs` holds this module to.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use crate::diagnostics::{Diagnostic, DiagnosticId, Severity};
use crate::model::{Asset, AssetKind, Project};
use crate::persist::{self, Preserved};
use crate::sha256;

pub const MANIFEST: &str = "package-manifest.json";
const FORMAT: &str = "anime-compositor-package";
const DEVICES: &[&str] = &[
    "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8",
    "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
];

/// D-61's naming rule: one path segment every supported file system accepts.
pub fn safe(name: &str) -> String {
    let name: String = name
        .chars()
        .map(|c| {
            if "<>:\"/\\|?*".contains(c) || (c as u32) < 32 {
                '_'
            } else {
                c
            }
        })
        .collect();
    let mut name = name.trim_end_matches(['.', ' ']).to_string();
    if name.is_empty() {
        name.push('_');
    }
    let stem = name.split('.').next().unwrap_or("").to_lowercase();
    if DEVICES.contains(&stem.as_str()) {
        name.insert(0, '_');
    }
    name
}

/// The first of `name`, `name-2`, `name-3`... not in `taken`, ignoring case; then taken.
fn unique(name: &str, taken: &mut HashSet<String>) -> String {
    let mut candidate = name.to_string();
    let mut n = 1;
    while taken.contains(&candidate.to_lowercase()) {
        n += 1;
        candidate = match name.rfind('.') {
            Some(dot) if dot > 0 => format!("{}-{n}{}", &name[..dot], &name[dot..]),
            _ => format!("{name}-{n}"),
        };
    }
    taken.insert(candidate.to_lowercase());
    candidate
}

/// An asset's stored paths with the frame numbers each stands for, in first-frame order.
fn stored_files(asset: &Asset) -> Vec<(&str, Vec<u32>)> {
    if asset.kind == AssetKind::Still {
        return asset.path.iter().map(|p| (p.as_str(), vec![])).collect();
    }
    let mut out: Vec<(&str, Vec<u32>)> = Vec::new();
    for (number, path) in &asset.frames {
        match out.iter_mut().find(|(p, _)| *p == path) {
            Some((_, numbers)) => numbers.push(*number),
            None => out.push((path, vec![*number])),
        }
    }
    out
}

/// A JSON string as Python's `json.dumps(..., ensure_ascii=False)` writes it.
fn quoted(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if (c as u32) < 32 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// A JSON value whose objects keep their key order, which `serde_json` here does not.
enum V {
    S(String),
    N(u64),
    B(bool),
    Null,
    A(Vec<V>),
    O(Vec<(&'static str, V)>),
}

impl V {
    fn s(s: &str) -> V {
        V::S(s.into())
    }

    /// Written as Python's `json.dumps(value, indent=2)` writes it.
    fn dump(&self, depth: usize, out: &mut String) {
        let pad = |out: &mut String, d: usize| {
            out.push('\n');
            out.push_str(&"  ".repeat(d));
        };
        match self {
            V::S(s) => out.push_str(&quoted(s)),
            V::N(n) => out.push_str(&n.to_string()),
            V::B(b) => out.push_str(&b.to_string()),
            V::Null => out.push_str("null"),
            V::A(items) if items.is_empty() => out.push_str("[]"),
            V::A(items) => {
                out.push('[');
                for (i, item) in items.iter().enumerate() {
                    out.push_str(if i == 0 { "" } else { "," });
                    pad(out, depth + 1);
                    item.dump(depth + 1, out);
                }
                pad(out, depth);
                out.push(']');
            }
            V::O(fields) => {
                out.push('{');
                for (i, (key, value)) in fields.iter().enumerate() {
                    out.push_str(if i == 0 { "" } else { "," });
                    pad(out, depth + 1);
                    out.push_str(&quoted(key));
                    out.push_str(": ");
                    value.dump(depth + 1, out);
                }
                pad(out, depth);
                out.push('}');
            }
        }
    }
}

/// Write `project` as a package into `dest`, which must be empty or absent.
///
/// `project_file` is where the open project is saved, or `None` if it never was; its folder is
/// what stored paths are relative to. Nothing about the open project is changed.
pub fn collect(
    project: &Project,
    preserved: &Preserved,
    project_file: Option<&Path>,
    dest: &Path,
) -> Result<(), Diagnostic> {
    collect_limited(project, preserved, project_file, dest, None)
}

/// [`collect`], made to fail after writing `file_limit` files, for FX-PACK-005. `None` is an
/// ordinary collect.
pub fn collect_limited(
    project: &Project,
    preserved: &Preserved,
    project_file: Option<&Path>,
    dest: &Path,
    file_limit: Option<usize>,
) -> Result<(), Diagnostic> {
    let mut partial = dest.as_os_str().to_os_string();
    partial.push(".partial");
    let partial = PathBuf::from(partial);
    let not_empty = fs::read_dir(dest).is_ok_and(|mut d| d.next().is_some());
    if not_empty || partial.exists() || dest.is_file() {
        return Err(Diagnostic::new(
            DiagnosticId::PackageDestinationNotEmpty,
            Severity::Error,
            "Files were not collected: the chosen folder is not empty.",
            format!(
                "{} holds something, or {} exists. Nothing was written.",
                dest.display(),
                partial.display()
            ),
        )
        .with_remediation("Choose an empty folder, or a new one."));
    }

    let dir = project_file.and_then(Path::parent);
    let name = project_file
        .and_then(Path::file_name)
        .map_or("project.json".into(), |n| n.to_string_lossy().into_owned());
    let resolve = |stored: &str| dir.map_or(PathBuf::from(stored), |d| d.join(stored));

    let mut packaged = project.clone();
    let mut folders = HashSet::new();
    let mut assets = Vec::new();
    let mut to_copy = Vec::new();
    for (asset, out) in project.assets.iter().zip(&mut packaged.assets) {
        let folder = unique(&safe(asset.id.as_str()), &mut folders);
        let mut names = HashSet::new();
        let mut places = HashMap::new();
        let mut files = Vec::new();
        for (stored, frames) in stored_files(asset) {
            let file_name = stored.rsplit(['/', '\\']).next().unwrap_or(stored);
            let path = format!("media/{folder}/{}", unique(&safe(file_name), &mut names));
            let source = resolve(stored);
            let data = if source.is_file() {
                fs::read(&source).ok()
            } else {
                None
            };
            let status = match (&data, asset.redistribute) {
                (None, _) => "missing",
                (Some(_), true) => "copied",
                (Some(_), false) => "excluded",
            };
            if status == "copied" {
                to_copy.push((source, path.clone()));
            }
            files.push(V::O(vec![
                (
                    "frames",
                    V::A(frames.into_iter().map(|n| V::N(n.into())).collect()),
                ),
                ("path", V::s(&path)),
                ("status", V::s(status)),
                (
                    "bytes",
                    data.as_ref().map_or(V::Null, |d| V::N(d.len() as u64)),
                ),
                (
                    "sha256",
                    data.as_deref().map_or(V::Null, |d| V::S(sha256::hex(d))),
                ),
            ]));
            places.insert(stored.to_string(), path);
        }
        let moved = |p: &mut String| *p = places[p.as_str()].clone();
        out.path.iter_mut().for_each(moved);
        out.frames.values_mut().for_each(moved);

        let used_by = project
            .compositions
            .iter()
            .flat_map(|c| {
                c.layers_in_order()
                    .filter(|l| l.asset_id == asset.id)
                    .map(|l| {
                        V::O(vec![
                            ("composition", V::s(c.id.as_str())),
                            ("layer", V::s(l.id.as_str())),
                        ])
                    })
            })
            .collect();
        assets.push(V::O(vec![
            ("id", V::s(asset.id.as_str())),
            ("name", V::s(&asset.name)),
            ("kind", V::s(asset.kind.as_str())),
            ("redistribute", V::B(asset.redistribute)),
            ("used_by", V::A(used_by)),
            ("files", V::A(files)),
        ]));
    }
    let mut manifest = String::new();
    V::O(vec![
        ("format", V::s(FORMAT)),
        ("version", V::N(1)),
        ("project", V::s(&name)),
        ("assets", V::A(assets)),
    ])
    .dump(0, &mut manifest);
    manifest.push('\n');

    let text = persist::to_json(&packaged, preserved);
    let mut written = 0;
    let outcome = (|| -> io::Result<()> {
        let mut step = || {
            written += 1;
            match file_limit {
                Some(limit) if written > limit => Err(io::Error::other(format!(
                    "stopped by the test after {limit} files"
                ))),
                _ => Ok(()),
            }
        };
        fs::create_dir_all(&partial)?;
        for (source, path) in &to_copy {
            step()?;
            let target = partial.join(path);
            fs::create_dir_all(target.parent().expect("under media"))?;
            fs::copy(source, target)?;
        }
        step()?;
        fs::write(partial.join(&name), &text)?;
        step()?;
        fs::write(partial.join(MANIFEST), &manifest)?;
        if dest.is_dir() {
            fs::remove_dir(dest)?; // empty, checked above
        }
        fs::rename(&partial, dest)
    })();
    outcome.map_err(|e| {
        let _ = fs::remove_dir_all(&partial);
        Diagnostic::new(
            DiagnosticId::PackageWriteFailed,
            Severity::Error,
            "Files were not collected: the package could not be written.",
            format!("Writing {} failed: {e}", dest.display()),
        )
        .with_remediation(
            "The partly written folder was removed and your project is unchanged. Try another \
             folder, or free some space.",
        )
    })
}

/// What checking found for one listed file.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Answer {
    Ok,
    Changed,
    Missing,
    Excluded,
    Unverified,
}

impl Answer {
    pub fn as_str(self) -> &'static str {
        match self {
            Answer::Ok => "ok",
            Answer::Changed => "changed",
            Answer::Missing => "missing",
            Answer::Excluded => "excluded",
            Answer::Unverified => "unverified",
        }
    }
}

pub struct Row {
    pub path: String,
    pub answer: Answer,
    /// `None` for `ok`.
    pub diagnostic: Option<Diagnostic>,
}

/// Check the package the project file `project_file` sits in against its manifest.
pub fn check(project_file: &Path) -> Result<Vec<Row>, Diagnostic> {
    let root = project_file.parent().unwrap_or(Path::new(""));
    let at = root.join(MANIFEST);
    let invalid = |why: String| {
        Diagnostic::new(
            DiagnosticId::PackageManifestInvalid,
            Severity::Error,
            "This project cannot be checked: its package manifest is missing or unreadable.",
            format!("{}: {why}", at.display()),
        )
        .with_remediation("Only a folder made by File > Collect Files... can be checked.")
    };
    let text = fs::read_to_string(&at).map_err(|e| invalid(e.to_string()))?;
    let manifest: J = serde_json::from_str(&text).map_err(|e| invalid(e.to_string()))?;
    if manifest["format"] != FORMAT || manifest["version"] != 1 {
        return Err(invalid("not a version 1 package manifest".into()));
    }
    let files = manifest["assets"]
        .as_array()
        .ok_or_else(|| invalid("no asset list".into()))?
        .iter()
        .map(|a| {
            a["files"]
                .as_array()
                .ok_or_else(|| invalid("an asset has no file list".into()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut rows = Vec::new();
    for f in files.into_iter().flatten() {
        let path = f["path"]
            .as_str()
            .filter(|p| !p.split('/').any(|s| s == ".." || s.is_empty()))
            .ok_or_else(|| invalid("a file has no usable path".into()))?;
        let data = fs::read(root.join(path)).ok();
        let (answer, id, severity, message) = match (&data, f["sha256"].as_str()) {
            (None, _) if f["status"] == "excluded" => (
                Answer::Excluded,
                DiagnosticId::PackageMediaExcluded,
                Severity::Info,
                "Left out on purpose: its media may not be passed on. Put your own copy here.",
            ),
            (None, _) => (
                Answer::Missing,
                DiagnosticId::MediaMissing,
                Severity::Warning,
                "Missing: this drawing is not in the package.",
            ),
            (Some(_), None) => (
                Answer::Unverified,
                DiagnosticId::PackageFileUnverified,
                Severity::Info,
                "Present, but it was missing when the package was made, so nothing can vouch for it.",
            ),
            (Some(d), Some(hash))
                if f["bytes"].as_u64() == Some(d.len() as u64) && sha256::hex(d) == hash =>
            {
                rows.push(Row { path: path.into(), answer: Answer::Ok, diagnostic: None });
                continue;
            }
            (Some(_), Some(_)) => (
                Answer::Changed,
                DiagnosticId::PackageFileChanged,
                Severity::Error,
                "Changed: this file is not the one that was packaged.",
            ),
        };
        rows.push(Row {
            path: path.into(),
            answer,
            diagnostic: Some(Diagnostic::new(id, severity, message, path)),
        });
    }
    Ok(rows)
}
