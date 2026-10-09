//! D-351 (EFFECTS.md P0-15): picture statistics, a count of a layer's whole picture at an
//! effect's place in its stack, and the four effects that read them: Stretch Levels, Stretch
//! Contrast, Stretch Color and Spread Tones, after After Effects' Auto Levels, Auto Contrast,
//! Auto Color and Equalize. Document 21 has the rules in words; `tools/auto_tone_reference.py`
//! works them a second way. Nothing is ported: Adobe does not publish its methods.
//!
//! The count is of the whole buffer the stack runs on, so the tiles a frame is drawn in and
//! B-158's part of a frame never change it. It is a sum in a fixed order (blocks of rows, each
//! counted alone, added in turn), so it is the same bits every run.

use crate::WorkingBuffer;
use rayon::prelude::*;

const LUMA: [f64; 3] = [0.2126, 0.7152, 0.0722];

/// A value 0 to 1 to its bin, 0 to 255.
pub fn bin(v: f64) -> usize {
    (v * 255.0 + 0.5).floor().clamp(0.0, 255.0) as usize
}

/// Red, green, blue and brightness histograms, each pixel adding its covering to its bin, and
/// for each brightness bin the covering times the colour of the pixels in it.
#[derive(Clone, Debug, PartialEq)]
pub struct Stats {
    pub hist: [[f64; 256]; 4],
    pub sums: [[f64; 256]; 3],
}

impl Stats {
    fn empty() -> Stats {
        Stats { hist: [[0.0; 256]; 4], sums: [[0.0; 256]; 3] }
    }

    fn add(&mut self, other: &Stats, by: f64) {
        for (a, b) in self.hist.iter_mut().zip(&other.hist).chain(self.sums.iter_mut().zip(&other.sums)) {
            for (x, y) in a.iter_mut().zip(b) {
                *x += y * by;
            }
        }
    }

    /// The covering of everything that shows.
    pub fn total(&self) -> f64 {
        self.hist[3].iter().sum()
    }

    /// Every pixel of `source` whose covering is above 0, its straight colour through the sRGB
    /// curve; `None` when nothing shows.
    pub fn of(source: &WorkingBuffer) -> Option<Stats> {
        let w = source.width().max(1);
        let blocks: Vec<Stats> = source
            .data()
            .par_chunks(w * 4 * 64)
            .map(|block| {
                let mut s = Stats::empty();
                let mut last: Option<([u32; 4], [usize; 4], [f64; 3])> = None;
                for px in block.chunks_exact(4) {
                    let a = px[3] as f64;
                    if a <= 0.0 {
                        continue;
                    }
                    let key = [px[0].to_bits(), px[1].to_bits(), px[2].to_bits(), px[3].to_bits()];
                    let (b, e) = match last {
                        Some((k, b, e)) if k == key => (b, e),
                        _ => {
                            let e: [f64; 3] = std::array::from_fn(|c| crate::grade::to_srgb((px[c] as f64 / a).clamp(0.0, 1.0)));
                            let y = LUMA[0] * e[0] + LUMA[1] * e[1] + LUMA[2] * e[2];
                            ([bin(e[0]), bin(e[1]), bin(e[2]), bin(y)], e)
                        }
                    };
                    last = Some((key, b, e));
                    for c in 0..4 {
                        s.hist[c][b[c]] += a;
                    }
                    for c in 0..3 {
                        s.sums[c][b[3]] += a * e[c];
                    }
                }
                s
            })
            .collect();
        let mut all = Stats::empty();
        for b in &blocks {
            all.add(b, 1.0);
        }
        (all.total() > 0.0).then_some(all)
    }

    /// Frames' statistics, each over its own total, added.
    pub fn added(many: &[Stats]) -> Option<Stats> {
        let mut all = Stats::empty();
        for s in many {
            all.add(s, 1.0 / s.total());
        }
        (!many.is_empty()).then_some(all)
    }

    /// How far apart two pictures' brightness is: the histograms in 16 groups of 16 bins, each
    /// over its total, and half the sum of the differences, 0 to 1.
    pub fn difference(&self, other: &Stats) -> f64 {
        let (t, u) = (self.total(), other.total());
        let group = |h: &[f64; 256], g: usize| h[g * 16..g * 16 + 16].iter().sum::<f64>();
        0.5 * (0..16).map(|g| (group(&self.hist[3], g) / t - group(&other.hist[3], g) / u).abs()).sum::<f64>()
    }
}

