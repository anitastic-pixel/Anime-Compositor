//! B-813: D-933, Light Sweep's edge look "rim": a soft rim all round the drawing, brightest
//! facing the light, after the edge After Effects' CC Light Sweep draws; the rule is ours.
//!
//! Writes `verification/B-813_light_sweep_rim_table.md`.
//!
//! Every expected pixel is `Fixtures/light_sweep/expected_light_sweep_rim.json`, written by
//! `tools/light_sweep_rim_reference.py` before this code existed and printed in document 25 as
//! FX-SWEEP-027 to 036. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{Table, MAIN};

use anime_compositor::effects::Effect;
use anime_compositor::model::Id;

/// The Light Sweep in `file`, as written when it is refused.
fn sweep(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let layer = d
        .project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layers_in_order()
        .find(|l| l.effects.iter().any(|e| matches!(e.effect, Effect::LightSweep { .. })));
    let fx = &layer.expect("a layer with a Light Sweep").effects[0];
    fx.invalid().unwrap_or_else(|| fx.effect.clone())
}

#[test]
fn b813_light_sweep_rim() {
    let mut t = Table::new(
        "light_sweep",
        "# B-813: Light Sweep's rim\n\nD-933, accepted on 2026-10-27 under the owner's request for \
         the tutorial follow-alongs. Every expected pixel is \
         `Fixtures/light_sweep/expected_light_sweep_rim.json`, written by \
         `tools/light_sweep_rim_reference.py` before this code existed and printed in document 25 \
         as FX-SWEEP-027 to 036. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5. \
         FX-SWEEP-001 to 026 are B-134's table, run again unchanged.\n",
    );

    t.heading("FX-SWEEP-027 to 036 (document 25)");
    t.fixtures("expected_light_sweep_rim.json");

    t.heading("A wrong word is refused (what it warns)");
    let why = sweep(&t, "fx_sweep_036.json").why_invalid();
    t.row(
        "fx_sweep_036.json: the warning names the edge look",
        &why,
        why.contains("edge look"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_sweep_027.json",
        "fx_sweep_031.json",
        "fx_sweep_034.json",
        "fx_sweep_036.json",
    ]);
    let saved = t.saved_parameters("fx_sweep_031.json");
    t.row(
        "fx_sweep_031.json is saved with its edge look rim",
        &saved.to_string(),
        saved["edge_look"] == "rim",
    );
    let saved = t.saved_parameters("fx_sweep_001.json");
    t.row(
        "fx_sweep_001.json, from before it, is saved without it",
        &saved.to_string(),
        saved.get("edge_look").is_none(),
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_sweep_027.json", 0),
        ("fx_sweep_031.json", 0),
        ("fx_sweep_035.json", 0),
    ]);

    t.finish("B-813_light_sweep_rim_table.md");
}
