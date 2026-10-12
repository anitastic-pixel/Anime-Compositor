"""Shatter, worked a second way.

D-633 adds Shatter (`core.shatter`), after After Effects' Shatter (Simulation): the layer broken
into pieces of a tiling pattern or of a custom map, each piece freed when a force sphere reaches
it (and a gradient layer allows), then thrown from the force's centre, pulled by gravity, slowed
by viscosity and tumbling, drawn as a thick slab seen by a camera and lit. Adobe's CS5 help names
the controls and says what several do (the patterns; a custom map whose colours make the pieces
and whose alpha says where pieces are; White Tiles Fixed; two forces, each a sphere with a
position, depth, radius and strength, a negative strength pulling pieces in; the gradient's
Shatter Threshold, white areas first, and Invert Gradient; Rotation Speed, Tumble Axis,
Randomness, Viscosity, Mass Variance, Gravity, its Direction and Inclination; the front, side and
back textures and their modes; Render All, Layer or Pieces; the camera, light and material shared
with Card Wipe and Card Dance). It publishes no formula and few settings when added, so the rule,
the units, the shapes of the patterns and most settings when added are ours (document 21), and
nothing is ported. Time counts from the layer's in point (D-630).

The settings. `view` "rendered" (the wireframe views come with D-635); `render` "all" (when
added), "layer" (only pieces not yet freed) or "pieces" (only freed pieces); `pattern` "bricks"
(when added), "squares", "hexagons", "triangles_1", "triangles_2", "rhombus", "planks",
"herringbone_1", "octagons_squares", "glass" or "custom"; `custom_shatter_map` D-189's layer word,
"" when added (the layer's own drawing), fitted as `stretch`; `white_tiles_fixed` "off" (when
added) or "on"; `repetitions` 1 to 1000 tiles across the layer, 10; `direction` -3600 to 3600
degrees, 0; `origin` x then y, -1000 to 1000 per cent of the layer, [50, 50]; `extrusion_depth`
0 to 1 layer widths, 0.2. For each force F of 1 and 2: `force_F_position` x then y, -1000 to 1000
per cent, [50, 50]; `force_F_depth` -10 to 10 layer widths toward the viewer, 0.1;
`force_F_radius` 0 to 10 layer widths, 0.4 for force 1 and 0 for force 2; `force_F_strength`
-100 to 100 layer widths a second, 5. `gradient_layer` a layer word, "" when added (no gradient:
every piece reached is freed), fitted as `stretch`; `shatter_threshold` 0 to 100 per cent, 0;
`invert_gradient` "off" (when added) or "on". `rotation_speed` 0 to 10 turns a second, 0.2;
`tumble_axis` "free" (when added), "none", "x", "y", "z", "xy", "xz" or "yz"; `randomness` 0 to 1,
0.1; `viscosity` 0 to 10 a second, 0.1; `mass_variance` 0 to 100 per cent, 30; `gravity` 0 to 100
layer widths a second squared, 3; `gravity_direction` -3600 to 3600 degrees, 180 (down);
`gravity_inclination` -90 to 90 degrees, 0. `color` #rrggbb, "#ffffff"; `opacity` 0 to 1, 1;
`front_mode`, `side_mode` and `back_mode` each "color", "layer", "tinted_layer",
"color_plus_opacity", "layer_plus_opacity" or "tinted_layer_plus_opacity", when added "layer",
"tinted_layer" and "layer"; `front_layer`, `side_layer` and `back_layer` layer words, "" when
added (the layer's own drawing), fitted as `stretch`. The camera and the light, Card Dance's
exactly: `camera_system` "camera_position", "comp_camera" or "corner_pins" (D-634, with Card
Dance's four pins and `auto_focal_length`, read as when added and written only when given);
`camera_x_rotation`,
`camera_y_rotation`, `camera_z_rotation`, `camera_xy_position`, `camera_z_position`,
`focal_length`, `light_type`, `light_intensity`, `light_color`, `light_position`, `light_depth`,
`ambient_light`, `diffuse_reflection`, `specular_reflection`, `highlight_sharpness`, with Card
Dance's ranges and settings when added. Every number is keyable; the words are exact.

The rule, in the layer's own W by H space, y down, z away from the viewer, the origin at its
middle C0 = (W/2, H/2, 0).

1. The pattern. A pixel (x, y)'s centre p = (x + 1/2, y + 1/2) goes to pattern space:
   q = Rot(-direction) (p - (origin_x W/100, origin_y H/100)) repetitions / W, Rot turning
   (a, b) to (a cos - b sin, a sin + b cos). Its tile, with fl the floor and r3 = sqrt 3 / 2:
   squares (fl qx, fl qy); bricks, rows 1/2 high, every other row moved half a brick:
   r = fl(2 qy), (fl(qx + (r mod 2)/2), r); planks, rows 1/4 high, 2 long, each row moved 0.7:
   r = fl(4 qy), (fl((qx + 0.7 r)/2), r); triangles_1, each square cut corner to corner:
   (fl qx, fl qy, qx - fl qx > qy - fl qy); triangles_2, equal triangles: a = qy/r3,
   b = qx - a/2, (fl b, fl a, (b - fl b) + (a - fl a) > 1); rhombus (fl(qx + qy), fl(qx - qy));
   hexagons, the nearest of the four points (i + di) e1 + (j + dj) e2, di and dj 0 or 1, i = fl b,
   j = fl a, e1 = (1, 0), e2 = (1/2, r3), nearest by squared distance, ties to the lower (dj, di);
   herringbone_1, columns 1/2 wide of planks slanting up and down by turns: c = fl(2 qx),
   s = 1 for even c and -1 for odd, (c, fl(2 qy + s (2 qx - c))); octagons_squares: (i, j) =
   (fl(qx + 1/2), fl(qy + 1/2)), the octagon (i, j) where |qx - i| + |qy - j| <= sqrt(2)/2, else
   the square (fl qx, fl qy); glass, the nearest seed of the cells (ci, cj) within two of
   (fl qx, fl qy), the seed (ci + g(ci, cj, 0), cj + g(ci, cj, 1)), ties to the lower (cj, ci),
   g(i, j, a) = unit(mix(mix(i, j), 100 + a)) with document 22's splitmix64 `mix` and `unit` on
   two's complement 64-bit integers.
2. The pieces. Pixels of one tile are one piece. "custom": the map picture (the layer's own drawing
   for "", clear for a layer not in the composition or not begun) read per pixel as Card Dance's
   red, green and blue (linear_to_srgb of the clamped premultiplied channel) and alpha: a pixel
   with alpha under 1/2 is in no piece; the others take one of eight colours, each channel at
   least 1/2 or not, and each 4-connected region of one colour is a piece. Pieces are numbered by
   their first pixel, rows then columns. A piece's balance point (bx, by) is the mean of its
   pixel centres, b = (bx - W/2, by - H/2, 0); its randoms r(i, a) = unit(mix(i, a)).
3. The settings history. Sample j is every number at composition frame n0 + j, j = 0 .. n - n0,
   n0 the layer's in point; step m (four a frame, dt = 1/(4 fps) seconds) reads the samples
   m div 4 and the next, in a straight line by (m mod 4)/4.
4. The release. With the gradient picture (the current frame's, none for ""), a piece's value v
   is document 21's luma of its pixel (min(fl bx, W - 1), min(fl by, H - 1)), 1 - v when
   inverted; it may go where there is no gradient or where v + threshold/100 >= 1. A piece is
   freed at the first step m = 0 .. 4 (n - n0) at which it may go and one of its pixel centres
   (px - W/2, py - H/2, 0) lies inside a force F with radius > 0: closer than radius W to
   (pos_x W/100 - W/2, pos_y H/100 - H/2, -depth W), force 1 first; the custom pieces of all
   three colours white never go with "on". Not freed by now: at rest.
5. The flight, from step m0 to step M = 4 (n - n0), with the numbers of the freeing force and of
   step m0: speed = strength W (1 + randomness (2 r(i,1) - 1)) / (1 + mass/100 (r(i,2) - 1/2));
   v = speed unit(b - centre) (zero where b is the centre); the spin rate
   w = 2 pi rotation_speed (1 + randomness (2 r(i,3) - 1)) about the axis: free
   unit(2 r(i,4) - 1, 2 r(i,5) - 1, 2 r(i,6) - 1), xy, xz and yz the same with the third zeroed, x,
   y and z the unit axes, none no spin. Each step m0 .. M - 1, with its own gravity G, direction
   d, inclination c and viscosity k: g = G W (sin d cos c, -cos d cos c, -sin c); with
   e = exp(-k dt) and k > 1e-9, x += (v - g/k)(1 - e)/k + g dt/k, v = v e + g (1 - e)/k,
   angle += w (1 - e)/k, w = w e; else x += v dt + g dt^2/2, v += g dt, angle += w dt. The piece is
   then T(p) = R (p - b) + b + x, R the turn by angle about the axis.
6. The slab. Each of a piece's pixels (i, j) is the box [i, i+1] x [j, j+1] x [0, e] in the
   piece's own (u, v, z) space, e = extrusion_depth W, its front at z = 0 facing the viewer. The
   camera is Card Dance's step 4 (comp_camera drawing nothing where A_l <= 0). A ray (o, d) seen
   in the piece is o' = R^T (o - b - x) + (bx, by, 0), in the layer's pixels (u, v) and depth z,
   and d' = R^T d. For e > 0, the box is entered at the latest of the three slab entries, if before
   the earliest exit and with t > 0; the face is the axis of that entry: z, the front where
   d'z > 0 and the back otherwise; x or y a side, its normal against the ray on that axis. For
   e = 0 the plane z = 0 at t > 0, inside [i, i+1) x [j, j+1), the front where d'z > 0. A piece
   is hit at its nearest box; the nearest piece wins, ties to the lower number. "layer" draws
   only pieces not freed, "pieces" only freed ones.
7. The colour at the hit (u, v): the face's picture, the front and side layers at (u, v) and the
   back at (2 bx - u, v), each by document 21's bilinear (transparent outside), "" the layer's
   own drawing, clear for a layer not in the composition or not begun. Its mode: layer, the
   picture; tinted_layer, its colour times the colour; color, the colour, alpha 1; each
   "_plus_opacity" all four times the opacity. Lit by Card Dance's step 7 with the face's normal
   R n (front (0, 0, -1), back (0, 0, 1)).
8. The output at buffer pixel (X, Y): the winning hit's colour, or clear.

The layer never grows. A draft changes nothing, every distance being a share of the layer.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawings' 8-bit values, tests every pixel box of every piece for every pixel, and walks
the release step by step. The rule steps at a tile's edges, at a sphere's surface, at the
threshold, at a box's edges and where two pieces tie; every case is checked that nothing lies
near one.

Every case is a project of one composition 16 by 8, eight frames at 24 a second, in
`Fixtures/shatter/`: the map layers, every one switched off, and on top of them the layer
`holder`, showing the drawing `shot`, with the effect. The expected pixels are in
`Fixtures/shatter/expected_shatter.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/shatter_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from expression_reference import mix, unit, MASK  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import compound_blur_reference as K  # noqa: E402
import smooth_reference as SM  # noqa: E402
import card_dance_reference as CD  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "shatter"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES, FPS = 16, 8, 8, 24
assert (W, H) == (CD.W, CD.H)
CLEAR = [0.0, 0.0, 0.0, 0.0]
R3 = math.sqrt(3) / 2


def cracks_drawing():
    """16 by 8, a custom shatter map: red left, white and blue in the middle, green right, with a
    second red island; the bottom right 3 by 2 clear."""
    def px(x, y):
        if x >= 13 and y >= 6:
            return (0, 0, 0, 0)
        if x < 5:
            return (255, 255, 255, 255) if (x, y) in ((1, 1), (2, 1), (1, 2), (2, 2)) else \
                (230, 30, 30, 255)
        if x < 10:
            return (250, 250, 250, 255) if y < 4 else (30, 30, 230, 255)
        return (230, 30, 30, 255) if (x >= 12 and y < 2) else (30, 220, 40, 255)
    return [[px(x, y) for x in range(W)] for y in range(H)]


DRAWINGS = dict(CD.DRAWINGS, cracks=cracks_drawing())
MAPS = {"back": "back", "ramp": "ramp", "colours": "colours", "cracks": "cracks", "late": "back"}


def CD_decoded(name):
    rows = DRAWINGS[name]
    s2l = CD.srgb_to_linear
    return L.pic(len(rows[0]), len(rows),
                 [[s2l(r / 255) * a / 255, s2l(g / 255) * a / 255, s2l(b / 255) * a / 255, a / 255]
                  for row in rows for r, g, b, a in row])


def map_picture(name, frame):
    """A layer word's picture fitted to the layer; None for ""; clear for a layer not in the
    composition or not begun."""
    if name == "":
        return None
    if name not in MAPS or (name == "late" and frame < 4):
        return L.pic(W, H)
    return L.fit(CD_decoded(MAPS[name]), "stretch", W, H)


# --- the rule -------------------------------------------------------------------------------

PATTERNS = ("bricks", "squares", "hexagons", "triangles_1", "triangles_2", "rhombus", "planks",
            "herringbone_1", "octagons_squares", "glass", "custom")
LATER_PATTERNS = ("herringbone_2", "squares_triangles", "stars_triangles", "overlapping_squares",
                  "chevrons", "puzzle", "js", "eggs", "crescents", "carpenters_wheel")
TUMBLE = ("free", "none", "x", "y", "z", "xy", "xz", "yz")
MODES = ("color", "layer", "tinted_layer", "color_plus_opacity", "layer_plus_opacity",
         "tinted_layer_plus_opacity")
fl = math.floor


def u64(i):
    return i & MASK


def g_hash(i, j, a):
    return unit(mix(mix(u64(i), u64(j)), 100 + a))


def r_hash(i, a):
    return unit(mix(i, a))


def tile(pattern, qx, qy):
    """Step 1's tile of a point in pattern space."""
    if pattern == "squares":
        return (fl(qx), fl(qy))
    if pattern == "bricks":
        r = fl(2 * qy)
        return (fl(qx + (r % 2) / 2), r)
    if pattern == "planks":
        r = fl(4 * qy)
        return (fl((qx + 0.7 * r) / 2), r)
    if pattern == "triangles_1":
        return (fl(qx), fl(qy), qx - fl(qx) > qy - fl(qy))
    a, b = qy / R3, qx - qy / R3 / 2
    if pattern == "triangles_2":
        return (fl(b), fl(a), (b - fl(b)) + (a - fl(a)) > 1)
    if pattern == "rhombus":
        return (fl(qx + qy), fl(qx - qy))
    if pattern == "hexagons":
        best = None
        for dj in (0, 1):
            for di in (0, 1):
                i, j = fl(b) + di, fl(a) + dj
                cx, cy = i + j / 2, j * R3
                d2 = (qx - cx) ** 2 + (qy - cy) ** 2
                if best is None or d2 < best[0]:
                    best = (d2, i, j)
        return best[1:]
    if pattern == "herringbone_1":
        c = fl(2 * qx)
        s = 1 if c % 2 == 0 else -1
        return (c, fl(2 * qy + s * (2 * qx - c)))
    if pattern == "octagons_squares":
        i, j = fl(qx + 0.5), fl(qy + 0.5)
        if abs(qx - i) + abs(qy - j) <= math.sqrt(2) / 2:
            return ("o", i, j)
        return ("s", fl(qx), fl(qy))
    if pattern == "glass":
        best = None
        for dj in range(-2, 3):
            for di in range(-2, 3):
                ci, cj = fl(qx) + di, fl(qy) + dj
                sx, sy = ci + g_hash(ci, cj, 0), cj + g_hash(ci, cj, 1)
                d2 = (qx - sx) ** 2 + (qy - sy) ** 2
                if best is None or d2 < best[0]:
                    best = (d2, ci, cj)
        return best[1:]
    raise ValueError(pattern)


