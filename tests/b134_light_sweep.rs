//! B-134: Light Sweep in the core, against D-199.
//!
//! Writes `verification/B-134_light_sweep_table.md`.
//!
//! Every expected pixel is `Fixtures/light_sweep/expected_light_sweep.json`, written by
//! `tools/light_sweep_reference.py` before this code existed and printed in document 25 as
//! FX-SWEEP-001 to 026. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a badge, empty round it, and sweeps a shine across it six ways into
//! `verification/B-134 pictures/`, three times enlarged, over a dark blue where it is clear.

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

/// The centre; direction, width, sweep intensity, edge intensity and edge thickness; and shape,
/// light colour and light reception.
fn sweep(center: [f64; 2], n: [f64; 5], words: [&str; 3]) -> Effect {
    Effect::LightSweep {
        center,
        direction: n[0],
        shape: words[0].to_string(),
        width: n[1],
        sweep_intensity: n[2],
        edge_intensity: n[3],
        edge_thickness: n[4],
        light_color: words[1].to_string(),
        light_reception: words[2].to_string(),
    }
}

const MIDDLE: [f64; 2] = [50.0, 50.0];
const ADDED: [f64; 5] = [-30.0, 50.0, 50.0, 100.0, 1.0];
const WORDS: [&str; 3] = ["smooth", "#ffffff", "add"];
const PLATE: (usize, usize) = (160, 100);
const BLUE: [u8; 3] = [58, 111, 216];
const RED: [u8; 3] = [200, 40, 40];
const GOLD: [u8; 3] = [224, 168, 48];
const LINE: [u8; 3] = [30, 26, 36];
const BACK: [f64; 3] = [32.0, 40.0, 56.0];

/// The badge: a blue plate with a dark border three pixels wide, a red bar across and a gold
/// disc in the middle; nothing round it.
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

/// The plate as the composition's one layer, with `effects`, drawn, straight 8-bit; and the same
/// enlarged three times over a dark blue where it is clear.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/light_sweep/fx_sweep_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("plate.png");
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
            let a = bytes[i + 3] as f64 / 255.0;
            let over = |c: u8, b: f64| (c as f64 * a + b * (1.0 - a)).round() as u8;
            [over(bytes[i], BACK[0]), over(bytes[i + 1], BACK[1]), over(bytes[i + 2], BACK[2]), 255]
        })
        .collect();
    (bytes, big, said)
}

