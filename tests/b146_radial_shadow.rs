//! B-146: Radial Shadow in the core, against D-211.
//!
//! Writes `verification/B-146_radial_shadow_table.md`, and pictures in
//! `verification/B-146 pictures/`.
//!
//! Every expected pixel is `Fixtures/radial_shadow/expected_radial_shadow.json`, written by
//! `tools/radial_shadow_reference.py` before this code existed and printed in document 25 as
//! FX-RSHADOW-001 to 025. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, set, Table};
use serde_json::Value as J;

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

/// Radial Shadow with its colour, opacity, light, distance, softness, render, colour influence
/// and shadow only.
#[allow(clippy::too_many_arguments)]
fn shadow(color: &str, opacity: f64, light: [f64; 2], distance: f64, softness: f64, render: &str, influence: f64, only: &str) -> Effect {
    Effect::RadialShadow {
        color: color.to_string(),
        opacity,
        light,
        distance,
        softness,
        render: render.to_string(),
        color_influence: influence,
        shadow_only: only.to_string(),
    }
}

/// As it starts.
fn plain() -> Effect {
    shadow("#000000", 50.0, [50.0, 0.0], 10.0, 0.0, "regular", 100.0, "off")
}

const PLATE: (usize, usize) = (160, 100);

/// A small made-up cel on nothing: a round head of skin with a dark line round it, purple hair
/// on top and a blush on the left cheek, in the middle, with room round it for the shadow.
fn drawing() -> Vec<u8> {
    let (w, h) = PLATE;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (fx, fy) = (x as f64 + 0.5, y as f64 + 0.5);
            let e = ((fx - 80.0) / 22.0).powi(2) + ((fy - 50.0) / 24.0).powi(2);
            let c = if e >= 1.0 {
                [0, 0, 0, 0]
            } else if e >= 0.75 {
                [30, 26, 36, 255]
            } else if fy < 36.0 {
                [90, 60, 150, 255]
            } else if (fx - 68.0).hypot(fy - 56.0) < 4.0 {
                [230, 150, 150, 255]
            } else {
                [246, 214, 190, 255]
            };
            bytes.extend(c);
        }
    }
    bytes
}

