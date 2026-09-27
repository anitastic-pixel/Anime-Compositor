"""Light rays, worked a second way.

D-124 adds `core.light_rays`. It draws streaks of light from the bright parts of a layer, away
from a chosen point, as light pours through a gap or round a sun: each bright pixel is smeared
outward along the line from the centre through it, and the streaks are added on top of the
layer, tinted by a colour. It is this program's own method, modelled on After Effects' CC Light
Rays in spirit, not claimed to match it, and built from D-89's bright test and Radial Blur's
zoom (D-95 with D-110); nothing is ported. Document 21 is the rule in words; this file is the reference for the numbers document
25 pins against it.

The rule. (1) The light Lt: each pixel that shows, whose brightest channel in 8-bit steps is at
least the threshold's share of 255, exactly as D-89's glow tests it (`glow_reference.glows`);
every other pixel gives nothing. (2) The rays Ry: the light through Radial Blur's zoom about the
centre with its amount set to `length`, transparent outside the layer
(`radial_blur_reference.blurred`): each pixel is the plain average of n bilinear samples of the
light on the line from the pixel back toward the centre, n = min(ceil(r * length / 100) + 1,
256), r the pixel's distance from the centre; with length 0 it is the light itself. (3) With C
the colour's linear value and O the layer, the output is O + intensity * C * Ry in red, green
and blue, and min(1, O.a + intensity * Ry.a) in covering; the colour is not cut off at white.
The layer does not grow: the rays are cut at its edge. `center` is two numbers, per cent of the
drawing's width and height, 50, 50 its middle, as Radial Blur's; nothing is scaled for a draft.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: glow's three colours, drawn below. The drawing goes into
`Fixtures/light_rays/media`, the projects into `Fixtures/light_rays`, and the expected frames
into `Fixtures/light_rays/expected_light_rays.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/light_rays_reference.py
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
import radial_blur_reference as RB  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "light_rays"
TOLERANCE = 2e-5  # document 25's default for a filter
CENTER, LENGTH, THRESHOLD, INTENSITY = (-1000, 1000), (0, 100), (0, 100), (0, 10)
NAMES = ("center", "length", "threshold", "intensity", "color")


# --- the rule -------------------------------------------------------------------------------

def light_rays(pixels, center, length, threshold, intensity, color, size=(W, H)):
    """The layer's pixels with the rays added. `pixels` is the drawing's 8-bit straight pixels,
    row by row, `size` its own w by h."""
    w, h = size
    working = [R.working(p) for p in pixels]
    # 1. The light: D-89's bright test, the pixel as it is.
    bright = {"based_on": "bright"}
    light = {"px": [q if G.glows(p, bright, threshold, 0) else [0.0] * 4
                    for p, q in zip(pixels, working)], "left": 0, "top": 0, "w": w, "h": h}
    # 2. The rays: Radial Blur's zoom about the centre. Its centre is per cent of its own 16 by
    # 10, so the centre is handed over rescaled to land at per cent of this drawing's size.
    at = (center[0] * w / RB.W, center[1] * h / RB.H)
    tint = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    out = []
    for i, o in enumerate(working):
        ry = RB.blurred(light, "zoom", length, at, i % w, i // w)
        # 3. Added on top, tinted; the covering stops at full.
        out.append([o[ch] + intensity * tint[ch] * ry[ch] for ch in range(3)]
                   + [min(1.0, o[3] + intensity * ry[3])])
    return out


# --- the drawing ----------------------------------------------------------------------------

BRIGHT, EDGE, DARK = G.BRIGHT, G.EDGE, G.DARK  # yellow 98 %, brown exactly 60 %, purple 43 %
SOFT = BRIGHT[:3] + (128,)                     # the yellow at half covering, a soft edge
NONE = S.NONE


def lamp(x, y):
    """A yellow block in columns 5 to 7 and rows 3 to 6, left of the middle, with a
    half-covering edge down column 4; a brown block in columns 10 and 11, rows 3 to 6, right of
    the middle; a purple line down column 14, rows 2 to 7. Everything else is empty."""
    if 3 <= y <= 6 and 5 <= x <= 7:
        return BRIGHT
    if 3 <= y <= 6 and x == 4:
        return SOFT
    if 3 <= y <= 6 and 10 <= x <= 11:
        return EDGE
    if 2 <= y <= 7 and x == 14:
        return DARK
    return NONE


DRAWINGS = {"lamp": [[lamp(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(center=(50, 50), length=50, threshold=70, intensity=1, color="#ffffff", shift=0):
    return {"drawing": "lamp", "center": center, "length": length, "threshold": threshold,
            "intensity": intensity, "color": color, "shift": shift}


def clamp(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    center = [clamp(v, CENTER) for v in value_at(c["center"], frame_no)]
    number = lambda k, r: clamp(value_at(c[k], frame_no), r)  # noqa: E731
    return R.frame(light_rays(pixels, center, number("length", LENGTH),
                              number("threshold", THRESHOLD), number("intensity", INTENSITY),
                              c["color"]), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(intensity=0, shift=c["shift"]), 0)


CASES = {
    "FX-RAYS-001": ("The settings as they start: centre 50, 50, the point (8, 5), length 50, "
                    "threshold 70, intensity 1, white. Only the yellow, 98 %, is bright enough; "
                    "its light streaks outward, away from the centre: left across its soft "
                    "edge into the empty columns 0 to 3, and up and down into the empty rows "
                    "above and below it, fading as it goes, and it brightens the yellow "
                    "itself. The brown, 60 %, and the purple give no light, and columns 9 to "
                    "15 are untouched; nothing is drawn past the layer's edge.",
                    case(), [0]),
    "FX-RAYS-002": ("Intensity 0: the drawing, untouched.",
                    case(intensity=0), [0]),
    "FX-RAYS-003": ("Length 0: no streaks, but the light is still added onto itself: each "
                    "yellow pixel twice as bright, and the half-covering edge, whose light is "
                    "its own, doubled to full covering.",
                    case(length=0), [0]),
    "FX-RAYS-004": ("Threshold 100: nothing is that bright, so the drawing is untouched.",
                    case(threshold=100), [0]),
    "FX-RAYS-005": ("Threshold 60: the brown, at exactly 60 %, is lit too, and its light "
                    "streaks right, over the purple line and on to the drawing's right edge.",
                    case(threshold=60), [0]),
    "FX-RAYS-006": ("Length 100, the most: each pixel gathers the light all the way from the "
                    "centre out to itself, so the streaks reach further than FX-RAYS-001's; and "
                    "as the yellow touches the centre, every pixel of the drawing takes some "
                    "of its light, right of the centre too, faintest at the right edge.",
                    case(length=100), [0]),
    "FX-RAYS-007": ("Intensity 2.5: two and a half times FX-RAYS-001's rays, and where they "
                    "add past white they are not cut off; the covering stops at full.",
                    case(intensity=2.5), [0]),
    "FX-RAYS-008": ("Colour #ff8000, orange: FX-RAYS-001's rays tinted, their red as before, "
                    "their green a fifth and their blue gone; the covering as FX-RAYS-001's.",
                    case(color="#ff8000"), [0]),
    "FX-RAYS-009": ("Colour #000000, black: the rays add no colour but still add covering, so "
                    "they fall as a dark shadow on the empty pixels; the solid yellow is "
                    "unchanged, and its half-covering edge gains covering but no colour, so it "
                    "darkens.",
                    case(color="#000000"), [0]),
    "FX-RAYS-010": ("FX-RAYS-008 with its colour written in capitals, #FF8000: the same.",
                    case(color="#FF8000"), [0]),
    "FX-RAYS-011": ("Centre 0, 0, the top left corner: the streaks run down and right, away "
                    "from the corner, onto the empty pixels right of the yellow and below it; "
                    "the corner itself stays empty.",
                    case(center=(0, 0)), [0]),
    "FX-RAYS-012": ("Centre 50, -100, above the drawing: the streaks run straight down from "
                    "the yellow to the bottom edge, and nothing lights the rows above it.",
                    case(center=(50, -100)), [0]),
    "FX-RAYS-013": ("Length keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is "
                    "FX-RAYS-003, frame 2 is FX-RAYS-001, frame 4 is FX-RAYS-006.",
                    case(length=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-RAYS-014": ("Threshold keyed from 100 at frame 0 to 60 at frame 4, linear: frame 0 is "
                    "the drawing, frame 2, at 80, lights the yellow alone and is FX-RAYS-001, "
                    "frame 4 is FX-RAYS-005.",
                    case(threshold=keyed((0, 100), (4, 60))), [0, 2, 4]),
    "FX-RAYS-015": ("Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is "
                    "FX-RAYS-001, frame 2 streams from 25, 25, frame 4 is FX-RAYS-011.",
                    case(center=keyed((0, (50, 50)), (4, (0, 0)))), [0, 2, 4]),
    "FX-RAYS-016": ("Intensity eased from 0 at frame 0 to 10 at frame 4 on a curve that "
                    "overshoots: at frame 2 it would pass 10, is held at 10, and is intensity "
                    "10 plain, as frame 4 is.",
                    case(intensity=keyed((0, 0, OVERSHOOT), (4, 10))), [0, 2, 4]),
    "FX-RAYS-017": ("FX-RAYS-001 moved three pixels right: the rays are drawn on the drawing "
                    "before it is moved, so it is FX-RAYS-001 moved, and the three columns left "
                    "of the drawing stay empty, as the layer does not grow.",
                    case(shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-RAYS-018": ("Length 101, above 100.", case(length=101)),
    "FX-RAYS-019": ("Threshold -1, below 0.", case(threshold=-1)),
    "FX-RAYS-020": ("Intensity 11, above 10.", case(intensity=11)),
    "FX-RAYS-021": ("Intensity keyed to 20 at frame 4.", case(intensity=keyed((0, 1), (4, 20)))),
    "FX-RAYS-022": ("Centre 50, -1001, past ten heights.", case(center=(50, -1001))),
    "FX-RAYS-023": ("A colour written \"#12345\", one digit short.", case(color="#12345")),
    "FX-RAYS-024": ("A colour written \"orange\", a name, not #rrggbb.", case(color="orange")),
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
        "instance_id": "fx-0-0", "type_id": "core.light_rays", "enabled": True,
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

    (OUT / "expected_light_rays.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b: all(abs(p - q) < 1e-12 for u, v in zip(a, b)  # noqa: E731
                            for p, q in zip(u, v))
    one = c["FX-RAYS-001"]["0"]
    yellow = [(x, y) for x in range(4, 8) for y in range(3, 7)]

    # The rule's pieces: the bright test is D-89's; the zoom's samples run from the pixel back
    # toward the centre, as many as the path needs.
    assert G.glows(EDGE, {"based_on": "bright"}, 60, 0)
    assert not G.glows(EDGE, {"based_on": "bright"}, 60.1, 0)
    assert len(RB.samples("zoom", 50, (50, 50), 0, 4)) == math.ceil(math.hypot(7.5, 0.5) / 2) + 1
    # Everywhere: nothing is taken away, the covering stays inside 0 to 1, and a colour never
    # goes below nothing.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-RAYS-017" else 0))
        for px in frames.values():
            for p, q in zip(px, base):
                assert 0 <= p[3] <= 1 and p[3] >= q[3], fx
                assert all(p[ch] >= q[ch] - 1e-15 for ch in range(3)), fx

    # 001: the empty columns left of the yellow are lit, less the further out, and the empty
    # rows above and below it; the yellow is brighter; columns 9 to 15 are untouched.
    for y in range(3, 7):
        assert drawn[at(0, y)][3] == 0 and 0 < one[at(0, y)][3] < one[at(3, y)][3], y
        assert one[at(4, y)][3] > drawn[at(4, y)][3] and one[at(6, y)][0] > drawn[at(6, y)][0]
    assert all(one[at(x, y)] == drawn[at(x, y)] for x in range(9, W) for y in range(H))
    assert all(drawn[at(x, y)][3] == 0 < one[at(x, y)][3] for x in range(4, 8) for y in (0, 9))
    assert c["FX-RAYS-002"]["0"] == drawn and c["FX-RAYS-004"]["0"] == drawn
    # 003: length 0 adds each lit pixel onto itself; the soft edge doubles to full covering.
    three = c["FX-RAYS-003"]["0"]
    for i in range(W * H):
        lit = drawn[i] if drawn[i][3] and drawn[i][:3] == R.working(BRIGHT)[:3] else None
        lit = lit or (drawn[i] if drawn[i] == R.working(SOFT) else [0.0] * 4)
        assert three[i] == [drawn[i][ch] + lit[ch] for ch in range(3)] \
            + [min(1.0, drawn[i][3] + lit[3])], i
    assert three[at(4, 4)][3] == 1.0 and drawn[at(4, 4)][3] < 1
    # 005: the brown streams right, over the purple and to the right edge.
    five = c["FX-RAYS-005"]["0"]
    assert five[at(14, 4)][0] > drawn[at(14, 4)][0] and five[at(15, 4)][3] > 0
    assert five[at(12, 4)][3] > 0 and one[at(12, 4)][3] == 0
    # 006: longer: column 0 is lit more, and light crosses right of the centre.
    six = c["FX-RAYS-006"]["0"]
    assert six[at(0, 4)][3] > one[at(0, 4)][3]
    assert six[at(9, 5)][3] > one[at(9, 5)][3]
    assert all(six[i] != drawn[i] for i in range(W * H))
    # 007: two and a half times the rays, past white.
    seven = c["FX-RAYS-007"]["0"]
    for i in range(W * H):
        for ch in range(3):
            assert abs(seven[i][ch] - drawn[i][ch] - 2.5 * (one[i][ch] - drawn[i][ch])) < 1e-12
    assert any(v > 1 for p in seven for v in p[:3])
    # 008: orange, #ff8000: red as white's, green times srgb_to_linear(128 / 255), no blue.
    eight = c["FX-RAYS-008"]["0"]
    g = srgb_to_linear(128 / 255)
    for i in range(W * H):
        assert eight[i][0] == one[i][0] and eight[i][2] == drawn[i][2] and eight[i][3] == one[i][3]
        assert abs(eight[i][1] - drawn[i][1] - g * (one[i][1] - drawn[i][1])) < 1e-12
    # 009: black: colour unchanged, covering as white's.
    nine = c["FX-RAYS-009"]["0"]
    assert all(nine[i][:3] == drawn[i][:3] and nine[i][3] == one[i][3] for i in range(W * H))
    assert nine[at(1, 4)][3] > 0 and nine[at(1, 4)][:3] == [0.0] * 3
    assert nine[at(4, 4)][3] > drawn[at(4, 4)][3] and nine[at(5, 4)] == drawn[at(5, 4)]
    assert c["FX-RAYS-010"]["0"] == eight
    # 011: about the top left corner, down and right; the corner stays empty.
    eleven = c["FX-RAYS-011"]["0"]
    assert eleven[at(0, 0)] == [0.0] * 4
    assert all(eleven[at(9, y)][3] > 0 and one[at(9, y)][3] == 0 for y in (5, 6, 8))
    assert eleven[at(0, 4)] == drawn[at(0, 4)]
    # 012: from above: straight down, nothing above the yellow.
    twelve = c["FX-RAYS-012"]["0"]
    assert all(twelve[at(x, y)][3] > 0 for x in (5, 6, 7) for y in (7, 8, 9))
    assert all(twelve[at(x, y)] == drawn[at(x, y)] for x in range(W) for y in (0, 1, 2))
    # The keyed cases meet the plain ones at their frames.
    k = c["FX-RAYS-013"]
    assert k["0"] == three and k["2"] == one and k["4"] == six
    k = c["FX-RAYS-014"]
    assert k["0"] == drawn and k["2"] == one and k["4"] == five
    k = c["FX-RAYS-015"]
    assert k["0"] == one and near(k["4"], eleven)
    assert near(k["2"], render(case(center=(25, 25)), 0)) and k["2"] != one
    k = c["FX-RAYS-016"]
    assert ease(OVERSHOOT, 0.5) > 1
    assert k["0"] == drawn and k["2"] == k["4"] == render(case(intensity=10), 0)
    # Moved: FX-RAYS-001 three columns on, nothing left of the drawing.
    moved = c["FX-RAYS-017"]
    assert moved["0"] == moved["3"]
    assert all(moved["0"][at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved["0"][at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    for p in yellow:
        assert one[at(*p)] != drawn[at(*p)]
    print("checked")


if __name__ == "__main__":
    main()
