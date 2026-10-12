"""Mercury, worked a second way.

D-674 adds Mercury (`core.mercury`), after CycoreFX's CC Mr. Mercury (Simulation): the layer
turned into drops of liquid metal, born at a producer, thrown out and falling, swelling and
shrinking over their lives, running together where they meet and lit as a shiny liquid. The
CycoreFX manual names the controls and says what each is for (Radius X and Y, the producer's
size; Producer, its place; Direction; Velocity; Birth Rate; Longevity; Gravity; Resistance;
Extra, more randomness; Animation, the way the drops move; Blob Influence, how much they run
together; Influence Map, how they grow and shrink over their lives; Blob Birth Size and Blob
Death Size; and the Light and Shading groups shared with CC Blobbylize). It publishes no
formula, so the rule, the units, the ranges, the ways the drops move and the settings when
added are ours. The layer is replaced by its drops: where there is none the layer is clear. No
state is carried from frame to frame (document 20): every drop is worked from the in point, its
path in closed form. There is no seed control, as CycoreFX's has none.

At composition frame f, the layer's in point i0, r frames a second, u = f - i0 frames since the
in point, on a drawing w by h:

1. The drops born so far, as Drizzle's (D-672): C(0) = 0, C(m + 1) = C(m) + (rate(m) + rate(m +
   1)) / (2 r), rate(m) `birth_rate` (drops a second) at frame i0 + m; drop j (j = 1, 2, ...) is
   born at b_j, the first t with C(t) = j, between whole frames m and m + 1 (C(m) < j <= C(m +
   1)). L = `longevity` in seconds, as it is at the in point. Drop j is alive at u when C(u - L r)
   < j <= C(u); its age a = (u - b_j) / r seconds, s = a / L. With L = 0 no drop lives.
2. Every setting of a drop's birth and path is read at frame i0 + m, the whole frame its birth
   follows, so a keyed producer leaves a trail and a keyed direction turns only the drops born
   after it: the producer, its radii, the direction, velocity, extra, gravity, resistance and
   the two sizes. u_k = (U(0, j, 0, 0, k) + 1) / 2, U Noise's hash (D-119) at seed 0.
3. Born at p_0 = P + (R_x sqrt(u_0) cos(2 pi u_1), R_y sqrt(u_0) sin(2 pi u_1)): P =
   (`producer_x` w, `producer_y` h) / 100, R_x = (`radius_x` / 100) h, R_y likewise. The drop
   lives only where u_2 < the layer's alpha at p_0 (the bilinear read below), and takes the
   layer's straight colour there.
4. Its speed v = S (1 - (`extra` / 100) u_3), S = (`velocity` / 100) h a second. With the
   resistance k = `resistance` / 100 a second, F_1 = a and F_2 = a^2 / 2 when k = 0, else F_1 = (1 -
   e^(-k a)) / k and F_2 = (a - F_1) / k; the gravity G = (`gravity` / 100) h a second a second,
   downwards. By `animation`:
   - `explosive`: thrown at the angle 2 pi u_4, p = p_0 + v (cos, sin) F_1 + (0, G F_2);
   - `direction`: thrown the way (as Vector Blur's angle, D-336) `direction` + (u_4 - 1/2) 90
     (`extra` / 100) degrees points, likewise;
   - `twirl`: thrown square to its place from the producer, o = p_0 - P, along (-o_y, o_x) / |o|
     (the way `direction` points where o = 0), so the drops turn round the producer like a
     pinwheel;
   - `vortex`: circling the producer: at the distance |o| + (v / 2) F_1 and the angle atan2(o_y,
     o_x) + 2 pi (v / h) F_1, plus (0, G F_2);
   - `fixed`: staying at p_0.
5. Its radius r_j = R_0 (B + (D - B) s) m(s), R_0 = h / 20, B = `birth_size` / 100, D =
   `death_size` / 100; by `influence_map`, m(s) = min(1, 4 s, 4 (1 - s)) for `in_out`, min(1, 4
   s) for `in`, min(1, 4 (1 - s)) for `out`, 1 for `constant`: swelling in over the first
   quarter of its life, shrinking out over the last. A drop of radius 0 is not drawn.
6. The drops run together as metaballs: with A = `blob_influence` / 100, drop j reaches rho_j =
   r_j (1 + A), and at a pixel centre (X, Y), d its distance and q = d / rho_j, it adds f = (1 -
   q^2)^2 to the field F where q < 1, -4 (1 - q^2) (dX, dY) / rho_j^2 to its slope (F_X, F_Y),
   f rho_j to a weight and f times its colour to a colour sum. The surface is where F >= tau =
   (1 - 1 / (1 + A)^2)^2, so a drop alone is a disc of radius r_j; with A = 0 the drops are hard
   discs over each other.
7. Where F > 0 the pixel's coverage cov = clamp((F - tau) / |(F_X, F_Y)| + 1/2, 0, 1), or 1 where
   the slope is 0 and F > tau, else 0: the surface's edge, a pixel soft. Its colour col = the
   colour sum / F; c = the weight / F; the surface's normal N = (-c F_X, -c F_Y, 2 sqrt(max(F -
   tau, 0))) / |.|, (0, 0, 1) where that is 0: a drop alone is a half sphere.
8. Lit by Drizzle's light (D-672, Blobbylize's rule, D-379): L = (100 u_x, 100 u_y,
   `light_height`) / |.|, (u_x, u_y) the way `light_direction` points, or for a point light (l_x
   - X, l_y - Y, `light_height`) / |.|, (l_x, l_y) = (`light_position_x` w, `light_position_y`
   h) / 100; nl = N.L, rz = 2 nl N_z - L_z, shine = rz^(1 / `roughness`) where nl > 0 and rz > 0,
   else 0; each channel lit = col (ambient + diffuse I Lc max(nl, 0)) + specular I (Lc + (col -
   Lc) metal) shine, I = `light_intensity` / 100, Lc the light's colour linear, the shares over
   100. The pixel is (lit cov, cov); where F = 0 or cov = 0 it is clear.

The bilinear read, premultiplied and linear: fx = clamp(X' - 1/2, 0, w - 1), likewise fy; the
four pixels round it, the last row and column repeated at the edge.

`birth_rate` 0 to 1000 drops a second, 20 when added; `longevity` 0 to 100 seconds, 2;
`producer_x` and `_y` -1000 to 1000 per cent, 50 and 50; `radius_x` and `_y` 0 to 1000 per cent of
the height, 10 and 10; `direction` -3600 to 3600, 0; `velocity` 0 to 1000, 50; `extra` 0 to 100,
50; `gravity` -1000 to 1000, 50; `resistance` 0 to 1000, 0; `blob_influence` 0 to 400, 100;
`birth_size` and `death_size` 0 to 1000, 100 and 50; `light_intensity` 0 to 400, 100;
`light_height` -1000 to 1000, 100; `light_position_x` and `_y` -1000 to 1000, 30 and 30;
`light_direction` -3600 to 3600, -45; `ambient` 20, `diffuse` 60, `specular` 80, `metal` 100,
each 0 to 100; `roughness` 0.001 to 1, 0.05; `animation` `explosive`, `direction`, `twirl`,
`vortex` or `fixed`, explosive; `influence_map` `in_out`, `in`, `out` or `constant`, in_out;
`light_type` `distant` or `point`, distant; `light_color` #rrggbb, #ffffff. Every number
keyable: `birth_rate` summed from the in point, `longevity` read at the in point, the birth and
path settings at each drop's birth, the rest as they are now. Every length is a share of the
drawing's height, but for a point light's height, in pixels, which a draft scales. The layer
does not grow: drops thrown past its edge are cut there. After Effects' lights are not offered.

**This file never runs the build's code path.** It sums every live drop at every pixel, where
the build skips the drops out of reach, in double precision from the drawings' 8-bit values.

Every case is a composition 48 pixels by 32 holding one drawing the same size: Bubbles' colour
field, or the field with its right half clear. The projects go into `Fixtures/mercury`, the
expected frames into `Fixtures/mercury/expected_mercury.json`. Every case is "not built yet"
until B-553 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/mercury_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from vector_blur_reference import along  # noqa: E402
from noise_reference import u as U  # noqa: E402
import smooth_reference as S  # noqa: E402
import blobbylize_reference as BL  # noqa: E402
import bubbles_reference as BU  # noqa: E402

W, H, FPS = 48, 32, 24
assert (BU.W, BU.H) == (W, H)
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "mercury"
TOLERANCE = 2e-5  # document 25's default for a filter
TYPE = "core.mercury"
SETTINGS = [("birth_rate", 0, 1000, 20), ("longevity", 0, 100, 2),
            ("producer_x", -1000, 1000, 50), ("producer_y", -1000, 1000, 50),
            ("radius_x", 0, 1000, 10), ("radius_y", 0, 1000, 10),
            ("direction", -3600, 3600, 0), ("velocity", 0, 1000, 50), ("extra", 0, 100, 50),
            ("gravity", -1000, 1000, 50), ("resistance", 0, 1000, 0),
            ("blob_influence", 0, 400, 100), ("birth_size", 0, 1000, 100),
            ("death_size", 0, 1000, 50), ("light_intensity", 0, 400, 100),
            ("light_height", -1000, 1000, 100), ("light_position_x", -1000, 1000, 30),
            ("light_position_y", -1000, 1000, 30), ("light_direction", -3600, 3600, -45),
            ("ambient", 0, 100, 20), ("diffuse", 0, 100, 60), ("specular", 0, 100, 80),
            ("roughness", 0.001, 1, 0.05), ("metal", 0, 100, 100)]
WORDS = [("animation", ("explosive", "direction", "twirl", "vortex", "fixed"), "explosive"),
         ("influence_map", ("in_out", "in", "out", "constant"), "in_out"),
         ("light_type", ("distant", "point"), "distant")]
NAMES = [s[0] for s in SETTINGS]
RANGES = {s[0]: (s[1], s[2]) for s in SETTINGS}


# --- the rule -------------------------------------------------------------------------------

def clamp(v, lo, hi):
    return min(hi, max(lo, v))


def settings(c, f):
    return {n: clamp(value_at(c[n], f), *RANGES[n]) for n in NAMES}


def counts(c, u):
    """Step 1: C(0) .. C(u)."""
    i0, r = c["in_frame"], c["rate"]
    rate = [settings(c, i0 + m)["birth_rate"] for m in range(u + 1)]
    out = [0.0]
    for m in range(u):
        out.append(out[-1] + (rate[m] + rate[m + 1]) / (2 * r))
    return out


def count_at(C, t):
    if t <= 0:
        return 0.0
    m = math.floor(t)
    if m >= len(C) - 1:
        return C[-1]
    return C[m] + (C[m + 1] - C[m]) * (t - m)


def size_share(word, s):
    if word == "in_out":
        return min(1.0, 4 * s, 4 * (1 - s))
    if word == "in":
        return min(1.0, 4 * s)
    if word == "out":
        return min(1.0, 4 * (1 - s))
    return 1.0


def drops(c, f, src, w=W, h=H):
    """Steps 1 to 5: the live drops at frame f as (x, y, r, straight colour), in birth order."""
    u = f - c["in_frame"]
    r = c["rate"]
    life = clamp(value_at(c["longevity"], c["in_frame"]), *RANGES["longevity"])
    if u < 0 or life == 0:
        return []
    C = counts(c, u)
    lo, hi = count_at(C, u - life * r), C[-1]
    out = []
    m = 0
    for j in range(math.floor(lo) + 1, math.floor(hi) + 1):
        if j <= lo:
            continue
        while C[m + 1] < j:
            m += 1
        b = m + (j - C[m]) / (C[m + 1] - C[m])
        a = (u - b) / r
        s = a / life
        v = settings(c, c["in_frame"] + m)
        uk = [(U(0, j, 0, 0, k) + 1) / 2 for k in range(5)]
        px, py = v["producer_x"] / 100 * w, v["producer_y"] / 100 * h
        rx, ry = v["radius_x"] / 100 * h, v["radius_y"] / 100 * h
        x0 = px + rx * math.sqrt(uk[0]) * math.cos(2 * math.pi * uk[1])
        y0 = py + ry * math.sqrt(uk[0]) * math.sin(2 * math.pi * uk[1])
        col = BU.bilinear(src, x0, y0, w, h)
        if not uk[2] < col[3]:
            continue
        sp = v["velocity"] / 100 * h * (1 - v["extra"] / 100 * uk[3])
        k = v["resistance"] / 100
        f1 = a if k == 0 else -math.expm1(-k * a) / k
        f2 = a * a / 2 if k == 0 else (a - f1) / k
        g = v["gravity"] / 100 * h * f2
        anim = c["animation"]
        ox, oy = x0 - px, y0 - py
        if anim == "fixed":
            x, y = x0, y0
        elif anim == "vortex":
            rad = math.hypot(ox, oy) + sp / 2 * f1
            ang = math.atan2(oy, ox) + 2 * math.pi * sp / h * f1
            x, y = px + rad * math.cos(ang), py + rad * math.sin(ang) + g
        else:
            if anim == "explosive":
                t = 2 * math.pi * uk[4]
                dx, dy = math.cos(t), math.sin(t)
            elif anim == "direction":
                dx, dy = along(v["direction"] + (uk[4] - 0.5) * 90 * v["extra"] / 100)
            else:  # twirl
                n = math.hypot(ox, oy)
                dx, dy = (-oy / n, ox / n) if n > 0 else along(v["direction"])
            x, y = x0 + sp * dx * f1, y0 + sp * dy * f1 + g
        rj = h / 20 * (v["birth_size"] / 100 + (v["death_size"] - v["birth_size"]) / 100 * s) \
            * size_share(c["influence_map"], s)
        if rj <= 0:
            continue
        out.append((x, y, rj, [col[q] / col[3] for q in range(3)]))
    return out


def mercury(v, light_type, light_color, live, w=W, h=H):
    big_a = v["blob_influence"] / 100
    tau = (1 - 1 / (1 + big_a) ** 2) ** 2
    lc = BL.hex_linear(light_color)
    inten = v["light_intensity"] / 100
    amb, dif, spc, metal = (v[x] / 100 for x in ("ambient", "diffuse", "specular", "metal"))
    ux, uy = along(v["light_direction"])
    out = []
    for py in range(h):
        for px in range(w):
            X, Y = px + 0.5, py + 0.5
            F = fx = fy = wt = 0.0
            cs = [0.0, 0.0, 0.0]
            for (x, y, rj, col) in live:
                rho = rj * (1 + big_a)
                dx, dy = X - x, Y - y
                q2 = (dx * dx + dy * dy) / (rho * rho)
                if q2 >= 1:
                    continue
                one = 1 - q2
                f = one * one
                F += f
                fx += -4 * one * dx / (rho * rho)
                fy += -4 * one * dy / (rho * rho)
                wt += f * rho
                for i in range(3):
                    cs[i] += f * col[i]
            if F <= 0:
                out.append([0.0] * 4)
                continue
            gl = math.hypot(fx, fy)
            if gl > 0:
                cov = clamp((F - tau) / gl + 0.5, 0, 1)
            else:
                cov = 1.0 if F > tau else 0.0
            if cov == 0:
                out.append([0.0] * 4)
                continue
            col = [q / F for q in cs]
            cc = wt / F
            nn = [-cc * fx, -cc * fy, 2 * math.sqrt(max(F - tau, 0.0))]
            ln = math.sqrt(sum(t * t for t in nn))
            nn = [t / ln for t in nn] if ln > 0 else [0.0, 0.0, 1.0]
            if light_type == "point":
                lv = [v["light_position_x"] / 100 * w - X, v["light_position_y"] / 100 * h - Y,
                      v["light_height"]]
            else:
                lv = [100 * ux, 100 * uy, v["light_height"]]
            ll = math.sqrt(sum(t * t for t in lv))
            lv = [t / ll for t in lv] if ll > 0 else [0.0] * 3
            nl = sum(nn[i] * lv[i] for i in range(3))
            rz = 2 * nl * nn[2] - lv[2]
            shine = rz ** (1 / v["roughness"]) if nl > 0 and rz > 0 else 0.0
            lit = [col[i] * (amb + dif * inten * lc[i] * max(nl, 0.0))
                   + spc * inten * (lc[i] + (col[i] - lc[i]) * metal) * shine for i in range(3)]
            out.append([lit[i] * cov for i in range(3)] + [cov])
    return out


def valid(c):
    for n, lo, hi, _ in SETTINGS:
        v = c[n]
        values = [k[1] for k in v["keys"]] if isinstance(v, dict) else [v]
        if not all(lo <= x <= hi for x in values):
            return False
    if not all(c[name] in words for name, words, _ in WORDS):
        return False
    lc = c["light_color"]
    return len(lc) == 7 and lc[0] == "#" and all(ch in "0123456789abcdefABCDEF" for ch in lc[1:])


def render(c, f):
    src = BU.working(c["drawing"])
    if not valid(c) or f < c["in_frame"]:
        return BU.frame(src, c["shift"])
    out = mercury(settings(c, f), c["light_type"], c["light_color"], drops(c, f, src))
    return BU.frame(out, c["shift"])


def plain(c):
    return BU.frame(BU.working(c["drawing"]), c["shift"])


# --- the cases ------------------------------------------------------------------------------

def case(drawing="field", rate=FPS, in_frame=0, shift=0, **given):
    c = {s[0]: s[3] for s in SETTINGS}
    c.update({w[0]: w[2] for w in WORDS})
    c["light_color"] = "#ffffff"
    for key, v in given.items():
        assert key in c, key
        c[key] = v
    c.update(drawing=drawing, rate=rate, in_frame=in_frame, shift=shift)
    return c


# A close look: six drops a second, born 5 pixels across and shrinking to half, from a producer
# 13 pixels wide in the middle, thrown at 16 pixels a second.
BIG = dict(birth_rate=6, birth_size=300, death_size=150, radius_x=20, radius_y=20)
# Unlit: Ambient 100 and no diffuse or specular light, so each drop is its colour, flat.
FLAT = dict(ambient=100, diffuse=0, specular=0)


def big(**kw):
    return case(**{**BIG, **kw})


CASES = {
    "FX-MERCURY-001": ("As added: twenty drops a second from the middle, thrown every way at "
                       "half the picture's height a second and falling, each 1.6 pixels across "
                       "the radius here, swelling in and shrinking out over its two seconds; "
                       "the picture replaced by them. At frame 0 none is born and the layer is "
                       "clear.", case(), (0, 12, 24)),
    "FX-MERCURY-002": ("A close look: about six drops a second, 5 pixels across the radius, lit "
                       "from the top left; frame 6 one young drop, frame 24 several running "
                       "together.", big(), (6, 12, 24)),
    "FX-MERCURY-003": ("Animation Direction, Direction 90, Extra 0: every drop thrown to the "
                       "right at one speed, falling.", big(animation="direction", direction=90,
                                                             extra=0), (18,)),
    "FX-MERCURY-004": ("Animation Direction, Direction 90, Extra 100: thrown to the right in a "
                       "fan 90 degrees wide, at speeds from none to full.",
                       big(animation="direction", direction=90, extra=100), (18,)),
    "FX-MERCURY-005": ("Animation Twirl, no gravity: the drops thrown round the producer like "
                       "a pinwheel.", big(animation="twirl", gravity=0, velocity=100), (18,)),
    "FX-MERCURY-006": ("Animation Vortex, no gravity: the drops spiralling out round the "
                       "producer.", big(animation="vortex", gravity=0, velocity=100), (18,)),
    "FX-MERCURY-007": ("Animation Fixed: the drops stay where they are born, swelling and "
                       "shrinking.", big(animation="fixed"), (18,)),
    "FX-MERCURY-008": ("Gravity -50: the drops float up.", big(gravity=-50), (18,)),
    "FX-MERCURY-009": ("Resistance 300: the drops slow and hang closer to the producer.",
                       big(resistance=300, velocity=150), (18,)),
    "FX-MERCURY-010": ("Blob Influence 0: hard discs over each other, not running together.",
                       big(blob_influence=0), (24,)),
    "FX-MERCURY-011": ("Blob Influence 300: the drops reach far and run into one puddle.",
                       big(blob_influence=300), (24,)),
    "FX-MERCURY-012": ("Influence Map In: drops swell in and never shrink out.",
                       big(influence_map="in"), (24,)),
    "FX-MERCURY-013": ("Influence Map Out: drops born at full size, shrinking out.",
                       big(influence_map="out"), (24,)),
    "FX-MERCURY-014": ("Influence Map Constant: no swelling or shrinking but from the birth "
                       "size to the death size.", big(influence_map="constant"), (24,)),
    "FX-MERCURY-015": ("Birth Size 0, Death Size 400: drops born as points, growing to 6.4 "
                       "pixels across the radius.",
                       big(birth_size=0, death_size=400, influence_map="constant"), (24,)),
    "FX-MERCURY-016": ("Longevity 0: no drop lives, the layer clear.", big(longevity=0), (12,)),
    "FX-MERCURY-017": ("Birth Rate 0: no drops, the layer clear.", big(birth_rate=0), (12,)),
    "FX-MERCURY-018": ("The picture with its right half clear, the producer as wide as the "
                       "picture: drops are born only on the left half.",
                       big(drawing="half", radius_x=75, radius_y=40, birth_rate=24,
                           animation="fixed", influence_map="constant"), (12,)),
    "FX-MERCURY-019": ("Birth Rate keyed from 0 at frame 0 to 24 at frame 12: by frame 11 five "
                       "drops have been born, the rate summed from the in point.",
                       big(birth_rate=keyed((0, 0), (12, 24)), animation="fixed",
                           influence_map="constant"), (6, 11)),
    "FX-MERCURY-020": ("The producer keyed from 20 per cent across at frame 0 to 80 at frame "
                       "24, no gravity or speed: the drops left in a trail behind it.",
                       big(producer_x=keyed((0, 20), (24, 80)), animation="fixed",
                           influence_map="constant", birth_rate=12), (24,)),
    "FX-MERCURY-021": ("FX-MERCURY-002 moved 3 pixels right: the same, moved.", big(shift=3),
                       (24,)),
    "FX-MERCURY-022": ("FX-MERCURY-002 with the layer starting at frame 6: frame 12 is 002's "
                       "frame 6 and frame 30 its 24.", big(in_frame=6), (12, 30)),
    "FX-MERCURY-023": ("An orange point light near the top left, 10 pixels up, Metal 0: "
                       "white-ish drops lit orange on the side facing it.",
                       big(light_type="point", light_position_x=20, light_position_y=20,
                           light_height=10, light_color="#ff8000", metal=0), (24,)),
    "FX-MERCURY-024": ("Unlit: each drop the picture's colour where it was born, flat.",
                       big(**FLAT), (24,)),
    "FX-MERCURY-025": ("FX-MERCURY-002 at 30 frames a second: frame 24 is four fifths of a "
                       "second in, not one.", big(rate=30), (24,)),
    "FX-MERCURY-026": ("Animation Direction, the direction keyed from 0 at frame 0 to 180 at "
                       "frame 12, no gravity: the drops born early still go up, the late ones "
                       "down.", big(animation="direction", direction=keyed((0, 0), (12, 180)),
                                    extra=0, gravity=0, longevity=4), (24,)),
}

# D-46: the file is read, the effect is kept as written and left out of every frame.
INVALID = {
    "FX-MERCURY-027": ("Birth Rate -1, below 0.", big(birth_rate=-1)),
    "FX-MERCURY-028": ("Longevity 101, above 100 seconds.", big(longevity=101)),
    "FX-MERCURY-029": ("Radius X -1, below 0.", big(radius_x=-1)),
    "FX-MERCURY-030": ("Velocity -1, below 0.", big(velocity=-1)),
    "FX-MERCURY-031": ("Extra 101, above 100.", big(extra=101)),
    "FX-MERCURY-032": ("Gravity 1001, above 1000.", big(gravity=1001)),
    "FX-MERCURY-033": ("Blob Influence 401, above 400.", big(blob_influence=401)),
    "FX-MERCURY-034": ("Death Size 1001, above 1000.", big(death_size=1001)),
    "FX-MERCURY-035": ("An animation written \"spin\".", big(animation="spin")),
    "FX-MERCURY-036": ("An influence map written \"both\".", big(influence_map="both")),
    "FX-MERCURY-037": ("A light colour written \"silver\".", big(light_color="silver")),
    "FX-MERCURY-038": ("Gravity keyed to 2000 at frame 4, above 1000.",
                       big(gravity=keyed((0, 50), (4, 2000)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    d = c["drawing"]
    frames = 36
    params = {n: setting_json(c[n]) for n in NAMES}
    params.update({w[0]: c[w[0]] for w in WORDS})
    params["light_color"] = c["light_color"]
    return {
        "schema_version": 0,
        "project_id": "proj-" + fx.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-" + d, "kind": "still", "name": d, "path": f"media/{d}.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
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
        }],
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
        src = BU.working(c["drawing"])
        print(f"{fx}: " + ", ".join(
            f"frame {f} {len(drops(c, int(f), src))} drops, "
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
    (OUT / "expected_mercury.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    clear = [[0.0] * 4 for _ in range(W * H)]
    src = BU.working("field")
    assert c["FX-MERCURY-001"]["0"] == clear
    for fx in ("FX-MERCURY-016", "FX-MERCURY-017"):
        assert c[fx]["12"] == clear, fx
    assert [len(drops(big(), f, src)) for f in (6, 24)] == [1, 5]
    assert len(drops(big(birth_rate=keyed((0, 0), (12, 24))), 11, src)) == 5  # C(11) = 121 / 24
    two = c["FX-MERCURY-002"]
    assert all(two[f] != clear for f in two) and two["12"] != two["24"]
    assert c["FX-MERCURY-022"]["12"] == two["6"] and c["FX-MERCURY-022"]["30"] == two["24"]
    moved = c["FX-MERCURY-021"]["24"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == two["24"][y * W:y * W + W - 3]
    for n in range(3, 16):
        assert c["FX-MERCURY-%03d" % n] != two, n
    for n in (23, 24, 25):
        assert c["FX-MERCURY-%03d" % n]["24"] != two["24"], n
    # the clear half: no drop born right of the middle
    half = drops(c=big(drawing="half", radius_x=75, radius_y=40, birth_rate=24,
                       animation="fixed", influence_map="constant"),
                 f=12, src=BU.working("half"))
    assert half and all(x < W / 2 + 1 for x, *_ in half)
    # a keyed producer leaves a trail: the drops' places spread across
    trail = drops(big(producer_x=keyed((0, 20), (24, 80)), animation="fixed",
                      influence_map="constant", birth_rate=12), 24, src)
    assert max(x for x, *_ in trail) - min(x for x, *_ in trail) > W / 3
    # a keyed direction: early drops up, late drops down
    turned = drops(big(animation="direction", direction=keyed((0, 0), (12, 180)), extra=0,
                       gravity=0, longevity=4), 24, src)
    assert turned[0][1] < H / 2 < turned[-1][1]
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(a >= -1e-12 for a in p) and p[3] <= 1 + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
