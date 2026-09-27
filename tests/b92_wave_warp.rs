//! B-92: wave warp in the core, against D-149.
//!
//! Writes `verification/B-92_wave_warp_table.md`.
//!
//! Every expected pixel is `Fixtures/wave_warp/expected_wave_warp.json`, written by
//! `tools/wave_warp_reference.py` before this code existed and printed in document 25 as
//! FX-WAVE-001 to 024. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

/// Height, width, direction, speed and phase, then the two words.
fn wave(n: [f64; 5], shape: &str, edges: &str) -> Effect {
    Effect::WaveWarp {
        shape: shape.to_string(),
        height: n[0],
        width: n[1],
        direction: n[2],
        speed: n[3],
        phase: n[4],
        edges: edges.to_string(),
        frame: 0,
    }
}

const START: [f64; 5] = [10.0, 40.0, 90.0, 0.0, 0.0];

/// The settings as they start with one number changed.
fn with(i: usize, v: f64) -> Effect {
    let mut n = START;
    n[i] = v;
    wave(n, "sine", "transparent")
}

#[test]
fn b92_wave_warp() {
    let mut t = Table::new(
        "wave_warp",
        "# B-92: wave warp\n\nD-149, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the sixteenth of the third batch. Every expected pixel is \
         `Fixtures/wave_warp/expected_wave_warp.json`, written by \
         `tools/wave_warp_reference.py` before this code existed and printed in document 25 as \
         FX-WAVE-001 to 024. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-WAVE-001 to 024 (document 25)");
    t.fixtures("expected_wave_warp.json");

    t.heading("How far it reaches");
    let got = with(0, 2.5).bounds_expansion();
    t.row(
        "with transparent edges, height 2.5 grows the drawing's bounds by 3 pixels",
        &got.to_string(),
        got == 3,
    );
    let got = wave([2.5, 40.0, 90.0, 0.0, 0.0], "sine", "repeat").bounds_expansion();
    t.row("with repeat edges it grows them by nothing", &got.to_string(), got == 0);
    let got = with(0, 0.0).bounds_expansion();
    t.row("at height 0 it grows them by nothing", &got.to_string(), got == 0);
    let mut draft = with(0, 10.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the height and the width",
        &format!("{draft:?}"),
        draft == wave([5.0, 20.0, 90.0, 0.0, 0.0], "sine", "transparent"),
    );
    let mut draft = with(1, 1.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft of width 1 holds the width at 1, its range's bottom, rather than \
         leaving the effect out",
        &format!("{draft:?}"),
        draft == wave([5.0, 1.0, 90.0, 0.0, 0.0], "sine", "transparent"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_wave_001.json",
        "fx_wave_004.json",
        "fx_wave_011.json",
        "fx_wave_012.json",
        "fx_wave_013.json",
        "fx_wave_016.json",
        "fx_wave_017.json",
        "fx_wave_018.json",
        "fx_wave_019.json",
        "fx_wave_020.json",
        "fx_wave_021.json",
        "fx_wave_022.json",
        "fx_wave_023.json",
        "fx_wave_024.json",
    ]);
    t.shape_refused(
        "fx_wave_001.json",
        "no `edges` at all",
        r#"{"shape": "sine", "height": 10, "width": 40, "direction": 90, "speed": 0, "phase": 0}"#,
    );
    t.shape_refused(
        "fx_wave_001.json",
        "a height that is a word",
        r#"{"shape": "sine", "height": "tall", "width": 40, "direction": 90, "speed": 0, "phase": 0, "edges": "transparent"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_wave_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("height 1001", set(with(0, 1001.0))),
            ("width 0", set(with(1, 0.0))),
            ("direction 3601", set(with(2, 3601.0))),
            ("speed -361", set(with(3, -361.0))),
            ("phase 100001", set(with(4, 100001.0))),
            ("shape \"square\"", set(wave(START, "square", "transparent"))),
            ("edges \"Repeat\"", set(wave(START, "sine", "Repeat"))),
            ("height keyed to 1500", keys("height", &[(0, &[10.0]), (4, &[1500.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_wave_001.json",
        vec![
            (
                "every number at its bottom,",
                set(wave([0.0, 1.0, -3600.0, -360.0, -100000.0], "triangle", "repeat")),
            ),
            (
                "every number at its top,",
                set(wave([1000.0, 10000.0, 3600.0, 360.0, 100000.0], "sine", "transparent")),
            ),
            ("phase keyed from 0 to 360", keys("phase", &[(0, &[0.0]), (4, &[360.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_wave_001.json", 0),
        ("fx_wave_006.json", 0),
        ("fx_wave_010.json", 2),
        ("fx_wave_011.json", 0),
    ]);

    t.finish("B-92_wave_warp_table.md");
}
