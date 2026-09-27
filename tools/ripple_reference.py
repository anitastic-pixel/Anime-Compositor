"""Ripple, worked a second way.

D-150 adds `core.ripple`, rings spreading out from a point, as on a pond after a stone falls in:
each pixel of the output takes the picture from a little way along the line through the centre,
outward or inward by turns, so straight lines bend into rings about the centre. `center` is the
point, two numbers, per cent of the drawing, each -1000 to 1000, starting at 50, 50;
`amplitude`, 0 to 1000, starting at 5, is the most a pixel is pushed, in pixels; `wavelength`,
1 to 10000, starting at 30, is how many pixels apart the rings are; `speed`, -360 to 360
degrees a frame, starting at 20, moves the rings outward (inward when below 0), one full turn of
360 moving them one wavelength; `phase`, -100000 to 100000 degrees, starting at 0, sets where
the rings stand at frame 0; and `fade`, 0 to 100000, starting at 0, off, makes the push die away
to nothing at that many pixels from the centre. Amplitude, wavelength and fade are distances, so
a draft preview scales them. It is this program's own method, modelled on After Effects' Ripple
in spirit and not claimed to match it; nothing is ported. Document 21 is the rule in words; this
file is the reference for the numbers document 25 pins against it.

The rule. With amplitude 0 the output is the input. Otherwise, at each pixel, P its centre in
the drawing's own space, c = (center.x / 100 * w, center.y / 100 * h) the centre point and
d = |P - c|: at d == 0 the output is the input pixel; else, with frame the composition frame,
a hidden value filled in by the build as Noise's is,
w = sin(2 pi d / wavelength - radians(phase + speed * frame)), f = 1 when fade is 0, else
max(0, 1 - d / fade), and the output is document 21's bilinear sample of the input, transparent
outside it, at P + amplitude * w * f * (P - c) / d. The layer does not grow. A pixel whose push
is 0 (f = 0 past the fade, or w = 0 on a still ring) is its input pixel exactly; and as d nears
0 with phase + speed * frame a whole turn the push nears 0 too.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: Turbulent Displace's stripes (`tools/turbulent_displace_reference.py`), imported.
The drawing goes into `Fixtures/ripple/media`, the projects into `Fixtures/ripple`, and the
expected frames into `Fixtures/ripple/expected_ripple.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/ripple_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
from turbulent_displace_reference import stripes  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "ripple"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"amplitude": (0, 1000), "wavelength": (1, 10000), "speed": (-360, 360),
          "phase": (-100000, 100000), "fade": (0, 100000)}
CENTER = (-1000, 1000)
NAMES = ("center", "amplitude", "wavelength", "speed", "phase", "fade")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def push(center, amplitude, wavelength, speed, phase, fade, frame_no, x, y, size=(W, H)):
    """The push at pixel (x, y) of the drawing's own space, and its centre P."""
    px, py = x + 0.5, y + 0.5
    cx, cy = center[0] / 100 * size[0], center[1] / 100 * size[1]
    dx, dy = px - cx, py - cy
    d = math.sqrt(dx * dx + dy * dy)
    if amplitude == 0 or d == 0:
        return (0.0, 0.0), (px, py)
    w = math.sin(2 * math.pi * d / wavelength - math.radians(phase + speed * frame_no))
    f = 1.0 if fade == 0 else max(0.0, 1 - d / fade)
    k = amplitude * w * f / d
    return (k * dx, k * dy), (px, py)


def rippled(layer, center, amplitude, wavelength, speed, phase, fade, frame_no, x, y):
    """The output at pixel (x, y) of the drawing's own space; empty outside the layer."""
    if not (0 <= x < layer["w"] and 0 <= y < layer["h"]):
        return EMPTY
    (dx, dy), (px, py) = push(center, amplitude, wavelength, speed, phase, fade, frame_no, x, y,
                              (layer["w"], layer["h"]))
    if dx == 0 and dy == 0:
        return layer["px"][y * layer["w"] + x]
    return bilinear(layer, px + dx, py + dy)


# --- the drawing ----------------------------------------------------------------------------

DRAWINGS = {"stripes": [[stripes(x, y) for x in range(W)] for y in range(H)]}


def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


# --- the cases ------------------------------------------------------------------------------

def case(center=(50, 50), amplitude=5, wavelength=30, speed=20, phase=0, fade=0, shift=0):
    return {"drawing": "stripes", "center": center, "amplitude": amplitude,
            "wavelength": wavelength, "speed": speed, "phase": phase, "fade": fade,
            "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def settings(c, frame_no):
    center = [min(CENTER[1], max(CENTER[0], v)) for v in value_at(c["center"], frame_no)]
    return [center] + [held(c, k, frame_no) for k in NAMES[1:]]


def render(c, frame_no):
    layer = drawn_layer(c["drawing"])
    n = settings(c, frame_no)
    return [rippled(layer, *n, frame_no, x - c["shift"], y) for y in range(H) for x in range(W)]


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(amplitude=0, shift=c["shift"]), 0)


