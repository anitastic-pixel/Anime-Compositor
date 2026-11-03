//! B-814: D-934, CC Glass's light type "ae_lights" (the bend alone, unlit) and edge look "rim"
//! (the flat surface kept, every slope darkened), after After Effects' CC Glass in the Liquid
//! Glass tutorial; the rules are ours.
//!
//! Writes `verification/B-814_glass_look_table.md`.
//!
//! Every expected pixel is `Fixtures/glass/expected_glass_look.json`, written by
//! `tools/glass_look_reference.py` before this code existed and printed in document 25 as
//! FX-GLASS-028 to 038. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{Table, MAIN};

use anime_compositor::effects::Effect;
use anime_compositor::model::Id;

/// The CC Glass in `file`, as written when it is refused.
fn glass(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let layer = d
        .project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layers_in_order()
        .find(|l| l.effects.iter().any(|e| matches!(e.effect, Effect::Glass { .. })));
    let fx = layer
        .expect("a layer with a CC Glass")
        .effects
        .iter()
        .find(|e| matches!(e.effect, Effect::Glass { .. }))
        .unwrap();
    fx.invalid().unwrap_or_else(|| fx.effect.clone())
}

#[test]
fn b814_glass_look() {
    let mut t = Table::new(
        "glass",
        "# B-814: CC Glass's AE Lights and rim look\n\nD-934, accepted on 2026-10-27 under the \
         owner's request for the tutorial follow-alongs. Every expected pixel is \
         `Fixtures/glass/expected_glass_look.json`, written by `tools/glass_look_reference.py` \
         before this code existed and printed in document 25 as FX-GLASS-028 to 038. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 2e-5. FX-GLASS-001 to 027 are B-441's table, \
         run again unchanged.\n",
    );

    t.heading("FX-GLASS-028 to 038 (document 25)");
    t.fixtures("expected_glass_look.json");

    t.heading("A wrong word is refused (what it warns)");
    let why = glass(&t, "fx_glass_038.json").why_invalid();
    t.row(
        "fx_glass_038.json: the warning names the edge look",
        &why,
        why.contains("edge look"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_glass_028.json",
        "fx_glass_034.json",
        "fx_glass_036.json",
        "fx_glass_038.json",
    ]);
    let saved = t.saved_parameters("fx_glass_028.json");
    t.row(
        "fx_glass_028.json is saved with its edge look rim",
        &saved.to_string(),
        saved["light_look"] == "rim",
    );
    let saved = t.saved_parameters("fx_glass_001.json");
    t.row(
        "fx_glass_001.json, from before it, is saved without it",
        &saved.to_string(),
        saved.get("light_look").is_none(),
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_glass_028.json", 0),
        ("fx_glass_030.json", 0),
        ("fx_glass_034.json", 0),
    ]);

    t.finish("B-814_glass_look_table.md");
}
