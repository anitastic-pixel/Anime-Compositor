//! B-141: Roughen Edges in the core, against D-206.
//!
//! Writes `verification/B-141_roughen_edges_table.md`, and pictures in `verification/B-141 pictures/`.
//!
//! Every expected pixel is `Fixtures/roughen_edges/expected_roughen_edges.json`, written by
//! `tools/roughen_edges_reference.py` before this code existed and printed in document 25 as
//! FX-ROUGH-001 to 024. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The edge type and colour, then `[border, size, complexity, evolution, speed, seed]`.
fn roughen(kind: &str, color: &str, n: [f64; 6]) -> Effect {
    Effect::RoughenEdges {
        edge_type: kind.to_string(),
        edge_color: color.to_string(),
        border: n[0],
        size: n[1],
        complexity: n[2],
        evolution: n[3],
        speed: n[4],
        seed: n[5],
        frame: 0,
    }
}

const RUST: &str = "#8a3c14";
const START: [f64; 6] = [8.0, 10.0, 3.0, 0.0, 0.0, 0.0];
const PLATE: (usize, usize) = (160, 100);

/// A small made-up drawing on nothing: a red card with round corners, a blue disc and a yellow
/// bar, hard-edged.
fn drawing() -> Vec<u8> {
    let (w, h) = PLATE;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (fx, fy) = (x as f64 + 0.5, y as f64 + 0.5);
            let (dx, dy) = ((24.0 - fx).max(fx - 75.0).max(0.0), (24.0 - fy).max(fy - 77.0).max(0.0));
            let c = if (14.0..85.0).contains(&fx) && (14.0..87.0).contains(&fy) && dx.hypot(dy) <= 10.0 {
                [200, 50, 50, 255]
            } else if (fx - 119.5).hypot(fy - 43.5) < 27.5 {
                [40, 90, 210, 255]
            } else if (96.0..151.0).contains(&fx) && (76.0..89.0).contains(&fy) {
                [240, 200, 40, 255]
            } else {
                [0, 0, 0, 0]
            };
            bytes.extend(c);
        }
    }
    bytes
}

/// The drawing as the composition's one layer, with `effects`, drawn, straight 8-bit; and the
/// same enlarged three times.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/roughen_edges/fx_rough_001.json")).unwrap(),
    )
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

/// One Roughen Edges, or none, as a layer's `effects`.
fn stack(steps: &[(&str, &str, [f64; 6])]) -> J {
    J::Array(
        steps
            .iter()
            .enumerate()
            .map(|(i, (kind, color, n))| {
                serde_json::json!({
                    "instance_id": format!("fx-0-{i}"), "type_id": "core.roughen_edges", "enabled": true,
                    "parameters": { "edge_type": kind, "edge_color": color, "border": n[0], "size": n[1],
                        "complexity": n[2], "evolution": n[3], "speed": n[4], "seed": n[5] },
                })
            })
            .collect(),
    )
}

