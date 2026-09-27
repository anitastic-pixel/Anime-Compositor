"""Gradient Map, worked a second way.

D-129 adds `core.gradient_map`: each pixel's brightness mapped to a ramp of three colours, a
shadow colour for black, a midtone colour at the midpoint and a highlight colour for white, as a
colourist grades a whole cel into one mood (a night, a sunset, a flashback's sepia). With the
starting colours, black, #808080 and white, it turns the picture to grey. `midpoint` is 1 to 99
(50), how bright a pixel must be to take the midtone colour, and `amount` is 0 to 100 (100), how
far each pixel is moved toward its mapped colour. Pixels that do not show stay as they are, and
every pixel keeps its own covering. It is this program's own method, modelled on After Effects'
Gradient Map (its Colorama and Tritone in spirit); nothing is ported. Document 21 is the rule in
words; this file is the reference for the numbers document 25 pins against it.

The rule. At a pixel with covering a > 0, with b its straight linear colour, the brightness is
t = linear_to_srgb(clamp(0.2126 b.r + 0.7152 b.g + 0.0722 b.b, 0, 1)) and m = midpoint / 100.
If t <= m, s = t / m and E = shadow + s (midtone - shadow); otherwise s = (t - m) / (1 - m) and
E = midtone + s (highlight - midtone), per channel, each colour's 8-bit value / 255 (encoded).
M = srgb_to_linear(E), and the output is Gradient's blend mix, normal, with op = amount / 100:
out = ((b + op (M - b)) * a, a). No growth.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: swatches, drawn below. The drawing goes into `Fixtures/gradient_map/media`, the
projects into `Fixtures/gradient_map`, and the expected frames into
`Fixtures/gradient_map/expected_gradient_map.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/gradient_map_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
from gradient_reference import BLENDS  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "gradient_map"
TOLERANCE = 2e-5  # document 25's default for a filter
MIDPOINT = (1, 99)
AMOUNT = (0, 100)
NAMES = ("shadow_color", "midtone_color", "highlight_color", "midpoint", "amount")


# --- the rule -------------------------------------------------------------------------------

def brightness(b):
    """A straight linear colour's brightness, encoded, 0 to 1."""
    return S.linear_to_srgb(min(1.0, max(0.0, 0.2126 * b[0] + 0.7152 * b[1] + 0.0722 * b[2])))


def ramp(t, m, colors):
    """The encoded colour at brightness t on the ramp shadow, midtone at m, highlight."""
    lo, mid, hi = colors
    if t <= m:
        s, u, v = t / m, lo, mid
    else:
        s, u, v = (t - m) / (1 - m), mid, hi
    return [x + s * (y - x) for x, y in zip(u, v)]


def gradient_map(pixels, shadow_color, midtone_color, highlight_color, midpoint, amount):
    """`pixels` are 8-bit straight RGBA."""
    colors = [[v / 255 for v in R.hex_color(c.lower())]
              for c in (shadow_color, midtone_color, highlight_color)]
    m, op, mix = midpoint / 100, amount / 100, BLENDS["normal"]
    out = []
    for p in pixels:
        if p[3] == 0:
            out.append(R.working(p))
            continue
        a = p[3] / 255
        b = [srgb_to_linear(p[c] / 255) for c in range(3)]
        g = [srgb_to_linear(e) for e in ramp(brightness(b), m, colors)]
        out.append([(b[c] + op * (mix(b[c], g[c]) - b[c])) * a for c in range(3)] + [a])
    return out


# --- the drawing ----------------------------------------------------------------------------

LINE = R.LINE                    # #1e1a24
SOFT = R.SOFT                    # the line at half covering, its antialiased edge
SKIN = R.SKIN                    # #f6d6be
SOFT_SKIN = SKIN[:3] + (128,)    # the skin at half covering
SHADE = (220, 160, 140, 255)     # the skin's shadow, #dca08c
TRACE = R.TRACE                  # a red, #c82828
WHITE = (255, 255, 255, 255)
BLUE = (64, 96, 255, 255)        # #4060ff
BLACK = (0, 0, 0, 255)
NONE = S.NONE


