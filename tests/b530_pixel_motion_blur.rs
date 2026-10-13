//! B-530: Pixel Motion Blur in the core, against D-650.
//!
//! Writes `verification/D-650_pixel_motion_blur_table.md`.
//!
//! Every expected pixel is `Fixtures/pixel_motion_blur/expected_pixel_motion_blur.json`, written
//! by `tools/pixel_motion_blur_reference.py` (with the motion estimate of `tools/flow_reference.py`)
//! before this code existed and printed in document 25 as FX-PMB-001 to 024. Tolerance 2e-5.
//! Nothing here is a snapshot of a run.
//!
//! It also draws a ball bouncing on ones over a night-blue solid and smears it three ways into
//! `verification/D-650 pictures/`.

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

fn pmb(control: &str, angle: f64, samples: f64, detail: f64) -> Effect {
    Effect::PixelMotionBlur {
        shutter_control: control.to_string(),
        shutter_angle: angle,
        shutter_samples: samples,
        vector_detail: detail,
        frames: None,
    }
}

fn main_comp() -> Id {
    Id::new(effect_table::MAIN)
}

/// Set the holder's Pixel Motion Blur.
fn set(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: main_comp(), layer_id: Id::new("holder"), instance_id: Id::new("fx-1"), effect }
}

/// Key a setting of the holder's Pixel Motion Blur, linearly.
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
    let mut project: J =
        serde_json::from_str(&fs::read_to_string(effect_table::repo("Fixtures/pixel_motion_blur/fx_pmb_001.json")).unwrap()).unwrap();
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

fn effect(control: &str, angle: f64, samples: f64, detail: f64) -> J {
    serde_json::json!([{
        "instance_id": "fx-1", "type_id": "core.pixel_motion_blur", "enabled": true,
        "parameters": {"shutter_control": control, "shutter_angle": angle, "shutter_samples": samples, "vector_detail": detail},
    }])
}

