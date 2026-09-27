//! B-106: rain in the core, against D-163.
//!
//! Writes `verification/B-106_rain_table.md`.
//!
//! Every expected pixel is `Fixtures/rain/expected_rain.json`, written by
//! `tools/rain_reference.py` before this code existed and printed in document 25 as FX-RAIN-001
//! to 027. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

/// `[density, spacing, length, width, direction, speed, seed, opacity]`.
fn rain(n: [f64; 8], color: &str) -> Effect {
    Effect::Rain {
        color: color.to_string(),
        density: n[0],
        spacing: n[1],
        length: n[2],
        width: n[3],
        direction: n[4],
        speed: n[5],
        seed: n[6],
        opacity: n[7],
        frame: 0,
    }
}

const START: [f64; 8] = [30.0, 24.0, 20.0, 1.0, 170.0, 30.0, 0.0, 60.0];

fn with(i: usize, v: f64) -> Effect {
    let mut n = START;
    n[i] = v;
    rain(n, "#c8d8ff")
}

#[test]
fn b106_rain() {
    let mut t = Table::new(
        "rain",
        "# B-106: rain\n\nD-163, accepted on 2026-09-26 by the owner's message asking for thirty \
         more effects, the thirtieth of the third batch. Every expected pixel is \
         `Fixtures/rain/expected_rain.json`, written by `tools/rain_reference.py` before this \
         code existed and printed in document 25 as FX-RAIN-001 to 027. The build's frame is \
         compared sample by sample; the answer is the largest difference over all of them, \
         against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-RAIN-001 to 027 (document 25)");
    t.fixtures("expected_rain.json");

    t.heading("How far it reaches");
    let got = with(3, 20.0).bounds_expansion();
    t.row("width 20 grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = rain([30.0, 24.0, 20.0, 2.0, 170.0, 30.0, 5.0, 60.0], "#c8d8ff");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the spacing, length, width and speed, and nothing else",
        &format!("{draft:?}"),
        draft == rain([30.0, 12.0, 10.0, 1.0, 170.0, 15.0, 5.0, 60.0], "#c8d8ff"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_rain_001.json",
        "fx_rain_014.json",
        "fx_rain_016.json",
        "fx_rain_017.json",
        "fx_rain_020.json",
        "fx_rain_021.json",
        "fx_rain_022.json",
        "fx_rain_023.json",
        "fx_rain_024.json",
        "fx_rain_025.json",
        "fx_rain_026.json",
        "fx_rain_027.json",
    ]);
    let params = t.saved_parameters("fx_rain_015.json");
    t.row(
        "fx_rain_015.json, its colour written in capitals, is saved in small letters, as Colour          Key's is",
        &params["color"].to_string(),
        params["color"] == serde_json::json!("#c8d8ff"),
    );
    t.shape_refused(
        "fx_rain_001.json",
        "no `speed` at all",
        r##"{"color": "#c8d8ff", "density": 30, "spacing": 24, "length": 20, "width": 1,
            "direction": 170, "seed": 0, "opacity": 60}"##,
    );
    t.shape_refused(
        "fx_rain_001.json",
        "a density that is a word",
        r##"{"color": "#c8d8ff", "density": "heavy", "spacing": 24, "length": 20, "width": 1,
            "direction": 170, "speed": 30, "seed": 0, "opacity": 60}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_rain_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("density 101", set(with(0, 101.0))),
            ("spacing 1", set(with(1, 1.0))),
            ("length -1", set(with(2, -1.0))),
            ("width 21", set(with(3, 21.0))),
            ("direction 3601", set(with(4, 3601.0))),
            ("speed 1001", set(with(5, 1001.0))),
            ("seed 100001", set(with(6, 100001.0))),
            ("opacity 101", set(with(7, 101.0))),
            ("colour \"#c8d8f\"", set(rain(START, "#c8d8f"))),
            ("colour \"blue\"", set(rain(START, "blue"))),
            ("speed keyed to 1001", keys("speed", &[(0, &[30.0]), (4, &[1001.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_rain_001.json",
        vec![
            (
                "every number at its bottom,",
                set(rain([0.0, 2.0, 0.0, 0.0, -3600.0, 0.0, 0.0, 0.0], "#000000")),
            ),
            (
                "every number at its top,",
                set(rain([100.0, 1000.0, 1000.0, 20.0, 3600.0, 1000.0, 100000.0, 100.0], "#ffffff")),
            ),
            ("opacity keyed from 0 to 100", keys("opacity", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_rain_002.json", 0),
        ("fx_rain_010.json", 4),
        ("fx_rain_019.json", 0),
    ]);

    t.finish("B-106_rain_table.md");
}
