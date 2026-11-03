//! B-817: D-937, Add Grain's Viewing Mode, Final Output or Preview, after After Effects' Add
//! Grain, whose Preview shows the grain in a 200 by 200 box framed in white.
//!
//! Writes `verification/B-817_grain_view_table.md`.
//!
//! Every expected pixel is `Fixtures/addgrain/expected_addgrain_view.json`, written by
//! `tools/addgrain_view_reference.py` before this code existed and printed in document 25 as
//! FX-ADDGRAIN-042 to 047. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{Table, MAIN};

use anime_compositor::effects::Effect;
use anime_compositor::model::Id;

/// The Add Grain in `file`, as written when it is refused.
fn grain(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let layer = d
        .project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layers_in_order()
        .find(|l| l.effects.iter().any(|e| matches!(e.effect, Effect::AddGrain { .. })));
    let fx = layer
        .expect("a layer with an Add Grain")
        .effects
        .iter()
        .find(|e| matches!(e.effect, Effect::AddGrain { .. }))
        .unwrap();
    fx.invalid().unwrap_or_else(|| fx.effect.clone())
}

#[test]
fn b817_grain_view() {
    let mut t = Table::new(
        "addgrain",
        "# B-817: Add Grain's Viewing Mode\n\nD-937, accepted on 2026-10-27 under the owner's \
         request for the tutorial follow-alongs. Every expected pixel is \
         `Fixtures/addgrain/expected_addgrain_view.json`, written by \
         `tools/addgrain_view_reference.py` before this code existed and printed in document 25 \
         as FX-ADDGRAIN-042 to 047. The build's frame is compared sample by sample; the answer \
         is the largest difference over all of them, against the catalogue's tolerance.\n",
    );

    t.heading("FX-ADDGRAIN-042 to 047 (document 25)");
    t.fixtures("expected_addgrain_view.json");

    t.heading("A wrong word is refused (what it warns)");
    let why = grain(&t, "fx_addgrain_047.json").why_invalid();
    t.row("fx_addgrain_047.json: the warning names viewing mode", &why, why.contains("viewing mode"));

    t.heading("The file");
    t.round_trips(&["fx_addgrain_042.json", "fx_addgrain_043.json", "fx_addgrain_047.json"]);
    let saved = t.saved_parameters("fx_addgrain_042.json");
    t.row(
        "fx_addgrain_042.json is saved with viewing mode preview",
        &saved.to_string(),
        saved["viewing_mode"] == "preview",
    );
    let saved = t.saved_parameters("fx_addgrain_001.json");
    t.row(
        "fx_addgrain_001.json, from before it, is saved without the word",
        &saved.to_string(),
        saved.get("viewing_mode").is_none(),
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_addgrain_042.json", 0), ("fx_addgrain_046.json", 0)]);

    t.finish("B-817_grain_view_table.md");
}
