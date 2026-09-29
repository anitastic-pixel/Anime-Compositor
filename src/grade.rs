//! The batch of ten's colour effects, each a pixel at a time on a layer's own pixels: D-111's
//! curves, D-112's levels, D-113's hue and saturation, D-114's gradient and D-119's noise; and
//! the second batch's: D-126's vignette, D-128's fractal noise (whose field D-127's turbulent
//! displace shares), D-129's gradient map and D-130's colour balance.
//!
//! This program's own methods; nothing is ported. Each effect's `tools/<name>_reference.py` is
//! the same rule worked a second way, and its `tests/b5x_<name>.rs` holds this to its numbers.

use crate::WorkingBuffer;
use rayon::prelude::*;

/// Document 21's sRGB curves, in double precision, as the reference tools write them.
pub(crate) fn to_linear(c: f64) -> f64 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

pub(crate) fn to_srgb(c: f64) -> f64 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

/// The batch's shared colour rule: a pixel that shows is taken to its straight colour through
/// the sRGB curve, 0 to 1, goes through `f(pixel index, colour)`, and the result, held inside
/// 0 to 1, comes back to linear at the pixel's own covering. A pixel that does not show is left
/// as it is.
///
/// P-21: a row at a time. A cel is painted in flat colours, so a pixel whose four numbers are
/// bit for bit the pixel's before it on the row takes that pixel's colour through the curve,
/// and, unless `placed` says `f` reads the pixel's index, its result too: the same bits,
/// without the powers.
fn grade_pixels(source: &mut WorkingBuffer, placed: bool, f: impl Fn(usize, [f64; 3]) -> [f64; 3] + Sync) {
    let w = source.width().max(1);
    source
        .data_mut()
        .par_chunks_mut(w * 4)
        .enumerate()
        .for_each(|(y, row)| {
            let mut last: Option<([u32; 4], [f64; 3], [f32; 3])> = None;
            for (x, px) in row.chunks_exact_mut(4).enumerate() {
                let a = px[3] as f64;
                if a <= 0.0 {
                    continue;
                }
                let key = [px[0].to_bits(), px[1].to_bits(), px[2].to_bits(), px[3].to_bits()];
                let e = match last {
                    Some((k, e, out)) if k == key => {
                        if !placed {
                            px[..3].copy_from_slice(&out);
                            continue;
                        }
                        e
                    }
                    _ => std::array::from_fn(|c| to_srgb((px[c] as f64 / a).clamp(0.0, 1.0))),
                };
                let out = f(y * w + x, e);
                for c in 0..3 {
                    px[c] = (to_linear(out[c].clamp(0.0, 1.0)) * a) as f32;
                }
                last = Some((key, e, [px[0], px[1], px[2]]));
            }
        });
}

/// P-22: `f` on each pixel, a row at a time, a pixel bit for bit the one before it on its row
/// given that one's result, as `grade_pixels` does. `f` reads nothing but the pixel.
fn each_pixel(source: &mut WorkingBuffer, f: impl Fn(&mut [f32]) + Sync) {
    let w = source.width().max(1);
    source.data_mut().par_chunks_mut(w * 4).for_each(|row| {
        let mut last: Option<([u32; 4], [f32; 4])> = None;
        for px in row.chunks_exact_mut(4) {
            let key = [px[0].to_bits(), px[1].to_bits(), px[2].to_bits(), px[3].to_bits()];
            match last {
                Some((k, out)) if k == key => px.copy_from_slice(&out),
                _ => {
                    f(px);
                    last = Some((key, [px[0], px[1], px[2], px[3]]));
                }
            }
        }
    });
}

/// The same, a channel at a time on the 0 to 255 scale: `f(channel, value)`.
fn grade(source: &mut WorkingBuffer, f: impl Fn(usize, f64) -> f64 + Sync) {
    grade_pixels(source, false, |_, e| std::array::from_fn(|c| f(c, e[c] * 255.0) / 255.0));
}

/// D-111's default curve, which changes nothing.
pub(crate) fn is_straight(points: &[Vec<f64>]) -> bool {
    points == [vec![0.0, 0.0], vec![255.0, 255.0]]
}

/// D-111: the natural cubic spline through `points`, which are valid: each point's in and out,
/// and the second derivative there. B-65 sends these to the card.
pub(crate) fn knots(points: &[Vec<f64>]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let xs: Vec<f64> = points.iter().map(|p| p[0]).collect();
    let ys: Vec<f64> = points.iter().map(|p| p[1]).collect();
    let n = xs.len();
    let h: Vec<f64> = (0..n - 1).map(|i| xs[i + 1] - xs[i]).collect();
    let mut m = vec![0.0; n];
    if n > 2 {
        // The tridiagonal system for the inner second derivatives, eliminated down and back up.
        let k = n - 2;
        let sub: Vec<f64> = (1..n - 1).map(|i| h[i - 1]).collect();
        let mut dia: Vec<f64> = (1..n - 1).map(|i| 2.0 * (h[i - 1] + h[i])).collect();
        let sup: Vec<f64> = (1..n - 1).map(|i| h[i]).collect();
        let mut rhs: Vec<f64> = (1..n - 1)
            .map(|i| 6.0 * ((ys[i + 1] - ys[i]) / h[i] - (ys[i] - ys[i - 1]) / h[i - 1]))
            .collect();
        for j in 1..k {
            let f = sub[j] / dia[j - 1];
            dia[j] -= f * sup[j - 1];
            rhs[j] -= f * rhs[j - 1];
        }
        for j in (0..k).rev() {
            let next = if j + 1 < k { sup[j] * m[j + 2] } else { 0.0 };
            m[j + 1] = (rhs[j] - next) / dia[j];
        }
    }
    (xs, ys, m)
}

