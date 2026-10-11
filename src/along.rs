//! D-356 (`docs/effects/EFFECTS.md` P0-22): effects that draw along a path.
//!
//! A path here is an outline as `mask::flatten` or `shape::flatten` gives it, with whether it is
//! closed (D-357: a shape's path may be open), in the coordinates it is drawn in. [`runs`] measures the paths along their length, trims them to a start and an end, and
//! lays dabs on them a step apart; [`distance`] finds the nearest dab of one run. Path Stroke
//! (`core.stroke`) is the first effect drawn with them; Vegas, Scribble, Write-on, Audio Waveform
//! and Energy Stroke (D-508) followed.

use rayon::prelude::*;

use crate::WorkingBuffer;

/// The dabs on one straight piece of a path: the first dab's centre, the last's, and the number
/// of steps between them, 0 for one dab. A negative count is a brush laid all along, from the
/// first point to the last.
pub(crate) type Run = [f64; 5];

/// The runs of a brush laid along `paths`, outlines each with whether it is closed, between `start` and `end` per cent
/// of each one's length (taken smaller first), a dab every `step` pixels from each path's first
/// point, so the dabs stay put as the start moves; `step` 0 lays the brush all along. With
/// `sequential`, the paths are one length, end to end in order, and `start` and `end` are per
/// cent of it. Document 21's Path Stroke paragraph is the rule.
pub(crate) fn runs(paths: &[(Vec<(f64, f64)>, bool)], start: f64, end: f64, step: f64, sequential: bool) -> Vec<Run> {
    let (s, e) = (start.min(end) / 100.0, start.max(end) / 100.0);
    let pieces: Vec<Vec<((f64, f64), (f64, f64))>> = paths.iter().map(|(p, closed)| crate::shape::path_segments(p, *closed)).collect();
    let length = |(a, b): &((f64, f64), (f64, f64))| (b.0 - a.0).hypot(b.1 - a.1);
    let lengths: Vec<f64> = pieces.iter().map(|p| p.iter().map(length).sum()).collect();
    let total: f64 = lengths.iter().sum();
    let (mut before, mut out) = (0.0, Vec::new());
    for (pieces, &l) in pieces.iter().zip(&lengths) {
        let (lo, hi) = if sequential { ((s * total - before).max(0.0), (e * total - before).min(l)) } else { (s * l, e * l) };
        before += l;
        if lo >= hi {
            continue;
        }
        let mut at = 0.0;
        for piece in pieces {
            let n = length(piece);
            let (u0, u1) = (lo.max(at), hi.min(at + n));
            let (a, b) = *piece;
            let point = |u: f64| {
                let f = (u - at) / n;
                (a.0 + f * (b.0 - a.0), a.1 + f * (b.1 - a.1))
            };
            if n > 0.0 && u0 <= u1 {
                if step == 0.0 {
                    if u0 < u1 {
                        let (p, q) = (point(u0), point(u1));
                        out.push([p.0, p.1, q.0, q.1, -1.0]);
                    }
                } else {
                    let (k0, k1) = ((u0 / step - 1e-12).ceil(), (u1 / step + 1e-12).floor());
                    if k0 <= k1 {
                        let (p, q) = (point(k0 * step), point(k1 * step));
                        out.push([p.0, p.1, q.0, q.1, k1 - k0]);
                    }
                }
            }
            at += n;
        }
    }
    out
}

/// Path Stroke's runs: `paths` trimmed to `start` and `end` with a dab every `spacing` per cent
/// of the brush `size` (0 for all along), measured where the paths are and moved by `origin`,
/// the drawing's corner in a buffer grown by an effect above. The processor and the card share it.
pub(crate) fn stroke_runs(paths: &[(Vec<(f64, f64)>, bool)], (ox, oy): (usize, usize), [start, end, spacing, size]: [f64; 4], sequential: bool) -> Vec<Run> {
    let mut out = runs(paths, start, end, spacing / 100.0 * size, sequential);
    for run in &mut out {
        run[0] += ox as f64;
        run[1] += oy as f64;
        run[2] += ox as f64;
        run[3] += oy as f64;
    }
    out
}

/// The distance from (`x`, `y`) to the nearest dab of `run`: the one nearest along it, which on
/// a straight piece is the nearest of all. Ties are the same distance, whichever is taken.
pub(crate) fn distance(run: &Run, x: f64, y: f64) -> f64 {
    let [px, py, qx, qy, n] = *run;
    let (dx, dy) = (qx - px, qy - py);
    let l2 = dx * dx + dy * dy;
    let mut t = if l2 == 0.0 { 0.0 } else { (((x - px) * dx + (y - py) * dy) / l2).clamp(0.0, 1.0) };
    if n > 0.0 {
        t = (t * n + 0.5).floor() / n;
    }
    (x - px - t * dx).hypot(y - py - t * dy)
}

/// D-420: the way across `(dx, dy)` to its left on the screen, `(dy, -dx)` at length one: up
/// for a way drawn left to right, and up when it has no length.
pub(crate) fn facing(dx: f64, dy: f64) -> (f64, f64) {
    let length = dx.hypot(dy);
    if length == 0.0 {
        (0.0, -1.0)
    } else {
        (dy / length, -dx / length)
    }
}

