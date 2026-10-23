//! B-629: D-750, Curl Noise's Uniform Scaling, Scale Width and Scale Height, after After Effects'
//! Curl Noise Transform: with Uniform Scaling off the noise is stretched across and down apart.
//!
//! Writes `verification/B-629_curl_scale_table.md` and draws pictures into
//! `verification/B-629 pictures/`.
//!
//! Every expected pixel is `Fixtures/curl_noise/expected_curl_transform.json`, written by
//! `tools/curl_transform_reference.py` before this code existed and printed in document 25 as
//! FX-CURL-079 to 085. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, changed, on_card, repo, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::PreviewQuality;
use anime_compositor::{persist, png_out, OutputDepth};

/// A Curl Noise with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"source": "internal", "speed": 10, "direction": 0, "size": 100, "offset": [0, 0], "evolution": 0,
        "turbulence_speed": 20, "swirl": 45, "density": 0, "smoothness": 50, "vertical_bias": 50, "sample_count": 12, "sample_radius": 30,
        "flow_softness": 20, "edge_definition": 50, "flow_falloff": 0, "view": "final_render", "contrast": 100, "brightness": 0,
        "clip_hdr_results": "on", "channel": "rgb"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.curl_noise", "enabled": true, "parameters": all})
}

/// `effects` on the street (in `dir`) at frame 0, drawn by the processor, straight 8-bit, with
/// what it warned of.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    let layer = &mut comp["layers"][0];
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (frame.to_srgb8_straight(), said)
}

/// The mean step in red between neighbours across and down, in levels of 255.
fn steps(p: &[u8]) -> (f64, f64) {
    let (w, h) = (TOWN.0 as usize, TOWN.1 as usize);
    let red = |x: usize, y: usize| p[4 * (y * w + x)] as f64;
    let (mut across, mut down) = (0.0, 0.0);
    for y in 0..h - 1 {
        for x in 0..w - 1 {
            across += (red(x + 1, y) - red(x, y)).abs();
            down += (red(x, y + 1) - red(x, y)).abs();
        }
    }
    let n = ((w - 1) * (h - 1)) as f64;
    (across / n, down / n)
}

