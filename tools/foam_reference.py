"""Foam, worked a second way.

D-678 adds Foam (`core.foam`), after After Effects' Foam (Simulation): bubbles born in a producer
that grow, drift on the wind, wander in turbulence, push each other apart, stick together,
wobble and pop, drawn as outlines (the Draft view) or as textured bubbles (Rendered). Adobe's
manual names the controls and says what each is for; it publishes no formula, so the rule, the
units, the ranges and the settings when added are ours. No text or code was copied.

**Nothing is carried from frame to frame** (document 20): every frame steps the bubbles from the
layer's in point, so a frame drawn alone is the same frame played from the start. Bubbles push
each other, so there is no closed form; the build carries the same ceiling (`ponytail:`).

At composition frame n, the layer's in point i0, r frames a second, u = n - i0 frames since the
in point, on a drawing w by h (its own size), cz = (w / 2, h / 2) its middle, every number held
inside its range, S[i] the settings at frame i0 + i:

1. Steps. Q = 1, 2 or 4 steps a frame for `quality` normal, high or intense; dt = 1 / (r Q)
   seconds; step m = 0 .. u Q - 1 starts at t_m = m / (r Q) and reads S[m // Q]. The bubbles
   are drawn at T = u / r. Randomness: (U(seed, k, q, 0, 0) + 1) / 2, U Noise's hash (D-119),
   seed the whole part of `random_seed` at frame n; the turbulence field V(c, x, y, z) is Add
   Grain's smooth value noise (D-443, `grade::value`, not block) on channel 300 + c.
2. The producer, in the simulation's own space: with `zoom_producer_point` on, its centre
   P = (`producer_x` w, `producer_y` h) / 100 and its half sizes (A, B) = (`producer_x_size` w,
   `producer_y_size` h) / 2; off, P = cz + (that point - cz) / z and (A, B) / z, z = `zoom`, so
   it stays where it was put on screen. It is an ellipse turned `producer_orientation` degrees
   clockwise (angles are After Effects' way, 0 up, clockwise; (sin a, -cos a) the way a points).
3. A step from t_m, first the deaths. A bubble k born at b_k is r_k(t) = R_k min(1, (t - b_k) g
   10) across the radius, g = `growth_speed`. It pops when t_m - b_k >= its life; it leaves when
   |x - w / 2| - r > U w / 2 or |y - h / 2| - r > U h / 2, U = `universe_size`. Each popping
   bubble j (in number order) pushes each bubble i that neither pops nor leaves (in number
   order): with D = 2 (r_i + r_j), d the distance between them and D > 0, 0 < d < D, v_i gains
   (`pop_velocity` h / 4)(1 - d / D)(x_i - x_j) / d. Then the popping and leaving go.
4. Then the births. B_0 = 0, B_(m+1) = B_m + 30 `production_rate` dt; bubbles k = next, ... with
   k + 1/2 <= B_(m+1) are born at b_k = t_m, from seven randoms u_0 .. u_6 (q = 0 .. 6): at P +
   turn(rho cos psi A, rho sin psi B), rho = sqrt(u_0), psi = 2 pi u_1; full radius R_k =
   max(0, `size` (h / 40)(1 + `size_variance` (2 u_2 - 1))); life (`lifespan` / 30) u_3 ^ (1 /
   `strength`) seconds, 0 with strength 0; velocity (`initial_speed` h / 4) times the way
   `initial_direction` points. u_4, u_5 and u_6 are kept for the wobble and the turn. Size,
   variance, life and the first push are read at birth and kept.
5. Then the move. From the bubbles as they are at t_m (all of them before any moves), each
   bubble i's acceleration: the wind (`wind_speed` h / 4) times the way `wind_direction`
   points; the turbulence (`turbulence` h / 2)(V(0, x / L, y / L, t_m), V(1, ...)), L = h / 4;
   then over every other bubble j in number order, S = r_i + r_j, d their distance, when S > 0
   and 0 < d < 1.5 S: the weight 1 - d / (1.5 S) is summed into W and times v_j into Vs; when
   also d < S, the push K (S - d) / d (x_i - x_j), K = min(`repulsion` 4 h / S, 0.5 / dt^2).
   With W > 0, the stickiness min(`stickiness` 10 W, 0.5 / dt)(Vs / W - v_i). Then v_i = (v_i
   + a dt) exp(-10 `viscosity` dt) and x_i = x_i + v_i dt.
6. At T the popping and leaving of step 3 go once more (without pushes), and the rest are
   drawn, each at C = cz + z (x - cz), radius R = z r(T) with g at n, not drawn with R <= 0.
7. Its turn theta: fixed 0; `physical` (2 u_5 - 1) 180 (T - b) degrees; `velocity` the way it
   moves, atan2(v_x, -v_y) (0 when still). The wobble alpha = theta + 2 pi (u_4 + (1/2 + u_6)
   (T - b)) in radians. At a pixel centre (X, Y), (dx, dy) = (X, Y) - C, e its length; the
   radius that way R_w = R (1 + o (cos 2 alpha (dx^2 - dy^2) + sin 2 alpha 2 dx dy) / e^2), o =
   `wobble`, R where e = 0. q = e / R_w; (u_x, u_y) = turn(-theta)(dx, dy) / R_w, both divided
   by q where q > 1.
8. `view` draft: the producer first, in red (1, 0.15, 0.1), then each bubble in number order
   as a ring in blue (0.2, 0.5, 1), straight colour at full alpha, laid over at clamp(1 -
   |e - R_w|, 0, 1). The producer's ring: its centre on screen cz + z (P - cz), half sizes z A
   and z B (each at least 1/2), (l_x, l_y) = turn(-orientation)(X, Y - centre); k =
   sqrt((l_x / A)^2 + (l_y / B)^2), g2 = sqrt((l_x / A^2)^2 + (l_y / B^2)^2); with g2 > 0 its
   distance (k^2 - k) / g2 and the ring clamp(1 - |that|, 0, 1).
9. `view` rendered, each bubble at cover clamp(R_w - e + 1/2, 0, 1), its colour by
   `bubble_texture`, straight colour s and alpha a, then premultiplied: the highlight h =
   max(0, 1 - ((u_x + 0.35)^2 + (u_y + 0.35)^2) / 0.0625)^2;
   default_bubble a = 0.12 + 0.68 q^4, s = (0.85 + 0.15 q^2, 0.92, 1 - 0.15 q^2);
   spit a = 0.45 + 0.45 q^2, s = (0.95, 0.95, 0.93);
   bubblegum a = 0.95, s = (1, 0.45, 0.7)(0.5 + 0.5 max(0, -0.4 u_x - 0.4 u_y + 0.824
   sqrt(max(0, 1 - q^2))));
   dishwater a = 0.3 + 0.45 q^3, s = (0.72, 0.68, 0.58);
   then a + (1 - a) 0.85 h and s + (1 - s) h; layer: the drawing at ((u_x + 1) w / 2,
   (u_y + 1) h / 2) by the bilinear read below, as it is, no highlight.
   With `reflection_strength` f > 0 the colour's three are c (1 - f) + E a f, E the drawing at
   (1 - v) (C + 2 (dx, dy) / max(q, 1)) + v ((u_x + 1) w / 2, (u_y + 1) h / 2), v =
   `reflection_convergence`; the alpha stays.
   `blend_mode` transparent: screened, all four, d + c cover - d c cover, in number order (the
   order makes no difference); solid_new_on_top: over, in number order (the newest on top);
   solid_old_on_top: over, newest first.
10. The layer becomes the foam alone: every pixel starts clear.

The bilinear read, premultiplied and linear: fx = clamp(X' - 1/2, 0, w - 1), likewise fy; the
four pixels round it, the last row and column repeated at the edge.

Every distance is a share of the drawing's height and every place a share of its size, so a
draft is the same picture smaller. The numbers, all keyable: `producer_x`, `producer_y` -1000
to 1000 per cent, 50 and 50; `producer_x_size`, `producer_y_size` 0 to 2, 0.05; `producer_
orientation` -3600 to 3600, 0; `production_rate` 0 to 100, 1 (30 bubbles a second);
`size` 0 to 10, 0.5; `size_variance` 0 to 1, 0.5; `lifespan` 1 to 3000 thirtieths of a second,
300; `growth_speed` 0.01 to 10, 0.1 (full size in a second); `strength` 0 to 100, 10;
`initial_speed` 0 to 100, 0; `initial_direction` -3600 to 3600, 0; `wind_speed` 0 to 100, 0.5;
`wind_direction` -3600 to 3600, 90; `turbulence` 0 to 100, 0.5; `wobble` 0 to 1, 0.05;
`repulsion` 0 to 100, 1; `pop_velocity` 0 to 100, 0; `viscosity` 0 to 1, 0.1; `stickiness` 0
to 1, 0.75; `zoom` 0.05 to 50, 1; `universe_size` 0.1 to 10, 1; `reflection_strength` and
`reflection_convergence` 0 to 1, 0 and 0; `random_seed` 0 to 10000, 1. The words: `view` draft
or rendered, draft; `zoom_producer_point` off or on, off; `blend_mode` transparent,
solid_old_on_top or solid_new_on_top, transparent; `bubble_texture` default_bubble, spit,
bubblegum, dishwater or layer, default_bubble; `bubble_orientation` fixed, physical or
velocity, fixed; `quality` normal, high or intense, normal.

**This file never runs the build's code path.** It tries every pair of bubbles where the build
looks only in nearby cells (the same pairs in the same order), and works in double precision
on lists, straight from the drawing's 8-bit values.

Every case is a composition 48 pixels by 32 holding one drawing the same size, Bubbles' colour
field or the field with its right half clear. The projects go into `Fixtures/foam`, the expected
frames into `Fixtures/foam/expected_foam.json`. Every case is "not built yet" until B-557 lands
(D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/foam_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from noise_reference import u as U  # noqa: E402
from addgrain_reference import value as V  # noqa: E402
import smooth_reference as S  # noqa: E402
import bubbles_reference as BU  # noqa: E402

W, H, FPS = 48, 32, 24
assert (BU.W, BU.H) == (W, H)
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "foam"
TOLERANCE = 2e-5  # document 25's default for a filter
TYPE = "core.foam"
SETTINGS = [("producer_x", -1000, 1000, 50), ("producer_y", -1000, 1000, 50),
            ("producer_x_size", 0, 2, 0.05), ("producer_y_size", 0, 2, 0.05),
            ("producer_orientation", -3600, 3600, 0), ("production_rate", 0, 100, 1),
            ("size", 0, 10, 0.5), ("size_variance", 0, 1, 0.5), ("lifespan", 1, 3000, 300),
            ("growth_speed", 0.01, 10, 0.1), ("strength", 0, 100, 10),
            ("initial_speed", 0, 100, 0), ("initial_direction", -3600, 3600, 0),
            ("wind_speed", 0, 100, 0.5), ("wind_direction", -3600, 3600, 90),
            ("turbulence", 0, 100, 0.5), ("wobble", 0, 1, 0.05), ("repulsion", 0, 100, 1),
            ("pop_velocity", 0, 100, 0), ("viscosity", 0, 1, 0.1), ("stickiness", 0, 1, 0.75),
            ("zoom", 0.05, 50, 1), ("universe_size", 0.1, 10, 1),
            ("reflection_strength", 0, 1, 0), ("reflection_convergence", 0, 1, 0),
            ("random_seed", 0, 10000, 1)]
WORDS = [("view", ("draft", "rendered"), "draft"),
         ("zoom_producer_point", ("off", "on"), "off"),
         ("blend_mode", ("transparent", "solid_old_on_top", "solid_new_on_top"), "transparent"),
         ("bubble_texture", ("default_bubble", "spit", "bubblegum", "dishwater", "layer"),
          "default_bubble"),
         ("bubble_orientation", ("fixed", "physical", "velocity"), "fixed"),
         ("quality", ("normal", "high", "intense"), "normal")]
NAMES = [s[0] for s in SETTINGS]
RANGES = {s[0]: (s[1], s[2]) for s in SETTINGS}
STEPS = {"normal": 1, "high": 2, "intense": 4}
BLUE, RED = (0.2, 0.5, 1.0), (1.0, 0.15, 0.1)


# --- the rule -------------------------------------------------------------------------------

def clamp(v, lo, hi):
    return min(hi, max(lo, v))


def settings(c, f):
    return {n: clamp(value_at(c[n], f), *RANGES[n]) for n in NAMES}


def way(deg):
    a = math.radians(deg)
    return math.sin(a), -math.cos(a)


def producer(v, w=W, h=H):
    """Step 2: the producer's centre and half sizes in the simulation's own space."""
    px, py = v["producer_x"] / 100 * w, v["producer_y"] / 100 * h
    a, b = v["producer_x_size"] * w / 2, v["producer_y_size"] * h / 2
    if v["zoom_producer_point"] == "off":
        z = v["zoom"]
        px, py = w / 2 + (px - w / 2) / z, h / 2 + (py - h / 2) / z
        a, b = a / z, b / z
    return px, py, a, b


def radius(bub, t, g):
    return bub["R"] * min(1.0, (t - bub["b"]) * g * 10)


def gone(alive, t, v, w=W, h=H):
    """Step 3's popping and leaving sets, as lists of places in `alive`."""
    pops, leaves = [], []
    for i, bub in enumerate(alive):
        r = radius(bub, t, v["growth_speed"])
        if t - bub["b"] >= bub["life"]:
            pops.append(i)
        elif (abs(bub["x"] - w / 2) - r > v["universe_size"] * w / 2
              or abs(bub["y"] - h / 2) - r > v["universe_size"] * h / 2):
            leaves.append(i)
    return pops, leaves