/// D-420: the point a share `u` of the way round the closed outline `o` (0 its first point, 1
/// back to it), and the facing of the piece it falls on.
pub(crate) fn point_along(o: &[(f64, f64)], u: f64) -> ((f64, f64), (f64, f64)) {
    if o.is_empty() {
        return ((0.0, 0.0), (0.0, -1.0));
    }
    let piece = |i: usize| (o[i], o[(i + 1) % o.len()]);
    let length = |(a, b): ((f64, f64), (f64, f64))| (b.0 - a.0).hypot(b.1 - a.1);
    let total: f64 = (0..o.len()).map(|i| length(piece(i))).sum();
    if total == 0.0 {
        return (o[0], (0.0, -1.0));
    }
    let (s, mut walked) = (u * total, 0.0);
    for i in 0..o.len() {
        let (a, b) = piece(i);
        let l = length((a, b));
        if l > 0.0 && walked + l >= s {
            let v = (s - walked) / l;
            return ((a.0 + v * (b.0 - a.0), a.1 + v * (b.1 - a.1)), facing(b.0 - a.0, b.1 - a.1));
        }
        walked += l;
    }
    let (a, b) = (0..o.len()).rev().map(piece).find(|(a, b)| a != b).unwrap_or(piece(0));
    (b, facing(b.0 - a.0, b.1 - a.1))
}

/// A run's box, outside which it is further than `reach` from every point: left, right, top,
/// bottom.
pub(crate) fn run_box(run: &Run, reach: f64) -> [f64; 4] {
    [run[0].min(run[2]) - reach, run[0].max(run[2]) + reach, run[1].min(run[3]) - reach, run[1].max(run[3]) + reach]
}

/// D-356: Path Stroke. A round brush `size` pixels across laid on `runs` (in the buffer's
/// coordinates), `hardness` and `opacity` 0 to 100, `color` linear, `style` "on_original",
/// "on_transparent" or "reveal". The covering at a pixel's centre is a smoothstep of the
/// distance to the nearest dab, soft over the brush's soft edge, smoothed over one pixel when
/// hard; dabs do not build up. The buffer never grows.
pub(crate) fn path_stroke(buffer: &mut WorkingBuffer, runs: &[Run], [size, hardness, opacity]: [f64; 3], color: [f64; 3], style: &str) {
    let w = buffer.width();
    let r = size / 2.0;
    let soft = (r * (1.0 - hardness / 100.0)).max(1.0);
    let reach = r + 0.5;
    let runs: Vec<(Run, [f64; 4])> = if size == 0.0 || opacity == 0.0 { Vec::new() } else { runs.iter().map(|run| (*run, run_box(run, reach))).collect() };
    let every = style != "on_original";
    buffer.data_mut().par_chunks_exact_mut(4 * w).enumerate().for_each(|(j, row)| {
        let y = j as f64 + 0.5;
        let mut near = vec![f64::INFINITY; w];
        for (run, [left, right, top, bottom]) in &runs {
            if y <= *top || y >= *bottom {
                continue;
            }
            let x0 = (left - 0.5).ceil().max(0.0) as usize;
            let x1 = ((right - 0.5).floor().max(-1.0) + 1.0).min(w as f64) as usize;
            for (i, d) in near.iter_mut().enumerate().take(x1).skip(x0) {
                *d = d.min(distance(run, i as f64 + 0.5, y));
            }
        }
        for (px, d) in row.chunks_exact_mut(4).zip(&near) {
            if !every && *d >= reach {
                continue;
            }
            let t = ((r + 0.5 - d) / soft).clamp(0.0, 1.0);
            lay(px, opacity / 100.0 * t * t * (3.0 - 2.0 * t), color, style);
        }
    });
}

/// A brush covering `c` laid on one pixel by Path Stroke's paint styles.
fn lay(px: &mut [f32], c: f64, color: [f64; 3], style: &str) {
    match style {
        "on_original" => {
            for ch in 0..3 {
                px[ch] = (px[ch] as f64 * (1.0 - c) + color[ch] * c) as f32;
            }
            px[3] = (px[3] as f64 * (1.0 - c) + c) as f32;
        }
        "on_transparent" => {
            for ch in 0..3 {
                px[ch] = (color[ch] * c) as f32;
            }
            px[3] = c as f32;
        }
        _ => {
            for v in px.iter_mut() {
                *v = (*v as f64 * c) as f32;
            }
        }
    }
}

