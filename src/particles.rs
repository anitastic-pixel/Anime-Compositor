//! D-610: the particle engine (P0-6) and its first effect, Particle Space
//! (`core.particle_space`), modelled on what CycoreFX's CC Particle World is for. Every rule is
//! ours, written after what each control is for (document 21); `tools/particle_space_reference.py`
//! is the reference, and this file follows it step for step.
//!
//! No state is carried from frame to frame (document 20): a particle at a frame is a pure
//! function of the seed, its number and the settings history, so a frame drawn alone equals the
//! same frame played from the start.
//!
//! D-612: Particle Plane (`core.particle_plane`), the same engine flat, after what CycoreFX's CC
//! Particle Systems II is for: its settings worked as Particle Space's ([`plane_row`]), born,
//! thrown and swirled in the layer's plane (`Run::flat`), drawn straight onto the layer, with an
//! Opacity Map and a Star of its own; `tools/particle_plane_reference.py` is its reference.
//!
//! D-615: Particle Emitter (`core.particle_emitter`), the same engine in 3D, after what Trapcode
//! Particular is for: its settings worked as Particle Space's ([`emitter_row`]), born from its
//! own point, box, ball or grid and thrown its own ways ([`Emitter`]), each particle with its own
//! life, size, opacity and colour, drawn as two new sprites (soft and glowing spheres, kinds 9
//! and 10) or as the star, the faded sphere or the line; `tools/particle_emitter_reference.py` is
//! its reference.
//!
//! D-616: Particle Grid (`core.particle_grid`), after what Trapcode Form is for: particles that
//! stay put, in a grid of points or on layered spheres in 3D, scattered, twisted and rippled by
//! the engine's noise, coloured, sized and pushed by other layers ([`grid_points`]), drawn with
//! the Emitter's camera and sprites; `tools/particle_grid_reference.py` is its reference.

use crate::effects::{
    Effect, PARTICLE_EMITTER_SETTINGS, PARTICLE_EMITTER_WORDS, PARTICLE_GRID_SETTINGS,
    PARTICLE_GRID_WORDS, PARTICLE_PLANE_WORDS, PARTICLE_SPACE_WORDS,
};
use crate::WorkingBuffer;
use rayon::prelude::*;
use std::borrow::Cow;
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
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
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
const ROTATION_SPEED: usize = 44;
const INITIAL_ROTATION: usize = 45;

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
/// Composite With Original on, the count; then [`STRIDE`] numbers a sprite, farthest first.
pub(crate) const HEAD: usize = 7;
/// D-611: the numbers a sprite.
pub(crate) const STRIDE: usize = 11;
/// The tiles the sprites are binned into, as the card's workgroups.
pub(crate) const TILE: usize = 16;

/// D-612: the Star's kind in the sprite list.
const STAR_KIND: usize = 8;
/// D-612: the star's outline, point j 36 j degrees clockwise from up, the outer five 1 from the
/// centre and the inner five 0.4: the reference's table, the rule itself.
const STAR: [[f64; 2]; 10] = [
    [0.0, -1.0],
    [0.23511410091698925, -0.323606797749979],
    [0.9510565162951535, -0.30901699437494745],
    [0.3804226065180615, 0.12360679774997894],
    [0.5877852522924732, 0.8090169943749473],
    [4.898587196589413e-17, 0.4],
    [-0.587785252292473, 0.8090169943749475],
    [-0.3804226065180614, 0.12360679774997903],
    [-0.9510565162951536, -0.30901699437494723],
    [-0.23511410091698937, -0.32360679774997897],
];

/// The settings' history for one frame, as the reference's `Run`. D-612: `flat`, Particle
/// Plane's, everything in the layer's plane; its settings are worked into Particle Space's, so
/// the rows are owned or borrowed.
struct Run<'a> {
    now: [f64; 46],
    samples: Option<Cow<'a, [[f64; 46]]>>,
    u: i64,
    fps: f64,
    seed: u64,
    mode: usize,
    floor_action: usize,
    flat: bool,
}

/// D-612: how the sprites look, from either effect's words and colours: the kind, Color Map,
/// transfer mode and Depth Cue as indices, Composite With Original, the eight colours in Particle
/// Space's order, Particle Plane's Opacity Map with its five stops / 100, and the composition
/// camera compose found.
struct Look<'a> {
    kind: usize,
    cmap: usize,
    transfer: usize,
    cue: usize,
    keep: bool,
    colors: [&'a str; 8],
    fade: Option<(usize, [f64; 5])>,
    view: [f64; 4],
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
    PARTICLE_SPACE_WORDS[i]
        .1
        .iter()
        .position(|w| *w == words[i])
        .unwrap_or(0)
}

impl Run<'_> {
    /// The settings sample `i` frames after the in point, held at the last.
    fn sample(&self, i: usize, n: usize) -> f64 {
        self.samples
            .as_deref()
            .map_or(self.now[n], |s| s[i.min(s.len() - 1)][n])
    }

    fn at(&self, n: usize, t: f64) -> f64 {
        let Some(s) = self.samples.as_deref() else {
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
        let q: Vec<f64> = (0..n)
            .map(|i| 100.0 * self.sample(i, RATE) / self.fps)
            .collect();
        let lmax = (0..n)
            .map(|i| self.sample(i, LONGEVITY))
            .fold(f64::MIN, f64::max)
            * self.fps;
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
                    let x = if den > 0.0 {
                        (2.0 * d / den).min(1.0)
                    } else {
                        1.0
                    };
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
            return if g.iter().any(|c| *c != 0.0) {
                mul(unit_or(g, [0.0, -1.0, 0.0]), -1.0)
            } else {
                [0.0, -1.0, 0.0]
            };
        }
        unit_or(self.vec(AXIS, t), [0.0, -1.0, 0.0])
    }

    /// The particle's start: place, velocity and its fixed numbers.
    fn born(&self, k: i64, b: f64) -> Particle {
        let r = |s| self.r(k, s);
        let p = self.vec(PRODUCER, b);
        let rad = self.vec(RADIUS, b);
        let x = if self.flat {
            // D-612: on the ellipse's disc in the plane.
            let (rho, turn) = (r(0).sqrt(), 2.0 * PI * r(2));
            [
                p[0] + rad[0] * rho * turn.cos(),
                p[1] + rad[1] * rho * turn.sin(),
                0.0,
            ]
        } else {
            let rho = r(0).powf(1.0 / 3.0);
            let d = sphere(1.0 - 2.0 * r(1), 2.0 * PI * r(2));
            [0, 1, 2].map(|i| p[i] + rad[i] * rho * d[i])
        };
        let a = self.axes(b);
        let h = if a[0].abs() >= 0.9 {
            [0.0, 1.0, 0.0]
        } else {
            [1.0, 0.0, 0.0]
        };
        let e1 = unit_or(cross(h, a), [1.0, 0.0, 0.0]);
        let e2 = cross(a, e1);
        let phi = 2.0 * PI * r(4);
        let cap = |cos_t: f64| {
            let sin_t = (1.0 - cos_t * cos_t).max(0.0).sqrt();
            add(
                mul(a, cos_t),
                mul(add(mul(e1, phi.cos()), mul(e2, phi.sin())), sin_t),
            )
        };
        let alpha = (self.at(EXTRA_ANGLE, b) / 2.0).to_radians();
        // D-612: flat, turned in the plane from the axis towards a right angle clockwise of it.
        let side = if r(4) < 0.5 { 1.0 } else { -1.0 };
        let right = [-a[1], a[0], 0.0];
        let turned = |t: f64| add(mul(a, t.cos()), mul(right, t.sin()));
        let way = match (self.flat, self.mode) {
            (true, DIRECTION_AXIS | FRACTAL_UNI) => turned(alpha * (2.0 * r(3) - 1.0)),
            (true, CONE_AXIS) => turned(side * alpha),
            (true, FIRE) => turned(20f64.to_radians() * (2.0 * r(3) - 1.0)),
            (true, JET_SIDEWAYS) => mul(right, side),
            (true, _) => [(2.0 * PI * r(3)).cos(), (2.0 * PI * r(3)).sin(), 0.0],
            (false, DIRECTION_AXIS | FRACTAL_UNI) => cap(1.0 - r(3) * (1.0 - alpha.cos())),
            (false, CONE_AXIS) => cap(alpha.cos()),
            (false, FIRE) => cap(1.0 - r(3) * (1.0 - 20f64.to_radians().cos())),
            (false, JET_SIDEWAYS) => add(mul(e1, phi.cos()), mul(e2, phi.sin())),
            (false, _) => sphere(1.0 - 2.0 * r(3), phi),
        };
        let speed = self.at(VELOCITY, b) * (1.0 + self.at(EXTRA, b) * (r(5) - 0.5));
        let mut v = mul(way, speed);
        let iv = self.at(INHERIT, b);
        if iv != 0.0 {
            let pv = mul(
                sub(self.vec(PRODUCER, b + 0.125), self.vec(PRODUCER, b - 0.125)),
                self.fps / 0.25,
            );
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
            // D-612: flat, the swirls turn about the plane's own axis.
            let a = if self.flat && self.mode != FIRE {
                [0.0, 0.0, 1.0]
            } else {
                self.axes(t0)
            };
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
            let q = [
                x[0] / sc + evo * 0.31,
                x[1] / sc + evo * 0.57,
                x[2] / sc + evo * 0.77,
            ];
            push = add(push, mul(curl(self.seed, q, self.flat), strength));
        }
        if p.iced {
            push[1] = 0.0;
        }
        v = add(v, mul(push, h));
        let g = self.vec(GRAVITY_VECTOR, t0);
        let gu = if g.iter().any(|c| *c != 0.0) {
            unit_or(g, [0.0; 3])
        } else {
            [0.0; 3]
        };
        let mut a = add(
            mul(
                gu,
                if self.mode == FIRE {
                    -at(GRAVITY)
                } else {
                    at(GRAVITY)
                },
            ),
            self.vec(WIND, t0),
        );
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
            let bounce = (at(BOUNCINESS) / 100.0
                + at(RANDOM_BOUNCINESS) / 100.0 * (2.0 * self.r(p.k, 100 + 2 * m) - 1.0))
                .max(0.0);
            let turn = at(SPREAD) / 100.0 * PI * (2.0 * self.r(p.k, 101 + 2 * m) - 1.0);
            let (sn, cs) = (turn.sin(), turn.cos());
            vc = [
                vc[0] * cs - vc[2] * sn,
                -vc[1] * bounce,
                vc[0] * sn + vc[2] * cs,
            ];
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
                let g = sphere(
                    crate::grade::unit(seed, ci, cj, cl, 200 + 2 * c),
                    PI * crate::grade::unit(seed, ci, cj, cl, 201 + 2 * c),
                );
                let wt = (if dx == 1 { w[0] } else { 1.0 - w[0] })
                    * (if dy == 1 { w[1] } else { 1.0 - w[1] })
                    * (if dz == 1 { w[2] } else { 1.0 - w[2] });
                total += wt * dot(g, [f[0] - dx as f64, f[1] - dy as f64, f[2] - dz as f64]);
            }
        }
    }
    total
}

