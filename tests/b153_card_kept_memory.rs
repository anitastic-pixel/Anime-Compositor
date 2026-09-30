//! B-153: the card keeps the memory it sends drawings through, the textures its drawings sit in
//! and its passes' working textures from frame to frame, rather than making them anew for each
//! (GPU plan G2, D-219). Two checks, on the reference shot with motion blur, with frame mix and
//! dissolve, and with motion blur beside an animated Roughen Edges, at Draft and Full, every
//! eighth frame played twice as the viewer asks:
//!
//! 1. the frames the card draws while it keeps its memory are, byte for byte, the frames a card
//!    opened afresh draws, which has nothing kept;
//! 2. the second time through, the card makes no new memory.
//!
//! Writes `verification/B-153_kept_memory_table.md`.

use std::fmt::Write as _;
use std::fs;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use serde_json::json;

mod common;
use common::repo;

/// The reference shot as B-152 changes it: motion blur on every layer, three of them moved by
/// keys; or frame mix on a stretched second layer and a dissolve on the fourth; or the first,
/// with an animated Roughen Edges in place of its motion blur.
fn shot(kind: &str) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: serde_json::Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    let comp = &mut j["compositions"][0];
    if kind == "mix" {
        comp["frame_blending"] = true.into();
        comp["layers"][1]["time_stretch"] = 150.into();
        comp["layers"][1]["frame_blend"] = "frame_mix".into();
        comp["layers"][3]["drawing_dissolve"] = 2.into();
    } else {
        comp["motion_blur"] = json!({"enabled": true, "shutter_angle": 180, "shutter_phase": -90, "samples": 8});
        let keys = |a: serde_json::Value, b: serde_json::Value| {
            json!({"base": a, "keyframes": [{"frame": 0, "value": a, "interp": "linear"}, {"frame": 239, "value": b, "interp": "linear"}]})
        };
        let layers = comp["layers"].as_array_mut().expect("layers");
        for l in layers.iter_mut() {
            l["motion_blur"] = true.into();
        }
        layers[1]["transform"]["scale"] = keys(json!([100, 100]), json!([130, 130]));
        layers[2]["transform"]["position"] = keys(json!([0, 0]), json!([600, 120]));
        layers[3]["transform"]["rotation"] = keys(json!(0), json!(40));
        if kind == "rough" {
            layers[0]["motion_blur"] = false.into();
            layers[0]["effects"] = json!([{"instance_id": "b153-r", "type_id": "core.roughen_edges", "enabled": true,
                "parameters": {"edge_type": "roughen_color", "edge_color": "#8a3c14", "border": 6, "size": 8, "complexity": 3, "evolution": 30, "speed": 10, "seed": 3}}]);
        }
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{kind}: {}", d.message)).document.project().clone()
}

#[test]
fn b153_card_kept_memory() {
    let out = repo("verification/B-153_kept_memory_table.md");
    let mut gpu = match Gpu::new() {
        Ok(gpu) => gpu,
        Err(why) => {
            fs::write(&out, format!("# B-153: the card keeps its memory\n\n**NOT RUN.** No usable card: {why}\n\nNo check in this table was run, so none of them passes.\n"))
                .expect("write the B-153 table");
            return;
        }
    };
    let (root, comp) = (repo("Fixtures/reference_shot"), Id::new("comp-reference-shot"));
    let (mut rows, mut checks, mut passed) = (String::new(), 0, 0);
    for (kind, name) in [
        ("blur", "the reference shot with motion blur"),
        ("mix", "the reference shot with frame mix and dissolve"),
        ("rough", "the reference shot with motion blur and Roughen Edges"),
    ] {
        let project = shot(kind);
        for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
            let mut cache = CelCache::viewer();
            gpu.forget();
            let (mut compared, mut same, mut made) = (0, 0, [0; 2]);
            for play in 0..2 {
                let before = gpu.made();
                for frame in (0..240).step_by(8) {
                    let mut log = FrameLog::new(3);
                    let (kept, ..) = preview::preview_frame_srgb8(&project, &comp, frame, &root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                        .unwrap_or_else(|d| panic!("{name} frame {frame}: {}", d.message));
                    if play == 1 && frame % 48 == 0 {
                        let mut fresh = Gpu::new().expect("a second card");
                        let mut log = FrameLog::new(3);
                        let (new, ..) = preview::preview_frame_srgb8(&project, &comp, frame, &root, quality, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer(), &mut fresh)
                            .unwrap_or_else(|d| panic!("{name} frame {frame} on a fresh card: {}", d.message));
                        compared += 1;
                        same += (kept == new) as usize;
                    }
                }
                made[play] = gpu.made() - before;
            }
            for (pass, what) in [
                (compared > 0 && same == compared, format!("{same} of {compared} frames byte for byte a fresh card's")),
                (made[1] == 0, format!("{} pieces of memory made the first time through, {} the second", made[0], made[1])),
            ] {
                checks += 1;
                passed += pass as usize;
                let _ = writeln!(rows, "| {name} | {} | {what} | {} |", quality.label(), if pass { "PASS" } else { "FAIL" });
            }
        }
    }
    let s = format!(
        "# B-153: the card keeps its memory from frame to frame\n\n\
         Written by `tests/b153_card_kept_memory.rs`. The card: {}.\n\n\
         Each shot is played twice at Draft and at Full, every eighth frame, as the viewer asks. \
         **The rules:** every 48th frame of the second time through is byte for byte the frame a \
         card opened afresh draws, with nothing kept; and the second time through makes no new \
         memory on the card (textures and the memory drawings are sent through), because what the \
         first time made is used again (D-219).\n\n\
         **{passed} of {checks} checks pass.**\n\n\
         | Shot | Quality | What was found | Result |\n|---|---|---|---|\n{rows}",
        gpu.about(),
    );
    fs::write(&out, s).expect("write the B-153 table");
    assert_eq!(passed, checks, "B-153: {passed} of {checks} checks pass");
}
