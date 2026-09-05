//! G0 spike SP-04: render a fixed sequence twice and compare decoded pixels byte for
//! byte, establishing determinism.
//!
//! Quarantined per document 06. Not production code, and deliberately not the renderer:
//! it is the smallest composite that exercises the properties document 21 line 131
//! actually demands of one.
//!
//!   "Tile results must not depend on the number of worker threads, the order of
//!    completion, or scheduling. Two renders of the same request on the same build
//!    produce identical bytes."
//!
//! So the question is not "does the same code give the same answer" - it trivially does.
//! It is whether tiling across a varying number of rayon workers changes the result, and
//! whether float accumulation plus PNG quantisation is stable. Both are tested here by
//! rendering the same frames at several thread counts and comparing every byte.
//!
//! Working space follows document 21: linear-light, premultiplied, f32.
//!
//! Usage: sp04_determinism   -> writes spike-output/sp04 and prints the fixture table

use rayon::prelude::*;
use std::fs;

const W: usize = 1920;
const H: usize = 1080;
const TILE: usize = 64;
const FRAMES: usize = 8;

/// A layer as document 19 would describe one, reduced to what compositing needs.
struct Layer {
    /// straight, linear-light RGB
    rgb: [f32; 3],
    alpha: f32,
    /// centre and radius in pixels; the shape is irrelevant, the coverage maths is not
    cx: f32,
    cy: f32,
    r: f32,
    /// 0.0 = hard binary edge (aliased), > 0 = feathered width in pixels
    feather: f32,
}

fn layers_for(frame: usize) -> Vec<Layer> {
    let t = frame as f32;
    vec![
        // Background: opaque mid-grey field, per document 22's reason for wanting one.
        Layer { rgb: [0.216, 0.216, 0.216], alpha: 1.0, cx: 0.0, cy: 0.0, r: 1.0e9, feather: 0.0 },
        // Soft antialiased edge, moving. Exercises partial coverage every frame.
        Layer { rgb: [0.9, 0.2, 0.15], alpha: 1.0, cx: 500.0 + 37.0 * t, cy: 540.0, r: 220.0, feather: 2.5 },
        // Hard aliased edge. Any interpolation shows up as values that should not exist.
        Layer { rgb: [0.05, 0.65, 0.95], alpha: 1.0, cx: 1300.0 - 23.0 * t, cy: 430.0, r: 180.0, feather: 0.0 },
        // Semi-transparent interior at 50 percent, the third alpha regime.
        Layer { rgb: [0.95, 0.85, 0.1], alpha: 0.5, cx: 900.0, cy: 700.0 + 19.0 * t, r: 260.0, feather: 1.5 },
    ]
}

/// Coverage of one pixel centre by one layer. Deterministic, no randomness, no lookup.
#[inline]
fn coverage(l: &Layer, x: usize, y: usize) -> f32 {
    let px = x as f32 + 0.5; // document 21: pixel (i,j) has centre (i+0.5, j+0.5)
    let py = y as f32 + 0.5;
    let d = ((px - l.cx) * (px - l.cx) + (py - l.cy) * (py - l.cy)).sqrt();
    if l.feather <= 0.0 {
        if d <= l.r { 1.0 } else { 0.0 }
    } else {
        ((l.r + l.feather * 0.5 - d) / l.feather).clamp(0.0, 1.0)
    }
}

/// Render one tile. Pure function of (frame, tile bounds): this is the property that
/// makes thread count irrelevant, and it is what the spike is really testing.
fn render_tile(frame: usize, x0: usize, y0: usize, x1: usize, y1: usize) -> Vec<f32> {
    let layers = layers_for(frame);
    let mut out = vec![0.0f32; (x1 - x0) * (y1 - y0) * 4];
    for y in y0..y1 {
        for x in x0..x1 {
            // Premultiplied linear-light accumulator, normal-over, back to front.
            let (mut r, mut g, mut b, mut a) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
            for l in &layers {
                let cov = coverage(l, x, y);
                if cov == 0.0 {
                    continue;
                }
                let sa = l.alpha * cov;
                let (sr, sg, sb) = (l.rgb[0] * sa, l.rgb[1] * sa, l.rgb[2] * sa);
                let inv = 1.0 - sa;
                r = sr + r * inv;
                g = sg + g * inv;
                b = sb + b * inv;
                a = sa + a * inv;
            }
            let o = ((y - y0) * (x1 - x0) + (x - x0)) * 4;
            out[o] = r;
            out[o + 1] = g;
            out[o + 2] = b;
            out[o + 3] = a;
        }
    }
    out
}

