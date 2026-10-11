"""Screen Key, worked a second way.

`core.screen_key` (D-540) is this program's own green and blue screen keyer. It is not a copy of
The Foundry's Keylight, which is licensed: no Keylight code or text is used, and the setting
names are our own. What it shares with Keylight is the openly published idea every colour
difference keyer uses, and the order an artist works in (pick the screen colour, look at the
matte, firm it up with the clip levels, remove the spill, then shrink and soften the edge):

- Colour difference matte (Vlahos' blue screen patents; Steve Wright, "Digital Compositing for
  Film and Video", chapter on colour difference keys): how far a pixel's screen channel stands
  above the other two, measured against how far the screen colour's does. Keylight's own user
  guide describes its saturation the same way, "comparing the intensity of the primary
  component against a weighted average of the two other components", with its balance at 100%
  taking the smaller, at 0% the larger and at 50% their average; Screen Balance here is that.
- Despill by limiting (Wright, the unspill operations; the openfx Despill node's spill mix):
  the screen channel is held no higher than a blend of the other two.
- Matte refinement with this program's own Simple Choker (D-159) and Refine Soft Matte's
  guided-filter fit (He, Sun and Tang 2010, as Key Cleaner, D-503).

The settings. `screen_color`, a "#rrggbb" colour, "#00ff00" when added; `screen_gain`, 0 to
200, 100 when added; `screen_balance`, 0 to 100, 50; `clip_black` and `clip_white`, 0 to 100,
0 and 100; `despill_bias`, 0 to 100, 50; `despill`, 0 to 100, 100; `shrink_grow`, -100 to 100
pixels, 0; `softness`, 0 to 100 pixels, 0; `edge_refine`, 0 to 100 pixels, 0; `view`,
"final" (when added), "screen_matte" or "status"; every word exact. Every number keyable.

The rule. k is the screen colour's 8-bit values over 255. Its screen channel s is the largest
of k's three (on a tie green, then blue, then red); o1 and o2 are the other two in red, green,
blue order. For a colour c, weigh(c, t) = max(c_o1, c_o2) (1 - t) + min(c_o1, c_o2) t, and
D(c) = c_s - weigh(c, balance / 100). At a pixel with covering a > 0, e is its straight colour
through the sRGB curve held inside 0 to 1:

1. The screen matte, 1 foreground and 0 screen: m = 1 - (gain / 100) D(e) / D(k) held inside 0
   to 1; m = 1 everywhere when D(k) <= 0 (a grey screen colour keys nothing). At a = 0, m = 1.
2. The clip levels, cb = clip_black / 100 and cw = clip_white / 100: m = (m - cb) / (cw - cb)
   held inside 0 to 1; where cw <= cb, 0 at or below cb and 1 above it.
3. Shrink or grow: with shrink_grow g != 0, each m becomes the least (g < 0) or the greatest
   (g > 0) m over Simple Choker's disc, the whole steps (dx, dy) with dx^2 + dy^2 <= g^2, the
   picture's edge pixels held beyond it (so the frame's own edge is never eaten).
4. Softness: with softness sigma > 0, m through document 21's Gaussian at sigma (reach
   ceil(3 sigma), weights normalised after truncation), across then down, edges held.
5. Despill: L = weigh(e, despill_bias / 100); e_s less (despill / 100) max(0, e_s - L).
6. "final": the despilled colour back to linear, at covering a m. Then, with edge_refine
   r >= 1, Key Cleaner's rule at radius floor(r), alpha contrast 0, strength 100 (Refine Soft
   Matte's guided fit on the edge band) on the whole result.
   "screen_matte": m after step 4 as an encoded grey, back to linear at the pixel's covering a.
   "status": white where m >= 0.999, black where m <= 0.001, else the encoded grey 0.5, back
   to linear at covering a (Keylight's guide: three colours, one grey for every blend).
   A pixel with a = 0 is left transparent in every view, except where the edge refine's fit
   reaches it.

**This file never runs the build's code path.** It works in double precision on lists (and
numpy for the guided fit, as tools/key_cleaner_reference.py), straight from the drawing's 8-bit
values, where the build works on its single-precision buffers. The rule is continuous in the
colour except at a clip with cw <= cb and at status's two thresholds; every such case is
checked that no pixel lies within 0.02 of a step (a matte of exactly 0 or 1 aside).

The drawing is Color Difference Key's (tools/color_difference_key_reference.py): black, white,
grey, a green screen (40, 180, 60), a blue screen (30, 70, 200), skin, pure blue, cyan, pure
green, yellow, red, magenta, a violet, a pale blue, and the blue screen at half covering, each
row darker by 8%. The projects go into `Fixtures/screen_key`, the expected frames into
`Fixtures/screen_key/expected_screen_key.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/screen_key_reference.py
"""