/// The drawing as the composition's one layer, with `effects`, drawn, straight 8-bit; and the
/// same enlarged three times.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/radial_shadow/fx_rshadow_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("drawing.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(PLATE.0);
    comp["height"] = J::from(PLATE.1);
    let layer = &mut comp["layers"][0];
    let middle = serde_json::json!([PLATE.0 as f64 / 2.0, PLATE.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    let bytes = frame.to_srgb8_straight();
    let big: Vec<u8> = (0..PLATE.1 * 3)
        .flat_map(|y| (0..PLATE.0 * 3).map(move |x| (x, y)))
        .flat_map(|(x, y)| {
            let i = (y / 3 * PLATE.0 + x / 3) * 4;
            bytes[i..i + 4].to_vec()
        })
        .collect();
    (bytes, big, said)
}

/// One Radial Shadow as a layer's `effects`, from its settings as the file writes them.
fn one(parameters: J) -> J {
    let mut p = serde_json::json!({
        "color": "#000000", "opacity": 50, "light": [50, 0], "distance": 10, "softness": 0,
        "render": "regular", "color_influence": 100, "shadow_only": "off",
    });
    for (k, v) in parameters.as_object().unwrap() {
        p[k] = v.clone();
    }
    serde_json::json!([{ "instance_id": "fx-0-0", "type_id": "core.radial_shadow", "enabled": true, "parameters": p }])
}

#[test]
fn b146_radial_shadow() {
    let mut t = Table::new(
        "radial_shadow",
        "# B-146: Radial Shadow\n\nD-211, accepted on 2026-09-28 with the After Effects picks \
         (B9). Every expected pixel is `Fixtures/radial_shadow/expected_radial_shadow.json`, \
         written by `tools/radial_shadow_reference.py` before this code existed and printed in \
         document 25 as FX-RSHADOW-001 to 025. The build's frame is compared sample by sample; \
         the answer is the largest difference over all of them, against the catalogue's \
         tolerance of 2e-5.\n",
    );

    t.heading("FX-RSHADOW-001 to 025 (document 25)");
    t.fixtures("expected_radial_shadow.json");

    t.heading("How far it reaches");
    let got = plain().bounds_expansion();
    t.row(
        "it declares no fixed growth, as Corner Pin does: how far the layer grows depends on the \
         light and the layer's own size, and is worked out as it is drawn (FX-RSHADOW-001, 004 and \
         015 above grow it)",
        &got.to_string(),
        got == 0,
    );
    let mut draft = shadow("#2040a0", 75.0, [20.0, 30.0], 40.0, 12.0, "glass_edge", 60.0, "on");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the softening, a distance in pixels, and keeps the light, \
         a share of the drawing, and the projection distance, a ratio",
        &format!("{draft:?}"),
        draft == shadow("#2040a0", 75.0, [20.0, 30.0], 40.0, 6.0, "glass_edge", 60.0, "on"),
    );

    t.heading("The file");
    t.round_trips(&(1..=25).map(|n| format!("fx_rshadow_{n:03}.json")).collect::<Vec<_>>().iter().map(String::as_str).collect::<Vec<_>>());
    t.shape_refused(
        "fx_rshadow_001.json",
        "no `render` at all",
        r##"{"color": "#000000", "opacity": 50, "light": [50, 0], "distance": 10, "softness": 0, "color_influence": 100, "shadow_only": "off"}"##,
    );
    t.shape_refused(
        "fx_rshadow_001.json",
        "a light of one number",
        r##"{"color": "#000000", "opacity": 50, "light": [50], "distance": 10, "softness": 0, "render": "regular", "color_influence": 100, "shadow_only": "off"}"##,
    );
    t.shape_refused(
        "fx_rshadow_001.json",
        "a distance that is a word",
        r##"{"color": "#000000", "opacity": 50, "light": [50, 0], "distance": "far", "softness": 0, "render": "regular", "color_influence": 100, "shadow_only": "off"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_rshadow_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("opacity 101", set(shadow("#000000", 101.0, [50.0, 0.0], 10.0, 0.0, "regular", 100.0, "off"))),
            ("distance -1", set(shadow("#000000", 50.0, [50.0, 0.0], -1.0, 0.0, "regular", 100.0, "off"))),
            ("distance 1001", set(shadow("#000000", 50.0, [50.0, 0.0], 1001.0, 0.0, "regular", 100.0, "off"))),
            ("softness 501", set(shadow("#000000", 50.0, [50.0, 0.0], 10.0, 501.0, "regular", 100.0, "off"))),
            ("colour influence 101", set(shadow("#000000", 50.0, [50.0, 0.0], 10.0, 0.0, "regular", 101.0, "off"))),
            ("light down 1001", set(shadow("#000000", 50.0, [50.0, 1001.0], 10.0, 0.0, "regular", 100.0, "off"))),
            ("colour \"black\"", set(shadow("black", 50.0, [50.0, 0.0], 10.0, 0.0, "regular", 100.0, "off"))),
            ("render \"glassy\"", set(shadow("#000000", 50.0, [50.0, 0.0], 10.0, 0.0, "glassy", 100.0, "off"))),
            ("render \"Glass_Edge\", written with capitals", set(shadow("#000000", 50.0, [50.0, 0.0], 10.0, 0.0, "Glass_Edge", 100.0, "off"))),
            ("shadow only \"yes\"", set(shadow("#000000", 50.0, [50.0, 0.0], 10.0, 0.0, "regular", 100.0, "yes"))),
            ("distance keyed to 1200", keys("distance", &[(0, &[10.0]), (4, &[1200.0])])),
            ("light keyed to 2000 across", keys("light", &[(0, &[50.0, 0.0]), (4, &[2000.0, 0.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_rshadow_001.json",
        vec![
            ("every number at its bottom,", set(shadow("#2040a0", 0.0, [-1000.0, -1000.0], 0.0, 0.0, "glass_edge", 0.0, "on"))),
            ("every number at its top,", set(shadow("#ffffff", 100.0, [1000.0, 1000.0], 1000.0, 500.0, "regular", 100.0, "off"))),
            ("light keyed across the top", keys("light", &[(0, &[0.0, 0.0]), (4, &[100.0, 0.0])])),
            ("colour influence keyed from 0 to 100", keys("color_influence", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_rshadow_001.json", 0),
        ("fx_rshadow_006.json", 0),
        ("fx_rshadow_011.json", 2),
        ("fx_rshadow_015.json", 0),
    ]);

    t.heading("Pictures: a made-up cel, in `verification/B-146 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-146 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8], scale: usize| {
        png_out::write_rgba(&dir.join(name), w * scale, h * scale, OutputDepth::Eight, &[], bytes).unwrap()
    };
    write("drawing.png", &drawing(), 1);
    let px = |bytes: &[u8], (x, y): (usize, usize)| {
        let i = (y * w + x) * 4;
        [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
    };
    let near = |a: [u8; 4], b: [u8; 4]| a.iter().zip(b).all(|(&u, v)| u.abs_diff(v) <= 1);
    let every = || (0..h).flat_map(move |y| (0..w).map(move |x| (x, y)));
    let shown = |bytes: &[u8]| every().filter(|&p| px(bytes, p)[3] > 0).count();
    let half_black = [0, 0, 0, 128];

    let (before, big, said) = picture(&dir, one(serde_json::json!({ "opacity": 0 })));
    write("before.png", &big, 3);
    // The drawing's fully covered pixels, which a shadow behind never changes.
    let solid: Vec<_> = every().filter(|&p| px(&before, p)[3] == 255).collect();
    let kept = |bytes: &[u8]| solid.iter().filter(|&&p| px(bytes, p) != px(&before, p)).count();
    t.row(
        "before.png, opacity 0: the drawing as it is, drawn cleanly",
        &format!("{said:?}, {} pixels shown", shown(&before)),
        said.is_empty() && !solid.is_empty(),
    );

    let (starts, big, said) = picture(&dir, one(serde_json::json!({})));
    write("starts.png", &big, 3);
    t.row(
        "starts.png, as it starts: the light at the top of the middle, a tenth bigger, so half-black \
         shadow peeps out below the chin at (80, 78) and nothing is above the hair at (80, 22); the \
         drawing's covered pixels unchanged; draws cleanly",
        &format!("{said:?}, below {:?}, above {:?}, {} changed", px(&starts, (80, 78)), px(&starts, (80, 22)), kept(&starts)),
        said.is_empty() && near(px(&starts, (80, 78)), half_black) && px(&starts, (80, 22))[3] == 0 && kept(&starts) == 0,
    );

    let corner = serde_json::json!({ "light": [0, 0], "distance": 40 });
    let (four, big, said) = picture(&dir, one(corner.clone()));
    write("corner.png", &big, 3);
    t.row(
        "corner.png, the light at the top-left, distance 40: the shadow thrown down and to the \
         right, half black at (120, 85), nothing up and to the left at (50, 20); draws cleanly",
        &format!("{said:?}, {:?} and {:?}, {} changed", px(&four, (120, 85)), px(&four, (50, 20)), kept(&four)),
        said.is_empty() && near(px(&four, (120, 85)), half_black) && px(&four, (50, 20))[3] == 0 && kept(&four) == 0,
    );

    let (ring, big, said) = picture(&dir, one(serde_json::json!({ "light": [50, 50], "distance": 50 })));
    write("middle.png", &big, 3);
    let round = [(52, 50), (108, 50), (80, 20), (80, 80)];
    t.row(
        "middle.png, the light in the middle, distance 50: a dark ring on every side, left, right, \
         above and below, where the drawing had nothing; draws cleanly",
        &format!("{said:?}, {:?}", round.map(|p| px(&ring, p))),
        said.is_empty() && round.iter().all(|&p| px(&before, p)[3] == 0 && near(px(&ring, p), half_black)),
    );

    let (soft, big, said) = picture(&dir, one(serde_json::json!({ "light": [0, 0], "distance": 40, "softness": 12 })));
    write("soft.png", &big, 3);
    t.row(
        "soft.png, corner.png with softening 12: the shadow's edge blurred, so it reaches more \
         pixels than corner.png's; draws cleanly",
        &format!("{said:?}, pixels shown: soft {}, corner {}", shown(&soft), shown(&four)),
        said.is_empty() && shown(&soft) > shown(&four) && kept(&soft) == 0,
    );

    let (blue, big, said) = picture(&dir, one(serde_json::json!({ "light": [0, 0], "distance": 40, "opacity": 100, "color": "#2040a0" })));
    write("blue.png", &big, 3);
    t.row(
        "blue.png, corner.png in blue #2040a0 at opacity 100: inside the shadow it is exactly that \
         blue, fully covered; draws cleanly",
        &format!("{said:?}, {:?}", px(&blue, (120, 85))),
        said.is_empty() && near(px(&blue, (120, 85)), [32, 64, 160, 255]),
    );

    let glass_at = |influence: f64| one(serde_json::json!({ "light": [0, 0], "distance": 40, "opacity": 100, "render": "glass_edge", "color_influence": influence }));
    let (glass, big, said) = picture(&dir, glass_at(100.0));
    write("glass.png", &big, 3);
    let (halfway, big2, said2) = picture(&dir, glass_at(50.0));
    write("glass_half.png", &big2, 3);
    let skin = px(&before, (86, 61));
    let (g, m) = (px(&glass, (120, 85)), px(&halfway, (120, 85)));
    t.row(
        "glass.png, Glass Edge at colour influence 100: the shadow at (120, 85) is the skin it was \
         cast from, (86, 61) on the drawing; glass_half.png, influence 50, is darker than the skin \
         and lighter than black in every channel; both draw cleanly",
        &format!("{said:?} {said2:?}, skin {skin:?}, glass {g:?}, half {m:?}"),
        said.is_empty() && said2.is_empty() && near(g, skin) && (0..3).all(|c| 0 < m[c] && m[c] < g[c]) && m[3] == 255,
    );

    let (only, big, said) = picture(&dir, one(serde_json::json!({ "light": [0, 0], "distance": 40, "shadow_only": "on" })));
    write("shadow_only.png", &big, 3);
    t.row(
        "shadow_only.png, Shadow Only on: the drawing is gone, so (90, 60), skin before, is now the \
         half-black shadow, and the shadow elsewhere is corner.png's; draws cleanly",
        &format!("{said:?}, before {:?}, now {:?}", px(&before, (90, 60)), px(&only, (90, 60))),
        said.is_empty() && near(px(&only, (90, 60)), half_black) && px(&only, (120, 85)) == px(&four, (120, 85)),
    );

    let (low, big, said) = picture(&dir, one(serde_json::json!({ "light": [110, 80], "distance": 60 })));
    write("low_right.png", &big, 3);
    t.row(
        "low_right.png, the light low and off to the right, distance 60: the shadow thrown up and to \
         the left, dark at (40, 20) where the drawing had nothing; draws cleanly",
        &format!("{said:?}, {:?}", px(&low, (40, 20))),
        said.is_empty() && px(&before, (40, 20))[3] == 0 && near(px(&low, (40, 20)), half_black) && kept(&low) == 0,
    );

    t.finish("B-146_radial_shadow_table.md");
}
