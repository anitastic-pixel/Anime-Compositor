"""Light Burst, worked a second way.

D-422 adds `core.light_burst`, our name for CycoreFX's CC Light Burst 2.5, which its manual calls
"a type of radial blur that creates an effect that looks like the source layer is exploding with
light". The whole layer is the light: each pixel is smeared outward along the line from a chosen
centre through it, and the streaks are added on top of the layer. Its controls are CC Light Burst
2.5's: Center, Intensity, Ray Length, Burst (Straight, Fade, Center), Set Color and Color. The
manual gives words for each, but no formula, ranges or defaults, so the rule, the ranges and the
defaults are ours; nothing is ported. It is built from Light Rays' last step (D-124) and Spin &
Zoom Blur's three zooms (D-361). Document 21 is the rule in words; this file is the reference
for the numbers document 25 pins against it.

The rule. (1) The light Lt: the layer itself, as it is (no bright test: the manual's "the source
layer uses the alpha channel to act as a light gel"). With `set_color` "on", each pixel's red,
green and blue are replaced by its covering, so the light is white wherever the layer shows, as
much as it shows (the manual: "give the light rays the color you select in the Color control").
(2) The rays Ry: the light through Spin & Zoom Blur's zoom about the centre, quality 50, with its
amount set to `ray_length` and its kind by `burst` (`spin_zoom_blur_reference.blurred`):
"straight" its straight zoom (the manual: "Blurred pixels radiates out from the center point
with constant strength"), "fade" its fading zoom ("radiate out from the center point and fades
out") and "center" its centered zoom ("radiates in and out equally from the center point");
transparent outside the layer; with length 0 it is the light itself. (3) With k = intensity / 100,
T the colour's linear value when `set_color` is "on" and white otherwise, and O the layer, the
output is O + k T Ry in red, green and blue, and min(1, O.a + k Ry.a) in covering, as Light
Rays' last step; nothing is cut off at white. The layer does not grow: the rays are cut at its
edge. `center` is two numbers, per cent of the drawing's width and height, 50, 50 its middle, as
Light Rays'; nothing is scaled for a draft.

Ranges and defaults (ours): center -1000 to 1000, 50, 50; intensity 0 to 2000 per cent, 100;
ray_length 0 to 100 per cent of the way to the centre, 50; burst "straight"; set_color "off"
(the manual's default for its Replace Colors box); color #ffffff, which counts only with
set_color "on". CC's Halo Alpha box is not built (a gap in D-422).

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding Light Rays' lamp drawing, unmoved unless the
case says. The drawing goes into `Fixtures/light_burst/media`, the projects into
`Fixtures/light_burst`, and the expected frames into
`Fixtures/light_burst/expected_light_burst.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/light_burst_reference.py
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
import spin_zoom_blur_reference as SZ  # noqa: E402
import light_rays_reference as LR  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "light_burst"
TOLERANCE = 2e-5  # document 25's default for a filter
CENTER, INTENSITY, LENGTH = (-1000, 1000), (0, 2000), (0, 100)
KINDS = {"straight": "straight_zoom", "fade": "fading_zoom", "center": "centered_zoom"}
NAMES = ("center", "intensity", "ray_length", "burst", "set_color", "color")


# --- the rule -------------------------------------------------------------------------------

def light_burst(pixels, center, intensity, length, burst, set_color, color):
    """The layer's pixels with the burst added. `pixels` is the drawing's 8-bit straight pixels,
    row by row, 16 by 10."""
    working = [R.working(p) for p in pixels]
    # 1. The light: the layer, or white at its covering.
    lit = [[q[3]] * 4 for q in working] if set_color == "on" else working
    light = {"px": lit, "left": 0, "top": 0, "w": W, "h": H}
    tint = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())] \
        if set_color == "on" else [1.0] * 3
    k = intensity / 100
    out = []
    for i, o in enumerate(working):
        # 2. The rays: Spin & Zoom Blur's zoom of the burst's kind, a sample a pixel of path.
        ry = SZ.blurred(light, KINDS[burst], length, 50, center, i % W, i // W)
        # 3. Added on top, tinted; the covering stops at full.
        out.append([o[ch] + k * tint[ch] * ry[ch] for ch in range(3)]
                   + [min(1.0, o[3] + k * ry[3])])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(center=(50, 50), intensity=100, ray_length=50, burst="straight", set_color="off",
         color="#ffffff", shift=0):
    return {"drawing": "lamp", "center": center, "intensity": intensity,
            "ray_length": ray_length, "burst": burst, "set_color": set_color, "color": color,
            "shift": shift}


def clamp(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    pixels = [p for row in LR.DRAWINGS[c["drawing"]] for p in row]
    center = [clamp(v, CENTER) for v in value_at(c["center"], frame_no)]
    number = lambda k, r: clamp(value_at(c[k], frame_no), r)  # noqa: E731
    return R.frame(light_burst(pixels, center, number("intensity", INTENSITY),
                               number("ray_length", LENGTH), c["burst"], c["set_color"],
                               c["color"]), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(intensity=0, shift=c["shift"]), 0)


CASES = {
    "FX-BURST-001": ("The settings as they start: centre 50, 50, the point (8, 5), intensity "
                     "100, ray length 50, Straight, Set Color off. Every pixel that shows is "
                     "light, the yellow, the brown and the purple alike, in its own colour: the "
                     "yellow streaks left into the empty columns 0 to 3, the brown and purple "
                     "right to the drawing's edge, and each streaks up and down into the empty "
                     "rows, fading as it goes; each is brightened by its own light. Nothing is "
                     "drawn past the layer's edge.",
                     case(), [0]),
    "FX-BURST-002": ("Intensity 0: the drawing, untouched.",
                     case(intensity=0), [0]),
    "FX-BURST-003": ("Ray length 0: no streaks, but the light is still added onto itself: "
                     "every colour twice as bright, and the half-covering edge doubled to full "
                     "covering.",
                     case(ray_length=0), [0]),
    "FX-BURST-004": ("Fade: the streaks reach as far as FX-BURST-001's but fade as they run "
                     "out, each pixel's own light counted most, so the streaks over the empty "
                     "pixels are fainter, and the brown's inner column, which a straight burst "
                     "mixes with the empty gap beside it, keeps more of its own colour.",
                     case(burst="fade"), [0]),
    "FX-BURST-005": ("Center: the streaks run half as far out and half as far in, toward the "
                     "centre, so the brown's light falls inward on the empty column 9, "
                     "which FX-BURST-001 leaves empty.",
                     case(burst="center"), [0]),
    "FX-BURST-006": ("Ray length 100, the most: each pixel gathers light all the way from the "
                     "centre out to itself, so the streaks reach further than FX-BURST-001's.",
                     case(ray_length=100), [0]),
    "FX-BURST-007": ("Intensity 250: two and a half times FX-BURST-001's rays, past white and "
                     "not cut off; the covering stops at full.",
                     case(intensity=250), [0]),
    "FX-BURST-008": ("Intensity 2000, the most: twenty times the rays.",
                     case(intensity=2000), [0]),
    "FX-BURST-009": ("Set Color on, #ff8000, orange: the rays take the colour where the layer "
                     "shows, so the yellow, brown and purple all burst orange: red as much as "
                     "the covering, a fifth as much green and no blue. The covering as "
                     "FX-BURST-001's.",
                     case(set_color="on", color="#ff8000"), [0]),
    "FX-BURST-010": ("Set Color on, white: the rays are white light, so even the purple bursts "
                     "white.",
                     case(set_color="on"), [0]),
    "FX-BURST-011": ("Set Color off with colour #ff8000: the colour does not count; "
                     "FX-BURST-001.",
                     case(color="#ff8000"), [0]),
    "FX-BURST-012": ("FX-BURST-009 with its colour written in capitals, #FF8000: the same.",
                     case(set_color="on", color="#FF8000"), [0]),
    "FX-BURST-013": ("Set Color on, #000000, black: the rays add covering but no colour, a dark "
                     "shadow over the empty pixels, and the half-covering edge darkens.",
                     case(set_color="on", color="#000000"), [0]),
    "FX-BURST-014": ("Centre 0, 0, the top left corner: the streaks run down and right, away "
                     "from the corner; the corner itself stays empty.",
                     case(center=(0, 0)), [0]),
    "FX-BURST-015": ("Centre 50, -100, above the drawing: the streaks run straight down to the "
                     "bottom edge, and nothing lights the empty rows above the drawing.",
                     case(center=(50, -100)), [0]),
    "FX-BURST-016": ("Ray length keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is "
                     "FX-BURST-003, frame 2 is FX-BURST-001, frame 4 is FX-BURST-006.",
                     case(ray_length=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-BURST-017": ("Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 "
                     "is FX-BURST-001, frame 2 bursts from 25, 25, frame 4 is FX-BURST-014.",
                     case(center=keyed((0, (50, 50)), (4, (0, 0)))), [0, 2, 4]),
    "FX-BURST-018": ("Intensity eased from 0 at frame 0 to 2000 at frame 4 on a curve that "
                     "overshoots: at frame 2 it would pass 2000, is held at 2000, and is "
                     "FX-BURST-008, as frame 4 is.",
                     case(intensity=keyed((0, 0, OVERSHOOT), (4, 2000))), [0, 2, 4]),
    "FX-BURST-019": ("Center, ray length 100: half the way in and half again out.",
                     case(burst="center", ray_length=100), [0]),
    "FX-BURST-020": ("Fade, Set Color on, #40c0ff, intensity 300, centre 25, 75: the kinds and "
                     "the colour together.",
                     case(burst="fade", set_color="on", color="#40c0ff", intensity=300,
                          center=(25, 75)), [0]),
    "FX-BURST-021": ("FX-BURST-001 moved three pixels right: the burst is drawn on the drawing "
                     "before it is moved, so it is FX-BURST-001 moved, and the three columns "
                     "left of the drawing stay empty, as the layer does not grow.",
                     case(shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-BURST-022": ("Ray length 101, above 100.", case(ray_length=101)),
    "FX-BURST-023": ("Intensity -1, below 0.", case(intensity=-1)),
    "FX-BURST-024": ("Intensity 2001, above 2000.", case(intensity=2001)),
    "FX-BURST-025": ("Intensity keyed to 3000 at frame 4.",
                     case(intensity=keyed((0, 100), (4, 3000)))),
    "FX-BURST-026": ("Centre -1001, 50, past ten widths.", case(center=(-1001, 50))),
    "FX-BURST-027": ("Burst \"sideways\", not one of the three.", case(burst="sideways")),
    "FX-BURST-028": ("Set Color \"yes\", neither \"off\" nor \"on\".", case(set_color="yes")),
    "FX-BURST-029": ("A colour written \"#12345\", one digit short.", case(color="#12345")),
    "FX-BURST-030": ("A colour written \"orange\", a name, not #rrggbb, with Set Color on.",
                     case(set_color="on", color="orange")),
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
        "instance_id": "fx-0-0", "type_id": "core.light_burst", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in LR.DRAWINGS.items():
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

    (OUT / "expected_light_burst.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b: all(abs(p - q) < 1e-12 for u, v in zip(a, b)  # noqa: E731
                            for p, q in zip(u, v))
    one = c["FX-BURST-001"]["0"]

    # Everywhere: nothing is taken away and the covering stays inside 0 to 1.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-BURST-021" else 0))
        for px in frames.values():
            for p, q in zip(px, base):
                assert 0 <= p[3] <= 1 and p[3] >= q[3], fx
                assert all(p[ch] >= q[ch] - 1e-15 for ch in range(3)), fx

    # 001: every colour is light. The yellow streaks left into columns 0 to 3, the purple
    # right to the edge column 15, and the empty rows above and below are lit.
    for y in range(3, 7):
        assert drawn[at(0, y)][3] == 0 and 0 < one[at(0, y)][3] < one[at(3, y)][3], y
        assert one[at(15, y)][3] > 0 and drawn[at(15, y)][3] == 0, y
        assert one[at(11, y)][0] > drawn[at(11, y)][0], y  # the brown, not bright, is lit
    assert all(drawn[at(x, 0)][3] == 0 < one[at(x, 0)][3] for x in (5, 6, 7))
    # Column 9, just right of the centre, lies between it and the brown: a straight burst only
    # streaks outward, so it stays empty.
    assert all(one[at(9, y)] == drawn[at(9, y)] for y in range(H))
    assert c["FX-BURST-002"]["0"] == drawn
    # 003: length 0 adds the layer onto itself.
    three = c["FX-BURST-003"]["0"]
    for i in range(W * H):
        assert three[i] == [2 * drawn[i][ch] for ch in range(3)] + [min(1.0, 2 * drawn[i][3])]
    assert three[at(4, 4)][3] == 1.0 and drawn[at(4, 4)][3] < 1
    # 004: fade, the same reach, the pixel's own light weighed most: the streaks over empty
    # pixels fainter, the brown's inner column, beside the empty gap, keeping more of its own.
    four = c["FX-BURST-004"]["0"]
    assert all((four[i][3] > 0) == (one[i][3] > 0) for i in range(W * H))
    assert all(four[at(x, 4)][3] < one[at(x, 4)][3] for x in (0, 1, 2, 3, 8, 12, 13, 15))
    assert four[at(10, 4)][0] > one[at(10, 4)][0]
    # 005: centre, light runs inward too: the brown lights column 9, toward the centre.
    five = c["FX-BURST-005"]["0"]
    assert five[at(9, 4)][3] > 0 and one[at(9, 4)][3] == 0
    # 006: longer.
    six = c["FX-BURST-006"]["0"]
    assert six[at(0, 4)][3] > one[at(0, 4)][3]
    # 007, 008: the rays scaled, past white.
    for fx, f in (("FX-BURST-007", 2.5), ("FX-BURST-008", 20)):
        px = c[fx]["0"]
        for i in range(W * H):
            for ch in range(3):
                assert abs(px[i][ch] - drawn[i][ch] - f * (one[i][ch] - drawn[i][ch])) < 1e-12
        assert any(v > 1 for p in px for v in p[:3]), fx
    # 009: orange at the covering: red k Ry.a, green a fifth, no blue; the covering as 001's.
    nine, ten = c["FX-BURST-009"]["0"], c["FX-BURST-010"]["0"]
    g = srgb_to_linear(128 / 255)
    for i in range(W * H):
        assert nine[i][3] == one[i][3] and nine[i][2] == drawn[i][2]
        assert abs(nine[i][0] - drawn[i][0] - (one[i][3] - drawn[i][3])) < 1e-12 \
            or one[i][3] == 1.0
        assert abs(nine[i][1] - drawn[i][1] - g * (ten[i][1] - drawn[i][1])) < 1e-12
        # 010: white light, the same in every channel.
        assert abs((ten[i][0] - drawn[i][0]) - (ten[i][2] - drawn[i][2])) < 1e-12
    # The purple, with little red and green, bursts white with Set Color on.
    assert ten[at(15, 4)][1] > one[at(15, 4)][1]
    assert c["FX-BURST-011"]["0"] == one and c["FX-BURST-012"]["0"] == nine
    # 013: black: colour unchanged, covering as 001's.
    thirteen = c["FX-BURST-013"]["0"]
    assert all(thirteen[i][:3] == drawn[i][:3] and thirteen[i][3] == one[i][3]
               for i in range(W * H))
    # 014: about the corner; the corner stays empty.
    fourteen = c["FX-BURST-014"]["0"]
    assert fourteen[at(0, 0)] == [0.0] * 4 and fourteen[at(0, 4)] == drawn[at(0, 4)]
    assert fourteen[at(9, 8)][3] > 0 and one[at(9, 8)][3] == 0
    # 015: from above, straight down; the empty rows above the drawing stay empty.
    fifteen = c["FX-BURST-015"]["0"]
    assert all(fifteen[at(x, y)] == drawn[at(x, y)] for x in range(W) for y in (0, 1))
    assert all(fifteen[at(x, 9)][3] > 0 for x in (5, 6, 7, 10, 11, 14))
    # The keyed cases meet the plain ones at their frames.
    k = c["FX-BURST-016"]
    assert k["0"] == three and k["2"] == one and k["4"] == six
    k = c["FX-BURST-017"]
    assert k["0"] == one and near(k["4"], fourteen)
    assert near(k["2"], render(case(center=(25, 25)), 0)) and k["2"] != one
    k = c["FX-BURST-018"]
    assert ease(OVERSHOOT, 0.5) > 1
    assert k["0"] == drawn and k["2"] == k["4"] == c["FX-BURST-008"]["0"]
    # 019: centre at 100 reaches further in than 005.
    assert c["FX-BURST-019"]["0"][at(9, 3)][3] > five[at(9, 3)][3]
    # Moved: FX-BURST-001 three columns on, nothing left of the drawing.
    moved = c["FX-BURST-021"]
    assert moved["0"] == moved["3"]
    assert all(moved["0"][at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved["0"][at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    # Straight with no tint is Light Rays with threshold 0 and intensity 1, exactly.
    pixels = [p for row in LR.DRAWINGS["lamp"] for p in row]
    rays = LR.light_rays(pixels, (50, 50), 50, 0, 1, "#ffffff")
    assert near(R.frame(rays, 0), one)
    print("checked")


if __name__ == "__main__":
    main()
