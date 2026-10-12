"""Card Dance, worked a second way.

D-632 adds Card Dance (`core.card_dance`), after After Effects' Card Dance (Simulation): the
layer cut into a grid of cards, each card moved, turned and scaled by numbers read from one of
two gradient layers under its centre, seen by a camera and lit. Adobe's CS5 help names the
controls and says what several do (Rows & Columns, Independent or Columns Follow Rows, up to 1000
each; a Back Layer for the cards' backs; two Gradient Layers; the Rotation Order and the
Transformation Order; for each of X, Y and Z Position, X, Y and Z Rotation and X and Y Scale a
Source, red, green, blue, intensity or alpha of either gradient layer, a Multiplier and an Offset;
white reads +1, black -1 and 50% grey 0, times the multiplier plus the offset; a brighter Z moves
a card toward the camera; Rotation in degrees; the Camera System and the lighting and material
groups it shares with Card Wipe). It publishes no formula and few settings when added, so the
rule, the units of Position and every setting when added are ours (document 21), and nothing is
ported. It carries no time: each frame reads only that frame's settings and layers.

The settings. `rows_columns` "independent" (when added) or "columns_follow_rows"; `rows` and
`columns` 1 to 1000, 8, each rounded to the nearest whole number (halves away from zero);
`back_layer` D-189's layer word, "" when added (clear backs), fitted to the layer as `stretch`;
`gradient_layer_1` and `gradient_layer_2` layer words, "" when added (the layer's own drawing),
fitted as `stretch`; `rotation_order` "xyz" (when added), "xzy", "yxz", "yzx", "zxy" or "zyx", the
turns in the order written; `transformation_order` "scale_rotate_position" (when added),
"scale_position_rotate", "rotate_scale_position", "rotate_position_scale",
"position_scale_rotate" or "position_rotate_scale". For each T of `x_position`, `y_position`,
`z_position`, `x_rotation`, `y_rotation`, `z_rotation`, `x_scale` and `y_scale`: `T_source`
"none", "red_1", "green_1", "blue_1", "intensity_1", "alpha_1", or the same ending "_2";
`T_multiplier` and `T_offset`, -100 to 100 card sizes for the positions (1 and 0 when added),
-3600 to 3600 degrees for the rotations (90 and 0), -10 to 10 for the scales (0.5 and 1). When
added every source is "none" but `z_position_source`, "intensity_1". The camera:
`camera_system` "camera_position" (when added), "comp_camera" or "corner_pins" (D-634); Card Wipe's `camera_x_rotation`,
`camera_y_rotation`, `camera_z_rotation` -3600 to 3600 degrees, 0; `camera_xy_position` x then y,
-1000 to 1000 per cent of the layer, [50, 50]; `camera_z_position` 0.01 to 10, 1;
`focal_length` 1 to 1000 mm on a 36 mm frame, 50; for "corner_pins" (D-634)
`upper_left_corner`, `upper_right_corner`, `lower_left_corner`, `lower_right_corner` x then y,
-1000 to 1000 per cent of the layer, when added the layer's own corners [0, 0], [100, 0],
[0, 100] and [100, 100], and `auto_focal_length` "off" (when added) or "on". Files from before
D-634 lack those five: they read as when added, and are written only when given. The light and material, Card Wipe's:
`light_type` "distant" (when added) or "point"; `light_intensity` 0 to 10, 1; `light_color`
#rrggbb, "#ffffff"; `light_position` x then y, -1000 to 1000 per cent, [50, 50]; `light_depth`
0 to 10 layer widths toward the viewer, 1; `ambient_light` 0.25, `diffuse_reflection` 0.75,
`specular_reflection` 0, each 0 to 2; `highlight_sharpness` 1 to 100, 5. Every number is keyable;
the words are exact.

The rule, in the layer's own W by H space, y down, z away from the viewer, the origin at its
middle C0 = (W/2, H/2, 0).

1. The cards. R = rows and C = columns, or C = R with "columns_follow_rows", each rounded. A card
   is cw = W/C by ch = H/R; card (a, b), column a and row b, has its home centre
   (cx, cy) = ((a + 1/2) cw, (b + 1/2) ch), q = (cx - W/2, cy - H/2, 0) from C0.
2. The sources. The gradient picture is the named layer's picture fitted to the layer, the
   layer's own drawing for "", a clear picture for a layer not in the composition (D-189's
   warning) or not yet begun. Its pixel (min(floor cx, W - 1), min(floor cy, H - 1)), premultiplied
   (r, g, b, a), gives v: red linear_to_srgb(clamp(r, 0, 1)), green and blue likewise, intensity
   document 21's luma (clamped, encoded), alpha a. A setting reads (2 v - 1) multiplier + offset,
   or the offset alone for "none".
3. The card's place. With the numbers px, py, pz, rx, ry, rz, sx, sy: the scale
   S = diag(sx, sy, 1); the turn R the product of Rx(rx), Ry(ry), Rz(rz) (each as in Card Wipe,
   Rx (x, y cos - z sin, y sin + z cos), Ry (x cos + z sin, y, -x sin + z cos), Rz (x cos - y sin,
   x sin + y cos, z)) with the first-written turn applied first (so "xyz" is Rz Ry Rx); the move
   P = (px cw, py ch, -pz ch) (a positive Z toward the viewer). Starting from A = I, B = 0, each
   step in the transformation order, applied first to last, is: scale A = S A, B = S B; rotate
   A = R A, B = R B; position B = B + P. The card's point (lx, ly), |lx| <= cw/2 and |ly| <= ch/2,
   is at A (lx, ly, 0) + B + q.
4. The camera. "camera_position": Card Wipe's: F = W focal/36, M = Rz Ry Rx of the camera's turns;
   the eye o = M^T ((xy_x W/100, xy_y H/100, -z F) - C0) and the ray through pixel (x, y)
   v = M^T (x + 1/2 - W/2, y + 1/2 - H/2, F). "comp_camera": the composition's camera, or D-58's
   default one where the file has none, given in the layer's own pixels as its place (ax, ay) and
   its distance A_l from the layer's plane (compose works them out, as for Particle Space): M = I,
   o = (ax - W/2, ay - H/2, -A_l), v = (x + 1/2 - ax, y + 1/2 - ay, A_l); nothing is drawn where
   A_l <= 0. The camera turns are not used. "corner_pins" (D-634, ours; Adobe gives no rule),
   `corner_camera`: the plane map Hs taking the unit square's corners (0, 0), (1, 0), (1, 1),
   (0, 1) to the four pins (p W/100 - W/2, q H/100 - H/2), upper left, upper right, lower right,
   lower left, in the closed form (with d1 = P1 - P2, d2 = P3 - P2, d3 = P0 - P1 + P2 - P3:
   g = det(d3, d2)/det(d1, d2), h = det(d1, d3)/det(d1, d2), its columns (P1 - P0 + g P1, g),
   (P3 - P0 + h P3, h) and (P0, 1)); nothing is drawn where det(d1, d2) = 0. The map from the
   layer's own centred point (u, v): h1 = Hs0/W, h2 = Hs1/H, h3 = (Hs0 + Hs1)/2 + Hs2. f: with
   `auto_focal_length` "on", sqrt(-(h1x h2x + h1y h2y)/(h1z h2z)) where h1z h2z is not 0 and
   that lies within Focal Length's range (W/36 to 1000 W/36, so a shape with next to no
   perspective is not thrown far off by rounding); otherwise F = W focal/36. a1, a2, t = (h1x/f, h1y/f, h1z), the same for h2
   and h3, each times s = 2/(|a1| + |a2|), its sign that of t's z; nothing is drawn where t's z
   is 0. a3 = unit(a1 x a2) sqrt(|a1| |a2|), A = [a1 a2 a3]; nothing is drawn where A has no
   inverse. o = -A^-1 t, v = A^-1 (x + 1/2 - W/2, y + 1/2 - H/2, f), M = I (the light stays on
   the layer). The camera turns, place, Z position and, with Auto, the focal length are not
   used. The layer's own corners with Auto "off" are the camera as added, exactly.
5. The hits. For each card, with A0 and A1 A's first two columns, solve
   lx A0 + ly A1 - t v = o - B - q; the card is missed where that has no single answer (an
   edge-on card, a scale of 0). A hit needs t > 0, |lx| <= cw/2 and |ly| <= ch/2. The nearest hit
   (least t) wins; where two tie exactly, the first card in rows then columns.
6. The colour. With N = A0 x A1 and s the sign of sx sy, the ray meets the front where
   s (v . N) > 0: the drawing at (cx + lx, cy + ly) by document 21's bilinear, transparent
   outside; otherwise the back: the back picture at (cx - lx, cy + ly), or clear with no back
   picture (none named, not in the composition, with the warning, or not begun).
7. The light, Card Wipe's: Pl = M^T ((lp_x W/100, lp_y H/100, -depth W) - C0); the hit
   h = o + t v; L = unit(Pl) for "distant", unit(Pl - h) for "point" (a zero step stays zero). The
   front's normal n = -s N/|N|, the back's its opposite. V = -v/|v|, Hv = L + V; the highlight
   sp = max(n.Hv/|Hv|, 0)^sharpness where n.L > 0 and |Hv| > 0, else 0. With i the intensity, Lc
   the light's linear colour, A, D, S the ambient, diffuse and specular numbers, on the sampled
   premultiplied (c, alpha):

       rgb = c (A + D i Lc max(n.L, 0)) + S i sp Lc alpha,  alpha unchanged.

8. The output at buffer pixel (X, Y), x = X - ox, y = Y - oy: the nearest hit's colour, or clear.

The layer never grows. A draft changes nothing, every distance being a share of the layer.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawings' 8-bit values, and tries every card for every pixel. The rule steps at a card's
edges, where two cards tie at differing colours, at edge-on cards and where the highlight is cut
at n.L = 0; every case is checked that no pixel lies near one.

Every case is a project of one composition 16 by 8, five frames at 24 a second, in
`Fixtures/card_dance/`: the map layers, every one switched off, and on top of them the layer
`holder`, showing the drawing `shot`, with the effect; some carry a composition camera. The
expected pixels are in `Fixtures/card_dance/expected_card_dance.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/card_dance_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import compound_blur_reference as K  # noqa: E402
import smooth_reference as SM  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "card_dance"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 8, 5
CLEAR = [0.0, 0.0, 0.0, 0.0]


def shot_drawing():
    """16 by 8: red, green, blue and cream bands of four columns, lighter down; the bottom right
    3 by 2 clear."""
    bands = [(220, 40, 40), (40, 180, 60), (50, 70, 220), (240, 225, 180)]

    def px(x, y):
        if x >= 13 and y >= 6:
            return (0, 0, 0, 0)
        r, g, b = bands[x // 4]
        k = 0.55 + 0.45 * y / 7 + 0.02 * (x % 4)
        return (min(255, round(r * k)), min(255, round(g * k)), min(255, round(b * k)), 255)
    return [[px(x, y) for x in range(W)] for y in range(H)]


def back_drawing():
    """8 by 4: red rising across, blue rising down, green steady; a white marker at (0, 0)."""
    return [[(255, 255, 255, 255) if (x, y) == (0, 0) else
             (round(40 + 200 * x / 7), 150, round(40 + 200 * y / 3), 255)
             for x in range(8)] for y in range(4)]


def ramp_drawing():
    """16 by 8 greys: lighter down the picture, a little darker to the right."""
    grey = lambda x, y: round(255 * (0.7 * y / 7 + 0.3 * (15 - x) / 15))  # noqa: E731
    return [[(grey(x, y),) * 3 + (255,) for x in range(W)] for y in range(H)]


def colours_drawing():
    """16 by 8: red rising across, green rising down, blue falling across, alpha falling down."""
    return [[(round(30 + 220 * x / 15), round(20 + 230 * y / 7), round(240 - 200 * x / 15),
              round(255 - 150 * y / 7)) for x in range(W)] for y in range(H)]


DRAWINGS = {"shot": shot_drawing(), "back": back_drawing(), "ramp": ramp_drawing(),
            "colours": colours_drawing()}
MAPS = {"back": "back", "ramp": "ramp", "colours": "colours", "late": "back"}


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


def map_picture(name, frame):
    """A map layer's picture fitted to the layer; None for none named; a clear picture for a
    layer not in the composition or not begun."""
    if name == "":
        return None
    if name not in MAPS or (name == "late" and frame < 4):
        return L.pic(W, H)
    return L.fit(decoded(MAPS[name]), "stretch", W, H)


# --- the rule -------------------------------------------------------------------------------

TRANSFORMS = ("x_position", "y_position", "z_position", "x_rotation", "y_rotation",
              "z_rotation", "x_scale", "y_scale")
SOURCES = ("none",) + tuple(f"{c}_{n}" for n in (1, 2)
                            for c in ("red", "green", "blue", "intensity", "alpha"))
ROTATION_ORDERS = ("xyz", "xzy", "yxz", "yzx", "zxy", "zyx")
TRANSFORMATION_ORDERS = ("scale_rotate_position", "scale_position_rotate",
                         "rotate_scale_position", "rotate_position_scale",
                         "position_scale_rotate", "position_rotate_scale")


def rnd(v):
    return math.floor(v + 0.5)


def mul(a, b):
    return [[sum(a[r][i] * b[i][c] for i in range(3)) for c in range(3)] for r in range(3)]


def app(m, v):
    return [sum(m[r][i] * v[i] for i in range(3)) for r in range(3)]


def app_t(m, v):
    return [sum(m[i][r] * v[i] for i in range(3)) for r in range(3)]


def rot(axis, deg):
    t = math.radians(deg)
    c, s = math.cos(t), math.sin(t)
    return {"x": [[1, 0, 0], [0, c, -s], [0, s, c]],
            "y": [[c, 0, s], [0, 1, 0], [-s, 0, c]],
            "z": [[c, -s, 0], [s, c, 0], [0, 0, 1]]}[axis]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dot(a, b):
    return sum(p * q for p, q in zip(a, b))


def unit_vec(v):
    n = math.sqrt(dot(v, v))
    return [0.0, 0.0, 0.0] if n == 0 else [q / n for q in v]


def source_value(word, pixels):
    """Step 2's v from the two pixels under the card's centre."""
    channel, n = word.rsplit("_", 1)
    p = pixels[int(n) - 1]
    if channel == "alpha":
        return p[3]
    if channel == "intensity":
        return K.luma(p)
    return SM.linear_to_srgb(min(1.0, max(0.0, p[{"red": 0, "green": 1, "blue": 2}[channel]])))


def cards(k, drawing, grads):
    """Steps 1 to 3: per card (cx, cy, cw, ch, A, B, s), rows then columns."""
    rows = rnd(k["rows"])
    cols = rows if k["rows_columns"] == "columns_follow_rows" else rnd(k["columns"])
    cw, ch = W / cols, H / rows
    pics = [drawing if g is None else g for g in grads]
    out = []
    for b in range(rows):
        for a in range(cols):
            cx, cy = (a + 0.5) * cw, (b + 0.5) * ch
            gx, gy = min(math.floor(cx), W - 1), min(math.floor(cy), H - 1)
            pixels = [L.at(p, gx, gy) for p in pics]
            num = {}
            for t in TRANSFORMS:
                word = k[t + "_source"]
                g = 0.0 if word == "none" else 2 * source_value(word, pixels) - 1
                num[t] = g * k[t + "_multiplier"] + k[t + "_offset"] if word != "none" else k[t + "_offset"]
            sx, sy = num["x_scale"], num["y_scale"]
            S = [[sx, 0, 0], [0, sy, 0], [0, 0, 1]]
            R = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
            for axis in k["rotation_order"]:
                R = mul(rot(axis, num[axis + "_rotation"]), R)
            P = [num["x_position"] * cw, num["y_position"] * ch, -num["z_position"] * ch]
            A, B = [[1, 0, 0], [0, 1, 0], [0, 0, 1]], [0.0, 0.0, 0.0]
            for step in k["transformation_order"].split("_"):
                if step == "scale":
                    A, B = mul(S, A), app(S, B)
                elif step == "rotate":
                    A, B = mul(R, A), app(R, B)
                else:
                    B = [B[i] + P[i] for i in range(3)]
            s = (sx * sy > 0) - (sx * sy < 0)
            out.append((cx, cy, cw, ch, A, B, s))
    return out


def view(k, cam):
    """Step 4: the turn M, the eye o and the ray for a pixel; None where nothing is drawn."""
    if k["camera_system"] == "comp_camera":
        ax, ay, al = cam
        if al <= 0:
            return None
        ident = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
        return ident, [ax - W / 2, ay - H / 2, -al], lambda x, y: [x + 0.5 - ax, y + 0.5 - ay, al]
    if k["camera_system"] == "corner_pins":
        seen = corner_camera(k, W, H)
        return None if seen is None else ([[1, 0, 0], [0, 1, 0], [0, 0, 1]],) + seen
    m = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    for axis in "xyz":
        m = mul(rot(axis, k["camera_" + axis + "_rotation"]), m)
    f = W * k["focal_length"] / 36
    xy = k["camera_xy_position"]
    o = app_t(m, [xy[0] * W / 100 - W / 2, xy[1] * H / 100 - H / 2, -k["camera_z_position"] * f])
    return m, o, lambda x, y: app_t(m, [x + 0.5 - W / 2, y + 0.5 - H / 2, f])


PINS = ("upper_left_corner", "upper_right_corner", "lower_left_corner", "lower_right_corner")
PIN_DEFAULTS = {"upper_left_corner": [0, 0], "upper_right_corner": [100, 0],
                "lower_left_corner": [0, 100], "lower_right_corner": [100, 100],
                "auto_focal_length": "off"}


def inverse3(a):
    """The inverse of a 3 by 3, or None where its determinant is 0."""
    c = [[a[(r + 1) % 3][(q + 1) % 3] * a[(r + 2) % 3][(q + 2) % 3]
          - a[(r + 1) % 3][(q + 2) % 3] * a[(r + 2) % 3][(q + 1) % 3] for q in range(3)]
         for r in range(3)]
    det = sum(a[0][q] * c[0][q] for q in range(3))
    if det == 0:
        return None
    return [[c[q][r] / det for q in range(3)] for r in range(3)]


def corner_camera(k, w, h):
    """D-634: the eye o and the ray for a pixel from the four pins, on a w by h layer; None
    where nothing is drawn."""
    p0, p1, p3, p2 = ([k[n][0] * w / 100 - w / 2, k[n][1] * h / 100 - h / 2] for n in PINS)
    d1 = [p1[i] - p2[i] for i in range(2)]
    d2 = [p3[i] - p2[i] for i in range(2)]
    d3 = [p0[i] - p1[i] + p2[i] - p3[i] for i in range(2)]
    den = d1[0] * d2[1] - d2[0] * d1[1]
    if den == 0:
        return None
    g = (d3[0] * d2[1] - d2[0] * d3[1]) / den
    hh = (d1[0] * d3[1] - d3[0] * d1[1]) / den
    c0 = [p1[0] - p0[0] + g * p1[0], p1[1] - p0[1] + g * p1[1], g]
    c1 = [p3[0] - p0[0] + hh * p3[0], p3[1] - p0[1] + hh * p3[1], hh]
    c2 = [p0[0], p0[1], 1.0]
    h1 = [q / w for q in c0]
    h2 = [q / h for q in c1]
    h3 = [(c0[i] + c1[i]) / 2 + c2[i] for i in range(3)]
    f = w * k["focal_length"] / 36
    if k["auto_focal_length"] == "on" and h1[2] * h2[2] != 0:
        ff = -(h1[0] * h2[0] + h1[1] * h2[1]) / (h1[2] * h2[2])
        if (w / 36) ** 2 <= ff <= (1000 * w / 36) ** 2:
            f = math.sqrt(ff)
    a1, a2, tt = ([q[0] / f, q[1] / f, q[2]] for q in (h1, h2, h3))
    if tt[2] == 0:
        return None
    n1, n2 = math.sqrt(dot(a1, a1)), math.sqrt(dot(a2, a2))
    s = 2 / (n1 + n2) * (1 if tt[2] > 0 else -1)
    a1, a2, tt = ([q * s for q in v] for v in (a1, a2, tt))
    n1, n2 = abs(s) * n1, abs(s) * n2
    a3 = [q * math.sqrt(n1 * n2) for q in unit_vec(cross(a1, a2))]
    ai = inverse3([[a1[r], a2[r], a3[r]] for r in range(3)])
    if ai is None:
        return None
    o = [-q for q in app(ai, tt)]
    return o, lambda x, y: app(ai, [x + 0.5 - w / 2, y + 0.5 - h / 2, f])


def det3(c0, c1, c2):
    return dot(c0, cross(c1, c2))


def hits(deck, o, d, grow=0.0):
    """Step 5: every hit, as (t, index, card, lx, ly); `grow` widens every card a little, for
    the check that no pixel lies near an edge."""
    found = []
    for i, (cx, cy, cw, ch, A, B, s) in enumerate(deck):
        a0, a1 = [A[r][0] for r in range(3)], [A[r][1] for r in range(3)]
        nv = [-q for q in d]
        rhs = [o[0] - B[0] - (cx - W / 2), o[1] - B[1] - (cy - H / 2), o[2] - B[2]]
        det = det3(a0, a1, nv)
        if det == 0:
            continue
        lx = det3(rhs, a1, nv) / det
        ly = det3(a0, rhs, nv) / det
        t = det3(a0, a1, rhs) / det
        if t <= 0:
            continue
        if abs(lx) <= cw / 2 + grow and abs(ly) <= ch / 2 + grow:
            found.append((t, i, deck[i], lx, ly))
    return found


def colour(hexa):
    return [srgb_to_linear(int(hexa[i:i + 2], 16) / 255) for i in (1, 3, 5)]


def normal_light(k, m, o, d, hit):
    t, _, (cx, cy, cw, ch, A, B, s), lx, ly = hit
    a0, a1 = [A[r][0] for r in range(3)], [A[r][1] for r in range(3)]
    nn = cross(a0, a1)
    front = s * dot(d, nn) > 0
    n = [-s * q for q in unit_vec(nn)]
    if not front:
        n = [-q for q in n]
    lp = k["light_position"]
    pl = app_t(m, [lp[0] * W / 100 - W / 2, lp[1] * H / 100 - H / 2, -k["light_depth"] * W])
    h = [o[i] + t * d[i] for i in range(3)]
    lv = unit_vec(pl if k["light_type"] == "distant" else [pl[i] - h[i] for i in range(3)])
    return front, n, lv


def shade(k, m, o, d, hit, drawing, back):
    """Steps 6 and 7 for the winning hit."""
    t, _, (cx, cy, cw, ch, A, B, s), lx, ly = hit
    front, n, lv = normal_light(k, m, o, d, hit)
    if front:
        px = L.bilinear(drawing, cx + lx, cy + ly)
    elif back is None:
        px = list(CLEAR)
    else:
        px = L.bilinear(back, cx - lx, cy + ly)
    vv = unit_vec([-q for q in d])
    hv = [lv[i] + vv[i] for i in range(3)]
    nl = dot(n, lv)
    hn = math.sqrt(dot(hv, hv))
    sp = 0.0
    if nl > 0 and hn > 0:
        sp = max(dot(n, hv) / hn, 0.0) ** k["highlight_sharpness"]
    lc, i_ = colour(k["light_color"]), k["light_intensity"]
    A_, D, S = k["ambient_light"], k["diffuse_reflection"], k["specular_reflection"]
    return [px[j] * (A_ + D * i_ * lc[j] * max(nl, 0.0)) + S * i_ * sp * lc[j] * px[3]
            for j in range(3)] + [px[3]]


def card_dance(k, drawing, back, grads, cam):
    seen = view(k, cam)
    if seen is None:
        return [list(CLEAR) for _ in range(W * H)]
    m, o, ray = seen
    deck = cards(k, drawing, grads)
    out = []
    for y in range(H):
        for x in range(W):
            d = ray(x, y)
            found = hits(deck, o, d)
            if not found:
                out.append(list(CLEAR))
                continue
            best = min(found, key=lambda h: (h[0], h[1]))
            out.append(shade(k, m, o, d, best, drawing, back))
    return out


# --- the cases ------------------------------------------------------------------------------

MULT = {"position": 1, "rotation": 90, "scale": 0.5}
OFFS = {"position": 0, "rotation": 0, "scale": 1}
RANGE = {"position": 100, "rotation": 3600, "scale": 10}

NUMBERS = (("rows", "columns") + tuple(f"{t}_{w}" for t in TRANSFORMS for w in ("multiplier", "offset"))
           + ("camera_x_rotation", "camera_y_rotation", "camera_z_rotation", "camera_xy_position",
              "camera_z_position", "focal_length", "light_intensity", "light_position",
              "light_depth", "ambient_light", "diffuse_reflection", "specular_reflection",
              "highlight_sharpness"))
WORDS = (("rows_columns", "back_layer", "gradient_layer_1", "gradient_layer_2", "rotation_order",
          "transformation_order") + tuple(t + "_source" for t in TRANSFORMS)
         + ("camera_system", "light_type", "light_color"))
DEFAULTS = dict(
    {"rows_columns": "independent", "rows": 8, "columns": 8, "back_layer": "",
     "gradient_layer_1": "", "gradient_layer_2": "", "rotation_order": "xyz",
     "transformation_order": "scale_rotate_position", "camera_system": "camera_position",
     "camera_x_rotation": 0, "camera_y_rotation": 0, "camera_z_rotation": 0,
     "camera_xy_position": [50, 50], "camera_z_position": 1, "focal_length": 50,
     "light_type": "distant", "light_intensity": 1, "light_color": "#ffffff",
     "light_position": [50, 50], "light_depth": 1, "ambient_light": 0.25,
     "diffuse_reflection": 0.75, "specular_reflection": 0, "highlight_sharpness": 5},
    **{t + "_source": ("intensity_1" if t == "z_position" else "none") for t in TRANSFORMS},
    **{t + "_multiplier": MULT[t.split("_")[1]] for t in TRANSFORMS},
    **{t + "_offset": OFFS[t.split("_")[1]] for t in TRANSFORMS})
assert set(DEFAULTS) == set(NUMBERS) | set(WORDS)


def case(shift=(0, 0), camera=None, **kw):
    """Every setting as added, then the case's; `camera` is (x, y, depth, zoom) for a
    composition camera whose plane at depth 0 is drawn 1:1."""
    c = dict(DEFAULTS, **PIN_DEFAULTS, shift=shift, camera=camera, given=set(kw))
    for n, v in kw.items():
        assert n in DEFAULTS or n in PIN_DEFAULTS, n
        c[n] = v
    return c


def grid(**kw):
    """Four columns and two rows, nothing moving unless the case says, so no card's edge falls
    on a pixel's centre."""
    return case(**dict(dict(rows=2, columns=4, z_position_source="none"), **kw))


