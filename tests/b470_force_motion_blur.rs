//! B-470: CC Force Motion Blur in the core, against D-581.
//!
//! Writes `verification/D-581_force_motion_blur_table.md`.
//!
//! Every expected pixel is `Fixtures/force_motion_blur/expected_force_motion_blur.json`, written by
//! `tools/force_motion_blur_reference.py` before this code existed and printed in document 25 as
//! FX-FMB-001 to 030. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also checks that an effect setting keyed inside the holder is read at each moment, which
//! the reference does not draw, against the build's own frames with that setting fixed; and it
//! draws a square crossing a night-blue solid inside a composition layer, blurred three ways,
//! into `verification/D-581 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::Table;
use serde_json::{json, Value as J};

use anime_compositor::command::{Command, Document};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::model::{Id, Interp};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

fn fmb(samples: f64, override_native: &str, angle: f64, native: &str) -> Effect {
    Effect::ForceMotionBlur {
        motion_blur_samples: samples,
        override_native_shutter: override_native.to_string(),
        shutter_angle: angle,
        native_motion_blur: native.to_string(),
        picture: None,
    }
}

fn main_comp() -> Id {
    Id::new(effect_table::MAIN)
}

/// Set the holder's CC Force Motion Blur.
fn set(effect: Effect) -> Command {
    Command::SetEffectParameters {
        composition: main_comp(),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        effect,
    }
}

/// Key a setting of the holder's CC Force Motion Blur, linearly.
fn keys(setting: &str, values: &[(i32, f64)]) -> Command {
    Command::SetEffectKeys {
        composition: main_comp(),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        setting: setting.to_string(),
        keys: values
            .iter()
            .map(|&(frame, v)| EffectKey {
                frame,
                value: vec![v],
                interp: Interp::Linear,
            })
            .collect(),
    }
}

/// The whole composition, to see that a refused command changed nothing.
fn everything(document: &Document) -> String {
    format!(
        "{:?}",
        document.project().composition(&main_comp()).unwrap()
    )
}