/// The noise field's curl at `q`; D-612: flat, channel 2's gradient turned in the plane.
fn curl(seed: u64, q: V, flat: bool) -> V {
    let e = 1e-3;
    let d = |c: u64, axis: usize| {
        let (mut hi, mut lo) = (q, q);
        hi[axis] += e;
        lo[axis] -= e;
        (noise(seed, c, hi) - noise(seed, c, lo)) / (2.0 * e)
    };
    if flat {
        return [d(2, 1), -d(2, 0), 0.0];
    }
    [d(2, 1) - d(1, 2), d(0, 2) - d(2, 0), d(1, 0) - d(0, 1)]
}

/// The pull `a` and resistance `k` over `h` seconds, in closed form.
fn closed(x: V, v: V, a: V, k: f64, h: f64) -> (V, V) {
    if k > 0.0 {
        let e = (-k * h).exp();
        let m = -(-k * h).exp_m1() / k;
        (
            [0, 1, 2].map(|i| x[i] + a[i] / k * h + (v[i] - a[i] / k) * m),
            [0, 1, 2].map(|i| a[i] / k + (v[i] - a[i] / k) * e),
        )
    } else {
        (
            [0, 1, 2].map(|i| x[i] + v[i] * h + a[i] * h * h / 2.0),
            [0, 1, 2].map(|i| v[i] + a[i] * h),
        )
    }
}

/// D-612: Particle Plane's settings (`effects::PARTICLE_PLANE_SETTINGS`) as Particle Space's: its
/// own copied across, Direction as the axis (sin d, -cos d, 0), gravity straight down, no wind,
/// turbulence, floor or Volume Shade, the light from 45 degrees at 45.
pub(crate) fn plane_row(v: &[f64; 25]) -> [f64; 46] {
    let mut e = [0.0; 46];
    e[TURBULENCE_SCALE] = 0.25;
    e[LIGHT_DIRECTION] = 45.0;
    e[LIGHT_HEIGHT] = 45.0;
    e[GRAVITY_VECTOR + 1] = 1.0;
    let d = v[10].to_radians();
    (e[AXIS], e[AXIS + 1]) = (d.sin(), -d.cos());
    for (to, from) in [
        (RATE, 0),
        (LONGEVITY, 1),
        (PRODUCER, 2),
        (PRODUCER + 1, 3),
        (RADIUS, 4),
        (RADIUS + 1, 5),
        (VELOCITY, 6),
        (INHERIT, 7),
        (GRAVITY, 8),
        (RESISTANCE, 9),
        (EXTRA, 11),
        (EXTRA_ANGLE, 12),
        (BIRTH_SIZE, 13),
        (DEATH_SIZE, 14),
        (SIZE_VARIATION, 15),
        (OPACITY, 16),
        (SEED, 22),
        (ROTATION_SPEED, 23),
        (INITIAL_ROTATION, 24),
    ] {
        e[to] = v[from];
    }
    e
}

fn run_of(e: &Effect) -> Option<(Run<'_>, Look<'_>)> {
    match e {
        Effect::ParticleSpace {
            values,
            words,
            colors,
            clock,
            view,
            history,
            ..
        } => {
            let u = clock[0] as i64;
            if u < 0 {
                return None;
            }
            let w = |i| word(words, i);
            let run = Run {
                now: *values,
                samples: history.as_deref().map(|h| Cow::Borrowed(h.as_slice())),
                u,
                fps: clock[1],
                seed: crate::grade::mix(values[SEED].floor() as u64),
                mode: w(0),
                floor_action: w(1),
                flat: false,
            };
            let look = Look {
                kind: w(2),
                cmap: w(3),
                transfer: w(4),
                cue: w(5),
                keep: w(6) == 1,
                colors: std::array::from_fn(|i| colors[i].as_str()),
                fade: None,
                view: *view,
            };
            Some((run, look))
        }
        Effect::ParticlePlane {
            values,
            words,
            colors,
            clock,
            history,
            ..
        } => {
            let u = clock[0] as i64;
            if u < 0 {
                return None;
            }
            let w = |i: usize| {
                PARTICLE_PLANE_WORDS[i]
                    .1
                    .iter()
                    .position(|x| *x == words[i])
                    .unwrap_or(0)
            };
            let now = plane_row(values);
            let run = Run {
                now,
                samples: history
                    .as_deref()
                    .map(|h| Cow::Owned(h.iter().map(plane_row).collect())),
                u,
                fps: clock[1],
                seed: crate::grade::mix(now[SEED].floor() as u64),
                mode: w(0),
                floor_action: 0,
                flat: true,
            };
            let look = Look {
                // D-710: the textured disc and faded disc, Particle Space's 6 and 7; D-714: the
                // textured square, its 5.
                kind: [0, 1, 2, 3, 4, STAR_KIND, 6, 7, 5][w(1)],
                cmap: w(3),
                transfer: w(4),
                cue: 0,
                keep: w(5) == 1,
                colors: std::array::from_fn(|i| colors.get(i).map_or("#000000", |c| c.as_str())),
                fade: Some((w(2), std::array::from_fn(|i| values[17 + i] / 100.0))),
                view: [0.0; 4],
            };
            Some((run, look))
        }
        _ => unreachable!("particles of another effect"),
    }
}

/// D-612: Opacity Map `m` at age fraction `f`, `o` its Custom stops.
fn opacity_map(m: usize, o: [f64; 5], f: f64) -> f64 {
    match m {
        0 => 1.0,
        1 => f,
        2 => 1.0 - f,
        3 => 1.0 - (2.0 * f - 1.0).abs(),
        _ => {
            let i = ((f * 4.0).floor() as usize).min(3);
            o[i] + (o[i + 1] - o[i]) * (f * 4.0 - i as f64)
        }
    }
}

/// The engine's own numbers for B-490's test: every alive particle at the frame, oldest first,
/// as [k, b, x, y, z, vx, vy, vz]. `e`'s clock and history already filled as compose fills them.
#[doc(hidden)]
pub fn alive(e: &Effect) -> Vec<[f64; 8]> {
    let Some((run, ..)) = run_of(e) else {
        return Vec::new();
    };
    run.alive()
        .into_iter()
        .map(|(p, _)| {
            [
                p.k as f64, p.b, p.x[0], p.x[1], p.x[2], p.v[0], p.v[1], p.v[2],
            ]
        })
        .collect()
}

