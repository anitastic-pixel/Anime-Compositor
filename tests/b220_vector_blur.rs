//! B-220: D-336, CC Vector Blur, from P-26's tutorial 2.
//!
//! The tutorial smears its lightning along the slopes of its own brightness with After Effects'
//! CC Vector Blur. Every expected pixel is `Fixtures/vector_blur/expected_vector_blur.json`,
//! written by `tools/vector_blur_reference.py` before this code existed.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::command::Command;
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

/// `base` with the type `word` and the amount `amount`.
fn typed(base: &Effect, word: &str, to: f64) -> Effect {
    let mut e = base.clone();
    if let Effect::VectorBlur { kind, amount, .. } = &mut e {
        *kind = word.to_string();
        *amount = to;
    }
    e
}

/// Set the `holder` layer's CC Vector Blur; the fixtures' drawing is `holder`, not `art`.
fn set(effect: Effect) -> Command {
    Command::SetEffectParameters {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        effect,
    }
}

/// A soft white disc on black, brightest in the middle of the picture, for a map.
fn disc() -> Vec<u8> {
    let (w, h) = TOWN;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let d = ((x as f64 + 0.5 - w as f64 / 2.0).powi(2) + (y as f64 + 0.5 - h as f64 / 2.0).powi(2)).sqrt();
            let v = (255.0 * (1.0 - d / 110.0)).clamp(0.0, 255.0) as u8;
            bytes.extend([v, v, v, 255]);
        }
    }
    bytes
}

