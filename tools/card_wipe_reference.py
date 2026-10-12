"""Card Wipe, worked a second way.

`core.card_wipe`, modelled on After Effects' Card Wipe (Transition), which "simulates a group of
cards displaying a picture and then flipping to display another picture". Adobe's Transition
effects page names the controls and says what each does in a sentence: Transition Completion,
Transition Width ("the width of the area that actively changes"), Back Layer ("the layer that
appears in segments on the backs of the cards"), Rows & Columns (Independent, or Columns Follows
Rows, "the number of columns is always the same as the number of rows"), Rows and Columns ("up
to 1000", "evenly distributed across a layer"), Card Scale ("smaller than 1 ... revealing the
underlying layer in the gaps", "greater than 1 ... a blocky mosaic as they overlap"), Flip Axis,
Flip Direction, Flip Order (with a gradient: "cards flip first where the gradient is black and
last where the gradient is white"), Gradient Layer, Timing Randomness ("at 0, the cards flip in
order"), Random Seed, the Camera Position group (X, Y and Z Rotation, X, Y Position, Z Position,
"smaller numbers move the camera closer", Focal Length, "smaller numbers zoom in"), and the
lighting and material groups it shares with Card Dance. It publishes no formula and no settings
when added; the page on the shared lighting and material controls could not be read (it answered
403). So the rule and every setting when added are ours (document 21, D-580), and nothing is
ported.

It draws its own fake 3D, as CC Cylinder (D-530) and CC Sphere (D-532) do: a camera and a light
inside the effect. The composition's camera and lights are not used (P0-12, a gap), so Camera
System's Comp Camera and the First Comp Light are not built (gaps); its Corner Pins is (D-634). Transform
Order is fixed (gap), and Position Jitter and Rotation Jitter are not built (gaps). The cards are
drawn with one ray a pixel, so their edges are not smoothed, and the nearest card a ray meets
wins even where that card's picture is clear (gaps).

The settings. `transition_completion` 0 to 100, 0 when added; `transition_width` 0 to 100, 50;
`back_layer` D-189's layer word, "" when added, fitted to the drawing as `stretch` fits it;
`rows_columns` "independent" (when added) or "columns_follow_rows"; `rows` and `columns` 1 to
1000, 8, each rounded to the nearest whole number (halves away from zero); `card_scale` 0 to 10,
1; `flip_axis` "x" (when added), "y" or "random"; `flip_direction` "positive" (when added),
"negative" or "random"; `flip_order` "left_to_right" (when added), "right_to_left",
"top_to_bottom", "bottom_to_top", "top_left_to_bottom_right", "bottom_left_to_top_right",
"top_right_to_bottom_left", "bottom_right_to_top_left" or "gradient"; `gradient_layer` D-189's
word, "", fitted as `stretch`; `timing_randomness` 0 to 1, 0; `random_seed` 0 to 100000, 0,
taken down to a whole number. The camera: `x_rotation`, `y_rotation`, `z_rotation` -3600 to 3600
degrees, 0; `xy_position` x then y, -1000 to 1000 per cent of the drawing's width and height,
[50, 50]; `z_position` 0.01 to 10, 1, the eye's distance in focal lengths; `focal_length` 1 to
1000 mm on a 36 mm frame, 50. `camera_system` "camera_position" (when added) or "corner_pins"
(D-634); `upper_left_corner`, `upper_right_corner`, `lower_left_corner`, `lower_right_corner`
x then y, -1000 to 1000 per cent, the drawing's corners when added; `auto_focal_length` "off"
(when added) or "on". These six are read as when added where a file leaves them out (a file
from before D-634), and written only when given. The light and material: `light_type` "distant" (when added) or
"point"; `light_intensity` 0 to 10, 1; `light_color` #rrggbb, "#ffffff"; `light_position` x then
y, -1000 to 1000 per cent, [50, 50]; `light_depth` 0 to 10 drawing widths toward the viewer, 1;
`ambient_light` 0 to 2, 0.25; `diffuse_reflection` 0 to 2, 0.75; `specular_reflection` 0 to 2, 0;
`highlight_sharpness` 1 to 100, 5. Every number is keyable; the words are exact.

The rule, in the drawing's own W by H space, y down, z away from the viewer.

1. The cards. R = rows and C = columns, or C = R with "columns_follow_rows", each rounded. A
   card is cw = W/C by ch = H/R; card (a, b), column a and row b, has its centre at
   (cx, cy) = ((a + 1/2) cw, (b + 1/2) ch), and is drawn s = card_scale times its size: half
   sizes hx = s cw / 2, hy = s ch / 2.
2. When each card flips. With u = (a + 1/2)/C and v = (b + 1/2)/R the card's order o is u, 1 - u,
   v, 1 - v, (u + v)/2, (u + 1 - v)/2, (1 - u + v)/2 or (2 - u - v)/2 for the eight directions in
   the order listed, or, for "gradient", the brightness (document 21's luma, clamped to 0..1) of
   the gradient picture's pixel (floor cx, floor cy); with no gradient picture (none named, or a
   layer not in the composition, with D-189's warning) every o is 0. With
   g = unit(mix(floor seed), a, b, 0, ch) (`tools/noise_reference.py`), T the randomness,
   o' = (1 - T) o + T (g0 + 1)/2. With c = completion/100 and w = width/100 the card's progress is
   p = clamp((c (1 + w) - o') / w, 0, 1), or for w = 0, 1 where c > 0 and c >= o' and 0
   otherwise. Its axis is x, y, or for "random" x where g1 < 0 and y otherwise; its sign is +1,
   -1, or for "random" -1 where g2 < 0 and +1 otherwise. It is turned by theta = sign p pi about
   its own axis through its centre: about x, (x, y cos - z sin, y sin + z cos); about y,
   (x cos + z sin, y, -x sin + z cos).
3. The camera. F = W focal/36. The eye sits at E = (xy_x W/100, xy_y H/100, -z F) and looks
   through the pixel's centre along d = (x + 1/2 - W/2, y + 1/2 - H/2, F), turned with the whole
   scene about the drawing's middle C0 = (W/2, H/2, 0) by M = Rz Ry Rx (x first, each as in step
   2's turns, Rz (x cos - y sin, x sin + y cos)): in the cards' space the eye is o = M^T (E - C0)
   and the ray's direction v = M^T d. As added this is the plain picture, each pixel on itself.
   With "corner_pins" the eye o and the ray's direction v come instead from the four pins by
   Card Dance's Corner Pins rule (`tools/card_dance_reference.py`, step 4), and M is the identity,
   so the light stays on the layer; the rotations, position and Z position are not used, and
   where that rule draws nothing every pixel is clear. The pins as added give the camera as
   added.
4. The hits. For card (a, b), with q = (cx - W/2, cy - H/2, 0), the ray in the card's own frame
   is o'' = R^T (o - q), v'' = R^T v, R the card's turn. Where v''z is not 0, t = -o''z / v''z;
   a hit needs t > 0 and |lx| <= hx, |ly| <= hy for (lx, ly) = o''xy + t v''xy. Card Scale 0
   draws nothing. The nearest hit (least t) wins; where two tie exactly, the first card in rows
   then columns.
5. The colour. Where v''z > 0 the ray meets the front: the drawing at (cx + lx/s, cy + ly/s) by
   document 21's bilinear, transparent outside. Otherwise the back: the back picture at
   (cx - lx/s, cy + ly/s) for a card turned about y, (cx + lx/s, cy - ly/s) about x, so a card
   turned over shows its share of the back picture upright; with no back picture (none named, or
   a layer not in the composition, with D-189's warning) the back is clear. A layer not yet begun
   is a clear picture.
6. The light, in the cards' space: P = M^T ((lp_x W/100, lp_y H/100, -depth W) - C0); the hit
   h = o + t v; L = unit(P) for "distant", unit(P - h) for "point" (a zero step stays zero). The
   front's normal is (0, sin theta, -cos theta) about x, (-sin theta, 0, -cos theta) about y, the
   back's its opposite. V = -v/|v|, Hv = L + V; the highlight sp = max(N.Hv/|Hv|, 0)^sharpness
   where N.L > 0 and |Hv| > 0, else 0. With i the intensity, Lc the light's linear colour, A, D,
   S the ambient, diffuse and specular numbers, on the sampled premultiplied (c, alpha):

       rgb = c (A + D i Lc max(N.L, 0)) + S i sp Lc alpha,  alpha unchanged.

7. The output at buffer pixel (X, Y), x = X - ox, y = Y - oy: the nearest hit's colour, or clear.

The layer never grows. A draft changes nothing, every distance being a share of the drawing.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawings' 8-bit values, and tries every card for every pixel. The rule steps at a card's
edges, where two cards tie at differing colours, and where the highlight is cut at N.L = 0; every
case is checked that no pixel lies near one.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/card_wipe/`: the
map layers, every one switched off, and on top of them the layer `holder`, showing the drawing
`shot` (red, blue and cream squares, the bottom right 4 by 3 clear), with the effect. The
expected pixels are in `Fixtures/card_wipe/expected_card_wipe.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/card_wipe_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from noise_reference import u as unit  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import compound_blur_reference as K  # noqa: E402
import card_dance_reference as CD  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "card_wipe"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
CLEAR = [0.0, 0.0, 0.0, 0.0]


def back_drawing():
    """8 by 5: red rising across, blue rising down, green steady; a white marker at (0, 0)."""
    return [[(255, 255, 255, 255) if (x, y) == (0, 0) else
             (round(40 + 200 * x / 7), 150, round(40 + 200 * y / 4), 255)
             for x in range(8)] for y in range(5)]


def ramp_drawing():
    """16 by 10 greys: lighter down the picture, a little darker to the right."""
    grey = lambda x, y: round(255 * (0.7 * y / 9 + 0.3 * (15 - x) / 15))  # noqa: E731
    return [[(grey(x, y),) * 3 + (255,) for x in range(16)] for y in range(10)]


DRAWINGS = {"shot": K.DRAWINGS["holder"], "back": back_drawing(), "ramp": ramp_drawing()}
MAPS = {"back": "back", "ramp": "ramp", "late": "back"}


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


def map_picture(name, frame):
    """A map layer's picture fitted to the drawing, or None for none named or not there."""
    if name == "" or name not in MAPS:
        return None
    p = L.EMPTY if name == "late" and frame < 4 else decoded(MAPS[name])
    return L.fit(p, "stretch", W, H)