/// D-610: the sprite list the processor and the card both draw (see [`HEAD`]): each sprite its
/// kind (0 line, 1 faded sphere, 2 shaded sphere, 3 disc, 4 bubble), its centre (X, Y) in the
/// drawing's own pixels, a line's other end (X2, Y2) and width or the others' radius, its
/// opacity and its linear colour, then 0. D-611's textured kinds (5 square, 6 disc, 7 faded
/// disc): kind, X, Y, radius, the turn's cosine and sine, opacity, the colour (Depth Cue's colour
/// when a texture is read, else the Color Map's, fogged) and Depth Cue Fog's share fd. With a
/// layer named whose picture is empty, the textured kinds draw nothing. `source` is the drawing
/// as it reaches the effect, its corner at `(ox, oy)`. Before the in point, none and the drawing
/// kept.
pub(crate) fn sprite_numbers(
    source: &WorkingBuffer,
    e: &Effect,
    (ox, oy): (usize, usize),
) -> Vec<f64> {
    // D-615: Particle Emitter's own.
    if let Effect::ParticleEmitter { .. } = e {
        return emitter_numbers(source, e, (ox, oy));
    }
    // D-616: Particle Grid's own.
    if let Effect::ParticleGrid { .. } = e {
        return grid_numbers(source, e, (ox, oy));
    }
    let Some((run, look)) = run_of(e) else {
        return vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
    };
    let view = look.view;
    let now = run.now;
    let (w0, h0) = (
        (source.width() - 2 * ox) as f64,
        (source.height() - 2 * oy) as f64,
    );
    // The camera: a function from a world point to (X, Y, pixels a unit, depth).
    let m = {
        type M3 = [[f64; 3]; 3];
        let mat = |a: M3, b: M3| -> M3 {
            std::array::from_fn(|i| {
                std::array::from_fn(|j| (0..3).map(|k| a[i][k] * b[k][j]).sum())
            })
        };
        let (sx, cx) = (
            now[ROTATION].to_radians().sin(),
            now[ROTATION].to_radians().cos(),
        );
        let (sy, cy) = (
            now[ROTATION + 1].to_radians().sin(),
            now[ROTATION + 1].to_radians().cos(),
        );
        let (sz, cz) = (
            now[ROTATION + 2].to_radians().sin(),
            now[ROTATION + 2].to_radians().cos(),
        );
        mat(
            [[1.0, 0.0, 0.0], [0.0, cx, -sx], [0.0, sx, cx]],
            mat(
                [[cy, 0.0, sy], [0.0, 1.0, 0.0], [-sy, 0.0, cy]],
                [[cz, -sz, 0.0], [sz, cz, 0.0], [0.0, 0.0, 1.0]],
            ),
        )
    };
    let f = (w0 / 2.0) / (now[FOV].to_radians() / 2.0).tan();
    let dist = now[DISTANCE];
    let see = |x: V| -> Option<[f64; 4]> {
        // D-612: flat, straight onto the layer, every particle at depth 1.
        if run.flat {
            return Some([w0 / 2.0 + x[0] * w0, h0 / 2.0 + x[1] * w0, w0, 1.0]);
        }
        if view[0] != 0.0 {
            let (ax, ay, al) = (view[1], view[2], view[3]);
            let zp = x[2] * w0;
            let depth = (al + zp) / w0;
            if al <= 0.0 || depth <= 0.01 {
                return None;
            }
            let ratio = al / (al + zp);
            return Some([
                ax + (w0 / 2.0 + x[0] * w0 - ax) * ratio,
                ay + (h0 / 2.0 + x[1] * w0 - ay) * ratio,
                w0 * ratio,
                depth,
            ]);
        }
        let c = [0, 1, 2].map(|i| m[i][0] * x[0] + m[i][1] * x[1] + m[i][2] * x[2]);
        let cz = c[2] + dist;
        (cz > 0.01).then(|| {
            [
                w0 / 2.0 + f * c[0] / cz,
                h0 / 2.0 + f * c[1] / cz,
                f / cz,
                cz,
            ]
        })
    };
    let linear = |c: &str| crate::effects::encoded(c).map(crate::grade::to_linear);
    let origin = |x0: V| -> V {
        let (gx, gy) = (
            (w0 / 2.0 + x0[0] * w0).floor(),
            (h0 / 2.0 + x0[1] * w0).floor(),
        );
        if !(0.0 <= gx && gx < w0 && 0.0 <= gy && gy < h0) {
            return [0.0; 3];
        }
        let p = source
            .pixel(gx as usize + ox, gy as usize + oy)
            .map(f64::from);
        if p[3] > 0.0 {
            [p[0] / p[3], p[1] / p[3], p[2] / p[3]]
        } else {
            [0.0; 3]
        }
    };
    let mix3 = |a: V, b: V, t: f64| [0, 1, 2].map(|i| a[i] + (b[i] - a[i]) * t);
    let colors = look.colors;
    let (birth, death, cue_color) = (linear(colors[0]), linear(colors[1]), linear(colors[7]));
    let stops: Vec<V> = (2..7).map(|i| linear(colors[i])).collect();
    let (kind, cmap, cue) = (look.kind, look.cmap, look.cue);
    let (transfer, keep) = (look.transfer as f64, if look.keep { 1.0 } else { 0.0 });
    let (textured, read) = ((5..=7).contains(&kind), texture(e).is_some());
    if textured && !read && texture_named(e) {
        return vec![0.0, 0.0, 0.0, 0.0, transfer, keep, 0.0];
    }
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
    let mut drawn: Vec<(f64, i64, [f64; STRIDE])> = run
        .alive()
        .into_iter()
        .filter_map(|(p, f)| {
            let (bs, ds) = p.sizes;
            let size = (bs + (ds - bs) * f) * p.size_factor;
            let [x, y, ppu, depth] = see(p.x)?;
            let mut rad = size / 100.0 * ppu / 2.0;
            let mut col = colour(f, origin(p.x0));
            let mut alpha =
                now[OPACITY] / 100.0 * look.fade.map_or(1.0, |(m, o)| opacity_map(m, o, f));
            let mut fog = 1.0;
            if cue != 0 {
                let fd = (1.0 - depth / now[CUE_DISTANCE]).clamp(0.0, 1.0);
                if cue == 1 {
                    alpha *= fd;
                } else {
                    col = mix3(col, cue_color, 1.0 - fd);
                    fog = fd;
                }
            }
            let sprite = if kind == 0 {
                let tail = see(sub(p.x, mul(p.v, size / 0.15 / run.fps)))?;
                [
                    0.0,
                    x,
                    y,
                    tail[0],
                    tail[1],
                    (2.0 * rad).max(1.0),
                    alpha,
                    col[0],
                    col[1],
                    col[2],
                    0.0,
                ]
            } else {
                if rad < 0.7 {
                    alpha *= (rad / 0.7) * (rad / 0.7);
                    rad = 0.7;
                }
                if textured {
                    let age = (run.u as f64 - p.b) / run.fps;
                    let turn = (run.at(INITIAL_ROTATION, p.b) * run.r(p.k, 10)
                        + run.at(ROTATION_SPEED, p.b) * age)
                        .to_radians();
                    let c = if read { cue_color } else { col };
                    [
                        kind as f64,
                        x,
                        y,
                        rad,
                        turn.cos(),
                        turn.sin(),
                        alpha,
                        c[0],
                        c[1],
                        c[2],
                        fog,
                    ]
                } else {
                    [
                        kind as f64,
                        x,
                        y,
                        rad,
                        0.0,
                        0.0,
                        alpha,
                        col[0],
                        col[1],
                        col[2],
                        0.0,
                    ]
                }
            };
            Some((depth, p.k, sprite))
        })
        .collect();
    drawn.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    let (d, h) = (
        now[LIGHT_DIRECTION].to_radians(),
        now[LIGHT_HEIGHT].to_radians(),
    );
    let mut k = vec![
        now[VOLUME_SHADE] / 100.0,
        h.cos() * d.sin(),
        -h.cos() * d.cos(),
        -h.sin(),
        transfer,
        keep,
        drawn.len() as f64,
    ];
    k.reserve(STRIDE * drawn.len());
    for (_, _, s) in drawn {
        k.extend(s);
    }
    k
}

