"""Ripple Pulse, worked a second way.

D-390 adds `core.ripple_pulse`, after CycoreFX's CC Ripple Pulse: rings that run out from a
centre, made by keying Pulse Level, a pebble dropped in a pool. Each change in the level sends a
ring outward; the rings are the level's history, so the picture at a frame depends on the level
at the frames before it. CycoreFX's manual says what each control does in a sentence and
publishes no formula; the rule below is this program's own reading of it, and nothing is ported.

The rule. W and H are the drawing's own size. At composition frame f, with the composition's
frames per second fps:

1. n = floor(`time_span` fps + 0.5) frames of history; L_j, j = 0 to n, is `pulse_level` as it is
   at frame f - j, read through the layer's time stretch as its keys are, its expression run, and
   held to its range (before the first key it is the first key's value).
2. The rings: the change g_j = L_j - L_j+1 made j frames ago has run out to u = j + 1/2, where u
   = n r / R measures the distance r from the centre C (`center`, per cent of W and H) in frames
   of travel and R is half the drawing's diagonal; so a ring takes the time span to reach the
   drawing's corners from its middle. D(u) is g joined by straight lines between those places,
   g_0 inside u = 1/2, and 0 from u = n + 1/2 out (g_n = 0); with n = 0 there are no rings.
3. A pixel's centre P, r = |P - C| > 0, reads the drawing at P - s (P - C) / r, s =
   `amplitude` / 10 D(u): a rise in the level pushes the picture outward from the centre, a fall
   draws it in. Document 21's bilinear sample, transparent outside; P = C reads itself. The layer
   does not grow.
4. `render_bump_map` `on` gives the rings' heights instead, opaque grey everywhere:
   clamp(1/2 + `amplitude` (z(u) - L_n) / 2000, 0, 1) in every channel, z(u) the level L joined by
   straight lines with L_j at u = j, held to L_0 inside and L_n outside. A map for Glass or
   Displacement Map.

`center` -1000 to 1000 per cent each way, 50, 50; `pulse_level` -1000 to 1000, 0; `time_span` 0
to 10 seconds, 1; `amplitude` 0 to 1000, 10; `render_bump_map` `off` or `on`, `off`. The numbers
are keyable; only `pulse_level` is read at the frames before. The values when added are chosen
here. For a draft the amplitude is halved, so the bump map's slopes are halved with it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 at 24 frames a second holding Bulge's striped
drawing, the same size, unmoved unless the case says. The drawing goes into
`Fixtures/ripple_pulse/media`, the projects into `Fixtures/ripple_pulse`, and the expected frames
into `Fixtures/ripple_pulse/expected_ripple_pulse.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/ripple_pulse_reference.py
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
import bulge_reference as B  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402

W, H = R.W, R.H
FPS = 24
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "ripple_pulse"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"center": (-1000, 1000), "pulse_level": (-1000, 1000), "time_span": (0, 10),
          "amplitude": (0, 1000)}
NAMES = ("center", "pulse_level", "time_span", "amplitude", "render_bump_map")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def held(c, k, frame_no):
    v = value_at(c[k], frame_no)
    lo, hi = RANGES[k]
    return (tuple(min(hi, max(lo, u)) for u in v) if isinstance(v, (list, tuple))
            else min(hi, max(lo, v)))


def levels(c, frame_no):
    n = math.floor(held(c, "time_span", frame_no) * FPS + 0.5)
    return [held(c, "pulse_level", frame_no - j) for j in range(n + 1)]


def push(L, u):
    """D(u)."""
    n = len(L) - 1
    g = [L[j] - L[j + 1] for j in range(n)] + [0.0]
    if u <= 0.5:
        return g[0]
    if u >= n + 0.5:
        return 0.0
    j = math.floor(u - 0.5)
    t = u - 0.5 - j
    return g[j] * (1 - t) + g[j + 1] * t


def height(L, u):
    """z(u)."""
    n = len(L) - 1
    if u >= n:
        return L[n]
    j = math.floor(u)
    t = u - j
    return L[j] * (1 - t) + L[j + 1] * t


def ripple_pulse(layer, s_, L):
    cx, cy = s_["center"][0] / 100 * W, s_["center"][1] / 100 * H
    big = math.hypot(W, H) / 2
    n = len(L) - 1
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            x, y = layer["left"] + i + 0.5, layer["top"] + j + 0.5
            dx, dy = x - cx, y - cy
            r = math.hypot(dx, dy)
            u = n * r / big
            if s_["render_bump_map"] == "on":
                v = min(1.0, max(0.0, 0.5 + s_["amplitude"] * (height(L, u) - L[n]) / 2000))
                px.append([v, v, v, 1.0])
                continue
            if r == 0:
                px.append(bilinear(layer, x, y))
                continue
            s = s_["amplitude"] / 10 * push(L, u)
            px.append(bilinear(layer, x - s * dx / r, y - s * dy / r))
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, **more):
    c = {"drawing": "stripes", "shift": shift, "center": (50, 50), "pulse_level": 0,
         "time_span": 1, "amplitude": 10, "render_bump_map": "off"}
    c.update(more)
    return c


def settings(c, frame_no):
    return {"center": held(c, "center", frame_no), "amplitude": held(c, "amplitude", frame_no),
            "render_bump_map": c["render_bump_map"]}


def render(c, frame_no):
    layer = B.drawn_layer(c["drawing"])
    out = ripple_pulse(layer, settings(c, frame_no), levels(c, frame_no))
    return [out["px"][y * W + x - c["shift"]] if 0 <= x - c["shift"] < W else EMPTY
            for y in range(H) for x in range(W)]


def plain(c):
    layer = B.drawn_layer(c["drawing"])
    return [list(layer["px"][y * W + x - c["shift"]]) if 0 <= x - c["shift"] < W else EMPTY
            for y in range(H) for x in range(W)]


# A drop: the level jumps from 0 to 10 at frame 1 and stays; Time Span 0.25 s, 6 frames.
DROP = dict(pulse_level=keyed((0, 0, "hold"), (1, 10)), time_span=0.25, amplitude=2)
RAMP = dict(pulse_level=keyed((0, 0), (4, 20)), time_span=0.25, amplitude=2)

CASES = {
    "FX-RPULSE-001": ("The settings as they start: Pulse Level 0 and never keyed, so there is "
                      "no ring: the drawing, untouched.", case(), [0, 4]),
    "FX-RPULSE-002": ("A drop: the level held at 0 until frame 1, then 10, Time Span 0.25 "
                      "seconds (6 frames), Amplitude 2: frame 0 nothing; at frame 1 the middle "
                      "pushed out 2 pixels; then a ring 2 pixels strong running outward, half "
                      "way to the corners by frame 4.", case(**DROP), [0, 1, 2, 4]),
    "FX-RPULSE-003": ("The same drop downward, 0 to -10: the ring draws the picture in.",
                      case(**dict(DROP, pulse_level=keyed((0, 0, "hold"), (1, -10)))),
                      [1, 4]),
    "FX-RPULSE-004": ("The level rising steadily from 0 at frame 0 to 20 at frame 4: a pulse of "
                      "5 a frame, a growing disc pushed out a pixel.", case(**RAMP), [0, 2, 4]),
    "FX-RPULSE-005": ("FX-RPULSE-002 centred at the left quarter, 25, 50.",
                      case(center=(25, 50), **DROP), [4]),
    "FX-RPULSE-006": ("FX-RPULSE-002 with Render Bump Map on: grey heights instead, the "
                      "middle at 0.51 and the ring's slope down to 0.5 at frame 4.",
                      case(render_bump_map="on", **DROP), [0, 1, 4]),
    "FX-RPULSE-007": ("FX-RPULSE-002 with Time Span 0: no history, so no ring.",
                      case(**dict(DROP, time_span=0)), [1, 4]),
    "FX-RPULSE-008": ("FX-RPULSE-002 with Time Span 1 second (24 frames): the ring runs four "
                      "times slower, still near the middle at frame 4.",
                      case(**dict(DROP, time_span=1)), [4]),
    "FX-RPULSE-009": ("FX-RPULSE-002 with the layer moved three pixels right: the same, moved; "
                      "nothing grows.", case(3, **DROP), [4]),
    "FX-RPULSE-010": ("FX-RPULSE-002 with Amplitude 0: the drawing, untouched.",
                      case(**dict(DROP, amplitude=0)), [4]),
    "FX-RPULSE-011": ("The level eased from 0 at frame 0 to 1000 at frame 4 on a curve that "
                      "overshoots, Amplitude 0.01, Time Span 0.25: the level is held at 1000 "
                      "before the history is read, so frame 4 is the history 0, ..., 1000, 1000.",
                      case(pulse_level=keyed((0, 0, OVERSHOOT), (4, 1000)), time_span=0.25,
                           amplitude=0.01), [2, 4]),
    "FX-RPULSE-012": ("FX-RPULSE-006's bump map with Amplitude 200: the middle's grey held at "
                      "white.", case(**dict(DROP, render_bump_map="on", amplitude=200)), [4]),
}

INVALID = {
    "FX-RPULSE-013": ("Pulse Level 1001, above 1000.", case(pulse_level=1001)),
    "FX-RPULSE-014": ("Time Span 11, above 10 seconds.", case(time_span=11)),
    "FX-RPULSE-015": ("Time Span -1, below 0.", case(time_span=-1)),
    "FX-RPULSE-016": ("Amplitude 1001, above 1000.", case(amplitude=1001)),
    "FX-RPULSE-017": ("Render Bump Map \"yes\", not a word it takes.",
                      case(render_bump_map="yes")),
    "FX-RPULSE-018": ("Centre at 50, 1001, past ten heights.", case(center=(50, 1001))),
    "FX-RPULSE-019": ("Pulse Level keyed to -2000 at frame 4.",
                      case(pulse_level=keyed((0, 0), (4, -2000)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    assert comp["frame_rate"] == {"numerator": FPS, "denominator": 1}
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.ripple_pulse", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in B.DRAWINGS.items():
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

    (OUT / "expected_ripple_pulse.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    layer = B.drawn_layer("stripes")

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    # The history and the rings.
    drop = case(**DROP)
    assert levels(drop, 0) == [0] * 7 and levels(drop, 1) == [10] + [0] * 6
    assert levels(drop, 4) == [10] * 4 + [0] * 3
    L = levels(drop, 4)
    assert push(L, 0.5) == 0 and push(L, 3.5) == 10 and push(L, 3) == 5 and push(L, 4) == 5
    assert push(L, 6.5) == 0 and push(L, 100) == 0
    assert push([5], 0) == 0 and height([5], 3) == 5
    assert height(L, 0) == 10 and height(L, 3.5) == 5 and height(L, 6) == 0

    assert all(f == drawn for f in c["FX-RPULSE-001"].values())
    two = c["FX-RPULSE-002"]
    assert two["0"] == drawn and two["1"] != drawn and two["4"] != two["2"] != two["1"]
    # Frame 1: every pixel within half a frame's travel of the middle moves 2 pixels out.
    big = math.hypot(W, H) / 2
    for x in range(W):
        for y in range(H):
            dx, dy = x + 0.5 - W / 2, y + 0.5 - H / 2
            r = math.hypot(dx, dy)
            if 0 < r <= big / 12:
                assert near(two["1"][at(x, y)], bilinear(layer, x + 0.5 - 2 * dx / r,
                                                         y + 0.5 - 2 * dy / r))
    assert c["FX-RPULSE-003"]["4"] != two["4"]
    four = c["FX-RPULSE-004"]
    assert four["0"] == drawn and four["2"] != drawn
    assert c["FX-RPULSE-005"]["4"] != two["4"]
    six = c["FX-RPULSE-006"]
    assert all(p == [0.5, 0.5, 0.5, 1.0] for p in six["0"])
    assert near(six["4"][at(8, 5)], [0.51, 0.51, 0.51, 1.0])
    assert all(f == drawn for f in c["FX-RPULSE-007"].values())
    assert c["FX-RPULSE-008"]["4"] not in (drawn, two["4"])
    nine = c["FX-RPULSE-009"]["4"]
    assert all(nine[at(x + 3, y)] == two["4"][at(x, y)] for x in range(W - 3) for y in range(H))
    assert c["FX-RPULSE-010"]["4"] == drawn
    eleven = case(pulse_level=keyed((0, 0, OVERSHOOT), (4, 1000)), time_span=0.25)
    assert value_at(eleven["pulse_level"], 3) > 1000 and levels(eleven, 4)[:2] == [1000, 1000]
    assert all(p == [1.0, 1.0, 1.0, 1.0] for p in c["FX-RPULSE-012"]["4"][at(7, 4):at(9, 4)])
    print("checked")


if __name__ == "__main__":
    main()