def settings(c, frame):
    k = {n: value_at(c[n], frame) for n in NUMBERS + PINS}
    k.update({w: c[w] for w in WORDS + ("auto_focal_length",)})
    return k


def cam_of(c):
    """What compose works out for the composition camera, in the layer's own pixels: the
    file's camera, or D-58's default (zoom W 50/36, that far back, at the middle)."""
    if c["camera"] is None:
        z = W * 50 / 36
        x, y, depth = W / 2, H / 2, -z
    else:
        x, y, depth, zoom = c["camera"]
        # Only cameras that draw the layer's plane 1:1, or one on the plane, which draws
        # nothing whatever its zoom (D-58 refuses a zoom of 0).
        assert zoom == -depth or depth == 0, "only cameras that draw the layer's plane 1:1"
    return (x - c["shift"][0], y - c["shift"][1], 0 - depth)


def inputs(c, frame):
    k = settings(c, frame)
    return (k, decoded("shot"), map_picture(c["back_layer"], frame),
            [map_picture(c["gradient_layer_1"], frame), map_picture(c["gradient_layer_2"], frame)],
            cam_of(c))


def move_of(c):
    dx, dy = c["shift"]
    if c["camera"] is not None:
        assert c["camera"][0] == int(c["camera"][0]) and c["camera"][1] == int(c["camera"][1])
        dx, dy = dx + int(W / 2 - c["camera"][0]), dy + int(H / 2 - c["camera"][1])
    return dx, dy


