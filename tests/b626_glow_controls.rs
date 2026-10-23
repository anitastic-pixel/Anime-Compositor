//! B-626: D-747, After Effects' further Glow controls (audit row 17): Composite Original,
//! Glow Dimensions, Glow Colors A & B and the Glow Operations this program has a formula for.
//!
//! FX-GLDISP-009 to 016 are `Fixtures/glow_display/expected_glow_display.json`, written by
//! `tools/glow_display_reference.py` in D-747's commit, and held by B-625's test. This one draws
//! an orange disc on grey at 8 bpc with each control and saves the pictures it is judged by.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, GlowLook, GLOW_OPERATIONS};
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

const SIZE: (usize, usize) = (240, 135);
const LOOK_KEYS: [&str; 9] = [
    "composite", "dimensions", "glow_colors", "color_looping", "color_loops", "color_phase", "midpoint", "color_a", "color_b",
];

/// The project: the disc at 8 bpc with one Glow, After Effects units, of `glow`'s parameters
/// over threshold 60, radius 10, intensity 1; `glow` null leaves the Glow out.
fn project(glow: &J) -> String {
    let (w, h) = SIZE;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let mut parameters = json!({"based_on": "bright", "threshold": 60, "colors": [], "tolerance": 0,
        "radius": 10, "intensity": 1, "operation": "add", "tint": "", "units": "after_effects"});
    for (k, v) in glow.as_object().into_iter().flatten() {
        parameters[k] = v.clone();
    }
    let effects = if glow.is_null() {
        json!([])
    } else {
        json!([{"instance_id": "fx-glow", "type_id": "core.glow", "enabled": true, "parameters": parameters}])
    };
    json!({
        "schema_version": 0, "project_id": "proj-b626",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-art", "kind": "still", "name": "art", "path": "disc.png",
            "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 24,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 24},
            "layer_order": ["art"], "eight_bpc": true,
            "layers": [{
                "id": "art", "kind": "raster", "name": "art", "asset_id": "asset-art", "enabled": true,
                "locked": false, "in_frame": 0, "out_frame": 24, "source_offset_frames": 0,
                "transform": {
                    "anchor": t(json!([w as f64 / 2.0, h as f64 / 2.0])), "position": t(json!([w as f64 / 2.0, h as f64 / 2.0])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal",
                "effects": effects
            }]
        }]
    })
    .to_string()
}

