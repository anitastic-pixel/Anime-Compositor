//! B-641: D-762, Cell Pattern gains After Effects' Offset, Overflow, Evolution Options (Cycle
//! Evolution, Cycle) and Tiling Options (Enable Tiling, Cells Horizontal, Cells Vertical), and
//! Contrast up to 10000. A file without them draws as before.
//!
//! Writes `verification/B-641_cell_pattern_more_table.md` and draws pictures into
//! `verification/B-641 pictures/`.
//!
//! Every expected pixel is `Fixtures/cell_pattern/expected_cell_pattern.json`, written by
//! `tools/cell_pattern_reference.py` before this code existed and printed in document 25 as
//! FX-CELL-034 to 059. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    changed, distance, keys, reference, repo, same_json, saved, set, town, Table, MAIN, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

fn is_cell(e: &Effect) -> bool {
    matches!(e, Effect::CellPattern { .. })
}

/// A Cell Pattern as added from the panel, size 40, with `p` laid over it.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"pattern": "bubbles", "invert": "off", "contrast": 100, "disperse": 1,
        "size": 40, "evolution": 0, "seed": 0, "dark_color": "#000000", "light_color": "#ffffff",
        "opacity": 100, "blend": "normal"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.cell_pattern", "enabled": true, "parameters": all})
}

/// `effects` on the street at `frame`, drawn on the processor, straight 8-bit, with what it
/// warned of.
fn picture(dir: &Path, effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let out = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (out.to_srgb8_straight(), said)
}

/// `json` opened and drawn at `frame` on the processor, from the folder `root`.
fn draw(json: &J, root: &Path, frame: i32) -> Vec<f32> {
    let loaded = persist::load_str(&json.to_string()).expect("the file reads");
    let mut log = FrameLog::new(3);
    render_frame(loaded.document.project(), &Id::new(MAIN), frame, root, 64, &mut log)
        .expect("the frame draws")
        .data()
        .to_vec()
}

/// fx_cell_001's Cell Pattern with `f` applied to its new settings.
fn more(
    t: &Table,
    f: impl FnOnce(&mut [f64; 2], &mut String, &mut String, &mut f64, &mut String, &mut [f64; 2], &mut f64),
) -> Effect {
    changed(t, "fx_cell_001.json", |e| {
        if let Effect::CellPattern {
            offset,
            overflow,
            cycle_evolution,
            cycle,
            tiling,
            tiles_horizontal,
            tiles_vertical,
            contrast,
            ..
        } = e
        {
            let mut tiles = [*tiles_horizontal, *tiles_vertical];
            f(offset, overflow, cycle_evolution, cycle, tiling, &mut tiles, contrast);
            [*tiles_horizontal, *tiles_vertical] = tiles;
        }
    })
}

