//! B-616: chosen keys of a mask's, a shape's and a group's numbers moved, deleted and reversed
//! together through the requests the timeline sends, against D-741.
//!
//! Writes `verification/B-616_key_requests_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/key_requests/expected_key_requests.json`, written by
//! `tools/key_requests_reference.py` before any code: for each case a project, a layer, the
//! request and the keys chosen, and every key the layer holds afterwards with its frame, value
//! and ease; the numbers left with no keys and their value; and whether it is refused.
use super::*;
use serde_json::Value as J;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

fn saved(viewer: &Mutex<Viewer>) -> J {
    let held = viewer.lock().expect("the viewer lock was poisoned");
    serde_json::from_str(&persist::to_json(held.document.project(), &held.preserved)).unwrap()
}

/// The same, numbers compared as numbers, so 0 and 0.0 are one.
fn same(a: &J, b: &J) -> bool {
    match (a, b) {
        (J::Number(x), J::Number(y)) => (x.as_f64().unwrap() - y.as_f64().unwrap()).abs() < 1e-9,
        (J::Array(x), J::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(a, b)| same(a, b))
        }
        (J::Object(x), J::Object(y)) => {
            x.len() == y.len() && x.iter().all(|(k, a)| y.get(k).is_some_and(|b| same(a, b)))
        }
        _ => a == b,
    }
}

/// Every key the layer holds, as [where, frame, value, interp, ease], in order of where and frame.
fn keys(v: &J, at: &str, out: &mut Vec<J>) {
    match v {
        J::Object(m) => {
            for (k, x) in m {
                if k == "keyframes" {
                    for key in x.as_array().into_iter().flatten() {
                        out.push(serde_json::json!([
                            at,
                            key["frame"],
                            key["value"],
                            key["interp"],
                            key.get("ease").cloned().unwrap_or(J::Null)
                        ]));
                    }
                } else {
                    keys(x, &format!("{at}/{k}"), out);
                }
            }
        }
        J::Array(a) => a
            .iter()
            .enumerate()
            .for_each(|(i, x)| keys(x, &format!("{at}/{i}"), out)),
        _ => {}
    }
}

fn keys_of(layer: &J) -> Vec<J> {
    let mut out = Vec::new();
    keys(layer, "", &mut out);
    out.sort_by(|a, b| (a[0].as_str(), a[1].as_i64()).cmp(&(b[0].as_str(), b[1].as_i64())));
    out
}

fn plural(n: usize) -> &'static str {
    if n == 1 {
        ""
    } else {
        "s"
    }
}

/// `v` with every key list emptied.
fn keyless(v: &mut J) {
    match v {
        J::Object(m) => m.iter_mut().for_each(|(k, x)| match k.as_str() {
            "keyframes" => *x = J::Array(Vec::new()),
            _ => keyless(x),
        }),
        J::Array(a) => a.iter_mut().for_each(keyless),
        _ => {}
    }
}

