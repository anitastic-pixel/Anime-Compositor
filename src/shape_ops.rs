//! D-738 (d): the path operations a shape group may carry, each taking a path and giving one
//! back, worked exactly as `tools/shape_operation_reference.py` writes them down.

use crate::mask::MaskPoint;

type P = (f64, f64);

/// An operation on a path of more points than this leaves it as it is, and Zig Zag and Wiggle
/// Paths split each piece into no more than this many divided by the path's pieces.
pub const LIMIT: usize = 100_000;

/// How far toward the corner Round Corners' handles reach, lottie-web's (MIT) number.
const ROUND: f64 = 0.5519;

/// Wiggle Paths' place for the noise every point shares.
const SHARED: u64 = 1 << 32;

fn add(a: P, b: P) -> P {
    (a.0 + b.0, a.1 + b.1)
}

fn sub(a: P, b: P) -> P {
    (a.0 - b.0, a.1 - b.1)
}

fn unit(v: P) -> P {
    let n = v.0.hypot(v.1);
    if n != 0.0 {
        (v.0 / n, v.1 / n)
    } else {
        (0.0, 0.0)
    }
}

/// The cubic a, b, c, d from point `i` to the next.
fn piece(pts: &[MaskPoint], i: usize) -> [P; 4] {
    let (p0, p1) = (pts[i], pts[(i + 1) % pts.len()]);
    [
        p0.point,
        add(p0.point, p0.out_handle),
        add(p1.point, p1.in_handle),
        p1.point,
    ]
}

fn bez([a, b, c, d]: [P; 4], t: f64) -> P {
    let u = 1.0 - t;
    let w = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
    (
        w.0 * a.0 + w.1 * b.0 + w.2 * c.0 + w.3 * d.0,
        w.0 * a.1 + w.1 * b.1 + w.2 * c.1 + w.3 * d.1,
    )
}

fn dbez([a, b, c, d]: [P; 4], t: f64) -> P {
    let u = 1.0 - t;
    let k = |a: f64, b: f64, c: f64, d: f64| {
        3.0 * (u * u * (b - a) + 2.0 * u * t * (c - b) + t * t * (d - c))
    };
    (k(a.0, b.0, c.0, d.0), k(a.1, b.1, c.1, d.1))
}

fn first_moving(vs: [P; 3]) -> P {
    vs.into_iter()
        .find(|v| *v != (0.0, 0.0))
        .unwrap_or((0.0, 0.0))
}

fn point(point: P, in_handle: P, out_handle: P) -> MaskPoint {
    MaskPoint {
        point,
        in_handle,
        out_handle,
    }
}

/// Round Corners, lottie-web's: each corner with no handles, other than an open path's ends,
/// becomes two points, each min(radius, half that side) toward its neighbour.
pub fn round_corners(pts: Vec<MaskPoint>, closed: bool, radius: f64) -> Vec<MaskPoint> {
    if radius == 0.0 {
        return pts;
    }
    let n = pts.len();
    let mut out = Vec::with_capacity(n * 2);
    for (i, p) in pts.iter().enumerate() {
        let corner = p.in_handle == (0.0, 0.0) && p.out_handle == (0.0, 0.0);
        if !corner || (!closed && (i == 0 || i == n - 1)) {
            out.push(*p);
            continue;
        }
        let v = p.point;
        for (near, first) in [
            (pts[(i + n - 1) % n].point, true),
            (pts[(i + 1) % n].point, false),
        ] {
            let d = (v.0 - near.0).hypot(v.1 - near.1);
            let f = if d != 0.0 {
                (d / 2.0).min(radius) / d
            } else {
                0.0
            };
            let q = (v.0 + (near.0 - v.0) * f, v.1 + (near.1 - v.1) * f);
            let h = ((v.0 - q.0) * ROUND, (v.1 - q.1) * ROUND);
            out.push(if first {
                point(q, (0.0, 0.0), h)
            } else {
                point(q, h, (0.0, 0.0))
            });
        }
    }
    out
}

