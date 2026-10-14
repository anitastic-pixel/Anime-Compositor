//! B-532: Warp Stabilizer in the core, against D-653.
//!
//! Writes `verification/D-653_warp_stabilizer_table.md`.
//!
//! Every expected pixel is `Fixtures/warp_stabilizer/expected_warp_stabilizer.json`, written by
//! `tools/warp_stabilizer_reference.py` (with the motion estimate of `tools/flow_reference.py`)
//! before this code existed and printed in document 25 as FX-WS-001 to 030. Tolerance 2e-5.
//! Nothing here is a snapshot of a run.
//!
//! It also draws a striped scene filmed by a shaking hand, over a night-blue solid, steadied
//! three ways, into `verification/D-653 pictures/`.

mod effect_table;

use std::f64::consts::PI;
use std::fs;
use std::path::Path;

use effect_table::Table;
use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::model::{Id, Interp};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// Warp Stabilizer as added, with its result, method, framing and smoothness set.
fn ws(result: &str, method: &str, framing: &str, smoothness: f64) -> Effect {
    Effect::WarpStabilizer {
        result: result.to_string(),
        smoothness,
        method: method.to_string(),
        preserve_scale: "off".to_string(),
        framing: framing.to_string(),
        maximum_scale: 150.0,
        action_safe_margin: 0.0,
        additional_scale: 100.0,
        detailed_analysis: "off".to_string(),
        picture: None,
    }
}

fn as_added() -> Effect {
    ws("smooth_motion", "position_scale_rotation", "stabilize_crop_auto_scale", 50.0)
}

fn main_comp() -> Id {
    Id::new(effect_table::MAIN)
}

/// Set the holder's Warp Stabilizer.
fn set(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: main_comp(), layer_id: Id::new("holder"), instance_id: Id::new("fx-1"), effect }
}

/// Key a setting of the holder's Warp Stabilizer, linearly.
fn keys(setting: &str, values: &[(i32, f64)]) -> Command {
    Command::SetEffectKeys {
        composition: main_comp(),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        setting: setting.to_string(),
        keys: values.iter().map(|&(frame, v)| EffectKey { frame, value: vec![v], interp: Interp::Linear }).collect(),
    }
}

/// The whole composition, to see that a refused command changed nothing.
fn everything(document: &Document) -> String {
    format!("{:?}", document.project().composition(&main_comp()).unwrap())
}

fn refused(t: &mut Table, document: &mut Document, commands: Vec<(&str, Command)>) {
    let held = everything(document);
    for (what, command) in commands {
        let refused = document.apply(command).err();
        let untouched = everything(document) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
}

const PICTURE: (usize, usize) = (480, 270);
const FRAMES: usize = 24;

/// Where the hand holds the camera at drawing `m` (from 0): a slow pan of half a pixel a frame
/// to the right, and a jolt of up to 5 pixels across and 4 down.
fn hand_at(m: usize) -> (f64, f64) {
    let m = m as f64;
    (0.5 * m + 5.0 * (2.1 * m).sin(), 4.0 * (1.7 * m + 0.5).cos() - 4.0 * 0.5f64.cos())
}

/// Drawing `m`: a striped, opaque scene seen through the shaking camera.
fn scene(m: usize) -> Vec<u8> {
    let (w, h) = PICTURE;
    let (ox, oy) = hand_at(m);
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (u, v) = (x as f64 + 0.5 - ox, y as f64 + 0.5 - oy);
            let t = 0.5 + 0.3 * (2.0 * PI * u / 53.0).sin() + 0.15 * (2.0 * PI * v / 29.0 + u / 31.0).cos();
            let c = |a: f64| (a.clamp(0.0, 255.0)).round() as u8;
            bytes.extend([c(255.0 * t), c(90.0 + 100.0 * t), c(230.0 - 150.0 * t), 255]);
        }
    }
    bytes
}