/// D-610: the sprites of `k` that may touch each 16 by 16 tile of a buffer `w` by `h`, the
/// drawing's corner at `(ox, oy)`, farthest first: each tile's start in the list (and the end),
/// then the list.
pub(crate) fn tile_lists(
    k: &[f64],
    (ox, oy): (usize, usize),
    w: usize,
    h: usize,
) -> (Vec<usize>, Vec<usize>) {
    let (tw, th) = (w.div_ceil(TILE), h.div_ceil(TILE));
    let mut lists = vec![Vec::new(); tw * th];
    for (n, s) in k[HEAD..].chunks_exact(STRIDE).enumerate() {
        let (xs, ys) = if s[0] == 0.0 {
            let r = s[5] / 2.0 + 1.0;
            (
                (s[1].min(s[3]) - r, s[1].max(s[3]) + r),
                (s[2].min(s[4]) - r, s[2].max(s[4]) + r),
            )
        } else {
            // D-611: a turned square reaches as far as its corners.
            // D-615: a soft sphere as far as half its feather beyond.
            let r =
                s[3] * if s[0] == 5.0 {
                    std::f64::consts::SQRT_2
                } else {
                    1.0
                } + 1.0
                    + if s[0] == 9.0 { s[4] / 2.0 } else { 0.0 };
            ((s[1] - r, s[1] + r), (s[2] - r, s[2] + r))
        };
        let span = |lo: f64, hi: f64, o: usize, size: usize| {
            let a = (lo + o as f64).floor().clamp(0.0, size as f64) as usize;
            let b = ((hi + o as f64).ceil() + 1.0).clamp(0.0, size as f64) as usize;
            (a < b).then(|| a / TILE..=(b - 1) / TILE)
        };
        let (Some(cols), Some(rows)) = (span(xs.0, xs.1, ox, w), span(ys.0, ys.1, oy, h)) else {
            continue;
        };
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

/// D-610: sprite `n` of `k` at the own pixel centre (x, y), premultiplied, or `None`. D-611:
/// `tex` the texture's atlas and levels, when one is read.
fn cover(k: &[f64], n: usize, x: f64, y: f64, tex: Option<Tex>) -> Option<[f64; 4]> {
    let s = &k[HEAD + STRIDE * n..][..STRIDE];
    if s[0] == STAR_KIND as f64 {
        let cv = star_cover(s[3], x - s[1], y - s[2]);
        if cv <= 0.0 {
            return None;
        }
        let a = s[6] * cv;
        return Some([s[7] * a, s[8] * a, s[9] * a, a]);
    }
    // D-615: the Emitter's soft sphere (9), feathered over s[4], and its glowing sphere (10), a
    // core of half the radius feathered over s[4] and a glow out to the radius; never lit.
    if s[0] == 9.0 || s[0] == 10.0 {
        let (rad, w) = (s[3], s[4]);
        let e = (x - s[1]).hypot(y - s[2]);
        let cv = if s[0] == 9.0 {
            ((rad - e) / w + 0.5).clamp(0.0, 1.0)
        } else {
            let core = ((rad / 2.0 - e) / w + 0.5).clamp(0.0, 1.0);
            core + (1.0 - core) * (1.0 - e / rad).max(0.0).powi(2) / 2.0
        };
        if cv <= 0.0 {
            return None;
        }
        let a = s[6] * cv;
        return Some([s[7] * a, s[8] * a, s[9] * a, a]);
    }
    if s[0] >= 5.0 {
        let (rad, dx, dy) = (s[3], x - s[1], y - s[2]);
        let (a, c) = (dx * s[4] + dy * s[5], -dx * s[5] + dy * s[4]);
        let e = dx.hypot(dy);
        let m = if s[0] == 5.0 {
            (rad - a.abs() + 0.5).clamp(0.0, 1.0) * (rad - c.abs() + 0.5).clamp(0.0, 1.0)
        } else if s[0] == 6.0 {
            (rad - e + 0.5).clamp(0.0, 1.0)
        } else {
            let t = (1.0 - (e / rad) * (e / rad)).max(0.0);
            t * t
        };
        if m <= 0.0 {
            return None;
        }
        let aa = s[6] * m;
        let Some(tex) = tex else {
            return Some([s[7] * aa, s[8] * aa, s[9] * aa, aa]);
        };
        let q = texel(tex, a / (2.0 * rad) + 0.5, c / (2.0 * rad) + 0.5, rad);
        let fd = s[10];
        let rgb = |i: usize| aa * (fd * q[i] + (1.0 - fd) * s[7 + i] * q[3]);
        return Some([rgb(0), rgb(1), rgb(2), aa * q[3]]);
    }
    let (cv, shade) = if s[0] == 0.0 {
        let (ax, ay, dx, dy) = (s[1], s[2], s[3] - s[1], s[4] - s[2]);
        let ll = dx * dx + dy * dy;
        let t = if ll > 0.0 {
            (((x - ax) * dx + (y - ay) * dy) / ll).clamp(0.0, 1.0)
        } else {
            0.0
        };
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
            if s[0] == 4.0 {
                c * (0.25 + 0.75 * (e / rad).min(1.0).powi(4))
            } else {
                c
            }
        };
        let v = if s[0] == 2.0 || s[0] == 4.0 {
            1.0
        } else {
            k[0]
        };
        let mut shade = 1.0;
        if v > 0.0 {
            let (dx, dy) = ((x - s[1]) / rad, (y - s[2]) / rad);
            let q = dx * dx + dy * dy;
            let nv = if q < 1.0 {
                [dx, dy, -(1.0 - q).sqrt()]
            } else {
                [dx / q.sqrt(), dy / q.sqrt(), 0.0]
            };
            shade =
                1.0 - v + v * (0.2 + 0.8 * (nv[0] * k[1] + nv[1] * k[2] + nv[2] * k[3]).max(0.0));
        }
        (cv, shade)
    };
    if cv <= 0.0 {
        return None;
    }
    let a = s[6] * cv;
    Some([s[7] * shade * a, s[8] * shade * a, s[9] * shade * a, a])
}

/// D-612: a star of outer radius `rad` at (dx, dy) from its centre: with e the distance to the
/// nearest of its ten edges, 1/2 + e inside (the even-odd rule, a ray to the right), 1/2 - e
/// outside, held to 0 to 1.
fn star_cover(rad: f64, dx: f64, dy: f64) -> f64 {
    let mut e = f64::INFINITY;
    let mut inside = false;
    for j in 0..10 {
        let (a, b) = (STAR[j], STAR[(j + 1) % 10]);
        let (ax, ay, bx, by) = (a[0] * rad, a[1] * rad, b[0] * rad, b[1] * rad);
        let (ex, ey) = (bx - ax, by - ay);
        let t = (((dx - ax) * ex + (dy - ay) * ey) / (ex * ex + ey * ey)).clamp(0.0, 1.0);
        e = e.min((dx - ax - t * ex).hypot(dy - ay - t * ey));
        if (ay > dy) != (by > dy) && dx < ax + (bx - ax) * (dy - ay) / (by - ay) {
            inside = !inside;
        }
    }
    (if inside { 0.5 + e } else { 0.5 - e }).clamp(0.0, 1.0)
}

/// D-610: the sprites `ids` of `k` laid over `d` at the own pixel centre (x, y), by the
/// transfer mode.
fn pixel(k: &[f64], ids: &[usize], x: f64, y: f64, mut d: [f64; 4], tex: Option<Tex>) -> [f64; 4] {
    let mode = k[4];
    for &n in ids {
        let Some(s) = cover(k, n, x, y, tex) else {
            continue;
        };
        d = if mode == 0.0 {
            [0, 1, 2, 3].map(|i| s[i] + (1.0 - s[3]) * d[i])
        } else if mode == 1.0 {
            [0, 1, 2, 3].map(|i| s[i] + d[i] - s[i] * d[i])
        } else {
            [
                s[0] + d[0],
                s[1] + d[1],
                s[2] + d[2],
                (s[3] + d[3]).min(1.0),
            ]
        };
    }
    d
}

/// D-610: Particle Space, after the purpose of CycoreFX's CC Particle World (document 21): the
/// particles drawn over a clear layer, or over the drawing with Composite With Original on,
/// farthest first. `(ox, oy)` the drawing's corner in the buffer. Never grows.
pub(crate) fn particle_space(source: &mut WorkingBuffer, e: &Effect, (ox, oy): (usize, usize)) {
    if texture_missing(e) {
        return;
    }
    let k = sprite_numbers(source, e, (ox, oy));
    if k[5] == 1.0 && k[6] == 0.0 {
        return;
    }
    let tex = texture(e).map(|t| (atlas(t), levels(t.width(), t.height())));
    let tex = tex.as_ref().map(|(a, l)| (a, l.as_slice()));
    let (w, h) = (source.width(), source.height());
    let (starts, list) = tile_lists(&k, (ox, oy), w, h);
    let (tw, keep) = (w.div_ceil(TILE), k[5] == 1.0);
    source
        .data_mut()
        .par_chunks_mut(w * 4 * TILE)
        .enumerate()
        .for_each(|(ty, band)| {
            for (row, line) in band.chunks_exact_mut(w * 4).enumerate() {
                let y = (ty * TILE + row) as f64 - oy as f64 + 0.5;
                for (bx, px) in line.chunks_exact_mut(4).enumerate() {
                    let t = ty * tw + bx / TILE;
                    let d = if keep {
                        [px[0], px[1], px[2], px[3]].map(f64::from)
                    } else {
                        [0.0; 4]
                    };
                    let out = pixel(
                        &k,
                        &list[starts[t]..starts[t + 1]],
                        bx as f64 - ox as f64 + 0.5,
                        y,
                        d,
                        tex,
                    );
                    for (v, o) in px.iter_mut().zip(out) {
                        *v = o as f32;
                    }
                }
            }
        });
}

/// D-611: a texture's atlas and its levels.
type Tex<'a> = (&'a WorkingBuffer, &'a [[usize; 4]]);

fn texture_named(e: &Effect) -> bool {
    matches!(e, Effect::ParticleSpace { texture_layer, .. } | Effect::ParticlePlane { texture_layer, .. } if texture_layer.as_str() != Some(""))
}

/// D-611: a texture layer named but not read (not in the composition, which compose said, or a
/// circle): the effect is left out (D-189's rule 4).
pub(crate) fn texture_missing(e: &Effect) -> bool {
    matches!(
        e,
        Effect::ParticleSpace { texture: None, .. } | Effect::ParticlePlane { texture: None, .. }
    ) && texture_named(e)
}

/// D-611: the texture read, when it has a picture.
pub(crate) fn texture(e: &Effect) -> Option<&WorkingBuffer> {
    let (Effect::ParticleSpace {
        texture: Some(t), ..
    }
    | Effect::ParticlePlane {
        texture: Some(t), ..
    }) = e
    else {
        return None;
    };
    (t.0.width() > 0 && t.0.height() > 0).then_some(&*t.0)
}

/// D-611: a texture's levels, level 0 first, as [x, y, w, h] in its atlas: level 0 at the
/// corner, each next ceil(w / 2) by ceil(h / 2) down to 1 by 1, stacked down level 0's right.
pub(crate) fn levels(w: usize, h: usize) -> Vec<[usize; 4]> {
    let mut out = vec![[0, 0, w, h]];
    while let Some(&[x, y, lw, lh]) = out.last().filter(|l| l[2] > 1 || l[3] > 1) {
        let (nx, ny) = if out.len() == 1 { (w, 0) } else { (x, y + lh) };
        out.push([nx, ny, lw.div_ceil(2), lh.div_ceil(2)]);
    }
    out
}

/// D-611: the atlas of [`levels`], each pixel of a level the mean of the 2 by 2 under it that
/// are inside.
// ponytail: the levels are made again every frame; keep them with the map if large textures
// grow slow.
pub(crate) fn atlas(t: &WorkingBuffer) -> WorkingBuffer {
    let lv = levels(t.width(), t.height());
    let width = lv.iter().map(|l| l[0] + l[2]).max().unwrap_or(0);
    let height = lv.iter().map(|l| l[1] + l[3]).max().unwrap_or(0);
    let mut out = WorkingBuffer::transparent(width, height);
    let mut prev: Vec<[f64; 4]> = t
        .data()
        .chunks_exact(4)
        .map(|p| [0, 1, 2, 3].map(|i| f64::from(p[i])))
        .collect();
    let set = |out: &mut WorkingBuffer, x: usize, y: usize, v: [f32; 4]| {
        out.data_mut()[(y * width + x) * 4..][..4].copy_from_slice(&v)
    };
    for y in 0..t.height() {
        for x in 0..t.width() {
            set(&mut out, x, y, t.pixel(x, y));
        }
    }
    for j in 1..lv.len() {
        let ([_, _, pw, ph], [x0, y0, w, h]) = (lv[j - 1], lv[j]);
        let mut next = Vec::with_capacity(w * h);
        for y in 0..h {
            for x in 0..w {
                let taps: Vec<[f64; 4]> = [(0, 0), (1, 0), (0, 1), (1, 1)]
                    .iter()
                    .map(|(dx, dy)| (2 * x + dx, 2 * y + dy))
                    .filter(|&(xx, yy)| xx < pw && yy < ph)
                    .map(|(xx, yy)| prev[yy * pw + xx])
                    .collect();
                let m = [0, 1, 2, 3]
                    .map(|i| taps.iter().map(|p| p[i]).sum::<f64>() / taps.len() as f64);
                set(&mut out, x0 + x, y0 + y, m.map(|v| v as f32));
                next.push(m);
            }
        }
        prev = next;
    }
    out
}

/// D-611: the texture at (s, u) across a sprite of radius `rad`: document 21's bilinear, each
/// tap held to its level's edge, at level log2(size / 2 rad) mixed towards the next.
fn texel((t, lv): Tex, s: f64, u: f64, rad: f64) -> [f64; 4] {
    let top = lv.len() - 1;
    let lam = (lv[0][2].max(lv[0][3]) as f64 / (2.0 * rad))
        .log2()
        .clamp(0.0, top as f64);
    let j = lam.floor() as usize;
    let held = |[x0, y0, w, h]: [usize; 4]| {
        let (fx, fy) = (s * w as f64 - 0.5, u * h as f64 - 0.5);
        let (bx, by) = (fx.floor(), fy.floor());
        let (ux, uy) = (fx - bx, fy - by);
        let mut out = [0.0; 4];
        for (dy, wy) in [(0.0, 1.0 - uy), (1.0, uy)] {
            for (dx, wx) in [(0.0, 1.0 - ux), (1.0, ux)] {
                let px = (bx + dx).clamp(0.0, (w - 1) as f64) as usize;
                let py = (by + dy).clamp(0.0, (h - 1) as f64) as usize;
                let q = t.pixel(x0 + px, y0 + py);
                for i in 0..4 {
                    out[i] += f64::from(q[i]) * wx * wy;
                }
            }
        }
        out
    };
    let a = held(lv[j]);
    if j == top {
        return a;
    }
    let (b, f) = (held(lv[j + 1]), lam - j as f64);
    [0, 1, 2, 3].map(|i| a[i] + (b[i] - a[i]) * f)
}

/// D-615: Particle Emitter's numbers at one frame, as compose keeps them in its history: the
/// settings of [`PARTICLE_EMITTER_SETTINGS`] in that order, then Emitter Position's x and y, each
/// held inside its range.
#[doc(hidden)]
pub fn emitter_numbers_row(e: &Effect) -> [f64; 56] {
    let Effect::ParticleEmitter {
        values,
        emitter_position,
        ..
    } = e
    else {
        unreachable!("Particle Emitter's numbers of another effect")
    };
    let mut out = [0.0; 56];
    for (o, (v, (_, lo, hi, _))) in out
        .iter_mut()
        .zip(values.iter().zip(PARTICLE_EMITTER_SETTINGS))
    {
        *o = v.clamp(lo, hi);
    }
    for c in 0..2 {
        out[54 + c] = emitter_position
            .get(c)
            .copied()
            .unwrap_or(0.0)
            .clamp(-1000.0, 1000.0);
    }
    out
}

/// D-615: one frame's Emitter numbers ([`emitter_numbers_row`]) as Particle Space's settings, on
/// a layer `w` by `h`: the engine's unit is the width, its origin the layer's centre; gravity
/// straight down; everything the Explosive animation does not read left at 0.
fn emitter_row(v: &[f64; 56], w: f64, h: f64) -> [f64; 46] {
    let mut e = [0.0; 46];
    e[RATE] = v[0] / 100.0;
    e[LONGEVITY] = v[15];
    e[PRODUCER] = v[54] / 100.0 - 0.5;
    e[PRODUCER + 1] = (v[55] / 100.0 - 0.5) * h / w;
    e[PRODUCER + 2] = v[1] / w;
    e[GRAVITY] = v[33] / w;
    e[GRAVITY_VECTOR + 1] = 1.0;
    e[RESISTANCE] = v[34];
    for i in 0..3 {
        e[WIND + i] = v[35 + i] / w;
    }
    e[TURBULENCE] = v[38] / w;
    e[TURBULENCE_SCALE] = v[39] / w;
    e[EVOLUTION] = v[40];
    e[FLOOR] = (v[41] / 100.0 - 0.5) * h / w;
    e[BOUNCINESS] = v[42];
    e[RANDOM_BOUNCINESS] = v[43];
    e[SEED] = v[44];
    e
}

/// D-615: row `n` of `rows` at frame `t` after the in point, straight-line between frames and held
/// at either end; `now` when nothing is keyed. As [`Run::at`].
fn held<const N: usize>(rows: Option<&[[f64; N]]>, now: &[f64; N], n: usize, t: f64) -> f64 {
    let Some(s) = rows else { return now[n] };
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

/// D-615: the turn Rx Ry Rz of three angles in degrees, Rz first, as Particle Space's camera.
fn turn3(d: [f64; 3]) -> [[f64; 3]; 3] {
    type M3 = [[f64; 3]; 3];
    let mat = |a: M3, b: M3| -> M3 {
        std::array::from_fn(|i| std::array::from_fn(|j| (0..3).map(|k| a[i][k] * b[k][j]).sum()))
    };
    let (sx, cx) = (d[0].to_radians().sin(), d[0].to_radians().cos());
    let (sy, cy) = (d[1].to_radians().sin(), d[1].to_radians().cos());
    let (sz, cz) = (d[2].to_radians().sin(), d[2].to_radians().cos());
    mat(
        [[1.0, 0.0, 0.0], [0.0, cx, -sx], [0.0, sx, cx]],
        mat(
            [[cy, 0.0, sy], [0.0, 1.0, 0.0], [-sy, 0.0, cy]],
            [[cz, -sz, 0.0], [sz, cz, 0.0], [0.0, 0.0, 1.0]],
        ),
    )
}

fn apply3(m: [[f64; 3]; 3], v: V) -> V {
    [0, 1, 2].map(|i| m[i][0] * v[0] + m[i][1] * v[1] + m[i][2] * v[2])
}

/// D-615: one frame of a Particle Emitter: the engine's run on its settings, its own numbers'
/// history (`own`, or `own_now` alone when nothing is keyed), the layer's width and the words as
/// indices in [`PARTICLE_EMITTER_WORDS`]' order.
struct Emitter<'a> {
    run: Run<'a>,
    own: Option<&'a [[f64; 56]]>,
    own_now: [f64; 56],
    w: f64,
    kinds: [usize; 9],
}

// The Emitter's numbers, where [`emitter_numbers_row`] holds them.
const EM_SIZE: usize = 2;
const EM_ROTATION: usize = 5;
const EM_GRID: usize = 8;
// D-711: the second system's settings, Aux Particles/sec first.
const EM_AUX: usize = 45;

/// D-711: an aux particle alive at the frame: the particle, its size, opacity, age fraction, and
/// its thrower's place on the gradient and brightness when it was thrown.
type Thrown = (Particle, [f64; 5]);

fn emitter_of(e: &Effect, w: f64, h: f64) -> Option<Emitter<'_>> {
    let Effect::ParticleEmitter {
        words,
        clock,
        history,
        ..
    } = e
    else {
        unreachable!("Particle Emitter's particles of another effect")
    };
    let u = clock[0] as i64;
    if u < 0 {
        return None;
    }
    let kinds = std::array::from_fn(|i| {
        PARTICLE_EMITTER_WORDS[i]
            .1
            .iter()
            .position(|x| *x == words[i])
            .unwrap_or(0)
    });
    let own_now = emitter_numbers_row(e);
    let own = history.as_deref().map(Vec::as_slice);
    let mut now = emitter_row(&own_now, w, h);
    let mut samples: Option<Vec<[f64; 46]>> =
        own.map(|s| s.iter().map(|r| emitter_row(r, w, h)).collect());
    // D-711: with the second system on, every particle that may still have a thrown one alive is
    // born: the longest life widened by the longest Aux Life and a frame.
    if kinds[6] != 0 {
        let rows = own.unwrap_or(std::slice::from_ref(&own_now));
        let top = |n: usize| rows.iter().map(|r| r[n]).fold(f64::MIN, f64::max);
        let wide = top(15) + top(EM_AUX + 2) + 1.0 / clock[1];
        now[LONGEVITY] = wide;
        for s in samples.iter_mut().flatten() {
            s[LONGEVITY] = wide;
        }
    }
    // ponytail: the numbers at the frame are held inside their ranges again after a draft has
    // divided the lengths, so a Turbulence Scale below 2 drafted at half is worked as 1; keep the
    // undivided ones if that ever shows.
    let run = Run {
        now,
        samples: samples.map(Cow::Owned),
        u,
        fps: clock[1],
        seed: crate::grade::mix(now[SEED].floor() as u64),
        mode: 0,
        floor_action: [0, 3, 1, 2][kinds[5]],
        flat: false,
    };
    Some(Emitter {
        run,
        own,
        own_now,
        w,
        kinds,
    })
}

