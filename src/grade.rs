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
pub(crate) fn grade_pixels(source: &mut WorkingBuffer, placed: bool, f: impl Fn(usize, [f64; 3]) -> [f64; 3] + Sync) {
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
/// already valid; all five curves straight changes nothing.
pub(crate) fn curves(source: &mut WorkingBuffer, master: &[Vec<f64>], rgb: [&[Vec<f64>]; 3], alpha: &[Vec<f64>]) {
    if !(is_straight(master) && rgb.iter().all(|c| is_straight(c))) {
        let m = spline(master);
        let c = rgb.map(spline);
        grade(source, |i, x| m(c[i](x).clamp(0.0, 255.0)).clamp(0.0, 255.0));
    }
    // D-302: then the covering through its own curve, 0 to 255, the colour kept. A pixel that
    // did not show has no colour, so one the curve makes show is black.
    if !is_straight(alpha) {
        alpha_through(source, spline(alpha));
    }
}

/// D-302: the covering through `f`, 0 to 255 in and out, the colour kept. A pixel that did not
/// show has no colour, so one `f` makes show is black. Arbitrary Map's alpha table too (D-395).
pub(crate) fn alpha_through(source: &mut WorkingBuffer, f: impl Fn(f64) -> f64 + Sync) {
    each_pixel(source, |px| {
        let a = px[3] as f64;
        let to = f(a * 255.0).clamp(0.0, 255.0) / 255.0;
        for c in 0..3 {
            px[c] = if a > 0.0 { (px[c] as f64 / a * to) as f32 } else { 0.0 };
        }
        px[3] = to as f32;
    });
}

/// D-112: each channel from the input range to 0..1, held there, bent by the gamma and laid on
/// the output range. An input white equal to its black is a threshold. The settings are
/// already valid and held; the defaults change nothing.
pub(crate) fn levels(source: &mut WorkingBuffer, settings: [f64; 5]) {
    let plain = crate::effects::LEVELS_PLAIN;
    channel_levels(source, &[settings, plain, plain, plain, plain]);
}

/// D-383: each colour channel through its own set, then through the RGB set, a set that changes
/// nothing skipped, so with only the RGB set this is [`levels`] to the bit; then the covering
/// through the alpha set, 0 to 255, the colour kept, as D-302's Curves. `sets` are valid, in
/// the order RGB, red, green, blue, alpha.
pub(crate) fn channel_levels(source: &mut WorkingBuffer, sets: &[[f64; 5]; 5]) {
    use crate::effects::LEVELS_PLAIN as PLAIN;
    let level = |[ib, iw, gamma, ob, ow]: [f64; 5], x: f64| {
        let v = if iw == ib {
            if x >= ib { 1.0 } else { 0.0 }
        } else {
            ((x - ib) / (iw - ib)).clamp(0.0, 1.0)
        };
        ob + v.powf(1.0 / gamma) * (ow - ob)
    };
    let set = |s: [f64; 5], x: f64| if s == PLAIN { x } else { level(s, x) };
    if sets[..4].iter().any(|s| *s != PLAIN) {
        grade(source, |c, x| set(sets[0], set(sets[c + 1], x)));
    }
    if sets[4] != PLAIN {
        each_pixel(source, |px| {
            let a = px[3] as f64;
            let to = level(sets[4], a * 255.0) / 255.0;
            for c in 0..3 {
                px[c] = if a > 0.0 { (px[c] as f64 / a * to) as f32 } else { 0.0 };
            }
            px[3] = to as f32;
        });
    }
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
pub(crate) fn from_hsl([h, s, l]: [f64; 3]) -> [f64; 3] {
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
///
/// D-307: each of `ranges` adds its hue, saturation and lightness, weighted by how far the
/// pixel's hue lies inside it: fully within 15 degrees of its centre, fading to nothing at 45, so
/// neighbouring ranges share the hues between them. A grey has no hue and takes none of them.
pub(crate) fn hue_saturation(
    source: &mut WorkingBuffer,
    hue: f64,
    saturation: f64,
    lightness: f64,
    ranges: &[[f64; 3]; 6],
) {
    let plain = *ranges == [[0.0; 3]; 6];
    if hue == 0.0 && saturation == 0.0 && lightness == 0.0 && plain {
        return;
    }
    grade_pixels(source, false, |_, e| {
        let [h, s, l] = to_hsl(e);
        let [hue, saturation, lightness] = if plain || s == 0.0 {
            [hue, saturation, lightness]
        } else {
            let mut t = [hue, saturation, lightness];
            for (i, r) in ranges.iter().enumerate() {
                let d = (h - 60.0 * i as f64).rem_euclid(360.0);
                let w = ((45.0 - d.min(360.0 - d)) / 30.0).clamp(0.0, 1.0);
                t[0] += w * r[0];
                t[1] += w * r[1].clamp(-100.0, 100.0);
                t[2] += w * r[2].clamp(-100.0, 100.0);
            }
            [t[0], t[1], t[2].clamp(-100.0, 100.0)]
        };
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
/// D-319: Screen in Float working depth, Nuke's rule: `b + c - b*c` while either is at most 1,
/// else the larger, because `1 - (1-b)(1-c)` turns back down once both pass white.
pub(crate) fn screen_float(b: f64, c: f64) -> f64 {
    if b <= 1.0 || c <= 1.0 {
        b + c - b * c
    } else {
        b.max(c)
    }
}

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
/// round `(x, y, z)`, mixed by the smoothed place inside it. D-299: `block`, each cell one
/// number across and down, the depth still mixed so it evolves smoothly; `period`, when not 0,
/// the depth's cells repeat after that many.
fn value(base: u64, ch: u64, x: f64, y: f64, z: f64, block: bool, period: i64) -> f64 {
    let (i, j, k) = (x.floor(), y.floor(), z.floor());
    let fade = |t: f64| t * t * t * (t * (6.0 * t - 15.0) + 10.0);
    let s = if block { [0.0, 0.0, fade(z - k)] } else { [fade(x - i), fade(y - j), fade(z - k)] };
    let (i, j, k) = (i as i64, j as i64, k as i64);
    let depth = |k: i64| if period > 0 { k.rem_euclid(period) } else { k };
    let mut v = 0.0;
    for corner in 0..8 {
        let d = [corner & 1, (corner >> 1) & 1, corner >> 2];
        let w: f64 = (0..3).map(|a| if d[a] == 1 { s[a] } else { 1.0 - s[a] }).product();
        // D-443: a corner that weighs nothing adds nothing (the sum is the same to the bit).
        if w != 0.0 {
            v += w * unit(base, i + d[0] as i64, j + d[1] as i64, depth(k + d[2] as i64), ch);
        }
    }
    v
}

/// D-443: After Effects' Add Grain, by document 21's rule. Each channel's grain is [`value`]
/// at the pixel's middle in the drawing's own space (its corner at `(ox, oy)` in `source`), in
/// cells `size aspect_ratio` across and `size` down, at depth frame times speed (its whole part
/// unless Animate Smoothly), block and smooth mixed by Softness; Saturation pulls the three to
/// their mean, the channel intensities scale them, and the tonal weight from the pixel's
/// brightness sets how strongly it is laid on by Film, Add or Overlay. The settings are already
/// valid; intensity 0 changes nothing.
pub(crate) fn add_grain(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    let crate::effects::Effect::AddGrain {
        intensity,
        size,
        softness,
        aspect_ratio,
        red_intensity,
        green_intensity,
        blue_intensity,
        monochromatic,
        saturation,
        blending_mode,
        shadows,
        midtones,
        highlights,
        midpoint,
        animation_speed,
        animate_smoothly,
        random_seed,
        frame,
    } = e
    else {
        return;
    };
    if *intensity == 0.0 {
        return;
    }
    let w = source.width();
    let (base, mono, soft) = (mix(random_seed.floor() as u64), monochromatic == "on", *softness);
    let phase = *frame as f64 * animation_speed;
    let z = if animate_smoothly == "on" { phase } else { phase.floor() };
    let (sx, sy) = (size * aspect_ratio, *size);
    let chan = [*red_intensity, *green_intensity, *blue_intensity];
    let (mid, k) = (*midpoint, 0.1 * intensity);
    let blend = match blending_mode.as_str() {
        "film" => 0,
        "add" => 1,
        _ => 2,
    };
    grade_pixels(source, true, |i, e| {
        let x = ((i % w) as f64 - ox as f64 + 0.5) / sx;
        let y = ((i / w) as f64 - oy as f64 + 0.5) / sy;
        let grain = |ch: u64| {
            if soft == 0.0 {
                value(base, ch, x, y, z, true, 0)
            } else if soft == 1.0 {
                value(base, ch, x, y, z, false, 0)
            } else {
                (1.0 - soft) * value(base, ch, x, y, z, true, 0) + soft * value(base, ch, x, y, z, false, 0)
            }
        };
        let mut g = if mono { [grain(0); 3] } else { [grain(0), grain(1), grain(2)] };
        if !mono {
            let m = (g[0] + g[1] + g[2]) / 3.0;
            g = g.map(|v| m + saturation * (v - m));
        }
        let lum = 0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2];
        let weight = if lum <= mid {
            shadows + (midtones - shadows) * lum / mid
        } else {
            midtones + (highlights - midtones) * (lum - mid) / (1.0 - mid)
        };
        let amount = k * weight;
        std::array::from_fn(|c| {
            let (e, a) = (e[c], amount * (g[c] * chan[c]));
            match blend {
                0 => e + a * 4.0 * e * (1.0 - e),
                1 => e + a,
                _ => {
                    let b = (0.5 + 0.5 * a).clamp(0.0, 1.0);
                    if e <= 0.5 { 2.0 * e * b } else { 1.0 - 2.0 * (1.0 - e) * (1.0 - b) }
                }
            }
        })
    });
}

/// D-454: the grain in `picture`, per channel of its straight colour through the sRGB curve,
/// held in 0 to 1, by Immerkaer's fast noise estimate (J. Immerkaer, "Fast Noise Variance
/// Estimation", CVIU 64(2):300-302, 1996): sqrt(pi / 2) / (6 N) times the sum of
/// |[[1,-2,1],[-2,4,-2],[1,-2,1]] * I| over the N 3 by 3 windows whose nine pixels all show; 0
/// when there is none.
pub(crate) fn grain_levels(picture: &WorkingBuffer) -> [f64; 3] {
    let (w, h, d) = (picture.width(), picture.height(), picture.data());
    if w < 3 || h < 3 {
        return [0.0; 3];
    }
    const MASK: [f64; 9] = [1.0, -2.0, 1.0, -2.0, 4.0, -2.0, 1.0, -2.0, 1.0];
    // Bands of window rows, each encoding only its own rows (and the two below) so the whole
    // picture is never held encoded at once.
    const BAND: usize = 32;
    let (sum, n) = (0..(h - 2).div_ceil(BAND))
        .into_par_iter()
        .map(|b| {
            let (y0, y1) = (b * BAND, ((b + 1) * BAND).min(h - 2));
            let enc: Vec<Option<[f64; 3]>> = d[y0 * w * 4..(y1 + 2) * w * 4]
                .chunks_exact(4)
                .map(|p| {
                    let a = p[3] as f64;
                    (a > 0.0).then(|| std::array::from_fn(|c| to_srgb((p[c] as f64 / a).clamp(0.0, 1.0))))
                })
                .collect();
            let (mut s, mut n) = ([0.0f64; 3], 0usize);
            for y in 0..y1 - y0 {
                'window: for x in 0..w - 2 {
                    let mut f = [0.0f64; 3];
                    for (k, m) in MASK.iter().enumerate() {
                        let Some(e) = enc[(y + k / 3) * w + x + k % 3] else {
                            continue 'window;
                        };
                        for c in 0..3 {
                            f[c] += m * e[c];
                        }
                    }
                    for c in 0..3 {
                        s[c] += f[c].abs();
                    }
                    n += 1;
                }
            }
            (s, n)
        })
        .reduce(|| ([0.0; 3], 0), |a, b| ([a.0[0] + b.0[0], a.0[1] + b.0[1], a.0[2] + b.0[2]], a.1 + b.1));
    if n == 0 {
        return [0.0; 3];
    }
    sum.map(|s| (std::f64::consts::PI / 2.0).sqrt() / (6.0 * n as f64) * s)
}

/// D-455: Remove Grain's range spreads h per channel, for a measured noise: 2 n sigma_c (the
/// ratio from Zhang and Gunturk, IEEE TIP 17(12), 2008), or with Multichannel one spread for all
/// three, 2 n sqrt(mean of sigma_c^2). None when nothing would be removed (n 0, no noise, or not
/// yet measured).
pub(crate) fn grain_spreads(e: &crate::effects::Effect) -> Option<[f64; 3]> {
    let crate::effects::Effect::RemoveGrain { noise_reduction: n, mode, noise: Some(s), .. } = e else {
        return None;
    };
    let h = if mode == "multichannel" {
        [2.0 * n * ((s[0] * s[0] + s[1] * s[1] + s[2] * s[2]) / 3.0).sqrt(); 3]
    } else {
        s.map(|s| 2.0 * n * s)
    };
    (h.iter().any(|&h| h > 0.0)).then_some(h)
}

/// D-455: After Effects' Remove Grain, by document 21's rule. The layer's grain `noise` (measured
/// here when compose did not), then a bilateral filter (Tomasi and Manduchi, ICCV 1998) on the
/// encoded straight colour, pass k of `passes` over the last pass's output: each pixel that shows
/// the mean of the taps within 2k pixels that show, weighed by covering, a bell on distance of
/// spread k and a bell on the colour difference of spread [`grain_spreads`] (Multichannel one
/// bell on the mean squared difference, Single Channel one per channel, a channel with spread 0
/// kept). Back through the curve at the pixel's covering, then the Unsharp Mask as Sharpen's
/// (`layer_fx::sharpen`). The settings are already valid.
// ponytail: each pass weighs its whole disc, about 200 taps at the fourth pass; a separable or
// guided approximation would be the upgrade, at the price of the rule.
pub(crate) fn remove_grain(source: &mut WorkingBuffer, e: &crate::effects::Effect) {
    let crate::effects::Effect::RemoveGrain { noise_reduction, passes, mode, unsharp_amount, unsharp_radius, unsharp_threshold, noise } = e else {
        return;
    };
    let mut measured = e.clone();
    if noise.is_none() && *noise_reduction > 0.0 {
        if let crate::effects::Effect::RemoveGrain { noise, .. } = &mut measured {
            *noise = Some(grain_levels(source));
        }
    }
    if let Some(h) = grain_spreads(&measured) {
        let (w, ht) = (source.width(), source.height());
        let multi = mode == "multichannel";
        let inv = h.map(|h| if h > 0.0 { 1.0 / (2.0 * h * h) } else { 0.0 });
        let cover: Vec<f64> = source.data().chunks_exact(4).map(|p| p[3] as f64).collect();
        let mut enc: Vec<[f64; 3]> = source
            .data()
            .par_chunks_exact(4)
            .map(|p| {
                let a = p[3] as f64;
                if a > 0.0 { std::array::from_fn(|c| to_srgb((p[c] as f64 / a).clamp(0.0, 1.0))) } else { [0.0; 3] }
            })
            .collect();
        for k in 1..=passes.floor() as isize {
            let r = 2 * k;
            let taps: Vec<(isize, isize, f64)> = (-r..=r)
                .flat_map(|dy| (-r..=r).map(move |dx| (dx, dy)))
                .filter(|&(dx, dy)| dx * dx + dy * dy <= r * r)
                .map(|(dx, dy)| (dx, dy, (-((dx * dx + dy * dy) as f64) / (2.0 * (k * k) as f64)).exp()))
                .collect();
            enc = (0..w * ht)
                .into_par_iter()
                .map(|i| {
                    let own = enc[i];
                    if cover[i] <= 0.0 {
                        return own;
                    }
                    let (x, y) = ((i % w) as isize, (i / w) as isize);
                    let (mut sum, mut weight) = ([0.0f64; 3], [0.0f64; 3]);
                    for &(dx, dy, g) in &taps {
                        let (tx, ty) = (x + dx, y + dy);
                        if tx < 0 || ty < 0 || tx >= w as isize || ty >= ht as isize {
                            continue;
                        }
                        let j = ty as usize * w + tx as usize;
                        if cover[j] <= 0.0 {
                            continue;
                        }
                        let (t, ga) = (enc[j], g * cover[j]);
                        let d: [f64; 3] = std::array::from_fn(|c| t[c] - own[c]);
                        let wt: [f64; 3] = if multi {
                            [ga * (-(d[0] * d[0] + d[1] * d[1] + d[2] * d[2]) / 3.0 * inv[0]).exp(); 3]
                        } else {
                            std::array::from_fn(|c| ga * (-d[c] * d[c] * inv[c]).exp())
                        };
                        for c in 0..3 {
                            sum[c] += wt[c] * t[c];
                            weight[c] += wt[c];
                        }
                    }
                    std::array::from_fn(|c| if h[c] > 0.0 { sum[c] / weight[c] } else { own[c] })
                })
                .collect();
        }
        source.data_mut().par_chunks_exact_mut(4).zip(enc.par_iter()).for_each(|(px, e)| {
            let a = px[3] as f64;
            if a > 0.0 {
                (0..3).for_each(|c| px[c] = (to_linear(e[c]) * a) as f32);
            }
        });
    }
    crate::layer_fx::sharpen(source, *unsharp_amount, *unsharp_radius, *unsharp_threshold);
}

/// D-456: this program's own Deband, by document 21's rule (`tools/deband_reference.py`). Each
/// pixel that shows: its straight colour through the sRGB curve, e; its largest step g in
/// levels to its four neighbours that show; its edge weight E from the threshold and softness.
/// A normalised blur across then down that edges lend nothing to and that stops at them, pulled
/// back on edges by the Edge Blur Reduction; then each pixel reads its neighbour by the
/// diffusion's offset (D-119's hash, seed 0, in the drawing's own space, its corner at
/// `(ox, oy)` in `source`), an edge lending only by 1 - E; then triangular dither, one number for
/// the three; each reseeded by P0-23's held step and pulled back on edges by its reduction.
/// Views Edge Only (E as grey) and Selected Edge (the edges in red). The covering is kept. The
/// settings are already valid; blur, diffusion and noise all 0 in view Normal change nothing.
pub(crate) fn deband(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    let crate::effects::Effect::Deband {
        view,
        edge_threshold: t,
        edge_softness: s,
        blur_amount,
        edge_blur_reduction: ebr,
        diffusion_amount: dm,
        edge_diffusion_reduction: edr,
        diffusion_reseed_interval,
        noise_amount: na,
        edge_noise_reduction: enr,
        noise_reseed_interval,
        frame,
    } = e
    else {
        return;
    };
    if view == "normal" && *blur_amount == 0.0 && *dm == 0.0 && *na == 0.0 {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let cover: Vec<f64> = source.data().chunks_exact(4).map(|p| p[3] as f64).collect();
    let enc: Vec<[f64; 3]> = source
        .data()
        .par_chunks_exact(4)
        .map(|p| {
            let a = p[3] as f64;
            if a > 0.0 { std::array::from_fn(|c| to_srgb((p[c] as f64 / a).clamp(0.0, 1.0))) } else { [0.0; 3] }
        })
        .collect();
    let on = |x: isize, y: isize| x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h;
    let edge: Vec<f64> = (0..w * h)
        .into_par_iter()
        .map(|i| {
            if cover[i] <= 0.0 {
                return 0.0;
            }
            let (x, y) = ((i % w) as isize, (i / w) as isize);
            let mut g = 0.0f64;
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (tx, ty) = (x + dx, y + dy);
                if on(tx, ty) {
                    let j = ty as usize * w + tx as usize;
                    if cover[j] > 0.0 {
                        (0..3).for_each(|c| g = g.max((enc[j][c] - enc[i][c]).abs() * 255.0));
                    }
                }
            }
            if *s > 0.0 { ((g - t) / s).clamp(0.0, 1.0) } else if g > *t { 1.0 } else { 0.0 }
        })
        .collect();
    let k: Vec<f64> = crate::effects::gaussian_weights(blur_amount / 3.0).into_iter().map(f64::from).collect();
    let r = k.len() / 2;
    // Each pixel's sums along a row (or column) of `v`, each tap times its weight and the
    // product of (1 - E) over the pixels strictly between.
    let gather = |v: &[[f64; 4]], across: bool| -> Vec<[f64; 4]> {
        (0..w * h)
            .into_par_iter()
            .map(|i| {
                let (x, y) = ((i % w) as isize, (i / w) as isize);
                let mut total = v[i].map(|u| k[r] * u);
                for sign in [-1isize, 1] {
                    let mut through = 1.0;
                    for d in 1..=r as isize {
                        let (tx, ty) = if across { (x + sign * d, y) } else { (x, y + sign * d) };
                        if !on(tx, ty) {
                            break;
                        }
                        let j = ty as usize * w + tx as usize;
                        (0..4).for_each(|c| total[c] += k[r + d as usize] * through * v[j][c]);
                        through *= 1.0 - edge[j];
                    }
                }
                total
            })
            .collect()
    };
    let m: Vec<[f64; 3]> = if r > 0 {
        let lent: Vec<[f64; 4]> = (0..w * h)
            .map(|i| {
                let wt = if cover[i] > 0.0 { cover[i] * (1.0 - edge[i]) } else { 0.0 };
                [wt * enc[i][0], wt * enc[i][1], wt * enc[i][2], wt]
            })
            .collect();
        let b = gather(&gather(&lent, true), false);
        (0..w * h)
            .map(|i| {
                if cover[i] > 0.0 && b[i][3] > 0.0 {
                    let f = 1.0 - edge[i] * ebr / 100.0;
                    std::array::from_fn(|c| enc[i][c] + (b[i][c] / b[i][3] - enc[i][c]) * f)
                } else {
                    enc[i]
                }
            })
            .collect()
    } else {
        enc
    };
    let base = mix(0);
    let fd = crate::effects::turbulent_seed(0.0, *diffusion_reseed_interval, *frame) as i64;
    let fnz = crate::effects::turbulent_seed(0.0, *noise_reseed_interval, *frame) as i64;
    let away = |v: f64| (v.abs() + 0.5).floor().copysign(v) as isize;
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let a = cover[i];
        if a <= 0.0 {
            return;
        }
        let (x, y) = ((i % w) as isize, (i / w) as isize);
        let (lx, ly) = (x as i64 - ox as i64, y as i64 - oy as i64);
        let own = m[i];
        let mut d = own;
        if *dm > 0.0 {
            let kk = dm * (1.0 - edge[i] * edr / 100.0);
            let (vx, vy) = (kk * unit(base, lx, ly, fd, 0), kk * unit(base, lx, ly, fd, 1));
            let (tx, ty) = (x + away(vx), y + away(vy));
            if on(tx, ty) {
                let j = ty as usize * w + tx as usize;
                if cover[j] > 0.0 {
                    d = std::array::from_fn(|c| m[j][c] * (1.0 - edge[j]) + own[c] * edge[j]);
                }
            }
        }
        let tri = (unit(base, lx, ly, fnz, 2) + unit(base, lx, ly, fnz, 3)) / 2.0;
        let n = tri * na / 255.0 * (1.0 - edge[i] * enr / 100.0);
        let mut o = d.map(|v| (v + n).clamp(0.0, 1.0));
        let ei = edge[i];
        match view.as_str() {
            "edge_only" => o = [ei; 3],
            "selected_edge" => o = [o[0] * (1.0 - ei) + ei, o[1] * (1.0 - ei), o[2] * (1.0 - ei)],
            _ => {}
        }
        (0..3).for_each(|c| px[c] = (to_linear(o[c]) * a) as f32);
    });
}

/// D-454: Match Grain as the Add Grain it lays: its channel intensities times the source's
/// measured grain, sigma 10 sqrt(3) (Add Grain's block grain at intensity 1 spreads 0.1 /
/// sqrt(3)). None when nothing would be added: no source picture, intensity 0 or no channel left.
pub(crate) fn matched_grain(e: &crate::effects::Effect) -> Option<crate::effects::Effect> {
    let crate::effects::Effect::MatchGrain { grain, map: Some(m), .. } = e else {
        return None;
    };
    let mut g = (**grain).clone();
    let crate::effects::Effect::AddGrain { intensity, red_intensity, green_intensity, blue_intensity, .. } = &mut g else {
        return None;
    };
    let (s, k) = (grain_levels(&m.0), 10.0 * 3f64.sqrt());
    *red_intensity *= s[0] * k;
    *green_intensity *= s[1] * k;
    *blue_intensity *= s[2] * k;
    (*intensity != 0.0 && [*red_intensity, *green_intensity, *blue_intensity] != [0.0; 3]).then_some(g)
}

/// D-450: After Effects' Noise Alpha, by document 21's rule. The noise at the pixel in the
/// drawing's own space (its corner at `(ox, oy)` in `source`): the Random kinds [`unit`] for the
/// seed, the Animation kinds [`value`] in blocks of a pixel at depth phase / 360 for seed 0,
/// repeating after Cycle turns when Cycle Noise is on; Squared pushes it out to
/// sign(n) (1 - (1 - |n|)^2). Times the amount, it moves the covering everywhere (Add), where
/// fully covered (Clamp), in proportion (Scale) or where partly covered (Edges), and a covering
/// pushed out of 0 to 1 is clipped, reflected back (Wrap Back) or wrapped round (Wrap). The
/// colour is kept, black where there was none. The settings are already valid; amount 0
/// changes nothing.
pub(crate) fn noise_alpha(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    let crate::effects::Effect::NoiseAlpha { noise, amount, original_alpha, overflow, random_seed, noise_phase, cycle_noise, cycle } = e else {
        return;
    };
    if *amount == 0.0 {
        return;
    }
    let animated = noise.ends_with("_animation");
    let squared = noise.starts_with("squared");
    let base = mix(if animated { 0 } else { random_seed.floor() as u64 });
    let (z, k) = (noise_phase / 360.0, amount / 100.0);
    let period = if cycle_noise == "on" { cycle.floor() as i64 } else { 0 };
    let how = crate::effects::NOISE_ALPHA_ORIGINALS.iter().position(|o| o == original_alpha).unwrap_or(0);
    let wrap = crate::effects::NOISE_ALPHA_OVERFLOWS.iter().position(|o| o == overflow).unwrap_or(0);
    let w = source.width().max(1);
    source.data_mut().par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        for (x, px) in row.chunks_exact_mut(4).enumerate() {
            let a = px[3] as f64;
            // Clamp, Add, Scale, Edges.
            let skip = match how {
                0 => a != 1.0,
                2 => a <= 0.0,
                3 => !(a > 0.0 && a < 1.0),
                _ => false,
            };
            if skip {
                continue;
            }
            let (x, y) = (x as f64 - ox as f64, y as f64 - oy as f64);
            let mut n = value(base, 0, x, y, if animated { z } else { 0.0 }, true, period);
            if squared {
                n = (1.0 - (1.0 - n.abs()).powi(2)).copysign(n);
            }
            let d = k * n;
            let mut t = if how == 2 { a + d * a } else { a + d };
            if !(0.0..=1.0).contains(&t) {
                t = match wrap {
                    1 if t > 1.0 => 2.0 - t,
                    1 => -t,
                    2 if t > 1.0 => t - 1.0,
                    2 => t + 1.0,
                    _ => t,
                }
                .clamp(0.0, 1.0);
            }
            if t == a {
                continue;
            }
            for c in 0..3 {
                px[c] = if a > 0.0 { (px[c] as f64 / a * t) as f32 } else { 0.0 };
            }
            px[3] = t as f32;
        }
    });
}