/// D-111: the spline through `points`, flat outside its end points.
fn spline(points: &[Vec<f64>]) -> impl Fn(f64) -> f64 {
    let (xs, ys, m) = knots(points);
    let n = xs.len();
    let h: Vec<f64> = (0..n - 1).map(|i| xs[i + 1] - xs[i]).collect();
    move |x: f64| {
        if x <= xs[0] {
            return ys[0];
        }
        if x >= xs[n - 1] {
            return ys[n - 1];
        }
        let i = (0..n - 1).rev().find(|&k| xs[k] <= x).unwrap_or(0);
        let (t, hi) = (x - xs[i], h[i]);
        ys[i] + t * ((ys[i + 1] - ys[i]) / hi - hi * (2.0 * m[i] + m[i + 1]) / 6.0)
            + t * t * m[i] / 2.0
            + t.powi(3) * (m[i + 1] - m[i]) / (6.0 * hi)
    }
}

/// D-111: each channel through its own curve, then through the master. The settings are
/// already valid; all four curves straight changes nothing.
pub(crate) fn curves(source: &mut WorkingBuffer, master: &[Vec<f64>], rgb: [&[Vec<f64>]; 3]) {
    if is_straight(master) && rgb.iter().all(|c| is_straight(c)) {
        return;
    }
    let m = spline(master);
    let c = rgb.map(spline);
    grade(source, |i, x| m(c[i](x).clamp(0.0, 255.0)).clamp(0.0, 255.0));
}

/// D-112: each channel from the input range to 0..1, held there, bent by the gamma and laid on
/// the output range. An input white equal to its black is a threshold. The settings are
/// already valid and held; the defaults change nothing.
pub(crate) fn levels(source: &mut WorkingBuffer, [ib, iw, gamma, ob, ow]: [f64; 5]) {
    if [ib, iw, gamma, ob, ow] == [0.0, 255.0, 1.0, 0.0, 255.0] {
        return;
    }
    grade(source, |_, x| {
        let v = if iw == ib {
            if x >= ib { 1.0 } else { 0.0 }
        } else {
            ((x - ib) / (iw - ib)).clamp(0.0, 1.0)
        };
        ob + v.powf(1.0 / gamma) * (ow - ob)
    });
}

/// D-113: a straight colour, 0 to 1, as hue in degrees, saturation and lightness. Of equal
/// largest channels, red counts before green and green before blue.
pub(crate) fn to_hsl([r, g, b]: [f64; 3]) -> [f64; 3] {
    let (mx, mn) = (r.max(g).max(b), r.min(g).min(b));
    let c = mx - mn;
    let l = (mx + mn) / 2.0;
    if c == 0.0 {
        return [0.0, 0.0, l];
    }
    let s = c / (1.0 - (2.0 * l - 1.0).abs());
    let h = if mx == r {
        60.0 * ((g - b) / c).rem_euclid(6.0)
    } else if mx == g {
        60.0 * ((b - r) / c + 2.0)
    } else {
        60.0 * ((r - g) / c + 4.0)
    };
    [h, s, l]
}

/// D-113: hue, saturation and lightness back to a colour, by the sextant of the hue.
fn from_hsl([h, s, l]: [f64; 3]) -> [f64; 3] {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = l - c / 2.0;
    let [r, g, b] = match ((h / 60.0).floor() as i64).rem_euclid(6) {
        0 => [c, x, 0.0],
        1 => [x, c, 0.0],
        2 => [0.0, c, x],
        3 => [0.0, x, c],
        4 => [x, 0.0, c],
        _ => [c, 0.0, x],
    };
    [r + m, g + m, b + m]
}

/// D-113: the hue turned by `hue` degrees, the saturation scaled by `saturation` per cent and
/// held inside 0 to 1, and the lightness taken `lightness` per cent of the way to white, or to
/// black below 0. The settings are already valid and held; all three 0 changes nothing.
pub(crate) fn hue_saturation(source: &mut WorkingBuffer, hue: f64, saturation: f64, lightness: f64) {
    if hue == 0.0 && saturation == 0.0 && lightness == 0.0 {
        return;
    }
    grade_pixels(source, false, |_, e| {
        let [h, s, l] = to_hsl(e);
        let l = if lightness >= 0.0 {
            l + (1.0 - l) * lightness / 100.0
        } else {
            l * (1.0 + lightness / 100.0)
        };
        from_hsl([
            (h + hue).rem_euclid(360.0),
            (s * (1.0 + saturation / 100.0)).clamp(0.0, 1.0),
            l,
        ])
    });
}

/// D-114 and D-117: how a straight colour `b` takes a chosen colour `c` under `blend`, "normal",
/// "multiply", "screen" or "add"; add is not held back. D-185 adds "overlay" and
/// "soft_light", the W3C's, which only Paraffin takes.
pub(crate) fn mixer(blend: &str) -> fn(f64, f64) -> f64 {
    match blend {
        "multiply" => |b, c| b * c,
        "screen" => |b, c| 1.0 - (1.0 - b) * (1.0 - c),
        "add" => |b, c| b + c,
        "overlay" => |b, c| if b <= 0.5 { 2.0 * b * c } else { 1.0 - 2.0 * (1.0 - b) * (1.0 - c) },
        "soft_light" => |b, c| {
            if c <= 0.5 {
                b - (1.0 - 2.0 * c) * b * (1.0 - b)
            } else {
                let d = if b <= 0.25 { ((16.0 * b - 12.0) * b + 4.0) * b } else { b.sqrt() };
                b + (2.0 * c - 1.0) * (d - b)
            }
        },
        _ => |_, c| c,
    }
}

