//! D-99's pictures and timings: a drawing's effects run at draft size in Draft preview, against
//! the rule before it, which ran them at full size and then shrank the result.
//!
//! "Before" is built the way Draft preview built a frame before D-99: the full-size frame plan,
//! shrunk by `scale_plan`, rendered. That is exact for a composition without nested ones, which
//! the reference shot is. "After" is `preview_frame` at Draft, which is what the viewer shows now.
//!
//! Timings are reported and never asserted (document 12). Each is the median of five renders
//! with no cache, so each includes reading the drawings.
//!
//! It writes `verification/D-99 proposal/`: one picture per effect and `measurements.md`.

use std::fmt::Write as _;
use std::fs;
use std::time::Instant;

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectInstance};
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::png_out;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::render;
use anime_compositor::OutputDepth;

mod common;
use common::repo;

const COMP: &str = "comp-reference-shot";
/// The whole-frame background, and the small ball in front of it.
const LAYERS: [(&str, &str); 2] = [("layer-1", "background"), ("layer-2", "ball")];
const FRAME: i32 = 100;

fn before(project: &Project) -> Vec<u8> {
    let plan = compose::plan_frame_at(
        project,
        &Id::new(COMP),
        FRAME,
        &repo("Fixtures/reference_shot"),
        PreviewQuality::Full,
        &mut FrameLog::new(8),
        &mut CelCache::none(),
    )
    .expect("frame 100 is in the shot");
    let plan = preview::scale_plan(plan, PreviewQuality::Draft);
    render::render(&plan, DEFAULT_TILE_SIZE).to_srgb8_straight()
}

fn after(project: &Project) -> Vec<u8> {
    preview::preview_frame(
        project,
        &Id::new(COMP),
        FRAME,
        &repo("Fixtures/reference_shot"),
        PreviewQuality::Draft,
        DEFAULT_TILE_SIZE,
        &mut FrameLog::new(8),
    )
    .expect("frame 100 is in the shot")
    .to_srgb8_straight()
}

fn median_ms(f: impl Fn() -> Vec<u8>) -> (Vec<u8>, f64) {
    let mut out = Vec::new();
    let mut ms: Vec<f64> = (0..5)
        .map(|_| {
            let at = Instant::now();
            out = f();
            at.elapsed().as_secs_f64() * 1000.0
        })
        .collect();
    ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (out, ms[2])
}

/// Straight sRGB over the grey the viewer shows behind a transparent frame.
fn over_grey(p: &[u8]) -> [u8; 3] {
    let a = p[3] as f64 / 255.0;
    std::array::from_fn(|i| (p[i] as f64 * a + 128.0 * (1.0 - a)).round() as u8)
}

#[test]
#[ignore = "D-99: pictures and timings, run deliberately with --release --ignored"]
fn d99_draft_effects() {
    let loaded = persist::load(&repo("verification/B-08a_project.json"))
        .unwrap_or_else(|d| panic!("the reference shot: {}", d.message));
    let colors = vec!["#000000".to_string()];
    let cases: Vec<(&str, &str, Effect)> = vec![
        (
            "gaussian_blur",
            "Gaussian Blur, sigma 8",
            Effect::GaussianBlur { sigma_px: 8.0 },
        ),
        (
            "glow",
            "Glow, bright parts over 40, radius 40, intensity 2",
            Effect::Glow {
                based_on: "bright".into(),
                threshold: 40.0,
                colors: Vec::new(),
                tolerance: 0.0,
                radius: 40.0,
                intensity: 2.0,
                operation: "add".into(),
                tint: String::new(),
            },
        ),
        (
            "directional_blur",
            "Directional Blur, direction 30, length 40",
            Effect::DirectionalBlur {
                direction: 30.0,
                length: 40.0,
            },
        ),
        (
            "bloom_star",
            "Bloom, radius 30, star streaks of 60 at angle 20",
            Effect::Bloom {
                threshold: 40.0,
                radius: 30.0,
                intensity: 2.0,
                streaks: "star".into(),
                length: 60.0,
                angle: 20.0,
            },
        ),
        (
            "radial_blur",
            "Radial Blur, zoom 20 from the centre",
            Effect::RadialBlur {
                kind: "zoom".into(),
                amount: 20.0,
                center: [50.0, 50.0],
            },
        ),
        (
            "line_width",
            "Line Width, 4 thicker, black lines",
            Effect::LineWidth {
                width: 4.0,
                based_on: "colors".into(),
                colors,
                tolerance: 40.0,
            },
        ),
    ];

    let dir = repo("verification/D-99 proposal");
    fs::create_dir_all(&dir).unwrap();
    let (_, plain_ms) = median_ms(|| after(loaded.document.project()));
    let mut s = format!(
        "The same frame with no effect at all takes {plain_ms:.1} ms.\n\n\
         | Layer | Effect | Largest | Pixels moving more than 3 | Before, ms | After, ms | Times faster |\n\
         |---|---|---|---|---|---|---|\n",
    );
    for (layer, what) in LAYERS {
        for (file, name, effect) in cases.clone() {
            let file = format!("{what}_{file}");
            let mut doc = Document::new(loaded.document.project().clone());
            doc.apply(Command::AddEffect {
                composition: Id::new(COMP),
                layer_id: Id::new(layer),
                effect: EffectInstance::new(Id::new("fx"), effect),
                index: None,
            })
            .unwrap_or_else(|d| panic!("{name}: {}", d.message));
            let project = doc.project();
            let (was, was_ms) = median_ms(|| before(project));
            let (now, now_ms) = median_ms(|| after(project));
            assert_eq!(
                was.len(),
                now.len(),
                "{name}: the two frames are the same size"
            );

            let (w, h) = PreviewQuality::Draft.extent(1920, 1080);
            let (mut largest, mut moved) = (0u8, 0usize);
            // Three panels side by side, each doubled so single pixels can be seen.
            let (pw, ph) = (w * 3 * 2, h * 2);
            let mut panel = vec![0u8; pw * ph * 4];
            for y in 0..h {
                for x in 0..w {
                    let i = (y * w + x) * 4;
                    let (a, b) = (over_grey(&was[i..i + 4]), over_grey(&now[i..i + 4]));
                    let d = (0..3).map(|c| a[c].abs_diff(b[c])).max().unwrap();
                    largest = largest.max(d);
                    moved += (d > 3) as usize;
                    let diff: [u8; 3] =
                        std::array::from_fn(|c| a[c].abs_diff(b[c]).saturating_mul(4));
                    for (k, rgb) in [a, b, diff].into_iter().enumerate() {
                        for (dy, dx) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                            let o = ((y * 2 + dy) * pw + (k * w + x) * 2 + dx) * 4;
                            panel[o..o + 3].copy_from_slice(&rgb);
                            panel[o + 3] = 255;
                        }
                    }
                }
            }
            png_out::write_rgba(
                &dir.join(format!("{file}.png")),
                pw,
                ph,
                OutputDepth::Eight,
                &[(
                    "Source",
                    format!(
                    "{name} on {layer}, frame {FRAME} of verification/B-08a_project.json, Draft: \
                     before D-99, after D-99, difference x4. By tests/d99_draft_effects.rs"
                ),
                )],
                &panel,
            )
            .expect("write the picture");
            let _ = writeln!(
                s,
                "| {what} | {name} | {largest} | {:.1}% | {was_ms:.1} | {now_ms:.1} | {:.1}x |",
                moved as f64 * 100.0 / (w * h) as f64,
                was_ms / now_ms
            );
        }
    }
    fs::write(dir.join("measurements_table.md"), s).expect("write the table");
}
