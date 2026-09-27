"""Turbulent displace, worked a second way.

D-127 adds `core.turbulent_displace`, a seeded, animated warp: heat haze, water, a wobble. Each
pixel of the output takes the picture from a point pushed a little way off, up to `amount`
pixels, the push worked out from a smooth noise field, so neighbouring pixels are pushed alike
and straight lines bend into waves. `size` is how many pixels one wave of the field spans;
`complexity` is how many finer layers of wobble are added on top; `evolution` moves the field
through a third direction, one full turn of 360 degrees moving it one cell, and `speed` adds that
many degrees every frame, so with speed not 0 the wobble moves from frame to frame. With `edges`
"transparent" the layer grows by the amount on every side so a picture pushed past its edge is
kept, and a push that reaches past the picture reads emptiness; with "repeat" it does not grow
and a push past the edge reads the nearest edge pixel. It is this program's own method, modelled
on After Effects' Turbulent Displace in spirit and not claimed to match it; nothing is ported.
Document 21 is the rule in words; this file is the reference for the numbers document 25 pins
against it.

The rule. The noise field F is Fractal Noise's (D-128), imported from
`tools/fractal_noise_reference.py`. With amount 0 the output is the input and nothing grows.
Otherwise, with "transparent", the input grows by g = ceil(amount) transparent pixels on every
side; with "repeat", g = 0. Each output pixel, centre P in the drawing's own space (a grown pixel
has negative coordinates), takes the field's point ((X + 0.5) / size, (Y + 0.5) / size, z),
z = (evolution + speed * frame) / 360, frame the composition frame, a hidden value, and
D = amount * (F(seed, 0, point, complexity), F(seed, 1, point, complexity)); the output is
document 21's bilinear sample of the input at P + D, transparent outside it, or with "repeat"
D-109's held sample, the point first held inside the input's pixel centres.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says, drawn below. The drawing goes into `Fixtures/turbulent_displace/media`, the
projects into `Fixtures/turbulent_displace`, and the expected frames into
`Fixtures/turbulent_displace/expected_turbulent_displace.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/turbulent_displace_reference.py
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
from fractal_noise_reference import F, point  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "turbulent_displace"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"amount": (0, 1000), "size": (1, 1000), "complexity": (1, 8),
          "evolution": (-100000, 100000), "speed": (-360, 360), "seed": (0, 100000)}
FLOORED = ("complexity", "seed")
NAMES = ("amount", "size", "complexity", "evolution", "speed", "seed", "edges")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def growth(amount, edges):
    return math.ceil(amount) if edges == "transparent" else 0


def push(amount, size, complexity, evolution, speed, seed, frame_no, x, y):
    """D at pixel (x, y) of the drawing's own space."""
    p = point(x, y, size, evolution, speed, frame_no)
    return amount * F(seed, 0, *p, complexity), amount * F(seed, 1, *p, complexity)


def displaced(layer, amount, size, complexity, evolution, speed, seed, edges, frame_no, x, y):
    """The output at pixel (x, y) of the drawing's own space; empty outside the grown layer."""
    if amount == 0:
        return layer["px"][y * layer["w"] + x] if 0 <= x < layer["w"] and 0 <= y < layer["h"] \
            else EMPTY
    g = growth(amount, edges)
    if not (-g <= x < layer["w"] + g and -g <= y < layer["h"] + g):
        return EMPTY
    dx, dy = push(amount, size, complexity, evolution, speed, seed, frame_no, x, y)
    return bilinear(layer, x + 0.5 + dx, y + 0.5 + dy, edges)


# --- the drawing ----------------------------------------------------------------------------

LINE, SKIN, SOFT_SKIN, NONE = R.LINE, R.SKIN, (246, 214, 190, 128), S.NONE
BAND = (58, 111, 216, 255)  # #3a6fd8, the ball's blue band