# --- the rule -------------------------------------------------------------------------------

def rnd(v):
    return math.floor(v + 0.5)


def turn(m, v):
    return [sum(m[r][i] * v[i] for i in range(3)) for r in range(3)]


def turn_t(m, v):
    return [sum(m[i][r] * v[i] for i in range(3)) for r in range(3)]


def camera_matrix(k):
    x, y, z = (math.radians(k[n]) for n in ("x_rotation", "y_rotation", "z_rotation"))
    rx = [[1, 0, 0], [0, math.cos(x), -math.sin(x)], [0, math.sin(x), math.cos(x)]]
    ry = [[math.cos(y), 0, math.sin(y)], [0, 1, 0], [-math.sin(y), 0, math.cos(y)]]
    rz = [[math.cos(z), -math.sin(z), 0], [math.sin(z), math.cos(z), 0], [0, 0, 1]]
    mul = lambda a, b: [[sum(a[r][i] * b[i][c] for i in range(3)) for c in range(3)]  # noqa: E731
                        for r in range(3)]
    return mul(rz, mul(ry, rx))


ORDERS = {
    "left_to_right": lambda u, v: u, "right_to_left": lambda u, v: 1 - u,
    "top_to_bottom": lambda u, v: v, "bottom_to_top": lambda u, v: 1 - v,
    "top_left_to_bottom_right": lambda u, v: (u + v) / 2,
    "bottom_left_to_top_right": lambda u, v: (u + 1 - v) / 2,
    "top_right_to_bottom_left": lambda u, v: (1 - u + v) / 2,
    "bottom_right_to_top_left": lambda u, v: (2 - u - v) / 2,
}


