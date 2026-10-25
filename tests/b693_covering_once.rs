//! B-693 (D-813): Drop Shadow and Rim Light blur the covering's alpha alone. Both used to copy
//! the drawing, set its colour to nought and blur all four channels, then read back only alpha;
//! now they blur a plane of alpha alone, in the same order, so the sums are the same to the bit.
//!
//! A change of speed only, so its expected value is the old build's own output: `b693_hashes`
//! writes the SHA-256 of every float of every result below to
//! `target/b693/hashes_<B693_LABEL>.txt`. The old build is this file compiled against the
//! commit before B-693 with `B693_LABEL=before` and `B693_REPO` set to this checkout. The
//! results: three made-up drawings (a soft disc on a gradient, a ragged noise field with holes,
//! a one-pixel line), each under 96 Drop Shadows and 72 Rim Lights spanning every setting.

use std::fs;
use std::path::PathBuf;

use anime_compositor::effects::{apply_stack, Effect, EffectInstance};
use anime_compositor::model::Id;
use anime_compositor::sha256;
use anime_compositor::WorkingBuffer;

fn repo(rel: &str) -> PathBuf {
    std::env::var("B693_REPO")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
        .join(rel)
}

fn drawing(kind: usize) -> WorkingBuffer {
    let (w, h) = [(97, 61), (64, 64), (40, 9)][kind];
    let mut buf = WorkingBuffer::transparent(w, h);
    let mut seed = 0x2545_f491u32;
    for (i, px) in buf.data_mut().chunks_exact_mut(4).enumerate() {
        let (x, y) = ((i % w) as f32, (i / w) as f32);
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        let noise = (seed >> 8) as f32 / (1u32 << 24) as f32;
        let a = match kind {
            0 => (1.0 - (((x - 48.3).powi(2) + (y - 30.1).powi(2)).sqrt() - 20.0) / 3.0).clamp(0.0, 1.0),
            1 => if noise < 0.3 { 0.0 } else { noise },
            _ => if y == 4.0 && x > 3.0 { 1.0 } else { 0.0 },
        };
        let (r, g, b) = (x / w as f32, y / h as f32, noise);
        px.copy_from_slice(&[r * a, g * a, b * a, a]);
    }
    buf
}

fn effects() -> Vec<Effect> {
    let mut out = Vec::new();
    for (opacity, color) in [(100.0, "#000000"), (37.5, "#3080ff")] {
        for direction in [0.0, 37.0, 135.0, 300.0] {
            for distance in [0.0, 3.3, 12.0] {
                for softness in [0.0, 7.5] {
                    for only in ["off", "on"] {
                        out.push(Effect::DropShadow {
                            color: color.into(),
                            opacity,
                            direction,
                            distance,
                            softness: if distance == 12.0 { softness * 8.0 } else { softness },
                            shadow_only: only.into(),
                        });
                    }
                }
            }
        }
    }
    for direction in [0.0, 45.0, 200.0] {
        for width in [0.0, 1.5, 4.0] {
            for softness in [0.0, 3.0] {
                for (intensity, blend) in [(100.0, "normal"), (60.0, "add"), (100.0, "screen"), (80.0, "normal")] {
                    out.push(Effect::RimLight {
                        color: "#ffe0a0".into(),
                        direction,
                        width,
                        softness,
                        intensity,
                        blend: blend.into(),
                    });
                }
            }
        }
    }
    out
}

fn result(kind: usize, effect: &Effect) -> (String, String) {
    let mut buf = drawing(kind);
    let offset = apply_stack(&mut buf, &[EffectInstance::new(Id::new("fx"), effect.clone())], |_, _, why| {
        panic!("{effect:?} bypassed: {why:?}")
    });
    let bytes: Vec<u8> = buf.data().iter().flat_map(|v| v.to_le_bytes()).collect();
    (
        format!("drawing {kind} {effect:?} offset {offset:?} size {}x{}", buf.width(), buf.height()),
        sha256::hex(&bytes),
    )
}

fn label() -> String {
    std::env::var("B693_LABEL").unwrap_or_else(|_| "after".to_string())
}

#[test]
#[ignore = "B-693: run by name, once on the old build and once on this one"]
fn b693_hashes() {
    let mut lines = Vec::new();
    for kind in 0..3 {
        for effect in effects() {
            let (what, sha) = result(kind, &effect);
            lines.push(format!("{what}\t{sha}"));
        }
    }
    let dir = repo("target/b693");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join(format!("hashes_{}.txt", label())), lines.join("\n") + "\n").unwrap();
    eprintln!("{} results", lines.len());
}

/// One Drop Shadow (softness 40, distance 12) and one Rim Light (width 4, softness 3) on a
/// 1920 by 1080 soft disc, the median of 5 of each in ms, appended to
/// `target/b693/timing_<label>.txt`; run alternately with the old build.
#[test]
#[ignore = "B-693: a measurement, run by name"]
fn b693_timing() {
    let (w, h) = (1920, 1080);
    let mut disc = WorkingBuffer::transparent(w, h);
    for (i, px) in disc.data_mut().chunks_exact_mut(4).enumerate() {
        let (x, y) = ((i % w) as f32, (i / w) as f32);
        let a = (1.0 - (((x - 960.0).powi(2) + (y - 540.0).powi(2)).sqrt() - 400.0) / 3.0).clamp(0.0, 1.0);
        px.copy_from_slice(&[0.8 * a, 0.5 * a, 0.2 * a, a]);
    }
    let shadow = Effect::DropShadow {
        color: "#000000".into(),
        opacity: 50.0,
        direction: 135.0,
        distance: 12.0,
        softness: 40.0,
        shadow_only: "off".into(),
    };
    let rim = Effect::RimLight {
        color: "#ffffff".into(),
        direction: 45.0,
        width: 4.0,
        softness: 3.0,
        intensity: 100.0,
        blend: "normal".into(),
    };
    let mut row = String::new();
    for effect in [shadow, rim] {
        let mut t: Vec<f64> = (0..5)
            .map(|_| {
                let mut buf = disc.clone();
                let start = std::time::Instant::now();
                apply_stack(&mut buf, &[EffectInstance::new(Id::new("fx"), effect.clone())], |_, _, _| {});
                start.elapsed().as_secs_f64() * 1000.0
            })
            .collect();
        t.sort_by(f64::total_cmp);
        row.push_str(&format!("{:.1}\t", t[2]));
    }
    let dir = repo("target/b693");
    fs::create_dir_all(&dir).unwrap();
    let file = dir.join(format!("timing_{}.txt", label()));
    let old = fs::read_to_string(&file).unwrap_or_default();
    fs::write(&file, format!("{old}{}\n", row.trim_end())).unwrap();
    eprintln!("{} {row}", label());
}

// ---- B-693's own checks from here down; the old build's copy of this file is cut here.

/// Every result is the old build's to the bit: `verification/B-693_old_build_hashes.txt` is
/// `b693_hashes` run on the commit before B-693 (c4b0696c).
#[test]
fn b693_a_every_shadow_and_rim_is_the_old_builds() {
    let old = fs::read_to_string(repo("verification/B-693_old_build_hashes.txt")).unwrap();
    let mut old = old.lines();
    let mut n = 0;
    for kind in 0..3 {
        for effect in effects() {
            let (what, sha) = result(kind, &effect);
            assert_eq!(old.next(), Some(format!("{what}\t{sha}").as_str()), "{what}");
            n += 1;
        }
    }
    assert_eq!(old.next(), None);
    assert_eq!(n, 504);
}