/// D-451: After Effects' Noise HLS, by document 21's rule. Three noises in -1..1 at the pixel in
/// the drawing's own space (its corner at `(ox, oy)` in `source`), channel 0 for the hue, 1 the
/// lightness and 2 the saturation, [`value`] for seed 0 at depth `z`: one number a pixel
/// (`uniform`), the same pushed out to sign(n) (1 - (1 - |n|)^2) (`squared`), or smooth in cells
/// of `size` pixels at the pixel's middle (`grain`). Through D-113's HSL the hue turns
/// 180 n `amounts[0]` / 100 degrees, and the lightness and saturation move n `amounts[1]` / 100
/// and n `amounts[2]` / 100, held in 0 to 1. The settings are already valid; all three amounts 0
/// change nothing.
pub(crate) fn noise_hls(source: &mut WorkingBuffer, noise: &str, amounts: [f64; 3], size: f64, z: f64, (ox, oy): (usize, usize)) {
    if amounts == [0.0; 3] {
        return;
    }
    let (w, base) = (source.width().max(1), mix(0));
    let [hue, lightness, saturation] = amounts.map(|v| v / 100.0);
    grade_pixels(source, true, |i, e| {
        let (x, y) = ((i % w) as f64 - ox as f64, (i / w) as f64 - oy as f64);
        let n: [f64; 3] = std::array::from_fn(|k| match noise {
            "grain" => value(base, k as u64, (x + 0.5) / size, (y + 0.5) / size, z, false, 0),
            "squared" => {
                let n = value(base, k as u64, x, y, z, true, 0);
                (1.0 - (1.0 - n.abs()).powi(2)).copysign(n)
            }
            _ => value(base, k as u64, x, y, z, true, 0),
        });
        let [h, s, l] = to_hsl(e);
        from_hsl([
            (h + 180.0 * n[0] * hue).rem_euclid(360.0),
            (s + n[2] * saturation).clamp(0.0, 1.0),
            (l + n[1] * lightness).clamp(0.0, 1.0),
        ])
    });
}

/// D-299: After Effects' Fractal Type, Noise Type and Cycle Evolution. The default is D-128's
/// basic, smooth noise that never repeats.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Look {
    /// Each octave's distance from the middle, so the clouds crease where it is nought.
    pub turbulent: bool,
    pub block: bool,
    /// Whole turns of evolution after which it repeats, 0 for never.
    pub cycle: i64,
}

/// D-127 and D-128's fractal noise: `octaves` of [`value`], each half the last's strength at
/// twice its fineness, over the sum of the strengths. Channel `ch`'s octave `o` is channel
/// `8 o + ch` of the value noise.
pub(crate) fn fractal(base: u64, ch: u64, p: (f64, f64, f64), octaves: usize) -> f64 {
    fractal_with(base, ch, p, octaves, Look::default())
}