def pattern_point(k, x, y, nudge=(0.0, 0.0)):
    t = math.radians(-k["direction"])
    c, s = math.cos(t), math.sin(t)
    ax = x + 0.5 + nudge[0] - k["origin"][0] * W / 100
    ay = y + 0.5 + nudge[1] - k["origin"][1] * H / 100
    sc = k["repetitions"] / W
    return (ax * c - ay * s) * sc, (ax * s + ay * c) * sc


def channel(p, i):
    return SM.linear_to_srgb(min(1.0, max(0.0, p[i])))


def pieces(k, drawing, custom):
    """Step 2: a list of pieces, each (cells, bx, by, white)."""
    owner = {}
    if k["pattern"] == "custom":
        pic = drawing if custom is None else custom
        colour = {}
        for y in range(H):
            for x in range(W):
                p = L.at(pic, x, y)
                if p[3] >= 0.5:
                    colour[(x, y)] = tuple(channel(p, i) >= 0.5 for i in range(3))
        groups = []
        for y in range(H):
            for x in range(W):
                if (x, y) not in colour or (x, y) in owner:
                    continue
                n = len(groups)
                stack, cells = [(x, y)], []
                owner[(x, y)] = n
                while stack:
                    cx, cy = stack.pop()
                    cells.append((cx, cy))
                    for nx, ny in ((cx + 1, cy), (cx - 1, cy), (cx, cy + 1), (cx, cy - 1)):
                        if colour.get((nx, ny)) == colour[(x, y)] and (nx, ny) not in owner:
                            owner[(nx, ny)] = n
                            stack.append((nx, ny))
                groups.append((sorted(cells, key=lambda c: (c[1], c[0])), colour[(x, y)] == (True,) * 3))
    else:
        keys, groups = {}, []
        for y in range(H):
            for x in range(W):
                t = tile(k["pattern"], *pattern_point(k, x, y))
                if t not in keys:
                    keys[t] = len(groups)
                    groups.append(([], False))
                groups[keys[t]][0].append((x, y))
    out = []
    for cells, white in groups:
        bx = sum(c[0] + 0.5 for c in cells) / len(cells)
        by = sum(c[1] + 0.5 for c in cells) / len(cells)
        out.append((cells, bx, by, white))
    return out


