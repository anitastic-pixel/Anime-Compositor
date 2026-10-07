//! B-213: D-332, keys on every keyable setting survive a save.
//!
//! Writes `verification/D-332_keyed_settings_table.md`.
//!
//! `Fixtures/projects/keyed_settings_project.json` is tutorial 1's Wave layer as the app saved it,
//! with one Bulge added, printed in document 25 as FX-KEYALL-001 to 004. FX-KEYALL-005, every
//! setting of every effect, is the app's `keyed_settings` test, because only the app adds effects.

mod effect_table;

use std::fs;

use effect_table::Table;

use anime_compositor::effects::EffectInstance;
use anime_compositor::model::{Id, Interp, Property};
use anime_compositor::persist;

const FILE: &str = "keyed_settings_project.json";

/// One setting's keys as `frame: value ease`, channels joined with `/`.
fn keys(e: &EffectInstance, name: &str) -> String {
    let Some(track) = e.tracks.get(name) else { return "no keys".into() };
    let channel = |p: &Property| {
        p.keyframes()
            .iter()
            .map(|k| {
                let ease = match k.interp {
                    Interp::Linear => "linear",
                    Interp::Hold => "hold",
                    Interp::Ease { .. } => "eased",
                };
                format!("{}: {} {ease}", k.frame, k.value.as_scalar().unwrap_or(f64::NAN))
            })
            .collect::<Vec<_>>()
            .join("; ")
    };
    track.iter().map(channel).collect::<Vec<_>>().join(" / ")
}

fn wave(loaded: &persist::Loaded) -> Vec<EffectInstance> {
    let comp = &loaded.document.project().compositions[0];
    comp.layer(&Id::new("layer-1")).expect("the Wave layer").effects.clone()
}

#[test]
fn keys_on_every_keyable_setting_survive_a_save() {
    let mut t = Table::new(
        "projects",
        "# D-332: keys on every keyable setting survive a save\n\n\
         Written by `tests/b213_keyed_settings.rs` from `Fixtures/projects/keyed_settings_project.json`, \
         tutorial 1's Wave layer as the app saved it, with a Bulge added. Before D-332 this file \
         would not open: *At /compositions/0/layers/0/effects/0/parameters/offset: expected an array.*\n",
    );
    t.heading("FX-KEYALL-001 to 004");
    let text = fs::read_to_string(t.root.join(FILE)).unwrap();
    let opened = persist::load_str(&text);
    t.row(
        "FX-KEYALL-001: the file opens with no error",
        &opened.as_ref().map_or_else(|d| d.detail.clone(), |_| "it opens".into()),
        opened.is_ok(),
    );
    let opened = opened.unwrap_or_else(|d| panic!("{}", d.detail));
    let fx = wave(&opened);
    let want_offset = "0: 0 eased; 34: 0 eased / 0: 0 eased; 34: 410 eased";
    let got = keys(&fx[0], "offset");
    t.row(&format!("FX-KEYALL-002: Offset Turbulence keys are {want_offset}"), &got, got == want_offset);
    let bulge = fx.iter().find(|e| e.instance_id.as_str() == "fx-90").expect("the Bulge");
    let got = format!("{}, and {}", keys(bulge, "vertical_radius"), keys(bulge, "taper_radius"));
    let want = "0: 190 linear; 12: 120 linear, and 0: 0 linear; 12: 240 linear";
    t.row(&format!("FX-KEYALL-003: Vertical Radius and Taper Radius keys are {want}"), &got, got == want);

    let written = persist::to_json(opened.document.project(), &opened.preserved);
    let again = persist::load_str(&written).unwrap_or_else(|d| panic!("{}", d.detail));
    let fx2 = wave(&again);
    let b2 = fx2.iter().find(|e| e.instance_id.as_str() == "fx-90").expect("the Bulge");
    let same = keys(&fx2[0], "offset") == want_offset
        && format!("{}, and {}", keys(b2, "vertical_radius"), keys(b2, "taper_radius")) == want;
    let resaved = persist::to_json(again.document.project(), &again.preserved) == written;
    t.row(
        "FX-KEYALL-004: saved and opened again, the same keys, and saving that gives the same text",
        &format!("same keys {same}, same text {resaved}"),
        same && resaved,
    );

    t.finish("D-332_keyed_settings_table.md");
}
