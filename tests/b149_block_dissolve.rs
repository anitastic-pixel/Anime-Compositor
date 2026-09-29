//! B-149: Block Dissolve in the core, against D-214.
//!
//! Writes `verification/B-149_block_dissolve_table.md`, and pictures in
//! `verification/B-149 pictures/`.
//!
//! Every expected pixel is `Fixtures/block_dissolve/expected_block_dissolve.json`, written by
//! `tools/block_dissolve_reference.py` before this code existed and printed in document 25 as
//! FX-BDISSOLVE-001 to 019. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// Block Dissolve with its completion, block width, block height and feather.
fn dissolve(completion: f64, block_width: f64, block_height: f64, feather: f64) -> Effect {
    Effect::BlockDissolve {
        completion,
        block_width,
        block_height,
        feather,
    }
}

const PLATE: (usize, usize) = (160, 100);

/// A made-up title card on nothing: a navy panel with a pink band across it and a yellow disc.
fn card() -> Vec<u8> {
    let (w, h) = PLATE;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (fx, fy) = (x as f64 + 0.5, y as f64 + 0.5);
            let c = if !(8.0..152.0).contains(&fx) || !(8.0..92.0).contains(&fy) {
                [0, 0, 0, 0]
            } else if (fx - 80.0).hypot(fy - 38.0) < 20.0 {
                [255, 220, 60, 255]
            } else if (40.0..60.0).contains(&fy) {
                [240, 110, 150, 255]
            } else {
                [30, 40, 90, 255]
            };
            bytes.extend(c);
        }
    }
    bytes
}