def cards(k, grad):
    """Steps 1 and 2: per card (a, b, cx, cy, axis, sin, cos), rows then columns."""
    rows = rnd(k["rows"])
    cols = rows if k["rows_columns"] == "columns_follow_rows" else rnd(k["columns"])
    cw, ch = W / cols, H / rows
    seed = math.floor(k["random_seed"])
    t_r, c, w = k["timing_randomness"], k["transition_completion"] / 100, k["transition_width"] / 100
    out = []
    for b in range(rows):
        for a in range(cols):
            cx, cy = (a + 0.5) * cw, (b + 0.5) * ch
            u, v = (a + 0.5) / cols, (b + 0.5) / rows
            if k["flip_order"] == "gradient":
                o = 0.0 if grad is None else K.luma(L.at(grad, min(math.floor(cx), W - 1),
                                                         min(math.floor(cy), H - 1)))
            else:
                o = ORDERS[k["flip_order"]](u, v)
            o2 = (1 - t_r) * o + t_r * (unit(seed, a, b, 0, 0) + 1) / 2
            if w > 0:
                p = min(max((c * (1 + w) - o2) / w, 0.0), 1.0)
            else:
                p = 1.0 if c > 0 and c >= o2 else 0.0
            axis = {"x": 0, "y": 1}.get(k["flip_axis"])
            if axis is None:
                axis = 0 if unit(seed, a, b, 0, 1) < 0 else 1
            sign = {"positive": 1, "negative": -1}.get(k["flip_direction"])
            if sign is None:
                sign = -1 if unit(seed, a, b, 0, 2) < 0 else 1
            th = sign * p * math.pi
            out.append((a, b, cx, cy, cw, ch, axis, math.sin(th), math.cos(th)))
    return out


def inv(axis, s, c, v):
    if axis == 0:
        return [v[0], c * v[1] + s * v[2], -s * v[1] + c * v[2]]
    return [c * v[0] - s * v[2], v[1], s * v[0] + c * v[2]]


def hits(k, deck, o, d, grow=0.0):
    """Step 4: every hit, as (t, index, card, lx, ly, front); `grow` widens every card a
    little, for the check that no pixel lies near an edge."""
    sc = k["card_scale"]
    found = []
    if sc == 0:
        return found
    for i, (a, b, cx, cy, cw, ch, axis, s, c) in enumerate(deck):
        o1 = [o[0] - (cx - W / 2), o[1] - (cy - H / 2), o[2]]
        o2, d2 = inv(axis, s, c, o1), inv(axis, s, c, d)
        if d2[2] == 0:
            continue
        t = -o2[2] / d2[2]
        if t <= 0:
            continue
        lx, ly = o2[0] + t * d2[0], o2[1] + t * d2[1]
        if abs(lx) <= sc * cw / 2 + grow and abs(ly) <= sc * ch / 2 + grow:
            found.append((t, i, deck[i], lx, ly, d2[2] > 0, o2, d2))
    return found


def colour(hexa):
    return [srgb_to_linear(int(hexa[i:i + 2], 16) / 255) for i in (1, 3, 5)]


def unit_vec(v):
    n = math.sqrt(sum(q * q for q in v))
    return [0.0, 0.0, 0.0] if n == 0 else [q / n for q in v]