/// The scene filmed by hand over a night-blue solid, with `effects` on the scene.
fn picture(effects: J) -> Document {
    let mut project: J =
        serde_json::from_str(&fs::read_to_string(effect_table::repo("Fixtures/warp_stabilizer/fx_ws_001.json")).unwrap()).unwrap();
    project["assets"][0]["pattern"] = J::from("scene_##.png");
    project["assets"][0]["frames"] =
        (1..=FRAMES).map(|m| (m.to_string(), J::from(format!("scene_{m:02}.png")))).collect::<serde_json::Map<_, _>>().into();
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(PICTURE.0);
    comp["height"] = J::from(PICTURE.1);
    comp["duration_frames"] = J::from(FRAMES);
    comp["work_area"]["end_frame_exclusive"] = J::from(FRAMES);
    let layers = comp["layers"].as_array_mut().unwrap();
    let holder = &mut layers[0];
    holder["out_frame"] = J::from(FRAMES);
    holder["effects"] = effects;
    holder["exposure_spans"] = (0..FRAMES)
        .map(|m| serde_json::json!({"start_frame": m, "end_frame_exclusive": m + 1, "drawing_number": m + 1}))
        .collect();
    // Night blue, sRGB 20, 28 and 60, in the working space.
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
    persist::load_str(&project.to_string()).expect("the picture's project reads").document
}

/// `document`'s frame at `quality`, as bytes, with what it said.
fn draw(document: &Document, dir: &Path, frame: i32, quality: PreviewQuality) -> (usize, Vec<u8>, Vec<String>) {
    let mut log = FrameLog::new(3);
    let drawn = preview::preview_frame(document.project(), &main_comp(), frame, dir, quality, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.width(), drawn.to_srgb8_straight(), said)
}

/// How many pixels show the night-blue solid, that is, nothing of the scene.
fn uncovered(bytes: &[u8]) -> usize {
    bytes.chunks(4).filter(|p| *p == [20, 28, 60, 255]).count()
}

/// The mean difference, in 8-bit levels over red, green and blue, between two frames' middle
/// halves.
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

/// Warp Stabilizer's saved parameters as added, with `result` and `framing`.
fn added(result: &str, framing: &str) -> serde_json::Map<String, J> {
    serde_json::json!({
        "result": result, "smoothness": 50, "method": "position_scale_rotation", "preserve_scale": "off",
        "framing": framing, "maximum_scale": 150, "action_safe_margin": 0, "additional_scale": 100,
        "detailed_analysis": "off",
    })
    .as_object()
    .unwrap()
    .clone()
}

fn effect(result: &str, framing: &str) -> J {
    serde_json::json!([{"instance_id": "fx-1", "type_id": "core.warp_stabilizer", "enabled": true, "parameters": added(result, framing)}])
}