def render(c, frame):
    out = L.pic(W, H, card_dance(*inputs(c, frame)))
    dx, dy = move_of(c)
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = move_of(c)
    o = decoded("shot")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


BACK = dict(back_layer="back")
SPIN = dict(z_rotation_offset=30, x_rotation_offset=40, y_rotation_offset=-25)
MIX = dict(x_scale_offset=0.6, y_scale_offset=0.8, z_rotation_offset=30, x_position_offset=0.3,
           y_position_offset=-0.2)
DEPTH = dict(z_position_source="intensity_1", z_position_multiplier=0.4, gradient_layer_1="ramp")

CASES = {
    "FX-CD-001": ("As added: eight by eight cards, each moved toward the viewer by the "
                  "brightness of the layer's own picture under its centre (white one card "
                  "height nearer, black one farther).", case(), (0,)),
    "FX-CD-002": ("Four by two cards with every source none: the picture unchanged.", grid(),
                  (0,)),
    "FX-CD-003": ("Z position from intensity 1 of the gradient layer `ramp`, multiplier 0.4: "
                  "the brighter (lower left) cards nearer and larger.", grid(**DEPTH), (0,)),
    "FX-CD-004": ("X position offset 0.25: every card a quarter of a card to the right.",
                  grid(x_position_offset=0.25), (0,)),
    "FX-CD-005": ("Y position from green 1 of `colours`, multiplier 0.3: cards lower where the "
                  "gradient is greener.", grid(y_position_source="green_1",
                                               gradient_layer_1="colours",
                                               y_position_multiplier=0.3), (0,)),
    "FX-CD-006": ("X rotation offset 180 with the back layer: every card turned over top to "
                  "bottom, showing its share of the back mirrored across.",
                  grid(x_rotation_offset=180, **BACK), (0,)),
    "FX-CD-007": ("Y rotation offset 180 with the back layer: the back picture whole, upright, "
                  "its white marker at the top left.", grid(y_rotation_offset=180, **BACK),
                  (0,)),
    "FX-CD-008": ("Y rotation offset 180 without a back layer: every card's back clear.",
                  grid(y_rotation_offset=180), (0,)),
    "FX-CD-009": ("Z rotation from red 1 of `colours`, multiplier 60: cards turned in the "
                  "picture's plane, more to the right.",
                  grid(z_rotation_source="red_1", gradient_layer_1="colours",
                       z_rotation_multiplier=60), (0,)),
    "FX-CD-010": ("X scale offset 0.5: half-width cards with gaps between them.",
                  grid(x_scale_offset=0.5), (0,)),
    "FX-CD-011": ("Y scale from blue 2 of `colours` as gradient layer 2, multiplier 0.4: "
                  "taller cards to the left.", grid(y_scale_source="blue_2",
                                                    gradient_layer_2="colours",
                                                    y_scale_multiplier=0.4), (0,)),
    "FX-CD-012": ("X scale from alpha 1 of `colours`, multiplier 0.5, offset 0.5: narrower "
                  "cards where the gradient is clearer (lower).",
                  grid(x_scale_source="alpha_1", gradient_layer_1="colours",
                       x_scale_multiplier=0.5, x_scale_offset=0.5), (0,)),
    "FX-CD-013": ("Z position from intensity 2 with gradient layer 2 `ramp`.",
                  grid(z_position_source="intensity_2", z_position_multiplier=0.4,
                       gradient_layer_2="ramp"), (0,)),
    "FX-CD-014": ("Three turns, rotation order xyz, with the back layer.",
                  grid(**SPIN, **BACK), (0,)),
}
for i, order in enumerate(ROTATION_ORDERS[1:]):
    CASES[f"FX-CD-{15 + i:03d}"] = (f"FX-CD-014 with rotation order {order}.",
                                    grid(rotation_order=order, **SPIN, **BACK), (0,))
