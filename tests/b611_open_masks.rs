//! B-611: open masks, against D-737.
//!
//! Writes `verification/B-611_open_masks_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/open_masks/`, written by `tools/open_mask_reference.py` before any code, its stroke
//! numbers `tools/stroke_reference.py`'s worked on an open path.
//!
//! # What is checked
//!
//! Every case's frames within the catalogue's 2e-5, warning of nothing; every file opened and
//! saved holds what it held, `closed` where it was written and nowhere else; opening and closing
//! a mask by command is one step to undo each; the two refused files are refused, naming closed.

mod effect_table;

use std::fs;

use effect_table::{Table, MAIN};
use serde_json::Value as J;

use anime_compositor::command::Command;
use anime_compositor::mask::{MaskMode, MaskPoint};
use anime_compositor::model::Id;
use anime_compositor::persist;

#[test]
fn b611_open_masks() {
    let mut t = Table::new(
        "open_masks",
        "# B-611: open masks\n\nD-737: a mask may be open, with no piece from its last point \
         back to its first. It cuts nothing and is drawn along open by the effects that take a \
         path. Every expected pixel is `Fixtures/open_masks/expected_open_masks.json`, written \
         by `tools/open_mask_reference.py` before this code existed and printed in document 25 \
         as FX-MSKO. The build's frame is compared sample by sample; the answer is the largest \
         difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-MSKO-001 to 010 (document 25)");
    t.fixtures("expected_open_masks.json");

    t.heading("The file");
    let files = [
        "fx_msko_001.json",
        "fx_msko_002.json",
        "fx_msko_003.json",
        "fx_msko_004.json",
        "fx_msko_005.json",
        "fx_msko_006.json",
        "fx_msko_010.json",
    ];
    t.round_trips(&files);
    let expected: J =
        serde_json::from_str(&fs::read_to_string(t.root.join("expected_open_masks.json")).unwrap())
            .unwrap();
    for (case, c) in expected["refused"].as_object().unwrap() {
        let file = c["file"].as_str().unwrap();
        let (id, word) = (c["refused"].as_str().unwrap(), c["names"].as_str().unwrap());
        let read = persist::load_str(&fs::read_to_string(t.root.join(file)).unwrap());
        t.row(
            &format!("{case}: {file} is refused, {id}, naming {word}"),
            &match &read {
                Ok(_) => "read".to_string(),
                Err(d) => format!("{}: {}", d.id.as_str(), d.detail),
            },
            read.err()
                .is_some_and(|d| d.id.as_str() == id && d.detail.contains(word)),
        );
    }

    t.heading("Opening and closing, one step each");
    let mut document = t.load("fx_msko_002.json").document;
    let masks = |d: &anime_compositor::command::Document| {
        d.project()
            .composition(&Id::new(MAIN))
            .unwrap()
            .layer(&Id::new("art"))
            .unwrap()
            .masks
            .clone()
    };
    let set = |masks| Command::SetMasks {
        composition: Id::new(MAIN),
        layer_id: Id::new("art"),
        masks,
    };
    let closed = masks(&document);
    let mut open = closed.clone();
    open[0].closed = false;
    let depth = document.undo_depth();
    let taken = document.apply(set(open.clone())).is_ok();
    let drawn = t.render(&document, 0, 64).data()
        == t.render(&t.load("fx_msko_001.json").document, 0, 64).data();
    t.row(
        "the closed roof opened by one command draws FX-MSKO-001's frame",
        &format!("taken {taken}, the same pixels {drawn}"),
        taken && drawn && document.undo_depth() == depth + 1,
    );
    document.undo();
    t.row(
        "one undo closes it again",
        &format!("closed {}", masks(&document)[0].closed),
        masks(&document)[0].closed,
    );
    let mut add = open.clone();
    add[0].mode = MaskMode::Add;
    let refused = document.apply(set(add)).err();
    t.row(
        "an open mask of mode Add is refused with a sentence, and nothing changes",
        &refused
            .as_ref()
            .map_or("taken".to_string(), |d| d.message.clone()),
        refused.is_some() && masks(&document)[0].closed,
    );
    let mut line = open.clone();
    line[0].points.truncate(2);
    let two = document.apply(set(line.clone())).is_ok();
    line[0].closed = true;
    let closed_two = document.apply(set(line)).err();
    t.row(
        "an open mask of two points is taken; closed, two points are refused",
        &format!(
            "open taken {two}; closed: {}",
            closed_two
                .as_ref()
                .map_or("taken".to_string(), |d| d.message.clone())
        ),
        two && closed_two.is_some(),
    );
    // A Z drawn back across itself: its first and last legs cross.
    let mut crossing = open;
    crossing[0].points = [(2.0, 2.0), (12.0, 8.0), (12.0, 2.0), (2.0, 8.0)]
        .map(|(x, y)| MaskPoint::corner(x, y))
        .to_vec();
    let crossed = document.apply(set(crossing)).is_ok();
    t.row(
        "an open mask whose legs cross is taken: it encloses nothing",
        &format!("taken {crossed}"),
        crossed,
    );

    t.finish("B-611_open_masks_table.md");
}