import math
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import color_difference_key_reference as D  # noqa: E402  (draws the shared picture)
import key_cleaner_reference as KC  # noqa: E402
from outline_reference import steps  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "screen_key"
NUMBERS = {"screen_gain": (0, 200), "screen_balance": (0, 100), "clip_black": (0, 100),
           "clip_white": (0, 100), "despill_bias": (0, 100), "despill": (0, 100),
           "shrink_grow": (-100, 100), "softness": (0, 100), "edge_refine": (0, 100)}
STARTS = {"screen_gain": 100, "screen_balance": 50, "clip_black": 0, "clip_white": 100,
          "despill_bias": 50, "despill": 100, "shrink_grow": 0, "softness": 0, "edge_refine": 0}
GREEN_SCREEN = "#28b43c"  # (40, 180, 60), the drawing's green screen
BLUE_SCREEN = "#1e46c8"   # (30, 70, 200), the drawing's blue screen
STEPS = []  # (value, threshold) at every hard step taken, to check the margin


# --- the rule -------------------------------------------------------------------------------

def channels(k):
    s = max((1, 2, 0), key=lambda i: (k[i], -(1, 2, 0).index(i)))
    o1, o2 = (i for i in range(3) if i != s)
    return s, o1, o2


def weigh(c, o1, o2, t):
    return max(c[o1], c[o2]) * (1 - t) + min(c[o1], c[o2]) * t


def matte(px, k):
    s, o1, o2 = channels(k["screen"])
    diff = lambda c: c[s] - weigh(c, o1, o2, k["screen_balance"] / 100)  # noqa: E731
    ds = diff(k["screen"])
    cb, cw = k["clip_black"] / 100, k["clip_white"] / 100
    out = []
    for p in px:
        if p[3] == 0 or ds <= 0:
            m = 1.0
        else:
            m = min(1.0, max(0.0, 1 - k["screen_gain"] / 100 * diff(B.encoded(p)) / ds))
        if cw > cb:
            m = min(1.0, max(0.0, (m - cb) / (cw - cb)))
        else:
            STEPS.append((m, cb))
            m = 0.0 if m <= cb else 1.0
        out.append(m)
    return out


def held(m, x, y):
    return m[min(H - 1, max(0, y)) * W + min(W - 1, max(0, x))]


def choked(m, g):
    if g == 0:
        return m
    near = steps(abs(g))
    pick = max if g > 0 else min
    return [pick(held(m, x + dx, y + dy) for dx, dy in near) for y in range(H) for x in range(W)]


def softened(m, sigma):
    if sigma <= 0:
        return m
    r = math.ceil(3 * sigma)
    taps = [math.exp(-(d * d) / (2 * sigma * sigma)) for d in range(-r, r + 1)]
    taps = [t / sum(taps) for t in taps]
    across = [sum(t * held(m, x + i - r, y) for i, t in enumerate(taps))
              for y in range(H) for x in range(W)]
    return [sum(t * held(across, x, y + i - r) for i, t in enumerate(taps))
            for y in range(H) for x in range(W)]


def despilled(e, k):
    s, o1, o2 = channels(k["screen"])
    limit = weigh(e, o1, o2, k["despill_bias"] / 100)
    e = list(e)
    e[s] -= k["despill"] / 100 * max(0.0, e[s] - limit)
    return e