for i, order in enumerate(TRANSFORMATION_ORDERS):
    words = order.replace("_", ", ")
    CASES[f"FX-CD-{20 + i:03d}"] = (f"Scale 0.6 by 0.8, turned 30 about z, moved 0.3 right "
                                    f"and 0.2 up, in the order {words}.",
                                    grid(transformation_order=order, **MIX), (0,))
CASES.update({
    "FX-CD-026": ("Scale -1 across: each card mirrored left to right, still its front.",
                  grid(x_scale_offset=-1, **BACK), (0,)),
    "FX-CD-027": ("Overlapping cards: scale 1.4 at the same depth, the first card in rows "
                  "then columns on top.", grid(x_scale_offset=1.4, y_scale_offset=1.4), (0,)),
    "FX-CD-028": ("Z position offset 10 card heights toward the viewer: every card behind the "
                  "camera, nothing drawn.", grid(z_position_offset=10), (0,)),
    "FX-CD-029": ("Columns follow rows, rows 3 (columns 4 ignored), with depth from `ramp`.",
                  case(rows=3, columns=4, rows_columns="columns_follow_rows", **DEPTH), (0,)),
    "FX-CD-030": ("The camera turned 30 degrees about y, with depth from `ramp`.",
                  grid(camera_y_rotation=30, **DEPTH, **BACK), (0,)),
    "FX-CD-031": ("The camera turned -20 about x and 15 about z.",
                  grid(camera_x_rotation=-20, camera_z_rotation=15, **DEPTH), (0,)),
    "FX-CD-032": ("The camera at x 40, y 60 per cent.",
                  grid(camera_xy_position=[40, 60], **DEPTH), (0,)),
    "FX-CD-033": ("Camera Z position 1.7 and focal length 20, turned 25 about y.",
                  grid(camera_z_position=1.7, focal_length=20, camera_y_rotation=25, **DEPTH),
                  (0,)),
    "FX-CD-034": ("Comp Camera with no camera in the file: D-58's default camera, the same as "
                  "the effect's own camera as added; the effect's camera turns are not used.",
                  grid(camera_system="comp_camera", camera_y_rotation=30, **DEPTH), (0,)),
    "FX-CD-035": ("Comp Camera with the composition camera 12 pixels back (zoom 12): a strong "
                  "perspective on the cards.", grid(camera_system="comp_camera",
                                                    camera=(8, 4, -12, 12), **DEPTH), (0,)),
    "FX-CD-036": ("Comp Camera with the composition camera 2 to the right: the cards seen from "
                  "the right and the whole picture moved 2 left.",
                  grid(camera_system="comp_camera", camera=(10, 4, -12, 12), **DEPTH), (0,)),
    "FX-CD-037": ("FX-CD-035 on the holder moved 3 right and 1 down: the camera sees the layer "
                  "from its upper left.", grid(camera_system="comp_camera",
                                               camera=(8, 4, -12, 12), shift=(3, 1), **DEPTH),
                  (0,)),
    "FX-CD-038": ("Comp Camera with the composition camera on the layer's plane (depth 0, "
                  "zoom 12): nothing drawn.", grid(camera_system="comp_camera",
                                                   camera=(8, 4, 0, 12),
                                         **DEPTH), (0,)),
    "FX-CD-039": ("A distant light from the top left, depth 0.5, on turned cards.",
                  grid(light_position=[0, 0], light_depth=0.5, **SPIN, **BACK), (0,)),
    "FX-CD-040": ("A point light in the middle, depth 0.3.",
                  grid(light_type="point", light_depth=0.3, **SPIN, **BACK), (0,)),
    "FX-CD-041": ("Specular 1, sharpness 20: a highlight on the turned cards.",
                  grid(specular_reflection=1, highlight_sharpness=20, light_position=[20, 30],
                       **SPIN, **BACK), (0,)),
    "FX-CD-042": ("Light colour #ffcc88 at intensity 1.5, ambient 0.5, diffuse 0.25.",
                  grid(light_color="#ffcc88", light_intensity=1.5, ambient_light=0.5,
                       diffuse_reflection=0.25, **SPIN, **BACK), (0,)),
    "FX-CD-043": ("Y rotation offset keyed 0 at frame 0 to 180 at frame 4, with the back "
                  "layer: frame 0 the picture, frame 4 FX-CD-007.",
                  grid(y_rotation_offset=keyed((0, 0), (4, 180)), **BACK), (0, 2, 4)),
    "FX-CD-044": ("FX-CD-003 on the holder moved 3 right and 1 down: the same, moved.",
                  grid(shift=(3, 1), **DEPTH), (0,)),
    "FX-CD-045": ("A gradient layer not in the composition, `gone`: read as clear, so every "
                  "card at -1 times the multiplier, 0.4 farther; and the warning every frame.",
                  grid(z_position_source="intensity_1", z_position_multiplier=0.4,
                       gradient_layer_1="gone"), (0, 4)),
    "FX-CD-046": ("The back layer with its in point at frame 4: clear backs before it, the "
                  "back at it.", grid(y_rotation_offset=180, back_layer="late"), (0, 4)),
    "FX-CD-047": ("A back layer not in the composition, `gone`: clear backs, and the warning.",
                  grid(y_rotation_offset=180, back_layer="gone"), (0,)),
    "FX-CD-048": ("Sixteen columns of one row, each turned by red 1 of `colours` about y with "
                  "the back layer: blinds opening across.",
                  case(rows=1, columns=16, z_position_source="none", y_rotation_source="red_1",
                       gradient_layer_1="colours", y_rotation_multiplier=70,
                       y_rotation_offset=70, **BACK), (0,)),
})

