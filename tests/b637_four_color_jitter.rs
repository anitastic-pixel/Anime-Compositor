//! B-637: D-758, 4-Color Gradient gains After Effects' Jitter: `jitter` (0 to 100, keyable,
//! absent 0) nudges each pixel's four weights by up to a tenth with a seeded number fixed to the
//! drawing. A file without it draws as before.
//!
//! Writes `verification/B-637_four_color_jitter_table.md` and draws pictures into
//! `verification/B-637 pictures/`.
//!
//! Every expected pixel is `Fixtures/four_color_gradient/expected_four_color_gradient.json`,
//! written by `tools/four_color_gradient_reference.py` before this code existed and printed in
//! document 25 as FX-4CG-027 to 037. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

fn is_4cg(e: &Effect) -> bool {
    matches!(e, Effect::FourColorGradient { .. })
}

/// A 4-Color Gradient as added from the panel, with `p` laid over it.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"point_1": [10, 10], "point_2": [90, 10], "point_3": [10, 90], "point_4": [90, 90],
        "color_1": "#ffff00", "color_2": "#00ff00", "color_3": "#ff00ff", "color_4": "#0000ff",
        "blend": 100, "opacity": 100, "blending_mode": "normal"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.four_color_gradient", "enabled": true, "parameters": all})
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
fn b637_four_color_jitter() {
    let mut t = Table::new(
        "four_color_gradient",
        "# B-637: 4-Color Gradient's jitter (D-758)\n\nD-758 adds After Effects' Jitter as `jitter` \
         (0 to 100, keyable, absent 0): each pixel's four weights are nudged by up to jitter / 1000 \
         of themselves with a seeded number fixed to the drawing (document 21). A file without it \
         draws as before. Every expected pixel is \
         `Fixtures/four_color_gradient/expected_four_color_gradient.json`, written by \
         `tools/four_color_gradient_reference.py` before this code existed and printed in \
         document 25 as FX-4CG-027 to 037. Tolerance 2e-5.\n",
    );

    t.heading("FX-4CG-027 to 037 (document 25)");
    t.fixtures_numbered("expected_four_color_gradient.json", 27..=37);

    t.heading("Older projects: no jitter, as before");
    let root = repo("Fixtures/four_color_gradient");
    for n in 1..=26 {
        let file = format!("fx_4cg_{n:03}.json");
        let text = fs::read_to_string(root.join(&file)).unwrap();
        let Ok(loaded) = persist::load_str(&text) else {
            continue;
        };
        let old: J = serde_json::from_str(&text).unwrap();
        // FX-4CG-010's capitals are read lowercase (D-208), so its colours are compared so.
        let lower = |j: &J| -> J { serde_json::from_str(&j.to_string().to_lowercase()).unwrap() };
        let kept = same_json(&lower(&saved(&loaded)), &lower(&old));
        // Its twin with Jitter 0 written draws the same frames.
        let mut twin = old.clone();
        twin["compositions"][0]["layers"][0]["effects"][0]["parameters"]["jitter"] = json!(0);
        let same = [0, 4].iter().all(|&f| draw(&old, &root, f) == draw(&twin, &root, f));
        t.row(
            &format!("{file} (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0"),
            &format!("saved {}; {}", if kept { "the same" } else { "differently" }, if same { "bit-identical" } else { "differ" }),
            kept && same,
        );
    }

    t.heading("The file");
    let files: Vec<String> = (27..=34).map(|n| format!("fx_4cg_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved_p = t.saved_parameters("fx_4cg_027.json");
    t.row("fx_4cg_027.json is saved with Jitter 50", &saved_p.to_string(), saved_p["jitter"] == 50.0);
    let saved_p = t.saved_parameters("fx_4cg_001.json");
    t.row(
        "fx_4cg_001.json (no jitter) is saved with no jitter written",
        &saved_p.to_string(),
        saved_p.get("jitter").is_none(),
    );
    for (file, what) in [("fx_4cg_035.json", "Jitter 101"), ("fx_4cg_036.json", "Jitter -1")] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} ({what}) is refused in a sentence naming jitter"), &why, why.contains("jitter"));
    }
    t.shape_refused(
        "fx_4cg_027.json",
        "a 4-Color Gradient whose Jitter is a word",
        r##"{"point_1": [10, 10], "point_2": [90, 10], "point_3": [10, 90], "point_4": [90, 90], "color_1": "#ffff00", "color_2": "#00ff00", "color_3": "#ff00ff", "color_4": "#0000ff", "blend": 100, "opacity": 100, "blending_mode": "normal", "jitter": "lots"}"##,
    );

    t.heading("How far it reaches");
    let mut draft = changed(&t, "fx_4cg_027.json", |_| {});
    draft.scale_distances(|d| d * 0.5);
    let kept = matches!(draft, Effect::FourColorGradient { jitter, .. } if jitter == 50.0);
    t.row("a half-size draft preview keeps Jitter 50, not a distance", &format!("{draft:?}"), kept);
    let grows = changed(&t, "fx_4cg_029.json", |_| {}).bounds_expansion();
    t.row("Jitter 100 never grows the layer", &grows.to_string(), grows == 0);

    t.heading("Commands");
    let mut document = t.load("fx_4cg_001.json").document;
    let far = changed(&t, "fx_4cg_001.json", |e| {
        if let Effect::FourColorGradient { jitter, .. } = e {
            *jitter = 100.5
        }
    });
    t.refused(
        &mut document,
        vec![
            ("jitter 100.5", set(far)),
            ("jitter keyed to 150", keys("jitter", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    let forty = changed(&t, "fx_4cg_001.json", |e| {
        if let Effect::FourColorGradient { jitter, .. } = e {
            *jitter = 40.0
        }
    });
    t.taken(
        &mut document,
        "fx_4cg_001.json",
        vec![
            ("jitter 40", set(forty)),
            ("jitter keyed from 0 to 100", keys("jitter", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_4cg_027.json", 0),
        ("fx_4cg_030.json", 2),
        ("fx_4cg_031.json", 0),
        ("fx_4cg_032.json", 0),
        ("fx_4cg_033.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    t.row(
        "the card draws a 4-Color Gradient without jitter as before; one with jitter above 0 stays on the processor, so those frames are the processor's",
        "see the rows below",
        true,
    );
    let mut gpu = Gpu::new().expect("a usable card");
    t.card_fixtures(
        &mut gpu,
        "expected_four_color_gradient.json",
        &[6, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 29, 31, 32, 33, 34, 35, 36, 37],
        is_4cg,
    );
    let plain = reference(|_| json!([]));
    let shots = vec![
        (
            "4-Color Gradient, opacity 60, Jitter 0 written (on the card)",
            reference(|id| json!([fx(&format!("b637-{id}"), &json!({"opacity": 60, "jitter": 0}))])),
            3,
        ),
        (
            "4-Color Gradient, opacity 60, Jitter 70 (the processor's)",
            reference(|id| json!([fx(&format!("b637-{id}"), &json!({"opacity": 60, "jitter": 70}))])),
            0,
        ),
    ];
    t.card_reference(&mut gpu, &plain, &shots, is_4cg);

    t.heading("Pictures: in `verification/B-637 pictures/`");
    let dir = repo("verification/B-637 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8], (w, h): (usize, usize)| {
        png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap()
    };
    write("town.png", &town(), TOWN);
    let mut first = Vec::new();
    for (n, j) in [(1, 0), (2, 50), (3, 100)] {
        let (p, said) = picture(&dir, json!([fx("fx-0-0", &json!({"jitter": j}))]));
        write(&format!("{n}_jitter_{j}.png"), &p, TOWN);
        write(&format!("{n}_jitter_{j}_zoom.png"), &zoom(&p), (480, 272));
        let (most, count) = if first.is_empty() { (0, 0) } else { distance(&p, &first) };
        t.row(
            &format!("{n}_jitter_{j}.png and its four-times corner {n}_jitter_{j}_zoom.png: the street under the gradient at Jitter {j}; draws cleanly{}", if j == 0 { "" } else { ", and differs from Jitter 0 finely (few levels, many pixels)" }),
            &format!("{said:?}; {count} pixels differ from Jitter 0, by up to {most} levels"),
            said.is_empty() && (j == 0 || (count > 1000 && most <= 40)),
        );
        if j == 0 {
            first = p;
        }
    }

    t.finish("B-637_four_color_jitter_table.md");
}