def shade(k, m, o, d, hit, drawing, back):
    """Steps 5 and 6 for the winning hit."""
    t, _, (a, b, cx, cy, cw, ch, axis, s, c), lx, ly, front, _, _ = hit
    sc = k["card_scale"]
    if front:
        px = L.bilinear(drawing, cx + lx / sc, cy + ly / sc)
    elif back is None:
        px = list(CLEAR)
    elif axis == 1:
        px = L.bilinear(back, cx - lx / sc, cy + ly / sc)
    else:
        px = L.bilinear(back, cx + lx / sc, cy - ly / sc)
    lp = k["light_position"]
    pl = turn_t(m, [lp[0] * W / 100 - W / 2, lp[1] * H / 100 - H / 2, -k["light_depth"] * W])
    h = [o[i] + t * d[i] for i in range(3)]
    lv = unit_vec(pl if k["light_type"] == "distant" else [pl[i] - h[i] for i in range(3)])
    n = [0.0, s, -c] if axis == 0 else [-s, 0.0, -c]
    if not front:
        n = [-q for q in n]
    vv = unit_vec([-q for q in d])
    hv = [lv[i] + vv[i] for i in range(3)]
    nl = sum(n[i] * lv[i] for i in range(3))
    hn = math.sqrt(sum(q * q for q in hv))
    sp = 0.0
    if nl > 0 and hn > 0:
        sp = max(sum(n[i] * hv[i] for i in range(3)) / hn, 0.0) ** k["highlight_sharpness"]
    lc, i_ = colour(k["light_color"]), k["light_intensity"]
    A, D, S = k["ambient_light"], k["diffuse_reflection"], k["specular_reflection"]
    return [px[j] * (A + D * i_ * lc[j] * max(nl, 0.0)) + S * i_ * sp * lc[j] * px[3]
            for j in range(3)] + [px[3]]


def view(k):
    """Step 3: the eye in the cards' space, the turn, and the ray for a pixel; None where
    nothing is drawn."""
    if k["camera_system"] == "corner_pins":
        cam = CD.corner_camera(k, W, H)
        if cam is None:
            return None
        return [[1, 0, 0], [0, 1, 0], [0, 0, 1]], cam[0], cam[1]
    m = camera_matrix(k)
    f = W * k["focal_length"] / 36
    xy = k["xy_position"]
    o = turn_t(m, [xy[0] * W / 100 - W / 2, xy[1] * H / 100 - H / 2, -k["z_position"] * f])
    return m, o, lambda x, y: turn_t(m, [x + 0.5 - W / 2, y + 0.5 - H / 2, f])


def card_wipe(k, drawing, back, grad):
    deck = cards(k, grad)
    if view(k) is None:
        return [list(CLEAR) for _ in range(W * H)]
    m, o, ray = view(k)
    out = []
    for y in range(H):
        for x in range(W):
            d = ray(x, y)
            found = hits(k, deck, o, d)
            if not found:
                out.append(list(CLEAR))
                continue
            best = min(found, key=lambda h: (h[0], h[1]))
            out.append(shade(k, m, o, d, best, drawing, back))
    return out


# --- the cases ------------------------------------------------------------------------------

NUMBERS = ("transition_completion", "transition_width", "rows", "columns", "card_scale",
           "timing_randomness", "random_seed", "x_rotation", "y_rotation", "z_rotation",
           "xy_position", "z_position", "focal_length", "light_intensity", "light_position",
           "light_depth", "ambient_light", "diffuse_reflection", "specular_reflection",
           "highlight_sharpness")
WORDS = ("back_layer", "rows_columns", "flip_axis", "flip_direction", "flip_order",
         "gradient_layer", "light_type", "light_color")
DEFAULTS = {
    "transition_completion": 0, "transition_width": 50, "back_layer": "",
    "rows_columns": "independent", "rows": 8, "columns": 8, "card_scale": 1, "flip_axis": "x",
    "flip_direction": "positive", "flip_order": "left_to_right", "gradient_layer": "",
    "timing_randomness": 0, "random_seed": 0, "x_rotation": 0, "y_rotation": 0, "z_rotation": 0,
    "xy_position": [50, 50], "z_position": 1, "focal_length": 50, "light_type": "distant",
    "light_intensity": 1, "light_color": "#ffffff", "light_position": [50, 50], "light_depth": 1,
    "ambient_light": 0.25, "diffuse_reflection": 0.75, "specular_reflection": 0,
    "highlight_sharpness": 5,
}
# D-634: read as when added where a file leaves them out, written only when given.
PIN_DEFAULTS = dict(CD.PIN_DEFAULTS, camera_system="camera_position")


def case(shift=(0, 0), **kw):
    c = dict(DEFAULTS, **PIN_DEFAULTS, shift=shift, given=set(kw))
    for n, v in kw.items():
        assert n in DEFAULTS or n in PIN_DEFAULTS, n
        c[n] = v
    return c


def grid(**kw):
    """Four columns and two rows, so no card's edge falls on a pixel's centre."""
    return case(**dict(dict(rows=2, columns=4), **kw))


HALF = dict(transition_completion=50)


def settings(c, frame):
    k = {n: value_at(c[n], frame) for n in NUMBERS + CD.PINS}
    k.update({w: c[w] for w in WORDS + ("camera_system", "auto_focal_length")})
    return k


def worked(c, frame):
    o = decoded("shot")
    k = settings(c, frame)
    return card_wipe(k, o, map_picture(c["back_layer"], frame),
                     map_picture(c["gradient_layer"], frame))


def render(c, frame):
    out = L.pic(W, H, worked(c, frame))
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded("shot")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


