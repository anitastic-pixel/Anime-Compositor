//! Audit P9 (2026-10-14): the card keeps Curl Noise's field between frames. The field (which way
//! each flow line runs, and the grey it starts from) comes from Curl Noise's own noise, its
//! settings and the time, never from the picture, so the card draws it again only when those
//! change. A Source This Layer field is the picture's own, and is drawn every time.
//!
//! Each case draws a first frame, then a second whose picture underneath has changed (a Levels
//! before the Curl Noise is keyed, or set differently), and counts the passes the card ran for the
//! second. The same second frame is then drawn on a card that has forgotten everything: where
//! the field is kept, the first drawing runs exactly one pass fewer (the three layers are the same
//! size with the same settings, so they share one field), and where it must be drawn again, the
//! same number. Both drawings must be the very same picture, every byte, and within 1 level of
//! 255 of the processor's (D-165), or no further from it than the card was before this change.
//!
//! Writes `verification/P9_curl_field_table.md`.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use serde_json::{json, Value as J};

mod common;
use common::repo;

/// The fields one frame needs: the three layers are the same size with the same Curl Noise, so
/// they share one.
const FIELDS: u64 = 1;

/// Source This Layer: the flow lines follow the picture's own lightness, unblurred, and a line
/// through one of its saddles goes one way or the other on a hair (D-583); after the Levels the
/// card's picture differs from the processor's by such hairs. D-749 (B-461's This Layer as
/// added): such pixels may part, but at most 1 in 200 of a Full frame's. Before D-749 the row
/// held the card to 8 levels on 5024 pixels (abc457d2, 2026-10-14).
const SADDLES: usize = 1920 * 1080 / 200;

fn keyed(a: f64, b: f64) -> J {
    json!({"base": a, "keyframes": [{"frame": 0, "value": a, "interp": "linear"}, {"frame": 239, "value": b, "interp": "linear"}]})
}

/// The reference shot with a Levels (gamma `gamma`) then a Curl Noise (as added, changed by `curl`)
/// on its first three layers.
fn shot(gamma: J, curl: J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json"))
        .expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let mut all = json!({"source": "internal", "speed": 10, "direction": 0, "size": 100, "offset": [0, 0], "evolution": 0,
        "turbulence_speed": 20, "swirl": 45, "density": 0, "smoothness": 50, "vertical_bias": 50, "sample_count": 12, "sample_radius": 30,
        "flow_softness": 20, "edge_definition": 50, "flow_falloff": 0, "view": "final_render", "contrast": 100, "brightness": 0,
        "clip_hdr_results": "on", "channel": "rgb"});
    for (k, v) in curl.as_object().unwrap() {
        all[k] = v.clone();
    }
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        j["compositions"][0]["layers"][i]["effects"] = json!([
            {"instance_id": format!("p9-l{id}"), "type_id": "core.levels", "enabled": true,
             "parameters": {"input_black": 20, "input_white": 230, "gamma": gamma, "output_black": 10, "output_white": 245}},
            {"instance_id": format!("p9-c{id}"), "type_id": "core.curl_noise", "enabled": true, "parameters": all},
        ]);
    }
    persist::load_str(&j.to_string())
        .unwrap_or_else(|d| panic!("the reference shot: {}", d.message))
        .document
        .project()
        .clone()
}

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        let d = p
            .iter()
            .zip(q)
            .map(|(x, y)| x.abs_diff(*y))
            .max()
            .unwrap_or(0);
        largest = largest.max(d);
        pixels += (d > 0) as usize;
    }
    (largest, pixels)
}

/// The card's picture of `frame`, the passes it ran for it, and whether it refused the frame.
fn card(
    gpu: &mut Gpu,
    project: &Project,
    comp: &Id,
    root: &Path,
    frame: i32,
    quality: PreviewQuality,
) -> (Vec<u8>, u64, bool) {
    let mut log = FrameLog::new(3);
    let before = gpu.dispatched();
    let (g, ..) = preview::preview_frame_srgb8(
        project,
        comp,
        frame,
        root,
        quality,
        DEFAULT_TILE_SIZE,
        &mut log,
        &mut CelCache::viewer(),
        gpu,
    )
    .unwrap_or_else(|d| panic!("frame {frame} on the GPU: {}", d.message));
    let refused = log
        .finish()
        .iter()
        .any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str());
    (g, gpu.dispatched() - before, refused)
}

