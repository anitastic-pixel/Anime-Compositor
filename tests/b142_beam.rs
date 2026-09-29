//! B-142: Beam in the core, against D-207.
//!
//! Writes `verification/B-142_beam_table.md`, and pictures in `verification/B-142 pictures/`.
//!
//! Every expected pixel is `Fixtures/beam/expected_beam.json`, written by
//! `tools/beam_reference.py` before this code existed and printed in document 25 as FX-BEAM-001
//! to 029. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The two points, `[length, time, start thickness, end thickness, softness]`, the two colours
/// and the composite word.
fn beam(start: [f64; 2], end: [f64; 2], n: [f64; 5], inside: &str, outside: &str, composite: &str) -> Effect {
    Effect::Beam {
        start,
        end,
        length: n[0],
        time: n[1],
        start_thickness: n[2],
        end_thickness: n[3],
        softness: n[4],
        inside_color: inside.to_string(),
        outside_color: outside.to_string(),
        composite: composite.to_string(),
    }
}

const BLUE: &str = "#3c8cff";
const START: [f64; 5] = [25.0, 0.0, 8.0, 8.0, 50.0];
const PLATE: (usize, usize) = (160, 100);

/// The beam as it starts, with other numbers.
fn plain(n: [f64; 5]) -> Effect {
    beam([10.0, 50.0], [90.0, 50.0], n, "#ffffff", BLUE, "on")
}

/// A small made-up night: a sky darkening upward and dark rooftops along the bottom.
fn drawing() -> Vec<u8> {
    let (w, h) = PLATE;
    let roofs = [(0, 30, 70), (30, 52, 80), (52, 90, 64), (90, 118, 76), (118, 160, 68)];
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let roof = roofs.iter().any(|&(x0, x1, top)| (x0..=x1).contains(&x) && y >= top);
            bytes.extend(if roof { [14, 16, 28, 255] } else { [20 + y as u8 / 4, 30 + y as u8 / 3, 70 + y as u8 / 2, 255] });
        }
    }
    bytes
}

/// The drawing as the composition's one layer, with `effects`, drawn, straight 8-bit; and the
/// same enlarged three times.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J =
        serde_json::from_str(&fs::read_to_string(effect_table::repo("Fixtures/beam/fx_beam_001.json")).unwrap())
            .unwrap();
    project["assets"][0]["path"] = J::from("drawing.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(PLATE.0);
    comp["height"] = J::from(PLATE.1);
    let layer = &mut comp["layers"][0];
    let middle = serde_json::json!([PLATE.0 as f64 / 2.0, PLATE.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    let bytes = frame.to_srgb8_straight();
    let big: Vec<u8> = (0..PLATE.1 * 3)
        .flat_map(|y| (0..PLATE.0 * 3).map(move |x| (x, y)))
        .flat_map(|(x, y)| {
            let i = (y / 3 * PLATE.0 + x / 3) * 4;
            bytes[i..i + 4].to_vec()
        })
        .collect();
    (bytes, big, said)
}

/// One Beam, or none, as a layer's `effects`.
fn stack(steps: &[&Effect]) -> J {
    J::Array(
        steps
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let Effect::Beam {
                    start,
                    end,
                    length,
                    time,
                    start_thickness,
                    end_thickness,
                    softness,
                    inside_color,
                    outside_color,
                    composite,
                } = e
                else {
                    unreachable!()
                };
                serde_json::json!({
                    "instance_id": format!("fx-0-{i}"), "type_id": "core.beam", "enabled": true,
                    "parameters": { "start": start, "end": end, "length": length, "time": time,
                        "start_thickness": start_thickness, "end_thickness": end_thickness,
                        "softness": softness, "inside_color": inside_color,
                        "outside_color": outside_color, "composite": composite },
                })
            })
            .collect(),
    )
}

