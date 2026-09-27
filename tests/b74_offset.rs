//! B-74: the offset in the core, against D-131.
//!
//! Writes `verification/B-74_offset_table.md`.
//!
//! Every expected pixel is `Fixtures/offset/expected_offset.json`, written by
//! `tools/offset_reference.py` before this code existed and printed in document 25 as
//! FX-OFFSET-001 to 023. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn offset(x: f64, y: f64) -> Effect {
    Effect::Offset { shift: [x, y] }
}

#[test]
fn b74_offset() {
    let mut t = Table::new(
        "offset",
        "# B-74: offset\n\nD-131, accepted by the owner on 2026-09-26, the ninth of the second \
         batch of ten. Every expected pixel is `Fixtures/offset/expected_offset.json`, written \
         by `tools/offset_reference.py` before this code existed and printed in document 25 as \
         FX-OFFSET-001 to 023. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-OFFSET-001 to 023 (document 25)");
    t.fixtures("expected_offset.json");

    t.heading("How far it reaches");
    let got = offset(35.0, -23.0).bounds_expansion();
    t.row(
        "it grows the drawing's bounds by nothing: what leaves one edge comes in at the other",
        &got.to_string(),
        got == 0,
    );
    let mut draft = offset(40.0, -12.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview slides by half: shift 40, -12 becomes 20, -6",
        &format!("{draft:?}"),
        draft == offset(20.0, -6.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_offset_001.json",
        "fx_offset_002.json",
        "fx_offset_011.json",
        "fx_offset_013.json",
        "fx_offset_015.json",
        "fx_offset_016.json",
        "fx_offset_017.json",
        "fx_offset_019.json",
        "fx_offset_021.json",
        "fx_offset_022.json",
    ]);
    t.shape_refused("fx_offset_001.json", "no `shift` at all", r#"{}"#);
    t.shape_refused(
        "fx_offset_001.json",
        "a shift of one number",
        r#"{"shift": [3]}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_offset_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("shift 100001, 0", set(offset(100001.0, 0.0))),
            ("shift 0, -100001", set(offset(0.0, -100001.0))),
            ("shift 100000.5, 0", set(offset(100000.5, 0.0))),
            ("shift keyed to 150000, 0", keys("shift", &[(0, &[0.0, 0.0]), (4, &[150000.0, 0.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_offset_001.json",
        vec![
            ("shift 100000, -100000, the most,", set(offset(100000.0, -100000.0))),
            ("shift -2.25, 1.75, parts of a pixel,", set(offset(-2.25, 1.75))),
            ("shift keyed from 0, 0 to 8, 4", keys("shift", &[(0, &[0.0, 0.0]), (4, &[8.0, 4.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_offset_011.json", 0), ("fx_offset_017.json", 0)]);

    t.finish("B-74_offset_table.md");
}
