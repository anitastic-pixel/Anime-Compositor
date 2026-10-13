//! B-531: Timewarp in the core, against D-652.
//!
//! Writes `verification/D-652_timewarp_table.md`.
//!
//! Every expected pixel is `Fixtures/timewarp/expected_timewarp.json`, written by
//! `tools/timewarp_reference.py` (with the motion estimate of `tools/flow_reference.py`) before
//! this code existed and printed in document 25 as FX-TW-001 to 033. Tolerance 2e-5. Nothing here
//! is a snapshot of a run.
//!
//! It also draws a ball bouncing on ones over a night-blue solid, slowed to half speed three
//! ways, into `verification/D-652 pictures/`.

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

/// Timewarp as added, with its method, speed and error threshold set.
fn tw(method: &str, speed: f64, error_threshold: f64) -> Effect {
    Effect::Timewarp {
        method: method.to_string(),
        adjust_time_by: "speed".to_string(),
        speed,
        source_frame: 0.0,
        vector_detail: 20.0,
        global_smoothness: 0.5,
        local_smoothness: 0.5,
        smoothing_iterations: 4.0,
        build_from_one_image: "off".to_string(),
        correct_luminance_changes: "off".to_string(),
        filtering: "normal".to_string(),
        error_threshold,
        block_size: 6.0,
        red_weight: 0.3,
        green_weight: 0.6,
        blue_weight: 0.1,
        enable_motion_blur: "off".to_string(),
        shutter_control: "automatic".to_string(),
        shutter_angle: 180.0,
        shutter_samples: 5.0,
        picture: None,
    }
}

fn main_comp() -> Id {
    Id::new(effect_table::MAIN)
}

/// Set the holder's Timewarp.
fn set(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: main_comp(), layer_id: Id::new("holder"), instance_id: Id::new("fx-1"), effect }
}

/// Key a setting of the holder's Timewarp, linearly.
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

/// Where drawing `m` (from 0) has the ball: moving 16 pixels right a frame, bouncing.
fn ball_at(m: usize) -> (f64, f64) {
    (50.0 + 16.0 * m as f64, 225.0 - 170.0 * (PI * (m as f64 + 3.0) / 12.0).cos().abs())
}

/// Drawing `m`: an orange ball, radius 24, with an ink ring its outer 5 pixels, on nothing.
fn ball(m: usize) -> Vec<u8> {
    let (w, h) = PICTURE;
    let (cx, cy) = ball_at(m);
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let d = (x as f64 + 0.5 - cx).hypot(y as f64 + 0.5 - cy);
            let cover = (24.0 - d).clamp(0.0, 1.0);
            let ink = (d - 19.0).clamp(0.0, 1.0);
            let mix = |a: f64, b: f64| (a * (1.0 - ink) + b * ink).round() as u8;
            bytes.extend([mix(245.0, 70.0), mix(150.0, 30.0), mix(50.0, 20.0), (255.0 * cover).round() as u8]);
        }
    }
    bytes
}

