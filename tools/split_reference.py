"""Split, worked a second way.

D-393 adds `core.split`, after CycoreFX's CC Split: the picture torn open along a line between
two points, the two sides pushed apart into an eye-shaped gap widest at the middle and closed at
both points. CycoreFX's notes say what each control does in a sentence and publish no formula;
the rule below is this program's own reading of them, and nothing is ported.

The rule. A is `point_a` and B is `point_b`, per cent of the drawing's own width and height as
Radial Blur's centre is; L = |B - A|, u the unit step from A to B and n = (-u_y, u_x) across it.
A pixel's centre P lies t = (P - A) . u / L of the way along and d = (P - A) . n to the side.
The gap's half width there is g = (S / 2) sin(pi t) for t from 0 to 1, and 0 past the points,
where S is `split` in pixels, the gap's full width at the middle. Each side is squeezed into the
band from g to D = g + L / 2 out from the line, and past D nothing moves: a pixel with
g <= |d| < D reads the drawing at

    A + t L u + sign(d) (|d| - g) D / (D - g) n,

and one inside the gap reads the line itself, at A + t L u. The sample is document 21's bilinear
sample, transparent outside the drawing, times the share of the pixel outside the gap, taken
across it: 1 - max(0, min(|d| + 1/2, g) - max(|d| - 1/2, -g)), so the gap's edges are smooth to a
pixel and its middle is transparent. Split 0 or A on B leaves the picture as it was. The layer
does not grow. In a draft the split, in pixels, is scaled with the picture.

`point_a` -1000 to 1000 per cent each way, 25, 50 when added; `point_b` the same, 75, 50;
`split` 0 to 1000 pixels, 50. The numbers are keyable. The values when added are chosen here.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/split/media`, the projects into
`Fixtures/split`, and the expected frames into `Fixtures/split/expected_split.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/split_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "split"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"point_a": (-1000, 1000), "point_b": (-1000, 1000), "split": (0, 1000)}
NAMES = ("point_a", "point_b", "split")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def source(s_, px, py):
    """(where the pixel centre (px, py) reads the drawing, the share of it left showing)."""
    ax, ay = s_["point_a"][0] / 100 * W, s_["point_a"][1] / 100 * H
    bx, by = s_["point_b"][0] / 100 * W, s_["point_b"][1] / 100 * H
    length = math.hypot(bx - ax, by - ay)
    if length == 0 or s_["split"] <= 0:
        return (px, py), 1.0
    ux, uy = (bx - ax) / length, (by - ay) / length
    nx, ny = -uy, ux
    along = (px - ax) * ux + (py - ay) * uy
    d = (px - ax) * nx + (py - ay) * ny
    t = along / length
    g = s_["split"] / 2 * math.sin(math.pi * t) if 0 <= t <= 1 else 0.0
    a = abs(d)
    cover = 1 - max(0.0, min(a + 0.5, g) - max(a - 0.5, -g))
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

def case(a=(25, 50), b=(75, 50), split=50, shift=0):
    return {"drawing": "stripes", "point_a": a, "point_b": b, "split": split, "shift": shift}


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
    layer = B.drawn_layer(c["drawing"])
    return [list(layer["px"][y * W + x - c["shift"]]) if 0 <= x - c["shift"] < W else EMPTY
            for y in range(H) for x in range(W)]


CASES = {
    "FX-SPLIT-001": ("The settings as they start: from 25, 50 to 75, 50, split 50, wider than "
                     "the drawing: the middle torn wide open, transparent from top to bottom "
                     "between the points, the ends kept.", case(), [0]),
    "FX-SPLIT-002": ("Split 4: a gap 4 pixels wide at the middle of the middle row, closed at "
                     "4, 5 and 12, 5, the band pushed up and down and squeezed into the 4 "
                     "pixels each side; columns left of 4 and right of 12 untouched.",
                     case(split=4), [0]),
    "FX-SPLIT-003": ("Split 0: the drawing, untouched.", case(split=0), [0]),
    "FX-SPLIT-004": ("Both points at 50, 50: the drawing, untouched.",
                     case(a=(50, 50), b=(50, 50)), [0]),
    "FX-SPLIT-005": ("From 50, 10 to 50, 90, split 4: torn down the middle column, the "
                     "stripes pushed left and right.", case(a=(50, 10), b=(50, 90), split=4),
                     [0]),
    "FX-SPLIT-006": ("From 10, 10 to 90, 90, split 3: torn along the diagonal.",
                     case(a=(10, 10), b=(90, 90), split=3), [0]),
    "FX-SPLIT-007": ("Split keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 the "
                     "drawing, frame 2 FX-SPLIT-002.", case(split=keyed((0, 0), (4, 8))),
                     [0, 2, 4]),
    "FX-SPLIT-008": ("Point B keyed from 25, 50 at frame 0 to 75, 50 at frame 4, split 4: "
                     "frame 0 nothing torn, frame 4 FX-SPLIT-002.",
                     case(b=keyed((0, (25, 50)), (4, (75, 50))), split=4), [0, 2, 4]),
    "FX-SPLIT-009": ("FX-SPLIT-002 moved three pixels right: the same, moved; nothing grows.",
                     case(split=4, shift=3), [0]),
    "FX-SPLIT-010": ("Split eased from 0 at frame 0 to 1000 at frame 4 on a curve that "
                     "overshoots: at frame 2 it would pass 1000 and is held there.",
                     case(split=keyed((0, 0, OVERSHOOT), (4, 1000))), [0, 2, 4]),
    "FX-SPLIT-011": ("FX-SPLIT-002 with the points swapped: the same picture.",
                     case(a=(75, 50), b=(25, 50), split=4), [0]),
    "FX-SPLIT-012": ("From -50, 50 to 150, 50, split 6: the points off the drawing, the tear "
                     "running right across it, 6 pixels wide at the middle.",
                     case(a=(-50, 50), b=(150, 50), split=6), [0]),
}

INVALID = {
    "FX-SPLIT-013": ("Split 1001, above 1000.", case(split=1001)),
    "FX-SPLIT-014": ("Split -1, below 0.", case(split=-1)),
    "FX-SPLIT-015": ("Point A at 1001, 50, past ten widths.", case(a=(1001, 50))),
    "FX-SPLIT-016": ("Point B at 50, -1001, past ten heights.", case(b=(50, -1001))),
    "FX-SPLIT-017": ("Split keyed to 1500 at frame 4.", case(split=keyed((0, 0), (4, 1500)))),
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
        "instance_id": "fx-0-0", "type_id": "core.split", "enabled": True,
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

    (OUT / "expected_split.json").write_text(json.dumps(expected, indent=1) + "\n",
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

    # The squeeze meets the gap's edge at the line and the untouched picture at D.
    s_ = {"point_a": (25, 50), "point_b": (75, 50), "split": 4}
    assert near([source(s_, 8.0, 5.0 + 2 + 1e-12)[0]], [(8.0, 5.0)])
    assert near([source(s_, 8.0, 5.0 + 6)[0]], [(8.0, 11.0)])
    assert source(s_, 8.0, 5.0)[1] == 0 and source(s_, 8.0, 5.0 + 3)[1] == 1
    one = c["FX-SPLIT-001"]["0"]
    assert one[at(8, 0)] == EMPTY and one[at(8, 9)] == EMPTY
    two = c["FX-SPLIT-002"]["0"]
    assert two[at(8, 4)] == EMPTY == two[at(8, 5)]
    assert all(two[at(x, y)] == drawn[at(x, y)] for x in (0, 1, 2, 3, 13, 14, 15)
               for y in range(H))
    for fx in ("FX-SPLIT-003", "FX-SPLIT-004"):
        assert c[fx]["0"] == drawn, fx
    five = c["FX-SPLIT-005"]["0"]
    assert five[at(7, 5)] == EMPTY == five[at(8, 5)] and five[at(0, 5)] == drawn[at(0, 5)]
    assert not near(c["FX-SPLIT-006"]["0"], drawn)
    seven = c["FX-SPLIT-007"]
    assert seven["0"] == drawn and seven["2"] == two
    eight = c["FX-SPLIT-008"]
    assert eight["0"] == drawn and eight["4"] == two and eight["2"] not in (drawn, two)
    nine = c["FX-SPLIT-009"]["0"]
    assert all(nine[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(nine[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    ten = c["FX-SPLIT-010"]
    assert ten["0"] == drawn and ten["2"] == ten["4"] == render(case(split=1000), 0)
    assert near(c["FX-SPLIT-011"]["0"], two)
    twelve = c["FX-SPLIT-012"]["0"]
    assert twelve[at(0, 5)] != drawn[at(0, 5)] and twelve[at(8, 5)] == EMPTY
    print("checked")


if __name__ == "__main__":
    main()