/// The black bin, the lowest whose running total from the bottom is above `black` per cent of
/// the whole, and the white bin, the highest whose running total from the top is above `white`.
pub fn clip_points(h: &[f64; 256], black: f64, white: f64) -> (usize, usize) {
    let t: f64 = h.iter().sum();
    let (lo, hi) = (t * black / 100.0, t * white / 100.0);
    let mut run = 0.0;
    let kb = (0..256).find(|&k| {
        run += h[k];
        run > lo
    });
    let mut run = 0.0;
    let kw = (0..256).rev().find(|&k| {
        run += h[k];
        run > hi
    });
    (kb.unwrap_or(255), kw.unwrap_or(0))
}

/// Spread Tones' map of a histogram: each bin to its running total, less the lowest bin's
/// weight, over the whole less that; `None` for a histogram all in one bin.
fn spread_map(h: &[f64; 256]) -> Option<[f64; 256]> {
    let t: f64 = h.iter().sum();
    let c0 = h.iter().copied().find(|&v| v != 0.0)?;
    if t - c0 <= 0.0 {
        return None;
    }
    let mut run = 0.0;
    Some(std::array::from_fn(|k| {
        run += h[k];
        ((run - c0) / (t - c0)).clamp(0.0, 1.0)
    }))
}

type Tone = Box<dyn Fn([f64; 3]) -> [f64; 3] + Sync>;

/// What a Stretch effect does to a colour, by `kind` ("levels", "contrast" or "color").
fn stretch(kind: &str, s: &Stats, black: f64, white: f64, snap: bool) -> Option<Tone> {
    // Each channel: a line through (lo, 0) and (hi, 1), or nothing, then a power.
    let mut lines: [Option<(f64, f64)>; 3] = [None; 3];
    let mut power = [1.0; 3];
    if kind == "color" {
        let (kb, kw) = clip_points(&s.hist[3], black, white);
        if kw <= kb {
            return None;
        }
        let colour = |r: std::ops::RangeInclusive<usize>| -> [f64; 3] {
            let t: f64 = s.hist[3][r.clone()].iter().sum();
            std::array::from_fn(|c| s.sums[c][r.clone()].iter().sum::<f64>() / t)
        };
        let (dark, light) = (colour(0..=kb), colour(kw..=255));
        for c in 0..3 {
            if light[c] - dark[c] > 1e-6 {
                lines[c] = Some((dark[c], light[c]));
            }
        }
        if snap {
            let mean: [f64; 3] = std::array::from_fn(|c| s.sums[c].iter().sum::<f64>() / s.total());
            let m: [f64; 3] = std::array::from_fn(|c| line(lines[c], mean[c]).clamp(0.0, 1.0));
            let t = (m[0] + m[1] + m[2]) / 3.0;
            for c in 0..3 {
                if 0.0 < m[c] && m[c] < 1.0 && 0.0 < t && t < 1.0 {
                    power[c] = (t.ln() / m[c].ln()).clamp(0.1, 10.0);
                }
            }
        }
    } else {
        let pooled: [f64; 256] = std::array::from_fn(|k| s.hist[0][k] + s.hist[1][k] + s.hist[2][k]);
        for c in 0..3 {
            let (kb, kw) = clip_points(if kind == "levels" { &s.hist[c] } else { &pooled }, black, white);
            if kw > kb {
                lines[c] = Some((kb as f64 / 255.0, kw as f64 / 255.0));
            }
        }
    }
    Some(Box::new(move |e| {
        std::array::from_fn(|c| {
            let v = line(lines[c], e[c]);
            if power[c] == 1.0 { v } else { v.clamp(0.0, 1.0).powf(power[c]) }
        })
    }))
}

fn line(l: Option<(f64, f64)>, v: f64) -> f64 {
    match l {
        Some((lo, hi)) => (v - lo) / (hi - lo),
        None => v,
    }
}

/// What Spread Tones does to a colour: `equalize` "rgb", "photoshop" or "brightness", `amount`
/// 0 to 100.
fn spread(s: &Stats, equalize: &str, amount: f64) -> Option<Tone> {
    let a = amount / 100.0;
    if equalize == "brightness" {
        let m = spread_map(&s.hist[3])?;
        return Some(Box::new(move |e| {
            let y = LUMA[0] * e[0] + LUMA[1] * e[1] + LUMA[2] * e[2];
            if y <= 0.0 {
                return e;
            }
            let k = m[bin(y)] / y;
            std::array::from_fn(|c| e[c] + a * (e[c] * k - e[c]))
        }));
    }
    let maps: [Option<[f64; 256]>; 3] = if equalize == "rgb" {
        std::array::from_fn(|c| spread_map(&s.hist[c]))
    } else {
        let one = spread_map(&std::array::from_fn(|k| s.hist[0][k] + s.hist[1][k] + s.hist[2][k]));
        [one; 3]
    };
    Some(Box::new(move |e| {
        std::array::from_fn(|c| match &maps[c] {
            Some(m) => e[c] + a * (m[bin(e[c])] - e[c]),
            None => e[c],
        })
    }))
}

