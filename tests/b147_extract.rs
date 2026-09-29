//! B-147: Extract in the core, against D-212.
//!
//! Writes `verification/B-147_extract_table.md`, and pictures in `verification/B-147 pictures/`.
//!
//! Every expected pixel is `Fixtures/extract/expected_extract.json`, written by
//! `tools/extract_reference.py` before this code existed and printed in document 25 as
//! FX-EXTRACT-001 to 027. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// Extract with its channel, black and white points, black and white softness, and invert.
fn extract(channel: &str, black: f64, white: f64, black_soft: f64, white_soft: f64, invert: &str) -> Effect {
    Effect::Extract {
        channel: channel.to_string(),
        black_point: black,
        white_point: white,
        black_softness: black_soft,
        white_softness: white_soft,
        invert: invert.to_string(),
    }
}

/// As it starts.
fn plain() -> Effect {
    extract("luminance", 0.0, 255.0, 0.0, 0.0, "off")
}

const PLATE: (usize, usize) = (160, 100);

/// A small made-up scan: a face in dark line on cream paper, with a mouth, a red scarf below, a
/// patch of blue sky in the top-left corner and a yellow star on the right.
fn drawing() -> Vec<u8> {
    let (w, h) = PLATE;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (fx, fy) = (x as f64 + 0.5, y as f64 + 0.5);
            let e = ((fx - 80.0) / 22.0).powi(2) + ((fy - 41.0) / 23.0).powi(2);
            let c = if x <= 40 && y <= 30 {
                [70, 120, 210, 255]
            } else if (e < 1.0 && e >= 0.8) || ((70..=90).contains(&x) && (49..=50).contains(&y)) {
                [20, 18, 24, 255]
            } else if e < 1.0 {
                [246, 214, 190, 255]
            } else if (56..=104).contains(&x) && (66..=80).contains(&y) {
                [200, 40, 50, 255]
            } else if (fx - 130.0).abs() + (fy - 36.0).abs() < 14.0 {
                [250, 210, 40, 255]
            } else {
                [240, 234, 222, 255]
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
        &fs::read_to_string(effect_table::repo("Fixtures/extract/fx_extract_001.json")).unwrap(),
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

/// One Extract as a layer's `effects`, from its settings as the file writes them.
fn one(parameters: J) -> J {
    let mut p = serde_json::json!({
        "channel": "luminance", "black_point": 0, "white_point": 255, "black_softness": 0,
        "white_softness": 0, "invert": "off",
    });
    for (k, v) in parameters.as_object().unwrap() {
        p[k] = v.clone();
    }
    serde_json::json!([{ "instance_id": "fx-0-0", "type_id": "core.extract", "enabled": true, "parameters": p }])
}

#[test]
fn b147_extract() {
    let mut t = Table::new(
        "extract",
        "# B-147: Extract\n\nD-212, accepted on 2026-09-28 with the After Effects picks (B10). \
         Every expected pixel is `Fixtures/extract/expected_extract.json`, written by \
         `tools/extract_reference.py` before this code existed and printed in document 25 as \
         FX-EXTRACT-001 to 027. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-EXTRACT-001 to 026 (document 25)");
    t.fixtures_numbered("expected_extract.json", 1..=26);

    t.heading("FX-EXTRACT-027, in dispute (D-215, proposed)");
    t.in_dispute("fx_extract_027.json", "FX-EXTRACT-027", "D-215, proposed");

    t.heading("How far it reaches");
    let got = plain().bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = extract("green", 30.0, 220.0, 10.0, 20.0, "on");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: it has no distances",
        &format!("{draft:?}"),
        draft == extract("green", 30.0, 220.0, 10.0, 20.0, "on"),
    );

    t.heading("The file");
    t.round_trips(&(1..=26).map(|n| format!("fx_extract_{n:03}.json")).collect::<Vec<_>>().iter().map(String::as_str).collect::<Vec<_>>());
    t.shape_refused(
        "fx_extract_001.json",
        "no `channel` at all",
        r#"{"black_point": 0, "white_point": 255, "black_softness": 0, "white_softness": 0, "invert": "off"}"#,
    );
    t.shape_refused(
        "fx_extract_001.json",
        "a white softness that is a list",
        r#"{"channel": "luminance", "black_point": 0, "white_point": 255, "black_softness": 0, "white_softness": [0], "invert": "off"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_extract_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("black point -1", set(extract("luminance", -1.0, 255.0, 0.0, 0.0, "off"))),
            ("white point 256", set(extract("luminance", 0.0, 256.0, 0.0, 0.0, "off"))),
            ("black softness 256", set(extract("luminance", 0.0, 255.0, 256.0, 0.0, "off"))),
            ("white softness -1", set(extract("luminance", 0.0, 255.0, 0.0, -1.0, "off"))),
            ("channel \"luma\"", set(extract("luma", 0.0, 255.0, 0.0, 0.0, "off"))),
            ("channel \"Red\", written with a capital", set(extract("Red", 0.0, 255.0, 0.0, 0.0, "off"))),
            ("invert \"yes\"", set(extract("luminance", 0.0, 255.0, 0.0, 0.0, "yes"))),
            ("black point keyed to 300", keys("black_point", &[(0, &[0.0]), (4, &[300.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_extract_001.json",
        vec![
            ("every number at its bottom,", set(extract("alpha", 0.0, 0.0, 0.0, 0.0, "on"))),
            ("every number at its top,", set(extract("blue", 255.0, 255.0, 255.0, 255.0, "off"))),
            ("black point keyed from 0 to 255", keys("black_point", &[(0, &[0.0]), (4, &[255.0])])),
            ("white softness keyed from 0 to 255", keys("white_softness", &[(0, &[0.0]), (4, &[255.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_extract_001.json", 0),
        ("fx_extract_006.json", 0),
        ("fx_extract_015.json", 2),
        ("fx_extract_017.json", 0),
    ]);

    t.heading("Pictures: a made-up scan, in `verification/B-147 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-147 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8], scale: usize| {
        png_out::write_rgba(&dir.join(name), w * scale, h * scale, OutputDepth::Eight, &[], bytes).unwrap()
    };
    let art = drawing();
    write("drawing.png", &art, 1);
    let px = |bytes: &[u8], (x, y): (usize, usize)| {
        let i = (y * w + x) * 4;
        [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
    };
    let near = |a: [u8; 4], b: [u8; 4]| a.iter().zip(b).all(|(&u, v)| u.abs_diff(v) <= 1);
    // One point on each part of the drawing.
    let parts = [
        ("paper", (150, 90)),
        ("sky", (10, 10)),
        ("line", (80, 49)),
        ("skin", (80, 30)),
        ("scarf", (80, 73)),
        ("star", (130, 36)),
    ];
    let kept = |bytes: &[u8]| -> Vec<&str> {
        parts.iter().filter(|(_, p)| near(px(bytes, *p), px(&art, *p))).map(|(n, _)| *n).collect()
    };
    let gone = |bytes: &[u8]| -> Vec<&str> { parts.iter().filter(|(_, p)| px(bytes, *p)[3] == 0).map(|(n, _)| *n).collect() };

    let (before, big, said) = picture(&dir, one(serde_json::json!({})));
    write("before.png", &big, 3);
    t.row(
        "before.png, as it starts: every part of the drawing kept as it was; draws cleanly",
        &format!("{said:?}, kept {:?}", kept(&before)),
        said.is_empty() && kept(&before).len() == parts.len(),
    );

    let mut sheet = |name: &str, settings: J, says: &str, want_kept: &[&str], want_gone: &[&str]| {
        let (bytes, big, said) = picture(&dir, one(settings));
        write(name, &big, 3);
        let (k, g) = (kept(&bytes), gone(&bytes));
        t.row(
            &format!("{name}, {says}; draws cleanly"),
            &format!("{said:?}, kept {k:?}, gone {g:?}"),
            said.is_empty() && k == want_kept && g == want_gone,
        );
        bytes
    };
    sheet(
        "paper_gone.png",
        serde_json::json!({ "white_point": 220 }),
        "white point 220: the cream paper, brighter, taken out; the line, sky, skin, scarf and star kept",
        &["sky", "line", "skin", "scarf", "star"],
        &["paper"],
    );
    let soft = sheet(
        "paper_soft.png",
        serde_json::json!({ "white_point": 220, "white_softness": 40 }),
        "white point 220 with white softness 40: the paper gone, the line, sky and scarf whole, and the \
         skin and star, within 40 of the point, faded part way",
        &["sky", "line", "scarf"],
        &["paper"],
    );
    sheet(
        "line_gone.png",
        serde_json::json!({ "black_point": 40 }),
        "black point 40: the dark line and mouth taken out, everything else kept",
        &["paper", "sky", "skin", "scarf", "star"],
        &["line"],
    );
    sheet(
        "line_only.png",
        serde_json::json!({ "black_point": 40, "invert": "on" }),
        "black point 40 inverted: only the line and mouth kept",
        &["line"],
        &["paper", "sky", "skin", "scarf", "star"],
    );
    sheet(
        "middle.png",
        serde_json::json!({ "black_point": 60, "white_point": 200 }),
        "black point 60 to white point 200: only the middle tones, the sky and the scarf, kept",
        &["sky", "scarf"],
        &["paper", "line", "skin", "star"],
    );
    sheet(
        "red_channel.png",
        serde_json::json!({ "channel": "red", "white_point": 100 }),
        "channel red, white point 100: what has little red, the line and the sky, kept; the paper, \
         skin, scarf and star, all red enough, taken out",
        &["sky", "line"],
        &["paper", "skin", "scarf", "star"],
    );
    let (skin, star) = (px(&soft, (80, 30))[3], px(&soft, (130, 36))[3]);
    t.row(
        "in paper_soft.png the skin, nearer the white point, is fainter than the star, and both show a little",
        &format!("skin covering {skin}, star covering {star}"),
        0 < skin && skin < star && star < 255,
    );

    t.finish("B-147_extract_table.md");
}