/// The ball bouncing on ones over a night-blue solid, with `effects` on the ball.
fn picture(effects: J) -> Document {
    let mut project: J = serde_json::from_str(&fs::read_to_string(effect_table::repo("Fixtures/timewarp/fx_tw_001.json")).unwrap()).unwrap();
    project["assets"][0]["pattern"] = J::from("ball_##.png");
    project["assets"][0]["frames"] =
        (1..=FRAMES).map(|m| (m.to_string(), J::from(format!("ball_{m:02}.png")))).collect::<serde_json::Map<_, _>>().into();
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

/// How many pixels show anything but the background.
fn touched(bytes: &[u8]) -> usize {
    let background = &bytes[0..4];
    bytes.chunks(4).filter(|p| *p != background).count()
}

/// Timewarp's saved parameters as added, with `method` and `speed`.
fn added(method: &str, speed: f64) -> serde_json::Map<String, J> {
    serde_json::json!({
        "method": method, "adjust_time_by": "speed", "speed": speed, "source_frame": 0,
        "vector_detail": 20, "global_smoothness": 0.5, "local_smoothness": 0.5, "smoothing_iterations": 4,
        "build_from_one_image": "off", "correct_luminance_changes": "off", "filtering": "normal",
        "error_threshold": 1, "block_size": 6, "red_weight": 0.3, "green_weight": 0.6, "blue_weight": 0.1,
        "enable_motion_blur": "off", "shutter_control": "automatic", "shutter_angle": 180, "shutter_samples": 5,
    })
    .as_object()
    .unwrap()
    .clone()
}

fn effect(method: &str, speed: f64) -> J {
    serde_json::json!([{"instance_id": "fx-1", "type_id": "core.timewarp", "enabled": true, "parameters": added(method, speed)}])
}

#[test]
fn b531_timewarp() {
    let mut t = Table::new(
        "timewarp",
        "# B-531: Timewarp\n\nD-652, under the owner's /loop request (P0-13, D-600: \
         \"Blur + retiming only\"). Every expected pixel is \
         `Fixtures/timewarp/expected_timewarp.json`, written by `tools/timewarp_reference.py` \
         and `tools/flow_reference.py` before this code existed and printed in document 25 as \
         FX-TW-001 to 033. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-TW-001 to 033 (document 25)");
    t.fixtures("expected_timewarp.json");

    t.heading("How far it reaches");
    let got = tw("pixel_motion", 1000.0, 10.0).bounds_expansion();
    t.row("Timewarp never grows the drawing's bounds", &got.to_string(), got == 0);
    let mut draft = tw("pixel_motion", 50.0, 1.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview leaves every setting as it is: Block Size is counted at each \
         size the motion is found at, as Vector Detail is",
        &format!("{draft:?}"),
        draft == tw("pixel_motion", 50.0, 1.0),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=26).map(|n| format!("fx_tw_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_tw_001.json");
    t.row("the picture it makes is never saved", &format!("{:?}", params.get("picture")), params.get("picture").is_none());
    let without = |key: &str| {
        let mut p = added("pixel_motion", 50.0);
        p.remove(key);
        J::Object(p).to_string()
    };
    let with = |key: &str, value: J| {
        let mut p = added("pixel_motion", 50.0);
        p.insert(key.to_string(), value);
        J::Object(p).to_string()
    };
    for (what, parameters) in [
        ("no `method` at all", without("method")),
        ("no `speed` at all", without("speed")),
        ("no `blue_weight` at all", without("blue_weight")),
        ("a Speed that is a word", with("speed", J::from("fast"))),
        ("a Filtering that is a number", with("filtering", J::from(1))),
    ] {
        t.shape_refused("fx_tw_001.json", what, &parameters);
    }

    t.heading("Commands");
    let mut document = t.load("fx_tw_001.json").document;
    refused(
        &mut t,
        &mut document,
        vec![
            ("Speed 1001", set(tw("pixel_motion", 1001.0, 1.0))),
            ("Error Threshold -1", set(tw("pixel_motion", 50.0, -1.0))),
            ("Method \"optical\"", set(tw("optical", 50.0, 1.0))),
            ("Shutter Samples keyed to 100 at frame 4", keys("shutter_samples", &[(0, 5.0), (4, 100.0)])),
        ],
    );
    t.taken(
        &mut document,
        "fx_tw_001.json",
        vec![
            ("Frame Mix at Speed 200, Error Threshold 10", set(tw("frame_mix", 200.0, 10.0))),
            ("Speed keyed from 100 at frame 0 to -100 at frame 4", keys("speed", &[(0, 100.0), (4, -100.0)])),
        ],
    );

    t.heading("Pictures: a ball bouncing on ones over a night-blue solid, at half speed, in `verification/D-652 pictures/`");
    let dir = effect_table::repo("verification/D-652 pictures");
    fs::create_dir_all(&dir).unwrap();
    // The drawings go beside the system's temporary files; only what the effect made is kept.
    let balls = std::env::temp_dir().join("b531_balls");
    fs::create_dir_all(&balls).unwrap();
    let (w, h) = PICTURE;
    for m in 0..FRAMES {
        png_out::write_rgba(&balls.join(format!("ball_{:02}.png", m + 1)), w, h, OutputDepth::Eight, &[], &ball(m)).unwrap();
    }
    let full = PreviewQuality::Full;
    // At Speed 50 frame 17 shows source time 8.25, a quarter of the way from drawing 9 (frame 8)
    // to drawing 10 (frame 9).
    let (_, eight, said) = draw(&picture(J::Array(vec![])), &balls, 8, full);
    png_out::write_rgba(&dir.join("1_frame_8_no_effect.png"), w, h, OutputDepth::Eight, &[], &eight).unwrap();
    let one = touched(&eight);
    t.row("1_frame_8_no_effect.png, frame 8 with no effect, draws cleanly", &format!("{said:?}, {one} pixels show the ball"), said.is_empty());
    let (_, nine, _) = draw(&picture(J::Array(vec![])), &balls, 9, full);
    png_out::write_rgba(&dir.join("2_frame_9_no_effect.png"), w, h, OutputDepth::Eight, &[], &nine).unwrap();
    let (_, whole, said) = draw(&picture(effect("whole_frames", 50.0)), &balls, 17, full);
    png_out::write_rgba(&dir.join("3_whole_frames.png"), w, h, OutputDepth::Eight, &[], &whole).unwrap();
    t.row(
        "3_whole_frames.png, Whole Frames at Speed 50, frame 17: draws cleanly, byte-identical to frame 8 with no effect",
        &format!("{said:?}, {}", if whole == eight { "byte-identical" } else { "differs" }),
        said.is_empty() && whole == eight,
    );
    let (_, mix, said) = draw(&picture(effect("frame_mix", 50.0)), &balls, 17, full);
    png_out::write_rgba(&dir.join("4_frame_mix.png"), w, h, OutputDepth::Eight, &[], &mix).unwrap();
    let two = touched(&mix);
    t.row(
        "4_frame_mix.png, Frame Mix at Speed 50, frame 17: draws cleanly, two faint balls covering more pixels than one",
        &format!("{said:?}, {two} pixels show a ball"),
        said.is_empty() && two > one,
    );
    let (_, moved, said) = draw(&picture(effect("pixel_motion", 50.0)), &balls, 17, full);
    png_out::write_rgba(&dir.join("5_pixel_motion.png"), w, h, OutputDepth::Eight, &[], &moved).unwrap();
    let now = touched(&moved);
    t.row(
        "5_pixel_motion.png, as added (Pixel Motion at Speed 50), frame 17: draws cleanly, one ball \
         between the two, covering fewer pixels than Frame Mix's two",
        &format!("{said:?}, {now} pixels show the ball"),
        said.is_empty() && now < two,
    );
    let (dw, bytes, said) = draw(&picture(effect("pixel_motion", 50.0)), &balls, 17, PreviewQuality::Draft);
    png_out::write_rgba(&dir.join("6_pixel_motion_draft.png"), dw, bytes.len() / 4 / dw, OutputDepth::Eight, &[], &bytes).unwrap();
    t.row("6_pixel_motion_draft.png, as added at Draft, draws cleanly, a quarter the size", &format!("{said:?}, {dw} wide"), said.is_empty() && dw == w / 4);

    t.heading("Played frame after frame with the viewer's cache");
    let played = picture(effect("pixel_motion", 50.0));
    let mut cache = anime_compositor::cache::CelCache::viewer();
    let warm = |cache: &mut anime_compositor::cache::CelCache, frame: i32| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(played.project(), &main_comp(), frame, &balls, full, 64, &mut log, cache).expect("the picture draws")
    };
    let alone = |frame: i32| {
        let mut log = FrameLog::new(3);
        preview::preview_frame(played.project(), &main_comp(), frame, &balls, full, 64, &mut log).expect("the picture draws")
    };
    let same = (0..FRAMES as i32).filter(|&frame| warm(&mut cache, frame).data() == alone(frame).data()).count();
    t.row(
        "as added, frames 0 to 23 asked for in order through one viewer's cache, as playing asks: \
         each is byte-identical to the same frame drawn alone with no cache, as an export draws it",
        &format!("{same} of {FRAMES} byte-identical"),
        same == FRAMES,
    );
    let before = warm(&mut cache, 17).data().to_vec();
    png_out::write_rgba(&balls.join("ball_10.png"), w, h, OutputDepth::Eight, &[], &ball(0)).unwrap();
    let changed = warm(&mut cache, 17);
    t.row(
        "drawing 10 (source frame 9, which frame 17 is made from) rewritten as drawing 1 after it \
         was played: frame 17 through the same cache follows the new drawing, byte-identical to \
         frame 17 drawn alone",
        &format!(
            "{}, {}",
            if changed.data() == alone(17).data() { "byte-identical" } else { "differs" },
            if changed.data() != before.as_slice() { "changed" } else { "unchanged" }
        ),
        changed.data() == alone(17).data() && changed.data() != before.as_slice(),
    );
    png_out::write_rgba(&balls.join("ball_10.png"), w, h, OutputDepth::Eight, &[], &ball(9)).unwrap();

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_tw_001.json", 2), ("fx_tw_012.json", 2), ("fx_tw_016.json", 1), ("fx_tw_019.json", 2)]);

    t.finish("D-652_timewarp_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-227's way: the reference shot (1920 by 1080, 24 a second) with, on each of
/// its first three layers, Timewarp and then a Noise that changes every frame, so nothing is
/// kept; every eighth frame of 240 asked for whole as the viewer asks, Draw on: GPU. The first
/// loop starts with empty caches and its 30 frames' median is "first"; the median of the seven
/// loops after it is "again"; "afresh" is every frame with empty caches, with the Timewarp stage's
/// own time. Timewarp itself is drawn by the processor.
#[test]
#[ignore = "B-531: a measurement, run deliberately with --release --ignored"]
fn b531_timewarp_timing() {
    use anime_compositor::cache::CelCache;
    use anime_compositor::compose::DEFAULT_TILE_SIZE;
    use anime_compositor::gpu::Gpu;
    use serde_json::json;
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | Quality | First | Again | Afresh | Timewarp stage, afresh |\n|---|---|---:|---:|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let shots: [(&str, Option<&str>, PreviewQuality); 6] = [
        ("Noise alone", None, PreviewQuality::Full),
        ("Timewarp Whole Frames at Speed 50, then Noise", Some("whole_frames"), PreviewQuality::Full),
        ("Timewarp Frame Mix at Speed 50, then Noise", Some("frame_mix"), PreviewQuality::Full),
        ("Timewarp as added (Pixel Motion at Speed 50), then Noise", Some("pixel_motion"), PreviewQuality::Full),
        ("Noise alone", None, PreviewQuality::Half),
        ("Timewarp as added, then Noise", Some("pixel_motion"), PreviewQuality::Half),
    ];
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    for (name, method, quality) in shots {
        let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
        for (i, id) in ["a", "b", "c"].iter().enumerate() {
            let mut v = Vec::new();
            if let Some(method) = method {
                v.push(json!({"instance_id": format!("{id}t"), "type_id": "core.timewarp", "enabled": true, "parameters": added(method, 50.0)}));
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
            stage.push(anime_compositor::perf::read(anime_compositor::perf::Stage::EffectTimewarp).0 as f64 / 1e6);
        }
        let _ = writeln!(s, "| {name} | {quality:?} | {:.1} | {:.1} | {:.1} | {:.1} |", median(first), median(times), median(fresh), median(stage));
    }
    let out = std::env::var("B531_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-531_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
