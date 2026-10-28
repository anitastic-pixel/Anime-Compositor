//! B-811: D-931, after After Effects' Set Matte (Channel): a layer's matte replaced by one
//! channel of another layer of the composition, named by D-189's layer setting.
//!
//! Writes `verification/B-811_set_matte_table.md`.
//!
//! Every expected pixel is `Fixtures/set_matte/expected_set_matte.json`, written by
//! `tools/set_matte_reference.py` before this code existed and printed in document 25 as
//! FX-SETMATTE-001 to 024. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;

use effect_table::{repo, Table, MAIN};
use serde_json::Value as J;

use anime_compositor::effects::Effect;
use anime_compositor::model::Id;
use anime_compositor::persist;

/// The `holder` layer's Set Matte in `file`.
fn holder_effect(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let layer = d
        .project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layers_in_order()
        .find(|l| l.effects.iter().any(|e| matches!(e.effect, Effect::SetMatte { .. })));
    layer.expect("a layer with a Set Matte").effects[0].effect.clone()
}

#[test]
fn b811_set_matte() {
    let mut t = Table::new(
        "set_matte",
        "# B-811: Set Matte\n\nD-931, accepted on 2026-10-27 under the owner's request for the \
         tutorial follow-alongs. Every expected pixel is `Fixtures/set_matte/expected_set_matte.json`, \
         written by `tools/set_matte_reference.py` before this code existed and printed in \
         document 25 as FX-SETMATTE-001 to 024. The build's frame is compared sample by sample; \
         the answer is the largest difference over all of them, against the catalogue's \
         tolerance of 2e-5.\n",
    );

    t.heading("FX-SETMATTE-001 to 024 (document 25)");
    t.fixtures_numbered("expected_set_matte.json", 1..=24);

    t.heading("Files whose layers read each other (document 25)");
    let expected: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/set_matte/expected_set_matte.json")).unwrap(),
    )
    .unwrap();
    for (file, case) in expected["loads"].as_object().unwrap() {
        let says = case["says"].as_str().unwrap();
        let id = case["refused"].as_str().unwrap();
        let got = persist::load(&t.root.join(file)).err();
        t.row(
            &format!("{file}: {says}"),
            &got.as_ref()
                .map_or("opened".to_string(), |d| format!("{} {}", d.id.as_str(), d.message)),
            got.is_some_and(|d| d.id.as_str() == id),
        );
    }

    t.heading("Wrong words are refused (what each warns)");
    for (file, want) in [
        ("fx_setmatte_019.json", "use for matte"),
        ("fx_setmatte_020.json", "invert matte"),
        ("fx_setmatte_021.json", "stretch matte to fit"),
        ("fx_setmatte_022.json", "composite matte with original"),
        ("fx_setmatte_023.json", "premultiply matte layer"),
        ("fx_setmatte_024.json", "matte layer"),
    ] {
        let why = holder_effect(&t, file).why_invalid();
        t.row(&format!("{file}: the warning names the {want}"), &why, why.contains(want));
    }

    t.heading("The file");
    let files: Vec<String> = (1..=24).map(|n| format!("fx_setmatte_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_setmatte_013.json");
    t.row(
        "fx_setmatte_013.json is saved with its six settings, the layer as written",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 6
            && saved["layer"] == "patch"
            && saved["stretch_matte_to_fit"] == "off",
    );
    let e = holder_effect(&t, "fx_setmatte_002.json");
    t.row(
        "it grows the drawing's bounds by nothing: the layer keeps its size",
        &e.bounds_expansion().to_string(),
        e.bounds_expansion() == 0,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_setmatte_002.json", 0),
        ("fx_setmatte_008.json", 0),
        ("fx_setmatte_013.json", 0),
        ("fx_setmatte_014.json", 0),
        ("fx_setmatte_016.json", 4),
        ("fx_setmatte_017.json", 0),
        ("fx_setmatte_018.json", 0),
    ]);

    t.finish("B-811_set_matte_table.md");
}