INVALID = {
    "FX-CD-049": ("Rotation order \"xxy\".", grid(rotation_order="xxy")),
    "FX-CD-050": ("Transformation order \"scale\".", grid(transformation_order="scale")),
    "FX-CD-051": ("X position source \"purple_1\".", grid(x_position_source="purple_1")),
    "FX-CD-052": ("Camera system \"corner_pins\" with the pins as added (D-634): the camera "
                  "as added, so the picture unchanged.", grid(camera_system="corner_pins")),
    "FX-CD-053": ("Rows 0, below 1.", grid(rows=0)),
    "FX-CD-054": ("Columns 1001, above 1000.", grid(columns=1001)),
    "FX-CD-055": ("Z position multiplier 101, above 100.", grid(z_position_multiplier=101)),
    "FX-CD-056": ("X scale offset -11, below -10.", grid(x_scale_offset=-11)),
    "FX-CD-057": ("Y rotation offset 3601, above 3600.", grid(y_rotation_offset=3601)),
    "FX-CD-058": ("Camera Z position 0, below 0.01.", grid(camera_z_position=0)),
    "FX-CD-059": ("Focal length 0, below 1.", grid(focal_length=0)),
    "FX-CD-060": ("Light type \"spot\".", grid(light_type="spot")),
    "FX-CD-061": ("Light colour \"white\": not a #rrggbb colour.", grid(light_color="white")),
    "FX-CD-062": ("Gradient layer 2 written as the number 3, not a word.",
                  grid(gradient_layer_2=3)),
    "FX-CD-063": ("Rows & Columns \"both\".", grid(rows_columns="both")),
    "FX-CD-064": ("X position offset keyed to 120 at frame 4.",
                  grid(x_position_offset=keyed((0, 0), (4, 120)))),
}