/// Pucker & Bloat, lottie-web's: points toward the mean of the points by the amount, handles'
/// ends away from it.
pub fn pucker_bloat(pts: Vec<MaskPoint>, amount: f64) -> Vec<MaskPoint> {
    let a = amount / 100.0;
    let n = pts.len();
    if n == 0 || a == 0.0 {
        return pts;
    }
    let mut c = (0.0, 0.0);
    for p in &pts {
        c = (c.0 + p.point.0, c.1 + p.point.1);
    }
    let c = (c.0 / n as f64, c.1 / n as f64);
    pts.iter()
        .map(|p| {
            let v = p.point;
            let q = (v.0 + (c.0 - v.0) * a, v.1 + (c.1 - v.1) * a);
            let handle = |h: P| {
                let e = add(v, h);
                (e.0 + (c.0 - e.0) * -a - q.0, e.1 + (c.1 - e.1) * -a - q.1)
            };
            point(q, handle(p.in_handle), handle(p.out_handle))
        })
        .collect()
}

/// Each straight piece given handles a third of the way along it, so that equal steps of t are
/// equal steps along it.
fn thirds(mut pts: Vec<MaskPoint>, closed: bool) -> Vec<MaskPoint> {
    let n = pts.len();
    let pieces = if closed { n } else { n.saturating_sub(1) };
    for i in 0..pieces {
        let j = (i + 1) % n;
        if pts[i].out_handle == (0.0, 0.0) && pts[j].in_handle == (0.0, 0.0) {
            let (a, b) = (pts[i].point, pts[j].point);
            pts[i].out_handle = ((b.0 - a.0) / 3.0, (b.1 - a.1) / 3.0);
            pts[j].in_handle = ((a.0 - b.0) / 3.0, (a.1 - b.1) / 3.0);
        }
    }
    pts
}

/// Twist: every point and handle end turned clockwise about the center, by the angle at the
/// center and less outward, not at all at the farthest point.
pub fn twist(pts: Vec<MaskPoint>, closed: bool, angle: f64, c: P) -> Vec<MaskPoint> {
    if angle == 0.0 || pts.is_empty() {
        return pts;
    }
    let pts = thirds(pts, closed);
    let far = pts
        .iter()
        .map(|p| (p.point.0 - c.0).hypot(p.point.1 - c.1))
        .fold(f64::NEG_INFINITY, f64::max);
    if far == 0.0 {
        return pts;
    }
    let turned = |e: P| {
        let (dx, dy) = (e.0 - c.0, e.1 - c.1);
        let f = (1.0 - dx.hypot(dy) / far).max(0.0);
        let (s, co) = (angle * f).to_radians().sin_cos();
        (c.0 + co * dx - s * dy, c.1 + s * dx + co * dy)
    };
    pts.iter()
        .map(|p| {
            let q = turned(p.point);
            point(
                q,
                sub(turned(add(p.point, p.in_handle)), q),
                sub(turned(add(p.point, p.out_handle)), q),
            )
        })
        .collect()
}

