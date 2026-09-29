//! B-135: Radio Waves in the core, against D-200.
//!
//! Writes `verification/B-135_radio_waves_table.md`.
//!
//! Every expected pixel is `Fixtures/radio_waves/expected_radio_waves.json`, written by
//! `tools/radio_waves_reference.py` before this code existed and printed in document 25 as
//! FX-RWAVE-001 to 027. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a badge, empty round it, and sends rings out over it six ways into
//! `verification/B-135 pictures/`, three times enlarged, over a dark blue where it is clear.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, set, Table};
use serde_json::Value as J;

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

/// The producer point; `[sides, interval, expansion, orientation, direction, velocity, spin,
/// lifespan, opacity, fade in, fade out, start width, end width]`; and profile and colour.
fn waves(producer_point: [f64; 2], n: [f64; 13], words: [&str; 2]) -> Effect {
    Effect::RadioWaves {
        producer_point,
        sides: n[0],
        interval: n[1],
        expansion: n[2],
        orientation: n[3],
        direction: n[4],
        velocity: n[5],
        spin: n[6],
        lifespan: n[7],
        opacity: n[8],
        fade_in_time: n[9],
        fade_out_time: n[10],
        start_width: n[11],
        end_width: n[12],
        profile: words[0].to_string(),
        color: words[1].to_string(),
        frame: 0,
    }
}

const MIDDLE: [f64; 2] = [50.0, 50.0];
const ADDED: [f64; 13] = [64.0, 24.0, 5.0, 0.0, 90.0, 0.0, 0.0, 96.0, 100.0, 0.0, 48.0, 5.0, 5.0];
const WORDS: [&str; 2] = ["square", "#ffffff"];

fn with(i: usize, v: f64) -> Effect {
    let mut n = ADDED;
    n[i] = v;
    waves(MIDDLE, n, WORDS)
}

const PLATE: (usize, usize) = (160, 100);
const BLUE: [u8; 3] = [58, 111, 216];
const RED: [u8; 3] = [200, 40, 40];
const GOLD: [u8; 3] = [224, 168, 48];
const LINE: [u8; 3] = [30, 26, 36];
const BACK: [f64; 3] = [32.0, 40.0, 56.0];

/// Light Sweep's badge: a blue plate with a dark border three pixels wide, a red bar across and
/// a gold disc in the middle; nothing round it.
fn plate() -> Vec<u8> {
    let mut bytes = Vec::new();
    for y in 0..PLATE.1 {
        for x in 0..PLATE.0 {
            let (fx, fy) = (x as f64 + 0.5, y as f64 + 0.5);
            let c = if !(40..120).contains(&x) || !(15..85).contains(&y) {
                None
            } else if x < 43 || x >= 117 || y < 18 || y >= 82 {
                Some(LINE)
            } else if (fx - 80.0).hypot(fy - 50.0) < 12.0 {
                Some(GOLD)
            } else if (44..56).contains(&y) {
                Some(RED)
            } else {
                Some(BLUE)
            };
            bytes.extend(c.map_or([0; 4], |c| [c[0], c[1], c[2], 255]));
        }
    }
    bytes
}