#[test]
fn p9_curl_field() {
    let out = repo("verification/P9_curl_field_table.md");
    let mut gpu = match Gpu::new() {
        Ok(gpu) => gpu,
        Err(why) => {
            fs::write(&out, format!("# Audit P9: Curl Noise's field kept\n\n**NOT RUN.** No usable card: {why}\n\nNo check in this table was run, so none of them passes.\n"))
                .expect("write the P9 table");
            return;
        }
    };
    let comp = Id::new("comp-reference-shot");
    let root = repo("Fixtures/reference_shot");
    let still = json!({"speed": 0, "turbulence_speed": 0});
    // (case, first project and frame, second project and frame, whether the field is kept)
    let cases: Vec<(&str, (Project, i32), (Project, i32), bool)> = vec![
        (
            "Levels keyed, so the picture changes; Curl Noise still (Speed 0, Turbulence Speed 0); frame 0 then frame 100",
            (shot(keyed(1.0, 2.0), still.clone()), 0),
            (shot(keyed(1.0, 2.0), still.clone()), 100),
            true,
        ),
        (
            "Levels changed from gamma 1.2 to 1.5; Curl Noise as added; frame 50 both times",
            (shot(json!(1.2), json!({})), 50),
            (shot(json!(1.5), json!({})), 50),
            true,
        ),
        (
            "Levels keyed; Curl Noise moving with time (Speed 10, Turbulence Speed 20, as added); frame 0 then frame 100",
            (shot(keyed(1.0, 2.0), json!({})), 0),
            (shot(keyed(1.0, 2.0), json!({})), 100),
            false,
        ),
        (
            "Levels changed from gamma 1.2 to 1.5 and Swirl from 45 to 90; frame 50 both times",
            (shot(json!(1.2), json!({})), 50),
            (shot(json!(1.5), json!({"swirl": 90})), 50),
            false,
        ),
        (
            "Levels changed from gamma 1.2 to 1.5 and Size from 100 to 60; frame 50 both times",
            (shot(json!(1.2), json!({})), 50),
            (shot(json!(1.5), json!({"size": 60})), 50),
            false,
        ),
        (
            "Source This Layer (the field is the picture's own); Levels keyed; Curl Noise still; frame 0 then frame 100",
            (shot(keyed(1.0, 2.0), json!({"source": "this_layer", "speed": 0, "turbulence_speed": 0})), 0),
            (shot(keyed(1.0, 2.0), json!({"source": "this_layer", "speed": 0, "turbulence_speed": 0})), 100),
            false,
        ),
    ];
    let (mut rows, mut checks, mut passed) = (String::new(), 0, 0);
    for (case, (p0, f0), (p1, f1), kept) in &cases {
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            gpu.forget();
            let (_, _, r0) = card(&mut gpu, p0, &comp, &root, *f0, quality);
            let (g, passes, r1) = card(&mut gpu, p1, &comp, &root, *f1, quality);
            gpu.forget();
            let (fresh, fresh_passes, r2) = card(&mut gpu, p1, &comp, &root, *f1, quality);
            let mut log = FrameLog::new(3);
            let c = preview::preview_frame_cached(
                p1,
                &comp,
                *f1,
                &root,
                quality,
                DEFAULT_TILE_SIZE,
                &mut log,
                &mut CelCache::viewer(),
            )
            .unwrap_or_else(|d| panic!("{case} on the CPU: {}", d.message))
            .to_srgb8_straight();
            let same = distance(&g, &fresh);
            let cpu = distance(&g, &c);
            let saved = fresh_passes.saturating_sub(passes);
            let expected = if *kept { FIELDS } else { 0 };
            let refused = r0 || r1 || r2;
            let pass = !refused
                && passes + saved == fresh_passes
                && saved == expected
                && same == (0, 0)
                && (cpu.0 <= 1 || case.starts_with("Source This Layer") && cpu.1 <= SADDLES);
            checks += 1;
            passed += pass as usize;
            let _ = writeln!(
                rows,
                "| {case} | {} | {} | {passes} | {fresh_passes} | {} | {}, {} pixels | {}, {} pixels | {} |",
                quality.label(),
                if *kept { "kept (1 pass fewer)" } else { "drawn again (no fewer)" },
                if passes + saved != fresh_passes { "more than a fresh card".to_string() } else if saved == FIELDS { "kept".into() } else { format!("{saved} fewer") },
                same.0,
                same.1,
                cpu.0,
                cpu.1,
                match (refused, pass) {
                    (true, _) => "FAIL: the card refused the frame",
                    (false, true) => "PASS",
                    (false, false) => "**FAIL**",
                }
            );
        }
    }
    let s = format!(
        "# Audit P9: the card keeps Curl Noise's field\n\n\
         Written by `tests/p9_curl_field.rs`. The card: {}.\n\n\
         Curl Noise first works out a field (which way each flow line runs, and the grey it starts \
         from), then draws the flow lines through it. The field comes from Curl Noise's own noise, \
         its settings and the time, never from the picture, so the card now keeps it and draws it \
         again only when one of those changes. A Source This Layer field is made from the picture, \
         so it is drawn every time, as before.\n\n\
         Each case puts a Levels, then a Curl Noise, on the reference shot's first three layers, \
         draws a first frame, then a second whose picture underneath has changed. The second is then \
         drawn again on a card that has forgotten everything.\n\n\
         Before this change the card drew the field for each of the three layers, every frame \
         (13 passes for these frames). The three layers are the same size with the same settings, \
         so they now share one field (11 passes on a forgotten card).\n\n\
         **The rule:** where the field is kept, the second frame runs exactly 1 pass fewer than on \
         the forgotten card (the one shared field); where it must be drawn again, the same number. \
         Either way the second frame is **the very same picture as the forgotten card's** (0 levels, \
         0 pixels apart) and **within 1 level of 255 of the processor's**. Source This Layer is \
         the exception: its flow lines follow the picture's own lightness, and a line through one of \
         its saddles goes one way or the other on a hair after the Levels (D-583), so those rows pass \
         when no more than 1 in 200 of a Full frame's pixels (10368) part from the processor's, at any \
         level (D-749).\n\n\
         **{passed} of {checks} checks pass.**\n\n\
         | Case | Quality | The field must be | Passes run | Passes on a forgotten card | The field was | Difference from the forgotten card's picture (of 255) | Difference from the processor's (of 255) | Result |\n\
         |---|---|---|---:|---:|---|---|---|---|\n{rows}",
        gpu.about(),
    );
    fs::write(&out, s).expect("write the P9 table");
    assert_eq!(
        passed, checks,
        "Audit P9: {passed} of {checks} checks pass; see verification/P9_curl_field_table.md"
    );
}
