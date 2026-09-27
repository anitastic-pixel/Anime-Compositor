"""Bulge, worked a second way.

D-152 adds `core.bulge`: a round part of the picture swelled out, as if seen through a lens or
pushed from behind, or with a negative height pinched in. Inside a circle of `radius` pixels about
`center`, per cent of the drawing's own width and height as Radial Blur's centre is, each pixel
takes the picture from a point pulled toward the centre, most at the centre and not at all at the
circle's edge, so the middle is magnified and the ring around it squeezed; a negative `height`
pushes the point outward instead, so the middle shrinks. Outside the circle every pixel is kept
exactly. It is this program's own method, modelled on After Effects' Bulge in spirit and not
claimed to match it; nothing is ported. Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

The rule. With height 0 or radius 0 the output is the input. Otherwise, with w, h the drawing's
own size, c = (center_x / 100 * w, center_y / 100 * h) and P a pixel's centre, d = |P - c|: at
d >= radius the input pixel is kept exactly; else r = d / radius,
m = max(0, 1 - height (1 - r)^2 / 2), and the output is document 21's bilinear sample of the
input at c + m (P - c), transparent outside it. At the circle's edge m = 1, so the warp meets the
kept pixels without a seam; at the centre m = 1 - height / 2, 0 from height 2 up, where the whole
middle reads the centre point itself. The layer does not grow; a draft scales the radius.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says, drawn below. The drawing goes into `Fixtures/bulge/media`, the projects into
`Fixtures/bulge`, and the expected frames into `Fixtures/bulge/expected_bulge.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/bulge_reference.py
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
from radial_blur_reference import bilinear  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "bulge"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"center": (-1000, 1000), "radius": (0, 10000), "height": (-4, 4)}
NAMES = ("center", "radius", "height")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def centre(center, w=W, h=H):
    return center[0] / 100 * w, center[1] / 100 * h


def scale(center, radius, height, px, py):
    """m at the point (px, py) of the drawing's own pixels, or None where the pixel is kept."""
    if height == 0 or radius == 0:
        return None
    cx, cy = centre(center)
    d = math.hypot(px - cx, py - cy)
    if d >= radius:
        return None
    r = d / radius
    return max(0.0, 1 - height * (1 - r) ** 2 / 2)


def bulged(layer, center, radius, height, x, y):
    """The output at pixel (x, y) of the drawing's own space; empty outside the drawing."""
    if not (0 <= x < layer["w"] and 0 <= y < layer["h"]):
        return EMPTY
    px, py = x + 0.5, y + 0.5
    m = scale(center, radius, height, px, py)
    if m is None:
        return list(layer["px"][y * layer["w"] + x])
    cx, cy = centre(center)
    return bilinear(layer, cx + m * (px - cx), cy + m * (py - cy))


# --- the drawing ----------------------------------------------------------------------------

LINE, SKIN, SOFT_SKIN, NONE = R.LINE, R.SKIN, (246, 214, 190, 128), S.NONE
BAND = (58, 111, 216, 255)  # #3a6fd8, the ball's blue band


def stripes(x, y):
    """Rows 1 to 8, columns 0 to 13: upright stripes two pixels wide, skin and line by turns
    (line in columns 2, 3, 6, 7, 10 and 11), crossed by a blue band in rows 4 and 5, so a swell
    or a pinch either way shows. Column 14 is the skin at half covering, a soft edge; column 15
    and rows 0 and 9 are empty. The middle, (8, 5), falls on the corner between four band
    pixels."""
    if x == 15 or y in (0, 9):
        return NONE
    if x == 14:
        return SOFT_SKIN
    if y in (4, 5):
        return BAND
    return LINE if (x // 2) % 2 else SKIN


DRAWINGS = {"stripes": [[stripes(x, y) for x in range(W)] for y in range(H)]}


def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


# --- the cases ------------------------------------------------------------------------------

def case(center=(50, 50), radius=50, height=1, shift=0):
    return {"drawing": "stripes", "center": center, "radius": radius, "height": height,
            "shift": shift}


def settings(c, frame_no):
    held = {}
    for k in NAMES:
        lo, hi = RANGES[k]
        v = value_at(c[k], frame_no)
        held[k] = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def render(c, frame_no):
    layer = drawn_layer(c["drawing"])
    s = settings(c, frame_no)
    return [bulged(layer, s["center"], s["radius"], s["height"], x - c["shift"], y)
            for y in range(H) for x in range(W)]


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(height=0, shift=c["shift"]), 0)


