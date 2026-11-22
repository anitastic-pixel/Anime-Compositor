//! D-1139: Chromatic Aberration keeps the frame's edge its own colour (P-28 bug 1).
//!
//! Writes `verification/D-1139_chromatic_edge_table.md`.
//!
//! Every expected pixel is `Fixtures/chromatic_edge/expected_chromatic_edge.json`, written by
//! `tools/chromatic_edge_reference.py` before the fix and printed in document 25 as
//! FX-CHROMAEDGE-001 to 003. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::Table;

use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;

#[test]
fn d1139_chromatic_edge() {
    let mut t = Table::new(
        "chromatic_edge",
        "# D-1139: Chromatic Aberration at the drawing's edge\n\nD-1139, accepted by the owner \
         on 2026-11-21. A sample past the edge of the effect's input reads the nearest edge \
         pixel, on the processor and on the card. Every expected pixel is \
         `Fixtures/chromatic_edge/expected_chromatic_edge.json`, written by \
         `tools/chromatic_edge_reference.py` before the fix and printed in document 25 as \
         FX-CHROMAEDGE-001 to 003. The build's frame is compared sample by sample; the answer \
         is the largest difference over all of them, against the catalogue's tolerance of \
         2e-5.\n",
    );

    t.heading("FX-CHROMAEDGE-001 to 003 (document 25)");
    t.fixtures("expected_chromatic_edge.json");

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_chromaedge_001.json", 0), ("fx_chromaedge_003.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // 001's effect is on an adjustment layer, which the card's per-layer count does not list
    // (measured 0 of 2 frames before the fix); the frame itself is still compared.
    t.card_fixtures(&mut gpu, "expected_chromatic_edge.json", &[1], |e| {
        matches!(e, Effect::ChromaticAberration { .. })
    });

    t.finish("D-1139_chromatic_edge_table.md");
}
