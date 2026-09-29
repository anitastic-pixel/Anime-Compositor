//! B-125b: a layer of this composition as an effect's setting (D-189), on the processor.
//!
//! Writes `verification/B-125b_layer_map_table.md` and `verification/B-125 pictures/maps.png`.
//!
//! Every expected size, pixel and diagnostic is `Fixtures/layer_map/expected_layer_map.json`,
//! written by `tools/effect_layer_reference.py` before this code existed and printed in document
//! 25 as FX-LMAP-001 to 042. No effect holds a layer setting until Compound Blur (A2), so each map
//! is asked of the program directly, as that effect will ask for it.

mod effect_table;

use std::fs;

use effect_table::{largest_difference, Table, MAIN};
use serde_json::Value as J;

use anime_compositor::compose;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::preview::PreviewQuality;
use anime_compositor::{png_out, OutputDepth, WorkingBuffer};

/// Enlarged this many times each way in the picture.
const ZOOM: usize = 16;

#[test]
fn b125_layer_map_table() {
    let mut t = Table::new(
        "layer_map",
        "# B-125b: a layer of this composition as an effect's setting (D-189)\n\n\
         Written by `tests/b125_layer_map.rs`. Each row is one case of document 25's FX-LMAP \
         table: the program makes the map an effect on the holder would read from the layer its \
         setting names, and it is compared with `Fixtures/layer_map/expected_layer_map.json`, \
         which `tools/effect_layer_reference.py` worked out pixel by pixel from documents 20 and \
         21 without running the program. Tolerance 1e-6. Nothing is seen in the app until \
         Compound Blur (A2), the first effect with a layer setting; `verification/B-125 \
         pictures/maps.png` shows every map the program made, enlarged.\n",
    );
    let expected: J = serde_json::from_str(
        &fs::read_to_string(t.root.join("expected_layer_map.json")).unwrap(),
    )
    .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let loaded = t.load(expected["project"].as_str().unwrap());
    let project = loaded.document.project();

    t.heading("FX-LMAP-001 to 042: the map each case reads");
    let mut maps: Vec<WorkingBuffer> = Vec::new();
    for (name, case) in expected["cases"].as_object().unwrap() {
        let frame = case["frame"].as_i64().unwrap() as i32;
        let quality = match case["quality"].as_str().unwrap() {
            "full" => PreviewQuality::Full,
            _ => PreviewQuality::Draft,
        };
        let mut log = FrameLog::new(usize::MAX);
        let map = compose::layer_map(
            project,
            &Id::new(MAIN),
            &Id::new(case["holder"].as_str().unwrap()),
            case["named"].as_str().unwrap(),
            case["fit"].as_str().unwrap(),
            frame,
            &t.root,
            quality,
            &mut log,
        );
        let said: Vec<&str> = log.ids_at(frame).iter().map(|d| d.as_str()).collect();
        let want: Vec<&str> = case["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .map(|d| d.as_str().unwrap())
            .collect();
        let (built, ok) = match (&map, case.get("map").filter(|m| !m.is_null())) {
            (None, None) => ("no map".to_string(), true),
            (Some(m), None) => (format!("a map {} by {}, where none is", m.width(), m.height()), false),
            (None, Some(_)) => ("no map, where one is".to_string(), false),
            (Some(m), Some(e)) => {
                let size = (m.width() as u64, m.height() as u64);
                let wanted = (e["width"].as_u64().unwrap(), e["height"].as_u64().unwrap());
                if size != wanted {
                    (format!("{} by {}, where {} by {}", size.0, size.1, wanted.0, wanted.1), false)
                } else {
                    let d = largest_difference(m, &e["pixels"]);
                    (format!("{} by {}, largest difference {d:.1e}", size.0, size.1), d <= tolerance)
                }
            }
        };
        t.row(
            &format!("{name}: {}", case["says"].as_str().unwrap()),
            &format!("{built}; says {said:?}"),
            ok && said == want,
        );
        maps.extend(map.filter(|m| m.width() > 0));
    }

    // Every map the program made, in the order of the table, each on a checkerboard so its clear
    // parts show, in one sheet.
    let columns = 6;
    let cell = (8 * ZOOM + ZOOM, 6 * ZOOM + ZOOM);
    let rows = maps.len().div_ceil(columns);
    let (w, h) = (columns * cell.0 + ZOOM, rows * cell.1 + ZOOM);
    // Dark round the maps, so an adjustment layer's white shows.
    let mut sheet = [48u8, 48, 48, 255].repeat(w * h);
    for (i, map) in maps.iter().enumerate() {
        let (left, top) = (ZOOM + (i % columns) * cell.0, ZOOM + (i / columns) * cell.1);
        let rgba = map.to_srgb8_straight();
        for y in 0..map.height() * ZOOM {
            for x in 0..map.width() * ZOOM {
                let p = &rgba[((y / ZOOM) * map.width() + x / ZOOM) * 4..][..4];
                let ground = if (x / 4 + y / 4) % 2 == 0 { 200.0 } else { 150.0 };
                let a = p[3] as f64 / 255.0;
                let at = ((top + y) * w + left + x) * 4;
                for c in 0..3 {
                    sheet[at + c] = (p[c] as f64 * a + ground * (1.0 - a)).round() as u8;
                }
            }
        }
    }
    let dir = effect_table::repo("verification/B-125 pictures");
    fs::create_dir_all(&dir).unwrap();
    png_out::write_rgba(&dir.join("maps.png"), w, h, OutputDepth::Eight, &[], &sheet).unwrap();

    t.finish("B-125b_layer_map_table.md");
}
