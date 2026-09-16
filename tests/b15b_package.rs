//! B-15b: collecting a shot into a package, and checking a package (D-61, T-13).
//!
//! Every expected value comes from `Fixtures/packaging/`, which `tools/package_reference.py`
//! wrote from D-61 in Python with Python's own SHA-256. This test collects the same shot with
//! the build, compares, moves the package somewhere else and renders it there, and runs the
//! six check situations and the refusals of document 25's FX-PACK-001 to 006.
//!
//! Writes `verification/B-15b_package_table.md`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::command::Command;
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::model::{Id, Project};
use anime_compositor::package::{self, Row};
use anime_compositor::persist;
use anime_compositor::sha256;

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// Every file under `dir`, as package paths with forward slashes, sorted.
fn tree(dir: &Path, prefix: &str, out: &mut Vec<String>) {
    for entry in fs::read_dir(dir).expect("read a folder") {
        let entry = entry.unwrap();
        let name = format!("{prefix}{}", entry.file_name().to_string_lossy());
        if entry.path().is_dir() {
            tree(&entry.path(), &format!("{name}/"), out);
        } else {
            out.push(name);
        }
    }
    out.sort();
}

fn frames(project: &Project, root: &Path, comp: &str) -> Vec<Vec<f32>> {
    let comp = Id::new(comp);
    let c = project.compositions.iter().find(|c| c.id == comp).unwrap();
    (c.start_frame..c.start_frame + c.duration_frames as i32)
        .map(|f| {
            let mut log = FrameLog::new(3);
            render_frame(project, &comp, f, root, 128, &mut log)
                .expect("render")
                .data()
                .to_vec()
        })
        .collect()
}

fn answers(rows: &[Row]) -> Vec<(String, String)> {
    rows.iter()
        .map(|r| (r.path.clone(), r.answer.as_str().to_string()))
        .collect()
}

