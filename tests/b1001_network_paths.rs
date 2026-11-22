//! B-1001: network paths and Collect Files (D-268, from the P-25 security pass).
//!
//! D-268, accepted by the owner on 2026-11-21 ("proceed"):
//!
//! - (M2) A media path in a project that names a network share (`\\server\share\...`,
//!   `//server/...`, `\\?\UNC\...`, `\??\UNC\...`) or a device (`\\.\...`) is not read: reading
//!   it makes Windows send the person's sign-in proof to that server. The reference is kept as
//!   it is, and the person is told `INVALID_PATH` (document 28: "Path fails normalization/access
//!   policy; reject operation with remediation"). A drive letter, mapped or not (`Z:\`), and the
//!   long local form `\\?\C:\` are left alone.
//! - (L1) Collect Files copies only files of a kind the program imports. Anything else, and a
//!   file on a network share, is not read and not copied; the manifest lists it as `not media`
//!   or `network share`, checking the package answers `excluded` for it, and collecting names it
//!   by the path the project gave.
//!
//! Every expected value below is written by hand from those words, before the build had the
//! rule, and the test builds its small project in a scratch folder (Fixtures/ is read-only to
//! this work). The network name is `nas.invalid`: `.invalid` is reserved and never resolves, so
//! a build that wrongly reads one of these paths fails on the diagnostic it gives, not by
//! reaching a real server.
//!
//! Writes `verification/B-1001_network_paths_table.md`.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::model::Id;
use anime_compositor::{media, package, persist};

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// `/` written as `\`, so the Windows forms can be spelled readably below.
fn win(s: &str) -> String {
    s.replace('/', "\\")
}

/// Every file under `dir`, with forward slashes, sorted.
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

struct Table {
    out: String,
    checks: usize,
    passed: usize,
}

impl Table {
    fn row(&mut self, what: &str, expected: impl ToString, actual: impl ToString) {
        let (expected, actual) = (expected.to_string(), actual.to_string());
        let ok = expected == actual;
        self.checks += 1;
        self.passed += ok as usize;
        let verdict = if ok { "yes" } else { "**NO**" };
        let cell = |s: &str| s.replace('|', "\\|").replace('\n', " ");
        self.out.push_str(&format!(
            "| {what} | {} | {} | {verdict} |\n",
            cell(&expected),
            cell(&actual)
        ));
    }

    fn heading(&mut self, text: &str) {
        self.out.push_str(&format!(
            "\n## {text}\n\n| Check | Expected | The build's answer | Matches |\n| --- | --- | --- | --- |\n"
        ));
    }
}

