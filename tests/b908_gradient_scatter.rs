//! B-908: D-1029, Gradient gains After Effects' Ramp Scatter and Blend With Original:
//! `ramp_scatter` (0 to 256, keyable, absent 0) moves each pixel's place along the ramp by up to
//! ramp_scatter / 512 with a seeded number fixed to the drawing, and `blend_with_original`
//! (0 to 100, keyable, absent 0) mixes the layer as it was back in. A file without them draws as
//! before.
//!
//! Writes `verification/B-908_gradient_scatter_table.md` and draws pictures into
//! `verification/B-908 pictures/`.
//!
//! Every expected pixel is `Fixtures/gradient/expected_gradient.json`, written by
//! `tools/gradient_reference.py` before this code existed and printed in document 25 as
//! FX-GRAD-023 to 032. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    changed, distance, keys, reference, repo, same_json, saved, set, town, Table, MAIN, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

fn is_gradient(e: &Effect) -> bool {
    matches!(e, Effect::Gradient { .. })
}

/// The night city's sky (lane w12f's video 2): dark to light blue, top to bottom, laid on, with
/// `p` laid over it.
fn sky(id: &str, p: &J) -> J {
    let mut all = json!({"shape": "linear", "start": [50, 0], "end": [50, 100],
        "start_color": "#0a1430", "end_color": "#3c6eb4", "start_opacity": 100, "end_opacity": 100,
        "blend": "normal"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.gradient", "enabled": true, "parameters": all})
}

/// `effects` on the street at frame 0, drawn on the processor, straight 8-bit, with what it
/// warned of.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let out = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (out.to_srgb8_straight(), said)
}

/// A 120 by 68 corner of the street, from (180, 100), enlarged four times.
fn zoom(bytes: &[u8]) -> Vec<u8> {
    (0..68 * 4)
        .flat_map(|y| (0..120 * 4).map(move |x| (x, y)))
        .flat_map(|(x, y)| {
            let i = ((100 + y / 4) * TOWN.0 + 180 + x / 4) * 4;
            bytes[i..i + 4].to_vec()
        })
        .collect()
}

/// `json` opened and drawn at `frame` on the processor, from the folder `root`.
fn draw(json: &J, root: &Path, frame: i32) -> Vec<f32> {
    let loaded = persist::load_str(&json.to_string()).expect("the file reads");
    let mut log = FrameLog::new(3);
    render_frame(loaded.document.project(), &Id::new(MAIN), frame, root, 64, &mut log)
        .expect("the frame draws")
        .data()
        .to_vec()
}

#[test]
fn b908_gradient_scatter() {
    let mut t = Table::new(
        "gradient",
        "# B-908: Gradient's Ramp Scatter and Blend With Original (D-1029)\n\nD-1029 adds After \
         Effects' Ramp Scatter as `ramp_scatter` (0 to 256, keyable, absent 0): each pixel's place \
         along the ramp is moved by up to ramp_scatter / 512 with a seeded number fixed to the \
         drawing, against banding; and Blend With Original as `blend_with_original` (0 to 100, \
         keyable, absent 0): the layer as it was mixed back in by that share. A file without them \
         draws as before. Every expected pixel is `Fixtures/gradient/expected_gradient.json`, \
         written by `tools/gradient_reference.py` before this code existed and printed in \
         document 25 as FX-GRAD-023 to 032.\n",
    );

    t.heading("FX-GRAD-023 to 032 (document 25)");
    t.fixtures_numbered("expected_gradient.json", 23..=32);

    t.heading("Older projects: neither setting, as before");
    let root = repo("Fixtures/gradient");
    for n in 1..=22 {
        let file = format!("fx_grad_{n:03}.json");
        let text = fs::read_to_string(root.join(&file)).unwrap();
        let Ok(loaded) = persist::load_str(&text) else {
            continue;
        };
        let old: J = serde_json::from_str(&text).unwrap();
        // FX-GRAD-014's capitals are read lowercase (D-114), so its colours are compared so.
        let lower = |j: &J| -> J { serde_json::from_str(&j.to_string().to_lowercase()).unwrap() };
        let kept = same_json(&lower(&saved(&loaded)), &lower(&old));
        // Its twin with both written 0 draws the same frames.
        let mut twin = old.clone();
        let p = &mut twin["compositions"][0]["layers"][0]["effects"][0]["parameters"];
        p["ramp_scatter"] = json!(0);
        p["blend_with_original"] = json!(0);
        let same = [0, 4].iter().all(|&f| draw(&old, &root, f) == draw(&twin, &root, f));
        t.row(
            &format!("{file} (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0"),
            &format!("saved {}; {}", if kept { "the same" } else { "differently" }, if same { "bit-identical" } else { "differ" }),
            kept && same,
        );
    }

    t.heading("The file");
    let files: Vec<String> = (23..=29).map(|n| format!("fx_grad_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let p = t.saved_parameters("fx_grad_023.json");
    t.row("fx_grad_023.json is saved with Ramp Scatter 64 and no Blend With Original written", &p.to_string(),
        p["ramp_scatter"] == 64.0 && p.get("blend_with_original").is_none());
    let p = t.saved_parameters("fx_grad_025.json");
    t.row("fx_grad_025.json is saved with Blend With Original 50", &p.to_string(), p["blend_with_original"] == 50.0);
    let p = t.saved_parameters("fx_grad_001.json");
    t.row(
        "fx_grad_001.json (neither) is saved with neither written",
        &p.to_string(),
        p.get("ramp_scatter").is_none() && p.get("blend_with_original").is_none(),
    );
    for (file, what, word) in [
        ("fx_grad_030.json", "Ramp Scatter 257", "ramp scatter"),
        ("fx_grad_031.json", "Blend With Original 101", "blend with original"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} ({what}) is refused in a sentence naming {word}"), &why, why.contains(word));
    }
    t.shape_refused(
        "fx_grad_023.json",
        "a Gradient whose Ramp Scatter is a word",
        r##"{"shape": "linear", "start": [50, 0], "end": [50, 100], "start_color": "#ffffff", "end_color": "#6450a0", "start_opacity": 100, "end_opacity": 100, "blend": "normal", "ramp_scatter": "lots"}"##,
    );

    t.heading("How far it reaches");
    let mut draft = changed(&t, "fx_grad_023.json", |_| {});
    draft.scale_distances(|d| d * 0.5);
    let kept = matches!(draft, Effect::Gradient { ramp_scatter, .. } if ramp_scatter == 64.0);
    t.row("a half-size draft preview keeps Ramp Scatter 64, not a distance", &format!("{draft:?}"), kept);
    let grows = changed(&t, "fx_grad_027.json", |_| {}).bounds_expansion();
    t.row("Ramp Scatter never grows the layer", &grows.to_string(), grows == 0);

    t.heading("Commands");
    let mut document = t.load("fx_grad_001.json").document;
    let far = changed(&t, "fx_grad_001.json", |e| {
        if let Effect::Gradient { ramp_scatter, .. } = e {
            *ramp_scatter = 256.5
        }
    });
    t.refused(
        &mut document,
        vec![
            ("ramp scatter 256.5", set(far)),
            ("blend with original keyed to 150", keys("blend_with_original", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    let forty = changed(&t, "fx_grad_001.json", |e| {
        if let Effect::Gradient { ramp_scatter, blend_with_original, .. } = e {
            *ramp_scatter = 40.0;
            *blend_with_original = 25.0;
        }
    });
    t.taken(
        &mut document,
        "fx_grad_001.json",
        vec![
            ("ramp scatter 40, blend with original 25", set(forty)),
            ("ramp scatter keyed from 0 to 256", keys("ramp_scatter", &[(0, &[0.0]), (4, &[256.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_grad_023.json", 0), ("fx_grad_027.json", 2), ("fx_grad_028.json", 0), ("fx_grad_025.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    t.row(
        "the card draws a Gradient with both settings 0 as before; one with either above 0 stays on the processor, so those frames are the processor's",
        "see the rows below",
        true,
    );
    let mut gpu = Gpu::new().expect("a usable card");
    t.card_fixtures(
        &mut gpu,
        "expected_gradient.json",
        &[12, 16, 17, 18, 19, 20, 21, 22, 23, 25, 26, 28, 29, 30, 31, 32],
        is_gradient,
    );
    let plain = reference(|_| json!([]));
    let shots = vec![
        (
            "the night sky, both settings 0 written (on the card)",
            reference(|id| json!([sky(&format!("b908-{id}"), &json!({"ramp_scatter": 0, "blend_with_original": 0, "start_opacity": 60, "end_opacity": 60}))])),
            3,
        ),
        (
            "the night sky, Ramp Scatter 64, Blend With Original 40 (the processor's)",
            reference(|id| json!([sky(&format!("b908-{id}"), &json!({"ramp_scatter": 64, "blend_with_original": 40}))])),
            0,
        ),
    ];
    t.card_reference(&mut gpu, &plain, &shots, is_gradient);

    t.heading("Pictures: in `verification/B-908 pictures/`");
    let dir = repo("verification/B-908 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8], (w, h): (usize, usize)| {
        png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap()
    };
    write("town.png", &town(), TOWN);
    let mut first = Vec::new();
    for (n, s) in [(1, 0), (2, 64), (3, 256)] {
        let (p, said) = picture(&dir, json!([sky("fx-0-0", &json!({"ramp_scatter": s}))]));
        write(&format!("{n}_scatter_{s}.png"), &p, TOWN);
        write(&format!("{n}_scatter_{s}_zoom.png"), &zoom(&p), (480, 272));
        let (most, count) = if first.is_empty() { (0, 0) } else { distance(&p, &first) };
        t.row(
            &format!("{n}_scatter_{s}.png and its four-times corner {n}_scatter_{s}_zoom.png: the street under the night sky at Ramp Scatter {s}; draws cleanly{}", if s == 0 { "" } else { ", and differs from Scatter 0 over many pixels, by at most a third of the light" }),
            &format!("{said:?}; {count} pixels differ from Scatter 0, by up to {most} levels"),
            said.is_empty() && (s == 0 || (count > 1000 && most <= 80)),
        );
        if s == 0 {
            first = p;
        }
    }
    let (p, said) = picture(&dir, json!([sky("fx-0-0", &json!({"blend_with_original": 50}))]));
    write("4_blend_with_original_50.png", &p, TOWN);
    let (most, _) = distance(&p, &first);
    t.row(
        "4_blend_with_original_50.png: the night sky at Blend With Original 50, the street showing through half; draws cleanly and differs from 1_scatter_0.png",
        &format!("{said:?}; up to {most} levels from Scatter 0"),
        said.is_empty() && most > 20,
    );

    t.finish("B-908_gradient_scatter_table.md");
}
