//! B-96: mirror in the core, against D-153.
//!
//! Writes `verification/B-96_mirror_table.md`.
//!
//! Every expected pixel is `Fixtures/mirror/expected_mirror.json`, written by
//! `tools/mirror_reference.py` before this code existed and printed in document 25 as
//! FX-MIRROR-001 to 024. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn mirror(center: [f64; 2], angle: f64) -> Effect {
    Effect::Mirror { center, angle }
}

const MIDDLE: [f64; 2] = [50.0, 50.0];

#[test]
fn b96_mirror() {
    let mut t = Table::new(
        "mirror",
        "# B-96: mirror\n\nD-153, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the twentieth of the third batch. Every expected pixel is \
         `Fixtures/mirror/expected_mirror.json`, written by `tools/mirror_reference.py` before \
         this code existed and printed in document 25 as FX-MIRROR-001 to 024. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-MIRROR-001 to 024 (document 25)");
    t.fixtures("expected_mirror.json");

    t.heading("How far it reaches");
    let got = mirror(MIDDLE, 45.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = mirror(MIDDLE, 30.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: the centre is a share of the drawing",
        &format!("{draft:?}"),
        draft == mirror(MIDDLE, 30.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_mirror_001.json",
        "fx_mirror_008.json",
        "fx_mirror_010.json",
        "fx_mirror_015.json",
        "fx_mirror_016.json",
        "fx_mirror_017.json",
        "fx_mirror_019.json",
        "fx_mirror_020.json",
        "fx_mirror_021.json",
        "fx_mirror_022.json",
        "fx_mirror_023.json",
        "fx_mirror_024.json",
    ]);
    t.shape_refused(
        "fx_mirror_001.json",
        "no `angle` at all",
        r#"{"center": [50, 50]}"#,
    );
    t.shape_refused(
        "fx_mirror_001.json",
        "a centre of one number",
        r#"{"center": [50], "angle": 0}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_mirror_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("centre 1001, 50", set(mirror([1001.0, 50.0], 0.0))),
            ("centre 50, -1001", set(mirror([50.0, -1001.0], 0.0))),
            ("angle 3601", set(mirror(MIDDLE, 3601.0))),
            ("angle -3601", set(mirror(MIDDLE, -3601.0))),
            ("angle keyed to 4000", keys("angle", &[(0, &[0.0]), (4, &[4000.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_mirror_001.json",
        vec![
            ("every number at its bottom,", set(mirror([-1000.0, -1000.0], -3600.0))),
            ("every number at its top,", set(mirror([1000.0, 1000.0], 3600.0))),
            ("angle keyed from 0 to 180", keys("angle", &[(0, &[0.0]), (4, &[180.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_mirror_001.json", 0),
        ("fx_mirror_007.json", 0),
        ("fx_mirror_008.json", 0),
        ("fx_mirror_016.json", 2),
    ]);

    t.finish("B-96_mirror_table.md");
}