/// `art` as the composition's one layer, with `effects`, drawn, straight 8-bit; and the same
/// enlarged three times.
fn picture(dir: &Path, art: &str, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/block_dissolve/fx_bdissolve_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from(art);
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

/// One Block Dissolve as a layer's `effects`, from its settings as the file writes them.
fn one(parameters: J) -> J {
    let mut p = serde_json::json!({ "completion": 0, "block_width": 1, "block_height": 1, "feather": 0 });
    for (k, v) in parameters.as_object().unwrap() {
        p[k] = v.clone();
    }
    serde_json::json!([{ "instance_id": "fx-0-0", "type_id": "core.block_dissolve", "enabled": true, "parameters": p }])
}

#[test]
fn b149_block_dissolve() {
    let mut t = Table::new(
        "block_dissolve",
        "# B-149: Block Dissolve\n\nD-214, accepted on 2026-09-28 with the After Effects picks \
         (B12): After Effects' Block Dissolve in purpose and names, by this program's own rule; its \
         Soft Edges switch is left out. Every expected pixel is \
         `Fixtures/block_dissolve/expected_block_dissolve.json`, written by \
         `tools/block_dissolve_reference.py` before this code existed and printed in document 25 as \
         FX-BDISSOLVE-001 to 019. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-BDISSOLVE-001 to 019 (document 25)");
    t.fixtures_numbered("expected_block_dissolve.json", 1..=19);

    t.heading("How far it reaches");
    let got = dissolve(50.0, 4.0, 4.0, 40.0).bounds_expansion();
    t.row("Block Dissolve grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = dissolve(40.0, 12.0, 6.0, 3.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the block width, block height and feather, and leaves the completion",
        &format!("{draft:?}"),
        draft == dissolve(40.0, 6.0, 3.0, 1.5),
    );
    let mut draft = dissolve(40.0, 1.0, 1.5, 0.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview never makes a block less than a pixel",
        &format!("{draft:?}"),
        draft == dissolve(40.0, 1.0, 1.0, 0.0),
    );

    t.heading("The file");
    t.round_trips(
        &(1..=19)
            .map(|n| format!("fx_bdissolve_{n:03}.json"))
            .collect::<Vec<_>>()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
    );
    t.shape_refused("fx_bdissolve_003.json", "no `feather` at all", r#"{"completion": 50, "block_width": 4, "block_height": 4}"#);
    t.shape_refused(
        "fx_bdissolve_003.json",
        "a block width that is a list",
        r#"{"completion": 50, "block_width": [4], "block_height": 4, "feather": 0}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_bdissolve_003.json").document;
    t.refused(
        &mut document,
        vec![
            ("completion -1", set(dissolve(-1.0, 4.0, 4.0, 0.0))),
            ("completion 100.5", set(dissolve(100.5, 4.0, 4.0, 0.0))),
            ("block width 0.5", set(dissolve(50.0, 0.5, 4.0, 0.0))),
            ("block height 10001", set(dissolve(50.0, 4.0, 10001.0, 0.0))),
            ("feather -1", set(dissolve(50.0, 4.0, 4.0, -1.0))),
            ("completion keyed to 150", keys("completion", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_bdissolve_003.json",
        vec![
            ("every number at its bottom,", set(dissolve(0.0, 1.0, 1.0, 0.0))),
            ("every number at its top,", set(dissolve(100.0, 10000.0, 10000.0, 10000.0))),
            ("completion keyed from 0 to 100", keys("completion", &[(0, &[0.0]), (4, &[100.0])])),
            ("block width keyed from 1 to 20", keys("block_width", &[(0, &[1.0]), (4, &[20.0])])),
            ("feather keyed from 0 to 10", keys("feather", &[(0, &[0.0]), (4, &[10.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_bdissolve_002.json", 0),
        ("fx_bdissolve_007.json", 0),
        ("fx_bdissolve_008.json", 0),
        ("fx_bdissolve_009.json", 2),
        ("fx_bdissolve_012.json", 0),
        ("fx_bdissolve_013.json", 0),
    ]);

    t.heading("Pictures: a made-up title card, in `verification/B-149 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-149 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8], scale: usize| {
        png_out::write_rgba(&dir.join(name), w * scale, h * scale, OutputDepth::Eight, &[], bytes).unwrap()
    };
    let art = card();
    write("card.png", &art, 1);
    let shown = art.chunks(4).filter(|p| p[3] > 0).count();
    // Each shown pixel as it was, gone, or part gone with its colour kept.
    #[derive(Default, Debug)]
    struct Count {
        kept: usize,
        gone: usize,
        part: usize,
        recoloured: usize,
    }
    let count = |bytes: &[u8]| {
        let mut c = Count::default();
        for (a, b) in art.chunks(4).zip(bytes.chunks(4)).filter(|(a, _)| a[3] > 0) {
            if a == b {
                c.kept += 1;
            } else if b[3] == 0 {
                c.gone += 1;
            } else {
                c.part += 1;
                if a[..3].iter().zip(&b[..3]).any(|(x, y)| x.abs_diff(*y) > 2) {
                    c.recoloured += 1;
                }
            }
        }
        c
    };
    let empty_stays = |bytes: &[u8]| art.chunks(4).zip(bytes.chunks(4)).all(|(a, b)| a[3] > 0 || b[3] == 0);
    let mut drawn = std::collections::HashMap::new();
    let mut sheet = |name: &str, settings: J, says: &str, want: &dyn Fn(&Count) -> bool| {
        let (bytes, big, said) = picture(&dir, "card.png", one(settings));
        write(name, &big, 3);
        let c = count(&bytes);
        t.row(
            &format!("{name}, {says}; nothing appears where the card is empty; draws cleanly"),
            &format!(
                "{said:?}, of {shown} shown pixels {} as they were, {} gone, {} part gone ({} recoloured)",
                c.kept, c.gone, c.part, c.recoloured
            ),
            said.is_empty() && empty_stays(&bytes) && c.recoloured == 0 && want(&c),
        );
        drawn.insert(name.to_string(), bytes);
    };
    let share = |c: &Count| c.gone as f64 / shown as f64;
    sheet("card_start.png", serde_json::json!({}), "as it starts: the card as it was", &|c| c.kept == shown);
    sheet(
        "card_quarter.png",
        serde_json::json!({ "completion": 25, "block_width": 10, "block_height": 10 }),
        "completion 25 in blocks 10 by 10: about a quarter gone, whole blocks",
        &|c| c.part == 0 && (0.1..0.4).contains(&share(c)),
    );
    sheet(
        "card_half.png",
        serde_json::json!({ "completion": 50, "block_width": 10, "block_height": 10 }),
        "completion 50: about half gone",
        &|c| c.part == 0 && (0.35..0.65).contains(&share(c)),
    );
    sheet(
        "card_three_quarters.png",
        serde_json::json!({ "completion": 75, "block_width": 10, "block_height": 10 }),
        "completion 75: about three quarters gone",
        &|c| c.part == 0 && (0.6..0.9).contains(&share(c)),
    );
    sheet(
        "card_bricks.png",
        serde_json::json!({ "completion": 50, "block_width": 40, "block_height": 5 }),
        "completion 50 in blocks 40 wide by 5 tall: long flat bricks, some gone",
        &|c| c.part == 0 && (0.2..0.8).contains(&share(c)),
    );
    sheet(
        "card_pixels.png",
        serde_json::json!({ "completion": 50 }),
        "completion 50 in blocks 1 by 1: single pixels, about half gone",
        &|c| c.part == 0 && (0.45..0.55).contains(&share(c)),
    );
    sheet(
        "card_feather.png",
        serde_json::json!({ "completion": 50, "block_width": 10, "block_height": 10, "feather": 8 }),
        "completion 50 in blocks 10 by 10 with a feather of 8: soft-edged blocks, their colour kept",
        &|c| c.part > 1000 && c.kept > 0 && c.gone > 0,
    );
    sheet(
        "card_gone.png",
        serde_json::json!({ "completion": 100, "block_width": 10, "block_height": 10 }),
        "completion 100: nothing left",
        &|c| c.gone == shown,
    );
    let gone = |name: &str| -> Vec<bool> { drawn[name].chunks(4).map(|p| p[3] == 0).collect() };
    let (quarter, half, three) = (gone("card_quarter.png"), gone("card_half.png"), gone("card_three_quarters.png"));
    let never_back = |a: &[bool], b: &[bool]| a.iter().zip(b).all(|(x, y)| !x || *y);
    t.row(
        "every pixel gone at 25 is gone at 50, and every one gone at 50 is gone at 75: a block once gone stays gone",
        &format!(
            "{} gone at 25, {} at 50, {} at 75",
            quarter.iter().filter(|g| **g).count(),
            half.iter().filter(|g| **g).count(),
            three.iter().filter(|g| **g).count()
        ),
        never_back(&quarter, &half) && never_back(&half, &three),
    );

    t.finish("B-149_block_dissolve_table.md");
}