/// D-299: [`fractal`] with its look. Turbulent sums each octave's size, 0 to 1, and maps the
/// share back to -1..1; octave `o`'s depth is `2^o` times finer, so it repeats after `2^o`
/// times the cycle's cells.
pub(crate) fn fractal_with(base: u64, ch: u64, (x, y, z): (f64, f64, f64), octaves: usize, look: Look) -> f64 {
    let (mut sum, mut total, mut amp, mut fine) = (0.0, 0.0, 1.0, 1.0);
    for o in 0..octaves {
        let v = value(base, 8 * o as u64 + ch, x * fine, y * fine, z * fine, look.block, look.cycle << o);
        sum += amp * if look.turbulent { v.abs() } else { v };
        total += amp;
        amp *= 0.5;
        fine *= 2.0;
    }
    if look.turbulent { 2.0 * sum / total - 1.0 } else { sum / total }
}

/// D-128's settings, read once for a frame: the noise's size, octaves and depth (`z`, the
/// evolution and the frame's turns), its contrast and brightness, the two colours encoded 0 to
/// 1, and how it is mixed in. D-299: `size` is across and `size_y` down; `offset` slides the
/// clouds, in pixels; `invert` turns them over before the contrast.
pub(crate) struct Fractal {
    pub size: f64,
    pub size_y: f64,
    pub offset: [f64; 2],
    pub invert: bool,
    pub look: Look,
    pub octaves: usize,
    pub seed: f64,
    pub z: f64,
    pub contrast: f64,
    pub brightness: f64,
    pub colors: [[f64; 3]; 2],
    pub opacity: f64,
    pub blend: String,
    /// D-319: the composition works in Float depth, so the pattern is not held to white.
    pub float: bool,
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
    let mixer = if f.float && f.blend == "screen" { screen_float } else { mixer };
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let a = px[3] as f64;
            if a <= 0.0 {
                return;
            }
            let x = ((i % w) as f64 - ox as f64 + 0.5 - f.offset[0]) / f.size;
            let y = ((i / w) as f64 - oy as f64 + 0.5 - f.offset[1]) / f.size_y;
            let n = fractal_with(base, 0, (x, y, f.z), f.octaves, f.look);
            let n = if f.invert { -n } else { n };
            let v = 0.5 + 0.5 * n * f.contrast / 100.0 + f.brightness / 100.0;
            // D-319: in Float only black holds; past white stays past white.
            let v = if f.float { v.max(0.0) } else { v.clamp(0.0, 1.0) };
            for c in 0..3 {
                let [d, l] = [f.colors[0][c], f.colors[1][c]];
                let color = to_linear(d + v * (l - d));
                let b = px[c] as f64 / a;
                px[c] = ((b + o * (mixer(b, color) - b)) * a) as f32;
            }
        });
}

/// D-446: [`value`] for seed 0, smooth, with its slope across and down from the smoothing
/// curve's own slope 30 t^2 (1 - t)^2, over all eight corners.
pub(crate) fn value_slope(base: u64, ch: u64, x: f64, y: f64, z: f64) -> [f64; 3] {
    let (i, j, k) = (x.floor(), y.floor(), z.floor());
    let (tx, ty, tz) = (x - i, y - j, z - k);
    let fade = |t: f64| t * t * t * (t * (6.0 * t - 15.0) + 10.0);
    let slope = |t: f64| 30.0 * t * t * (1.0 - t) * (1.0 - t);
    let (sx, sy, sz) = (fade(tx), fade(ty), fade(tz));
    let (dx, dy) = (slope(tx), slope(ty));
    let (i, j, k) = (i as i64, j as i64, k as i64);
    let (mut v, mut gx, mut gy) = (0.0, 0.0, 0.0);
    for corner in 0..8 {
        let (a, b, c) = (corner & 1, (corner >> 1) & 1, corner >> 2);
        let wx = if a == 1 { sx } else { 1.0 - sx };
        let wy = if b == 1 { sy } else { 1.0 - sy };
        let wz = if c == 1 { sz } else { 1.0 - sz };
        let n = unit(base, i + a, j + b, k + c, ch);
        v += wx * wy * wz * n;
        gx += (if a == 1 { dx } else { -dx }) * wy * wz * n;
        gy += wx * (if b == 1 { dy } else { -dy }) * wz * n;
    }
    [v, gx, gy]
}

/// D-446: Curl Noise's settings, read once for a frame: the field's size, how far it has slid,
/// its depth, the four octaves' weights, the across and down bias, the share of blocks in the
/// seed and the swirl.
pub(crate) struct Curl {
    pub size: f64,
    pub shift: [f64; 2],
    pub z: f64,
    pub amps: [f64; 4],
    pub bias: [f64; 2],
    pub edge: f64,
    pub swirl: f64,
}

impl Curl {
    pub(crate) fn new(e: &crate::effects::Effect) -> Option<Curl> {
        let crate::effects::Effect::CurlNoise {
            speed, direction, size, offset, evolution, turbulence_speed, swirl, density, smoothness, vertical_bias, edge_definition, frame, ..
        } = e
        else {
            return None;
        };
        let t = direction.to_radians();
        let f = *frame as f64;
        let drift = [t.sin() * speed / 10.0, -t.cos() * speed / 10.0];
        let r = 0.7 - 0.4 * smoothness / 100.0;
        let b = vertical_bias / 100.0;
        Some(Curl {
            size: size * 2f64.powf(-density / 50.0),
            shift: [offset[0] + drift[0] * f, offset[1] + drift[1] * f],
            z: (evolution + turbulence_speed * f / 10.0) / 360.0,
            amps: [1.0, r, r * r, r * r * r],
            bias: [(2.0 * (1.0 - b)).min(1.0), (2.0 * b).min(1.0)],
            edge: edge_definition / 100.0,
            swirl: *swirl,
        })
    }

    /// The flow across and down (one long, or nought), the noise and the seed at the drawing's
    /// point `(px, py)`, in single precision as the card keeps them.
    pub(crate) fn at(&self, px: f64, py: f64) -> [f32; 4] {
        let base = mix(0);
        let (x, y, s) = (px - self.shift[0], py - self.shift[1], self.size);
        let (mut n, mut gx, mut gy, mut total, mut fine) = (0.0, 0.0, 0.0, 0.0, 1.0);
        for (o, amp) in self.amps.iter().enumerate() {
            let [v, a, b] = value_slope(base, 8 * o as u64, x / s * fine, y / s * fine, self.z * fine);
            n += amp * v;
            gx += amp * fine * a;
            gy += amp * fine * b;
            total += amp;
            fine *= 2.0;
        }
        let (n, gx, gy) = (n / total, gx / total / s, gy / total / s);
        let t = (-90.0 + self.swirl * n).to_radians();
        let (c, sn) = (t.cos(), t.sin());
        let vx = (c * gx - sn * gy) * self.bias[0];
        let vy = (sn * gx + c * gy) * self.bias[1];
        let length = vx.hypot(vy);
        let d = if length > 0.0 { [vx / length, vy / length] } else { [0.0, 0.0] };
        let cell = s / 50.0;
        let (sx, sy, e) = (x / cell, y / cell, self.edge);
        let seed = if e == 1.0 {
            value(base, 7, sx, sy, self.z, true, 0)
        } else if e == 0.0 {
            value(base, 7, sx, sy, self.z, false, 0)
        } else {
            e * value(base, 7, sx, sy, self.z, true, 0) + (1.0 - e) * value(base, 7, sx, sy, self.z, false, 0)
        };
        [d[0] as f32, d[1] as f32, n as f32, seed as f32]
    }
}

/// D-446: the margin round the buffer the field is worked over, so a flow line from any pixel
/// stays inside it.
pub(crate) fn curl_margin(radius: f64) -> usize {
    radius.ceil() as usize + 2
}

/// D-446: After Effects' Curl Noise, by document 21's rule: the field worked for every pixel of
/// `source` and the margin round it, in the drawing's own space (its corner at `(ox, oy)`),
/// then at each pixel that shows the flow lines (Final Render), the noise (Input Noise) or the
/// flow and noise as colours (Curl Generation), through Contrast and Brightness, into the
/// channel chosen. The settings are already valid.
pub(crate) fn curl_noise(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    let crate::effects::Effect::CurlNoise {
        sample_count, sample_radius, flow_softness, flow_falloff, view, contrast, brightness, clip_hdr_results, channel, float, ..
    } = e
    else {
        return;
    };
    let Some(curl) = Curl::new(e) else { return };
    let (radius, steps) = (*sample_radius, sample_count.floor());
    let m = curl_margin(radius);
    let (w, h) = (source.width(), source.height());
    let (gw, gh) = (w + 2 * m, h + 2 * m);
    let mut field = WorkingBuffer::transparent(gw, gh);
    field.data_mut().par_chunks_mut(gw * 4).enumerate().for_each(|(gy, row)| {
        for (gx, px) in row.chunks_exact_mut(4).enumerate() {
            let p = curl.at(gx as f64 - m as f64 - ox as f64 + 0.5, gy as f64 - m as f64 - oy as f64 + 0.5);
            px.copy_from_slice(&p);
        }
    });
    let view = crate::effects::CURL_VIEWS.iter().position(|v| v == view).unwrap_or(0);
    let channel = crate::effects::CURL_CHANNELS.iter().position(|c| c == channel).unwrap_or(0);
    let (fs, ff) = (flow_softness / 100.0, flow_falloff / 100.0);
    let hold = !(clip_hdr_results == "off" && *float);
    let field = &field;
    source.data_mut().par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        for (x, px) in row.chunks_exact_mut(4).enumerate() {
            let a = px[3] as f64;
            if a <= 0.0 {
                continue;
            }
            let [dx, dy, n, seed] = field.pixel(x + m, y + m).map(|v| v as f64);
            let o = match view {
                1 => [0.5 + 0.5 * n; 3],
                2 => [0.5 + 0.5 * dx, 0.5 + 0.5 * dy, 0.5 + 0.5 * n],
                _ => {
                    let step = radius / steps;
                    let (mut sw, mut sws, mut ss) = (1.0, 1.0, seed);
                    for way in [1.0, -1.0] {
                        let (mut qx, mut qy) = ((x + m) as f64 + 0.5, (y + m) as f64 + 0.5);
                        let (mut ex, mut ey) = (dx, dy);
                        for i in 1..=steps as usize {
                            let length = ex.hypot(ey);
                            if length < 1e-6 {
                                break;
                            }
                            qx += way * step * ex / length;
                            qy += way * step * ey / length;
                            let s = crate::render::sample_bilinear(field, qx, qy);
                            (ex, ey) = (s[0] as f64, s[1] as f64);
                            let wt = 1.0 - i as f64 / (steps + 1.0);
                            ss += wt * s[3] as f64;
                            sw += wt;
                            sws += wt * wt;
                        }
                    }
                    let lines = ss / sw * (sw / sws.sqrt());
                    let mixed = n + (1.0 - fs) * (radius / 8.0).min(1.0) * (lines - n);
                    [0.5 + 0.5 * (mixed * (1.0 - ff * (0.5 - 0.5 * n))); 3]
                }
            };
            let o = o.map(|v| {
                let v = 0.5 + (v - 0.5) * contrast / 100.0 + brightness / 100.0;
                if hold { v.clamp(0.0, 1.0) } else { v.max(0.0) }
            });
            match channel {
                0 => {
                    for c in 0..3 {
                        px[c] = (to_linear(o[c]) * a) as f32;
                    }
                }
                4 => {
                    let t = a * ((o[0] + o[1] + o[2]) / 3.0).clamp(0.0, 1.0);
                    for c in 0..3 {
                        px[c] = (px[c] as f64 / a * t) as f32;
                    }
                    px[3] = t as f32;
                }
                c => px[c - 1] = (to_linear(o[c - 1]) * a) as f32,
            }
        }
    });
}

/// D-447: Brush Strokes' numbers, as the processor and the card share them: the stroke
/// direction `d` (across it is (-d.y, d.x)), the cells' length `su` and width `sv`, the stroke
/// length, the brush size, min(randomness, 1), the largest slope, the largest sine of it, how far
/// past its line a stroke can reach, the hash's seed and frame, the surface's number in
/// [`crate::effects::BRUSH_SURFACES`] and the blend out of 1.
pub(crate) struct Brush {
    pub d: [f64; 2],
    pub su: f64,
    pub sv: f64,
    pub len: f64,
    pub r: f64,
    pub m: f64,
    pub slope: f64,
    pub s: f64,
    pub reach: f64,
    pub base: u64,
    pub frame: i64,
    pub surface: usize,
    pub blend: f64,
}

impl Brush {
    pub(crate) fn new(e: &crate::effects::Effect) -> Option<Brush> {
        let crate::effects::Effect::BrushStrokes {
            stroke_angle, brush_size, stroke_length, stroke_density, stroke_randomness, paint_surface, blend_with_original, random_seed, frame, ..
        } = e
        else {
            return None;
        };
        let t = stroke_angle.to_radians();
        let (r, len, density) = (*brush_size, *stroke_length, *stroke_density);
        let slope = stroke_randomness * 10f64.to_radians().tan();
        Some(Brush {
            d: [t.sin(), -t.cos()],
            su: (len.max(2.0 * r) / density).max(1.0),
            sv: (2.0 * r / density).max(1.0),
            len,
            r,
            m: stroke_randomness.min(1.0),
            slope,
            s: slope / (1.0 + slope * slope).sqrt(),
            reach: 1.25 * r + 0.5,
            base: mix(random_seed.floor() as u64),
            frame: *frame as i64,
            surface: crate::effects::BRUSH_SURFACES.iter().position(|v| v == paint_surface).unwrap_or(0),
            blend: blend_with_original / 100.0,
        })
    }
}