impl Emitter<'_> {
    fn mine(&self, n: usize, t: f64) -> f64 {
        held(self.own, &self.own_now, n, t)
    }

    /// Particle k's start at b, and its own life (frames), size, opacity, place on the gradient
    /// and brightness.
    fn born(&self, k: i64, b: f64) -> (Particle, [f64; 5]) {
        let run = &self.run;
        let r = |s| run.r(k, s);
        let at = |n| self.mine(n, b);
        let m = turn3([at(EM_ROTATION), at(EM_ROTATION + 1), at(EM_ROTATION + 2)]);
        let size = [at(EM_SIZE), at(EM_SIZE + 1), at(EM_SIZE + 2)];
        let off = match self.kinds[0] {
            0 => [0.0; 3],
            1 => [
                size[0] * (r(0) - 0.5),
                size[1] * (r(1) - 0.5),
                size[2] * (r(2) - 0.5),
            ],
            2 => {
                let rho = r(0).powf(1.0 / 3.0);
                let d = sphere(1.0 - 2.0 * r(1), 2.0 * PI * r(2));
                [0, 1, 2].map(|i| size[i] / 2.0 * rho * d[i])
            }
            _ => {
                let n = [0, 1, 2].map(|i| at(EM_GRID + i).floor() as i64);
                let j = k % (n[0] * n[1] * n[2]);
                let idx = [j % n[0], (j / n[0]) % n[1], j / (n[0] * n[1])];
                [0, 1, 2].map(|i| size[i] * ((idx[i] as f64 + 0.5) / n[i] as f64 - 0.5))
            }
        };
        let d = apply3(m, off);
        let p = run.vec(PRODUCER, b);
        let x = [0, 1, 2].map(|i| p[i] + d[i] / self.w);
        let a = apply3(m, [0.0, -1.0, 0.0]);
        let h = if a[0].abs() >= 0.9 {
            [0.0, 1.0, 0.0]
        } else {
            [1.0, 0.0, 0.0]
        };
        let e1 = unit_or(cross(h, a), [1.0, 0.0, 0.0]);
        let e2 = cross(a, e1);
        let phi = 2.0 * PI * r(4);
        let spread = at(14);
        let alpha = (180.0 * spread / 100.0).to_radians();
        let u = sphere(1.0 - 2.0 * r(3), phi);
        let cap = |ct: f64| {
            let st = (1.0 - ct * ct).max(0.0).sqrt();
            add(
                mul(a, ct),
                mul(add(mul(e1, phi.cos()), mul(e2, phi.sin())), st),
            )
        };
        let way = match self.kinds[1] {
            0 => u,
            dir @ (1 | 2) => {
                let w = cap(1.0 - r(3) * (1.0 - alpha.cos()));
                if dir == 2 && r(10) < 0.5 {
                    mul(w, -1.0)
                } else {
                    w
                }
            }
            3 => cap(-(alpha / 2.0 * (2.0 * r(3) - 1.0)).sin()),
            dir => {
                let o = if d.iter().any(|c| *c != 0.0) {
                    unit_or(d, u)
                } else {
                    u
                };
                let o = if dir == 5 { mul(o, -1.0) } else { o };
                unit_or(add(o, mul(u, spread / 100.0)), o)
            }
        };
        let speed = at(11) / self.w * (1.0 + at(12) / 100.0 * (2.0 * r(5) - 1.0));
        let mut v = mul(way, speed);
        let vm = at(13);
        if vm != 0.0 {
            let pv = mul(
                sub(run.vec(PRODUCER, b + 0.125), run.vec(PRODUCER, b - 0.125)),
                run.fps / 0.25,
            );
            v = add(v, mul(pv, vm / 100.0));
        }
        let p = Particle {
            k,
            b,
            x,
            v,
            x0: x,
            sizes: (0.0, 0.0),
            size_factor: 1.0,
            twirl: 1.0,
            glued: false,
            iced: false,
            bounces: 0,
        };
        let own = [
            self.life(k, b),
            at(17) * (1.0 + at(18) / 100.0 * (2.0 * r(6) - 1.0)),
            at(24) / 100.0 * (1.0 - at(25) / 100.0 * r(12)),
            r(13),
            1.0 - at(31) / 100.0 * r(14),
        ];
        (p, own)
    }

    /// Particle k's life in frames, born at b.
    fn life(&self, k: i64, b: f64) -> f64 {
        self.mine(15, b) * self.run.fps * (1.0 - self.mine(16, b) / 100.0 * self.run.r(k, 11))
    }

    /// Particle k, born at b, as it is at u, on D-610's steps.
    fn moved(&self, k: i64, b: f64) -> (Particle, [f64; 5]) {
        let (mut p, own) = self.born(k, b);
        let mut t0 = b;
        for j in (4.0 * b).floor() as i64 + 1..=4 * self.run.u {
            let t1 = j as f64 / 4.0;
            self.run.step(&mut p, t0, (t1 - t0) / self.run.fps);
            t0 = t1;
        }
        (p, own)
    }

    /// D-711: particle k, born at b with `life` frames, stepped as [`Self::moved`] while a step
    /// starts before its death, and the (t, place, speed, bounces) at b and after each step.
    #[allow(clippy::type_complexity)]
    fn path(&self, k: i64, b: f64, life: f64) -> ([f64; 5], Vec<(f64, V, V, u64)>) {
        let (mut p, own) = self.born(k, b);
        let mut kept = vec![(b, p.x, p.v, 0)];
        let mut t0 = b;
        for j in (4.0 * b).floor() as i64 + 1..=4 * self.run.u {
            if t0 >= b + life {
                break;
            }
            let t1 = j as f64 / 4.0;
            self.run.step(&mut p, t0, (t1 - t0) / self.run.fps);
            kept.push((t1, p.x, p.v, p.bounces));
            t0 = t1;
        }
        (own, kept)
    }

    /// D-711: the (number, birth, place, speed) of each aux particle a thrower born at b with
    /// `life` frames throws by u: Continuously at Aux Particles/sec read at b, straight-line
    /// between its kept steps; At Bounce, Aux Particles/Bounce at each bounce's step end.
    fn throws(&self, b: f64, life: f64, kept: &[(f64, V, V, u64)]) -> Vec<(i64, f64, V, V)> {
        let u = self.run.u as f64;
        let mut out = Vec::new();
        if self.kinds[6] == 1 {
            let q = self.mine(EM_AUX, b) / self.run.fps;
            if q <= 0.0 {
                return out;
            }
            let mut i = 1;
            for j in 0.. {
                let c = b + (j as f64 + 0.5) / q;
                if c >= b + life || c > u {
                    break;
                }
                while kept[i].0 < c {
                    i += 1;
                }
                let ((t0, x0, v0, _), (t1, x1, v1, _)) = (kept[i - 1], kept[i]);
                let s = (c - t0) / (t1 - t0);
                out.push((
                    j,
                    c,
                    [0, 1, 2].map(|a| x0[a] + (x1[a] - x0[a]) * s),
                    [0, 1, 2].map(|a| v0[a] + (v1[a] - v0[a]) * s),
                ));
            }
            return out;
        }
        for w in kept.windows(2) {
            let ((_, _, _, n0), (t1, x1, v1, n1)) = (w[0], w[1]);
            for m in n0..n1 {
                let n = self.mine(EM_AUX + 1, t1).floor() as i64;
                for i in 0..n {
                    out.push((1000 * m as i64 + i, t1, x1, v1));
                }
            }
        }
        out
    }

    /// D-711: the aux particles alive at u, thrower by thrower, each by number. Aux particle j of
    /// k is numbered -1 - (2^32 k + j), thrown every way at Aux Velocity plus Aux Inherit Velocity
    /// of its thrower's speed, then moved on D-610's steps.
    fn thrown(&self) -> Vec<Thrown> {
        if self.kinds[6] == 0 {
            return Vec::new();
        }
        let (run, u) = (&self.run, self.run.u as f64);
        let over_life = self.kinds[3] == 1;
        run.births()
            .into_par_iter()
            .flat_map_iter(|(k, b)| {
                let life = self.life(k, b);
                let (own, kept) = self.path(k, b, life);
                self.throws(b, life, &kept)
                    .into_iter()
                    .filter_map(move |(j, c, x, vm)| {
                        let alife = self.mine(EM_AUX + 2, c) * run.fps;
                        if !(u - c < alife) {
                            return None;
                        }
                        let key = !k.wrapping_shl(32).wrapping_add(j);
                        let d = sphere(1.0 - 2.0 * run.r(key, 0), 2.0 * PI * run.r(key, 1));
                        let (speed, inherit) = (
                            self.mine(EM_AUX + 3, c) / self.w,
                            self.mine(EM_AUX + 4, c) / 100.0,
                        );
                        let v = [0, 1, 2].map(|a| d[a] * speed + vm[a] * inherit);
                        let mut p = Particle {
                            k: key,
                            b: c,
                            x,
                            v,
                            x0: x,
                            sizes: (0.0, 0.0),
                            size_factor: 1.0,
                            twirl: 1.0,
                            glued: false,
                            iced: false,
                            bounces: 0,
                        };
                        let mut t0 = c;
                        for jj in (4.0 * c).floor() as i64 + 1..=4 * run.u {
                            let t1 = jj as f64 / 4.0;
                            run.step(&mut p, t0, (t1 - t0) / run.fps);
                            t0 = t1;
                        }
                        let g = if over_life {
                            ((c - b) / life).min(1.0)
                        } else {
                            own[3]
                        };
                        Some((
                            p,
                            [
                                self.mine(EM_AUX + 5, c),
                                self.mine(EM_AUX + 7, c) / 100.0,
                                (u - c) / alife,
                                g,
                                own[4],
                            ],
                        ))
                    })
            })
            .collect()
    }

    /// The alive particles, oldest first, with their own numbers and age fractions.
    fn alive(&self) -> Vec<(Particle, [f64; 5], f64)> {
        let u = self.run.u as f64;
        self.run
            .births()
            .into_par_iter()
            .filter_map(|(k, b)| {
                let (life, age) = (self.life(k, b), u - b);
                (age < life).then(|| {
                    let (p, own) = self.moved(k, b);
                    (p, own, age / life)
                })
            })
            .collect()
    }
}