#[test]
fn b641_cell_pattern_more() {
    let mut t = Table::new(
        "cell_pattern",
        "# B-641: Cell Pattern's Offset, Overflow, Evolution Options and Tiling Options (D-762)\n\n\
         D-762 adds After Effects' Offset (`offset`, pixels, x then y, keyable, absent 0, 0), \
         Overflow (`overflow`: clip, soft_clamp or wrap_back, absent clip), Evolution Options \
         (`cycle_evolution` off or on, absent off; `cycle`, 1 to 1000 whole turns, absent 1) and \
         Tiling Options (`tiling` off or on, absent off; `tiles_horizontal` and `tiles_vertical`, \
         1 to 1000 cells, absent 10), and lets Contrast reach 10000 (document 21). A file without \
         them draws as before. Every expected pixel is \
         `Fixtures/cell_pattern/expected_cell_pattern.json`, written by \
         `tools/cell_pattern_reference.py` before this code existed and printed in document 25 as \
         FX-CELL-034 to 059. Tolerance 2e-5.\n",
    );

    t.heading("FX-CELL-034 to 059 (document 25)");
    t.fixtures_numbered("expected_cell_pattern.json", 34..=59);

    t.heading("Older projects: none of the new settings, drawn and saved as before");
    let root = repo("Fixtures/cell_pattern");
    for n in 1..=33 {
        let file = format!("fx_cell_{n:03}.json");
        let text = fs::read_to_string(root.join(&file)).unwrap();
        let Ok(loaded) = persist::load_str(&text) else {
            continue;
        };
        let old: J = serde_json::from_str(&text).unwrap();
        // FX-CELL-014's capitals are read lowercase (D-209), so its colours are compared so.
        let lower = |j: &J| -> J { serde_json::from_str(&j.to_string().to_lowercase()).unwrap() };
        let kept = same_json(&lower(&saved(&loaded)), &lower(&old));
        // Its twin with every new setting written at its absent value draws the same frames.
        let mut twin = old.clone();
        let p = &mut twin["compositions"][0]["layers"][0]["effects"][0]["parameters"];
        for (k, v) in [
            ("offset", json!([0, 0])),
            ("overflow", json!("clip")),
            ("cycle_evolution", json!("off")),
            ("cycle", json!(1)),
            ("tiling", json!("off")),
            ("tiles_horizontal", json!(10)),
            ("tiles_vertical", json!(10)),
        ] {
            p[k] = v;
        }
        let same = [0, 4].iter().all(|&f| draw(&old, &root, f) == draw(&twin, &root, f));
        t.row(
            &format!("{file} (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values"),
            &format!("saved {}; {}", if kept { "the same" } else { "differently" }, if same { "bit-identical" } else { "differ" }),
            kept && same,
        );
    }

    t.heading("The file");
    let files: Vec<String> = (34..=49).map(|n| format!("fx_cell_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let p = t.saved_parameters("fx_cell_001.json");
    let none = [
        "offset",
        "overflow",
        "cycle_evolution",
        "cycle",
        "tiling",
        "tiles_horizontal",
        "tiles_vertical",
    ]
    .iter()
    .all(|k| p.get(k).is_none());
    t.row(
        "fx_cell_001.json (none of the new settings) is saved with none of them written",
        &p.to_string(),
        none,
    );
    let p = t.saved_parameters("fx_cell_049.json");
    t.row(
        "fx_cell_049.json, all seven written at their absent values, is saved with all seven kept",
        &p.to_string(),
        p["offset"].as_array().is_some_and(|a| a.iter().all(|v| v.as_f64() == Some(0.0)))
            && p["overflow"] == "clip"
            && p["cycle_evolution"] == "off"
            && p["cycle"] == 1.0
            && p["tiling"] == "off"
            && p["tiles_horizontal"] == 10.0
            && p["tiles_vertical"] == 10.0,
    );
    for (n, what, word) in [
        (50, "Contrast 10001", "contrast"),
        (51, "Overflow \"allow_hdr\"", "overflow"),
        (52, "Cycle Evolution \"yes\"", "cycle evolution"),
        (53, "Cycle 0", "cycle"),
        (54, "Cycle 1001", "cycle"),
        (55, "Enable Tiling \"yes\"", "tiling"),
        (56, "Cells Horizontal 0", "tiles horizontal"),
        (57, "Cells Vertical 1001", "tiles vertical"),
        (58, "Offset 100001, 0", "offset"),
    ] {
        let file = format!("fx_cell_{n:03}.json");
        let why = changed(&t, &file, |_| {}).why_invalid();
        t.row(
            &format!("{file} ({what}) is refused in a sentence naming {word}"),
            &why,
            why.contains(word),
        );
    }
    let base = r##""pattern": "bubbles", "invert": "off", "contrast": 100, "disperse": 1, "size": 4, "evolution": 0, "seed": 0, "dark_color": "#000000", "light_color": "#ffffff", "opacity": 100, "blend": "normal""##;
    for (what, extra) in [
        ("an offset that is one number", r#""offset": 3"#),
        ("an overflow that is a number", r#""overflow": 2"#),
        ("a cycle that is a word", r#""cycle": "two""#),
        ("a tiling that is a number", r#""tiling": 1"#),
    ] {
        t.shape_refused("fx_cell_001.json", what, &format!("{{{base}, {extra}}}"));
    }

    t.heading("How far it reaches");
    let mut draft = changed(&t, "fx_cell_036.json", |_| {});
    draft.scale_distances(|d| d * 0.5);
    let ok = matches!(&draft, Effect::CellPattern { offset, size, .. } if *offset == [-0.75, 1.25] && *size == 2.0);
    t.row(
        "a half-size draft preview halves the offset, -1.5, 2.5 to -0.75, 1.25, with the size",
        &format!("{draft:?}"),
        ok,
    );
    let mut draft = changed(&t, "fx_cell_047.json", |_| {});
    draft.scale_distances(|d| d * 0.5);
    let ok = matches!(&draft, Effect::CellPattern { tiles_horizontal, tiles_vertical, cycle, .. }
        if *tiles_horizontal == 2.0 && *tiles_vertical == 2.0 && *cycle == 1.0);
    t.row(
        "it keeps the cells across and down and the cycle, which are counts, not distances",
        &format!("{draft:?}"),
        ok,
    );
    let grows = changed(&t, "fx_cell_036.json", |_| {}).bounds_expansion();
    t.row("an offset never grows the layer", &grows.to_string(), grows == 0);

    t.heading("Commands");
    let mut document = t.load("fx_cell_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("overflow \"allow_hdr\"", set(more(&t, |_, o, _, _, _, _, _| *o = "allow_hdr".into()))),
            ("overflow \"Clip\", with a capital", set(more(&t, |_, o, _, _, _, _, _| *o = "Clip".into()))),
            ("cycle evolution \"yes\"", set(more(&t, |_, _, c, _, _, _, _| *c = "yes".into()))),
            ("cycle 0", set(more(&t, |_, _, _, n, _, _, _| *n = 0.0))),
            ("cycle 1001", set(more(&t, |_, _, _, n, _, _, _| *n = 1001.0))),
            ("tiling \"yes\"", set(more(&t, |_, _, _, _, w, _, _| *w = "yes".into()))),
            ("cells horizontal 0", set(more(&t, |_, _, _, _, _, s, _| s[0] = 0.0))),
            ("cells vertical 1001", set(more(&t, |_, _, _, _, _, s, _| s[1] = 1001.0))),
            ("offset 100001, 0", set(more(&t, |o, _, _, _, _, _, _| *o = [100001.0, 0.0]))),
            ("contrast 10001", set(more(&t, |_, _, _, _, _, _, c| *c = 10001.0))),
            ("offset keyed to -100001, 0", keys("offset", &[(0, &[0.0, 0.0]), (4, &[-100001.0, 0.0])])),
            ("cycle keyed to 0", keys("cycle", &[(0, &[1.0]), (4, &[0.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_cell_001.json",
        vec![
            (
                "every new setting at its bottom, contrast 0,",
                set(more(&t, |o, w, c, n, l, s, k| {
                    (*o, *w, *c, *n, *l, *s, *k) = ([-100000.0; 2], "clip".into(), "on".into(), 1.0, "on".into(), [1.0; 2], 0.0)
                })),
            ),
            (
                "every new setting at its top, contrast 10000,",
                set(more(&t, |o, w, c, n, l, s, k| {
                    (*o, *w, *c, *n, *l, *s, *k) = ([100000.0; 2], "wrap_back".into(), "on".into(), 1000.0, "on".into(), [1000.0; 2], 10000.0)
                })),
            ),
            ("offset keyed from 0, 0 to 50, -50", keys("offset", &[(0, &[0.0, 0.0]), (4, &[50.0, -50.0])])),
            ("cycle keyed from 1 to 5", keys("cycle", &[(0, &[1.0]), (4, &[5.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_cell_036.json", 0),
        ("fx_cell_037.json", 2),
        ("fx_cell_040.json", 0),
        ("fx_cell_042.json", 0),
        ("fx_cell_044.json", 2),
        ("fx_cell_047.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // FX-CELL-016 is opacity 0, which leaves the layer untouched and nothing for the card.
    let refused: Vec<u32> = [16].into_iter().chain(23..=33).chain(50..=59).collect();
    t.card_fixtures(&mut gpu, "expected_cell_pattern.json", &refused, is_cell);
    let plain = reference(|_| json!([]));
    let shots = vec![
        (
            "Cell Pattern, opacity 60, offset 25, -10, overflow wrap back at contrast 400",
            reference(|id| json!([fx(&format!("b641-{id}"), &json!({"opacity": 60, "offset": [25, -10], "overflow": "wrap_back", "contrast": 400}))])),
            3,
        ),
        (
            "Cell Pattern, opacity 60, soft clamp at contrast 300, tiled 6 by 4, cycle 3 at evolution 500",
            reference(|id| json!([fx(&format!("b641-{id}"), &json!({"opacity": 60, "overflow": "soft_clamp", "contrast": 300, "tiling": "on", "tiles_horizontal": 6, "tiles_vertical": 4, "cycle_evolution": "on", "cycle": 3, "evolution": 500}))])),
            3,
        ),
    ];
    t.card_reference(&mut gpu, &plain, &shots, is_cell);

    t.heading("Pictures: in `verification/B-641 pictures/`, the street under size-40 bubbles");
    let dir = repo("verification/B-641 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| {
        png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap()
    };
    write("town.png", &town());
    let (w, h) = TOWN;
    let px = |b: &[u8], x: usize, y: usize| b[(y * w + x) * 4..(y * w + x) * 4 + 4].to_vec();
    let shot = |name: &str, p: J, frame: i32| {
        let (bytes, said) = picture(&dir, json!([fx("fx-0-0", &p)]), frame);
        write(name, &bytes);
        (bytes, said)
    };

    let (start, said0) = shot("offset_0.png", json!({}), 0);
    let (moved, said1) = shot("offset_40_0.png", json!({"offset": [40, 0]}), 0);
    let follows = (0..h).all(|y| (40..w).all(|x| px(&moved, x, y) == px(&start, x - 40, y)));
    t.row(
        "offset_0.png and offset_40_0.png: Offset 40, 0 slides the whole pattern 40 pixels right, each pixel the one 40 to its left in the picture without; both draw cleanly",
        &format!("{said0:?} {said1:?}; slid exactly: {follows}"),
        said0.is_empty() && said1.is_empty() && follows,
    );

    let ends = |b: &[u8]| (0..w * h).filter(|i| b[i * 4 + 1] == 0 || b[i * 4 + 1] == 255).count();
    let mut overflow = Vec::new();
    for word in ["clip", "soft_clamp", "wrap_back"] {
        let (b, said) = shot(&format!("overflow_{word}.png"), json!({"contrast": 300, "overflow": word}), 0);
        overflow.push((word, ends(&b), said, b));
    }
    let (clip, soft, wrap) = (&overflow[0], &overflow[1], &overflow[2]);
    t.row(
        "overflow_clip.png, overflow_soft_clamp.png, overflow_wrap_back.png, contrast 300: Clip pins many pixels at black or white; Soft Clamp eases them in, so far fewer sit there; Wrap Back folds them back, drawing rings inside the balls; all draw cleanly",
        &format!("pixels at black or white: clip {}, soft clamp {}, wrap back {}; {:?} {:?} {:?}", clip.1, soft.1, wrap.1, clip.2, soft.2, wrap.2),
        clip.2.is_empty() && soft.2.is_empty() && wrap.2.is_empty() && soft.1 * 4 < clip.1 && wrap.1 < clip.1 && distance(&wrap.3, &clip.3).1 > 1000,
    );

    let looped = json!({"cycle_evolution": "on", "cycle": 2,
        "evolution": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 4, "value": 720, "interp": "linear"}]}});
    let frames: Vec<(Vec<u8>, Vec<String>)> = (0..=4).map(|f| shot(&format!("cycle_frame_{f}.png"), looped.clone(), f)).collect();
    t.row(
        "cycle_frame_0.png to cycle_frame_4.png: Cycle Evolution on, Cycle 2, Evolution keyed 0 to 720 over frames 0 to 4: the cells turn at their own speeds, so frame 2 (one turn) is not frame 0, and frame 4 (two turns, the cycle's end) is frame 0 again pixel for pixel: the loop closes; all draw cleanly",
        &format!("frame 4 against frame 0: {} pixels differ; frame 2 against frame 0: {}", distance(&frames[4].0, &frames[0].0).1, distance(&frames[2].0, &frames[0].0).1),
        frames.iter().all(|f| f.1.is_empty()) && frames[4].0 == frames[0].0 && frames[2].0 != frames[0].0,
    );

    let (tiled, said) = shot("tiling_4_by_2.png", json!({"tiling": "on", "tiles_horizontal": 4, "tiles_vertical": 2}), 0);
    let repeats = (0..h).all(|y| (160..w).all(|x| px(&tiled, x, y) == px(&tiled, x - 160, y)))
        && (80..h).all(|y| (0..w).all(|x| px(&tiled, x, y) == px(&tiled, x, y - 80)));
    t.row(
        "tiling_4_by_2.png: Enable Tiling on, 4 cells across and 2 down at size 40: the pattern repeats every 160 pixels across and 80 down; draws cleanly",
        &format!("{said:?}; repeats: {repeats}"),
        said.is_empty() && repeats,
    );
    let (bubbles, said) = shot("bubbles_unchanged.png", json!({"overflow": "clip", "cycle_evolution": "off", "tiling": "off", "offset": [0, 0]}), 0);
    t.row(
        "bubbles_unchanged.png: the Bubbles look is kept: a Bubbles with every new setting written at its absent value is offset_0.png pixel for pixel",
        &format!("{said:?}; {} pixels differ", distance(&bubbles, &start).1),
        said.is_empty() && bubbles == start,
    );

    t.finish("B-641_cell_pattern_more_table.md");
}
