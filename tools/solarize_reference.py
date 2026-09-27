"""Solarize, worked a second way.

D-142 adds `core.solarize`. It turns the bright part of each colour channel over, as a
photograph flashed with light in the darkroom does: a channel at or above the threshold is
inverted, one below it is kept, so the lights go dark and strange while the darks stay as they
are. `threshold` is one number, 0 to 255, starting at 128, on the 8-bit scale of the colour as
it is written. Each channel is decided on its own, so a colour whose red is above the threshold
and whose blue is below changes hue. At 0 every channel of every shown pixel is inverted, as
Invert does; at 255 only channels already at full are, so white turns black. The covering is
kept, so a soft edge stays soft, and a pixel that does not show stays as it is. It is this
program's own method, modelled on After Effects' Solarize; nothing is ported. Document 21 is the
rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. At a pixel with covering a > 0, with e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) its
encoded straight colour, per channel e'_c = 1 - e_c when 255 e_c + 1e-4 >= threshold, else e_c,
and the output is (srgb_to_linear(e'_c) * a, a). The 1e-4 decides an 8-bit value sitting exactly
on the threshold the same way in the build and here: it is inverted. The choice is a cliff, so
the check below asserts that no channel it decides sits within 1e-5 of it. A channel that is
kept is the input's own value, exactly. The layer does not grow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: bands of colour, drawn below. The drawing goes into `Fixtures/solarize/media`,
the projects into `Fixtures/solarize`, and the expected frames into
`Fixtures/solarize/expected_solarize.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/solarize_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "solarize"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGE = (0, 255)
START = 128
STEP = 1e-4   # added before comparing, so an 8-bit value on the threshold is inverted
CLIFF = 1e-5  # no decided channel may sit nearer the cliff than this


# --- the rule -------------------------------------------------------------------------------

def flips(e, threshold):
    """Whether an encoded channel value is at or above the threshold, and so inverted."""
    return 255 * e + STEP >= threshold


def solarize(pixels, threshold):
    out = [R.working(p) for p in pixels]
    for w in out:
        a = w[3]
        if a > 0:
            for c in range(3):
                e = S.linear_to_srgb(min(1.0, max(0.0, w[c] / a)))
                if flips(e, threshold):
                    w[c] = srgb_to_linear(1 - e) * a
    return out


def margin(pixels, threshold):
    """The nearest any shown channel comes to the cliff, in 8-bit steps."""
    return min((abs(255 * S.linear_to_srgb(min(1.0, max(0.0, w[c] / w[3]))) + STEP - threshold)
                for w in (R.working(p) for p in pixels) if w[3] > 0 for c in range(3)),
               default=float("inf"))


# --- the drawing ----------------------------------------------------------------------------

BLACK = (0, 0, 0, 255)
LINE = R.LINE                    # #1e1a24, the line
DARK = (64, 64, 64, 255)         # #404040: on the step at threshold 64
BELOW = (127, 127, 127, 255)     # #7f7f7f: one below the starting threshold, kept
GREY = (128, 128, 128, 255)      # #808080: on the starting threshold, inverted
LIGHT = (192, 192, 192, 255)     # #c0c0c0
SKIN = R.SKIN                    # #f6d6be: red and green above 200, blue below
TRACE = R.TRACE                  # #c82828: a red colour-trace line, red on the step at 200
BLUE = (58, 90, 154, 255)        # #3a5a9a: a night blue, one channel each side of 128
WHITE = (255, 255, 255, 255)
SOFT = (246, 214, 190, 128)      # the skin at half covering, a soft edge
SOFT_GREY = (128, 128, 128, 128)  # the grey at half covering, on the starting threshold
NONE = S.NONE
BANDS = [NONE, BLACK, LINE, DARK, BELOW, GREY, LIGHT, SKIN, TRACE, BLUE, WHITE, SOFT, SOFT_GREY,
         NONE, NONE, NONE]


def bands(x, y):
    """Rows 0 and 9 and columns 0 and 13 to 15 are empty. Between, one column each: black 1,
    line 2, dark grey 3, #7f7f7f 4, grey 5, light grey 6, skin 7, red trace 8, night blue 9,
    white 10, the skin at half covering 11 and the grey at half covering 12."""
    return NONE if y in (0, 9) else BANDS[x]


DRAWINGS = {"bands": [[bands(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(threshold=START, shift=0):
    return {"drawing": "bands", "threshold": threshold, "shift": shift}


def held(c, frame_no):
    return min(RANGE[1], max(RANGE[0], value_at(c["threshold"], frame_no)))


def drawn_pixels(c):
    return [p for row in DRAWINGS[c["drawing"]] for p in row]


def render(c, frame_no):
    return R.frame(solarize(drawn_pixels(c), held(c, frame_no)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return R.frame([R.working(p) for p in drawn_pixels(c)], c["shift"])


CASES = {
    "FX-SOLAR-001": ("Threshold 128, as it starts: every channel at 128 or above is turned over. "
                     "The grey #808080, on the threshold, turns #7f7f7f, while #7f7f7f beside it, "
                     "one below, stays; the light grey turns #3f3f3f and the white black; the "
                     "skin turns a deep blue #092941 (red, green and blue all above 128); the red "
                     "trace keeps its green and blue and its red falls to 55; the night blue "
                     "keeps its red and green and its blue falls to 101; black, the line and the "
                     "dark grey, all below, stay as they are.",
                     case(), [0]),
    "FX-SOLAR-002": ("Threshold 0: every channel of every shown pixel is turned over, as Invert "
                     "does: black turns white, white black, the skin #092941.",
                     case(threshold=0), [0]),
    "FX-SOLAR-003": ("Threshold 255: only a channel already at full is turned over, so the white "
                     "turns black and every other pixel stays as it is.",
                     case(threshold=255), [0]),
    "FX-SOLAR-004": ("Threshold 1: every channel above 0 is turned over, so black, all at 0, "
                     "stays black, while every other shown pixel is inverted as in "
                     "FX-SOLAR-002.",
                     case(threshold=1), [0]),
    "FX-SOLAR-005": ("Threshold 64: the dark grey, on the threshold, turns #bfbfbf; the line, "
                     "below it, stays; the night blue keeps its red 58 and turns over its green "
                     "and blue, to #3aa565.",
                     case(threshold=64), [0]),
    "FX-SOLAR-006": ("Threshold 129: the grey #808080 and its soft edge, now one below, stay as "
                     "they are; the light grey, the skin and the white are turned over as in "
                     "FX-SOLAR-001.",
                     case(threshold=129), [0]),
    "FX-SOLAR-007": ("Threshold 200: the skin turns over its red and green and keeps its blue "
                     "190, to #0929be, a blue-purple; the red trace's red, on the threshold, "
                     "falls to 55; the light grey, 192, stays; the white turns black.",
                     case(threshold=200), [0]),
    "FX-SOLAR-008": ("Threshold 127.5, between two 8-bit values: the same as 128, "
                     "FX-SOLAR-001, as no written value lies between them.",
                     case(threshold=127.5), [0]),
    "FX-SOLAR-009": ("Threshold keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is "
                     "FX-SOLAR-002, frame 2 (127.5) FX-SOLAR-001 and frame 4 FX-SOLAR-003.",
                     case(threshold=keyed((0, 0), (4, 255))), [0, 2, 4]),
    "FX-SOLAR-010": ("Threshold keyed 64 at frame 0, held, then 200 at frame 4: frames 0 and 2 "
                     "are FX-SOLAR-005 and frame 4 FX-SOLAR-007.",
                     case(threshold=keyed((0, 64, "hold"), (4, 200))), [0, 2, 4]),
    "FX-SOLAR-011": ("Threshold eased from 128 at frame 0 to 255 at frame 4 on a curve that "
                     "overshoots: at frame 2 it has gone past 255 and is held there, so frames "
                     "2 and 4 are both FX-SOLAR-003.",
                     case(threshold=keyed((0, 128, OVERSHOOT), (4, 255))), [0, 2, 4]),
    "FX-SOLAR-012": ("FX-SOLAR-001 moved three pixels right: the same, moved.",
                     case(shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-SOLAR-013": ("Threshold -1: below 0.", case(threshold=-1)),
    "FX-SOLAR-014": ("Threshold 255.5: above 255.", case(threshold=255.5)),
    "FX-SOLAR-015": ("Threshold keyed to 300 at frame 4.",
                     case(threshold=keyed((0, 128), (4, 300)))),
    "FX-SOLAR-016": ("Threshold keyed from -50 at frame 0.",
                     case(threshold=keyed((0, -50), (4, 128)))),
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
        "instance_id": "fx-0-0", "type_id": "core.solarize", "enabled": True,
        "parameters": {"threshold": setting_json(c["threshold"])}}]
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

    (OUT / "expected_solarize.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    lin = lambda *e8: [srgb_to_linear(v / 255) for v in e8] + [1.0]  # noqa: E731
    col = {q: (BANDS.index(q), 4) for q in BANDS[1:13]}
    shown = list(col)
    inv = lambda q: lin(*[255 - v for v in q[:3]])  # noqa: E731
    same = lambda f, *qs: all(f[at(*col[q])] == drawn[at(*col[q])] for q in qs)  # noqa: E731
    got = lambda f, q: f[at(*col[q])]  # noqa: E731
    empty = [(0, 4), (13, 4), (15, 4), (4, 0), (4, 9)]

    # The rule's own pieces: on the threshold is inverted, one below is not; at 0 everything is,
    # at 255 only a full channel; a channel that is kept is the input's own value, exactly.
    assert flips(128 / 255, 128) and not flips(127 / 255, 128)
    assert flips(0, 0) and flips(1, 255) and not flips(254 / 255, 255)
    assert flips(128 / 255, 127.5) and not flips(127 / 255, 127.5)
    assert solarize([GREY], 129)[0] == R.working(GREY)

    # The cliff: no channel any valid case decides sits within 1e-5 of the threshold.
    for fx, (_, cs, frames) in CASES.items():
        for f in frames:
            assert margin(drawn_pixels(cs), held(cs, f)) >= CLIFF, (fx, f)
    assert abs(margin([GREY], 128) - STEP) < 1e-9  # the nearest: an 8-bit value on the step

    # A pixel that does not show, and the covering, are kept by every case; the soft skin and
    # the soft grey are turned as the skin and the grey, at half covering.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-SOLAR-012" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)
            if fx != "FX-SOLAR-012":
                for soft, full in ((SOFT, SKIN), (SOFT_GREY, GREY)):
                    assert near([v / (128 / 255) for v in got(px, soft)[:3]], got(px, full)[:3])

    one = c["FX-SOLAR-001"]["0"]
    assert near(got(one, GREY), lin(127, 127, 127)) and same(one, BELOW)
    assert near(got(one, LIGHT), lin(63, 63, 63)) and near(got(one, WHITE), lin(0, 0, 0))
    assert near(got(one, SKIN), lin(9, 41, 65)) and near(got(one, TRACE), lin(55, 40, 40))
    assert got(one, TRACE)[1:] == drawn[at(*col[TRACE])][1:]
    assert near(got(one, BLUE), lin(58, 90, 101)) and same(one, BLACK, LINE, DARK)
    two = c["FX-SOLAR-002"]["0"]
    assert all(near(got(two, q), inv(q)) for q in shown if q[3] == 255)
    assert near(got(two, BLACK), lin(255, 255, 255)) and near(got(two, SKIN), lin(9, 41, 65))
    three = c["FX-SOLAR-003"]["0"]
    assert near(got(three, WHITE), lin(0, 0, 0))
    assert same(three, *[q for q in shown if q != WHITE])
    four = c["FX-SOLAR-004"]["0"]
    assert same(four, BLACK) and all(near(got(four, q), got(two, q)) for q in shown if q != BLACK)
    five = c["FX-SOLAR-005"]["0"]
    assert near(got(five, DARK), lin(191, 191, 191)) and same(five, LINE, BLACK)
    assert near(got(five, BLUE), lin(58, 165, 101))
    six = c["FX-SOLAR-006"]["0"]
    assert same(six, GREY, SOFT_GREY, BELOW)
    assert all(got(six, q) == got(one, q) for q in (LIGHT, SKIN, WHITE, TRACE, BLUE, SOFT))
    seven = c["FX-SOLAR-007"]["0"]
    assert near(got(seven, SKIN), lin(9, 41, 190)) and near(got(seven, TRACE), lin(55, 40, 40))
    assert same(seven, LIGHT, GREY, BLUE) and near(got(seven, WHITE), lin(0, 0, 0))
    assert c["FX-SOLAR-008"]["0"] == one
    nine = c["FX-SOLAR-009"]
    assert nine["0"] == two and nine["2"] == one and nine["4"] == three
    assert like(nine["2"], render(case(threshold=127.5), 0))
    ten = c["FX-SOLAR-010"]
    assert ten["0"] == five and ten["2"] == five and ten["4"] == seven
    eleven = c["FX-SOLAR-011"]
    assert ease(OVERSHOOT, 0.5) > 1 and value_at(case(threshold=keyed(
        (0, 128, OVERSHOOT), (4, 255)))["threshold"], 2) > 255
    assert eleven["0"] == one and eleven["2"] == three and eleven["4"] == three
    moved = c["FX-SOLAR-012"]["0"]
    assert moved == c["FX-SOLAR-012"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == one[at(0, y):at(W - 3, y)]
    for name, frames in c.items():
        if name != "FX-SOLAR-012":
            for px in frames.values():
                assert all(px[at(*xy)] == [0.0] * 4 for xy in empty)
    print("checked")


if __name__ == "__main__":
    main()