CASES = {
    "FX-CW-001": ("As added: nothing has flipped, so the picture is unchanged.", case(), (0,)),
    "FX-CW-002": ("Completion 100 with no back layer: every card turned over, its back clear.",
                  grid(transition_completion=100), (0,)),
    "FX-CW-003": ("Completion 50, width 50, left to right, four columns: the first column "
                  "turned over (clear), the second three quarters, the third a quarter, the "
                  "fourth not begun.", grid(**HALF), (0,)),
    "FX-CW-004": ("FX-CW-003 with the back layer: the turned cards show its share of the back.",
                  grid(back_layer="back", **HALF), (0,)),
    "FX-CW-005": ("Completion 100 with the back layer: the back picture whole, upright, its "
                  "white marker at the top left.", grid(back_layer="back",
                                                        transition_completion=100), (0,)),
    "FX-CW-006": ("FX-CW-004 turning about the y axis.",
                  grid(back_layer="back", flip_axis="y", **HALF), (0,)),
    "FX-CW-007": ("FX-CW-004 turning the negative way.",
                  grid(back_layer="back", flip_direction="negative", **HALF), (0,)),
    "FX-CW-008": ("Right to left.", grid(back_layer="back", flip_order="right_to_left", **HALF),
                  (0,)),
    "FX-CW-009": ("Top to bottom.", grid(back_layer="back", flip_order="top_to_bottom", **HALF),
                  (0,)),
    "FX-CW-010": ("Bottom to top.", grid(back_layer="back", flip_order="bottom_to_top", **HALF),
                  (0,)),
    "FX-CW-011": ("Top left to bottom right.", grid(back_layer="back",
                                                    flip_order="top_left_to_bottom_right",
                                                    **HALF), (0,)),
    "FX-CW-012": ("Bottom left to top right.", grid(back_layer="back",
                                                    flip_order="bottom_left_to_top_right",
                                                    **HALF), (0,)),
    "FX-CW-013": ("Top right to bottom left.", grid(back_layer="back",
                                                    flip_order="top_right_to_bottom_left",
                                                    **HALF), (0,)),
    "FX-CW-014": ("Bottom right to top left.", grid(back_layer="back",
                                                    flip_order="bottom_right_to_top_left",
                                                    **HALF), (0,)),
    "FX-CW-015": ("Flip order by the gradient layer `ramp`: dark cards (top right) first.",
                  grid(back_layer="back", flip_order="gradient", gradient_layer="ramp",
                       transition_completion=40), (0,)),
    "FX-CW-016": ("Flip order gradient with no gradient layer: every card at the same point, "
                  "60 per cent of the way over.", grid(flip_order="gradient",
                                                       transition_completion=20), (0,)),
    "FX-CW-017": ("Timing randomness 1, seed 7: the order wholly random.",
                  grid(back_layer="back", timing_randomness=1, random_seed=7, **HALF), (0,)),
    "FX-CW-018": ("FX-CW-017 with seed 8: another order.",
                  grid(back_layer="back", timing_randomness=1, random_seed=8, **HALF), (0,)),
    "FX-CW-019": ("Random axis and direction, seed 3, timing randomness 0.5.",
                  grid(back_layer="back", flip_axis="random", flip_direction="random",
                       timing_randomness=0.5, random_seed=3, **HALF), (0,)),
    "FX-CW-020": ("Width 0: the cards in the first half turned over, the rest not.",
                  grid(back_layer="back", transition_width=0, **HALF), (0,)),
    "FX-CW-021": ("Card scale 0.5, not begun: smaller cards, gaps between them.",
                  grid(card_scale=0.5), (0,)),
    "FX-CW-022": ("Card scale 0.5 at completion 40.", grid(card_scale=0.5,
                                                          transition_completion=40), (0,)),
    "FX-CW-023": ("Card scale 1.5, four rows, not begun: larger cards overlapping, the first "
                  "in rows then columns on top.", grid(rows=4, card_scale=1.5), (0,)),
    "FX-CW-024": ("Card scale 1.5, four rows, at completion 40.",
                  grid(rows=4, card_scale=1.5, transition_completion=40), (0,)),
    "FX-CW-025": ("Columns follow rows, rows 3 (columns 4 ignored): three by three.",
                  case(rows=3, columns=4, rows_columns="columns_follow_rows",
                       back_layer="back", **HALF), (0,)),
    "FX-CW-026": ("Rows 5 by columns 8.", case(rows=5, columns=8, back_layer="back", **HALF),
                  (0,)),
    "FX-CW-027": ("The camera turned 30 degrees about y.",
                  grid(back_layer="back", y_rotation=30, **HALF), (0,)),
    "FX-CW-028": ("The camera turned -20 about x and 15 about z.",
                  grid(back_layer="back", x_rotation=-20, z_rotation=15, **HALF), (0,)),
    "FX-CW-029": ("The camera at x 40, y 60 per cent: the cards moved right and up.",
                  grid(back_layer="back", xy_position=[40, 60], **HALF), (0,)),
    "FX-CW-030": ("Z position 1.7: the camera farther away, the cards smaller.",
                  grid(back_layer="back", z_position=1.7, **HALF), (0,)),
    "FX-CW-031": ("Focal length 20 with the camera turned 40 about y: a wide lens.",
                  grid(back_layer="back", focal_length=20, y_rotation=40, **HALF), (0,)),
    "FX-CW-032": ("A distant light from the top left, depth 0.5.",
                  grid(back_layer="back", light_position=[0, 0], light_depth=0.5, **HALF),
                  (0,)),
    "FX-CW-033": ("A point light in the middle, depth 0.3.",
                  grid(back_layer="back", light_type="point", light_depth=0.3, **HALF), (0,)),
    "FX-CW-034": ("Specular 1, sharpness 20: a highlight on the turning cards.",
                  grid(back_layer="back", specular_reflection=1, highlight_sharpness=20,
                       light_position=[20, 30], **HALF), (0,)),
    "FX-CW-035": ("Light colour #ffcc88 at intensity 1.5.",
                  grid(back_layer="back", light_color="#ffcc88", light_intensity=1.5, **HALF),
                  (0,)),
    "FX-CW-036": ("Ambient 0.5, diffuse 0.25.",
                  grid(back_layer="back", ambient_light=0.5, diffuse_reflection=0.25, **HALF),
                  (0,)),
    "FX-CW-037": ("Completion keyed 0 at frame 0 to 100 at frame 4: frame 0 is the picture, "
                  "frame 2 FX-CW-004, frame 4 FX-CW-005.",
                  grid(back_layer="back", transition_completion=keyed((0, 0), (4, 100))),
                  (0, 2, 4)),
    "FX-CW-038": ("FX-CW-004 on the holder moved 3 right and 1 down: the same, moved.",
                  grid(back_layer="back", shift=(3, 1), **HALF), (0,)),
    "FX-CW-039": ("A back layer not in the composition, `gone`: clear backs, as FX-CW-003, "
                  "and the warning every frame.", grid(back_layer="gone", **HALF), (0, 4)),
    "FX-CW-040": ("The back layer with its in point at frame 4: clear backs before it, "
                  "FX-CW-004 at it.", grid(back_layer="late", **HALF), (0, 4)),
    "FX-CW-041": ("Ten rows, one column, turning about x: venetian blinds.",
                  case(rows=10, columns=1, back_layer="back", flip_order="top_to_bottom",
                       **HALF), (0,)),
}