#[test]
fn b142_beam() {
    let mut t = Table::new(
        "beam",
        "# B-142: Beam\n\nD-207, accepted on 2026-09-28 with the After Effects picks (B5). Every \
         expected pixel is `Fixtures/beam/expected_beam.json`, written by \
         `tools/beam_reference.py` before this code existed and printed in document 25 as \
         FX-BEAM-001 to 029. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-BEAM-001 to 029 (document 25)");
    t.fixtures("expected_beam.json");

    t.heading("How far it reaches");
    let got = plain(START).bounds_expansion();
    t.row("it never grows the layer: it declares no growth", &got.to_string(), got == 0);
    let mut draft = plain([40.0, 30.0, 6.0, 20.0, 50.0]);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the two thicknesses, and keeps the points, length, time and softness",
        &format!("{draft:?}"),
        draft == plain([40.0, 30.0, 3.0, 10.0, 50.0]),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=29).filter(|&n| n != 10).map(|n| format!("fx_beam_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_beam_010.json");
    t.row(
        "fx_beam_010.json, its outside colour written in capitals, is saved in small letters, as Snowfall's is",
        &params["outside_color"].to_string(),
        params["outside_color"] == serde_json::json!("#ff3020"),
    );
    t.shape_refused(
        "fx_beam_001.json",
        "no `composite` at all",
        r##"{"start": [25, 45], "end": [75, 45], "length": 100, "time": 0, "start_thickness": 4, "end_thickness": 4, "softness": 0, "inside_color": "#ffffff", "outside_color": "#3c8cff"}"##,
    );
    t.shape_refused(
        "fx_beam_001.json",
        "a start that is one number",
        r##"{"start": 25, "end": [75, 45], "length": 100, "time": 0, "start_thickness": 4, "end_thickness": 4, "softness": 0, "inside_color": "#ffffff", "outside_color": "#3c8cff", "composite": "on"}"##,
    );
    t.shape_refused(
        "fx_beam_001.json",
        "an inside colour that is a number",
        r##"{"start": [25, 45], "end": [75, 45], "length": 100, "time": 0, "start_thickness": 4, "end_thickness": 4, "softness": 0, "inside_color": 8, "outside_color": "#3c8cff", "composite": "on"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_beam_001.json").document;
    let with = |start: [f64; 2], n: [f64; 5]| beam(start, [90.0, 50.0], n, "#ffffff", BLUE, "on");
    t.refused(
        &mut document,
        vec![
            ("length -1", set(plain([-1.0, 0.0, 8.0, 8.0, 50.0]))),
            ("length 101", set(plain([101.0, 0.0, 8.0, 8.0, 50.0]))),
            ("time -1", set(plain([25.0, -1.0, 8.0, 8.0, 50.0]))),
            ("time 101", set(plain([25.0, 101.0, 8.0, 8.0, 50.0]))),
            ("starting thickness -1", set(plain([25.0, 0.0, -1.0, 8.0, 50.0]))),
            ("ending thickness 501", set(plain([25.0, 0.0, 8.0, 501.0, 50.0]))),
            ("softness 101", set(plain([25.0, 0.0, 8.0, 8.0, 101.0]))),
            ("start -1001 across", set(with([-1001.0, 50.0], START))),
            ("start 1001 down", set(with([10.0, 1001.0], START))),
            ("inside colour \"#12345\"", set(beam([10.0, 50.0], [90.0, 50.0], START, "#12345", BLUE, "on"))),
            ("outside colour \"blue\"", set(beam([10.0, 50.0], [90.0, 50.0], START, "#ffffff", "blue", "on"))),
            ("composite \"yes\"", set(beam([10.0, 50.0], [90.0, 50.0], START, "#ffffff", BLUE, "yes"))),
            ("composite \"On\", written with a capital", set(beam([10.0, 50.0], [90.0, 50.0], START, "#ffffff", BLUE, "On"))),
            ("time keyed to 101", keys("time", &[(0, &[0.0]), (4, &[101.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_beam_001.json",
        vec![
            (
                "every number at its bottom,",
                set(beam([-1000.0, -1000.0], [-1000.0, -1000.0], [0.0; 5], "#000000", "#000000", "off")),
            ),
            (
                "every number at its top,",
                set(beam([1000.0, 1000.0], [1000.0, 1000.0], [100.0, 100.0, 500.0, 500.0, 100.0], "#FFFFFF", "#FFFFFF", "on")),
            ),
            ("time keyed from 0 to 100", keys("time", &[(0, &[0.0]), (4, &[100.0])])),
            ("start keyed from (0, 0) to (100, 100)", keys("start", &[(0, &[0.0, 0.0]), (4, &[100.0, 100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_beam_001.json", 0), ("fx_beam_007.json", 0), ("fx_beam_013.json", 0), ("fx_beam_015.json", 2)]);

    t.heading("Pictures: a made-up night, in `verification/B-142 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-142 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8], scale: usize| {
        png_out::write_rgba(&dir.join(name), w * scale, h * scale, OutputDepth::Eight, &[], bytes).unwrap()
    };
    write("drawing.png", &drawing(), 1);
    let px = |bytes: &[u8], (x, y): (usize, usize)| {
        let i = (y * w + x) * 4;
        [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
    };
    let near = |a: [u8; 4], b: [u8; 4]| a.iter().zip(b).all(|(&u, v)| u.abs_diff(v) <= 1);
    let every = || (0..h).flat_map(move |y| (0..w).map(move |x| (x, y)));
    let (before, big, said) = picture(&dir, stack(&[]));
    write("before.png", &big, 3);
    t.row("before.png, the drawing with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());

    // The pixels that differ from before.png, and whether all of them lie in a box.
    let changed = |after: &[u8]| every().filter(|&p| !near(px(after, p), px(&before, p))).collect::<Vec<_>>();
    let inside = |after: &[u8], (x0, x1, y0, y1): (usize, usize, usize, usize)| {
        changed(after).iter().all(|&(x, y)| (x0..=x1).contains(&x) && (y0..=y1).contains(&y))
    };
    let white = |p: [u8; 4]| p[..3].iter().all(|&c| c >= 230);
    let same = |after: &[u8], p| near(px(after, p), px(&before, p));

    // The line runs from 16 to 144 pixels across, along row 50; as it starts, 16 to 48 is lit.
    for (file, what, n, lit, dark, reach) in [
        (
            "as_it_starts.png",
            "as it starts, length 25, time 0: the first quarter of the line lit, white in the middle at 32 across",
            START,
            (32, 50),
            (80, 50),
            (8, 56, 42, 58),
        ),
        (
            "time_50.png",
            "time 50: the lit quarter in the middle of the line, white at 80 across, the start dark",
            [25.0, 50.0, 8.0, 8.0, 50.0],
            (80, 50),
            (32, 50),
            (56, 104, 42, 58),
        ),
        (
            "time_100.png",
            "time 100: the lit quarter at the end of the line, white at 128 across",
            [25.0, 100.0, 8.0, 8.0, 50.0],
            (128, 50),
            (80, 50),
            (104, 152, 42, 58),
        ),
    ] {
        let (after, big, said) = picture(&dir, stack(&[&plain(n)]));
        write(file, &big, 3);
        t.row(
            &format!("{file}, {what}; unchanged at {dark:?}; nothing changed further than 8 pixels from the lit stretch; draws cleanly"),
            &format!("{said:?}, {:?} at {lit:?}, {} pixels changed", px(&after, lit), changed(&after).len()),
            said.is_empty() && white(px(&after, lit)) && same(&after, dark) && inside(&after, reach),
        );
    }

    let (full, big, said) = picture(&dir, stack(&[&plain([100.0, 0.0, 8.0, 8.0, 50.0])]));
    write("length_100.png", &big, 3);
    t.row(
        "length_100.png, length 100: the whole line lit, white at 32, 80 and 128 across; nothing changed \
         further than 8 pixels from it; draws cleanly",
        &format!("{said:?}, {} pixels changed", changed(&full).len()),
        said.is_empty()
            && [(32, 50), (80, 50), (128, 50)].iter().all(|&p| white(px(&full, p)))
            && inside(&full, (8, 152, 42, 58)),
    );

    let (taper, big, said) = picture(&dir, stack(&[&plain([100.0, 0.0, 2.0, 20.0, 50.0])]));
    write("thickness_2_to_20.png", &big, 3);
    let tall = |x: usize| (0..h).filter(|&y| !same(&taper, (x, y))).count();
    t.row(
        "thickness_2_to_20.png, from 2 pixels thick at the start to 20 at the end: more rows lit at 136 \
         across than at 80, and more at 80 than at 24; draws cleanly",
        &format!("{said:?}, rows changed at 24, 80 and 136 across: {}, {}, {}", tall(24), tall(80), tall(136)),
        said.is_empty() && tall(24) < tall(80) && tall(80) < tall(136),
    );

    let (hard, big, said) = picture(&dir, stack(&[&plain([100.0, 0.0, 8.0, 8.0, 0.0])]));
    write("softness_0.png", &big, 3);
    let (soft, big, said_soft) = picture(&dir, stack(&[&plain([100.0, 0.0, 8.0, 8.0, 100.0])]));
    write("softness_100.png", &big, 3);
    t.row(
        "softness_0.png and softness_100.png: both white along the line at 80 across; 6 rows above it \
         untouched at softness 0 and lit at softness 100, which reaches twice as far; both draw cleanly",
        &format!("{said:?} {said_soft:?}, {} and {} pixels changed", changed(&hard).len(), changed(&soft).len()),
        said.is_empty()
            && said_soft.is_empty()
            && white(px(&hard, (80, 50)))
            && white(px(&soft, (80, 50)))
            && same(&hard, (80, 44))
            && !same(&soft, (80, 44))
            && changed(&soft).len() > changed(&hard).len(),
    );

    let (slant, big, said) = picture(&dir, stack(&[&beam([10.0, 90.0], [90.0, 10.0], [100.0, 0.0, 12.0, 12.0, 50.0], "#ffe080", "#ff3020", "on")]));
    write("yellow_in_red.png", &big, 3);
    let mid = px(&slant, (80, 50));
    t.row(
        "yellow_in_red.png, yellow #ffe080 in red #ff3020 from low left to high right, 12 thick: the \
         middle at 80 across near the yellow; the corners top left and bottom right untouched; draws cleanly",
        &format!("{said:?}, {mid:?} in the middle"),
        said.is_empty()
            && mid[0] >= 240
            && (190..=235).contains(&mid[1])
            && mid[2] <= 150
            && same(&slant, (4, 4))
            && same(&slant, (155, 95)),
    );

    let (alone, big, said) = picture(&dir, stack(&[&beam([10.0, 50.0], [90.0, 50.0], [100.0, 0.0, 8.0, 8.0, 50.0], "#ffffff", BLUE, "off")]));
    write("composite_off.png", &big, 3);
    let clear = every().filter(|&p| px(&alone, p)[3] == 0).count();
    t.row(
        "composite_off.png, Composite On Original off: the night gone, the beam alone, white along the \
         line and clear more than 8 pixels from it; draws cleanly",
        &format!("{said:?}, {:?} on the line, {clear} of {} pixels clear", px(&alone, (80, 50)), w * h),
        said.is_empty()
            && white(px(&alone, (80, 50)))
            && px(&alone, (80, 50))[3] == 255
            && every().filter(|&(_, y)| !(42..=58).contains(&y)).all(|p| px(&alone, p)[3] == 0),
    );

    t.finish("B-142_beam_table.md");
}
