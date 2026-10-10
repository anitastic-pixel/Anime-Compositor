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

/// D-400: Shadow/Highlight with both amounts 0 leaves the layer exactly as it is.
pub(crate) fn shadow_highlight_untouched(shadow: f64, highlight: f64) -> bool {
    shadow == 0.0 && highlight == 0.0
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
/// radius, the highlight tonal width and radius, and the colour correction, as the settings
/// hold them. Each pixel's lightness blurred by Gaussian Blur's kernel at each radius, the edges
/// repeated, then [`shadow_highlight_pixel`]; the colour held inside 0 to 1 at the pixel's own
/// covering, which is never changed.
pub(crate) fn shadow_highlight(source: &mut WorkingBuffer, v: [f64; 7]) {
    let [shadow, highlight, shadow_width, shadow_radius, highlight_width, highlight_radius, cc] = v;
    if shadow_highlight_untouched(shadow, highlight) {
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
    let ls = blurred(plane.clone(), shadow_radius);
    let lh = if highlight_radius == shadow_radius { ls.clone() } else { blurred(plane, highlight_radius) };
    let (ls, lh) = (ls.data(), lh.data());
    let k = [2.0 * shadow / 100.0, 2.0 * highlight / 100.0, (1.0 - shadow_width / 100.0).min(0.99), (1.0 - highlight_width / 100.0).min(0.99), cc / 100.0];
    source.data_mut().par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        for (x, px) in row.chunks_exact_mut(4).enumerate() {
            let a = px[3] as f64;
            if a <= 0.0 {
                continue;
            }
            let i = (y * w + x) * 4;
            let lpq = shadow_highlight_lab(straight(px, a));
            let o = shadow_highlight_unlab(shadow_highlight_pixel(lpq, 1.0 - ls[i] as f64, 1.0 - lh[i] as f64, k));
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