INVALID = {
    "FX-CW-042": ("Flip axis \"z\".", grid(flip_axis="z")),
    "FX-CW-043": ("Flip order \"diagonal\".", grid(flip_order="diagonal")),
    "FX-CW-044": ("Rows 0, below 1.", grid(rows=0)),
    "FX-CW-045": ("Columns 1001, above 1000.", grid(columns=1001)),
    "FX-CW-046": ("Card scale -1, below 0.", grid(card_scale=-1)),
    "FX-CW-047": ("Card scale 11, above 10.", grid(card_scale=11)),
    "FX-CW-048": ("Timing randomness 1.5, above 1.", grid(timing_randomness=1.5)),
    "FX-CW-049": ("Z position 0, below 0.01.", grid(z_position=0)),
    "FX-CW-050": ("Focal length 0, below 1.", grid(focal_length=0)),
    "FX-CW-051": ("Light type \"spot\".", grid(light_type="spot")),
    "FX-CW-052": ("Light colour \"white\": not a #rrggbb colour.", grid(light_color="white")),
    "FX-CW-053": ("Highlight sharpness 0, below 1.", grid(highlight_sharpness=0)),
    "FX-CW-054": ("A back layer written as the number 3, not a word.", grid(back_layer=3)),
    "FX-CW-055": ("Rows & Columns \"both\".", grid(rows_columns="both")),
    "FX-CW-056": ("Completion keyed to 120 at frame 4.",
                  grid(transition_completion=keyed((0, 0), (4, 120)))),
}

# D-634: the Corner Pins camera, with Card Dance's pin sets.
PIN = dict(camera_system="corner_pins")
KEYSTONE = dict(PIN, upper_left_corner=[20, 10], upper_right_corner=[85, 0],
                lower_right_corner=[100, 95])
SLANT = dict(PIN, upper_left_corner=[12.5, -12.5], upper_right_corner=[100, 0],
             lower_right_corner=[87.5, 112.5], lower_left_corner=[0, 100])
QUAD = dict(PIN, upper_left_corner=[5, 10], upper_right_corner=[90, -5],
            lower_right_corner=[100, 95], lower_left_corner=[-5, 105])
