//! B-57: the gradient in the core, against D-114.
//!
//! Writes `verification/B-57_gradient_table.md`.
//!
//! Every expected pixel is `Fixtures/gradient/expected_gradient.json`, written by
//! `tools/gradient_reference.py` before this code existed and printed in document 25 as
//! FX-GRAD-001 to 022. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

/// The starting settings, with `change` made to them.
fn grad(change: impl FnOnce(&mut Effect)) -> Effect {
    let mut e = Effect::Gradient {
        shape: "linear".into(),
        start: [50.0, 0.0],
        end: [50.0, 100.0],
        start_color: "#ffffff".into(),
        end_color: "#6450a0".into(),
        start_opacity: 0.0,
        end_opacity: 50.0,
        blend: "multiply".into(),
    };
    change(&mut e);
    e
}

/// The starting settings with one of them changed.
macro_rules! with {
    ($field:ident = $value:expr) => {
        grad(|e| {
            if let Effect::Gradient { $field, .. } = e {
                *$field = $value.into();
            }
        })
    };
}

#[test]
fn b57_gradient() {
    let mut t = Table::new(
        "gradient",
        "# B-57: gradient\n\nD-114, accepted by the owner on 2026-09-26 in the batch of ten. \
         Every expected pixel is `Fixtures/gradient/expected_gradient.json`, written by \
         `tools/gradient_reference.py` before this code existed and printed in document 25 as \
         FX-GRAD-001 to 022. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-GRAD-001 to 022 (document 25)");
    t.fixtures("expected_gradient.json");

    t.heading("How far it reaches");
    let all = grad(|_| {});
    let got = all.bounds_expansion();
    t.row(
        "it grows the drawing's bounds by nothing: each pixel is coloured where it is",
        &got.to_string(),
        got == 0,
    );
    let mut draft = all.clone();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: the points are shares of the drawing, not \
         distances",
        &format!("{draft:?}"),
        draft == all,
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_grad_001.json",
        "fx_grad_005.json",
        "fx_grad_010.json",
        "fx_grad_011.json",
        "fx_grad_016.json",
        "fx_grad_018.json",
        "fx_grad_019.json",
        "fx_grad_020.json",
        "fx_grad_021.json",
        "fx_grad_022.json",
    ]);
    let colour = t.saved_parameters("fx_grad_014.json")["start_color"].clone();
    t.row(
        "fx_grad_014.json, its colours in capitals, is saved with them in small letters",
        &colour.to_string(),
        colour == "#ffffff",
    );
    t.shape_refused(
        "fx_grad_001.json",
        "no `blend` at all",
        r##"{"shape": "linear", "start": [50, 0], "end": [50, 100], "start_color": "#ffffff",
            "end_color": "#6450a0", "start_opacity": 0, "end_opacity": 50}"##,
    );
    t.shape_refused(
        "fx_grad_001.json",
        "a start point of one number",
        r##"{"shape": "linear", "start": [50], "end": [50, 100], "start_color": "#ffffff",
            "end_color": "#6450a0", "start_opacity": 0, "end_opacity": 50, "blend": "multiply"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_grad_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("start opacity 101", set(with!(start_opacity = 101.0))),
            ("end opacity -1", set(with!(end_opacity = -1.0))),
            ("start point 1001, 0", set(with!(start = [1001.0, 0.0]))),
            ("end point 50, -1001", set(with!(end = [50.0, -1001.0]))),
            ("shape \"conic\"", set(with!(shape = "conic"))),
            ("blend \"overlay\"", set(with!(blend = "overlay"))),
            ("start colour \"#12345\"", set(with!(start_color = "#12345"))),
            ("end colour \"violet\"", set(with!(end_color = "violet"))),
            (
                "end opacity keyed to 150",
                keys("end_opacity", &[(0, &[0.0]), (4, &[150.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_grad_001.json",
        vec![
            (
                "both opacities 100, the top,",
                set(grad(|e| {
                    if let Effect::Gradient {
                        start_opacity,
                        end_opacity,
                        ..
                    } = e
                    {
                        *start_opacity = 100.0;
                        *end_opacity = 100.0;
                    }
                })),
            ),
            (
                "start point -1000, 1000, the corner of the range,",
                set(with!(start = [-1000.0, 1000.0])),
            ),
            (
                "radial, screen",
                set(grad(|e| {
                    if let Effect::Gradient { shape, blend, .. } = e {
                        *shape = "radial".into();
                        *blend = "screen".into();
                    }
                })),
            ),
            (
                "the start point keyed from 50, 0 to 50, 100",
                keys("start", &[(0, &[50.0, 0.0]), (4, &[50.0, 100.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_grad_005.json", 0), ("fx_grad_010.json", 2)]);

    t.finish("B-57_gradient_table.md");
}
