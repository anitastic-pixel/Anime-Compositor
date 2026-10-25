//! B-708: D-828, After Effects' Preserve Underlying Transparency, the T switch.
//!
//! Writes `verification/B-708_preserve_transparency_table.md`.
//!
//! # Where the expected values come from
//!
//! `tools/preserve_transparency_reference.py` works each frame from document 21's blend equation
//! without the layer's own term over nothing, the alpha beneath kept ("atop"), for After Effects'
//! help page's description of the switch (`Fixtures/preserve_transparency/`, FX-PRESERVE-001 to
//! 008, 020 and 021). Nothing here was captured from the program.

use serde_json::Value as J;
use std::fmt::Write as _;
use std::path::PathBuf;

use anime_compositor::cache::CelCache;
use anime_compositor::command::Command;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};

const PAGE: &str = include_str!("../app/ui/index.html");

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Fixtures/preserve_transparency")
}

fn expected() -> J {
    serde_json::from_str(
        &std::fs::read_to_string(dir().join("expected_preserve_transparency.json")).unwrap(),
    )
    .unwrap()
}

fn read(name: &str) -> String {
    std::fs::read_to_string(dir().join(name)).unwrap()
}

fn levels(a: &[u8], b: &[u8]) -> u8 {
    a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0)
}

/// Every drawn case against the reference on the CPU, every refused one refused with its reason,
/// and on the card (when there is one) the four modes it draws within 1 level of the CPU's frame
/// and on the card; the others handed to the CPU and its frame exactly.
#[test]
fn each_case_draws_the_reference_frame_and_the_refused_ones_are_refused() {
    let want = expected();
    let tol = want["tolerance"].as_f64().unwrap();
    let w = want["width"].as_u64().unwrap() as usize;
    let mut gpu = Gpu::new().ok();
    let mut table = String::from(
        "# B-708: the T switch (Preserve Underlying Transparency) against the reference\n\n\
         Written by `tests/b708_preserve_transparency.rs`. Each case is \
         `Fixtures/preserve_transparency/`'s, worked by `tools/preserve_transparency_reference.py`: \
         a 10 by 8 frame, a backdrop of strips opaque, at 50%, opaque, at 25% and clear, under \
         four strips in the mode, the last at 50%. CPU: the largest difference over every channel \
         of every pixel, linear premultiplied, against the tolerance. Card: the largest difference \
         from the CPU's frame in levels of 255, and where the frame was drawn.\n\n\
         | Fixture | Mode | T | Pixels | CPU largest difference | Tolerance | Card | Card vs CPU (levels) | Pass |\n\
         | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n",
    );
    let (mut passed, mut total) = (0, 0);
    for (id, case) in want["cases"].as_object().unwrap() {
        let loaded = persist::load_str(&read(case["project"].as_str().unwrap()))
            .unwrap_or_else(|e| panic!("{id} reads: {e:?}"));
        let project = loaded.document.project();
        let mut log = FrameLog::new(8);
        let f = render_frame(project, &Id::new("comp-main"), 0, std::path::Path::new("."), 4, &mut log)
            .expect("the frame draws");
        let pixels = case["frames"]["0"].as_array().unwrap();
        let mut worst = 0.0f64;
        for (i, p) in pixels.iter().enumerate() {
            let got = f.pixel(i % w, i / w);
            for c in 0..4 {
                worst = worst.max((got[c] as f64 - p[c].as_f64().unwrap()).abs());
            }
        }
        let mode = case["mode"].as_str().unwrap();
        let card_draws = matches!(mode, "normal" | "multiply" | "screen" | "add");
        let (card, card_ok, card_levels) = match gpu.as_mut() {
            None => ("no card".to_string(), true, "-".to_string()),
            Some(gpu) => {
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(8);
                let c = preview::preview_frame_cached(
                    project,
                    &Id::new("comp-main"),
                    0,
                    std::path::Path::new("."),
                    PreviewQuality::Full,
                    DEFAULT_TILE_SIZE,
                    &mut log,
                    &mut cache,
                )
                .expect("the CPU frame draws")
                .to_srgb8_straight();
                let mut log = FrameLog::new(8);
                let (g, ..) = preview::preview_frame_srgb8(
                    project,
                    &Id::new("comp-main"),
                    0,
                    std::path::Path::new("."),
                    PreviewQuality::Full,
                    DEFAULT_TILE_SIZE,
                    &mut log,
                    &mut cache,
                    gpu,
                )
                .expect("the card's frame draws");
                let on_cpu = log
                    .finish()
                    .iter()
                    .any(|d| d.id == DiagnosticId::GpuPreviewOnCpu);
                let d = levels(&c, &g);
                let ok = on_cpu != card_draws && if card_draws { d <= 1 } else { d == 0 };
                (if on_cpu { "CPU" } else { "GPU" }.to_string(), ok, d.to_string())
            }
        };
        let ok = worst <= tol && card_ok;
        total += 1;
        passed += ok as usize;
        let _ = writeln!(
            table,
            "| {id} | {mode} | {} | {} | {worst:.2e} | {tol:.0e} | {card} | {card_levels} | {} |",
            if case["preserve"].as_bool().unwrap() { "on" } else { "off" },
            pixels.len(),
            if ok { "yes" } else { "**NO**" }
        );
    }
    for (id, case) in want["refused"].as_object().unwrap() {
        let why = format!(
            "{:?}",
            persist::load_str(&read(case["project"].as_str().unwrap()))
                .err()
                .expect("a T switch on a layer without one is refused")
        );
        let reason = case["reason"].as_str().unwrap();
        let ok = why.contains(reason);
        total += 1;
        passed += ok as usize;
        let _ = writeln!(
            table,
            "| {id} | {} layer (refused) | on | - | reason: {reason} | - | - | - | {} |",
            case["kind"].as_str().unwrap(),
            if ok { "yes" } else { "**NO**" }
        );
    }
    let _ = writeln!(table, "\nB-708: {passed} of {total} fixtures pass.");
    std::fs::write(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("verification/B-708_preserve_transparency_table.md"),
        &table,
    )
    .unwrap();
    assert_eq!(passed, total, "see verification/B-708_preserve_transparency_table.md");
}

