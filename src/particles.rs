//! D-610: the particle engine (P0-6) and its first effect, Particle Space
//! (`core.particle_space`), modelled on what CycoreFX's CC Particle World is for. Every rule is
//! ours, written after what each control is for (document 21); `tools/particle_space_reference.py`
//! is the reference, and this file follows it step for step.
//!
//! No state is carried from frame to frame (document 20): a particle at a frame is a pure
//! function of the seed, its number and the settings history, so a frame drawn alone equals the
//! same frame played from the start.

use crate::effects::{Effect, PARTICLE_SPACE_WORDS};
use crate::WorkingBuffer;
use rayon::prelude::*;
use std::f64::consts::PI;

type V = [f64; 3];

fn add(a: V, b: V) -> V {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn sub(a: V, b: V) -> V {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn mul(a: V, s: f64) -> V {
    [a[0] * s, a[1] * s, a[2] * s]
}

fn dot(a: V, b: V) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: V, b: V) -> V {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn unit_or(v: V, fallback: V) -> V {
    let n = dot(v, v).sqrt();
    if n > 0.0 {
        [v[0] / n, v[1] / n, v[2] / n]
    } else {
        fallback
    }
}

fn sphere(z: f64, turn: f64) -> V {
    let s = (1.0 - z * z).max(0.0).sqrt();
    [s * turn.cos(), s * turn.sin(), z]
}

// Where each setting sits in `values` (`effects::PARTICLE_SPACE_SETTINGS`); a vector's three
// follow one another.
const RATE: usize = 0;
const LONGEVITY: usize = 1;
const PRODUCER: usize = 2;
const RADIUS: usize = 5;
const VELOCITY: usize = 8;
const INHERIT: usize = 9;
const GRAVITY: usize = 10;
const RESISTANCE: usize = 11;
const AXIS: usize = 12;
const EXTRA: usize = 15;
const EXTRA_ANGLE: usize = 16;
const GRAVITY_VECTOR: usize = 17;
const FLOOR: usize = 20;
const BOUNCINESS: usize = 21;
const RANDOM_BOUNCINESS: usize = 22;
const SPREAD: usize = 23;
const WIND: usize = 24;
const TURBULENCE: usize = 27;
const TURBULENCE_SCALE: usize = 28;
const EVOLUTION: usize = 29;
const BIRTH_SIZE: usize = 30;
const DEATH_SIZE: usize = 31;
const SIZE_VARIATION: usize = 32;
const OPACITY: usize = 33;
const VOLUME_SHADE: usize = 34;
const DISTANCE: usize = 35;
const ROTATION: usize = 36;
const FOV: usize = 39;
const CUE_DISTANCE: usize = 40;
const LIGHT_DIRECTION: usize = 41;
const LIGHT_HEIGHT: usize = 42;
const SEED: usize = 43;

// The animation modes, in `PARTICLE_SPACE_WORDS[0]`'s order.
const DIRECTION_AXIS: usize = 1;
const CONE_AXIS: usize = 2;
const VISCOUS: usize = 3;
const TWIRL: usize = 4;
const TWIRLY: usize = 5;
const VORTEX: usize = 6;
const FIRE: usize = 7;
const JET_SIDEWAYS: usize = 8;
const FRACTAL_OMNI: usize = 9;
const FRACTAL_UNI: usize = 10;

/// The sprite list's head: Volume Shade / 100, the light (three), the transfer mode, 1 with
/// Composite With Original on, the count; then ten numbers a sprite, farthest first.
pub(crate) const HEAD: usize = 7;
/// The tiles the sprites are binned into, as the card's workgroups.
pub(crate) const TILE: usize = 16;

/// The settings' history for one frame, as the reference's `Run`.
struct Run<'a> {
    now: &'a [f64; 44],
    samples: Option<&'a [[f64; 44]]>,
    u: i64,
    fps: f64,
    seed: u64,
    mode: usize,
    floor_action: usize,
}

/// A particle as it moves.
struct Particle {
    k: i64,
    b: f64,
    x: V,
    v: V,
    x0: V,
    sizes: (f64, f64),
    size_factor: f64,
    twirl: f64,
    glued: bool,
    iced: bool,
    bounces: u64,
}

fn word(words: &[String; 7], i: usize) -> usize {
    PARTICLE_SPACE_WORDS[i].1.iter().position(|w| *w == words[i]).unwrap_or(0)
}

impl Run<'_> {
    /// The settings sample `i` frames after the in point, held at the last.
    fn sample(&self, i: usize, n: usize) -> f64 {
        self.samples.map_or(self.now[n], |s| s[i.min(s.len() - 1)][n])
    }

    fn at(&self, n: usize, t: f64) -> f64 {
        let Some(s) = self.samples else {
            return self.now[n];
        };
        let last = s.len() - 1;
        let i0 = t.floor();
        if i0 < 0.0 {
            return s[0][n];
        }
        if i0 as usize >= last {
            return s[last][n];
        }
        let (a, b) = (s[i0 as usize][n], s[i0 as usize + 1][n]);
        a + (b - a) * (t - i0)
    }

    fn vec(&self, n: usize, t: f64) -> V {
        [self.at(n, t), self.at(n + 1, t), self.at(n + 2, t)]
    }

    fn r(&self, k: i64, stream: u64) -> f64 {
        (crate::grade::unit(self.seed, k, stream as i64, 0, 0) + 1.0) / 2.0
    }

    /// Every (k, b_k) with b_k <= u that may be alive, oldest first.
    fn births(&self) -> Vec<(i64, f64)> {
        let n = self.u as usize + 2;
        let q: Vec<f64> = (0..n).map(|i| 100.0 * self.sample(i, RATE) / self.fps).collect();
        let lmax = (0..n).map(|i| self.sample(i, LONGEVITY)).fold(f64::MIN, f64::max) * self.fps;
        let u = self.u as f64;
        let mut out = Vec::new();
        if q.iter().all(|v| *v == q[0]) {
            if q[0] <= 0.0 {
                return out;
            }
            let lo = (q[0] * (u - lmax) - 0.5).floor().max(0.0) as i64;
            let hi = (q[0] * u - 0.5).floor() as i64 + 1;
            for k in lo..=hi {
                let b = (k as f64 + 0.5) / q[0];
                if b <= u {
                    out.push((k, b));
                }
            }
            return out;
        }
        let mut total = 0.0;
        for i in 0..self.u as usize {
            let next = total + (q[i] + q[i + 1]) / 2.0;
            if (i + 1) as f64 >= u - lmax {
                let (a, b) = ((q[i + 1] - q[i]) / 2.0, q[i]);
                let mut k = (total - 0.5_f64).floor() as i64 + 1;
                while k as f64 + 0.5 <= next {
                    let d = k as f64 + 0.5 - total;
                    let den = b + (b * b + 4.0 * a * d).max(0.0).sqrt();
                    let x = if den > 0.0 { (2.0 * d / den).min(1.0) } else { 1.0 };
                    out.push((k, i as f64 + x));
                    k += 1;
                }
            }
            total = next;
        }
        out
    }

    fn axes(&self, t: f64) -> V {
        if self.mode == FIRE {
            let g = self.vec(GRAVITY_VECTOR, t);
            return if g.iter().any(|c| *c != 0.0) { mul(unit_or(g, [0.0, -1.0, 0.0]), -1.0) } else { [0.0, -1.0, 0.0] };
        }
        unit_or(self.vec(AXIS, t), [0.0, -1.0, 0.0])
    }

    /// The particle's start: place, velocity and its fixed numbers.
    fn born(&self, k: i64, b: f64) -> Particle {
        let r = |s| self.r(k, s);
        let p = self.vec(PRODUCER, b);
        let rad = self.vec(RADIUS, b);
        let rho = r(0).powf(1.0 / 3.0);
        let d = sphere(1.0 - 2.0 * r(1), 2.0 * PI * r(2));
        let x = [0, 1, 2].map(|i| p[i] + rad[i] * rho * d[i]);
        let a = self.axes(b);
        let h = if a[0].abs() >= 0.9 { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] };
        let e1 = unit_or(cross(h, a), [1.0, 0.0, 0.0]);
        let e2 = cross(a, e1);
        let phi = 2.0 * PI * r(4);
        let cap = |cos_t: f64| {
            let sin_t = (1.0 - cos_t * cos_t).max(0.0).sqrt();
            add(mul(a, cos_t), mul(add(mul(e1, phi.cos()), mul(e2, phi.sin())), sin_t))
        };
        let alpha = (self.at(EXTRA_ANGLE, b) / 2.0).to_radians();
        let way = match self.mode {
            DIRECTION_AXIS | FRACTAL_UNI => cap(1.0 - r(3) * (1.0 - alpha.cos())),
            CONE_AXIS => cap(alpha.cos()),
            FIRE => cap(1.0 - r(3) * (1.0 - 20f64.to_radians().cos())),
            JET_SIDEWAYS => add(mul(e1, phi.cos()), mul(e2, phi.sin())),
            _ => sphere(1.0 - 2.0 * r(3), phi),
        };
        let speed = self.at(VELOCITY, b) * (1.0 + self.at(EXTRA, b) * (r(5) - 0.5));
        let mut v = mul(way, speed);
        let iv = self.at(INHERIT, b);
        if iv != 0.0 {
            let pv = mul(sub(self.vec(PRODUCER, b + 0.125), self.vec(PRODUCER, b - 0.125)), self.fps / 0.25);
            v = add(v, mul(pv, iv / 100.0));
        }
        let sv = self.at(SIZE_VARIATION, b) / 100.0;
        Particle {
            k,
            b,
            x,
            v,
            x0: x,
            sizes: (self.at(BIRTH_SIZE, b), self.at(DEATH_SIZE, b)),
            size_factor: 1.0 + sv * (2.0 * r(6) - 1.0),
            twirl: 2.0 * r(9),
            glued: false,
            iced: false,
            bounces: 0,
        }
    }

    fn step(&self, p: &mut Particle, t0: f64, h: f64) {
        if p.glued {
            return;
        }
        let at = |n| self.at(n, t0);
        let (mut x, mut v) = (p.x, p.v);
        let mut push = [0.0; 3];
        if matches!(self.mode, TWIRL | TWIRLY | VORTEX | FIRE) {
            let a = self.axes(t0);
            let dd = sub(x, self.vec(PRODUCER, t0));
            let dp = sub(dd, mul(a, dot(dd, a)));
            let vel = at(VELOCITY);
            if self.mode == FIRE {
                push = mul(dp, -4.0);
            } else {
                push = mul(cross(a, dp), 2.0 * vel / (dot(dp, dp) + 0.01));
                if self.mode == TWIRLY {
                    push = mul(push, p.twirl);
                }
                if self.mode == VORTEX {
                    push = add(push, mul(dp, -2.0 * vel));
                }
            }
        }
        let strength = at(TURBULENCE)
            + match self.mode {
                FIRE => 0.5,
                FRACTAL_OMNI | FRACTAL_UNI => 1.0,
                _ => 0.0,
            };
        if strength > 0.0 {
            let (evo, sc) = (at(EVOLUTION), at(TURBULENCE_SCALE));
            let q = [x[0] / sc + evo * 0.31, x[1] / sc + evo * 0.57, x[2] / sc + evo * 0.77];
            push = add(push, mul(curl(self.seed, q), strength));
        }
        if p.iced {
            push[1] = 0.0;
        }
        v = add(v, mul(push, h));
        let g = self.vec(GRAVITY_VECTOR, t0);
        let gu = if g.iter().any(|c| *c != 0.0) { unit_or(g, [0.0; 3]) } else { [0.0; 3] };
        let mut a = add(mul(gu, if self.mode == FIRE { -at(GRAVITY) } else { at(GRAVITY) }), self.vec(WIND, t0));
        let k = at(RESISTANCE) + if self.mode == VISCOUS { 3.0 } else { 0.0 };
        if p.iced {
            a[1] = 0.0;
        }
        let floor = at(FLOOR);
        let mut rem = h;
        let mut hops = 0;
        loop {
            let (x1, v1) = closed(x, v, a, k, rem);
            if self.floor_action == 0 || p.iced || !(x[1] <= floor && floor < x1[1]) {
                (p.x, p.v) = (x1, v1);
                return;
            }
            let (mut lo, mut hi) = (0.0, rem);
            for _ in 0..30 {
                let mid = (lo + hi) / 2.0;
                if closed(x, v, a, k, mid).0[1] > floor {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            let (mut xc, mut vc) = closed(x, v, a, k, hi);
            xc[1] = floor;
            rem -= hi;
            match self.floor_action {
                // Glue.
                2 => {
                    (p.x, p.v, p.glued) = (xc, [0.0; 3], true);
                    return;
                }
                // Ice.
                1 => {
                    vc[1] = 0.0;
                    a[1] = 0.0;
                    p.iced = true;
                    (x, v) = (xc, vc);
                    continue;
                }
                _ => {}
            }
            let m = p.bounces;
            let bounce = (at(BOUNCINESS) / 100.0 + at(RANDOM_BOUNCINESS) / 100.0 * (2.0 * self.r(p.k, 100 + 2 * m) - 1.0)).max(0.0);
            let turn = at(SPREAD) / 100.0 * PI * (2.0 * self.r(p.k, 101 + 2 * m) - 1.0);
            let (sn, cs) = (turn.sin(), turn.cos());
            vc = [vc[0] * cs - vc[2] * sn, -vc[1] * bounce, vc[0] * sn + vc[2] * cs];
            p.bounces += 1;
            hops += 1;
            (x, v) = (xc, vc);
            if hops == 4 {
                v[1] = 0.0;
                (p.x, p.v) = closed(x, v, [a[0], 0.0, a[2]], k, rem);
                return;
            }
        }
    }

    /// Particle k, born at b, as it is at u: from b to the first quarter frame after it, then
    /// quarter to quarter.
    // ponytail: every frame steps every alive particle from its birth, O(alive x steps); keep
    // the particles between frames if long lives at high rates grow slow.
    fn moved(&self, k: i64, b: f64) -> Particle {
        let mut p = self.born(k, b);
        let mut t0 = b;
        for j in (4.0 * b).floor() as i64 + 1..=4 * self.u {
            let t1 = j as f64 / 4.0;
            self.step(&mut p, t0, (t1 - t0) / self.fps);
            t0 = t1;
        }
        p
    }

    /// The alive particles, oldest first, with their age fractions.
    fn alive(&self) -> Vec<(Particle, f64)> {
        let u = self.u as f64;
        self.births()
            .into_par_iter()
            .filter_map(|(k, b)| {
                let life = self.at(LONGEVITY, b) * self.fps;
                let age = u - b;
                (age < life).then(|| (self.moved(k, b), age / life))
            })
            .collect()
    }
}

fn fade(t: f64) -> f64 {
    t * t * t * (t * (6.0 * t - 15.0) + 10.0)
}

/// Perlin's gradient noise, channel `c`, at `q`.
fn noise(seed: u64, c: u64, q: V) -> f64 {
    let i = q.map(f64::floor);
    let f = [q[0] - i[0], q[1] - i[1], q[2] - i[2]];
    let w = f.map(fade);
    let mut total = 0.0;
    for dz in 0..2 {
        for dy in 0..2 {
            for dx in 0..2 {
                let (ci, cj, cl) = (i[0] as i64 + dx, i[1] as i64 + dy, i[2] as i64 + dz);
                let g = sphere(crate::grade::unit(seed, ci, cj, cl, 200 + 2 * c), PI * crate::grade::unit(seed, ci, cj, cl, 201 + 2 * c));
                let wt = (if dx == 1 { w[0] } else { 1.0 - w[0] }) * (if dy == 1 { w[1] } else { 1.0 - w[1] }) * (if dz == 1 { w[2] } else { 1.0 - w[2] });
                total += wt * dot(g, [f[0] - dx as f64, f[1] - dy as f64, f[2] - dz as f64]);
            }
        }
    }
    total
}

fn curl(seed: u64, q: V) -> V {
    let e = 1e-3;
    let d = |c: u64, axis: usize| {
        let (mut hi, mut lo) = (q, q);
        hi[axis] += e;
        lo[axis] -= e;
        (noise(seed, c, hi) - noise(seed, c, lo)) / (2.0 * e)
    };
    [d(2, 1) - d(1, 2), d(0, 2) - d(2, 0), d(1, 0) - d(0, 1)]
}

/// The pull `a` and resistance `k` over `h` seconds, in closed form.
fn closed(x: V, v: V, a: V, k: f64, h: f64) -> (V, V) {
    if k > 0.0 {
        let e = (-k * h).exp();
        let m = -(-k * h).exp_m1() / k;
        ([0, 1, 2].map(|i| x[i] + a[i] / k * h + (v[i] - a[i] / k) * m), [0, 1, 2].map(|i| a[i] / k + (v[i] - a[i] / k) * e))
    } else {
        ([0, 1, 2].map(|i| x[i] + v[i] * h + a[i] * h * h / 2.0), [0, 1, 2].map(|i| v[i] + a[i] * h))
    }
}

fn run_of<'a>(e: &'a Effect) -> Option<(Run<'a>, &'a [String; 7], &'a [String; 8], &'a [f64; 4])> {
    let Effect::ParticleSpace { values, words, colors, clock, view, history } = e else {
        unreachable!("Particle Space's particles of another effect")
    };
    let u = clock[0] as i64;
    if u < 0 {
        return None;
    }
    let run = Run {
        now: values,
        samples: history.as_deref().map(|h| h.as_slice()),
        u,
        fps: clock[1],
        seed: crate::grade::mix(values[SEED].floor() as u64),
        mode: word(words, 0),
        floor_action: word(words, 1),
    };
    Some((run, words, colors, view))
}

