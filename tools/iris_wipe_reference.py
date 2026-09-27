"""Iris Wipe, worked a second way.

D-158 adds `core.iris_wipe`, the anime and cartoon ending: a circle closing on a point until the
picture is gone. `completion`, 0 to 100 starting at 0, is how far the wipe has gone: at 0 the
drawing is untouched, at 100 it is gone. `center`, per cent of the drawing's own width and height
as Radial Blur's centre is, starting at 50, 50, is the point the circle closes on. `feather`, 0
to 10000 pixels starting at 0, softens the circle's edge over that width. `invert`, "off" or
"on" (off), turns it around: on, a hole opens from the centre and grows until the picture is
gone. The wipe takes all four channels together, so a pixel is kept, gone, or on a feathered
edge faded as a whole. It is this program's own method, modelled on After Effects' Iris Wipe in
spirit and not claimed to match it; nothing is ported. Document 21 is the rule in words; this
file is the reference for the numbers document 25 pins against it.

The rule. At completion 0 the output is the input exactly; at 100 every pixel is transparent,
all four channels 0. Otherwise, with w, h the drawing's own size, c = (center_x / 100 * w,
center_y / 100 * h), R the largest distance from c to the corners (0, 0), (w, 0), (0, h), (w, h),
P a pixel's centre, d = |P - c|, cc = completion / 100 and f = feather: off,
r = (1 - cc)(R + f) - f / 2 and k = clamp((r - d) / f + 0.5, 0, 1), or with f = 0, 1 when
d <= r and 0 otherwise; on, r = cc (R + f) - f / 2 and k = clamp((d - r) / f + 0.5, 0, 1), or
with f = 0, 1 when d >= r and 0 otherwise. The output is p k, all four channels. The layer does
not grow; a draft scales the feather. With f = 0 the choice is a cliff: `check` asserts that no
pixel it decides sits within 1e-5 of the circle.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says, drawn below. The drawing goes into `Fixtures/iris_wipe/media`, the projects into
`Fixtures/iris_wipe`, and the expected frames into `Fixtures/iris_wipe/expected_iris_wipe.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/iris_wipe_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "iris_wipe"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"completion": (0, 100), "center": (-1000, 1000), "feather": (0, 10000)}
NAMES = ("completion", "center", "feather", "invert")
CLIFF = 1e-5  # no decided pixel may sit closer than this to the circle
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def circle(completion, center, feather, invert, w=W, h=H):
    """The centre c and the radius r of the wipe's circle, in the drawing's own pixels."""
    cx, cy = center[0] / 100 * w, center[1] / 100 * h
    big = max(math.hypot(cx - a, cy - b) for a in (0, w) for b in (0, h))
    cc = completion / 100
    r = ((1 - cc) if invert == "off" else cc) * (big + feather) - feather / 2
    return (cx, cy), r


def keep(completion, center, feather, invert, px, py, w=W, h=H):
    """k at the point (px, py) of the drawing's own pixels."""
    if completion == 0:
        return 1.0
    if completion == 100:
        return 0.0
    (cx, cy), r = circle(completion, center, feather, invert, w, h)
    d = math.hypot(px - cx, py - cy)
    inside = r - d if invert == "off" else d - r
    if feather > 0:
        return min(1.0, max(0.0, inside / feather + 0.5))
    return 1.0 if inside >= 0 else 0.0