/// Zig Zag: each piece split into ridges + 1, every point then moved by size, one way and the
/// other in turn, square to the path.
pub fn zig_zag(
    pts: Vec<MaskPoint>,
    closed: bool,
    size: f64,
    ridges: f64,
    smooth: bool,
) -> Vec<MaskPoint> {
    let n = pts.len();
    if n < 2 {
        return pts;
    }
    let pts = thirds(pts, closed);
    let pieces = if closed { n } else { n - 1 };
    let m = (ridges.floor() as usize + 1).min((LIMIT / pieces).max(1));
    let mut along: Vec<(P, P)> = Vec::new();
    for i in 0..n {
        let mut dirs = Vec::with_capacity(2);
        if closed || i > 0 {
            let [a, b, c, d] = piece(&pts, (i + n - 1) % n);
            dirs.push(unit(first_moving([sub(d, c), sub(d, b), sub(d, a)])));
        }
        if closed || i < n - 1 {
            let [a, b, c, d] = piece(&pts, i);
            dirs.push(unit(first_moving([sub(b, a), sub(c, a), sub(d, a)])));
        }
        let sum = dirs.iter().fold((0.0, 0.0), |s, d| (s.0 + d.0, s.1 + d.1));
        let mut tangent = unit(sum);
        if tangent == (0.0, 0.0) {
            tangent = dirs[dirs.len() - 1];
        }
        along.push((pts[i].point, tangent));
        if closed || i < n - 1 {
            let q = piece(&pts, i);
            for k in 1..m {
                let t = k as f64 / m as f64;
                let mut tangent = unit(dbez(q, t));
                if tangent == (0.0, 0.0) {
                    tangent = unit(sub(q[3], q[0]));
                }
                along.push((bez(q, t), tangent));
            }
        }
    }
    let count = along.len();
    along
        .iter()
        .enumerate()
        .map(|(j, &(q, t))| {
            let by = if j % 2 == 0 { size } else { -size };
            let moved = (q.0 - t.1 * by, q.1 + t.0 * by);
            let (mut hi, mut ho) = ((0.0, 0.0), (0.0, 0.0));
            if smooth {
                if closed || j > 0 {
                    let before = along[(j + count - 1) % count].0;
                    let d = (q.0 - before.0).hypot(q.1 - before.1) / 2.0;
                    hi = (-t.0 * d, -t.1 * d);
                }
                if closed || j < count - 1 {
                    let after = along[(j + 1) % count].0;
                    let d = (q.0 - after.0).hypot(q.1 - after.1) / 2.0;
                    ho = (t.0 * d, t.1 * d);
                }
            }
            point(moved, hi, ho)
        })
        .collect()
}