/// D-442: Scribble's runs in a buffer `size` pixels across with the drawing's corner at
/// `origin`: lines across the masks compose chose, joined into open paths and trimmed to Start
/// and End, laid all along. Document 21's Scribble paragraph is the rule; an effect that is not a
/// Scribble with masks has none.
pub(crate) fn scribble_runs(effect: &crate::effects::Effect, (bw, bh): (usize, usize), (ox, oy): (usize, usize)) -> Vec<Run> {
    let crate::effects::Effect::Scribble { scribble, fill_type, edge_width, angle, start, end, fill_paths_sequentially, masks: Some(masks), .. } = effect else {
        return Vec::new();
    };
    let (s, c) = angle.to_radians().sin_cos();
    let uv = |x: f64, y: f64| (x * c - y * s, x * s + y * c);
    let bw = bw as f64;
    let bh = bh as f64;
    let frame = [uv(0.0, 0.0), uv(bw, 0.0), uv(bw, bh), uv(0.0, bh)];
    let chosen: Vec<Mask> = masks.iter().map(|(o, mode, inv)| (o.iter().map(|&(x, y)| uv(x + ox as f64, y + oy as f64)).collect(), *mode, *inv)).collect();
    let using = scribble == "all_masks_using_modes";
    let groups: Vec<Vec<Mask>> = match scribble.as_str() {
        "all_masks" => chosen.into_iter().map(|m| vec![m]).collect(),
        "all_masks_using_modes" => vec![chosen.into_iter().filter(|m| m.1 != crate::mask::MaskMode::None).collect()],
        _ => vec![chosen],
    };
    let back = |(u, v): (f64, f64)| (u * c + v * s, -u * s + v * c);
    let lines: Vec<Vec<Vec<(f64, f64)>>> = groups
        .iter()
        .enumerate()
        .map(|(g, group)| group_lines(effect, g as u32, group, &frame, using, fill_type, *edge_width).into_iter().map(|p| p.into_iter().map(back).collect()).collect())
        .collect();
    let open = |paths: Vec<Vec<(f64, f64)>>| -> Vec<(Vec<(f64, f64)>, bool)> { paths.into_iter().map(|p| (p, false)).collect() };
    if fill_paths_sequentially == "on" && scribble == "all_masks" {
        runs(&open(lines.concat()), *start, *end, 0.0, true)
    } else {
        lines.into_iter().flat_map(|group| runs(&open(group), *start, *end, 0.0, true)).collect()
    }
}

/// A mask as Scribble reads it: its outline (turned so its lines run along u), mode and
/// whether it is inverted.
type Mask = (Vec<(f64, f64)>, crate::mask::MaskMode, bool);

/// Scribble's 32-bit mix.
fn mix(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^ (x >> 16)
}

/// A number from 0 to 1 for (seed, slot, kind, group, line, end).
fn hashed(seed: u32, slot: i64, rest: [u32; 4]) -> f64 {
    let mut h = mix(seed);
    for v in std::iter::once(slot as u32).chain(rest) {
        h = mix(h ^ v);
    }
    h as f64 / 4294967296.0
}

/// The even-odd inside of an outline on the line at `v`, as intervals of u.
fn crossings(poly: &[(f64, f64)], v: f64) -> Vec<(f64, f64)> {
    let mut xs: Vec<f64> = Vec::new();
    for (k, &(u0, v0)) in poly.iter().enumerate() {
        let (u1, v1) = poly[(k + 1) % poly.len()];
        if (v0 <= v) != (v1 <= v) {
            xs.push(u0 + (v - v0) / (v1 - v0) * (u1 - u0));
        }
    }
    xs.sort_by(f64::total_cmp);
    xs.chunks_exact(2).filter(|p| p[1] > p[0]).map(|p| (p[0], p[1])).collect()
}

/// Where the line at `v` crosses the points within `r` of the piece a-b.
fn capsule(a: (f64, f64), b: (f64, f64), r: f64, v: f64) -> Option<(f64, f64)> {
    let mut found: Vec<f64> = Vec::new();
    for (pu, pv) in [a, b] {
        let dv = v - pv;
        if dv * dv <= r * r {
            let s = (r * r - dv * dv).sqrt();
            found.extend([pu - s, pu + s]);
        }
    }
    let (du, dw) = (b.0 - a.0, b.1 - a.1);
    let length = du.hypot(dw);
    if length > 0.0 {
        let (nu, nv) = (-dw / length * r, du / length * r);
        let quad = [(a.0 + nu, a.1 + nv), (b.0 + nu, b.1 + nv), (b.0 - nu, b.1 - nv), (a.0 - nu, a.1 - nv)];
        for k in 0..4 {
            let ((q0u, q0v), (q1u, q1v)) = (quad[k], quad[(k + 1) % 4]);
            if (q0v - v) * (q1v - v) <= 0.0 && q0v != q1v {
                found.push(q0u + (v - q0v) / (q1v - q0v) * (q1u - q0u));
            }
        }
    }
    (!found.is_empty()).then(|| (found.iter().copied().fold(f64::INFINITY, f64::min), found.iter().copied().fold(f64::NEG_INFINITY, f64::max)))
}

/// The points within `r` of an outline on the line at `v`.
fn band(poly: &[(f64, f64)], r: f64, v: f64) -> Vec<(f64, f64)> {
    let mut pieces: Vec<(f64, f64)> = (0..poly.len()).filter_map(|k| capsule(poly[k], poly[(k + 1) % poly.len()], r, v)).filter(|c| c.1 > c.0).collect();
    pieces.sort_by(|p, q| p.0.total_cmp(&q.0).then(p.1.total_cmp(&q.1)));
    let mut out: Vec<(f64, f64)> = Vec::new();
    for (lo, hi) in pieces {
        match out.last_mut() {
            Some(last) if lo <= last.1 => last.1 = last.1.max(hi),
            _ => out.push((lo, hi)),
        }
    }
    out
}