/// D-185: `color` encoded 0 to 1 washed over the figure, the pixels at least half covered, from
/// `direction` degrees clockwise from up, strongest at the figure's near edge and fading on a
/// smoothstep to nothing `spread` per cent of the way across, at up to `opacity`, 0 to 100, by
/// `blend`, on encoded values. A pixel the wash does not reach is left exactly as it is. The
/// settings are already valid.
pub(crate) fn paraffin(source: &mut WorkingBuffer, color: [f64; 3], direction: f64, spread: f64, opacity: f64, blend: &str) {
    if opacity == 0.0 {
        return;
    }
    let Some([ux, uy, near, reach]) = paraffin_span(source, direction, spread) else { return };
    let w = source.width().max(1);
    let toward = |i: usize| ((i % w) as f64 + 0.5) * ux + ((i / w) as f64 + 0.5) * uy;
    let (k, mix) = (opacity / 100.0, mixer(blend));
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let a = px[3] as f64;
        let s = ((near - toward(i)) / reach).clamp(0.0, 1.0);
        let o = (1.0 - s * s * (3.0 - 2.0 * s)) * k;
        if a <= 0.0 || o <= 0.0 {
            return;
        }
        for c in 0..3 {
            let b = to_srgb((px[c] as f64 / a).clamp(0.0, 1.0));
            px[c] = (to_linear((b + o * (mix(b, color[c]) - b)).clamp(0.0, 1.0)) * a) as f32;
        }
    });
}

/// D-185: the wash's way across `[ux, uy]`, the covered pixels' farthest place along it and how
/// far back the wash reaches, or none when spread is 0 or no pixel is covered half or more.
/// B-123's card takes the same.
pub(crate) fn paraffin_span(source: &WorkingBuffer, direction: f64, spread: f64) -> Option<[f64; 4]> {
    if spread == 0.0 {
        return None;
    }
    let w = source.width().max(1);
    let r = direction.to_radians();
    let (ux, uy) = (r.sin(), -r.cos());
    let (mut near, mut far) = (f64::NEG_INFINITY, f64::INFINITY);
    for (i, px) in source.data().chunks_exact(4).enumerate() {
        if px[3] >= 0.5 {
            let n = ((i % w) as f64 + 0.5) * ux + ((i / w) as f64 + 0.5) * uy;
            (near, far) = (near.max(n), far.min(n));
        }
    }
    (near >= far).then(|| [ux, uy, near, spread / 100.0 * (near - far + 1.0)])
}

/// D-114's settings, read once for a frame: the two points in the buffer's pixels, the two
/// colours encoded 0 to 1, and the two strengths 0 to 100.
pub(crate) struct Gradient {
    pub radial: bool,
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub colors: [[f64; 3]; 2],
    pub opacity: [f64; 2],
    pub blend: String,
}

/// D-114: a colour gradient laid over each pixel that shows, from `start` to `end`, mixed in by
/// its blend at its strength; each pixel keeps its own covering, so nothing spills past the
/// cel. The settings are already valid.
pub(crate) fn gradient(source: &mut WorkingBuffer, g: &Gradient) {
    if g.opacity == [0.0, 0.0] {
        return;
    }
    let w = source.width();
    let (sx, sy) = g.start;
    let (ex, ey) = (g.end.0 - sx, g.end.1 - sy);
    let ll = ex * ex + ey * ey;
    let mix = mixer(&g.blend);
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let a = px[3] as f64;
            if a <= 0.0 {
                return;
            }
            let (dx, dy) = ((i % w) as f64 + 0.5 - sx, (i / w) as f64 + 0.5 - sy);
            let t = if ll == 0.0 {
                1.0
            } else if g.radial {
                (dx * dx + dy * dy).sqrt() / ll.sqrt()
            } else {
                (dx * ex + dy * ey) / ll
            }
            .clamp(0.0, 1.0);
            let o = (g.opacity[0] + t * (g.opacity[1] - g.opacity[0])) / 100.0;
            for c in 0..3 {
                let [c0, c1] = [g.colors[0][c], g.colors[1][c]];
                let color = to_linear(c0 + t * (c1 - c0));
                let b = px[c] as f64 / a;
                px[c] = ((b + o * (mix(b, color) - b)) * a) as f32;
            }
        });
}

/// D-208: four colours, encoded 0 to 1, pinned to four points in the buffer's pixels, mixed at
/// each pixel that shows by (nearest distance / distance) to the power 200 / `blend`, and laid on
/// by `mode` at `opacity`, 0 to 100, as Gradient lays its colour on; each pixel keeps its own
/// covering. The settings are already valid.
pub(crate) fn four_color_gradient(
    source: &mut WorkingBuffer,
    points: [(f64, f64); 4],
    colors: [[f64; 3]; 4],
    blend: f64,
    opacity: f64,
    mode: &str,
) {
    if opacity == 0.0 {
        return;
    }
    let w = source.width();
    // Squared distances, so the power is halved.
    let (e, o, mix) = (100.0 / blend, opacity / 100.0, mixer(mode));
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let a = px[3] as f64;
            if a <= 0.0 {
                return;
            }
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let d = points.map(|(sx, sy)| (x - sx) * (x - sx) + (y - sy) * (y - sy));
            let m = d.iter().copied().fold(f64::INFINITY, f64::min);
            let wk = d.map(|d| if m == 0.0 { f64::from(u8::from(d == 0.0)) } else { (m / d).powf(e) });
            let sum: f64 = wk.iter().sum();
            for c in 0..3 {
                let g = to_linear((0..4).map(|k| wk[k] * colors[k][c]).sum::<f64>() / sum);
                let b = px[c] as f64 / a;
                px[c] = ((b + o * (mix(b, g) - b)) * a) as f32;
            }
        });
}

