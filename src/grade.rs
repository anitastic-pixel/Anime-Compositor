//! The batch of ten's colour effects, each a pixel at a time on a layer's own pixels: D-111's
//! curves, D-112's levels and D-113's hue and saturation.
//!
//! This program's own methods; nothing is ported. Each effect's `tools/<name>_reference.py` is
//! the same rule worked a second way, and its `tests/b5x_<name>.rs` holds this to its numbers.

use crate::WorkingBuffer;
use rayon::prelude::*;

/// Document 21's sRGB curves, in double precision, as the reference tools write them.
fn to_linear(c: f64) -> f64 {
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
fn grade_pixels(source: &mut WorkingBuffer, f: impl Fn(usize, [f64; 3]) -> [f64; 3] + Sync) {
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let a = px[3] as f64;
            if a <= 0.0 {
                return;
            }
            let e = std::array::from_fn(|c| to_srgb((px[c] as f64 / a).clamp(0.0, 1.0)));
            let out = f(i, e);
            for c in 0..3 {
                px[c] = (to_linear(out[c].clamp(0.0, 1.0)) * a) as f32;
            }
        });
}

/// The same, a channel at a time on the 0 to 255 scale: `f(channel, value)`.
fn grade(source: &mut WorkingBuffer, f: impl Fn(usize, f64) -> f64 + Sync) {
    grade_pixels(source, |_, e| std::array::from_fn(|c| f(c, e[c] * 255.0) / 255.0));
}

/// D-111's default curve, which changes nothing.
pub(crate) fn is_straight(points: &[Vec<f64>]) -> bool {
    points == [vec![0.0, 0.0], vec![255.0, 255.0]]
}

/// D-111: the natural cubic spline through `points`, which are valid, flat outside its end
/// points.
fn spline(points: &[Vec<f64>]) -> impl Fn(f64) -> f64 {
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
    grade_pixels(source, |_, e| {
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
