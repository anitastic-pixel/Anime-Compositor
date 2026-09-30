//! B-159: after an edit, the viewer starts the frame from the layers below the edited one as it
//! last drew them, rather than drawing every layer again (GPU plan G10, D-241). The reference
//! shot as it is, with an adjustment layer, with a Light Wrap, with motion blur and with effects,
//! at Full and Draft, frames 0 and 100, the whole frame and the middle quarter; each layer edited
//! in turn three ways (opacity 80%, opacity 60%, moved), each edit drawn by one viewer that keeps
//! what it drew:
//!
//! 1. every frame is, byte for byte, the frame a viewer that keeps nothing draws of the same edit;
//! 2. the second and third edits of any layer but the bottom one start from the layers below it
//!    as kept, which is the saving;
//! 3. the next frame, drawn after the edits, starts from nothing kept, and is the same bytes.
//!
//! Writes `verification/B-159_below_table.md`.

use std::fmt::Write as _;
use std::fs;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use serde_json::{json, Value};

mod common;
use common::repo;

const SHOTS: [&str; 5] = ["as it is", "adjustment layer", "Light Wrap", "motion blur", "effects"];

fn shot(kind: &str) -> Value {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    let comp = &mut j["compositions"][0];
    match kind {
        "adjustment layer" => {
            let still = json!({"base": 0, "keyframes": []});
            let adj = json!({"id": "b159-adj", "kind": "adjustment", "name": "adj", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 240,
                "transform": {"anchor": {"base": [0, 0], "keyframes": []}, "position": {"base": [0, 0], "keyframes": []},
                    "scale": {"base": [100, 100], "keyframes": []}, "rotation": still, "opacity": {"base": 0.7, "keyframes": []}},
                "mask": null, "matte": null, "blend_mode": "normal",
                "effects": [{"instance_id": "b159-e", "type_id": "core.exposure", "enabled": true, "parameters": {"stops": 1}}]});
            comp["layers"].as_array_mut().expect("layers").insert(2, adj);
            comp["layer_order"].as_array_mut().expect("layer order").insert(2, "b159-adj".into());
        }
        "Light Wrap" => {
            comp["layers"][2]["effects"] = json!([{"instance_id": "b159-w", "type_id": "core.light_wrap", "enabled": true,
                "parameters": {"width": 20, "intensity": 100, "blend": "screen"}}]);
        }
        "motion blur" => {
            comp["motion_blur"] = json!({"enabled": true, "shutter_angle": 180, "shutter_phase": -90, "samples": 8});
            let keys = |a: Value, b: Value| {
                json!({"base": a, "keyframes": [{"frame": 0, "value": a, "interp": "linear"}, {"frame": 239, "value": b, "interp": "linear"}]})
            };
            let layers = comp["layers"].as_array_mut().expect("layers");
            for l in layers.iter_mut() {
                l["motion_blur"] = true.into();
            }
            layers[1]["transform"]["scale"] = keys(json!([100, 100]), json!([130, 130]));
            layers[2]["transform"]["position"] = keys(json!([0, 0]), json!([600, 120]));
        }
        "effects" => {
            comp["layers"][1]["effects"] = json!([{"instance_id": "b159-b", "type_id": "core.gaussian_blur", "enabled": true,
                "parameters": {"sigma_px": 6.0}}]);
            comp["layers"][3]["effects"] = json!([{"instance_id": "b159-x", "type_id": "core.exposure", "enabled": true,
                "parameters": {"stops": 0.5}}]);
        }
        _ => {}
    }
    j
}

const EDITS: [&str; 3] = ["opacity 80%", "opacity 60%", "moved"];

fn edited(base: &Value, layer: usize, edit: usize) -> Project {
    let mut j = base.clone();
    let t = &mut j["compositions"][0]["layers"][layer]["transform"];
    match edit {
        0 => t["opacity"] = json!({"base": 0.8, "keyframes": []}),
        1 => t["opacity"] = json!({"base": 0.6, "keyframes": []}),
        _ => t["position"] = json!({"base": [37, -12], "keyframes": []}),
    }
    load(&j)
}

fn load(j: &Value) -> Project {
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{}", d.message)).document.project().clone()
}

const MIDDLE: [f64; 4] = [0.25, 0.25, 0.75, 0.75];

/// One frame, whole or the middle quarter, with `cache`.
fn draw(p: &Project, frame: i32, quality: PreviewQuality, part: bool, cache: &mut CelCache) -> Vec<u32> {
    let (root, comp) = (repo("Fixtures/reference_shot"), Id::new("comp-reference-shot"));
    let mut log = FrameLog::new(3);
    let pixels = if part {
        preview::preview_part(p, &comp, frame, &root, quality, DEFAULT_TILE_SIZE, &mut log, cache, MIDDLE)
            .unwrap_or_else(|d| panic!("{}", d.message))
            .0
    } else {
        preview::preview_frame_cached(p, &comp, frame, &root, quality, DEFAULT_TILE_SIZE, &mut log, cache)
            .unwrap_or_else(|d| panic!("{}", d.message))
    };
    pixels.data().iter().map(|v| v.to_bits()).collect()
}

