//! B-615: every key travels with its layer when the layer is moved, against D-740.
//!
//! Writes `verification/B-615_layer_shift_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/layer_shift/`, written by `tools/layer_shift_reference.py` before any code:
//! `expected_layer_shift.json` names, for each case, a project in `Fixtures/`, a keyed layer in
//! it and the frame its in point is moved to, and lists every key that layer holds, where it
//! sits and its frame after the move, in order of where; `host.json` keys what no other fixture
//! does.
//!
//! # What is checked
//!
//! Moved with `layer.shift`, the saved layer holds those keys at those frames;
//! everything else saved is as before the move but the in and out points, each moved the same;
//! and one undo gives back the project as saved before the move.

use std::fs;

use serde_json::Value as J;

use anime_compositor::command::Command;
use anime_compositor::model::Id;
use anime_compositor::persist;

mod common;
use common::repo;

/// Every key under `v` as (where, frame), as the reference tool lists them before sorting.
fn keys(v: &J, at: &str, out: &mut Vec<(String, i64)>) {
    match v {
        J::Object(m) => {
            for (k, x) in m {
                let here = format!("{at}/{k}");
                if k == "keyframes" || k == "keys" {
                    for key in x.as_array().into_iter().flatten() {
                        if let Some(f) = key.get("frame").and_then(J::as_i64) {
                            out.push((here.clone(), f));
                        }
                    }
                }
                keys(x, &here, out);
            }
        }
        J::Array(a) => a.iter().for_each(|x| keys(x, &format!("{at}/*"), out)),
        _ => {}
    }
}

/// `v` with every key's frame and the in and out points `by` earlier.
fn back(v: &mut J, by: i64, top: bool) {
    match v {
        J::Object(m) => {
            for (k, x) in m.iter_mut() {
                if k == "keyframes" || k == "keys" {
                    for key in x.as_array_mut().into_iter().flatten() {
                        if let Some(f) = key.get("frame").and_then(J::as_i64) {
                            key["frame"] = (f - by).into();
                        }
                    }
                }
                if top && (k == "in_frame" || k == "out_frame") {
                    *x = (x.as_i64().unwrap() - by).into();
                }
                back(x, by, false);
            }
        }
        J::Array(a) => a.iter_mut().for_each(|x| back(x, by, false)),
        _ => {}
    }
}

#[test]
fn b615_layer_shift() {
    let root = repo("Fixtures/layer_shift");
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root.join("expected_layer_shift.json")).unwrap())
            .unwrap();
    let mut out = String::from(
        "# B-615: every key travels with its layer\n\n\
         Written by `tests/b615_layer_shift.rs` from `Fixtures/layer_shift/`, against D-740. \
         Each line is one check: what `tools/layer_shift_reference.py` says, and what the build \
         gives.\n\n\
         | Case | Check | Should | The build | Matches |\n| --- | --- | --- | --- | --- |\n",
    );
    let (mut checks, mut passed) = (0, 0);
    let mut row = |who: &str, what: &str, ok: bool, should: String, built: String| {
        checks += 1;
        passed += ok as usize;
        let verdict = if ok { "yes" } else { "**NO**" };
        out.push_str(&format!(
            "| {who} | {what} | {should} | {built} | {verdict} |\n"
        ));
    };
    for case in expected["cases"].as_array().unwrap() {
        let cid = case["id"].as_str().unwrap();
        let file = repo("Fixtures").join(case["project"].as_str().unwrap());
        let loaded = persist::load_str(&fs::read_to_string(file).unwrap()).expect("it reads");
        let mut doc = loaded.document;
        let save = |doc: &anime_compositor::command::Document| -> J {
            serde_json::from_str(&persist::to_json(doc.project(), &loaded.preserved)).unwrap()
        };
        let before = save(&doc);
        let (comp, layer) = (
            case["composition"].as_str().unwrap(),
            case["layer"].as_str().unwrap(),
        );
        let find = |p: &J| -> (usize, usize) {
            let ci = p["compositions"]
                .as_array()
                .unwrap()
                .iter()
                .position(|c| c["id"] == comp)
                .unwrap();
            let li = p["compositions"][ci]["layers"]
                .as_array()
                .unwrap()
                .iter()
                .position(|l| l["id"] == layer)
                .unwrap();
            (ci, li)
        };
        let (ci, li) = find(&before);
        let to = case["in_frame"].as_i64().unwrap();
        let by = to
            - before["compositions"][ci]["layers"][li]["in_frame"]
                .as_i64()
                .unwrap_or(0);
        let moved = doc.apply(Command::ShiftLayer {
            composition: Id::new(comp.to_string()),
            layer_id: Id::new(layer.to_string()),
            in_frame: to as i32,
        });
        row(
            cid,
            &format!("{layer} moved to frame {to}"),
            moved.is_ok(),
            "moved".into(),
            match &moved {
                Ok(_) => "moved".into(),
                Err(d) => format!("refused: {}", d.message),
            },
        );
        let after = save(&doc);

        let mut got = Vec::new();
        keys(&after["compositions"][ci]["layers"][li], "", &mut got);
        got.sort_by(|a, b| a.0.cmp(&b.0));
        let want: Vec<(String, i64)> = case["keys"]
            .as_array()
            .unwrap()
            .iter()
            .map(|k| (k[0].as_str().unwrap().to_string(), k[1].as_i64().unwrap()))
            .collect();
        let wrong: Vec<String> = want
            .iter()
            .zip(&got)
            .filter(|(w, g)| w != g)
            .map(|(w, g)| format!("{} at {} not {}", g.0, g.1, w.1))
            .collect();
        let ok = got == want;
        row(
            cid,
            "every key moved with it",
            ok,
            format!("{} keys, each {by} frames on", want.len()),
            if ok {
                format!("{} keys, each {by} frames on", got.len())
            } else if got.len() != want.len() {
                format!("{} keys", got.len())
            } else {
                format!("{} left behind: {}", wrong.len(), wrong.join("; "))
            },
        );

        let mut undone = after.clone();
        back(&mut undone["compositions"][ci]["layers"][li], by, true);
        let same = undone == before;
        row(
            cid,
            "nothing else changed",
            same,
            "the in and out points and keys only".into(),
            if same {
                "the in and out points and keys only"
            } else {
                "more"
            }
            .into(),
        );

        doc.undo();
        let same = save(&doc) == before;
        row(
            cid,
            "one undo",
            same,
            "the project as before".into(),
            if same {
                "the project as before"
            } else {
                "different"
            }
            .into(),
        );
    }

    out.push_str(&format!("\nB-615: {passed} of {checks} checks pass.\n"));
    fs::write(repo("verification/B-615_layer_shift_table.md"), &out).unwrap();
    assert_eq!(
        passed, checks,
        "see verification/B-615_layer_shift_table.md"
    );
}