/// A file from before D-828 has no `preserve_transparency`: it opens with the switch off on every
/// layer, draws as it did, and saves without the key. A file with the switch saves it back.
#[test]
fn old_files_open_unchanged_and_the_switch_saves_back() {
    let old = read("fx_preserve_008.json");
    assert!(!old.contains("preserve_transparency"));
    let loaded = persist::load_str(&old).expect("an old-style file reads");
    let comp = &loaded.document.project().compositions[0];
    assert!(comp.layers_in_order().all(|l| !l.preserve_transparency));
    let saved = persist::to_json(loaded.document.project(), &loaded.preserved);
    assert!(!saved.contains("preserve_transparency"), "nothing new is written");

    let on = persist::load_str(&read("fx_preserve_001.json")).unwrap();
    let saved: J =
        serde_json::from_str(&persist::to_json(on.document.project(), &on.preserved)).unwrap();
    let layers = saved["compositions"][0]["layers"].as_array().unwrap();
    assert_eq!(
        layers.iter().filter(|l| l["preserve_transparency"] == J::Bool(true)).count(),
        4
    );
}

/// The command turns it on and off on a layer that draws, and refuses an adjustment layer.
#[test]
fn the_command_sets_it_and_refuses_an_adjustment_layer() {
    let mut loaded = persist::load_str(&read("fx_preserve_008.json")).unwrap();
    let comp = Id::new("comp-main");
    let set = |layer: &str, value| Command::SetLayerPreserveTransparency {
        composition: comp.clone(),
        layer_id: Id::new(layer),
        value,
    };
    loaded.document.apply(set("top-0", true)).expect("a solid takes the switch");
    let on = |l: &anime_compositor::persist::Loaded| {
        l.document.project().compositions[0]
            .layer(&Id::new("top-0"))
            .unwrap()
            .preserve_transparency
    };
    assert!(on(&loaded));
    loaded.document.apply(set("top-0", false)).unwrap();
    assert!(!on(&loaded));

    let mut adj = persist::load_str(
        &read("fx_preserve_020.json").replace("\"preserve_transparency\": true", "\"enabled\": true"),
    );
    // The refused file with the key taken out reads; the command then refuses the switch.
    if let Ok(adj) = adj.as_mut() {
        assert!(adj.document.apply(set("adj", true)).is_err());
    } else {
        panic!("the adjustment file without the key reads");
    }
}

/// The page draws the switch, sends the command, and names it in the column headings.
#[test]
fn the_page_has_the_switch() {
    assert!(PAGE.contains("'/layer.toggle_preserve_transparency?layer='"));
    assert!(PAGE.contains("layer.preserve_transparency"));
    assert!(PAGE.contains("<span title=\"Preserve Underlying Transparency\">T</span>"));
}
