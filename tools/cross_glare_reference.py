"""Cross glare, worked a second way.

D-161 adds `core.cross_glare`, star glints: each bright point of a drawing is streaked in a
cross or a star, as light catches a blade, an eye or a jewel in an anime shot. The streaks run
out from every bright pixel along `points` directions, evenly spread round the circle from
`angle`, fading to nothing `length` pixels out, and are added on top of the drawing, tinted by a
colour. It is this program's own method, modelled on the cross filter of anime compositing and
star-glint effects such as Trapcode Starglow in spirit, not claimed to match any; it takes its
light from Light Rays' (D-124) and nothing is ported. Document 21 is the rule in words; this
file is the reference for the numbers document 25 pins against it.

The rule. (1) The light Lt: each pixel that shows, whose brightest channel in 8-bit steps is at
least the threshold's share of 255, exactly as D-89's glow tests it (`glow_reference.glows`),
as it is; every other pixel gives nothing. (2) L = floor(length). With L 0 or intensity 0 the
output is the input and nothing grows. Otherwise the input first grows by L transparent pixels
on every side. The streaks' directions are v_j = u(angle + 360 j / n), j = 0 to n - 1,
n = floor(points), u(0) up and u(90) right, exact at quarter turns; the weights are
w_t = (1 - t / (L + 1))^2 for t = 1 to L, and W their sum. The glare at a pixel with centre P is
G(P) = sum over j and t of (w_t / W) bilinear(Lt, P - t v_j), transparent outside, so each
bright pixel's light shows t pixels along each v_j, fading with t; a streak that runs between
pixel centres, as a diagonal does, is spread over the pixels either side of its line.
(3) With C the colour's linear value and O the grown input, the output is O + intensity C G in
red, green and blue, and min(1, O.a + intensity G.a) in covering (Light Rays' step (3)); the
colour is not cut off at white. `length` is a distance, scaled for a draft.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says, drawn below. The drawing goes into `Fixtures/cross_glare/media`, the projects into
`Fixtures/cross_glare`, and the expected frames into
`Fixtures/cross_glare/expected_cross_glare.json`. A layer that grows is shown with its grown
pixels in place.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/cross_glare_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import glow_reference as G  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
from drop_shadow_reference import unit  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "cross_glare"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"threshold": (0, 100), "length": (0, 1000), "points": (1, 8),
          "angle": (-3600, 3600), "intensity": (0, 10)}
NUMBERS = ("threshold", "length", "points", "angle", "intensity")
NAMES = NUMBERS + ("color",)
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def light_of(pixels, size, threshold):
    """Lt as a layer in the drawing's own space: D-89's bright test, the pixel as it is."""
    w, h = size
    bright = {"based_on": "bright"}
    return {"px": [R.working(p) if G.glows(p, bright, threshold, 0) else EMPTY for p in pixels],
            "left": 0, "top": 0, "w": w, "h": h}


def streaks(points, angle):
    n = math.floor(points)
    return [unit(angle + 360 * j / n) for j in range(n)]


def weights(L):
    w = [(1 - t / (L + 1)) ** 2 for t in range(1, L + 1)]
    total = sum(w)
    return [v / total for v in w]


def glare(light, dirs, wts, X, Y):
    """G at the point (X, Y) of the drawing's own space."""
    out = [0.0] * 4
    for vx, vy in dirs:
        for t, wt in enumerate(wts, 1):
            s = bilinear(light, X - t * vx, Y - t * vy)
            if s[3]:
                for ch in range(4):
                    out[ch] += wt * s[ch]
    return out