/// D-119: SplitMix64's finaliser, on 64-bit words.
pub(crate) fn mix(z: u64) -> u64 {
    let z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    let z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// D-119: film grain. Each channel of a pixel that shows moves by up to `amount` / 200 either
/// way through the sRGB curve, by a number in -1..1 fixed by the seed's whole part, the pixel's
/// place in the drawing's own space (its corner at `(ox, oy)` in `source` after the effects
/// above grew it), `frame` and the channel, or channel 0 for all three unless `color`. The
/// settings are already valid; amount 0 changes nothing.
pub(crate) fn noise(
    source: &mut WorkingBuffer,
    amount: f64,
    color: bool,
    seed: f64,
    frame: i32,
    (ox, oy): (usize, usize),
) {
    if amount == 0.0 {
        return;
    }
    let w = source.width();
    let (base, k) = (mix(seed.floor() as u64), amount / 200.0);
    grade_pixels(source, true, |i, e| {
        let (x, y) = ((i % w) as i64 - ox as i64, (i / w) as i64 - oy as i64);
        let h = mix(mix(mix(base ^ x as u64) ^ y as u64) ^ frame as i64 as u64);
        std::array::from_fn(|c| {
            let n = (mix(h ^ if color { c as u64 } else { 0 }) >> 11) as f64;
            e[c] + k * (n / (1u64 << 53) as f64 * 2.0 - 1.0)
        })
    });
}

/// D-119's number in -1..1 for a seed already through [`mix`] and four whole numbers.
pub(crate) fn unit(base: u64, x: i64, y: i64, f: i64, ch: u64) -> f64 {
    let h = mix(mix(mix(mix(base ^ x as u64) ^ y as u64) ^ f as u64) ^ ch);
    (h >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
}

/// D-127 and D-128's smooth value noise in -1..1: [`unit`] at the eight corners of the cell
/// round `(x, y, z)`, mixed by the smoothed place inside it.
fn value(base: u64, ch: u64, x: f64, y: f64, z: f64) -> f64 {
    let (i, j, k) = (x.floor(), y.floor(), z.floor());
    let fade = |t: f64| t * t * t * (t * (6.0 * t - 15.0) + 10.0);
    let s = [fade(x - i), fade(y - j), fade(z - k)];
    let (i, j, k) = (i as i64, j as i64, k as i64);
    let mut v = 0.0;
    for corner in 0..8 {
        let d = [corner & 1, (corner >> 1) & 1, corner >> 2];
        let w: f64 = (0..3).map(|a| if d[a] == 1 { s[a] } else { 1.0 - s[a] }).product();
        v += w * unit(base, i + d[0] as i64, j + d[1] as i64, k + d[2] as i64, ch);
    }
    v
}

/// D-127 and D-128's fractal noise: `octaves` of [`value`], each half the last's strength at
/// twice its fineness, over the sum of the strengths. Channel `ch`'s octave `o` is channel
/// `8 o + ch` of the value noise.
pub(crate) fn fractal(base: u64, ch: u64, (x, y, z): (f64, f64, f64), octaves: usize) -> f64 {
    let (mut sum, mut total, mut amp, mut fine) = (0.0, 0.0, 1.0, 1.0);
    for o in 0..octaves {
        sum += amp * value(base, 8 * o as u64 + ch, x * fine, y * fine, z * fine);
        total += amp;
        amp *= 0.5;
        fine *= 2.0;
    }
    sum / total
}

/// D-128's settings, read once for a frame: the noise's size, octaves and depth (`z`, the
/// evolution and the frame's turns), its contrast and brightness, the two colours encoded 0 to
/// 1, and how it is mixed in.
pub(crate) struct Fractal {
    pub size: f64,
    pub octaves: usize,
    pub seed: f64,
    pub z: f64,
    pub contrast: f64,
    pub brightness: f64,
    pub colors: [[f64; 3]; 2],
    pub opacity: f64,
    pub blend: String,
}

/// D-128: a cloudy pattern of two colours over each pixel that shows, fixed to the drawing's own
/// space (its corner at `(ox, oy)` in `source`), mixed in by its blend at its opacity. The
/// settings are already valid.
pub(crate) fn fractal_noise(source: &mut WorkingBuffer, f: &Fractal, (ox, oy): (usize, usize)) {
    if f.opacity == 0.0 {
        return;
    }
    let w = source.width();
    let base = mix(f.seed.floor() as u64);
    let (mixer, o) = (mixer(&f.blend), f.opacity / 100.0);
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let a = px[3] as f64;
            if a <= 0.0 {
                return;
            }
            let x = ((i % w) as f64 - ox as f64 + 0.5) / f.size;
            let y = ((i / w) as f64 - oy as f64 + 0.5) / f.size;
            let n = fractal(base, 0, (x, y, f.z), f.octaves);
            let v = (0.5 + 0.5 * n * f.contrast / 100.0 + f.brightness / 100.0).clamp(0.0, 1.0);
            for c in 0..3 {
                let [d, l] = [f.colors[0][c], f.colors[1][c]];
                let color = to_linear(d + v * (l - d));
                let b = px[c] as f64 / a;
                px[c] = ((b + o * (mixer(b, color) - b)) * a) as f32;
            }
        });
}

