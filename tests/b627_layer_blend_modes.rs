//! B-627: D-748, a layer's Hard Light, Darken, Lighten, Difference, Hue, Saturation, Color and
//! Luminosity blend modes.
//!
//! Writes `verification/B-627_layer_blend_modes_table.md`.
//!
//! # Where the expected values come from
//!
//! `tools/layer_blend_modes_reference.py` works each frame from document 21's blend equation and
//! the textbook formulas on encoded colours (`Fixtures/layer_blend_modes/`, FX-LBLEND-001 to
//! 009). Nothing here was captured from the program.

use serde_json::Value as J;
use std::path::PathBuf;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{BlendMode, Id};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};

const PAGE: &str = include_str!("../app/ui/index.html");

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Fixtures/layer_blend_modes")
}

fn expected() -> J {
    serde_json::from_str(
        &std::fs::read_to_string(dir().join("expected_layer_blend_modes.json")).unwrap(),
    )
    .unwrap()
}

fn read(name: &str) -> String {
    std::fs::read_to_string(dir().join(name)).unwrap()
}

#[test]
fn each_mode_draws_the_reference_frame_and_the_unknown_one_is_refused() {
    let want = expected();
    let tol = want["tolerance"].as_f64().unwrap();
    let w = want["width"].as_u64().unwrap() as usize;
    let mut table = String::from(
        "# B-627: a layer's eight new blend modes against the reference\n\n\
         Written by `tests/b627_layer_blend_modes.rs`. Each case is `Fixtures/layer_blend_modes/`'s, \
         worked by `tools/layer_blend_modes_reference.py`: a 10 by 8 frame, four backdrop strips \
         under four strips in the mode, the last at 50%. Largest difference over every channel of \
         every pixel, against the tolerance.\n\n\
         | Fixture | Mode | Pixels | Largest difference | Tolerance | Pass |\n| --- | --- | --- | --- | --- | --- |\n",
    );
    let (mut passed, mut total) = (0, 0);
    for (id, case) in want["cases"].as_object().unwrap() {
        let loaded = persist::load_str(&read(case["project"].as_str().unwrap()))
            .unwrap_or_else(|e| panic!("{id} reads: {e:?}"));
        let mut log = FrameLog::new(8);
        let f = render_frame(
            loaded.document.project(),
            &Id::new("comp-main"),
            0,
            std::path::Path::new("."),
            4,
            &mut log,
        )
        .expect("the frame draws");
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
    for (id, case) in want["refused"].as_object().unwrap() {
        let why = format!(
            "{:?}",
            persist::load_str(&read(case["project"].as_str().unwrap()))
                .err()
                .expect("an unknown mode is refused")
        );
        let reason = case["reason"].as_str().unwrap();
        let ok = why.contains(reason);
        total += 1;
        passed += ok as usize;
        table.push_str(&format!(
            "| {id} | {} (refused) | - | reason: {reason} | - | {} |\n",
            case["mode"].as_str().unwrap(),
            if ok { "yes" } else { "**NO**" }
        ));
    }
    table.push_str(&format!("\nB-627: {passed} of {total} fixtures pass.\n"));
    std::fs::write(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("verification/B-627_layer_blend_modes_table.md"),
        &table,
    )
    .unwrap();
    assert_eq!(passed, total, "see verification/B-627_layer_blend_modes_table.md");
}

/// A file from before D-748 (D-301's eight modes, in their old order on the layers) opens and
/// saves each mode as it was; a file in a new mode saves it back too.
#[test]
fn old_and_new_files_keep_their_modes() {
    let mut text: J = serde_json::from_str(&read("fx_lblend_001.json")).unwrap();
    for mode in BlendMode::ALL.map(BlendMode::as_str) {
        text["compositions"][0]["layers"][4]["blend_mode"] = J::from(mode);
        let loaded = persist::load_str(&text.to_string()).expect("the file reads");
        let saved: J = serde_json::from_str(&persist::to_json(
            loaded.document.project(),
            &loaded.preserved,
        ))
        .unwrap();
        assert_eq!(saved["compositions"][0]["layers"][4]["blend_mode"], mode);
    }
}

/// The page's list is the core's, in the same order.
#[test]
fn the_page_lists_the_core_modes() {
    let from = PAGE.find("const BLENDS = [").expect("the page has BLENDS") + 16;
    let to = from + PAGE[from..].find(']').unwrap();
    let page: Vec<&str> = PAGE[from..to]
        .split(',')
        .map(|s| s.trim().trim_matches('\''))
        .collect();
    assert_eq!(page, BlendMode::ALL.map(BlendMode::as_str).to_vec());
}

/// B-748 (D-868): the card draws the eight, within 1 level of 255 of the CPU's picture.
#[test]
fn the_card_draws_them_as_the_cpu_does() {
    let Ok(mut gpu) = Gpu::new() else {
        eprintln!("no usable card; nothing to check");
        return;
    };
    for case in expected()["cases"].as_object().unwrap().values() {
        let loaded = persist::load_str(&read(case["project"].as_str().unwrap())).unwrap();
        let project = loaded.document.project();
        let mut cache = CelCache::viewer();
        let mut log = FrameLog::new(8);
        let cpu = preview::preview_frame_cached(
            project, &Id::new("comp-main"), 0, &dir(), PreviewQuality::Full, 4, &mut log, &mut cache,
        )
        .expect("the CPU frame draws")
        .to_srgb8_straight();
        let mut log = FrameLog::new(8);
        let (card, ..) = preview::preview_frame_srgb8(
            project,
            &Id::new("comp-main"),
            0,
            &dir(),
            PreviewQuality::Full,
            4,
            &mut log,
            &mut cache,
            &mut gpu,
        )
        .expect("the frame draws");
        assert!(
            !log.finish().iter().any(|d| d.id == DiagnosticId::GpuPreviewOnCpu),
            "{}: drawn on the card",
            case["mode"]
        );
        let worst = cpu.iter().zip(&card).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
        assert!(worst <= 1, "{}: the card within 1 level of the CPU, {worst}", case["mode"]);
    }
}