fn refused(t: &mut Table, document: &mut Document, commands: Vec<(&str, Command)>) {
    let held = everything(document);
    for (what, command) in commands {
        let refused = document.apply(command).err();
        let untouched = everything(document) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused
                .as_ref()
                .map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
}

fn read(file: &str) -> J {
    serde_json::from_str(
        &fs::read_to_string(effect_table::repo(&format!(
            "Fixtures/force_motion_blur/{file}"
        )))
        .unwrap(),
    )
    .unwrap()
}

fn open(project: &J) -> Document {
    persist::load_str(&project.to_string())
        .expect("the project reads")
        .document
}

/// FX-FMB-002 with the inner bar still at 6 and an Exposure on it; `stops` is its Stops, a
/// number or keys; `holder_fx` whether the holder keeps its CC Force Motion Blur.
fn exposed(stops: J, holder_fx: bool) -> Document {
    let mut p = read("fx_fmb_002.json");
    if !holder_fx {
        p["compositions"][0]["layers"][0]["effects"] = J::Array(vec![]);
    }
    let bar = &mut p["compositions"][1]["layers"][0];
    bar["transform"]["position"] = json!({"base": [6, 0], "keyframes": []});
    bar["effects"] = json!([{"instance_id": "fx-e", "type_id": "core.exposure", "enabled": true,
        "parameters": {"stops": stops, "gamma": 1.0}}]);
    open(&p)
}

const PICTURE: (usize, usize) = (480, 270);
const FRAMES: usize = 24;

/// A composition layer 480 by 270 over a night-blue solid; inside it, a white square 48 wide
/// crossing 32 pixels a frame, its left edge at 40 on frame 0. `effects` are the holder's.
fn picture(effects: J) -> Document {
    let mut p = read("fx_fmb_002.json");
    let (w, h) = PICTURE;
    for comp in p["compositions"].as_array_mut().unwrap() {
        comp["width"] = J::from(w);
        comp["height"] = J::from(h);
        comp["duration_frames"] = J::from(FRAMES);
        comp["work_area"]["end_frame_exclusive"] = J::from(FRAMES);
        for layer in comp["layers"].as_array_mut().unwrap() {
            layer["out_frame"] = J::from(FRAMES);
        }
    }
    let bar = &mut p["compositions"][1]["layers"][0];
    bar["solid"] = json!({"color": [1.0, 1.0, 1.0], "width": 48, "height": 48});
    bar["transform"]["position"] = json!({"base": [40, 111], "keyframes": [
        {"frame": 0, "value": [40, 111], "interp": "linear"}, {"frame": 12, "value": [424, 111], "interp": "linear"}]});
    let layers = p["compositions"][0]["layers"].as_array_mut().unwrap();
    layers[0]["effects"] = effects;
    let mut night = p["compositions"][1]["layers"][0].clone();
    night["id"] = J::from("night");
    night["name"] = J::from("night");
    night["solid"] = json!({"color": [0.0070, 0.0116, 0.0452], "width": w, "height": h});
    night["transform"]["position"] = json!({"base": [0, 0], "keyframes": []});
    let layers = p["compositions"][0]["layers"].as_array_mut().unwrap();
    layers.insert(0, night);
    p["compositions"][0]["layer_order"]
        .as_array_mut()
        .unwrap()
        .insert(0, J::from("night"));
    open(&p)
}

fn effect(samples: i32, override_native: &str, angle: i32) -> J {
    json!([{
        "instance_id": "fx-1", "type_id": "core.force_motion_blur", "enabled": true,
        "parameters": {"motion_blur_samples": samples, "override_native_shutter": override_native,
            "shutter_angle": angle, "native_motion_blur": "on"},
    }])
}

/// `document`'s frame at Full, as bytes, with what it said.
fn draw(document: &Document, dir: &Path, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut log = FrameLog::new(3);
    let drawn = preview::preview_frame(
        document.project(),
        &main_comp(),
        frame,
        dir,
        PreviewQuality::Full,
        64,
        &mut log,
    )
    .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (drawn.to_srgb8_straight(), said)
}

/// The first and last column where anything but the background shows.
fn span(bytes: &[u8], w: usize) -> (usize, usize) {
    let background = &bytes[0..4];
    let columns: Vec<usize> = (0..bytes.len() / 4)
        .filter(|&i| &bytes[i * 4..i * 4 + 4] != background)
        .map(|i| i % w)
        .collect();
    (
        *columns.iter().min().unwrap(),
        *columns.iter().max().unwrap(),
    )
}

#[test]
fn b470_force_motion_blur() {
    let mut t = Table::new(
        "force_motion_blur",
        "# B-470: CC Force Motion Blur\n\nD-581 (a), accepted by the owner on 2026-10-12: \"go with \
         your recommendations on all six\". Every expected pixel is \
         `Fixtures/force_motion_blur/expected_force_motion_blur.json`, written by \
         `tools/force_motion_blur_reference.py` before this code existed and printed in document 25 \
         as FX-FMB-001 to 030. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-FMB-001 to 030 (document 25)");
    t.fixtures("expected_force_motion_blur.json");

    t.heading("How far it reaches");
    let got = fmb(64.0, "on", 720.0, "on").bounds_expansion();
    t.row(
        "CC Force Motion Blur never grows the holder's bounds",
        &got.to_string(),
        got == 0,
    );
    let mut draft = fmb(8.0, "on", 360.0, "off");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview leaves every setting as it is, since none is a distance",
        &format!("{draft:?}"),
        draft == fmb(8.0, "on", 360.0, "off"),
    );

    t.heading("An effect setting keyed inside the holder is read at each moment");
    let blurred = t.render(&exposed(json!({"base": 0, "keyframes": [
        {"frame": 0, "value": 0, "interp": "linear"}, {"frame": 2, "value": 2, "interp": "linear"}]}), true), 1, 64);
    let fixed: Vec<Vec<f32>> = [1.0, 1.125, 1.25, 1.375]
        .iter()
        .map(|&s| t.render(&exposed(J::from(s), false), 1, 64).data().to_vec())
        .collect();
    let most = blurred
        .data()
        .iter()
        .enumerate()
        .map(|(i, &v)| (v - fixed.iter().map(|f| f[i]).sum::<f32>() / 4.0).abs())
        .fold(0.0f32, f32::max);
    t.row(
        "the inner bar still, its Exposure keyed from 0 stops at frame 0 to 2 at frame 2; Override on, \
         180 degrees, 4 samples: frame 1 is the average of the bar with Stops fixed at 1, 1.125, 1.25 \
         and 1.375 and no CC Force Motion Blur (within 1e-5)",
        &format!("largest difference {most:e}"),
        most <= 1e-5,
    );

    t.heading("The file");
    let files: Vec<String> = (1..=30).map(|n| format!("fx_fmb_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_fmb_001.json");
    t.row(
        "the averaged picture it lays is never saved",
        &format!("{:?}", params.get("picture")),
        params.get("picture").is_none(),
    );
    for (what, parameters) in [
        (
            "no `native_motion_blur` at all",
            r#"{"motion_blur_samples": 8, "override_native_shutter": "off", "shutter_angle": 180}"#,
        ),
        (
            "no `motion_blur_samples` at all",
            r#"{"override_native_shutter": "off", "shutter_angle": 180, "native_motion_blur": "on"}"#,
        ),
        (
            "a sample count that is a word",
            r#"{"motion_blur_samples": "eight", "override_native_shutter": "off", "shutter_angle": 180, "native_motion_blur": "on"}"#,
        ),
        (
            "an Override Native Shutter that is a number",
            r#"{"motion_blur_samples": 8, "override_native_shutter": 1, "shutter_angle": 180, "native_motion_blur": "on"}"#,
        ),
    ] {
        t.shape_refused("fx_fmb_001.json", what, parameters);
    }

    t.heading("Commands");
    let mut document = t.load("fx_fmb_001.json").document;
    refused(
        &mut t,
        &mut document,
        vec![
            ("Motion Blur Samples 0", set(fmb(0.0, "off", 180.0, "on"))),
            ("Motion Blur Samples 65", set(fmb(65.0, "off", 180.0, "on"))),
            ("Shutter Angle 721", set(fmb(8.0, "on", 721.0, "on"))),
            (
                "Override Native Shutter \"maybe\"",
                set(fmb(8.0, "maybe", 180.0, "on")),
            ),
            (
                "Motion Blur Samples keyed to 100 at frame 4",
                keys("motion_blur_samples", &[(0, 8.0), (4, 100.0)]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_fmb_001.json",
        vec![
            (
                "16 samples, Override on at 360 degrees, Native Motion Blur off",
                set(fmb(16.0, "on", 360.0, "off")),
            ),
            (
                "Motion Blur Samples keyed from 1 at frame 0 to 8 at frame 4",
                keys("motion_blur_samples", &[(0, 1.0), (4, 8.0)]),
            ),
        ],
    );

    t.heading("Pictures: a white square crossing a night-blue solid inside a composition layer, in `verification/D-581 pictures/`");
    let dir = effect_table::repo("verification/D-581 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PICTURE;
    for (name, effects, from, to, what) in [
        (
            "1_before",
            J::Array(vec![]),
            232,
            279,
            "frame 6 with no effect: the square in columns 232 to 279",
        ),
        (
            "2_as_added",
            effect(8, "off", 180),
            232,
            279,
            "as added, the composition's motion blur off: nothing to blur, the same columns",
        ),
        (
            "3_override_180",
            effect(8, "on", 180),
            232,
            293,
            "Override on, 180 degrees, 8 samples: a smear 14 pixels ahead",
        ),
        (
            "4_override_360",
            effect(16, "on", 360),
            232,
            309,
            "Override on, 360 degrees, 16 samples: a smear 30 pixels ahead",
        ),
    ] {
        let (bytes, said) = draw(&picture(effects), &t.root, 6);
        png_out::write_rgba(
            &dir.join(format!("{name}.png")),
            w,
            h,
            OutputDepth::Eight,
            &[],
            &bytes,
        )
        .unwrap();
        let got = span(&bytes, w);
        t.row(
            &format!("{name}.png, {what}, draws cleanly in columns {from} to {to}"),
            &format!("{said:?}, columns {} to {}", got.0, got.1),
            said.is_empty() && got.0 == from && got.1 == to,
        );
    }

    t.heading("Played frame after frame with the viewer's cache (D-528)");
    for (what, played, frames) in [
        (
            "the 360-degree picture, frames 0 to 23",
            picture(effect(16, "on", 360)),
            FRAMES as i32,
        ),
        (
            "FX-FMB-013, native blur inside, frames 0 to 4",
            t.load("fx_fmb_013.json").document,
            5,
        ),
    ] {
        let mut cache = anime_compositor::cache::CelCache::viewer();
        let same = (0..frames)
            .filter(|&frame| {
                let mut log = FrameLog::new(3);
                let warm = preview::preview_frame_cached(
                    played.project(),
                    &main_comp(),
                    frame,
                    &t.root,
                    PreviewQuality::Full,
                    64,
                    &mut log,
                    &mut cache,
                )
                .expect("the frame draws");
                let mut log = FrameLog::new(3);
                let alone = preview::preview_frame(
                    played.project(),
                    &main_comp(),
                    frame,
                    &t.root,
                    PreviewQuality::Full,
                    64,
                    &mut log,
                )
                .expect("the frame draws");
                warm.data() == alone.data()
            })
            .count();
        t.row(
            &format!("{what} asked for in order through one viewer's cache, as playing asks: each is \
                      byte-identical to the same frame drawn alone with no cache, as an export draws it"),
            &format!("{same} of {frames} byte-identical; {} bytes kept for effects", cache.effect_held_bytes()),
            same == frames as usize,
        );
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_fmb_002.json", 1),
        ("fx_fmb_013.json", 1),
        ("fx_fmb_016.json", 2),
        ("fx_fmb_019.json", 2),
    ]);

    t.finish("D-581_force_motion_blur_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-227's way: the reference shot (1920 by 1080, 24 a second) precomposed into
/// one composition layer with, on it, CC Force Motion Blur and then a Noise that changes every
/// frame; every eighth frame of 240 asked for whole at Full as the viewer asks, Draw on: GPU. The
/// first loop starts with empty caches and its 30 frames' median is "first"; the median of the
/// seven loops after it is "again". Each moment is the reference shot drawn again.
#[test]
#[ignore = "B-470: a measurement, run deliberately with --release --ignored"]
fn b470_force_motion_blur_timing() {
    use anime_compositor::cache::CelCache;
    use anime_compositor::compose::DEFAULT_TILE_SIZE;
    use anime_compositor::gpu::Gpu;
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | Quality | First | Again |\n|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
    );
    let automatic = anime_compositor::cache::automatic_budget();
    let shots: [(&str, Option<(i32, i32)>, bool); 5] = [
        ("Precomposed, Noise alone", None, false),
        (
            "CC Force Motion Blur, Override 180 degrees, 8 samples, then Noise",
            Some((8, 180)),
            false,
        ),
        (
            "CC Force Motion Blur, Override 360 degrees, 16 samples, then Noise",
            Some((16, 360)),
            false,
        ),
        ("Precomposed, Noise alone, played", None, true),
        (
            "CC Force Motion Blur, 180 degrees, 8 samples, then Noise, played",
            Some((8, 180)),
            true,
        ),
    ];
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json"))
        .expect("read the reference shot");
    let holder = read("fx_fmb_002.json")["compositions"][0]["layers"][0].clone();
    for (name, blur, played) in shots {
        let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
        let mut outer = j["compositions"][0].clone();
        let mut layer = holder.clone();
        layer["composition_id"] = J::from("comp-reference-shot");
        layer["out_frame"] = outer["duration_frames"].clone();
        let mut v = Vec::new();
        if let Some((samples, angle)) = blur {
            v.push(json!({"instance_id": "fx-1", "type_id": "core.force_motion_blur", "enabled": true,
                "parameters": {"motion_blur_samples": samples, "override_native_shutter": "on", "shutter_angle": angle, "native_motion_blur": "on"}}));
        }
        v.push(
            json!({"instance_id": "fx-n", "type_id": "core.noise", "enabled": true,
            "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}}),
        );
        layer["effects"] = J::Array(v);
        outer["id"] = J::from("comp-precomposed");
        outer["name"] = J::from("comp-precomposed");
        outer["layers"] = json!([layer]);
        outer["layer_order"] = json!(["holder"]);
        j["compositions"].as_array_mut().unwrap().push(outer);
        let project = persist::load_str(&j.to_string())
            .expect("the precomposed shot reads")
            .document
            .project()
            .clone();
        let (comp, root) = (
            Id::new("comp-precomposed"),
            effect_table::repo("Fixtures/reference_shot"),
        );
        let mut cache = if played {
            CelCache::viewer_sized(automatic)
        } else {
            CelCache::viewer()
        };
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        let (passes, frames): (usize, Vec<i32>) = if played {
            (2, (0..48).collect())
        } else {
            (8, (0..240).step_by(8).collect())
        };
        for pass in 0..passes {
            for &frame in &frames {
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                drop(
                    preview::preview_frame_srgb8(
                        &project,
                        &comp,
                        frame,
                        &root,
                        PreviewQuality::Full,
                        DEFAULT_TILE_SIZE,
                        &mut log,
                        &mut cache,
                        &mut gpu,
                    )
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
        let _ = writeln!(
            s,
            "| {name} | Full | {:.1} | {:.1} |",
            median(first),
            median(times)
        );
    }
    let _ = writeln!(s, "\nPlayed rows: frames 0 to 47 in order, twice, the viewer's cache {} MB (Memory: Automatic).", automatic / 1_000_000);
    let out = std::env::var("B470_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| effect_table::repo("verification/B-470_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