/// D-615: the Emitter's particles for B-495's test, as [`alive`]'s, on a layer `w` by `h`.
/// D-711: then the aux particles, for B-581's.
#[doc(hidden)]
pub fn emitter_alive(e: &Effect, w: f64, h: f64) -> Vec<[f64; 8]> {
    let Some(em) = emitter_of(e, w, h) else {
        return Vec::new();
    };
    let row = |p: &Particle| {
        [
            p.k as f64, p.b, p.x[0], p.x[1], p.x[2], p.v[0], p.v[1], p.v[2],
        ]
    };
    em.alive()
        .iter()
        .map(|(p, ..)| row(p))
        .chain(em.thrown().iter().map(|(p, _)| row(p)))
        .collect()
}

/// D-615: Particle Emitter's sprite list, as [`sprite_numbers`]': Sphere (kind 9, its feather's
/// width fifth), Glow Sphere (10, likewise), Star (8), Cloudlet (1, the faded sphere) or Streak
/// (0, the line), none lit, seen through the composition camera or else straight on from 50/36
/// of the width in front.
fn emitter_numbers(source: &WorkingBuffer, e: &Effect, (ox, oy): (usize, usize)) -> Vec<f64> {
    let (w0, h0) = (
        (source.width() - 2 * ox) as f64,
        (source.height() - 2 * oy) as f64,
    );
    let Some(em) = emitter_of(e, w0, h0) else {
        return vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
    };
    let Effect::ParticleEmitter { colors, view, .. } = e else {
        unreachable!()
    };
    let see = seen(*view, w0, h0);
    let now = &em.own_now;
    let levels = |from: usize| -> [f64; 5] { std::array::from_fn(|i| now[from + i] / 100.0) };
    let (sizes, fades, feather) = (levels(19), levels(26), now[32] / 100.0);
    let curve = |l: [f64; 5], f: f64| {
        let i = ((f * 4.0).floor() as usize).min(3);
        l[i] + (l[i + 1] - l[i]) * (f * 4.0 - i as f64)
    };
    let linear = |c: &str| crate::effects::encoded(c).map(crate::grade::to_linear);
    let stops: [V; 5] = std::array::from_fn(|i| linear(&colors[1 + i]));
    let gradient = |f: f64| {
        let i = ((f * 4.0).floor() as usize).min(3);
        [0, 1, 2].map(|c| stops[i][c] + (stops[i + 1][c] - stops[i][c]) * (f * 4.0 - i as f64))
    };
    let (kind, set_color) = (em.kinds[2], em.kinds[3]);
    let sprite = |kind: usize, p: &Particle, size: f64, alpha: f64, col: V| {
        let [x, y, ppu, depth] = see(p.x)?;
        let rad = size * ppu / (2.0 * w0);
        let sprite = if kind == 4 {
            let tail = see(sub(p.x, mul(p.v, 1.0 / em.run.fps)))?;
            [
                0.0,
                x,
                y,
                tail[0],
                tail[1],
                (2.0 * rad).max(1.0),
                alpha,
                col[0],
                col[1],
                col[2],
                0.0,
            ]
        } else {
            sprite_dot(kind, x, y, rad, alpha, col, feather)
        };
        Some((depth, p.k, sprite))
    };
    let mut drawn: Vec<(f64, i64, [f64; STRIDE])> = em
        .alive()
        .into_iter()
        .filter_map(|(p, own, f)| {
            let col = match set_color {
                0 => linear(&colors[0]),
                1 => gradient(f),
                _ => gradient(own[3]),
            }
            .map(|c| c * own[4]);
            sprite(
                kind,
                &p,
                own[1] * curve(sizes, f),
                own[2] * curve(fades, f),
                col,
            )
        })
        .collect();
    // D-711: the aux particles, From Main the thrower's colour when thrown, or Aux Color; their
    // size and opacity straight-line to the at-death levels read now.
    let (aux_size, aux_fade) = (now[EM_AUX + 6] / 100.0 - 1.0, now[EM_AUX + 8] / 100.0 - 1.0);
    let aux_color = linear(&colors[6]);
    drawn.extend(em.thrown().into_iter().filter_map(|(p, own)| {
        let col = match (em.kinds[8], set_color) {
            (1, _) => aux_color,
            (_, 0) => linear(&colors[0]).map(|c| c * own[4]),
            _ => gradient(own[3]).map(|c| c * own[4]),
        };
        sprite(
            em.kinds[7],
            &p,
            own[0] * (1.0 + aux_size * own[2]),
            own[1] * (1.0 + aux_fade * own[2]),
            col,
        )
    }));
    sprite_list(drawn, em.kinds[4])
}