/// The plate as the composition's one layer, with `effects`, drawn at `frame`, straight 8-bit;
/// and the same enlarged three times over a dark blue where it is clear.
fn picture(dir: &Path, effects: J, frame: i32) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/radio_waves/fx_rwave_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("plate.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(PLATE.0);
    comp["height"] = J::from(PLATE.1);
    comp["duration_frames"] = J::from(120);
    comp["work_area"]["end_frame_exclusive"] = J::from(120);
    let layer = &mut comp["layers"][0];
    layer["out_frame"] = J::from(120);
    let middle = serde_json::json!([PLATE.0 as f64 / 2.0, PLATE.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), frame, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    let bytes = drawn.to_srgb8_straight();
    let big: Vec<u8> = (0..PLATE.1 * 3)
        .flat_map(|y| (0..PLATE.0 * 3).map(move |x| (x, y)))
        .flat_map(|(x, y)| {
            let i = (y / 3 * PLATE.0 + x / 3) * 4;
            let a = bytes[i + 3] as f64 / 255.0;
            let over = |c: u8, b: f64| (c as f64 * a + b * (1.0 - a)).round() as u8;
            [over(bytes[i], BACK[0]), over(bytes[i + 1], BACK[1]), over(bytes[i + 2], BACK[2]), 255]
        })
        .collect();
    (bytes, big, said)
}

#[test]
fn b135_radio_waves() {
    let mut t = Table::new(
        "radio_waves",
        "# B-135: Radio Waves\n\nD-200, accepted on 2026-09-28 with the After Effects picks (A10). \
         Every expected pixel is `Fixtures/radio_waves/expected_radio_waves.json`, written by \
         `tools/radio_waves_reference.py` before this code existed and printed in document 25 as \
         FX-RWAVE-001 to 027. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-RWAVE-001 to 027 (document 25)");
    t.fixtures("expected_radio_waves.json");

    t.heading("How far it reaches");
    for (what, e) in [
        ("as added, it never grows the drawing's bounds", waves(MIDDLE, ADDED, WORDS)),
        ("expansion 1000, the most, does not grow them either", with(2, 1000.0)),
        ("end width 1000, the most, does not grow them either", with(12, 1000.0)),
    ] {
        let got = e.bounds_expansion();
        t.row(what, &got.to_string(), got == 0);
    }
    let mut draft = waves(MIDDLE, [64.0, 24.0, 5.0, 0.0, 90.0, 3.0, 2.0, 96.0, 100.0, 0.0, 48.0, 5.0, 8.0], WORDS);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the expansion, 5 to 2.5, the velocity, 3 to 1.5, and the \
         widths, 5 to 2.5 and 8 to 4, and keeps the rest",
        &format!("{draft:?}"),
        draft == waves(MIDDLE, [64.0, 24.0, 2.5, 0.0, 90.0, 1.5, 2.0, 96.0, 100.0, 0.0, 48.0, 2.5, 4.0], WORDS),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_rwave_001.json",
        "fx_rwave_004.json",
        "fx_rwave_005.json",
        "fx_rwave_010.json",
        "fx_rwave_011.json",
        "fx_rwave_013.json",
        "fx_rwave_014.json",
        "fx_rwave_015.json",
        "fx_rwave_016.json",
        "fx_rwave_019.json",
        "fx_rwave_020.json",
        "fx_rwave_021.json",
        "fx_rwave_024.json",
        "fx_rwave_026.json",
        "fx_rwave_027.json",
    ]);
    let params = t.saved_parameters("fx_rwave_001.json");
    t.row(
        "the frame the waves are drawn at is never saved",
        &format!("{:?}", params.get("frame")),
        params.get("frame").is_none(),
    );
    t.shape_refused(
        "fx_rwave_001.json",
        "no `color` at all",
        r##"{"producer_point": [50, 50], "sides": 64, "interval": 24, "expansion": 5, "orientation": 0,
            "direction": 90, "velocity": 0, "spin": 0, "lifespan": 96, "opacity": 100, "fade_in_time": 0,
            "fade_out_time": 48, "start_width": 5, "end_width": 5, "profile": "square"}"##,
    );
    t.shape_refused(
        "fx_rwave_001.json",
        "a producer point of one number",
        r##"{"producer_point": [50], "sides": 64, "interval": 24, "expansion": 5, "orientation": 0,
            "direction": 90, "velocity": 0, "spin": 0, "lifespan": 96, "opacity": 100, "fade_in_time": 0,
            "fade_out_time": 48, "start_width": 5, "end_width": 5, "profile": "square", "color": "#ffffff"}"##,
    );
    t.shape_refused(
        "fx_rwave_001.json",
        "sides that are a word",
        r##"{"producer_point": [50, 50], "sides": "round", "interval": 24, "expansion": 5, "orientation": 0,
            "direction": 90, "velocity": 0, "spin": 0, "lifespan": 96, "opacity": 100, "fade_in_time": 0,
            "fade_out_time": 48, "start_width": 5, "end_width": 5, "profile": "square", "color": "#ffffff"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_rwave_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("producer x 1000.5", set(waves([1000.5, 50.0], ADDED, WORDS))),
            ("sides 2.9", set(with(0, 2.9))),
            ("sides 64.5", set(with(0, 64.5))),
            ("interval 0.9", set(with(1, 0.9))),
            ("expansion -1", set(with(2, -1.0))),
            ("orientation 3600.5", set(with(3, 3600.5))),
            ("direction -3601", set(with(4, -3601.0))),
            ("velocity 1000.5", set(with(5, 1000.5))),
            ("spin 361", set(with(6, 361.0))),
            ("lifespan 0.5", set(with(7, 0.5))),
            ("opacity 101", set(with(8, 101.0))),
            ("fade-in time -1", set(with(9, -1.0))),
            ("fade-out time 1000.5", set(with(10, 1000.5))),
            ("start width -1", set(with(11, -1.0))),
            ("end width 1000.5", set(with(12, 1000.5))),
            ("profile \"bell\"", set(waves(MIDDLE, ADDED, ["bell", "#ffffff"]))),
            ("colour \"#fff\"", set(waves(MIDDLE, ADDED, ["square", "#fff"]))),
            ("sides keyed to 70", keys("sides", &[(0, &[64.0]), (4, &[70.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_rwave_001.json",
        vec![
            (
                "every number at its top, the last profile,",
                set(waves(
                    [1000.0, 1000.0],
                    [64.0, 1000.0, 1000.0, 3600.0, 3600.0, 1000.0, 360.0, 1000.0, 100.0, 1000.0, 1000.0, 1000.0, 1000.0],
                    ["sine", "#000000"],
                )),
            ),
            (
                "every number at its bottom, the other profile,",
                set(waves(
                    [-1000.0, -1000.0],
                    [3.0, 1.0, 0.0, -3600.0, -3600.0, 0.0, -360.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                    ["triangle", "#ffb040"],
                )),
            ),
            ("the producer point keyed across the layer", keys("producer_point", &[(0, &[0.0, 50.0]), (4, &[100.0, 50.0])])),
            ("the spin keyed from 0 to 90", keys("spin", &[(0, &[0.0]), (4, &[90.0])])),
        ],
    );

    t.heading("Pictures: a badge, in `verification/B-135 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-135 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let drawn = plate();
    png_out::write_rgba(&dir.join("plate.png"), w, h, OutputDepth::Eight, &[], &drawn).unwrap();
    let (_, big, said) = picture(&dir, J::Array(vec![]), 0);
    png_out::write_rgba(&dir.join("before.png"), w * 3, h * 3, OutputDepth::Eight, &[], &big).unwrap();
    t.row("before.png, the badge with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());
    let px = |bytes: &[u8], (x, y): (usize, usize)| {
        let i = (y * w + x) * 4;
        [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
    };
    let (middle, corner) = ((80, 50), (5, 5));
    for (name, what, frame, point, n, words, lit, kept) in [
        (
            "as_added_frame_8",
            "as it is added, at frame 8: one ring 40 pixels out, over the badge and the empty space above it",
            8,
            MIDDLE,
            ADDED,
            WORDS,
            vec![(120, 50), (80, 10)],
            vec![middle, corner],
        ),
        (
            "as_added_frame_30",
            "at frame 30: the second ring 30 pixels out, the first gone past the edges",
            30,
            MIDDLE,
            ADDED,
            WORDS,
            vec![(110, 50)],
            vec![(120, 50), middle, corner],
        ),
        (
            "shockwave",
            "a shockwave at frame 8: one ring, 10 wide thinning to 1 over 20 frames, triangle, fading out",
            8,
            MIDDLE,
            [64.0, 1000.0, 6.0, 0.0, 90.0, 0.0, 0.0, 20.0, 100.0, 0.0, 12.0, 10.0, 1.0],
            ["triangle", "#ffffff"],
            vec![(128, 50)],
            vec![middle, corner],
        ),
        (
            "turning_squares",
            "turning squares at frame 20: 4 sides, a new one every 6 frames, spin 4 a frame",
            20,
            MIDDLE,
            [4.0, 6.0, 3.0, 0.0, 90.0, 0.0, 4.0, 30.0, 100.0, 0.0, 0.0, 2.0, 2.0],
            WORDS,
            vec![],
            vec![middle, corner],
        ),
        (
            "drifting_triangles",
            "triangles sent from (25, 50) at frame 24, drifting right 2 pixels a frame",
            24,
            [25.0, 50.0],
            [3.0, 8.0, 2.0, 0.0, 90.0, 2.0, 0.0, 30.0, 100.0, 0.0, 0.0, 2.0, 2.0],
            WORDS,
            vec![(40, 50)],
            vec![corner],
        ),
        (
            "orange_sine",
            "orange #ffb040, sine, 8 wide, opacity 80, at frame 30",
            30,
            MIDDLE,
            [64.0, 8.0, 3.0, 0.0, 90.0, 0.0, 0.0, 40.0, 80.0, 0.0, 20.0, 8.0, 8.0],
            ["sine", "#ffb040"],
            vec![(98, 50)],
            vec![middle, (110, 50)],
        ),
    ] {
        let effects = serde_json::json!([{
            "instance_id": "fx-0-0", "type_id": "core.radio_waves", "enabled": true,
            "parameters": {
                "producer_point": point, "sides": n[0], "interval": n[1], "expansion": n[2],
                "orientation": n[3], "direction": n[4], "velocity": n[5], "spin": n[6], "lifespan": n[7],
                "opacity": n[8], "fade_in_time": n[9], "fade_out_time": n[10], "start_width": n[11],
                "end_width": n[12], "profile": words[0], "color": words[1],
            },
        }]);
        let (bytes, big, said) = picture(&dir, effects, frame);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w * 3, h * 3, OutputDepth::Eight, &[], &big)
            .unwrap();
        let changed = (0..w * h).filter(|&i| bytes[i * 4..i * 4 + 4] != drawn[i * 4..i * 4 + 4]).count();
        let moved = |p| px(&bytes, p) != px(&drawn, p);
        let orange = name != "orange_sine" || lit.iter().all(|&p| px(&bytes, p)[0] > px(&bytes, p)[2]);
        t.row(
            &format!(
                "{name}.png, {what}; draws cleanly, changes some pixels, paints {lit:?}{} and leaves {kept:?} \
                 exactly",
                if name == "orange_sine" { " redder than blue" } else { "" }
            ),
            &format!(
                "{said:?}; {changed} changed; painted {:?}; left {:?}",
                lit.iter().map(|&p| px(&bytes, p)).collect::<Vec<_>>(),
                kept.iter().map(|&p| !moved(p)).collect::<Vec<_>>()
            ),
            said.is_empty()
                && changed > 0
                && lit.iter().all(|&p| moved(p))
                && kept.iter().all(|&p| !moved(p))
                && orange,
        );
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_rwave_001.json", 0), ("fx_rwave_008.json", 4), ("fx_rwave_019.json", 2)]);

    t.finish("B-135_radio_waves_table.md");
}