#[test]
fn b159_below() {
    // A viewer that keeps nothing it drew: cels are kept (B-08b, which changes nothing but the
    // clock), no effect results and no layers below.
    let mut plain = CelCache::with_budget(1 << 32);
    let (mut rows, mut checks, mut passed) = (String::new(), 0, 0);
    for kind in SHOTS {
        let base = shot(kind);
        let layers = base["compositions"][0]["layers"].as_array().expect("layers").len();
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100] {
                for part in [false, true] {
                    let mut viewer = CelCache::viewer();
                    draw(&load(&base), frame, quality, part, &mut viewer);
                    let mut row = |what: String, warm: Vec<u32>, cold: Vec<u32>, reused: bool, must: Option<bool>| {
                        let same = warm == cold;
                        let ok = same && must.is_none_or(|m| m == reused);
                        checks += 1;
                        passed += ok as usize;
                        writeln!(
                            rows,
                            "| {kind} | {} | {frame} | {} | {what} | {} | {} | {} |",
                            quality.label(),
                            if part { "middle quarter" } else { "whole" },
                            if same { "yes" } else { "**NO**" },
                            if reused { "yes" } else { "no" },
                            if ok { "pass" } else { "**FAIL**" },
                        )
                        .unwrap();
                    };
                    for layer in 0..layers {
                        for (edit, name) in EDITS.iter().enumerate() {
                            let p = edited(&base, layer, edit);
                            let before = viewer.below_reused();
                            let warm = draw(&p, frame, quality, part, &mut viewer);
                            let reused = viewer.below_reused() > before;
                            let cold = draw(&p, frame, quality, part, &mut plain);
                            let must = (layer > 0 && edit > 0).then_some(true);
                            row(format!("layer {} {name}", layer + 1), warm, cold, reused, must);
                        }
                    }
                    let p = load(&base);
                    let before = viewer.below_reused();
                    let warm = draw(&p, frame + 1, quality, part, &mut viewer);
                    let reused = viewer.below_reused() > before;
                    let cold = draw(&p, frame + 1, quality, part, &mut plain);
                    row(format!("next frame, {}, as it is", frame + 1), warm, cold, reused, Some(false));
                }
            }
        }
    }
    let text = format!(
        "# B-159: the layers below an edit, kept\n\nWritten by `tests/b159_below.rs`. Each layer of the reference shot, \
         in five forms, is edited in turn three ways, and each edit is drawn by one viewer that keeps what it drew \
         and compared, every working-space value by its bits, with a viewer that keeps nothing. \"Started from kept\" \
         is whether the viewer began that frame from the layers below the edited one as it last drew them; the second \
         and third edits of every layer but the bottom one must, and the next frame after the edits must not. \
         Layers count from the bottom; the adjustment layer is layer 3 of its shot.\n\n\
         **{passed} of {checks} pass.**\n\n\
         | Shot | Quality | Frame | Drawn | Edit | Same bytes | Started from kept | Result |\n\
         |---|---|---:|---|---|---|---|---|\n{rows}"
    );
    fs::write(repo("verification/B-159_below_table.md"), text).expect("write the B-159 table");
    assert_eq!(passed, checks, "see verification/B-159_below_table.md");
}

/// The saving: frame 10 of the reference shot and of the declared ten-layer fixture, one layer's
/// opacity set to 80% and 60% by turns, twenty times, as a slider dragged back and forth asks,
/// by one viewer that keeps what it drew. The top layer, the middle one, and the second from
/// the bottom, which has the least below it to keep.
///
/// Writes `verification/B-159_timing_raw.md`; the committed table is made from three runs.
#[test]
#[ignore = "a measurement; run it deliberately, in release, on the recorded machine"]
fn b159_timing() {
    let (_, declared_root, declared) = common::build_fixture("b159");
    let declared: Value = serde_json::from_str(&declared).expect("the fixture is JSON");
    let shots = [
        ("reference shot", shot("as it is"), repo("Fixtures/reference_shot")),
        ("declared ten-layer fixture", declared, declared_root),
    ];
    let mut rows = String::new();
    for (name, base, root) in &shots {
        let comp = Id::new(base["compositions"][0]["id"].as_str().expect("an id"));
        let order: Vec<String> = base["compositions"][0]["layer_order"]
            .as_array()
            .expect("a layer order")
            .iter()
            .map(|v| v.as_str().expect("an id").to_string())
            .collect();
        let n = order.len();
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for (which, at) in [("top", n - 1), ("middle", n / 2), ("second from bottom", 1)] {
                let with = |opacity: f64| {
                    let mut j = base.clone();
                    for l in j["compositions"][0]["layers"].as_array_mut().expect("layers") {
                        if l["id"] == order[at].as_str() {
                            l["transform"]["opacity"] = json!({"base": opacity, "keyframes": []});
                        }
                    }
                    load(&j)
                };
                let edits = [with(0.8), with(0.6)];
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let mut once = |p: &Project, cache: &mut CelCache| {
                    let at = std::time::Instant::now();
                    preview::preview_frame_cached(p, &comp, 10, root, quality, DEFAULT_TILE_SIZE, &mut log, cache)
                        .unwrap_or_else(|d| panic!("{}", d.message));
                    at.elapsed().as_secs_f64() * 1000.0
                };
                // The frame as it is, then the first edit, which is where what is below is kept.
                once(&load(base), &mut cache);
                once(&edits[1], &mut cache);
                let mut ms: Vec<f64> = (0..20).map(|i| once(&edits[i % 2], &mut cache)).collect();
                ms.sort_by(f64::total_cmp);
                writeln!(rows, "| {name} | {} | {which} ({} of {n}) | {:.1} |", quality.label(), at + 1, ms[ms.len() / 2]).unwrap();
            }
        }
    }
    let text = format!(
        "# B-159: an edit drawn again, one run\n\nWritten by `b159_timing`, release build. Median of twenty draws \
         of frame 10, milliseconds of planning and drawing on the processor, the layer's opacity 80% and 60% by turns.\n\n\
         | Shot | Quality | Layer edited | ms |\n|---|---|---|---:|\n{rows}"
    );
    fs::write(repo("verification/B-159_timing_raw.md"), text).expect("write the table");
}
