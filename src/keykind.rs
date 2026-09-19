//! D-69: what an edit must keep true of auto, continuous and roving keys, and a position
//! taken apart into X and Y and put together again.
//!
//! None of this changes how a frame is worked out. The file always holds the eases and the
//! frames that are rendered; a key's kind or `roving` says what an edit leaves behind. The
//! formulas are D-69's, and `tools/keykind_reference.py` works the same ones a second way.

use crate::model::{Interp, Keyframe, Kind, Property, Value};

/// The ease that is the straight line, which a linear segment becomes when a side of it is set.
const LINE: [f64; 4] = [1.0 / 3.0, 1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0];
/// A curved path's length is measured along this many straight pieces.
const PIECES: usize = 64;

/// Which side of a key an edit set by hand, so the other side follows it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side {
    In,
    Out,
}

pub fn curve(key: &Keyframe) -> [f64; 4] {
    match key.interp {
        Interp::Ease { x1, y1, x2, y2 } => [x1, y1, x2, y2],
        _ => LINE,
    }
}

fn ease(c: [f64; 4]) -> Interp {
    Interp::Ease {
        x1: c[0],
        y1: c[1],
        x2: c[2],
        y2: c[3],
    }
}

/// How far the segment from `a` to `b` goes, at `a`'s end and at `b`'s: the difference for a
/// number, signed; for a pair the straight distance, or three times the path handle there.
fn reach(a: &Keyframe, b: &Keyframe) -> (f64, f64) {
    match (a.value, b.value) {
        (Value::Scalar(x), Value::Scalar(y)) => (y - x, y - x),
        (Value::Vec2(ax, ay), Value::Vec2(bx, by)) => {
            let chord = (bx - ax).hypot(by - ay);
            (
                a.spatial.map_or(chord, |s| 3.0 * s[2].hypot(s[3])),
                b.spatial.map_or(chord, |s| 3.0 * s[0].hypot(s[1])),
            )
        }
        _ => (0.0, 0.0),
    }
}

/// The speed into key `i` and out of it. `None` on a side with no segment, a hold, no reach,
/// or a handle of no length, which has no slope to read.
fn speeds(keys: &[Keyframe], i: usize) -> (Option<f64>, Option<f64>) {
    let k = &keys[i];
    let mut s_in = None;
    let mut s_out = None;
    if i > 0 && keys[i - 1].interp != Interp::Hold {
        let p = &keys[i - 1];
        let l = reach(p, k).1;
        let c = curve(p);
        if l != 0.0 && c[2] < 1.0 {
            s_in = Some((1.0 - c[3]) / (1.0 - c[2]) * l / (k.frame - p.frame) as f64);
        }
    }
    if i + 1 < keys.len() && k.interp != Interp::Hold {
        let n = &keys[i + 1];
        let l = reach(k, n).0;
        let c = curve(k);
        if l != 0.0 && c[0] > 0.0 {
            s_out = Some(c[1] / c[0] * l / (n.frame - k.frame) as f64);
        }
    }
    (s_in, s_out)
}

/// Give one side of key `i` the speed `s`. The handle keeps its length, or becomes a third.
fn set_speed(keys: &mut [Keyframe], i: usize, s: f64, side: Side, third: bool) {
    match side {
        Side::In => {
            let (p, k) = (keys[i - 1], keys[i]);
            let mut c = curve(&p);
            if third {
                c[2] = 2.0 / 3.0;
            }
            c[3] = 1.0 - s * (1.0 - c[2]) * (k.frame - p.frame) as f64 / reach(&p, &k).1;
            keys[i - 1].interp = ease(c);
        }
        Side::Out => {
            let (k, n) = (keys[i], keys[i + 1]);
            let mut c = curve(&k);
            if third {
                c[0] = 1.0 / 3.0;
            }
            c[1] = s * c[0] * (n.frame - k.frame) as f64 / reach(&k, &n).0;
            keys[i].interp = ease(c);
        }
    }
}

/// Make every auto and continuous key true again. `touched` names the sides an edit set by
/// hand: a continuous key's other side follows a touched one rather than meeting it half way.
pub fn settle(keys: &mut [Keyframe], touched: &[(usize, Side)]) {
    for i in 0..keys.len() {
        let (s_in, s_out) = speeds(keys, i);
        match keys[i].kind {
            Kind::Bezier => {}
            Kind::Auto => {
                let last = keys.len() - 1;
                if i == 0 || i == last {
                    // An end key leaves, or arrives, as a straight line.
                    if i == 0 && s_out.is_some() {
                        let l = reach(&keys[0], &keys[1]).0;
                        let s = l / (keys[1].frame - keys[0].frame) as f64;
                        set_speed(keys, 0, s, Side::Out, true);
                    } else if i == last && i > 0 && s_in.is_some() {
                        let l = reach(&keys[i - 1], &keys[i]).1;
                        let s = l / (keys[i].frame - keys[i - 1].frame) as f64;
                        set_speed(keys, i, s, Side::In, true);
                    }
                    continue;
                }
                let s = (reach(&keys[i - 1], &keys[i]).1 + reach(&keys[i], &keys[i + 1]).0)
                    / (keys[i + 1].frame - keys[i - 1].frame) as f64;
                if s_in.is_some() {
                    set_speed(keys, i, s, Side::In, true);
                }
                if s_out.is_some() {
                    set_speed(keys, i, s, Side::Out, true);
                }
            }
            Kind::Continuous => {
                let (Some(a), Some(b)) = (s_in, s_out) else {
                    continue;
                };
                let s = if touched.contains(&(i, Side::Out)) {
                    b
                } else if touched.contains(&(i, Side::In)) {
                    a
                } else {
                    (a + b) / 2.0
                };
                set_speed(keys, i, s, Side::In, false);
                set_speed(keys, i, s, Side::Out, false);
            }
        }
    }
}