def cross_glare(pixels, threshold, length, points, angle, intensity, color, size=(W, H)):
    """A function giving the output at pixel (x, y) of the drawing's own space, a grown pixel
    having negative coordinates or coordinates past the size; empty outside the grown layer.
    `pixels` is the drawing's 8-bit straight pixels, row by row, `size` its own w by h."""
    w, h = size
    working = [R.working(p) for p in pixels]
    inside = lambda x, y: 0 <= x < w and 0 <= y < h  # noqa: E731
    L = math.floor(length)
    if L == 0 or intensity == 0:
        return lambda x, y: working[y * w + x] if inside(x, y) else EMPTY
    light = light_of(pixels, size, threshold)
    dirs, wts = streaks(points, angle), weights(L)
    tint = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]

    def at(x, y):
        if not (-L <= x < w + L and -L <= y < h + L):
            return EMPTY
        o = working[y * w + x] if inside(x, y) else EMPTY
        g = glare(light, dirs, wts, x + 0.5, y + 0.5)
        return [o[ch] + intensity * tint[ch] * g[ch] for ch in range(3)] \
            + [min(1.0, o[3] + intensity * g[3])]
    return at


# --- the drawing ----------------------------------------------------------------------------

BRIGHT, EDGE, DARK = G.BRIGHT, G.EDGE, G.DARK  # yellow 98 %, brown exactly 60 %, purple 43 %
SOFT = BRIGHT[:3] + (128,)                     # the yellow at half covering, a soft edge
NONE = S.NONE
YELLOW, SOFT_AT, BROWN_AT = (2, 4), (12, 2), (12, 7)


def sparks(x, y):
    """A yellow point at (2, 4), near the left edge; a purple bar down column 5, rows 1 to 8,
    which the streaks cross; the yellow at half covering, a soft point, at (12, 2); a brown
    point at (12, 7). Everything else is empty."""
    if (x, y) == YELLOW:
        return BRIGHT
    if (x, y) == SOFT_AT:
        return SOFT
    if (x, y) == BROWN_AT:
        return EDGE
    if x == 5 and 1 <= y <= 8:
        return DARK
    return NONE


