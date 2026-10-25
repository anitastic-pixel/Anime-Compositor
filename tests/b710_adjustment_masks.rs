//! B-710 (D-830): the effects that work from a layer's masks, on an adjustment layer, draw along
//! the adjustment layer's own masks on the frame beneath.
//!
//! Before D-830 each of these, on an adjustment layer, drew nothing and said EFFECT_PATH_MISSING
//! every frame (D-786, D-791). No expected pixel here comes from the new code: each is the same
//! effect on the layer itself carrying the same masks, drawn by code that existed before D-830.

#![recursion_limit = "256"]

mod effect_table;

use std::path::Path;

use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::preview::PreviewQuality;

use effect_table::{both, repo, Table};

fn mask(name: &str, pts: &[[f64; 2]], feather: f64) -> J {
    json!({"name": name, "enabled": true, "inverted": false, "mode": "none", "opacity": 1.0,
        "feather_px": feather, "expansion_px": 0.0, "closed": true,
        "path": {"base": {"points": pts.iter().map(|p| json!({"point": p, "in": [0, 0], "out": [0, 0]})).collect::<Vec<_>>()},
                 "keyframes": []}})
}

fn read(path: &Path) -> J {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

/// `p` with `masks` on its layer `art`, or (`on_art` false) with `art`'s effects and those masks
/// moved to an adjustment layer `adj` just above it, the size of the composition. The adjustment
/// layer's Position is moved by `nudge` and its Scale is `scale` percent, about its centre.
fn adjusted(mut p: J, masks: &J, on_art: bool, nudge: [f64; 2], scale: f64) -> J {
    let c = &mut p["compositions"][0];
    let (w, h) = (c["width"].as_f64().unwrap(), c["height"].as_f64().unwrap());
    let layers = c["layers"].as_array_mut().unwrap();
    let i = layers.iter().position(|l| l["id"] == "art").unwrap();
    let art = &mut layers[i];
    art.as_object_mut().unwrap().remove("mask");
    if on_art {
        art["masks"] = masks.clone();
        return p;
    }
    let mut adj = json!({"id": "adj", "kind": "adjustment", "name": "adj", "enabled": true,
        "locked": false, "in_frame": art["in_frame"], "out_frame": art["out_frame"],
        "transform": {"anchor": {"base": [w / 2.0, h / 2.0], "keyframes": []},
                      "position": {"base": [w / 2.0 + nudge[0], h / 2.0 + nudge[1]], "keyframes": []},
                      "scale": {"base": [scale, scale], "keyframes": []},
                      "rotation": {"base": 0, "keyframes": []},
                      "opacity": {"base": 1, "keyframes": []}},
        "matte": null, "blend_mode": "normal"});
    adj["effects"] = art["effects"].take();
    art["effects"] = json!([]);
    adj["masks"] = masks.clone();
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

fn largest(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0, f32::max)
}

/// The bands with one effect of `type_id`, its parameters a fixture's with `changes`.
fn bands(base: &J, type_id: &str, changes: &J) -> J {
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
        ("core.fill", "fill/fx_fill_001.json"),
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
fn b710_adjustment_masks() {
    let mut t = Table::new(
        "effect_mix",
        "# D-830: effects along an adjustment layer's masks\n\nB-710. Before D-830 each effect \
         below, on an adjustment layer, was not drawn and said EFFECT_PATH_MISSING every frame \
         (D-786, D-791). Now it draws along the adjustment layer's own masks on the frame \
         beneath, as After Effects' does. The bands are `Fixtures/effect_mix`'s; the masks are \
         two, drawn as paths only (mode None), so the adjustment layer covers the whole frame. \
         Each expected frame is the same effect on the band layer itself carrying the same \
         masks, drawn by code that existed before D-830; \"unlike the frame without it\" shows \
         the effect drew something. Fill's own cases are FX-FILLADJ-007, 008 and 010 to 012 \
         (B-666's table).\n",
    );
    let base = fixture_parameters();
    let mix = repo("Fixtures/effect_mix");
    let two = json!([
        mask("Mask 1", &[[3.0, 2.0], [12.0, 2.0], [12.0, 8.0], [3.0, 8.0]], 0.0),
        mask("Mask 2", &[[5.0, 3.0], [10.0, 3.0], [10.0, 7.0], [5.0, 7.0]], 0.0),
    ]);
    let mut nothing = adjusted(bands(&base, "core.stroke", &json!({})), &two, false, [0.0, 0.0], 100.0);
    nothing["compositions"][0]["layers"][1]["effects"] = json!([]);

    let cases = [
        ("Path Stroke on Mask 1", "core.stroke", json!({})),
        ("Path Stroke on All Masks", "core.stroke", json!({"all_masks": "on"})),
        ("Vegas on Mask 1", "core.vegas", json!({})),
        ("Energy Stroke along the masks", "core.energy_stroke", json!({"source": "masks"})),
        ("Energy Stroke between its points, Alpha Mode Masks", "core.energy_stroke", json!({"alpha_mode": "masks"})),
        ("Audio Spectrum on Path 1", "core.audio_spectrum", json!({"path": 1})),
        ("Audio Waveform on Path 1", "core.audio_waveform", json!({"path": 1})),
        ("Inner/Outer Key, Foreground 1", "core.inner_outer_key", json!({"foreground": 1})),
        ("Reshape, Mask 1 to Mask 2", "core.reshape", json!({"source_mask": 1, "destination_mask": 2})),
        ("Mask Smear, Mask 1 moved 3 across", "core.mask_smear", json!({"source_mask": 1, "mask_offset": [3, 0]})),
        ("Scribble on Mask 1", "core.scribble", json!({})),
        ("Liquify with Freeze Area Mask 1 (fx_liquify_brushes_011's)", "core.liquify", json!({})),
        ("Particle Playground, Wall Mask 1", "core.particle_playground", json!({"persistent_map_layer": "", "wall_mask": 1})),
        ("Fill, Fill Mask 2", "core.fill", json!({"mask": 2})),
        ("Fill, All Masks", "core.fill", json!({"all_masks": "on"})),
    ];

    t.heading("Along the adjustment layer's masks, as on the layer itself");
    for (what, id, changes) in &cases {
        let p = bands(&base, id, changes);
        for frame in [0, 2] {
            let (got, said) = draw(&adjusted(p.clone(), &two, false, [0.0, 0.0], 100.0), &mix, frame);
            let (want, _) = draw(&adjusted(p.clone(), &two, true, [0.0, 0.0], 100.0), &mix, frame);
            let (plain, _) = draw(&nothing, &mix, frame);
            let d = largest(&got, &want);
            t.row(
                &format!("{what}, frame {frame}: on the adjustment layer as on the layer itself, nothing said"),
                &format!(
                    "largest difference {d:.1e}, {} the frame without it, said {said:?}",
                    if got == plain { "the same as" } else { "unlike" }
                ),
                d <= 1e-6 && got != plain && said.is_empty(),
            );
        }
    }

    // The adjustment layer at 200% about its centre and 1 pixel right: its masks go with it, as
    // the same masks drawn twice the size about the frame's centre, 1 pixel right, feathered
    // twice as far, on the layer itself. The effects' own widths stay the frame's pixels.
    t.heading("The masks go with the adjustment layer's transform");
    let big = |m: &J, feather: f64| -> J {
        let pts: Vec<[f64; 2]> = m["path"]["base"]["points"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| [8.0 + 2.0 * (p["point"][0].as_f64().unwrap() - 8.0) + 1.0, 5.0 + 2.0 * (p["point"][1].as_f64().unwrap() - 5.0)])
            .collect();
        mask(m["name"].as_str().unwrap(), &pts, feather)
    };
    let soft = json!([
        mask("Mask 1", &[[5.0, 3.0], [10.0, 3.0], [10.0, 7.0], [5.0, 7.0]], 1.0),
        mask("Mask 2", &[[6.0, 4.0], [9.0, 4.0], [9.0, 6.0], [6.0, 6.0]], 0.0),
    ]);
    let soft_big = json!([big(&soft[0], 2.0), big(&soft[1], 0.0)]);
    for (what, id, changes) in [
        ("Path Stroke on Mask 1", "core.stroke", json!({})),
        ("Fill, Fill Mask 1 (feathered)", "core.fill", json!({"mask": 1})),
        ("Scribble on Mask 1", "core.scribble", json!({})),
        ("Reshape, Mask 1 to Mask 2", "core.reshape", json!({"source_mask": 1, "destination_mask": 2})),
    ] {
        let p = bands(&base, id, &changes);
        let (got, said) = draw(&adjusted(p.clone(), &soft, false, [1.0, 0.0], 200.0), &mix, 0);
        let (want, _) = draw(&adjusted(p.clone(), &soft_big, true, [0.0, 0.0], 100.0), &mix, 0);
        let (unmoved, _) = draw(&adjusted(p.clone(), &soft, true, [0.0, 0.0], 100.0), &mix, 0);
        let d = largest(&got, &want);
        t.row(
            &format!("{what}: the adjustment layer at 200% and 1 pixel right, as its masks so moved on the layer itself"),
            &format!(
                "largest difference {d:.1e}, {} the masks unmoved, said {said:?}",
                if got == unmoved { "the same as" } else { "unlike" }
            ),
            d <= 1e-5 && got != unmoved && said.is_empty(),
        );
    }

    t.heading("On the graphics card");
    match Gpu::new().ok() {
        None => t.row("the card", "no card on this machine", true),
        Some(mut gpu) => {
            for (what, id, changes) in &cases {
                let p = adjusted(bands(&base, id, changes), &two, false, [0.0, 0.0], 100.0);
                let loaded = persist::load_str(&p.to_string()).unwrap();
                let ((levels, n), refused, cpu, card) =
                    both(&mut gpu, loaded.document.project(), &Id::new("comp-main"), &mix, 0, PreviewQuality::Full);
                t.row(
                    &format!("{what}: the card's frame within 1 level of the CPU's"),
                    &format!(
                        "{levels} levels at most ({n} pixels differ), {}, said {cpu:?} / {card:?}",
                        if refused { "handed to the CPU" } else { "drawn on the card" }
                    ),
                    levels <= 1,
                );
            }
        }
    }

    t.heading("Old projects");
    for (dir, file) in [("adjust", "fx_adj_004.json"), ("fill_adjustment", "fx_filladj_004.json"), ("fill_adjustment", "fx_filladj_001.json")] {
        let text = std::fs::read_to_string(repo(&format!("Fixtures/{dir}/{file}"))).unwrap();
        let loaded = persist::load_str(&text).unwrap();
        let saved: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
        let again = persist::load_str(&serde_json::to_string(&saved).unwrap()).unwrap();
        let same = persist::to_json(again.document.project(), &again.preserved) == serde_json::to_string(&saved).unwrap()
            || effect_table::same_json(
                &serde_json::from_str(&persist::to_json(again.document.project(), &again.preserved)).unwrap(),
                &saved,
            );
        t.row(&format!("{file} opens, saves and opens again holding what it held"), if same { "the same" } else { "differs" }, same);
    }
    t.finish("B-710_adjustment_masks_table.md");
}