def key(px, k, n):
    m = softened(choked(matte(px, k), k["shrink_grow"]), k["softness"])
    out = []
    for p, v in zip(px, m):
        a = p[3] / 255
        if p[3] == 0:
            out.append([0.0] * 4)
        elif k["view"] == "final":
            out.append([S.srgb_to_linear(c) * a * v for c in despilled(B.encoded(p), k)]
                       + [a * v])
        else:
            if k["view"] == "status":
                if 0 < v < 1:  # a whole 0 or 1 is exact, however it is worked
                    STEPS.extend([(v, 0.999), (v, 0.001)])
                v = 1.0 if v >= 0.999 else 0.0 if v <= 0.001 else 0.5
            g = S.srgb_to_linear(v) * a
            out.append([g, g, g, a])
    if k["view"] == "final" and math.floor(k["edge_refine"]) >= 1:
        P = np.array(out, dtype=float).reshape(H, W, 4)
        P = KC.key_cleaner(P, KC.cleaner(k["edge_refine"], 0, 100), n)
        out = [list(map(float, q)) for q in P.reshape(-1, 4)]
    return out


# --- the cases ------------------------------------------------------------------------------

def case(view="final", screen_color="#00ff00", shift=0, **numbers):
    c = {"view": view, "screen_color": screen_color, "shift": shift, **STARTS}
    for name, v in numbers.items():
        assert name in NUMBERS, name
        c[name] = v
    return c


def settings(c, frame_no):
    k = {"view": c["view"], "screen": D.hex_colour(c["screen_color"])}
    for name, (lo, hi) in NUMBERS.items():
        k[name] = min(hi, max(lo, value_at(c[name], frame_no)))
    return k