BACK_HALF = dict(back_layer="back", **HALF)
PIN_CASES = {
    "FX-CW-057": ("Camera system \"corner_pins\" with the pins as added: FX-CW-004 exactly.",
                  grid(**PIN, **BACK_HALF), (0,)),
    "FX-CW-058": ("Corner Pins, the upper corners pulled in to [20, 10] and [85, 0] and the "
                  "lower right up to [100, 95], Auto Focal Length off: FX-CW-004 seen at a slant.",
                  grid(**KEYSTONE, **BACK_HALF), (0,)),
    "FX-CW-059": ("FX-CW-058 with Auto Focal Length on: the lens worked out from the pins.",
                  grid(auto_focal_length="on", **KEYSTONE, **BACK_HALF), (0,)),
    "FX-CW-060": ("Corner Pins on a parallelogram, Auto Focal Length on: no perspective to work "
                  "a lens from, so the focal length is used.",
                  grid(auto_focal_length="on", **SLANT, **BACK_HALF), (0,)),
    "FX-CW-061": ("FX-CW-060 with Auto Focal Length off: the same picture.",
                  grid(**SLANT, **BACK_HALF), (0,)),
    "FX-CW-062": ("Corner Pins on a general four-sided shape, Auto Focal Length on, a point "
                  "light at [20, 30] depth 0.3, specular 1, sharpness 20: the light stays on "
                  "the layer.", grid(auto_focal_length="on", light_type="point",
                                     light_position=[20, 30], light_depth=0.3,
                                     specular_reflection=1, highlight_sharpness=20, **QUAD,
                                     **BACK_HALF), (0,)),
    "FX-CW-063": ("FX-CW-059 with the camera turned 30 about y, at x 40, y 50, Z position 1.7: "
                  "Corner Pins does not use them, so FX-CW-059.",
                  grid(auto_focal_length="on", y_rotation=30, xy_position=[40, 50],
                       z_position=1.7, **KEYSTONE, **BACK_HALF), (0,)),
    "FX-CW-064": ("Camera system \"camera_position\" written out: FX-CW-004.",
                  grid(camera_system="camera_position", **BACK_HALF), (0,)),
    "FX-CW-065": ("Corner Pins with the upper right corner keyed [100, 0] at frame 0 to "
                  "[80, 20] at frame 4: frame 0 FX-CW-004, frame 4 pulled in.",
                  grid(upper_right_corner=keyed((0, [100, 0]), (4, [80, 20])), **PIN,
                       **BACK_HALF), (0, 4)),
}
PIN_INVALID = {
    "FX-CW-066": ("Camera system \"comp_camera\", not built for Card Wipe (a gap).",
                  grid(camera_system="comp_camera")),
    "FX-CW-067": ("Camera system \"orbit\".", grid(camera_system="orbit")),
    "FX-CW-068": ("Upper left corner [-1001, 0], below -1000.",
                  grid(upper_left_corner=[-1001, 0], **PIN)),
    "FX-CW-069": ("Auto focal length \"maybe\".", grid(auto_focal_length="maybe", **PIN)),
}


def effect(fid, c):
    params = {w: c[w] for w in WORDS}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    params.update({n: setting_json(c[n]) for n in CD.PINS if n in c["given"]})
    params.update({w: c[w] for w in ("camera_system", "auto_focal_length") if w in c["given"]})
    return {"instance_id": fid, "type_id": "core.card_wipe", "enabled": True,
            "parameters": params}


def map_layers():
    return [L.raster(n, "asset-" + n, enabled=False, out_frame=FRAMES) for n in ("back", "ramp")] + [
        L.raster("late", "asset-back", enabled=False, in_frame=4, out_frame=FRAMES)]