TIMED = ("force_1_position", "force_1_depth", "force_1_radius", "force_1_strength",
         "force_2_position", "force_2_depth", "force_2_radius", "force_2_strength",
         "shatter_threshold", "rotation_speed", "randomness", "viscosity", "mass_variance",
         "gravity", "gravity_direction", "gravity_inclination")


def lerp(a, b, f):
    if isinstance(a, list):
        return [lerp(p, q, f) for p, q in zip(a, b)]
    return a + (b - a) * f


def at_step(history, m):
    f, part = divmod(m, 4)
    a = history[f]
    b = history[f + 1] if f + 1 < len(history) else a
    return {n: lerp(a[n], b[n], part / 4) for n in TIMED}


def centre(s, f):
    p = s[f"force_{f}_position"]
    return [p[0] * W / 100 - W / 2, p[1] * H / 100 - H / 2, -s[f"force_{f}_depth"] * W]


def inside(s, f, cells):
    c, r = centre(s, f), s[f"force_{f}_radius"] * W
    if r <= 0:
        return False
    return any((x + 0.5 - W / 2 - c[0]) ** 2 + (y + 0.5 - H / 2 - c[1]) ** 2 + c[2] ** 2 < r * r
               for x, y in cells)


def value(k, grad, piece):
    if grad is None:
        return None
    _, bx, by, _ = piece
    v = K.luma(L.at(grad, min(fl(bx), W - 1), min(fl(by), H - 1)))
    return 1 - v if k["invert_gradient"] == "on" else v


def released(k, history, grad, piece):
    """Step 4: (step, force) of the release, or None."""
    if k["pattern"] == "custom" and k["white_tiles_fixed"] == "on" and piece[3]:
        return None
    v = value(k, grad, piece)
    for m in range(4 * (len(history) - 1) + 1):
        s = at_step(history, m)
        if v is not None and v + s["shatter_threshold"] / 100 < 1:
            continue
        for f in (1, 2):
            if inside(s, f, piece[0]):
                return m, f
    return None


def axis_of(k, i):
    t = k["tumble_axis"]
    if t == "none":
        return None
    if t in ("x", "y", "z"):
        return [float(t == "x"), float(t == "y"), float(t == "z")]
    a = [2 * r_hash(i, 4) - 1, 2 * r_hash(i, 5) - 1, 2 * r_hash(i, 6) - 1]
    if t != "free":
        a = [a[j] if "xyz"[j] in t else 0.0 for j in range(3)]
    return CD.unit_vec(a)


