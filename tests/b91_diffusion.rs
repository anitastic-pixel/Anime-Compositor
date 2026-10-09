//! B-91: diffusion in the core, against D-148.
//!
//! Writes `verification/B-91_diffusion_table.md`.
//!
//! Every expected pixel is `Fixtures/diffusion/expected_diffusion.json`, written by
//! `tools/diffusion_reference.py` before this code existed and printed in document 25 as
//! FX-DIFFUSE-001 to 018. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn diffusion(radius: f64, amount: f64, blend: &str) -> Effect {
    Effect::Diffusion {
        radius,
        amount,
        blend: blend.to_string(),
        second_amount: 0.0,
        second_blend: "soft_light".to_string(),
    }
}

#[test]
fn b91_diffusion() {
    let mut t = Table::new(
        "diffusion",
        "# B-91: diffusion\n\nD-148, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the fifteenth of the third batch. Every expected pixel is \
         `Fixtures/diffusion/expected_diffusion.json`, written by \
         `tools/diffusion_reference.py` before this code existed and printed in document 25 as \
         FX-DIFFUSE-001 to 018. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-DIFFUSE-001 to 018 (document 25)");
    t.fixtures("expected_diffusion.json");

    t.heading("How far it reaches");
    let got = diffusion(500.0, 100.0, "screen").bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = diffusion(10.0, 50.0, "screen");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the radius, 10 to 5, and nothing else",
        &format!("{draft:?}"),
        draft == diffusion(5.0, 50.0, "screen"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_diffuse_001.json",
        "fx_diffuse_007.json",
        "fx_diffuse_008.json",
        "fx_diffuse_010.json",
        "fx_diffuse_011.json",
        "fx_diffuse_013.json",
        "fx_diffuse_014.json",
        "fx_diffuse_015.json",
        "fx_diffuse_016.json",
        "fx_diffuse_017.json",
        "fx_diffuse_018.json",
    ]);
    t.shape_refused(
        "fx_diffuse_001.json",
        "no `blend` at all",
        r#"{"radius": 10, "amount": 50}"#,
    );
    t.shape_refused(
        "fx_diffuse_001.json",
        "a radius that is a word",
        r#"{"radius": "wide", "amount": 50, "blend": "screen"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_diffuse_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("radius 501", set(diffusion(501.0, 50.0, "screen"))),
            ("radius -1", set(diffusion(-1.0, 50.0, "screen"))),
            ("amount 101", set(diffusion(10.0, 101.0, "screen"))),
            ("amount -1", set(diffusion(10.0, -1.0, "screen"))),
            ("blend \"add\"", set(diffusion(10.0, 50.0, "add"))),
            ("radius keyed to 600", keys("radius", &[(0, &[10.0]), (4, &[600.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_diffuse_001.json",
        vec![
            ("radius 0 and amount 0, the bottoms,", set(diffusion(0.0, 0.0, "normal"))),
            ("radius 500 and amount 100, the tops,", set(diffusion(500.0, 100.0, "lighten"))),
            ("radius keyed from 0 to 20", keys("radius", &[(0, &[0.0]), (4, &[20.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_diffuse_001.json", 0),
        ("fx_diffuse_008.json", 0),
        ("fx_diffuse_012.json", 3),
    ]);

    t.finish("B-91_diffusion_table.md");
}
