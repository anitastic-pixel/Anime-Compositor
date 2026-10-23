//! B-624: Wave Warp gains After Effects' Circle and Pinning, against D-745.
//!
//! Writes `verification/B-624_wave_warp_table.md`.
//!
//! Every expected pixel is `Fixtures/wave_warp/expected_wave_warp.json`, written by
//! `tools/wave_warp_reference.py` before this code and printed in document 25 as FX-WAVE-001 to
//! 038. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{set, settings, Table};

use anime_compositor::effects::Effect;

/// The start, with a shape and a pinning.
fn wave(shape: &str, pinning: &str) -> Effect {
    Effect::WaveWarp {
        shape: shape.to_string(),
        height: 10.0,
        width: 40.0,
        direction: 90.0,
        speed: 0.0,
        phase: 0.0,
        edges: "transparent".to_string(),
        pinning: pinning.to_string(),
        frame: 0,
    }
}

#[test]
fn b624_wave_warp() {
    let mut t = Table::new(
        "wave_warp",
        "# B-624: Wave Warp gains After Effects' Circle and Pinning\n\nD-745, accepted on \
         2026-10-23 by the owner's message \"Proceed, if an effect is more like a \
         inspiration/modified version of a AE effect, only improve it.\" Wave Warp is inspired \
         by After Effects' (D-149), so nothing it drew before changes: Width keeps its meaning, \
         one whole wave, about twice After Effects' Width. Shape gains circle and a new Pinning \
         holds edges of the drawing still. Every expected pixel is \
         `Fixtures/wave_warp/expected_wave_warp.json`, written by \
         `tools/wave_warp_reference.py` before this code and printed in document 25 as \
         FX-WAVE-001 to 038. The build's frame is compared sample by sample against the \
         catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-WAVE-001 to 038 (document 25)");
    t.fixtures("expected_wave_warp.json");

    t.heading("A project saved before Pinning");
    let loaded = t.load("fx_wave_003.json");
    let got = settings(&loaded.document);
    t.row(
        "fx_wave_003.json, written before D-745 with no pinning, opens with pinning none",
        &got,
        got.contains("pinning: \"none\""),
    );
    let saved = t.saved_parameters("fx_wave_003.json");
    t.row(
        "saving it again writes no pinning",
        &saved.to_string(),
        saved.get("pinning").is_none(),
    );
    let saved = t.saved_parameters("fx_wave_034.json");
    t.row(
        "a file that wrote pinning none keeps it when saved",
        &saved.to_string(),
        saved.get("pinning").and_then(|p| p.as_str()) == Some("none"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_wave_003.json",
        "fx_wave_025.json",
        "fx_wave_027.json",
        "fx_wave_032.json",
        "fx_wave_034.json",
        "fx_wave_036.json",
        "fx_wave_037.json",
        "fx_wave_038.json",
    ]);
    t.shape_refused(
        "fx_wave_027.json",
        "a pinning that is a number",
        r#"{"shape": "sine", "height": 2, "width": 8, "direction": 90, "speed": 0, "phase": 0, "edges": "transparent", "pinning": 4}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_wave_003.json").document;
    t.refused(
        &mut document,
        vec![
            ("pinning \"center\"", set(wave("sine", "center"))),
            ("pinning \"All Edges\"", set(wave("sine", "All Edges"))),
            ("pinning \"\"", set(wave("sine", ""))),
            ("shape \"square\"", set(wave("square", "none"))),
        ],
    );
    t.taken(
        &mut document,
        "fx_wave_003.json",
        vec![
            ("shape circle,", set(wave("circle", "none"))),
            ("pinning all_edges,", set(wave("sine", "all_edges"))),
            ("pinning left_edge,", set(wave("sine", "left_edge"))),
            ("pinning top_edge,", set(wave("sine", "top_edge"))),
            ("pinning right_edge,", set(wave("sine", "right_edge"))),
            ("pinning bottom_edge,", set(wave("sine", "bottom_edge"))),
            ("pinning horizontal_edges,", set(wave("triangle", "horizontal_edges"))),
            ("pinning vertical_edges,", set(wave("circle", "vertical_edges"))),
            ("pinning none,", set(wave("sine", "none"))),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_wave_025.json", 0),
        ("fx_wave_027.json", 0),
        ("fx_wave_035.json", 0),
        ("fx_wave_036.json", 2),
    ]);

    t.finish("B-624_wave_warp_table.md");
}
