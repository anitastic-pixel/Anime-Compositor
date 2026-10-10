"""Split 2, worked a second way.

D-394 adds `core.split_2`, after CycoreFX's CC Split 2: Split (D-393, `tools/split_reference.py`)
with each side of the tear opened by its own amount. The owner chose on 2026-10-09 "Second name,
one engine (Recommended)": the same rule as Split under a second name, as Levels (Individual
Controls) is over Levels (D-383). CycoreFX publishes no formula; the rule is this program's own.

The rule. A, B, L, u, n, t and d are Split's: A is `point_a` and B is `point_b`, per cent of the
drawing's own width and height; L = |B - A|, u the unit step from A to B, n = (-u_y, u_x) across
it; a pixel's centre P lies t = (P - A) . u / L of the way along and d = (P - A) . n to the side.
`split_1` opens the side with d < 0, the one on your left as you walk from A to B on the screen
(above the line for the points as added); `split_2` the side with d >= 0, on your right (below).
That side's half gap is g = (S / 2) sin(pi t) for t from 0 to 1, and 0 past the points, S the
side's own amount in pixels, so the gap runs from -g1 to g2 across the line and is (S1 + S2) / 2
wide at the middle. On each side, with that side's g and D = g + L / 2, a pixel with
g <= |d| < D reads the drawing at

    A + t L u + sign(d) (|d| - g) D / (D - g) n,

one inside the gap reads the line itself, at A + t L u, and one past D is not moved. The sample is
document 21's bilinear sample, transparent outside the drawing, times the share of the pixel
outside the gap, taken across it: 1 - max(0, min(d + 1/2, g2) - max(d - 1/2, -g1)). Both amounts
0, or A on B, leaves the picture as it was. With the two amounts equal to S, the picture is
Split's with split S. The layer does not grow. In a draft both amounts, in pixels, are scaled
with the picture.

`point_a` and `point_b` as Split's (25, 50 and 75, 50 when added); `split_1` and `split_2` 0 to
1000 pixels, 50 each when added, so a new Split 2 draws as a new Split. The numbers are keyable.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/split_2/media`, the projects into
`Fixtures/split_2`, and the expected frames into `Fixtures/split_2/expected_split_2.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/split2_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import bulge_reference as B  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
import split_reference as SP  # noqa: E402

W, H = SP.W, SP.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "split_2"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"point_a": (-1000, 1000), "point_b": (-1000, 1000), "split_1": (0, 1000),
          "split_2": (0, 1000)}
NAMES = ("point_a", "point_b", "split_1", "split_2")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def source(s_, px, py):
    """(where the pixel centre (px, py) reads the drawing, the share of it left showing)."""
    ax, ay = s_["point_a"][0] / 100 * W, s_["point_a"][1] / 100 * H
    bx, by = s_["point_b"][0] / 100 * W, s_["point_b"][1] / 100 * H
    length = math.hypot(bx - ax, by - ay)
    if length == 0 or (s_["split_1"] <= 0 and s_["split_2"] <= 0):
        return (px, py), 1.0
    ux, uy = (bx - ax) / length, (by - ay) / length
    nx, ny = -uy, ux
    along = (px - ax) * ux + (py - ay) * uy
    d = (px - ax) * nx + (py - ay) * ny
    t = along / length
    wave = math.sin(math.pi * t) if 0 <= t <= 1 else 0.0
    g1, g2 = s_["split_1"] / 2 * wave, s_["split_2"] / 2 * wave
    cover = 1 - max(0.0, min(d + 0.5, g2) - max(d - 0.5, -g1))
    g = g2 if d >= 0 else g1
    a = abs(d)
    far = g + length / 2
    if a >= far:
        return (px, py), cover
    side = 1 if d >= 0 else -1
    k = side * (a - g) * far / (far - g) if a >= g else 0.0
    return (ax + along * ux + k * nx, ay + along * uy + k * ny), cover


def seen(layer, s_, x, y):
    if not (0 <= x < layer["w"] and 0 <= y < layer["h"]):
        return EMPTY
    (sx, sy), cover = source(s_, x + 0.5, y + 0.5)
    return [v * cover for v in bilinear(layer, sx, sy)]


# --- the cases ------------------------------------------------------------------------------

def case(a=(25, 50), b=(75, 50), s1=50, s2=50, shift=0):
    return {"drawing": "stripes", "point_a": a, "point_b": b, "split_1": s1, "split_2": s2,
            "shift": shift}


def settings(c, frame_no):
    held = {}
    for k in NAMES:
        v = value_at(c[k], frame_no)
        lo, hi = RANGES[k]
        v = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
             else min(hi, max(lo, v)))
        held[k] = v
    return held


def render(c, frame_no):
    layer = B.drawn_layer(c["drawing"])
    s_ = settings(c, frame_no)
    return [seen(layer, s_, x - c["shift"], y) for y in range(H) for x in range(W)]


def plain(c):
    return SP.plain(c)


CASES = {
    "FX-SPLIT2-001": ("The settings as they start: from 25, 50 to 75, 50, both sides 50: "
                      "Split's picture as it starts (FX-SPLIT-001).", case(), [0]),
    "FX-SPLIT2-002": ("Both sides 4: Split's picture with split 4 (FX-SPLIT-002).",
                      case(s1=4, s2=4), [0]),
    "FX-SPLIT2-003": ("Split 1 4, Split 2 0: only the upper side opens, the rows above the "
                      "line pushed up and squeezed; every row below the line untouched.",
                      case(s1=4, s2=0), [0]),
    "FX-SPLIT2-004": ("Split 1 0, Split 2 4: only the lower side opens; every row above the "
                      "line untouched.", case(s1=0, s2=4), [0]),
    "FX-SPLIT2-005": ("Both sides 0: the drawing, untouched.", case(s1=0, s2=0), [0]),
    "FX-SPLIT2-006": ("Both points at 50, 50: the drawing, untouched.",
                      case(a=(50, 50), b=(50, 50)), [0]),
    "FX-SPLIT2-007": ("From 50, 10 to 50, 90, Split 1 4, Split 2 0: walking down the screen "
                      "your left is the right of the picture, so only the right half opens.",
                      case(a=(50, 10), b=(50, 90), s1=4, s2=0), [0]),
    "FX-SPLIT2-008": ("From 10, 10 to 90, 90, Split 1 3, Split 2 6: torn along the diagonal, "
                      "the lower left side opened twice as far.",
                      case(a=(10, 10), b=(90, 90), s1=3, s2=6), [0]),
    "FX-SPLIT2-009": ("Split 1 keyed from 0 at frame 0 to 8 at frame 4, linear, Split 2 4: "
                      "frame 0 FX-SPLIT2-004, frame 2 FX-SPLIT2-002.",
                      case(s1=keyed((0, 0), (4, 8)), s2=4), [0, 2, 4]),
    "FX-SPLIT2-010": ("Split 2 keyed from 0 at frame 0 to 8 at frame 4, Split 1 0: frame 0 the "
                      "drawing, frame 2 FX-SPLIT2-004.", case(s1=0, s2=keyed((0, 0), (4, 8))),
                      [0, 2, 4]),
    "FX-SPLIT2-011": ("FX-SPLIT2-003 moved three pixels right: the same, moved; nothing grows.",
                      case(s1=4, s2=0, shift=3), [0]),
    "FX-SPLIT2-012": ("Split 2 eased from 0 at frame 0 to 1000 at frame 4 on a curve that "
                      "overshoots, Split 1 0: at frame 2 it would pass 1000 and is held there.",
                      case(s1=0, s2=keyed((0, 0, OVERSHOOT), (4, 1000))), [0, 2, 4]),
    "FX-SPLIT2-013": ("FX-SPLIT2-003 with the points swapped: the sides swap, so the lower "
                      "side opens, FX-SPLIT2-004's picture.",
                      case(a=(75, 50), b=(25, 50), s1=4, s2=0), [0]),
    "FX-SPLIT2-014": ("From -50, 50 to 150, 50, Split 1 6, Split 2 2: the points off the "
                      "drawing, the tear right across it, opened 3 up and 1 down at the middle.",
                      case(a=(-50, 50), b=(150, 50), s1=6, s2=2), [0]),
}

INVALID = {
    "FX-SPLIT2-015": ("Split 1 1001, above 1000.", case(s1=1001)),
    "FX-SPLIT2-016": ("Split 2 -1, below 0.", case(s2=-1)),
    "FX-SPLIT2-017": ("Point A at 1001, 50, past ten widths.", case(a=(1001, 50))),
    "FX-SPLIT2-018": ("Point B at 50, -1001, past ten heights.", case(b=(50, -1001))),
    "FX-SPLIT2-019": ("Split 2 keyed to 1500 at frame 4.", case(s2=keyed((0, 0), (4, 1500)))),
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
        "instance_id": "fx-0-0", "type_id": "core.split_2", "enabled": True,
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

    (OUT / "expected_split_2.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def near(a, b):
    return all(abs(u - v) < 1e-9 for p, q in zip(a, b) for u, v in zip(p, q))


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    # Equal amounts are Split, worked by Split's own reference.
    assert c["FX-SPLIT2-001"]["0"] == SP.render(SP.case(), 0)
    two = c["FX-SPLIT2-002"]["0"]
    assert two == SP.render(SP.case(split=4), 0)
    three, four = c["FX-SPLIT2-003"]["0"], c["FX-SPLIT2-004"]["0"]
    assert all(three[at(x, y)] == drawn[at(x, y)] for x in range(W) for y in range(5, H))
    assert any(three[at(x, y)] != drawn[at(x, y)] for x in range(W) for y in range(5))
    assert all(four[at(x, y)] == drawn[at(x, y)] for x in range(W) for y in range(5))
    assert any(four[at(x, y)] != drawn[at(x, y)] for x in range(W) for y in range(5, H))
    assert three[at(8, 4)] == EMPTY and four[at(8, 5)] == EMPTY
    for fx in ("FX-SPLIT2-005", "FX-SPLIT2-006"):
        assert c[fx]["0"] == drawn, fx
    seven = c["FX-SPLIT2-007"]["0"]
    assert seven[at(8, 5)] == EMPTY and all(seven[at(x, y)] == drawn[at(x, y)]
                                            for x in range(8) for y in range(H))
    assert not near(c["FX-SPLIT2-008"]["0"], drawn)
    nine = c["FX-SPLIT2-009"]
    assert nine["0"] == four and nine["2"] == two
    ten = c["FX-SPLIT2-010"]
    assert ten["0"] == drawn and ten["2"] == four
    eleven = c["FX-SPLIT2-011"]["0"]
    assert all(eleven[at(x, y)] == three[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(eleven[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    twelve = c["FX-SPLIT2-012"]
    assert twelve["0"] == drawn and twelve["2"] == twelve["4"] == render(case(s1=0, s2=1000), 0)
    assert near(c["FX-SPLIT2-013"]["0"], four)
    fourteen = c["FX-SPLIT2-014"]["0"]
    assert fourteen[at(8, 4)] == EMPTY and fourteen[at(0, 5)] != drawn[at(0, 5)]
    print("checked")


if __name__ == "__main__":
    main()
