//! B-60: the rim light in the core, against D-117.
//!
//! Writes `verification/B-60_rim_light_table.md`.
//!
//! Every expected pixel is `Fixtures/rim_light/expected_rim_light.json`, written by
//! `tools/rim_light_reference.py` before this code existed and printed in document 25 as
//! FX-RIM-001 to 028. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn rim(color: &str, direction: f64, width: f64, softness: f64, intensity: f64, blend: &str) -> Effect {
    Effect::RimLight {
        color: color.into(),
        direction,
        width,
        softness,
        intensity,
        blend: blend.into(),
    }
}

#[test]
fn b60_rim_light() {
    let mut t = Table::new(
        "rim_light",
        "# B-60: rim light\n\nD-117, accepted by the owner on 2026-09-26 in the batch of ten. \
         Every expected pixel is `Fixtures/rim_light/expected_rim_light.json`, written by \
         `tools/rim_light_reference.py` before this code existed and printed in document 25 as \
         FX-RIM-001 to 028. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-RIM-001 to 028 (document 25)");
    t.fixtures("expected_rim_light.json");

    t.heading("How far it reaches");
    let all = rim("#ffffff", 45.0, 3.0, 1.0, 100.0, "normal");
    let got = all.bounds_expansion();
    t.row(
        "it grows the drawing's bounds by nothing: only pixels that show are lit",
        &got.to_string(),
        got == 0,
    );
    let mut draft = rim("#ffffff", 45.0, 4.0, 2.0, 100.0, "normal");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the width and the softness, and nothing else",
        &format!("{draft:?}"),
        draft == rim("#ffffff", 45.0, 2.0, 1.0, 100.0, "normal"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_rim_001.json",
        "fx_rim_013.json",
        "fx_rim_017.json",
        "fx_rim_018.json",
        "fx_rim_019.json",
        "fx_rim_022.json",
        "fx_rim_023.json",
        "fx_rim_024.json",
        "fx_rim_025.json",
        "fx_rim_026.json",
        "fx_rim_027.json",
        "fx_rim_028.json",
    ]);
    let colour = t.saved_parameters("fx_rim_016.json")["color"].clone();
    t.row(
        "fx_rim_016.json, its colour in capitals, is saved with it in small letters",
        &colour.to_string(),
        colour == "#ffb040",
    );
    t.shape_refused(
        "fx_rim_001.json",
        "no `blend` at all",
        r##"{"color": "#ffffff", "direction": 45, "width": 3, "softness": 1, "intensity": 100}"##,
    );
    t.shape_refused(
        "fx_rim_001.json",
        "a width written as a word",
        r##"{"color": "#ffffff", "direction": 45, "width": "three", "softness": 1,
            "intensity": 100, "blend": "normal"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_rim_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("width 101", set(rim("#ffffff", 45.0, 101.0, 1.0, 100.0, "normal"))),
            ("width -1", set(rim("#ffffff", 45.0, -1.0, 1.0, 100.0, "normal"))),
            ("softness 101", set(rim("#ffffff", 45.0, 3.0, 101.0, 100.0, "normal"))),
            ("intensity -1", set(rim("#ffffff", 45.0, 3.0, 1.0, -1.0, "normal"))),
            ("intensity 101", set(rim("#ffffff", 45.0, 3.0, 1.0, 101.0, "normal"))),
            ("direction -3601", set(rim("#ffffff", -3601.0, 3.0, 1.0, 100.0, "normal"))),
            ("blend \"overlay\"", set(rim("#ffffff", 45.0, 3.0, 1.0, 100.0, "overlay"))),
            ("colour \"#fff\"", set(rim("#fff", 45.0, 3.0, 1.0, 100.0, "normal"))),
            ("width keyed to 150", keys("width", &[(0, &[3.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_rim_001.json",
        vec![
            ("width, softness and intensity 100, direction 3600, the tops,", set(rim("#ffb040", 3600.0, 100.0, 100.0, 100.0, "add"))),
            ("width, softness and intensity 0, direction -3600, the bottoms,", set(rim("#ffffff", -3600.0, 0.0, 0.0, 0.0, "multiply"))),
            ("direction keyed from 0 to 180", keys("direction", &[(0, &[0.0]), (4, &[180.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_rim_001.json", 0), ("fx_rim_018.json", 2)]);

    t.finish("B-60_rim_light_table.md");
}
