//! B-52: Repeat Edge Pixels for Gaussian, Directional and Radial Blur, against D-109.
//!
//! Writes `verification/B-52_edges_table.md`.
//!
//! Every expected pixel is `Fixtures/edges/expected_edges.json`, written by
//! `tools/edges_reference.py` before this code existed and printed in document 25 as
//! FX-EDGES-001 to 012. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{set, Table};

use anime_compositor::effects::Effect;

fn gauss(sigma_px: f64, edges: &str) -> Effect {
    Effect::GaussianBlur { sigma_px, edges: edges.into(), dimensions: "both".into() }
}

fn streak(direction: f64, length: f64, edges: &str) -> Effect {
    Effect::DirectionalBlur { direction, length, edges: edges.into() }
}

fn turn(kind: &str, amount: f64, edges: &str) -> Effect {
    Effect::RadialBlur { kind: kind.into(), amount, center: [50.0, 50.0], edges: edges.into() }
}

#[test]
fn b52_edges() {
    let mut t = Table::new(
        "edges",
        "# B-52: Repeat Edge Pixels\n\nD-109, proposed on 2026-09-26 and built at the owner's \
         \"proceed, add to other blurs if needed\". Every expected pixel is \
         `Fixtures/edges/expected_edges.json`, written by `tools/edges_reference.py` before this \
         code existed and printed in document 25 as FX-EDGES-001 to 012. The build's frame is \
         compared sample by sample; the answer is the largest difference over all of them, \
         against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-EDGES-001 to 012 (document 25)");
    t.fixtures("expected_edges.json");

    t.heading("How far it reaches");
    for (what, effect, want) in [
        ("a Gaussian Blur of sigma 4 repeating its edges does not grow", gauss(4.0, "repeat"), 0),
        ("one leaving them transparent grows by its radius, as before", gauss(4.0, "transparent"), 12),
        ("a Directional Blur 20 long repeating its edges does not grow", streak(30.0, 20.0, "repeat"), 0),
        ("one leaving them transparent grows by half its length, as before", streak(30.0, 20.0, "transparent"), 10),
        ("a Radial Blur repeating its edges does not grow, as it never did", turn("zoom", 40.0, "repeat"), 0),
    ] {
        let got = effect.bounds_expansion();
        t.row(what, &got.to_string(), got == want);
    }
    let mut draft = streak(30.0, 20.0, "repeat");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the length and keeps the edges",
        &format!("{draft:?}"),
        draft == streak(30.0, 10.0, "repeat"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_edges_001.json",
        "fx_edges_003.json",
        "fx_edges_004.json",
        "fx_edges_005.json",
        "fx_edges_006.json",
        "fx_edges_007.json",
        "fx_edges_008.json",
        "fx_edges_009.json",
        "fx_edges_010.json",
        "fx_edges_011.json",
        "fx_edges_012.json",
    ]);
    let saved = t.saved_parameters("fx_edges_002.json");
    t.row(
        "fx_edges_002.json's written \"transparent\" is not written back, as a file from before \
         D-109 has it",
        &saved.to_string(),
        saved.get("edges").is_none() && saved.get("sigma_px").is_some(),
    );
    let saved = t.saved_parameters("fx_edges_009.json");
    t.row(
        "fx_edges_009.json's first blur, with no edges, is saved with none",
        &saved.to_string(),
        saved.get("edges").is_none(),
    );
    t.shape_refused(
        "fx_edges_001.json",
        "edges written as a number",
        r##"{"sigma_px": 1, "edges": 1}"##,
    );
    t.shape_refused(
        "fx_edges_007.json",
        "edges written as keys",
        r##"{"type": "spin", "amount": 30, "center": [50, 50], "edges": {"base": "repeat", "keyframes": []}}"##,
    );

    t.heading("Commands");
    for (file, wrong, right) in [
        ("fx_edges_001.json", gauss(1.0, "wrap"), gauss(1.0, "transparent")),
        ("fx_edges_004.json", streak(90.0, 6.0, "Repeat"), streak(90.0, 6.0, "transparent")),
        ("fx_edges_007.json", turn("spin", 30.0, ""), turn("spin", 30.0, "transparent")),
    ] {
        let mut document = t.load(file).document;
        t.refused(&mut document, vec![(&format!("{file}: {wrong:?}"), set(wrong.clone()))]);
        t.taken(&mut document, file, vec![(&format!("{file}: {right:?}"), set(right))]);
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_edges_001.json", 0), ("fx_edges_005.json", 0), ("fx_edges_008.json", 0)]);

    t.finish("B-52_edges_table.md");
}
