//! B-103: speed lines in the core, against D-160.
//!
//! Writes `verification/B-103_speed_lines_table.md`.
//!
//! Every expected pixel is `Fixtures/speed_lines/expected_speed_lines.json`, written by
//! `tools/speed_lines_reference.py` before this code existed and printed in document 25 as
//! FX-SPEED-001 to 029. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

/// `[count, thickness, inner, inner_jitter, angle_jitter, seed, hold, opacity]`.
fn lines(center: [f64; 2], color: &str, n: [f64; 8]) -> Effect {
    Effect::SpeedLines {
        center,
        color: color.to_string(),
        count: n[0],
        thickness: n[1],
        inner: n[2],
        inner_jitter: n[3],
        angle_jitter: n[4],
        seed: n[5],
        hold: n[6],
        opacity: n[7],
        frame: 0,
    }
}

const MIDDLE: [f64; 2] = [50.0, 50.0];
const START: [f64; 8] = [120.0, 1.5, 150.0, 40.0, 50.0, 0.0, 2.0, 100.0];

fn with(i: usize, v: f64) -> Effect {
    let mut n = START;
    n[i] = v;
    lines(MIDDLE, "#000000", n)
}

#[test]
fn b103_speed_lines() {
    let mut t = Table::new(
        "speed_lines",
        "# B-103: speed lines\n\nD-160, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the twenty-seventh of the third batch. Every expected pixel is \
         `Fixtures/speed_lines/expected_speed_lines.json`, written by \
         `tools/speed_lines_reference.py` before this code existed and printed in document 25 as \
         FX-SPEED-001 to 029. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-SPEED-001 to 029 (document 25)");
    t.fixtures("expected_speed_lines.json");

    t.heading("How far it reaches");
    let got = with(1, 30.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = with(2, 150.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the inner edge, 150 to 75, and nothing else",
        &format!("{draft:?}"),
        draft == with(2, 75.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_speed_001.json",
        "fx_speed_003.json",
        "fx_speed_011.json",
        "fx_speed_017.json",
        "fx_speed_018.json",
        "fx_speed_019.json",
        "fx_speed_022.json",
        "fx_speed_023.json",
        "fx_speed_024.json",
        "fx_speed_025.json",
        "fx_speed_026.json",
        "fx_speed_027.json",
        "fx_speed_028.json",
        "fx_speed_029.json",
    ]);
    t.shape_refused(
        "fx_speed_001.json",
        "no `count` at all",
        r##"{"center": [50, 50], "color": "#000000", "thickness": 1.5, "inner": 150,
            "inner_jitter": 40, "angle_jitter": 50, "seed": 0, "hold": 2, "opacity": 100}"##,
    );
    t.shape_refused(
        "fx_speed_001.json",
        "a colour that is a number",
        r#"{"center": [50, 50], "color": 0, "count": 120, "thickness": 1.5, "inner": 150,
            "inner_jitter": 40, "angle_jitter": 50, "seed": 0, "hold": 2, "opacity": 100}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_speed_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("count 3", set(with(0, 3.0))),
            ("thickness 31", set(with(1, 31.0))),
            ("inner -1", set(with(2, -1.0))),
            ("inner jitter 101", set(with(3, 101.0))),
            ("angle jitter 101", set(with(4, 101.0))),
            ("seed 100001", set(with(5, 100001.0))),
            ("hold 0", set(with(6, 0.0))),
            ("opacity 101", set(with(7, 101.0))),
            ("centre 50, 1001", set(lines([50.0, 1001.0], "#000000", START))),
            ("colour \"#12345\"", set(lines(MIDDLE, "#12345", START))),
            ("opacity keyed to 150", keys("opacity", &[(0, &[100.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_speed_001.json",
        vec![
            (
                "every number at its bottom,",
                set(lines([-1000.0, -1000.0], "#000000", [4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0])),
            ),
            (
                "every number at its top,",
                set(lines(
                    [1000.0, 1000.0],
                    "#ffffff",
                    [1000.0, 30.0, 100000.0, 100.0, 100.0, 100000.0, 100.0, 100.0],
                )),
            ),
            ("count keyed from 4 to 20", keys("count", &[(0, &[4.0]), (4, &[20.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_speed_003.json", 0),
        ("fx_speed_010.json", 0),
        ("fx_speed_018.json", 4),
    ]);

    t.finish("B-103_speed_lines_table.md");
}