/// The intervals where `op(in a, in b)` holds.
fn combine(a: &[(f64, f64)], b: &[(f64, f64)], op: fn(bool, bool) -> bool) -> Vec<(f64, f64)> {
    let mut xs: Vec<f64> = a.iter().chain(b).flat_map(|&(lo, hi)| [lo, hi]).collect();
    xs.sort_by(f64::total_cmp);
    xs.dedup();
    let inside = |set: &[(f64, f64)], m: f64| set.iter().any(|&(lo, hi)| lo < m && m < hi);
    let mut out: Vec<(f64, f64)> = Vec::new();
    for x in xs.windows(2) {
        let m = (x[0] + x[1]) / 2.0;
        if op(inside(a, m), inside(b, m)) {
            match out.last_mut() {
                Some(last) if last.1 == x[0] => last.1 = x[1],
                _ => out.push((x[0], x[1])),
            }
        }
    }
    out
}

/// One mask's own region on the line at `v`, by the fill type.
fn region(poly: &[(f64, f64)], kind: &str, w: f64, v: f64) -> Vec<(f64, f64)> {
    let inside = crossings(poly, v);
    let clockwise = || (0..poly.len()).map(|k| poly[k].0 * poly[(k + 1) % poly.len()].1 - poly[(k + 1) % poly.len()].0 * poly[k].1).sum::<f64>() >= 0.0;
    let inner = match kind {
        "inside" => return inside,
        "centered_edge" => return band(poly, w / 2.0, v),
        "inside_edge" => true,
        "outside_edge" => false,
        side => (side == "right_edge") == clockwise(),
    };
    let op: fn(bool, bool) -> bool = if inner { |p, q| p && q } else { |p, q| p && !q };
    combine(&band(poly, w, v), &inside, op)
}

/// A group's open lines in u and v, `g` its place.
fn group_lines(effect: &crate::effects::Effect, g: u32, group: &[Mask], frame: &[(f64, f64)], using: bool, kind: &str, w: f64) -> Vec<Vec<(f64, f64)>> {
    use crate::mask::MaskMode;
    let crate::effects::Effect::Scribble {
        curviness, curviness_variation, spacing, spacing_variation, path_overlap, path_overlap_variation, wiggle_type, wiggles_per_second, random_seed, time, ..
    } = effect
    else {
        return Vec::new();
    };
    let seed = random_seed.floor() as u32;
    let wiggled = |kind: u32, i: u32, j: u32| {
        let r = |slot: i64| hashed(seed, slot, [kind, g, i, j]);
        let f = time * wiggles_per_second;
        match wiggle_type.as_str() {
            _ if *wiggles_per_second == 0.0 => r(0),
            "static" => r(0),
            "jumpy" => r((f + 1e-9).floor() as i64),
            _ => {
                let s = f.floor();
                let q = f - s;
                let e = q * q * (3.0 - 2.0 * q);
                r(s as i64) * (1.0 - e) + r(s as i64 + 1) * e
            }
        }
    };
    let varied = |base: f64, spread: f64, kind: u32, i: u32, j: u32| base + spread * (2.0 * wiggled(kind, i, j) - 1.0);
    let gap = |i: u32| varied(*spacing, *spacing_variation, 1, i, 0).max(0.5);
    let edge = if kind == "inside" { 0.0 } else { w };
    let vs = group.iter().flat_map(|m| m.0.iter().map(|p| p.1));
    let (mut lo, mut hi) = vs.fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| (lo.min(v), hi.max(v)));
    if group.iter().all(|m| m.0.is_empty()) {
        return Vec::new();
    }
    lo -= edge;
    hi += edge;
    if using && (group.iter().any(|m| m.2) || matches!(group[0].1, MaskMode::Subtract | MaskMode::Intersect)) {
        for p in frame {
            lo = lo.min(p.1);
            hi = hi.max(p.1);
        }
    }
    let line = |v: f64| -> Vec<(f64, f64)> {
        if !using {
            return region(&group[0].0, kind, w, v);
        }
        let whole = crossings(frame, v);
        let mut acc: Option<Vec<(f64, f64)>> = None;
        for (poly, mode, inverted) in group {
            let mut r = region(poly, kind, w, v);
            if *inverted {
                r = combine(&whole, &r, |p, q| p && !q);
            }
            let into = acc.get_or_insert_with(|| if matches!(mode, MaskMode::Subtract | MaskMode::Intersect) { whole.clone() } else { Vec::new() });
            let op: fn(bool, bool) -> bool = match mode {
                MaskMode::Subtract => |p, q| p && !q,
                MaskMode::Intersect => |p, q| p && q,
                MaskMode::Difference => |p, q| p != q,
                _ => |p, q| p || q,
            };
            *into = combine(into, &r, op);
        }
        acc.unwrap_or_default()
    };
    let (mut out, mut cur, mut last): (Vec<Vec<(f64, f64)>>, Option<Vec<(f64, f64)>>, Option<u32>) = (Vec::new(), None, None);
    let mut i = 0u32;
    let mut v = lo + gap(0) / 2.0;
    // ponytail: at most 100000 lines a group, ample for any spacing over any buffer this program
    // draws; a cap only so a mask far off the frame cannot hang a frame.
    while v <= hi && i < 100_000 {
        let mut spans: Vec<(f64, f64)> = Vec::new();
        for (k, (a, b)) in line(v).into_iter().enumerate() {
            let k = k as u32;
            let a = a - varied(*path_overlap, *path_overlap_variation, 2, i, 2 * k);
            let b = b + varied(*path_overlap, *path_overlap_variation, 2, i, 2 * k + 1);
            if b > a {
                spans.push((a, b));
            }
        }
        if i % 2 == 1 {
            spans = spans.into_iter().rev().map(|(a, b)| (b, a)).collect();
        }
        for (k, &(a, b)) in spans.iter().enumerate() {
            match cur.as_mut() {
                Some(path) if k == 0 && last == Some(i.wrapping_sub(1)) && i > 0 => {
                    let p0 = *path.last().unwrap_or(&(a, v));
                    let p3 = (a, v);
                    let t = if (i - 1) % 2 == 0 { 1.0 } else { -1.0 };
                    let cv = varied(*curviness, *curviness_variation, 3, i - 1, 0).max(0.0);
                    let h = cv / 100.0 * (2.0 / 3.0) * (p3.0 - p0.0).hypot(p3.1 - p0.1);
                    let (p1, p2) = ((p0.0 + h * t, p0.1), (p3.0 + h * t, p3.1));
                    for m in 1..=8 {
                        let q = m as f64 / 8.0;
                        let mq = 1.0 - q;
                        let (b0, b1, b2, b3) = (mq * mq * mq, 3.0 * mq * mq * q, 3.0 * mq * q * q, q * q * q);
                        path.push((b0 * p0.0 + b1 * p1.0 + b2 * p2.0 + b3 * p3.0, b0 * p0.1 + b1 * p1.1 + b2 * p2.1 + b3 * p3.1));
                    }
                    path.push((b, v));
                }
                _ => {
                    out.extend(cur.take());
                    cur = Some(vec![(a, v), (b, v)]);
                }
            }
        }
        if !spans.is_empty() {
            last = Some(i);
        }
        i += 1;
        v += gap(i);
    }
    out.extend(cur);
    out
}