#[test]
fn b532_warp_stabilizer() {
    let mut t = Table::new(
        "warp_stabilizer",
        "# B-532: Warp Stabilizer\n\nD-653, under the owner's /loop request (P0-13, D-600: \
         \"Blur + retiming only\"). Every expected pixel is \
         `Fixtures/warp_stabilizer/expected_warp_stabilizer.json`, written by \
         `tools/warp_stabilizer_reference.py` and `tools/flow_reference.py` before this code \
         existed and printed in document 25 as FX-WS-001 to 030. The build's frame is compared \
         sample by sample; the answer is the largest difference over all of them, against the \
         catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-WS-001 to 030 (document 25)");
    t.fixtures("expected_warp_stabilizer.json");

    t.heading("How far it reaches");
    let got = ws("smooth_motion", "perspective", "stabilize_only", 1000.0).bounds_expansion();
    t.row("Warp Stabilizer never grows the drawing's bounds", &got.to_string(), got == 0);
    let mut draft = as_added();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview leaves every setting as it is: the move is found at the size \
         drawn, and every setting is a share of the picture or a count of frames",
        &format!("{draft:?}"),
        draft == as_added(),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=22).map(|n| format!("fx_ws_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_ws_001.json");
    t.row("the picture it makes is never saved", &format!("{:?}", params.get("picture")), params.get("picture").is_none());
    let without = |key: &str| {
        let mut p = added("smooth_motion", "stabilize_crop_auto_scale");
        p.remove(key);
        J::Object(p).to_string()
    };
    let with = |key: &str, value: J| {
        let mut p = added("smooth_motion", "stabilize_crop_auto_scale");
        p.insert(key.to_string(), value);
        J::Object(p).to_string()
    };
    for (what, parameters) in [
        ("no `result` at all", without("result")),
        ("no `smoothness` at all", without("smoothness")),
        ("no `detailed_analysis` at all", without("detailed_analysis")),
        ("a Smoothness that is a word", with("smoothness", J::from("lots"))),
        ("a Framing that is a number", with("framing", J::from(1))),
    ] {
        t.shape_refused("fx_ws_001.json", what, &parameters);
    }

    t.heading("Commands");
    let mut document = t.load("fx_ws_001.json").document;
    refused(
        &mut t,
        &mut document,
        vec![
            ("Smoothness 1001", set(ws("smooth_motion", "position_scale_rotation", "stabilize_crop_auto_scale", 1001.0))),
            ("Method \"subspace_warp\"", set(ws("smooth_motion", "subspace_warp", "stabilize_crop_auto_scale", 50.0))),
            ("Framing \"stabilize_synthesize_edges\"", set(ws("smooth_motion", "position_scale_rotation", "stabilize_synthesize_edges", 50.0))),
            ("Maximum Scale keyed to 2000 at frame 4", keys("maximum_scale", &[(0, 150.0), (4, 2000.0)])),
        ],
    );
    t.taken(
        &mut document,
        "fx_ws_001.json",
        vec![
            ("No Motion, Perspective, Stabilize Only", set(ws("no_motion", "perspective", "stabilize_only", 50.0))),
            ("Additional Scale keyed from 100 at frame 0 to 140 at frame 4", keys("additional_scale", &[(0, 100.0), (4, 140.0)])),
        ],
    );

    t.heading("Pictures: a striped scene filmed by a shaking hand, over a night-blue solid, in `verification/D-653 pictures/`");
    let dir = effect_table::repo("verification/D-653 pictures");
    fs::create_dir_all(&dir).unwrap();
    // The drawings go beside the system's temporary files; only what the effect made is kept.
    let scenes = std::env::temp_dir().join("b532_scenes");
    fs::create_dir_all(&scenes).unwrap();
    let (w, h) = PICTURE;
    for m in 0..FRAMES {
        png_out::write_rgba(&scenes.join(format!("scene_{:02}.png", m + 1)), w, h, OutputDepth::Eight, &[], &scene(m)).unwrap();
    }
    let full = PreviewQuality::Full;
    let (_, three, said) = draw(&picture(J::Array(vec![])), &scenes, 3, full);
    png_out::write_rgba(&dir.join("1_frame_3_no_effect.png"), w, h, OutputDepth::Eight, &[], &three).unwrap();
    t.row("1_frame_3_no_effect.png, frame 3 with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());
    let (_, four, _) = draw(&picture(J::Array(vec![])), &scenes, 4, full);
    png_out::write_rgba(&dir.join("2_frame_4_no_effect.png"), w, h, OutputDepth::Eight, &[], &four).unwrap();
    let shaken = jitter(&three, &four);
    t.row("frames 3 and 4 with no effect differ by the shake", &format!("{shaken:.1} levels on average in the middle"), shaken > 0.0);
    let (_, steady3, said3) = draw(&picture(effect("smooth_motion", "stabilize_crop_auto_scale")), &scenes, 3, full);
    let (_, steady4, said4) = draw(&picture(effect("smooth_motion", "stabilize_crop_auto_scale")), &scenes, 4, full);
    png_out::write_rgba(&dir.join("3_as_added_frame_3.png"), w, h, OutputDepth::Eight, &[], &steady3).unwrap();
    png_out::write_rgba(&dir.join("4_as_added_frame_4.png"), w, h, OutputDepth::Eight, &[], &steady4).unwrap();
    t.row(
        "3_as_added_frame_3.png and 4_as_added_frame_4.png, as added: draw cleanly, enlarged so no \
         night blue shows at the edges",
        &format!("{said3:?} {said4:?}, {} and {} night-blue pixels", uncovered(&steady3), uncovered(&steady4)),
        said3.is_empty() && said4.is_empty() && uncovered(&steady3) == 0 && uncovered(&steady4) == 0,
    );
    let (_, only, said) = draw(&picture(effect("smooth_motion", "stabilize_only")), &scenes, 4, full);
    png_out::write_rgba(&dir.join("5_stabilize_only_frame_4.png"), w, h, OutputDepth::Eight, &[], &only).unwrap();
    t.row(
        "5_stabilize_only_frame_4.png, Stabilize Only: draws cleanly, the edges the shake uncovers \
         show the night blue",
        &format!("{said:?}, {} night-blue pixels", uncovered(&only)),
        said.is_empty() && uncovered(&only) > 0,
    );
    let (_, held3, _) = draw(&picture(effect("no_motion", "stabilize_only")), &scenes, 3, full);
    let (_, held4, said) = draw(&picture(effect("no_motion", "stabilize_only")), &scenes, 4, full);
    png_out::write_rgba(&dir.join("6_no_motion_frame_4.png"), w, h, OutputDepth::Eight, &[], &held4).unwrap();
    let still = jitter(&held3, &held4);
    t.row(
        "6_no_motion_frame_4.png, No Motion, Stabilize Only: draws cleanly, and frames 3 and 4 \
         differ in the middle by under a third of what the shaken frames do",
        &format!("{said:?}, {still:.1} levels against {shaken:.1}"),
        said.is_empty() && still < shaken / 3.0,
    );
    let (dw, bytes, said) = draw(&picture(effect("smooth_motion", "stabilize_crop_auto_scale")), &scenes, 4, PreviewQuality::Draft);
    png_out::write_rgba(&dir.join("7_as_added_draft.png"), dw, bytes.len() / 4 / dw, OutputDepth::Eight, &[], &bytes).unwrap();
    t.row("7_as_added_draft.png, as added at Draft, draws cleanly, a quarter the size", &format!("{said:?}, {dw} wide"), said.is_empty() && dw == w / 4);

    t.heading("Played frame after frame with the viewer's cache");
    let played = picture(effect("smooth_motion", "stabilize_crop_auto_scale"));
    let mut cache = anime_compositor::cache::CelCache::viewer();
    let warm = |cache: &mut anime_compositor::cache::CelCache, frame: i32| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(played.project(), &main_comp(), frame, &scenes, full, 64, &mut log, cache).expect("the picture draws")
    };
    let alone = |frame: i32| {
        let mut log = FrameLog::new(3);
        preview::preview_frame(played.project(), &main_comp(), frame, &scenes, full, 64, &mut log).expect("the picture draws")
    };
    const PLAYED: i32 = 8;
    let same = (0..PLAYED).filter(|&frame| warm(&mut cache, frame).data() == alone(frame).data()).count();
    t.row(
        "as added, frames 0 to 7 asked for in order through one viewer's cache, as playing asks: \
         each is byte-identical to the same frame drawn alone with no cache, as an export draws it",
        &format!("{same} of {PLAYED} byte-identical"),
        same == PLAYED as usize,
    );
    let before = warm(&mut cache, 3).data().to_vec();
    png_out::write_rgba(&scenes.join("scene_06.png"), w, h, OutputDepth::Eight, &[], &scene(0)).unwrap();
    let changed = warm(&mut cache, 3);
    t.row(
        "drawing 6 (frame 5, inside frame 3's window) rewritten as drawing 1 after it was played: \
         frame 3 through the same cache follows the new drawing, byte-identical to frame 3 drawn alone",
        &format!(
            "{}, {}",
            if changed.data() == alone(3).data() { "byte-identical" } else { "differs" },
            if changed.data() != before.as_slice() { "changed" } else { "unchanged" }
        ),
        changed.data() == alone(3).data() && changed.data() != before.as_slice(),
    );
    png_out::write_rgba(&scenes.join("scene_06.png"), w, h, OutputDepth::Eight, &[], &scene(5)).unwrap();

    t.heading("Each pair's move kept from frame to frame (audit P1)");
    use anime_compositor::compose::{forget_kept_moves, kept_moves_reused};
    let added_fx = picture(effect("smooth_motion", "stabilize_crop_auto_scale"));
    forget_kept_moves();
    draw(&added_fx, &scenes, 3, full);
    let was = kept_moves_reused();
    let (_, after, _) = draw(&added_fx, &scenes, 4, full);
    let reused = kept_moves_reused() - was;
    forget_kept_moves();
    let was = kept_moves_reused();
    let (_, cold, _) = draw(&added_fx, &scenes, 4, full);
    let cold_reused = kept_moves_reused() - was;
    t.row(
        "as added, frame 4 drawn right after frame 3 uses moves frame 3 worked, and is \
         byte-identical to frame 4 drawn with nothing kept",
        &format!(
            "{reused} moves reused after frame 3, {cold_reused} cold; {}",
            if after == cold { "byte-identical" } else { "differs" }
        ),
        reused > 0 && cold_reused == 0 && after == cold,
    );
    let mut detailed = added("smooth_motion", "stabilize_crop_auto_scale");
    detailed.insert("detailed_analysis".into(), J::from("on"));
    let detailed_fx =
        picture(serde_json::json!([{"instance_id": "fx-1", "type_id": "core.warp_stabilizer", "enabled": true, "parameters": detailed}]));
    let was = kept_moves_reused();
    let (_, changed, _) = draw(&detailed_fx, &scenes, 4, full);
    let changed_reused = kept_moves_reused() - was;
    forget_kept_moves();
    let (_, changed_cold, _) = draw(&detailed_fx, &scenes, 4, full);
    t.row(
        "Detailed Analysis switched on after frame 4 was drawn as added: no kept move is used, \
         and frame 4 is byte-identical to frame 4 with Detailed Analysis drawn with nothing kept, \
         and differs from as added",
        &format!(
            "{changed_reused} moves reused; {}; {}",
            if changed == changed_cold { "byte-identical" } else { "differs" },
            if changed != cold { "differs from as added" } else { "same as as added" }
        ),
        changed_reused == 0 && changed == changed_cold && changed != cold,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_ws_001.json", 2), ("fx_ws_003.json", 2), ("fx_ws_006.json", 2), ("fx_ws_019.json", 2)]);

    t.finish("D-653_warp_stabilizer_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-227's way: the reference shot (1920 by 1080, 24 a second) with, on each of
/// its first three layers, Warp Stabilizer and then a Noise that changes every frame, so nothing
/// is kept; every eighth frame of 240 asked for whole as the viewer asks, Draw on: GPU. The first
/// loop starts with empty caches and its 30 frames' median is "first"; the median of the seven
/// loops after it is "again"; "afresh" is every frame with empty caches, with the Warp Stabilizer
/// stage's own time. Warp Stabilizer itself is drawn by the processor.
#[test]
#[ignore = "B-532: a measurement, run deliberately with --release --ignored"]
fn b532_warp_stabilizer_timing() {
    use anime_compositor::cache::CelCache;
    use anime_compositor::compose::DEFAULT_TILE_SIZE;
    use anime_compositor::gpu::Gpu;
    use serde_json::json;
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | Quality | First | Again | Afresh | Warp Stabilizer stage, afresh |\n|---|---|---:|---:|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let shots: [(&str, Option<(&str, &str)>, PreviewQuality); 5] = [
        ("Noise alone", None, PreviewQuality::Full),
        ("Warp Stabilizer as added, then Noise", Some(("stabilize_crop_auto_scale", "off")), PreviewQuality::Full),
        ("Warp Stabilizer, Stabilize Only, Detailed Analysis on, then Noise", Some(("stabilize_only", "on")), PreviewQuality::Full),
        ("Noise alone", None, PreviewQuality::Half),
        ("Warp Stabilizer as added, then Noise", Some(("stabilize_crop_auto_scale", "off")), PreviewQuality::Half),
    ];
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    for (name, setting, quality) in shots {
        let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
        for (i, id) in ["a", "b", "c"].iter().enumerate() {
            let mut v = Vec::new();
            if let Some((framing, detailed)) = setting {
                let mut p = added("smooth_motion", framing);
                p.insert("detailed_analysis".into(), J::from(detailed));
                v.push(json!({"instance_id": format!("{id}w"), "type_id": "core.warp_stabilizer", "enabled": true, "parameters": p}));
            }
            v.push(json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}}));
            j["compositions"][0]["layers"][i]["effects"] = J::Array(v);
        }
        let project = persist::load_str(&j.to_string()).expect("the reference shot reads").document.project().clone();
        let (comp, root) = (Id::new("comp-reference-shot"), effect_table::repo("Fixtures/reference_shot"));
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..8 {
            for frame in (0..240).step_by(8) {
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                drop(
                    preview::preview_frame_srgb8(&project, &comp, frame, &root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                        .expect("GPU frame"),
                );
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                if pass > 0 {
                    times.push(ms)
                } else {
                    first.push(ms)
                }
            }
        }
        let (mut fresh, mut stage) = (Vec::new(), Vec::new());
        anime_compositor::perf::enable();
        for frame in (0..240).step_by(8) {
            let mut cache = CelCache::viewer();
            gpu.forget();
            anime_compositor::perf::reset();
            let mut log = FrameLog::new(3);
            let t = std::time::Instant::now();
            drop(
                preview::preview_frame_srgb8(&project, &comp, frame, &root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                    .expect("GPU frame"),
            );
            fresh.push(t.elapsed().as_secs_f64() * 1000.0);
            stage.push(anime_compositor::perf::read(anime_compositor::perf::Stage::EffectWarpStabilizer).0 as f64 / 1e6);
        }
        let _ = writeln!(s, "| {name} | {quality:?} | {:.1} | {:.1} | {:.1} | {:.1} |", median(first), median(times), median(fresh), median(stage));
    }
    let out = std::env::var("B532_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-532_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
