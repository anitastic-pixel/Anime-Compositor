//! B-533: Rolling Shutter Repair in the core, against D-654.
//!
//! Writes `verification/D-654_rolling_shutter_repair_table.md`.
//!
//! Every expected pixel is `Fixtures/rolling_shutter_repair/expected_rolling_shutter_repair.json`,
//! written by `tools/rolling_shutter_repair_reference.py` (with the motion estimate of
//! `tools/flow_reference.py` and Warp Stabilizer's fit of `tools/warp_stabilizer_reference.py`)
//! before this code existed and printed in document 25 as FX-RSR-001 to 026. Tolerance 2e-5.
//! Nothing here is a snapshot of a run.
//!
//! It also draws a striped scene filmed by a panning phone camera, its stripes leaning, over a
//! night-blue solid, repaired four ways, into `verification/D-654 pictures/`.

mod effect_table;

use std::f64::consts::PI;
use std::fs;
use std::path::Path;

use effect_table::{time_passes, Table};
use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::model::{Id, Interp};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// Rolling Shutter Repair with its rate, scan direction and method, the rest as added.
fn rsr(rate: f64, scan: &str, method: &str) -> Effect {
    Effect::RollingShutterRepair {
        rolling_shutter_rate: rate,
        scan_direction: scan.to_string(),
        method: method.to_string(),
        detailed_analysis: "off".to_string(),
        pixel_motion_detail: 20.0,
        picture: None,
    }
}

fn as_added() -> Effect {
    rsr(50.0, "top_to_bottom", "warp")
}

fn main_comp() -> Id {
    Id::new(effect_table::MAIN)
}

/// Set the holder's Rolling Shutter Repair.
fn set(effect: Effect) -> Command {
    Command::SetEffectParameters {
        composition: main_comp(),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        effect,
    }
}

/// Key a setting of the holder's Rolling Shutter Repair, linearly.
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
const FRAMES: usize = 12;
/// The pan, pixels a frame to the right, and the camera's scan, the part of a frame it takes to
/// read the picture top to bottom.
const PAN: f64 = 6.0;
const CAMERA: f64 = 0.8;

/// Drawing `m` (from 0): an upright-striped, opaque scene panning right, each row seen a little
/// later than the row above, so the stripes lean.
fn scene(m: usize) -> Vec<u8> {
    let (w, h) = PICTURE;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        let t = m as f64 + CAMERA * ((y as f64 + 0.5) / h as f64 - 0.5);
        for x in 0..w {
            let (u, v) = (x as f64 + 0.5 - PAN * t, y as f64 + 0.5);
            let s = 0.5
                + 0.35 * (2.0 * PI * u / 41.0).sin()
                + 0.1 * (2.0 * PI * v / 37.0 + u / 23.0).cos();
            let c = |a: f64| (a.clamp(0.0, 255.0)).round() as u8;
            bytes.extend([c(255.0 * s), c(80.0 + 120.0 * s), c(220.0 - 140.0 * s), 255]);
        }
    }
    bytes
}

/// The scene over a night-blue solid, with `effects` on the scene.
fn picture(effects: J) -> Document {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo(
            "Fixtures/rolling_shutter_repair/fx_rsr_001.json",
        ))
        .unwrap(),
    )
    .unwrap();
    project["assets"][0]["pattern"] = J::from("pan_##.png");
    project["assets"][0]["frames"] = (1..=FRAMES)
        .map(|m| (m.to_string(), J::from(format!("pan_{m:02}.png"))))
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

