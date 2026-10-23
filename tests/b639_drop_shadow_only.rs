//! B-639: Drop Shadow's Shadow Only, against D-760.
//!
//! Writes `verification/B-639_drop_shadow_only_table.md`.
//!
//! Every expected pixel is `Fixtures/drop_shadow/expected_drop_shadow.json`, written by
//! `tools/drop_shadow_reference.py` before this code existed and printed in document 25 as
//! FX-SHADOW-024 to 030. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{changed, set, Table};

use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;

fn shadow(only: &str) -> Effect {
    Effect::DropShadow {
        color: "#000000".into(),
        opacity: 50.0,
        direction: 135.0,
        distance: 5.0,
        softness: 0.0,
        shadow_only: only.into(),
    }
}

#[test]
fn b639_drop_shadow_only() {
    let mut t = Table::new(
        "drop_shadow",
        "# B-639: Drop Shadow's Shadow Only\n\nD-760, accepted by the owner on 2026-10-23 in the \
         audit's waves (\"if it is to be a 1 to 1 match, then go for it\"). Every expected pixel \
         is `Fixtures/drop_shadow/expected_drop_shadow.json`, written by \
         `tools/drop_shadow_reference.py` before this code existed and printed in document 25 as \
         FX-SHADOW-024 to 030. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5. \
         FX-SHADOW-001 to 023 are B-58's and did not move.\n",
    );

    t.heading("FX-SHADOW-024 to 030 (document 25)");
    t.fixtures_numbered("expected_drop_shadow.json", 24..=30);

    t.heading("A file from before D-760");
    let old = changed(&t, "fx_shadow_001.json", |_| {});
    t.row(
        "fx_shadow_001.json, which has no `shadow_only`, opens with Shadow Only off",
        &format!("{old:?}"),
        old == shadow("off"),
    );
    let a = t.render(&t.load("fx_shadow_001.json").document, 0, 64);
    let b = t.render(&t.load("fx_shadow_026.json").document, 0, 64);
    t.row(
        "its frame 0 is the frame of fx_shadow_026.json, which says \"off\"",
        if a.data() == b.data() {
            "byte-identical"
        } else {
            "differ"
        },
        a.data() == b.data(),
    );
    let kept = t.saved_parameters("fx_shadow_001.json");
    t.row(
        "saved again, it still has no `shadow_only`",
        &kept.to_string(),
        kept.get("shadow_only").is_none(),
    );
    t.round_trips(&[
        "fx_shadow_001.json",
        "fx_shadow_024.json",
        "fx_shadow_026.json",
        "fx_shadow_029.json",
        "fx_shadow_030.json",
    ]);
    let on = t.saved_parameters("fx_shadow_024.json")["shadow_only"].clone();
    t.row(
        "fx_shadow_024.json is saved with Shadow Only \"on\"",
        &on.to_string(),
        on == "on",
    );
    let off = t.saved_parameters("fx_shadow_026.json")["shadow_only"].clone();
    t.row(
        "fx_shadow_026.json, which wrote \"off\", keeps it",
        &off.to_string(),
        off == "off",
    );

    t.heading("Commands");
    let mut document = t.load("fx_shadow_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("Shadow Only \"On\"", set(shadow("On"))),
            ("Shadow Only \"yes\"", set(shadow("yes"))),
        ],
    );
    t.taken(
        &mut document,
        "fx_shadow_001.json",
        vec![("Shadow Only \"on\"", set(shadow("on")))],
    );
    let mut document = t.load("fx_shadow_001.json").document;
    let _ = document.apply(set(shadow("on")));
    let a = t.render(&document, 0, 64);
    let b = t.render(&t.load("fx_shadow_024.json").document, 0, 64);
    t.row(
        "fx_shadow_001.json with Shadow Only turned on draws fx_shadow_024.json's frame",
        if a.data() == b.data() {
            "byte-identical"
        } else {
            "differ"
        },
        a.data() == b.data(),
    );

    t.heading("The card against the processor");
    let mut gpu = Gpu::new().expect("a usable card");
    t.card_fixtures(
        &mut gpu,
        "expected_drop_shadow.json",
        &[18, 19, 20, 21, 22, 23, 29, 30],
        |e| matches!(e, Effect::DropShadow { .. }),
    );

    t.finish("B-639_drop_shadow_only_table.md");
}