def swatches(x, y):
    """A box of line in columns 2 to 14 and rows 1 to 8 holding, in rows 2 to 7, skin in
    columns 3 to 5, shadow in 6 to 8, red in 9 and 10, white in 11, blue in 12 and black in 13;
    down its left side, column 1, the line at half covering in rows 1 to 4 and the skin at half
    covering in rows 5 to 8. Column 0, column 15 and rows 0 and 9 are empty."""
    if not (1 <= x <= 14 and 1 <= y <= 8):
        return NONE
    if x == 1:
        return SOFT if y <= 4 else SOFT_SKIN
    if x in (2, 14) or y in (1, 8):
        return LINE
    return {3: SKIN, 4: SKIN, 5: SKIN, 6: SHADE, 7: SHADE, 8: SHADE, 9: TRACE, 10: TRACE,
            11: WHITE, 12: BLUE, 13: BLACK}[x]


DRAWINGS = {"swatches": [[swatches(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(shadow_color="#000000", midtone_color="#808080", highlight_color="#ffffff",
         midpoint=50, amount=100, shift=0):
    return {"drawing": "swatches", "shadow_color": shadow_color, "midtone_color": midtone_color,
            "highlight_color": highlight_color, "midpoint": midpoint, "amount": amount,
            "shift": shift}


def clamp(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(gradient_map(pixels, c["shadow_color"], c["midtone_color"],
                                c["highlight_color"],
                                clamp(value_at(c["midpoint"], frame_no), MIDPOINT),
                                clamp(value_at(c["amount"], frame_no), AMOUNT)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(amount=0, shift=c["shift"]), 0)


SUNSET = {"shadow_color": "#2a1650", "midtone_color": "#c85a50", "highlight_color": "#ffe6b4"}

CASES = {
    "FX-GRADMAP-001": ("The settings as they start: black, #808080 at the midpoint 50, white, "
                       "amount 100. Every pixel that shows turns grey by its brightness, red, "
                       "green and blue alike: the white stays white, the black stays black, the "
                       "skin is the lightest of the rest and the line the darkest; a pixel at "
                       "half covering takes the same grey as its colour at full covering, at its "
                       "own covering, and the empty pixels stay empty.",
                       case(), [0]),
    "FX-GRADMAP-002": ("Amount 0: the drawing, untouched.",
                       case(amount=0), [0]),
    "FX-GRADMAP-003": ("Amount 50: every pixel halfway, in linear light, between the drawing "
                       "and FX-GRADMAP-001.",
                       case(amount=50), [0]),
    "FX-GRADMAP-004": ("A sunset ramp, #2a1650, #c85a50 at 50, #ffe6b4: the black turns "
                       "#2a1650 and the white #ffe6b4 exactly, each at its own covering; the "
                       "line is deep violet, the red and the blue a dusky red, the skin and its "
                       "shadow peach.",
                       case(**SUNSET), [0]),
    "FX-GRADMAP-005": ("Midpoint 25: the midtone grey is reached at a quarter of the brightness, "
                       "so every pixel between black and white is lighter than in "
                       "FX-GRADMAP-001, and black and white are as they were.",
                       case(midpoint=25), [0]),
    "FX-GRADMAP-006": ("Midpoint 75: every pixel between black and white is darker than in "
                       "FX-GRADMAP-001, and black and white are as they were.",
                       case(midpoint=75), [0]),
    "FX-GRADMAP-007": ("Midpoint 1, the least: every pixel between black and white is at or "
                       "above the midpoint, so each lies on the grey-to-white half of the ramp, "
                       "at least #808080; black stays black.",
                       case(midpoint=1), [0]),
    "FX-GRADMAP-008": ("Midpoint 99, the most: every pixel between black and white lies on the "
                       "black-to-grey half, at most #808080; white stays white.",
                       case(midpoint=99), [0]),
    "FX-GRADMAP-009": ("All three colours #6450a0: every pixel that shows is #6450a0 at its own "
                       "covering, whatever its brightness.",
                       case(shadow_color="#6450a0", midtone_color="#6450a0",
                            highlight_color="#6450a0"), [0]),
    "FX-GRADMAP-010": ("Shadow white and highlight black, the ramp turned over: a negative in "
                       "grey; the white turns black, the black turns white, and the line, the "
                       "darkest colour, is now the lightest but the black.",
                       case(shadow_color="#ffffff", highlight_color="#000000"), [0]),
    "FX-GRADMAP-011": ("Only the midtone changed, to #ff0000: black and white stay black and "
                       "white, and every pixel between is reddened, its green and blue the same "
                       "and below its red.",
                       case(midtone_color="#ff0000"), [0]),
    "FX-GRADMAP-012": ("Midpoint keyed from 25 at frame 0 to 75 at frame 4, linear: frame 0 is "
                       "FX-GRADMAP-005, frame 2, at 50, is FX-GRADMAP-001, frame 4 is "
                       "FX-GRADMAP-006.",
                       case(midpoint=keyed((0, 25), (4, 75))), [0, 2, 4]),
    "FX-GRADMAP-013": ("Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is "
                       "the drawing, frame 2 is FX-GRADMAP-003, frame 4 is FX-GRADMAP-001.",
                       case(amount=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-GRADMAP-014": ("Amount keyed from 0 at frame 0 to 100 at frame 4, eased past its end "
                       "(132.5 at frame 2): frame 2 is held at 100 and is FX-GRADMAP-001, as "
                       "frame 4 is.",
                       case(amount=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-GRADMAP-015": ("FX-GRADMAP-004 with its colours written in capitals: the same.",
                       case(**{k: v.upper() for k, v in SUNSET.items()}), [0]),
    "FX-GRADMAP-016": ("FX-GRADMAP-004 moved three pixels right: the same, moved.",
                       case(shift=3, **SUNSET), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-GRADMAP-017": ("Midpoint 0, below 1.", case(midpoint=0)),
    "FX-GRADMAP-018": ("Midpoint 100, above 99.", case(midpoint=100)),
    "FX-GRADMAP-019": ("Amount 101, above 100.", case(amount=101)),
    "FX-GRADMAP-020": ("Amount -1, below 0.", case(amount=-1)),
    "FX-GRADMAP-021": ("Midpoint keyed to 150 at frame 4.",
                       case(midpoint=keyed((0, 50), (4, 150)))),
    "FX-GRADMAP-022": ("A shadow colour written \"#12345\", one digit short.",
                       case(shadow_color="#12345")),
    "FX-GRADMAP-023": ("A highlight colour written \"white\", a name, not #rrggbb.",
                       case(highlight_color="white")),
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
        "instance_id": "fx-0-0", "type_id": "core.gradient_map", "enabled": True,
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

    (OUT / "expected_gradient_map.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda u, v, e=1e-12: all(abs(p - q) < e for p, q in zip(u, v))  # noqa: E731
    close = lambda f, g: all(near(u, v) for u, v in zip(f, g))  # noqa: E731
    shows = [i for i in range(W * H) if drawn[i][3] > 0]
    straight = lambda p: [v / p[3] for v in p[:3]]  # noqa: E731
    lin = lambda hx: [srgb_to_linear(v / 255) for v in R.hex_color(hx.lower())]  # noqa: E731
    painted = lambda f, i, rgb: near(f[i], [v * drawn[i][3] for v in rgb] + [drawn[i][3]])  # noqa: E731,E501
    white, black = at(11, 4), at(13, 4)
    between = [i for i in shows if not (i % W in (11, 13) and 2 <= i // W <= 7)]

    # The rule's own pieces.
    assert abs(brightness([1.0, 1.0, 1.0]) - 1) < 1e-15 and brightness([0.0, 0.0, 0.0]) == 0.0
    assert brightness([2.0, 2.0, 2.0]) == brightness([1.0, 1.0, 1.0])  # held before the curve
    three = ([0.0] * 3, [0.5] * 3, [1.0] * 3)
    assert ramp(0.5, 0.5, three) == [0.5] * 3 and ramp(0.25, 0.5, three) == [0.25] * 3
    assert ramp(0.75, 0.5, three) == [0.75] * 3 and ramp(1.0, 0.01, three) == [1.0] * 3
    assert ramp(0.5, 0.25, three) == [0.5 + 0.5 / 3] * 3
    # A pixel that does not show, and the covering, are kept by every case.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-GRADMAP-016" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)

    one = c["FX-GRADMAP-001"]["0"]
    grey = lambda i: straight(one[i])[0]  # noqa: E731
    for i in shows:
        assert near(one[i][:3], [one[i][0]] * 3), i
    assert painted(one, white, [1.0] * 3) and painted(one, black, [0.0] * 3)
    greys = {x: grey(at(x, 4)) for x in range(2, 14)}
    assert max(v for x, v in greys.items() if x != 11) == greys[3]
    assert min(v for x, v in greys.items() if x != 13) == greys[2]
    assert near(straight(one[at(1, 2)]), straight(one[at(2, 4)]))       # the soft line
    assert near(straight(one[at(1, 6)]), straight(one[at(4, 4)]))       # the soft skin
    assert one[at(1, 6)][3] == 128 / 255
    assert c["FX-GRADMAP-002"]["0"] == drawn
    assert close(c["FX-GRADMAP-003"]["0"],
                 [[(u + v) / 2 for u, v in zip(p, q)] for p, q in zip(drawn, one)])
    four = c["FX-GRADMAP-004"]["0"]
    assert painted(four, black, lin("#2a1650")) and painted(four, white, lin("#ffe6b4"))
    assert four[at(2, 4)][2] > four[at(2, 4)][1]                          # the line, violet
    for x in (9, 12):
        r, g, b = straight(four[at(x, 4)])
        assert r > 2 * max(g, b), x                                       # a dusky red
    for x in (4, 7):
        r, g, b = straight(four[at(x, 4)])
        assert r > g > b and r > 0.5, x                                   # peach
    five, six = c["FX-GRADMAP-005"]["0"], c["FX-GRADMAP-006"]["0"]
    for i in between:
        assert five[i][0] > one[i][0] > six[i][0], i
    for f in (five, six):
        assert near(f[white], one[white]) and near(f[black], one[black])
    half = srgb_to_linear(128 / 255)
    seven, eight = c["FX-GRADMAP-007"]["0"], c["FX-GRADMAP-008"]["0"]
    for i in between:
        assert straight(seven[i])[0] >= half - 1e-12 >= straight(eight[i])[0] - 2e-12, i
    assert painted(seven, black, [0.0] * 3) and painted(eight, white, [1.0] * 3)
    assert all(painted(c["FX-GRADMAP-009"]["0"], i, lin("#6450a0")) for i in shows)
    ten = c["FX-GRADMAP-010"]["0"]
    assert painted(ten, white, [0.0] * 3) and painted(ten, black, [1.0] * 3)
    assert max(straight(ten[i])[0] for i in shows if i % W != 13) == straight(ten[at(2, 4)])[0]
    eleven = c["FX-GRADMAP-011"]["0"]
    assert painted(eleven, white, [1.0] * 3) and painted(eleven, black, [0.0] * 3)
    for i in between:
        r, g, b = eleven[i][:3]
        assert abs(g - b) < 1e-12 and r > g, i
    twelve, thirteen, fourteen = (c[f"FX-GRADMAP-0{n}"] for n in (12, 13, 14))
    assert twelve["0"] == five and twelve["2"] == one and twelve["4"] == six
    assert thirteen["0"] == drawn and thirteen["2"] == c["FX-GRADMAP-003"]["0"]
    assert thirteen["4"] == one
    assert ease(OVERSHOOT, 0.5) > 1 and value_at(case(amount=keyed((0, 0, OVERSHOOT),
                                                                   (4, 100)))["amount"], 2) > 100
    assert fourteen["0"] == drawn and fourteen["2"] == one and fourteen["4"] == one
    assert c["FX-GRADMAP-015"]["0"] == four
    moved = c["FX-GRADMAP-016"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == four[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