/// How far the stripes lean, in pixels: the best shift, over the middle half's columns, between
/// the green of rows an eighth to three eighths down and of rows five to seven eighths down,
/// sub-pixel by a parabola. The tools' `slant`, at this size.
fn lean(bytes: &[u8]) -> f64 {
    let (w, h) = PICTURE;
    let band = |y0: usize, y1: usize| -> Vec<f64> {
        (0..w)
            .map(|x| {
                (y0..y1)
                    .map(|y| bytes[(y * w + x) * 4 + 1] as f64)
                    .sum::<f64>()
                    / (y1 - y0) as f64
            })
            .collect()
    };
    let (top, bottom) = (band(h / 8, 3 * h / 8), band(5 * h / 8, 7 * h / 8));
    let score = |s: i32| {
        (w / 4..3 * w / 4)
            .map(|x| (top[x] - bottom[(x as i32 + s) as usize]).powi(2))
            .sum::<f64>()
    };
    let best = (-12..=12)
        .min_by(|&a, &b| score(a).total_cmp(&score(b)))
        .unwrap();
    let (a, b, c) = (score(best - 1), score(best), score(best + 1));
    if a - 2.0 * b + c > 0.0 {
        best as f64 + (a - c) / (2.0 * (a - 2.0 * b + c))
    } else {
        best as f64
    }
}

/// Rolling Shutter Repair's saved parameters as added.
fn added() -> serde_json::Map<String, J> {
    serde_json::json!({
        "rolling_shutter_rate": 50, "scan_direction": "top_to_bottom", "method": "warp",
        "detailed_analysis": "off", "pixel_motion_detail": 20,
    })
    .as_object()
    .unwrap()
    .clone()
}

fn effect(rate: f64, scan: &str, method: &str) -> J {
    let mut p = added();
    p.insert("rolling_shutter_rate".into(), J::from(rate));
    p.insert("scan_direction".into(), J::from(scan));
    p.insert("method".into(), J::from(method));
    serde_json::json!([{"instance_id": "fx-1", "type_id": "core.rolling_shutter_repair", "enabled": true, "parameters": p}])
}