/// Full frame, tiled, across whatever rayon pool is current.
fn render_frame(frame: usize) -> Vec<f32> {
    let tiles_x = W.div_ceil(TILE);
    let tiles_y = H.div_ceil(TILE);
    let tiles: Vec<(usize, usize)> = (0..tiles_y)
        .flat_map(|ty| (0..tiles_x).map(move |tx| (tx, ty)))
        .collect();

    let rendered: Vec<(usize, usize, Vec<f32>)> = tiles
        .par_iter()
        .map(|&(tx, ty)| {
            let x0 = tx * TILE;
            let y0 = ty * TILE;
            let x1 = (x0 + TILE).min(W);
            let y1 = (y0 + TILE).min(H);
            (x0, y0, render_tile(frame, x0, y0, x1, y1))
        })
        .collect();

    // Composition into the full buffer is by tile origin, never by completion order.
    let mut buf = vec![0.0f32; W * H * 4];
    for (x0, y0, tile) in rendered {
        let tw = (x0 + TILE).min(W) - x0;
        let th = (y0 + TILE).min(H) - y0;
        for y in 0..th {
            let src = y * tw * 4;
            let dst = ((y0 + y) * W + x0) * 4;
            buf[dst..dst + tw * 4].copy_from_slice(&tile[src..src + tw * 4]);
        }
    }
    buf
}

/// Document 21: convert linear working RGB to the output encoding, write straight alpha.
fn encode_srgb8(buf: &[f32]) -> Vec<u8> {
    let mut out = vec![0u8; W * H * 4];
    for i in 0..W * H {
        let a = buf[i * 4 + 3];
        for c in 0..3 {
            // unpremultiply, then sRGB transfer, then quantise
            let lin = if a > 0.0 { buf[i * 4 + c] / a } else { 0.0 };
            let lin = lin.clamp(0.0, 1.0);
            let s = if lin <= 0.003_130_8 {
                12.92 * lin
            } else {
                1.055 * lin.powf(1.0 / 2.4) - 0.055
            };
            out[i * 4 + c] = (s * 255.0 + 0.5) as u8;
        }
        out[i * 4 + 3] = (a.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
    }
    out
}

fn write_png(path: &std::path::Path, rgba: &[u8]) {
    let file = fs::File::create(path).unwrap();
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), W as u32, H as u32);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    let mut w = enc.write_header().unwrap();
    w.write_image_data(rgba).unwrap();
}

/// Render the whole sequence in a pool of exactly `threads` workers.
///
/// Render and encode are timed separately on purpose. Only `render_frame` is tiled and
/// parallel; `encode_srgb8` is serial and does a `powf` per channel, so a single combined
/// figure measures Amdahl's law on the encode, not tile scaling. Reporting that combined
/// number as "tile scaling" would be a false performance claim.
fn render_sequence(threads: usize) -> (Vec<Vec<u8>>, f64, f64) {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();
    pool.install(|| {
        let mut render_ms = 0.0;
        let mut encode_ms = 0.0;
        let mut out = Vec::with_capacity(FRAMES);
        for f in 0..FRAMES {
            let t = std::time::Instant::now();
            let buf = render_frame(f);
            render_ms += t.elapsed().as_secs_f64() * 1000.0;
            let t = std::time::Instant::now();
            out.push(encode_srgb8(&buf));
            encode_ms += t.elapsed().as_secs_f64() * 1000.0;
        }
        (out, render_ms, encode_ms)
    })
}

