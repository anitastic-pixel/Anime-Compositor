//! D-89's glow: document 21's rule, on a layer's own pixels.
//!
//! This program's own method, modelled on After Effects' Glow and built from document 21's
//! Gaussian blur and D-88's colour test; nothing is ported. `tools/glow_reference.py` is the same
//! rule worked a second way, and `tests/b33_glow.rs` holds this to its numbers.

use crate::color::srgb_to_linear;
use crate::render::Glow;
use crate::selective_blur::parse_hex;
use crate::WorkingBuffer;
use rayon::prelude::*;

/// B-51: a Glow's settings read once, for the CPU here and for the card (`render::OnCard`).
/// Every number is already inside its range and every colour already parses.
#[allow(clippy::too_many_arguments)]
pub(crate) fn settings(
    based_on: &str,
    threshold: f64,
    colors: &[String],
    tolerance: f64,
    radius: f64,
    intensity: f64,
    operation: &str,
    tint: &str,
    units: &str,
    look: &crate::effects::GlowLook,
) -> Glow {
    let mut targets = [[0; 3]; 8];
    let chosen = crate::selective_blur::targets(colors);
    targets[..chosen.len()].copy_from_slice(&chosen);
    // The words are already valid (`effects::glow_fault`); the first is never reached.
    let word = |w: &str, list: &[&'static str]| list.iter().copied().find(|x| *x == w).unwrap_or(list[0]);
    let hex = |c: &str| parse_hex(c).unwrap_or([0; 3]);
    Glow {
        operation: word(operation, &crate::effects::GLOW_OPERATIONS),
        composite: word(&look.composite, &["behind", "on_top", "none"]),
        axes: (look.dimensions != "vertical", look.dimensions != "horizontal"),
        ab: (look.glow_colors == "a_and_b").then(|| crate::render::GlowAb {
            a: hex(&look.color_a),
            b: hex(&look.color_b),
            looping: word(&look.color_looping, &["triangle_aba", "sawtooth_ab", "sawtooth_ba", "triangle_bab"]),
            loops: look.color_loops,
            phase: look.color_phase,
            midpoint: look.midpoint,
        }),
        bright: based_on == "bright",
        threshold,
        targets,
        count: chosen.len(),
        tolerance,
        radius,
        intensity,
        screen: operation != "add",
        tint: parse_hex(tint),
        after_effects: units == "after_effects",
        display: false,
        fast: look.quality == "fast",
    }
}

/// (1) Whether a premultiplied pixel glows. A pixel that does not show never glows. The test is
/// on its 8-bit encoded colour, as a drawing program stores it: bright parts by its brightest
/// channel, as Bloom's, and chosen colours by D-88.
pub(crate) fn glows(px: &[f32], g: &Glow) -> bool {
    if g.bright {
        return crate::bloom::bright(px, g.threshold);
    }
    crate::selective_blur::chosen(px, &g.targets[..g.count], g.tolerance)
}

/// Lay the glow of `source` on top of it, in place, and return how far it grew on each side.
/// Intensity 0, or no pixel that glows, changes nothing.
pub(crate) fn glow(source: &mut WorkingBuffer, g: &Glow) -> usize {
    let (w, h) = (source.width(), source.height());
    if w == 0 || h == 0 {
        return 0;
    }
    // D-747: Composite Original None leaves the light alone, so with none it leaves nothing.
    let none = g.composite == "none";
    if g.intensity == 0.0 {
        if none {
            *source = WorkingBuffer::transparent(w, h);
        }
        return 0;
    }
    // D-337: in display values the tint is used as written, and what glows is still tested on
    // the pixel's own linear value.
    let tint = g.tint.map(|t| {
        t.map(|v| {
            if g.display {
                v as f32 / 255.0
            } else {
                srgb_to_linear(v as f32 / 255.0)
            }
        })
    });
    let lights = |px: &[f32]| {
        if g.display && px[3] > 0.0 {
            let a = px[3];
            glows(
                &[
                    srgb_to_linear(px[0] / a) * a,
                    srgb_to_linear(px[1] / a) * a,
                    srgb_to_linear(px[2] / a) * a,
                    a,
                ],
                g,
            )
        } else {
            glows(px, g)
        }
    };

    // (2) The light a glowing pixel gives: the pixel itself, or the tint at its covering.
    // D-746: in display values and After Effects units each channel's light is it times
    // GI * (t + 16 (1 - t)) / 3, held at 1, and based on bright each channel glows on its own.
    let ae = g.display && g.after_effects;
    let t = g.threshold / 100.0;
    let s = (g.intensity * (t + 16.0 * (1.0 - t)) / 3.0) as f32;
    let held = |v: f32| (s * v).min(1.0);
    let mut light = WorkingBuffer::transparent(w, h);
    light
        .data_mut()
        .par_chunks_exact_mut(4)
        .zip(source.data().par_chunks_exact(4))
        .for_each(|(l, px)| {
            let a = px[3];
            if ae && g.bright && tint.is_none() {
                if a > 0.0 {
                    let on = [0, 1, 2].map(|c| {
                        100.0 * crate::color::quantise_u8(px[c] / a) as f64 >= 255.0 * g.threshold
                    });
                    if on.contains(&true) {
                        for c in 0..3 {
                            l[c] = if on[c] { held(px[c] / a) * a } else { 0.0 };
                        }
                        l[3] = a;
                    }
                }
            } else if lights(px) {
                match (tint, ae) {
                    (Some(t), true) => l.copy_from_slice(&[held(t[0]) * a, held(t[1]) * a, held(t[2]) * a, a]),
                    (Some(t), false) => l.copy_from_slice(&[t[0] * a, t[1] * a, t[2] * a, a]),
                    (None, true) => l.copy_from_slice(&[held(px[0] / a) * a, held(px[1] / a) * a, held(px[2] / a) * a, a]),
                    (None, false) => l.copy_from_slice(px),
                }
            }
        });
    if light.data().chunks_exact(4).all(|l| l[3] == 0.0) {
        if none {
            *source = WorkingBuffer::transparent(w, h);
        }
        return 0;
    }

    // (3) Spread: document 21's blur at sigma radius / 3, growing the light's bounds; D-322's
    // After Effects units blur as Gaussian Blur at Blurriness radius.
    // D-746: in display values at sigma 0.4 times the radius, Valeri Visuals' white disc.
    let (sigma, long) = if ae {
        (0.4 * g.radius, true)
    } else {
        crate::effects::glow_reach(
            g.radius,
            if g.after_effects {
                "after_effects"
            } else {
                "classic"
            },
        )
    };
    // D-914: Quality Fast above sigma 8, in both dimensions, on the reduced blur's cells.
    let d = crate::blurs::cell(sigma);
    let r = if g.fast && g.axes == (true, true) && d > 1 {
        let r = crate::effects::reach_radius(sigma, long);
        light = crate::blurs::reduced(&light, sigma, d, long, r);
        r
    } else {
        crate::effects::blur_axes(
            &mut light,
            &crate::effects::reach_weights(sigma, long),
            // D-747: Glow Dimensions.
            g.axes,
        )
    };

    // (4) Strength and (5) on top of the picture, which is empty outside its own bounds.
    // D-331: Creative COW's GI*(GT/100) + GI*16*(1-GT/100) is the colour read straight: the
    // colour at the intensity, the covering divided by t + 16 (1 - t), in linear light.
    // D-746 (replacing D-337's GI * c): in display values the intensity is spent in (2), and the light is laid on at 1.
    let (k, ka) = if ae {
        (1.0, 1.0)
    } else if g.after_effects {
        let t = g.threshold / 100.0;
        (g.intensity as f32, (1.0 / (t + 16.0 * (1.0 - t))) as f32)
    } else {
        (g.intensity as f32, g.intensity as f32)
    };
    // D-747: A & B colours in the layer's values, as the tint's.
    let ab = g.ab.map(|ab| {
        let c = |v: [u8; 3]| {
            v.map(|x| {
                if g.display {
                    x as f32 / 255.0
                } else {
                    srgb_to_linear(x as f32 / 255.0)
                }
            })
        };
        (c(ab.a), c(ab.b), ab)
    });
    let (mix, whole) = (crate::grade::noise_mixer(g.operation), crate::grade::noise_whole(g.operation));
    let lw = light.width();
    let o = source.data();
    light
        .data_mut()
        .par_chunks_exact_mut(lw * 4)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, l) in row.chunks_exact_mut(4).enumerate() {
                let (sx, sy) = (x as isize - r as isize, y as isize - r as isize);
                let p = if (0..w as isize).contains(&sx) && (0..h as isize).contains(&sy) {
                    let i = (sy as usize * w + sx as usize) * 4;
                    [o[i], o[i + 1], o[i + 2], o[i + 3]]
                } else {
                    [0.0; 4]
                };
                let mut lc = [l[0] * k, l[1] * k, l[2] * k, l[3] * ka];
                if let Some((ca, cb, ab)) = ab {
                    lc = colored(lc, ca, cb, &ab);
                }
                match (none, g.operation) {
                    (true, _) => {
                        l.copy_from_slice(&lc);
                        l[3] = l[3].min(1.0);
                    }
                    (false, "add") => {
                        for c in 0..3 {
                            l[c] = p[c] + lc[c];
                        }
                        l[3] = (p[3] + lc[3]).min(1.0);
                    }
                    (false, "screen") => {
                        for c in 0..4 {
                            let v = lc[c].clamp(0.0, 1.0);
                            l[c] = p[c] + v - p[c] * v;
                        }
                    }
                    // D-747: any other operation, the light on top of the picture or behind it.
                    _ => {
                        let la = lc[3].clamp(0.0, 1.0);
                        let lcol = [0, 1, 2].map(|c| if lc[3] > 0.0 { (lc[c] / lc[3]).clamp(0.0, 1.0) } else { 0.0 });
                        let pcol = [0, 1, 2].map(|c| if p[3] > 0.0 { p[c] / p[3] } else { 0.0 });
                        let ((da, dc), (sa, sc)) = if g.composite == "on_top" {
                            ((p[3], pcol), (la, lcol))
                        } else {
                            ((la, lcol), (p[3], pcol))
                        };
                        l.copy_from_slice(&blend((da, dc), (sa, sc), mix, whole));
                    }
                }
            }
        });
    *source = light;
    r
}