#[test]
fn b530_pixel_motion_blur() {
    let mut t = Table::new(
        "pixel_motion_blur",
        "# B-530: Pixel Motion Blur\n\nD-650, under the owner's /loop request (P0-13, D-600: \
         \"Blur + retiming only\"). Every expected pixel is \
         `Fixtures/pixel_motion_blur/expected_pixel_motion_blur.json`, written by \
         `tools/pixel_motion_blur_reference.py` and `tools/flow_reference.py` before this code \
         existed and printed in document 25 as FX-PMB-001 to 024. The build's frame is compared \
         sample by sample; the answer is the largest difference over all of them, against the \
         catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-PMB-001 to 024 (document 25)");
    t.fixtures("expected_pixel_motion_blur.json");

    t.heading("How far it reaches");
    let got = pmb("manual", 720.0, 64.0, 100.0).bounds_expansion();
    t.row("Pixel Motion Blur never grows the drawing's bounds", &got.to_string(), got == 0);
    let mut draft = pmb("manual", 360.0, 8.0, 50.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview leaves every setting as it is, since none is a distance",
        &format!("{draft:?}"),
        draft == pmb("manual", 360.0, 8.0, 50.0),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=19).map(|n| format!("fx_pmb_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_pmb_001.json");
    t.row("the frames either side it reads are never saved", &format!("{:?}", params.get("frames")), params.get("frames").is_none());
    for (what, parameters) in [
        ("no `shutter_control` at all", r#"{"shutter_angle": 180, "shutter_samples": 16, "vector_detail": 20}"#),
        ("no `vector_detail` at all", r#"{"shutter_control": "automatic", "shutter_angle": 180, "shutter_samples": 16}"#),
        ("an angle that is a word", r#"{"shutter_control": "manual", "shutter_angle": "half", "shutter_samples": 16, "vector_detail": 20}"#),
        ("a Shutter Control that is a number", r#"{"shutter_control": 1, "shutter_angle": 180, "shutter_samples": 16, "vector_detail": 20}"#),
    ] {
        t.shape_refused("fx_pmb_001.json", what, parameters);
    }

    t.heading("Commands");
    let mut document = t.load("fx_pmb_001.json").document;
    refused(
        &mut t,
        &mut document,
        vec![
            ("Shutter Angle 721", set(pmb("manual", 721.0, 16.0, 20.0))),
            ("Shutter Samples 1", set(pmb("manual", 180.0, 1.0, 20.0))),
            ("Vector Detail -1", set(pmb("automatic", 180.0, 16.0, -1.0))),
            ("Shutter Control \"maybe\"", set(pmb("maybe", 180.0, 16.0, 20.0))),
            ("Shutter Samples keyed to 100 at frame 4", keys("shutter_samples", &[(0, 16.0), (4, 100.0)])),
        ],
    );
    t.taken(
        &mut document,
        "fx_pmb_001.json",
        vec![
            ("Manual, 360 degrees in 8 samples, Vector Detail 100", set(pmb("manual", 360.0, 8.0, 100.0))),
            ("Shutter Angle keyed from 0 at frame 0 to 720 at frame 4", keys("shutter_angle", &[(0, 0.0), (4, 720.0)])),
        ],
    );

    t.heading("Pictures: a ball bouncing on ones over a night-blue solid, in `verification/D-650 pictures/`");
    let dir = effect_table::repo("verification/D-650 pictures");
    fs::create_dir_all(&dir).unwrap();
    // The drawings go beside the system's temporary files; only what the effect made is kept.
    let balls = std::env::temp_dir().join("b530_balls");
    fs::create_dir_all(&balls).unwrap();
    let (w, h) = PICTURE;
    for m in 0..FRAMES {
        png_out::write_rgba(&balls.join(format!("ball_{:02}.png", m + 1)), w, h, OutputDepth::Eight, &[], &ball(m)).unwrap();
    }
    let full = PreviewQuality::Full;
    let (_, before, said) = draw(&picture(J::Array(vec![])), &balls, 16, full);
    png_out::write_rgba(&dir.join("1_before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    let mut last = touched(&before);
    t.row("1_before.png, frame 16 with no effect, draws cleanly", &format!("{said:?}, {last} pixels show the ball"), said.is_empty());
    for (name, effects, what) in [
        ("2_as_added", effect("automatic", 180.0, 16.0, 20.0), "as added: half a frame's travel"),
        ("3_one_frame", effect("manual", 360.0, 16.0, 20.0), "Manual 360 degrees: a whole frame's travel"),
        ("4_two_frames", effect("manual", 720.0, 32.0, 20.0), "Manual 720 degrees in 32 samples: two frames' travel"),
    ] {
        let (_, bytes, said) = draw(&picture(effects), &balls, 16, full);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let now = touched(&bytes);
        t.row(
            &format!("{name}.png, {what}, draws cleanly, the smear covering more pixels than the picture before"),
            &format!("{said:?}, {now} pixels show the ball"),
            said.is_empty() && now > last,
        );
        last = now;
    }
    let (dw, bytes, said) = draw(&picture(effect("automatic", 180.0, 16.0, 20.0)), &balls, 16, PreviewQuality::Draft);
    png_out::write_rgba(&dir.join("5_as_added_draft.png"), dw, bytes.len() / 4 / dw, OutputDepth::Eight, &[], &bytes).unwrap();
    t.row(
        "5_as_added_draft.png, as added at Draft, draws cleanly, a quarter the size",
        &format!("{said:?}, {dw} wide"),
        said.is_empty() && dw == w / 4,
    );

    t.heading("Played frame after frame with the viewer's cache");
    let played = picture(effect("automatic", 180.0, 16.0, 20.0));
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
    let before = warm(&mut cache, 15).data().to_vec();
    png_out::write_rgba(&balls.join("ball_17.png"), w, h, OutputDepth::Eight, &[], &ball(0)).unwrap();
    let changed = warm(&mut cache, 15);
    t.row(
        "drawing 17 (frame 16, the frame after 15) rewritten as drawing 1 after it was played: frame \
         15 through the same cache follows the new drawing, byte-identical to frame 15 drawn alone",
        &format!(
            "{}, {}",
            if changed.data() == alone(15).data() { "byte-identical" } else { "differs" },
            if changed.data() != before.as_slice() { "changed" } else { "unchanged" }
        ),
        changed.data() == alone(15).data() && changed.data() != before.as_slice(),
    );
    png_out::write_rgba(&balls.join("ball_17.png"), w, h, OutputDepth::Eight, &[], &ball(16)).unwrap();
    // On twos frames 2 and 3 show one drawing with different frames either side, so the frame
    // kept for one must never be handed back for the other.
    let twos = t.load("fx_pmb_018.json").document;
    let fixtures = effect_table::repo("Fixtures/pixel_motion_blur");
    let mut cache = anime_compositor::cache::CelCache::viewer();
    let same = (0..6)
        .filter(|&frame| {
            let mut log = FrameLog::new(3);
            let warm = preview::preview_frame_cached(twos.project(), &main_comp(), frame, &fixtures, full, 64, &mut log, &mut cache).unwrap();
            let mut log = FrameLog::new(3);
            warm.data() == preview::preview_frame(twos.project(), &main_comp(), frame, &fixtures, full, 64, &mut log).unwrap().data()
        })
        .count();
    t.row(
        "FX-PMB-018 (on twos), frames 0 to 5 asked for in order through one viewer's cache: each \
         byte-identical to the same frame drawn alone, though frames 2 and 3 show one drawing",
        &format!("{same} of 6 byte-identical"),
        same == 6,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_pmb_001.json", 2), ("fx_pmb_010.json", 2), ("fx_pmb_013.json", 2), ("fx_pmb_015.json", 2)]);

    t.finish("D-650_pixel_motion_blur_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-227's way: the reference shot (1920 by 1080, 24 a second) with, on each of
/// its first three layers, Pixel Motion Blur and then a Noise that changes every frame, so nothing
/// is kept; every eighth frame of 240 asked for whole as the viewer asks, Draw on: GPU. The first
/// loop starts with empty caches and its 30 frames' median is "first"; the median of the seven
/// loops after it is "again". Pixel Motion Blur itself is drawn by the processor.
#[test]
#[ignore = "B-530: a measurement, run deliberately with --release --ignored"]
fn b530_pixel_motion_blur_timing() {
    use anime_compositor::cache::CelCache;
    use anime_compositor::compose::DEFAULT_TILE_SIZE;
    use anime_compositor::gpu::Gpu;
    use serde_json::json;
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | Quality | First | Again | Afresh | Pixel Motion Blur stage, afresh |\n|---|---|---:|---:|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let added = ("automatic", 180.0, 16.0, 20.0);
    let shots: [(&str, Option<(&str, f64, f64, f64)>, PreviewQuality); 5] = [
        ("Noise alone", None, PreviewQuality::Full),
        ("Pixel Motion Blur as added (Automatic: 180 degrees, 16 samples; Vector Detail 20), then Noise", Some(added), PreviewQuality::Full),
        ("Pixel Motion Blur Manual 360 degrees, 64 samples, Vector Detail 100, then Noise", Some(("manual", 360.0, 64.0, 100.0)), PreviewQuality::Full),
        ("Noise alone", None, PreviewQuality::Half),
        ("Pixel Motion Blur as added, then Noise", Some(added), PreviewQuality::Half),
    ];
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    for (name, settings, quality) in shots {
        let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
        for (i, id) in ["a", "b", "c"].iter().enumerate() {
            let mut v = Vec::new();
            if let Some((c, a, n, d)) = settings {
                v.push(json!({"instance_id": format!("{id}p"), "type_id": "core.pixel_motion_blur", "enabled": true,
                    "parameters": {"shutter_control": c, "shutter_angle": a, "shutter_samples": n, "vector_detail": d}}));
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
        // Afresh: every frame with an empty cache, so each Pixel Motion Blur is worked, not reused.
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
            stage.push(anime_compositor::perf::read(anime_compositor::perf::Stage::EffectPixelMotionBlur).0 as f64 / 1e6);
        }
        let _ = writeln!(s, "| {name} | {quality:?} | {:.1} | {:.1} | {:.1} | {:.1} |", median(first), median(times), median(fresh), median(stage));
    }
    let out = std::env::var("B530_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-530_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
