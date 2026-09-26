//! B-36: directional blur in the core, against D-92 as D-98 (B-42) reads it.
//!
//! Writes `verification/B-36_directional_blur_table.md`.
//!
//! Every expected pixel is `Fixtures/directional_blur/expected_directional_blur.json`, written
//! by `tools/directional_blur_reference.py` before this code existed and printed in document 25
//! as FX-DIRBLUR-001 to 015. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn dirblur(direction: f64, length: f64) -> Effect {
    Effect::DirectionalBlur { direction, length }
}

#[test]
fn b36_directional_blur() {
    let mut t = Table::new(
        "directional_blur",
        "# B-36: directional blur\n\nD-92, accepted by the owner on 2026-09-25, read by D-98's lines since \
         B-42 the same day. Every expected \
         pixel is `Fixtures/directional_blur/expected_directional_blur.json`, written by \
         `tools/directional_blur_reference.py` before this code existed and printed in document \
         25 as FX-DIRBLUR-001 to 015. The build's frame is compared sample by sample; the answer \
         is the largest difference over all of them, against the catalogue's tolerance of \
         2e-5.\n",
    );

    t.heading("FX-DIRBLUR-001 to 015 (document 25)");
    t.fixtures("expected_directional_blur.json");

    t.heading("How far it reaches");
    for (length, want) in [(0.0, 0), (1.0, 1), (4.0, 2), (5.0, 3), (500.0, 250)] {
        let got = dirblur(45.0, length).bounds_expansion();
        t.row(
            &format!("length {length} grows the drawing's bounds by {want}, half, rounded up"),
            &got.to_string(),
            got == want,
        );
    }
    let mut draft = dirblur(90.0, 8.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the length and keeps the direction",
        &format!("{draft:?}"),
        draft == dirblur(90.0, 4.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_dirblur_001.json",
        "fx_dirblur_007.json",
        "fx_dirblur_010.json",
        "fx_dirblur_011.json",
        "fx_dirblur_012.json",
        "fx_dirblur_013.json",
        "fx_dirblur_014.json",
        "fx_dirblur_015.json",
    ]);
    t.shape_refused(
        "fx_dirblur_001.json",
        "no `length` at all",
        r#"{"direction": 0}"#,
    );
    t.shape_refused(
        "fx_dirblur_001.json",
        "a direction that is a word",
        r#"{"direction": "up", "length": 4}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_dirblur_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("length 501", set(dirblur(0.0, 501.0))),
            ("length -1", set(dirblur(0.0, -1.0))),
            ("direction 3601", set(dirblur(3601.0, 4.0))),
            ("direction -3601", set(dirblur(-3601.0, 4.0))),
            (
                "length keyed to 600",
                keys("length", &[(0, &[0.0]), (4, &[600.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_dirblur_001.json",
        vec![
            (
                "length 500 and direction 3600, the top of the ranges,",
                set(dirblur(3600.0, 500.0)),
            ),
            (
                "length 0 and direction -3600, the bottom,",
                set(dirblur(-3600.0, 0.0)),
            ),
            (
                "direction keyed from 0 to 90",
                keys("direction", &[(0, &[0.0]), (4, &[90.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_dirblur_004.json", 0), ("fx_dirblur_009.json", 3)]);

    t.finish("B-36_directional_blur_table.md");
}