/// The frame as 8-bit straight pixels, and the warnings.
fn draw(dir: &Path, text: &str) -> (Vec<u8>, Vec<String>) {
    let loaded = persist::load_str(text).expect("the project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

fn at(b: &[u8], x: usize, y: usize) -> [u8; 4] {
    let i = (y * SIZE.0 + x) * 4;
    [b[i], b[i + 1], b[i + 2], b[i + 3]]
}

fn look_of(text: &str) -> (String, GlowLook) {
    let loaded = persist::load_str(text).expect("the project reads");
    let comp = &loaded.document.project().compositions[0];
    match &comp.layer(&Id::new("art")).expect("the layer").effects[0].effect {
        Effect::Glow { operation, look, .. } => (operation.clone(), look.clone()),
        other => panic!("{other:?}"),
    }
}

#[test]
fn b626_glow_controls() {
    let mut t = Table::new(
        "glow_display",
        "# B-626: After Effects' further Glow controls\n\nD-747, audit row 17 \
         (https://claude.ai/artifact/A63KdY44N4HfUHzUnLQhTp). An orange disc (255, 160, 40) of \
         radius 20 on grey 64, at 8 bpc, with a Glow in After Effects units (threshold 60, radius \
         10, intensity 1) and each new control. The rule is our basis: After Effects publishes no \
         formula, and Wave 1's tutorials use the starting values only. FX-GLDISP-009 to 016 are \
         held by B-625's test.\n",
    );
    let dir = effect_table::repo("verification/B-626 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = SIZE;
    let (cx, cy) = (w / 2, h / 2);
    let mut px = vec![255u8; w * h * 4];
    for y in 0..h {
        for x in 0..w {
            let d = (x as f64 + 0.5 - cx as f64).hypot(y as f64 + 0.5 - cy as f64);
            px[(y * w + x) * 4..][..3].copy_from_slice(if d <= 20.0 { &[255, 160, 40] } else { &[64; 3] });
        }
    }
    png_out::write_rgba(&dir.join("disc.png"), w, h, OutputDepth::Eight, &[], &px).unwrap();
    let save = |name: &str, b: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], b).unwrap();
    let (plain, _) = draw(&dir, &project(&J::Null));

    t.heading("An older project, without the new settings, reads them at After Effects' start and draws as before");
    let old = project(&json!({}));
    let (_, look) = look_of(&old);
    t.row(
        "the settings it reads: Behind, both, Original Colors, Triangle A>B>A, 1 loop, phase 0, midpoint 50, #ffffff and #000000",
        &format!("{look:?}"),
        look == GlowLook::default()
            && (look.composite.as_str(), look.dimensions.as_str(), look.glow_colors.as_str()) == ("behind", "both", "original")
            && (look.color_loops, look.color_phase, look.midpoint) == (1.0, 0.0, 50.0),
    );
    let (before, said) = draw(&dir, &old);
    save("1_as_before.png", &before);
    let written = json!({"composite": "behind", "dimensions": "both", "glow_colors": "original",
        "color_looping": "triangle_aba", "color_loops": 1, "color_phase": 0, "midpoint": 50,
        "color_a": "#ffffff", "color_b": "#000000"});
    let (same, _) = draw(&dir, &project(&written));
    t.row("it draws cleanly, and as the same Glow with the starting values written out", &format!("{said:?}"), said.is_empty() && same == before);
    let saved = persist::to_json(persist::load_str(&old).unwrap().document.project(), &Default::default());
    let kept: Vec<&str> = LOOK_KEYS.into_iter().filter(|k| saved.contains(&format!("\"{k}\""))).collect();
    t.row("saving it again writes none of the new settings", &format!("{kept:?}"), kept.is_empty());

    t.heading("Composite Original None: the light alone");
    let (none, said) = draw(&dir, &project(&json!({"composite": "none"})));
    save("2_composite_none.png", &none);
    let (corner, middle) = (at(&none, 4, 4), at(&none, cx, cy));
    t.row(
        "draws cleanly; a corner far from the disc is empty, the disc's middle is its light",
        &format!("{said:?} corner {corner:?} middle {middle:?}"),
        said.is_empty() && corner[3] == 0 && middle[3] == 255 && middle[0] == 255,
    );
    let (zero, _) = draw(&dir, &project(&json!({"composite": "none", "intensity": 0})));
    t.row("with Intensity 0 nothing is left", &format!("{:?}", at(&zero, cx, cy)), zero.chunks(4).all(|p| p[3] == 0));

    t.heading("Glow Dimensions");
    let above = |b: &[u8]| at(b, cx, cy - 26);
    let right = |b: &[u8]| at(b, cx + 26, cy);
    let (hor, said) = draw(&dir, &project(&json!({"dimensions": "horizontal"})));
    save("3_horizontal.png", &hor);
    t.row(
        "Horizontal: 6 pixels right of the disc the grey is lit; 6 pixels above it, where its row has no disc, it stays 64",
        &format!("{said:?} right {:?} above {:?}", right(&hor), above(&hor)),
        said.is_empty() && right(&hor)[0] > 70 && above(&hor)[..3] == [64; 3],
    );
    let (ver, said) = draw(&dir, &project(&json!({"dimensions": "vertical"})));
    t.row(
        "Vertical: the other way round",
        &format!("{said:?} right {:?} above {:?}", right(&ver), above(&ver)),
        said.is_empty() && above(&ver)[0] > 70 && right(&ver)[..3] == [64; 3],
    );
    t.row(
        "Horizontal and Vertical: both lit",
        &format!("right {:?} above {:?}", right(&before), above(&before)),
        right(&before)[0] > 70 && above(&before)[0] > 70,
    );

    t.heading("Glow Colors A & B: Color A red, Color B blue, Sawtooth A>B, Composite Original None");
    let ab = json!({"composite": "none", "glow_colors": "a_and_b", "color_looping": "sawtooth_ab", "color_a": "#ff0000", "color_b": "#0000ff"});
    let (colored, said) = draw(&dir, &project(&ab));
    save("4_colors_a_and_b.png", &colored);
    let (near, far) = (at(&colored, cx, cy), at(&colored, cx + 30, cy));
    t.row(
        "the brightest light is Color A, red; the faint light 10 pixels out leans to Color B, blue",
        &format!("{said:?} middle {near:?} 10 out {far:?}"),
        said.is_empty() && near[0] == 255 && near[2] == 0 && far[2] > far[0],
    );
    let mut swapped = ab.clone();
    swapped["color_looping"] = json!("sawtooth_ba");
    let (ba, _) = draw(&dir, &project(&swapped));
    t.row("Sawtooth B>A turns the middle blue", &format!("{:?}", at(&ba, cx, cy)), at(&ba, cx, cy)[2] > at(&ba, cx, cy)[0]);
    let mut looped = ab.clone();
    looped["color_loops"] = json!(3);
    looped["color_phase"] = json!(90);
    looped["midpoint"] = json!(25);
    let (lp, said) = draw(&dir, &project(&looped));
    t.row("Color Loops 3, Color Phase 90 and A & B Midpoint 25 draw cleanly and change it", &format!("{said:?}"), said.is_empty() && lp != colored);
    let (orig, _) = draw(&dir, &project(&json!({"color_a": "#ff0000", "color_b": "#0000ff"})));
    t.row("with Original Colors, Color A and Color B do nothing", "", orig == before);

    t.heading("Glow Operations");
    let (normal_top, _) = draw(&dir, &project(&json!({"operation": "normal", "composite": "on_top"})));
    save("5_normal_on_top.png", &normal_top);
    let mid = at(&normal_top, cx, cy);
    t.row(
        "Normal On Top: the light covers the disc, so its green rises from 160 towards the light's",
        &format!("{mid:?}"),
        mid[1] > 200,
    );
    let (normal_behind, _) = draw(&dir, &project(&json!({"operation": "normal"})));
    t.row("Normal Behind: the opaque picture covers its light, so nothing changes", "", normal_behind == plain);
    let (lighten, _) = draw(&dir, &project(&json!({"operation": "lighten", "composite": "on_top"})));
    t.row("Lighten On Top never darkens", "", lighten.iter().zip(&plain).all(|(a, b)| a >= b));
    let (add_top, _) = draw(&dir, &project(&json!({"composite": "on_top"})));
    t.row("Add On Top lays the light on as Add Behind does", "", add_top == before);
    for op in GLOW_OPERATIONS {
        for composite in ["on_top", "behind"] {
            let (b, said) = draw(&dir, &project(&json!({"operation": op, "composite": composite})));
            // Two leave this picture as it was, by the W3C's own formulas: Normal Behind (the
            // opaque picture covers the light), and Saturation On Top (grey has no saturation to
            // change, and the orange given the light's full saturation is clipped back to itself).
            let same = (op, composite) == ("normal", "behind") || (op, composite) == ("saturation", "on_top");
            t.row(
                &format!("{op} {composite} draws cleanly{}", if same { " and leaves this picture as it was" } else { " and changes the frame" }),
                &format!("{said:?}; the middle {:?}, 6 pixels out {:?}", at(&b, cx, cy), at(&b, cx + 26, cy)),
                said.is_empty() && (b != plain) != same,
            );
        }
    }

    t.heading("The card draws only what it can: Add or Screen, both directions, the light's own colours, the picture kept (in linear light)");
    for (what, glow, card) in [
        ("as before", json!({}), 1),
        ("Screen On Top", json!({"operation": "screen", "composite": "on_top"}), 1),
        ("Composite Original None", json!({"composite": "none"}), 0),
        ("Horizontal", json!({"dimensions": "horizontal"}), 0),
        ("A & B Colors", json!({"glow_colors": "a_and_b"}), 0),
        ("Overlay", json!({"operation": "overlay"}), 0),
    ] {
        // At 8 bpc the card is left nothing (D-330), so this is the same picture in linear light.
        let linear = project(&glow).replace(r#""eight_bpc":true"#, r#""eight_bpc":false"#);
        assert!(!linear.contains(r#""eight_bpc":true"#));
        let loaded = persist::load_str(&linear).unwrap();
        let mut log = FrameLog::new(3);
        let plan = anime_compositor::compose::plan_frame_for_card(
            loaded.document.project(),
            &Id::new(MAIN),
            0,
            &dir,
            anime_compositor::preview::PreviewQuality::Full,
            &mut log,
            &mut anime_compositor::cache::CelCache::viewer(),
        )
        .expect("plan the frame");
        let n = plan.layers.iter().flat_map(|l| &l.on_card)
            .filter(|c| matches!(c.unmixed(), anime_compositor::render::OnCard::Glow(_)))
            .count();
        t.row(&format!("{what}: Glows on the card, {card}"), &n.to_string(), n == card);
    }

    t.heading("Saved and opened again");
    let all = json!({"operation": "overlay", "composite": "on_top", "dimensions": "vertical", "glow_colors": "a_and_b",
        "color_looping": "triangle_bab", "color_loops": 4, "color_phase": -45, "midpoint": 70, "color_a": "#12abef", "color_b": "#fedcba"});
    let first = look_of(&project(&all));
    let again = persist::to_json(persist::load_str(&project(&all)).unwrap().document.project(), &Default::default());
    t.row("every setting comes back as written", &format!("{first:?}"), look_of(&again) == first && first.1.color_loops == 4.0 && first.1.dimensions == "vertical");

    t.heading("Refused, with a warning, and the Glow left out (no silent fallback)");
    for (k, v) in [
        ("composite", json!("sideways")),
        ("dimensions", json!("diagonal")),
        ("glow_colors", json!("rainbow")),
        ("color_looping", json!("zigzag")),
        ("color_loops", json!(0)),
        ("color_loops", json!(128)),
        ("color_phase", json!(4000)),
        ("midpoint", json!(101)),
        ("color_a", json!("red")),
        ("operation", json!("color_dodge")),
    ] {
        let (b, said) = draw(&dir, &project(&json!({ k: v.clone() })));
        t.row(&format!("{k} {v}"), &format!("{said:?}"), !said.is_empty() && b == plain);
    }

    fs::remove_file(dir.join("disc.png")).unwrap();
    t.finish("B-626_glow_controls_table.md");
}
