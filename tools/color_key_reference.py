"""Colour key, worked a second way.

D-97 adds `core.color_key`. It makes the pixels of chosen colours transparent, as a green screen
is taken out from behind a figure, with a soft band past the tolerance where pixels go only part
of the way. A pixel's distance from a chosen colour is worked on its 8-bit red, green and blue,
as D-88's choice is: with `match` "rgb" it is the largest of the three channel differences, so
at softness 0 exactly D-88's chosen pixels go; with `match` "hue" it is the difference of the two
hues, 180 degrees counted as 255, and a grey, which has no hue, is 255 from everything. Of
several colours the nearest counts. A pixel at or within `tolerance` goes; one further than the
tolerance plus `softness` stays; between, its colour and covering are both multiplied by how far
across the band it is. With no colour chosen it changes nothing. It is this program's own
method, modelled on After Effects' Color Key; nothing is ported. Document 21 is the rule in
words; this file is the reference for the numbers document 25 pins against it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a figure in front of a green screen, drawn below. The drawing goes into
`Fixtures/color_key/media`, the projects into `Fixtures/color_key`, and the expected frames into
`Fixtures/color_key/expected_color_key.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_key_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_key"
TOLERANCE = 2e-5  # document 25's default for a filter
MAX = 255  # the top of both tolerance and softness


# --- the rule -------------------------------------------------------------------------------

def hue(q):
    """The hue of 8-bit red, green and blue in degrees, 0 to 360, or None for a grey. Of equal
    largest channels, red counts before green and green before blue."""
    r, g, b = q
    hi, lo = max(q), min(q)
    c = hi - lo
    if c == 0:
        return None
    if hi == r:
        return 60 * (((g - b) / c) % 6)
    if hi == g:
        return 60 * ((b - r) / c + 2)
    return 60 * ((r - g) / c + 4)


def distance(q, t, match):
    """How far the 8-bit colour `q` is from the chosen colour `t`, 0 to 255."""
    if match == "rgb":
        return max(abs(q[i] - t[i]) for i in range(3))
    a, b = hue(q), hue(t)
    if a is None or b is None:
        return 255
    d = abs(a - b)
    return min(d, 360 - d) * 255 / 180


def keep(d, tolerance, softness):
    """How much of a pixel stays: none at or within the tolerance, all of it past the band."""
    if d <= tolerance:
        return 0.0
    if softness == 0:
        return 1.0
    return min(1.0, (d - tolerance) / softness)


def color_key(pixels, colors, tolerance, softness, match):
    if not colors:
        return [R.working(p) for p in pixels]
    targets = [R.hex_color(s.lower()) for s in colors]
    out = []
    for p in pixels:
        w = R.working(p)
        if p[3] > 0:
            d = min(distance(p[:3], t, match) for t in targets)
            f = keep(d, tolerance, softness)
            w = [v * f for v in w]
        out.append(w)
    return out


# --- the drawing ----------------------------------------------------------------------------

GREEN = (0, 177, 64, 255)        # the screen, #00b140
EDGE = (0, 177, 64, 128)         # the screen at half covering, down the right edge
SHADOW = (0, 100, 36, 255)       # the figure's shadow on the screen, #006424, the same hue darker
SPILL = (90, 160, 100, 255)      # the green light spilt on the figure's left side, #5aa064
SKIN = R.SKIN                    # #f6d6be
LINE = R.LINE                    # #1e1a24
GREY = (128, 128, 128, 255)      # a grey button, #808080
NONE = S.NONE
GREEN_HEX, GREY_HEX, SKIN_HEX = "#00b140", "#808080", "#f6d6be"


def screen(x, y):
    """Column 0 is empty and column 15 is the screen at half covering. Rows 8 and 9 are the
    shadow. The figure is a box of line in columns 5 to 10 and rows 2 to 7, filled with skin,
    with a grey button at (7, 4) and spill down column 4 beside it. The rest is the screen."""
    if x == 0:
        return NONE
    if x == 15:
        return EDGE
    if y >= 8:
        return SHADOW
    if 2 <= y <= 7:
        if x == 4:
            return SPILL
        if (x, y) == (7, 4):
            return GREY
        if x in (5, 10) or y in (2, 7):
            return LINE
        if 5 < x < 10:
            return SKIN
    return GREEN


DRAWINGS = {"screen": [[screen(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(colors=(GREEN_HEX,), tolerance=0, softness=0, match="rgb", shift=0):
    return {"drawing": "screen", "colors": list(colors), "tolerance": tolerance,
            "softness": softness, "match": match, "shift": shift}


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    held = lambda k: min(MAX, max(0.0, value_at(c[k], frame_no)))  # noqa: E731
    return R.frame(color_key(pixels, c["colors"], held("tolerance"), held("softness"),
                             c["match"]), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(colors=(), shift=c["shift"]), 0)


CASES = {
    "FX-KEY-001": ("The screen's green chosen, tolerance 0: the screen goes, its half-covering "
                   "right edge too; the shadow, the spill and the figure stay.",
                   case(), [0]),
    "FX-KEY-002": ("Tolerance 80: the shadow, 77 from the green on its green channel, goes too; "
                   "the spill, 90 from it on its red, stays.",
                   case(tolerance=80), [0]),
    "FX-KEY-003": ("Tolerance 60, softness 40: the shadow, 17 into the band, keeps 17/40 of its "
                   "colour and covering, and the spill, 30 into it, keeps 30/40; the figure, far "
                   "past the band, stays whole.",
                   case(tolerance=60, softness=40), [0]),
    "FX-KEY-004": ("Match hue, tolerance 10: the shadow, the same hue as the screen, goes with "
                   "it; the spill, 13 degrees of hue away (18.6 of 255), stays, and so does the "
                   "figure and its grey button.",
                   case(tolerance=10, match="hue"), [0]),
    "FX-KEY-005": ("Match hue, tolerance 10, softness 20: the spill keeps (18.6 - 10) / 20 of "
                   "itself.",
                   case(tolerance=10, softness=20, match="hue"), [0]),
    "FX-KEY-006": ("No colour chosen: the drawing, untouched.",
                   case(colors=()), [0]),
    "FX-KEY-007": ("No colour chosen, match hue, tolerance 255: the drawing, untouched.",
                   case(colors=(), tolerance=255, match="hue"), [0]),
    "FX-KEY-008": ("Tolerance keyed from 0 at frame 0 to 100 at frame 4, linear: frames 0 and "
                   "2, tolerance 0 and 50, are FX-KEY-001; frame 4, tolerance 100, takes the "
                   "shadow and the spill too.",
                   case(tolerance=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-KEY-009": ("Tolerance 60, softness keyed from 0 at frame 0 to 100 at frame 4: frame 0 "
                   "keeps the shadow and the spill whole; frame 4 keeps 17/100 of the shadow "
                   "and 30/100 of the spill, and the band now reaches the line, 151 from the "
                   "green, which keeps 91/100, and the button, 128 from it, 68/100.",
                   case(tolerance=60, softness=keyed((0, 0), (4, 100))), [0, 4]),
    "FX-KEY-010": ("The green and the skin chosen: the screen and the figure's skin go; the "
                   "line, the button, the spill and the shadow stay.",
                   case(colors=[GREEN_HEX, SKIN_HEX]), [0]),
    "FX-KEY-011": ("FX-KEY-001 with the colour written in capitals: the same.",
                   case(colors=[GREEN_HEX.upper()]), [0]),
    "FX-KEY-012": ("The grey chosen, match rgb: only the button goes.",
                   case(colors=[GREY_HEX]), [0]),
    "FX-KEY-013": ("The grey chosen, match hue, tolerance 254: a grey is 255 from everything, "
                   "so nothing goes.",
                   case(colors=[GREY_HEX], tolerance=254, match="hue"), [0]),
    "FX-KEY-014": ("The grey chosen, match hue, tolerance 255: everything is within 255, so "
                   "the frame is empty.",
                   case(colors=[GREY_HEX], tolerance=255, match="hue"), [0]),
    "FX-KEY-015": ("FX-KEY-003 moved three pixels right: the same, moved.",
                   case(tolerance=60, softness=40, shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-KEY-016": ("Tolerance 256, above 255.", case(tolerance=256)),
    "FX-KEY-017": ("Tolerance -1, below 0.", case(tolerance=-1)),
    "FX-KEY-018": ("Softness 256, above 255.", case(softness=256)),
    "FX-KEY-019": ("Softness keyed to 300 at frame 4.", case(softness=keyed((0, 0), (4, 300)))),
    "FX-KEY-020": ("Nine colours, one more than eight.",
                   case(colors=[f"#0000{i:02x}" for i in range(9)])),
    "FX-KEY-021": ("A colour written \"#12345\", one digit short.",
                   case(colors=[GREEN_HEX, "#12345"])),
    "FX-KEY-022": ("Match \"hsv\", which is not a choice.", case(match="hsv")),
    "FX-KEY-023": ("Match \"RGB\", in capitals, which is kept as written and is not the word.",
                   case(match="RGB")),
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
        "instance_id": "fx-0-0", "type_id": "core.color_key", "enabled": True,
        "parameters": {k: setting_json(c[k])
                       for k in ("colors", "tolerance", "softness", "match")}}]
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

    (OUT / "expected_color_key.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    empty = [[0.0] * 4] * (W * H)
    at = lambda x, y: y * W + x  # noqa: E731
    gone = lambda f, x, y: f[at(x, y)] == [0.0] * 4  # noqa: E731
    same = lambda f, x, y: f[at(x, y)] == drawn[at(x, y)]  # noqa: E731
    part = lambda f, x, y, k: all(abs(f[at(x, y)][i] - drawn[at(x, y)][i] * k) < 1e-15  # noqa: E731
                                  for i in range(4))
    shadow, spill, skin, line, button, edge = (3, 9), (4, 4), (6, 3), (5, 3), (7, 4), (15, 4)

    # The rule's own pieces.
    assert abs(hue(GREEN[:3]) - hue(SHADOW[:3])) < 0.1
    assert abs(distance(SPILL[:3], GREEN[:3], "hue") - 18.6) < 0.05
    assert distance(GREY[:3], GREEN[:3], "hue") == 255 == distance(GREEN[:3], GREY[:3], "hue")
    assert hue((255, 0, 0)) == 0 and hue((255, 0, 1)) > 359 and hue((0, 0, 255)) == 240
    assert distance((255, 0, 1), (255, 1, 0), "hue") < 1  # across 0 degrees, the short way

    one = c["FX-KEY-001"]["0"]
    for y in range(H):
        for x in range(W):
            want_gone = drawn[at(x, y)][3] > 0 and DRAWINGS["screen"][y][x][:3] == GREEN[:3]
            assert gone(one, x, y) if want_gone else same(one, x, y), (x, y)
    assert gone(one, *edge) and same(one, *shadow) and same(one, *spill)
    two = c["FX-KEY-002"]["0"]
    assert gone(two, *shadow) and same(two, *spill) and same(two, *skin)
    three = c["FX-KEY-003"]["0"]
    assert gone(three, 1, 1) and part(three, *shadow, 17 / 40) and part(three, *spill, 30 / 40)
    assert same(three, *skin) and same(three, *line) and same(three, *button)
    four = c["FX-KEY-004"]["0"]
    assert gone(four, 1, 1) and gone(four, *shadow) and gone(four, *edge)
    assert same(four, *spill) and same(four, *skin) and same(four, *button)
    five = c["FX-KEY-005"]["0"]
    k = (distance(SPILL[:3], GREEN[:3], "hue") - 10) / 20
    assert part(five, *spill, k) and gone(five, *shadow) and same(five, *skin)
    assert c["FX-KEY-006"]["0"] == drawn and c["FX-KEY-007"]["0"] == drawn
    eight = c["FX-KEY-008"]
    assert eight["0"] == one and eight["2"] == one
    assert gone(eight["4"], *shadow) and gone(eight["4"], *spill) and same(eight["4"], *skin)
    nine = c["FX-KEY-009"]
    assert same(nine["0"], *shadow) and same(nine["0"], *spill) and gone(nine["0"], 1, 1)
    assert part(nine["4"], *shadow, 17 / 100) and part(nine["4"], *spill, 30 / 100)
    assert part(nine["4"], *line, 91 / 100) and part(nine["4"], *button, 68 / 100)
    assert same(nine["4"], *skin)
    ten = c["FX-KEY-010"]["0"]
    assert gone(ten, 1, 1) and gone(ten, *skin) and same(ten, *line) and same(ten, *button)
    assert same(ten, *spill) and same(ten, *shadow)
    assert c["FX-KEY-011"]["0"] == one
    twelve = c["FX-KEY-012"]["0"]
    assert sum(twelve[i] != drawn[i] for i in range(W * H)) == 1 and gone(twelve, *button)
    assert c["FX-KEY-013"]["0"] == drawn
    assert c["FX-KEY-014"]["0"] == empty
    moved = c["FX-KEY-015"]["0"]
    assert moved == c["FX-KEY-015"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == three[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