#[test]
fn b629_curl_scale() {
    let mut t = Table::new(
        "curl_noise",
        "# B-629: Curl Noise, Uniform Scaling, Scale Width and Scale Height (D-750)\n\n`core.curl_noise` takes After \
         Effects' Uniform Scaling (on when added) and, with it off, Scale Width and Scale Height (100 each): the noise is \
         read at q = c + (mx (d_x cos + d_y sin), my (-d_x sin + d_y cos)), mx = 100 / Scale Width and my = 100 / Scale \
         Height; with Uniform Scaling on, mx = my = 100 / Scale as before. A picture (This Layer or Other Layer) is not \
         scaled. A file from before reads as Uniform Scaling on and draws as it did. Every expected pixel is \
         `Fixtures/curl_noise/expected_curl_transform.json`, written by `tools/curl_transform_reference.py` before this \
         code existed and printed in document 25 as FX-CURL-079 to 085. Tolerance 2e-5.\n",
    );

    t.heading("FX-CURL-079 to 085 (document 25)");
    t.fixtures_numbered("expected_curl_transform.json", 79..=85);

    t.heading("The file");
    let all: Vec<String> = (79..=85).map(|n| format!("fx_curl_{n:03}.json")).collect();
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let old = t.saved_parameters("fx_curl_001.json");
    t.row(
        "fx_curl_001.json, from before D-750, saves without the three new settings",
        &old.to_string(),
        ["uniform_scaling", "scale_width", "scale_height"]
            .iter()
            .all(|k| old.get(k).is_none()),
    );
    let read = changed(&t, "fx_curl_001.json", |_| {});
    let as_added = matches!(&read, Effect::CurlNoise { uniform_scaling, scale_width, scale_height, .. }
        if uniform_scaling == "on" && *scale_width == 100.0 && *scale_height == 100.0);
    t.row(
        "fx_curl_001.json opens as Uniform Scaling on, Scale Width 100, Scale Height 100",
        &format!("{as_added}"),
        as_added,
    );
    let saved = t.saved_parameters("fx_curl_079.json");
    t.row(
        "fx_curl_079.json is saved with Uniform Scaling off, Scale Width 200 and Scale Height 100",
        &saved.to_string(),
        saved["uniform_scaling"] == "off" && saved["scale_width"] == json!(200),
    );
    for (file, want) in [
        (
            "fx_curl_083.json",
            "Curl Noise's scale width runs from 1 to 10000, and this is 0.5.",
        ),
        (
            "fx_curl_084.json",
            "Curl Noise's uniform scaling is \"on\" or \"off\", and this is \"yes\".",
        ),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == want);
    }

    t.heading("Commands");
    let mut document = t.load("fx_curl_079.json").document;
    let with = |f: fn(&mut Effect)| changed(&t, "fx_curl_079.json", f);
    let refused = vec![
        (
            "scale height 20000",
            set(with(|e| {
                if let Effect::CurlNoise { scale_height, .. } = e {
                    *scale_height = 20000.0
                }
            })),
        ),
        (
            "uniform scaling \"yes\"",
            set(with(|e| {
                if let Effect::CurlNoise { uniform_scaling, .. } = e {
                    *uniform_scaling = "yes".into()
                }
            })),
        ),
    ];
    let taken = vec![(
        "uniform scaling off, scale width 50, scale height 400",
        set(with(|e| {
            if let Effect::CurlNoise {
                uniform_scaling,
                scale_width,
                scale_height,
                ..
            } = e
            {
                *uniform_scaling = "off".into();
                *scale_width = 50.0;
                *scale_height = 400.0;
            }
        })),
    )];
    t.refused(&mut document, refused);
    t.taken(&mut document, "fx_curl_079.json", taken);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_curl_079.json", 0), ("fx_curl_082.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    for n in 79..=85u32 {
        let file = format!("fx_curl_{n:03}.json");
        let project = persist::load(&t.root.join(&file))
            .unwrap()
            .document
            .project()
            .clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| {
                    matches!(e, Effect::CurlNoise { .. })
                });
            }
        }
        // The refused ones (083 to 085) leave nothing for the card.
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == (n >= 83),
        );
    }

    t.heading("Pictures: the street, Input Noise, in `verification/B-629 pictures/`");
    let dir = repo("verification/B-629 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| {
        png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes)
            .unwrap()
    };
    write("town.png", &town());
    let street = |p: J| picture(&dir, json!([fx("fx-0-0", &p)]));
    let noise = json!({"size": 10, "edge_definition": 0, "view": "input_noise"});
    let with_noise = |more: J| {
        let mut p = noise.clone();
        for (k, v) in more.as_object().unwrap() {
            p[k] = v.clone();
        }
        p
    };
    let (plain, said) = street(noise.clone());
    write("1_uniform_on.png", &plain);
    let (a0, d0) = steps(&plain);
    t.row(
        "1_uniform_on.png, Size 10, Edge Definition 0 (no greys clipped), Uniform Scaling on: the noise as before, as busy across as down; draws cleanly",
        &format!("{said:?}; mean step across {a0:.2}, down {d0:.2} levels"),
        said.is_empty() && a0 / d0 > 0.7 && a0 / d0 < 1.4,
    );
    let (ignored, said) = street(with_noise(json!({"scale_width": 300, "scale_height": 50})));
    t.row(
        "Uniform Scaling on with Scale Width 300 and Scale Height 50 written: the two are not read, the same picture as 1_uniform_on.png",
        &format!("{said:?}, the same {}", ignored == plain),
        said.is_empty() && ignored == plain,
    );
    let (wide, said) = street(with_noise(json!({"uniform_scaling": "off", "scale_width": 300, "scale_height": 100})));
    write("2_width_300.png", &wide);
    let (a1, d1) = steps(&wide);
    t.row(
        "2_width_300.png, Uniform Scaling off, Scale Width 300, Scale Height 100: the blobs three times as wide, as tall as before, so the steps across are under half the steps down",
        &format!("{said:?}; mean step across {a1:.2}, down {d1:.2} levels"),
        said.is_empty() && a1 < 0.5 * d1,
    );
    let (streaks, said) = street(with_noise(json!({"uniform_scaling": "off", "scale_width": 7500, "scale_height": 5})));
    write("3_streaks_7500_by_5.png", &streaks);
    let (a2, d2) = steps(&streaks);
    t.row(
        "3_streaks_7500_by_5.png, Scale Width 7500, Scale Height 5 (fimje's long streaks): long level streaks, the steps down more than ten times the steps across",
        &format!("{said:?}; mean step across {a2:.2}, down {d2:.2} levels"),
        said.is_empty() && d2 > 10.0 * a2,
    );

    t.finish("B-629_curl_scale_table.md");
}
