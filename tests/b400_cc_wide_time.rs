//! B-400: CC Wide Time in the core, against D-520.
//!
//! Writes `verification/D-520_cc_wide_time_table.md`.
//!
//! Every expected pixel is `Fixtures/cc_wide_time/expected_cc_wide_time.json`, written by
//! `tools/cc_wide_time_reference.py` before this code existed and printed in document 25 as
//! FX-WIDE-001 to 019. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Native Motion Blur needs a composition inside the holder, which the reference does not draw:
//! it is checked here on the build's own frames, one against another.
//!
//! It also draws a ball bouncing on ones over a night-blue solid and smears it four ways into
//! `verification/D-520 pictures/`.

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

fn wide(forward_steps: f64, backward_steps: f64, native_motion_blur: &str) -> Effect {
    Effect::CcWideTime {
        forward_steps,
        backward_steps,
        native_motion_blur: native_motion_blur.to_string(),
        picture: None,
    }
}

fn main_comp() -> Id {
    Id::new(effect_table::MAIN)
}

/// Set the holder's CC Wide Time.
fn set(effect: Effect) -> Command {
    Command::SetEffectParameters {
        composition: main_comp(),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        effect,
    }
}

/// Key a setting of the holder's CC Wide Time, linearly.
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

const PICTURE: (usize, usize) = (480, 270);
const FRAMES: usize = 24;

/// Where drawing `m` (from 0) has the ball: moving 16 pixels right a frame, bouncing.
fn ball_at(m: usize) -> (f64, f64) {
    (
        50.0 + 16.0 * m as f64,
        225.0 - 170.0 * (PI * (m as f64 + 3.0) / 12.0).cos().abs(),
    )
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
            bytes.extend([
                mix(245.0, 70.0),
                mix(150.0, 30.0),
                mix(50.0, 20.0),
                (255.0 * cover).round() as u8,
            ]);
        }
    }
    bytes
}

/// The ball bouncing on ones over a night-blue solid, with `effects` on the ball.
fn picture(effects: J) -> Document {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/cc_wide_time/fx_wide_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["pattern"] = J::from("ball_##.png");
    project["assets"][0]["frames"] = (1..=FRAMES)
        .map(|m| (m.to_string(), J::from(format!("ball_{m:02}.png"))))
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
    comp["layer_order"]
        .as_array_mut()
        .unwrap()
        .insert(0, J::from("night"));
    persist::load_str(&project.to_string())
        .expect("the picture's project reads")
        .document
}

/// `document`'s frame at `quality`, as bytes, with what it said.
fn draw(
    document: &Document,
    dir: &Path,
    frame: i32,
    quality: PreviewQuality,
) -> (usize, Vec<u8>, Vec<String>) {
    let mut log = FrameLog::new(3);
    let drawn = preview::preview_frame(
        document.project(),
        &main_comp(),
        frame,
        dir,
        quality,
        64,
        &mut log,
    )
    .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (drawn.width(), drawn.to_srgb8_straight(), said)
}

/// The first and last column where anything but the background shows, in the first `rows` rows.
fn span(bytes: &[u8], w: usize, rows: usize) -> (usize, usize) {
    let background = &bytes[0..4];
    let columns: Vec<usize> = (0..w * rows)
        .filter(|&i| &bytes[i * 4..i * 4 + 4] != background)
        .map(|i| i % w)
        .collect();
    (
        *columns.iter().min().unwrap(),
        *columns.iter().max().unwrap(),
    )
}

fn effect(forward: i32, backward: i32) -> J {
    serde_json::json!([{
        "instance_id": "fx-1", "type_id": "core.cc_wide_time", "enabled": true,
        "parameters": {"forward_steps": forward, "backward_steps": backward, "native_motion_blur": "off"},
    }])
}