/// D-747: Glow Colors A & B. The light's brightest channel v, from 0 to 1, picks a place along
/// the loops of A and B (v 1 is A, at Color Phase 0), bent so that the half-way mix falls at
/// A & B Midpoint, and the light takes that colour at v. Our basis; After Effects publishes none.
fn colored(lc: [f32; 4], ca: [f32; 3], cb: [f32; 3], ab: &crate::render::GlowAb) -> [f32; 4] {
    let v = lc[0].max(lc[1]).max(lc[2]).clamp(0.0, 1.0);
    let u = (ab.loops * (1.0 - v as f64) + ab.phase / 360.0).rem_euclid(1.0);
    let f = match ab.looping {
        "sawtooth_ab" => u,
        "sawtooth_ba" => 1.0 - u,
        "triangle_bab" => (2.0 * u - 1.0).abs(),
        _ => 1.0 - (2.0 * u - 1.0).abs(),
    };
    let m = (ab.midpoint / 100.0).clamp(1e-3, 1.0 - 1e-3);
    let f = f.powf(0.5f64.ln() / m.ln()) as f32;
    [0, 1, 2, 3].map(|c| if c == 3 { lc[3] } else { (ca[c] + (cb[c] - ca[c]) * f) * v })
}

/// D-747: `s` laid on `d` with an operation (`grade::noise_mixer`, or `grade::noise_whole`'s
/// whole-colour ones), straight colours in, a premultiplied pixel out: the W3C's
/// (1 - as) ad cd + (1 - ad) as cs + as ad B(cd, cs).
fn blend(
    (da, dc): (f32, [f32; 3]),
    (sa, sc): (f32, [f32; 3]),
    mix: fn(f64, f64) -> f64,
    whole: Option<fn([f64; 3], [f64; 3]) -> [f64; 3]>,
) -> [f32; 4] {
    let (d, s) = (dc.map(|v| v as f64), sc.map(|v| v as f64));
    let b = whole.map_or_else(|| [0, 1, 2].map(|c| mix(d[c], s[c])), |f| f(d, s));
    let (da64, sa64) = (da as f64, sa as f64);
    let out = [0, 1, 2].map(|c| ((1.0 - sa64) * da64 * d[c] + (1.0 - da64) * sa64 * s[c] + sa64 * da64 * b[c]) as f32);
    [out[0], out[1], out[2], sa + da - sa * da]
}
