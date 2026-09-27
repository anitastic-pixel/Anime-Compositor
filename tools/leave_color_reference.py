"""Leave Color, worked a second way.

D-141 adds `core.leave_color`. It turns a drawing grey except for one chosen colour, as a
film keeps a single red coat in a black-and-white scene, or an anime cut drains a flashback of
all but a heroine's ribbon. `color` is the colour to keep, `#rrggbb` (#ff0000); `tolerance`,
0 to 100 (15), how far round the colour wheel from it a hue is still kept whole, 100 being the
far side of the wheel; `softness`, 0 to 100 (10), a band past the tolerance over which the
keeping fades out; and `amount`, 0 to 100 (100), how far the other colours are turned grey.
Colours are compared by hue alone, so a light or a dark red is kept as a red is; a grey, having
no hue, is never kept, and a grey `color` keeps nothing. The grey a pixel turns is its own
brightness, so the picture reads as a black-and-white print. Pixels that do not show stay as
they are, and every pixel keeps its own covering. It is this program's own method, modelled on
After Effects' Leave Color; nothing is ported. Document 21 is the rule in words; this file is
the reference for the numbers document 25 pins against it.

The rule. The hue of an encoded colour, in degrees 0..360, is the HSV hue: with mx, mn the
largest and smallest channel, none when mx == mn; else when red is the largest (checked first),
60 ((g - b) / (mx - mn)) taken into [0, 360); when green, 60 ((b - r) / (mx - mn) + 2); when
blue, 60 ((r - g) / (mx - mn) + 4) (colour key's hue, D-97, imported). Amount 0: the output is
the input. At a pixel with covering a > 0, with e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) its
encoded straight colour: dist = min(|h - h_c|, 360 - |h - h_c|) / 180, with h the pixel's hue
and h_c the colour's (its 8-bit values / 255); dist = 1 when either has no hue. t = tolerance /
100 and s = softness / 100; the kept part k = 1 when dist <= t, else 0 when s == 0 or
dist >= t + s, else 1 - (dist - t) / s. d = amount / 100 (1 - k), Y(e) = 0.2126 e_r +
0.7152 e_g + 0.0722 e_b, e'_c = e_c + d (Y(e) - e_c), and the output is
(srgb_to_linear(clamp(e'_c, 0, 1)) * a, a). A pixel with d = 0 is, by the rule, its own colour,
and is given back exactly. At softness 0 the tolerance is a cliff: the check asserts no pixel's
dist lies within 1e-5 of it. The layer does not grow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: swatches round the colour wheel, drawn below. The drawing goes into
`Fixtures/leave_color/media`, the projects into `Fixtures/leave_color`, and the expected frames
into `Fixtures/leave_color/expected_leave_color.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/leave_color_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
from color_key_reference import hue  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "leave_color"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"tolerance": (0, 100), "softness": (0, 100), "amount": (0, 100)}
NAMES = ("color", "tolerance", "softness", "amount")
CLIFF = 1e-5  # the shared definitions' margin round a yes-or-no choice


# --- the rule -------------------------------------------------------------------------------

def luma(e):
    """The encoded luma Y(e) of an encoded colour."""
    return 0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2]


def distance(h, hc):
    """How far round the wheel two hues are, 0 to 1; 1 when either has none."""
    if h is None or hc is None:
        return 1.0
    return min(abs(h - hc), 360 - abs(h - hc)) / 180


def kept(dist, t, s):
    """How much of its colour a pixel keeps, 0 to 1."""
    if dist <= t:
        return 1.0
    if s == 0 or dist >= t + s:
        return 0.0
    return 1 - (dist - t) / s


def leave(e, hc, t, s, amount):
    """One encoded straight colour, drained toward its own grey unless near the hue hc."""
    d = amount / 100 * (1 - kept(distance(hue(e), hc), t, s))
    y = luma(e)
    return d, [v + d * (y - v) for v in e]


def encoded(w):
    return [S.linear_to_srgb(min(1.0, max(0.0, v / w[3]))) for v in w[:3]]


def leave_color(pixels, color, tolerance, softness, amount):
    """`pixels` are 8-bit straight RGBA."""
    out = [R.working(p) for p in pixels]
    if amount == 0:
        return out  # the build exits early
    hc = hue([v / 255 for v in R.hex_color(color.lower())])
    t, s = tolerance / 100, softness / 100
    for w in out:
        a = w[3]
        if a > 0:
            d, e = leave(encoded(w), hc, t, s, amount)
            if d > 0:
                w[:3] = [srgb_to_linear(min(1.0, max(0.0, v))) * a for v in e]
    return out


# --- the drawing ----------------------------------------------------------------------------

LINE = R.LINE                        # #1e1a24, hue 264
RED = R.TRACE                        # #c82828, hue 0
PINK = (255, 0, 64, 255)             # #ff0040, hue 344.94, a red across the wheel's seam
SHADE = (220, 160, 140, 255)         # #dca08c, the skin's shadow, hue 15
SKIN = R.SKIN                        # #f6d6be, hue 25.71
ORANGE = (255, 153, 0, 255)          # #ff9900, hue 36
YELLOW = (255, 255, 0, 255)          # #ffff00, hue 60
GREEN = (64, 192, 64, 255)           # #40c040, hue 120
TEAL = (64, 192, 192, 255)           # #40c0c0, hue 180
BLUE = (64, 96, 255, 255)            # #4060ff, hue 229.95
VIOLET = (144, 64, 192, 255)         # #9040c0, hue 277.5
MAGENTA = (255, 0, 255, 255)         # #ff00ff, hue 300
SOFT_RED = RED[:3] + (128,)          # the red at half covering, a soft edge
SOFT_BLUE = BLUE[:3] + (128,)        # the blue at half covering
NONE = S.NONE
SWATCHES = {3: RED, 4: PINK, 5: SHADE, 6: SKIN, 7: ORANGE, 8: YELLOW, 9: GREEN, 10: TEAL,
            11: BLUE, 12: VIOLET, 13: MAGENTA}


def swatches(x, y):
    """A box of line in columns 2 to 14 and rows 1 to 8 holding, in rows 2 to 7, one column
    each of red, pink, shadow, skin, orange, yellow, green, teal, blue, violet and magenta
    (columns 3 to 13); down its left side, column 1, the red at half covering in rows 1 to 4
    and the blue at half covering in rows 5 to 8. Column 0, column 15 and rows 0 and 9 are
    empty."""
    if not (1 <= x <= 14 and 1 <= y <= 8):
        return NONE
    if x == 1:
        return SOFT_RED if y <= 4 else SOFT_BLUE
    if x in (2, 14) or y in (1, 8):
        return LINE
    return SWATCHES[x]


DRAWINGS = {"swatches": [[swatches(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(color="#ff0000", tolerance=15, softness=10, amount=100, shift=0):
    return {"drawing": "swatches", "color": color, "tolerance": tolerance,
            "softness": softness, "amount": amount, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def settings(c, frame_no):
    return (c["color"],) + tuple(held(c, k, frame_no) for k in NAMES[1:])


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(leave_color(pixels, *settings(c, frame_no)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(amount=0, shift=c["shift"]), 0)


CASES = {
    "FX-LEAVE-001": ("The settings as they start: #ff0000, tolerance 15, softness 10, amount "
                     "100. The red, the pink across the wheel's seam (15 degrees the other "
                     "way), the skin's shadow and the skin, all within 27 degrees of red, are "
                     "kept exactly; the orange, 36 degrees off, half way across the soft band, "
                     "is half drained; the yellow, green, teal, blue, violet, magenta and the "
                     "line turn fully grey, each to its own brightness. The red at half "
                     "covering is kept; the blue at half covering turns the same grey as the "
                     "blue, at its own covering; the empty pixels stay empty.",
                     case(), [0]),
    "FX-LEAVE-002": ("Amount 0: the drawing, untouched.",
                     case(amount=0), [0]),
    "FX-LEAVE-003": ("Amount 50: every drained colour goes half way, on the encoded scale, to "
                     "its grey, and the orange a quarter of the way; the kept colours stay.",
                     case(amount=50), [0]),
    "FX-LEAVE-004": ("Tolerance 5, softness 0, a hard edge 9 degrees round red: only the red "
                     "and the soft red are kept; the pink and the skin's shadow, 15 degrees "
                     "off, turn fully grey, as does everything else.",
                     case(tolerance=5, softness=0), [0]),
    "FX-LEAVE-005": ("Tolerance 40, softness 0, 72 degrees each way: the warm half of the "
                     "wheel is kept, red, pink, shadow, skin, orange, yellow and magenta (60 "
                     "degrees off); green, teal, blue, violet (82.5 off) and the line (96 off) "
                     "turn fully grey.",
                     case(tolerance=40, softness=0), [0]),
    "FX-LEAVE-006": ("Softness 0 at the start's tolerance 15: the skin, 25.7 degrees off, is "
                     "still kept, but the orange, 36 off, now turns fully grey; the rest is "
                     "FX-LEAVE-001.",
                     case(softness=0), [0]),
    "FX-LEAVE-007": ("Softness 30, a wide band from 27 to 81 degrees: the orange keeps five "
                     "sixths of its colour, the yellow and the magenta, both 60 degrees off, "
                     "the same part, 7/18; the violet, 82.5 off, and everything further turn "
                     "fully grey.",
                     case(softness=30), [0]),
    "FX-LEAVE-008": ("Colour #4060ff, the blue: the blue and the soft blue are kept; the line, "
                     "34 degrees off, keeps about three fifths of its colour; the violet, 48 "
                     "off, the teal, 50 off, and all the warm colours turn fully grey.",
                     case(color="#4060ff"), [0]),
    "FX-LEAVE-009": ("Colour #808080, a grey, has no hue: nothing is kept, and every pixel "
                     "that shows turns fully grey, each to its own brightness.",
                     case(color="#808080"), [0]),
    "FX-LEAVE-010": ("FX-LEAVE-008 with its colour written in capitals, #4060FF: the same.",
                     case(color="#4060FF"), [0]),
    "FX-LEAVE-011": ("Colour #ff9900, the orange, keeping the skin tones: the orange, the "
                     "skin, the shadow and the yellow, all within 27 degrees, are kept; the "
                     "red, 36 off, is half drained; the pink and the rest turn fully grey.",
                     case(color="#ff9900"), [0]),
    "FX-LEAVE-012": ("Tolerance 100: every hue is at most 180 degrees off, so every colour "
                     "is kept and the drawing is untouched.",
                     case(tolerance=100), [0]),
    "FX-LEAVE-013": ("Tolerance keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 "
                     "keeps the red whole, the pink and the shadow a sixth, and drains the "
                     "rest; frame 2, at 50, keeps the violet whole and the line two thirds, "
                     "the green, teal and blue turned grey; frame 4 is FX-LEAVE-012, the "
                     "drawing untouched.",
                     case(tolerance=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-LEAVE-014": ("Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is "
                     "the drawing, frame 2 is FX-LEAVE-003, frame 4 is FX-LEAVE-001.",
                     case(amount=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-LEAVE-015": ("Amount keyed from 0 at frame 0 to 100 at frame 4, eased past its end: "
                     "frame 2 is held at 100 and is FX-LEAVE-001, as frame 4 is.",
                     case(amount=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-LEAVE-016": ("FX-LEAVE-001 moved three pixels right: the same, moved.",
                     case(shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-LEAVE-017": ("Tolerance 101, above 100.", case(tolerance=101)),
    "FX-LEAVE-018": ("Softness -1, below 0.", case(softness=-1)),
    "FX-LEAVE-019": ("Amount 101, above 100.", case(amount=101)),
    "FX-LEAVE-020": ("Tolerance keyed to 150 at frame 4.",
                     case(tolerance=keyed((0, 15), (4, 150)))),
    "FX-LEAVE-021": ("A colour written \"#ff00\", two digits short.", case(color="#ff00")),
    "FX-LEAVE-022": ("A colour written \"red\", a name, not #rrggbb.", case(color="red")),
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
        "instance_id": "fx-0-0", "type_id": "core.leave_color", "enabled": True,
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

    (OUT / "expected_leave_color.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda u, v, e=1e-12: all(abs(p - q) < e for p, q in zip(u, v))  # noqa: E731
    close = lambda f, g: all(near(u, v) for u, v in zip(f, g))  # noqa: E731
    enc = lambda q: [v / 255 for v in q[:3]]  # noqa: E731
    # The encoded straight colour of an output pixel, and how far it has gone toward its grey.
    out_enc = lambda f, xy: encoded(f[at(*xy)])  # noqa: E731

    def drained(f, xy):
        e0, e1 = enc(BANDS[xy]), out_enc(f, xy)
        y = luma(e0)
        ds = [(u - v) / (y - v) for u, v in zip(e1, e0) if abs(y - v) > 1e-3]
        assert ds and max(ds) - min(ds) < 1e-9, (xy, ds)
        return ds[0]

    red, pink, shade, skin, orange, yellow, green, teal, blue, violet, magenta = (
        (x, 4) for x in range(3, 14))
    line, soft_red, soft_blue = (2, 4), (1, 2), (1, 6)
    BANDS = {(x, 4): SWATCHES[x] for x in range(3, 14)}
    BANDS.update({line: LINE, soft_red: RED, soft_blue: BLUE})
    swatch = list(BANDS)
    deg = lambda q: hue(enc(q))  # noqa: E731

    # The rule's own pieces: the hues of the drawing, the wheel's seam, a grey has none.
    for q, h in ((RED, 0), (SHADE, 15), (ORANGE, 36), (YELLOW, 60), (GREEN, 120), (TEAL, 180),
                 (VIOLET, 277.5), (MAGENTA, 300), (LINE, 264)):
        assert abs(deg(q) - h) < 1e-12, (q, h)
    assert abs(deg(PINK) - (360 - 60 * 64 / 255)) < 1e-12 and abs(deg(SKIN) - 60 * 24 / 56) < 1e-12
    assert hue((0.5, 0.5, 0.5)) is None and distance(None, 0) == 1 and distance(10, None) == 1
    assert abs(distance(350, 10) - 20 / 180) < 1e-15 and distance(0, 180) == 1
    assert kept(0.1, 0.1, 0) == 1 and kept(0.1 + 1e-9, 0.1, 0) == 0
    assert abs(kept(0.2, 0.15, 0.1) - 0.5) < 1e-12 and kept(0.25, 0.15, 0.1) == 0
    assert leave([0.2, 0.5, 0.9], 0, 0, 0, 100)[1] == [luma([0.2, 0.5, 0.9])] * 3
    assert abs(luma([1, 1, 1]) - 1) < 1e-15

    # A pixel that does not show, and the covering, are kept by every case; a pixel at half
    # covering is treated as its colour at full covering.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-LEAVE-016" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)
            if fx != "FX-LEAVE-016":
                for s, full in ((soft_red, red), (soft_blue, blue)):
                    assert near(out_enc(px, s), out_enc(px, full), 1e-9), (fx, s)

    # The cliff: at softness 0, no pixel's distance is within 1e-5 of the tolerance.
    for fx, (_, cs, frames) in CASES.items():
        for f in frames:
            col, tol, soft, amount = settings(cs, f)
            if soft == 0 and amount > 0:
                hc = hue(enc(R.hex_color(col.lower())))
                for q in BANDS.values():
                    assert abs(distance(deg(q), hc) - tol / 100) > CLIFF, (fx, q)

    one = c["FX-LEAVE-001"]["0"]
    keep = lambda f, xys: all(f[at(*xy)] == drawn[at(*xy)] for xy in xys)  # noqa: E731
    grey = lambda f, xys: all(abs(drained(f, xy) - 1) < 1e-9 for xy in xys)  # noqa: E731
    assert keep(one, (red, pink, shade, skin, soft_red))
    assert abs(drained(one, orange) - 0.5) < 1e-9
    assert grey(one, (yellow, green, teal, blue, violet, magenta, line, soft_blue))
    for xy in (yellow, blue, line):
        assert near(out_enc(one, xy), [luma(enc(BANDS[xy]))] * 3, 1e-12)
    assert c["FX-LEAVE-002"]["0"] == drawn
    three = c["FX-LEAVE-003"]["0"]
    assert keep(three, (red, pink, shade, skin))
    assert abs(drained(three, orange) - 0.25) < 1e-9
    assert all(abs(drained(three, xy) - 0.5) < 1e-9 for xy in (yellow, green, blue, line))
    four = c["FX-LEAVE-004"]["0"]
    assert keep(four, (red, soft_red)) and grey(four, [xy for xy in swatch if xy not in
                                                       (red, soft_red)])
    five = c["FX-LEAVE-005"]["0"]
    assert keep(five, (red, pink, shade, skin, orange, yellow, magenta))
    assert grey(five, (green, teal, blue, violet, line))
    six = c["FX-LEAVE-006"]["0"]
    assert keep(six, (skin,)) and grey(six, (orange,))
    assert all(six[i] == one[i] for i in range(W * H) if i % W != 7)
    seven = c["FX-LEAVE-007"]["0"]
    assert abs(drained(seven, orange) - 1 / 6) < 1e-9
    assert abs(drained(seven, yellow) - 11 / 18) < 1e-9
    assert abs(drained(seven, magenta) - 11 / 18) < 1e-9
    assert grey(seven, (violet, line, green, teal, blue))
    eight = c["FX-LEAVE-008"]["0"]
    assert keep(eight, (blue, soft_blue))
    assert 0.35 < drained(eight, line) < 0.45
    assert grey(eight, (violet, teal, red, pink, shade, skin, orange, yellow, green, magenta))
    nine = c["FX-LEAVE-009"]["0"]
    assert grey(nine, swatch)
    assert c["FX-LEAVE-010"]["0"] == eight
    eleven = c["FX-LEAVE-011"]["0"]
    assert keep(eleven, (orange, skin, shade, yellow))
    assert abs(drained(eleven, red) - 0.5) < 1e-9
    assert grey(eleven, (pink, magenta, green, teal, blue, violet, line))
    assert c["FX-LEAVE-012"]["0"] == drawn
    thirteen = c["FX-LEAVE-013"]
    assert keep(thirteen["0"], (red,)) and grey(thirteen["0"], (skin, orange, blue))
    for xy, h in ((pink, 60 * 64 / 255), (shade, 15)):
        assert abs(drained(thirteen["0"], xy) - h / 18) < 1e-9, xy  # 1 - (h/180 - 0) / 0.1
    assert close(thirteen["2"], render(case(tolerance=50), 0))
    assert keep(thirteen["2"], (violet, magenta, yellow)) and grey(thirteen["2"],
                                                                   (green, teal, blue))
    assert abs(drained(thirteen["2"], line) - 1 / 3) < 1e-9
    assert thirteen["4"] == drawn
    fourteen = c["FX-LEAVE-014"]
    assert fourteen["0"] == drawn and fourteen["2"] == three and fourteen["4"] == one
    fifteen = c["FX-LEAVE-015"]
    assert ease(OVERSHOOT, 0.5) > 1 and value_at(case(amount=keyed(
        (0, 0, OVERSHOOT), (4, 100)))["amount"], 2) > 100
    assert fifteen["0"] == drawn and fifteen["2"] == one and fifteen["4"] == one
    moved = c["FX-LEAVE-016"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == one[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