/// D-441: Write-on. Path Stroke's round brush laid once at each of `marks`, [x, y (in the
/// buffer's coordinates), size, hardness, opacity], the mark covering most winning at each pixel
/// (marks do not build up); `color` linear, `style` as Path Stroke's. The buffer never grows.
pub(crate) fn write_on(buffer: &mut WorkingBuffer, marks: &[[f64; 5]], color: [f64; 3], style: &str) {
    let w = buffer.width();
    // Each mark as its centre, radius, soft edge and covering at most.
    let marks: Vec<[f64; 5]> = marks
        .iter()
        .filter(|m| m[2] != 0.0 && m[4] != 0.0)
        .map(|&[x, y, size, hardness, opacity]| {
            let r = size / 2.0;
            [x, y, r, (r * (1.0 - hardness / 100.0)).max(1.0), opacity / 100.0]
        })
        .collect();
    let every = style != "on_original";
    buffer.data_mut().par_chunks_exact_mut(4 * w).enumerate().for_each(|(j, row)| {
        let y = j as f64 + 0.5;
        let mut cover = vec![0.0f64; w];
        for &[mx, my, r, soft, opacity] in &marks {
            let reach = r + 0.5;
            if y <= my - reach || y >= my + reach {
                continue;
            }
            let x0 = (mx - reach - 0.5).ceil().max(0.0) as usize;
            let x1 = ((mx + reach - 0.5).floor().max(-1.0) + 1.0).min(w as f64) as usize;
            for (i, c) in cover.iter_mut().enumerate().take(x1).skip(x0) {
                let t = ((reach - (i as f64 + 0.5 - mx).hypot(y - my)) / soft).clamp(0.0, 1.0);
                *c = c.max(opacity * t * t * (3.0 - 2.0 * t));
            }
        }
        for (px, &c) in row.chunks_exact_mut(4).zip(&cover) {
            if every || c > 0.0 {
                lay(px, c, color, style);
            }
        }
    });
}

/// One straight bit of a Vegas dash: its ends P and Q (in the buffer's coordinates) and their
/// places along the dash, 0 its start and 1 its end.
pub(crate) type Bit = [f64; 6];

/// D-444: Vegas's dashes on `paths`, moved by `origin` as [`stroke_runs`] moves its runs:
/// `segments` (its floor) dashes on each path, each `length` of the path over `segments` long,
/// begun `rotation` degrees round (and with `random`, a phase of its own from `seed`), `even`
/// spread round the path or else end to end. A dash runs on past a path's end from its start.
/// Document 21's Vegas paragraph is the rule. The processor and the card share it.
pub(crate) fn vegas_bits(paths: &[(Vec<(f64, f64)>, bool)], (ox, oy): (usize, usize), [segments, length, rotation, seed]: [f64; 4], even: bool, random: bool) -> Vec<Bit> {
    let count = segments.floor();
    let mut out = Vec::new();
    for (c, (p, closed)) in paths.iter().enumerate() {
        let pieces = crate::shape::path_segments(p, *closed);
        let size = |(a, b): &((f64, f64), (f64, f64))| (b.0 - a.0).hypot(b.1 - a.1);
        let total: f64 = pieces.iter().map(size).sum();
        let l = length * total / count;
        if total <= 0.0 || l <= 0.0 {
            continue;
        }
        let phase = if random { (crate::grade::mix(crate::grade::mix(seed.floor() as u64) ^ c as u64) >> 11) as f64 / 2f64.powi(53) } else { 0.0 };
        let o = (rotation / 360.0 + phase) * total;
        for i in 0..count as usize {
            let a = (o + i as f64 * if even { total / count } else { l }).rem_euclid(total);
            let mut at = 0.0;
            for piece in &pieces {
                let n = size(piece);
                let ((ax, ay), (bx, by)) = *piece;
                for k in [0.0, 1.0] {
                    let lo = at + k * total;
                    let (u0, u1) = (a.max(lo), (a + l).min(lo + n));
                    if n > 0.0 && u0 < u1 {
                        let point = |u: f64| (ax + (u - lo) / n * (bx - ax) + ox as f64, ay + (u - lo) / n * (by - ay) + oy as f64);
                        let ((px, py), (qx, qy)) = (point(u0), point(u1));
                        out.push([px, py, qx, qy, (u0 - a) / l, (u1 - a) / l]);
                    }
                }
                at += n;
            }
        }
    }
    out
}