#[test]
fn b533_rolling_shutter_repair() {
    let mut t = Table::new(
        "rolling_shutter_repair",
        "# B-533: Rolling Shutter Repair\n\nD-654, under the owner's /loop request (P0-13, D-600: \
         \"Blur + retiming only\"). Every expected pixel is \
         `Fixtures/rolling_shutter_repair/expected_rolling_shutter_repair.json`, written by \
         `tools/rolling_shutter_repair_reference.py`, `tools/warp_stabilizer_reference.py` and \
         `tools/flow_reference.py` before this code existed and printed in document 25 as \
         FX-RSR-001 to 026. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-RSR-001 to 026 (document 25)");
    t.fixtures("expected_rolling_shutter_repair.json");

    t.heading("How far it reaches");
    let got = rsr(100.0, "left_to_right", "pixel_motion").bounds_expansion();
    t.row(
        "Rolling Shutter Repair never grows the drawing's bounds",
        &got.to_string(),
        got == 0,
    );
    let mut draft = as_added();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview leaves every setting as it is: the motion is found at the size \
         drawn, the rate is a share of a frame and the detail a share of the estimate's",
        &format!("{draft:?}"),
        draft == as_added(),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=19).map(|n| format!("fx_rsr_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_rsr_001.json");
    t.row(
        "the picture it makes is never saved",
        &format!("{:?}", params.get("picture")),
        params.get("picture").is_none(),
    );
    let without = |key: &str| {
        let mut p = added();
        p.remove(key);
        J::Object(p).to_string()
    };
    let with = |key: &str, value: J| {
        let mut p = added();
        p.insert(key.to_string(), value);
        J::Object(p).to_string()
    };
    for (what, parameters) in [
        (
            "no `rolling_shutter_rate` at all",
            without("rolling_shutter_rate"),
        ),
        ("no `method` at all", without("method")),
        (
            "no `pixel_motion_detail` at all",
            without("pixel_motion_detail"),
        ),
        (
            "a Rolling Shutter Rate that is a word",
            with("rolling_shutter_rate", J::from("fast")),
        ),
        (
            "a Scan Direction that is a number",
            with("scan_direction", J::from(1)),
        ),
    ] {
        t.shape_refused("fx_rsr_001.json", what, &parameters);
    }

    t.heading("Commands");
    let mut document = t.load("fx_rsr_001.json").document;
    refused(
        &mut t,
        &mut document,
        vec![
            (
                "Rolling Shutter Rate 101",
                set(rsr(101.0, "top_to_bottom", "warp")),
            ),
            (
                "Scan Direction \"diagonal\"",
                set(rsr(50.0, "diagonal", "warp")),
            ),
            (
                "Method \"optical\"",
                set(rsr(50.0, "top_to_bottom", "optical")),
            ),
            (
                "Pixel Motion Detail keyed to 150 at frame 4",
                keys("pixel_motion_detail", &[(0, 20.0), (4, 150.0)]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_rsr_001.json",
        vec![
            (
                "Pixel Motion, Left to Right, rate 80",
                set(rsr(80.0, "left_to_right", "pixel_motion")),
            ),
            (
                "Rolling Shutter Rate keyed from 0 at frame 0 to 100 at frame 4",
                keys("rolling_shutter_rate", &[(0, 0.0), (4, 100.0)]),
            ),
        ],
    );

    t.heading("Pictures: a striped scene filmed by a panning phone camera, over a night-blue solid, in `verification/D-654 pictures/`");
    let dir = effect_table::repo("verification/D-654 pictures");
    fs::create_dir_all(&dir).unwrap();
    // The drawings go beside the system's temporary files; only what the effect made is kept.
    let pans = std::env::temp_dir().join("b533_pans");
    fs::create_dir_all(&pans).unwrap();
    let (w, h) = PICTURE;
    for m in 0..FRAMES {
        png_out::write_rgba(
            &pans.join(format!("pan_{:02}.png", m + 1)),
            w,
            h,
            OutputDepth::Eight,
            &[],
            &scene(m),
        )
        .unwrap();
    }
    let full = PreviewQuality::Full;
    let (_, plain, said) = draw(&picture(J::Array(vec![])), &pans, 3, full);
    png_out::write_rgba(
        &dir.join("1_frame_3_no_effect.png"),
        w,
        h,
        OutputDepth::Eight,
        &[],
        &plain,
    )
    .unwrap();
    let leaning = lean(&plain);
    t.row(
        "1_frame_3_no_effect.png, frame 3 with no effect: draws cleanly, the stripes leaning by about \
         the pan times the scan between the bands (6 x 0.8 / 2 = 2.4 pixels)",
        &format!("{said:?}, {leaning:.2} pixels"),
        said.is_empty() && (leaning - PAN * CAMERA / 2.0).abs() < 0.5,
    );
    let shots = [
        (
            "2_as_added.png",
            "as added, 50 per cent: the lean lessened",
            effect(50.0, "top_to_bottom", "warp"),
            1.0,
        ),
        (
            "3_rate_80.png",
            "rate 80, the camera's own: the stripes upright, the lean under a third",
            effect(80.0, "top_to_bottom", "warp"),
            1.0 / 3.0,
        ),
        (
            "4_pixel_motion_80.png",
            "Pixel Motion at 80: the lean under a half",
            effect(80.0, "top_to_bottom", "pixel_motion"),
            0.5,
        ),
    ];
    for (file, what, fx, under) in shots {
        let (_, bytes, said) = draw(&picture(fx), &pans, 3, full);
        png_out::write_rgba(&dir.join(file), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let l = lean(&bytes);
        t.row(
            &format!("{file}, {what}; draws cleanly"),
            &format!("{said:?}, {l:.2} pixels"),
            said.is_empty() && l.abs() < leaning.abs() * under,
        );
    }
    let (_, worse, said) = draw(
        &picture(effect(80.0, "bottom_to_top", "warp")),
        &pans,
        3,
        full,
    );
    png_out::write_rgba(
        &dir.join("5_bottom_to_top_80.png"),
        w,
        h,
        OutputDepth::Eight,
        &[],
        &worse,
    )
    .unwrap();
    let l = lean(&worse);
    t.row(
        "5_bottom_to_top_80.png, the wrong Scan Direction: draws cleanly, the lean made worse, over \
         one and a half times",
        &format!("{said:?}, {l:.2} pixels"),
        said.is_empty() && l.abs() > leaning.abs() * 1.5,
    );
    let (dw, bytes, said) = draw(
        &picture(effect(50.0, "top_to_bottom", "warp")),
        &pans,
        3,
        PreviewQuality::Draft,
    );
    png_out::write_rgba(
        &dir.join("6_as_added_draft.png"),
        dw,
        bytes.len() / 4 / dw,
        OutputDepth::Eight,
        &[],
        &bytes,
    )
    .unwrap();
    t.row(
        "6_as_added_draft.png, as added at Draft, draws cleanly, a quarter the size",
        &format!("{said:?}, {dw} wide"),
        said.is_empty() && dw == w / 4,
    );

    t.heading("Played frame after frame with the viewer's cache");
    let played = picture(effect(50.0, "top_to_bottom", "warp"));
    let mut cache = anime_compositor::cache::CelCache::viewer();
    let warm = |cache: &mut anime_compositor::cache::CelCache, frame: i32| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(
            played.project(),
            &main_comp(),
            frame,
            &pans,
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
            &pans,
            full,
            64,
            &mut log,
        )
        .expect("the picture draws")
    };
    const PLAYED: i32 = 8;
    let same = (0..PLAYED)
        .filter(|&frame| warm(&mut cache, frame).data() == alone(frame).data())
        .count();
    t.row(
        "as added, frames 0 to 7 asked for in order through one viewer's cache, as playing asks: \
         each is byte-identical to the same frame drawn alone with no cache, as an export draws it",
        &format!("{same} of {PLAYED} byte-identical"),
        same == PLAYED as usize,
    );
    let before = warm(&mut cache, 3).data().to_vec();
    png_out::write_rgba(
        &pans.join("pan_05.png"),
        w,
        h,
        OutputDepth::Eight,
        &[],
        &scene(0),
    )
    .unwrap();
    let changed = warm(&mut cache, 3);
    t.row(
        "drawing 5 (frame 4, the frame after frame 3) rewritten as drawing 1 after it was played: \
         frame 3 through the same cache follows the new drawing, byte-identical to frame 3 drawn alone",
        &format!(
            "{}, {}",
            if changed.data() == alone(3).data() { "byte-identical" } else { "differs" },
            if changed.data() != before.as_slice() { "changed" } else { "unchanged" }
        ),
        changed.data() == alone(3).data() && changed.data() != before.as_slice(),
    );
    png_out::write_rgba(
        &pans.join("pan_05.png"),
        w,
        h,
        OutputDepth::Eight,
        &[],
        &scene(4),
    )
    .unwrap();

    t.heading("Warp's moves kept for a frame shown again (audit P5)");
    use anime_compositor::compose::{forget_kept_moves, kept_moves_reused};
    let added_fx = picture(effect(50.0, "top_to_bottom", "warp"));
    forget_kept_moves();
    let was = kept_moves_reused();
    let (_, cold, _) = draw(&added_fx, &pans, 3, full);
    let cold_reused = kept_moves_reused() - was;
    let was = kept_moves_reused();
    let (_, again, _) = draw(&added_fx, &pans, 3, full);
    let reused = kept_moves_reused() - was;
    t.row(
        "as added, frame 3 drawn a second time uses both moves the first drawing worked, and is \
         byte-identical to the first, drawn with nothing kept",
        &format!(
            "{reused} moves reused the second time, {cold_reused} the first; {}",
            if again == cold {
                "byte-identical"
            } else {
                "differs"
            }
        ),
        reused == 2 && cold_reused == 0 && again == cold,
    );
    let mut detailed = added();
    detailed.insert("detailed_analysis".into(), J::from("on"));
    let detailed_fx = picture(
        serde_json::json!([{"instance_id": "fx-1", "type_id": "core.rolling_shutter_repair", "enabled": true, "parameters": detailed}]),
    );
    let was = kept_moves_reused();
    let (_, changed, _) = draw(&detailed_fx, &pans, 3, full);
    let changed_reused = kept_moves_reused() - was;
    forget_kept_moves();
    let (_, changed_cold, _) = draw(&detailed_fx, &pans, 3, full);
    t.row(
        "Detailed Analysis switched on after frame 3 was drawn as added: no kept move is used, \
         and frame 3 is byte-identical to frame 3 with Detailed Analysis drawn with nothing kept",
        &format!(
            "{changed_reused} moves reused; {}; {}",
            if changed == changed_cold {
                "byte-identical"
            } else {
                "differs"
            },
            if changed != cold {
                "differs from as added"
            } else {
                "same as as added"
            }
        ),
        changed_reused == 0 && changed == changed_cold,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_rsr_001.json", 2),
        ("fx_rsr_008.json", 2),
        ("fx_rsr_013.json", 2),
        ("fx_rsr_019.json", 2),
    ]);

    t.finish("D-654_rolling_shutter_repair_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-227's way: the reference shot (1920 by 1080, 24 a second) with, on each of
/// its first three layers, Rolling Shutter Repair and then a Noise that changes every frame, so
/// nothing is kept; every eighth frame of 240 asked for whole as the viewer asks, Draw on: GPU.
/// The first loop starts with empty caches and its 30 frames' median is "first"; the median of the
/// seven loops after it is "again"; "afresh" is every frame with empty caches, with the Rolling
/// Shutter Repair stage's own time. Rolling Shutter Repair itself is drawn by the processor.
#[test]
#[ignore = "B-533: a measurement, run deliberately with --release --ignored"]
fn b533_rolling_shutter_repair_timing() {
    use anime_compositor::cache::CelCache;
    use anime_compositor::compose::DEFAULT_TILE_SIZE;
    use anime_compositor::gpu::Gpu;
    use serde_json::json;
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | Quality | First | Again | Afresh | Rolling Shutter Repair stage, afresh |\n|---|---|---:|---:|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let shots: [(&str, Option<&str>, PreviewQuality); 5] = [
        ("Noise alone", None, PreviewQuality::Full),
        (
            "Rolling Shutter Repair as added (Warp), then Noise",
            Some("warp"),
            PreviewQuality::Full,
        ),
        (
            "Rolling Shutter Repair, Pixel Motion, then Noise",
            Some("pixel_motion"),
            PreviewQuality::Full,
        ),
        ("Noise alone", None, PreviewQuality::Half),
        (
            "Rolling Shutter Repair as added (Warp), then Noise",
            Some("warp"),
            PreviewQuality::Half,
        ),
    ];
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json"))
        .expect("read the reference shot");
    for (name, method, quality) in shots {
        let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
        for (i, id) in ["a", "b", "c"].iter().enumerate() {
            let mut v = Vec::new();
            if let Some(method) = method {
                let mut p = added();
                p.insert("method".into(), J::from(method));
                v.push(json!({"instance_id": format!("{id}r"), "type_id": "core.rolling_shutter_repair", "enabled": true, "parameters": p}));
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
        let (first, times) = time_passes(&mut gpu, false, 8, &project, &comp, &root, quality);
        let (mut fresh, mut stage) = (Vec::new(), Vec::new());
        anime_compositor::perf::enable();
        for frame in (0..240).step_by(8) {
            let mut cache = CelCache::viewer();
            gpu.forget();
            anime_compositor::perf::reset();
            let mut log = FrameLog::new(3);
            let t = std::time::Instant::now();
            drop(
                preview::preview_frame_srgb8(
                    &project,
                    &comp,
                    frame,
                    &root,
                    quality,
                    DEFAULT_TILE_SIZE,
                    &mut log,
                    &mut cache,
                    &mut gpu,
                )
                .expect("GPU frame"),
            );
            fresh.push(t.elapsed().as_secs_f64() * 1000.0);
            stage.push(
                anime_compositor::perf::read(
                    anime_compositor::perf::Stage::EffectRollingShutterRepair,
                )
                .0 as f64
                    / 1e6,
            );
        }
        let _ = writeln!(
            s,
            "| {name} | {quality:?} | {:.1} | {:.1} | {:.1} | {:.1} |",
            median(first),
            median(times),
            median(fresh),
            median(stage)
        );
    }
    let out = std::env::var("B533_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| effect_table::repo("verification/B-533_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