def simulate(c, f, w=W, h=H):
    """Steps 1 to 6: the bubbles alive at frame f, each a dict, in number order, and T."""
    fps = c["rate"]
    u = f - c["in_frame"]
    q = STEPS[c["quality"]]
    dt = 1 / (fps * q)
    seed = math.floor(settings(c, f)["random_seed"])
    words = {w_[0]: c[w_[0]] for w_ in WORDS}
    alive, born, total = [], 0, 0.0
    for m in range(u * q):
        v = {**settings(c, c["in_frame"] + m // q), **words}
        t = m / (fps * q)
        g = v["growth_speed"]
        pops, leaves = gone(alive, t, v, w, h)
        out = set(pops) | set(leaves)
        for j in pops:
            bj = alive[j]
            rj = radius(bj, t, g)
            for i, bi in enumerate(alive):
                if i in out:
                    continue
                ri = radius(bi, t, g)
                D = 2 * (ri + rj)
                dx, dy = bi["x"] - bj["x"], bi["y"] - bj["y"]
                d = math.sqrt(dx * dx + dy * dy)
                if D > 0 and 0 < d < D:
                    k = v["pop_velocity"] * h / 4 * (1 - d / D) / d
                    bi["vx"] += k * dx
                    bi["vy"] += k * dy
        alive = [b for i, b in enumerate(alive) if i not in out]
        total += 30 * v["production_rate"] * dt
        px, py, A, B = producer(v, w, h)
        co, si = math.cos(math.radians(v["producer_orientation"])), \
            math.sin(math.radians(v["producer_orientation"]))
        while born + 0.5 <= total:
            k = born
            uq = [(U(seed, k, qq, 0, 0) + 1) / 2 for qq in range(7)]
            rho, psi = math.sqrt(uq[0]), 2 * math.pi * uq[1]
            lx, ly = rho * math.cos(psi) * A, rho * math.sin(psi) * B
            sx, sy = way(v["initial_direction"])
            sp = v["initial_speed"] * h / 4
            st = v["strength"]
            alive.append({
                "k": k, "b": t,
                "x": px + (lx * co - ly * si), "y": py + (lx * si + ly * co),
                "vx": sp * sx, "vy": sp * sy,
                "R": max(0.0, v["size"] * (h / 40) * (1 + v["size_variance"] * (2 * uq[2] - 1))),
                "life": v["lifespan"] / 30 * uq[3] ** (1 / st) if st > 0 else 0.0,
                "u": uq})
            born += 1
        wx, wy = way(v["wind_direction"])
        wind = v["wind_speed"] * h / 4
        tu = v["turbulence"] * h / 2
        L = h / 4
        kmax = 0.5 / (dt * dt)
        acc = []
        for i, bi in enumerate(alive):
            ri = radius(bi, t, g)
            ax = wind * wx + tu * V(seed, 300, bi["x"] / L, bi["y"] / L, t, False)
            ay = wind * wy + tu * V(seed, 301, bi["x"] / L, bi["y"] / L, t, False)
            ws, vsx, vsy = 0.0, 0.0, 0.0
            for j, bj in enumerate(alive):
                if j == i:
                    continue
                s_ = ri + radius(bj, t, g)
                dx, dy = bi["x"] - bj["x"], bi["y"] - bj["y"]
                d = math.sqrt(dx * dx + dy * dy)
                if s_ > 0 and 0 < d < 1.5 * s_:
                    wt = 1 - d / (1.5 * s_)
                    ws += wt
                    vsx += wt * bj["vx"]
                    vsy += wt * bj["vy"]
                    if d < s_:
                        k = min(v["repulsion"] * 4 * h / s_, kmax) * (s_ - d) / d
                        ax += k * dx
                        ay += k * dy
            if ws > 0:
                rate = min(v["stickiness"] * 10 * ws, 0.5 / dt)
                ax += rate * (vsx / ws - bi["vx"])
                ay += rate * (vsy / ws - bi["vy"])
            acc.append((ax, ay))
        drag = math.exp(-10 * v["viscosity"] * dt)
        for bi, (ax, ay) in zip(alive, acc):
            bi["vx"] = (bi["vx"] + ax * dt) * drag
            bi["vy"] = (bi["vy"] + ay * dt) * drag
            bi["x"] += bi["vx"] * dt
            bi["y"] += bi["vy"] * dt
    T = u / fps
    v = {**settings(c, f), **words}
    pops, leaves = gone(alive, T, v, w, h)
    out = set(pops) | set(leaves)
    return [b for i, b in enumerate(alive) if i not in out], T


def ring(e, rw):
    return clamp(1 - abs(e - rw), 0, 1)


def over(o, col, cov):
    return [col[q] * cov + o[q] * (1 - col[3] * cov) for q in range(4)]


def texture(word, ux, uy, qq):
    if word == "layer":
        return None
    hl = max(0.0, 1 - ((ux + 0.35) ** 2 + (uy + 0.35) ** 2) / 0.0625) ** 2
    if word == "default_bubble":
        a, s = 0.12 + 0.68 * qq ** 4, (0.85 + 0.15 * qq * qq, 0.92, 1 - 0.15 * qq * qq)
    elif word == "spit":
        a, s = 0.45 + 0.45 * qq * qq, (0.95, 0.95, 0.93)
    elif word == "bubblegum":
        lit = 0.5 + 0.5 * max(0.0, -0.4 * ux - 0.4 * uy + 0.824 * math.sqrt(max(0.0, 1 - qq * qq)))
        a, s = 0.95, (1.0 * lit, 0.45 * lit, 0.7 * lit)
    else:
        a, s = 0.3 + 0.45 * qq ** 3, (0.72, 0.68, 0.58)
    a = a + (1 - a) * 0.85 * hl
    s = [x + (1 - x) * hl for x in s]
    return [x * a for x in s] + [a]


def draw(c, f, src, w=W, h=H):
    """Steps 6 to 10."""
    alive, T = simulate(c, f, w, h)
    v = {**settings(c, f), **{w_[0]: c[w_[0]] for w_ in WORDS}}
    z = v["zoom"]
    cx, cy = w / 2, h / 2
    sprites = []
    for bub in alive:
        R = z * radius(bub, T, v["growth_speed"])
        if not R > 0:
            continue
        age = T - bub["b"]
        uq = bub["u"]
        if v["bubble_orientation"] == "physical":
            th = math.radians((2 * uq[5] - 1) * 180 * age)
        elif v["bubble_orientation"] == "velocity" and (bub["vx"] != 0 or bub["vy"] != 0):
            th = math.atan2(bub["vx"], -bub["vy"])
        else:
            th = 0.0
        al = th + 2 * math.pi * (uq[4] + (0.5 + uq[6]) * age)
        sprites.append((cx + z * (bub["x"] - cx), cy + z * (bub["y"] - cy), R,
                        math.cos(th), math.sin(th), math.cos(2 * al), math.sin(2 * al)))
    if v["view"] == "rendered" and v["blend_mode"] == "solid_old_on_top":
        sprites.reverse()
    px, py, A, B = producer(v, w, h)
    pcx, pcy = cx + z * (px - cx), cy + z * (py - cy)
    pa, pb = max(0.5, z * A), max(0.5, z * B)
    po = math.radians(v["producer_orientation"])
    pco, psi = math.cos(po), math.sin(po)
    o_ = v["wobble"]
    fr, cv = v["reflection_strength"], v["reflection_convergence"]
    out = []
    for yy in range(h):
        for xx in range(w):
            X, Y = xx + 0.5, yy + 0.5
            acc = [0.0] * 4
            if v["view"] == "draft":
                lx = (X - pcx) * pco + (Y - pcy) * psi
                ly = -(X - pcx) * psi + (Y - pcy) * pco
                k = math.sqrt((lx / pa) ** 2 + (ly / pb) ** 2)
                g2 = math.sqrt((lx / (pa * pa)) ** 2 + (ly / (pb * pb)) ** 2)
                if g2 > 0:
                    cov = clamp(1 - abs((k * k - k) / g2), 0, 1)
                    acc = over(acc, list(RED) + [1.0], cov)
            for (sx, sy, R, cth, sth, c2, s2) in sprites:
                dx, dy = X - sx, Y - sy
                e2 = dx * dx + dy * dy
                e = math.sqrt(e2)
                rw = R * (1 + o_ * (c2 * (dx * dx - dy * dy) + s2 * 2 * dx * dy) / e2) if e2 > 0 else R
                if v["view"] == "draft":
                    cov = ring(e, rw)
                    if cov > 0:
                        acc = over(acc, list(BLUE) + [1.0], cov)
                    continue
                cov = clamp(rw - e + 0.5, 0, 1)
                if cov == 0:
                    continue
                qq = e / rw
                ux = (dx * cth + dy * sth) / rw
                uy = (-dx * sth + dy * cth) / rw
                cut = max(qq, 1.0)
                ux, uy, qq = ux / cut, uy / cut, min(qq, 1.0)
                col = texture(v["bubble_texture"], ux, uy, qq)
                if col is None:
                    col = BU.bilinear(src, (ux + 1) * w / 2, (uy + 1) * h / 2, w, h)
                if fr > 0:
                    ex = (1 - cv) * (sx + 2 * dx / cut) + cv * ((ux + 1) * w / 2)
                    ey = (1 - cv) * (sy + 2 * dy / cut) + cv * ((uy + 1) * h / 2)
                    E = BU.bilinear(src, ex, ey, w, h)
                    col = [col[q] * (1 - fr) + E[q] * col[3] * fr for q in range(3)] + [col[3]]
                if v["blend_mode"] == "transparent":
                    acc = [acc[q] + col[q] * cov - acc[q] * col[q] * cov for q in range(4)]
                else:
                    acc = over(acc, col, cov)
            out.append(acc)
    return out


def valid(c):
    for n, lo, hi, _ in SETTINGS:
        v = c[n]
        values = [k[1] for k in v["keys"]] if isinstance(v, dict) else [v]
        if not all(lo <= x <= hi for x in values):
            return False
    return all(c[name] in words for name, words, _ in WORDS)


def render(c, f):
    src = BU.working(c["drawing"])
    if not valid(c):
        return BU.frame(src, c["shift"])
    return BU.frame(draw(c, f, src), c["shift"])


def plain(c):
    return BU.frame(BU.working(c["drawing"]), c["shift"])


# --- the cases ------------------------------------------------------------------------------

def case(drawing="field", rate=FPS, in_frame=0, shift=0, **given):
    c = {s[0]: s[3] for s in SETTINGS}
    c.update({w[0]: w[2] for w in WORDS})
    for key, v in given.items():
        assert key in c, key
        c[key] = v
    c.update(drawing=drawing, rate=rate, in_frame=in_frame, shift=shift)
    return c


# A close look: bubbles about 1 to 4 pixels across the radius from a producer a third of the
# picture, fifteen a second, drawn.
BIG = dict(size=3, producer_x_size=0.3, producer_y_size=0.3, production_rate=0.5,
           view="rendered")


def big(**kw):
    return case(**{**BIG, **kw})


CASES = {
    "FX-FOAM-001": ("As added: the Draft view, the red producer in the middle and thirty tiny "
                    "blue rings a second drifting right; frame 0 the producer alone.",
                    case(), (0, 12, 24, 35)),
    "FX-FOAM-002": ("A close look, Rendered: bigger bubbles from a bigger producer, fifteen a "
                    "second; frame 0 clear, frame 1 the first born, too small to see.",
                    big(), (0, 1, 12, 24)),
    "FX-FOAM-003": ("The close look in the Draft view: rings.", big(view="draft"), (12,)),
    "FX-FOAM-004": ("The producer at 25, 75, wide and flat, turned 30 degrees.",
                    big(producer_x=25, producer_y=75, producer_x_size=0.6,
                        producer_y_size=0.1, producer_orientation=30, view="draft"), (12,)),
    "FX-FOAM-005": ("Zoom 2, the producer kept where it was put on screen: bubbles twice as "
                    "big, drifting twice as fast.", big(zoom=2), (12,)),
    "FX-FOAM-006": ("Zoom 2 with Zoom Producer Point on: the producer grown too.",
                    big(zoom=2, zoom_producer_point="on", view="draft"), (12,)),
    "FX-FOAM-007": ("Production Rate 2: sixty a second.", big(production_rate=2), (12,)),
    "FX-FOAM-008": ("Size Variance 0: every bubble the same size.", big(size_variance=0),
                    (12,)),
    "FX-FOAM-009": ("Size 6: big bubbles crowding and pushing.", big(size=6), (12,)),
    "FX-FOAM-010": ("Lifespan 15 (half a second): the old ones popped.", big(lifespan=15),
                    (24,)),
    "FX-FOAM-011": ("Bubble Growth Speed 1: full size in a tenth of a second.",
                    big(growth_speed=1), (6,)),
    "FX-FOAM-012": ("Strength 1 with Lifespan 30: most pop well before the second is out.",
                    big(strength=1, lifespan=30), (24,)),
    "FX-FOAM-013": ("Initial Speed 2 towards 180 degrees: thrown down.",
                    big(initial_speed=2, initial_direction=180), (12,)),
    "FX-FOAM-014": ("Wind 4 towards 0 degrees: blown up and away.",
                    big(wind_speed=4, wind_direction=0), (24,)),
    "FX-FOAM-015": ("Turbulence 5: wandering.", big(turbulence=5), (24,)),
    "FX-FOAM-016": ("Wobble 0.4: squashed and stretched.", big(wobble=0.4, size=5), (12,)),
    "FX-FOAM-017": ("Repulsion 0: bubbles overlap.", big(repulsion=0, size=5), (24,)),
    "FX-FOAM-018": ("Repulsion 10: pushed hard apart.", big(repulsion=10, size=5), (24,)),
    "FX-FOAM-019": ("Pop Velocity 20 with short lives: each pop shoves its neighbours.",
                    big(pop_velocity=20, lifespan=15, strength=2, size=5), (24,)),
    "FX-FOAM-020": ("Viscosity 1: the wind barely moves them.", big(viscosity=1), (24,)),
    "FX-FOAM-021": ("Stickiness 0: neighbours do not drag each other.",
                    big(stickiness=0, size=5), (24,)),
    "FX-FOAM-022": ("Universe Size 0.5 with a strong wind: gone before the layer's edge.",
                    big(universe_size=0.5, wind_speed=4), (24,)),
    "FX-FOAM-023": ("Blend Mode Solid Old on Top.", big(blend_mode="solid_old_on_top", size=5),
                    (24,)),
    "FX-FOAM-024": ("Blend Mode Solid New on Top.", big(blend_mode="solid_new_on_top", size=5),
                    (24,)),
    "FX-FOAM-025": ("Bubble Texture Spit.", big(bubble_texture="spit", size=5), (24,)),
    "FX-FOAM-026": ("Bubble Texture Bubblegum.", big(bubble_texture="bubblegum", size=5),
                    (24,)),
    "FX-FOAM-027": ("Bubble Texture Dishwater.", big(bubble_texture="dishwater", size=5),
                    (24,)),
    "FX-FOAM-028": ("Bubble Texture Layer: the picture in every bubble.",
                    big(bubble_texture="layer", size=6), (24,)),
    "FX-FOAM-029": ("Layer texture, Bubble Orientation Physical: each picture turned its own "
                    "way.", big(bubble_texture="layer", size=6, bubble_orientation="physical"),
                    (24,)),
    "FX-FOAM-030": ("Layer texture, Bubble Orientation Velocity: each picture's top the way it "
                    "moves.", big(bubble_texture="layer", size=6, bubble_orientation="velocity"),
                    (24,)),
    "FX-FOAM-031": ("Reflection Strength 0.7, Convergence 0: each bubble shows the picture "
                    "round it.", big(reflection_strength=0.7, size=5), (24,)),
    "FX-FOAM-032": ("Reflection Strength 0.7, Convergence 1: each bubble shows the whole "
                    "picture.", big(reflection_strength=0.7, reflection_convergence=1, size=5),
                    (24,)),
    "FX-FOAM-033": ("Simulation Quality High: two steps a frame.", big(quality="high", size=5),
                    (24,)),
    "FX-FOAM-034": ("Simulation Quality Intense: four steps a frame.",
                    big(quality="intense", size=5), (12,)),
    "FX-FOAM-035": ("Random Seed 7: other bubbles.", big(random_seed=7), (24,)),
    "FX-FOAM-036": ("Production Rate keyed from 0 at frame 0 to 2 at frame 24: born slowly, "
                    "then faster.", big(production_rate=keyed((0, 0), (24, 2))), (24,)),
    "FX-FOAM-037": ("Wind Direction keyed from 0 to 180 over frames 0 to 24: blown up, then "
                    "round and down.", big(wind_speed=4, wind_direction=keyed((0, 0), (24, 180))),
                    (24,)),
    "FX-FOAM-038": ("FX-FOAM-002 moved 3 pixels right: the same, moved.", big(shift=3), (12,)),
    "FX-FOAM-039": ("FX-FOAM-002 with the layer starting at frame 6: frame 6 clear, frame 18 "
                    "002's frame 12.", big(in_frame=6), (6, 18)),
    "FX-FOAM-040": ("FX-FOAM-002 at 30 frames a second: frame 12 is two fifths of a second "
                    "in.", big(rate=30), (12,)),
    "FX-FOAM-041": ("The half picture with the Layer texture: half of each bubble's picture "
                    "clear.", big(drawing="half", bubble_texture="layer", size=6), (24,)),
}

# D-46: the file is read, the effect is kept as written and left out of every frame.
INVALID = {
    "FX-FOAM-042": ("Production Rate -1, below 0.", big(production_rate=-1)),
    "FX-FOAM-043": ("Size 11, above 10.", big(size=11)),
    "FX-FOAM-044": ("Size Variance 2, above 1.", big(size_variance=2)),
    "FX-FOAM-045": ("Lifespan 0, below 1.", big(lifespan=0)),
    "FX-FOAM-046": ("Bubble Growth Speed 0, below 0.01.", big(growth_speed=0)),
    "FX-FOAM-047": ("Strength 101, above 100.", big(strength=101)),
    "FX-FOAM-048": ("Wind Speed -1, below 0.", big(wind_speed=-1)),
    "FX-FOAM-049": ("Viscosity 2, above 1.", big(viscosity=2)),
    "FX-FOAM-050": ("Zoom 0, below 0.05.", big(zoom=0)),
    "FX-FOAM-051": ("Universe Size 11, above 10.", big(universe_size=11)),
    "FX-FOAM-052": ("Reflection Strength 1.5, above 1.", big(reflection_strength=1.5)),
    "FX-FOAM-053": ("Random Seed -1, below 0.", big(random_seed=-1)),
    "FX-FOAM-054": ("Producer X 1001, above 1000.", big(producer_x=1001)),
    "FX-FOAM-055": ("A view written \"flow\".", big(view="flow")),
    "FX-FOAM-056": ("A blend mode written \"add\".", big(blend_mode="add")),
    "FX-FOAM-057": ("A bubble texture written \"soap\".", big(bubble_texture="soap")),
    "FX-FOAM-058": ("A bubble orientation written \"spin\".", big(bubble_orientation="spin")),
    "FX-FOAM-059": ("A quality written \"ultra\".", big(quality="ultra")),
    "FX-FOAM-060": ("A zoom producer point written \"yes\".", big(zoom_producer_point="yes")),
    "FX-FOAM-061": ("Production Rate keyed to 200 at frame 4, above 100.",
                    big(production_rate=keyed((0, 1), (4, 200)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    d = c["drawing"]
    frames = 36
    params = {n: setting_json(c[n]) for n in NAMES}
    params.update({w[0]: c[w[0]] for w in WORDS})
    comp = {
        "id": "comp-main", "name": "Main", "width": W, "height": H,
        "pixel_aspect_ratio": 1, "frame_rate": {"numerator": c["rate"], "denominator": 1},
        "start_frame": 0, "duration_frames": frames,
        "work_area": {"start_frame": 0, "end_frame_exclusive": frames},
        "layer_order": ["art"],
        "layers": [{
            "id": "art", "kind": "raster", "name": "art", "asset_id": "asset-" + d,
            "enabled": True, "locked": False, "in_frame": c["in_frame"], "out_frame": frames,
            "source_offset_frames": 0,
            "transform": {
                "anchor": prop([0, 0]), "position": prop([c["shift"], 0]),
                "scale": prop([100, 100]), "rotation": prop(0), "opacity": prop(1),
            },
            "exposure_spans": [], "mask": None, "matte": None, "blend_mode": "normal",
            "effects": [{"instance_id": "fx-0-0", "type_id": TYPE, "enabled": True,
                         "parameters": params}],
        }],
    }
    return {
        "schema_version": 0,
        "project_id": "proj-" + fx.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-" + d, "kind": "still", "name": d, "path": f"media/{d}.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [comp],
    }


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in BU.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        assert valid(c), fx
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        print(f"{fx}: " + ", ".join(
            f"frame {f} {len(simulate(c, int(f))[0])} bubbles, "
            f"{sum(p[3] > 0 for p in px)} covered"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        assert not valid(c), fx
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_foam.json").write_text(json.dumps(expected, indent=1) + "\n",
                                            encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx.rsplit("-", 1)[1]: v["frames"] for fx, v in expected["cases"].items()}
    clear = [[0.0] * 4 for _ in range(W * H)]
    assert c["002"]["0"] == clear and c["039"]["6"] == clear
    assert c["001"]["0"] != clear  # the producer
    two = c["002"]
    assert two["12"] != clear and two["24"] != two["12"]
    assert c["039"]["18"] == two["12"]
    moved = c["038"]["12"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == two["12"][y * W:y * W + W - 3]
    for n in (3, 5, 7, 8, 9, 13, 40):
        assert c["%03d" % n]["12"] != two["12"], n
    for n in (10, 12, 14, 15, 20, 22, 35, 36, 37):
        assert c["%03d" % n]["24"] != two["24"], n
    five = render(big(size=5), 24)
    for n in (17, 18, 21, 23, 24, 25, 26, 27, 31, 32, 33):
        assert c["%03d" % n]["24"] != five, n
    assert c["017"]["24"] != c["018"]["24"]
    assert c["023"]["24"] != c["024"]["24"]
    assert c["019"]["24"] != render(big(lifespan=15, strength=2, size=5), 24)
    assert c["028"]["24"] != c["029"]["24"] != c["030"]["24"]
    assert c["041"]["24"] != c["028"]["24"]
    assert c["011"]["6"] != render(big(), 6)
    assert c["016"]["12"] != render(big(size=5), 12)
    assert c["034"]["12"] != render(big(size=5), 12)
    assert c["006"]["12"] != render(big(zoom=2, view="draft"), 12)
    # Bubbles do touch in the close look, so repulsion and stickiness are tried.
    alive, T = simulate(big(size=5), 24)
    touching = sum(1 for i, a in enumerate(alive) for b in alive[i + 1:]
                   if math.hypot(a["x"] - b["x"], a["y"] - b["y"])
                   < radius(a, T, 0.1) + radius(b, T, 0.1))
    assert touching > 0, touching
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(a >= -1e-12 for a in p) and p[3] <= 1 + 1e-12, fx
                assert all(p[q] <= p[3] + 1e-12 for q in range(3)), fx
    print("checks passed")


if __name__ == "__main__":
    main()
