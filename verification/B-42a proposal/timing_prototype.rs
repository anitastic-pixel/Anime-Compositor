// D-98 timing prototype, thrown away after the proposal and not the build's code: D-92's samples against D-98's lines, one thread, opaque 1920x1080.
use std::time::Instant;

const W: usize = 1920;
const H: usize = 1080;

fn px(img: &[[f32; 4]], x: i64, y: i64) -> [f32; 4] {
    if x < 0 || y < 0 || x >= W as i64 || y >= H as i64 {
        [0.0; 4]
    } else {
        img[y as usize * W + x as usize]
    }
}

fn bilinear(img: &[[f32; 4]], x: f64, y: f64) -> [f32; 4] {
    let (fx, fy) = (x - 0.5, y - 0.5);
    let (x0, y0) = (fx.floor(), fy.floor());
    let (ux, uy) = (fx - x0, fy - y0);
    let mut out = [0.0f32; 4];
    for (dy, wy) in [(0, 1.0 - uy), (1, uy)] {
        for (dx, wx) in [(0, 1.0 - ux), (1, ux)] {
            let w = (wx * wy) as f32;
            if w == 0.0 {
                continue;
            }
            let p = px(img, x0 as i64 + dx, y0 as i64 + dy);
            for i in 0..4 {
                out[i] += p[i] * w;
            }
        }
    }
    out
}

fn d92(img: &[[f32; 4]], dir: f64, len: f64) -> Vec<[f32; 4]> {
    let g = (len / 2.0).ceil() as i64;
    let (ow, oh) = (W as i64 + 2 * g, H as i64 + 2 * g);
    let n = len.ceil() as usize + 1;
    let a = dir.to_radians();
    let u = (a.sin(), -a.cos());
    let mut out = vec![[0.0f32; 4]; (ow * oh) as usize];
    for y in 0..oh {
        let row = &mut out[(y * ow) as usize..((y + 1) * ow) as usize];
        for k in 0..n {
            let t = -len / 2.0 + k as f64 * len / (n - 1) as f64;
            for x in 0..ow {
                let s = bilinear(img, (x - g) as f64 + 0.5 + t * u.0, (y - g) as f64 + 0.5 + t * u.1);
                for i in 0..4 {
                    row[x as usize][i] += s[i];
                }
            }
        }
        for p in row.iter_mut() {
            for v in p.iter_mut() {
                *v /= n as f32;
            }
        }
    }
    out
}

fn tent_integral(a: f64, b: f64) -> f64 {
    let up = |v: f64| {
        let v = v.clamp(-1.0, 1.0);
        if v < 0.0 { (1.0 + v).powi(2) / 2.0 } else { 1.0 - (1.0 - v).powi(2) / 2.0 }
    };
    up(b) - up(a)
}