def wiped(pixels, completion, center, feather, invert, w=W, h=H):
    """The drawing's working pixels, w by h, each times its k."""
    out = []
    for i, p in enumerate(pixels):
        k = keep(completion, center, feather, invert, i % w + 0.5, i // w + 0.5, w, h)
        out.append([v * k for v in p])
    return out


# --- the drawing ----------------------------------------------------------------------------

LINE, SKIN, SOFT_SKIN, NONE = R.LINE, R.SKIN, (246, 214, 190, 128), S.NONE
BAND = (58, 111, 216, 255)  # #3a6fd8, the ball's blue band


def cel(x, y):
    """Rows 1 to 8, columns 0 to 13: a box of line (column 0, rows 1 and 8) filled with skin and
    crossed by a blue band in rows 4 and 5. Column 14 is the skin at half covering, a soft edge;
    column 15 and rows 0 and 9 are empty. The middle, (8, 5), falls on the corner between four
    band pixels."""
    if x == 15 or y in (0, 9):
        return NONE
    if x == 14:
        return SOFT_SKIN
    if y in (4, 5):
        return BAND
    return LINE if x == 0 or y in (1, 8) else SKIN


DRAWINGS = {"cel": [[cel(x, y) for x in range(W)] for y in range(H)]}


def drawn_pixels(name):
    return [R.working(p) for row in DRAWINGS[name] for p in row]


# --- the cases ------------------------------------------------------------------------------

def case(completion=0, center=(50, 50), feather=0, invert="off", shift=0):
    return {"drawing": "cel", "completion": completion, "center": center, "feather": feather,
            "invert": invert, "shift": shift}


def settings(c, frame_no):
    held = {"invert": c["invert"]}
    for k in RANGES:
        lo, hi = RANGES[k]
        v = value_at(c[k], frame_no)
        held[k] = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def render(c, frame_no):
    s = settings(c, frame_no)
    return R.frame(wiped(drawn_pixels(c["drawing"]), s["completion"], s["center"], s["feather"],
                         s["invert"]), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return R.frame(drawn_pixels(c["drawing"]), c["shift"])


HALF = {"completion": 50}

CASES = {
    "FX-IRIS-001": ("The settings as they start: completion 0, centre 50, 50, feather 0, invert "
                    "off: the drawing, untouched.",
                    case(), [0]),
    "FX-IRIS-002": ("Completion 100: every pixel is transparent, all four channels 0.",
                    case(completion=100), [0]),
    "FX-IRIS-003": ("Completion 50: the circle about the middle, (8, 5), has closed to half "
                    "the distance to the farthest corner, a radius of 4.717. The 68 pixels "
                    "whose centres lie inside it are kept exactly, a round patch of skin and "
                    "band; every pixel outside it is transparent, the line box and the soft "
                    "column among them.",
                    case(**HALF), [0]),
    "FX-IRIS-004": ("Completion 50, invert on: a hole of the same radius, 4.717, opened from "
                    "the middle: the 68 pixels inside it are transparent and every other pixel "
                    "is kept exactly, so FX-IRIS-003 and this add up to the drawing.",
                    case(invert="on", **HALF), [0]),
    "FX-IRIS-005": ("Completion 25: the circle has closed a quarter of the way, to a radius of "
                    "7.075: the 128 pixels inside it are kept, and the 32 outside it, the "
                    "whole of columns 0 and 15 and the ends of rows 0, 1, 8 and 9, are gone.",
                    case(completion=25), [0]),
    "FX-IRIS-006": ("Completion 75: the circle has closed to a radius of 2.358, and the pixels "
                    "inside it are the four by four block of columns 6 to 9 and rows 3 to 6, "
                    "at this size a square, not yet a round shape; everything else is "
                    "transparent.",
                    case(completion=75), [0]),
    "FX-IRIS-007": ("Completion 25, invert on: a hole of radius 2.358, the same circle as "
                    "FX-IRIS-006's, so the same four by four block is transparent and every "
                    "other pixel kept: the two add up to the drawing.",
                    case(completion=25, invert="on"), [0]),
    "FX-IRIS-008": ("Completion 50, feather 4: the circle's edge is soft over 4 pixels about "
                    "the same radius, 4.717. Pixels within 2.717 of the middle are kept "
                    "exactly, those 6.717 or more away are gone, and between them each pixel is "
                    "faded as a whole, all four channels by one amount that falls with the "
                    "distance; the soft column's half covering is faded further.",
                    case(feather=4, **HALF), [0]),
    "FX-IRIS-009": ("Completion 50, feather 4, invert on: the soft hole, each pixel faded by "
                    "one minus FX-IRIS-008's amount, so the two add up to the drawing.",
                    case(feather=4, invert="on", **HALF), [0]),
    "FX-IRIS-010": ("Completion 50, feather 10000, the most: the edge is so wide that every "
                    "pixel is faded to within a thousandth of half, a gentle fade of the whole "
                    "drawing rather than a circle.",
                    case(feather=10000, **HALF), [0]),
    "FX-IRIS-011": ("Completion 0 with feather 4 and invert on: completion 0 is the input "
                    "exactly, whatever the other settings: the drawing, untouched.",
                    case(feather=4, invert="on"), [0]),
    "FX-IRIS-012": ("Completion 100 with feather 4 and invert on: completion 100 is every "
                    "pixel transparent, whatever the other settings.",
                    case(completion=100, feather=4, invert="on"), [0]),
    "FX-IRIS-013": ("Centre 25, 50, completion 50: the circle closes on (4, 5); its farthest "
                    "corner is 13 away, so its radius is 6.5, reaching past the drawing's left "
                    "edge, and every pixel from column 10 on is gone.",
                    case(center=(25, 50), **HALF), [0]),
    "FX-IRIS-014": ("Centre 100, 100, completion 50: the circle closes on the drawing's "
                    "bottom right corner, a quarter of it showing, radius 9.434: the pixels "
                    "near that corner are kept, the top left gone.",
                    case(center=(100, 100), **HALF), [0]),
    "FX-IRIS-015": ("Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is "
                    "the drawing, frame 1 FX-IRIS-005, frame 2 FX-IRIS-003, frame 3 "
                    "FX-IRIS-006, frame 4 FX-IRIS-002: the circle closes, each frame's kept "
                    "pixels among the last's.",
                    case(completion=keyed((0, 0), (4, 100))), [0, 1, 2, 3, 4]),
    "FX-IRIS-016": ("Completion 50, centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4, "
                    "linear: frame 0 is FX-IRIS-003, frame 2 is centre 37.5, 50, and frame 4 is "
                    "FX-IRIS-013.",
                    case(center=keyed((0, (50, 50)), (4, (25, 50))), **HALF), [0, 2, 4]),
    "FX-IRIS-017": ("Completion 50, feather keyed from 0 at frame 0 to 8 at frame 4, linear: "
                    "frame 0 is FX-IRIS-003, frame 2 is FX-IRIS-008, and frame 4 is feather 8.",
                    case(feather=keyed((0, 0), (4, 8)), **HALF), [0, 2, 4]),
    "FX-IRIS-018": ("Completion eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                    "overshoots: at frame 2 it would pass 100, is held at 100, and is "
                    "FX-IRIS-002, as frame 4 is; frame 0 is the drawing.",
                    case(completion=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-IRIS-019": ("FX-IRIS-003 moved three pixels right: the same, moved; the circle is "
                    "worked in the drawing's own space and moves with it, nothing grows, and "
                    "the three columns left of the drawing stay empty.",
                    case(shift=3, **HALF), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-IRIS-020": ("Completion -1, below 0.", case(completion=-1)),
    "FX-IRIS-021": ("Completion 101, above 100.", case(completion=101)),
    "FX-IRIS-022": ("Feather -1, below 0.", case(feather=-1, **HALF)),
    "FX-IRIS-023": ("Feather 10001, above 10000.", case(feather=10001, **HALF)),
    "FX-IRIS-024": ("Centre 1001, 50, past ten widths.", case(center=(1001, 50), **HALF)),
    "FX-IRIS-025": ("Invert \"yes\", not \"off\" or \"on\".", case(invert="yes", **HALF)),
    "FX-IRIS-026": ("Invert \"On\": the word is exact, so a capital is not it.",
                    case(invert="On", **HALF)),
    "FX-IRIS-027": ("Completion keyed to 150 at frame 4.",
                    case(completion=keyed((0, 0), (4, 150)))),
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
        "instance_id": "fx-0-0", "type_id": "core.iris_wipe", "enabled": True,
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

    (OUT / "expected_iris_wipe.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    everywhere = [(x, y) for y in range(H) for x in range(W)]
    dist = lambda x, y, cx=8, cy=5: math.hypot(x + 0.5 - cx, y + 0.5 - cy)  # noqa: E731
    gone = lambda px: [(x, y) for x, y in everywhere if px[at(x, y)] == EMPTY]  # noqa: E731

    def kept(px):
        return {(x, y) for x, y in everywhere if drawn[at(x, y)][3] > 0
                and px[at(x, y)] == drawn[at(x, y)]}

    def added(a, b):
        return all(close([u + v for u, v in zip(a[i], b[i])], drawn[i]) for i in range(W * H))

    # The rule's own pieces: R is the farthest corner; off and on share the circle at cc and
    # 1 - cc; completion 0 and 100 win over every other setting.
    big = math.sqrt(89)
    assert circle(50, (50, 50), 0, "off") == ((8, 5), big / 2)
    assert abs(circle(50, (50, 50), 4, "off")[1] - big / 2) < 1e-12
    assert abs(circle(50, (50, 50), 4, "on")[1] - big / 2) < 1e-12
    assert circle(25, (50, 50), 0, "on")[1] == circle(75, (50, 50), 0, "off")[1]
    assert circle(50, (25, 50), 0, "off") == ((4, 5), 6.5)
    assert abs(circle(50, (100, 100), 0, "off")[1] - math.sqrt(356) / 2) < 1e-12
    assert keep(0, (50, 50), 4, "on", 8, 5) == 1 and keep(100, (50, 50), 0, "on", 0, 0) == 0
    edge = circle(50, (0, 0), 0, "off")[1]  # a point exactly on the circle about (0, 0)
    assert keep(50, (0, 0), 0, "off", edge, 0) == 1 == keep(50, (0, 0), 0, "on", edge, 0)
    assert abs(keep(50, (0, 0), 4, "off", edge, 0) - 0.5) < 1e-12

    # Every case: each pixel is the drawing's times one k in 0..1 on all four channels, an empty
    # pixel stays empty, and with no feather no decided pixel sits within 1e-5 of the circle
    # (the cliff). The invalid ones are the drawing.
    for fx, frames in c.items():
        s = expected["cases"][fx]
        cs = CASES[fx][1] if fx in CASES else INVALID[fx][1]
        base = plain(cs)
        if "warning" in s:
            assert all(px == base for px in frames.values())
            continue
        for f, px in frames.items():
            for p, q in zip(px, base):
                if q[3] == 0:
                    assert p == EMPTY, fx
                else:
                    k = p[3] / q[3]
                    assert 0 <= k <= 1 and close(p, [v * k for v in q]), (fx, p, q)
            st = settings(cs, int(f))
            if st["feather"] == 0 and 0 < st["completion"] < 100:
                (cx, cy), r = circle(st["completion"], st["center"], 0, st["invert"])
                for x, y in everywhere:
                    assert abs(dist(x, y, cx, cy) - r) >= CLIFF, (fx, f, x, y)

    shown = kept(drawn)
    assert c["FX-IRIS-001"]["0"] == drawn
    assert gone(c["FX-IRIS-002"]["0"]) == everywhere
    three = c["FX-IRIS-003"]["0"]
    inner = {(x, y) for x, y in everywhere if dist(x, y) < big / 2}
    assert len(inner) == 68 and kept(three) == inner & shown
    assert set(gone(three)) >= set(everywhere) - inner
    assert all(three[at(x, y)] == EMPTY for x in (0, 14) for y in range(H))  # line box, soft
    four = c["FX-IRIS-004"]["0"]
    assert set(gone(four)) >= inner and kept(four) == shown - inner and added(three, four)
    five = c["FX-IRIS-005"]["0"]
    wide = {(x, y) for x, y in everywhere if dist(x, y) < 0.75 * big}
    assert len(wide) == 128 and kept(five) == wide & shown
    ends = ({(x, y) for x in (0, 15) for y in range(H)}
            | {(x, y) for x in (1, 14) for y in (0, 1, 8, 9)}
            | {(x, y) for x in (2, 13) for y in (0, 9)})
    assert set(everywhere) - wide == ends and len(ends) == 32
    assert all(five[at(x, y)] == EMPTY for x, y in ends)
    six = c["FX-IRIS-006"]["0"]
    block = {(x, y) for x in range(6, 10) for y in range(3, 7)}
    assert kept(six) == block and set(gone(six)) == set(everywhere) - block
    seven = c["FX-IRIS-007"]["0"]
    assert kept(seven) == shown - block and added(six, seven)
    eight = c["FX-IRIS-008"]["0"]
    last = None
    for x, y in sorted(everywhere, key=lambda p: dist(*p)):
        if drawn[at(x, y)][3] == 0:
            continue
        k = eight[at(x, y)][3] / drawn[at(x, y)][3]
        want = min(1, max(0, (big / 2 - dist(x, y)) / 4 + 0.5))
        assert abs(k - want) < 1e-12
        assert (k == 1) == (dist(x, y) <= big / 2 - 2) and (k == 0) == (dist(x, y) >= big / 2 + 2)
        assert last is None or k <= last + 1e-12
        last = k
    assert 0 < eight[at(14, 4)][3] < 128 / 255 * 0.1  # the soft column faded further
    assert any(0 < eight[i][3] < drawn[i][3] for i in range(W * H) if drawn[i][3] == 1)
    assert added(eight, c["FX-IRIS-009"]["0"])
    ten = c["FX-IRIS-010"]["0"]
    for i in range(W * H):
        if drawn[i][3] > 0:
            assert abs(ten[i][3] / drawn[i][3] - 0.5) < 0.001
    assert c["FX-IRIS-011"]["0"] == drawn
    assert gone(c["FX-IRIS-012"]["0"]) == everywhere
    thirteen = c["FX-IRIS-013"]["0"]
    left = {(x, y) for x, y in everywhere if dist(x, y, 4, 5) < 6.5}
    assert kept(thirteen) == left & shown and all(x < 10 for x, _ in left)
    assert (0, 5) in kept(thirteen)
    fourteen = c["FX-IRIS-014"]["0"]
    corner = {(x, y) for x, y in everywhere if dist(x, y, 16, 10) < math.sqrt(356) / 2}
    assert kept(fourteen) == corner & shown and (13, 8) in corner and (0, 1) not in corner
    fifteen = c["FX-IRIS-015"]
    assert fifteen["0"] == drawn and fifteen["1"] == five and fifteen["2"] == three
    assert fifteen["3"] == six and fifteen["4"] == c["FX-IRIS-002"]["0"]
    for a, b in zip("0123", "1234"):
        assert kept(fifteen[b]) < kept(fifteen[a])
    sixteen = c["FX-IRIS-016"]
    assert sixteen["0"] == three and sixteen["4"] == thirteen
    assert sixteen["2"] == render(case(center=(37.5, 50), **HALF), 0)
    assert sixteen["2"] not in (three, thirteen)
    seventeen = c["FX-IRIS-017"]
    assert seventeen["0"] == three and seventeen["2"] == eight
    assert seventeen["4"] == render(case(feather=8, **HALF), 0) != eight
    eighteen = c["FX-IRIS-018"]
    assert ease(OVERSHOOT, 0.5) * 100 > 100
    assert eighteen["0"] == drawn and eighteen["2"] == eighteen["4"] == c["FX-IRIS-002"]["0"]
    nineteen = c["FX-IRIS-019"]["0"]
    assert all(nineteen[at(x, y)] == three[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(nineteen[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    print("checked")


if __name__ == "__main__":
    main()
