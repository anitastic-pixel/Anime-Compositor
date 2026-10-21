//! B-608: animation presets, against D-739.
//!
//! Writes `verification/B-608_animation_preset_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/animation_preset/` and `Fixtures/starter_presets/animation.fxpreset`, written by
//! `tools/animation_preset_reference.py` before any code: the preset files, `host.json`, the
//! project they are applied to, and `expected_animation_preset.json`, what each layer's saved
//! record must then hold.
//!
//! # What is checked
//!
//! Each file is read as an import reads one. Each preset is applied to its layer of `host.json` at
//! its frame through the commands the window sends, as one step, and the project saved; the
//! layer's record must hold the keys, kinds, expressions, bases, lengths and values the fixture
//! names, frames exactly and numbers within 1e-9. Refused files must give the diagnostic and name
//! the word; a refused application must say the word and send nothing. The build's starter file is
//! the fixture's, reads, has the panel's sentences, and survives Export presets.

use std::fs;

use serde_json::Value as J;

use anime_compositor::model::Id;
use anime_compositor::persist;

mod common;
use common::repo;

/// The value at `path` inside `record`, `-1` counting from a list's end.
fn at<'a>(record: &'a J, path: &str) -> Option<&'a J> {
    path.split('/').try_fold(record, |v, step| match v {
        J::Array(list) => {
            let i: i64 = step.parse().ok()?;
            let i = if i < 0 { list.len() as i64 + i } else { i };
            list.get(usize::try_from(i).ok()?)
        }
        _ => v.get(step),
    })
}

/// The same JSON, numbers within 1e-9.
fn close(a: &J, b: &J) -> bool {
    match (a, b) {
        (J::Number(x), J::Number(y)) => (x.as_f64().unwrap() - y.as_f64().unwrap()).abs() <= 1e-9,
        (J::Array(x), J::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(a, b)| close(a, b))
        }
        (J::Object(x), J::Object(y)) => {
            x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| close(v, w)))
        }
        _ => a == b,
    }
}

fn keys(record: &J) -> J {
    J::Array(
        record["keyframes"]
            .as_array()
            .map(|l| {
                l.iter()
                    .map(|k| J::Array(vec![k["frame"].clone(), k["value"].clone()]))
                    .collect()
            })
            .unwrap_or_default(),
    )
}

