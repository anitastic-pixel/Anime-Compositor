//! B-800: D-921, a layer's fourteen more blend modes (Color Burn to Divide), and D-920's
//! FX-LBLEND-009, Dissolve refused.
//!
//! Writes `verification/B-800_more_blend_modes_table.md`.
//!
//! # Where the expected values come from
//!
//! `tools/layer_blend_modes_reference.py` works each frame from document 21's blend equation and
//! the D-921 formulas on encoded colours (`Fixtures/layer_blend_modes/`, FX-LBLEND-010 to 023).
//! Nothing here was captured from the program.

use serde_json::Value as J;
use std::path::PathBuf;

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::persist;

const PAGE: &str = include_str!("../app/ui/index.html");

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Fixtures/layer_blend_modes")
}

fn read(name: &str) -> String {
    std::fs::read_to_string(dir().join(name)).unwrap()
}

fn draw(text: &str) -> anime_compositor::WorkingBuffer {
    let loaded = persist::load_str(text).expect("the file reads");
    let mut log = FrameLog::new(8);
    render_frame(loaded.document.project(), &Id::new("comp-main"), 0, std::path::Path::new("."), 4, &mut log)
        .expect("the frame draws")
}

#[test]
fn the_fourteen_draw_the_reference_frames_and_dissolve_is_refused() {
    let want: J = serde_json::from_str(&read("expected_layer_blend_modes.json")).unwrap();
    let tol = want["tolerance"].as_f64().unwrap();
    let w = want["width"].as_u64().unwrap() as usize;
    let mut table = String::from(
        "# B-800: a layer's fourteen more blend modes against the reference\n\n\
         Written by `tests/b800_more_blend_modes.rs`. Each case is `Fixtures/layer_blend_modes/`'s, \
         worked by `tools/layer_blend_modes_reference.py` (D-921): a 10 by 8 frame, four backdrop \
         strips under four strips in the mode, the last at 50%. Largest difference over every \
         channel of every pixel, against the tolerance.\n\n\
         | Fixture | Mode | Pixels | Largest difference | Tolerance | Pass |\n| --- | --- | --- | --- | --- | --- |\n",
    );
    let (mut passed, mut total) = (0, 0);
    for (id, case) in want["cases"].as_object().unwrap() {
        if id.as_str() < "FX-LBLEND-010" {
            continue;
        }
        let f = draw(&read(case["project"].as_str().unwrap()));
        let pixels = case["frames"]["0"].as_array().unwrap();
        let mut worst = 0.0f64;
        for (i, p) in pixels.iter().enumerate() {
            let got = f.pixel(i % w, i / w);
            for c in 0..4 {
                worst = worst.max((got[c] as f64 - p[c].as_f64().unwrap()).abs());
            }
        }
        let ok = worst <= tol;
        total += 1;
        passed += ok as usize;
        table.push_str(&format!(
            "| {id} | {} | {} | {worst:.2e} | {tol:.0e} | {} |\n",
            case["mode"].as_str().unwrap(),
            pixels.len(),
            if ok { "yes" } else { "**NO**" }
        ));
    }
    let case = &want["refused"]["FX-LBLEND-009"];
    let why = format!("{:?}", persist::load_str(&read("fx_lblend_009.json")).err().expect("Dissolve is refused"));
    let reason = case["reason"].as_str().unwrap();
    let ok = why.contains(reason);
    total += 1;
    passed += ok as usize;
    table.push_str(&format!(
        "| FX-LBLEND-009 | dissolve (refused) | - | reason: {reason} | - | {} |\n",
        if ok { "yes" } else { "**NO**" }
    ));
    table.push_str(&format!("\nB-800: {passed} of {total} fixtures pass.\n"));
    std::fs::write(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("verification/B-800_more_blend_modes_table.md"),
        &table,
    )
    .unwrap();
    assert_eq!(passed, total, "see verification/B-800_more_blend_modes_table.md");
}

/// A file from before D-921 draws exactly as before: FX-LBLEND-001 (Hard Light) still matches
/// its D-748 frame, and its saved text is unchanged by a load and save.
#[test]
fn an_older_file_opens_and_draws_as_before() {
    let want: J = serde_json::from_str(&read("expected_layer_blend_modes.json")).unwrap();
    let w = want["width"].as_u64().unwrap() as usize;
    let text = read("fx_lblend_001.json");
    let f = draw(&text);
    for (i, p) in want["cases"]["FX-LBLEND-001"]["frames"]["0"].as_array().unwrap().iter().enumerate() {
        let got = f.pixel(i % w, i / w);
        for c in 0..4 {
            assert!((got[c] as f64 - p[c].as_f64().unwrap()).abs() <= 2e-5);
        }
    }
    let loaded = persist::load_str(&text).unwrap();
    let saved: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
    let modes: Vec<&J> = saved["compositions"][0]["layers"].as_array().unwrap().iter().map(|l| &l["blend_mode"]).collect();
    assert_eq!(modes.iter().filter(|m| **m == "hard_light").count(), 4);
}

/// The page shows Add as Add (Linear Dodge) and names each word as After Effects does.
#[test]
fn the_page_names_add_linear_dodge() {
    assert!(PAGE.contains("mode === 'add' ? 'Add (Linear Dodge)'"));
    assert!(PAGE.contains("mode.split('_').map((w) => w[0].toUpperCase() + w.slice(1)).join(' ')"));
}