/// The project: one drawing on this computer, a text file posing as a drawing, five network or
/// device paths, a sequence half on a share, a mapped drive letter that is not there, and a
/// sound file on a share. The layer shape is borrowed from D-61's packaging shot.
fn shot(dir: &Path) -> PathBuf {
    let _ = fs::remove_dir_all(dir);
    fs::create_dir_all(dir).unwrap();
    fs::copy(
        repo("Fixtures/packaging/source/media/cel_0001.png"),
        dir.join("local.png"),
    )
    .unwrap();
    fs::write(
        dir.join("notes.txt"),
        "Private notes. A project naming this file must not get it copied into a package.\n",
    )
    .unwrap();
    let template: J = serde_json::from_slice(
        &fs::read(repo("Fixtures/packaging/source/shot.json")).unwrap(),
    )
    .unwrap();
    let interp = json!({ "color_space": "srgb", "alpha": "straight" });
    let still = |id: &str, kind: &str, path: &str| {
        json!({ "id": id, "kind": kind, "name": id, "path": path, "interpretation": interp })
    };
    let assets = json!([
        still("asset-local", "still", "local.png"),
        still("asset-notes", "still", "notes.txt"),
        still("asset-unc", "still", &win("//nas.invalid/share/cel.png")),
        still("asset-fwd", "still", "//nas.invalid/share/fwd.png"),
        still("asset-verbatim", "still", &win("//?/UNC/nas.invalid/share/verbatim.png")),
        still("asset-device", "still", &win("//./UNC/nas.invalid/share/device.png")),
        {
            "id": "asset-seq", "kind": "image_sequence", "name": "asset-seq",
            "pattern": "cel_####.png",
            "frames": { "1": "local.png", "2": win("//nas.invalid/share/cel_0002.png") },
            "interpretation": interp
        },
        still("asset-mapped", "still", &win("Z:/d268-mapped/cel.png")),
        still("asset-voice", "audio", &win("//nas.invalid/share/voice.wav")),
    ]);
    let mut comp = template["compositions"][0].clone();
    let layer = comp["layers"][0].clone();
    let layers: Vec<J> = [("layer-local", "asset-local"), ("layer-unc", "asset-unc")]
        .iter()
        .map(|(id, asset)| {
            let mut l = layer.clone();
            l["id"] = json!(id);
            l["name"] = json!(id);
            l["asset_id"] = json!(asset);
            l
        })
        .collect();
    comp["id"] = json!("comp-main");
    comp["layer_order"] = json!(["layer-unc", "layer-local"]);
    comp["layers"] = json!(layers);
    let mut project = template.clone();
    project["project_id"] = json!("proj-network-paths");
    project["assets"] = assets;
    project["compositions"] = json!([comp]);
    let file = dir.join("shot.json");
    fs::write(&file, serde_json::to_string_pretty(&project).unwrap()).unwrap();
    file
}

