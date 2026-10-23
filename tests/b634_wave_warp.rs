//! B-634: Wave Warp gains After Effects' remaining wave types, against D-755.
//!
//! Writes `verification/B-634_wave_warp_table.md`.
//!
//! Every expected pixel is `Fixtures/wave_warp/expected_wave_warp.json`, written by
//! `tools/wave_warp_reference.py` before this code and printed in document 25 as FX-WAVE-001 to
//! 047. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{set, settings, Table};

use anime_compositor::effects::Effect;

/// FX-WAVE-003's settings (height 2, width 8), with a shape.
fn wave(shape: &str) -> Effect {
    Effect::WaveWarp {
        shape: shape.to_string(),
        height: 2.0,
        width: 8.0,
        direction: 90.0,
        speed: 0.0,
        phase: 0.0,
        edges: "transparent".to_string(),
        pinning: "none".to_string(),
        frame: 0,
    }
}

#[test]
fn b634_wave_warp() {
    let mut t = Table::new(
        "wave_warp",
        "# B-634: Wave Warp gains After Effects' remaining wave types\n\nD-755, accepted on \
         2026-10-23 by the owner's message \"Proceed, if an effect is more like a \
         inspiration/modified version of a AE effect, only improve it.\" Wave Warp is inspired \
         by After Effects' (D-149), so nothing it drew before changes. Shape gains square, \
         sawtooth, semicircle and uncircle, placed against the sine as After Effects' stills \
         show them, and noise and smooth noise, ours in spirit: fixed, the same at every frame. \
         Every expected pixel is `Fixtures/wave_warp/expected_wave_warp.json`, written by \
         `tools/wave_warp_reference.py` before this code and printed in document 25 as \
         FX-WAVE-001 to 047. The build's frame is compared sample by sample against the \
         catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-WAVE-001 to 047 (document 25)");
    t.fixtures("expected_wave_warp.json");

    t.heading("A project saved before the new shapes");
    let loaded = t.load("fx_wave_003.json");
    let got = settings(&loaded.document);
    t.row(
        "fx_wave_003.json, written before D-755, opens with shape sine as it always did",
        &got,
        got.contains("shape: \"sine\""),
    );
    let saved = t.saved_parameters("fx_wave_025.json");
    t.row(
        "a circle saved before D-755 is saved again as circle",
        &saved.to_string(),
        saved.get("shape").and_then(|s| s.as_str()) == Some("circle"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_wave_003.json",
        "fx_wave_004.json",
        "fx_wave_023.json",
        "fx_wave_025.json",
        "fx_wave_039.json",
        "fx_wave_040.json",
        "fx_wave_041.json",
        "fx_wave_042.json",
        "fx_wave_043.json",
        "fx_wave_044.json",
        "fx_wave_045.json",
        "fx_wave_046.json",
        "fx_wave_047.json",
    ]);

    t.heading("Commands");
    let mut document = t.load("fx_wave_003.json").document;
    t.refused(
        &mut document,
        vec![
            ("shape \"Smooth Noise\"", set(wave("Smooth Noise"))),
            ("shape \"Square\"", set(wave("Square"))),
            ("shape \"hexagon\"", set(wave("hexagon"))),
        ],
    );
    t.taken(
        &mut document,
        "fx_wave_003.json",
        vec![
            ("shape square,", set(wave("square"))),
            ("shape sawtooth,", set(wave("sawtooth"))),
            ("shape semicircle,", set(wave("semicircle"))),
            ("shape uncircle,", set(wave("uncircle"))),
            ("shape noise,", set(wave("noise"))),
            ("shape smooth_noise,", set(wave("smooth_noise"))),
            ("shape sine,", set(wave("sine"))),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_wave_039.json", 0),
        ("fx_wave_041.json", 0),
        ("fx_wave_044.json", 0),
        ("fx_wave_045.json", 2),
        ("fx_wave_046.json", 0),
    ]);

    t.finish("B-634_wave_warp_table.md");
}