def stripes(x, y):
    """Rows 1 to 8, columns 0 to 13: upright stripes two pixels wide, line and skin by turns,
    crossed by a blue band in rows 4 and 5, so a push either way shows; the stripes touch the
    drawing's left edge. Column 14 is the skin at half covering; column 15 and rows 0 and 9 are
    empty."""
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

def case(amount=10, size=60, complexity=2, evolution=0, speed=20, seed=0, edges="transparent",
         shift=0):
    return {"drawing": "stripes", "amount": amount, "size": size, "complexity": complexity,
            "evolution": evolution, "speed": speed, "seed": seed, "edges": edges, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = min(hi, max(lo, value_at(c[k], frame_no)))
    return math.floor(v) if k in FLOORED else v


def render(c, frame_no):
    layer = drawn_layer(c["drawing"])
    n = [held(c, k, frame_no) for k in NAMES[:-1]]
    return [displaced(layer, *n, c["edges"], frame_no, x - c["shift"], y)
            for y in range(H) for x in range(W)]


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(amount=0, shift=c["shift"]), 0)


WAVE = {"amount": 3, "size": 8}  # three pixels at most, a wave every eight: the stripes bend

CASES = {
    "FX-TURB-001": ("The settings as they start: amount 10, size 60, complexity 2, evolution "
                    "0, speed 20, seed 0, edges transparent. A wave is wider than the drawing, "
                    "so each pixel reads from two to four pixels left of it and two to five "
                    "below, not quite alike everywhere: the drawing is carried right and up and "
                    "its stripes lean and bend gently; the warp moves a little from frame 0 to "
                    "2 to 4. The first three columns read from past the drawing's left edge and "
                    "are empty on every frame.",
                    case(), [0, 2, 4]),
    "FX-TURB-002": ("Amount 0: the drawing, untouched, and nothing grows.",
                    case(amount=0), [0, 2]),
    "FX-TURB-003": ("Amount 3, size 8: a wave every eight pixels, so the stripes bend into "
                    "waves and the blue band wobbles; frame 4 is warped differently from frame "
                    "0.",
                    case(**WAVE), [0, 4]),
    "FX-TURB-004": ("Amount 3, size 8, speed 0: the warp holds still, the same on frames 0, "
                    "2 and 4, and is FX-TURB-003's frame 0.",
                    case(speed=0, **WAVE), [0, 2, 4]),
    "FX-TURB-005": ("FX-TURB-004 with edges repeat: the layer does not grow, and a push past "
                    "the drawing's edge reads the nearest edge pixel, so where FX-TURB-004 "
                    "reads emptiness at the left edge this reads the stripe there.",
                    case(speed=0, edges="repeat", **WAVE), [0]),
    "FX-TURB-006": ("Complexity 1, amount 3, size 8, speed 0: one octave, a smoother warp "
                    "than FX-TURB-004's two.",
                    case(complexity=1, speed=0, **WAVE), [0]),
    "FX-TURB-007": ("Complexity 8, amount 3, size 8, speed 0: eight octaves, a rougher warp.",
                    case(complexity=8, speed=0, **WAVE), [0]),
    "FX-TURB-008": ("Size 1, amount 3, speed 0: a wave every pixel, so the drawing breaks up "
                    "into a jumble.",
                    case(amount=3, size=1, speed=0), [0]),
    "FX-TURB-009": ("Size 1000, amount 3, speed 0: the wave is so wide that every pixel is "
                    "pushed alike, within a hundredth of a pixel: the drawing slides whole.",
                    case(amount=3, size=1000, speed=0), [0]),
    "FX-TURB-010": ("Amount 3, size 8, speed 90: frame 0 is FX-TURB-004; frame 2 is evolution "
                    "180, and frame 4 is evolution 360, FX-TURB-011.",
                    case(speed=90, **WAVE), [0, 2, 4]),
    "FX-TURB-011": ("Amount 3, size 8, evolution 360, speed 0: one full turn moves the field "
                    "one cell through its third direction, a warp of its own.",
                    case(evolution=360, speed=0, **WAVE), [0]),
    "FX-TURB-012": ("Amount 3, size 8, speed 0, seed 8: a warp of its own.",
                    case(seed=8, speed=0, **WAVE), [0]),
    "FX-TURB-013": ("Seed 8.5, which counts as 8: FX-TURB-012.",
                    case(seed=8.5, speed=0, **WAVE), [0]),
    "FX-TURB-014": ("Complexity 2.7, which counts as 2: FX-TURB-004.",
                    case(complexity=2.7, speed=0, **WAVE), [0]),
    "FX-TURB-015": ("Size 8, speed 0, amount keyed from 0 at frame 0 to 4 at frame 4, linear: "
                    "frame 0 is the drawing, frame 2 is amount 2, and frame 4 amount 4.",
                    case(amount=keyed((0, 0), (4, 4)), size=8, speed=0), [0, 2, 4]),
    "FX-TURB-016": ("Amount 3, size 8, speed keyed from 0 at frame 0 to 360 at frame 4, eased "
                    "past its end (477 at frame 2): frame 0 is FX-TURB-004, and frame 2 is held "
                    "at 360, so it is speed 360's frame 2.",
                    case(speed=keyed((0, 0, OVERSHOOT), (4, 360)), **WAVE), [0, 2, 4]),
    "FX-TURB-017": ("FX-TURB-012 moved three pixels right: the warp is worked in the drawing's "
                    "own space, so it moves with it, and the three columns left of the drawing "
                    "show the grown pixels, into which the stripes are pushed.",
                    case(seed=8, speed=0, shift=3, **WAVE), [0]),
    "FX-TURB-018": ("FX-TURB-005, edges repeat, moved three pixels right: nothing grows, so "
                    "the three columns left of the drawing stay empty.",
                    case(speed=0, edges="repeat", shift=3, **WAVE), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-TURB-019": ("Amount 1001, above 1000.", case(amount=1001)),
    "FX-TURB-020": ("Size 0, below 1.", case(size=0)),
    "FX-TURB-021": ("Complexity 0, below 1.", case(complexity=0)),
    "FX-TURB-022": ("Speed -361, below -360.", case(speed=-361)),
    "FX-TURB-023": ("Seed 100001, above 100000.", case(seed=100001)),
    "FX-TURB-024": ("Amount keyed to 1500 at frame 4.", case(amount=keyed((0, 10), (4, 1500)))),
    "FX-TURB-025": ("Edges \"wrap\", which is not a choice.", case(edges="wrap")),
    "FX-TURB-026": ("Edges \"Repeat\": the word is exact, so a capital is not the choice.",
                    case(edges="Repeat")),
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
        "instance_id": "fx-0-0", "type_id": "core.turbulent_displace", "enabled": True,
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

    (OUT / "expected_turbulent_displace.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                          encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    layer = drawn_layer("stripes")
    moved = lambda f, n: [f[at(x - n, y)] if x >= n else EMPTY  # noqa: E731
                          for y in range(H) for x in range(W)]

    # The rule's own pieces.
    assert growth(10, "transparent") == 10 and growth(2.1, "transparent") == 3
    assert growth(10, "repeat") == 0
    assert displaced(layer, 3, 8, 2, 0, 0, 0, "transparent", 0, -4, 4) == EMPTY  # past g
    assert displaced(layer, 3, 8, 2, 0, 0, 0, "repeat", 0, -1, 4) == EMPTY  # no growth
    dx, dy = push(3, 8, 2, 0, 0, 0, 0, 5, 5)
    assert (dx, dy) == (3 * F(0, 0, 5.5 / 8, 5.5 / 8, 0, 2), 3 * F(0, 1, 5.5 / 8, 5.5 / 8, 0, 2))
    assert dx != dy

    one = c["FX-TURB-001"]
    assert one["0"] != one["2"] and one["2"] != one["4"] and one["0"] != one["4"]
    assert all(one[f] != drawn for f in one)
    for f in (0, 2, 4):
        d = [push(10, 60, 2, 0, 20, 0, f, x, y) for x in range(W) for y in range(H)]
        assert all(-4 < p[0] < -1.5 and 2.5 < p[1] < 5 for p in d)
        assert max(p[0] for p in d) - min(p[0] for p in d) > 1  # not alike: it bends
        assert all(one[str(f)][at(x, y)] == EMPTY for x in range(3) for y in range(H))
    assert c["FX-TURB-002"]["0"] == drawn == c["FX-TURB-002"]["2"]
    three, four = c["FX-TURB-003"], c["FX-TURB-004"]
    assert three["0"] != three["4"] and three["0"] == four["0"] == four["2"] == four["4"]
    # The warp bends: two pixels of one column are pushed differently.
    pushes = [push(3, 8, 2, 0, 0, 0, 0, 5, y) for y in range(H)]
    assert max(p[0] for p in pushes) - min(p[0] for p in pushes) > 0.5
    five = c["FX-TURB-005"]["0"]
    # Repeat: every pixel whose sample lies inside the drawing's pixel centres is FX-TURB-004's;
    # one whose sample falls left of the drawing reads the stripe there, not emptiness.
    same = differ = 0
    for y in range(H):
        for x in range(W):
            px, py = push(3, 8, 2, 0, 0, 0, 0, x, y)
            sx, sy = x + 0.5 + px, y + 0.5 + py
            if 0.5 <= sx <= W - 0.5 and 0.5 <= sy <= H - 0.5:
                assert near([five[at(x, y)]], [four["0"][at(x, y)]]), (x, y)
                same += 1
            elif sx < 0.5 and 1.5 <= sy <= 7.5:
                assert five[at(x, y)][3] > four["0"][at(x, y)][3] + 1e-6, (x, y)
                differ += 1
    assert same > 100 and differ > 0
    assert c["FX-TURB-006"]["0"] != four["0"] and c["FX-TURB-007"]["0"] != four["0"]
    wide = [push(3, 1000, 2, 0, 0, 0, 0, x, y) for x in range(W) for y in range(H)]
    for k in (0, 1):
        assert max(p[k] for p in wide) - min(p[k] for p in wide) < 0.01
    ten = c["FX-TURB-010"]
    assert ten["0"] == four["0"] and ten["4"] == c["FX-TURB-011"]["0"] != four["0"]
    assert ten["2"] == render(case(evolution=180, speed=0, **WAVE), 0) != ten["0"]
    assert c["FX-TURB-012"]["0"] != four["0"] and c["FX-TURB-013"]["0"] == c["FX-TURB-012"]["0"]
    assert c["FX-TURB-014"]["0"] == four["0"]
    fifteen = c["FX-TURB-015"]
    assert fifteen["0"] == drawn
    assert fifteen["2"] == render(case(amount=2, size=8, speed=0), 0)
    assert fifteen["4"] == render(case(amount=4, size=8, speed=0), 0)
    sixteen = c["FX-TURB-016"]
    assert sixteen["0"] == four["0"]
    assert sixteen["2"] == render(case(speed=360, **WAVE), 2)
    assert sixteen["4"] == render(case(speed=360, **WAVE), 4) != sixteen["2"]
    seventeen, eighteen = c["FX-TURB-017"]["0"], c["FX-TURB-018"]["0"]
    for x in range(3, W):
        for y in range(H):
            assert seventeen[at(x, y)] == c["FX-TURB-012"]["0"][at(x - 3, y)]
            assert eighteen[at(x, y)] == five[at(x - 3, y)]
    assert any(seventeen[at(x, y)][3] > 0 for x in range(3) for y in range(H))  # grown, shown
    assert all(eighteen[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    assert moved(drawn, 3) == plain(case(shift=3))
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