RING = {"amplitude": 1, "wavelength": 4, "speed": 0}  # a ring every four pixels, still
MIDDLE = (53.125, 55)  # the point (8.5, 5.5), the centre of pixel (8, 5)

CASES = {
    "FX-RIPPLE-001": ("The settings as they start: centre 50, 50, the point (8, 5), amplitude 5, "
                      "wavelength 30, speed 20, phase 0, fade 0. Each pixel reads from up to "
                      "five pixels along its line through the centre. On frame 0 every pixel "
                      "reads from further out, so the drawing is drawn in toward the centre, "
                      "a small patch with empty all round it. The rings move outward: the "
                      "still ring, where nothing moves, is at the centre on frame 0, 3.33 "
                      "pixels out on frame 2 and 6.67 on frame 4, and inside it each pixel "
                      "reads from nearer the centre, so the middle of the band is magnified "
                      "while the rest is still drawn in; the three frames differ.",
                      case(), [0, 2, 4]),
    "FX-RIPPLE-002": ("Amplitude 0: the drawing, untouched, on every frame.",
                      case(amplitude=0), [0, 4]),
    "FX-RIPPLE-003": ("Amplitude 1, wavelength 4, speed 0: a ring every four pixels, each pixel "
                      "reading from at most one pixel away, so the stripes wobble in rings "
                      "about the centre; with speed 0 the rings hold still, the same on frames "
                      "0, 2 and 4.",
                      case(**RING), [0, 2, 4]),
    "FX-RIPPLE-004": ("FX-RIPPLE-003 with speed 90: frame 0 is FX-RIPPLE-003; frame 2, half a "
                      "turn on, is phase 180, FX-RIPPLE-005; and frame 4, a full turn on, is "
                      "FX-RIPPLE-003 again, to within rounding.",
                      case(**{**RING, "speed": 90}), [0, 2, 4]),
    "FX-RIPPLE-005": ("FX-RIPPLE-003 with phase 180: half a turn, so every pixel reads from the "
                      "same distance the other way, out where FX-RIPPLE-003 reads in.",
                      case(phase=180, **RING), [0]),
    "FX-RIPPLE-006": ("FX-RIPPLE-003 with wavelength 8: the rings twice as far apart, a gentler "
                      "wobble.",
                      case(**{**RING, "wavelength": 8}), [0]),
    "FX-RIPPLE-007": ("FX-RIPPLE-003 with speed -90: the rings move inward. Frame 1, a quarter "
                      "turn back, differs from speed 90's frame 1, a quarter turn on; frame 2, "
                      "half a turn back, is FX-RIPPLE-004's frame 2, half a turn on, to within "
                      "rounding.",
                      case(**{**RING, "speed": -90}), [1, 2]),
    "FX-RIPPLE-008": ("Speed 0, fade 6: the reach dies away with the distance from the centre, "
                      "to nothing at six pixels, so every pixel six or more from (8, 5) is the "
                      "drawing's own, and every nearer pixel reads from less far than it would "
                      "with fade 0.",
                      case(speed=0, fade=6), [0]),
    "FX-RIPPLE-009": ("FX-RIPPLE-003 with the centre at 53.125, 55, the middle of pixel (8, 5): "
                      "that pixel, at no distance from the centre, is its own; the rest ripple "
                      "about it.",
                      case(center=MIDDLE, **RING), [0]),
    "FX-RIPPLE-010": ("The centre at 53.125, 55 and fade 0.5, the rest as they start: pixel "
                      "(8, 5) is at the centre and every other pixel at least a pixel away, "
                      "past the fade, so nothing moves and the drawing is untouched on "
                      "every frame.",
                      case(center=MIDDLE, fade=0.5), [0, 2]),
    "FX-RIPPLE-011": ("FX-RIPPLE-003 with the centre 0, 0, the top left corner: the rings are "
                      "quarter circles about the corner.",
                      case(center=(0, 0), **RING), [0]),
    "FX-RIPPLE-012": ("FX-RIPPLE-003 with the centre keyed from 50, 50 at frame 0 to 0, 0 at "
                      "frame 4, linear: frame 0 is FX-RIPPLE-003, frame 2 rings about 25, 25, "
                      "the point (4, 2.5), and frame 4 is FX-RIPPLE-011.",
                      case(center=keyed((0, (50, 50)), (4, (0, 0))), **RING), [0, 2, 4]),
    "FX-RIPPLE-013": ("Wavelength 4, speed 0, amplitude keyed from 0 at frame 0 to 2 at frame "
                      "4, linear: frame 0 is the drawing, frame 2 is amplitude 1, FX-RIPPLE-003, "
                      "and frame 4 amplitude 2.",
                      case(amplitude=keyed((0, 0), (4, 2)), wavelength=4, speed=0), [0, 2, 4]),
    "FX-RIPPLE-014": ("Fade keyed from 20 at frame 0 to 0 at frame 4, eased past its end (-6.5 "
                      "at frame 2), the rest as they start: frame 0 is fade 20; frame 2 is held "
                      "at 0, no fade, and is FX-RIPPLE-001's frame 2; frame 4 is FX-RIPPLE-001's "
                      "frame 4.",
                      case(fade=keyed((0, 20, OVERSHOOT), (4, 0))), [0, 2, 4]),
    "FX-RIPPLE-015": ("FX-RIPPLE-001 moved three pixels right: the rings are worked in the "
                      "drawing's own space, so they move with it, and the three columns left of "
                      "the drawing stay empty, as the layer does not grow.",
                      case(shift=3), [0, 2]),
    "FX-RIPPLE-016": ("Amplitude 1, wavelength 1, speed 0: a ring every pixel, so neighbouring "
                      "pixels read from unalike places and the stripes break up.",
                      case(amplitude=1, wavelength=1, speed=0), [0]),
    "FX-RIPPLE-017": ("Speed 360, the rest as they start: a full turn every frame, so the "
                      "ripple seems to stand still, every frame FX-RIPPLE-001's frame 0 to "
                      "within rounding.",
                      case(speed=360), [0, 2, 4]),
    "FX-RIPPLE-018": ("Speed 0, phase 40: FX-RIPPLE-001's frame 2, which is phase 0 plus two "
                      "frames of 20 degrees.",
                      case(speed=0, phase=40), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-RIPPLE-019": ("Amplitude 1001, above 1000.", case(amplitude=1001)),
    "FX-RIPPLE-020": ("Amplitude -1, below 0.", case(amplitude=-1)),
    "FX-RIPPLE-021": ("Wavelength 0, below 1.", case(wavelength=0)),
    "FX-RIPPLE-022": ("Speed 361, above 360.", case(speed=361)),
    "FX-RIPPLE-023": ("Phase -100001, below -100000.", case(phase=-100001)),
    "FX-RIPPLE-024": ("Fade -1, below 0.", case(fade=-1)),
    "FX-RIPPLE-025": ("Centre 50, 1001, past ten heights.", case(center=(50, 1001))),
    "FX-RIPPLE-026": ("Amplitude keyed to 1500 at frame 4.",
                      case(amplitude=keyed((0, 5), (4, 1500)))),
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
        "instance_id": "fx-0-0", "type_id": "core.ripple", "enabled": True,
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

    (OUT / "expected_ripple.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    moved = lambda f, n: [f[at(x - n, y)] if x >= n else EMPTY  # noqa: E731
                          for y in range(H) for x in range(W)]
    everywhere = [(x, y) for y in range(H) for x in range(W)]
    dist = lambda x, y, cx=8, cy=5: math.hypot(x + 0.5 - cx, y + 0.5 - cy)  # noqa: E731
    size = lambda p: math.hypot(*p[0])  # noqa: E731

    # The rule's own pieces.
    assert push((50, 50), 0, 30, 20, 0, 0, 2, 3, 3)[0] == (0.0, 0.0)
    assert push(MIDDLE, 5, 30, 20, 0, 0, 2, 8, 5)[0] == (0.0, 0.0)  # d == 0, even at phase 40
    (dx, dy), _ = push((50, 50), 5, 30, 0, 0, 0, 0, 11, 5)  # P (11.5, 5.5), d = 3.5355
    d = math.hypot(3.5, 0.5)
    assert abs(dx - 5 * math.sin(2 * math.pi * d / 30) * 3.5 / d) < 1e-12 and dx > 0  # outward
    assert abs(dy - 5 * math.sin(2 * math.pi * d / 30) * 0.5 / d) < 1e-12
    assert push((50, 50), 5, 30, 0, 0, 3, 0, 11, 5)[0][0] < dx  # a fade shrinks it
    assert push((50, 50), 5, 30, 0, 0, 3, 0, 12, 5)[0] == (0.0, 0.0)  # past the fade
    for p in everywhere:  # the push is never more than the amplitude
        assert size(push((50, 50), 5, 30, 20, 0, 0, 3, *p)) <= 5 + 1e-12

    one = c["FX-RIPPLE-001"]
    assert one["0"] != one["2"] and one["2"] != one["4"] and one["0"] != one["4"]
    assert all(one[f] != drawn for f in one)
    for f, still in ((2, 10 / 3), (4, 20 / 3)):  # the still ring moves outward
        assert abs(math.sin(2 * math.pi * still / 30 - math.radians(20 * f))) < 1e-12
    assert max(size(push((50, 50), 5, 30, 20, 0, 0, 0, *p)) for p in everywhere) > 4.5
    assert any(one["0"][at(x, y)][3] < drawn[at(x, y)][3] - 0.1 for x, y in everywhere)
    for x, y in everywhere:  # frame 0 reads outward everywhere; later, inward inside the ring
        (px, py), _ = push((50, 50), 5, 30, 20, 0, 0, 0, x, y)
        assert px * (x + 0.5 - 8) + py * (y + 0.5 - 5) > 0
        for f, still in ((2, 10 / 3), (4, 20 / 3)):
            (px, py), _ = push((50, 50), 5, 30, 20, 0, 0, f, x, y)
            out = px * (x + 0.5 - 8) + py * (y + 0.5 - 5)
            assert out < 0 if dist(x, y) < still else out > 0
    empty = lambda px: sum(p[3] == 0 for p in px)  # noqa: E731
    assert empty(one["0"]) > empty(drawn) + 60 and empty(one["0"]) > empty(one["2"])
    assert c["FX-RIPPLE-002"]["0"] == drawn == c["FX-RIPPLE-002"]["4"]
    three = c["FX-RIPPLE-003"]
    assert three["0"] == three["2"] == three["4"] != drawn
    four, five = c["FX-RIPPLE-004"], c["FX-RIPPLE-005"]["0"]
    assert four["0"] == three["0"] and four["2"] == five != three["0"]
    assert near(four["4"], three["0"]) and four["4"] != four["2"]
    for p in everywhere:  # phase 180 pushes every pixel the other way
        a, b = push((50, 50), 1, 4, 0, 0, 0, 0, *p)[0], push((50, 50), 1, 4, 0, 180, 0, 0, *p)[0]
        assert abs(a[0] + b[0]) < 1e-12 and abs(a[1] + b[1]) < 1e-12
    assert any(size(push((50, 50), 1, 4, 0, 0, 0, 0, *p)) > 0.9 for p in everywhere)
    assert c["FX-RIPPLE-006"]["0"] not in (three["0"], drawn)
    seven = c["FX-RIPPLE-007"]
    assert seven["1"] != render(case(**{**RING, "speed": 90}), 1)
    assert near(seven["2"], four["2"]) and seven["1"] != seven["2"]
    eight = c["FX-RIPPLE-008"]["0"]
    plain_eight = render(case(speed=0), 0)
    moved_in = 0
    for x, y in everywhere:
        if dist(x, y) >= 6:
            assert eight[at(x, y)] == drawn[at(x, y)]
        else:
            faded = size(push((50, 50), 5, 30, 0, 0, 6, 0, x, y))
            full = size(push((50, 50), 5, 30, 0, 0, 0, 0, x, y))
            assert faded < full
            moved_in += eight[at(x, y)] != drawn[at(x, y)]
    assert moved_in > 10 and eight != plain_eight
    nine = c["FX-RIPPLE-009"]["0"]
    assert nine[at(8, 5)] == drawn[at(8, 5)] and nine != drawn and nine != three["0"]
    ten = c["FX-RIPPLE-010"]
    assert ten["0"] == drawn == ten["2"]
    eleven = c["FX-RIPPLE-011"]["0"]
    assert eleven not in (drawn, three["0"])
    twelve = c["FX-RIPPLE-012"]
    assert twelve["0"] == three["0"] and twelve["4"] == eleven
    assert twelve["2"] == render(case(center=(25, 25), **RING), 0) not in (three["0"], eleven)
    thirteen = c["FX-RIPPLE-013"]
    assert thirteen["0"] == drawn and thirteen["2"] == three["0"]
    assert thirteen["4"] == render(case(amplitude=2, wavelength=4, speed=0), 0) != three["0"]
    fourteen = c["FX-RIPPLE-014"]
    assert abs(value_at(keyed((0, 20, OVERSHOOT), (4, 0)), 2) + 6.5) < 0.05
    assert fourteen["0"] == render(case(fade=20), 0) != one["0"]
    assert fourteen["2"] == one["2"] and fourteen["4"] == one["4"]
    fifteen = c["FX-RIPPLE-015"]
    for f in ("0", "2"):
        assert fifteen[f] == moved(one[f], 3)
        assert all(fifteen[f][at(x, y)] == EMPTY for x in range(3) for y in range(H))
    assert moved(drawn, 3) == plain(case(shift=3))
    sixteen = c["FX-RIPPLE-016"]["0"]
    assert sixteen not in (drawn, three["0"])
    seventeen = c["FX-RIPPLE-017"]
    assert all(near(seventeen[f], one["0"]) for f in seventeen)
    assert c["FX-RIPPLE-018"]["0"] == one["2"]
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