/// D-351: the effect on `source`, by the statistics `given` (temporal smoothing's, which compose
/// filled in) or else `source`'s own. Nothing showing, nothing changes.
pub(crate) fn apply(source: &mut WorkingBuffer, effect: &crate::effects::Effect) {
    use crate::effects::Effect;
    let own;
    let given = match effect {
        Effect::AutoTone { stats: Some(s), .. } => s.as_ref(),
        _ => match Stats::of(source) {
            Some(s) => {
                own = s;
                &own
            }
            None => return,
        },
    };
    let tone = match effect {
        Effect::AutoTone { kind, black_clip, white_clip, snap_neutral_midtones, .. } => {
            stretch(kind, given, *black_clip, *white_clip, *kind == "color" && snap_neutral_midtones == "on")
        }
        Effect::SpreadTones { equalize, amount } => spread(given, equalize, *amount),
        _ => None,
    };
    if let Some(tone) = tone {
        crate::grade::grade_pixels(source, false, |_, e| tone(e));
    }
}

/// D-375: Color Link's colour, each channel 0 to 1, from a picture's statistics: `sample` one of
/// `effects::LINK_SAMPLES`, `clip` 0 to 49 per cent each end.
pub(crate) fn link_colour(s: &Stats, sample: &str, clip: f64) -> [f64; 3] {
    match sample {
        "median" => std::array::from_fn(|c| clip_points(&s.hist[c], 50.0, 50.0).0 as f64 / 255.0),
        "brightest" | "darkest" => {
            let (kb, kw) = clip_points(&s.hist[3], clip, clip);
            let k = if sample == "brightest" { kw } else { kb };
            std::array::from_fn(|c| s.sums[c][k] / s.hist[3][k])
        }
        _ => std::array::from_fn(|c| {
            let (kb, kw) = clip_points(&s.hist[c], clip, clip);
            match sample {
                "max_rgb" => kw as f64 / 255.0,
                "min_rgb" => kb as f64 / 255.0,
                _ => {
                    let h = &s.hist[c][kb..=kw];
                    h.iter().zip(kb..).map(|(v, k)| v * k as f64).sum::<f64>() / h.iter().sum::<f64>() / 255.0
                }
            }
        }),
    }
}

/// D-375: Color Link's tint, colour `c` at `opacity` 0 to 1 by `blend` (Paraffin's mixer). With
/// `stencil` only where the layer shows, at its own covering; without, laid over the whole layer
/// as one layer of colour, the empty pixels too. Opacity 0 leaves the layer exactly as it is.
pub(crate) fn link(source: &mut WorkingBuffer, c: [f64; 3], opacity: f64, stencil: bool, blend: &str) {
    if opacity <= 0.0 {
        return;
    }
    let (o, mix) = (opacity, crate::grade::mixer(blend));
    if stencil {
        crate::grade::grade_pixels(source, false, |_, e| std::array::from_fn(|i| e[i] + o * (mix(e[i], c[i]) - e[i])));
        return;
    }
    source.data_mut().par_chunks_exact_mut(4).for_each(|px| {
        let a = px[3] as f64;
        let e: [f64; 3] = std::array::from_fn(|i| if a > 0.0 { crate::grade::to_srgb((px[i] as f64 / a).clamp(0.0, 1.0)) } else { 0.0 });
        let a2 = o + a * (1.0 - o);
        for i in 0..3 {
            let r = (o * (1.0 - a) * c[i] + o * a * mix(e[i], c[i]) + (1.0 - o) * a * e[i]) / a2;
            px[i] = (crate::grade::to_linear(r.clamp(0.0, 1.0)) * a2) as f32;
        }
        px[3] = a2 as f32;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_points_and_maps() {
        let mut h = [0.0; 256];
        h[10] = 1.0;
        h[20] = 98.0;
        h[200] = 1.0;
        assert_eq!(clip_points(&h, 0.0, 0.0), (10, 200));
        assert_eq!(clip_points(&h, 1.5, 1.5), (20, 20));
        let m = spread_map(&h).unwrap();
        assert_eq!((m[10], m[200]), (0.0, 1.0));
        let mut one = [0.0; 256];
        one[7] = 3.0;
        assert!(spread_map(&one).is_none());
    }
}