/// The engine's own numbers for B-490's test: every alive particle at the frame, oldest first,
/// as [k, b, x, y, z, vx, vy, vz]. `e`'s clock and history already filled as compose fills them.
#[doc(hidden)]
pub fn alive(e: &Effect) -> Vec<[f64; 8]> {
    let Some((run, ..)) = run_of(e) else { return Vec::new() };
    run.alive().into_iter().map(|(p, _)| [p.k as f64, p.b, p.x[0], p.x[1], p.x[2], p.v[0], p.v[1], p.v[2]]).collect()
}

/// D-610: the sprite list the processor and the card both draw (see [`HEAD`]): each sprite its
/// kind (0 line, 1 faded sphere, 2 shaded sphere, 3 disc, 4 bubble), its centre (X, Y) in the
/// drawing's own pixels, a line's other end (X2, Y2) and width or the others' radius, its
/// opacity and its linear colour. `source` is the drawing as it reaches the effect, its corner at
/// `(ox, oy)`. Before the in point, none and the drawing kept.
pub(crate) fn sprite_numbers(source: &WorkingBuffer, e: &Effect, (ox, oy): (usize, usize)) -> Vec<f64> {
    let Some((run, words, colors, view)) = run_of(e) else {
        return vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
    };
    let now = run.now;
    let (w0, h0) = ((source.width() - 2 * ox) as f64, (source.height() - 2 * oy) as f64);
    // The camera: a function from a world point to (X, Y, pixels a unit, depth).
    let m = {
        type M3 = [[f64; 3]; 3];
        let mat = |a: M3, b: M3| -> M3 { std::array::from_fn(|i| std::array::from_fn(|j| (0..3).map(|k| a[i][k] * b[k][j]).sum())) };
        let (sx, cx) = (now[ROTATION].to_radians().sin(), now[ROTATION].to_radians().cos());
        let (sy, cy) = (now[ROTATION + 1].to_radians().sin(), now[ROTATION + 1].to_radians().cos());
        let (sz, cz) = (now[ROTATION + 2].to_radians().sin(), now[ROTATION + 2].to_radians().cos());
        mat([[1.0, 0.0, 0.0], [0.0, cx, -sx], [0.0, sx, cx]], mat([[cy, 0.0, sy], [0.0, 1.0, 0.0], [-sy, 0.0, cy]], [[cz, -sz, 0.0], [sz, cz, 0.0], [0.0, 0.0, 1.0]]))
    };
    let f = (w0 / 2.0) / (now[FOV].to_radians() / 2.0).tan();
    let dist = now[DISTANCE];
    let see = |x: V| -> Option<[f64; 4]> {
        if view[0] != 0.0 {
            let (ax, ay, al) = (view[1], view[2], view[3]);
            let zp = x[2] * w0;
            let depth = (al + zp) / w0;
            if al <= 0.0 || depth <= 0.01 {
                return None;
            }
            let ratio = al / (al + zp);
            return Some([ax + (w0 / 2.0 + x[0] * w0 - ax) * ratio, ay + (h0 / 2.0 + x[1] * w0 - ay) * ratio, w0 * ratio, depth]);
        }
        let c = [0, 1, 2].map(|i| m[i][0] * x[0] + m[i][1] * x[1] + m[i][2] * x[2]);
        let cz = c[2] + dist;
        (cz > 0.01).then(|| [w0 / 2.0 + f * c[0] / cz, h0 / 2.0 + f * c[1] / cz, f / cz, cz])
    };
    let linear = |c: &str| crate::effects::encoded(c).map(crate::grade::to_linear);
    let origin = |x0: V| -> V {
        let (gx, gy) = ((w0 / 2.0 + x0[0] * w0).floor(), (h0 / 2.0 + x0[1] * w0).floor());
        if !(0.0 <= gx && gx < w0 && 0.0 <= gy && gy < h0) {
            return [0.0; 3];
        }
        let p = source.pixel(gx as usize + ox, gy as usize + oy).map(f64::from);
        if p[3] > 0.0 { [p[0] / p[3], p[1] / p[3], p[2] / p[3]] } else { [0.0; 3] }
    };
    let mix3 = |a: V, b: V, t: f64| [0, 1, 2].map(|i| a[i] + (b[i] - a[i]) * t);
    let (birth, death, cue_color) = (linear(&colors[0]), linear(&colors[1]), linear(&colors[7]));
    let stops: Vec<V> = (2..7).map(|i| linear(&colors[i])).collect();
    let (kind, cmap, cue) = (word(words, 2), word(words, 3), word(words, 5));
    let colour = |f: f64, org: V| match cmap {
        0 => mix3(birth, death, f),
        1 => mix3(org, death, f),
        2 => mix3(birth, org, f),
        3 => org,
        _ => {
            let i = ((f * 4.0).floor() as usize).min(3);
            mix3(stops[i], stops[i + 1], f * 4.0 - i as f64)
        }
    };
    let mut drawn: Vec<(f64, i64, [f64; 10])> = run
        .alive()
        .into_iter()
        .filter_map(|(p, f)| {
            let (bs, ds) = p.sizes;
            let size = (bs + (ds - bs) * f) * p.size_factor;
            let [x, y, ppu, depth] = see(p.x)?;
            let mut rad = size / 100.0 * ppu / 2.0;
            let mut col = colour(f, origin(p.x0));
            let mut alpha = now[OPACITY] / 100.0;
            if cue != 0 {
                let fd = (1.0 - depth / now[CUE_DISTANCE]).clamp(0.0, 1.0);
                if cue == 1 {
                    alpha *= fd;
                } else {
                    col = mix3(col, cue_color, 1.0 - fd);
                }
            }
            let sprite = if kind == 0 {
                let tail = see(sub(p.x, mul(p.v, size / 0.15 / run.fps)))?;
                [0.0, x, y, tail[0], tail[1], (2.0 * rad).max(1.0), alpha, col[0], col[1], col[2]]
            } else {
                if rad < 0.7 {
                    alpha *= (rad / 0.7) * (rad / 0.7);
                    rad = 0.7;
                }
                [kind as f64, x, y, rad, 0.0, 0.0, alpha, col[0], col[1], col[2]]
            };
            Some((depth, p.k, sprite))
        })
        .collect();
    drawn.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    let (d, h) = (now[LIGHT_DIRECTION].to_radians(), now[LIGHT_HEIGHT].to_radians());
    let mut k = vec![
        now[VOLUME_SHADE] / 100.0,
        h.cos() * d.sin(),
        -h.cos() * d.cos(),
        -h.sin(),
        word(words, 4) as f64,
        if word(words, 6) == 1 { 1.0 } else { 0.0 },
        drawn.len() as f64,
    ];
    k.reserve(10 * drawn.len());
    for (_, _, s) in drawn {
        k.extend(s);
    }
    k
}

