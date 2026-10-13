//! D-613: Particle Playground (`core.particle_playground`), the particle engine's third effect
//! (P0-6), after what After Effects' Particle Playground is for: a Cannon, a Grid, a Layer
//! Exploder and a Particle Exploder, drawn as dots, as the exploded layer's pieces or as a layer
//! (Layer Map), pulled by Gravity and kept in by a Wall, each force with its Affects. Every rule
//! is ours (document 21); `tools/particle_playground_reference.py` is the reference, and this
//! file follows it step for step.
//!
//! No state is carried from frame to frame (document 20): a particle at a frame is a pure
//! function of the seed, its own numbers and the settings sampled once a frame from the in point,
//! stepped on D-610's grid of 4 steps a frame. Processor only, as Wave World (no card pass).

use crate::effects::{Effect, PARTICLE_PLAYGROUND_FROM, PARTICLE_PLAYGROUND_SETTINGS};
use crate::WorkingBuffer;
use rayon::prelude::*;
use std::f64::consts::PI;

/// The numbers' places in an effect's `values`, as `PARTICLE_PLAYGROUND_SETTINGS` lists them.
pub(crate) const BARREL: usize = 0;
const PPS: usize = 1;
const DIRECTION: usize = 2;
const DIRECTION_SPREAD: usize = 3;
const VELOCITY: usize = 4;
const VELOCITY_SPREAD: usize = 5;
const CANNON_RADIUS: usize = 6;
const GRID_WIDTH: usize = 7;
const GRID_HEIGHT: usize = 8;
const ACROSS: usize = 9;
const DOWN: usize = 10;
const GRID_RADIUS: usize = 11;
const EXPLODE_RADIUS: usize = 12;
const EXPLODE_DISPERSION: usize = 13;
const PE_RADIUS: usize = 14;
const PE_DISPERSION: usize = 15;
const GRAVITY: usize = 16;
const GRAVITY_SPREAD: usize = 17;
const GRAVITY_DIRECTION: usize = 18;
pub const WALL_MASK: usize = 19;
/// Each Affects' Older/Younger Than at 20 + 2 g, its Age Feather at 21 + 2 g.
const AGE: usize = 20;
pub(crate) const SEED: usize = 28;
/// Cannon Position x and y, then Grid Position's, per cent of the drawing.
const CANNON_AT: usize = 29;
const GRID_AT: usize = 31;
pub(crate) const COUNT: usize = 33;

/// The emitters each of [`PARTICLE_PLAYGROUND_FROM`]'s words takes, in its order: bit 1 the
/// Cannon, 2 the Grid, 3 the Layer Exploder, 4 the Particle Exploder's children.
const BITS: [u8; 15] = [
    0b11110, 0b00010, 0b00100, 0b01000, 0b10000, 0b00110, 0b01010, 0b10010, 0b01100, 0b10100, 0b11000, 0b11100, 0b11010,
    0b10110, 0b01110,
];

/// One frame's inputs, as compose fills them: `values` the settings at the frame, `history` the
/// settings at each frame from the in point to the one after this when any is keyed, `u` the
/// frames since the in point, `fps`, each Affects' Particles From as an index of [`BITS`],
/// Auto-Orient, the Cannon's and the Grid's linear colours, the maps (D-189's, fitted `center`
/// to the drawing; the Explode Layer's read at the in point, the others at this frame; `None`
/// for none named or not found), the Wall (flattened, closed, in the drawing's pixels) and the
/// drawing's size.
pub(crate) struct Inputs<'a> {
    pub values: &'a [f64; COUNT],
    pub history: Option<&'a [[f64; COUNT]]>,
    pub u: i64,
    pub fps: f64,
    pub from: [usize; 4],
    pub orient: bool,
    pub colors: [[f64; 3]; 2],
    pub explode: Option<&'a WorkingBuffer>,
    pub layer_map: Option<&'a WorkingBuffer>,
    pub selection: [Option<&'a WorkingBuffer>; 4],
    pub wall: Option<&'a [[f64; 2]]>,
    pub size: (f64, f64),
}

#[derive(Clone)]
struct Particle {
    code: i64,
    a: i64,
    b: i64,
    born: f64,
    x: [f64; 2],
    v: [f64; 2],
    col: [f64; 4],
    rad: f64,
    parent: i64,
    origin: [f64; 2],
    side: f64,
}