/// D-209: a pattern of cells over each pixel that shows, fixed to the drawing's own space (its
/// corner at `(ox, oy)` in `source`). `numbers` are the contrast, disperse, size, evolution in
/// degrees, seed and opacity; the two colours are encoded 0 to 1 and mixed in by `blend` as
/// Fractal Noise's are. The settings are already valid.
pub(crate) fn cell_pattern(
    source: &mut WorkingBuffer,
    pattern: &str,
    invert: bool,
    [contrast, disperse, size, evolution, seed, opacity]: [f64; 6],
    colors: [[f64; 3]; 2],
    blend: &str,
    (ox, oy): (usize, usize),
) {
    let (w, h) = (source.width(), source.height());
    if opacity == 0.0 || w == 0 || h == 0 {
        return;
    }
    let (base, t) = (mix(seed.floor() as u64), evolution.to_radians());
    let at = |x: usize, o: usize| (x as f64 - o as f64 + 0.5) / size;
    // Each cell's point and grey, worked once for the cells the buffer sees and two round them.
    let (m0, n0) = (at(0, ox).floor() as i64 - 2, at(0, oy).floor() as i64 - 2);
    let cols = (at(w - 1, ox).floor() as i64 + 3 - m0) as usize;
    let rows = (at(h - 1, oy).floor() as i64 + 3 - n0) as usize;
    let points: Vec<(f64, f64, f64)> = (0..cols * rows)
        .into_par_iter()
        .map(|k| {
            let (m, n) = (m0 + (k % cols) as i64, n0 + (k / cols) as i64);
            let u = [0, 1, 2, 3].map(|c| unit(base, m, n, 0, c));
            let r = disperse / 2.0 * (u[0] + 1.0) / 2.0;
            let a = std::f64::consts::PI * u[1] + if u[2] >= 0.0 { t } else { -t };
            (m as f64 + 0.5 + r * a.cos(), n as f64 + 0.5 + r * a.sin(), (u[3] + 1.0) / 2.0)
        })
        .collect();
    let (mixer, o) = (mixer(blend), opacity / 100.0);
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let a = px[3] as f64;
            if a <= 0.0 {
                return;
            }
            let (x, y) = (at(i % w, ox), at(i / w, oy));
            let (ci, cj) = ((x.floor() as i64 - 2 - m0) as usize, (y.floor() as i64 - 2 - n0) as usize);
            // The nearest and second nearest, squared, the first found keeping a tie.
            let (mut f1, mut f2, mut g) = (f64::INFINITY, f64::INFINITY, 0.0);
            for n in cj..cj + 5 {
                for &(qx, qy, grey) in &points[n * cols + ci..n * cols + ci + 5] {
                    let d = (x - qx) * (x - qx) + (y - qy) * (y - qy);
                    if d < f1 {
                        (f1, f2, g) = (d, f1, grey);
                    } else if d < f2 {
                        f2 = d;
                    }
                }
            }
            let (f1, f2) = (f1.sqrt(), f2.sqrt());
            let k = if f1 + f2 > 0.0 { 2.0 * f1 / (f1 + f2) } else { 0.0 };
            let v = match pattern {
                "bubbles" => (1.0 - k * k).sqrt(),
                "crystals" => 1.0 - k,
                "plates" => (4.0 * (1.0 - k)).min(1.0),
                _ => g,
            };
            let v = if invert { 1.0 - v } else { v };
            let v = (0.5 + (v - 0.5) * contrast / 100.0).clamp(0.0, 1.0);
            for c in 0..3 {
                let [d, l] = [colors[0][c], colors[1][c]];
                let color = to_linear(d + v * (l - d));
                let b = px[c] as f64 / a;
                px[c] = ((b + o * (mixer(b, color) - b)) * a) as f32;
            }
        });
}

/// D-129: each pixel that shows takes the ramp's colour at its lightness, shadow to midtone
/// below `midpoint` per cent and midtone to highlight above it, mixed in at `amount` per cent.
/// The colours are encoded 0 to 1; the settings are already valid.
pub(crate) fn gradient_map(source: &mut WorkingBuffer, colors: [[f64; 3]; 3], midpoint: f64, amount: f64) {
    if amount == 0.0 {
        return;
    }
    let (m, o) = (midpoint / 100.0, amount / 100.0);
    source.data_mut().par_chunks_exact_mut(4).for_each(|px| {
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        let b = [0, 1, 2].map(|c| px[c] as f64 / a);
        let t = to_srgb((0.2126 * b[0] + 0.7152 * b[1] + 0.0722 * b[2]).clamp(0.0, 1.0));
        let (lo, hi, s) = if t <= m {
            (colors[0], colors[1], t / m)
        } else {
            (colors[1], colors[2], (t - m) / (1.0 - m))
        };
        for c in 0..3 {
            let g = to_linear(lo[c] + s * (hi[c] - lo[c]));
            px[c] = ((b[c] + o * (g - b[c])) * a) as f32;
        }
    });
}

