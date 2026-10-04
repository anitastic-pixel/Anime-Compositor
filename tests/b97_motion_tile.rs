//! B-97: motion tile in the core, against D-154.
//!
//! Writes `verification/B-97_motion_tile_table.md`.
//!
//! Every expected pixel is `Fixtures/motion_tile/expected_motion_tile.json`, written by
//! `tools/motion_tile_reference.py` before this code existed and printed in document 25 as
//! FX-TILE-001 to 023. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn motion_tile(output_width: f64, output_height: f64, mirror: &str) -> Effect {
    Effect::MotionTile {
        output_width,
        output_height,
        mirror: mirror.to_string(),
        tile_center: [50.0, 50.0],
        tile_width: 100.0,
        tile_height: 100.0,
    }
}

#[test]
fn b97_motion_tile() {
    let mut t = Table::new(
        "motion_tile",
        "# B-97: motion tile\n\nD-154, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the twenty-first of the third batch. Every expected pixel is \
         `Fixtures/motion_tile/expected_motion_tile.json`, written by \
         `tools/motion_tile_reference.py` before this code existed and printed in document 25 \
         as FX-TILE-001 to 023. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-TILE-001 to 023 (document 25)");
    t.fixtures("expected_motion_tile.json");

    t.heading("How far it reaches");
    let got = motion_tile(1000.0, 1000.0, "on").bounds_expansion();
    t.row(
        "it declares no fixed growth to the card, which never runs it: its growth depends on \
         the size the drawing reaches it at, and is counted as the stack runs, as FX-TILE-003 \
         to 016 show",
        &got.to_string(),
        got == 0,
    );
    let mut draft = motion_tile(300.0, 150.0, "on");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: the sizes are shares of the drawing",
        &format!("{draft:?}"),
        draft == motion_tile(300.0, 150.0, "on"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_tile_001.json",
        "fx_tile_006.json",
        "fx_tile_011.json",
        "fx_tile_013.json",
        "fx_tile_014.json",
        "fx_tile_016.json",
        "fx_tile_017.json",
        "fx_tile_018.json",
        "fx_tile_019.json",
        "fx_tile_020.json",
        "fx_tile_021.json",
        "fx_tile_022.json",
        "fx_tile_023.json",
    ]);
    t.shape_refused(
        "fx_tile_001.json",
        "no `mirror` at all",
        r#"{"output_width": 100, "output_height": 100}"#,
    );
    t.shape_refused(
        "fx_tile_001.json",
        "a mirror that is a number",
        r#"{"output_width": 100, "output_height": 100, "mirror": 1}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_tile_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("output width 99", set(motion_tile(99.0, 100.0, "off"))),
            ("output width 1001", set(motion_tile(1001.0, 100.0, "off"))),
            ("output height 99.5", set(motion_tile(100.0, 99.5, "off"))),
            ("output height 1000.5", set(motion_tile(100.0, 1000.5, "off"))),
            ("mirror \"yes\"", set(motion_tile(100.0, 100.0, "yes"))),
            ("mirror \"On\"", set(motion_tile(100.0, 100.0, "On"))),
            ("output width keyed to 1200", keys("output_width", &[(0, &[200.0]), (4, &[1200.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_tile_001.json",
        vec![
            ("every number at its bottom,", set(motion_tile(100.0, 100.0, "on"))),
            ("every number at its top,", set(motion_tile(1000.0, 1000.0, "off"))),
            ("output width keyed from 100 to 300", keys("output_width", &[(0, &[100.0]), (4, &[300.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_tile_006.json", 0),
        ("fx_tile_009.json", 0),
        ("fx_tile_015.json", 3),
        ("fx_tile_016.json", 0),
    ]);

    t.finish("B-97_motion_tile_table.md");
}
