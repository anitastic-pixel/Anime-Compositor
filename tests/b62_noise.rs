//! B-62: the noise in the core, against D-119.
//!
//! Writes `verification/B-62_noise_table.md`.
//!
//! Every expected pixel is `Fixtures/noise/expected_noise.json`, written by
//! `tools/noise_reference.py` before this code existed and printed in document 25 as
//! FX-NOISE-001 to 018. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;
use anime_compositor::model::Id;

fn noise(amount: f64, mode: &str, seed: f64, animate: &str) -> Effect {
    Effect::Noise {
        amount,
        mode: mode.into(),
        seed,
        animate: animate.into(),
        frame: 0,
    }
}

#[test]
fn b62_noise() {
    let mut t = Table::new(
        "noise",
        "# B-62: noise\n\nD-119, accepted by the owner on 2026-09-26 in the batch of ten. \
         Every expected pixel is `Fixtures/noise/expected_noise.json`, written by \
         `tools/noise_reference.py` before this code existed and printed in document 25 as \
         FX-NOISE-001 to 018. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-NOISE-001 to 018 (document 25)");
    t.fixtures("expected_noise.json");

    t.heading("How far it reaches");
    let e = noise(100.0, "color", 7.0, "on");
    let got = e.bounds_expansion();
    t.row(
        "it grows the drawing's bounds by nothing: only pixels that show change",
        &got.to_string(),
        got == 0,
    );
    let mut draft = e.clone();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes none of its settings: the grain is a pixel wide",
        &format!("{draft:?}"),
        draft == e,
    );

    t.heading("The grain's frame");
    for (file, want, what) in [
        ("fx_noise_001.json", 3, "animated, the grain at frame 3 is frame 3's"),
        ("fx_noise_002.json", 0, "not animated, the grain at frame 3 is frame 0's"),
    ] {
        let doc = t.load(file).document;
        let comp = doc.project().composition(&Id::new("comp-main")).unwrap();
        let got = match comp.layer(&Id::new("art")).unwrap().effects[0].at(3).effect {
            Effect::Noise { frame, .. } => frame,
            _ => -1,
        };
        t.row(&format!("{file}: {what}"), &got.to_string(), got == want);
    }

    t.heading("The file");
    t.round_trips(&[
        "fx_noise_001.json",
        "fx_noise_002.json",
        "fx_noise_003.json",
        "fx_noise_006.json",
        "fx_noise_010.json",
        "fx_noise_011.json",
        "fx_noise_013.json",
        "fx_noise_014.json",
        "fx_noise_015.json",
        "fx_noise_016.json",
        "fx_noise_017.json",
        "fx_noise_018.json",
    ]);
    t.shape_refused(
        "fx_noise_001.json",
        "no `animate` at all",
        r##"{"amount": 10, "mode": "mono", "seed": 0}"##,
    );
    t.shape_refused(
        "fx_noise_001.json",
        "an amount written as a word",
        r##"{"amount": "ten", "mode": "mono", "seed": 0, "animate": "on"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_noise_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("amount 101", set(noise(101.0, "mono", 0.0, "on"))),
            ("amount -1", set(noise(-1.0, "mono", 0.0, "on"))),
            ("seed -1", set(noise(10.0, "mono", -1.0, "on"))),
            ("seed 100001", set(noise(10.0, "mono", 100001.0, "on"))),
            ("mode \"Mono\"", set(noise(10.0, "Mono", 0.0, "on"))),
            ("animate \"yes\"", set(noise(10.0, "mono", 0.0, "yes"))),
            ("amount keyed to 150", keys("amount", &[(0, &[10.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_noise_001.json",
        vec![
            ("amount 100 and seed 100000, the tops, in colour and still,", set(noise(100.0, "color", 100000.0, "off"))),
            ("amount 0 and seed 0, the bottoms,", set(noise(0.0, "mono", 0.0, "on"))),
            ("seed keyed from 0 to 3.5", keys("seed", &[(0, &[0.0]), (4, &[3.5])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_noise_001.json", 2), ("fx_noise_012.json", 4)]);

    t.finish("B-62_noise_table.md");
}