/// The town carrying `parameters`' CC Vector Blur, with the disc under it switched off.
fn picture(dir: &Path, parameters: Option<J>) -> (Vec<u8>, Vec<String>) {
    let (w, h) = TOWN;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let layer = |id: &str, effects: J, enabled: bool| {
        json!({
            "id": id, "kind": "raster", "name": id, "asset_id": format!("asset-{id}"), "enabled": enabled,
            "locked": false, "in_frame": 0, "out_frame": 24, "source_offset_frames": 0,
            "transform": {
                "anchor": t(json!([w as f64 / 2.0, h as f64 / 2.0])), "position": t(json!([w as f64 / 2.0, h as f64 / 2.0])),
                "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
            },
            "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal", "effects": effects
        })
    };
    let asset = |id: &str| {
        json!({"id": format!("asset-{id}"), "kind": "still", "name": id, "path": format!("{id}.png"),
            "interpretation": {"color_space": "srgb", "alpha": "straight"}})
    };
    let effects = parameters.map_or(json!([]), |p| {
        json!([{"instance_id": "fx-0-0", "type_id": "core.vector_blur", "enabled": true, "parameters": p}])
    });
    let project = json!({
        "schema_version": 0, "project_id": "proj-b220-picture",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [asset("town"), asset("disc")],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 24,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 24},
            "layer_order": ["town", "disc"],
            "layers": [layer("town", effects, true), layer("disc", json!([]), false)]
        }]
    });
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b220_vector_blur() {
    let mut t = Table::new(
        "vector_blur",
        "# B-220: CC Vector Blur\n\nD-336, from P-26's tutorial 2. Every expected pixel is \
         `Fixtures/vector_blur/expected_vector_blur.json`, written by \
         `tools/vector_blur_reference.py` before this code existed and printed in document 25 as \
         FX-VBLUR-001 to 027. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-VBLUR-001 to 027 (document 25)");
    t.fixtures("expected_vector_blur.json");

    t.heading("The file");
    let files: Vec<String> = (1..=27).map(|n| format!("fx_vblur_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let mut document = t.load("fx_vblur_001.json").document;
    let effect_of = |d: &anime_compositor::command::Document| {
        d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].effect.clone()
    };
    let base = effect_of(&document);
    for (file, said) in [
        ("fx_vblur_019.json", "CC Vector Blur's type is natural, constant, perpendicular, direction_center or direction_fading, and this is \"Natural\"."),
        ("fx_vblur_024.json", "CC Vector Blur's property is red, green, blue, alpha, luminance, lightness, hue or saturation, and this is \"brightness\"."),
        ("fx_vblur_026.json", "CC Vector Blur's vector map is the name of a layer of this composition, and this is 3."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == said);
    }

    t.heading("Commands");
    for (what, effect) in [("type \"Natural\"", typed(&base, "Natural", 10.0)), ("amount 501", typed(&base, "natural", 501.0))] {
        let refused = document.apply(set(effect)).err();
        let untouched = effect_of(&document) == base;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
    t.taken(
        &mut document,
        "fx_vblur_001.json",
        vec![("type perpendicular, amount 4,", set(typed(&base, "perpendicular", 4.0))), ("type direction_fading,", set(typed(&base, "direction_fading", 4.0)))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_vblur_001.json", 0), ("fx_vblur_009.json", 0), ("fx_vblur_012.json", 0), ("fx_vblur_018.json", 0)]);

    t.heading("Pictures: a street, in `verification/D-336 pictures/`");
    let dir = effect_table::repo("verification/D-336 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    for (name, bytes) in [("town", town()), ("disc", disc())] {
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
    }
    let (before, said) = picture(&dir, None);
    png_out::write_rgba(&dir.join("before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    t.row("before.png, the street, draws cleanly", &format!("{said:?}"), said.is_empty());
    let changed = |b: &[u8], far: bool| {
        (0..w * h)
            .filter(|&i| {
                let (x, y) = ((i % w) as f64 + 0.5 - w as f64 / 2.0, (i / w) as f64 + 0.5 - h as f64 / 2.0);
                (!far || (x * x + y * y).sqrt() > 125.0) && b[i * 4..][..4] != before[i * 4..][..4]
            })
            .count()
    };
    let shots: [(&str, &str, J, bool); 5] = [
        ("as_added", "as added: Natural, Amount 10, its own lightness, Map Softness 30", json!("natural"), false),
        ("natural_disc", "Natural over the disc, Amount 12, Ridge Smoothness 0: smeared out from the middle, and nowhere past the disc's edge", json!("natural"), true),
        ("perpendicular_disc", "Perpendicular over the disc, Amount 12, Ridge Smoothness 0: smeared round the middle, and nowhere past the disc's edge", json!("perpendicular"), true),
        ("twist_disc", "Direction Center over the disc, Amount 6, Revolutions 1: the smear's way turns once from the edge to the middle", json!("direction_center"), false),
        ("tutorial", "tutorial 2's setting, Natural, Amount 4, Ridge Smoothness 20, its own lightness", json!("natural"), false),
    ];
    for (name, what, kind, outside_untouched) in shots {
        let p = match name {
            "as_added" => json!({"type": kind, "amount": 10, "angle_offset": 0, "ridge_smoothness": 10, "layer": "", "fit": "stretch", "property": "lightness", "map_softness": 30}),
            "natural_disc" | "perpendicular_disc" => json!({"type": kind, "amount": 12, "angle_offset": 0, "ridge_smoothness": 0, "layer": "disc", "fit": "stretch", "property": "lightness", "map_softness": 0}),
            "twist_disc" => json!({"type": kind, "amount": 6, "angle_offset": 0, "ridge_smoothness": 1, "layer": "disc", "fit": "stretch", "property": "lightness", "map_softness": 0}),
            _ => json!({"type": kind, "amount": 4, "angle_offset": 0, "ridge_smoothness": 20, "layer": "", "fit": "stretch", "property": "lightness", "map_softness": 30}),
        };
        let (bytes, said) = picture(&dir, Some(p));
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let (all, far) = (changed(&bytes, false), changed(&bytes, true));
        t.row(
            &format!("{name}.png, {what}"),
            &format!("{said:?}, {all} pixels changed, {far} of them more than 125 from the middle"),
            said.is_empty() && all > 0 && (!outside_untouched || far == 0),
        );
    }

    t.finish("D-336_vector_blur_table.md");
}
