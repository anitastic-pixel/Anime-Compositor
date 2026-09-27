"""Black & White, worked a second way.

D-136 adds `core.black_white`. It turns a drawing grey, as black-and-white film does, but lets
each of six colours decide how light its own grey comes out: `reds`, `yellows`, `greens`,
`cyans`, `blues` and `magentas`, each -200 to 300, starting at 40, 60, 40, 60, 20 and 80. A pure
colour becomes a grey as light as its own number over 100 on the written scale (pure red at 40
is a grey 40 per cent of the way up), a colour between two of them takes a share of each, and a
grey, black or white stays exactly as it is, whatever the numbers. Raising a number lightens
that colour's grey, lowering it darkens it, as a coloured filter does in front of a black and
white camera. The covering is kept, so a soft edge stays soft, and a pixel that does not show
stays as it is. It is this program's own method, modelled on After Effects' Black & White;
nothing is ported. Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it.

The rule. At a pixel with covering a > 0, with e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) its
encoded straight colour, order the three channels from largest to smallest, ties in the order
red, green, blue (a stable sort, descending): mx, md, mn. The primary weight is the setting of
mx's channel (reds, greens or blues); the secondary weight is the setting of the pair of mx's
and md's channels (red and green: yellows; green and blue: cyans; red and blue: magentas).
g = mn + (md - mn) secondary / 100 + (mx - md) primary / 100, and every channel of the output is
srgb_to_linear(clamp(g, 0, 1)) * a, a unchanged. A tie multiplies its weight by 0, so the choice
at a tie never changes g: the rule has no cliff. The layer does not grow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: swatches of colour, drawn below. The drawing goes into
`Fixtures/black_white/media`, the projects into `Fixtures/black_white`, and the expected frames
into `Fixtures/black_white/expected_black_white.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/black_white_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "black_white"
TOLERANCE = 2e-5  # document 25's default for a filter
NAMES = ("reds", "yellows", "greens", "cyans", "blues", "magentas")
START = dict(zip(NAMES, (40, 60, 40, 60, 20, 80)))
RANGE = (-200, 300)
PRIMARY = ("reds", "greens", "blues")
PAIR = {frozenset((0, 1)): "yellows", frozenset((1, 2)): "cyans", frozenset((0, 2)): "magentas"}


# --- the rule -------------------------------------------------------------------------------

def order(e):
    """The channels from largest to smallest, ties in the order red, green, blue."""
    return sorted(range(3), key=lambda c: -e[c])


def grey_of(e, s):
    """One encoded straight colour's grey, unclamped, with s the six settings by name."""
    i, j, k = order(e)
    return (e[k] + (e[j] - e[k]) * s[PAIR[frozenset((i, j))]] / 100
            + (e[i] - e[j]) * s[PRIMARY[i]] / 100)


def black_white(pixels, s):
    out = [R.working(p) for p in pixels]
    for w in out:
        a = w[3]
        if a > 0:
            e = [S.linear_to_srgb(min(1.0, max(0.0, v / a))) for v in w[:3]]
            w[:3] = [srgb_to_linear(min(1.0, max(0.0, grey_of(e, s)))) * a] * 3
    return out


# --- the drawing ----------------------------------------------------------------------------

RED, YELLOW, GREEN = (255, 0, 0, 255), (255, 255, 0, 255), (0, 255, 0, 255)
CYAN, BLUE, MAGENTA = (0, 255, 255, 255), (0, 0, 255, 255), (255, 0, 255, 255)
GREY, WHITE, BLACK = (128, 128, 128, 255), (255, 255, 255, 255), (0, 0, 0, 255)
SOFT_RED = (255, 0, 0, 128)      # the red at half covering, a soft edge
SKIN = R.SKIN                    # #f6d6be: red, then green, then blue
PINK = (255, 64, 128, 255)       # #ff4080: red, then blue, then green
LEAF = (128, 192, 64, 255)       # #80c040: green, then red, then blue
TEAL = (32, 192, 128, 255)       # #20c080: green, then blue, then red
LINE = R.LINE                    # #1e1a24: blue, then red, then green
SKY = (64, 128, 192, 255)        # #4080c0: blue, then green, then red
TRACE = (200, 40, 40, 255)       # #c82828: red, green and blue tied below it
SHADE = (220, 160, 140, 255)     # #dca08c, the skin's shadow
SOFT_SKIN = (246, 214, 190, 128)  # the skin at half covering
NONE = S.NONE
TOP = [NONE, RED, YELLOW, GREEN, CYAN, BLUE, MAGENTA, GREY, WHITE, BLACK, SOFT_RED] + [NONE] * 5
BOTTOM = [NONE, SKIN, PINK, LEAF, TEAL, LINE, SKY, TRACE, SHADE, SOFT_SKIN] + [NONE] * 6


