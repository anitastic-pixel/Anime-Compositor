//! B-633: Mosaic gains After Effects' Horizontal and Vertical Blocks and Sharp Colors, against
//! D-754.
//!
//! Writes `verification/B-633_mosaic_counts_table.md`.
//!
//! Every expected pixel is `Fixtures/mosaic/expected_mosaic.json`, written by
//! `tools/mosaic_reference.py` before this code and printed in document 25 as FX-MOSAIC-001 to
//! 037. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, settings, Table};

use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;

/// Mode count, `h` by `v` blocks, Sharp Colors `sharp`.
fn counts(h: f64, v: f64, sharp: &str) -> Effect {
    Effect::Mosaic {
        size: 10.0,
        mode: "count".to_string(),
        horizontal: h,
        vertical: v,
        sharp_colors: sharp.to_string(),
    }
}

/// Mode `mode`, everything else as a Mosaic starts.
fn mode(mode: &str) -> Effect {
    Effect::Mosaic {
        size: 10.0,
        mode: mode.to_string(),
        horizontal: 10.0,
        vertical: 10.0,
        sharp_colors: "off".to_string(),
    }
}

#[test]
fn b633_mosaic_counts() {
    let mut t = Table::new(
        "mosaic",
        "# B-633: Mosaic gains After Effects' Horizontal and Vertical Blocks and Sharp Colors\n\n\
         D-754, accepted on 2026-10-23 by the owner's message \"Proceed, if an effect is more \
         like a inspiration/modified version of a AE effect, only improve it.\" Mosaic is \
         inspired by After Effects' (D-144), so nothing it drew before changes: size mode and its \
         fixtures stay, and a file that names no mode is size mode. Mode count splits the \
         drawing's own box evenly into Horizontal by Vertical blocks; Sharp Colors gives each \
         block its middle pixel instead of the mean. Every expected pixel is \
         `Fixtures/mosaic/expected_mosaic.json`, written by `tools/mosaic_reference.py` before \
         this code and printed in document 25 as FX-MOSAIC-001 to 037. The build's frame is \
         compared sample by sample against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-MOSAIC-001 to 037 (document 25)");
    t.fixtures("expected_mosaic.json");

    t.heading("A project saved before D-754");
    let loaded = t.load("fx_mosaic_001.json");
    let got = settings(&loaded.document);
    t.row(
        "fx_mosaic_001.json, written before D-754 with only a size, opens in size mode, Sharp \
         Colors off",
        &got,
        got.contains("mode: \"size\"") && got.contains("sharp_colors: \"off\""),
    );
    let saved = t.saved_parameters("fx_mosaic_001.json");
    t.row(
        "saving it again writes only the size",
        &saved.to_string(),
        saved.as_object().map(|o| o.len()) == Some(1) && saved.get("size").is_some(),
    );
    let saved = t.saved_parameters("fx_mosaic_032.json");
    t.row(
        "a file that wrote mode size and Sharp Colors off keeps them when saved",
        &saved.to_string(),
        saved.get("mode").and_then(|m| m.as_str()) == Some("size")
            && saved.get("sharp_colors").and_then(|m| m.as_str()) == Some("off"),
    );
    let mut draft = counts(10.0, 4.0, "off");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft halves the size but keeps the counts, so it shows the same blocks",
        &format!("{draft:?}"),
        draft
            == Effect::Mosaic {
                size: 5.0,
                mode: "count".to_string(),
                horizontal: 10.0,
                vertical: 4.0,
                sharp_colors: "off".to_string(),
            },
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_mosaic_001.json",
        "fx_mosaic_021.json",
        "fx_mosaic_022.json",
        "fx_mosaic_026.json",
        "fx_mosaic_027.json",
        "fx_mosaic_030.json",
        "fx_mosaic_031.json",
        "fx_mosaic_032.json",
        "fx_mosaic_033.json",
        "fx_mosaic_034.json",
        "fx_mosaic_037.json",
    ]);
    t.shape_refused(
        "fx_mosaic_021.json",
        "a mode that is a number",
        r#"{"size": 10, "mode": 2, "horizontal": 10, "vertical": 10}"#,
    );
    t.shape_refused(
        "fx_mosaic_021.json",
        "Horizontal Blocks that is a word",
        r#"{"size": 10, "mode": "count", "horizontal": "ten", "vertical": 10}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_mosaic_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("mode \"grid\"", set(mode("grid"))),
            ("mode \"Count\"", set(mode("Count"))),
            ("Sharp Colors \"yes\"", set(counts(10.0, 10.0, "yes"))),
            ("Horizontal Blocks 0", set(counts(0.0, 10.0, "off"))),
            ("Vertical Blocks 1001", set(counts(10.0, 1001.0, "off"))),
            (
                "Horizontal Blocks keyed to 0.5",
                keys("horizontal", &[(0, &[10.0]), (4, &[0.5])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_mosaic_001.json",
        vec![
            ("mode count, 10 by 10,", set(counts(10.0, 10.0, "off"))),
            ("1 by 1, the bottom,", set(counts(1.0, 1.0, "off"))),
            ("1000 by 1000, the top,", set(counts(1000.0, 1000.0, "off"))),
            ("Sharp Colors on,", set(counts(4.0, 2.0, "on"))),
            ("mode size,", set(mode("size"))),
            (
                "Vertical Blocks keyed from 2 to 6",
                keys("vertical", &[(0, &[2.0]), (4, &[6.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_mosaic_021.json", 0),
        ("fx_mosaic_023.json", 0),
        ("fx_mosaic_026.json", 0),
        ("fx_mosaic_027.json", 0),
        ("fx_mosaic_028.json", 0),
        ("fx_mosaic_029.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Left out of every frame: size 1 (002), and the files the effect is refused in.
    let none = [2, 15, 16, 17, 18, 19, 20, 33, 34, 35, 36, 37];
    t.card_fixtures(&mut gpu, "expected_mosaic.json", &none, |e| {
        matches!(e, Effect::Mosaic { .. })
    });

    t.finish("B-633_mosaic_counts_table.md");
}
