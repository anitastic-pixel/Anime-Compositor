//! B-657: Radio Waves' Parameters Are Set At and Sides up to 128, against D-777.
//!
//! Writes `verification/B-657_radio_waves_birth_table.md`.
//!
//! Every expected pixel is `Fixtures/radio_waves/expected_radio_waves.json`, written by
//! `tools/radio_waves_reference.py` before this code existed and printed in document 25 as
//! FX-RWAVE-028 to 035. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{changed, keys, set, Table};

use anime_compositor::effects::Effect;

#[test]
fn b657_radio_waves_birth() {
    let mut t = Table::new(
        "radio_waves",
        "# B-657: Radio Waves, Parameters Are Set At and Sides up to 128\n\nD-777, accepted on \
         2026-10-24. Every expected pixel is `Fixtures/radio_waves/expected_radio_waves.json`, \
         written by `tools/radio_waves_reference.py` before this code existed and printed in \
         document 25 as FX-RWAVE-028 to 035. The build's frame is compared sample by sample; the \
         answer is the largest difference over all of them, against the catalogue's tolerance of \
         2e-5.\n",
    );

    t.heading("FX-RWAVE-028 to 035 (document 25)");
    t.fixtures_numbered("expected_radio_waves.json", 28..=35);

    t.heading("Projects from before D-777");
    t.fixtures_numbered("expected_radio_waves.json", 15..=15);
    let old = t.saved_parameters("fx_rwave_015.json");
    t.row(
        "fx_rwave_015.json, written before D-777, saves no `parameters_at`: each frame, as before",
        &format!("{:?}", old.get("parameters_at")),
        old.get("parameters_at").is_none(),
    );
    let loaded = format!("{:?}", t.load("fx_rwave_015.json").document.project());
    t.row(
        "and opens as each frame",
        if loaded.contains("parameters_at: \"each_frame\"") { "each_frame" } else { "not each_frame" },
        loaded.contains("parameters_at: \"each_frame\""),
    );

    t.heading("The file");
    t.round_trips(&["fx_rwave_029.json", "fx_rwave_030.json", "fx_rwave_031.json", "fx_rwave_035.json"]);
    let born = t.saved_parameters("fx_rwave_029.json");
    t.row(
        "the waves' birth settings are never saved",
        &format!("{:?}", born.get("births")),
        born.get("births").is_none() && born["parameters_at"] == "birth",
    );

    t.heading("Commands");
    let mut document = t.load("fx_rwave_032.json").document;
    let with = |t: &Table, f: fn(&mut Effect)| changed(t, "fx_rwave_032.json", f);
    t.refused(
        &mut document,
        vec![
            (
                "parameters set at \"death\"",
                set(with(&t, |e| {
                    if let Effect::RadioWaves { parameters_at, .. } = e {
                        *parameters_at = "death".into();
                    }
                })),
            ),
            (
                "sides 128.5",
                set(with(&t, |e| {
                    if let Effect::RadioWaves { sides, .. } = e {
                        *sides = 128.5;
                    }
                })),
            ),
            ("sides keyed to 129", keys("sides", &[(0, &[64.0]), (4, &[129.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_rwave_032.json",
        vec![
            (
                "sides 128",
                set(with(&t, |e| {
                    if let Effect::RadioWaves { sides, .. } = e {
                        *sides = 128.0;
                    }
                })),
            ),
            (
                "each frame",
                set(with(&t, |e| {
                    if let Effect::RadioWaves { parameters_at, .. } = e {
                        *parameters_at = "each_frame".into();
                    }
                })),
            ),
            (
                "the producer point keyed across the layer",
                keys("producer_point", &[(0, &[0.0, 50.0]), (4, &[100.0, 50.0])]),
            ),
        ],
    );

    t.heading("A half-size draft");
    let mut draft = Effect::RadioWaves {
        producer_point: [50.0, 50.0],
        sides: 64.0,
        interval: 2.0,
        expansion: 2.0,
        orientation: 0.0,
        direction: 90.0,
        velocity: 3.0,
        spin: 0.0,
        lifespan: 5.0,
        opacity: 100.0,
        fade_in_time: 0.0,
        fade_out_time: 0.0,
        start_width: 1.0,
        end_width: 4.0,
        profile: "square".into(),
        color: "#ffffff".into(),
        frame: 4,
        parameters_at: "birth".into(),
        births: Some(vec![[
            2.0, 25.0, 50.0, 6.0, 0.0, 90.0, 8.0, 0.0, 5.0, 100.0, 0.0, 0.0, 2.0, 10.0,
        ]]),
    };
    draft.scale_distances(|d| d * 0.5);
    let kept = match &draft {
        Effect::RadioWaves { births: Some(b), .. } => b[0],
        _ => [0.0; 14],
    };
    t.row(
        "halves a wave's expansion, 6 to 3, velocity, 8 to 4, and widths, 2 to 1 and 10 to 5, \
         and keeps its birth frame, producer, lifespan and opacity",
        &format!("{kept:?}"),
        kept == [2.0, 25.0, 50.0, 3.0, 0.0, 90.0, 4.0, 0.0, 5.0, 100.0, 0.0, 0.0, 1.0, 5.0],
    );

    t.finish("B-657_radio_waves_birth_table.md");
}