#[test]
fn b141_roughen_edges() {
    let mut t = Table::new(
        "roughen_edges",
        "# B-141: Roughen Edges\n\nD-206, accepted on 2026-09-28 with the After Effects picks \
         (B4). Every expected pixel is `Fixtures/roughen_edges/expected_roughen_edges.json`, \
         written by `tools/roughen_edges_reference.py` before this code existed and printed in \
         document 25 as FX-ROUGH-001 to 024. The build's frame is compared sample by sample; \
         the answer is the largest difference over all of them, against the catalogue's tolerance \
         of 2e-5.\n",
    );

    t.heading("FX-ROUGH-001 to 024 (document 25)");
    t.fixtures("expected_roughen_edges.json");

    t.heading("How far it reaches");
    let got = roughen("roughen", RUST, START).bounds_expansion();
    t.row("it never grows the layer: it declares no growth", &got.to_string(), got == 0);
    let mut draft = roughen("roughen_color", RUST, [16.0, 20.0, 4.0, 90.0, 5.0, 3.0]);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the border and the scale, and keeps the rest",
        &format!("{draft:?}"),
        draft == roughen("roughen_color", RUST, [8.0, 10.0, 4.0, 90.0, 5.0, 3.0]),
    );
    let mut draft = roughen("roughen", RUST, [16.0, 1.5, 4.0, 90.0, 5.0, 3.0]);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a scale of 1.5 at half size is held at 1, the least the setting takes",
        &format!("{draft:?}"),
        draft == roughen("roughen", RUST, [8.0, 1.0, 4.0, 90.0, 5.0, 3.0]),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=24).filter(|&n| n != 5).map(|n| format!("fx_rough_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_rough_005.json");
    t.row(
        "fx_rough_005.json, its colour written in capitals, is saved in small letters, as Snowfall's is",
        &params["edge_color"].to_string(),
        params["edge_color"] == serde_json::json!("#2060ff"),
    );
    t.shape_refused(
        "fx_rough_001.json",
        "no `edge_type` at all",
        r##"{"edge_color": "#8a3c14", "border": 2, "size": 4, "complexity": 3, "evolution": 0, "speed": 0, "seed": 0}"##,
    );
    t.shape_refused(
        "fx_rough_001.json",
        "an edge colour that is a number",
        r#"{"edge_type": "roughen", "edge_color": 8, "border": 2, "size": 4, "complexity": 3, "evolution": 0, "speed": 0, "seed": 0}"#,
    );
    t.shape_refused(
        "fx_rough_001.json",
        "a border that is a word",
        r##"{"edge_type": "roughen", "edge_color": "#8a3c14", "border": "two", "size": 4, "complexity": 3, "evolution": 0, "speed": 0, "seed": 0}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_rough_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("border -1", set(roughen("roughen", RUST, [-1.0, 10.0, 3.0, 0.0, 0.0, 0.0]))),
            ("border 501", set(roughen("roughen", RUST, [501.0, 10.0, 3.0, 0.0, 0.0, 0.0]))),
            ("scale 0", set(roughen("roughen", RUST, [8.0, 0.0, 3.0, 0.0, 0.0, 0.0]))),
            ("scale 1001", set(roughen("roughen", RUST, [8.0, 1001.0, 3.0, 0.0, 0.0, 0.0]))),
            ("complexity 0", set(roughen("roughen", RUST, [8.0, 10.0, 0.0, 0.0, 0.0, 0.0]))),
            ("complexity 11", set(roughen("roughen", RUST, [8.0, 10.0, 11.0, 0.0, 0.0, 0.0]))),
            ("evolution 100001", set(roughen("roughen", RUST, [8.0, 10.0, 3.0, 100001.0, 0.0, 0.0]))),
            ("evolution speed -361", set(roughen("roughen", RUST, [8.0, 10.0, 3.0, 0.0, -361.0, 0.0]))),
            ("seed 100001", set(roughen("roughen", RUST, [8.0, 10.0, 3.0, 0.0, 0.0, 100001.0]))),
            ("edge type \"spiky\"", set(roughen("spiky", RUST, START))),
            ("edge type \"Roughen\", written with a capital", set(roughen("Roughen", RUST, START))),
            ("edge colour \"#12345\"", set(roughen("roughen_color", "#12345", START))),
            ("border keyed to 501", keys("border", &[(0, &[2.0]), (4, &[501.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_rough_001.json",
        vec![
            ("every number at its bottom,", set(roughen("roughen_color", "#000000", [0.0, 1.0, 1.0, -100000.0, -360.0, 0.0]))),
            ("every number at its top,", set(roughen("roughen", "#FFFFFF", [500.0, 1000.0, 10.0, 100000.0, 360.0, 100000.0]))),
            ("border keyed from 0 to 20", keys("border", &[(0, &[0.0]), (4, &[20.0])])),
            ("evolution keyed from 0 to 720", keys("evolution", &[(0, &[0.0]), (4, &[720.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_rough_001.json", 0), ("fx_rough_005.json", 0), ("fx_rough_010.json", 2), ("fx_rough_015.json", 0)]);

    t.heading("Pictures: a made-up drawing on nothing, in `verification/B-141 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-141 pictures");
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

    // The pixels with no clear pixel of before.png within `r` either way, and the covered pixels
    // with one within 2.
    let deep = |r: usize| {
        let before = &before;
        every().filter(move |&(x, y)| {
            (y.saturating_sub(r)..(y + r + 1).min(h))
                .all(|v| (x.saturating_sub(r)..(x + r + 1).min(w)).all(|u| px(before, (u, v))[3] == 255))
        })
    };
    let rim: Vec<_> = every().filter(|&p| px(&before, p)[3] == 255 && !deep(2).any(|q| q == p)).collect();
    let grown = |after: &[u8]| every().filter(|&p| px(after, p)[3] > px(&before, p)[3]).count();
    let same_deep = |after: &[u8], r: usize| deep(r).filter(|&p| near(px(after, p), px(&before, p))).count();
    let bitten = |after: &[u8]| rim.iter().filter(|&&p| px(after, p)[3] < 128).count();

    let (start, big, said) = picture(&dir, stack(&[("roughen", RUST, START)]));
    write("as_it_starts.png", &big, 3);
    t.row(
        "as_it_starts.png, as it starts, border 8, scale 10, complexity 3: no pixel gains covering; \
         every pixel with nothing clear within 9 pixels unchanged; the edges bitten, many of the \
         pixels just inside them now more clear than covered; draws cleanly",
        &format!(
            "{said:?}, {} grown, {} of {} deep pixels unchanged, {} of {} edge pixels bitten",
            grown(&start), same_deep(&start, 9), deep(9).count(), bitten(&start), rim.len()
        ),
        said.is_empty() && grown(&start) == 0 && same_deep(&start, 9) == deep(9).count() && bitten(&start) * 5 > rim.len(),
    );

    let (wide, big, said) = picture(&dir, stack(&[("roughen", RUST, [16.0, 10.0, 3.0, 0.0, 0.0, 0.0])]));
    write("border_16.png", &big, 3);
    let clear = |bytes: &[u8]| every().filter(|&p| px(bytes, p)[3] == 0).count();
    t.row(
        "border_16.png, border 16: deeper bites, more of the drawing clear than as_it_starts.png; \
         no pixel gains covering; every pixel with nothing clear within 17 pixels unchanged; draws cleanly",
        &format!("{said:?}, {} clear against {}, {} grown, {} of {} deep pixels unchanged", clear(&wide), clear(&start), grown(&wide), same_deep(&wide, 17), deep(17).count()),
        said.is_empty() && clear(&wide) > clear(&start) && grown(&wide) == 0 && same_deep(&wide, 17) == deep(17).count(),
    );

    for (file, what, n) in [
        ("scale_40.png", "scale 40: wider, gentler bites", [8.0, 40.0, 3.0, 0.0, 0.0, 0.0]),
        ("complexity_6.png", "complexity 6: finer detail along the bites", [8.0, 10.0, 6.0, 0.0, 0.0, 0.0]),
        ("seed_3.png", "seed 3: other bites", [8.0, 10.0, 3.0, 0.0, 0.0, 3.0]),
    ] {
        let (after, big, said) = picture(&dir, stack(&[("roughen", RUST, n)]));
        write(file, &big, 3);
        t.row(
            &format!(
                "{file}, {what}: different from as_it_starts.png; no pixel gains covering; every \
                 pixel with nothing clear within 9 pixels unchanged; draws cleanly"
            ),
            &format!("{said:?}, {} grown, {} of {} deep pixels unchanged", grown(&after), same_deep(&after, 9), deep(9).count()),
            said.is_empty() && after != start && grown(&after) == 0 && same_deep(&after, 9) == deep(9).count(),
        );
    }

    let rust = [138, 60, 20];
    let rusty = |bytes: &[u8]| every().filter(|&p| px(bytes, p)[3] > 0 && near(px(bytes, p), [rust[0], rust[1], rust[2], px(bytes, p)[3]])).count();
    let (color, big, said) = picture(&dir, stack(&[("roughen_color", RUST, START)]));
    write("roughen_color.png", &big, 3);
    let same_cover = every().filter(|&p| px(&color, p)[3].abs_diff(px(&start, p)[3]) <= 1).count();
    t.row(
        "roughen_color.png, Roughen Color in rust: the same covering as as_it_starts.png, every \
         pixel within 1; the new edge rust #8a3c14 in many pixels; every pixel with nothing clear \
         within 17 pixels unchanged; draws cleanly",
        &format!(
            "{said:?}, {same_cover} of {} the same covering, {} rust, {} of {} deep pixels unchanged",
            w * h, rusty(&color), same_deep(&color, 17), deep(17).count()
        ),
        said.is_empty() && same_cover == w * h && rusty(&color) * 10 > rim.len() && same_deep(&color, 17) == deep(17).count(),
    );

    let (blue, big, said) = picture(&dir, stack(&[("roughen_color", "#2060ff", [16.0, 10.0, 3.0, 0.0, 0.0, 0.0])]));
    write("roughen_color_blue_16.png", &big, 3);
    let same_cover = every().filter(|&p| px(&blue, p)[3].abs_diff(px(&wide, p)[3]) <= 1).count();
    let bluish = every().filter(|&p| px(&blue, p)[3] > 0 && near(px(&blue, p), [0x20, 0x60, 0xff, px(&blue, p)[3]])).count();
    t.row(
        "roughen_color_blue_16.png, Roughen Color in blue #2060ff at border 16: the same covering \
         as border_16.png, every pixel within 1; a wider band, more pixels of the edge colour than \
         roughen_color.png has of rust; draws cleanly",
        &format!("{said:?}, {same_cover} of {} the same covering, {bluish} blue against {} rust", w * h, rusty(&color)),
        said.is_empty() && same_cover == w * h && bluish > rusty(&color),
    );

    t.finish("B-141_roughen_edges_table.md");
}