def turn(axis, angle):
    if axis is None or angle == 0:
        return [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
    x, y, z = axis
    c, s = math.cos(angle), math.sin(angle)
    t = 1 - c
    return [[c + x * x * t, x * y * t - z * s, x * z * t + y * s],
            [y * x * t + z * s, c + y * y * t, y * z * t - x * s],
            [z * x * t - y * s, z * y * t + x * s, c + z * z * t]]


def flight(k, history, i, piece, rel):
    """Step 5: (R, x) of a piece."""
    if rel is None:
        return turn(None, 0), [0.0, 0.0, 0.0]
    m0, f = rel
    s0 = at_step(history, m0)
    _, bx, by, _ = piece
    b = [bx - W / 2, by - H / 2, 0.0]
    mass = 1 + s0["mass_variance"] / 100 * (r_hash(i, 2) - 0.5)
    speed = s0[f"force_{f}_strength"] * W * (1 + s0["randomness"] * (2 * r_hash(i, 1) - 1)) / mass
    away = [b[j] - centre(s0, f)[j] for j in range(3)]
    v = [speed * q for q in CD.unit_vec(away)]
    w = 2 * math.pi * s0["rotation_speed"] * (1 + s0["randomness"] * (2 * r_hash(i, 3) - 1))
    axis = axis_of(k, i)
    x, angle = [0.0, 0.0, 0.0], 0.0
    dt = 1 / (4 * FPS)
    for m in range(m0, 4 * (len(history) - 1)):
        s = at_step(history, m)
        d, c = math.radians(s["gravity_direction"]), math.radians(s["gravity_inclination"])
        G = s["gravity"] * W
        g = [G * math.sin(d) * math.cos(c), -G * math.cos(d) * math.cos(c), -G * math.sin(c)]
        kk = s["viscosity"]
        if kk > 1e-9:
            e = math.exp(-kk * dt)
            x = [x[j] + (v[j] - g[j] / kk) * (1 - e) / kk + g[j] * dt / kk for j in range(3)]
            v = [v[j] * e + g[j] * (1 - e) / kk for j in range(3)]
            angle += w * (1 - e) / kk
            w *= e
        else:
            x = [x[j] + v[j] * dt + g[j] * dt * dt / 2 for j in range(3)]
            v = [v[j] + g[j] * dt for j in range(3)]
            angle += w * dt
    return turn(axis, angle), x


def box_hit(o, d, i, j, e, grow=0.0):
    """Step 6 for one pixel box: (t, face) or None; face 'front', 'back', 'x', 'y'."""
    if e == 0:
        if d[2] == 0:
            return None
        t = -o[2] / d[2]
        if t <= 0:
            return None
        u, v = o[0] + t * d[0], o[1] + t * d[1]
        if i - grow <= u < i + 1 + grow and j - grow <= v < j + 1 + grow:
            return t, ("front" if d[2] > 0 else "back")
        return None
    lo, hi = [i - grow, j - grow, 0.0 - grow], [i + 1 + grow, j + 1 + grow, e + grow]
    enter, leave, axis = -math.inf, math.inf, None
    for a in range(3):
        if d[a] == 0:
            if not lo[a] <= o[a] <= hi[a]:
                return None
            continue
        t0, t1 = (lo[a] - o[a]) / d[a], (hi[a] - o[a]) / d[a]
        if t0 > t1:
            t0, t1 = t1, t0
        if t0 > enter:
            enter, axis = t0, a
        leave = min(leave, t1)
    if axis is None or enter >= leave or enter <= 0:
        return None
    face = {0: "x", 1: "y"}.get(axis) or ("front" if d[2] > 0 else "back")
    return enter, face


def local_ray(o, d, R, x, piece):
    _, bx, by, _ = piece
    b = [bx - W / 2, by - H / 2, 0.0]
    rel = [o[j] - b[j] - x[j] for j in range(3)]
    ol = CD.app_t(R, rel)
    dl = CD.app_t(R, d)
    return [ol[0] + bx, ol[1] + by, ol[2]], dl


def piece_hit(k, o, d, piece, R, x, grow=0.0):
    ol, dl = local_ray(o, d, R, x, piece)
    e = k["extrusion_depth"] * W
    best = None
    for (i, j) in piece[0]:
        h = box_hit(ol, dl, i, j, e, grow)
        if h is not None and (best is None or h[0] < best[0]):
            best = h
    if best is None:
        return None
    t, face = best
    return t, face, ol, dl


def picture_of(word, drawing, frame):
    return drawing if word == "" else map_picture(word, frame)


def textured(k, face, u, v, bx, pics):
    which = {"front": "front", "back": "back", "x": "side", "y": "side"}[face]
    mode = k[which + "_mode"]
    pic = pics[which]
    if which == "back":
        u = 2 * bx - u
    px = L.bilinear(pic, u, v)
    col = CD.colour(k["color"])
    if mode.startswith("color"):
        px = col + [1.0]
    elif mode.startswith("tinted_layer"):
        px = [px[j] * col[j] for j in range(3)] + [px[3]]
    if mode.endswith("_plus_opacity"):
        px = [q * k["opacity"] for q in px]
    return px


def lit(k, m, o, d, t, n, px):
    lp = k["light_position"]
    pl = CD.app_t(m, [lp[0] * W / 100 - W / 2, lp[1] * H / 100 - H / 2, -k["light_depth"] * W])
    h = [o[i] + t * d[i] for i in range(3)]
    lv = CD.unit_vec(pl if k["light_type"] == "distant" else [pl[i] - h[i] for i in range(3)])
    vv = CD.unit_vec([-q for q in d])
    hv = [lv[i] + vv[i] for i in range(3)]
    nl = CD.dot(n, lv)
    hn = math.sqrt(CD.dot(hv, hv))
    sp = 0.0
    if nl > 0 and hn > 0:
        sp = max(CD.dot(n, hv) / hn, 0.0) ** k["highlight_sharpness"]
    lc, i_ = CD.colour(k["light_color"]), k["light_intensity"]
    A_, D, S = k["ambient_light"], k["diffuse_reflection"], k["specular_reflection"]
    return [px[j] * (A_ + D * i_ * lc[j] * max(nl, 0.0)) + S * i_ * sp * lc[j] * px[3]
            for j in range(3)] + [px[3]], nl


def normal(face, dl, R):
    if face == "front":
        nl = [0.0, 0.0, -1.0]
    elif face == "back":
        nl = [0.0, 0.0, 1.0]
    else:
        a = 0 if face == "x" else 1
        nl = [0.0, 0.0, 0.0]
        nl[a] = -1.0 if dl[a] > 0 else 1.0
    return CD.app(R, nl)


def scene(k, history, drawing, maps, frame):
    """Steps 1 to 5: per piece (piece, R, x, freed)."""
    deck = pieces(k, drawing, maps["custom"])
    out = []
    for i, p in enumerate(deck):
        rel = released(k, history, maps["gradient"], p)
        R, x = flight(k, history, i, p, rel)
        freed = rel is not None
        if k["render"] == "layer" and freed or k["render"] == "pieces" and not freed:
            continue
        out.append((i, p, R, x))
    return out


def nearest(k, o, d, deck, grow=0.0):
    found = []
    for i, p, R, x in deck:
        h = piece_hit(k, o, d, p, R, x, grow)
        if h is not None:
            found.append((h[0], i, h, p, R))
    found.sort(key=lambda f: (f[0], f[1]))
    return found


def colour_of(k, m, o, d, hit, pics):
    t, i, (t_, face, ol, dl), p, R = hit
    u, v = ol[0] + t * dl[0], ol[1] + t * dl[1]
    px = textured(k, face, u, v, p[1], pics)
    return lit(k, m, o, d, t, normal(face, dl, R), px)


def shatter(k, history, drawing, maps, cam, frame):
    seen = CD.view(k, cam)
    if seen is None:
        return [list(CLEAR) for _ in range(W * H)]
    m, o, ray = seen
    deck = scene(k, history, drawing, maps, frame)
    pics = {w: maps[w] for w in ("front", "side", "back")}
    out = []
    for y in range(H):
        for x in range(W):
            d = ray(x, y)
            found = nearest(k, o, d, deck)
            out.append(list(CLEAR) if not found else colour_of(k, m, o, d, found[0], pics)[0])
    return out


# --- the cases ------------------------------------------------------------------------------

NUMBERS = ("repetitions", "direction", "origin", "extrusion_depth") + TIMED + (
    "opacity", "camera_x_rotation", "camera_y_rotation", "camera_z_rotation",
    "camera_xy_position", "camera_z_position", "focal_length", "light_intensity",
    "light_position", "light_depth", "ambient_light", "diffuse_reflection",
    "specular_reflection", "highlight_sharpness")
LAYERS = ("custom_shatter_map", "gradient_layer", "front_layer", "side_layer", "back_layer")
WORDS = ("view", "render", "pattern", "white_tiles_fixed", "invert_gradient", "tumble_axis",
         "color", "front_mode", "side_mode", "back_mode", "camera_system", "light_type",
         "light_color") + LAYERS
DEFAULTS = {
    "view": "rendered", "render": "all", "pattern": "bricks", "custom_shatter_map": "",
    "white_tiles_fixed": "off", "repetitions": 10, "direction": 0, "origin": [50, 50],
    "extrusion_depth": 0.2,
    "force_1_position": [50, 50], "force_1_depth": 0.1, "force_1_radius": 0.4,
    "force_1_strength": 5,
    "force_2_position": [50, 50], "force_2_depth": 0.1, "force_2_radius": 0,
    "force_2_strength": 5,
    "gradient_layer": "", "shatter_threshold": 0, "invert_gradient": "off",
    "rotation_speed": 0.2, "tumble_axis": "free", "randomness": 0.1, "viscosity": 0.1,
    "mass_variance": 30, "gravity": 3, "gravity_direction": 180, "gravity_inclination": 0,
    "color": "#ffffff", "opacity": 1, "front_mode": "layer", "side_mode": "tinted_layer",
    "back_mode": "layer", "front_layer": "", "side_layer": "", "back_layer": "",
    "camera_system": "camera_position", "camera_x_rotation": 0, "camera_y_rotation": 0,
    "camera_z_rotation": 0, "camera_xy_position": [50, 50], "camera_z_position": 1,
    "focal_length": 50, "light_type": "distant", "light_intensity": 1, "light_color": "#ffffff",
    "light_position": [50, 50], "light_depth": 1, "ambient_light": 0.25,
    "diffuse_reflection": 0.75, "specular_reflection": 0, "highlight_sharpness": 5}
assert set(DEFAULTS) == set(NUMBERS) | set(WORDS)


def case(shift=(0, 0), camera=None, in_frame=0, **kw):
    c = dict(DEFAULTS, **CD.PIN_DEFAULTS, shift=shift, camera=camera, in_frame=in_frame,
             given=set(kw))
    for n, v in kw.items():
        assert n in DEFAULTS or n in CD.PIN_DEFAULTS, n
        c[n] = v
    return c


def settings(c, frame):
    k = {n: value_at(c[n], frame) for n in NUMBERS + CD.PINS}
    k.update({w: c[w] for w in WORDS + ("auto_focal_length",)})
    return k


def inputs(c, frame):
    n0 = c["in_frame"]
    k = settings(c, frame)
    history = [settings(c, n0 + j) for j in range(frame - n0 + 1)]
    drawing = CD_decoded("shot")
    maps = {"custom": map_picture(c["custom_shatter_map"], frame),
            "gradient": map_picture(c["gradient_layer"], frame)}
    for w in ("front", "side", "back"):
        maps[w] = picture_of(c[w + "_layer"], drawing, frame)
    return k, history, drawing, maps, CD.cam_of(c)


def render(c, frame):
    if frame < c["in_frame"]:
        return [list(CLEAR) for _ in range(W * H)]
    out = L.pic(W, H, shatter(*inputs(c, frame), frame))
    dx, dy = CD.move_of(c)
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = CD.move_of(c)
    o = CD_decoded("shot")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


# Every piece freed at once and pushed gently away from a force half a layer toward the viewer,
# each at its own speed, so no two pieces lie at one depth.
PUSH = dict(force_1_radius=10, force_1_depth=0.5, force_1_strength=0.3, gravity=0, viscosity=0,
            randomness=0.6)
# The pieces turning in the picture's plane: their shapes.
SPIN = dict(PUSH, rotation_speed=0.5, tumble_axis="z", repetitions=3, origin=[48.3, 51.7])
# Thrown gently from the middle.
BURST = dict(force_1_strength=0.5, gravity=0.5, repetitions=4)
# Every piece freed, none thrown or turning: the picture moves whole.
STILL = dict(force_1_strength=0, gravity=0, viscosity=0, force_1_radius=10, randomness=0,
             rotation_speed=0)
TUMBLE_Y = dict(PUSH, rotation_speed=0.5, tumble_axis="y", repetitions=4)

CASES = {
    "FX-SH-001": ("As added: bricks ten across, a force in the middle; at frame 0 nothing has "
                  "moved yet, by frame 2 the middle pieces fly and fall.", case(), (0, 2)),
    "FX-SH-002": ("Force 1 radius 0: nothing freed, the picture unchanged.",
                  case(force_1_radius=0), (0, 4)),
}
for n, p in enumerate(PATTERNS[:-1]):
    extra = dict(direction=20) if p in ("squares", "planks") else {}
    CASES[f"FX-SH-{3 + n:03d}"] = (
        f"Pattern {p.replace('_', ' ')}, three across, every piece freed in place and turning "
        "in the picture's plane at its own speed: the pieces' shapes.",
        case(pattern=p, **dict(SPIN, **extra)), (3,))
CASES.update({
    "FX-SH-013": ("Bricks turned 30 degrees about an origin at 20, 40 per cent.",
                  case(**dict(SPIN, pattern="bricks", direction=30, origin=[20, 40])), (3,)),
    "FX-SH-014": ("A custom map `cracks`: each region of one colour a piece, the clear corner no "
                  "piece at all.", case(pattern="custom", custom_shatter_map="cracks", **SPIN),
                  (3,)),
    "FX-SH-015": ("FX-SH-014 with White Tiles Fixed: the white pieces stay.",
                  case(pattern="custom", custom_shatter_map="cracks", white_tiles_fixed="on",
                       **SPIN), (3,)),
    "FX-SH-016": ("Custom with no map named: the layer's own drawing makes the pieces.",
                  case(pattern="custom", **SPIN), (3,)),
    "FX-SH-017": ("A force at 25, 50 per cent, radius 0.2, strength 1: only the left pieces fly.",
                  case(force_1_position=[25, 50], force_1_radius=0.2, force_1_strength=1,
                       repetitions=4), (0, 3)),
    "FX-SH-018": ("Force 2 at 80, 30 per cent, radius 0.15, strength -1, pulling its pieces in "
                  "and through.", case(force_1_position=[25, 50], force_1_radius=0.2,
                                       force_1_strength=1, force_2_position=[80, 30],
                                       force_2_radius=0.15, force_2_strength=-1,
                                       repetitions=4), (3,)),
    "FX-SH-019": ("Force 1 strength -2: the pieces drawn toward the force and past it.",
                  case(force_1_strength=-2, repetitions=4), (2,)),
    "FX-SH-020": ("Gradient layer `ramp`, threshold 50: only the pieces over the lighter half "
                  "of the ramp freed.", case(gradient_layer="ramp", shatter_threshold=50,
                                             force_1_radius=10, **BURST), (3,)),
    "FX-SH-021": ("FX-SH-020 with the gradient inverted: the darker half freed.",
                  case(gradient_layer="ramp", shatter_threshold=50, invert_gradient="on",
                       force_1_radius=10, **BURST), (3,)),
    "FX-SH-022": ("Force 1 swept from the left edge to the right over frames 0 to 7, radius "
                  "0.15: pieces freed one after another, each frame drawn alone.",
                  case(force_1_position=keyed((0, [0, 50]), (7, [100, 50])),
                       force_1_radius=0.15, **BURST), (0, 3, 7)),
    "FX-SH-023": ("Threshold keyed 0 to 100 over frames 0 to 7 with gradient `ramp`: the "
                  "lighter pieces go first.",
                  case(gradient_layer="ramp", shatter_threshold=keyed((0, 0), (7, 100)),
                       force_1_radius=10, **BURST), (0, 3, 7)),
    "FX-SH-024": ("Gravity 2 alone, every piece freed: the pieces fall.",
                  case(**dict(STILL, gravity=2, repetitions=4)), (3, 6)),
    "FX-SH-025": ("Gravity 2 to the right (direction 90).",
                  case(**dict(STILL, gravity=2, gravity_direction=90, repetitions=4)), (4,)),
    "FX-SH-026": ("Gravity 2 inclined 45 degrees toward the viewer: falling and nearing.",
                  case(**dict(STILL, gravity=2, gravity_inclination=45, repetitions=4)), (4,)),
    "FX-SH-027": ("Viscosity 4 with strength 2: thrown hard and soon slowed.",
                  case(force_1_strength=2, viscosity=4, gravity=0, repetitions=4), (4,)),
    "FX-SH-028": ("Viscosity 0 with strength 1: never slowed.",
                  case(force_1_strength=1, viscosity=0, gravity=0, repetitions=4), (4,)),
    "FX-SH-029": ("Gravity keyed 0 at frame 0 to 4 at frame 5, stepped four times a frame.",
                  case(**dict(STILL, gravity=keyed((0, 0), (5, 4)), repetitions=4)), (5,)),
})
for n, t in enumerate(TUMBLE):
    CASES[f"FX-SH-{30 + n:03d}"] = (
        f"Tumble axis {t}, every piece freed in place and turning with the back layer `back`.",
        case(**dict(TUMBLE_Y, tumble_axis=t, back_layer="back")), (3,))
CASES.update({
    "FX-SH-038": ("Randomness 0: every piece the same speed and spin.",
                  case(randomness=0, **BURST), (3,)),
    "FX-SH-039": ("Mass variance 100: speeds from two thirds to twice.",
                  case(mass_variance=100, **BURST), (3,)),
    "FX-SH-040": ("Render Layer: only the pieces not freed, holes where the others were.",
                  case(render="layer", force_1_radius=0.25, camera_xy_position=[51.3, 48.6], **BURST), (3,)),
    "FX-SH-041": ("Render Pieces: only the freed pieces.",
                  case(render="pieces", force_1_radius=0.25, camera_xy_position=[51.3, 48.6], **BURST), (3,)),
    "FX-SH-042": ("Front mode Color, colour #ffcc88: the freed fronts flat orange, lit.",
                  case(front_mode="color", color="#ffcc88", **TUMBLE_Y), (3,)),
    "FX-SH-043": ("Front mode Tinted Layer, colour #88ccff.",
                  case(front_mode="tinted_layer", color="#88ccff", **TUMBLE_Y), (3,)),
    "FX-SH-044": ("Side mode Color, colour #ff4040, extrusion 0.3: red sides.",
                  case(side_mode="color", color="#ff4040", extrusion_depth=0.3, **TUMBLE_Y),
                  (3,)),
    "FX-SH-045": ("Side mode Layer with the side layer `colours`.",
                  case(side_mode="layer", side_layer="colours", extrusion_depth=0.3,
                       **TUMBLE_Y), (3,)),
    "FX-SH-046": ("Back mode Color + Opacity, opacity 0.5.",
                  case(back_mode="color_plus_opacity", opacity=0.5, color="#40ff40",
                       **TUMBLE_Y), (3,)),
    "FX-SH-047": ("Front mode Layer + Opacity 0.5 with the front layer `colours`.",
                  case(front_mode="layer_plus_opacity", opacity=0.5, front_layer="colours",
                       **TUMBLE_Y), (3,)),
    "FX-SH-048": ("Front mode Tinted Layer + Opacity, opacity 0.7.",
                  case(front_mode="tinted_layer_plus_opacity", opacity=0.7, color="#ffcc88",
                       **TUMBLE_Y), (3,)),
    "FX-SH-049": ("Extrusion 0: flat pieces, no sides.",
                  case(extrusion_depth=0, back_layer="back", **TUMBLE_Y), (3,)),
    "FX-SH-050": ("Extrusion 0.5: thick pieces.", case(extrusion_depth=0.5, **TUMBLE_Y), (3,)),
    "FX-SH-051": ("At rest, the camera turned 30 degrees about y: the slabs' sides show at the "
                  "picture's edge and the back shows nothing.",
                  case(force_1_radius=0, camera_y_rotation=30, repetitions=4), (0,)),
    "FX-SH-052": ("Comp Camera with no camera in the file: D-58's default camera, as the "
                  "effect's own as added.", case(camera_system="comp_camera", **BURST), (3,)),
    "FX-SH-053": ("Comp Camera 12 pixels back (zoom 12): a strong perspective on the flight.",
                  case(camera_system="comp_camera", camera=(8, 4, -12, 12), **BURST), (3,)),
    "FX-SH-054": ("FX-SH-053 on the holder moved 3 right and 1 down.",
                  case(camera_system="comp_camera", camera=(8, 4, -12, 12), shift=(3, 1),
                       **BURST), (3,)),
    "FX-SH-055": ("A point light at the top left, depth 0.3, specular 1, sharpness 10.",
                  case(light_type="point", light_position=[10, 10], light_depth=0.3,
                       specular_reflection=1, highlight_sharpness=10, **TUMBLE_Y), (3,)),
    "FX-SH-056": ("Light colour #ffcc88, intensity 1.5, ambient 0.5, diffuse 0.25.",
                  case(light_color="#ffcc88", light_intensity=1.5, ambient_light=0.5,
                       diffuse_reflection=0.25, **TUMBLE_Y), (3,)),
    "FX-SH-057": ("A gradient layer not in the composition, `gone`: read as clear, every value "
                  "0, nothing passes threshold 50; and the warning.",
                  case(gradient_layer="gone", shatter_threshold=50, force_1_radius=10,
                       **BURST), (3,)),
    "FX-SH-058": ("The back layer with its in point at frame 4: clear backs before it.",
                  case(back_layer="late", **TUMBLE_Y), (3, 4)),
    "FX-SH-059": ("The holder starting at frame 2: time counts from its in point, so frame 2 is "
                  "at rest and frame 5 is FX-SH-017's frame 3.",
                  case(in_frame=2, force_1_position=[25, 50], force_1_radius=0.2,
                       force_1_strength=1, repetitions=4), (2, 5)),
    "FX-SH-060": ("As added at frame 6.", case(), (6,)),
})

INVALID = {
    "FX-SH-061": ("Pattern \"puzzle\", not built yet (D-636).", case(pattern="puzzle")),
    "FX-SH-062": ("Pattern \"lace\".", case(pattern="lace")),
    "FX-SH-063": ("View \"wireframe_front\", not built yet (D-635).",
                  case(view="wireframe_front")),
    "FX-SH-064": ("Render \"most\".", case(render="most")),
    "FX-SH-065": ("Tumble axis \"w\".", case(tumble_axis="w")),
    "FX-SH-066": ("Front mode \"glow\".", case(front_mode="glow")),
    "FX-SH-067": ("Camera system \"corner_pins\" with the pins as added (D-634): the camera "
                  "as added, so FX-SH-001's picture.", case(camera_system="corner_pins")),
    "FX-SH-068": ("Repetitions 0, below 1.", case(repetitions=0)),
    "FX-SH-069": ("Extrusion depth 2, above 1.", case(extrusion_depth=2)),
    "FX-SH-070": ("Force 1 radius -1, below 0.", case(force_1_radius=-1)),
    "FX-SH-071": ("Shatter threshold 101, above 100.", case(shatter_threshold=101)),
    "FX-SH-072": ("Gravity inclination 91, above 90.", case(gravity_inclination=91)),
    "FX-SH-073": ("Colour \"white\": not a #rrggbb colour.", case(color="white")),
    "FX-SH-074": ("Custom shatter map written as the number 3, not a word.",
                  case(custom_shatter_map=3)),
    "FX-SH-075": ("White tiles fixed \"yes\".", case(white_tiles_fixed="yes")),
    "FX-SH-076": ("Gravity keyed to 200 at frame 4.", case(gravity=keyed((0, 3), (4, 200)))),
    "FX-SH-077": ("Invert gradient \"maybe\".", case(invert_gradient="maybe")),
    "FX-SH-078": ("Mass variance 101, above 100.", case(mass_variance=101)),
}

# D-566 (1): FX-SH-067 pinned Corner Pins as refused while it was not built; D-634 builds it.
BUILT = {"FX-SH-067"}

# D-634: the Corner Pins camera, with Card Dance's pins.
PIN = dict(camera_system="corner_pins")
KEYSTONE = dict(PIN, upper_left_corner=[20, 10], upper_right_corner=[85, 0],
                lower_right_corner=[100, 95])
QUAD = dict(PIN, upper_left_corner=[5, 10], upper_right_corner=[90, -5],
            lower_right_corner=[100, 95], lower_left_corner=[-5, 105])
PIN_CASES = {
    "FX-SH-079": ("Corner Pins with the pins as added, thrown from the middle: FX-SH-052 "
                  "exactly.", case(**PIN, **BURST), (3,)),
    "FX-SH-080": ("Corner Pins, the upper corners pulled in to [20, 10] and [85, 0] and the "
                  "lower right up to [100, 95], Auto Focal Length on, thrown from the middle.",
                  case(auto_focal_length="on", **KEYSTONE, **BURST), (3,)),
    "FX-SH-081": ("Corner Pins on a general four-sided shape, Auto Focal Length off, the "
                  "pieces tumbling about y, with a point light at the top left, specular 1: the "
                  "light stays on the layer.",
                  case(light_type="point", light_position=[10, 10], light_depth=0.3,
                       specular_reflection=1, highlight_sharpness=10, **QUAD, **TUMBLE_Y),
                  (3,)),
}
PIN_INVALID = {
    "FX-SH-082": ("Lower right corner [50, 1001], above 1000.",
                  case(lower_right_corner=[50, 1001], **PIN)),
    "FX-SH-083": ("Auto focal length \"yes\".", case(auto_focal_length="yes", **PIN)),
}


def effect(fid, c):
    params = {w: c[w] for w in WORDS}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    params.update({n: setting_json(c[n]) for n in CD.PINS if n in c["given"]})
    if "auto_focal_length" in c["given"]:
        params["auto_focal_length"] = c["auto_focal_length"]
    return {"instance_id": fid, "type_id": "core.shatter", "enabled": True, "parameters": params}


def map_layers():
    return [L.raster(n, "asset-" + n, enabled=False, out_frame=FRAMES)
            for n in ("back", "ramp", "colours", "cracks")] + [
        L.raster("late", "asset-back", enabled=False, in_frame=4, out_frame=FRAMES)]


def project_json(fx, c):
    holder = L.raster("holder", "asset-shot", position=c["shift"], in_frame=c["in_frame"],
                      out_frame=FRAMES)
    holder["effects"] = [effect("fx-1", c)]
    comp = L.composition("comp-main", W, H, FRAMES, map_layers() + [holder])
    if c["camera"] is not None:
        x, y, depth, zoom = c["camera"]
        comp["camera"] = {"position": L.prop([x, y]), "depth": L.prop(depth),
                          "zoom": L.prop(zoom)}
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [comp]}


