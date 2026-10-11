//! B-401: Time Difference in the core, against D-521.
//!
//! Writes `verification/D-521_time_difference_table.md`.
//!
//! Every expected pixel is `Fixtures/time_difference/expected_time_difference.json`, written by
//! `tools/time_difference_reference.py` before this code existed and printed in document 25 as
//! FX-TDIFF-001 to 031. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a ball bouncing on ones over a night-blue solid, as B-400 does, and shows where
//! it moved three ways into `verification/D-521 pictures/`.

mod effect_table;

use std::f64::consts::PI;
use std::fs;
use std::path::Path;

use effect_table::Table;
use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectInstance, EffectKey, TDIFF_ALPHAS};
use anime_compositor::model::{Id, Interp};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

fn tdiff(layer: J, time_offset: f64, contrast: f64, absolute: &str, alpha: &str) -> Effect {
    Effect::TimeDifference {
        layer,
        time_offset,
        contrast,
        absolute_difference: absolute.to_string(),
        alpha_channel: alpha.to_string(),
        map: None,
    }
}

/// One frame back, against itself, as added otherwise.
fn back(alpha: &str) -> Effect {
    tdiff(J::from(""), -1.0 / 24.0, 0.0, "on", alpha)
}

fn main_comp() -> Id {
    Id::new(effect_table::MAIN)
}

/// Set the effect `instance` on `layer`.
fn set(layer: &str, instance: &str, effect: Effect) -> Command {
    Command::SetEffectParameters {
        composition: main_comp(),
        layer_id: Id::new(layer),
        instance_id: Id::new(instance),
        effect,
    }
}

/// Add `effect` to the end of `layer`'s stack.
fn add(layer: &str, instance: &str, effect: Effect) -> Command {
    Command::AddEffect {
        composition: main_comp(),
        layer_id: Id::new(layer),
        effect: EffectInstance::new(Id::new(instance), effect),
        index: None,
    }
}

/// Key a setting of the holder's Time Difference, linearly.
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