def project_json(fx, c):
    holder = L.raster("holder", "asset-shot", position=c["shift"], out_frame=FRAMES)
    holder["effects"] = [effect("fx-1", c)]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, FRAMES, map_layers() + [holder])]}


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
        if c["back_layer"] == "gone":
            entry["warning"] = "EFFECT_LAYER_MISSING"
        expected["cases"][fx] = entry
        before = plain(c)
        print(f"{fx}: " + ", ".join(f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} "
                                    "changed" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
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

    (OUT / "expected_card_wipe.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


# As added, 8 rows on 10 pixels put a row's edge on pixel centres (y = 2.5 and 7.5). The numbers
# there are exact in binary (1.25, 0.625, t = 1), so the edge test is exact in any order of
# working, and the cards on either side hold the same colour there.
EXACT = {"FX-CW-001"}


def step_clear(fx, c, frames):
    """No pixel's ray lies near a card's edge, an edge-on card, two cards tied at differing
    colours, or the highlight's cut."""
    for f in frames:
        k = settings(c, f)
        drawing = decoded("shot")
        back, grad = map_picture(c["back_layer"], f), map_picture(c["gradient_layer"], f)
        deck = cards(k, grad)
        if view(k) is None:
            continue
        m, o, ray = view(k)
        sc = k["card_scale"]
        for y in range(H):
            for x in range(W):
                d = ray(x, y)
                for i, (a, b, cx, cy, cw, ch, axis, s, cc) in enumerate(deck):
                    o1 = [o[0] - (cx - W / 2), o[1] - (cy - H / 2), o[2]]
                    assert abs(inv(axis, s, cc, d)[2]) > 1e-9, (fx, f, x, y, i)
                # A card's edge near the pixel must not matter: the colour is the same with
                # every card a little larger or a little smaller.
                pick = lambda g: [  # noqa: E731
                    shade(k, m, o, d, min(hs, key=lambda h: (h[0], h[1])), drawing, back)
                    if hs else list(CLEAR) for hs in [hits(k, deck, o, d, g)]][0]
                mid = pick(0.0)
                for g in (-1e-6, 1e-6) if fx not in EXACT else ():
                    assert max(abs(p - q) for p, q in zip(pick(g), mid)) < 1e-9, (fx, f, x, y, g)
                found = hits(k, deck, o, d)
                found.sort(key=lambda h: (h[0], h[1]))
                if len(found) > 1:
                    t0, t1 = found[0][0], found[1][0]
                    if t1 - t0 <= 1e-6:
                        c0 = shade(k, m, o, d, found[0], drawing, back)
                        c1 = shade(k, m, o, d, found[1], drawing, back)
                        assert t0 == t1 or max(abs(p - q) for p, q in zip(c0, c1)) < 1e-12, \
                            (fx, f, x, y)
                        assert t0 != t1 or all(h[2][7] == 0 for h in found[:2]) or \
                            max(abs(p - q) for p, q in zip(c0, c1)) < 1e-12, (fx, f, x, y)
                if found and k["specular_reflection"] > 0:
                    t, _, card, lx, ly, front, _, _ = found[0]
                    axis, s, cc = card[6], card[7], card[8]
                    lp = k["light_position"]
                    pl = turn_t(m, [lp[0] * W / 100 - W / 2, lp[1] * H / 100 - H / 2,
                                    -k["light_depth"] * W])
                    h = [o[j] + t * d[j] for j in range(3)]
                    lv = unit_vec(pl if k["light_type"] == "distant" else
                                  [pl[j] - h[j] for j in range(3)])
                    n = [0.0, s, -cc] if axis == 0 else [-s, 0.0, -cc]
                    nl = sum(n[j] * lv[j] for j in range(3))
                    assert abs(nl) > 1e-6, (fx, f, x, y)


def near(a, b, eps=1e-12):
    return all(abs(x - y) <= eps for p, q in zip(a, b) for x, y in zip(p, q))


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    back = L.fit(decoded("back"), "stretch", W, H)["px"]
    # As added: the picture, to rounding.
    assert near(c["FX-CW-001"]["0"], drawn)
    # Completion 100, no back: clear; with the back: the back picture whole.
    assert all(p == CLEAR for p in c["FX-CW-002"]["0"])
    assert near(c["FX-CW-005"]["0"], back, 1e-9)
    # Half way, left to right: the first column clear, the last unchanged.
    h = c["FX-CW-003"]["0"]
    for y in range(H):
        for x in range(4):
            assert h[y * W + x] == CLEAR, (x, y)
        for x in range(12, 16):
            assert near([h[y * W + x]], [drawn[y * W + x]]), (x, y)
    # The back layer shows on the first column, nothing changes on the last.
    b4 = c["FX-CW-004"]["0"]
    assert all(near([b4[y * W + x]], [back[y * W + x]], 1e-9) for y in range(H) for x in range(4))
    # Right to left mirrors which columns are done.
    r = c["FX-CW-008"]["0"]
    assert all(near([r[y * W + x]], [back[y * W + x]], 1e-9) for y in range(H) for x in range(12, 16))
    # The gradient with none named: every card the same way over, so each column alike.
    g = c["FX-CW-016"]["0"]
    for y in range(H):
        for x in range(4):
            assert (g[y * W + x][3] == 0) == (g[y * W + x + 4][3] == 0) or drawn[y * W + x + 4][3] == 0 \
                or drawn[y * W + x][3] == 0
    # Seeds 7 and 8 differ; width 0 is a hard step.
    assert c["FX-CW-017"]["0"] != c["FX-CW-018"]["0"]
    w0 = c["FX-CW-020"]["0"]
    assert all(near([w0[y * W + x]], [back[y * W + x]], 1e-9) for y in range(H) for x in range(8))
    assert all(near([w0[y * W + x]], [drawn[y * W + x]]) for y in range(H) for x in range(8, 16))
    # Card scale 0.5: gaps between the cards are clear.
    s5 = c["FX-CW-021"]["0"]
    assert all(s5[y * W + x] == CLEAR for y in range(H) for x in (0, 3, 4, 7))
    # Keyed: frame 0 the picture, frame 2 half way, frame 4 the back.
    k = c["FX-CW-037"]
    assert near(k["0"], drawn) and near(k["2"], b4, 1e-12) and near(k["4"], back, 1e-9)
    # Gone: as no back; late: clear backs, then the back.
    assert c["FX-CW-039"]["0"] == h and c["FX-CW-039"]["4"] == h
    assert c["FX-CW-040"]["0"] == h and c["FX-CW-040"]["4"] == b4
    # Moved: the same picture, three right and one down.
    moved = c["FX-CW-038"]["0"]
    assert all(moved[(y + 1) * W + x + 3] == b4[y * W + x] for y in range(H - 1) for x in range(W - 3))
    # Lights change the picture; colour tints it warm.
    for fx in ("FX-CW-032", "FX-CW-033", "FX-CW-034", "FX-CW-035", "FX-CW-036"):
        assert c[fx]["0"] != b4, fx
    # D-634: the pins as added are the camera as added; the pins change the picture and the
    # camera's own settings do not.
    assert near(c["FX-CW-057"]["0"], b4, 1e-9) and near(c["FX-CW-064"]["0"], b4, 1e-12)
    assert near(c["FX-CW-065"]["0"], b4, 1e-9) and not near(c["FX-CW-065"]["4"], b4, 1e-6)
    assert not near(c["FX-CW-058"]["0"], b4, 1e-6)
    assert not near(c["FX-CW-058"]["0"], c["FX-CW-059"]["0"], 1e-6)
    assert near(c["FX-CW-060"]["0"], c["FX-CW-061"]["0"], 1e-12)
    assert near(c["FX-CW-063"]["0"], c["FX-CW-059"]["0"], 1e-12)
    print("checks passed")


if __name__ == "__main__":
    main()