/// D-610: the sprites of `k` that may touch each 16 by 16 tile of a buffer `w` by `h`, the
/// drawing's corner at `(ox, oy)`, farthest first: each tile's start in the list (and the end),
/// then the list.
pub(crate) fn tile_lists(k: &[f64], (ox, oy): (usize, usize), w: usize, h: usize) -> (Vec<usize>, Vec<usize>) {
    let (tw, th) = (w.div_ceil(TILE), h.div_ceil(TILE));
    let mut lists = vec![Vec::new(); tw * th];
    for (n, s) in k[HEAD..].chunks_exact(10).enumerate() {
        let (xs, ys) = if s[0] == 0.0 {
            let r = s[5] / 2.0 + 1.0;
            ((s[1].min(s[3]) - r, s[1].max(s[3]) + r), (s[2].min(s[4]) - r, s[2].max(s[4]) + r))
        } else {
            let r = s[3] + 1.0;
            ((s[1] - r, s[1] + r), (s[2] - r, s[2] + r))
        };
        let span = |lo: f64, hi: f64, o: usize, size: usize| {
            let a = (lo + o as f64).floor().clamp(0.0, size as f64) as usize;
            let b = ((hi + o as f64).ceil() + 1.0).clamp(0.0, size as f64) as usize;
            (a < b).then(|| a / TILE..=(b - 1) / TILE)
        };
        let (Some(cols), Some(rows)) = (span(xs.0, xs.1, ox, w), span(ys.0, ys.1, oy, h)) else { continue };
        for ty in rows {
            for tx in cols.clone() {
                lists[ty * tw + tx].push(n);
            }
        }
    }
    let mut starts = Vec::with_capacity(tw * th + 1);
    let mut all = Vec::new();
    for l in lists {
        starts.push(all.len());
        all.extend(l);
    }
    starts.push(all.len());
    (starts, all)
}