def render(c, frame_no):
    return R.frame(key(B.pixels(), settings(c, frame_no), frame_no), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {"view": c["view"], "screen_color": c["screen_color"]}
    params.update({name: setting_json(c[name]) for name in NUMBERS})
    return B.project_json(fx, "core.screen_key", params, c["shift"])


G = GREEN_SCREEN
CASES = {
    "FX-SKEY-001": ("The settings as they start: screen colour pure green, gain 100, balance "
                    "50, no clip, despill bias 50 at full strength. Pure green's top row is keyed "
                    "out, its darker rows keep a share in step with their darkness; the green "
                    "screen, less saturated than pure green, keeps about half its covering and "
                    "loses its green spill; skin, the greys and the reds keep all of theirs.", case(), [0]),
    "FX-SKEY-002": ("Screen colour picked from the green screen (40, 180, 60): its top row is "
                    "keyed out, its darker rows keep a little more each (their green stands "
                    "less above the rest); pure green, more saturated than the screen, is gone "
                    "in its top five rows; skin, greys, reds and blues are untouched apart from the despill.",
                    case(screen_color=G), [0]),
    "FX-SKEY-003": ("Screen gain 150: more is keyed; the green screen's darker rows fall further "
                    "and cyan and yellow, which stand half as far above, lose more.",
                    case(screen_color=G, screen_gain=150), [0]),
    "FX-SKEY-004": ("Screen balance 0: the screen channel is measured against the larger of "
                    "the other two, so yellow and cyan no longer read as screen.",
                    case(screen_color=G, screen_balance=0), [0]),
    "FX-SKEY-005": ("Screen balance 100: against the smaller of the other two, so yellow and "
                    "cyan read as screen as strongly as green.",
                    case(screen_color=G, screen_balance=100), [0]),
    "FX-SKEY-006": ("Clip black 30, clip white 70: the matte below 0.3 is screen, above 0.7 "
                    "foreground, stretched between; the green screen's darker rows clean up.",
                    case(screen_color=G, clip_black=30, clip_white=70), [0]),
    "FX-SKEY-007": ("Clip black 53 and clip white 53: a hard matte, each pixel in or out.",
                    case(screen_color=G, clip_black=53, clip_white=53), [0]),
    "FX-SKEY-008": ("Despill bias 0: the green is held to the larger of red and blue, the "
                    "gentlest despill.", case(screen_color=G, despill_bias=0), [0]),
    "FX-SKEY-009": ("Despill bias 100: the green is held to the smaller of red and blue, the "
                    "strongest despill.", case(screen_color=G, despill_bias=100), [0]),
    "FX-SKEY-010": ("Despill 0: the matte as FX-SKEY-002, the colours not despilled.",
                    case(screen_color=G, despill=0), [0]),
    "FX-SKEY-011": ("Blue screen (30, 70, 200) at balance 95, the advice for blue screens: the "
                    "blue screen and its half-covered copy are keyed, pure blue is gone, and so are "
                    "cyan, magenta and violet, whose blue stands as far above; pale blue keeps a "
                    "share; green, yellow and red keep all their covering.", case(screen_color=BLUE_SCREEN, screen_balance=95),
                    [0]),
    "FX-SKEY-012": ("View screen matte, FX-SKEY-006's settings: the matte as grey, covering "
                    "kept.", case("screen_matte", G, clip_black=30, clip_white=70), [0]),
    "FX-SKEY-013": ("View status, FX-SKEY-006's settings: white where all foreground, black "
                    "where all screen, one grey for every blend.",
                    case("status", G, clip_black=30, clip_white=70), [0]),
    "FX-SKEY-014": ("Shrink/grow -1: the foreground matte shrinks a pixel, so the columns "
                    "beside the keyed green screen and pure green lose their covering too; "
                    "the frame's own edge is not eaten.",
                    case(screen_color=G, clip_black=30, clip_white=70, shrink_grow=-1), [0]),
    "FX-SKEY-015": ("Shrink/grow 1.5: the foreground matte grows a pixel and a half, filling "
                    "the keyed green screen from its neighbours; pure green, between cyan and "
                    "yellow that are keyed too at the top, stays keyed there.",
                    case(screen_color=G, clip_black=30, clip_white=70, shrink_grow=1.5), [0]),
    "FX-SKEY-016": ("Softness 1: the matte blurred at sigma 1, the edge between the screen and "
                    "the foreground soft.",
                    case(screen_color=G, clip_black=30, clip_white=70, softness=1), [0]),
    "FX-SKEY-017": ("Edge refine 2: Refine Soft Matte's guided fit at radius 2 on the keyed "
                    "result's edges.", case(screen_color=G, clip_black=30, clip_white=70,
                                            edge_refine=2), [0]),
    "FX-SKEY-018": ("Screen gain keyed from 0 at frame 0 to 200 at frame 4, linear: nothing "
                    "keyed at frame 0, twice FX-SKEY-002's at frame 4.",
                    case(screen_color=G, screen_gain=keyed((0, 0), (4, 200))), [0, 2, 4]),
    "FX-SKEY-019": ("FX-SKEY-014 moved three pixels right: the same, moved.",
                    case(screen_color=G, clip_black=30, clip_white=70, shrink_grow=-1, shift=3),
                    [0, 3]),
}

INVALID = {
    "FX-SKEY-020": ("Screen gain 201, above 200.", case(screen_gain=201)),
    "FX-SKEY-021": ("Clip black -1, below 0.", case(clip_black=-1)),
    "FX-SKEY-022": ("Shrink/grow 101, above 100.", case(shrink_grow=101)),
    "FX-SKEY-023": ("View \"Status\": the word is exact, so a capital is not it.",
                    case("Status")),
    "FX-SKEY-024": ("Screen colour \"green\", not a #rrggbb colour.", case(screen_color="green")),
}


def main():
    expected = B.write_cases(OUT, "screen_key", CASES, INVALID, render, plain, file_json)
    for v, t in STEPS:
        assert abs(v - t) > 0.02, (v, t)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    cover = lambda f, x, y: f[at(x, y)][3]  # noqa: E731
    full = lambda f, x, y: abs(cover(f, x, y) - px[at(x, y)][3] / 255) < 1e-9  # noqa: E731
    gone = lambda f, x, y: cover(f, x, y) < 1e-9  # noqa: E731
    same = lambda f, x, y: near(f[at(x, y)], drawn[at(x, y)])  # noqa: E731
    col = lambda f, x: [cover(f, x, y) for y in range(H)]  # noqa: E731

    one = c["FX-SKEY-001"]["0"]
    assert gone(one, 9, 0) and col(one, 9) == sorted(col(one, 9)) and 0.3 < cover(one, 9, 4) < 0.35
    assert 0.4 < cover(one, 4, 0) < 0.6
    for x in (1, 2, 3, 6, 11):
        assert all(same(one, x, y) for y in range(H)), x
    two = c["FX-SKEY-002"]["0"]
    assert gone(two, 4, 0) and all(gone(two, 9, y) for y in range(5))
    assert col(two, 4) == sorted(col(two, 4)) and 0.6 < cover(two, 4, 9) < 0.8
    for x in (1, 2, 3, 6, 11, 12):
        assert all(same(two, x, y) for y in range(H)), x
    # The despill: a kept green screen pixel's green is no higher than red and blue's mean.
    for y in range(1, H):
        p = two[at(4, y)]
        assert p[1] <= (S.srgb_to_linear(S.linear_to_srgb(p[0] / p[3]) * 0.5 +
                                         S.linear_to_srgb(p[2] / p[3]) * 0.5)) * p[3] + 1e-9
    three = c["FX-SKEY-003"]["0"]
    assert all(cover(three, 4, y) <= cover(two, 4, y) for y in range(H))
    assert cover(three, 8, 0) < cover(two, 8, 0) and cover(three, 10, 0) < cover(two, 10, 0)
    four, five = c["FX-SKEY-004"]["0"], c["FX-SKEY-005"]["0"]
    assert full(four, 8, 0) and full(four, 10, 0)
    assert gone(five, 8, 0) and gone(five, 10, 0)
    six = c["FX-SKEY-006"]["0"]
    assert all(cover(six, 4, y) <= cover(two, 4, y) + 1e-12 for y in range(4))
    seven = c["FX-SKEY-007"]["0"]
    for x in range(1, W):
        for y in range(H):
            assert gone(seven, x, y) or full(seven, x, y), (x, y)
    eight, nine, ten = (c[f"FX-SKEY-0{i:02d}"]["0"] for i in (8, 9, 10))
    for y in range(1, H):
        g8, g9, g10 = (f[at(4, y)][1] / f[at(4, y)][3] for f in (eight, nine, ten))
        assert g9 < g8 < g10, y
        assert abs(cover(ten, 4, y) - cover(two, 4, y)) < 1e-12
    eleven = c["FX-SKEY-011"]["0"]
    assert gone(eleven, 5, 0) and gone(eleven, 15, 0) and gone(eleven, 7, 0)
    assert gone(eleven, 13, 0) and 0.3 < cover(eleven, 14, 0) < 0.7
    assert all(full(eleven, x, y) for x in (9, 10, 11) for y in range(H))
    assert all(same(eleven, x, y) for x in (1, 2, 3, 6) for y in range(H))
    twelve, thirteen = c["FX-SKEY-012"]["0"], c["FX-SKEY-013"]["0"]
    for i, (p, q) in enumerate(zip(twelve, six)):
        a = px[i][3] / 255
        assert abs(p[3] - a) < 1e-12 and p[0] == p[1] == p[2], i
        if a:
            assert abs(p[0] - S.srgb_to_linear(q[3] / a) * a) < 1e-12, i
    greys = {round(p[0] / p[3], 12) for p in thirteen if p[3]}
    assert greys <= {0.0, 1.0, round(S.srgb_to_linear(0.5), 12)} and len(greys) == 3, greys
    fourteen = c["FX-SKEY-014"]["0"]
    assert all(cover(fourteen, x, y) <= cover(six, x, y) + 1e-12 for x in range(W)
               for y in range(H))
    assert gone(fourteen, 3, 0) and gone(fourteen, 10, 0) and full(fourteen, 1, 0)
    assert full(fourteen, 1, 9) and full(fourteen, 14, 9)  # the frame's edge kept
    fifteen = c["FX-SKEY-015"]["0"]
    assert all(cover(fifteen, x, y) >= cover(six, x, y) - 1e-12 for x in range(W)
               for y in range(H))
    assert full(fifteen, 4, 0) and full(fifteen, 8, 0) and gone(fifteen, 9, 0)
    sixteen = c["FX-SKEY-016"]["0"]
    assert 0.01 < cover(sixteen, 3, 0) < 0.99 and 0.01 < cover(sixteen, 5, 0) < 0.99
    seventeen = c["FX-SKEY-017"]["0"]
    assert not like(seventeen, six)
    eighteen = c["FX-SKEY-018"]
    assert all(full(eighteen["0"], x, y) for x in range(1, W) for y in range(H))
    assert like(eighteen["2"], render(case(screen_color=G), 0))
    assert like(eighteen["4"], render(case(screen_color=G, screen_gain=200), 0))
    moved = c["FX-SKEY-019"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == fourteen[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        for f in frames.values():
            for p in f:
                assert all(-1e-9 <= v <= p[3] + 1e-9 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
