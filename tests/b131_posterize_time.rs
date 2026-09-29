//! B-131: posterize time in the core, against D-196.
//!
//! Writes `verification/B-131_posterize_time_table.md`.
//!
//! Every expected pixel is `Fixtures/posterize_time/expected_posterize_time.json`, written by
//! `tools/posterize_time_reference.py` before this code existed and printed in document 25 as
//! FX-PTIME-001 to 020. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a ball bouncing on ones over a night-blue solid, on ones, twos and threes, into
//! `verification/B-131 pictures/`.

mod effect_table;

use std::f64::consts::PI;
use std::fs;
use std::path::Path;

use effect_table::Table;
use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::plan_frame;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::model::{Id, Interp};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

fn main_comp() -> Id {
    Id::new(effect_table::MAIN)
}

fn rate(frame_rate: f64) -> Effect {
    Effect::PosterizeTime { frame_rate }
}

/// Set the holder's Posterize Time.
fn set(effect: Effect) -> Command {
    Command::SetEffectParameters {
        composition: main_comp(),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        effect,
    }
}

/// Key the holder's frame rate, linearly.
fn keys(values: &[(i32, f64)]) -> Command {
    Command::SetEffectKeys {
        composition: main_comp(),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        setting: "frame_rate".to_string(),
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

/// The ball bouncing on ones over a night-blue solid, with `effects` on the ball; drawing
/// `missing`, when there is one, names a file that is not there.
fn picture(effects: J, missing: Option<usize>) -> Document {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/posterize_time/fx_ptime_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["pattern"] = J::from("ball_##.png");
    project["assets"][0]["frames"] = (1..=FRAMES)
        .map(|m| {
            let file = if Some(m) == missing { "gone.png".to_string() } else { format!("ball_{m:02}.png") };
            (m.to_string(), J::from(file))
        })
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

fn posterize(frame_rate: f64) -> J {
    serde_json::json!([{
        "instance_id": "fx-1", "type_id": "core.posterize_time", "enabled": true,
        "parameters": {"frame_rate": frame_rate},
    }])
}

/// `bytes`, `w` wide, halved each way by averaging each two by two.
fn half(bytes: &[u8], w: usize) -> Vec<u8> {
    let h = bytes.len() / 4 / w;
    let mut out = Vec::with_capacity(bytes.len() / 4);
    for y in 0..h / 2 {
        for x in 0..w / 2 {
            for c in 0..4 {
                let sum: u32 = [(0, 0), (1, 0), (0, 1), (1, 1)]
                    .iter()
                    .map(|(dx, dy)| bytes[((2 * y + dy) * w + 2 * x + dx) * 4 + c] as u32)
                    .sum();
                out.push(((sum + 2) / 4) as u8);
            }
        }
    }
    out
}

#[test]
fn b131_posterize_time() {
    let mut t = Table::new(
        "posterize_time",
        "# B-131: posterize time\n\nD-196, accepted by the owner on 2026-09-28 (\"take everything\"). \
         Every expected pixel is `Fixtures/posterize_time/expected_posterize_time.json`, written by \
         `tools/posterize_time_reference.py` before this code existed and printed in document 25 as \
         FX-PTIME-001 to 020. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-PTIME-001 to 020 (document 25)");
    t.fixtures("expected_posterize_time.json");

    t.heading("How far it reaches");
    let got = rate(8.0).bounds_expansion();
    t.row("holding never grows the drawing's bounds", &got.to_string(), got == 0);
    let mut draft = rate(8.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview leaves the frame rate as it is, since it is not a distance",
        &format!("{draft:?}"),
        draft == rate(8.0),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=20).map(|n| format!("fx_ptime_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    for (what, parameters) in [
        ("no `frame_rate` at all", r#"{}"#),
        ("a frame rate that is a word", r#"{"frame_rate": "twos"}"#),
    ] {
        t.shape_refused("fx_ptime_001.json", what, parameters);
    }

    t.heading("Commands");
    let mut document = t.load("fx_ptime_001.json").document;
    refused(
        &mut t,
        &mut document,
        vec![
            ("Frame Rate 0.05", set(rate(0.05))),
            ("Frame Rate 100", set(rate(100.0))),
            ("Frame Rate keyed to 120 at frame 4", keys(&[(0, 12.0), (4, 120.0)])),
        ],
    );
    t.taken(
        &mut document,
        "fx_ptime_001.json",
        vec![
            ("Frame Rate 8", set(rate(8.0))),
            ("Frame Rate keyed from 24 at frame 0 to 8 at frame 4", keys(&[(0, 24.0), (4, 8.0)])),
        ],
    );

    t.heading("Pictures: a ball bouncing on ones over a night-blue solid, in `verification/B-131 pictures/`");
    let dir = effect_table::repo("verification/B-131 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PICTURE;
    for m in 0..FRAMES {
        png_out::write_rgba(&dir.join(format!("ball_{:02}.png", m + 1)), w, h, OutputDepth::Eight, &[], &ball(m)).unwrap();
    }
    let full = PreviewQuality::Full;
    let shots = |effects: J| -> Vec<(Vec<u8>, Vec<String>)> {
        let document = picture(effects, None);
        (12..20).map(|n| { let (_, b, s) = draw(&document, &dir, n, full); (b, s) }).collect()
    };
    let ones = shots(J::Array(vec![]));
    let twos = shots(posterize(12.0));
    let threes = shots(posterize(8.0));
    // The strip: frames 12 to 19 left to right, on ones, at 12 and at 8 top to bottom, each at
    // half size, 4 white pixels between them.
    let (sw, sh, gap) = (w / 2, h / 2, 4);
    let (across, down) = (8 * (sw + gap) - gap, 3 * (sh + gap) - gap);
    let mut strip = vec![255u8; across * down * 4];
    for (row, frames) in [&ones, &twos, &threes].iter().enumerate() {
        for (i, (bytes, _)) in frames.iter().enumerate() {
            let small = half(bytes, w);
            for y in 0..sh {
                let to = ((row * (sh + gap) + y) * across + i * (sw + gap)) * 4;
                strip[to..to + sw * 4].copy_from_slice(&small[y * sw * 4..(y + 1) * sw * 4]);
            }
        }
    }
    png_out::write_rgba(&dir.join("strip.png"), across, down, OutputDepth::Eight, &[], &strip).unwrap();
    let clean = [&ones, &twos, &threes].iter().all(|f| f.iter().all(|(_, s)| s.is_empty()));
    t.row(
        "strip.png, frames 12 to 19 left to right: on ones with no effect, then Posterize Time 12, \
         then 8, top to bottom; every frame draws cleanly",
        &format!("{} frames, nothing said: {clean}", ones.len() * 3),
        clean,
    );
    let shows = |frames: &Vec<(Vec<u8>, Vec<String>)>| -> Vec<i32> {
        frames.iter().map(|(b, _)| ones.iter().position(|(o, _)| o == b).map_or(-1, |i| i as i32 + 12)).collect()
    };
    for (what, frames, want) in [
        ("at 12, frames 12 to 19 show frames 12, 12, 14, 14, 16, 16, 18 and 18 byte for byte", &twos, vec![12, 12, 14, 14, 16, 16, 18, 18]),
        ("at 8, frames 12 to 19 show frames 12, 12, 12, 15, 15, 15, 18 and 18 byte for byte", &threes, vec![12, 12, 12, 15, 15, 15, 18, 18]),
    ] {
        let got = shows(frames);
        t.row(what, &format!("{got:?}"), got == want);
    }
    let dw = |document: &Document, n| draw(document, &dir, n, PreviewQuality::Draft);
    let d = picture(posterize(12.0), None);
    let (a, b, c) = (dw(&d, 12), dw(&d, 13), dw(&d, 14));
    t.row(
        "at Draft, 12 a second: frame 13 is frame 12 byte for byte, and frame 14 is not",
        &format!("{} wide; 13 {} 12, 14 {} 12", a.0, if b.1 == a.1 { "=" } else { "≠" }, if c.1 == a.1 { "=" } else { "≠" }),
        a.2.is_empty() && b.2.is_empty() && b.1 == a.1 && c.1 != a.1,
    );
    // Animated grain after it steps with the drawing.
    let grain = serde_json::json!({"instance_id": "fx-2", "type_id": "core.noise", "enabled": true,
        "parameters": {"amount": 30, "mode": "mono", "seed": 0, "animate": "on"}});
    let mut effects = posterize(12.0);
    effects.as_array_mut().unwrap().push(grain.clone());
    let d = picture(effects, None);
    let frames: Vec<Vec<u8>> = (12..15).map(|n| draw(&d, &dir, n, full).1).collect();
    let alone = picture(J::Array(vec![grain]), None);
    let moving = draw(&alone, &dir, 13, full).1 != draw(&alone, &dir, 12, full).1;
    t.row(
        "animated Noise after it holds with the drawing: frame 13 is frame 12 byte for byte and \
         frame 14 is not, where the Noise alone changes every frame",
        &format!(
            "13 {} 12, 14 {} 12, alone 13 {} 12",
            if frames[1] == frames[0] { "=" } else { "≠" },
            if frames[2] == frames[0] { "=" } else { "≠" },
            if moving { "≠" } else { "=" }
        ),
        frames[1] == frames[0] && frames[2] != frames[0] && moving,
    );
    // What holding a missing drawing says belongs to the frame drawn, so an export blocks it.
    let d = picture(posterize(12.0), Some(13));
    let mut log = FrameLog::new(8);
    let _ = plan_frame(d.project(), &main_comp(), 13, &dir, &mut log);
    let at13 = log.ids_at(13);
    let at12 = log.ids_at(12);
    t.row(
        "with drawing 13's file gone, frame 13, which holds frame 12 and so drawing 13, reports \
         the missing file against frame 13, not frame 12",
        &format!("frame 13 {:?}, frame 12 {:?}", at13.iter().map(|i| i.as_str()).collect::<Vec<_>>(), at12.iter().map(|i| i.as_str()).collect::<Vec<_>>()),
        at13 == vec![DiagnosticId::MediaMissing] && at12.is_empty(),
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_ptime_001.json", 3), ("fx_ptime_008.json", 3), ("fx_ptime_013.json", 3), ("fx_ptime_016.json", 3)]);

    t.finish("B-131_posterize_time_table.md");
}
