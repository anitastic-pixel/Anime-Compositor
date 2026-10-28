//! B-801: Layer Styles, Inner Shadow and Drop Shadow, against D-922.
//!
//! Writes `verification/B-801_layer_styles_table.md`.
//!
//! Every expected pixel is `Fixtures/layer_styles/expected_layer_styles.json`, written by
//! `tools/layer_styles_reference.py` before this code existed and printed in document 25 as
//! FX-LSTYLE-001 to 023. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{changed, set, Table, MAIN};

use anime_compositor::command::Command;
use anime_compositor::effects::Effect;
use anime_compositor::model::Id;

fn style(inner: bool, mode: &str, color: &str, light: &str) -> Effect {
    Effect::LayerStyle {
        inner,
        blend_mode: mode.into(),
        color: color.into(),
        opacity: 75.0,
        use_global_light: light.into(),
        angle: 120.0,
        distance: 5.0,
        spread: 0.0,
        size: 5.0,
        noise: 0.0,
    }
}

fn light(angle: f64) -> Command {
    Command::SetGlobalLight {
        composition: Id::new(MAIN),
        angle,
    }
}

#[test]
fn b801_layer_styles() {
    let mut t = Table::new(
        "layer_styles",
        "# B-801: Layer Styles, Inner Shadow and Drop Shadow\n\nD-922, accepted 2026-10-27 with \
         wave 12's tutorial follow-alongs (\"go through this playlist to finely tune our effects \
         ... to fully replicate all these tutorials\"). Every expected pixel is \
         `Fixtures/layer_styles/expected_layer_styles.json`, written by \
         `tools/layer_styles_reference.py` before this code existed and printed in document 25 \
         as FX-LSTYLE-001 to 023. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-LSTYLE-001 to 023 (document 25)");
    t.fixtures_numbered("expected_layer_styles.json", 1..=23);

    t.heading("The composition's Global Light");
    let old = t.load("fx_lstyle_001.json").document;
    let angle = old.project().composition(&Id::new(MAIN)).unwrap().global_light;
    t.row(
        "fx_lstyle_001.json, which has no `global_light_angle`, opens at 120",
        &angle.to_string(),
        angle == 120.0,
    );
    let saved = effect_table::saved(&t.load("fx_lstyle_001.json"));
    let none = saved["compositions"][0].get("global_light_angle").is_none();
    t.row(
        "saved again, it still has no `global_light_angle`",
        if none { "absent" } else { "written" },
        none,
    );
    let saved = effect_table::saved(&t.load("fx_lstyle_007.json"));
    let kept = saved["compositions"][0]["global_light_angle"].clone();
    t.row(
        "fx_lstyle_007.json is saved with its angle 90",
        &kept.to_string(),
        kept == 90,
    );
    let mut document = t.load("fx_lstyle_008.json").document;
    let _ = document.apply(set(Effect::LayerStyle {
        inner: true,
        blend_mode: "normal".into(),
        color: "#000000".into(),
        opacity: 100.0,
        use_global_light: "on".into(),
        angle: 0.0,
        distance: 2.0,
        spread: 0.0,
        size: 0.0,
        noise: 0.0,
    }));
    let _ = document.apply(light(90.0));
    let a = t.render(&document, 0, 64);
    let b = t.render(&t.load("fx_lstyle_008.json").document, 0, 64);
    t.row(
        "fx_lstyle_008.json with Use Global Light turned on, its angle 0 and the Global Light \
         set to 90 draws its own frame (light from above)",
        if a.data() == b.data() {
            "byte-identical"
        } else {
            "differ"
        },
        a.data() == b.data(),
    );

    t.heading("Saving and opening");
    t.round_trips(
        &(1..=23)
            .map(|n| format!("fx_lstyle_{n:03}.json"))
            .collect::<Vec<_>>()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
    );
    let inner = t.saved_parameters("fx_lstyle_005.json");
    t.row(
        "an Inner Shadow is saved with `choke`, not `spread`",
        &inner.to_string(),
        inner.get("choke").is_some() && inner.get("spread").is_none(),
    );
    let drop = t.saved_parameters("fx_lstyle_013.json");
    t.row(
        "a Drop Shadow is saved with `spread`, not `choke`",
        &drop.to_string(),
        drop.get("spread").is_some() && drop.get("choke").is_none(),
    );
    let read = changed(&t, "fx_lstyle_001.json", |_| {});
    t.row(
        "fx_lstyle_001.json opens as an Inner Shadow at After Effects' defaults",
        &format!("{read:?}"),
        read == style(true, "multiply", "#000000", "on"),
    );

    t.heading("Commands");
    let mut document = t.load("fx_lstyle_001.json").document;
    t.refused(
        &mut document,
        vec![
            (
                "a blend mode \"dissolve\"",
                set(style(true, "dissolve", "#000000", "on")),
            ),
            (
                "a blend mode \"stencil_luma\"",
                set(style(true, "stencil_luma", "#000000", "on")),
            ),
            (
                "a colour \"black\"",
                set(style(true, "multiply", "black", "on")),
            ),
            (
                "Use Global Light \"yes\"",
                set(style(true, "multiply", "#000000", "yes")),
            ),
            ("a Global Light angle of 3601", light(3601.0)),
            ("a Global Light angle that is not a number", light(f64::NAN)),
        ],
    );
    t.taken(
        &mut document,
        "fx_lstyle_001.json",
        vec![
            (
                "an Inner Shadow in Color Dodge, #ffff00",
                set(style(true, "color_dodge", "#ffff00", "off")),
            ),
            ("a Global Light angle of 45", light(45.0)),
        ],
    );

    t.heading("Tiles");
    t.tiles(&[
        ("fx_lstyle_001.json", 0),
        ("fx_lstyle_003.json", 0),
        ("fx_lstyle_006.json", 0),
        ("fx_lstyle_011.json", 0),
        ("fx_lstyle_013.json", 0),
        ("fx_lstyle_014.json", 0),
    ]);

    t.finish("B-801_layer_styles_table.md");
}
