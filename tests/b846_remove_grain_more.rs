//! B-846: D-966 Remove Grain's further groups, after After Effects' Remove Grain: Fine Tuning
//! (Chroma Suppression, Texture, Noise Size Bias, Clean Solid Areas), Channel Noise Reduction,
//! Temporal Filtering, Viewing Mode and the Preview Region.
//!
//! Writes `verification/D-966_remove_grain_more_table.md`.
//!
//! Every expected pixel is `Fixtures/remove_grain/expected_remove_grain_more.json`, written by
//! `tools/remove_grain_more_reference.py` before this code existed and printed in document 25 as
//! FX-REMOVEGRAIN-022 to 040. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{Table, MAIN};

use anime_compositor::model::Id;

use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;

fn is_remove(e: &Effect) -> bool {
    matches!(e, Effect::RemoveGrain { .. })
}

#[test]
fn b846_remove_grain_more() {
    let mut t = Table::new(
        "remove_grain",
        "# D-966: Remove Grain's further groups\n\nB-846, after After Effects' Remove Grain: \
         Channel Noise Reduction scales each channel's measured grain; Chroma Suppression \
         divides the colour part of a difference by 1 + it; Noise Size Bias widens the later \
         passes and narrows the earlier ones; Clean Solid Areas is one more pass; Texture moves \
         back toward the layer as given; Temporal Filtering first averages each pixel with the \
         frames before and after where their colour is near; the Viewing Mode shows the result, \
         a Preview region inside the layer as given, the sampled area or the matte. Every \
         expected pixel is `Fixtures/remove_grain/expected_remove_grain_more.json`, written by \
         `tools/remove_grain_more_reference.py` before this code existed and printed in \
         document 25 as FX-REMOVEGRAIN-022 to 040. Tolerance 2e-5.\n",
    );

    t.heading("FX-REMOVEGRAIN-022 to 040 (document 25)");
    t.fixtures("expected_remove_grain_more.json");

    t.heading("FX-REMOVEGRAIN-001 to 021 still draw as before (D-455)");
    t.fixtures("expected_remove_grain.json");

    t.heading("The file");
    let files: Vec<String> = (22..=40)
        .map(|n| format!("fx_removegrain_{n:03}.json"))
        .collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let old = t.saved_parameters("fx_removegrain_001.json");
    t.row(
        "fx_removegrain_001.json, written before D-966, is saved without any of the further settings",
        &old.to_string(),
        ["chroma_suppression", "texture", "viewing_mode", "temporal_filtering", "preview_center"]
            .iter()
            .all(|k| old.get(k).is_none()),
    );
    for n in 37..=40 {
        let file = format!("fx_removegrain_{n:03}.json");
        let loaded = t.load(&file);
        let why = loaded
            .document
            .project()
            .composition(&Id::new(MAIN))
            .unwrap()
            .layer(&Id::new("holder"))
            .unwrap()
            .effects[0]
            .effect
            .why_invalid();
        t.row(
            &format!("{file} is refused in a sentence that starts Remove Grain"),
            &why,
            why.starts_with("Remove Grain"),
        );
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_removegrain_022.json", 0),
        ("fx_removegrain_026.json", 0),
        ("fx_removegrain_030.json", 0),
        ("fx_removegrain_034.json", 2),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Preview, Noise Samples, Blending Matte and Temporal Filtering stay on the processor; 037 to
    // 040 are refused.
    let none = [30, 31, 32, 33, 34, 36, 37, 38, 39, 40];
    t.card_fixtures(&mut gpu, "expected_remove_grain_more.json", &none, is_remove);

    t.finish("D-966_remove_grain_more_table.md");
}