/// The length of the path from `a` to `b`: straight, or D-53's curve in 64 straight pieces.
fn length(a: &Keyframe, b: &Keyframe) -> f64 {
    let (Value::Vec2(x0, y0), Value::Vec2(x3, y3)) = (a.value, b.value) else {
        return 0.0;
    };
    if a.spatial.is_none() && b.spatial.is_none() {
        return (x3 - x0).hypot(y3 - y0);
    }
    let third = ((x3 - x0) / 3.0, (y3 - y0) / 3.0);
    let (ox, oy) = a.spatial.map_or(third, |s| (s[2], s[3]));
    let (ix, iy) = b.spatial.map_or((-third.0, -third.1), |s| (s[0], s[1]));
    let at = |t: f64| {
        let cubic = |p0: f64, p1: f64, p2: f64, p3: f64| {
            let s = 1.0 - t;
            s * s * s * p0 + 3.0 * s * s * t * p1 + 3.0 * s * t * t * p2 + t * t * t * p3
        };
        (
            cubic(x0, x0 + ox, x3 + ix, x3),
            cubic(y0, y0 + oy, y3 + iy, y3),
        )
    };
    (0..PIECES)
        .map(|n| {
            let (p, q) = (
                at(n as f64 / PIECES as f64),
                at((n + 1) as f64 / PIECES as f64),
            );
            (q.0 - p.0).hypot(q.1 - p.1)
        })
        .sum()
}

/// Put every roving key on its frame. `false` when a run has fewer frames than roving keys, or
/// a roving key has come to be first or last; the keys are then not to be used.
pub fn rove(keys: &mut [Keyframe]) -> bool {
    let mut i = 0;
    while i < keys.len() {
        if !keys[i].roving {
            i += 1;
            continue;
        }
        let mut j = i;
        while j < keys.len() && keys[j].roving {
            j += 1;
        }
        if i == 0 || j == keys.len() {
            return false;
        }
        let (first, end) = (keys[i - 1].frame, keys[j].frame);
        if ((end - first - 1) as usize) < j - i || end <= first {
            return false;
        }
        let lengths: Vec<f64> = (i - 1..j).map(|n| length(&keys[n], &keys[n + 1])).collect();
        let total: f64 = lengths.iter().sum();
        let mut so_far = 0.0;
        for n in i..j {
            so_far += lengths[n - i];
            let share = if total > 0.0 {
                so_far / total
            } else {
                (n - i + 1) as f64 / (j - i + 1) as f64
            };
            keys[n].frame = (first as f64 + (end - first) as f64 * share + 0.5).floor() as i32;
        }
        // Never two keys on one frame, and never outside the run.
        for n in i..j {
            keys[n].frame = keys[n].frame.max(keys[n - 1].frame + 1);
        }
        for n in (i..j).rev() {
            keys[n].frame = keys[n].frame.min(keys[n + 1].frame - 1);
        }
        i = j;
    }
    true
}

/// A position's keys as X's and Y's: the same frames, eases and kinds; no path, no roving.
pub fn separate(position: &Property) -> (Property, Property) {
    let half = |pick: fn((f64, f64)) -> f64| {
        let of = |v: Value| Value::Scalar(v.as_vec2().map_or(0.0, pick));
        let mut out = Property::constant(of(position.base()));
        for k in position.keyframes() {
            out.set_keyframe(Keyframe {
                value: of(k.value),
                spatial: None,
                roving: false,
                ..*k
            });
        }
        out
    };
    (half(|p| p.0), half(|p| p.1))
}

/// One position again: a key wherever either had one, holding both values at that frame, with
/// X's ease and kind where X had a key and else Y's.
pub fn join(x: &Property, y: &Property) -> Property {
    let n = |p: &Property, frame: i32| p.value_at(frame).as_scalar().unwrap_or(0.0);
    let mut out = Property::constant(Value::Vec2(
        x.base().as_scalar().unwrap_or(0.0),
        y.base().as_scalar().unwrap_or(0.0),
    ));
    for own in y.keyframes().iter().chain(x.keyframes()) {
        out.set_keyframe(Keyframe {
            value: Value::Vec2(n(x, own.frame), n(y, own.frame)),
            ..*own
        });
    }
    out
}