/// D-616: Particle Grid's numbers at one frame, as compose keeps them in its history: the
/// settings of [`PARTICLE_GRID_SETTINGS`] in that order, then Form Center's x and y, each held
/// inside its range.
#[doc(hidden)]
pub fn grid_numbers_row(e: &Effect) -> [f64; 30] {
    let Effect::ParticleGrid {
        values,
        form_center,
        ..
    } = e
    else {
        unreachable!("Particle Grid's numbers of another effect")
    };
    let mut out = [0.0; 30];
    for (o, (v, (_, lo, hi, _))) in out
        .iter_mut()
        .zip(values.iter().zip(PARTICLE_GRID_SETTINGS))
    {
        *o = v.clamp(lo, hi);
    }
    for c in 0..2 {
        out[28 + c] = form_center
            .get(c)
            .copied()
            .unwrap_or(0.0)
            .clamp(-1000.0, 1000.0);
    }
    out
}

/// D-616: document 21's bilinear sample of a map at (`sx`, `sy`), pixel centres at halves,
/// transparent outside, worked in double precision.
fn bilinear64(m: &WorkingBuffer, sx: f64, sy: f64) -> [f64; 4] {
    let (fx, fy) = (sx - 0.5, sy - 0.5);
    let (x0, y0) = (fx.floor(), fy.floor());
    let (ux, uy) = (fx - x0, fy - y0);
    let mut out = [0.0; 4];
    for (dy, wy) in [(0.0, 1.0 - uy), (1.0, uy)] {
        for (dx, wx) in [(0.0, 1.0 - ux), (1.0, ux)] {
            let (x, y) = (x0 + dx, y0 + dy);
            if x < 0.0 || y < 0.0 || x >= m.width() as f64 || y >= m.height() as f64 {
                continue;
            }
            let s = m.pixel(x as usize, y as usize);
            for c in 0..4 {
                // D-616: an opaque pixel's colour put back on the 8-bit level it lies by, as D-614's mappers.
                let v = f64::from(s[c]);
                let v = if c < 3 && s[3] == 1.0 && (0.0..=1.0).contains(&v) {
                    crate::grade::to_linear(crate::playground::level(crate::grade::to_srgb(v)))
                } else {
                    v
                };
                out[c] += v * wx * wy;
            }
        }
    }
    out
}

/// D-616: the compound blur's luminance of a premultiplied sample, sRGB-coded.
fn luma(p: [f64; 4]) -> f64 {
    crate::grade::to_srgb((0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2]).clamp(0.0, 1.0))
}

/// D-616: one point of the grid: its index, its place in the engine's units, its size in pixels,
/// its opacity and its linear colour (none where the Color Map is clear).
type GridPoint = (usize, V, f64, f64, Option<V>);