DRAWINGS = {"sparks": [[sparks(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(threshold=80, length=40, points=4, angle=45, intensity=1, color="#ffffff", shift=0):
    return {"drawing": "sparks", "threshold": threshold, "length": length, "points": points,
            "angle": angle, "intensity": intensity, "color": color, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    out = cross_glare(pixels, *[held(c, k, frame_no) for k in NUMBERS], c["color"])
    return [out(x - c["shift"], y) for y in range(H) for x in range(W)]


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(intensity=0, shift=c["shift"]), 0)


SHORT = {"length": 4}  # streaks four pixels long, strong enough to read on a small drawing

CASES = {
    "FX-GLARE-001": ("The settings as they start: threshold 80, length 40, points 4, angle 45, "
                     "intensity 1, white. The yellow, 98 %, and the soft yellow are bright "
                     "enough; the brown, 60 %, and the purple give no light. Each bright point "
                     "is streaked along both diagonals, an X, faint as the light is shared over "
                     "forty steps, running on past the frame; the soft point's streaks are half "
                     "as strong. The diagonals run between pixel centres, so each streak is "
                     "spread a little onto the pixels beside its line, and the yellow even "
                     "takes a little of its own light; pixels well off the diagonals stay as "
                     "they were.",
                     case(), [0]),
    "FX-GLARE-002": ("Intensity 0: the drawing, untouched, and nothing grows.",
                     case(intensity=0), [0]),
    "FX-GLARE-003": ("Length 0.9, floored to 0: no streaks, the drawing untouched.",
                     case(length=0.9), [0]),
    "FX-GLARE-004": ("Threshold 100: nothing is that bright, so the drawing is untouched.",
                     case(threshold=100), [0]),
    "FX-GLARE-005": ("Length 4: the X four pixels long each way, strongest next to the point: "
                     "the yellow's up-right streak crosses the purple bar at row 1, and "
                     "pixels further than four steps along a diagonal stay empty.",
                     case(**SHORT), [0]),
    "FX-GLARE-006": ("FX-GLARE-005 at threshold 60: the brown, at exactly 60 %, glints too.",
                     case(threshold=60, **SHORT), [0]),
    "FX-GLARE-007": ("Points 1: one streak from each point, up and to the right, and none the "
                     "other ways.",
                     case(points=1, **SHORT), [0]),
    "FX-GLARE-008": ("Points 2: a single line through each point, up-right and down-left.",
                     case(points=2, **SHORT), [0]),
    "FX-GLARE-009": ("Angle 0: a plus, the streaks straight up, right, down and left, each "
                     "landing on whole pixels: the pixel next to the yellow takes 0.8 squared "
                     "over 1.2 of its light, and the purple three to its right takes 0.4 "
                     "squared over 1.2 of it. The diagonal neighbours stay empty.",
                     case(angle=0, **SHORT), [0]),
    "FX-GLARE-010": ("Angle 90: a quarter turn gives the same four streaks, so it is "
                     "FX-GLARE-009.",
                     case(angle=90, **SHORT), [0]),
    "FX-GLARE-011": ("Angle 405, a whole turn past 45: FX-GLARE-005 exactly.",
                     case(angle=405, **SHORT), [0]),
    "FX-GLARE-012": ("Points 6: a six-pointed star, streaks every 60 degrees from 45.",
                     case(points=6, **SHORT), [0]),
    "FX-GLARE-013": ("Points 4.9, floored to 4: FX-GLARE-005 exactly.",
                     case(points=4.9, **SHORT), [0]),
    "FX-GLARE-014": ("Intensity 2.5: two and a half times FX-GLARE-005's streaks, not cut off "
                     "at white; the covering stops at full, so the purple and yellow "
                     "pixels the streaks cross keep a covering of 1.",
                     case(intensity=2.5, **SHORT), [0]),
    "FX-GLARE-015": ("Colour #ff8000, orange: FX-GLARE-005's streaks tinted, their red as "
                     "before, their green a fifth and their blue gone; the covering as "
                     "FX-GLARE-005's.",
                     case(color="#ff8000", **SHORT), [0]),
    "FX-GLARE-016": ("FX-GLARE-015 with its colour written in capitals, #FF8000: the same.",
                     case(color="#FF8000", **SHORT), [0]),
    "FX-GLARE-017": ("Colour #000000, black: the streaks add covering but no colour, a dark "
                     "cross; the solid pixels they cross keep their colour and the empty ones "
                     "turn black.",
                     case(color="#000000", **SHORT), [0]),
    "FX-GLARE-018": ("Length keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is the "
                     "drawing, frame 2 is FX-GLARE-005, frame 4 is length 8.",
                     case(length=keyed((0, 0), (4, 8))), [0, 2, 4]),
    "FX-GLARE-019": ("Length 4, points keyed from 1 at frame 0 to 8 at frame 4, linear: frame "
                     "0 is FX-GLARE-007, frame 2, at 4.5, is floored to 4 and is FX-GLARE-005, "
                     "frame 4 is an eight-pointed star.",
                     case(points=keyed((0, 1), (4, 8)), **SHORT), [0, 2, 4]),
    "FX-GLARE-020": ("Length 4, intensity eased from 0 at frame 0 to 10 at frame 4 on a curve "
                     "that overshoots: at frame 2 it would pass 10, is held at 10, and is "
                     "intensity 10 plain, as frame 4 is.",
                     case(intensity=keyed((0, 0, OVERSHOOT), (4, 10)), **SHORT), [0, 2, 4]),
    "FX-GLARE-021": ("FX-GLARE-009 moved three pixels right: the layer grew by 4, and the "
                     "yellow's left streak, which runs two pixels past the drawing's left edge, "
                     "shows in the two grown columns left of the drawing; column 0, past the "
                     "streak's end, stays empty.",
                     case(angle=0, shift=3, **SHORT), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-GLARE-022": ("Threshold 101, above 100.", case(threshold=101)),
    "FX-GLARE-023": ("Length 1001, above 1000.", case(length=1001)),
    "FX-GLARE-024": ("Points 0, below 1.", case(points=0)),
    "FX-GLARE-025": ("Points 9, above 8.", case(points=9)),
    "FX-GLARE-026": ("Angle -3601, below -3600.", case(angle=-3601)),
    "FX-GLARE-027": ("Intensity keyed to 11 at frame 4.", case(intensity=keyed((0, 1), (4, 11)))),
    "FX-GLARE-028": ("A colour written \"#12345\", one digit short.", case(color="#12345")),
    "FX-GLARE-029": ("A colour written \"white\", a name, not #rrggbb.", case(color="white")),
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
        "instance_id": "fx-0-0", "type_id": "core.cross_glare", "enabled": True,
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

    (OUT / "expected_cross_glare.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    lit = lambda px, x, y: px[at(x, y)][3] > drawn[at(x, y)][3] + 1e-9  # noqa: E731
    same = lambda px, x, y: px[at(x, y)] == drawn[at(x, y)]  # noqa: E731
    yx, yy = YELLOW
    yellow = R.working(BRIGHT)

    # The rule's pieces: the bright test is D-89's; the weights fall to nothing and sum to one;
    # the streaks are exact at quarter turns and spread evenly.
    assert G.glows(EDGE, {"based_on": "bright"}, 60, 0)
    assert not G.glows(EDGE, {"based_on": "bright"}, 60.1, 0)
    assert G.glows(SOFT, {"based_on": "bright"}, 80, 0) and not G.glows(DARK, {"based_on": "bright"}, 80, 0)
    assert abs(sum(weights(40)) - 1) < 1e-12 and weights(1) == [1.0]
    assert near([weights(4)], [[0.64 / 1.2, 0.36 / 1.2, 0.16 / 1.2, 0.04 / 1.2]])
    assert streaks(4, 0) == [(0.0, -1.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 0.0)]
    assert streaks(4.9, 45) == streaks(4, 45) == streaks(4, 405) and len(streaks(8, 0)) == 8
    # Everywhere: nothing is taken away, the covering stays inside 0 to 1.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-GLARE-021" else 0))
        for px in frames.values():
            for p, q in zip(px, base):
                assert 0 <= p[3] <= 1 and p[3] >= q[3], fx
                assert all(p[ch] >= q[ch] - 1e-15 for ch in range(3)), fx

    # 001: the X along the diagonals, off-diagonal pixels untouched, the bright points
    # themselves unlit by their own light, the soft point half as strong.
    one = c["FX-GLARE-001"]["0"]
    for k in (1, 2):
        for sx, sy in ((1, -1), (-1, -1), (1, 1), (-1, 1)):
            assert lit(one, yx + k * sx, yy + k * sy), (k, sx, sy)
    assert lit(one, 6, 0) and lit(one, 11, 3) and lit(one, 15, 5)
    for x, y in ((6, 4), (8, 2), (2, 0), (2, 9), (0, 4), (12, 5), (15, 2)):
        assert same(one, x, y), (x, y)
    assert same(one, *BROWN_AT) and same(one, 5, 4)
    assert one[at(*YELLOW)][0] > drawn[at(*YELLOW)][0]  # a little of its own light
    assert near([one[at(SOFT_AT[0] + 1, SOFT_AT[1] + 1)]],
                [[v * 128 / 255 for v in one[at(yx + 1, yy + 1)]]])
    assert c["FX-GLARE-002"]["0"] == drawn and c["FX-GLARE-003"]["0"] == drawn
    assert c["FX-GLARE-004"]["0"] == drawn
    # 005: shorter, stronger near the point, nothing past four steps; the bar is crossed.
    five = c["FX-GLARE-005"]["0"]
    assert five[at(yx + 1, yy - 1)][3] > one[at(yx + 1, yy - 1)][3]
    assert five[at(5, 1)][0] > drawn[at(5, 1)][0]  # the opaque bar brightens
    assert lit(one, 6, 0) and same(five, 6, 0)
    assert all(same(five, x, y) for x in range(8, W) for y in range(6, H))  # brown not lit
    # 006: the brown glints too.
    six = c["FX-GLARE-006"]["0"]
    assert all(lit(six, BROWN_AT[0] + a, BROWN_AT[1] + b) for a in (-1, 1) for b in (-1, 1))
    # 007 and 008: one streak up-right; two, up-right and down-left.
    seven, eight = c["FX-GLARE-007"]["0"], c["FX-GLARE-008"]["0"]
    assert lit(seven, yx + 1, yy - 1) and not any(
        lit(seven, yx + a, yy + b) for a, b in ((-1, -1), (1, 1), (-1, 1)))
    assert lit(eight, yx + 1, yy - 1) and lit(eight, yx - 1, yy + 1)
    assert not lit(eight, yx - 1, yy - 1) and not lit(eight, yx + 1, yy + 1)
    # 009: the plus lands on whole pixels, with the weights exactly.
    nine = c["FX-GLARE-009"]["0"]
    assert near([nine[at(yx + 1, yy)]], [[0.64 / 1.2 * v for v in yellow]])
    purple = R.working(DARK)
    assert near([nine[at(5, 4)][:3]], [[purple[ch] + 0.16 / 1.2 * yellow[ch] for ch in range(3)]])
    assert all(lit(nine, x, yy) for x in (0, 1, 3, 4, 6)) and lit(nine, yx, 0) and lit(nine, yx, 8)
    assert same(nine, 7, yy) and same(nine, yx, 9) and same(nine, yx, yy)
    assert all(same(nine, yx + a, yy + b) for a in (-1, 1) for b in (-1, 1))
    # The soft point's streak is 128 / 255 of the solid one's, the same distance out.
    softp = nine[at(SOFT_AT[0] + 1, SOFT_AT[1])]
    assert near([softp], [[v * 128 / 255 for v in nine[at(yx + 1, yy)]]])
    assert near(c["FX-GLARE-010"]["0"], nine) and c["FX-GLARE-010"]["0"] != five
    assert c["FX-GLARE-011"]["0"] == five and c["FX-GLARE-013"]["0"] == five
    twelve = c["FX-GLARE-012"]["0"]
    assert twelve != five and lit(twelve, yx - 1, yy + 1) and not lit(five, yx - 2, yy)
    # 014: two and a half times the streaks, past white; the covering stops at full.
    fourteen = c["FX-GLARE-014"]["0"]
    for i in range(W * H):
        for ch in range(3):
            assert abs(fourteen[i][ch] - drawn[i][ch] - 2.5 * (five[i][ch] - drawn[i][ch])) < 1e-12
    assert any(v > 1 for p in fourteen for v in p[:3])
    assert fourteen[at(5, 1)][3] == fourteen[at(*YELLOW)][3] == 1.0
    assert fourteen[at(5, 1)][0] > five[at(5, 1)][0] > drawn[at(5, 1)][0]
    # 015 and 016: orange; 017: black.
    fifteen = c["FX-GLARE-015"]["0"]
    g = srgb_to_linear(128 / 255)
    for i in range(W * H):
        assert fifteen[i][0] == five[i][0] and fifteen[i][2] == drawn[i][2]
        assert fifteen[i][3] == five[i][3]
        assert abs(fifteen[i][1] - drawn[i][1] - g * (five[i][1] - drawn[i][1])) < 1e-12
    assert c["FX-GLARE-016"]["0"] == fifteen
    black = c["FX-GLARE-017"]["0"]
    assert all(black[i][:3] == drawn[i][:3] and black[i][3] == five[i][3] for i in range(W * H))
    assert black[at(yx + 1, yy - 1)][3] > 0 and black[at(yx + 1, yy - 1)][:3] == [0.0] * 3
    # The keyed cases meet the plain ones at their frames.
    k = c["FX-GLARE-018"]
    assert k["0"] == drawn and k["2"] == five and k["4"] == render(case(length=8), 0) != five
    k = c["FX-GLARE-019"]
    assert k["0"] == seven and k["2"] == five
    assert k["4"] == render(case(points=8, **SHORT), 0) and lit(k["4"], yx, yy - 1)
    k = c["FX-GLARE-020"]
    assert ease(OVERSHOOT, 0.5) > 1
    assert k["0"] == drawn and k["2"] == k["4"] == render(case(intensity=10, **SHORT), 0)
    # 021: FX-GLARE-009 moved, its left streak shown on the grown pixels.
    moved = c["FX-GLARE-021"]["0"]
    assert all(moved[at(x, y)] == nine[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert moved[at(2, yy)][3] > moved[at(1, yy)][3] > 0
    assert all(moved[at(0, y)] == EMPTY for y in range(H))
    assert all(moved[at(x, y)] == EMPTY for x in (1, 2) for y in range(H) if y != yy)
    print("checked")


if __name__ == "__main__":
    main()