# D-566 (1): FX-CD-052 pinned Corner Pins as refused while it was not built; D-634 builds it.
BUILT = {"FX-CD-052"}

# D-634: the Corner Pins camera.
PIN = dict(camera_system="corner_pins")
KEYSTONE = dict(PIN, upper_left_corner=[20, 10], upper_right_corner=[85, 0],
                lower_right_corner=[100, 95])
SLANT = dict(PIN, upper_left_corner=[12.5, -12.5], upper_right_corner=[100, 0],
             lower_right_corner=[87.5, 112.5], lower_left_corner=[0, 100])
QUAD = dict(PIN, upper_left_corner=[5, 10], upper_right_corner=[90, -5],
            lower_right_corner=[100, 95], lower_left_corner=[-5, 105])
PIN_CASES = {
    "FX-CD-065": ("Corner Pins with the pins as added, three turns with the back layer: "
                  "FX-CD-014 exactly.", grid(**PIN, **SPIN, **BACK), (0,)),
    "FX-CD-066": ("Corner Pins, the upper corners pulled in to [20, 10] and [85, 0] and the "
                  "lower right up to [100, 95], Auto Focal Length off: the layer seen leaning "
                  "back and turned, the nearer cards larger.",
                  grid(**KEYSTONE, **DEPTH), (0,)),
    "FX-CD-067": ("FX-CD-066 with Auto Focal Length on: the focal length worked out from the "
                  "pins.", grid(auto_focal_length="on", **KEYSTONE, **DEPTH), (0,)),
    "FX-CD-068": ("Corner Pins turned and slanted with no perspective (a parallelogram), Auto "
                  "Focal Length on: no focal length can be worked out, so Focal Length's.",
                  grid(auto_focal_length="on", **SLANT, **DEPTH), (0,)),
    "FX-CD-069": ("FX-CD-068 with Auto Focal Length off: the same.",
                  grid(**SLANT, **DEPTH), (0,)),
    "FX-CD-070": ("Corner Pins on a general four-sided shape, Auto Focal Length on, three turns "
                  "with the back layer.", grid(auto_focal_length="on", **QUAD, **SPIN, **BACK),
                  (0,)),
    "FX-CD-071": ("FX-CD-067 with a point light at the top left, depth 0.3, specular 1, "
                  "sharpness 20: the light stays on the layer.",
                  grid(auto_focal_length="on", light_type="point", light_position=[20, 30],
                       light_depth=0.3, specular_reflection=1, highlight_sharpness=20,
                       **KEYSTONE, **SPIN, **BACK), (0,)),
    "FX-CD-072": ("FX-CD-067 with the camera turned 30 about y, at x 40 per cent and Z "
                  "position 3: Corner Pins does not use them, so FX-CD-067.",
                  grid(auto_focal_length="on", camera_y_rotation=30, camera_xy_position=[40, 50],
                       camera_z_position=3, **KEYSTONE, **DEPTH), (0,)),
    "FX-CD-073": ("The upper right pin keyed from [100, 0] at frame 0 to [80, 20] at frame 4: "
                  "frame 0 FX-CD-003.",
                  grid(upper_right_corner=keyed((0, [100, 0]), (4, [80, 20])), **PIN, **DEPTH),
                  (0, 4)),
}
PIN_INVALID = {
    "FX-CD-074": ("Upper left corner [-1001, 0], below -1000.",
                  grid(upper_left_corner=[-1001, 0], **PIN)),
    "FX-CD-075": ("Auto focal length \"maybe\".", grid(auto_focal_length="maybe", **PIN)),
    "FX-CD-076": ("Lower right corner keyed to [100, 1200] at frame 4.",
                  grid(lower_right_corner=keyed((0, [100, 100]), (4, [100, 1200])), **PIN)),
}