/// D-130: red, green and blue pushed by `shadows`, `midtones` and `highlights`, each -100..100,
/// through the sRGB curve, weighted by how dark or light the pixel is. The settings are already
/// valid; all nine 0 changes nothing.
pub(crate) fn color_balance(source: &mut WorkingBuffer, shadows: [f64; 3], midtones: [f64; 3], highlights: [f64; 3]) {
    if [shadows, midtones, highlights] == [[0.0; 3]; 3] {
        return;
    }
    grade_pixels(source, false, |_, e| {
        let l = 0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2];
        let ws = (1.0 - 2.0 * l).clamp(0.0, 1.0);
        let wh = (2.0 * l - 1.0).clamp(0.0, 1.0);
        let wm = 1.0 - ws - wh;
        std::array::from_fn(|c| e[c] + (ws * shadows[c] + wm * midtones[c] + wh * highlights[c]) / 200.0)
    });
}

/// D-126's settings, read once for a frame: the centre in the buffer's pixels, the oval's two
/// half-axes, where the fall begins and ends, the colour encoded 0 to 1 and the amount.
pub(crate) struct Vignette {
    pub center: (f64, f64),
    pub radii: (f64, f64),
    pub inner: f64,
    pub outer: f64,
    pub color: [f64; 3],
    pub amount: f64,
}

/// D-126: each pixel that shows mixed toward the colour by how far out it lies on the oval, 0
/// inside `inner` and all of `amount` per cent past `outer`, smoothly between.
pub(crate) fn vignette(source: &mut WorkingBuffer, v: &Vignette) {
    if v.amount == 0.0 {
        return;
    }
    let w = source.width();
    let (cx, cy) = v.center;
    let (rx, ry) = v.radii;
    let g = v.color.map(to_linear);
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let a = px[3] as f64;
            if a <= 0.0 {
                return;
            }
            let (dx, dy) = (((i % w) as f64 + 0.5 - cx) / rx, ((i / w) as f64 + 0.5 - cy) / ry);
            let d = (dx * dx + dy * dy).sqrt() / 2f64.sqrt();
            let t = if v.inner == v.outer {
                if d >= v.outer { 1.0 } else { 0.0 }
            } else {
                let t = ((d - v.inner) / (v.outer - v.inner)).clamp(0.0, 1.0);
                t * t * (3.0 - 2.0 * t)
            };
            let o = t * v.amount / 100.0;
            for c in 0..3 {
                let b = px[c] as f64 / a;
                px[c] = ((b + o * (g[c] - b)) * a) as f32;
            }
        });
}

/// D-134: `channel` (0 red, 1 green, 2 blue, 3 all three) turned toward its opposite by
/// `amount` per cent in encoded values; the channels not chosen are kept bit for bit.
pub(crate) fn invert(source: &mut WorkingBuffer, channel: usize, amount: f64) {
    if amount == 0.0 {
        return;
    }
    let t = amount / 100.0;
    each_pixel(source, |px| {
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        for c in 0..3 {
            if channel == 3 || channel == c {
                let e = to_srgb((px[c] as f64 / a).clamp(0.0, 1.0));
                px[c] = (to_linear((e + t * (1.0 - 2.0 * e)).clamp(0.0, 1.0)) * a) as f32;
            }
        }
    });
}

/// D-134's alpha: every pixel's covering turned toward its opposite, its straight colour kept
/// (black where it had none).
pub(crate) fn invert_alpha(source: &mut WorkingBuffer, amount: f64) {
    if amount == 0.0 {
        return;
    }
    let t = amount / 100.0;
    source.data_mut().par_chunks_exact_mut(4).for_each(|px| {
        let a = px[3] as f64;
        let b = if a > 0.0 { [0, 1, 2].map(|c| px[c] as f64 / a) } else { [0.0; 3] };
        let n = a + t * (1.0 - 2.0 * a);
        for c in 0..3 {
            px[c] = (b[c] * n) as f32;
        }
        px[3] = n as f32;
    });
}

/// D-135: contrast about the middle grey, then brightness added, in encoded values.
pub(crate) fn brightness_contrast(source: &mut WorkingBuffer, brightness: f64, contrast: f64) {
    if brightness == 0.0 && contrast == 0.0 {
        return;
    }
    let k = if contrast <= 0.0 { 1.0 + contrast / 100.0 } else { 1.0 / (1.0 - 0.99 * contrast / 100.0) };
    grade_pixels(source, false, |_, e| e.map(|v| (v - 0.5) * k + 0.5 + brightness / 255.0))
}

/// D-136: grey from the largest, middle and smallest channel, weighted by the setting of the
/// largest's colour and of the pair of the two largest. `w` is reds, yellows, greens, cyans,
/// blues, magentas.
pub(crate) fn black_white(source: &mut WorkingBuffer, w: [f64; 6]) {
    grade_pixels(source, false, |_, e| {
        let mut order = [0usize, 1, 2];
        order.sort_by(|&i, &j| e[j].total_cmp(&e[i]));
        let [hi, mid, lo] = order;
        let primary = w[2 * hi];
        let secondary = match hi + mid {
            1 => w[1],
            3 => w[3],
            _ => w[5],
        };
        let g = e[lo] + (e[mid] - e[lo]) * secondary / 100.0 + (e[hi] - e[mid]) * primary / 100.0;
        [g; 3]
    })
}

/// D-182: each pixel's colour through a .cube file's table.
pub(crate) fn color_lookup(source: &mut WorkingBuffer, cube: &crate::lut::Cube) {
    grade_pixels(source, false, |_, e| cube.lookup(e))
}

/// D-137: each channel cut to `levels` flat steps.
pub(crate) fn posterize(source: &mut WorkingBuffer, levels: f64) {
    let n = levels.floor();
    grade_pixels(source, false, |_, e| e.map(|v| (v * n + 1e-4).floor().min(n - 1.0) / (n - 1.0)))
}