/// D-616: every point of a Particle Grid on a layer `w` by `h`, the maps those of
/// [`PARTICLE_GRID_LAYERS`] (none where none is named); with the frame's numbers and the words
/// as indices in [`PARTICLE_GRID_WORDS`]' order. None before the layer's in point.
fn grid_points(
    e: &Effect,
    maps: [Option<&WorkingBuffer>; 4],
    w: f64,
    h: f64,
) -> Option<([f64; 30], [usize; 3], Vec<GridPoint>)> {
    let Effect::ParticleGrid {
        words,
        colors,
        clock,
        history,
        ..
    } = e
    else {
        unreachable!("Particle Grid's points of another effect")
    };
    let u = clock[0] as i64;
    if u < 0 {
        return None;
    }
    let kinds: [usize; 3] = std::array::from_fn(|i| {
        PARTICLE_GRID_WORDS[i]
            .1
            .iter()
            .position(|x| *x == words[i])
            .unwrap_or(0)
    });
    let s = grid_numbers_row(e);
    // The Flow settings summed from the in point, in seconds: straight lines between frames.
    let rows = history.as_deref().map(Vec::as_slice);
    let at = |k: usize, n: usize| rows.map_or(s[n], |r| r[k.min(r.len() - 1)][n]);
    let flow = |n: usize| {
        (0..u as usize)
            .map(|k| (at(k, n) + at(k + 1, n)) / 2.0)
            .sum::<f64>()
            / clock[1]
    };
    let off = [flow(19), flow(20), flow(21)];
    let evo = s[23] + flow(22);
    let seed = crate::grade::mix(s[27].floor() as u64);
    let r = |j: usize, st: u64| (crate::grade::unit(seed, j as i64, st as i64, 0, 0) + 1.0) / 2.0;
    let m = turn3([s[7], s[8], s[9]]);
    let centre = [s[28] / 100.0 - 0.5, (s[29] / 100.0 - 0.5) * h / w, s[6] / w];
    let size = [s[0], s[1], s[2]];
    let n = [
        s[3].floor() as usize,
        s[4].floor() as usize,
        s[5].floor() as usize,
    ];
    let linear = crate::effects::encoded(&colors[0]).map(crate::grade::to_linear);
    // ponytail: every point is worked every frame, up to 1000 by 1000 by 100 of them; cull the
    // ones off the layer first if big grids are slow.
    let points = (0..n[0] * n[1] * n[2])
        .into_par_iter()
        .filter_map(|j| {
            let (i, l, mm) = (j % n[0], (j / n[0]) % n[1], j / (n[0] * n[1]));
            // D-712: Box Strings and Layer place their points as Box Grid.
            let d0 = if kinds[0] != 1 {
                [
                    size[0] * ((i as f64 + 0.5) / n[0] as f64 - 0.5),
                    size[1] * ((l as f64 + 0.5) / n[1] as f64 - 0.5),
                    size[2] * ((mm as f64 + 0.5) / n[2] as f64 - 0.5),
                ]
            } else {
                let g = (mm + 1) as f64 / n[2] as f64;
                let (th, ph) = (
                    PI * (l as f64 + 0.5) / n[1] as f64,
                    2.0 * PI * i as f64 / n[0] as f64,
                );
                [
                    g * size[0] / 2.0 * th.sin() * ph.cos(),
                    -g * size[1] / 2.0 * th.cos(),
                    g * size[2] / 2.0 * th.sin() * ph.sin(),
                ]
            };
            let front = |k: usize| {
                if size[k] > 0.0 {
                    d0[k] / size[k] + 0.5
                } else {
                    0.5
                }
            };
            let (a, b) = (front(0), front(1));
            let got = maps.map(|mp| {
                mp.map(|mp| bilinear64(mp, a * mp.width() as f64, b * mp.height() as f64))
            });
            // D-712: Layer keeps only the points where the Base Layer is half opaque or more.
            if kinds[0] == 3 && got[3].is_some_and(|p| p[3] < 0.5) {
                return None;
            }
            let q: V =
                std::array::from_fn(|k| (d0[k] + off[k]) / s[18] + evo * [0.31, 0.57, 0.77][k]);
            let nn: [f64; 5] = std::array::from_fn(|c| noise(seed, c as u64, q));
            let mut d: V = std::array::from_fn(|k| d0[k] + s[15] * (2.0 * r(j, k as u64) - 1.0));
            if let Some(dm) = got[2] {
                d[2] -= s[26] * luma(dm);
            }
            let t = if size[1] > 0.0 { d0[1] / size[1] } else { 0.0 };
            let (sn, cs) = (
                (s[16] * t).to_radians().sin(),
                (s[16] * t).to_radians().cos(),
            );
            let d = apply3([[cs, 0.0, sn], [0.0, 1.0, 0.0], [-sn, 0.0, cs]], d);
            let d: V = std::array::from_fn(|k| d[k] + s[17] * nn[k]);
            let md = apply3(m, d);
            let x: V = std::array::from_fn(|k| centre[k] + md[k] / w);
            let mut px = s[10]
                * (1.0 + s[11] / 100.0 * (2.0 * r(j, 3) - 1.0))
                * (1.0 + s[24] / 100.0 * nn[3]).max(0.0);
            if let Some(sm) = got[1] {
                px *= luma(sm);
            }
            let mut alpha = s[12] / 100.0
                * (1.0 - s[13] / 100.0 * r(j, 4))
                * (1.0 + s[25] / 100.0 * nn[4]).clamp(0.0, 1.0);
            let col = match got[0] {
                None => Some(linear),
                Some(cm) if cm[3] > 0.0 => {
                    alpha *= cm[3];
                    Some([cm[0] / cm[3], cm[1] / cm[3], cm[2] / cm[3]])
                }
                Some(_) => None,
            };
            Some((j, x, px, alpha, col))
        })
        .collect();
    Some((s, kinds, points))
}

/// D-616: the Grid's points for B-496's test, as [`grid_points`]', each its index, place, size and
/// opacity, on a layer `w` by `h` with the maps given.
#[doc(hidden)]
pub fn grid_alive(e: &Effect, maps: [Option<&WorkingBuffer>; 4], w: f64, h: f64) -> Vec<[f64; 6]> {
    let Some((.., points)) = grid_points(e, maps, w, h) else {
        return Vec::new();
    };
    points
        .into_iter()
        .map(|(j, x, size, alpha, _)| [j as f64, x[0], x[1], x[2], size, alpha])
        .collect()
}

/// D-616: Particle Grid's sprite list, as [`sprite_numbers`]': the Emitter's camera and sprites
/// (Sphere, Glow Sphere, Star, Cloudlet), farthest first, equal depths by index.
fn grid_numbers(source: &WorkingBuffer, e: &Effect, (ox, oy): (usize, usize)) -> Vec<f64> {
    let (w0, h0) = (
        (source.width() - 2 * ox) as f64,
        (source.height() - 2 * oy) as f64,
    );
    let Effect::ParticleGrid { view, maps, .. } = e else {
        unreachable!()
    };
    let maps = [0, 1, 2, 3].map(|i| maps[i].as_ref().map(|m| &*m.0));
    let Some((s, kinds, points)) = grid_points(e, maps, w0, h0) else {
        return vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
    };
    let see = seen(*view, w0, h0);
    let drawn = if kinds[0] == 2 {
        // D-712: Box Strings, each point but a string's last a line to the next, as the
        // Emitter's streak, max(1, 2 R) wide at the start point's opacity, colour and depth.
        let nx = s[3].floor() as usize;
        points
            .iter()
            .filter_map(|&(j, x, size, alpha, col)| {
                if j % nx == nx - 1 {
                    return None;
                }
                let col = col?;
                let ([px, py, ppu, depth], [qx, qy, ..]) = (see(x)?, see(points[j + 1].1)?);
                let wd = (size * ppu / w0).max(1.0);
                Some((
                    depth,
                    j as i64,
                    [0.0, px, py, qx, qy, wd, alpha, col[0], col[1], col[2], 0.0],
                ))
            })
            .collect()
    } else {
        points
            .into_iter()
            .filter_map(|(j, x, size, alpha, col)| {
                let [px, py, ppu, depth] = see(x)?;
                Some((
                    depth,
                    j as i64,
                    sprite_dot(
                        kinds[1],
                        px,
                        py,
                        size * ppu / (2.0 * w0),
                        alpha,
                        col?,
                        s[14] / 100.0,
                    ),
                ))
            })
            .collect()
    };
    sprite_list(drawn, kinds[2])
}

/// D-615, D-616: a place in the engine's units as seen through the composition camera `view`
/// (compose's, or straight on from 50/36 of the width in front when none): the pixel, pixels per
/// unit and depth; none behind or too near the camera.
fn seen(view: [f64; 4], w0: f64, h0: f64) -> impl Fn(V) -> Option<[f64; 4]> {
    let view = if view[0] != 0.0 {
        view
    } else {
        [1.0, w0 / 2.0, h0 / 2.0, w0 * 50.0 / 36.0]
    };
    move |x: V| {
        let (ax, ay, al) = (view[1], view[2], view[3]);
        let zp = x[2] * w0;
        let depth = (al + zp) / w0;
        if al <= 0.0 || depth <= 0.01 {
            return None;
        }
        let ratio = al / (al + zp);
        Some([
            ax + (w0 / 2.0 + x[0] * w0 - ax) * ratio,
            ay + (h0 / 2.0 + x[1] * w0 - ay) * ratio,
            w0 * ratio,
            depth,
        ])
    }
}

/// D-615, D-616: one round sprite: Sphere (kind 9, its feather's width fifth), Glow Sphere (10,
/// likewise), Star (8) or Cloudlet (1, the faded sphere); never smaller than 0.7 pixels across
/// the radius, fainter by the true size below it.
fn sprite_dot(
    kind: usize,
    x: f64,
    y: f64,
    mut rad: f64,
    mut alpha: f64,
    col: V,
    feather: f64,
) -> [f64; STRIDE] {
    if rad < 0.7 {
        alpha *= (rad / 0.7) * (rad / 0.7);
        rad = 0.7;
    }
    let (k, w) = match kind {
        0 => (9.0, (feather * rad).max(1.0)),
        1 => (10.0, (feather * rad / 2.0).max(1.0)),
        2 => (STAR_KIND as f64, 0.0),
        _ => (1.0, 0.0),
    };
    [k, x, y, rad, w, 0.0, alpha, col[0], col[1], col[2], 0.0]
}

/// D-615, D-616: the sprites (depth, index, sprite) farthest first, equal depths by index, as
/// [`sprite_numbers`]' list over a clear layer, the transfer Normal, Add or Screen.
fn sprite_list(mut drawn: Vec<(f64, i64, [f64; STRIDE])>, transfer: usize) -> Vec<f64> {
    drawn.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    let mut k = vec![
        0.0,
        0.0,
        0.0,
        0.0,
        [0.0, 2.0, 1.0][transfer],
        0.0,
        drawn.len() as f64,
    ];
    k.reserve(STRIDE * drawn.len());
    for (_, _, s) in drawn {
        k.extend(s);
    }
    k
}