/// D-447: Brush Strokes (document 21). Each pixel, at `q` in the drawing's own space (its corner
/// at `(ox, oy)` in `source`), looks at the cells of strokes that can reach it on the grid turned
/// to the stroke direction, keeps the two highest in the pile that cover it, and lays them, the
/// lower first, on the paint surface; then mixes back towards itself by the blend. A stroke's
/// colour is `source`'s pixel under its start, held inside the buffer. The layer never grows.
pub(crate) fn brush_strokes(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    let Some(b) = Brush::new(e) else { return };
    let (w, h) = (source.width(), source.height());
    let src = source.clone();
    let n = [-b.d[1], b.d[0]];
    source.data_mut().par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        for (x, px) in row.chunks_exact_mut(4).enumerate() {
            let p = src.pixel(x, y).map(|v| v as f64);
            let (qx, qy) = (x as f64 + 0.5 - ox as f64, y as f64 + 0.5 - oy as f64);
            let (pu, pv) = (qx * b.d[0] + qy * b.d[1], qx * n[0] + qy * n[1]);
            // The two highest strokes covering the pixel: their place in the pile, colour and
            // covering.
            let mut top: [(f64, [f64; 4], f64); 2] = [(-1.0, [0.0; 4], 0.0); 2];
            let i0 = ((pu - b.len - b.reach) / b.su).floor() as i64 - 1;
            let i1 = ((pu + b.reach) / b.su).floor() as i64;
            let j0 = ((pv - b.reach - b.len * b.s) / b.sv).floor() as i64 - 1;
            let j1 = ((pv + b.reach + b.len * b.s) / b.sv).floor() as i64;
            for i in i0..=i1 {
                for j in j0..=j1 {
                    let g = mix(mix(mix(b.base ^ i as u64) ^ j as u64) ^ b.frame as u64);
                    let hk = |k: u64| (mix(g ^ k) >> 11) as f64 / (1u64 << 53) as f64;
                    let cu = (i as f64 + 0.5 + (hk(0) - 0.5) * b.m) * b.su;
                    let cv = (j as f64 + 0.5 + (hk(1) - 0.5) * b.m) * b.sv;
                    let t = (2.0 * hk(2) - 1.0) * b.slope;
                    let el = (1.0 + t * t).sqrt();
                    let (eu, ev) = (1.0 / el, t / el);
                    let len = b.len * (1.0 - 0.5 * b.m * hk(3));
                    let rho = b.r * (1.0 + 0.25 * b.m * (2.0 * hk(4) - 1.0));
                    let (wu, wv) = (pu - cu, pv - cv);
                    let a = (wu * eu + wv * ev).max(0.0).min(len);
                    let cov = (rho + 0.5 - (wu - a * eu).hypot(wv - a * ev)).clamp(0.0, 1.0);
                    if cov <= 0.0 {
                        continue;
                    }
                    let pile = hk(5);
                    if pile <= top[1].0 {
                        continue;
                    }
                    let sx = ((cu * b.d[0] + cv * n[0] + ox as f64).floor().max(0.0) as usize).min(w - 1);
                    let sy = ((cu * b.d[1] + cv * n[1] + oy as f64).floor().max(0.0) as usize).min(h - 1);
                    let stroke = (pile, src.pixel(sx, sy).map(|v| v as f64), cov);
                    if pile > top[0].0 {
                        top = [stroke, top[0]];
                    } else {
                        top[1] = stroke;
                    }
                }
            }
            let mut o = match b.surface {
                1 => [0.0; 4],
                2 => [1.0; 4],
                3 => [0.0, 0.0, 0.0, 1.0],
                _ => p,
            };
            for (pile, col, cov) in [top[1], top[0]] {
                if pile >= 0.0 {
                    o = std::array::from_fn(|c| col[c] * cov + o[c] * (1.0 - cov * col[3]));
                }
            }
            for c in 0..4 {
                px[c] = (o[c] + b.blend * (p[c] - o[c])) as f32;
            }
        }
    });
}

/// D-209: each cell's point and grey for Cell Pattern over a `w` by `h` buffer, worked once for
/// the cells the buffer sees and two round them: the first cell across and down, the cells
/// across, and the points row by row. The numbers are the disperse, size, evolution in degrees
/// and seed. B-151: the card reads the same points.
pub(crate) fn cell_points(
    [disperse, size, evolution, seed]: [f64; 4],
    (w, h): (usize, usize),
    (ox, oy): (usize, usize),
) -> (i64, i64, usize, Vec<(f64, f64, f64)>) {
    let (base, t) = (mix(seed.floor() as u64), evolution.to_radians());
    let at = |x: usize, o: usize| (x as f64 - o as f64 + 0.5) / size;
    let (m0, n0) = (at(0, ox).floor() as i64 - 2, at(0, oy).floor() as i64 - 2);
    let cols = (at(w - 1, ox).floor() as i64 + 3 - m0) as usize;
    let rows = (at(h - 1, oy).floor() as i64 + 3 - n0) as usize;
    let points = (0..cols * rows)
        .into_par_iter()
        .map(|k| {
            let (m, n) = (m0 + (k % cols) as i64, n0 + (k / cols) as i64);
            let u = [0, 1, 2, 3].map(|c| unit(base, m, n, 0, c));
            let r = disperse / 2.0 * (u[0] + 1.0) / 2.0;
            let a = std::f64::consts::PI * u[1] + if u[2] >= 0.0 { t } else { -t };
            (m as f64 + 0.5 + r * a.cos(), n as f64 + 0.5 + r * a.sin(), (u[3] + 1.0) / 2.0)
        })
        .collect();
    (m0, n0, cols, points)
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
    let at = |x: usize, o: usize| (x as f64 - o as f64 + 0.5) / size;
    let (m0, n0, cols, points) = cell_points([disperse, size, evolution, seed], (w, h), (ox, oy));
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
        let t = ramp_lightness(b);
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

/// D-129: where a straight linear colour sits on a ramp: its luma held inside 0 to 1, encoded.
fn ramp_lightness(b: [f64; 3]) -> f64 {
    to_srgb((0.2126 * b[0] + 0.7152 * b[1] + 0.0722 * b[2]).clamp(0.0, 1.0))
}

/// D-369: each pixel that shows takes the colour at its lightness on a ramp of `stops`, encoded,
/// dark to light and evenly spaced, as Gradient Map reads lightness.
pub(crate) fn toner(source: &mut WorkingBuffer, stops: &[[f64; 3]]) {
    let n = (stops.len() - 1) as f64;
    each_pixel(source, |px| {
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        let t = ramp_lightness([0, 1, 2].map(|c| px[c] as f64 / a)) * n;
        let k = (t.floor()).min(n - 1.0);
        let (lo, hi, s) = (stops[k as usize], stops[k as usize + 1], t - k);
        for c in 0..3 {
            px[c] = (to_linear((lo[c] + s * (hi[c] - lo[c])).clamp(0.0, 1.0)) * a) as f32;
        }
    });
}

/// D-316: a pixel's phase as Colorama reads it, 0 to 1: its straight colour encoded, then the
/// mean of the three (`intensity`), the encoded luma (`luminance`), one channel, or its alpha.
/// D-336 adds HSL `hue` (a turn as 0 to 1), `lightness` and `saturation`, for CC Vector Blur.
pub(crate) fn phase_of(get: &str, px: &[f32]) -> f64 {
    let a = px[3] as f64;
    if get == "alpha" {
        return a;
    }
    if a <= 0.0 {
        return 0.0;
    }
    let b = [0, 1, 2].map(|c| (px[c] as f64 / a).clamp(0.0, 1.0));
    let c = b.map(to_srgb);
    match get {
        "red" => c[0],
        "green" => c[1],
        "blue" => c[2],
        "luminance" => to_srgb(0.2126 * b[0] + 0.7152 * b[1] + 0.0722 * b[2]),
        "hue" => to_hsl(c)[0] / 360.0,
        "saturation" => to_hsl(c)[1],
        "lightness" => to_hsl(c)[2],
        // D-381: the largest encoded channel, and nothing.
        "value" => c[0].max(c[1]).max(c[2]),
        "zero" => 0.0,
        _ => (c[0] + c[1] + c[2]) / 3.0,
    }
}

/// D-381: Colorama's settings, read once for a frame, words as written and valid: the ring's
/// colours encoded and its opacities as shares, as many as its stops; the matching colour
/// encoded, its place in `effects::CHANGE_MATCHES`, the tolerance and softness as shares, or
/// `None` with matching off; the blend in per cent.
pub(crate) struct Colorama<'a> {
    pub get: &'a str,
    pub add_from: &'a str,
    pub add_mode: &'a str,
    pub shift: f64,
    pub repetitions: f64,
    pub ring: &'a [[f64; 3]],
    pub opacity: &'a [f64],
    pub interpolate: bool,
    pub modify: &'a str,
    pub modify_alpha: bool,
    pub change_empty: bool,
    pub matching: Option<([f64; 3], usize, f64, f64)>,
    pub masking: &'a str,
    pub composite: bool,
    pub blend: f64,
}

/// D-370: how near a distance `d` is, 1 within the tolerance `t`, falling straight to 0 across
/// the softness `w`, 0 beyond.
pub(crate) fn nearness(d: f64, t: f64, w: f64) -> f64 {
    if d <= t {
        1.0
    } else if w == 0.0 || d >= t + w {
        0.0
    } else {
        1.0 - (d - t) / w
    }
}

/// D-316: each pixel that shows takes the colour at its place round the ring (2 to 5 colours,
/// encoded, evenly spaced, the last running back into the first). Its place is its phase, read by
/// `get`, plus the phase of the first map under it (the maps lying on the drawing, whose corner is
/// at `origin`; clear outside it, and weighted by its covering), times the repetitions, plus the
/// shift in degrees over 360, and only the part past the whole number counts. The colour is mixed
/// back toward the pixel's own by `blend` per cent.
///
/// D-381: the first map read by `add_from` and added by `add_mode`; the ring held at its colours
/// without `interpolate`; the pixel given only what `modify` names of the ring's colour, and with
/// `modify_alpha` the ring's opacity as its covering, the empty pixels too with `change_empty`;
/// the change weighed by the matching colour's nearness and the second map, by `masking`, and
/// laid over the pixel, or alone without `composite`. With these at their start it is D-316's
/// sum exactly.
pub(crate) fn colorama(source: &mut WorkingBuffer, maps: [Option<&WorkingBuffer>; 2], (ox, oy): (usize, usize), c: &Colorama) {
    let (n, o) = (c.ring.len(), c.blend / 100.0);
    let w = source.width();
    let empty = c.modify_alpha && c.change_empty;
    let under = |m: &WorkingBuffer, x: usize, y: usize| {
        let (mx, my) = (x.wrapping_sub(ox), y.wrapping_sub(oy));
        if mx < m.width() && my < m.height() { m.pixel(mx, my) } else { [0.0; 4] }
    };
    source.data_mut().par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        for (x, px) in row.chunks_exact_mut(4).enumerate() {
            let a = px[3] as f64;
            if a <= 0.0 && !empty {
                continue;
            }
            let mut phase = phase_of(c.get, px);
            if let Some(m) = maps[0] {
                let q = under(m, x, y);
                let q = phase_of(c.add_from, &q) * if c.add_from == "alpha" { 1.0 } else { q[3] as f64 };
                phase = match c.add_mode {
                    "clamp" => (phase + q).min(1.0),
                    "average" => (phase + q) / 2.0,
                    "screen" => phase + q - phase * q,
                    _ => phase + q,
                };
            }
            let t = phase * c.repetitions + c.shift / 360.0;
            let p = (t - t.floor()) * n as f64;
            let (i, j) = (p.floor() as usize % n, (p.floor() as usize + 1) % n);
            let f = if c.interpolate { p - p.floor() } else { 0.0 };
            let m: [f64; 3] = std::array::from_fn(|ch| c.ring[i][ch] + f * (c.ring[j][ch] - c.ring[i][ch]));
            let b = [0, 1, 2].map(|ch| if a > 0.0 { px[ch] as f64 / a } else { 0.0 });
            // Encoded only when read, so D-316's sum below costs what it did.
            let e = || b.map(|v| to_srgb(v.clamp(0.0, 1.0)));
            let mut weight = c.matching.map_or(1.0, |(colour, kind, t, s)| nearness(change_distance(e(), colour, kind), t, s));
            if let Some(m) = maps[1] {
                let q = under(m, x, y);
                let lum = phase_of("luminance", &q) * q[3] as f64;
                weight *= match c.masking {
                    "inverse_luminance" => 1.0 - lum,
                    "alpha" => q[3] as f64,
                    "inverse_alpha" => 1.0 - q[3] as f64,
                    _ => lum,
                };
            }
            if c.modify == "all" && !c.modify_alpha && weight == 1.0 {
                // D-316's own sum, so a project from before D-381 draws exactly as it did.
                for ch in 0..3 {
                    let g = to_linear(m[ch]);
                    px[ch] = ((g + o * (b[ch] - g)) * a) as f32;
                }
                continue;
            }
            let e = e();
            let g = match c.modify {
                "all" => m,
                "none" => e,
                "red" => [m[0], e[1], e[2]],
                "green" => [e[0], m[1], e[2]],
                "blue" => [e[0], e[1], m[2]],
                word => {
                    let (he, hm) = (to_hsl(e), to_hsl(m));
                    from_hsl(match word {
                        "hue" => [hm[0], he[1], he[2]],
                        "lightness" => [he[0], he[1], hm[2]],
                        _ => [he[0], hm[1], he[2]],
                    })
                }
            }
            .map(|v| to_linear(v.clamp(0.0, 1.0)));
            let a2 = if c.modify_alpha { c.opacity[i] + f * (c.opacity[j] - c.opacity[i]) } else { a };
            let r = [g[0] * a2, g[1] * a2, g[2] * a2, a2];
            for ch in 0..4 {
                let was = px[ch] as f64;
                let laid = if c.composite { was + weight * (r[ch] - was) } else { weight * r[ch] };
                px[ch] = (laid + o * (was - laid)) as f32;
            }
        }
    });
}

