//! SP-03 compositor: the smallest thing that makes a scrub latency number honest.
//!
//! Quarantined per document 06. Not production code.
//!
//! SP-03 asks for the latency of presenting a *composited* 1080p frame while scrubbing.
//! Serving a precomputed buffer would answer a different and much easier question, so
//! every scrub step composites the frame from its layers on demand, tiled across rayon
//! exactly as ADR-011 describes.
//!
//! This is a deliberate copy of the compositor in sp01_sp04_core/src/bin/sp04_determinism.rs
//! rather than a shared module. SP-04 has already been run and its PASS recorded; editing
//! that file to extract a library would invalidate the artifact the result rests on. Both
//! copies are spike code and are discarded at integration.
//!
//! Working space follows document 21: linear-light, premultiplied, f32.

use rayon::prelude::*;

pub const W: usize = 1920;
pub const H: usize = 1080;
const TILE: usize = 64;

struct Layer {
    rgb: [f32; 3],
    alpha: f32,
    cx: f32,
    cy: f32,
    r: f32,
    feather: f32,
}

fn layers_for(frame: usize) -> Vec<Layer> {
    let t = frame as f32;
    vec![
        Layer { rgb: [0.216, 0.216, 0.216], alpha: 1.0, cx: 0.0, cy: 0.0, r: 1.0e9, feather: 0.0 },
        Layer { rgb: [0.9, 0.2, 0.15], alpha: 1.0, cx: 500.0 + 37.0 * t, cy: 540.0, r: 220.0, feather: 2.5 },
        Layer { rgb: [0.05, 0.65, 0.95], alpha: 1.0, cx: 1300.0 - 23.0 * t, cy: 430.0, r: 180.0, feather: 0.0 },
        Layer { rgb: [0.95, 0.85, 0.1], alpha: 0.5, cx: 900.0, cy: 700.0 + 19.0 * t, r: 260.0, feather: 1.5 },
    ]
}

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

fn render_tile(frame: usize, x0: usize, y0: usize, x1: usize, y1: usize) -> Vec<f32> {
    let layers = layers_for(frame);
    let mut out = vec![0.0f32; (x1 - x0) * (y1 - y0) * 4];
    for y in y0..y1 {
        for x in x0..x1 {
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

    // Merged by tile origin, never by completion order.
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

fn encode_srgb8(buf: &[f32]) -> Vec<u8> {
    let mut out = vec![0u8; W * H * 4];
    for i in 0..W * H {
        let a = buf[i * 4 + 3];
        for c in 0..3 {
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

/// Composite one frame and return display-ready straight RGBA8, with the core time in ms.
/// The caller needs the split: a scrub latency that does not say how much of it was the
/// renderer cannot tell anyone what to fix.
pub fn composite_rgba8(frame: usize) -> (Vec<u8>, f64) {
    let t = std::time::Instant::now();
    let buf = render_frame(frame);
    let rgba = encode_srgb8(&buf);
    (rgba, t.elapsed().as_secs_f64() * 1000.0)
}