/// D-610: sprite `n` of `k` at the own pixel centre (x, y), premultiplied, or `None`.
fn cover(k: &[f64], n: usize, x: f64, y: f64) -> Option<[f64; 4]> {
    let s = &k[HEAD + 10 * n..][..10];
    let (cv, shade) = if s[0] == 0.0 {
        let (ax, ay, dx, dy) = (s[1], s[2], s[3] - s[1], s[4] - s[2]);
        let ll = dx * dx + dy * dy;
        let t = if ll > 0.0 { (((x - ax) * dx + (y - ay) * dy) / ll).clamp(0.0, 1.0) } else { 0.0 };
        let e = (x - ax - t * dx).hypot(y - ay - t * dy);
        ((s[5] / 2.0 - e + 0.5).clamp(0.0, 1.0), 1.0)
    } else {
        let rad = s[3];
        let e = (x - s[1]).hypot(y - s[2]);
        let cv = if s[0] == 1.0 {
            let t = (1.0 - (e / rad) * (e / rad)).max(0.0);
            t * t
        } else {
            let c = (rad - e + 0.5).clamp(0.0, 1.0);
            if s[0] == 4.0 { c * (0.25 + 0.75 * (e / rad).min(1.0).powi(4)) } else { c }
        };
        let v = if s[0] == 2.0 || s[0] == 4.0 { 1.0 } else { k[0] };
        let mut shade = 1.0;
        if v > 0.0 {
            let (dx, dy) = ((x - s[1]) / rad, (y - s[2]) / rad);
            let q = dx * dx + dy * dy;
            let nv = if q < 1.0 { [dx, dy, -(1.0 - q).sqrt()] } else { [dx / q.sqrt(), dy / q.sqrt(), 0.0] };
            shade = 1.0 - v + v * (0.2 + 0.8 * (nv[0] * k[1] + nv[1] * k[2] + nv[2] * k[3]).max(0.0));
        }
        (cv, shade)
    };
    if cv <= 0.0 {
        return None;
    }
    let a = s[6] * cv;
    Some([s[7] * shade * a, s[8] * shade * a, s[9] * shade * a, a])
}