impl Particle {
    fn src(&self) -> u8 {
        if self.code <= 3 { self.code as u8 } else { 4 }
    }
}

struct Run<'a> {
    i: &'a Inputs<'a>,
    seed: u64,
}

fn clamp01(v: f64) -> f64 {
    v.clamp(0.0, 1.0)
}

fn heading(deg: f64) -> [f64; 2] {
    let t = deg.to_radians();
    [t.sin(), -t.cos()]
}

fn at(m: &WorkingBuffer, x: i64, y: i64) -> [f64; 4] {
    if x < 0 || y < 0 || x >= m.width() as i64 || y >= m.height() as i64 {
        return [0.0; 4];
    }
    m.pixel(x as usize, y as usize).map(f64::from)
}

fn bilinear(m: &WorkingBuffer, x: f64, y: f64) -> [f64; 4] {
    crate::render::sample_bilinear(m, x, y).map(f64::from)
}

fn lay(d: [f64; 4], s: [f64; 4]) -> [f64; 4] {
    [0, 1, 2, 3].map(|c| s[c] + (1.0 - s[3]) * d[c])
}

fn cross2(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}

impl Run<'_> {
    fn sample(&self, i: usize, n: usize) -> f64 {
        self.i.history.map_or(self.i.values[n], |s| s[i.min(s.len() - 1)][n])
    }

    fn at(&self, n: usize, t: f64) -> f64 {
        let Some(s) = self.i.history else { return self.i.values[n] };
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

    fn r(&self, p: &Particle, s: u64) -> f64 {
        (crate::grade::unit(self.seed, p.a, p.b, p.code, s) + 1.0) / 2.0
    }

    fn centre(&self, n: usize, t: f64) -> [f64; 2] {
        [self.at(n, t) * self.i.size.0 / 100.0, self.at(n + 1, t) * self.i.size.1 / 100.0]
    }

    /// D-610's births at Particles Per Second / fps a frame, nothing dying of age; worked as the
    /// engine works them, the rate as Birth Rate (Particles Per Second / 100).
    fn births(&self) -> Vec<(i64, f64)> {
        let u = self.i.u;
        let n = u as usize + 2;
        let q: Vec<f64> = (0..n).map(|i| 100.0 * (self.sample(i, PPS) / 100.0) / self.i.fps).collect();
        let mut out = Vec::new();
        if q.iter().all(|v| *v == q[0]) {
            if q[0] <= 0.0 {
                return out;
            }
            let hi = (q[0] * u as f64 - 0.5).floor() as i64 + 1;
            for k in 0..=hi {
                let b = (k as f64 + 0.5) / q[0];
                if b <= u as f64 {
                    out.push((k, b));
                }
            }
            return out;
        }
        let mut total = 0.0;
        for i in 0..u as usize {
            let next = total + (q[i] + q[i + 1]) / 2.0;
            let (a, b) = ((q[i + 1] - q[i]) / 2.0, q[i]);
            let mut k = (total - 0.5_f64).floor() as i64 + 1;
            while k as f64 + 0.5 <= next {
                let d = k as f64 + 0.5 - total;
                let den = b + (b * b + 4.0 * a * d).max(0.0).sqrt();
                let x = if den > 0.0 { (2.0 * d / den).min(1.0) } else { 1.0 };
                out.push((k, i as f64 + x));
                k += 1;
            }
            total = next;
        }
        out
    }

    fn particle(&self, code: i64, a: i64, b: i64, born: f64, x: [f64; 2], col: [f64; 4], rad: f64) -> Particle {
        Particle { code, a, b, born, x, v: [0.0; 2], col, rad, parent: 0, origin: [0.0; 2], side: 0.0 }
    }

    fn cannon(&self) -> Vec<Particle> {
        let col = { let c = self.i.colors[0]; [c[0], c[1], c[2], 1.0] };
        self.births()
            .into_iter()
            .map(|(k, b)| {
                let mut p = self.particle(1, k, 0, b, [0.0; 2], col, self.at(CANNON_RADIUS, b));
                let [cx, cy] = self.centre(CANNON_AT, b);
                let r = self.at(BARREL, b);
                p.x = if r < 0.0 {
                    let (rho, turn) = (-r * self.r(&p, 0).sqrt(), 2.0 * PI * self.r(&p, 1));
                    [cx + rho * turn.cos(), cy + rho * turn.sin()]
                } else {
                    [cx + r * (2.0 * self.r(&p, 0) - 1.0), cy + r * (2.0 * self.r(&p, 1) - 1.0)]
                };
                let d = self.at(DIRECTION, b) + self.at(DIRECTION_SPREAD, b) * (self.r(&p, 3) - 0.5);
                let speed = self.at(VELOCITY, b) + self.at(VELOCITY_SPREAD, b) * (self.r(&p, 5) - 0.5);
                let h = heading(d);
                p.v = [speed * h[0], speed * h[1]];
                p
            })
            .collect()
    }

    fn grid(&self) -> Vec<Particle> {
        let col = { let c = self.i.colors[1]; [c[0], c[1], c[2], 1.0] };
        let (w, h) = self.i.size;
        let mut out = Vec::new();
        for i in 0..=self.i.u as usize {
            let s = |n| self.sample(i, n);
            let (nx, ny) = (s(ACROSS).floor() as i64, s(DOWN).floor() as i64);
            let (cx, cy) = (s(GRID_AT) * w / 100.0, s(GRID_AT + 1) * h / 100.0);
            for row in 0..ny {
                for c in 0..nx {
                    let x = [
                        cx + s(GRID_WIDTH) * ((c as f64 + 0.5) / nx as f64 - 0.5),
                        cy + s(GRID_HEIGHT) * ((row as f64 + 0.5) / ny as f64 - 0.5),
                    ];
                    out.push(self.particle(2, row * nx + c, i as i64, i as f64, x, col, s(GRID_RADIUS)));
                }
            }
        }
        out
    }

    fn pieces(&self) -> Vec<Particle> {
        let Some(m) = self.i.explode else { return Vec::new() };
        let (w, h) = (m.width() as i64, m.height() as i64);
        let q = 2.0 * self.sample(0, EXPLODE_RADIUS);
        let (across, down) = ((w as f64 / q).ceil() as i64, (h as f64 / q).ceil() as i64);
        // The pixels whose centres lie in [lo, hi).
        let span = |lo: f64, hi: f64, size: i64| (lo - 0.5).ceil().max(0.0) as i64..((hi - 0.5).ceil() as i64).min(size);
        let mut out = Vec::new();
        for row in 0..down {
            for c in 0..across {
                let (xs, ys) = (span(c as f64 * q, (c + 1) as f64 * q, w), span(row as f64 * q, (row + 1) as f64 * q, h));
                let (mut sum, mut n, mut any) = ([0.0; 4], 0.0, false);
                for y in ys {
                    for x in xs.clone() {
                        let px = at(m, x, y);
                        any |= px[3] > 0.0;
                        for ch in 0..4 {
                            sum[ch] += px[ch];
                        }
                        n += 1.0;
                    }
                }
                if !any {
                    continue;
                }
                let o = [(c as f64 + 0.5) * q, (row as f64 + 0.5) * q];
                let mut p = self.particle(3, row * across + c, 0, 0.0, o, sum.map(|v| v / n), q / 2.0);
                p.origin = o;
                p.side = q;
                let speed = self.sample(0, EXPLODE_DISPERSION) * self.r(&p, 5);
                let hd = heading(360.0 * self.r(&p, 3));
                p.v = [speed * hd[0], speed * hd[1]];
                out.push(p);
            }
        }
        out
    }

    fn weight(&self, g: usize, p: &Particle, t: f64) -> f64 {
        if BITS[self.i.from[g]] & (1 << p.src()) == 0 {
            return 0.0;
        }
        let age = (t - p.born) / self.i.fps;
        let (tt, ff) = (self.at(AGE + 2 * g, t), self.at(AGE + 2 * g + 1, t));
        let mut w = if tt == 0.0 {
            1.0
        } else {
            let d = if tt > 0.0 { age - tt } else { -tt - age };
            if ff == 0.0 { if d >= 0.0 { 1.0 } else { 0.0 } } else { clamp01(d / ff + 0.5) }
        };
        if let Some(m) = self.i.selection[g] {
            let (gx, gy) = (p.x[0].floor(), p.x[1].floor());
            w *= if gx >= 0.0 && gy >= 0.0 && gx < self.i.size.0 && gy < self.i.size.1 {
                let s = at(m, gx as i64, gy as i64);
                crate::grade::to_srgb(clamp01(0.2126 * s[0] + 0.7152 * s[1] + 0.0722 * s[2]))
            } else {
                0.0
            };
        }
        w
    }

    fn affected(&self, g: usize, p: &Particle, t: f64) -> bool {
        self.weight(g, p, t) > self.r(p, 20 + g as u64)
    }

    fn children(&self, p: &Particle, t: f64) -> Vec<Particle> {
        let c = self.at(PE_RADIUS, t);
        let n = ((p.rad / c).powi(2) + 0.5).floor().clamp(1.0, 100.0) as i64;
        (0..n)
            .map(|i| {
                let mut ch = self.particle(p.code + 4 * (i + 1), p.a, p.b, t, p.x, p.col, c);
                ch.parent = p.code;
                let speed = self.at(PE_DISPERSION, t) * self.r(&ch, 5);
                let hd = heading(360.0 * self.r(&ch, 3));
                ch.v = [p.v[0] + speed * hd[0], p.v[1] + speed * hd[1]];
                ch
            })
            .collect()
    }

    /// The Wall: the move cut at the first edge crossed, turned off it, the rest moved again;
    /// after 4 bounces the particle stays.
    fn bounce(&self, p: &mut Particle, wall: &[[f64; 2]], a: [f64; 2], mut h: f64) {
        let (mut x, mut v) = (p.x, p.v);
        let mut skip = usize::MAX;
        for _ in 0..4 {
            let x1 = [0, 1].map(|i| x[i] + v[i] * h + a[i] * h * h / 2.0);
            let d = [x1[0] - x[0], x1[1] - x[1]];
            let mut best: Option<(f64, usize, [f64; 2])> = None;
            for j in 0..wall.len() {
                if j == skip {
                    continue;
                }
                let (pa, pb) = (wall[j], wall[(j + 1) % wall.len()]);
                let e = [pb[0] - pa[0], pb[1] - pa[1]];
                let den = cross2(d, e);
                if den == 0.0 {
                    continue;
                }
                let w0 = [pa[0] - x[0], pa[1] - x[1]];
                let (f, s) = (cross2(w0, e) / den, cross2(w0, d) / den);
                if f > 0.0 && f <= 1.0 && (0.0..=1.0).contains(&s) && best.is_none_or(|b| f < b.0) {
                    best = Some((f, j, e));
                }
            }
            let Some((f, j, e)) = best else {
                p.x = x1;
                p.v = [0, 1].map(|i| v[i] + a[i] * h);
                return;
            };
            let tau = f * h;
            let xc = [0, 1].map(|i| x[i] + f * d[i]);
            let vc = [0, 1].map(|i| v[i] + a[i] * tau);
            let le = e[0].hypot(e[1]);
            let nrm = [-e[1] / le, e[0] / le];
            let vn = vc[0] * nrm[0] + vc[1] * nrm[1];
            x = xc;
            v = [vc[0] - 2.0 * vn * nrm[0], vc[1] - 2.0 * vn * nrm[1]];
            h -= tau;
            skip = j;
        }
        p.x = x;
        p.v = v;
    }

    fn step(&self, p: &mut Particle, t0: f64, h: f64) {
        let mut a = [0.0; 2];
        if self.affected(2, p, t0) {
            let f = self.at(GRAVITY, t0) + self.at(GRAVITY_SPREAD, t0) * (self.r(p, 7) - 0.5);
            let g = heading(self.at(GRAVITY_DIRECTION, t0));
            a = [f * g[0], f * g[1]];
        }
        if let Some(wall) = self.i.wall {
            if self.affected(3, p, t0) {
                self.bounce(p, wall, a, h);
                return;
            }
        }
        let (x, v) = (p.x, p.v);
        p.x = [0, 1].map(|i| x[i] + v[i] * h + a[i] * h * h / 2.0);
        p.v = [0, 1].map(|i| v[i] + a[i] * h);
    }

    /// Particle p as it is at u: itself, or its children once it exploded.
    // ponytail: every frame steps every particle from its birth and nothing dies, O(N u^2) over
    // a shot; keep the particles between frames if long shots at high rates grow slow.
    fn moved(&self, mut p: Particle) -> Vec<Particle> {
        let u = self.i.u;
        let mut pts = vec![p.born];
        pts.extend(((4.0 * p.born).floor() as i64 + 1..=4 * u).map(|j| j as f64 / 4.0));
        let explodes = p.code <= 3;
        for (i, &t0) in pts.iter().enumerate() {
            if explodes && self.at(PE_RADIUS, t0) > 0.0 && self.affected(0, &p, t0) {
                return self.children(&p, t0).into_iter().flat_map(|c| self.moved(c)).collect();
            }
            if let Some(&t1) = pts.get(i + 1) {
                self.step(&mut p, t0, (t1 - t0) / self.i.fps);
            }
        }
        vec![p]
    }

    fn everyone(&self) -> Vec<Particle> {
        let born: Vec<Particle> = [self.cannon(), self.grid(), self.pieces()].concat();
        let mut out: Vec<Particle> = born.into_par_iter().flat_map_iter(|p| self.moved(p)).collect();
        let order = |c: i64| if c <= 3 { c - 1 } else { 3 };
        out.sort_by(|p, q| {
            p.born
                .total_cmp(&q.born)
                .then((order(p.code), p.parent, p.a, p.b, p.code).cmp(&(order(q.code), q.parent, q.a, q.b, q.code)))
        });
        out
    }
}