/// Rows other than `ok`, as document 25 prints them.
fn odd(rows: &[(String, String)]) -> String {
    let odd: Vec<String> = rows
        .iter()
        .filter(|(_, a)| a != "ok")
        .map(|(p, a)| format!("{}: {a}", p.split_once('/').unwrap().1))
        .collect();
    if odd.is_empty() {
        "none".into()
    } else {
        odd.join("; ")
    }
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

#[test]
fn b15b_package() {
    let source_file = repo("Fixtures/packaging/source/shot.json");
    let expected_manifest = fs::read(repo("Fixtures/packaging/expected_manifest.json")).unwrap();
    let expected: J = serde_json::from_slice(
        &fs::read(repo("Fixtures/packaging/expected_package.json")).unwrap(),
    )
    .unwrap();
    let scratch = repo("target/b15b-scratch");
    let _ = fs::remove_dir_all(&scratch);
    fs::create_dir_all(&scratch).unwrap();
    let loaded = persist::load(&source_file).expect("the fixture shot loads");
    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };

    // FX-PACK-006 is set up first: the open shot has one unsaved, undoable change.
    let mut document = loaded.document;
    let main = Id::new("comp-main");
    document
        .apply(Command::RenameLayer {
            composition: main.clone(),
            layer_id: Id::new("layer-bg"),
            name: "renamed, not saved".into(),
        })
        .unwrap();
    let before = (
        document.project().clone(),
        document.undo_depth(),
        document.is_dirty(),
        fs::read(&source_file).unwrap(),
    );
    // Collecting what the window holds would carry the rename; the fixture is the shot as
    // saved, so the package is made from the saved shot and the check is that the window's
    // copy is left alone.
    let saved = persist::load(&source_file).unwrap();
    let dest = scratch.join("package");
    let collected = package::collect(
        saved.document.project(),
        &saved.preserved,
        Some(&source_file),
        &dest,
    );
    assert!(collected.is_ok(), "{collected:?}");
    package::collect(
        document.project(),
        &loaded.preserved,
        Some(&source_file),
        &scratch.join("from-window"),
    )
    .unwrap();

    // FX-PACK-001
    t.heading("FX-PACK-001: what is in the package");
    let manifest = fs::read(dest.join(package::MANIFEST)).unwrap();
    t.row(
        "`package-manifest.json` is the expected file, byte for byte",
        &format!("{} bytes", manifest.len()),
        manifest == expected_manifest,
    );
    let listed: J = serde_json::from_slice(&manifest).unwrap();
    let folders: Vec<String> = listed["assets"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| {
            a["files"][0]["path"]
                .as_str()
                .unwrap()
                .split('/')
                .nth(1)
                .unwrap()
                .to_string()
        })
        .collect();
    let want: Vec<String> = serde_json::from_value(expected["folders"].clone()).unwrap();
    t.row(
        "Each asset's folder, in project order",
        &folders
            .iter()
            .map(|f| format!("`{f}`"))
            .collect::<Vec<_>>()
            .join(", "),
        folders == want,
    );
    let mut files = Vec::new();
    tree(&dest.join("media"), "media/", &mut files);
    let want: Vec<String> = serde_json::from_value(expected["copied"].clone()).unwrap();
    t.row(
        "The drawings copied, and nothing else under `media`",
        &format!(
            "{} files: {}",
            files.len(),
            files
                .iter()
                .map(|f| format!("`{f}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        files == want,
    );
    let mut top = Vec::new();
    for e in fs::read_dir(&dest).unwrap() {
        top.push(e.unwrap().file_name().to_string_lossy().into_owned());
    }
    top.sort();
    t.row(
        "Beside `media`: the project under its own name and the manifest",
        &top.join(", "),
        top == ["media", "package-manifest.json", "shot.json"],
    );
    let (mut same, mut hashed) = (true, 0);
    for asset in listed["assets"].as_array().unwrap() {
        for f in asset["files"].as_array().unwrap() {
            if let Ok(d) = fs::read(dest.join(f["path"].as_str().unwrap())) {
                same &= f["sha256"].as_str() == Some(&sha256::hex(&d));
                hashed += 1;
            }
        }
    }
    t.row(
        "The build's SHA-256 of every copied file is the one Python wrote",
        &format!(
            "{hashed} files, {}",
            if same { "all agree" } else { "they differ" }
        ),
        same && hashed == 10,
    );
    let fips = sha256::hex(b"")
        == "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        && sha256::hex(b"abc")
            == "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        && sha256::hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")
            == "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        && sha256::hex(&vec![b'a'; 1_000_000])
            == "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0";
    t.row(
        "The build's SHA-256 of FIPS 180-4's four test messages is the published answer",
        if fips {
            "all four agree"
        } else {
            "they differ"
        },
        fips,
    );

    // FX-PACK-002
    t.heading("FX-PACK-002: the project in the package");
    let packed = persist::load(&dest.join("shot.json")).expect("the package opens");
    let mut places = BTreeMap::new();
    for a in &packed.document.project().assets {
        let mut place = serde_json::Map::new();
        if let Some(p) = &a.path {
            place.insert("path".into(), p.as_str().into());
        }
        if !a.frames.is_empty() {
            place.insert(
                "frames".into(),
                a.frames
                    .iter()
                    .map(|(k, v)| (k.to_string(), J::from(v.as_str())))
                    .collect(),
            );
        }
        places.insert(a.id.as_str().to_string(), J::Object(place));
    }
    t.row(
        "Every drawing's path is its place in the package",
        "as `places` in the expected file",
        J::Object(places.into_iter().collect()) == expected["places"],
    );
    let mut unmoved = saved.document.project().clone();
    for (a, b) in unmoved
        .assets
        .iter_mut()
        .zip(&packed.document.project().assets)
    {
        a.path = b.path.clone();
        a.frames = b.frames.clone();
    }
    t.row(
        "Nothing else in the project changed, unknown data included",
        "the source project with only its paths replaced",
        persist::to_json(&unmoved, &saved.preserved)
            == fs::read_to_string(dest.join("shot.json")).unwrap(),
    );
    let kept: Vec<&str> = packed
        .document
        .project()
        .assets
        .iter()
        .filter(|a| !a.redistribute)
        .map(|a| a.id.as_str())
        .collect();
    t.row(
        "It still says which media may not be passed on",
        &kept.join(", "),
        kept == ["asset-licensed"]
            && fs::read_to_string(dest.join("shot.json"))
                .unwrap()
                .matches("\"redistribute\": false")
                .count()
                == 1,
    );
    let missing = |l: &persist::Loaded| {
        l.warnings
            .iter()
            .map(|w| {
                format!(
                    "{} ({})",
                    w.id.as_str(),
                    w.detail.split(" relative").next().unwrap()
                )
            })
            .collect::<Vec<_>>()
    };
    let said = missing(&packed);
    t.row(
        "Opened from the package, it reports `MEDIA_MISSING` for the licensed sheet and `gone_0001.png` only",
        &said.join("; "),
        said.len() == 2
            && packed.warnings.iter().all(|w| w.id == DiagnosticId::MediaMissing)
            && said[0].contains("media/asset-licensed/sheet.png")
            && said[1].contains("media/asset-gone/gone_0001.png")
            && !said[1].contains("cel_0001"),
    );

    // FX-PACK-003
    t.heading("FX-PACK-003: moved somewhere else");
    let far = scratch.join("moved here").join("深い 場所").join("package");
    fs::create_dir_all(far.parent().unwrap()).unwrap();
    fs::rename(&dest, &far).unwrap();
    let moved = persist::load(&far.join("shot.json")).expect("the moved package opens");
    let said = missing(&moved);
    t.row(
        &format!(
            "Opened from `{}`, it reports the same",
            "target/b15b-scratch/moved here/深い 場所/package"
        ),
        &said.join("; "),
        said == missing(&packed),
    );
    let source_root = source_file.parent().unwrap();
    let alt_same = frames(moved.document.project(), &far, "comp-alt")
        == frames(saved.document.project(), source_root, "comp-alt");
    t.row(
        "Every frame of `comp-alt` is the source's, to the byte",
        "3 frames compared",
        alt_same,
    );
    let sheet = far.join("media/asset-licensed/sheet.png");
    fs::create_dir_all(sheet.parent().unwrap()).unwrap();
    fs::copy(source_root.join("licensed/sheet.png"), &sheet).unwrap();
    let main_same = frames(moved.document.project(), &far, "comp-main")
        == frames(saved.document.project(), source_root, "comp-main");
    t.row(
        "With the licensed sheet put in its place, every frame of `comp-main` is the source's",
        "3 frames compared",
        main_same,
    );
    fs::remove_file(&sheet).unwrap();

    // FX-PACK-004
    t.heading("FX-PACK-004: checking, in six situations (files not `ok`)");
    let project = far.join("shot.json");
    let cel = far.join("media/asset-cel/cel_0001.png");
    let cel_bytes = fs::read(&cel).unwrap();
    let gone = far.join("media/asset-gone/gone_0001.png");
    type Step<'a> = &'a dyn Fn();
    let situations: [(&str, Step, Step); 6] = [
        ("as collected", &|| {}, &|| {}),
        (
            "one byte of cel_0001.png changed",
            &|| {
                let mut b = cel_bytes.clone();
                b[0] ^= 1;
                fs::write(&cel, b).unwrap();
            },
            &|| fs::write(&cel, &cel_bytes).unwrap(),
        ),
        (
            "cel_0001.png deleted",
            &|| fs::remove_file(&cel).unwrap(),
            &|| fs::write(&cel, &cel_bytes).unwrap(),
        ),
        (
            "the licensed sheet supplied by the recipient",
            &|| {
                fs::copy(source_root.join("licensed/sheet.png"), &sheet).unwrap();
            },
            &|| fs::remove_file(&sheet).unwrap(),
        ),
        (
            "a different file put where the licensed sheet goes",
            &|| fs::write(&sheet, &cel_bytes).unwrap(),
            &|| fs::remove_file(&sheet).unwrap(),
        ),
        (
            "a file put where the missing drawing goes",
            &|| fs::write(&gone, &cel_bytes).unwrap(),
            &|| fs::remove_file(&gone).unwrap(),
        ),
    ];
    let mut seen = Vec::new();
    for (i, (name, setup, undo)) in situations.iter().enumerate() {
        setup();
        let rows = package::check(&project).expect("a package checks");
        undo();
        let want: Vec<(String, String)> = expected["checks"][i]["rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                (
                    r["path"].as_str().unwrap().into(),
                    r["answer"].as_str().unwrap().into(),
                )
            })
            .collect();
        let got = answers(&rows);
        assert_eq!(expected["checks"][i]["situation"], *name);
        let right_ids = rows
            .iter()
            .all(|r| match (r.answer.as_str(), &r.diagnostic) {
                ("ok", None) => true,
                (a, Some(d)) => {
                    seen.push(d.id);
                    d.id == match a {
                        "changed" => DiagnosticId::PackageFileChanged,
                        "missing" => DiagnosticId::MediaMissing,
                        "excluded" => DiagnosticId::PackageMediaExcluded,
                        _ => DiagnosticId::PackageFileUnverified,
                    }
                }
                _ => false,
            });
        t.row(name, &odd(&got), got == want && right_ids);
    }
    let seen_ids: Vec<&str> = [
        DiagnosticId::PackageFileChanged,
        DiagnosticId::PackageMediaExcluded,
        DiagnosticId::PackageFileUnverified,
    ]
    .iter()
    .filter(|id| seen.contains(id))
    .map(|id| id.as_str())
    .collect();
    t.row(
        "Diagnostics said along the way",
        &format!(
            "`MEDIA_MISSING`, {}",
            seen_ids
                .iter()
                .map(|s| format!("`{s}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        seen_ids.len() == 3,
    );

    // FX-PACK-005
    t.heading("FX-PACK-005: refusals");
    let full = scratch.join("full");
    fs::create_dir_all(&full).unwrap();
    fs::write(full.join("keep.txt"), "mine").unwrap();
    let said = package::collect(
        saved.document.project(),
        &saved.preserved,
        Some(&source_file),
        &full,
    );
    let mut left = Vec::new();
    tree(&full, "", &mut left);
    t.row(
        "A folder holding a file is refused and left as it was",
        &format!(
            "{}; the folder holds {}",
            said.as_ref().err().map_or("collected", |d| d.id.as_str()),
            left.join(", ")
        ),
        matches!(&said, Err(d) if d.id == DiagnosticId::PackageDestinationNotEmpty)
            && left == ["keep.txt"]
            && !scratch.join("full.partial").exists(),
    );
    let empty = scratch.join("empty");
    fs::create_dir_all(&empty).unwrap();
    fs::create_dir_all(scratch.join("empty.partial")).unwrap();
    let said = package::collect(
        saved.document.project(),
        &saved.preserved,
        Some(&source_file),
        &empty,
    );
    t.row(
        "An empty folder with a `.partial` folder beside it is refused, and both are left as they were",
        said.as_ref().err().map_or("collected", |d| d.id.as_str()),
        matches!(&said, Err(d) if d.id == DiagnosticId::PackageDestinationNotEmpty)
            && fs::read_dir(&empty).unwrap().next().is_none()
            && fs::read_dir(scratch.join("empty.partial")).unwrap().next().is_none(),
    );
    let mut all_failed = true;
    for limit in 0..12 {
        let broken = scratch.join(format!("broken-{limit}"));
        let said = package::collect_limited(
            saved.document.project(),
            &saved.preserved,
            Some(&source_file),
            &broken,
            Some(limit),
        );
        let mut partial = broken.clone().into_os_string();
        partial.push(".partial");
        all_failed &= matches!(&said, Err(d) if d.id == DiagnosticId::PackageWriteFailed)
            && !broken.exists()
            && !Path::new(&partial).exists();
    }
    t.row(
        "A write made to fail after 0, 1, ... 11 of its 12 files gives `PACKAGE_WRITE_FAILED` and leaves neither the folder nor `.partial` behind",
        if all_failed { "all twelve" } else { "not all twelve" },
        all_failed,
    );
    let said = package::check(&source_file).err();
    t.row(
        "Checking a project with no manifest beside it",
        said.as_ref().map_or("checked", |d| d.id.as_str()),
        said.is_some_and(|d| d.id == DiagnosticId::PackageManifestInvalid),
    );
    let bad = scratch.join("bad");
    fs::create_dir_all(&bad).unwrap();
    fs::write(bad.join(package::MANIFEST), "{ not json").unwrap();
    let said = package::check(&bad.join("shot.json")).err();
    t.row(
        "Checking a project whose manifest is not readable",
        said.as_ref().map_or("checked", |d| d.id.as_str()),
        said.is_some_and(|d| d.id == DiagnosticId::PackageManifestInvalid),
    );

    // FX-PACK-006
    t.heading("FX-PACK-006: nothing in the window changes");
    t.row(
        "The open project, with its unsaved rename, is as it was",
        "same",
        *document.project() == before.0,
    );
    t.row(
        "Its undo history is as it was",
        &format!("{} step", document.undo_depth()),
        document.undo_depth() == before.1 && before.1 == 1,
    );
    t.row(
        "It still has unsaved changes",
        &document.is_dirty().to_string(),
        document.is_dirty() && before.2,
    );
    t.row(
        "Its file on disk is as it was",
        "same bytes",
        fs::read(&source_file).unwrap() == before.3,
    );
    let from_window = fs::read_to_string(scratch.join("from-window/shot.json")).unwrap();
    t.row(
        "A package made from the window carries the unsaved rename; the saved file does not",
        "yes",
        from_window.contains("renamed, not saved")
            && !String::from_utf8_lossy(&before.3).contains("renamed, not saved"),
    );

    let doc = format!(
        "# B-15b — collecting and checking a package (T-13)\n\n\
         **{} of {} checks pass.**\n\n\
         Generated by `tests/b15b_package.rs`. Covers R-14, test T-13 and the packaging \
         fixtures FX-PACK-001 to 006 of document 25, under D-61.\n\n\
         ## How to read this\n\n\
         The build collected `Fixtures/packaging/source/shot.json` into a new folder, the way \
         File > Collect Files... will once B-15c adds it. Each row is one thing document 25 \
         says must be true of the result. The expected answers were worked out separately by \
         `tools/package_reference.py`, in Python. The last column says whether the build's \
         answer matches. The checking rows list only the files whose answer is not `ok`, as \
         document 25 does.\n\n\
         What each answer means to the person receiving a package: `excluded` is media the \
         sender may not pass on (`PACKAGE_MEDIA_EXCLUDED`), `missing` was already missing when \
         the package was made or has been deleted since (`MEDIA_MISSING`), `changed` is a file \
         that is not the one packaged (`PACKAGE_FILE_CHANGED`), and `unverified` is a file put \
         where a missing drawing goes, which nothing can vouch for \
         (`PACKAGE_FILE_UNVERIFIED`). A folder that is not empty is refused with \
         `PACKAGE_DESTINATION_NOT_EMPTY`, a write that fails with `PACKAGE_WRITE_FAILED`, and a \
         missing or broken manifest with `PACKAGE_MANIFEST_INVALID`.\n",
        t.passed, t.checks
    );
    fs::write(repo("verification/B-15b_package_table.md"), doc + &t.out).unwrap();
    assert_eq!(
        t.passed, t.checks,
        "see verification/B-15b_package_table.md"
    );
}