fn main() {
    let out_dir = std::env::current_dir().unwrap().join("spike-output").join("sp04");
    fs::create_dir_all(&out_dir).unwrap();

    let logical = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let thread_counts: Vec<usize> = [1usize, 2, 4, logical / 2, logical]
        .into_iter()
        .filter(|&n| n >= 1)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();

    println!("SP-04 render determinism");
    println!("resolution   : {W}x{H}, {FRAMES} frames, {TILE}x{TILE} tiles");
    println!("logical cores: {logical}");
    println!("profile      : {}", if cfg!(debug_assertions) { "debug" } else { "release" });
    println!();

    // Reference: single-threaded. Everything else must match it byte for byte.
    let t0 = std::time::Instant::now();
    let (reference, ref_render_ms, ref_encode_ms) = render_sequence(1);
    let ref_ms = t0.elapsed().as_secs_f64() * 1000.0;

    for (i, f) in reference.iter().enumerate() {
        write_png(&out_dir.join(format!("ref_{i:04}.png")), f);
    }

    println!(
        "{:<8} {:<5} {:<12} {:<12} {:<12} {:<16} {:<22} {}",
        "THREADS", "RUN", "RENDER ms", "ENCODE ms", "TOTAL ms", "RENDER SPEEDUP", "EXPECTED", "RESULT"
    );
    println!("{}", "-".repeat(126));

    let mut all_identical = true;
    let mut timings: Vec<(usize, f64, f64)> = Vec::new();

    for &n in &thread_counts {
        // Two runs per thread count: the second catches instability that a single run
        // would attribute to thread count rather than to run-to-run variation.
        for run in 1..=2 {
            let t = std::time::Instant::now();
            let (seq, render_ms, encode_ms) = render_sequence(n);
            let ms = t.elapsed().as_secs_f64() * 1000.0;
            if run == 1 {
                timings.push((n, render_ms, encode_ms));
            }

            let mut diff_frames = Vec::new();
            for (i, (a, b)) in reference.iter().zip(seq.iter()).enumerate() {
                if a != b {
                    let bytes = a.iter().zip(b.iter()).filter(|(x, y)| x != y).count();
                    diff_frames.push(format!("frame {i}: {bytes} bytes differ"));
                }
            }
            let ok = diff_frames.is_empty();
            all_identical &= ok;

            println!(
                "{:<8} {:<5} {:<12.1} {:<12.1} {:<12.1} {:<16} {:<22} {}",
                n,
                run,
                render_ms,
                encode_ms,
                ms,
                format!("{:.2}x", ref_render_ms / render_ms),
                "identical to 1-thread",
                if ok {
                    "PASS".to_string()
                } else {
                    format!("FAIL - {}", diff_frames.join("; "))
                }
            );

            // Keep the first divergent output so a failure can be looked at, not read about.
            if !ok && run == 1 {
                for (i, f) in seq.iter().enumerate() {
                    write_png(&out_dir.join(format!("threads{n}_{i:04}.png")), f);
                }
            }
        }
    }

    println!();
    println!("SP-04 result: {}", if all_identical { "PASS" } else { "FAIL" });
    println!("reference frames written to {}", out_dir.display());
    println!();
    println!("Scaling on this machine (first run of each thread count).");
    println!("RENDER is the tiled parallel stage, the only part ADR-011 is about.");
    println!("ENCODE (sRGB transfer, one powf per channel) is serial in this spike and is");
    println!("NOT evidence about the renderer; it is reported so the total is not mistaken");
    println!("for tile scaling.");
    println!();
    println!("  {:<8} {:<12} {:<16} {:<12}", "THREADS", "RENDER ms", "RENDER SPEEDUP", "ENCODE ms");
    for (n, r, e) in &timings {
        println!("  {n:<8} {r:<12.1} {:<16} {e:<12.1}", format!("{:.2}x", ref_render_ms / r));
    }
    println!();
    println!("1-thread baseline: render {ref_render_ms:.1} ms, encode {ref_encode_ms:.1} ms, total {ref_ms:.1} ms");
    println!();
    println!("NOT RUN: determinism across different machines, CPUs or compiler versions.");
    println!("         This spike establishes determinism on one build on one machine,");
    println!("         which is what document 21 requires. Cross-machine reproducibility");
    println!("         is document 29's question and is not answered here.");

    std::process::exit(if all_identical { 0 } else { 1 });
}
