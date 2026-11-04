//! B-892: Warp Stabilizer's Crop Less <-> Smooth More, against D-1012.
//!
//! Writes `verification/D-1012_warp_stabilizer_2_table.md`.
//!
//! Every expected pixel is `Fixtures/warp_stabilizer_2/expected_warp_stabilizer_2.json`, written
//! by `tools/warp_stabilizer_2_reference.py` before this code existed and printed in document 25
//! as FX-WS2-001 to 010. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a striped scene filmed by a shaking hand, over a night-blue solid, steadied with
//! Stabilize, Crop at 100, 50 and 0, into `verification/D-1012 pictures/`.

mod effect_table;

use std::f64::consts::PI;
use std::fs;

use effect_table::{Table, MAIN};
use serde_json::Value as J;

use anime_compositor::command::Command;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::model::{Id, Interp};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

fn ws(framing: &str, c: f64) -> Effect {
    Effect::WarpStabilizer {
        result: "smooth_motion".to_string(),
        smoothness: 50.0,
        method: "position_scale_rotation".to_string(),
        preserve_scale: "off".to_string(),
        framing: framing.to_string(),
        maximum_scale: 150.0,
        action_safe_margin: 0.0,
        additional_scale: 100.0,
        crop_less_smooth_more: c,
        detailed_analysis: "off".to_string(),
        picture: None,
    }
}

fn set(effect: Effect) -> Command {
    Command::SetEffectParameters {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        effect,
    }
}

const PICTURE: (usize, usize) = (480, 270);
const FRAMES: usize = 24;

/// As B-532's: a slow pan and a jolt of up to 5 pixels across and 4 down.
fn hand_at(m: usize) -> (f64, f64) {
    let m = m as f64;
    (
        0.5 * m + 5.0 * (2.1 * m).sin(),
        4.0 * (1.7 * m + 0.5).cos() - 4.0 * 0.5f64.cos(),
    )
}

/// As B-532's: drawing `m`, a striped opaque scene seen through the shaking camera.
fn scene(m: usize) -> Vec<u8> {
    let (w, h) = PICTURE;
    let (ox, oy) = hand_at(m);
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (u, v) = (x as f64 + 0.5 - ox, y as f64 + 0.5 - oy);
            let t = 0.5
                + 0.3 * (2.0 * PI * u / 53.0).sin()
                + 0.15 * (2.0 * PI * v / 29.0 + u / 31.0).cos();
            let c = |a: f64| (a.clamp(0.0, 255.0)).round() as u8;
            bytes.extend([c(255.0 * t), c(90.0 + 100.0 * t), c(230.0 - 150.0 * t), 255]);
        }
    }
    bytes
}

