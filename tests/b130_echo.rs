//! B-130: echo in the core, against D-195.
//!
//! Writes `verification/B-130_echo_table.md`.
//!
//! Every expected pixel is `Fixtures/echo/expected_echo.json`, written by
//! `tools/echo_reference.py` before this code existed and printed in document 25 as FX-ECHO-001
//! to 030. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a ball bouncing on ones over a night-blue solid and echoes it six ways into
//! `verification/B-130 pictures/`.

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

fn echo(echo_time: f64, echoes: f64, intensity: f64, decay: f64, operator: &str) -> Effect {
    Effect::Echo { echo_time, echoes, intensity, decay, operator: operator.to_string(), picture: None }
}

fn main_comp() -> Id {
    Id::new(effect_table::MAIN)
}

/// Set the holder's Echo.
fn set(effect: Effect) -> Command {
    Command::SetEffectParameters {
        composition: main_comp(),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        effect,
    }
}

/// Key a setting of the holder's Echo, linearly.
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
        serde_json::from_str(&fs::read_to_string(effect_table::repo("Fixtures/echo/fx_echo_001.json")).unwrap()).unwrap();
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
    let drawn = preview::preview_frame(document.project(), &main_comp(), frame, dir, quality, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.width(), drawn.to_srgb8_straight(), said)
}

/// The first and last column where anything but the background shows, in the first `rows` rows.
fn span(bytes: &[u8], w: usize, rows: usize) -> (usize, usize) {
    let background = &bytes[0..4];
    let columns: Vec<usize> =
        (0..w * rows).filter(|&i| &bytes[i * 4..i * 4 + 4] != background).map(|i| i % w).collect();
    (*columns.iter().min().unwrap(), *columns.iter().max().unwrap())
}

fn effect(echo_time: i32, echoes: i32, decay: f64, operator: &str) -> J {
    serde_json::json!([{
        "instance_id": "fx-1", "type_id": "core.echo", "enabled": true,
        "parameters": {"echo_time": echo_time, "echoes": echoes, "intensity": 1, "decay": decay, "operator": operator},
    }])
}