#[test]
fn b1001_network_paths() {
    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    let scratch = repo("target/b1001-scratch");
    let _ = fs::remove_dir_all(&scratch);
    let file = shot(&scratch.join("source"));

    // ---- which paths are network paths ---------------------------------------------------
    t.heading("Which paths are not read (M2)");
    let cases: &[(String, bool)] = &[
        (win("//nas.invalid/share/cel.png"), true),
        ("//nas.invalid/share/cel.png".into(), true),
        ("/\\nas.invalid\\share\\cel.png".into(), true),
        (win("//?/UNC/nas.invalid/share/cel.png"), true),
        ("//?/UNC/nas.invalid/share/cel.png".into(), true),
        ("\\??\\UNC\\nas.invalid\\share\\cel.png".into(), true),
        (win("//./UNC/nas.invalid/share/cel.png"), true),
        (win("//./pipe/anything"), true),
        (win("//?/GLOBALROOT/Device/Mup/nas.invalid/share/cel.png"), true),
        (win("//?/C:/shots/cel.png"), false),
        (win("Z:/shots/cel.png"), false),
        ("C:/shots/cel.png".into(), false),
        ("media/cel_0001.png".into(), false),
        (win("../media/cel_0001.png"), false),
        (win("/shots/cel.png"), false),
    ];
    let word = |network: bool| if network { "not read" } else { "read" };
    for (path, network) in cases {
        t.row(
            &format!("`{path}`"),
            word(*network),
            word(media::on_network(path)),
        );
    }
    let refused = media::file_at(&scratch, &win("//nas.invalid/share/cel.png"));
    t.row(
        "a network path is refused with document 28's identifier",
        "INVALID_PATH",
        refused.as_ref().err().map_or("read", |d| d.id.as_str()),
    );
    t.row(
        "and the reason and the way out are said",
        "network share, sign-in, copy it to a drive on this computer, or a mapped drive letter",
        refused.as_ref().err().map_or("nothing said".to_string(), |d| {
            let said = format!("{} {} {}", d.message, d.detail, d.remediation.clone().unwrap_or_default());
            [
                ("network share", "network share"),
                ("sign-in", "sign-in"),
                ("this computer", "copy it to a drive on this computer"),
                ("mapped drive letter", "or a mapped drive letter"),
            ]
            .iter()
            .filter(|(k, _)| said.contains(k))
            .map(|(_, v)| *v)
            .collect::<Vec<_>>()
            .join(", ")
        }),
    );
    t.row(
        "a path on this computer is joined to the project's folder",
        scratch.join("local.png").display(),
        media::file_at(&scratch, "local.png").map_or("refused".into(), |p| p.display().to_string()),
    );

    // ---- opening the project ---------------------------------------------------------------
    t.heading("Opening a project that names them (M2)");
    let loaded = persist::load(&file).expect("the project opens");
    let invalid: Vec<&str> = [
        "//nas.invalid/share/cel.png",
        "//nas.invalid/share/fwd.png",
        "//?/UNC/nas.invalid/share/verbatim.png",
        "//./UNC/nas.invalid/share/device.png",
        "//nas.invalid/share/cel_0002.png",
        "//nas.invalid/share/voice.wav",
    ]
    .to_vec();
    let said_invalid: Vec<&_> = loaded
        .warnings
        .iter()
        .filter(|d| d.id == DiagnosticId::InvalidPath)
        .collect();
    t.row(
        "one INVALID_PATH for each network path, and none for anything else",
        invalid.len(),
        said_invalid.len(),
    );
    for path in &invalid {
        let stored = if path.contains("fwd") { path.to_string() } else { win(path) };
        t.row(
            &format!("`{stored}` is named"),
            "named",
            if said_invalid.iter().any(|d| d.message.contains(&stored) || d.detail.contains(&stored)) {
                "named"
            } else {
                "not named"
            },
        );
    }
    let missing: Vec<String> = loaded
        .warnings
        .iter()
        .filter(|d| d.id == DiagnosticId::MediaMissing)
        .map(|d| d.detail.split_whitespace().nth(1).unwrap_or("").to_string())
        .collect();
    t.row(
        "MEDIA_MISSING only for the mapped drive letter, which is looked for as usual",
        "asset-mapped",
        missing.join(", "),
    );
    let saved = scratch.join("source/resaved.json");
    let mut loaded = loaded;
    persist::save(&saved, &mut loaded.document, &loaded.preserved).expect("save");
    let project = loaded.document.project();
    t.row(
        "every stored path is kept exactly as the project gave it",
        true,
        project
            .assets
            .iter()
            .flat_map(|a| a.files())
            .any(|f| f == win("//nas.invalid/share/cel.png"))
            && project.assets.iter().flat_map(|a| a.files()).count() == 10,
    );
    t.row(
        "and saving it writes them back unchanged",
        true,
        persist::load(&saved)
            .map(|l| l.document.project().assets == project.assets)
            .unwrap_or(false),
    );

    // ---- drawing a frame -------------------------------------------------------------------
    t.heading("Drawing a frame (M2)");
    let mut log = FrameLog::new(8);
    let drawn = render_frame(project, &Id::new("comp-main"), 0, &scratch.join("source"), 128, &mut log);
    t.row("frame 0 still draws", true, drawn.is_ok());
    t.row(
        "the layer on the share says INVALID_PATH, and nothing else is said",
        "INVALID_PATH",
        log.ids_at(0)
            .iter()
            .map(|id| id.as_str())
            .collect::<Vec<_>>()
            .join(", "),
    );

    // ---- collecting ------------------------------------------------------------------------
    t.heading("Collect Files (L1, and M2 for the files it reads)");
    let dest = scratch.join("package");
    let left_out = package::collect(project, &loaded.preserved, Some(&file), &dest);
    t.row("collecting finishes", true, left_out.is_ok());
    let left_out = left_out.unwrap_or_default();
    let expected_left_out = vec![
        "notes.txt".to_string(),
        win("//nas.invalid/share/cel.png"),
        "//nas.invalid/share/fwd.png".to_string(),
        win("//?/UNC/nas.invalid/share/verbatim.png"),
        win("//./UNC/nas.invalid/share/device.png"),
        win("//nas.invalid/share/cel_0002.png"),
        win("//nas.invalid/share/voice.wav"),
    ];
    t.row(
        "the files it left out, by the path the project gave, in order",
        expected_left_out.join("; "),
        left_out.join("; "),
    );
    let mut written = Vec::new();
    if dest.exists() {
        tree(&dest, "", &mut written);
    }
    t.row(
        "only the drawing is copied (into both assets that use it)",
        "media/asset-local/local.png; media/asset-seq/local.png; package-manifest.json; shot.json",
        written.join("; "),
    );
    let manifest: J = fs::read(dest.join(package::MANIFEST))
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or_default();
    let rows: Vec<String> = manifest["assets"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|a| {
            a["files"].as_array().into_iter().flatten().map(move |f| {
                format!(
                    "{} {} {} {}",
                    f["path"].as_str().unwrap_or(""),
                    f["status"].as_str().unwrap_or(""),
                    f["bytes"],
                    f["sha256"].is_string()
                )
            })
        })
        .collect();
    let cel_bytes = fs::metadata(scratch.join("source/local.png")).map(|m| m.len()).unwrap_or(0);
    let expected_rows = vec![
        format!("media/asset-local/local.png copied {cel_bytes} true"),
        "media/asset-notes/notes.txt not media null false".to_string(),
        "media/asset-unc/cel.png network share null false".to_string(),
        "media/asset-fwd/fwd.png network share null false".to_string(),
        "media/asset-verbatim/verbatim.png network share null false".to_string(),
        "media/asset-device/device.png network share null false".to_string(),
        format!("media/asset-seq/local.png copied {cel_bytes} true"),
        "media/asset-seq/cel_0002.png network share null false".to_string(),
        "media/asset-mapped/cel.png missing null false".to_string(),
        "media/asset-voice/voice.wav network share null false".to_string(),
    ];
    t.row(
        "the manifest lists every file with its status; nothing left out was read (no size, no hash)",
        expected_rows.join("; "),
        rows.join("; "),
    );
    let checked = package::check(&dest.join("shot.json"));
    t.row(
        "checking the package: what was left out is excluded, the absent mapped file missing",
        [
            "media/asset-notes/notes.txt excluded",
            "media/asset-unc/cel.png excluded",
            "media/asset-fwd/fwd.png excluded",
            "media/asset-verbatim/verbatim.png excluded",
            "media/asset-device/device.png excluded",
            "media/asset-seq/cel_0002.png excluded",
            "media/asset-mapped/cel.png missing",
            "media/asset-voice/voice.wav excluded",
        ]
        .join("; "),
        checked.map_or_else(
            |d| d.id.as_str().to_string(),
            |rows| {
                rows.iter()
                    .filter(|r| r.answer.as_str() != "ok")
                    .map(|r| format!("{} {}", r.path, r.answer.as_str()))
                    .collect::<Vec<_>>()
                    .join("; ")
            },
        ),
    );

    let mut page = format!(
        "# B-1001: network paths and Collect Files (D-268)\n\n\
         Written by `tests/b1001_network_paths.rs`. Do not edit by hand.\n\n\
         What changed: a project that names a media file on a network share \
         (`\\\\server\\share\\...` and its other spellings) no longer makes the program read it, \
         because reading it would make Windows send your sign-in proof to that server. The \
         project keeps the path as it was, and you are told the file was not opened and how to \
         fix it: copy the file to a drive on this computer, or to a mapped drive letter such as \
         Z:. Collect Files now copies only the kinds of file the program can import, and names \
         every file it left out.\n\n\
         **{} of {} checks match.**\n",
        t.passed, t.checks
    );
    page.push_str(&t.out);
    fs::write(repo("verification/B-1001_network_paths_table.md"), page).unwrap();
    assert_eq!(t.passed, t.checks, "{}", t.out);
}