/// A composition layer, still, showing a composition 20 by 1 in which a bar 4 wide runs 16
/// pixels right over frames 0 to 2 with its motion blur switch on; the inner composition's blur
/// `inner_blur`, and CC Wide Time one step each way with `native` on the composition layer.
fn nested(native: &str, inner_blur: bool) -> Document {
    let mut p: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/motion_blur/fx_mb_028.json")).unwrap(),
    )
    .unwrap();
    let blur = p["compositions"][0]["motion_blur"].clone();
    let outer = &mut p["compositions"][0]["layers"][0];
    outer["transform"]["position"] = serde_json::json!({"base": [0, 0], "keyframes": []});
    outer["motion_blur"] = J::from(false);
    outer["effects"] = serde_json::json!([{
        "instance_id": "fx-1", "type_id": "core.cc_wide_time", "enabled": true,
        "parameters": {"forward_steps": 1, "backward_steps": 1, "native_motion_blur": native},
    }]);
    p["compositions"][0]["motion_blur"]["enabled"] = J::from(false);
    let inner = &mut p["compositions"][1];
    inner["width"] = J::from(20);
    inner["motion_blur"] = blur;
    inner["motion_blur"]["enabled"] = J::from(inner_blur);
    let bar = &mut inner["layers"][0];
    bar["motion_blur"] = J::from(true);
    bar["transform"]["position"] = serde_json::json!({"base": [0, 0], "keyframes": [
        {"frame": 0, "value": [0, 0], "interp": "linear"}, {"frame": 2, "value": [16, 0], "interp": "linear"}]});
    persist::load_str(&p.to_string())
        .expect("the nested project reads")
        .document
}