/// D-444: the opacity a share `s` along a Vegas dash: `start` at 0 to `mid` at `position`, then
/// to `end` at 1, in straight lines.
pub(crate) fn vegas_along(s: f64, [start, mid, end, position]: [f64; 4]) -> f64 {
    if s < position {
        start + (mid - start) * s / position
    } else if position < 1.0 {
        mid + (end - mid) * (s - position) / (1.0 - position)
    } else {
        mid
    }
}

/// D-444: Vegas's covering of a pixel centred at (`x`, `y`) by `bit`: its brush (half-width `r`,
/// soft over `soft`) times the opacity along the dash where the bit is nearest.
pub(crate) fn vegas_cover(bit: &Bit, x: f64, y: f64, r: f64, soft: f64, fade: [f64; 4]) -> f64 {
    let [px, py, qx, qy, sp, sq] = *bit;
    let (dx, dy) = (qx - px, qy - py);
    let l2 = dx * dx + dy * dy;
    let t = if l2 == 0.0 { 0.0 } else { (((x - px) * dx + (y - py) * dy) / l2).clamp(0.0, 1.0) };
    let d = (x - px - t * dx).hypot(y - py - t * dy);
    let v = ((r + 0.5 - d) / soft).clamp(0.0, 1.0);
    v * v * (3.0 - 2.0 * v) * vegas_along(sp + t * (sq - sp), fade)
}

/// D-444: Vegas. `bits` from [`vegas_bits`]; `width` pixels across, `hardness` 0 to 1; `fade`
/// the start, mid-point (already clamped) and end opacities and the mid-point's place; `color`
/// linear; `mode` "over", "under", "transparent" or "stencil". The covering is the largest any
/// bit gives; dashes do not build up. The buffer never grows.
pub(crate) fn vegas(buffer: &mut WorkingBuffer, bits: &[Bit], [width, hardness]: [f64; 2], fade: [f64; 4], color: [f64; 3], mode: &str) {
    let w = buffer.width();
    let r = width / 2.0;
    let soft = (r * (1.0 - hardness)).max(1.0);
    let reach = r + 0.5;
    let boxed: Vec<(Bit, [f64; 4])> = if width == 0.0 { Vec::new() } else { bits.iter().map(|b| (*b, run_box(&[b[0], b[1], b[2], b[3], 0.0], reach))).collect() };
    buffer.data_mut().par_chunks_exact_mut(4 * w).enumerate().for_each(|(j, row)| {
        let y = j as f64 + 0.5;
        let mut cover = vec![0.0f64; w];
        for (bit, [left, right, top, bottom]) in &boxed {
            if y <= *top || y >= *bottom {
                continue;
            }
            let x0 = (left - 0.5).ceil().max(0.0) as usize;
            let x1 = ((right - 0.5).floor().max(-1.0) + 1.0).min(w as f64) as usize;
            for (i, c) in cover.iter_mut().enumerate().take(x1).skip(x0) {
                *c = c.max(vegas_cover(bit, i as f64 + 0.5, y, r, soft, fade));
            }
        }
        for (px, &c) in row.chunks_exact_mut(4).zip(&cover) {
            if c == 0.0 && (mode == "over" || mode == "under") {
                continue;
            }
            let a = px[3] as f64;
            match mode {
                "over" => {
                    for ch in 0..3 {
                        px[ch] = (px[ch] as f64 * (1.0 - c) + color[ch] * c) as f32;
                    }
                    px[3] = (a * (1.0 - c) + c) as f32;
                }
                "under" => {
                    for ch in 0..3 {
                        px[ch] = (px[ch] as f64 + (1.0 - a) * color[ch] * c) as f32;
                    }
                    px[3] = (a + (1.0 - a) * c) as f32;
                }
                "transparent" => {
                    for ch in 0..3 {
                        px[ch] = (color[ch] * c) as f32;
                    }
                    px[3] = c as f32;
                }
                _ => {
                    for v in px.iter_mut() {
                        *v = (*v as f64 * c) as f32;
                    }
                }
            }
        }
    });
}

/// D-508: value noise for Energy Stroke: on the whole-number grid each corner (i, j) has
/// [`hashed`]`(seed, slot, [channel, i, j, 0])`, i taken modulo `period` when there is one;
/// between corners each axis is eased by smoothstep and mixed.
fn noise(seed: u32, slot: i64, channel: u32, x: f64, y: f64, period: Option<i64>) -> f64 {
    let (i, j) = (x.floor(), y.floor());
    let (fx, fy) = (x - i, y - j);
    let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let (i, j) = (i as i64, j as i64);
    let v = |a: i64, b: i64| hashed(seed, slot, [channel, period.map_or(a, |p| a.rem_euclid(p)) as u32, b as u32, 0]);
    let top = v(i, j) + (v(i + 1, j) - v(i, j)) * sx;
    let low = v(i, j + 1) + (v(i + 1, j + 1) - v(i, j + 1)) * sx;
    top + (low - top) * sy
}

