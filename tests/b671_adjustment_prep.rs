//! B-671 (D-791): the effects prepared from their own layer, on an adjustment layer.
//!
//! Before D-791 each of these, on an adjustment layer, was left out of the frame or drew part of
//! itself and nothing was said. Now an Audio Spectrum or Audio Waveform on no mask draws on the
//! frame beneath. (Every one that works from the layer's own masks said EFFECT_PATH_MISSING every
//! frame until D-830 drew it along the adjustment layer's masks: B-710.) Each expected frame is drawn by code that existed before D-791: the same effect on
//! the layer itself, or the frame the adjustment layer drew before (what the effect left out
//! taken out of the file by hand).

mod effect_table;

use std::path::{Path, PathBuf};

use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::persist;

use effect_table::{repo, same_json, Table};

fn mask(name: &str, pts: &[[f64; 2]]) -> J {
    json!({"name": name, "enabled": true, "inverted": false, "mode": "none", "opacity": 1.0,
        "feather_px": 0.0, "expansion_px": 0.0, "closed": true,
        "path": {"base": {"points": pts.iter().map(|p| json!({"point": p, "in": [0, 0], "out": [0, 0]})).collect::<Vec<_>>()},
                 "keyframes": []}})
}

fn read(path: &Path) -> J {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

/// `p` with the effects of its layer `art` moved to an adjustment layer `adj` just above it, the
/// adjustment the size of the composition and carrying `art`'s masks (`art` keeps none). With
/// `masks`, those are set on the holder first, drawn as paths only (mode None).
fn adjusted(mut p: J, masks: Option<J>, on_art: bool) -> J {
    let c = &mut p["compositions"][0];
    let (w, h) = (c["width"].as_f64().unwrap(), c["height"].as_f64().unwrap());
    let layers = c["layers"].as_array_mut().unwrap();
    let i = layers.iter().position(|l| l["id"] == "art").unwrap();
    let art = &mut layers[i];
    if let Some(m) = masks {
        art["masks"] = m;
        art.as_object_mut().unwrap().remove("mask");
    }
    if on_art {
        return p;
    }
    let mut adj = json!({"id": "adj", "kind": "adjustment", "name": "adj", "enabled": true,
        "locked": false, "in_frame": art["in_frame"], "out_frame": art["out_frame"],
        "transform": {"anchor": {"base": [w / 2.0, h / 2.0], "keyframes": []},
                      "position": {"base": [w / 2.0, h / 2.0], "keyframes": []},
                      "scale": {"base": [100, 100], "keyframes": []},
                      "rotation": {"base": 0, "keyframes": []},
                      "opacity": {"base": 1, "keyframes": []}},
        "matte": null, "blend_mode": "normal"});
    adj["effects"] = art["effects"].take();
    art["effects"] = json!([]);
    for k in ["masks", "mask"] {
        if let Some(m) = art.as_object_mut().unwrap().remove(k) {
            adj[k] = m;
        }
    }
    layers.push(adj);
    let order = c["layer_order"].as_array_mut().unwrap();
    let at = order.iter().position(|l| l == "art").unwrap();
    order.insert(at + 1, json!("adj"));
    p
}

fn draw(p: &J, root: &Path, frame: i32) -> (Vec<f32>, Vec<String>) {
    let loaded = persist::load_str(&p.to_string()).unwrap_or_else(|d| panic!("{}", d.message));
    let mut log = FrameLog::new(3);
    let b = render_frame(loaded.document.project(), &Id::new("comp-main"), frame, root, 64, &mut log)
        .expect("the frame draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (b.data().to_vec(), said)
}

/// The bands with one effect of `type_id`, its parameters a fixture's with `changes`.
fn bands(base: &J, type_id: &str, changes: J) -> J {
    let mut parameters = base[type_id].clone();
    for (k, v) in changes.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    let mut p = read(&repo("Fixtures/effect_mix/fx_mix_001.json"));
    p["compositions"][0]["layers"][0]["effects"] =
        json!([{"instance_id": "fx-1", "type_id": type_id, "enabled": true, "parameters": parameters}]);
    p
}

/// The first parameters each effect is written with in `Fixtures/`.
fn fixture_parameters() -> J {
    let wanted = [
        ("core.stroke", "open_masks/fx_msko_001.json"),
        ("core.vegas", "vegas/fx_vegas_001.json"),
        ("core.energy_stroke", "energy_stroke/fx_energy_001.json"),
        ("core.audio_spectrum", "audio_spectrum/fx_aspec_001.json"),
        ("core.audio_waveform", "audio_waveform/fx_awave_001.json"),
        ("core.inner_outer_key", "inner_outer_key/fx_iok_001.json"),
        ("core.reshape", "reshape/fx_reshape_001.json"),
        ("core.liquify", "liquify_brushes/fx_liquify_brushes_011.json"),
        ("core.mask_smear", "mask_smear/fx_smear_001.json"),
        ("core.scribble", "scribble/fx_scribble_001.json"),
        ("core.particle_playground", "particle_mapper/fx_pmap_001.json"),
    ];
    let mut out = json!({});
    for (id, file) in wanted {
        let p = read(&repo(&format!("Fixtures/{file}")));
        let found = p["compositions"][0]["layers"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|l| l["effects"].as_array().unwrap().iter())
            .find(|e| e["type_id"] == id)
            .unwrap_or_else(|| panic!("{file} has {id}"));
        out[id] = found["parameters"].clone();
    }
    out
}

#[test]
fn b671_adjustment_prep() {
    let mut t = Table::new(
        "effect_mix",
        "# D-791: effects prepared from their own layer, on an adjustment layer\n\nB-671. Before \
         D-791 each effect below, on an adjustment layer, was left out of the frame (or drew \
         without its frozen area, its wall or its cut) and nothing was said. Now an Audio Spectrum \
         or Audio Waveform on no mask hears its sound and draws on the frame beneath, as After \
         Effects' does. (Each effect that works from the layer's own masks said \
         EFFECT_PATH_MISSING every frame until D-830, which draws it along the adjustment \
         layer's masks: B-710's table.) The bands are \
         `Fixtures/effect_mix`'s; the adjustment layer carries two masks drawn as paths only \
         (mode None), so it covers the whole frame. No expected pixel comes from the new code: \
         each is the same effect on the layer itself, or the file without what was left out.\n",
    );
    let base = fixture_parameters();
    let mix = repo("Fixtures/effect_mix");
    let two = json!([
        mask("Mask 1", &[[3.0, 2.0], [12.0, 2.0], [12.0, 8.0], [3.0, 8.0]]),
        mask("Mask 2", &[[5.0, 3.0], [10.0, 3.0], [10.0, 7.0], [5.0, 7.0]]),
    ]);
    let on = |p: J| adjusted(p, Some(two.clone()), false);
    let nothing = on(bands(&base, "core.stroke", json!({})));
    let mut nothing = nothing;
    nothing["compositions"][0]["layers"][1]["effects"] = json!([]);

    t.heading("Drawn on the frame beneath, as on the layer itself");
    let sound = |dir: &str, file: &str| -> (J, PathBuf) {
        let d = repo(&format!("Fixtures/{dir}"));
        (read(&d.join(file)), d)
    };
    let works = [
        ("Audio Spectrum, no Audio Layer, Path 0", bands(&base, "core.audio_spectrum", json!({})), mix.clone(), false),
        ("Audio Waveform, no Audio Layer, Path 0", bands(&base, "core.audio_waveform", json!({})), mix.clone(), false),
        ("Audio Spectrum hearing a sound layer (fx_aspec_002)", sound("audio_spectrum", "fx_aspec_002.json").0, repo("Fixtures/audio_spectrum"), true),
        ("Audio Waveform hearing a sound layer (fx_awave_002)", sound("audio_waveform", "fx_awave_002.json").0, repo("Fixtures/audio_waveform"), true),
        ("Energy Stroke between its points (worked before D-791 too)", bands(&base, "core.energy_stroke", json!({})), mix.clone(), false),
        ("Particle Playground with no wall (worked before D-791 too)", bands(&base, "core.particle_playground", json!({"persistent_map_layer": ""})), mix.clone(), false),
    ];
    for (what, p, root, own_masks) in works {
        let masks = (!own_masks).then(|| two.clone());
        for frame in [0, 2] {
            let (got, said) = draw(&adjusted(p.clone(), masks.clone(), false), &root, frame);
            let (want, _) = draw(&adjusted(p.clone(), masks.clone(), true), &root, frame);
            let mut bare = adjusted(p.clone(), masks.clone(), false);
            let n = bare["compositions"][0]["layers"].as_array().unwrap().len();
            bare["compositions"][0]["layers"][n - 1]["effects"] = json!([]);
            let (plain, _) = draw(&bare, &root, frame);
            t.row(
                &format!("{what}, frame {frame}: on the adjustment layer as on the layer itself, nothing said"),
                &format!(
                    "{}, {} the adjustment layer without it, said {said:?}",
                    if got == want { "sample-identical" } else { "differs" },
                    if got == plain { "the same as" } else { "unlike" }
                ),
                got == want && got != plain && said.is_empty(),
            );
        }
    }

    // D-830: each effect that works from the layer's own masks now draws along the adjustment
    // layer's masks on the frame beneath; B-710's table checks every one of them against the
    // same effect on the layer itself.

    t.heading("Nothing named, nothing said (as before D-791)");
    for (what, id) in [
        ("Inner/Outer Key, Foreground 0", "core.inner_outer_key"),
        ("Reshape, no masks", "core.reshape"),
        ("Mask Smear, no masks", "core.mask_smear"),
    ] {
        let (got, said) = draw(&on(bands(&base, id, json!({}))), &mix, 2);
        let (want, _) = draw(&nothing, &mix, 2);
        t.row(
            &format!("{what}: the frame as without it, nothing said"),
            &format!("{}, said {said:?}", if got == want { "sample-identical" } else { "differs" }),
            got == want && said.is_empty(),
        );
    }

    t.heading("Old projects");
    for (dir, file) in [("audio_spectrum", "fx_aspec_002.json"), ("audio_waveform", "fx_awave_002.json"), ("fill_adjustment", "fx_filladj_001.json")] {
        let text = std::fs::read_to_string(repo(&format!("Fixtures/{dir}/{file}"))).unwrap();
        let loaded = persist::load_str(&text).unwrap();
        let saved: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
        let same = same_json(&saved, &serde_json::from_str(&text).unwrap());
        t.row(
            &format!("{file}, written before D-791, opens and saves holding what it held"),
            if same { "the same" } else { "differs" },
            same,
        );
    }
    t.finish("B-671_adjustment_prep_table.md");
}