#[test]
fn b608_animation_preset() {
    let root = repo("Fixtures/animation_preset");
    let expected: J = serde_json::from_str(
        &fs::read_to_string(root.join("expected_animation_preset.json")).unwrap(),
    )
    .unwrap();
    let host = fs::read_to_string(root.join("host.json")).unwrap();
    let comp = Id::new("comp-main");
    let mut out = String::from(
        "# B-608: animation presets\n\n\
         Written by `tests/b608_animation_preset.rs` from `Fixtures/animation_preset/`, against D-739. Each \
         line is one check: what `tools/animation_preset_reference.py` says the layer's saved record must hold \
         once the preset is applied, and what the build's holds. Keys are [frame, value], scale in per cent.\n\n\
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

    for (cid, c) in expected["cases"].as_object().unwrap() {
        let text = fs::read_to_string(root.join(c["file"].as_str().unwrap())).unwrap();
        let read = persist::read_presets(&text);
        if let Some(id) = c["refused"].as_str() {
            let word = c["names"].as_str().unwrap();
            let should = format!("refused, {id}, naming {word}");
            let built = match &read {
                Ok(_) => "read".to_string(),
                Err(d) => format!("refused, {}: {}", d.id.as_str(), d.message),
            };
            let ok = read
                .as_ref()
                .err()
                .is_some_and(|d| d.id.as_str() == id && d.message.contains(word));
            row(cid, "the file", ok, should, built);
            continue;
        }
        let presets = match read {
            Ok(p) => p,
            Err(d) => {
                row(cid, "the file reads", false, "read".into(), d.message);
                continue;
            }
        };
        let preset = match c["preset"].as_str() {
            Some(name) => presets.iter().find(|p| p.name == name),
            None => presets.first(),
        }
        .unwrap();
        let mut loaded = persist::load_str(&host).expect("the host reads");
        let layer_id = c["layer"].as_str().unwrap();
        let frame = c["at"].as_i64().unwrap() as i32;
        let layer = loaded.document.project().compositions[0]
            .layer(&Id::new(layer_id))
            .unwrap()
            .clone();
        let commands = preset.commands(&comp, &layer, frame, 1);
        if let Some(word) = c["apply_refused"].as_str() {
            let ok = commands.as_ref().err().is_some_and(|s| s.contains(word));
            row(
                cid,
                "applied",
                ok,
                format!("refused, naming {word}; nothing sent"),
                match commands {
                    Ok(c) => format!("{} commands", c.len()),
                    Err(s) => format!("refused: {s}"),
                },
            );
            continue;
        }
        let applied = commands.map_err(|s| s.to_string()).and_then(|c| {
            loaded
                .document
                .apply_all(c)
                .map(|_| ())
                .map_err(|d| d.message)
        });
        if let Err(said) = applied {
            row(cid, "applied", false, "applied".into(), said);
            continue;
        }
        let steps = loaded.document.undo_depth();
        row(
            cid,
            "steps to undo",
            steps == 1,
            "1".into(),
            steps.to_string(),
        );
        let saved: J = serde_json::from_str(&persist::to_json(
            loaded.document.project(),
            &loaded.preserved,
        ))
        .unwrap();
        let record = saved["compositions"][0]["layers"]
            .as_array()
            .unwrap()
            .iter()
            .find(|l| l["id"] == layer_id)
            .unwrap();
        for e in c["expect"].as_array().unwrap() {
            let path = e["path"].as_str().unwrap();
            let got = at(record, path).cloned().unwrap_or(J::Null);
            for (what, want) in e.as_object().unwrap() {
                let built = match what.as_str() {
                    "path" => continue,
                    "keys" => keys(&got),
                    "frames" => J::Array(
                        got["keyframes"]
                            .as_array()
                            .map(|l| l.iter().map(|k| k["frame"].clone()).collect())
                            .unwrap_or_default(),
                    ),
                    "kinds" => J::Array(
                        got["keyframes"]
                            .as_array()
                            .map(|l| {
                                l.iter()
                                    .map(|k| k.get("kind").cloned().unwrap_or("bezier".into()))
                                    .collect()
                            })
                            .unwrap_or_default(),
                    ),
                    "expression" => got["expression"]["text"].clone(),
                    "base" => got["base"].clone(),
                    "length" => got.as_array().map_or(J::Null, |l| l.len().into()),
                    "equals" => got.clone(),
                    other => panic!("{cid}: no check called {other}"),
                };
                row(
                    cid,
                    &format!("{path}: {what}"),
                    close(want, &built),
                    want.to_string(),
                    built.to_string(),
                );
            }
        }
    }

    // The starter set the build carries.
    let fixture = fs::read_to_string(repo("Fixtures/starter_presets/animation.fxpreset"))
        .unwrap()
        .replace("\r\n", "\n");
    let shipped = persist::STARTER_ANIMATIONS.replace("\r\n", "\n");
    row(
        "starters",
        "the build carries the fixture file",
        shipped == fixture,
        "the same text".into(),
        if shipped == fixture {
            "the same text"
        } else {
            "different text"
        }
        .into(),
    );
    let starters = persist::read_presets(persist::STARTER_ANIMATIONS).unwrap_or_default();
    for (cid, c) in expected["cases"].as_object().unwrap() {
        let (Some(name), Some(about)) = (c["preset"].as_str(), c["about"].as_str()) else {
            continue;
        };
        let i = starters.iter().position(|p| p.name == name);
        let built = i
            .and_then(|i| persist::STARTER_ANIMATIONS_ABOUT.get(i))
            .copied()
            .unwrap_or("none");
        row(
            cid,
            "the sentence the panel shows",
            built == about,
            about.into(),
            built.into(),
        );
    }
    row(
        "starters",
        "how many",
        starters.len() == 12,
        "12".into(),
        starters.len().to_string(),
    );
    let list: J =
        serde_json::from_str::<J>(persist::STARTER_ANIMATIONS).unwrap()["presets"].clone();
    let back = persist::write_presets(&list.to_string())
        .map_err(|d| d.message)
        .and_then(|t| persist::read_presets(&t).map_err(|d| d.message));
    row(
        "starters",
        "Export presets, then Import presets",
        back.as_ref().is_ok_and(|b| *b == starters),
        "the same twelve".into(),
        match &back {
            Ok(b) if *b == starters => "the same twelve".into(),
            Ok(b) => format!("{} different", b.len()),
            Err(said) => format!("refused: {said}"),
        },
    );

    out.push_str(&format!("\nB-608: {passed} of {checks} checks pass.\n"));
    fs::write(repo("verification/B-608_animation_preset_table.md"), &out).unwrap();
    assert_eq!(
        passed, checks,
        "see verification/B-608_animation_preset_table.md"
    );
}
