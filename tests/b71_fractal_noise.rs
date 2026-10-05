//! B-71: the fractal noise in the core, against D-128.
//!
//! Writes `verification/B-71_fractal_noise_table.md`.
//!
//! Every expected pixel is `Fixtures/fractal_noise/expected_fractal_noise.json`, written by
//! `tools/fractal_noise_reference.py` before this code existed and printed in document 25 as
//! FX-FRACTAL-001 to 028. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

/// `[size, complexity, contrast, brightness, evolution, speed, seed, opacity]`, the two colours
/// and the blend.
fn fractal(n: [f64; 8], dark: &str, light: &str, blend: &str) -> Effect {
    Effect::FractalNoise {
        size: n[0],
        complexity: n[1],
        contrast: n[2],
        brightness: n[3],
        evolution: n[4],
        speed: n[5],
        seed: n[6],
        dark_color: dark.to_string(),
        light_color: light.to_string(),
        opacity: n[7],
        blend: blend.to_string(),
        fractal_type: "basic".to_string(),
        noise_type: "smooth".to_string(),
        invert: "off".to_string(),
        offset: [0.0, 0.0],
        scale_width: 100.0,
        scale_height: 100.0,
        cycle: 0.0,
        frame: 0,
    }
}

const START: [f64; 8] = [100.0, 4.0, 100.0, 0.0, 0.0, 0.0, 0.0, 100.0];

fn with(i: usize, v: f64) -> Effect {
    let mut n = START;
    n[i] = v;
    fractal(n, "#000000", "#ffffff", "normal")
}

#[test]
fn b71_fractal_noise() {
    let mut t = Table::new(
        "fractal_noise",
        "# B-71: fractal noise\n\nD-128, accepted by the owner on 2026-09-26, the sixth of the \
         second batch of ten. Every expected pixel is \
         `Fixtures/fractal_noise/expected_fractal_noise.json`, written by \
         `tools/fractal_noise_reference.py` before this code existed and printed in document 25 \
         as FX-FRACTAL-001 to 028; D-318's wider ranges are \
         `expected_fractal_noise_d318.json`, FX-FRACTAL-029 to 033. The build's frame is \
         compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-FRACTAL-001 to 033 (document 25)");
    // D-318 widened complexity to 20 and brightness to -200..200, superseding 022 and 024.
    t.fixtures_numbered("expected_fractal_noise.json", 1..=21);
    t.fixtures_numbered("expected_fractal_noise.json", 23..=23);
    t.fixtures_numbered("expected_fractal_noise.json", 25..=28);
    t.superseded("fx_fractal_022.json", "FX-FRACTAL-022", "D-318");
    t.superseded("fx_fractal_024.json", "FX-FRACTAL-024", "D-318");
    t.fixtures("expected_fractal_noise_d318.json");

    t.heading("How far it reaches");
    let got = with(0, 1000.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = with(0, 40.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the size",
        &format!("{draft:?}"),
        draft == with(0, 20.0),
    );
    let mut draft = with(0, 1.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft of size 1 holds the size at 1, its range's bottom, rather than \
         leaving the effect out",
        &format!("{draft:?}"),
        draft == with(0, 1.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_fractal_001.json",
        "fx_fractal_003.json",
        "fx_fractal_011.json",
        "fx_fractal_013.json",
        "fx_fractal_014.json",
        "fx_fractal_017.json",
        "fx_fractal_019.json",
        "fx_fractal_020.json",
        "fx_fractal_021.json",
        "fx_fractal_026.json",
        "fx_fractal_027.json",
        "fx_fractal_028.json",
    ]);
    t.shape_refused(
        "fx_fractal_001.json",
        "no `light_color` at all",
        r##"{"size": 100, "complexity": 4, "contrast": 100, "brightness": 0, "evolution": 0, "speed": 0, "seed": 0, "dark_color": "#000000", "opacity": 100, "blend": "normal"}"##,
    );
    t.shape_refused(
        "fx_fractal_001.json",
        "a contrast that is a word",
        r##"{"size": 100, "complexity": 4, "contrast": "high", "brightness": 0, "evolution": 0, "speed": 0, "seed": 0, "dark_color": "#000000", "light_color": "#ffffff", "opacity": 100, "blend": "normal"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_fractal_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("size 0.5", set(with(0, 0.5))),
            ("complexity 21", set(with(1, 21.0))),
            ("contrast 1001", set(with(2, 1001.0))),
            ("brightness -201", set(with(3, -201.0))),
            ("evolution 100001", set(with(4, 100001.0))),
            ("speed 361", set(with(5, 361.0))),
            ("seed -1", set(with(6, -1.0))),
            ("opacity 101", set(with(7, 101.0))),
            ("dark colour \"#12345\"", set(fractal(START, "#12345", "#ffffff", "normal"))),
            ("light colour \"white\"", set(fractal(START, "#000000", "white", "normal"))),
            ("blend \"overlay\"", set(fractal(START, "#000000", "#ffffff", "overlay"))),
            ("speed keyed to 400", keys("speed", &[(0, &[0.0]), (4, &[400.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_fractal_001.json",
        vec![
            (
                "every number at the top of its range,",
                set(fractal(
                    [1000.0, 20.0, 1000.0, 200.0, 100000.0, 360.0, 100000.0, 100.0],
                    "#ffffff",
                    "#000000",
                    "add",
                )),
            ),
            (
                "every number at the bottom of its range,",
                set(fractal(
                    [1.0, 1.0, 0.0, -200.0, -100000.0, -360.0, 0.0, 0.0],
                    "#000000",
                    "#ffffff",
                    "multiply",
                )),
            ),
            ("evolution keyed from 0 to 720", keys("evolution", &[(0, &[0.0]), (4, &[720.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_fractal_002.json", 0), ("fx_fractal_009.json", 3)]);

    t.finish("B-71_fractal_noise_table.md");
}