/// D-130: red, green and blue pushed by `shadows`, `midtones` and `highlights`, each -100..100,
/// through the sRGB curve, weighted by how dark or light the pixel is. The settings are already
/// valid; all nine 0 changes nothing.
///
/// D-295: with `keep` (Preserve Luminosity), the pushed colour is moved back to the pixel's own
/// luma and pulled in toward that grey until it fits 0 to 1, by the compositing spec's SetLum
/// and ClipColor with these weights. Black and white stay black and white.
pub(crate) fn color_balance(source: &mut WorkingBuffer, shadows: [f64; 3], midtones: [f64; 3], highlights: [f64; 3], keep: bool) {
    if [shadows, midtones, highlights] == [[0.0; 3]; 3] {
        return;
    }
    let luma = |e: [f64; 3]| 0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2];
    grade_pixels(source, false, |_, e| {
        let l = luma(e);
        let ws = (1.0 - 2.0 * l).clamp(0.0, 1.0);
        let wh = (2.0 * l - 1.0).clamp(0.0, 1.0);
        let wm = 1.0 - ws - wh;
        let o: [f64; 3] = std::array::from_fn(|c| e[c] + (ws * shadows[c] + wm * midtones[c] + wh * highlights[c]) / 200.0);
        if !keep {
            return o;
        }
        let d = l - luma(o);
        let o = o.map(|v| v + d);
        let (n, x) = (o[0].min(o[1]).min(o[2]), o[0].max(o[1]).max(o[2]));
        let s = if n < 0.0 { l / (l - n) } else { 1.0 }.min(if x > 1.0 { (1.0 - l) / (x - l) } else { 1.0 });
        o.map(|v| l + (v - l) * s)
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

/// D-562: CC Vignette, `[cx, cy, t, amount, pin]` as `effects::cc_vignette_settings` gives
/// them. At a shown pixel s = |P - c| t, the gain g = (1 + s^2)^(-2 amount) (cos^4 of the angle
/// whose tangent is s, raised to the amount), pinned toward 1 by the encoded luma times `pin`,
/// and each straight channel b becomes min(b G, max(b, 1)): lightening never takes a channel past
/// 1, nor pulls one already past it back. Amount 0 or angle of view 0 change nothing.
pub(crate) fn cc_vignette(source: &mut WorkingBuffer, [cx, cy, t, amount, pin]: [f64; 5]) {
    if amount == 0.0 || t == 0.0 {
        return;
    }
    let w = source.width();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        let s = ((i % w) as f64 + 0.5 - cx).hypot((i / w) as f64 + 0.5 - cy) * t;
        let g = (1.0 + s * s).powf(-2.0 * amount);
        let b: [f64; 3] = std::array::from_fn(|c| px[c] as f64 / a);
        let gain = g + (1.0 - g) * luma(b.map(|v| to_srgb(v.clamp(0.0, 1.0)))) * pin;
        for c in 0..3 {
            px[c] = ((b[c] * gain).min(b[c].max(1.0)) * a) as f32;
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

/// D-468/D-469: Threshold's rule on a chosen channel, a second engine beside [`threshold`] (which
/// stays as it is). Each of the three decisions is 1 where 255 times the value plus 1e-4 reaches
/// its level in `levels`, else 0, turned over where `inverts` says. "luminance" decides the
/// encoded luma and "saturation" the HSL saturation, once for all three channels; "rgb" decides
/// each encoded channel by its own level; "alpha" decides the covering, the colour kept. The
/// result is mixed back toward the straight linear colour by `keep`, 1 for none of the layer and
/// 0 for all of it. A pixel that does not show is left as it is.
pub(crate) fn channel_threshold(source: &mut WorkingBuffer, levels: [f64; 3], channel: &str, inverts: [bool; 3], keep: f64) {
    let decide = |v: f64, c: usize| if (255.0 * v + 1e-4 >= levels[c]) != inverts[c] { 1.0 } else { 0.0 };
    each_pixel(source, |px| {
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        let b: [f64; 3] = std::array::from_fn(|c| (px[c] as f64 / a).clamp(0.0, 1.0));
        if channel == "alpha" {
            let n = a + keep * (decide(a, 0) - a);
            for c in 0..3 {
                px[c] = (b[c] * n) as f32;
            }
            px[3] = n as f32;
            return;
        }
        let e = b.map(to_srgb);
        let m = match channel {
            "rgb" => [decide(e[0], 0), decide(e[1], 1), decide(e[2], 2)],
            "saturation" => [decide(to_hsl(e)[1], 0); 3],
            _ => [decide(luma(e), 0); 3],
        };
        for c in 0..3 {
            px[c] = ((b[c] + keep * (m[c] - b[c])) * a) as f32;
        }
    });
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

/// D-305: each of red, green, blue and alpha, in that order in `take`, taken from one of
/// [`crate::effects::SHIFT_CHANNELS_FROM`] of the straight encoded colour: a channel, its luma,
/// HSL hue (a turn as 0 to 1), lightness or saturation, or 1, 0.5 or 0. The new colour is
/// multiplied by the new alpha; a clear pixel's colour is black.
pub(crate) fn shift_channels(source: &mut WorkingBuffer, take: [&str; 4]) {
    if take == ["red", "green", "blue", "alpha"] {
        return;
    }
    each_pixel(source, |px| {
        let a = px[3] as f64;
        let e = if a > 0.0 { [0, 1, 2].map(|c| to_srgb((px[c] as f64 / a).clamp(0.0, 1.0))) } else { [0.0; 3] };
        let (mx, mn) = (e[0].max(e[1]).max(e[2]), e[0].min(e[1]).min(e[2]));
        let l = (mx + mn) / 2.0;
        let from = |w: &str| match w {
            "red" => e[0],
            "green" => e[1],
            "blue" => e[2],
            "alpha" => a,
            "luminance" => luma(e),
            "hue" => hsv_hue(e).unwrap_or(0.0) / 360.0,
            "lightness" => l,
            "saturation" if mx > mn => (mx - mn) / (1.0 - (2.0 * l - 1.0).abs()),
            "full" => 1.0,
            "half" => 0.5,
            _ => 0.0,
        };
        let na = from(take[3]).clamp(0.0, 1.0);
        for c in 0..3 {
            px[c] = (to_linear(from(take[c]).clamp(0.0, 1.0)) * na) as f32;
        }
        px[3] = na as f32;
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

/// D-365: an encoded colour's brightness and colour swing as composite video weighs them, Y and
/// the length of (U, V).
fn video(e: [f64; 3]) -> (f64, f64) {
    let y = 0.299 * e[0] + 0.587 * e[1] + 0.114 * e[2];
    let (u, v) = (0.492 * (e[2] - y), 0.877 * (e[0] - y));
    (y, (u * u + v * v).sqrt())
}

/// D-365: a pixel is unsafe when the signal's top, `setup + (100 - setup)(Y + C)` IRE, is above
/// `max` by more than a ten-thousandth of an IRE (so a rounding cannot flip a white at a limit
/// of 100). The reduce methods darken it (`reduce_luminance`) or move it toward its grey
/// (`reduce_saturation`) until its top is on the limit; `key_out_unsafe` clears the unsafe
/// pixels and `key_out_safe` the rest, leaving the others exactly as they are.
pub(crate) fn broadcast_safe(source: &mut WorkingBuffer, setup: f64, method: &str, max: f64) {
    let over = |y: f64, c: f64| setup + (100.0 - setup) * (y + c) > max + 1e-4;
    if method.starts_with("key_out") {
        let unsafe_out = method == "key_out_unsafe";
        return each_pixel(source, |px| {
            let a = px[3] as f64;
            if a <= 0.0 {
                return;
            }
            let (y, c) = video(std::array::from_fn(|i| to_srgb((px[i] as f64 / a).clamp(0.0, 1.0))));
            if over(y, c) == unsafe_out {
                px.fill(0.0);
            }
        });
    }
    let m = (max - setup) / (100.0 - setup);
    let darken = method == "reduce_luminance";
    grade_pixels(source, false, |_, e| {
        let (y, c) = video(e);
        if !over(y, c) {
            e
        } else if darken {
            e.map(|v| v * m / (y + c))
        } else if y < m && c > 0.0 {
            e.map(|v| y + (v - y) * (m - y) / c)
        } else {
            [m; 3]
        }
    })
}

/// D-366: each pixel's place `t` between `black` and `white` by its encoded luma (a step at
/// `black` when `white` is not above it, a ten-thousandth of a level early as D-138's), and the correction there, from `d[0]` at 0 straight to
/// `d[1]` at a half and `d[2]` at 1, added; with `pinning`, faded in from black and white over
/// `pinning / 200` of the way. All three corrections 0 changes nothing.
pub(crate) fn color_neutralizer(source: &mut WorkingBuffer, d: [[f64; 3]; 3], pinning: f64, black: f64, white: f64) {
    if d == [[0.0; 3]; 3] {
        return;
    }
    let p = pinning / 200.0;
    grade_pixels(source, false, |_, e| {
        let v = 255.0 * luma(e);
        let t = if white > black {
            ((v - black) / (white - black)).clamp(0.0, 1.0)
        } else if v + 1e-4 >= black {
            1.0
        } else {
            0.0
        };
        let w = if p == 0.0 { 1.0 } else { (t.min(1.0 - t) / p).clamp(0.0, 1.0) };
        std::array::from_fn(|c| {
            let k = if t <= 0.5 { d[0][c] + (d[1][c] - d[0][c]) * 2.0 * t } else { d[1][c] + (d[2][c] - d[1][c]) * (2.0 * t - 1.0) };
            e[c] + w * k
        })
    })
}

/// D-367: each channel turned by its phase in degrees, a turn the whole range, what runs past an
/// end brought back by `overflow`: "wrap" round, "solarize" folded (a triangle), or "polarize",
/// the value the height of a point on a half circle turned half a turn a turn. A phase of 0
/// leaves its channel; all three 0 change nothing. A wrapped value within a billionth of a whole
/// number is taken as that number, so a rounding cannot send it round the other way.
pub(crate) fn color_offset(source: &mut WorkingBuffer, phases: [f64; 3], overflow: &str) {
    if phases == [0.0; 3] {
        return;
    }
    let turns = phases.map(|p| std::f64::consts::PI * p / 360.0).map(f64::sin_cos);
    grade_pixels(source, false, |_, e| {
        std::array::from_fn(|c| {
            let (v, f) = (e[c], phases[c] / 360.0);
            let u = v + f;
            let u = if (u - u.round()).abs() < 1e-9 { u.round() } else { u };
            match overflow {
                _ if f == 0.0 => v,
                "polarize" => {
                    let (sin, cos) = turns[c];
                    (1.0 - (1.0 - 2.0 * v) * cos + 2.0 * (v * (1.0 - v)).sqrt() * sin) / 2.0
                }
                "solarize" => 1.0 - (u.rem_euclid(2.0) - 1.0).abs(),
                _ if (0.0..=1.0).contains(&u) => u,
                _ => u - u.floor(),
            }
        })
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
        let rgb = from_hls(h, l, s);
        for i in 0..3 {
            px[i] = (to_linear((e[i] + k * (rgb[i] - e[i])).clamp(0.0, 1.0)) * a) as f32;
        }
    });
}

/// D-197: an encoded colour from its hue in degrees, lightness and saturation.
pub(crate) fn from_hls(h: f64, l: f64, s: f64) -> [f64; 3] {
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
    rgb.map(|v| v + l - chroma / 2.0)
}

/// D-370's settings, read once for a frame: the colour encoded; the hue transform in degrees,
/// lightness and saturation as shares; tolerance and softness as shares; the match's place in
/// `effects::CHANGE_MATCHES`.
pub(crate) struct ChangeColor {
    pub mask: bool,
    pub transforms: [f64; 3],
    pub color: [f64; 3],
    pub tolerance: f64,
    pub softness: f64,
    pub matching: usize,
    pub invert: bool,
}

/// D-370: how far encoded `e` lies from `c`, 0 to about 1: in RGB over the root of 3, in hue
/// over half a turn (1 when either is grey), or in unscaled Cb and Cr.
pub(crate) fn change_distance(e: [f64; 3], c: [f64; 3], matching: usize) -> f64 {
    match matching {
        0 => ((0..3).map(|i| (e[i] - c[i]).powi(2)).sum::<f64>() / 3.0).sqrt(),
        2 => {
            let cbcr = |v: [f64; 3]| {
                let y = 0.2126 * v[0] + 0.7152 * v[1] + 0.0722 * v[2];
                [(v[2] - y) / 1.8556, (v[0] - y) / 1.5748]
            };
            let (p, q) = (cbcr(e), cbcr(c));
            (p[0] - q[0]).hypot(p[1] - q[1])
        }
        _ => match (hsv_hue(e), hsv_hue(c)) {
            (Some(h), Some(g)) => {
                let d = (h - g).abs();
                d.min(360.0 - d) / 180.0
            }
            _ => 1.0,
        },
    }
}

/// D-370: each colour near `color` moved round the wheel and lighter or darker, stronger or
/// weaker, by how near it is; or with `mask` that nearness shown as grey. With every transform 0
/// the layer is left exactly as it is, and so is a pixel not near at all.
pub(crate) fn change_color(source: &mut WorkingBuffer, c: &ChangeColor) {
    if !c.mask && c.transforms == [0.0; 3] {
        return;
    }
    let (t, w) = (c.tolerance, c.softness);
    let push = |v: f64, p: f64| if p >= 0.0 { v + (1.0 - v) * p } else { v * (1.0 + p) };
    each_pixel(source, |px| {
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        let e = [0, 1, 2].map(|i| to_srgb((px[i] as f64 / a).clamp(0.0, 1.0)));
        let mut k = nearness(change_distance(e, c.color, c.matching), t, w);
        if c.invert {
            k = 1.0 - k;
        }
        if c.mask {
            px[..3].fill((to_linear(k) * a) as f32);
            return;
        }
        if k <= 0.0 {
            return;
        }
        let (h, l, s) = hls(e);
        let h = (h.unwrap_or(0.0) + c.transforms[0]).rem_euclid(360.0);
        let rgb = from_hls(h, push(l, c.transforms[1]), push(s, c.transforms[2]));
        for i in 0..3 {
            px[i] = (to_linear((e[i] + k * (rgb[i] - e[i])).clamp(0.0, 1.0)) * a) as f32;
        }
    });
}

/// D-374: Color Balance (HLS), each colour turned `hue` degrees round the wheel, and `light` and
/// `sat`, shares -1 to 1, added to its lightness and saturation, each held inside 0 to 1; a grey
/// stays grey. All three 0 leaves the layer exactly as it is.
pub(crate) fn color_balance_hls(source: &mut WorkingBuffer, hue: f64, light: f64, sat: f64) {
    if (hue, light, sat) == (0.0, 0.0, 0.0) {
        return;
    }
    grade_pixels(source, false, |_, e| {
        let (h, l, s) = hls(e);
        let s = if h.is_some() { (s + sat).clamp(0.0, 1.0) } else { 0.0 };
        from_hls((h.unwrap_or(0.0) + hue).rem_euclid(360.0), (l + light).clamp(0.0, 1.0), s)
    });
}

/// D-382: Gamma/Pedestal/Gain at its start leaves the layer exactly as it is.
pub(crate) fn gamma_pedestal_gain_untouched(stretch: f64, gamma: [f64; 3], pedestal: [f64; 3], gain: [f64; 3]) -> bool {
    stretch == 1.0 && gamma == [1.0; 3] && pedestal == [0.0; 3] && gain == [1.0; 3]
}

/// D-382: Gamma/Pedestal/Gain. Each encoded channel x first stretched to s x / (1 + (s - 1) x),
/// then pedestal + (gain - pedestal) x^(1 / gamma), its own three for red, green and blue, held
/// inside 0 to 1.
pub(crate) fn gamma_pedestal_gain(source: &mut WorkingBuffer, stretch: f64, gamma: [f64; 3], pedestal: [f64; 3], gain: [f64; 3]) {
    if gamma_pedestal_gain_untouched(stretch, gamma, pedestal, gain) {
        return;
    }
    grade_pixels(source, false, |_, e| {
        std::array::from_fn(|c| {
            let x = stretch * e[c] / (1.0 + (stretch - 1.0) * e[c]);
            pedestal[c] + (gain[c] - pedestal[c]) * x.powf(1.0 / gamma[c])
        })
    });
}

/// D-384: Photo Filter at density 0 leaves the layer exactly as it is.
pub(crate) fn photo_filter_untouched(density: f64) -> bool {
    density == 0.0
}

/// D-384: Photo Filter. Each encoded channel multiplied by 1 - d + d F, the filter's colour F
/// laid over at density `d` (0 to 1); with `keep`, the result scaled back to the pixel's own
/// Rec. 709 luma on the encoded values when its own is above 0; held inside 0 to 1.
pub(crate) fn photo_filter(source: &mut WorkingBuffer, filter: [f64; 3], d: f64, keep: bool) {
    if photo_filter_untouched(d) {
        return;
    }
    let luma = |c: [f64; 3]| 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
    grade_pixels(source, false, |_, e| {
        let f: [f64; 3] = std::array::from_fn(|c| e[c] * (1.0 - d + d * filter[c]));
        match keep && luma(f) > 0.0 {
            true => f.map(|v| v * luma(e) / luma(f)),
            false => f,
        }
    });
}

/// D-457: Advanced Spill Suppressor's settings as the card reads them: 1 for Ultra, the
/// suppression as a share, the screen channel and the other two (the key colour's largest, green
/// before blue before red on a tie), the key's HSV hue (-1 for a grey), the tolerance (0 to 100),
/// then the desaturate, spill range, spill colour correction and luma correction as shares.
pub(crate) fn spill_settings(effect: &crate::effects::Effect) -> [f64; 11] {
    let crate::effects::Effect::AdvancedSpillSuppressor {
        method,
        suppression,
        key_color,
        tolerance,
        desaturate,
        spill_range,
        spill_color_correction,
        luma_correction,
    } = effect
    else {
        unreachable!("spill_settings takes Advanced Spill Suppressor")
    };
    let key = crate::effects::encoded(key_color);
    let s = [1, 2, 0].into_iter().fold(1, |s, i| if key[i] > key[s] { i } else { s });
    let [p, q] = match s {
        0 => [1, 2],
        1 => [0, 2],
        _ => [0, 1],
    };
    [
        (method == "ultra") as u8 as f64,
        suppression / 100.0,
        s as f64,
        p as f64,
        q as f64,
        hsv_hue(key).unwrap_or(-1.0),
        *tolerance,
        desaturate / 100.0,
        spill_range / 100.0,
        spill_color_correction / 100.0,
        luma_correction / 100.0,
    ]
}

/// D-457: Advanced Spill Suppressor with the suppression 0, or Ultra with the tolerance 0,
/// leaves the layer exactly as it is.
pub(crate) fn spill_untouched(k: &[f64; 11]) -> bool {
    k[1] == 0.0 || (k[0] != 0.0 && k[6] == 0.0)
}

/// D-457: Advanced Spill Suppressor, the rule ours (Adobe publishes none). On the encoded colour
/// e, with S the suppression: Standard takes S times the excess of green, or of blue, over the
/// larger other channel off that channel (at most one has one). Ultra, on the screen channel s
/// and the other two p, q (`spill_settings`): the hue weight w = max(0, 1 - d / (1.8 T)), d the
/// distance round the wheel from the key's hue, 0 for a grey; the limit L = hi - R (hi - lo) on
/// the larger and smaller of e_p and e_q; x = S w max(0, e_s - L) taken off e_s and C x / 2
/// added to each of e_p and e_q; the colour moved toward its luma by D x / e_s; then each
/// channel raised by U times the luma lost. Held inside 0 to 1.
pub(crate) fn advanced_spill_suppressor(source: &mut WorkingBuffer, k: &[f64; 11]) {
    if spill_untouched(k) {
        return;
    }
    let sup = k[1];
    if k[0] == 0.0 {
        grade_pixels(source, false, |_, e| {
            let mut o = e;
            for (s, p, q) in [(1, 0, 2), (2, 0, 1)] {
                let excess = e[s] - e[p].max(e[q]);
                if excess > 0.0 {
                    o[s] = e[s] - sup * excess;
                }
            }
            o
        });
        return;
    }
    let (s, p, q, hk) = (k[2] as usize, k[3] as usize, k[4] as usize, k[5]);
    grade_pixels(source, false, |_, e| {
        let Some(h) = hsv_hue(e).filter(|_| hk >= 0.0) else {
            return e;
        };
        let d = ((h - hk + 180.0).rem_euclid(360.0) - 180.0).abs();
        let w = (1.0 - d / (1.8 * k[6])).max(0.0);
        let (hi, lo) = (e[p].max(e[q]), e[p].min(e[q]));
        let limit = hi - k[8] * (hi - lo);
        let x = sup * w * (e[s] - limit).max(0.0);
        let mut o = e;
        o[s] -= x;
        let give = k[9] * x / 2.0;
        o[p] += give;
        o[q] += give;
        let f = if e[s] > 0.0 { x / e[s] } else { 0.0 };
        let y = luma(o);
        let t = k[7] * f;
        let o = o.map(|v| v + (y - v) * t);
        let lift = k[10] * (luma(e) - luma(o));
        o.map(|v| v + lift)
    });
}

/// D-459: Color Difference Key's settings as the card reads them: the view's place in
/// [`crate::effects::CDK_VIEWS`], the key colour's HSV hue (0 for a grey), partial A's five
/// levels, partial B's, then the matte's five (its output always 0 to 255).
pub(crate) fn cdk_settings(effect: &crate::effects::Effect) -> [f64; 17] {
    let crate::effects::Effect::ColorDifferenceKey { view, key_color, partial_a, partial_b, matte } = effect else {
        unreachable!("cdk_settings takes Color Difference Key")
    };
    let mut k = [0.0; 17];
    k[0] = crate::effects::CDK_VIEWS.iter().position(|v| v == view).unwrap_or(0) as f64;
    k[1] = hsv_hue(crate::effects::encoded(key_color)).unwrap_or(0.0);
    k[2..7].copy_from_slice(partial_a);
    k[7..12].copy_from_slice(partial_b);
    k[12..15].copy_from_slice(matte);
    k[16] = 255.0;
    k
}

/// D-459: D-112's levels step on a matte value t, 0 to 1, as 255 t, back over 255.
fn cdk_level([ib, iw, gamma, ob, ow]: [f64; 5], t: f64) -> f64 {
    let x = 255.0 * t;
    let v = if iw == ib {
        if x >= ib { 1.0 } else { 0.0 }
    } else {
        ((x - ib) / (iw - ib)).clamp(0.0, 1.0)
    };
    (ob + v.powf(1.0 / gamma) * (ow - ob)) / 255.0
}

/// D-459: the encoded colour e's partial A uncorrected and corrected, partial B uncorrected and
/// corrected, and the matte uncorrected and corrected, in that order (the views' after "final"
/// and "source"). Hue windows of 60 degrees with a 60-degree shoulder about the key's hue (B)
/// and 60 degrees short of it (A), weighted by the HLS saturation toward a half, then a hard
/// light by the lightness; the partials combined by a screen. After Ihringer's reverse-
/// engineering of After Effects' (document 21).
pub(crate) fn cdk_mattes(e: [f64; 3], k: &[f64; 17]) -> [f64; 6] {
    let (h, l, s) = hls(e);
    let h = h.unwrap_or(0.0);
    let window = |centre: f64| {
        let d = (h - centre).rem_euclid(360.0);
        ((d.min(360.0 - d) - 60.0) / 60.0).clamp(0.0, 1.0)
    };
    let ta = s * window(k[1] - 60.0) + (1.0 - s) / 2.0;
    let tb = s * window(k[1]) + (1.0 - s) / 2.0;
    let (a, b) = if l < 0.5 {
        (2.0 * ta * l, 1.0 - 2.0 * (1.0 - tb) * l)
    } else {
        (1.0 - 2.0 * (1.0 - ta) * (1.0 - l), 2.0 * tb * (1.0 - l))
    };
    let set = |i: usize| [k[i], k[i + 1], k[i + 2], k[i + 3], k[i + 4]];
    let (ca, cb) = (cdk_level(set(2), a), cdk_level(set(7), b));
    let screen = |a: f64, b: f64| 1.0 - (1.0 - a) * (1.0 - b);
    [a, ca, b, cb, screen(a, b), cdk_level(set(12), screen(ca, cb))]
}

/// D-459: Color Difference Key. "final" multiplies every channel by the corrected matte, so the
/// colour is kept and the covering falls; "source" leaves the layer as it is; any other view
/// shows its matte as an encoded grey at the pixel's covering.
pub(crate) fn color_difference_key(source: &mut WorkingBuffer, k: &[f64; 17]) {
    let view = k[0] as usize;
    if view == 1 {
        return;
    }
    each_pixel(source, |px| {
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        let e = [0, 1, 2].map(|c| to_srgb((px[c] as f64 / a).clamp(0.0, 1.0)));
        let m = cdk_mattes(e, k);
        if view == 0 {
            for c in px.iter_mut() {
                *c = (*c as f64 * m[5]) as f32;
            }
            return;
        }
        let v = (to_linear(m[view - 2]) * a) as f32;
        px[..3].fill(v);
    });
}

/// D-500: Color Range's settings as the card reads them: the colour space's place in
/// [`crate::effects::CRANGE_SPACES`], the fuzziness, then the box's six ends.
pub(crate) fn crange_settings(effect: &crate::effects::Effect) -> [f64; 8] {
    let crate::effects::Effect::ColorRange { color_space, fuzziness, range } = effect else {
        unreachable!("crange_settings takes Color Range")
    };
    let mut k = [0.0; 8];
    k[0] = crate::effects::CRANGE_SPACES.iter().position(|v| v == color_space).unwrap_or(0) as f64;
    k[1] = *fuzziness;
    k[2..].copy_from_slice(range);
    k
}

/// D-500: the share of the encoded colour e that Color Range keeps. Its three components on a
/// 0 to 255 scale (Lab from the linear colour by Shadow/Highlight's conversion, a* and b* about
/// 128; full-range BT.601 YUV; RGB), how far each lies outside its end of the box, the largest,
/// over the fuzziness; with fuzziness 0, 0 inside the box and 1 outside. Our rule (document 21).
pub(crate) fn crange_kept(e: [f64; 3], k: &[f64; 8]) -> f64 {
    let [r, g, b] = e;
    let v = match k[0] as usize {
        0 => {
            let [l, p, q] = shadow_highlight_lab(e.map(to_linear));
            [255.0 * l, 128.0 + 128.0 * p, 128.0 + 128.0 * q]
        }
        1 => [
            255.0 * (0.299 * r + 0.587 * g + 0.114 * b),
            128.0 + 255.0 * (-0.168736 * r - 0.331264 * g + 0.5 * b),
            128.0 + 255.0 * (0.5 * r - 0.418688 * g - 0.081312 * b),
        ],
        _ => e.map(|c| 255.0 * c),
    };
    let d = (0..3).map(|i| (k[2 + 2 * i] - v[i]).max(v[i] - k[3 + 2 * i]).max(0.0)).fold(0.0, f64::max);
    match k[1] == 0.0 {
        true => if d == 0.0 { 0.0 } else { 1.0 },
        false => (d / k[1]).min(1.0),
    }
}

/// D-500: Color Range. Every channel times the share kept, so the colour is kept and only the
/// covering falls.
pub(crate) fn color_range(source: &mut WorkingBuffer, k: &[f64; 8]) {
    each_pixel(source, |px| {
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        let e = [0, 1, 2].map(|c| to_srgb((px[c] as f64 / a).clamp(0.0, 1.0)));
        let m = crange_kept(e, k);
        for c in px.iter_mut() {
            *c = (*c as f64 * m) as f32;
        }
    });
}

/// D-504: Linear Color Key's settings as the card reads them: the view's place in
/// [`crate::effects::LCKEY_VIEWS`], the match's in [`crate::effects::LCKEY_MATCHES`], the
/// tolerance and softness, the key colour encoded, and its hue (-1 for a grey).
pub(crate) fn lckey_settings(effect: &crate::effects::Effect) -> [f64; 8] {
    let crate::effects::Effect::LinearColorKey { view, key_color, match_colors, matching_tolerance, matching_softness, .. } =
        effect
    else {
        unreachable!("lckey_settings takes Linear Color Key")
    };
    let key = crate::effects::encoded(key_color);
    let place = |list: &[&str], w: &str| list.iter().position(|v| *v == w).unwrap_or(0) as f64;
    [
        place(&crate::effects::LCKEY_VIEWS, view),
        place(&crate::effects::LCKEY_MATCHES, match_colors),
        *matching_tolerance,
        *matching_softness,
        key[0],
        key[1],
        key[2],
        hsv_hue(key).unwrap_or(-1.0),
    ]
}

/// D-504: the share of the encoded colour e that Linear Color Key keeps. Its distance from the
/// key, 0 to 100: RGB, the largest channel difference; hue, the angle between their hues over
/// 180, 100 when either is a grey; chroma, the largest difference of their chromaticities
/// e / (eR + eG + eB), a black's a third each. Then (d - tolerance) / softness held inside 0 to
/// 1; with softness 0, 0 at or within the tolerance and 1 beyond. D-541: tolerance 0 keeps all,
/// "the entire image opaque" as Adobe says. Our rule (document 21).
pub(crate) fn lckey_kept(e: [f64; 3], k: &[f64; 8]) -> f64 {
    if k[2] == 0.0 {
        return 1.0;
    }
    let key = [k[4], k[5], k[6]];
    let chroma = |c: [f64; 3]| {
        let s = c[0] + c[1] + c[2];
        if s == 0.0 { [1.0 / 3.0; 3] } else { c.map(|v| v / s) }
    };
    let most = |a: [f64; 3], b: [f64; 3]| (0..3).map(|i| (a[i] - b[i]).abs()).fold(0.0, f64::max);
    let d = match k[1] as usize {
        0 => 100.0 * most(e, key),
        1 => match (hsv_hue(e), k[7] >= 0.0) {
            (Some(h), true) => {
                let a = (h - k[7]).abs() % 360.0;
                100.0 * a.min(360.0 - a) / 180.0
            }
            _ => 100.0,
        },
        _ => 100.0 * most(chroma(e), chroma(key)),
    };
    match k[3] == 0.0 {
        true => if d <= k[2] { 0.0 } else { 1.0 },
        false => ((d - k[2]) / k[3]).clamp(0.0, 1.0),
    }
}

/// D-504: Linear Color Key. "final" multiplies every channel by the share kept, so the colour is
/// kept and the covering falls; "source" leaves the layer as it is; "matte" shows the share as
/// an encoded grey at the pixel's covering.
pub(crate) fn linear_color_key(source: &mut WorkingBuffer, k: &[f64; 8]) {
    let view = k[0] as usize;
    if view == 1 {
        return;
    }
    each_pixel(source, |px| {
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        let e = [0, 1, 2].map(|c| to_srgb((px[c] as f64 / a).clamp(0.0, 1.0)));
        let m = lckey_kept(e, k);
        if view == 0 {
            for c in px.iter_mut() {
                *c = (*c as f64 * m) as f32;
            }
            return;
        }
        px[..3].fill((to_linear(m) * a) as f32);
    });
}

/// D-541: Linear Color Key's Keep Colors on `source`, `before` the input of the run of keys
/// above it (the same size: a key moves and grows nothing). Where `before` has covering a0, the
/// keep share kk is 1 - [`lckey_kept`] of its straight colour. "final": where a0 kk is above the
/// pixel's covering, the pixel becomes `before`'s times kk; "matte": the encoded grey
/// max(covering, a0 kk) / a0 at covering a0; "source" leaves the layer as it is.
pub(crate) fn linear_color_keep(source: &mut WorkingBuffer, before: &WorkingBuffer, k: &[f64; 8]) {
    let view = k[0] as usize;
    if view == 1 {
        return;
    }
    debug_assert_eq!((source.width(), source.height()), (before.width(), before.height()));
    let w = source.width().max(1) * 4;
    source.data_mut().par_chunks_mut(w).zip(before.data().par_chunks(w)).for_each(|(row, was)| {
        for (q, p) in row.chunks_exact_mut(4).zip(was.chunks_exact(4)) {
            let a0 = p[3] as f64;
            if a0 <= 0.0 {
                continue;
            }
            let e = [0, 1, 2].map(|c| to_srgb((p[c] as f64 / a0).clamp(0.0, 1.0)));
            let kk = 1.0 - lckey_kept(e, k);
            if view == 0 {
                if a0 * kk > q[3] as f64 {
                    // A channel at a time (B-222).
                    for c in 0..4 {
                        q[c] = (p[c] as f64 * kk) as f32;
                    }
                }
                continue;
            }
            let s = (q[3] as f64).max(a0 * kk) / a0;
            q[..3].fill((to_linear(s) * a0) as f32);
            q[3] = p[3];
        }
    });
}

/// D-540: Screen Key's settings as the card reads them: the view's place in
/// [`crate::effects::SKEY_VIEWS`], the screen channel and the other two (0 red, 1 green, 2 blue),
/// the gain, balance, clip black and clip white, despill bias and despill as shares, and the
/// screen colour encoded.
pub(crate) fn skey_settings(effect: &crate::effects::Effect) -> [f64; 13] {
    let crate::effects::Effect::ScreenKey {
        view,
        screen_color,
        screen_gain,
        screen_balance,
        clip_black,
        clip_white,
        despill_bias,
        despill,
        ..
    } = effect
    else {
        unreachable!("skey_settings takes Screen Key")
    };
    let k = crate::effects::encoded(screen_color);
    // The largest channel; on a tie green, then blue, then red.
    let s = [1usize, 2, 0].into_iter().fold(1, |best, i| if k[i] > k[best] { i } else { best });
    let others: Vec<usize> = (0..3).filter(|&i| i != s).collect();
    [
        crate::effects::SKEY_VIEWS.iter().position(|v| v == view).unwrap_or(0) as f64,
        s as f64,
        others[0] as f64,
        others[1] as f64,
        screen_gain / 100.0,
        screen_balance / 100.0,
        clip_black / 100.0,
        clip_white / 100.0,
        despill_bias / 100.0,
        despill / 100.0,
        k[0],
        k[1],
        k[2],
    ]
}

/// D-540: the larger of c's two other channels at 1 - t, the smaller at t.
fn skey_weigh(c: [f64; 3], k: &[f64; 13], t: f64) -> f64 {
    let (a, b) = (c[k[2] as usize], c[k[3] as usize]);
    a.max(b) * (1.0 - t) + a.min(b) * t
}

/// D-540: Screen Key's matte at the encoded colour e, 1 foreground and 0 screen, clip levels
/// taken (document 21's steps 1 and 2).
pub(crate) fn skey_matte(e: [f64; 3], k: &[f64; 13]) -> f64 {
    let s = k[1] as usize;
    let screen = [k[10], k[11], k[12]];
    let ds = screen[s] - skey_weigh(screen, k, k[5]);
    let m = if ds <= 0.0 { 1.0 } else { (1.0 - k[4] * (e[s] - skey_weigh(e, k, k[5])) / ds).clamp(0.0, 1.0) };
    skey_clip(m, k)
}

/// D-540: the clip levels: (m - black) / (white - black) held inside 0 to 1; a step at black
/// where white <= black.
fn skey_clip(m: f64, k: &[f64; 13]) -> f64 {
    match k[7] > k[6] {
        true => ((m - k[6]) / (k[7] - k[6])).clamp(0.0, 1.0),
        false => if m <= k[6] { 0.0 } else { 1.0 },
    }
}

/// D-540: the encoded colour e with its screen channel held no higher than the blend of the
/// other two at the despill bias, by the despill share.
pub(crate) fn skey_despill(e: [f64; 3], k: &[f64; 13]) -> [f64; 3] {
    let s = k[1] as usize;
    let mut out = e;
    out[s] -= k[9] * (e[s] - skey_weigh(e, k, k[8])).max(0.0);
    out
}

/// D-540: Screen Key, document 21's rule: the matte (empty pixels count as foreground), shrunk
/// or grown over Simple Choker's disc and softened by document 21's Gaussian, both with the
/// picture's edge held; then the view; then, for "final", Key Cleaner's fit at the edge refine.
/// The settings are valid.
pub(crate) fn screen_key(source: &mut WorkingBuffer, effect: &crate::effects::Effect) {
    let crate::effects::Effect::ScreenKey { shrink_grow, softness, edge_refine, .. } = *effect else {
        unreachable!("screen_key takes Screen Key")
    };
    let k = skey_settings(effect);
    let (w, h) = (source.width(), source.height());
    let straight = |px: &[f32]| [0, 1, 2].map(|c| to_srgb((px[c] as f64 / px[3] as f64).clamp(0.0, 1.0)));
    let mut m: Vec<f32> = source
        .data()
        .par_chunks_exact(4)
        .map(|px| if px[3] <= 0.0 { skey_clip(1.0, &k) } else { skey_matte(straight(px), &k) } as f32)
        .collect();
    if shrink_grow != 0.0 && w > 0 && h > 0 {
        // The matte as a layer's covering, padded with its edge pixels held, through Simple
        // Choker (whose positive choke shrinks).
        let n = shrink_grow.abs().ceil() as usize;
        let (pw, ph) = (w + 2 * n, h + 2 * n);
        let mut pad = WorkingBuffer::transparent(pw, ph);
        pad.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
            let x = (i % pw).saturating_sub(n).min(w - 1);
            let y = (i / pw).saturating_sub(n).min(h - 1);
            px.fill(m[y * w + x]);
        });
        let g = crate::layer_fx::simple_choker(&mut pad, -shrink_grow);
        let (bw, out) = (pw + 2 * g, pad.data());
        m.par_iter_mut().enumerate().for_each(|(i, v)| {
            *v = out[((i / w + n + g) * bw + i % w + n + g) * 4 + 3];
        });
    }
    if softness > 0.0 {
        let mut b = WorkingBuffer::transparent(w, h);
        b.data_mut().par_chunks_exact_mut(4).zip(&m).for_each(|(px, &v)| px.fill(v));
        crate::effects::held_blur(&mut b, softness);
        m.par_iter_mut().zip(b.data().par_chunks_exact(4)).for_each(|(v, px)| *v = px[3]);
    }
    let view = k[0] as usize;
    source.data_mut().par_chunks_exact_mut(4).zip(&m).for_each(|(px, &v)| {
        let a = px[3] as f64;
        if a <= 0.0 {
            px.fill(0.0);
            return;
        }
        let v = v as f64;
        match view {
            0 => {
                let c = skey_despill(straight(px), &k);
                for j in 0..3 {
                    px[j] = (to_linear(c[j]) * a * v) as f32;
                }
                px[3] = (a * v) as f32;
            }
            _ => {
                let v = match view {
                    1 => v,
                    _ if v >= 0.999 => 1.0,
                    _ if v <= 0.001 => 0.0,
                    _ => 0.5,
                };
                px[..3].fill((to_linear(v) * a) as f32);
            }
        }
    });
    if view == 0 && edge_refine >= 1.0 {
        crate::matte_refine::key_cleaner(source, edge_refine, 0.0, 1.0);
    }
}

/// D-505: Unmult's settings as the card reads them: 1 for a white background, the level L as a
/// share (black: black level; white: 1 - white level), the softness as a share, 1 to remove
/// colour matting, 1 to clip HDR results.
pub(crate) fn unmult_settings(effect: &crate::effects::Effect) -> [f64; 5] {
    let crate::effects::Effect::Unmult {
        background_color,
        black_level,
        white_level,
        softness,
        remove_color_matting,
        clip_hdr_results,
    } = effect
    else {
        unreachable!("unmult_settings takes Unmult")
    };
    let white = background_color == "white";
    [
        white as u8 as f64,
        if white { 1.0 - white_level / 100.0 } else { black_level / 100.0 },
        softness / 100.0,
        (remove_color_matting == "on") as u8 as f64,
        (clip_hdr_results == "on") as u8 as f64,
    ]
}

/// D-505: Unmult, on the linear premultiplied pixel (c, a); a = 0 is left as it is. m, how far
/// the pixel is from the background, held inside 0 to a: black, its brightest channel; white,
/// a less its darkest. The share (m - L) / softness held inside 0 to 1 (softness 0: 1 above L,
/// else 0); the new covering m times the share. Remove Color Matting on takes the colour from
/// before the background, c / m or (c - a + m) / m, held inside 0 to 1 if clipping; off keeps
/// c / a. Our rule (document 21); as added, Soft Physical Glow's unmult.
pub(crate) fn unmult(source: &mut WorkingBuffer, k: &[f64; 5]) {
    each_pixel(source, |px| {
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        let c = [0, 1, 2].map(|i| px[i] as f64);
        let white = k[0] == 1.0;
        let m = if white { a - c.iter().cloned().fold(f64::INFINITY, f64::min) } else { c.iter().cloned().fold(f64::NEG_INFINITY, f64::max) };
        let m = m.clamp(0.0, a);
        let share = match k[2] == 0.0 {
            true => if m > k[1] { 1.0 } else { 0.0 },
            false => ((m - k[1]) / k[2]).clamp(0.0, 1.0),
        };
        let cover = m * share;
        if cover == 0.0 {
            px.fill(0.0);
            return;
        }
        for i in 0..3 {
            let u = match (k[3] == 1.0, white) {
                (false, _) => c[i] / a,
                (true, false) => c[i] / m,
                (true, true) => (c[i] - a + m) / m,
            };
            let u = if k[3] == 1.0 && k[4] == 1.0 { u.clamp(0.0, 1.0) } else { u };
            px[i] = (u * cover) as f32;
        }
        px[3] = cover as f32;
    });
}

/// D-396: Selective Color with every amount 0 leaves the layer exactly as it is.
pub(crate) fn selective_color_untouched(families: &[[f64; 4]; 9]) -> bool {
    families.iter().flatten().all(|v| *v == 0.0)
}

/// D-396: Selective Color, by Bœsch's reverse-engineering of Photoshop's (FFmpeg's
/// selectivecolor), in fractions. On the encoded colour e with largest, middle and smallest mx,
/// md, mn, each family's weight: reds, greens, blues mx - md where red, green, blue is the
/// largest; cyans, magentas, yellows md - mn where red, green, blue is the smallest; whites
/// 2 mn - 1; neutrals 1 - (|mx - 0.5| + |mn - 0.5|); blacks 1 - 2 mx. For each family weighing
/// above 0, with its amounts over 100, each channel moves by w times (-1 - s) k - s (s its own
/// of cyan, magenta, yellow; k the black), times 1 - e when `relative`, held inside -e to 1 - e.
/// `families` in [`crate::effects::SELECTIVE_COLOR_FAMILIES`]' order, per cent.
pub(crate) fn selective_color(source: &mut WorkingBuffer, families: &[[f64; 4]; 9], relative: bool) {
    if selective_color_untouched(families) {
        return;
    }
    grade_pixels(source, false, |_, e| {
        let mx = e[0].max(e[1]).max(e[2]);
        let mn = e[0].min(e[1]).min(e[2]);
        let md = e[0].min(e[1]).max(e[0].max(e[1]).min(e[2]));
        let (top, low) = (mx - md, md - mn);
        let pick = |on: bool, w: f64| if on { w } else { 0.0 };
        let weights = [
            pick(e[0] == mx, top),
            pick(e[2] == mn, low),
            pick(e[1] == mx, top),
            pick(e[0] == mn, low),
            pick(e[2] == mx, top),
            pick(e[1] == mn, low),
            2.0 * mn - 1.0,
            1.0 - ((mx - 0.5).abs() + (mn - 0.5).abs()),
            1.0 - 2.0 * mx,
        ];
        let mut o = e;
        for (f, w) in families.iter().zip(weights) {
            if w <= 0.0 {
                continue;
            }
            let k = f[3] / 100.0;
            for c in 0..3 {
                let s = f[c] / 100.0;
                let t = (-1.0 - s) * k - s;
                let t = if relative { t * (1.0 - e[c]) } else { t };
                o[c] += t.clamp(-e[c], 1.0 - e[c]) * w;
            }
        }
        o
    });
}

/// D-400: Shadow/Highlight with both amounts 0, and (D-563) no midtone contrast and no clip,
/// leaves the layer exactly as it is.
pub(crate) fn shadow_highlight_untouched(shadow: f64, highlight: f64, midtone: f64, black_clip: f64, white_clip: f64) -> bool {
    [shadow, highlight, midtone, black_clip, white_clip] == [0.0; 5]
}

/// D-563: l, p, q with the lightness `l2`, the chroma following it toward black (times l2 / l)
/// or white (times (1 - l2) / (1 - l)).
pub(crate) fn shadow_highlight_follow([l, p, q]: [f64; 3], l2: f64) -> [f64; 3] {
    let k = if l2 < l {
        l2 / l
    } else if l2 > l {
        (1.0 - l2) / (1.0 - l)
    } else {
        1.0
    };
    [l2, p * k, q * k]
}

/// D-400: the sRGB matrix of IEC 61966-2-1, its inverse, and its own white, M (1, 1, 1).
const SH_M: [[f64; 3]; 3] = [[0.4124, 0.3576, 0.1805], [0.2126, 0.7152, 0.0722], [0.0193, 0.1192, 0.9505]];
const SH_MI: [[f64; 3]; 3] = [
    [3.2406254773200533, -1.5372079722103187, -0.4986285986982479],
    [-0.9689307147293194, 1.875756060885241, 0.04151752384295394],
    [0.05571012044551061, -0.2040210505984867, 1.0569959422543882],
];
const SH_WHITE: [f64; 3] = [0.9505, 1.0, 1.089];

/// D-400: straight linear colour to CIE L*a*b* on sRGB's own white, scaled as darktable scales
/// it: L* / 100, a* / 128, b* / 128.
pub(crate) fn shadow_highlight_lab(b: [f64; 3]) -> [f64; 3] {
    let d = 6.0 / 29.0;
    let f = |t: f64| if t > d * d * d { t.cbrt() } else { t / (3.0 * d * d) + 4.0 / 29.0 };
    let [x, y, z]: [f64; 3] = std::array::from_fn(|i| f((0..3).map(|j| SH_M[i][j] * b[j]).sum::<f64>() / SH_WHITE[i]));
    [(116.0 * y - 16.0) / 100.0, 500.0 * (x - y) / 128.0, 200.0 * (y - z) / 128.0]
}

/// D-400: [`shadow_highlight_lab`] undone.
pub(crate) fn shadow_highlight_unlab([l, p, q]: [f64; 3]) -> [f64; 3] {
    let d = 6.0 / 29.0;
    let finv = |u: f64| if u > d { u * u * u } else { 3.0 * d * d * (u - 4.0 / 29.0) };
    let fy = (100.0 * l + 16.0) / 116.0;
    let v: [f64; 3] = std::array::from_fn(|i| SH_WHITE[i] * finv([fy + 128.0 * p / 500.0, fy, fy - 128.0 * q / 200.0][i]));
    std::array::from_fn(|i| (0..3).map(|j| SH_MI[i][j] * v[j]).sum())
}

/// D-400: darktable's shadows and highlights rule (src/iop/shadhi.c, its Gaussian softening) on
/// one pixel's scaled l, p, q; `ts` and `th` its lightness blurred at the shadow and highlight
/// radius, inverted. `k`: the shadow and highlight amounts as 2 x / 100, the compressions
/// min(1 - width / 100, 0.99), the colour correction as a share. The highlights first, then the
/// shadows, each laid by the overlay once for each whole step of its amount squared.
pub(crate) fn shadow_highlight_pixel(lpq: [f64; 3], ts: f64, th: f64, k: [f64; 5]) -> [f64; 3] {
    let [s, h, cs, ch, cc] = k;
    let sign = |v: f64| if v < 0.0 { -1.0 } else { 1.0 };
    let recip = |v: f64| if v.abs() > 1e-6 { 1.0 / v } else { 1e6f64.copysign(v) };
    let [mut l, mut p, mut q] = lpq;
    for (amount, t, shadows) in [(h, th, false), (s, ts, true)] {
        let x = if shadows { t / (1.0 - cs) - cs / (1.0 - cs) } else { 1.0 - t / (1.0 - ch) }.clamp(0.0, 1.0);
        let (near, far) = if shadows { (cc, 1.0 - cc) } else { (1.0 - cc, cc) };
        let mut n = amount * amount;
        while n > 0.0 {
            let la = l;
            let lb = (t - 0.5) * sign(1.0 - la) + 0.5;
            let o = n.min(1.0) * x;
            n -= 1.0;
            let laid = if la > 0.5 { 1.0 - (1.0 - 2.0 * (la - 0.5)) * (1.0 - lb) } else { 2.0 * la * lb };
            l = la * (1.0 - o) + laid * o;
            let f = l * recip(la) * near + (1.0 - l) * recip(1.0 - la) * far;
            p = p * (1.0 - o) + p * f * o;
            q = q * (1.0 - o) + q * f * o;
        }
    }
    [l, p, q]
}

/// D-400: Shadow/Highlight. `v`: the shadow and highlight amounts, the shadow tonal width and
/// radius, the highlight tonal width and radius, the colour correction, and (D-563) the midtone
/// contrast and black and white clip, as the settings hold them. Each pixel's lightness blurred
/// by Gaussian Blur's kernel at each radius, the edges repeated, then
/// [`shadow_highlight_pixel`]; then the midtone contrast, Color Grade's curve on the lightness;
/// then, with a clip, the lightness along the line from the clip bins of the layer's own count
/// to its darkest and lightest bins in use (`frame_stats::clip_points`); the chroma following
/// each change; the colour held inside 0 to 1 at the pixel's own covering, which is never changed.
pub(crate) fn shadow_highlight(source: &mut WorkingBuffer, v: [f64; 10]) {
    let [shadow, highlight, shadow_width, shadow_radius, highlight_width, highlight_radius, cc, midtone, black_clip, white_clip] = v;
    if shadow_highlight_untouched(shadow, highlight, midtone, black_clip, white_clip) {
        return;
    }
    let w = source.width().max(1);
    let straight = |px: &[f32], a: f64| -> [f64; 3] { std::array::from_fn(|c| (px[c] as f64 / a).clamp(0.0, 1.0)) };
    let mut plane = source.clone();
    plane.data_mut().par_chunks_mut(w * 4).for_each(|row| {
        for px in row.chunks_exact_mut(4) {
            let a = px[3] as f64;
            let l = if a > 0.0 { shadow_highlight_lab(straight(px, a))[0] as f32 } else { 0.0 };
            px.copy_from_slice(&[l, l, l, 1.0]);
        }
    });
    let blurred = |mut b: WorkingBuffer, sigma: f64| {
        crate::effects::held_blur_axes(&mut b, &crate::effects::gaussian_weights(sigma), (true, true));
        b
    };
    // With both amounts 0 no step reads the blurs, so the plane stands in for them unblurred.
    let lifting = shadow != 0.0 || highlight != 0.0;
    let ls = if lifting { blurred(plane.clone(), shadow_radius) } else { plane.clone() };
    let lh = if !lifting || highlight_radius == shadow_radius { ls.clone() } else { blurred(plane, highlight_radius) };
    let (ls, lh) = (ls.data(), lh.data());
    let k = [2.0 * shadow / 100.0, 2.0 * highlight / 100.0, (1.0 - shadow_width / 100.0).min(0.99), (1.0 - highlight_width / 100.0).min(0.99), cc / 100.0];
    let m = midtone / 100.0;
    let lifted = |px: &[f32], i: usize| -> [f64; 3] {
        let lpq = shadow_highlight_pixel(shadow_highlight_lab(straight(px, px[3] as f64)), 1.0 - ls[i] as f64, 1.0 - lh[i] as f64, k);
        let l = lpq[0];
        shadow_highlight_follow(lpq, l + m * l * (1.0 - l) * (2.0 * l - 1.0))
    };
    // D-563: the clip's line, from the count of the lifted lightness, rows in blocks added in
    // turn so it is the same bits every run.
    let mut line = None;
    if black_clip > 0.0 || white_clip > 0.0 {
        let data = source.data();
        let blocks: Vec<[f64; 256]> = data
            .par_chunks(w * 4 * 64)
            .enumerate()
            .map(|(b, block)| {
                let mut h = [0.0; 256];
                for (j, px) in block.chunks_exact(4).enumerate() {
                    if px[3] > 0.0 {
                        h[crate::frame_stats::bin(lifted(px, b * w * 4 * 64 + j * 4)[0])] += px[3] as f64;
                    }
                }
                h
            })
            .collect();
        let mut h = [0.0; 256];
        for b in &blocks {
            for (x, y) in h.iter_mut().zip(b) {
                *x += y;
            }
        }
        let (kb, kw) = crate::frame_stats::clip_points(&h, black_clip, white_clip);
        let (lo, hi) = crate::frame_stats::clip_points(&h, 0.0, 0.0);
        if kw > kb {
            line = Some((kb as f64 / 255.0, lo as f64 / 255.0, (hi - lo) as f64 / (kw - kb) as f64));
        }
    }
    source.data_mut().par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        for (x, px) in row.chunks_exact_mut(4).enumerate() {
            let a = px[3] as f64;
            if a <= 0.0 {
                continue;
            }
            let mut lpq = lifted(px, (y * w + x) * 4);
            if let Some((kb, lo, slope)) = line {
                lpq = shadow_highlight_follow(lpq, (lo + (lpq[0] - kb) * slope).clamp(0.0, 1.0));
            }
            let o = shadow_highlight_unlab(lpq);
            for c in 0..3 {
                px[c] = (o[c].clamp(0.0, 1.0) * a) as f32;
            }
        }
    });
}

/// D-397: Color Grade's tone step. Each channel taken to linear light, multiplied by its gain
/// and held inside 0 to 1 when a gain is not 1; then, with `[c, h, s, w, b, f]` the contrast,
/// highlights, shadows, whites, blacks and faded film as shares, on each encoded channel one
/// after another: `v += c v (1 - v) (2v - 1)`, `h v² (1 - v)`, `s v (1 - v)²`, `w v² / 4`,
/// `b (1 - v)² / 4` and `f (0.25 (1 - v)² - 0.1 v²)`; held inside 0 to 1.
pub(crate) fn grade_tone(source: &mut WorkingBuffer, gains: [f64; 3], [c, h, s, w, b, f]: [f64; 6]) {
    let linear = gains != [1.0; 3];
    grade_pixels(source, false, |_, e| {
        std::array::from_fn(|i| {
            let mut v = e[i];
            if linear {
                v = to_srgb((to_linear(v) * gains[i]).clamp(0.0, 1.0));
            }
            v += c * v * (1.0 - v) * (2.0 * v - 1.0);
            v += h * v * v * (1.0 - v);
            v += s * v * (1.0 - v) * (1.0 - v);
            v += w * v * v / 4.0;
            v += b * (1.0 - v) * (1.0 - v) / 4.0;
            v + f * (0.25 * (1.0 - v) * (1.0 - v) - 0.1 * v * v)
        })
    });
}

/// D-397: Color Grade's look: `e + i (lookup(e) - e)`, the look at intensity `i` (0 to 2).
pub(crate) fn look(source: &mut WorkingBuffer, cube: &crate::lut::Cube, i: f64) {
    grade_pixels(source, false, |_, e| {
        let l = cube.lookup(e);
        std::array::from_fn(|c| e[c] + i * (l[c] - e[c]))
    })
}

/// D-397: the pure colour of `hue` degrees (HSL saturation 1, lightness 1/2), its luma taken out.
pub(crate) fn tint_push(hue: f64) -> [f64; 3] {
    let c = from_hsl([hue, 1.0, 0.5]);
    let l = luma(c);
    c.map(|v| v - l)
}

/// D-398: Color Grade's hue-versus-saturation curve at `hue` degrees. One point is its
/// saturation at every hue; with more, from each point to the next round the circle (the last
/// to the first through 360), eased by u² (3 - 2u). `points` are valid and not empty.
pub(crate) fn hue_curve(points: &[Vec<f64>], hue: f64) -> f64 {
    let n = points.len();
    for i in 0..n {
        let (h0, s0, s1) = (points[i][0], points[i][1], points[(i + 1) % n][1]);
        let h1 = if i + 1 == n { points[0][0] + 360.0 } else { points[i + 1][0] };
        let x = if hue >= h0 { hue } else { hue + 360.0 };
        if n > 1 && h0 <= x && x < h1 {
            let u = (x - h0) / (h1 - h0);
            return s0 + (s1 - s0) * u * u * (3.0 - 2.0 * u);
        }
    }
    points[0][1]
}

/// D-398: Color Grade's hue versus saturation, Vibrance's rule (D-140) with vibrance 0 and the
/// curve's saturation at the colour's hue (0 for a grey).
pub(crate) fn hue_vs_saturation(source: &mut WorkingBuffer, points: &[Vec<f64>]) {
    grade_pixels(source, false, |_, e| {
        let l = luma(e);
        let k = hue_curve(points, hls(e).0.unwrap_or(0.0)) / 100.0;
        e.map(|v| l + (v - l) * k)
    });
}

/// D-398: Color Grade's HSL Secondary, read once for a frame. `key`: the hue, its range and
/// softness in degrees, then the saturation's low and high, the lightness's low and high and
/// their softness, per cent. `gains`, the white balance's when either is set; `contrast` and
/// `saturation` as shares; `push`, the wheel and lightness over 200.
pub(crate) struct Secondary {
    pub key: [f64; 8],
    pub invert: bool,
    pub mask: bool,
    pub gains: Option<[f64; 3]>,
    pub contrast: f64,
    pub saturation: f64,
    pub push: [f64; 3],
}

/// D-398: 1 from `lo` to `hi`, falling to 0 over `soft` outside them, eased.
fn key_band(x: f64, lo: f64, hi: f64, soft: f64) -> f64 {
    let d = if x < lo { lo - x } else if x > hi { x - hi } else { 0.0 };
    if d <= 0.0 {
        return 1.0;
    }
    if soft == 0.0 {
        return 0.0;
    }
    let t = (d / soft).clamp(0.0, 1.0);
    1.0 - t * t * (3.0 - 2.0 * t)
}

/// D-398: how much of HSL Secondary's correction `e` takes, 0 to 1. Below a range of 180 a
/// grey, which has no hue, is not taken.
pub(crate) fn secondary_key(e: [f64; 3], s: &Secondary) -> f64 {
    let k = s.key;
    let (hue, l, sat) = hls(e);
    let wh = match (k[1] >= 180.0, hue) {
        (true, _) => 1.0,
        (false, None) => 0.0,
        (false, Some(h)) => key_band(((h - k[0] + 180.0).rem_euclid(360.0) - 180.0).abs(), 0.0, k[1], k[2]),
    };
    // A pure colour's saturation can come out a rounding above 1, which a band ending at 100
    // with no softness would drop; held inside 0 to 1 first.
    let m = wh * key_band(sat.min(1.0) * 100.0, k[3], k[4], k[7]) * key_band(l * 100.0, k[5], k[6], k[7]);
    if s.invert { 1.0 - m } else { m }
}

/// D-398: HSL Secondary: the key as a grey in the mask view; otherwise `e + m (c - e)`, c the
/// correction (the gains in linear light held inside 0 to 1, the contrast term, the
/// saturation about the luma, the push).
pub(crate) fn secondary(source: &mut WorkingBuffer, s: &Secondary) {
    grade_pixels(source, false, |_, e| {
        let m = secondary_key(e, s);
        if s.mask {
            return [m; 3];
        }
        let mut c = e;
        if let Some(g) = s.gains {
            c = std::array::from_fn(|i| to_srgb((to_linear(c[i]) * g[i]).clamp(0.0, 1.0)));
        }
        let k = s.contrast;
        c = c.map(|v| v + k * v * (1.0 - v) * (2.0 * v - 1.0));
        let l = luma(c);
        c = c.map(|v| l + (v - l) * s.saturation);
        std::array::from_fn(|i| e[i] + m * (c[i] + s.push[i] - e[i]))
    });
}

/// D-376: the mean encoded straight colour, each pixel weighted by its covering, of the pixels
/// of `source` whose centres lie within `r` of (`px`, `py`); with none, the pixel holding the
/// point, held inside the picture. `None` when what is counted has no covering.
pub(crate) fn stabilizer_sample(source: &WorkingBuffer, (px, py): (f64, f64), r: f64) -> Option<[f64; 3]> {
    let (w, h) = (source.width(), source.height());
    if w == 0 || h == 0 {
        return None;
    }
    let d = source.data();
    let (mut total, mut sums) = (0.0, [0.0; 3]);
    let mut add = |x: usize, y: usize| {
        let p = &d[(y * w + x) * 4..(y * w + x) * 4 + 4];
        let a = p[3] as f64;
        if a > 0.0 {
            total += a;
            for c in 0..3 {
                sums[c] += a * to_srgb((p[c] as f64 / a).clamp(0.0, 1.0));
            }
        }
    };
    let mut found = false;
    let (x0, x1) = ((px - r - 1.0).floor().max(0.0) as usize, ((px + r).ceil().max(0.0) as usize).min(w - 1));
    let (y0, y1) = ((py - r - 1.0).floor().max(0.0) as usize, ((py + r).ceil().max(0.0) as usize).min(h - 1));
    for y in y0..=y1 {
        for x in x0..=x1 {
            if (x as f64 + 0.5 - px).hypot(y as f64 + 0.5 - py) <= r {
                found = true;
                add(x, y);
            }
        }
    }
    if !found {
        add((px.floor().max(0.0) as usize).min(w - 1), (py.floor().max(0.0) as usize).min(h - 1));
    }
    (total > 0.0).then(|| sums.map(|s| s / total))
}

/// D-376: Color Stabilizer's correction, from this frame's samples `now` to the reference
/// frame's `then` (black, mid, white, as many as the mode reads): "brightness" moves every
/// channel by the black samples' difference in brightness; "levels" and "curves" map each
/// channel through the lines joining the pairs in order of the current value, the outer lines
/// carried on, a pair within a millionth of the one before left out.
pub(crate) fn color_stabilize(source: &mut WorkingBuffer, mode: &str, now: &[[f64; 3]], then: &[[f64; 3]]) {
    if mode == "brightness" {
        let y = |e: [f64; 3]| 0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2];
        let lift = y(then[0]) - y(now[0]);
        grade_pixels(source, false, |_, e| e.map(|v| v + lift));
        return;
    }
    let lines: [Vec<(f64, f64)>; 3] = std::array::from_fn(|c| {
        let mut pairs: Vec<(f64, f64)> = now.iter().zip(then).map(|(n, t)| (n[c], t[c])).collect();
        pairs.sort_by(|p, q| p.0.total_cmp(&q.0));
        let mut kept: Vec<(f64, f64)> = Vec::new();
        for p in pairs {
            if kept.last().is_none_or(|k| p.0 - k.0 > 1e-6) {
                kept.push(p);
            }
        }
        kept
    });
    let map = |l: &[(f64, f64)], v: f64| {
        if l.len() == 1 {
            return v + l[0].1 - l[0].0;
        }
        let i = l.partition_point(|p| p.0 <= v).clamp(1, l.len() - 1);
        let ((a, b), (c, d)) = (l[i - 1], l[i]);
        b + (v - a) * (d - b) / (c - a)
    };
    grade_pixels(source, false, |_, e| std::array::from_fn(|c| map(&lines[c], e[c])));
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
