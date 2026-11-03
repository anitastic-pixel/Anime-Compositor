//! B-815: D-935, Bulge's Antialiasing (Low or High) and Pin All Edges, after After Effects'
//! Bulge in jv4axtpnZps at 37:10; Adobe gives no formula, so the rules are ours.
//!
//! Writes `verification/B-815_bulge_quality_table.md`.
//!
//! Every expected pixel is `Fixtures/bulge/expected_bulge_quality.json`, written by
//! `tools/bulge_quality_reference.py` before this code existed and printed in document 25 as
//! FX-BULGE-024 to 034. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{Table, MAIN};

use anime_compositor::effects::Effect;
use anime_compositor::model::Id;

/// The Bulge in `file`, as written when it is refused.
fn bulge(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let layer = d
        .project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layers_in_order()
        .find(|l| l.effects.iter().any(|e| matches!(e.effect, Effect::Bulge { .. })));
    let fx = layer
        .expect("a layer with a Bulge")
        .effects
        .iter()
        .find(|e| matches!(e.effect, Effect::Bulge { .. }))
        .unwrap();
    fx.invalid().unwrap_or_else(|| fx.effect.clone())
}

#[test]
fn b815_bulge_quality() {
    let mut t = Table::new(
        "bulge",
        "# B-815: Bulge's Antialiasing and Pin All Edges\n\nD-935, accepted on 2026-10-27 under \
         the owner's request for the tutorial follow-alongs. Every expected pixel is \
         `Fixtures/bulge/expected_bulge_quality.json`, written by \
         `tools/bulge_quality_reference.py` before this code existed and printed in document 25 \
         as FX-BULGE-024 to 034. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance.\n",
    );

    t.heading("FX-BULGE-024 to 034 (document 25)");
    t.fixtures("expected_bulge_quality.json");

    t.heading("A wrong word is refused (what it warns)");
    let why = bulge(&t, "fx_bulge_033.json").why_invalid();
    t.row("fx_bulge_033.json: the warning names antialiasing", &why, why.contains("antialiasing"));
    let why = bulge(&t, "fx_bulge_034.json").why_invalid();
    t.row("fx_bulge_034.json: the warning names pin all edges", &why, why.contains("pin all edges"));

    t.heading("The file");
    t.round_trips(&["fx_bulge_024.json", "fx_bulge_028.json", "fx_bulge_030.json", "fx_bulge_033.json"]);
    let saved = t.saved_parameters("fx_bulge_030.json");
    t.row(
        "fx_bulge_030.json is saved with antialiasing high and pin all edges on",
        &saved.to_string(),
        saved["antialiasing"] == "high" && saved["pin_all_edges"] == "on",
    );
    let saved = t.saved_parameters("fx_bulge_001.json");
    t.row(
        "fx_bulge_001.json, from before them, is saved without either word",
        &saved.to_string(),
        saved.get("antialiasing").is_none() && saved.get("pin_all_edges").is_none(),
    );
    let saved = t.saved_parameters("fx_bulge_026.json");
    t.row(
        "fx_bulge_026.json, which writes Low and Off, keeps them as written",
        &saved.to_string(),
        saved["antialiasing"] == "low" && saved["pin_all_edges"] == "off",
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_bulge_024.json", 0), ("fx_bulge_028.json", 0), ("fx_bulge_030.json", 0)]);

    t.finish("B-815_bulge_quality_table.md");
}
