//! B-75: the light wrap in the core and the renderer, against D-132.
//!
//! Writes `verification/B-75_light_wrap_table.md`.
//!
//! Every expected pixel is `Fixtures/light_wrap/expected_light_wrap.json`, written by
//! `tools/light_wrap_reference.py` before this code existed and printed in document 25 as
//! FX-WRAP-001 to 024. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, Table};

use anime_compositor::command::Command;
use anime_compositor::effects::Effect;
use anime_compositor::model::Id;

fn wrap(width: f64, intensity: f64, blend: &str) -> Effect {
    Effect::LightWrap {
        width,
        intensity,
        blend: blend.to_string(),
    }
}

/// The box's Light Wrap, `fx-1-0`, as the box is the second layer, above the bands.
fn on_box(mut command: Command) -> Command {
    if let Command::SetEffectParameters { instance_id, .. } | Command::SetEffectKeys { instance_id, .. } =
        &mut command
    {
        *instance_id = Id::new("fx-1-0");
    }
    command
}

fn set(effect: Effect) -> Command {
    on_box(effect_table::set(effect))
}

#[test]
fn b75_light_wrap() {
    let mut t = Table::new(
        "light_wrap",
        "# B-75: light wrap\n\nD-132, accepted by the owner on 2026-09-26, the last of the second \
         batch of ten, with its light read from everything beneath the layer, the owner's answer \
         the same day. Every expected pixel is `Fixtures/light_wrap/expected_light_wrap.json`, \
         written by `tools/light_wrap_reference.py` before this code existed and printed in \
         document 25 as FX-WRAP-001 to 024. The build's frame is compared sample by sample; the \
         answer is the largest difference over all of them, against the catalogue's tolerance \
         of 2e-5.\n",
    );

    t.heading("FX-WRAP-001 to 024 (document 25)");
    t.fixtures("expected_light_wrap.json");

    t.heading("How far it reaches");
    let got = wrap(10.0, 100.0, "screen").bounds_expansion();
    t.row(
        "it grows the drawing's bounds by nothing: the light falls inside the layer's edge",
        &got.to_string(),
        got == 0,
    );
    let mut draft = wrap(24.0, 150.0, "add");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview reaches half as far in: width 24 becomes 12",
        &format!("{draft:?}"),
        draft == wrap(12.0, 150.0, "add"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_wrap_001.json",
        "fx_wrap_007.json",
        "fx_wrap_011.json",
        "fx_wrap_012.json",
        "fx_wrap_014.json",
        "fx_wrap_015.json",
        "fx_wrap_016.json",
        "fx_wrap_017.json",
        "fx_wrap_018.json",
        "fx_wrap_019.json",
        "fx_wrap_023.json",
        "fx_wrap_024.json",
    ]);
    t.shape_refused(
        "fx_wrap_001.json",
        "no `blend` at all",
        r#"{"width": 10, "intensity": 100}"#,
    );
    t.shape_refused(
        "fx_wrap_001.json",
        "a width that is a word",
        r#"{"width": "wide", "intensity": 100, "blend": "screen"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_wrap_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("width 501", set(wrap(501.0, 100.0, "screen"))),
            ("width -1", set(wrap(-1.0, 100.0, "screen"))),
            ("intensity 401", set(wrap(10.0, 401.0, "screen"))),
            ("blend \"multiply\"", set(wrap(10.0, 100.0, "multiply"))),
            ("width keyed to 600", on_box(keys("width", &[(0, &[10.0]), (4, &[600.0])]))),
        ],
    );
    t.taken(
        &mut document,
        "fx_wrap_001.json",
        vec![
            ("width 500, intensity 400 and blend add, the tops,", set(wrap(500.0, 400.0, "add"))),
            ("width 0 and intensity 0, the bottoms,", set(wrap(0.0, 0.0, "screen"))),
            ("intensity keyed from 0 to 400", on_box(keys("intensity", &[(0, &[0.0]), (4, &[400.0])]))),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_wrap_001.json", 0), ("fx_wrap_010.json", 0), ("fx_wrap_017.json", 4)]);

    t.finish("B-75_light_wrap_table.md");
}