/// D-508: every straight bit of Energy Stroke's beam in a buffer `size` pixels across with the
/// drawing's corner at `origin`: [ax, ay, bx, by, width at a, width at b], the widths shares of
/// the core and glow, each with its box (as [`run_box`]'s) outside which it does not reach, `k`
/// being [`energy_numbers`]; a bit that reaches nowhere is left out. Document 21's Energy Stroke
/// paragraph is the rule; the processor and the card share it. An effect that is not an Energy
/// Stroke, or has no paths, has none.
pub(crate) fn energy_bits(effect: &crate::effects::Effect, (bw, bh): (usize, usize), (ox, oy): (usize, usize), k: &[f64; 12]) -> Vec<([f64; 6], [f64; 4])> {
    let crate::effects::Effect::EnergyStroke {
        source,
        core_start: a,
        core_end: b,
        evolution,
        start_offset,
        end_offset,
        start_size,
        end_size,
        distortion_amount: amount,
        distortion_scale: scale,
        distortion_speed,
        random_seed,
        frame,
        paths,
        ..
    } = effect
    else {
        return Vec::new();
    };
    let (dw, dh) = ((bw - 2 * ox) as f64, (bh - 2 * oy) as f64);
    let line;
    let paths: &[(Vec<(f64, f64)>, bool)] = if source == "points" {
        line = [(vec![(a[0] / 100.0 * dw, a[1] / 100.0 * dh), (b[0] / 100.0 * dw, b[1] / 100.0 * dh)], false)];
        &line
    } else {
        paths.as_deref().unwrap_or(&[])
    };
    let (s, e) = (start_offset.min(*end_offset) / 100.0, start_offset.max(*end_offset) / 100.0);
    let seed = random_seed.floor() as u32;
    let tau = *frame as f64 * distortion_speed / 10.0;
    let step = (scale / 8.0).clamp(0.5, 2.0);
    let mut out = Vec::new();
    for (n, (outline, closed)) in paths.iter().enumerate() {
        let pieces = crate::shape::path_segments(outline, *closed);
        let mut starts = Vec::with_capacity(pieces.len());
        let mut l = 0.0;
        for (p, q) in &pieces {
            starts.push(l);
            l += (q.0 - p.0).hypot(q.1 - p.1);
        }
        let (lo, hi) = (s * l, e * l);
        if l == 0.0 || lo >= hi {
            continue;
        }
        let g0 = if *closed { (evolution / 360.0).rem_euclid(1.0) * l } else { 0.0 };
        let (from, to) = (g0 + lo, g0 + hi);
        let mut stops = vec![from, to];
        for &at in &starts {
            for c in [Some(at), closed.then_some(at + l)].into_iter().flatten() {
                if from < c && c < to {
                    stops.push(c);
                }
            }
        }
        if *amount > 0.0 {
            for q in (from / step).floor() as i64..=(to / step).ceil() as i64 {
                let g = q as f64 * step;
                if from < g && g < to {
                    stops.push(g);
                }
            }
        }
        stops.sort_by(f64::total_cmp);
        stops.dedup();
        let period = ((l / scale + 0.5).floor() as i64).max(1);
        let place = |u: f64| -> (f64, f64) {
            for ((p, q), &at) in pieces.iter().zip(&starts) {
                let n = (q.0 - p.0).hypot(q.1 - p.1);
                if n > 0.0 && u <= at + n {
                    let f = ((u - at) / n).clamp(0.0, 1.0);
                    return (p.0 + f * (q.0 - p.0), p.1 + f * (q.1 - p.1));
                }
            }
            pieces.iter().rev().find(|(p, q)| p != q).map_or(outline[0], |&(_, q)| q)
        };
        let cuts: Vec<((f64, f64), f64)> = stops
            .iter()
            .map(|&g| {
                let round = if *closed && g >= l { g - l } else { g };
                let mut p = place(round);
                if *amount > 0.0 {
                    let (x, per) = if *closed { (round * period as f64 / l, Some(period)) } else { (g / scale, None) };
                    p.0 += amount * (2.0 * noise(seed, n as i64, 1, x, tau, per) - 1.0);
                    p.1 += amount * (2.0 * noise(seed, n as i64, 2, x, tau, per) - 1.0);
                }
                (p, (start_size + (end_size - start_size) * (g - g0 - lo) / (hi - lo)) / 100.0)
            })
            .collect();
        let (fx, fy) = (ox as f64, oy as f64);
        out.extend(cuts.windows(2).filter_map(|c| {
            let bit = [c[0].0 .0 + fx, c[0].0 .1 + fy, c[1].0 .0 + fx, c[1].0 .1 + fy, c[0].1, c[1].1];
            let reach = energy_reach(k, bit[4], bit[5]);
            (reach > 0.0).then(|| (bit, run_box(&[bit[0], bit[1], bit[2], bit[3], -1.0], reach)))
        }));
    }
    out
}