#[test]
fn chosen_keys_of_masks_shapes_and_groups_move_delete_and_reverse() {
    let expected: J = serde_json::from_str(
        &std::fs::read_to_string(repo("Fixtures/key_requests/expected_key_requests.json")).unwrap(),
    )
    .unwrap();
    let mut out = String::from(
        "# B-616: chosen keys of masks', shapes' and groups' numbers\n\n\
         Written by `app/src/tests/key_requests.rs` from `Fixtures/key_requests/`, against D-741. \
         Each case sends the timeline's request with the keys chosen, as the page sends it. Each \
         line is one check: what `tools/key_requests_reference.py` says, and what the build \
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
        let viewer =
            Mutex::new(open(&file).unwrap_or_else(|d| panic!("open {cid}: {}", d.message)));
        let (comp, lid) = (
            case["composition"].as_str().unwrap(),
            case["layer"].as_str().unwrap(),
        );
        viewer.lock().unwrap().composition = Id::new(comp);
        let layer_of = |p: &J| -> J {
            let c = p["compositions"]
                .as_array()
                .unwrap()
                .iter()
                .find(|c| c["id"] == comp);
            let l = c.unwrap()["layers"]
                .as_array()
                .unwrap()
                .iter()
                .find(|l| l["id"] == lid);
            l.unwrap().clone()
        };
        let before = saved(&viewer);
        let chosen: Vec<&str> = case["chosen"]
            .as_array()
            .unwrap()
            .iter()
            .map(|k| k.as_str().unwrap())
            .collect();
        let named: String = chosen
            .iter()
            .map(|k| format!("&key={lid}%7C{}", k.replace('|', "%7C")))
            .collect();
        let request = match case["request"].as_str().unwrap() {
            "move" => format!("keyframe.move?by={}{named}", case["by"]),
            "delete" => format!("keyframe.add_remove?{}", &named[1..]),
            _ => format!("keyframe.reverse?{}", &named[1..]),
        };
        let said = {
            let (id, query) = request.split_once('?').unwrap();
            edit_command(&viewer, id, Some(query)).expect("a request the window answers")
        };
        let after = saved(&viewer);
        let refused = case["refused"].as_bool().unwrap();
        let what = match case["request"].as_str().unwrap() {
            "move" => format!(
                "{} key{} moved by {}",
                chosen.len(),
                plural(chosen.len()),
                case["by"]
            ),
            "delete" => format!("{} key{} deleted", chosen.len(), plural(chosen.len())),
            _ => format!("{} key{} reversed", chosen.len(), plural(chosen.len())),
        };
        let changed = !same(&after, &before);
        row(
            cid,
            &what,
            changed != refused,
            if refused { "refused" } else { "done" }.into(),
            if changed {
                "done".to_string()
            } else {
                format!("refused: {said}")
            },
        );

        let layer = layer_of(&after);
        let got = keys_of(&layer);
        let want = case["keys"].as_array().unwrap();
        let wrong: Vec<String> = want
            .iter()
            .filter(|w| !got.iter().any(|g| same(g, w)))
            .map(|w| format!("{} at {}", w[0].as_str().unwrap(), w[1]))
            .collect();
        let ok = got.len() == want.len() && wrong.is_empty();
        let places = |ks: &[J]| {
            let n = ks
                .iter()
                .map(|k| k[0].as_str().unwrap())
                .collect::<std::collections::BTreeSet<_>>()
                .len();
            format!(
                "{} key{} in {n} place{}",
                ks.len(),
                plural(ks.len()),
                plural(n)
            )
        };
        row(
            cid,
            "every key of the layer, frame, value and ease",
            ok,
            places(want),
            if ok {
                places(&got)
            } else if got.len() != want.len() {
                format!("{} keys", got.len())
            } else {
                format!("wrong: {}", wrong.join("; "))
            },
        );

        let mut kept = layer_of(&before);
        for base in case["bases"].as_array().unwrap() {
            let at = format!("{}/base", base[0].as_str().unwrap());
            let held = layer.pointer(&at).cloned().unwrap_or(J::Null);
            row(
                cid,
                &format!("{} with no keys left", base[0].as_str().unwrap()),
                same(&held, &base[1]),
                format!("holds {}", base[1]),
                format!("holds {held}"),
            );
            *kept.pointer_mut(&at).unwrap() = base[1].clone();
        }
        let (mut a, mut b) = (after.clone(), before.clone());
        let at = |p: &J| {
            let c = p["compositions"]
                .as_array()
                .unwrap()
                .iter()
                .position(|c| c["id"] == comp)
                .unwrap();
            let l = p["compositions"][c]["layers"]
                .as_array()
                .unwrap()
                .iter()
                .position(|l| l["id"] == lid)
                .unwrap();
            format!("/compositions/{c}/layers/{l}")
        };
        *b.pointer_mut(&at(&before)).unwrap() = kept;
        keyless(&mut a);
        keyless(&mut b);
        let ok = same(&a, &b);
        row(
            cid,
            "nothing else changed",
            ok,
            "the keys only".into(),
            if ok { "the keys only" } else { "more" }.into(),
        );

        if !refused {
            edit_command(&viewer, "edit.undo", None);
            let ok = same(&saved(&viewer), &before);
            row(
                cid,
                "one undo",
                ok,
                "the project as before".into(),
                if ok {
                    "the project as before"
                } else {
                    "different"
                }
                .into(),
            );
        }
    }
    out.push_str(&format!("\nB-616: {passed} of {checks} checks pass.\n"));
    std::fs::write(repo("verification/B-616_key_requests_table.md"), &out).unwrap();
    assert_eq!(
        passed, checks,
        "see verification/B-616_key_requests_table.md"
    );
}