// D-98's lines, mostly across or mostly down; 30 degrees is mostly down.
fn d98(img: &[[f32; 4]], dir: f64, len: f64) -> Vec<[f32; 4]> {
    let g = (len / 2.0).ceil() as i64;
    let (ow, oh) = (W as i64 + 2 * g, H as i64 + 2 * g);
    let a = dir.to_radians();
    let u = (a.sin(), -a.cos());
    // Mostly down at 30 degrees: exchange across and down.
    let (major, minor) = if u.0.abs() >= u.1.abs() { (u.0, u.1) } else { (u.1, u.0) };
    let down = u.0.abs() < u.1.abs();
    let (lw, lh) = if down { (oh, ow) } else { (ow, oh) }; // along, across the lines
    let s = minor / major;
    let d = len / len.ceil();
    let h = major.abs() * (len + d) / 2.0;
    let reach = (h + 1.0).ceil() as i64;
    let w: Vec<f64> = (-reach..=reach).map(|j| tent_integral(j as f64 - h, j as f64 + h) / (2.0 * h)).collect();
    // Columns weighing exactly 1 / 2H make the running sum; the rest are the ends.
    let full = 1.0 / (2.0 * h);
    let inner = (-reach..=reach).filter(|&j| (w[(j + reach) as usize] - full).abs() < 1e-15).map(|j| j.abs()).max().unwrap_or(-1);
    let ends: Vec<(i64, f64)> = (-reach..=reach).filter(|j| j.abs() > inner).map(|j| (j, w[(j + reach) as usize])).collect();
    let mut out = vec![[0.0f32; 4]; (ow * oh) as usize];
    let read = |along: i64, across: f64| -> [f32; 4] {
        // The layer sample at column `along` centre, height `across`, in output coordinates.
        let (x, y) = if down { (across, along as f64 + 0.5) } else { (along as f64 + 0.5, across) };
        bilinear(img, x - g as f64, y - g as f64)
    };
    let smin = (0.0f64).min(s * (lw - 1) as f64);
    let smax = (0.0f64).max(s * (lw - 1) as f64);
    let (k0, k1) = ((-smax).floor() as i64 - 1, (lh as f64 - smin).ceil() as i64 + 1);
    let span = (lw + 2 * reach) as usize;
    let mut v = vec![[0.0f32; 4]; span];
    let mut pre = vec![[0.0f64; 4]; span + 1];
    for k in k0..=k1 {
        // The line k: height k + 0.5 + X s at column X.
        for (i, X) in (-reach..lw + reach).enumerate() {
            v[i] = read(X, k as f64 + 0.5 + X as f64 * s);
            for c in 0..4 {
                pre[i + 1][c] = pre[i][c] + v[i][c] as f64;
            }
        }
        for x in 0..lw {
            let i = (x + reach) as usize;
            let mut r = [0.0f64; 4];
            for c in 0..4 {
                r[c] = (pre[(i as i64 + inner + 1) as usize][c] - pre[(i as i64 - inner) as usize][c]) * full;
            }
            for &(j, wj) in &ends {
                let q = v[(i as i64 + j) as usize];
                for c in 0..4 {
                    r[c] += q[c] as f64 * wj;
                }
            }
            // This line is the lower of the pair for the pixel at ceil(k + x s), the upper for
            // the one above it.
            let yl = (k as f64 + x as f64 * s).ceil() as i64;
            for (y, lo) in [(yl, true), (yl - 1, false)] {
                if y < 0 || y >= lh {
                    continue;
                }
                let f = (y as f64 - x as f64 * s) - (k - if lo { 0 } else { 1 }) as f64;
                let wb = if lo { 1.0 - f } else { f };
                let o = if down { (x * ow + y) as usize } else { (y * ow + x) as usize };
                for c in 0..4 {
                    out[o][c] += (r[c] * wb) as f32;
                }
            }
        }
    }
    out
}

fn main() {
    let img: Vec<[f32; 4]> = (0..W * H)
        .map(|i| {
            let (x, y) = ((i % W) as f32, (i / W) as f32);
            [(x * 0.01).sin() * 0.4 + 0.5, (y * 0.013).cos() * 0.4 + 0.5, ((x + y) * 0.007).sin() * 0.4 + 0.5, 1.0]
        })
        .collect();
    for len in [10.0, 100.0] {
        for (name, f) in [("D-92", d92 as fn(&[[f32; 4]], f64, f64) -> Vec<[f32; 4]>), ("D-98", d98)] {
            let mut times = vec![];
            let mut sum = 0.0f64;
            for _ in 0..5 {
                let t = Instant::now();
                let o = f(&img, 30.0, len);
                times.push(t.elapsed().as_secs_f64() * 1000.0);
                sum = o.iter().map(|p| p[0] as f64).sum();
            }
            times.sort_by(|a, b| a.partial_cmp(b).unwrap());
            println!("{name} direction 30 length {len}: median {:.1} ms (checksum {sum:.3})", times[2]);
        }
    }
}