/// D-508: Energy Stroke's numbers, shared with the card: the core size, its softness as a share,
/// the glow spread, the four rings' sigmas, the light per channel at full glow (the flicker times
/// the glow colour, linear, times the intensity share), the flicker and the composite (an index
/// into `effects::ENERGY_COMPOSITES`).
pub(crate) fn energy_numbers(effect: &crate::effects::Effect) -> [f64; 12] {
    let crate::effects::Effect::EnergyStroke {
        core_size, core_softness, glow_color, glow_intensity, glow_spread, flicker_amount, flicker_speed, random_seed, composite, frame, ..
    } = effect
    else {
        unreachable!("Energy Stroke's numbers of another effect")
    };
    let g = crate::effects::encoded(glow_color).map(crate::grade::to_linear);
    let i = glow_intensity / 100.0;
    let f = 1.0 - flicker_amount / 100.0 * noise(random_seed.floor() as u32, 0, 3, *frame as f64 * flicker_speed / 10.0, 0.0, None);
    let sig = |j: i32| (glow_spread / 3.0 / 2f64.powi(j)).max(0.25);
    let mode = crate::effects::ENERGY_COMPOSITES.iter().position(|c| c == composite).expect("Energy Stroke's composite was checked");
    [*core_size, core_softness / 100.0, *glow_spread, sig(0), sig(1), sig(2), sig(3), f * g[0] * i, f * g[1] * i, f * g[2] * i, f, mode as f64]
}

/// D-508: how far from a bit of widths `wa` and `wb` the beam reaches: the core's edge, and six
/// of the widest ring's sigmas for the glow, past which it is under a ten-millionth.
fn energy_reach(k: &[f64; 12], wa: f64, wb: f64) -> f64 {
    let core = k[0] * wa.max(wb) / 2.0;
    let core = if core > 0.0 { core + 0.5 } else { 0.0 };
    core.max(if k[2] > 0.0 && wa.max(wb) > 0.0 { 6.0 * k[3] } else { 0.0 })
}

/// D-508: Energy Stroke (document 21), `origin` the drawing's corner in the buffer: the core's
/// covering and the glow, each the largest over every bit, lit and laid on by the composite.
/// With no path to run along the layer is left as it is. The buffer never grows.
pub(crate) fn energy_stroke(buffer: &mut WorkingBuffer, effect: &crate::effects::Effect, origin: (usize, usize)) {
    let crate::effects::Effect::EnergyStroke { source, paths, .. } = effect else {
        return;
    };
    if source != "points" && paths.is_none() {
        return;
    }
    let w = buffer.width();
    let k = energy_numbers(effect);
    let bits = energy_bits(effect, (w, buffer.height()), origin, &k);
    let mode = k[11] as usize;
    buffer.data_mut().par_chunks_exact_mut(4 * w).enumerate().for_each(|(j, row)| {
        let y = j as f64 + 0.5;
        let (mut core, mut glow) = (vec![0.0f64; w], vec![0.0f64; w]);
        for (b, [left, right, top, bottom]) in &bits {
            if y <= *top || y >= *bottom {
                continue;
            }
            let x0 = (left - 0.5).ceil().max(0.0) as usize;
            let x1 = ((right - 0.5).floor().max(-1.0) + 1.0).min(w as f64) as usize;
            for i in x0..x1 {
                let (c, g) = energy_shade(&k, b, i as f64 + 0.5, y);
                core[i] = core[i].max(c);
                glow[i] = glow[i].max(g);
            }
        }
        for (i, px) in row.chunks_exact_mut(4).enumerate() {
            let (c, g) = (core[i], glow[i]);
            if mode == 0 && c == 0.0 && g == 0.0 {
                continue;
            }
            let light = [0, 1, 2].map(|ch| k[10] * c + (1.0 - c) * g * k[7 + ch]);
            let a = light[0].max(light[1]).max(light[2]).min(1.0);
            let out = match mode {
                0 => [px[0] as f64 + light[0], px[1] as f64 + light[1], px[2] as f64 + light[2], px[3] as f64 + (1.0 - px[3] as f64) * a],
                1 => [light[0], light[1], light[2], a],
                _ => [light[0], light[1], light[2], 1.0],
            };
            for (v, o) in px.iter_mut().zip(out) {
                *v = o as f32;
            }
        }
    });
}

/// D-508: the core's covering and the glow of bit `b` at (`x`, `y`).
fn energy_shade(k: &[f64; 12], b: &[f64; 6], x: f64, y: f64) -> (f64, f64) {
    let (dx, dy) = (b[2] - b[0], b[3] - b[1]);
    let l2 = dx * dx + dy * dy;
    let t = if l2 == 0.0 { 0.0 } else { (((x - b[0]) * dx + (y - b[1]) * dy) / l2).clamp(0.0, 1.0) };
    let d = (x - b[0] - t * dx).hypot(y - b[1] - t * dy);
    let w = b[4] + (b[5] - b[4]) * t;
    let r = k[0] * w / 2.0;
    let mut core = 0.0;
    if r > 0.0 {
        let soft = (r * k[1]).max(1.0);
        let u = ((r + 0.5 - d) / soft).clamp(0.0, 1.0);
        core = u * u * (3.0 - 2.0 * u);
    }
    let glow = if k[2] > 0.0 { w * k[3..7].iter().map(|q| (-d * d / (2.0 * q * q)).exp()).sum::<f64>() / 4.0 } else { 0.0 };
    (core, glow)
}