#[test]
fn b134_light_sweep() {
    let mut t = Table::new(
        "light_sweep",
        "# B-134: Light Sweep\n\nD-199, accepted on 2026-09-28 with the After Effects picks (A9). \
         Every expected pixel is `Fixtures/light_sweep/expected_light_sweep.json`, written by \
         `tools/light_sweep_reference.py` before this code existed and printed in document 25 as \
         FX-SWEEP-001 to 026. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-SWEEP-001 to 026 (document 25)");
    t.fixtures("expected_light_sweep.json");

    t.heading("How far it reaches");
    let got = sweep(MIDDLE, ADDED, WORDS).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = sweep(MIDDLE, [-30.0, 50.0, 50.0, 100.0, 6.0], WORDS);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the width, 50 to 25, and the edge thickness, 6 to 3, \
         and keeps the rest",
        &format!("{draft:?}"),
        draft == sweep(MIDDLE, [-30.0, 25.0, 50.0, 100.0, 3.0], WORDS),
    );
    let mut thin = sweep(MIDDLE, ADDED, WORDS);
    thin.scale_distances(|d| d * 0.5);
    t.row(
        "and an edge thickness of 1 stays 1, the least it can be",
        &format!("{thin:?}"),
        thin == sweep(MIDDLE, [-30.0, 25.0, 50.0, 100.0, 1.0], WORDS),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_sweep_001.json",
        "fx_sweep_006.json",
        "fx_sweep_008.json",
        "fx_sweep_010.json",
        "fx_sweep_012.json",
        "fx_sweep_014.json",
        "fx_sweep_015.json",
        "fx_sweep_016.json",
        "fx_sweep_019.json",
        "fx_sweep_024.json",
        "fx_sweep_026.json",
    ]);
    t.shape_refused(
        "fx_sweep_001.json",
        "no `light_reception` at all",
        r##"{"center": [50, 50], "direction": -30, "shape": "smooth", "width": 50, "sweep_intensity": 50, "edge_intensity": 100, "edge_thickness": 1, "light_color": "#ffffff"}"##,
    );
    t.shape_refused(
        "fx_sweep_001.json",
        "a centre of one number",
        r##"{"center": [50], "direction": -30, "shape": "smooth", "width": 50, "sweep_intensity": 50, "edge_intensity": 100, "edge_thickness": 1, "light_color": "#ffffff", "light_reception": "add"}"##,
    );
    t.shape_refused(
        "fx_sweep_001.json",
        "a width that is a word",
        r##"{"center": [50, 50], "direction": -30, "shape": "smooth", "width": "wide", "sweep_intensity": 50, "edge_intensity": 100, "edge_thickness": 1, "light_color": "#ffffff", "light_reception": "add"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_sweep_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("centre x 1000.5", set(sweep([1000.5, 50.0], ADDED, WORDS))),
            ("direction -3600.5", set(sweep(MIDDLE, [-3600.5, 50.0, 50.0, 100.0, 1.0], WORDS))),
            ("width -1", set(sweep(MIDDLE, [-30.0, -1.0, 50.0, 100.0, 1.0], WORDS))),
            ("width 10000.5", set(sweep(MIDDLE, [-30.0, 10000.5, 50.0, 100.0, 1.0], WORDS))),
            ("sweep intensity 101", set(sweep(MIDDLE, [-30.0, 50.0, 101.0, 100.0, 1.0], WORDS))),
            ("edge intensity -0.5", set(sweep(MIDDLE, [-30.0, 50.0, 50.0, -0.5, 1.0], WORDS))),
            ("edge thickness 0.9", set(sweep(MIDDLE, [-30.0, 50.0, 50.0, 100.0, 0.9], WORDS))),
            ("edge thickness 50.5", set(sweep(MIDDLE, [-30.0, 50.0, 50.0, 100.0, 50.5], WORDS))),
            ("shape \"round\"", set(sweep(MIDDLE, ADDED, ["round", "#ffffff", "add"]))),
            ("light colour \"#fff\"", set(sweep(MIDDLE, ADDED, ["smooth", "#fff", "add"]))),
            ("light reception \"glow\"", set(sweep(MIDDLE, ADDED, ["smooth", "#ffffff", "glow"]))),
            ("edge thickness keyed to 60", keys("edge_thickness", &[(0, &[1.0]), (4, &[60.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_sweep_001.json",
        vec![
            (
                "every setting at the top of its range, the last words,",
                set(sweep([1000.0, 1000.0], [3600.0, 10000.0, 100.0, 100.0, 50.0], ["sharp", "#000000", "cutout"])),
            ),
            (
                "every setting at the bottom, the other words,",
                set(sweep([-1000.0, -1000.0], [-3600.0, 0.0, 0.0, 0.0, 1.0], ["linear", "#ffb040", "composite"])),
            ),
            ("the centre keyed from (0, 50) to (100, 50)", keys("center", &[(0, &[0.0, 50.0]), (4, &[100.0, 50.0])])),
        ],
    );

    t.heading("Pictures: a badge, in `verification/B-134 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-134 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let drawn = plate();
    png_out::write_rgba(&dir.join("plate.png"), w, h, OutputDepth::Eight, &[], &drawn).unwrap();
    let (_, big, said) = picture(&dir, J::Array(vec![]));
    png_out::write_rgba(&dir.join("before.png"), w * 3, h * 3, OutputDepth::Eight, &[], &big).unwrap();
    t.row("before.png, the badge with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());
    let px = |bytes: &[u8], (x, y): (usize, usize)| {
        let i = (y * w + x) * 4;
        [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
    };
    let brighter = |a: [u8; 4], b: [u8; 4]| a[3] == b[3] && a[..3].iter().zip(&b[..3]).all(|(u, v)| u >= v) && a != b;
    let (corner, off) = ((5, 5), (118, 20));
    let at = |x: usize| (x, 50);
    for (name, what, center, n, words) in [
        ("start", "as it is added: a soft white band leaning left through the middle", MIDDLE, ADDED, WORDS),
        ("sweep_1", "a sharp band keyed along, a third of the way", [30.0, 50.0], [-30.0, 24.0, 70.0, 100.0, 1.0], ["sharp", "#ffffff", "add"]),
        ("sweep_2", "half way", MIDDLE, [-30.0, 24.0, 70.0, 100.0, 1.0], ["sharp", "#ffffff", "add"]),
        ("sweep_3", "two thirds of the way", [70.0, 50.0], [-30.0, 24.0, 70.0, 100.0, 1.0], ["sharp", "#ffffff", "add"]),
        ("edges", "the edges alone, three pixels deep", MIDDLE, [-30.0, 10000.0, 0.0, 100.0, 3.0], WORDS),
        ("composite", "composited in orange #ffb040", MIDDLE, [-30.0, 80.0, 90.0, 100.0, 1.0], ["smooth", "#ffb040", "composite"]),
        ("cutout", "cutout: only the light is left", MIDDLE, [-30.0, 80.0, 100.0, 100.0, 1.0], ["smooth", "#ffffff", "cutout"]),
    ] {
        let effects = serde_json::json!([{
            "instance_id": "fx-0-0", "type_id": "core.light_sweep", "enabled": true,
            "parameters": {
                "center": center, "direction": n[0], "shape": words[0], "width": n[1],
                "sweep_intensity": n[2], "edge_intensity": n[3], "edge_thickness": n[4],
                "light_color": words[1], "light_reception": words[2],
            },
        }]);
        let (bytes, big, said) = picture(&dir, effects);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w * 3, h * 3, OutputDepth::Eight, &[], &big)
            .unwrap();
        let got = |p| px(&bytes, p);
        let was = |p| px(&drawn, p);
        let (claim, ok) = match name {
            "start" => (
                "the gold middle brighter, (118, 20) off the band and the empty corner (5, 5) as they were",
                brighter(got(at(80)), was(at(80))) && got(off) == was(off) && got(corner) == was(corner),
            ),
            "edges" => (
                "the border's outer three pixels at (40, 50) and (42, 50) brighter, (43, 50) and the gold middle as \
                 they were",
                brighter(got(at(40)), was(at(40)))
                    && brighter(got(at(42)), was(at(42)))
                    && got(at(43)) == was(at(43))
                    && got(at(80)) == was(at(80)),
            ),
            "composite" => (
                "the gold middle within 8 of the orange, the empty corner (5, 5) as it was",
                got(at(80)).iter().zip([255, 176, 64, 255]).all(|(&u, v)| u.abs_diff(v) <= 8) && got(corner) == was(corner),
            ),
            "cutout" => (
                "the gold middle white, (118, 20) off the band cut away, the empty corner (5, 5) as it was",
                got(at(80)) == [255; 4] && got(off)[3] == 0 && got(corner) == was(corner),
            ),
            _ => {
                let lit = 48 + 32 * (name.as_bytes()[6] - b'1') as usize;
                let dark: Vec<usize> = [48, 80, 112].into_iter().filter(|&x| x != lit).collect();
                (
                    "the band's own spot on the middle row brighter, the other two spots of (48, 50), (80, 50) and \
                     (112, 50) as they were",
                    brighter(got(at(lit)), was(at(lit))) && dark.iter().all(|&x| got(at(x)) == was(at(x))),
                )
            }
        };
        let spots: Vec<[u8; 4]> = [at(40), at(42), at(43), at(48), at(80), at(112), off, corner].into_iter().map(got).collect();
        t.row(
            &format!("{name}.png, {what}; draws cleanly; {claim}"),
            &format!(
                "{said:?}; (40, 50), (42, 50), (43, 50), (48, 50), (80, 50), (112, 50), (118, 20), (5, 5): {spots:?}"
            ),
            said.is_empty() && ok,
        );
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_sweep_001.json", 0), ("fx_sweep_010.json", 0), ("fx_sweep_019.json", 0)]);

    t.finish("B-134_light_sweep_table.md");
}