#[test]
fn b400_cc_wide_time() {
    let mut t = Table::new(
        "cc_wide_time",
        "# B-400: CC Wide Time\n\nD-520, under the owner's /loop request of 2026-10-11. Every \
         expected pixel is `Fixtures/cc_wide_time/expected_cc_wide_time.json`, written by \
         `tools/cc_wide_time_reference.py` before this code existed and printed in document 25 \
         as FX-WIDE-001 to 019. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-WIDE-001 to 019 (document 25)");
    t.fixtures("expected_cc_wide_time.json");

    t.heading("How far it reaches");
    let got = wide(10.0, 10.0, "off").bounds_expansion();
    t.row(
        "CC Wide Time never grows the drawing's bounds",
        &got.to_string(),
        got == 0,
    );
    let mut draft = wide(5.0, 2.0, "on");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview leaves every setting as it is, since none is a distance",
        &format!("{draft:?}"),
        draft == wide(5.0, 2.0, "on"),
    );

    t.heading("Native Motion Blur, on a composition inside the holder");
    let frame = |d: &Document| t.render(d, 1, 64).data().to_vec();
    let off = frame(&nested("off", true));
    let on = frame(&nested("on", true));
    let unblurred = frame(&nested("on", false));
    let plain_off = frame(&nested("off", false));
    let most = off
        .iter()
        .zip(&on)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);
    t.row(
        "with the inner composition's motion blur enabled, Off and On draw frame 1 differently",
        &format!("largest difference {most}"),
        most > 0.01,
    );
    t.row(
        "Off draws frame 1 exactly as On does with the inner composition's blur switched off",
        if off == unblurred {
            "byte-identical"
        } else {
            "differ"
        },
        off == unblurred,
    );
    t.row(
        "with no blur inside, Off and On draw frame 1 the same",
        if plain_off == unblurred {
            "byte-identical"
        } else {
            "differ"
        },
        plain_off == unblurred,
    );

    t.heading("The file");
    let files: Vec<String> = (1..=19)
        .filter(|&n| n != 11)
        .map(|n| format!("fx_wide_{n:03}.json"))
        .collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    // FX-WIDE-011's mask is written in the file's first form, one `mask` of corners, which every
    // file since is saved as a `masks` list; it is kept, not dropped.
    let loaded = t.load("fx_wide_011.json");
    let saved = effect_table::saved(&loaded);
    let again = persist::load_str(&saved.to_string())
        .expect("the saved file opens")
        .document;
    let masks = &saved["compositions"][0]["layers"][0]["masks"];
    let same = t.render(&again, 3, 64).data() == t.render(&loaded.document, 3, 64).data();
    t.row(
        "fx_wide_011.json, its mask in the first form, is saved as one mask of the same four          corners in the current form, and the saved file draws frame 3 the same",
        &format!("{} mask(s), frame 3 {}", masks.as_array().map_or(0, Vec::len), if same { "byte-identical" } else { "differs" }),
        masks.as_array().is_some_and(|m| m.len() == 1 && m[0]["path"]["base"]["points"].as_array().is_some_and(|p| p.len() == 4))
            && same,
    );
    let params = t.saved_parameters("fx_wide_001.json");
    t.row(
        "the averaged frames it lays are never saved",
        &format!("{:?}", params.get("picture")),
        params.get("picture").is_none(),
    );
    for (what, parameters) in [
        (
            "no `native_motion_blur` at all",
            r#"{"forward_steps": 3, "backward_steps": 3}"#,
        ),
        (
            "no `forward_steps` at all",
            r#"{"backward_steps": 3, "native_motion_blur": "off"}"#,
        ),
        (
            "a step count that is a word",
            r#"{"forward_steps": "three", "backward_steps": 3, "native_motion_blur": "off"}"#,
        ),
        (
            "a Native Motion Blur that is a number",
            r#"{"forward_steps": 3, "backward_steps": 3, "native_motion_blur": 1}"#,
        ),
    ] {
        t.shape_refused("fx_wide_001.json", what, parameters);
    }

    t.heading("Commands");
    let mut document = t.load("fx_wide_001.json").document;
    refused(
        &mut t,
        &mut document,
        vec![
            ("Forward Steps 101", set(wide(101.0, 3.0, "off"))),
            ("Backward Steps -1", set(wide(3.0, -1.0, "off"))),
            ("Native Motion Blur \"maybe\"", set(wide(3.0, 3.0, "maybe"))),
            (
                "Backward Steps keyed to 150 at frame 4",
                keys("backward_steps", &[(0, 3.0), (4, 150.0)]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_wide_001.json",
        vec![
            (
                "Five forward, none back, Native Motion Blur on",
                set(wide(5.0, 0.0, "on")),
            ),
            (
                "Forward Steps keyed from 0 at frame 0 to 6 at frame 4",
                keys("forward_steps", &[(0, 0.0), (4, 6.0)]),
            ),
        ],
    );

    t.heading("Pictures: a ball bouncing on ones over a night-blue solid, in `verification/D-520 pictures/`");
    let dir = effect_table::repo("verification/D-520 pictures");
    fs::create_dir_all(&dir).unwrap();
    // The drawings go beside the system's temporary files; only what the effect made is kept.
    let balls = std::env::temp_dir().join("b400_balls");
    fs::create_dir_all(&balls).unwrap();
    let (w, h) = PICTURE;
    for m in 0..FRAMES {
        png_out::write_rgba(
            &balls.join(format!("ball_{:02}.png", m + 1)),
            w,
            h,
            OutputDepth::Eight,
            &[],
            &ball(m),
        )
        .unwrap();
    }
    let full = PreviewQuality::Full;
    let (_, before, said) = draw(&picture(J::Array(vec![])), &balls, 16, full);
    png_out::write_rgba(
        &dir.join("1_before.png"),
        w,
        h,
        OutputDepth::Eight,
        &[],
        &before,
    )
    .unwrap();
    let got = span(&before, w, h);
    t.row(
        "1_before.png, frame 16 with no effect, draws cleanly, the ball in columns 282 to 330",
        &format!("{said:?}, columns {} to {}", got.0, got.1),
        said.is_empty() && got.0.abs_diff(282) <= 3 && got.1.abs_diff(330) <= 3,
    );
    for (name, effects, from, to, what) in [
        (
            "2_as_added",
            effect(3, 3),
            234,
            378,
            "three steps each way, as added: drawings 14 to 20 averaged",
        ),
        (
            "3_behind",
            effect(0, 6),
            186,
            330,
            "six steps back only: a smear behind the ball, to drawing 11",
        ),
        (
            "4_ahead",
            effect(4, 0),
            282,
            394,
            "four steps forward only: a smear ahead, to drawing 21",
        ),
    ] {
        let (_, bytes, said) = draw(&picture(effects), &balls, 16, full);
        png_out::write_rgba(
            &dir.join(format!("{name}.png")),
            w,
            h,
            OutputDepth::Eight,
            &[],
            &bytes,
        )
        .unwrap();
        let got = span(&bytes, w, h);
        t.row(
            &format!("{name}.png, {what}, draws cleanly in columns {from} to {to}"),
            &format!("{said:?}, columns {} to {}", got.0, got.1),
            said.is_empty() && got.0.abs_diff(from) <= 3 && got.1.abs_diff(to) <= 3,
        );
    }
    let (dw, bytes, said) = draw(&picture(effect(3, 3)), &balls, 16, PreviewQuality::Draft);
    png_out::write_rgba(
        &dir.join("5_as_added_draft.png"),
        dw,
        bytes.len() / 4 / dw,
        OutputDepth::Eight,
        &[],
        &bytes,
    )
    .unwrap();
    // 270 is not a whole number of quarters: the Draft frame's last row is the solid half covered.
    let got = span(&bytes, dw, h / 4);
    t.row(
        "5_as_added_draft.png, as added at Draft, draws cleanly, a quarter the size: the smear in \
         columns 58 to 94 (the last row, half covered by the solid, left out)",
        &format!("{said:?}, {dw} wide, columns {} to {}", got.0, got.1),
        said.is_empty() && dw == w / 4 && got.0.abs_diff(58) <= 2 && got.1.abs_diff(94) <= 2,
    );

    t.heading("Played frame after frame with the viewer's cache (D-528)");
    let played = picture(effect(3, 3));
    let mut cache = anime_compositor::cache::CelCache::viewer();
    let warm = |cache: &mut anime_compositor::cache::CelCache, frame: i32| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(
            played.project(),
            &main_comp(),
            frame,
            &balls,
            full,
            64,
            &mut log,
            cache,
        )
        .expect("the picture draws")
    };
    let alone = |frame: i32| {
        let mut log = FrameLog::new(3);
        preview::preview_frame(
            played.project(),
            &main_comp(),
            frame,
            &balls,
            full,
            64,
            &mut log,
        )
        .expect("the picture draws")
    };
    let same = (0..FRAMES as i32)
        .filter(|&frame| warm(&mut cache, frame).data() == alone(frame).data())
        .count();
    t.row(
        "as added, frames 0 to 23 asked for in order through one viewer's cache, as playing asks: \
         each is byte-identical to the same frame drawn alone with no cache, as an export draws it",
        &format!(
            "{same} of {FRAMES} byte-identical; {} bytes kept for effects",
            cache.effect_held_bytes()
        ),
        same == FRAMES,
    );
    let before = warm(&mut cache, 15).data().to_vec();
    png_out::write_rgba(
        &balls.join("ball_17.png"),
        w,
        h,
        OutputDepth::Eight,
        &[],
        &ball(0),
    )
    .unwrap();
    let changed = warm(&mut cache, 15);
    t.row(
        "drawing 17 (inside frame 15's span, 12 to 18) rewritten as drawing 1 after it was played: \
         frame 15 through the same cache shows the new drawing, byte-identical to frame 15 drawn alone",
        &format!("{}, {}", if changed.data() == alone(15).data() { "byte-identical" } else { "differs" }, if changed.data() != before.as_slice() { "changed" } else { "unchanged" }),
        changed.data() == alone(15).data() && changed.data() != before.as_slice(),
    );
    png_out::write_rgba(
        &balls.join("ball_17.png"),
        w,
        h,
        OutputDepth::Eight,
        &[],
        &ball(16),
    )
    .unwrap();

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_wide_001.json", 3),
        ("fx_wide_006.json", 2),
        ("fx_wide_011.json", 3),
        ("fx_wide_012.json", 3),
    ]);

    t.finish("D-520_cc_wide_time_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-227's way: the reference shot (1920 by 1080, 24 a second) with, on each of
/// its first three layers, CC Wide Time and then a Noise that changes every frame, so nothing is
/// kept; every eighth frame of 240 asked for whole at Full as the viewer asks, Draw on: GPU. The
/// first loop starts with empty caches and its 30 frames' median is "first"; the median of the
/// seven loops after it is "again". CC Wide Time itself is drawn by the processor, as Echo is.
#[test]
#[ignore = "B-400: a measurement, run deliberately with --release --ignored"]
fn b400_cc_wide_time_timing() {
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
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
    );
    // D-528: the last two rows ask for frames 0 to 47 in order, as playing does, through a cache
    // the size the window gives the viewer with Memory set to Automatic; first is the first play's
    // median, again the second's.
    let automatic = anime_compositor::cache::automatic_budget();
    let shots: [(&str, Option<(i32, i32)>, bool); 5] = [
        ("Noise alone", None, false),
        (
            "CC Wide Time as added (3 and 3, 7 frames), then Noise",
            Some((3, 3)),
            false,
        ),
        (
            "CC Wide Time 10 and 10 (21 frames), then Noise",
            Some((10, 10)),
            false,
        ),
        ("Noise alone, played", None, true),
        (
            "CC Wide Time as added, then Noise, played",
            Some((3, 3)),
            true,
        ),
    ];
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json"))
        .expect("read the reference shot");
    for (name, steps, played) in shots {
        let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
        for (i, id) in ["a", "b", "c"].iter().enumerate() {
            let mut v = Vec::new();
            if let Some((f, b)) = steps {
                v.push(json!({"instance_id": format!("{id}w"), "type_id": "core.cc_wide_time", "enabled": true,
                    "parameters": {"forward_steps": f, "backward_steps": b, "native_motion_blur": "off"}}));
            }
            v.push(
                json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}}),
            );
            j["compositions"][0]["layers"][i]["effects"] = J::Array(v);
        }
        let project = persist::load_str(&j.to_string())
            .expect("the reference shot reads")
            .document
            .project()
            .clone();
        let (comp, root) = (
            Id::new("comp-reference-shot"),
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
    let out = std::env::var("B400_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| effect_table::repo("verification/B-400_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