fn refused(t: &mut Table, document: &mut Document, commands: Vec<(&str, Command, Option<&str>)>) {
    let held = everything(document);
    for (what, command, id) in commands {
        let refused = document.apply(command).err();
        let untouched = everything(document) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| format!("{} {}", d.id.as_str(), d.message)),
            refused.as_ref().is_some_and(|d| id.is_none_or(|id| d.id.as_str() == id)) && untouched,
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

/// The ball bouncing on ones over a night-blue solid, with `effects` on the ball. The project is
/// B-400's first fixture, read and changed here, never written.
fn picture(effects: J) -> Document {
    let mut project: J =
        serde_json::from_str(&fs::read_to_string(effect_table::repo("Fixtures/cc_wide_time/fx_wide_001.json")).unwrap()).unwrap();
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
    let drawn = preview::preview_frame(document.project(), &main_comp(), frame, dir, quality, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.width(), drawn.to_srgb8_straight(), said)
}

/// The first and last column where anything but the top-left pixel's colour shows, in the first
/// `rows` rows.
fn span(bytes: &[u8], w: usize, rows: usize) -> (usize, usize) {
    let background = &bytes[0..4];
    let columns: Vec<usize> =
        (0..w * rows).filter(|&i| &bytes[i * 4..i * 4 + 4] != background).map(|i| i % w).collect();
    (*columns.iter().min().unwrap(), *columns.iter().max().unwrap())
}

fn effect(offset: f64, contrast: f64, absolute: &str) -> J {
    serde_json::json!([{
        "instance_id": "fx-1", "type_id": "core.time_difference", "enabled": true,
        "parameters": {"layer": "", "time_offset": offset, "contrast": contrast,
            "absolute_difference": absolute, "alpha_channel": "full_on"},
    }])
}

#[test]
fn b401_time_difference() {
    let mut t = Table::new(
        "time_difference",
        "# B-401: Time Difference\n\nD-521, under the owner's /loop request of 2026-10-11. Every \
         expected pixel is `Fixtures/time_difference/expected_time_difference.json`, written by \
         `tools/time_difference_reference.py` before this code existed and printed in document 25 \
         as FX-TDIFF-001 to 031. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-TDIFF-001 to 031 (document 25)");
    t.fixtures("expected_time_difference.json");

    t.heading("Files whose layers read each other (document 25)");
    let expected: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/time_difference/expected_time_difference.json")).unwrap(),
    )
    .unwrap();
    for (file, case) in expected["loads"].as_object().unwrap() {
        let says = case["says"].as_str().unwrap();
        let id = case["refused"].as_str().unwrap();
        let got = persist::load(&effect_table::repo(&format!("Fixtures/time_difference/{file}"))).err();
        t.row(
            &format!("{file}: {says}"),
            &got.as_ref().map_or("opened".to_string(), |d| format!("{} {}", d.id.as_str(), d.message)),
            got.is_some_and(|d| d.id.as_str() == id),
        );
    }

    t.heading("How far it reaches");
    let got = back("original").bounds_expansion();
    t.row("Time Difference never grows the drawing's bounds", &got.to_string(), got == 0);
    let mut draft = tdiff(J::from("plate"), -0.5, 20.0, "off", "blend");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview leaves every setting as it is, since none is a distance",
        &format!("{draft:?}"),
        draft == tdiff(J::from("plate"), -0.5, 20.0, "off", "blend"),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=31).map(|n| format!("fx_tdiff_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_tdiff_030.json");
    t.row(
        "fx_tdiff_030.json, its target written as the number 3, is saved as the number 3",
        &params["layer"].to_string(),
        params["layer"] == serde_json::json!(3),
    );
    let params = t.saved_parameters("fx_tdiff_014.json");
    t.row("the target frame the effect reads is never saved", &format!("{:?}", params.get("map")), params.get("map").is_none());
    for (what, parameters) in [
        ("no `layer` at all", r#"{"time_offset": 0, "contrast": 0, "absolute_difference": "on", "alpha_channel": "original"}"#),
        ("no `alpha_channel` at all", r#"{"layer": "", "time_offset": 0, "contrast": 0, "absolute_difference": "on"}"#),
        ("a Contrast that is a word", r#"{"layer": "", "time_offset": 0, "contrast": "high", "absolute_difference": "on", "alpha_channel": "original"}"#),
        ("an Absolute Difference that is a number", r#"{"layer": "", "time_offset": 0, "contrast": 0, "absolute_difference": 1, "alpha_channel": "original"}"#),
    ] {
        t.shape_refused("fx_tdiff_001.json", what, parameters);
    }

    t.heading("Commands");
    let mut document = t.load("fx_tdiff_014.json").document;
    refused(
        &mut t,
        &mut document,
        vec![
            ("Contrast 101", set("holder", "fx-1", tdiff(J::from("plate"), 0.0, 101.0, "on", "original")), None),
            ("Time Offset -61 seconds", set("holder", "fx-1", tdiff(J::from("plate"), -61.0, 0.0, "on", "original")), None),
            ("Absolute Difference \"maybe\"", set("holder", "fx-1", tdiff(J::from("plate"), 0.0, 0.0, "maybe", "original")), None),
            ("Alpha Channel \"lightness\"", set("holder", "fx-1", tdiff(J::from("plate"), 0.0, 0.0, "on", "lightness")), None),
            ("a target that is the number 3", set("holder", "fx-1", tdiff(J::from(3), 0.0, 0.0, "on", "original")), None),
            ("Time Offset keyed to 100 seconds at frame 4", keys("time_offset", &[(0, 0.0), (4, 100.0)]), None),
            (
                "a Time Difference on `plate` naming `holder`, whose Time Difference names `plate`",
                add("plate", "fx-2", tdiff(J::from("holder"), 0.0, 0.0, "on", "original")),
                Some("EFFECT_LAYER_CYCLE"),
            ),
        ],
    );
    let taken = [
        ("naming the holder itself, which is no circle,", set("holder", "fx-1", tdiff(J::from("holder"), -0.5, 0.0, "on", "original"))),
        ("no target named, one frame back", set("holder", "fx-1", back("original"))),
        ("Time Offset keyed from 0 at frame 0 to -1 second at frame 4", keys("time_offset", &[(0, 0.0), (4, -1.0)])),
        ("a Time Difference on `plate` naming `late`", add("plate", "fx-2", tdiff(J::from("late"), 0.0, 0.0, "on", "original"))),
    ];
    let n = taken.len();
    for (what, command) in taken {
        let taken = document.apply(command).is_ok();
        t.row(&format!("{what} is taken"), if taken { "taken" } else { "refused" }, taken);
    }
    let all_taken = TDIFF_ALPHAS.iter().all(|w| {
        let taken = document.apply(set("holder", "fx-1", tdiff(J::from("twin"), 60.0, 100.0, "off", w))).is_ok();
        document.undo();
        taken
    });
    t.row(
        "each of the nine Alpha Channel words, with the widest Time Offset and Contrast, is taken",
        &TDIFF_ALPHAS.join(", "),
        all_taken,
    );
    let before = t.render(&t.load("fx_tdiff_014.json").document, 0, 64);
    for _ in 0..n {
        document.undo();
    }
    let same = t.render(&document, 0, 64).data() == before.data();
    t.row(&format!("undo {n} times: frame 0 is the frame it was"), if same { "byte-identical" } else { "differ" }, same);

    t.heading("Pictures: a ball bouncing on ones over a night-blue solid, in `verification/D-521 pictures/`");
    let dir = effect_table::repo("verification/D-521 pictures");
    fs::create_dir_all(&dir).unwrap();
    // The drawings go beside the system's temporary files; only what the effect made is kept.
    let balls = std::env::temp_dir().join("b401_balls");
    fs::create_dir_all(&balls).unwrap();
    let (w, h) = PICTURE;
    for m in 0..FRAMES {
        png_out::write_rgba(&balls.join(format!("ball_{:02}.png", m + 1)), w, h, OutputDepth::Eight, &[], &ball(m)).unwrap();
    }
    let full = PreviewQuality::Full;
    let (_, before, said) = draw(&picture(J::Array(vec![])), &balls, 16, full);
    png_out::write_rgba(&dir.join("1_before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    let got = span(&before, w, h);
    t.row(
        "1_before.png, frame 16 with no effect, draws cleanly, the ball in columns 282 to 330",
        &format!("{said:?}, columns {} to {}", got.0, got.1),
        said.is_empty() && got.0.abs_diff(282) <= 3 && got.1.abs_diff(330) <= 3,
    );
    for (name, effects, what) in [
        ("2_moved", effect(-1.0 / 24.0, 0.0, "on"), "one frame back, Alpha Channel Full On: black, the ball's old and new places lit"),
        ("3_grey", effect(-1.0 / 24.0, 0.0, "off"), "the same with Absolute Difference off: grey, the new place lighter, the old darker"),
        ("4_strong", effect(-1.0 / 24.0, 30.0, "on"), "the same as 2_moved with Contrast 30: every difference four times as strong"),
    ] {
        let (_, bytes, said) = draw(&picture(effects), &balls, 16, full);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let got = span(&bytes, w, h);
        // Drawing 16's ball spans columns 266 to 314, drawing 17's 282 to 330.
        t.row(
            &format!("{name}.png, {what}, draws cleanly in columns 266 to 330"),
            &format!("{said:?}, columns {} to {}", got.0, got.1),
            said.is_empty() && got.0.abs_diff(266) <= 3 && got.1.abs_diff(330) <= 3,
        );
    }
    let (dw, bytes, said) = draw(&picture(effect(-1.0 / 24.0, 0.0, "on")), &balls, 16, PreviewQuality::Draft);
    png_out::write_rgba(&dir.join("5_moved_draft.png"), dw, bytes.len() / 4 / dw, OutputDepth::Eight, &[], &bytes).unwrap();
    let got = span(&bytes, dw, h / 4);
    t.row(
        "5_moved_draft.png, 2_moved at Draft, draws cleanly, a quarter the size: columns 66 to 82",
        &format!("{said:?}, {dw} wide, columns {} to {}", got.0, got.1),
        said.is_empty() && dw == w / 4 && got.0.abs_diff(66) <= 2 && got.1.abs_diff(82) <= 2,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_tdiff_002.json", 2),
        ("fx_tdiff_015.json", 4),
        ("fx_tdiff_019.json", 2),
        ("fx_tdiff_023.json", 2),
    ]);

    t.finish("D-521_time_difference_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-227's way: the reference shot (1920 by 1080, 24 a second) with, on each of
/// its first three layers, Time Difference and then a Noise that changes every frame, so nothing
/// is kept; every eighth frame of 240 asked for whole at Full as the viewer asks, Draw on: GPU.
/// The first loop starts with empty caches and its 30 frames' median is "first"; the median of
/// the seven loops after it is "again". Time Difference itself is drawn by the processor.
#[test]
#[ignore = "B-401: a measurement, run deliberately with --release --ignored"]
fn b401_time_difference_timing() {
    use anime_compositor::cache::CelCache;
    use anime_compositor::compose::DEFAULT_TILE_SIZE;
    use anime_compositor::gpu::Gpu;
    use serde_json::json;
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | Quality | First | Again |\n|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let shots: [(&str, Option<f64>); 3] = [
        ("Noise alone", None),
        ("Time Difference as added (itself, now), then Noise", Some(0.0)),
        ("Time Difference one frame back, then Noise", Some(-1.0 / 24.0)),
    ];
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    for (name, offset) in shots {
        let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
        for (i, id) in ["a", "b", "c"].iter().enumerate() {
            let mut v = Vec::new();
            if let Some(o) = offset {
                v.push(json!({"instance_id": format!("{id}t"), "type_id": "core.time_difference", "enabled": true,
                    "parameters": {"layer": "", "time_offset": o, "contrast": 0, "absolute_difference": "on", "alpha_channel": "original"}}));
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
                drop(preview::preview_frame_srgb8(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                if pass > 0 { times.push(ms) } else { first.push(ms) }
            }
        }
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B401_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-401_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