def write(name, p):
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        step_clear(fx, c, frames)
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write(f"{fx.lower().replace('-', '_')}.json",
                                                project_json(fx, c)), "frames": rendered}
        if "gone" in [c[w] for w in LAYERS]:
            entry["warning"] = "EFFECT_LAYER_MISSING"
        expected["cases"][fx] = entry
        before = plain(c)
        print(f"{fx}: " + ", ".join(f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} "
                                    "changed" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        if fx in BUILT:
            step_clear(fx, c, (0, 4))
            expected["cases"][fx] = {"says": says, "project": write(
                f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)),
                "frames": {str(f): render(c, f) for f in (0, 4)}}
            print(f"{fx}: built")
            continue
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)),
            "frames": {"0": before, "4": before}, "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    for fx, (says, c, frames) in PIN_CASES.items():
        step_clear(fx, c, frames)
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)),
            "frames": {str(f): render(c, f) for f in frames}}
        print(f"{fx}: built")
    for fx, (says, c) in PIN_INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)),
            "frames": {"0": before, "4": before}, "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_shatter.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def step_clear(fx, c, frames):
    """No pixel centre near a tile's edge, no piece near a sphere's surface or the threshold, no
    ray near a box's edge or two pieces tied, and no highlight near its cut."""
    for f in frames:
        if f < c["in_frame"]:
            continue
        k, history, drawing, maps, cam = inputs(c, f)
        if k["pattern"] != "custom":
            for y in range(H):
                for x in range(W):
                    t = tile(k["pattern"], *pattern_point(k, x, y))
                    for nudge in ((1e-7, 0), (-1e-7, 0), (0, 1e-7), (0, -1e-7)):
                        assert tile(k["pattern"], *pattern_point(k, x, y, nudge)) == t, \
                            (fx, x, y, nudge)
        deck = pieces(k, drawing, maps["custom"])
        for i, p in enumerate(deck):
            v = value(k, maps["gradient"], p)
            for m in range(4 * (len(history) - 1) + 1):
                s = at_step(history, m)
                if v is not None:
                    assert abs(v + s["shatter_threshold"] / 100 - 1) > 1e-6, (fx, f, i, m)
                for fo in (1, 2):
                    cc, r = centre(s, fo), s[f"force_{fo}_radius"] * W
                    if r <= 0:
                        continue
                    for x, y in p[0]:
                        dd = math.sqrt((x + 0.5 - W / 2 - cc[0]) ** 2
                                       + (y + 0.5 - H / 2 - cc[1]) ** 2 + cc[2] ** 2)
                        assert abs(dd - r) > 1e-6, (fx, f, i, m, fo, x, y)
        seen = CD.view(k, cam)
        if seen is None:
            continue
        mm, o, ray = seen
        sc = scene(k, history, drawing, maps, f)
        pics = {w: maps[w] for w in ("front", "side", "back")}
        for y in range(H):
            for x in range(W):
                d = ray(x, y)

                # A box's edge near the ray must not matter: the same piece and the same face
                # with every box a little larger or a little smaller.
                def pick(g):
                    found = nearest(k, o, d, sc, g)
                    return None if not found else (found[0][1], found[0][2][1])
                mid = pick(0.0)
                for g in (-1e-6, 1e-6):
                    assert pick(g) == mid, (fx, f, x, y, g)
                found = nearest(k, o, d, sc)
                if len(found) > 1 and found[1][0] - found[0][0] <= 1e-6:
                    assert found[0][0] == found[1][0] or max(
                        abs(p - q) for p, q in zip(colour_of(k, mm, o, d, found[0], pics)[0],
                                                   colour_of(k, mm, o, d, found[1], pics)[0])
                    ) < 1e-12, (fx, f, x, y)
                if found and k["specular_reflection"] > 0:
                    assert abs(colour_of(k, mm, o, d, found[0], pics)[1]) > 1e-6, (fx, f, x, y)