def swatches(x, y):
    """Rows 0 and 9 are empty, and column 0. Rows 1 to 4: the six pure colours in columns 1 to
    6 (red, yellow, green, cyan, blue, magenta), grey, white and black in 7 to 9, and the red at
    half covering in 10. Rows 5 to 8: a colour for each order of the channels in columns 1 to 6
    (skin, pink, leaf, teal, the line, sky), the trace red in 7, the skin's shadow in 8 and the
    skin at half covering in 9. Everything to their right is empty."""
    return NONE if y in (0, 9) else (TOP if y < 5 else BOTTOM)[x]


DRAWINGS = {"swatches": [[swatches(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, **settings):
    c = {"drawing": "swatches", "shift": shift, **START}
    c.update(settings)
    return c


def held(v):
    return min(RANGE[1], max(RANGE[0], v))


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    s = {k: held(value_at(c[k], frame_no)) for k in NAMES}
    return R.frame(black_white(pixels, s), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return R.frame([R.working(p) for row in DRAWINGS[c["drawing"]] for p in row], c["shift"])


ALL = lambda v: dict(zip(NAMES, [v] * 6))  # noqa: E731
RED_FILTER = dict(zip(NAMES, (120, 110, -10, -50, -50, 120)))

CASES = {
    "FX-BW-001": ("The settings as they start, 40, 60, 40, 60, 20, 80: every pixel that shows "
                  "turns grey. Each pure colour becomes the grey of its own number: red and "
                  "green #666666 (102), yellow and cyan #999999 (153), blue #333333 (51), "
                  "magenta #cccccc (204). Grey, white and black stay exactly as they are. The "
                  "skin becomes 217.2 on the 8-bit scale, the pink 166, the leaf 128, the teal "
                  "and the sky 115.2, the line 30.4, the trace red 104 and the shadow 176; the "
                  "half-covered red and skin are the red's and the skin's greys at half "
                  "covering.",
                  case(), [0]),
    "FX-BW-002": ("All six at 100: each pixel becomes its largest channel, so every pure colour "
                  "turns white and the skin, the pink and the trace red take their red.",
                  case(**ALL(100)), [0]),
    "FX-BW-003": ("All six at 0: each pixel becomes its smallest channel, so every pure colour "
                  "turns black, the skin takes its blue and the sky its red.",
                  case(**ALL(0)), [0]),
    "FX-BW-004": ("Reds 300, the most: red, and every colour led by red (the skin, the pink, "
                  "the trace red, the shadow), is lifted to white, held there; yellow and "
                  "magenta, where red only ties for the lead, and every colour led by green or "
                  "blue, are as in FX-BW-001.",
                  case(reds=300), [0]),
    "FX-BW-005": ("Reds -200, the least: red, the trace red and the pink fall below 0 and are "
                  "held at black; the skin and the shadow darken; the rest as in FX-BW-001.",
                  case(reds=-200), [0]),
    "FX-BW-006": ("Yellows 300: yellow turns white, and the colours whose two largest channels "
                  "are red and green (the skin, the leaf, the shadow) lighten; red and green, "
                  "with nothing between their largest and smallest, stay as in FX-BW-001.",
                  case(yellows=300), [0]),
    "FX-BW-007": ("Greens -200: green turns black and the leaf and the teal, led by green, "
                  "darken; the rest as in FX-BW-001.",
                  case(greens=-200), [0]),
    "FX-BW-008": ("Cyans 0: cyan turns black, its smallest channel, and the teal and the sky, "
                  "whose two largest are green and blue, darken; the rest as in FX-BW-001.",
                  case(cyans=0), [0]),
    "FX-BW-009": ("Blues 300: blue turns white, and the line and the sky, led by blue, lighten; "
                  "the rest as in FX-BW-001.",
                  case(blues=300), [0]),
    "FX-BW-010": ("Magentas -200: magenta turns black, and the pink and the line, whose two "
                  "largest are red and blue, darken; the rest as in FX-BW-001.",
                  case(magentas=-200), [0]),
    "FX-BW-011": ("A red filter, 120, 110, -10, -50, -50, 120, as a red glass in front of a "
                  "black and white camera: the warm colours (red, yellow, magenta, the skin) "
                  "come out lighter than at the start and the cool ones (cyan, blue, the sky) "
                  "darker; blue and cyan fall below 0 and are held at black.",
                  case(**RED_FILTER), [0]),
    "FX-BW-012": ("Reds keyed from 40 at frame 0 to 300 at frame 4, linear: frame 0 is "
                  "FX-BW-001, frame 2 reds 170, frame 4 FX-BW-004.",
                  case(reds=keyed((0, 40), (4, 300))), [0, 2, 4]),
    "FX-BW-013": ("Blues keyed from -200 at frame 0 to 300 at frame 4: frame 0 blue is black, "
                  "frame 2 blues 50, a grey #808080 but for rounding (127.5), frame 4 blue is "
                  "white.",
                  case(blues=keyed((0, -200), (4, 300))), [0, 2, 4]),
    "FX-BW-014": ("Magentas eased from 80 at frame 0 to 300 at frame 4 on a curve that "
                  "overshoots: at frame 2 the number has gone past 300 and is held there, so "
                  "frames 2 and 4 are both magentas 300, magenta white.",
                  case(magentas=keyed((0, 80, OVERSHOOT), (4, 300))), [0, 2, 4]),
    "FX-BW-015": ("Greens eased from 40 at frame 0 to -200 at frame 4 on the same curve: at "
                  "frame 2 the number has gone past -200 and is held there, so frames 2 and 4 "
                  "are both FX-BW-007.",
                  case(greens=keyed((0, 40, OVERSHOOT), (4, -200))), [0, 2, 4]),
    "FX-BW-016": ("FX-BW-011 moved three pixels right: the same, moved.",
                  case(shift=3, **RED_FILTER), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-BW-017": ("Reds 301: above 300.", case(reds=301)),
    "FX-BW-018": ("Yellows -201: below -200.", case(yellows=-201)),
    "FX-BW-019": ("Greens 350: above 300.", case(greens=350)),
    "FX-BW-020": ("Cyans -200.5: below -200.", case(cyans=-200.5)),
    "FX-BW-021": ("Blues 1000: above 300.", case(blues=1000)),
    "FX-BW-022": ("Magentas keyed to -250 at frame 4.",
                  case(magentas=keyed((0, 80), (4, -250)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.black_white", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_black_white.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    grey8 = lambda v: [srgb_to_linear(min(1.0, max(0.0, v / 255)))] * 3 + [1.0]  # noqa: E731
    enc = lambda q: [v / 255 for v in q[:3]]  # noqa: E731
    top = {n: (x, 2) for n, x in zip(("red", "yellow", "green", "cyan", "blue", "magenta",
                                      "grey", "white", "black", "soft_red"), range(1, 11))}
    bot = {n: (x, 6) for n, x in zip(("skin", "pink", "leaf", "teal", "line", "sky", "trace",
                                      "shade", "soft_skin"), range(1, 10))}
    xy = {**top, **bot}
    colour = {n: (TOP if y < 5 else BOTTOM)[x] for n, (x, y) in xy.items()}
    red_led = ("red", "skin", "pink", "trace", "shade", "soft_red", "soft_skin")

    # The rule's own pieces: the order puts ties in the order red, green, blue; the pure
    # colours give their own numbers over 100; greys give themselves whatever the numbers; all
    # at 100 gives the largest channel and all at 0 the smallest; and a tie never changes g, so
    # a colour a hair either side of a tie gives nearly the same grey (no cliff).
    assert order((1, 1, 0)) == [0, 1, 2] and order((1, 0, 1)) == [0, 2, 1]
    assert order((0, 1, 1)) == [1, 2, 0] and order((0.5, 0.5, 0.5)) == [0, 1, 2]
    for n, k in (("red", "reds"), ("yellow", "yellows"), ("green", "greens"),
                 ("cyan", "cyans"), ("blue", "blues"), ("magenta", "magentas")):
        assert abs(grey_of(enc(colour[n]), START) - START[k] / 100) < 1e-15
    wild = dict(zip(NAMES, (300, -200, 170, -35, 222, -150)))
    for v in (0, 0.3, 128 / 255, 1):
        assert grey_of((v, v, v), wild) == v
    for q in TOP[1:11] + BOTTOM[1:10]:
        e = enc(q)
        assert abs(grey_of(e, ALL(100)) - max(e)) < 1e-15
        assert abs(grey_of(e, ALL(0)) - min(e)) < 1e-15
    for e in ((1, 1, 0), (1, 0, 1), (0, 1, 1), (0.8, 0.2, 0.2), (0.3, 0.3, 0.3), (0.2, 0.6, 0.6)):
        for d in ((1e-7, 0, 0), (0, 1e-7, 0), (0, 0, 1e-7)):
            f = [u + v for u, v in zip(e, d)]
            assert abs(grey_of(f, wild) - grey_of(e, wild)) < 1e-6, (e, d)
    worked = {"skin": 217.2, "pink": 166, "leaf": 128, "teal": 115.2, "sky": 115.2,
              "line": 30.4, "trace": 104, "shade": 176}
    for n, v in worked.items():
        assert abs(255 * grey_of(enc(colour[n]), START) - v) < 1e-9, n

    # Every case keeps every pixel's covering, leaves the empty pixels empty, turns every
    # pixel that shows grey, leaves grey, white and black exactly as drawn, and makes each
    # half-covered pixel the full one's grey at half covering.
    for fx, frames in c.items():
        shift = 3 if fx == "FX-BW-016" else 0
        base = plain(case(shift=shift))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                elif "warning" not in expected["cases"][fx]:
                    assert px[i][0] == px[i][1] == px[i][2], (fx, i)
            for n in ("grey", "white", "black"):
                x, y = xy[n]
                assert near(px[at(x + shift, y)], drawn[at(x, y)]), (fx, n)
            if "warning" not in expected["cases"][fx]:
                for s, f in (("soft_red", "red"), ("soft_skin", "skin")):
                    (sx, sy), (fx_, fy) = xy[s], xy[f]
                    assert near([v / (128 / 255) for v in px[at(sx + shift, sy)][:3]],
                                px[at(fx_ + shift, fy)][:3]), (fx, s)

    g = lambda f, n: f[at(*xy[n])]  # noqa: E731
    one = c["FX-BW-001"]["0"]
    for n, v in (("red", 102), ("green", 102), ("yellow", 153), ("cyan", 153), ("blue", 51),
                 ("magenta", 204), *worked.items()):
        assert near(g(one, n), grey8(v)), n
    two = c["FX-BW-002"]["0"]
    for n in ("red", "yellow", "green", "cyan", "blue", "magenta"):
        assert g(two, n) == grey8(255)
    for n in ("skin", "pink", "trace"):
        assert near(g(two, n), grey8(colour[n][0])), n
    three = c["FX-BW-003"]["0"]
    for n in ("red", "yellow", "green", "cyan", "blue", "magenta"):
        assert g(three, n) == [0.0, 0.0, 0.0, 1.0]
    assert near(g(three, "skin"), grey8(190)) and near(g(three, "sky"), grey8(64))
    others = lambda f, lit: [n for n in xy if n not in lit and  # noqa: E731
                             not near(g(f, n), g(one, n))]
    four = c["FX-BW-004"]["0"]
    assert all(g(four, n)[:3] == [g(four, n)[3]] * 3 for n in red_led)  # white at its covering
    assert others(four, red_led) == []
    five = c["FX-BW-005"]["0"]
    assert g(five, "red") == g(five, "trace") == g(five, "pink") == [0.0, 0.0, 0.0, 1.0]
    assert all(0 < g(five, n)[0] < g(one, n)[0] for n in ("skin", "shade"))
    assert others(five, red_led) == []
    six = c["FX-BW-006"]["0"]
    assert g(six, "yellow") == grey8(255)
    assert all(g(six, n)[0] > g(one, n)[0] for n in ("skin", "leaf", "shade"))
    assert others(six, ("yellow", "skin", "leaf", "shade", "soft_skin")) == []
    seven = c["FX-BW-007"]["0"]
    assert g(seven, "green") == [0.0, 0.0, 0.0, 1.0]
    assert all(g(seven, n)[0] < g(one, n)[0] for n in ("leaf", "teal"))
    assert others(seven, ("green", "leaf", "teal")) == []
    eight = c["FX-BW-008"]["0"]
    assert g(eight, "cyan") == [0.0, 0.0, 0.0, 1.0]
    assert all(g(eight, n)[0] < g(one, n)[0] for n in ("teal", "sky"))
    assert others(eight, ("cyan", "teal", "sky")) == []
    nine = c["FX-BW-009"]["0"]
    assert g(nine, "blue") == grey8(255)
    assert all(g(nine, n)[0] > g(one, n)[0] for n in ("line", "sky"))
    assert others(nine, ("blue", "line", "sky")) == []
    ten = c["FX-BW-010"]["0"]
    assert g(ten, "magenta") == [0.0, 0.0, 0.0, 1.0]
    assert all(g(ten, n)[0] < g(one, n)[0] for n in ("pink", "line"))
    assert others(ten, ("magenta", "pink", "line")) == []
    eleven = c["FX-BW-011"]["0"]
    assert all(g(eleven, n)[0] > g(one, n)[0] for n in ("red", "yellow", "magenta", "skin"))
    assert all(g(eleven, n)[0] < g(one, n)[0] for n in ("cyan", "blue", "sky"))
    assert g(eleven, "blue") == g(eleven, "cyan") == [0.0, 0.0, 0.0, 1.0]
    assert grey_of(enc(CYAN), RED_FILTER) < 0 and grey_of(enc(BLUE), RED_FILTER) < 0
    twelve = c["FX-BW-012"]
    assert twelve["0"] == one and like(twelve["4"], four)
    assert like(twelve["2"], render(case(reds=170), 0))
    thirteen = c["FX-BW-013"]
    assert like(thirteen["2"], render(case(blues=50), 0))
    assert g(thirteen["0"], "blue") == [0.0, 0.0, 0.0, 1.0] and g(thirteen["4"], "blue") == \
        grey8(255)
    assert near(g(thirteen["2"], "blue"), grey8(127.5))
    fourteen = c["FX-BW-014"]
    assert ease(OVERSHOOT, 0.5) > 1
    assert value_at(case(magentas=keyed((0, 80, OVERSHOOT), (4, 300)))["magentas"], 2) > 300
    assert fourteen["0"] == one and fourteen["2"] == fourteen["4"]
    assert like(fourteen["4"], render(case(magentas=300), 0))
    assert g(fourteen["4"], "magenta") == grey8(255)
    fifteen = c["FX-BW-015"]
    assert value_at(case(greens=keyed((0, 40, OVERSHOOT), (4, -200)))["greens"], 2) < -200
    assert fifteen["0"] == one and fifteen["2"] == seven and fifteen["4"] == seven
    moved = c["FX-BW-016"]["0"]
    assert moved == c["FX-BW-016"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == eleven[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
