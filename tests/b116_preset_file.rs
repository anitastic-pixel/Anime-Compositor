//! B-116: effect presets in a file, in the core, against D-180.
//!
//! Writes `verification/B-116_preset_file_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/preset_file/expected_preset_file.json`, written by `tools/preset_file_reference.py`
//! before any code. A valid case names the presets the build must read, by name and effect type,
//! in order; a refused case names the diagnostic and a word its message must contain.
//!
//! # What is also checked
//!
//! Every valid file is written again by `persist::write_presets`, as the window's Export does,
//! and read back: the same presets must come out. And every refused file must refuse the same
//! way when the window hands its presets to Export, so nothing is written that Import refuses.
//!
//! The window's two commands and the dialog are in `app/src/main.rs` and the playtest.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::persist;

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn described(presets: &[persist::Preset]) -> String {
    presets
        .iter()
        .map(|p| {
            let types: Vec<&str> = p.effects.iter().map(|e| e.type_id()).collect();
            format!("\"{}\": {}", p.name, types.join(", "))
        })
        .collect::<Vec<_>>()
        .join("; ")
}

#[test]
fn b116_preset_file() {
    let root = repo("Fixtures/preset_file");
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root.join("expected_preset_file.json")).unwrap()).unwrap();
    let mut out = String::from(
        "# B-116: effect presets in a file\n\n\
         Written by `tests/b116_preset_file.rs` from `Fixtures/preset_file/`, against D-180. Each line is one \
         preset file: what `tools/preset_file_reference.py` says the build must do with it, and what the build \
         did. A refused file must import nothing, and its message must name what was wrong.\n\n\
         | Case | Says | Should | The build | Matches |\n| --- | --- | --- | --- | --- |\n",
    );
    let (mut checks, mut passed) = (0, 0);
    let mut row = |id: &str, says: &str, should: String, built: String, ok: bool| {
        checks += 1;
        passed += ok as usize;
        let verdict = if ok { "yes" } else { "**NO**" };
        out.push_str(&format!("| {id} | {says} | {should} | {built} | {verdict} |\n"));
    };
    for (id, case) in expected["cases"].as_object().unwrap() {
        let text = fs::read_to_string(root.join(case["file"].as_str().unwrap())).unwrap();
        let says = case["says"].as_str().unwrap();
        let read = persist::read_presets(&text);
        if let Some(refused) = case["refused"].as_str() {
            let names = case["names"].as_str().unwrap();
            let should = format!("refuse, `{refused}`, naming {names}");
            match &read {
                Ok(p) => row(id, says, should, format!("read {}", described(p)), false),
                Err(d) => {
                    let ok = d.id.as_str() == refused && d.message.contains(names);
                    row(id, says, should, format!("refused, `{}`: {}", d.id.as_str(), d.message), ok);
                }
            }
            // Export refuses what Import refuses, where what is wrong is in the presets themselves,
            // the only part of a file the window hands to Export.
            if let Ok(file) = serde_json::from_str::<J>(&text) {
                let whole = file.as_object().is_some_and(|o| o.len() == 2);
                if let Some(list) = file.get("presets").filter(|_| whole && file["preset_file_version"] == 0) {
                    let written = persist::write_presets(&list.to_string());
                    row(
                        id,
                        "The same presets handed to Export.",
                        "write nothing".into(),
                        match &written {
                            Err(d) => format!("wrote nothing: {}", d.message),
                            Ok(_) => "wrote them".into(),
                        },
                        written.is_err(),
                    );
                }
            }
            continue;
        }
        let wanted: Vec<String> = case["presets"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| {
                let types: Vec<&str> = p["effects"].as_array().unwrap().iter().map(|t| t.as_str().unwrap()).collect();
                format!("\"{}\": {}", p["name"].as_str().unwrap(), types.join(", "))
            })
            .collect();
        let should = format!("read {}", wanted.join("; "));
        match &read {
            Err(d) => row(id, says, should, format!("refused, `{}`: {}", d.id.as_str(), d.message), false),
            Ok(p) => {
                let built = format!("read {}", described(p));
                row(id, says, should.clone(), built.clone(), built == should);
                // Written as Export writes it, then read back.
                let list = serde_json::from_str::<J>(&text).unwrap()["presets"].to_string();
                let again = persist::write_presets(&list).and_then(|t| persist::read_presets(&t));
                row(
                    id,
                    "Exported and imported again.",
                    "the same presets".into(),
                    match &again {
                        Ok(q) if q == p => "the same presets".into(),
                        Ok(q) => format!("different: {}", described(q)),
                        Err(d) => format!("refused: {}", d.message),
                    },
                    again.as_ref().is_ok_and(|q| q == p),
                );
            }
        }
    }
    out.push_str(&format!("\n**B-116: {passed} of {checks} checks pass.**\n"));
    fs::write(repo("verification/B-116_preset_file_table.md"), &out).unwrap();
    assert_eq!(passed, checks, "see verification/B-116_preset_file_table.md");
}