fn run<'a>(i: &'a Inputs<'a>) -> Option<Run<'a>> {
    (i.u >= 0).then(|| Run { i, seed: crate::grade::mix(i.values[SEED].floor() as u64) })
}

/// Every number of `e` at its frame in [`Inputs`]' `values` order, each held inside its range
/// (D-610's samples): the settings, then Cannon Position's x and y, then Grid Position's. Compose
/// fills the history with these.
#[doc(hidden)]
pub fn numbers(e: &Effect) -> [f64; COUNT] {
    let Effect::ParticlePlayground { values, cannon_position, grid_position, .. } = e else {
        unreachable!("Particle Playground's numbers of another effect")
    };
    let mut out = [0.0; COUNT];
    for (o, (v, (_, lo, hi, _))) in out.iter_mut().zip(values.iter().zip(PARTICLE_PLAYGROUND_SETTINGS)) {
        *o = v.clamp(lo, hi);
    }
    for (k, p) in [cannon_position, grid_position].into_iter().enumerate() {
        for c in 0..2 {
            out[CANNON_AT + 2 * k + c] = p.get(c).copied().unwrap_or(0.0).clamp(-1000.0, 1000.0);
        }
    }
    out
}

/// `e`'s inputs: its clock and history as compose filled them, `maps` in its layers' order.
fn inputs<'a>(e: &'a Effect, values: &'a [f64; COUNT], maps: [Option<&'a WorkingBuffer>; 6], wall: Option<&'a [[f64; 2]]>, size: (f64, f64)) -> Inputs<'a> {
    let Effect::ParticlePlayground { words, colors, clock, history, .. } = e else {
        unreachable!("Particle Playground's inputs of another effect")
    };
    let colour = |c: &str| crate::effects::encoded(c).map(crate::grade::to_linear);
    Inputs {
        values,
        history: history.as_deref().map(Vec::as_slice),
        u: clock[0] as i64,
        fps: clock[1],
        from: [0, 1, 2, 3].map(|g| PARTICLE_PLAYGROUND_FROM.iter().position(|w| *w == words[g]).unwrap_or(0)),
        orient: words[4] == "on",
        colors: [colour(&colors[0]), colour(&colors[1])],
        explode: maps[0],
        layer_map: maps[1],
        selection: [maps[2], maps[3], maps[4], maps[5]],
        wall,
        size,
    }
}