fn mix(x: u64) -> u64 {
    let x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let z = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    let z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn rnd(seed: u64, j: u64, axis: u64, k: i64) -> f64 {
    let h = mix(seed ^ mix(j ^ mix(axis ^ mix(k as u64))));
    (h >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
}

fn noise(seed: u64, j: u64, axis: u64, tau: f64) -> f64 {
    let k = tau.floor();
    let f = tau - k;
    let u = f * f * (3.0 - 2.0 * f);
    let k = k as i64;
    rnd(seed, j, axis, k) * (1.0 - u) + rnd(seed, j, axis, k + 1) * u
}

/// Each piece split exactly into `m` pieces of equal t.
fn split(pts: Vec<MaskPoint>, closed: bool, m: usize) -> Vec<MaskPoint> {
    let pts = thirds(pts, closed);
    let n = pts.len();
    let pieces = if closed { n } else { n - 1 };
    let third = 3.0 * m as f64;
    let mut out = Vec::with_capacity(pieces * m + 1);
    let mut ends = Vec::with_capacity(pieces);
    for i in 0..pieces {
        let q = piece(&pts, i);
        for k in 0..m {
            let t = k as f64 / m as f64;
            let dv = dbez(q, t);
            let (at, hin) = if k == 0 {
                (pts[i].point, pts[i].in_handle)
            } else {
                (bez(q, t), (-dv.0 / third, -dv.1 / third))
            };
            out.push(point(at, hin, (dv.0 / third, dv.1 / third)));
        }
        let dv = dbez(q, 1.0);
        ends.push((-dv.0 / third, -dv.1 / third));
    }
    if closed {
        out[0].in_handle = ends[pieces - 1];
    } else {
        out.push(point(
            pts[n - 1].point,
            ends[pieces - 1],
            pts[n - 1].out_handle,
        ));
    }
    for i in 1..pieces {
        out[i * m].in_handle = ends[i - 1];
    }
    out
}

/// Wiggle Paths' numbers, as the file names them.
pub struct Wiggle {
    pub size: f64,
    pub detail: f64,
    pub per_second: f64,
    pub correlation: f64,
    pub seed: f64,
}

/// Wiggle Paths: the path split `detail` times a piece, then each point moved by smooth noise in
/// time, partly shared by every point, its handles with it.
pub fn wiggle(pts: Vec<MaskPoint>, closed: bool, w: &Wiggle, seconds: f64) -> Vec<MaskPoint> {
    let n = pts.len();
    if n == 0 || w.size == 0.0 {
        return pts;
    }
    let detail = w.detail.floor() as usize;
    let pts = if n >= 2 && detail > 0 {
        let pieces = if closed { n } else { n - 1 };
        split(pts, closed, (detail + 1).min((LIMIT / pieces).max(1)))
    } else {
        pts
    };
    let tau = seconds * w.per_second;
    let (c, seed) = (w.correlation / 100.0, w.seed.floor() as u64);
    pts.iter()
        .enumerate()
        .map(|(j, p)| {
            let by = |a: u64| {
                w.size
                    * (c * noise(seed, SHARED, a, tau) + (1.0 - c) * noise(seed, j as u64, a, tau))
            };
            point(add(p.point, (by(0), by(1))), p.in_handle, p.out_handle)
        })
        .collect()
}

/// Where the line through `p1` along `e1` meets the line through `p2` along `e2`.
fn meet(p1: P, e1: P, p2: P, e2: P) -> Option<P> {
    let den = e1.0 * e2.1 - e1.1 * e2.0;
    if den == 0.0 {
        return None;
    }
    let t = ((p2.0 - p1.0) * e2.1 - (p2.1 - p1.1) * e2.0) / den;
    Some((p1.0 + e1.0 * t, p1.1 + e1.1 * t))
}

/// Offset Paths' joins.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Join {
    Miter,
    Round,
    Bevel,
}

/// Offset Paths: the path flattened, each side moved out by the amount (in, below 0), the
/// sides meeting where they turn in and joined where they turn out. Corners only.
pub fn offset(
    pts: Vec<MaskPoint>,
    closed: bool,
    amount: f64,
    join: Join,
    limit: f64,
) -> Vec<MaskPoint> {
    if amount == 0.0 {
        return pts;
    }
    let mut f: Vec<P> = Vec::new();
    for q in crate::mask::flatten(&pts, closed) {
        if f.last().is_none_or(|l| (q.0 - l.0).hypot(q.1 - l.1) > 1e-9) {
            f.push(q);
        }
    }
    if closed && f.len() > 1 {
        let (a, b) = (f[0], f[f.len() - 1]);
        if (a.0 - b.0).hypot(a.1 - b.1) <= 1e-9 {
            f.pop();
        }
    }
    let n = f.len();
    if n < 2 {
        return pts;
    }
    let mut area = 0.0;
    for i in 0..n {
        let (a, b) = (f[i], f[(i + 1) % n]);
        area += a.0 * b.1 - b.0 * a.1;
    }
    let s = if area < 0.0 { -1.0 } else { 1.0 };
    let edge = |i: usize| {
        let e = sub(f[(i + 1) % n], f[i]);
        let length = e.0.hypot(e.1);
        (e, (s * e.1 / length, -s * e.0 / length), length)
    };
    let moved = |p: P, nrm: P| (p.0 + amount * nrm.0, p.1 + amount * nrm.1);
    let mut out = Vec::with_capacity(n * 2);
    for (i, &p) in f.iter().enumerate() {
        if !closed && i == 0 {
            out.push(moved(p, edge(0).1));
            continue;
        }
        if !closed && i == n - 1 {
            out.push(moved(p, edge(n - 2).1));
            continue;
        }
        let ((e1, n1, l1), (e2, n2, l2)) = (edge((i + n - 1) % n), edge(i));
        let (p1, p2) = (moved(p, n1), moved(p, n2));
        let cross = e1.0 * e2.1 - e1.1 * e2.0;
        let outer = if cross.abs() <= 1e-12 * l1 * l2 {
            if e1.0 * e2.0 + e1.1 * e2.1 > 0.0 {
                out.push(p1);
                continue;
            }
            true
        } else {
            cross * s * amount > 0.0
        };
        if !outer {
            out.push(meet(p1, e1, p2, e2).unwrap_or(p1));
            continue;
        }
        match join {
            Join::Miter => match meet(p1, e1, p2, e2) {
                Some(q) if (q.0 - p.0).hypot(q.1 - p.1) <= limit * amount.abs() => out.push(q),
                _ => out.extend([p1, p2]),
            },
            Join::Bevel => out.extend([p1, p2]),
            Join::Round => {
                let (a1, a2) = (n1.1.atan2(n1.0), n2.1.atan2(n2.0));
                let mut delta = a2 - a1;
                if delta > std::f64::consts::PI {
                    delta -= 2.0 * std::f64::consts::PI;
                } else if delta <= -std::f64::consts::PI {
                    delta += 2.0 * std::f64::consts::PI;
                }
                let steps = ((delta.abs() / (std::f64::consts::PI / 18.0)).ceil() as usize).max(1);
                out.push(p1);
                for k in 1..steps {
                    let ang = a1 + delta * k as f64 / steps as f64;
                    out.push((p.0 + amount * ang.cos(), p.1 + amount * ang.sin()));
                }
                out.push(p2);
            }
        }
    }
    out.into_iter()
        .map(|q| MaskPoint::corner(q.0, q.1))
        .collect()
}

/// A merge over more sides than this, all its paths' together, leaves them as they are.
pub const MOST_SIDES: usize = 20_000;

/// Whether a point is inside outlines by the even-odd rule, `mask::point_inside`'s ray.
fn inside(outlines: &[Vec<P>], (x, y): P) -> bool {
    let mut k = false;
    for c in outlines {
        let n = c.len();
        for i in 0..n {
            let ((x0, y0), (x1, y1)) = (c[i], c[(i + 1) % n]);
            if (y0 <= y) != (y1 <= y) {
                let t = (y - y0) / (y1 - y0);
                if x < x0 + t * (x1 - x0) {
                    k = !k;
                }
            }
        }
    }
    k
}

/// A point as a map key, -0 and 0 the same as they compare.
fn key(p: P) -> (u64, u64) {
    ((p.0 + 0.0).to_bits(), (p.1 + 0.0).to_bits())
}

/// Merge Paths' add, subtract, intersect and exclude: the outlines of the area in any of
/// `shapes`, in the first and none of the rest, in all, or in an odd number, each shape's paths
/// flattened (D-77), an open one closed straight, its area by the even-odd rule. Corners only,
/// each closed, the area on each side's (-dy, dx) side; `None` over [`MOST_SIDES`] sides.
/// Worked exactly as `tools/shape_merge_reference.py` writes it down.
pub fn merge(shapes: &[&[(Vec<MaskPoint>, bool)]], mode: &str) -> Option<Vec<Vec<P>>> {
    const EPS: f64 = 1e-9;
    const D: f64 = 1e-6;
    const BOX: f64 = 1e-6;
    let regions: Vec<Vec<Vec<P>>> = shapes
        .iter()
        .map(|s| {
            s.iter()
                .map(|(pts, closed)| crate::mask::flatten(pts, *closed))
                .collect()
        })
        .collect();
    let mut segs: Vec<(P, P)> = Vec::new();
    for c in regions.iter().flatten() {
        for i in 0..c.len() {
            let (a, b) = (c[i], c[(i + 1) % c.len()]);
            if a != b {
                segs.push((a, b));
            }
        }
    }
    if segs.len() > MOST_SIDES {
        return None;
    }
    let member = |p: P| {
        let ins: Vec<bool> = regions.iter().map(|r| inside(r, p)).collect();
        match mode {
            "add" => ins.iter().any(|&k| k),
            "subtract" => ins.first() == Some(&true) && !ins[1..].iter().any(|&k| k),
            "intersect" => ins.iter().all(|&k| k),
            _ => ins.iter().filter(|&&k| k).count() % 2 == 1,
        }
    };
    let within = |f: f64| EPS < f && f < 1.0 - EPS;
    let mut cuts: Vec<Vec<(f64, P)>> = segs
        .iter()
        .map(|&(a, b)| vec![(0.0, a), (1.0, b)])
        .collect();
    let boxes: Vec<[f64; 4]> = segs
        .iter()
        .map(|&(a, b)| [a.0.min(b.0), a.1.min(b.1), a.0.max(b.0), a.1.max(b.1)])
        .collect();
    // ponytail: every pair of sides is asked, n² under MOST_SIDES; a sweep if merges grow.
    for s in 0..segs.len() {
        let ((px, py), (bx, by)) = segs[s];
        let (rx, ry) = (bx - px, by - py);
        for t in s + 1..segs.len() {
            let (bs, bt) = (boxes[s], boxes[t]);
            if bt[0] > bs[2] + BOX
                || bt[2] < bs[0] - BOX
                || bt[1] > bs[3] + BOX
                || bt[3] < bs[1] - BOX
            {
                continue;
            }
            let ((qx, qy), (dx, dy)) = segs[t];
            let (ux, uy) = (dx - qx, dy - qy);
            let (wx, wy) = (qx - px, qy - py);
            let den = rx * uy - ry * ux;
            if den != 0.0 {
                let ts = (wx * uy - wy * ux) / den;
                let tu = (wx * ry - wy * rx) / den;
                let end = |f: f64| f.abs() <= EPS || (f - 1.0).abs() <= EPS;
                if within(ts) && within(tu) {
                    let p = (px + ts * rx, py + ts * ry);
                    cuts[s].push((ts, p));
                    cuts[t].push((tu, p));
                } else if within(ts) && end(tu) {
                    let e = if tu.abs() <= EPS {
                        segs[t].0
                    } else {
                        segs[t].1
                    };
                    cuts[s].push((ts, e));
                } else if within(tu) && end(ts) {
                    let e = if ts.abs() <= EPS {
                        segs[s].0
                    } else {
                        segs[s].1
                    };
                    cuts[t].push((tu, e));
                }
                continue;
            }
            let (rr, uu) = (rx * rx + ry * ry, ux * ux + uy * uy);
            if (wx * ry - wy * rx).abs() > 1e-9 * rr.sqrt() {
                continue;
            }
            for e in [segs[t].0, segs[t].1] {
                let f = ((e.0 - px) * rx + (e.1 - py) * ry) / rr;
                if within(f) {
                    cuts[s].push((f, e));
                }
            }
            for e in [segs[s].0, segs[s].1] {
                let f = ((e.0 - qx) * ux + (e.1 - qy) * uy) / uu;
                if within(f) {
                    cuts[t].push((f, e));
                }
            }
        }
    }
    let mut kept: Vec<(P, P)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for mut cs in cuts {
        cs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        for w in cs.windows(2) {
            let (a, b) = (w[0].1, w[1].1);
            if a == b {
                continue;
            }
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let length = dx.hypot(dy);
            let (nx, ny) = (-dy / length, dx / length);
            let (mx, my) = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
            let left = member((mx + D * nx, my + D * ny));
            if left == member((mx - D * nx, my - D * ny)) {
                continue;
            }
            let e = if left { (a, b) } else { (b, a) };
            if seen.insert((key(e.0), key(e.1))) {
                kept.push(e);
            }
        }
    }
    let mut starts: std::collections::HashMap<(u64, u64), Vec<usize>> = Default::default();
    for (i, e) in kept.iter().enumerate() {
        starts.entry(key(e.0)).or_default().push(i);
    }
    let mut used = vec![false; kept.len()];
    let mut out = Vec::new();
    for i in 0..kept.len() {
        if std::mem::replace(&mut used[i], true) {
            continue;
        }
        let (a, mut end) = kept[i];
        let mut c = vec![a];
        while end != a {
            let next = starts
                .get(&key(end))
                .and_then(|js| js.iter().copied().find(|&j| !used[j]));
            c.push(end);
            let Some(j) = next else { break };
            used[j] = true;
            end = kept[j].1;
        }
        if c.len() >= 3 {
            out.push(c);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn splitmix64_of_0_is_the_published_number() {
        assert_eq!(super::mix(0), 0xE220_A839_7B1D_CDAF);
    }
}