#[test]
fn b130_echo() {
    let mut t = Table::new(
        "echo",
        "# B-130: echo\n\nD-195, accepted by the owner on 2026-09-28 (\"take everything\"). Every \
         expected pixel is `Fixtures/echo/expected_echo.json`, written by \
         `tools/echo_reference.py` before this code existed and printed in document 25 as \
         FX-ECHO-001 to 030. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-ECHO-001 to 030 (document 25)");
    t.fixtures("expected_echo.json");

    t.heading("How far it reaches");
    let got = echo(-2.0, 6.0, 1.0, 0.7, "add").bounds_expansion();
    t.row("an echo never grows the drawing's bounds", &got.to_string(), got == 0);
    let mut draft = echo(-2.0, 6.0, 0.9, 0.7, "screen");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview leaves every setting as it is, since none is a distance",
        &format!("{draft:?}"),
        draft == echo(-2.0, 6.0, 0.9, 0.7, "screen"),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=30).filter(|&n| n != 19).map(|n| format!("fx_echo_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    // FX-ECHO-019's mask is written in the file's first form, one `mask` of corners, which every
    // file since is saved as a `masks` list; it is kept, not dropped.
    let loaded = t.load("fx_echo_019.json");
    let saved = effect_table::saved(&loaded);
    let again = persist::load_str(&saved.to_string()).expect("the saved file opens").document;
    let masks = &saved["compositions"][0]["layers"][0]["masks"];
    let same = t.render(&again, 3, 64).data() == t.render(&loaded.document, 3, 64).data();
    t.row(
        "fx_echo_019.json, its mask in the first form, is saved as one mask of the same four \
         corners in the current form, and the saved file draws frame 3 the same",
        &format!("{} mask(s), frame 3 {}", masks.as_array().map_or(0, Vec::len), if same { "byte-identical" } else { "differs" }),
        masks.as_array().is_some_and(|m| m.len() == 1 && m[0]["path"]["base"]["points"].as_array().is_some_and(|p| p.len() == 4))
            && same,
    );
    let params = t.saved_parameters("fx_echo_001.json");
    t.row(
        "the echoes it lays are never saved",
        &format!("{:?}", params.get("picture")),
        params.get("picture").is_none(),
    );
    for (what, parameters) in [
        ("no `operator` at all", r#"{"echo_time": -1, "echoes": 1, "intensity": 1, "decay": 1}"#),
        ("no `echoes` at all", r#"{"echo_time": -1, "intensity": 1, "decay": 1, "operator": "add"}"#),
        ("an echo time that is a word", r#"{"echo_time": "back", "echoes": 1, "intensity": 1, "decay": 1, "operator": "add"}"#),
        ("an operator that is a number", r#"{"echo_time": -1, "echoes": 1, "intensity": 1, "decay": 1, "operator": 3}"#),
    ] {
        t.shape_refused("fx_echo_001.json", what, parameters);
    }

    t.heading("Commands");
    let mut document = t.load("fx_echo_001.json").document;
    refused(
        &mut t,
        &mut document,
        vec![
            ("Number Of Echoes 31", set(echo(-1.0, 31.0, 1.0, 1.0, "add"))),
            ("Starting Intensity 1.5", set(echo(-1.0, 1.0, 1.5, 1.0, "add"))),
            ("Decay -0.1", set(echo(-1.0, 1.0, 1.0, -0.1, "add"))),
            ("Echo Time 121", set(echo(121.0, 1.0, 1.0, 1.0, "add"))),
            ("Echo Operator \"multiply\"", set(echo(-1.0, 1.0, 1.0, 1.0, "multiply"))),
            ("Number Of Echoes keyed to 40 at frame 4", keys("echoes", &[(0, 1.0), (4, 40.0)])),
        ],
    );
    t.taken(
        &mut document,
        "fx_echo_001.json",
        vec![
            ("Screen with three echoes two frames back", set(echo(-2.0, 3.0, 1.0, 0.5, "screen"))),
            ("Echo Time keyed from -1 at frame 0 to -3 at frame 4", keys("echo_time", &[(0, -1.0), (4, -3.0)])),
        ],
    );

    t.heading("Pictures: a ball bouncing on ones over a night-blue solid, in `verification/B-130 pictures/`");
    let dir = effect_table::repo("verification/B-130 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PICTURE;
    for m in 0..FRAMES {
        png_out::write_rgba(&dir.join(format!("ball_{:02}.png", m + 1)), w, h, OutputDepth::Eight, &[], &ball(m)).unwrap();
    }
    let full = PreviewQuality::Full;
    let (_, before, said) = draw(&picture(J::Array(vec![])), &dir, 16, full);
    png_out::write_rgba(&dir.join("before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    let got = span(&before, w, h);
    t.row(
        "before.png, frame 16 with no effect, draws cleanly, the ball in columns 282 to 330",
        &format!("{said:?}, columns {} to {}", got.0, got.1),
        said.is_empty() && got.0.abs_diff(282) <= 3 && got.1.abs_diff(330) <= 3,
    );
    let (cx, cy) = ball_at(16);
    let inside = |i: usize, r: f64| ((i % w) as f64 + 0.5 - cx).hypot((i / w) as f64 + 0.5 - cy) < r;
    let brightness = |bytes: &[u8], i: usize| bytes[i * 4..i * 4 + 3].iter().map(|&b| b as u32).sum::<u32>();
    for (name, effects, from, what) in [
        ("trail_add", effect(-2, 6, 0.7, "add"), 90, "6 echoes 2 frames back, decay 0.7, Add: a trail back to drawing 5, brighter where the balls overlap"),
        ("trail_in_back", effect(-2, 6, 0.7, "composite_in_back"), 90, "the same, Composite In Back: the present ball whole on top"),
        ("smear", effect(-1, 11, 0.8, "add"), 106, "11 echoes 1 frame back, decay 0.8, Add: a smear back to drawing 6"),
        ("ahead_screen", effect(2, 3, 0.5, "screen"), 282, "Echo Time +2, 3 echoes, decay 0.5, Screen: where the ball is going, to drawing 23"),
        ("blend", effect(-1, 3, 1.0, "blend"), 234, "3 echoes 1 frame back, Blend: four drawings averaged"),
    ] {
        let to = if name == "ahead_screen" { 426 } else { 330 };
        let (_, bytes, said) = draw(&picture(effects), &dir, 16, full);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let got = span(&bytes, w, h);
        let (check, ok) = match name {
            "trail_add" => {
                let brighter = (0..w * h).filter(|&i| inside(i, 23.0) && brightness(&bytes, i) > brightness(&before, i)).count();
                (format!(", {brighter} pixels of the present ball brighter"), brighter > 0)
            }
            "trail_in_back" => {
                let differ = (0..w * h).filter(|&i| inside(i, 23.0) && bytes[i * 4..i * 4 + 4] != before[i * 4..i * 4 + 4]).count();
                (format!(", {differ} pixels of the present ball differ from before.png"), differ == 0)
            }
            _ => (String::new(), true),
        };
        t.row(
            &format!("{name}.png, {what}, draws cleanly in columns {from} to {to}"),
            &format!("{said:?}, columns {} to {}{check}", got.0, got.1),
            said.is_empty() && got.0.abs_diff(from) <= 3 && got.1.abs_diff(to) <= 3 && ok,
        );
    }
    let (dw, bytes, said) = draw(&picture(effect(-2, 6, 0.7, "add")), &dir, 16, PreviewQuality::Draft);
    // 270 is not a whole number of quarters: the Draft frame's last row is the solid half covered.
    let got = span(&bytes, dw, h / 4);
    t.row(
        "trail_add at Draft draws cleanly, a quarter the size: the trail in columns 22 to 83 (the \
         last row, half covered by the solid since 270 is no whole number of quarters, left out)",
        &format!("{said:?}, {dw} wide, columns {} to {}", got.0, got.1),
        said.is_empty() && dw == w / 4 && got.0.abs_diff(22) <= 2 && got.1.abs_diff(83) <= 2,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_echo_004.json", 4),
        ("fx_echo_010.json", 4),
        ("fx_echo_011.json", 4),
        ("fx_echo_019.json", 3),
        ("fx_echo_020.json", 2),
    ]);

    t.finish("B-130_echo_table.md");
}