SMALL = {"radius": 6}  # a circle six pixels round the middle: the drawing's ends stay as drawn

CASES = {
    "FX-BULGE-001": ("The settings as they start: centre 50, 50, radius 50, height 1. The "
                     "circle is far larger than the drawing, so every pixel is pulled toward the "
                     "middle, by about half at the middle and a third at the corners: the whole "
                     "drawing is magnified about the middle, the blue band spreading into rows 3 "
                     "and 6, and the empty top and bottom rows and right-hand column filled from "
                     "inside.",
                     case(), [0]),
    "FX-BULGE-002": ("Height 0: the drawing, untouched.",
                     case(height=0), [0]),
    "FX-BULGE-003": ("Radius 0: the drawing, untouched.",
                     case(radius=0), [0]),
    "FX-BULGE-004": ("Radius 6, height 1: only the pixels whose centres lie within 6 pixels of "
                     "the middle change, the band and the stripes swelling there; every pixel "
                     "farther out, the drawing's two ends among them, is kept exactly.",
                     case(**SMALL), [0]),
    "FX-BULGE-005": ("Radius 6, height -1: a pinch. Inside the circle every pixel reads from "
                     "farther out than itself, so the middle shrinks and the stripes there "
                     "narrow; outside it nothing changes.",
                     case(height=-1, **SMALL), [0]),
    "FX-BULGE-006": ("Radius 6, height 4, the most: every pixel within 1.76 pixels of the "
                     "middle, twelve of them, reads the middle point itself, the corner of four "
                     "band pixels, so they are all the band's blue, a flat disc.",
                     case(height=4, **SMALL), [0]),
    "FX-BULGE-007": ("Radius 6, height -4, the least: a hard pinch, the pixel next to the "
                     "middle reading from two and a half times as far out.",
                     case(height=-4, **SMALL), [0]),
    "FX-BULGE-008": ("Centre 25, 50, radius 6, height 1: the swell moves to (4, 5), and every "
                     "pixel 6 or more from there, every column from 10 on among them, is kept "
                     "exactly.",
                     case(center=(25, 50), **SMALL), [0]),
    "FX-BULGE-009": ("Centre 53.125, 55, radius 6, height 1: the middle is the centre of pixel "
                     "(8, 5) itself, 0 from it, so that pixel reads itself and is kept exactly; "
                     "the pixels round it swell.",
                     case(center=(53.125, 55), **SMALL), [0]),
    "FX-BULGE-010": ("Centre -100, 50, radius 6: the circle lies wholly left of the drawing, so "
                     "nothing is in reach: the drawing, untouched.",
                     case(center=(-100, 50), **SMALL), [0]),
    "FX-BULGE-011": ("Radius 6, height keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 "
                     "is the drawing, frame 2 is FX-BULGE-004, and frame 4 is height 2.",
                     case(height=keyed((0, 0), (4, 2)), **SMALL), [0, 2, 4]),
    "FX-BULGE-012": ("Height 1, radius keyed from 0 at frame 0 to 12 at frame 4, linear: the "
                     "swell opens out, frame 0 the drawing, frame 2 FX-BULGE-004 and frame 4 "
                     "radius 12.",
                     case(radius=keyed((0, 0), (4, 12))), [0, 2, 4]),
    "FX-BULGE-013": ("Radius 6, centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4, "
                     "linear: frame 0 is FX-BULGE-004, frame 2 is centre 37.5, 50, and frame 4 "
                     "is FX-BULGE-008.",
                     case(center=keyed((0, (50, 50)), (4, (25, 50))), **SMALL), [0, 2, 4]),
    "FX-BULGE-014": ("Radius 6, height eased from 0 at frame 0 to 4 at frame 4 on a curve that "
                     "overshoots: at frame 2 it would pass 4, is held at 4, and is FX-BULGE-006, "
                     "as frame 4 is; frame 0 is the drawing.",
                     case(height=keyed((0, 0, OVERSHOOT), (4, 4)), **SMALL), [0, 2, 4]),
    "FX-BULGE-015": ("FX-BULGE-004 moved three pixels right: the same, moved; the swell is "
                     "worked in the drawing's own space and moves with it, nothing grows, and "
                     "the three columns left of the drawing stay empty.",
                     case(shift=3, **SMALL), [0]),
    "FX-BULGE-016": ("Centre 75, 50, radius 5, height 1: the swell sits on (12, 5), near the "
                     "soft edge, so the soft column is spread outward and the empty column 15 "
                     "takes part of its covering where the circle reaches it: a warp moves "
                     "covering, and an empty pixel inside the circle need not stay empty.",
                     case(center=(75, 50), radius=5), [0]),
    "FX-BULGE-017": ("Radius 10000, the most, height 1: the circle is so wide that every pixel "
                     "is pulled almost exactly half way to the middle, within a hundredth of a "
                     "pixel: the drawing doubled in size about its middle.",
                     case(radius=10000), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-BULGE-018": ("Radius -1, below 0.", case(radius=-1)),
    "FX-BULGE-019": ("Radius 10001, above 10000.", case(radius=10001)),
    "FX-BULGE-020": ("Height 4.5, above 4.", case(height=4.5)),
    "FX-BULGE-021": ("Height -4.5, below -4.", case(height=-4.5)),
    "FX-BULGE-022": ("Centre 50, 1001, past ten heights.", case(center=(50, 1001))),
    "FX-BULGE-023": ("Height keyed to 5 at frame 4.", case(height=keyed((0, 1), (4, 5)))),
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
        "instance_id": "fx-0-0", "type_id": "core.bulge", "enabled": True,
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

    (OUT / "expected_bulge.json").write_text(json.dumps(expected, indent=1) + "\n",
                                             encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    within = lambda x, y, cen, rad: math.hypot(  # noqa: E731
        x + 0.5 - centre(cen)[0], y + 0.5 - centre(cen)[1]) < rad
    band = R.working(BAND)
    everywhere = [(x, y) for y in range(H) for x in range(W)]

    # The rule's own pieces: m is 1 - height / 2 at the centre and 1 at the circle's edge, never
    # below 0; nothing is kept inside the circle and everything past it.
    assert scale((50, 50), 6, 1, 8, 5) == 0.5 and scale((50, 50), 6, 4, 8, 5) == 0
    assert scale((50, 50), 6, -4, 8, 5) == 3
    assert abs(scale((50, 50), 6, 1, 8, 5 + 6 - 1e-9) - 1) < 1e-9
    assert scale((50, 50), 6, 1, 8, 11) is None and scale((50, 50), 0, 1, 8, 5) is None
    assert scale((50, 50), 6, 0, 8, 5) is None
    assert centre((25, 50)) == (4, 5) and centre((53.125, 55)) == (8.5, 5.5)

    # Every case keeps every channel within its covering; the invalid ones are the drawing.
    for fx, frames in c.items():
        s = expected["cases"][fx]
        base = plain(CASES[fx][1] if fx in CASES else INVALID[fx][1])
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in s:
            assert all(px == base for px in frames.values())

    one = c["FX-BULGE-001"]["0"]
    ms = [scale((50, 50), 50, 1, x + 0.5, y + 0.5) for x, y in everywhere]
    assert all(0.5 < m < 0.67 for m in ms)
    assert sum(one[i] != drawn[i] for i in range(W * H)) > 140
    for x in (7, 8):  # the band spreads into rows 3 and 6: more than half blue on its channel
        for y in (3, 6):
            assert one[at(x, y)][2] - drawn[at(x, y)][2] > (band[2] - drawn[at(x, y)][2]) / 2
    assert all(one[at(x, y)][3] > 0 for x, y in ((8, 0), (8, 9), (15, 5)))
    assert c["FX-BULGE-002"]["0"] == drawn and c["FX-BULGE-003"]["0"] == drawn

    def kept_outside(px, cen, rad):
        for x, y in everywhere:
            if not within(x, y, cen, rad):
                assert px[at(x, y)] == drawn[at(x, y)], (x, y)
        return sum(px[at(x, y)] != drawn[at(x, y)] for x, y in everywhere
                   if within(x, y, cen, rad))

    four = c["FX-BULGE-004"]["0"]
    assert kept_outside(four, (50, 50), 6) > 60
    assert all(four[at(x, y)] == drawn[at(x, y)] for x in (0, 1, 14, 15) for y in range(H))
    five = c["FX-BULGE-005"]["0"]
    assert kept_outside(five, (50, 50), 6) > 60
    assert all(scale((50, 50), 6, -1, x + 0.5, y + 0.5) > 1 for x, y in everywhere
               if within(x, y, (50, 50), 6))
    assert five != four
    six = c["FX-BULGE-006"]["0"]
    disc = [(x, y) for x, y in everywhere if math.hypot(x + 0.5 - 8, y + 0.5 - 5) <= 1.76]
    assert len(disc) == 12
    assert all(scale((50, 50), 6, 4, x + 0.5, y + 0.5) == 0 for x, y in disc)
    assert all(close(six[at(x, y)], band) for x, y in disc)
    assert 1 - math.sqrt(0.5) < 1.76 / 6 < 1.77 / 6  # m reaches 0 at r = 1 - 1 / sqrt(2)
    seven = c["FX-BULGE-007"]["0"]
    assert kept_outside(seven, (50, 50), 6) > 60
    assert abs(scale((50, 50), 6, -4, 8.5, 5.5) - 1 - 2 * (1 - math.sqrt(0.5) / 6) ** 2) < 1e-12
    assert 2.5 < scale((50, 50), 6, -4, 8.5, 5.5) < 2.6
    eight = c["FX-BULGE-008"]["0"]
    assert kept_outside(eight, (25, 50), 6) > 50
    assert all(eight[at(x, y)] == drawn[at(x, y)] for x in range(10, W) for y in range(H))
    nine = c["FX-BULGE-009"]["0"]
    assert nine[at(8, 5)] == drawn[at(8, 5)]
    assert kept_outside(nine, (53.125, 55), 6) > 60
    assert nine[at(8, 6)] != drawn[at(8, 6)] and nine[at(8, 3)] != drawn[at(8, 3)]
    assert c["FX-BULGE-010"]["0"] == drawn
    eleven = c["FX-BULGE-011"]
    assert eleven["0"] == drawn and eleven["2"] == four
    assert eleven["4"] == render(case(height=2, **SMALL), 0) != four
    twelve = c["FX-BULGE-012"]
    assert twelve["0"] == drawn and twelve["2"] == four
    assert twelve["4"] == render(case(radius=12), 0) != four
    thirteen = c["FX-BULGE-013"]
    assert thirteen["0"] == four and thirteen["4"] == eight
    assert thirteen["2"] == render(case(center=(37.5, 50), **SMALL), 0)
    assert thirteen["2"] not in (four, eight)
    fourteen = c["FX-BULGE-014"]
    assert ease(OVERSHOOT, 0.5) * 4 > 4
    assert fourteen["0"] == drawn and fourteen["2"] == six == fourteen["4"]
    fifteen = c["FX-BULGE-015"]["0"]
    assert all(fifteen[at(x, y)] == four[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(fifteen[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    sixteen = c["FX-BULGE-016"]["0"]
    assert kept_outside(sixteen, (75, 50), 5) > 30
    assert drawn[at(15, 5)] == EMPTY and 0 < sixteen[at(15, 5)][3] < 128 / 255
    assert sixteen[at(14, 5)] != drawn[at(14, 5)]
    assert drawn[at(15, 0)] == EMPTY == sixteen[at(15, 0)]  # past the circle: still empty
    seventeen = c["FX-BULGE-017"]["0"]
    for x, y in everywhere:
        m = scale((50, 50), 10000, 1, x + 0.5, y + 0.5)
        assert abs(m - 0.5) * math.hypot(x + 0.5 - 8, y + 0.5 - 5) < 0.01
    assert sum(seventeen[i] != drawn[i] for i in range(W * H)) > 140
    print("checked")


if __name__ == "__main__":
    main()
