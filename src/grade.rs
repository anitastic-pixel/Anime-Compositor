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

fn to_srgb(c: f64) -> f64 {
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
fn to_hsl([r, g, b]: [f64; 3]) -> [f64; 3] {
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
/// "multiply", "screen" or "add"; add is not held back.
pub(crate) fn mixer(blend: &str) -> fn(f64, f64) -> f64 {
    match blend {
        "multiply" => |b, c| b * c,
        "screen" => |b, c| 1.0 - (1.0 - b) * (1.0 - c),
        "add" => |b, c| b + c,
        _ => |_, c| c,
    }
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
    source.data_mut().par_chunks_exact_mut(4).for_each(|px| {
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
