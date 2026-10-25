//! B-725: Twirl's fall-off is D-845's, fitted on After Effects twirls from tutorials.
//!
//! Writes `verification/B-725_twirl_tail_table.md`.
//!
//! Every expected pixel is `Fixtures/twirl/expected_twirl.json`, rewritten by
//! `tools/twirl_reference.py` under D-845 before this code and printed in document 25 as
//! FX-TWIRL-001 to 027. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::Table;

use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;

#[test]
fn b725_twirl_tail() {
    let mut t = Table::new(
        "twirl",
        "# B-725: Twirl's fall-off, fitted on After Effects' own\n\nD-845. Twirl turns each \
         pixel by Angle times (e^(-2.25 u^2) - e^(-2.25)) / (1 - e^(-2.25)), u its distance over \
         the reach, in place of D-744's smoothstep: the whole angle at the centre, none at the \
         edge, and about 0.04 of the angle more from 0.6 to 0.9 of the reach, as seven After \
         Effects twirls from six tutorial makers measured. Every expected pixel is \
         `Fixtures/twirl/expected_twirl.json`, written by `tools/twirl_reference.py` before \
         this code and printed in document 25 as FX-TWIRL-001 to 027. The build's frame is \
         compared sample by sample against the catalogue's tolerance of 2e-5, and the card's \
         against the processor's within 1 level of 255.\n",
    );

    t.heading("FX-TWIRL-001 to 027 (document 25), on the processor");
    t.fixtures("expected_twirl.json");

    t.heading("The same cases on the card");
    let mut gpu = Gpu::new().expect("a usable card");
    t.card_fixtures(
        &mut gpu,
        "expected_twirl.json",
        &[3, 4, 16, 17, 18, 19, 20, 21, 26, 27],
        |e| matches!(e, Effect::Twirl { .. }),
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_twirl_022.json", 0), ("fx_twirl_025.json", 0)]);

    t.finish("B-725_twirl_tail_table.md");
}
