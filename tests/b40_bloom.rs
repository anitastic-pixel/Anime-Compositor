//! B-40: bloom in the core, against D-96.
//!
//! Writes `verification/B-40_bloom_table.md`.
//!
//! Every expected pixel is `Fixtures/bloom/expected_bloom.json`, written by
//! `tools/bloom_reference.py` before this code existed and printed in document 25 as
//! FX-BLOOM-001 to 028. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn bloom(radius: f64, intensity: f64, streaks: &str, length: f64, angle: f64) -> Effect {
    Effect::Bloom {
        threshold: 80.0,
        radius,
        intensity,
        streaks: streaks.to_string(),
        length,
        angle,
    }
}

#[test]
fn b40_bloom() {
    let mut t = Table::new(
        "bloom",
        "# B-40: bloom\n\nD-96, accepted by the owner on 2026-09-25. Every expected pixel is \
         `Fixtures/bloom/expected_bloom.json`, written by `tools/bloom_reference.py` before this \
         code existed and printed in document 25 as FX-BLOOM-001 to 028. The build's frame is \
         compared sample by sample; the answer is the largest difference over all of them, \
         against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-BLOOM-001 to 028 (document 25)");
    t.fixtures("expected_bloom.json");

    t.heading("How far it reaches");
    for (what, e, want) in [
        (
            "radius 20, no streaks, reaches the radius",
            bloom(20.0, 1.0, "none", 60.0, 0.0),
            20,
        ),
        (
            "radius 20 with a star of length 60 reaches the length",
            bloom(20.0, 1.0, "star", 60.0, 0.0),
            60,
        ),
        (
            "a cross of length 2.5 at radius 0 reaches 3",
            bloom(0.0, 1.0, "cross", 2.5, 0.0),
            3,
        ),
        (
            "with no streaks the length reaches nowhere",
            bloom(0.0, 1.0, "none", 500.0, 0.0),
            0,
        ),
    ] {
        let got = e.bounds_expansion();
        t.row(what, &got.to_string(), got == want);
    }
    let mut draft = bloom(20.0, 1.0, "cross", 60.0, 30.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the radius and the length, and nothing else",
        &format!("{draft:?}"),
        draft == bloom(10.0, 1.0, "cross", 30.0, 30.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_bloom_001.json",
        "fx_bloom_007.json",
        "fx_bloom_014.json",
        "fx_bloom_015.json",
        "fx_bloom_016.json",
        "fx_bloom_020.json",
        "fx_bloom_025.json",
        "fx_bloom_026.json",
        "fx_bloom_027.json",
        "fx_bloom_028.json",
    ]);
    t.shape_refused(
        "fx_bloom_001.json",
        "no `streaks` at all",
        r##"{"threshold": 80, "radius": 4, "intensity": 1, "length": 6, "angle": 0}"##,
    );
    t.shape_refused(
        "fx_bloom_001.json",
        "an angle written as a word",
        r##"{"threshold": 80, "radius": 4, "intensity": 1, "streaks": "cross", "length": 6,
            "angle": "north"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_bloom_001.json").document;
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = bloom(4.0, 1.0, "none", 6.0, 0.0);
        f(&mut e);
        set(e)
    };
    t.refused(
        &mut document,
        vec![
            ("threshold 101", with(&|e| e.set("threshold", &[101.0]))),
            ("radius 501", with(&|e| e.set("radius", &[501.0]))),
            ("radius -1", with(&|e| e.set("radius", &[-1.0]))),
            ("intensity 11", with(&|e| e.set("intensity", &[11.0]))),
            ("length 501", with(&|e| e.set("length", &[501.0]))),
            ("angle -3601", with(&|e| e.set("angle", &[-3601.0]))),
            ("streaks \"rays\"", set(bloom(4.0, 1.0, "rays", 6.0, 0.0))),
            ("streaks \"Star\"", set(bloom(4.0, 1.0, "Star", 6.0, 0.0))),
            (
                "length keyed to 600",
                keys("length", &[(0, &[0.0]), (4, &[600.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_bloom_001.json",
        vec![
            (
                "radius 500, intensity 10, a star of length 500 at angle 3600, the tops of the \
                 ranges,",
                set(Effect::Bloom {
                    threshold: 100.0,
                    radius: 500.0,
                    intensity: 10.0,
                    streaks: "star".to_string(),
                    length: 500.0,
                    angle: 3600.0,
                }),
            ),
            (
                "everything at the bottom of its range, angle -3600,",
                set(Effect::Bloom {
                    threshold: 0.0,
                    radius: 0.0,
                    intensity: 0.0,
                    streaks: "none".to_string(),
                    length: 0.0,
                    angle: -3600.0,
                }),
            ),
            (
                "angle keyed from 0 to 90",
                keys("angle", &[(0, &[0.0]), (4, &[90.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_bloom_012.json", 0), ("fx_bloom_018.json", 3)]);

    t.finish("B-40_bloom_table.md");
}
