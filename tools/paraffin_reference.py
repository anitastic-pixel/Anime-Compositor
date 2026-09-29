"""Paraffin, worked a second way.

D-185 adds `core.paraffin`: the paraffin (パラ) of anime finishing, a soft wash of colour laid
over a figure from one side, as a shadow is airbrushed up from a figure's feet or a warm light
down from its head. It differs from Gradient (D-114) in three ways: the wash is fitted to the
figure itself, the pixels at least half covered, so on a full-frame cel it runs across the figure
and not across the frame, and fits each drawing of a cycle; it fades on a smooth curve, as an
airbrush does, with no line where it starts or stops; and it has the two blends paraffin is most
often laid with, overlay and soft light, which tint a figure and keep its paint's light and dark.
It is this program's own rule; nothing is ported. Document 21 is the rule in words; this file is
the reference for the numbers document 25 pins against it.

The rule. With `spread` 0 or `opacity` 0 the output is the input. `u = (sin t, -cos t)` for
`direction` t in degrees, the way the wash comes from, clockwise from up; `n(x, y)` is the pixel
centre's reach that way, `(x + 0.5) u.x + (y + 0.5) u.y`. The figure is every pixel with covering
0.5 or more; if there is none the output is the input. `near` and `far` are the largest and
smallest `n` over the figure, and `reach = spread / 100 * (near - far + 1)`. At a pixel with
a > 0: `s = clamp((near - n) / reach, 0, 1)` and `o = (1 - s * s * (3 - 2 s)) * opacity / 100`.
The pixel's straight colour through the sRGB curve, held inside 0 to 1, is `b`, per channel, and
`c` the colour as written over 255; `f` is `c` (normal), `b c` (multiply), `1 - (1 - b)(1 - c)`
(screen), `b + c` (add), `2 b c` if `b <= 0.5` else `1 - 2 (1 - b)(1 - c)` (overlay), or
`b - (1 - 2 c) b (1 - b)` if `c <= 0.5` else `b + (2 c - 1)(D - b)`, with `D` = `((16 b - 12) b
+ 4) b` if `b <= 0.25` else `sqrt b` (soft light), as the W3C's compositing rules write them. The
output is `b + o (f - b)`, held inside 0 to 1, through the sRGB curve back to linear, at the
pixel's own covering: the shared colour rule of D-111's batch. A pixel with a = 0, or with o = 0, stays exactly as it is. The layer does not grow; no
setting is a distance, so a draft keeps them.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, Distance
Gradation's block, unmoved unless the case says. The drawing goes into `Fixtures/paraffin/media`,
the projects into `Fixtures/paraffin`, and the expected frames into
`Fixtures/paraffin/expected_paraffin.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/paraffin_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
from gradient_reference import BLENDS as GRADIENT_BLENDS  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import distance_gradation_reference as DG  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "paraffin"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"direction": (0, 360), "spread": (0, 100), "opacity": (0, 100)}
NAMES = ("color", "direction", "spread", "opacity", "blend")


# --- the rule -------------------------------------------------------------------------------

def soft_light(b, c):
    if c <= 0.5:
        return b - (1 - 2 * c) * b * (1 - b)
    d = ((16 * b - 12) * b + 4) * b if b <= 0.25 else math.sqrt(b)
    return b + (2 * c - 1) * (d - b)


BLENDS = {**GRADIENT_BLENDS,
          "overlay": lambda b, c: 2 * b * c if b <= 0.5 else 1 - 2 * (1 - b) * (1 - c),
          "soft_light": soft_light}


def clamp(v, lo, hi):
    return min(hi, max(lo, v))


def reaches(pixels, direction, w=W, h=H):
    """n at every pixel, and near and far over the figure, or None when there is no figure."""
    t = math.radians(direction)
    ux, uy = math.sin(t), -math.cos(t)
    n = [(i % w + 0.5) * ux + (i // w + 0.5) * uy for i in range(w * h)]
    figure = [n[i] for i in range(w * h) if pixels[i][3] >= 128]
    return (n, max(figure), min(figure)) if figure else (n, None, None)


def strength(n, near, reach, opacity):
    s = clamp((near - n) / reach, 0, 1)
    return (1 - s * s * (3 - 2 * s)) * opacity / 100


def paraffin(pixels, color, direction, spread, opacity, blend, w=W, h=H):
    """`pixels` are 8-bit straight RGBA, rows top to bottom, a drawing w by h."""
    if spread == 0 or opacity == 0:
        return [R.working(p) for p in pixels]
    n, near, far = reaches(pixels, direction, w, h)
    if near is None:
        return [R.working(p) for p in pixels]
    reach = spread / 100 * (near - far + 1)
    c = [v / 255 for v in R.hex_color(color.lower())]
    mix = BLENDS[blend]
    out = []
    for i, p in enumerate(pixels):
        wp = R.working(p)
        o = strength(n[i], near, reach, opacity)
        if p[3] > 0 and o > 0:
            a = wp[3]
            b = [S.linear_to_srgb(clamp(wp[k] / a, 0, 1)) for k in range(3)]
            wp = [srgb_to_linear(clamp(b[k] + o * (mix(b[k], c[k]) - b[k]), 0, 1)) * a
                  for k in range(3)] + [a]
        out.append(wp)
    return out


# --- the cases ------------------------------------------------------------------------------

VIOLET = DG.VIOLET  # #6450a0
DRAWINGS = {"block": DG.DRAWINGS["block"]}


def case(color=VIOLET, direction=180, spread=70, opacity=50, blend="multiply", shift=0):
    return {"drawing": "block", "color": color, "direction": direction, "spread": spread,
            "opacity": opacity, "blend": blend, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(paraffin(pixels, c["color"], held(c, "direction", frame_no),
                            held(c, "spread", frame_no), held(c, "opacity", frame_no),
                            c["blend"]), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(spread=0, shift=c["shift"]), 0)


FULL = {"spread": 100, "opacity": 100, "blend": "normal"}

CASES = {
    "FX-PARA-001": ("The settings as they start: #6450a0 from below (direction 180), spread 70, "
                    "opacity 50, multiply. The block's bottom row, row 8, is darkened toward "
                    "violet the most, each row above less, and rows 1 and 2, more than 70 per cent "
                    "of the figure's eight rows up, not at all; the dot at (14, 4) is darkened as "
                    "row 4 is; the empty pixels stay empty.",
                    case(), [0]),
    "FX-PARA-002": ("From below, spread 100, opacity 100, normal: row 8 wholly violet at each "
                    "pixel's own covering, and each row above less violet on the smooth curve, "
                    "row 1 by 1 - s s (3 - 2 s) at s = 7/8, about 0.043.",
                    case(**FULL), [0]),
    "FX-PARA-003": ("Direction 0, from above, the rest as FX-PARA-002: turned upside down, row 1 "
                    "wholly violet and row 8 the least.",
                    case(direction=0, **FULL), [0]),
    "FX-PARA-004": ("Direction 90, from the right, the rest as FX-PARA-002: the figure reaches "
                    "from column 0 to the dot at column 14, so the dot is wholly violet and the "
                    "block less violet toward the left; the quarter-covered column 11, not part "
                    "of the figure, is shaded as its place says.",
                    case(direction=90, **FULL), [0]),
    "FX-PARA-005": ("Spread 50, the rest as FX-PARA-002: the wash stops half way up the "
                    "figure; rows 1 to 4 and the dot are untouched.",
                    case(direction=180, spread=50, opacity=100, blend="normal"), [0]),
    "FX-PARA-006": ("Direction 45, from the upper right, the rest as FX-PARA-002: the dot and "
                    "the block's top right corner the most violet, the bottom left the least.",
                    case(direction=45, **FULL), [0]),
    "FX-PARA-007": ("Overlay, spread 100, opacity 100: the skin, light, is lifted toward the "
                    "violet's screen and its shadow, darker, pushed toward its multiply; the "
                    "most at row 8.",
                    case(spread=100, opacity=100, blend="overlay"), [0]),
    "FX-PARA-008": ("Soft light, spread 100, opacity 100: gentler than overlay; each channel "
                    "whose violet is below one half is darkened, blue is lifted.",
                    case(spread=100, opacity=100, blend="soft_light"), [0]),
    "FX-PARA-009": ("Multiply, spread 100, opacity 100: darkened toward violet, the most at "
                    "row 8.",
                    case(spread=100, opacity=100, blend="multiply"), [0]),
    "FX-PARA-010": ("Screen, spread 100, opacity 100: lightened, none darkened.",
                    case(spread=100, opacity=100, blend="screen"), [0]),
    "FX-PARA-011": ("Add, spread 100, opacity 100: the violet added and held at 1, so the skin's "
                    "red, already near 1, stops at 1.",
                    case(spread=100, opacity=100, blend="add"), [0]),
    "FX-PARA-012": ("Spread 0: the drawing, untouched.",
                    case(spread=0), [0]),
    "FX-PARA-013": ("Opacity 0: the drawing, untouched.",
                    case(opacity=0), [0]),
    "FX-PARA-014": ("FX-PARA-001 with the colour written in capitals, #6450A0: the same.",
                    case(color=VIOLET.upper()), [0]),
    "FX-PARA-015": ("Direction keyed from 0 at frame 0 to 180 at frame 4, linear, the rest as "
                    "FX-PARA-002: frame 0 is FX-PARA-003, frame 2 is FX-PARA-004 and frame 4 is "
                    "FX-PARA-002: the wash swings round the figure.",
                    case(direction=keyed((0, 0), (4, 180)), **FULL), [0, 2, 4]),
    "FX-PARA-016": ("Spread eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                    "overshoots, opacity 100, normal: at frame 2 it would pass 100, is held at "
                    "100, and is FX-PARA-002; frame 0 is the drawing.",
                    case(spread=keyed((0, 0, OVERSHOOT), (4, 100)), opacity=100,
                         blend="normal"), [0, 2]),
    "FX-PARA-017": ("FX-PARA-001 moved three pixels right: the same, moved; the wash is worked "
                    "on the drawing before it moves.",
                    case(shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-PARA-018": ("Direction 361, above 360.", case(direction=361)),
    "FX-PARA-019": ("Spread 101, above 100.", case(spread=101)),
    "FX-PARA-020": ("Opacity -1, below 0.", case(opacity=-1)),
    "FX-PARA-021": ("Spread keyed to 150 at frame 4.", case(spread=keyed((0, 50), (4, 150)))),
    "FX-PARA-022": ("Blend \"color_burn\", which is not a blend.", case(blend="color_burn")),
    "FX-PARA-023": ("Colour \"#12345\", one digit short.", case(color="#12345")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = DG.project_json(fx, {**c, "width": 0, "invert": "off"})
    p["compositions"][0]["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.paraffin", "enabled": True,
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

    (OUT / "expected_paraffin.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    pixels = [p for row in DRAWINGS["block"] for p in row]
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    shows = [i for i in range(W * H) if drawn[i][3] > 0]
    violet = [srgb_to_linear(v / 255) for v in R.hex_color(VIOLET)]
    # How far each pixel that shows is moved toward violet, when painted normal, on the green
    # channel through the curve: its strength.
    enc = lambda p: S.linear_to_srgb(p[1] / p[3])  # noqa: E731
    share = lambda f, i: (enc(f[i]) - enc(drawn[i])) / (0x50 / 255 - enc(drawn[i]))  # noqa: E731

    # The blends against values worked by hand, and the curve's ends.
    assert BLENDS["overlay"](0.25, 0.5) == 0.25 and BLENDS["overlay"](0.75, 0.5) == 0.75
    assert BLENDS["soft_light"](0.25, 0.5) == 0.25 and BLENDS["soft_light"](0.64, 1) == 0.8
    assert BLENDS["soft_light"](0.5, 0) == 0.25 and BLENDS["overlay"](1, 0) == 1
    assert strength(5, 5, 8, 100) == 1 and strength(-3, 5, 8, 100) == 0
    assert strength(1, 5, 8, 100) == 0.5
    # The figure: from below it runs from row 8 to row 1, eight rows; from the right from the
    # dot's column, 14, to column 0, fifteen columns; the quarter-covered column is not in it.
    _, near, far = reaches(pixels, 180)
    assert abs(near - 8.5) < 1e-12 and abs(far - 1.5) < 1e-12
    _, near, far = reaches(pixels, 90)
    assert abs(near - 14.5) < 1e-12 and abs(far - 0.5) < 1e-12
    assert reaches([[0, 0, 0, 127]] * (W * H), 0)[1] is None

    # Every case keeps every covering and leaves the empty pixels exactly as they are, and no
    # channel passes the covering.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-PARA-017" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)

    one = c["FX-PARA-001"]["0"]
    row_change = lambda f, y: abs(f[at(3, y)][1] - drawn[at(3, y)][1])  # noqa: E731
    assert row_change(one, 8) > row_change(one, 7) > row_change(one, 6) > row_change(one, 5)
    assert row_change(one, 4) > row_change(one, 3) > 0
    assert all(one[at(x, y)] == drawn[at(x, y)] for x in range(12) for y in (1, 2))
    assert one[at(14, 4)] != drawn[at(14, 4)]
    two = c["FX-PARA-002"]["0"]
    assert all(close(two[at(x, 8)], [v * two[at(x, 8)][3] for v in violet] + [two[at(x, 8)][3]],
                     1e-9) for x in range(12))
    s = 7 / 8
    assert abs(share(two, at(3, 1)) - (1 - s * s * (3 - 2 * s))) < 1e-9
    assert all(share(two, at(3, y)) > share(two, at(3, y - 1)) for y in range(2, 9))
    three = c["FX-PARA-003"]["0"]
    assert all(abs(share(three, at(3, y)) - share(two, at(3, 9 - y))) < 1e-9 for y in range(1, 9))
    four = c["FX-PARA-004"]["0"]
    assert close(four[at(14, 4)], violet + [1.0], 1e-9)
    assert all(share(four, at(x, 5)) > share(four, at(x - 1, 5)) for x in range(1, 10))
    assert four[at(11, 5)] != drawn[at(11, 5)]
    five = c["FX-PARA-005"]["0"]
    assert all(five[at(x, y)] == drawn[at(x, y)] for x in range(12) for y in range(1, 5))
    assert five[at(14, 4)] == drawn[at(14, 4)] and five[at(3, 8)] == two[at(3, 8)]
    six = c["FX-PARA-006"]["0"]
    assert share(six, at(9, 1)) > share(six, at(5, 4)) > share(six, at(0, 8))
    assert close(six[at(14, 4)], violet + [1.0], 1e-9)
    seven, eight = c["FX-PARA-007"]["0"], c["FX-PARA-008"]["0"]
    for f in (seven, eight):
        assert row_change(f, 8) > row_change(f, 4) > row_change(f, 1)
    # Overlay moves the skin's light green further than soft light does.
    assert abs(seven[at(3, 8)][1] - drawn[at(3, 8)][1]) > abs(eight[at(3, 8)][1]
                                                             - drawn[at(3, 8)][1])
    assert eight[at(3, 8)][2] > drawn[at(3, 8)][2] and eight[at(3, 8)][0] < drawn[at(3, 8)][0]
    nine = c["FX-PARA-009"]["0"]
    assert all(nine[i][k] < drawn[i][k] + 1e-12 for i in shows for k in range(3))
    ten = c["FX-PARA-010"]["0"]
    assert all(ten[i][k] > drawn[i][k] - 1e-12 for i in shows for k in range(3))
    eleven = c["FX-PARA-011"]["0"]
    assert eleven[at(3, 8)][0] == 1.0 and eleven[at(3, 8)][2] > ten[at(3, 8)][2]
    for fx in ("FX-PARA-012", "FX-PARA-013"):
        assert c[fx]["0"] == drawn, fx
    assert c["FX-PARA-014"]["0"] == one
    fifteen = c["FX-PARA-015"]
    assert close_all(fifteen["0"], three) and close_all(fifteen["2"], four)
    assert close_all(fifteen["4"], two)
    sixteen = c["FX-PARA-016"]
    assert ease(OVERSHOOT, 0.5) > 1 and sixteen["0"] == drawn and sixteen["2"] == two
    moved = c["FX-PARA-017"]
    assert moved["0"] == moved["3"]
    assert all(moved["0"][at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    print("checked")


def close_all(f, g, e=1e-9):
    return all(abs(u - v) < e for p, q in zip(f, g) for u, v in zip(p, q))


if __name__ == "__main__":
    main()
