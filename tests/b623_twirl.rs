//! B-623: Twirl turns as After Effects' does, against D-744.
//!
//! Writes `verification/B-623_twirl_table.md`.
//!
//! Every expected pixel is `Fixtures/twirl/expected_twirl.json`, written by
//! `tools/twirl_reference.py` before this code and printed in document 25 as FX-TWIRL-001 to
//! 027. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, settings, Table};

use anime_compositor::effects::Effect;

fn twirl(angle: f64, radius: f64, center: [f64; 2], twirl_radius: f64) -> Effect {
    Effect::Twirl {
        angle,
        radius,
        center,
        twirl_radius,
    }
}

const MIDDLE: [f64; 2] = [50.0, 50.0];

#[test]
fn b623_twirl() {
    let mut t = Table::new(
        "twirl",
        "# B-623: Twirl turns as After Effects' does\n\nD-744, accepted on 2026-10-23 by the \
         owner's message \"Proceed, if an effect is more like a inspiration/modified version of \
         a AE effect, only improve it. But it is to be a 1 to 1 match, then go for it.\" The \
         turn now eases out with a smoothstep, and a new Twirl Radius gives the reach in per \
         cent of the layer's larger side, as After Effects' does; Radius in pixels stays for \
         projects made before it. Every expected pixel is \
         `Fixtures/twirl/expected_twirl.json`, written by `tools/twirl_reference.py` before \
         this code and printed in document 25 as FX-TWIRL-001 to 027. The build's frame is \
         compared sample by sample against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-TWIRL-001 to 027 (document 25)");
    t.fixtures("expected_twirl.json");

    t.heading("A project saved before Twirl Radius");
    let loaded = t.load("fx_twirl_001.json");
    let got = settings(&loaded.document);
    let want = format!("{:?}", twirl(90.0, 50.0, MIDDLE, 0.0));
    t.row(
        "fx_twirl_001.json, written before D-744 with no twirl_radius, opens with Twirl Radius \
         0, so its Radius of 50 pixels still sets the reach",
        &got,
        got.contains(&want),
    );
    let saved = t.saved_parameters("fx_twirl_001.json");
    t.row(
        "saving it again writes no twirl_radius",
        &saved.to_string(),
        saved.get("twirl_radius").is_none(),
    );

    t.heading("How far it reaches");
    let got = twirl(3600.0, 10000.0, MIDDLE, 100.0).bounds_expansion();
    t.row(
        "it grows the drawing's bounds by nothing",
        &got.to_string(),
        got == 0,
    );
    let mut draft = twirl(90.0, 0.0, MIDDLE, 30.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview keeps Twirl Radius at 30 per cent, as the layer it is a \
         share of is halved too",
        &format!("{draft:?}"),
        draft == twirl(90.0, 0.0, MIDDLE, 30.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_twirl_001.json",
        "fx_twirl_022.json",
        "fx_twirl_023.json",
        "fx_twirl_024.json",
        "fx_twirl_025.json",
        "fx_twirl_026.json",
        "fx_twirl_027.json",
    ]);
    t.shape_refused(
        "fx_twirl_022.json",
        "a twirl radius that is a word",
        r#"{"angle": 90, "radius": 0, "center": [50, 50], "twirl_radius": "wide"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_twirl_022.json").document;
    t.refused(
        &mut document,
        vec![
            ("twirl radius 101", set(twirl(90.0, 0.0, MIDDLE, 101.0))),
            ("twirl radius -1", set(twirl(90.0, 0.0, MIDDLE, -1.0))),
            (
                "twirl radius keyed to 150",
                keys("twirl_radius", &[(0, &[30.0]), (4, &[150.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_twirl_022.json",
        vec![
            ("twirl radius 0,", set(twirl(90.0, 0.0, MIDDLE, 0.0))),
            ("twirl radius 100,", set(twirl(90.0, 0.0, MIDDLE, 100.0))),
            (
                "twirl radius keyed from 0 to 60",
                keys("twirl_radius", &[(0, &[0.0]), (4, &[60.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_twirl_022.json", 0), ("fx_twirl_025.json", 0)]);

    t.finish("B-623_twirl_table.md");
}
