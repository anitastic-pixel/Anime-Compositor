"""Camera shake, worked a second way.

D-162 adds `core.camera_shake`, a hand-held or impact shake: the whole drawing jolts to a new
place every frame, as if the camera were bumped, the staple of an anime hit or an explosion.
Each frame the drawing is moved by up to `amount` pixels each way and, with `rotation` not 0,
turned about its centre by up to that many degrees either way, both picked by Noise's seeded
hash from the frame, so the same seed gives the same shake every time. `hold` keeps each jolt
for that many frames (2 is a shake "on twos", as anime animates), and `seed` picks another shake.
The layer grows so that nothing moved or turned is cut off. It is this program's own method,
modelled on After Effects' Wiggle-driven camera shakes in spirit and not claimed to match them;
nothing is ported. Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it.

The rule. With amount 0 and rotation 0 the output is the input and nothing grows. Otherwise
m = floor(frame / hold), frame the composition frame, a hidden value, D = amount (U(seed, m, 0,
0, 0), U(seed, m, 0, 0, 1)) and beta = radians(rotation U(seed, m, 0, 0, 2)), U Noise's hash in
-1..1, imported from `tools/noise_reference.py`. c = o + (w / 2, h / 2) is the drawing's centre
and rho the largest distance from c to the input buffer's corners. The input first grows by
g = ceil(amount sqrt(2) + 2 rho sin(radians(rotation) / 2)) transparent pixels on every side;
the output at a pixel with centre P, in the drawing's own space (a grown pixel has negative
coordinates), is document 21's bilinear sample of the input at c + R(-beta)(P - c - D),
transparent outside it: the drawing moved by D and turned by beta, clockwise for beta above 0,
about its centre. `hold` and `seed` are held in their ranges and then floored.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: Wave Warp's stripes, imported from `tools/wave_warp_reference.py`. The drawing
goes into `Fixtures/camera_shake/media`, the projects into `Fixtures/camera_shake`, and the
expected frames into `Fixtures/camera_shake/expected_camera_shake.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/camera_shake_reference.py
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
from noise_reference import u as U  # noqa: E402
from wave_warp_reference import DRAWINGS  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "camera_shake"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"amount": (0, 1000), "rotation": (0, 45), "hold": (1, 100), "seed": (0, 100000)}
FLOORED = ("hold", "seed")
NAMES = ("amount", "rotation", "hold", "seed")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def jolt(amount, rotation, hold, seed, frame_no):
    """D and beta for this frame; hold and seed already floored."""
    m = frame_no // hold
    D = (amount * U(seed, m, 0, 0, 0), amount * U(seed, m, 0, 0, 1))
    return D, math.radians(rotation * U(seed, m, 0, 0, 2))


def centre(layer):
    return layer["w"] / 2, layer["h"] / 2  # o = (0, 0): the drawing is the input buffer here


def growth(layer, amount, rotation):
    if amount == 0 and rotation == 0:
        return 0
    cx, cy = centre(layer)
    rho = max(math.hypot(x - cx, y - cy) for x in (layer["left"], layer["left"] + layer["w"])
              for y in (layer["top"], layer["top"] + layer["h"]))
    return math.ceil(amount * math.sqrt(2) + 2 * rho * math.sin(math.radians(rotation) / 2))


def source(layer, D, beta, X, Y):
    """Where the output point (X, Y) reads the input: c + R(-beta)(P - c - D)."""
    cx, cy = centre(layer)
    vx, vy = X - cx - D[0], Y - cy - D[1]
    cb, sb = math.cos(beta), math.sin(beta)
    return cx + vx * cb + vy * sb, cy - vx * sb + vy * cb


def shaken(layer, amount, rotation, hold, seed, frame_no, x, y, cut=True):
    """The output at pixel (x, y) of the drawing's own space; empty outside the grown layer.
    `cut=False` samples past the grown layer too, for the check that nothing is cut."""
    inside = 0 <= x < layer["w"] and 0 <= y < layer["h"]
    if amount == 0 and rotation == 0:
        return layer["px"][y * layer["w"] + x] if inside else EMPTY
    g = growth(layer, amount, rotation)
    if cut and not (-g <= x < layer["w"] + g and -g <= y < layer["h"] + g):
        return EMPTY
    D, beta = jolt(amount, rotation, hold, seed, frame_no)
    return bilinear(layer, *source(layer, D, beta, x + 0.5, y + 0.5))


# --- the drawing ----------------------------------------------------------------------------

def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


# --- the cases ------------------------------------------------------------------------------

def case(amount=10, rotation=0, hold=1, seed=0, shift=0):
    return {"drawing": "stripes", "amount": amount, "rotation": rotation, "hold": hold,
            "seed": seed, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = min(hi, max(lo, value_at(c[k], frame_no)))
    return math.floor(v) if k in FLOORED else v


def render(c, frame_no, cut=True):
    layer = drawn_layer(c["drawing"])
    n = [held(c, k, frame_no) for k in NAMES]
    return [shaken(layer, *n, frame_no, x - c["shift"], y, cut)
            for y in range(H) for x in range(W)]


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(amount=0, shift=c["shift"]), 0)


ALL = [0, 1, 2, 3, 4]

CASES = {
    "FX-SHAKE-001": ("The settings as they start: amount 10, rotation 0, hold 1, seed 0. Each "
                     "frame the drawing jolts to a new place, up to 10 pixels each way, larger "
                     "than the drawing itself: frame 0 moves it 0.57 pixels left and 2.96 down, "
                     "frame 1 9.51 right and 9.94 up, out of the frame, so frame 1 is empty, "
                     "frame 2 6.04 left and 1.57 up, frame 3 1.54 right and 0.47 down, and "
                     "frame 4 8.05 left and 6.89 down.",
                     case(), ALL),
    "FX-SHAKE-002": ("Amount 0 and rotation 0: the drawing, untouched, and nothing grows.",
                     case(amount=0), [0, 2]),
    "FX-SHAKE-003": ("Amount 2: the same jolts at a fifth of the size, so the drawing stays in "
                     "the frame and each frame shows it moved a little differently, by a fifth "
                     "of FX-SHAKE-001's move: frame 0 by 0.11 left and 0.59 down, frame 1 by "
                     "1.90 right and 1.99 up.",
                     case(amount=2), ALL),
    "FX-SHAKE-004": ("Amount 2, hold 2: each jolt is kept for two frames, a shake on twos: "
                     "frames 0 and 1 are FX-SHAKE-003's frame 0, frames 2 and 3 its frame 1, "
                     "and frame 4 its frame 2.",
                     case(amount=2, hold=2), ALL),
    "FX-SHAKE-005": ("Amount 2, hold 5: one jolt kept for all five frames, each FX-SHAKE-003's "
                     "frame 0.",
                     case(amount=2, hold=5), [0, 2, 4]),
    "FX-SHAKE-006": ("Amount 2, hold 1.9, floored to 1: FX-SHAKE-003 exactly.",
                     case(amount=2, hold=1.9), [0, 2]),
    "FX-SHAKE-007": ("Amount 2, seed 1: another shake, frame 0 moving the drawing 1.18 pixels "
                     "left and 1.72 up where FX-SHAKE-003's frame 0 moves it down.",
                     case(amount=2, seed=1), [0, 2]),
    "FX-SHAKE-008": ("Amount 2, seed 1.6, floored to 1: FX-SHAKE-007 exactly.",
                     case(amount=2, seed=1.6), [0, 2]),
    "FX-SHAKE-009": ("Amount 0, rotation 10: the drawing is not moved, only turned about its "
                     "centre, frame 0 by 9.61 degrees anticlockwise, so the blue band tilts up "
                     "to the right, and frame 2 by 3.47 degrees clockwise, so it tilts down to "
                     "the right.",
                     case(amount=0, rotation=10), [0, 2]),
    "FX-SHAKE-010": ("Amount 2, rotation 10: moved as FX-SHAKE-003 and turned as FX-SHAKE-009, "
                     "different from both.",
                     case(amount=2, rotation=10), [0, 2]),
    "FX-SHAKE-011": ("Amount keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the "
                     "drawing, frame 2 is amount 2, FX-SHAKE-003's frame 2, and frame 4 amount "
                     "4.",
                     case(amount=keyed((0, 0), (4, 4))), [0, 2, 4]),
    "FX-SHAKE-012": ("Amount 0, rotation keyed from 0 at frame 0 to 45 at frame 4, eased past "
                     "its end (about 60 at frame 2): frame 0 is the drawing, and frame 2 is "
                     "held at 45, so it is rotation 45's frame 2.",
                     case(amount=0, rotation=keyed((0, 0, OVERSHOOT), (4, 45))), [0, 2, 4]),
    "FX-SHAKE-013": ("Amount 3, seed 7, moved three pixels right: frame 0 moves the drawing "
                     "2.73 pixels left and 0.25 down, into the grown pixels, and the three "
                     "columns left of the drawing show them in place; every other column is the "
                     "unmoved layer's, three columns on.",
                     case(amount=3, seed=7, shift=3), [0]),
    "FX-SHAKE-014": ("Amount 0, rotation 45, moved three pixels right: frame 0 turns the "
                     "drawing 43.25 degrees anticlockwise, and its top-left corner swings down "
                     "and out past its left edge into the grown pixels, shown in rows 5 to 7 of "
                     "the columns left of the drawing.",
                     case(amount=0, rotation=45, shift=3), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-SHAKE-015": ("Amount 1001, above 1000.", case(amount=1001)),
    "FX-SHAKE-016": ("Amount -1, below 0.", case(amount=-1)),
    "FX-SHAKE-017": ("Rotation 46, above 45.", case(rotation=46)),
    "FX-SHAKE-018": ("Hold 0, below 1.", case(hold=0)),
    "FX-SHAKE-019": ("Hold 101, above 100.", case(hold=101)),
    "FX-SHAKE-020": ("Seed 100001, above 100000.", case(seed=100001)),
    "FX-SHAKE-021": ("Amount keyed to 1500 at frame 4.", case(amount=keyed((0, 10), (4, 1500)))),
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
        "instance_id": "fx-0-0", "type_id": "core.camera_shake", "enabled": True,
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

    (OUT / "expected_camera_shake.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    layer = drawn_layer("stripes")
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    blue = lambda p: p[2] > p[0] + 0.1  # noqa: E731  the band, not skin or line
    close = lambda v, w: abs(v - w) < 5e-3  # noqa: E731  the says' two decimals

    # The rule's own pieces.
    assert growth(layer, 0, 0) == 0 and growth(layer, 10, 0) == 15 and growth(layer, 3, 0) == 5
    assert growth(layer, 0, 45) == math.ceil(2 * math.hypot(8, 5) * math.sin(math.pi / 8)) == 8
    assert source(layer, (0, 0), 0, 3.5, 2.5) == (3.5, 2.5)
    assert source(layer, (1.5, -2), 0, 3.5, 2.5) == (2.0, 4.5)  # moved by D: read from P - D
    sx, sy = source(layer, (0, 0), math.pi / 2, 8, 0)  # a quarter turn clockwise: the top of
    assert abs(sx - 3) < 1e-12 and abs(sy - 5) < 1e-12  # the centre reads from its left
    for f in range(12):
        D, beta = jolt(10, 45, 3, 0, f)
        assert (D, beta) == jolt(10, 45, 3, 0, f - f % 3)  # a jolt kept for three frames
        assert max(map(abs, D)) <= 10 and abs(beta) <= math.radians(45)
    moves = [jolt(10, 10, 1, 0, f) for f in ALL]
    want = [(-0.57, 2.96), (9.51, -9.94), (-6.04, -1.57), (1.54, 0.47), (-8.05, 6.89)]
    assert all(close(D[0], a) and close(D[1], b) for (D, _), (a, b) in zip(moves, want))
    assert close(math.degrees(moves[0][1]), -9.61) and close(math.degrees(moves[2][1]), 3.47)
    D7, _ = jolt(3, 0, 1, 7, 0)
    assert close(D7[0], -2.73) and close(D7[1], 0.25)
    D1, _ = jolt(2, 0, 1, 1, 0)
    assert close(D1[0], -1.18) and close(D1[1], -1.72)
    assert close(math.degrees(jolt(0, 45, 1, 0, 0)[1]), -43.25)

    # Nothing is cut: every case's pixels are the same sampled past the grown layer.
    for fx, (_, cs, frames) in CASES.items():
        for f in frames:
            assert c[fx][str(f)] == render(cs, f, cut=False), fx

    one = c["FX-SHAKE-001"]
    assert len({json.dumps(one[str(f)]) for f in ALL}) == 5
    assert all(p == EMPTY for p in one["1"])
    assert c["FX-SHAKE-002"]["0"] == drawn == c["FX-SHAKE-002"]["2"]
    three = c["FX-SHAKE-003"]
    assert len({json.dumps(three[str(f)]) for f in ALL} | {json.dumps(drawn)}) == 6
    assert close(jolt(2, 0, 1, 0, 1)[0][0], 1.90) and close(jolt(2, 0, 1, 0, 1)[0][1], -1.99)
    # frame 0 moved 0.59 down: the band's upper row keeps some band, the row below it takes it
    assert blue(three["0"][at(6, 6)]) and not blue(drawn[at(6, 6)])
    four = c["FX-SHAKE-004"]
    assert four["0"] == four["1"] == three["0"] and four["2"] == four["3"] == three["1"]
    assert four["4"] == three["2"]
    five = c["FX-SHAKE-005"]
    assert five["0"] == five["2"] == five["4"] == three["0"]
    assert c["FX-SHAKE-006"]["0"] == three["0"] and c["FX-SHAKE-006"]["2"] == three["2"]
    seven = c["FX-SHAKE-007"]
    assert seven["0"] != three["0"] and seven["2"] != three["2"]
    assert c["FX-SHAKE-008"] == seven
    nine = c["FX-SHAKE-009"]
    # Anticlockwise: the band's right end rises (rows 2 to 3 near the right, 6 to 7 near the left)
    assert blue(nine["0"][at(12, 3)]) and not blue(drawn[at(12, 3)])
    assert blue(nine["0"][at(2, 6)]) and not blue(drawn[at(2, 6)])
    assert not blue(nine["2"][at(12, 3)]) and blue(nine["2"][at(12, 5)])
    assert blue(nine["2"][at(2, 3)]) and blue(nine["2"][at(10, 6)]) and not blue(drawn[at(10, 6)])
    ten = c["FX-SHAKE-010"]
    assert ten["0"] not in (three["0"], nine["0"], drawn) and ten["2"] != nine["2"]
    eleven = c["FX-SHAKE-011"]
    assert eleven["0"] == drawn and eleven["2"] == three["2"]
    assert eleven["4"] == render(case(amount=4), 4) != three["4"]
    twelve = c["FX-SHAKE-012"]
    assert value_at(keyed((0, 0, OVERSHOOT), (4, 45)), 2) > 45
    assert twelve["0"] == drawn
    assert twelve["2"] == render(case(amount=0, rotation=45), 2) != drawn
    assert twelve["4"] == render(case(amount=0, rotation=45), 4)
    thirteen = c["FX-SHAKE-013"]["0"]
    unmoved = render(case(amount=3, seed=7), 0)
    for x in range(3, W):
        assert all(thirteen[at(x, y)] == unmoved[at(x - 3, y)] for y in range(H))
    for x in range(3):
        assert any(thirteen[at(x, y)][3] > 0.2 for y in range(H)), x  # grown, shown
    fourteen = c["FX-SHAKE-014"]["0"]
    # the corner lands left of the drawing, below its middle: rows 5 to 7 there, rows 0 to 4 empty
    assert any(fourteen[at(x, y)][3] > 0.2 for x in range(3) for y in range(5, 8))
    assert all(fourteen[at(x, y)] == EMPTY for x in range(3) for y in range(5))
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