/// D-610: the sprites `ids` of `k` laid over `d` at the own pixel centre (x, y), by the
/// transfer mode.
fn pixel(k: &[f64], ids: &[usize], x: f64, y: f64, mut d: [f64; 4]) -> [f64; 4] {
    let mode = k[4];
    for &n in ids {
        let Some(s) = cover(k, n, x, y) else { continue };
        d = if mode == 0.0 {
            [0, 1, 2, 3].map(|i| s[i] + (1.0 - s[3]) * d[i])
        } else if mode == 1.0 {
            [0, 1, 2, 3].map(|i| s[i] + d[i] - s[i] * d[i])
        } else {
            [s[0] + d[0], s[1] + d[1], s[2] + d[2], (s[3] + d[3]).min(1.0)]
        };
    }
    d
}

/// D-610: Particle Space, after the purpose of CycoreFX's CC Particle World (document 21): the
/// particles drawn over a clear layer, or over the drawing with Composite With Original on,
/// farthest first. `(ox, oy)` the drawing's corner in the buffer. Never grows.
pub(crate) fn particle_space(source: &mut WorkingBuffer, e: &Effect, (ox, oy): (usize, usize)) {
    let k = sprite_numbers(source, e, (ox, oy));
    if k[5] == 1.0 && k[6] == 0.0 {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let (starts, list) = tile_lists(&k, (ox, oy), w, h);
    let (tw, keep) = (w.div_ceil(TILE), k[5] == 1.0);
    source.data_mut().par_chunks_mut(w * 4 * TILE).enumerate().for_each(|(ty, band)| {
        for (row, line) in band.chunks_exact_mut(w * 4).enumerate() {
            let y = (ty * TILE + row) as f64 - oy as f64 + 0.5;
            for (bx, px) in line.chunks_exact_mut(4).enumerate() {
                let t = ty * tw + bx / TILE;
                let d = if keep { [px[0], px[1], px[2], px[3]].map(f64::from) } else { [0.0; 4] };
                let out = pixel(&k, &list[starts[t]..starts[t + 1]], bx as f64 - ox as f64 + 0.5, y, d);
                for (v, o) in px.iter_mut().zip(out) {
                    *v = o as f32;
                }
            }
        }
    });
}
