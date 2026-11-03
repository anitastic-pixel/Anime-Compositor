//! B-885: the Layer Style Gradient Overlay, against D-1005.
//!
//! Writes `verification/B-885_gradient_overlay_table.md`.
//!
//! Every expected pixel is `Fixtures/layer_styles/expected_gradient_overlay.json`, written by
//! `tools/gradient_overlay_reference.py` before this code existed and printed in document 25 as
//! FX-GOVER-001 to 025. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{changed, set, Table};

use anime_compositor::effects::Effect;

fn overlay(mode: &str, colors: &[(f64, &str)], opacities: &[[f64; 2]], style: &str) -> Effect {
    Effect::GradientOverlay {
        blend_mode: mode.into(),
        opacity: 100.0,
        color_stops: colors.iter().map(|(a, c)| (*a, c.to_string())).collect(),
        opacity_stops: opacities.iter().map(|p| p.to_vec()).collect(),
        angle: 90.0,
        style: style.into(),
        reverse: "off".into(),
        scale: 100.0,
        offset: [0.0, 0.0],
    }
}

const WHITE_BLACK: &[(f64, &str)] = &[(0.0, "#ffffff"), (100.0, "#000000")];
const SOLID: &[[f64; 2]] = &[[0.0, 100.0], [100.0, 100.0]];

#[test]
fn b885_gradient_overlay() {
    let mut t = Table::new(
        "layer_styles",
        "# B-885: the Layer Style Gradient Overlay\n\nD-1005, accepted 2026-10-27 with wave 12's \
         tutorial follow-alongs (\"go through this playlist to finely tune our effects ... to \
         fully replicate all these tutorials\"). Every expected pixel is \
         `Fixtures/layer_styles/expected_gradient_overlay.json`, written by \
         `tools/gradient_overlay_reference.py` before this code existed and printed in document \
         25 as FX-GOVER-001 to 025. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-GOVER-001 to 025 (document 25)");
    t.fixtures_numbered("expected_gradient_overlay.json", 1..=25);

    t.heading("Saving and opening");
    t.round_trips(
        &(1..=25)
            .map(|n| format!("fx_gover_{n:03}.json"))
            .collect::<Vec<_>>()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
    );
    let read = changed(&t, "fx_gover_001.json", |_| {});
    t.row(
        "fx_gover_001.json opens as a Gradient Overlay at After Effects' defaults",
        &format!("{read:?}"),
        read == overlay("normal", WHITE_BLACK, SOLID, "linear"),
    );
    let saved = t.saved_parameters("fx_gover_003.json");
    t.row(
        "the tutorial's file is saved with its stops as written",
        &format!("{} {}", saved["color_stops"], saved["opacity_stops"]),
        saved["color_stops"] == serde_json::json!([[0, "#232444"], [100, "#232444"]])
            && saved["opacity_stops"] == serde_json::json!([[65.6, 0], [100, 100]]),
    );
    let rest = r#""angle": 90, "style": "linear", "reverse": "off", "scale": 100, "offset": [0, 0]"#;
    t.shape_refused(
        "fx_gover_001.json",
        "a colour stop that is a word, not a location and a colour",
        &format!(
            r##"{{"blend_mode": "normal", "opacity": 100, "color_stops": ["#ffffff", [100, "#000000"]], "opacity_stops": [[0, 100], [100, 100]], {rest}}}"##
        ),
    );
    t.shape_refused(
        "fx_gover_001.json",
        "an opacity stop that is not numbers",
        &format!(
            r##"{{"blend_mode": "normal", "opacity": 100, "color_stops": [[0, "#ffffff"], [100, "#000000"]], "opacity_stops": [["a", 100], [100, 100]], {rest}}}"##
        ),
    );

    t.heading("Commands");
    let mut document = t.load("fx_gover_001.json").document;
    t.refused(
        &mut document,
        vec![
            (
                "a blend mode \"stencil_alpha\"",
                set(overlay("stencil_alpha", WHITE_BLACK, SOLID, "linear")),
            ),
            (
                "a style \"spiral\"",
                set(overlay("normal", WHITE_BLACK, SOLID, "spiral")),
            ),
            (
                "a stop colour \"white\"",
                set(overlay("normal", &[(0.0, "white"), (100.0, "#000000")], SOLID, "linear")),
            ),
            (
                "colour stops at 80 then 20",
                set(overlay("normal", &[(80.0, "#ffffff"), (20.0, "#000000")], SOLID, "linear")),
            ),
            (
                "seventeen colour stops",
                set(overlay("normal", &[(50.0, "#ffffff"); 17], SOLID, "linear")),
            ),
            (
                "an opacity stop of -1",
                set(overlay("normal", WHITE_BLACK, &[[0.0, -1.0], [100.0, 100.0]], "linear")),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_gover_001.json",
        vec![(
            "the tutorial's radial overlay",
            set(overlay(
                "overlay",
                &[(0.0, "#232444"), (100.0, "#232444")],
                &[[65.6, 0.0], [100.0, 100.0]],
                "radial",
            )),
        )],
    );

    t.heading("Tiles");
    t.tiles(&[
        ("fx_gover_001.json", 0),
        ("fx_gover_003.json", 0),
        ("fx_gover_005.json", 0),
        ("fx_gover_013.json", 0),
    ]);

    t.finish("B-885_gradient_overlay_table.md");
}
