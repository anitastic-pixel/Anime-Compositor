//! B-812: D-932, Lens Blur nearer After Effects' Camera Lens Blur: Diffraction Fringe and
//! Highlight Saturation, both rules ours.
//!
//! Writes `verification/B-812_lens_parity_table.md`.
//!
//! Every expected pixel is `Fixtures/lens_blur/expected_lens_blur_parity.json`, written by
//! `tools/lens_blur_parity_reference.py` before this code existed and printed in document 25 as
//! FX-LENS-070 to 085. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{Table, MAIN};

use anime_compositor::effects::Effect;
use anime_compositor::model::Id;

/// The first Lens Blur in `file`, at its first value out of range when it has one.
fn lens(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let layer = d
        .project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layers_in_order()
        .find(|l| l.effects.iter().any(|e| matches!(e.effect, Effect::LensBlur { .. })));
    let fx = &layer.expect("a layer with a Lens Blur").effects[0];
    fx.invalid().unwrap_or_else(|| fx.effect.clone())
}

#[test]
fn b812_lens_parity() {
    let mut t = Table::new(
        "lens_blur",
        "# B-812: Lens Blur's Diffraction Fringe and Highlight Saturation\n\nD-932, accepted on \
         2026-10-27 under the owner's request for the tutorial follow-alongs. Every expected pixel \
         is `Fixtures/lens_blur/expected_lens_blur_parity.json`, written by \
         `tools/lens_blur_parity_reference.py` before this code existed and printed in document 25 \
         as FX-LENS-070 to 085. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5. \
         FX-LENS-001 to 069 are B-59's, B-64's and B-238's tables, run again unchanged.\n",
    );

    t.heading("FX-LENS-070 to 085 (document 25)");
    t.fixtures_numbered("expected_lens_blur_parity.json", 70..=85);

    t.heading("Out-of-range settings are refused (what each warns)");
    for (file, want) in [
        ("fx_lens_081.json", "diffraction fringe"),
        ("fx_lens_082.json", "diffraction fringe"),
        ("fx_lens_083.json", "highlight saturation"),
        ("fx_lens_084.json", "highlight saturation"),
        ("fx_lens_085.json", "highlight saturation"),
    ] {
        let why = lens(&t, file).why_invalid();
        t.row(&format!("{file}: the warning names {want}"), &why, why.contains(want));
    }

    t.heading("The file");
    let files: Vec<String> = (70..=85).map(|n| format!("fx_lens_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_lens_072.json");
    t.row(
        "fx_lens_072.json is saved with its fringe at 100 and its saturation at 0, as written",
        &saved.to_string(),
        saved["diffraction_fringe"] == 100.0 && saved["highlight_saturation"] == 0.0,
    );
    let saved = t.saved_parameters("fx_lens_007.json");
    t.row(
        "fx_lens_007.json, from before them, is saved without them",
        &saved.to_string(),
        saved.get("diffraction_fringe").is_none() && saved.get("highlight_saturation").is_none(),
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_lens_071.json", 0),
        ("fx_lens_073.json", 0),
        ("fx_lens_076.json", 0),
        ("fx_lens_079.json", 0),
    ]);

    t.finish("B-812_lens_parity_table.md");
}