/// Every particle at the frame, in the order drawn, as [code, a, b, born, x, y, vx, vy], for
/// B-493's test: `e`'s clock and history filled as compose fills them, `maps` and `wall` as
/// compose reads them, `size` the drawing's.
#[doc(hidden)]
pub fn listed(e: &Effect, maps: [Option<&WorkingBuffer>; 6], wall: Option<&[[f64; 2]]>, size: (f64, f64)) -> Vec<[f64; 8]> {
    let values = numbers(e);
    let i = inputs(e, &values, maps, wall, size);
    let Some(r) = run(&i) else { return Vec::new() };
    r.everyone().iter().map(|p| [p.code as f64, p.a as f64, p.b as f64, p.born, p.x[0], p.x[1], p.v[0], p.v[1]]).collect()
}

/// D-613: `e` drawn over a clear layer; `(ox, oy)` the drawing's corner in the buffer. The
/// settings are already valid.
pub(crate) fn apply(source: &mut WorkingBuffer, e: &Effect, (ox, oy): (usize, usize)) {
    let Effect::ParticlePlayground { maps, wall, .. } = e else { unreachable!("Particle Playground of another effect") };
    let size = ((source.width() - 2 * ox) as f64, (source.height() - 2 * oy) as f64);
    let values = numbers(e);
    let i = inputs(e, &values, maps.each_ref().map(|m| m.as_ref().map(|m| &*m.0)), wall.as_deref(), size);
    particle_playground(source, &i, (ox, oy));
}