/// The encoded luma of an encoded colour (D-130's L, the third batch's `Y(e)`).
fn luma(e: [f64; 3]) -> f64 {
    0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2]
}

/// D-138: white where the encoded luma reaches `level` of 255, else black.
pub(crate) fn threshold(source: &mut WorkingBuffer, level: f64) {
    grade_pixels(source, false, |_, e| [if 255.0 * luma(e) + 1e-4 >= level { 1.0 } else { 0.0 }; 3])
}

/// D-212: each shown pixel kept, all four channels alike, by how far its value in `channel`, 0
/// to 255, lies inside the black and white points `[b, w]`, fading over the softnesses `[s, t]`
/// inside each; with `invert` the other side is kept.
pub(crate) fn extract(source: &mut WorkingBuffer, channel: &str, [b, w, s, t]: [f64; 4], invert: bool) {
    each_pixel(source, |px| {
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        let e = |c: usize| to_srgb((px[c] as f64 / a).clamp(0.0, 1.0));
        let v = 255.0
            * match channel {
                "red" => e(0),
                "green" => e(1),
                "blue" => e(2),
                "alpha" => a,
                _ => luma([e(0), e(1), e(2)]),
            };
        let low = if s > 0.0 { ((v - b) / s).clamp(0.0, 1.0) } else if v + 1e-4 >= b { 1.0 } else { 0.0 };
        let high = if t > 0.0 { ((w - v) / t).clamp(0.0, 1.0) } else if v - 1e-4 <= w { 1.0 } else { 0.0 };
        let m = if invert { 1.0 - low * high } else { low * high };
        for c in px.iter_mut() {
            *c = (*c as f64 * m) as f32;
        }
    })
}

/// D-139: each channel remade from its row: from red, from green, from blue and a constant, in
/// per cent; with `mono` every channel uses the red row.
pub(crate) fn channel_mixer(source: &mut WorkingBuffer, rows: [[f64; 4]; 3], mono: bool) {
    if !mono && rows == [[100.0, 0.0, 0.0, 0.0], [0.0, 100.0, 0.0, 0.0], [0.0, 0.0, 100.0, 0.0]] {
        return;
    }
    grade_pixels(source, false, |_, e| {
        std::array::from_fn(|c| {
            let r = rows[if mono { 0 } else { c }];
            (r[0] * e[0] + r[1] * e[1] + r[2] * e[2] + r[3]) / 100.0
        })
    })
}

/// D-140: colours pushed from or toward their grey, the dull ones more by `vibrance`, all alike
/// by `saturation`.
pub(crate) fn vibrance(source: &mut WorkingBuffer, vibrance: f64, saturation: f64) {
    if vibrance == 0.0 && saturation == 0.0 {
        return;
    }
    grade_pixels(source, false, |_, e| {
        let l = luma(e);
        let s = e[0].max(e[1]).max(e[2]) - e[0].min(e[1]).min(e[2]);
        let k = 1.0 + saturation / 100.0 + vibrance / 100.0 * (1.0 - s);
        e.map(|v| l + (v - l) * k)
    })
}

/// D-141's HSV hue of an encoded colour, in degrees, none for a grey.
pub(crate) fn hsv_hue(e: [f64; 3]) -> Option<f64> {
    let [r, g, b] = e;
    let (mx, mn) = (r.max(g).max(b), r.min(g).min(b));
    if mx == mn {
        return None;
    }
    let d = mx - mn;
    Some(if r == mx {
        (60.0 * ((g - b) / d)).rem_euclid(360.0)
    } else if g == mx {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    })
}

/// D-141: every colour whose hue lies further than `tolerance` (with `softness` beyond it) from
/// the chosen colour's is drained toward its grey by `amount` per cent. A pixel kept whole is
/// left exactly as it is.
pub(crate) fn leave_color(source: &mut WorkingBuffer, color: [f64; 3], tolerance: f64, softness: f64, amount: f64) {
    if amount == 0.0 {
        return;
    }
    let hc = hsv_hue(color);
    let (t, s, o) = (tolerance / 100.0, softness / 100.0, amount / 100.0);
    each_pixel(source, |px| {
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        let e = [0, 1, 2].map(|c| to_srgb((px[c] as f64 / a).clamp(0.0, 1.0)));
        let dist = match (hsv_hue(e), hc) {
            (Some(h), Some(hc)) => {
                let d = (h - hc).abs();
                d.min(360.0 - d) / 180.0
            }
            _ => 1.0,
        };
        let k = if dist <= t {
            1.0
        } else if s == 0.0 || dist >= t + s {
            0.0
        } else {
            1.0 - (dist - t) / s
        };
        if k == 1.0 {
            return;
        }
        let d = o * (1.0 - k);
        let y = luma(e);
        for c in 0..3 {
            px[c] = (to_linear((e[c] + d * (y - e[c])).clamp(0.0, 1.0)) * a) as f32;
        }
    });
}

/// D-197's hue (none for a grey), lightness and saturation of an encoded colour.
fn hls(e: [f64; 3]) -> (Option<f64>, f64, f64) {
    let (mx, mn) = (e[0].max(e[1]).max(e[2]), e[0].min(e[1]).min(e[2]));
    let l = (mx + mn) / 2.0;
    let s = if mx == mn { 0.0 } else { (mx - mn) / (1.0 - (2.0 * l - 1.0).abs()) };
    (hsv_hue(e), l, s)
}

