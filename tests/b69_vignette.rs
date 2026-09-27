//! B-69: the vignette in the core, against D-126.
//!
//! Writes `verification/B-69_vignette_table.md`.
//!
//! Every expected pixel is `Fixtures/vignette/expected_vignette.json`, written by
//! `tools/vignette_reference.py` before this code existed and printed in document 25 as
//! FX-VIGNETTE-001 to 025. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn vignette(amount: f64, color: &str, size: f64, roundness: f64, softness: f64, center: [f64; 2]) -> Effect {
    Effect::Vignette {
        amount,
        color: color.to_string(),
        size,
        roundness,
        softness,
        center,
    }
}

#[test]
fn b69_vignette() {
    let mut t = Table::new(
        "vignette",
        "# B-69: vignette\n\nD-126, accepted by the owner on 2026-09-26, the fourth of the second \
         batch of ten. Every expected pixel is `Fixtures/vignette/expected_vignette.json`, \
         written by `tools/vignette_reference.py` before this code existed and printed in \
         document 25 as FX-VIGNETTE-001 to 025. The build's frame is compared sample by sample; \
         the answer is the largest difference over all of them, against the catalogue's \
         tolerance of 2e-5.\n",
    );

    t.heading("FX-VIGNETTE-001 to 025 (document 25)");
    t.fixtures("expected_vignette.json");

    t.heading("How far it reaches");
    let got = vignette(100.0, "#000000", 200.0, 100.0, 100.0, [0.0, 0.0]).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let start = vignette(70.0, "#6450a0", 80.0, 40.0, 30.0, [40.0, 60.0]);
    let mut draft = start.clone();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: the size and the centre are shares of the \
         drawing",
        &format!("{draft:?}"),
        draft == start,
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_vignette_001.json",
        "fx_vignette_004.json",
        "fx_vignette_009.json",
        "fx_vignette_010.json",
        "fx_vignette_012.json",
        "fx_vignette_013.json",
        "fx_vignette_014.json",
        "fx_vignette_018.json",
        "fx_vignette_019.json",
        "fx_vignette_023.json",
        "fx_vignette_024.json",
        "fx_vignette_025.json",
    ]);
    t.shape_refused(
        "fx_vignette_001.json",
        "no `center` at all",
        r##"{"amount": 50, "color": "#000000", "size": 100, "roundness": 0, "softness": 50}"##,
    );
    t.shape_refused(
        "fx_vignette_001.json",
        "a size that is a word",
        r##"{"amount": 50, "color": "#000000", "size": "big", "roundness": 0, "softness": 50, "center": [50, 50]}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_vignette_001.json").document;
    let ok = |f: &dyn Fn(&mut [f64; 5])| {
        let mut v = [50.0, 100.0, 0.0, 50.0, 50.0];
        f(&mut v);
        set(vignette(v[0], "#000000", v[1], v[2], v[3], [v[4], 50.0]))
    };
    t.refused(
        &mut document,
        vec![
            ("amount 101", ok(&|v| v[0] = 101.0)),
            ("size 0.5", ok(&|v| v[1] = 0.5)),
            ("size 201", ok(&|v| v[1] = 201.0)),
            ("roundness -1", ok(&|v| v[2] = -1.0)),
            ("softness 101", ok(&|v| v[3] = 101.0)),
            ("centre 1001, 50", ok(&|v| v[4] = 1001.0)),
            ("colour \"black\"", set(vignette(50.0, "black", 100.0, 0.0, 50.0, [50.0, 50.0]))),
            ("size keyed to 250", keys("size", &[(0, &[100.0]), (4, &[250.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_vignette_001.json",
        vec![
            (
                "amount 100, size 200, roundness 100, softness 100 and centre 1000, 1000, the tops,",
                set(vignette(100.0, "#ffffff", 200.0, 100.0, 100.0, [1000.0, 1000.0])),
            ),
            (
                "amount 0, size 1, roundness 0, softness 0 and centre -1000, -1000, the bottoms,",
                set(vignette(0.0, "#000000", 1.0, 0.0, 0.0, [-1000.0, -1000.0])),
            ),
            ("softness keyed from 50 to 0", keys("softness", &[(0, &[50.0]), (4, &[0.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_vignette_001.json", 0), ("fx_vignette_018.json", 0)]);

    t.finish("B-69_vignette_table.md");
}