/// How one particle is drawn.
enum Look {
    Dot { rad: f64, col: [f64; 4] },
    Piece { side: f64, origin: [f64; 2] },
    Copy,
}

/// D-613: the particles laid over a clear layer, in the order born. `(ox, oy)` the drawing's
/// corner in the buffer. Never grows.
fn particle_playground(source: &mut WorkingBuffer, i: &Inputs, (ox, oy): (usize, usize)) {
    let w = source.width();
    let Some(r) = run(i) else {
        source.data_mut().fill(0.0);
        return;
    };
    let u = i.u as f64;
    let (mw, mh) = i.size;
    // Each particle's look, place, turn (cosine, sine) and reach.
    let drawn: Vec<(Look, [f64; 2], [f64; 2], f64)> = r
        .everyone()
        .into_iter()
        .map(|p| {
            let o = if i.orient && (p.v[0] != 0.0 || p.v[1] != 0.0) { p.v[0].atan2(-p.v[1]) } else { 0.0 };
            let turn = [o.cos(), o.sin()];
            if i.layer_map.is_some() && r.affected(1, &p, u) {
                return (Look::Copy, p.x, turn, (mw / 2.0 + 1.0).hypot(mh / 2.0 + 1.0));
            }
            if p.code == 3 {
                return (Look::Piece { side: p.side, origin: p.origin }, p.x, turn, (p.side / 2.0 + 0.5) * std::f64::consts::SQRT_2);
            }
            let (rad, k) = if p.rad < 0.7 { (0.7, (p.rad / 0.7).powi(2)) } else { (p.rad, 1.0) };
            (Look::Dot { rad, col: p.col.map(|c| c * k) }, p.x, turn, rad + 0.5)
        })
        .collect();
    source.data_mut().par_chunks_mut(w * 4).enumerate().for_each(|(row, line)| {
        let y = row as f64 - oy as f64 + 0.5;
        let mut out = vec![[0.0f64; 4]; w];
        for (look, x, [cs, sn], reach) in &drawn {
            if (y - x[1]).abs() > *reach {
                continue;
            }
            let lo = ((x[0] - reach + ox as f64 - 0.5).floor().max(0.0) as usize).min(w);
            let hi = ((x[0] + reach + ox as f64 + 0.5).ceil().max(0.0) as usize).min(w);
            for (bx, d) in out.iter_mut().enumerate().take(hi).skip(lo) {
                let (dx, dy) = (bx as f64 - ox as f64 + 0.5 - x[0], y - x[1]);
                // Rot(-o) of (dx, dy).
                let (lx, ly) = (dx * cs + dy * sn, -dx * sn + dy * cs);
                match look {
                    Look::Copy => {
                        let s = bilinear(i.layer_map.unwrap(), mw / 2.0 + lx, mh / 2.0 + ly);
                        if s[3] > 0.0 {
                            *d = lay(*d, s);
                        }
                    }
                    Look::Piece { side, origin } => {
                        let cv = clamp01(side / 2.0 - lx.abs() + 0.5) * clamp01(side / 2.0 - ly.abs() + 0.5);
                        if cv > 0.0 {
                            let hold = side / 2.0 - 0.5;
                            let s = bilinear(i.explode.unwrap(), origin[0] + lx.clamp(-hold, hold), origin[1] + ly.clamp(-hold, hold));
                            *d = lay(*d, s.map(|v| v * cv));
                        }
                    }
                    Look::Dot { rad, col } => {
                        let cv = clamp01(rad - dx.hypot(dy) + 0.5);
                        if cv > 0.0 {
                            *d = lay(*d, col.map(|v| v * cv));
                        }
                    }
                }
            }
        }
        for (px, o) in line.chunks_exact_mut(4).zip(out) {
            for (v, c) in px.iter_mut().zip(o) {
                *v = c as f32;
            }
        }
    });
}