/// D-197's settings, read once for a frame: `from` and `to` encoded, the three tolerances and
/// the softness 0 to 100.
pub(crate) struct ChangeToColor<'a> {
    pub from: [f64; 3],
    pub to: [f64; 3],
    pub change: &'a str,
    pub transforming: bool,
    pub tolerances: [f64; 3],
    pub softness: f64,
    pub matte: bool,
}

/// D-197: each colour near `from` by hue, lightness and saturation turned toward `to` by how near
/// it is, or with `matte` that nearness shown as grey. A pixel not near at all is left exactly as
/// it is.
pub(crate) fn change_to_color(source: &mut WorkingBuffer, c: &ChangeToColor) {
    let (hf, lf, sf) = hls(c.from);
    let (ht, lt, st) = hls(c.to);
    let (hf0, ht0) = (hf.unwrap_or(0.0), ht.unwrap_or(0.0));
    let t = c.tolerances.map(|v| v / 100.0);
    let w = c.softness / 100.0;
    let part = |d: f64, t: f64| {
        if d <= t {
            1.0
        } else if w == 0.0 || t == 0.0 || d >= t * (1.0 + w) {
            0.0
        } else {
            1.0 - (d - t) / (t * w)
        }
    };
    let (lightness, saturation) = (c.change.contains("lightness"), c.change.contains("saturation"));
    each_pixel(source, |px| {
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        let e = [0, 1, 2].map(|i| to_srgb((px[i] as f64 / a).clamp(0.0, 1.0)));
        let (h, l, s) = hls(e);
        let dh = match (h, hf) {
            (Some(h), Some(hf)) => {
                let d = (h - hf).abs();
                d.min(360.0 - d) / 180.0
            }
            _ => 1.0,
        };
        let k = part(dh, t[0]).min(part((l - lf).abs(), t[1])).min(part((s - sf).abs(), t[2]));
        if c.matte {
            let v = (to_linear(k) * a) as f32;
            px[..3].fill(v);
            return;
        }
        if k <= 0.0 {
            return;
        }
        let h = h.unwrap_or(0.0);
        let (h, l, s) = if c.transforming {
            (
                (h + ht0 - hf0).rem_euclid(360.0),
                if lightness { (l + lt - lf).clamp(0.0, 1.0) } else { l },
                if saturation { (s + st - sf).clamp(0.0, 1.0) } else { s },
            )
        } else {
            (ht0, if lightness { lt } else { l }, if saturation { st } else { s })
        };
        let chroma = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let hh = h / 60.0;
        let x = chroma * (1.0 - (hh.rem_euclid(2.0) - 1.0).abs());
        let rgb = match (hh.floor() as usize).min(5) {
            0 => [chroma, x, 0.0],
            1 => [x, chroma, 0.0],
            2 => [0.0, chroma, x],
            3 => [0.0, x, chroma],
            4 => [x, 0.0, chroma],
            _ => [chroma, 0.0, x],
        };
        let m = l - chroma / 2.0;
        for i in 0..3 {
            px[i] = (to_linear((e[i] + k * (rgb[i] + m - e[i])).clamp(0.0, 1.0)) * a) as f32;
        }
    });
}

/// D-142: each channel at or above `threshold` of 255 turned to its opposite; the rest kept bit
/// for bit.
pub(crate) fn solarize(source: &mut WorkingBuffer, threshold: f64) {
    each_pixel(source, |px| {
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        for c in 0..3 {
            let e = to_srgb((px[c] as f64 / a).clamp(0.0, 1.0));
            if 255.0 * e + 1e-4 >= threshold {
                px[c] = (to_linear(1.0 - e) * a) as f32;
            }
        }
    });
}

/// D-143's settings, read once for a frame.
pub(crate) struct Halftone {
    pub size: f64,
    pub angle: f64,
    pub ink: [f64; 3],
    pub paper: [f64; 3],
    pub amount: f64,
}

/// D-143: each pixel ink or paper by whether it lies inside its screen cell's dot, the dot as
/// large as the pixel is dark; mixed in at `amount` per cent. `(ox, oy)` is the drawing's corner
/// in the buffer.
pub(crate) fn halftone(source: &mut WorkingBuffer, h: &Halftone, (ox, oy): (usize, usize)) {
    if h.amount == 0.0 {
        return;
    }
    let w = source.width();
    let (sin, cos) = h.angle.to_radians().sin_cos();
    let (ink, paper) = (h.ink.map(to_linear), h.paper.map(to_linear));
    let o = h.amount / 100.0;
    let quarter = std::f64::consts::FRAC_PI_4;
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let a = px[3] as f64;
            if a <= 0.0 {
                return;
            }
            let (x, y) = ((i % w) as f64 - ox as f64 + 0.5, (i / w) as f64 - oy as f64 + 0.5);
            let (u, v) = ((x * cos + y * sin) / h.size, (-x * sin + y * cos) / h.size);
            let rho = ((u - u.floor() - 0.5).powi(2) + (v - v.floor() - 0.5).powi(2)).sqrt();
            let b = [0, 1, 2].map(|c| px[c] as f64 / a);
            let k = 1.0 - luma(b.map(|c| to_srgb(c.clamp(0.0, 1.0)))).clamp(0.0, 1.0);
            let r = if k <= quarter {
                (k / std::f64::consts::PI).sqrt()
            } else {
                0.5 + 0.21 * (k - quarter) / (1.0 - quarter)
            };
            let g = if rho < r { ink } else { paper };
            for c in 0..3 {
                px[c] = ((b[c] + o * (g[c] - b[c])) * a) as f32;
            }
        });
}