def near(a, b, eps=1e-12):
    return all(abs(x - y) <= eps for p, q in zip(a, b) for x, y in zip(p, q))


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    # At rest: the picture, to rounding.
    assert near(c["FX-SH-001"]["0"], drawn, 1e-9)
    assert not near(c["FX-SH-001"]["2"], drawn, 1e-6)
    assert near(c["FX-SH-002"]["0"], drawn, 1e-9) and near(c["FX-SH-002"]["4"], drawn, 1e-9)
    # Every pattern differs from every other.
    pats = [c[f"FX-SH-{n:03d}"]["3"] for n in range(3, 13)]
    for i in range(len(pats)):
        for j in range(i):
            assert not near(pats[i], pats[j], 1e-6), (i, j)
    # White tiles fixed changes the custom case; the custom map differs from the layer's own.
    assert not near(c["FX-SH-014"]["3"], c["FX-SH-015"]["3"], 1e-6)
    assert not near(c["FX-SH-014"]["3"], c["FX-SH-016"]["3"], 1e-6)
    # The sweep frees more as it goes; frame 0 of the sweep is at rest.
    assert near(c["FX-SH-022"]["0"], drawn, 1e-9)
    # Inverting the gradient changes which pieces go.
    assert not near(c["FX-SH-020"]["3"], c["FX-SH-021"]["3"], 1e-6)
    # Layer and Pieces split the picture between them: where one draws, the other is clear or
    # shows a piece.
    lay, pcs = c["FX-SH-040"]["3"], c["FX-SH-041"]["3"]
    assert any(p == CLEAR for p in lay) and any(p == CLEAR for p in pcs)
    # Tumble axes all differ, but none and z do not turn out of the plane alike.
    tum = [c[f"FX-SH-{n:03d}"]["3"] for n in range(30, 38)]
    for i in range(len(tum)):
        for j in range(i):
            assert not near(tum[i], tum[j], 1e-6), (i, j)
    # The no-camera Comp Camera is the effect's own camera.
    assert near(c["FX-SH-052"]["3"], render(case(**BURST), 3), 1e-9)
    # Moved under a fixed composition camera: seen from elsewhere, so a different picture.
    assert not near(c["FX-SH-054"]["3"], c["FX-SH-053"]["3"], 1e-6)
    # Time from the in point.
    assert near(c["FX-SH-059"]["2"], drawn, 1e-9)
    assert near(c["FX-SH-059"]["5"], c["FX-SH-017"]["3"], 1e-12)
    # Late back: clear backs at 3, the back at 4.
    assert c["FX-SH-058"]["3"] != c["FX-SH-058"]["4"]
    # A missing gradient: nothing passes, the picture.
    assert near(c["FX-SH-057"]["3"], drawn, 1e-9)
    # D-634: the pins as added are the camera as added; the pins change the picture.
    assert near(c["FX-SH-067"]["0"], drawn, 1e-9)
    assert near(c["FX-SH-067"]["4"], render(case(), 4), 1e-9)
    assert near(c["FX-SH-079"]["3"], c["FX-SH-052"]["3"], 1e-9)
    assert not near(c["FX-SH-080"]["3"], c["FX-SH-079"]["3"], 1e-6)
    print("checks passed")


if __name__ == "__main__":
    main()