def effect(fid, c):
    params = {w: c[w] for w in WORDS}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    params.update({n: setting_json(c[n]) for n in PINS if n in c["given"]})
    if "auto_focal_length" in c["given"]:
        params["auto_focal_length"] = c["auto_focal_length"]
    return {"instance_id": fid, "type_id": "core.card_dance", "enabled": True,
            "parameters": params}


def map_layers():
    return [L.raster(n, "asset-" + n, enabled=False, out_frame=FRAMES)
            for n in ("back", "ramp", "colours")] + [
        L.raster("late", "asset-back", enabled=False, in_frame=4, out_frame=FRAMES)]


def project_json(fx, c):
    holder = L.raster("holder", "asset-shot", position=c["shift"], out_frame=FRAMES)
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
        if "gone" in (c["back_layer"], c["gradient_layer_1"], c["gradient_layer_2"]):
            entry["warning"] = "EFFECT_LAYER_MISSING"
        # Document 28: the compositor leaves out a layer level with the composition camera or
        # behind it, with a warning on each frame; the layers here lie at depth 0.
        if c["camera"] is not None and c["camera"][2] >= 0:
            entry["frame_warning"] = "CAMERA_PLANE_BEHIND"
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

    (OUT / "expected_card_dance.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def step_clear(fx, c, frames):
    """No pixel's ray lies near a card's edge, an edge-on card, two cards tied at differing
    colours, or the highlight's cut."""
    for f in frames:
        k, drawing, back, grads, cam = inputs(c, f)
        seen = view(k, cam)
        if seen is None:
            continue
        m, o, ray = seen
        deck = cards(k, drawing, grads)
        for y in range(H):
            for x in range(W):
                d = ray(x, y)
                for i, (cx, cy, cw, ch, A, B, s) in enumerate(deck):
                    a0, a1 = [A[r][0] for r in range(3)], [A[r][1] for r in range(3)]
                    nn = cross(a0, a1)
                    assert s == 0 or abs(dot(unit_vec(d), unit_vec(nn))) > 1e-6, (fx, f, x, y, i)
                # A card's edge near the pixel must not matter: the colour is the same with
                # every card a little larger or a little smaller.
                pick = lambda g: [  # noqa: E731
                    shade(k, m, o, d, min(hs, key=lambda h: (h[0], h[1])), drawing, back)
                    if hs else list(CLEAR) for hs in [hits(deck, o, d, g)]][0]
                mid = pick(0.0)
                for g in (-1e-6, 1e-6):
                    assert max(abs(p - q) for p, q in zip(pick(g), mid)) < 1e-9, (fx, f, x, y, g)
                found = sorted(hits(deck, o, d), key=lambda h: (h[0], h[1]))
                if len(found) > 1 and found[1][0] - found[0][0] <= 1e-6:
                    c0 = shade(k, m, o, d, found[0], drawing, back)
                    c1 = shade(k, m, o, d, found[1], drawing, back)
                    # Exact ties (cards in one plane) keep the first card; a near tie must not
                    # change the colour.
                    assert found[0][0] == found[1][0] or \
                        max(abs(p - q) for p, q in zip(c0, c1)) < 1e-12, (fx, f, x, y)
                if found and k["specular_reflection"] > 0:
                    _, n, lv = normal_light(k, m, o, d, found[0])
                    assert abs(dot(n, lv)) > 1e-6, (fx, f, x, y)


def near(a, b, eps=1e-12):
    return all(abs(x - y) <= eps for p, q in zip(a, b) for x, y in zip(p, q))


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(grid())
    back = L.fit(decoded("back"), "stretch", W, H)["px"]
    # Nothing moving: the picture, to rounding; as added something moves.
    assert near(c["FX-CD-002"]["0"], drawn, 1e-9)
    assert not near(c["FX-CD-001"]["0"], drawn, 1e-6)
    # Turned about y: the back whole; without one, clear.
    assert near(c["FX-CD-007"]["0"], back, 1e-9)
    assert all(p == CLEAR for p in c["FX-CD-008"]["0"])
    # A quarter card right: the picture moved one pixel, the first column clear.
    q = c["FX-CD-004"]["0"]
    assert all(q[y * W] == CLEAR for y in range(H))
    assert near([q[y * W + x] for y in range(H) for x in range(1, W)],
                [drawn[y * W + x - 1] for y in range(H) for x in range(1, W)], 1e-9)
    # Half-width cards: the gaps clear.
    assert all(c["FX-CD-010"]["0"][y * W + x] == CLEAR for y in range(H) for x in (0, 3, 4, 7))
    # The six rotation orders and the six transformation orders all differ.
    rot_cases = [c[f"FX-CD-{n:03d}"]["0"] for n in range(14, 20)]
    tr_cases = [c[f"FX-CD-{n:03d}"]["0"] for n in range(20, 26)]
    for group in (rot_cases, tr_cases):
        for i in range(len(group)):
            for j in range(i):
                assert not near(group[i], group[j], 1e-6), (i, j)
    # Behind the camera, or a camera on the plane: nothing.
    assert all(p == CLEAR for p in c["FX-CD-028"]["0"])
    assert all(p == CLEAR for p in c["FX-CD-038"]["0"])
    # No camera in the file: the effect's own camera as added.
    assert near(c["FX-CD-034"]["0"], c["FX-CD-003"]["0"], 1e-9)
    # Keyed: frame 0 the picture, frame 4 the back.
    k = c["FX-CD-043"]
    assert near(k["0"], drawn, 1e-9) and near(k["4"], back, 1e-9)
    # Late back: clear, then the back; gone back: clear.
    assert all(p == CLEAR for p in c["FX-CD-046"]["0"]) and near(c["FX-CD-046"]["4"], back, 1e-9)
    assert all(p == CLEAR for p in c["FX-CD-047"]["0"])
    # Moved: the same picture, three right and one down.
    moved, still = c["FX-CD-044"]["0"], c["FX-CD-003"]["0"]
    assert all(moved[(y + 1) * W + x + 3] == still[y * W + x] for y in range(H - 1)
               for x in range(W - 3))
    # Lights change the picture.
    for fx in ("FX-CD-039", "FX-CD-040", "FX-CD-041", "FX-CD-042"):
        assert c[fx]["0"] != c["FX-CD-014"]["0"], fx
    # Mirrored across: still the front, so not clear where the picture is not.
    assert any(p[3] > 0 for p in c["FX-CD-026"]["0"])
    # D-634: the pins as added are the camera as added.
    assert near(c["FX-CD-052"]["0"], drawn, 1e-9) and near(c["FX-CD-052"]["4"], drawn, 1e-9)
    assert near(c["FX-CD-065"]["0"], c["FX-CD-014"]["0"], 1e-9)
    assert near(c["FX-CD-073"]["0"], c["FX-CD-003"]["0"], 1e-9)
    assert not near(c["FX-CD-073"]["4"], c["FX-CD-003"]["0"], 1e-6)
    # Auto changes a keystone, not a parallelogram; the camera's own settings are not used.
    assert not near(c["FX-CD-066"]["0"], c["FX-CD-067"]["0"], 1e-6)
    assert near(c["FX-CD-068"]["0"], c["FX-CD-069"]["0"], 1e-12)
    assert near(c["FX-CD-072"]["0"], c["FX-CD-067"]["0"], 1e-12)
    assert not near(c["FX-CD-066"]["0"], c["FX-CD-003"]["0"], 1e-6)
    # The flat layer lands on its pins: with nothing moving, the keystone's top row of pixel
    # centres inside the pins is drawn and the corners outside them are clear.
    flat = render(grid(**KEYSTONE), 0)
    assert flat[0] == CLEAR and flat[W - 1] == CLEAR and flat[W // 2][3] > 0
    assert flat[(H - 1) * W][3] > 0
    print("checks passed")


if __name__ == "__main__":
    main()