/// Frame `frame` of the scene over night blue, Warp Stabilizer with Stabilize, Crop at `c`.
fn draw(dir: &std::path::Path, c: f64, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/warp_stabilizer/fx_ws_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["pattern"] = J::from("scene_##.png");
    project["assets"][0]["frames"] = (1..=FRAMES)
        .map(|m| (m.to_string(), J::from(format!("scene_{m:02}.png"))))
        .collect::<serde_json::Map<_, _>>()
        .into();
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(PICTURE.0);
    comp["height"] = J::from(PICTURE.1);
    comp["duration_frames"] = J::from(FRAMES);
    comp["work_area"]["end_frame_exclusive"] = J::from(FRAMES);
    let layers = comp["layers"].as_array_mut().unwrap();
    let holder = &mut layers[0];
    holder["out_frame"] = J::from(FRAMES);
    holder["effects"] = serde_json::json!([{"instance_id": "fx-1", "type_id": "core.warp_stabilizer", "enabled": true, "parameters": {
        "result": "smooth_motion", "smoothness": 50, "method": "position_scale_rotation", "preserve_scale": "off",
        "framing": "stabilize_crop", "maximum_scale": 150, "action_safe_margin": 0, "additional_scale": 100,
        "crop_less_smooth_more": c, "detailed_analysis": "off"}}]);
    holder["exposure_spans"] = (0..FRAMES)
        .map(|m| serde_json::json!({"start_frame": m, "end_frame_exclusive": m + 1, "drawing_number": m + 1}))
        .collect();
    let mut night = holder.clone();
    for field in ["asset_id", "source_offset_frames", "exposure_spans"] {
        night.as_object_mut().unwrap().remove(field);
    }
    night["id"] = J::from("night");
    night["name"] = J::from("night");
    night["kind"] = J::from("solid");
    night["effects"] = J::Array(vec![]);
    night["solid"] = serde_json::json!({"color": [0.0070, 0.0116, 0.0452], "width": PICTURE.0, "height": PICTURE.1});
    layers.insert(0, night);
    comp["layer_order"].as_array_mut().unwrap().insert(0, J::from("night"));
    let document = persist::load_str(&project.to_string()).expect("the picture's project reads").document;
    let mut log = FrameLog::new(3);
    let drawn = preview::preview_frame(
        document.project(),
        &Id::new(MAIN),
        frame,
        dir,
        PreviewQuality::Full,
        64,
        &mut log,
    )
    .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

/// How many pixels show the night-blue solid, that is, nothing of the scene.
fn uncovered(bytes: &[u8]) -> usize {
    bytes.chunks(4).filter(|p| *p == [20, 28, 60, 255]).count()
}

/// The mean difference, in 8-bit levels, between two frames' middle halves.
fn jitter(a: &[u8], b: &[u8]) -> f64 {
    let (w, h) = PICTURE;
    let (mut sum, mut n) = (0.0, 0.0);
    for y in h / 4..3 * h / 4 {
        for x in w / 4..3 * w / 4 {
            for c in 0..3 {
                let i = (y * w + x) * 4 + c;
                sum += (a[i] as f64 - b[i] as f64).abs();
                n += 1.0;
            }
        }
    }
    sum / n
}

#[test]
fn b892_warp_stabilizer_2() {
    let mut t = Table::new(
        "warp_stabilizer_2",
        "# B-892: Warp Stabilizer's Crop Less <-> Smooth More\n\nD-1012, wave 12 (lane w12u, the \
         Warp Stabilizer tutorial at 5:00). Every expected pixel is \
         `Fixtures/warp_stabilizer_2/expected_warp_stabilizer_2.json`, written by \
         `tools/warp_stabilizer_2_reference.py` before this code existed and printed in document \
         25 as FX-WS2-001 to 010; tolerance 2e-5.\n",
    );

    t.heading("FX-WS2-001 to 010 (document 25)");
    t.fixtures("expected_warp_stabilizer_2.json");

    t.heading("The file");
    let files: Vec<String> = (1..=8).map(|n| format!("fx_ws2_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_ws2_008.json");
    t.row(
        "fx_ws2_008.json, a file from before D-1012, saves without the setting",
        &format!("{:?}", params.get("crop_less_smooth_more")),
        params.get("crop_less_smooth_more").is_none(),
    );
    for file in ["fx_ws2_009.json", "fx_ws2_010.json"] {
        let why = t
            .load(file)
            .document
            .project()
            .composition(&Id::new(MAIN))
            .unwrap()
            .layer(&Id::new("holder"))
            .unwrap()
            .effects[0]
            .effect
            .why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming the setting"),
            &why,
            why.contains("crop less smooth more"),
        );
    }

    t.heading("Commands");
    let mut document = t.load("fx_ws2_001.json").document;
    let held = format!("{:?}", document.project().composition(&Id::new(MAIN)).unwrap());
    let refused = document.apply(set(ws("stabilize_crop_auto_scale", 101.0))).err();
    t.row(
        "Crop Less <-> Smooth More 101 is refused with a sentence, and nothing changes",
        &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
        refused.is_some()
            && format!("{:?}", document.project().composition(&Id::new(MAIN)).unwrap()) == held,
    );
    t.taken(
        &mut document,
        "fx_ws2_001.json",
        vec![
            ("Crop Less <-> Smooth More 0", set(ws("stabilize_crop_auto_scale", 0.0))),
            (
                "Crop Less <-> Smooth More keyed from 0 at frame 0 to 100 at frame 4",
                Command::SetEffectKeys {
                    composition: Id::new(MAIN),
                    layer_id: Id::new("holder"),
                    instance_id: Id::new("fx-1"),
                    setting: "crop_less_smooth_more".to_string(),
                    keys: [(0, 0.0), (4, 100.0)]
                        .iter()
                        .map(|&(frame, v)| EffectKey { frame, value: vec![v], interp: Interp::Linear })
                        .collect(),
                },
            ),
        ],
    );

    t.heading("Pictures: the shaken striped scene, Stabilize, Crop, in `verification/D-1012 pictures/`");
    let dir = effect_table::repo("verification/D-1012 pictures");
    fs::create_dir_all(&dir).unwrap();
    let scenes = std::env::temp_dir().join("b892_scenes");
    fs::create_dir_all(&scenes).unwrap();
    let (w, h) = PICTURE;
    for m in 0..FRAMES {
        png_out::write_rgba(&scenes.join(format!("scene_{:02}.png", m + 1)), w, h, OutputDepth::Eight, &[], &scene(m))
            .unwrap();
    }
    let mut seen = vec![];
    for (name, c) in [("1_smooth_more_100", 100.0), ("2_as_added_50", 50.0), ("3_crop_less_0", 0.0)] {
        let (three, said3) = draw(&scenes, c, 3);
        let (four, said4) = draw(&scenes, c, 4);
        png_out::write_rgba(&dir.join(format!("{name}_frame_4.png")), w, h, OutputDepth::Eight, &[], &four).unwrap();
        let (cut, shake) = (uncovered(&four), jitter(&three, &four));
        t.row(
            &format!("{name}_frame_4.png, {c}: draws cleanly; night-blue pixels at frame 4, and how much frames 3 and 4 differ"),
            &format!("{said3:?} {said4:?}, {cut} pixels, {shake:.1} levels"),
            said3.is_empty() && said4.is_empty(),
        );
        seen.push((cut, shake));
    }
    t.row(
        "lower cuts less: fewer night-blue pixels at 50 than at 100, none at 0",
        &format!("{} > {} > {}", seen[0].0, seen[1].0, seen[2].0),
        seen[0].0 > seen[1].0 && seen[2].0 == 0,
    );
    t.row(
        "lower steadies less: frames 3 and 4 differ more at 50 than at 100, and most at 0",
        &format!("{:.1} < {:.1} < {:.1}", seen[0].1, seen[1].1, seen[2].1),
        seen[0].1 < seen[1].1 && seen[1].1 < seen[2].1,
    );

    t.finish("D-1012_warp_stabilizer_2_table.md");
}
