"""Fisheye, worked a second way.

D-387 adds `core.fisheye`, after CycoreFX's CC Lens: the picture seen through a round lens, swollen
out from the middle like a ball, or drawn in, with nothing outside the lens. CycoreFX's manual
says what each control does in a sentence and publishes no formula; the rule below is this
program's own reading of it, and nothing is ported.

The rule. C is `center`, per cent of the drawing's own width and height as Radial Blur's centre
is; the lens's radius R is `size` per cent of half the drawing's diagonal, so at 100 the lens
reaches the drawing's corners. A pixel's centre P lies r = |P - C| from the centre, rho =
min(r / R, 1) of the way out. With c = `convergence` / 100, it reads the drawing at
C + (P - C) f / rho (C itself when r is 0), where

- c >= 0: f = rho + c (2 asin(rho) / pi - rho), at c = 1 the picture laid round a ball and seen
  from the front: the middle grown by pi / 2 and the rim squeezed, a fish-eye;
- c < 0: f = rho - c (sin(pi rho / 2) - rho), the inverse curve, the middle drawn in and the rim
  stretched, a lens the other way.

The sample is document 21's bilinear sample, transparent outside the drawing, times the lens's
cover min(1, max(0, R - r + 0.5)), so the rim is smooth to a pixel and everything past it is
transparent. Size 0 draws nothing. Convergence 0 is the drawing cut to the circle. The layer
does not grow. The radius is a share of the diagonal, so a draft needs no change.

`center` -1000 to 1000 per cent each way, 50, 50 when added; `size` 0 to 1000, 50; `convergence`
-100 to 100, 50. The numbers are keyable. The values when added are chosen here.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/fisheye/media`, the projects into
`Fixtures/fisheye`, and the expected frames into `Fixtures/fisheye/expected_fisheye.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/fisheye_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "fisheye"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"center": (-1000, 1000), "size": (0, 1000), "convergence": (-100, 100)}
NAMES = ("center", "size", "convergence")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def curve(c, rho):
    if c >= 0:
        return rho + c * (2 * math.asin(rho) / math.pi - rho)
    return rho - c * (math.sin(math.pi * rho / 2) - rho)


def seen(layer, s_, x, y):
    if not (0 <= x < layer["w"] and 0 <= y < layer["h"]):
        return EMPTY
    radius = s_["size"] / 100 * math.hypot(W, H) / 2
    if radius <= 0:
        return EMPTY
    cx, cy = s_["center"][0] / 100 * W, s_["center"][1] / 100 * H
    dx, dy = x + 0.5 - cx, y + 0.5 - cy
    r = math.hypot(dx, dy)
    cover = min(1.0, max(0.0, radius - r + 0.5))
    if cover == 0:
        return EMPTY
    if r == 0:
        sx, sy = cx, cy
    else:
        rho = min(r / radius, 1.0)
        k = curve(s_["convergence"] / 100, rho) / rho
        sx, sy = cx + dx * k, cy + dy * k
    return [v * cover for v in bilinear(layer, sx, sy)]


# --- the cases ------------------------------------------------------------------------------

def case(center=(50, 50), size=50, convergence=50, shift=0):
    return {"drawing": "stripes", "center": center, "size": size, "convergence": convergence,
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
    layer = B.drawn_layer(c["drawing"])
    return [list(layer["px"][y * W + x - c["shift"]]) if 0 <= x - c["shift"] < W else EMPTY
            for y in range(H) for x in range(W)]


CASES = {
    "FX-FISHEYE-001": ("The settings as they start: the lens in the middle, its radius a "
                       "quarter of the diagonal, 4.7 pixels here, convergence 50: the stripes "
                       "swell out from the middle inside a circle, and the rest is transparent.",
                       case(), [0]),
    "FX-FISHEYE-002": ("Size 100, convergence 0: the lens reaches the corners, every pixel "
                       "covered: the drawing, untouched.", case(size=100, convergence=0), [0]),
    "FX-FISHEYE-003": ("Size 50, convergence 0: the drawing cut to the circle, its rim soft to "
                       "a pixel.", case(convergence=0), [0]),
    "FX-FISHEYE-004": ("Size 100, convergence 100: the whole drawing laid round a ball, the "
                       "middle grown by half again and the rim squeezed.",
                       case(size=100, convergence=100), [0]),
    "FX-FISHEYE-005": ("Size 100, convergence -100: the other way, the middle drawn in and the "
                       "rim stretched.", case(size=100, convergence=-100), [0]),
    "FX-FISHEYE-006": ("Size 100, convergence 50: half way to the ball.",
                       case(size=100, convergence=50), [0]),
    "FX-FISHEYE-007": ("Size 30 at the left quarter: a small lens over the left stripes, the "
                       "rest transparent.", case(center=(25, 50), size=30), [0]),
    "FX-FISHEYE-008": ("Size 0: nothing drawn.", case(size=0), [0]),
    "FX-FISHEYE-009": ("Size keyed from 0 at frame 0 to 100 at frame 4, linear: the lens opens "
                       "from nothing; frame 2 FX-FISHEYE-001.",
                       case(size=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-FISHEYE-010": ("Convergence keyed from -100 at frame 0 to 100 at frame 4, size 100: "
                       "frame 0 FX-FISHEYE-005, frame 2 the drawing, frame 4 FX-FISHEYE-004.",
                       case(size=100, convergence=keyed((0, -100), (4, 100))), [0, 2, 4]),
    "FX-FISHEYE-011": ("Centre keyed from 25, 50 at frame 0 to 75, 50 at frame 4: the lens "
                       "slides across.", case(center=keyed((0, (25, 50)), (4, (75, 50)))),
                       [0, 2, 4]),
    "FX-FISHEYE-012": ("FX-FISHEYE-001 moved three pixels right: the same, moved; nothing "
                       "grows.", case(shift=3), [0]),
    "FX-FISHEYE-013": ("Size eased from 0 at frame 0 to 1000 at frame 4 on a curve that "
                       "overshoots: at frame 2 it would pass 1000 and is held there.",
                       case(size=keyed((0, 0, OVERSHOOT), (4, 1000))), [0, 2, 4]),
    "FX-FISHEYE-014": ("Size 1000, convergence 100: a lens ten times the drawing, so only its "
                       "middle shows here: the drawing grown by about pi / 2 about the centre.",
                       case(size=1000, convergence=100), [0]),
}

INVALID = {
    "FX-FISHEYE-015": ("Size 1001, above 1000.", case(size=1001)),
    "FX-FISHEYE-016": ("Size -1, below 0.", case(size=-1)),
    "FX-FISHEYE-017": ("Convergence 101, above 100.", case(convergence=101)),
    "FX-FISHEYE-018": ("Convergence -101, below -100.", case(convergence=-101)),
    "FX-FISHEYE-019": ("Centre at 50, 1001, past ten heights.", case(center=(50, 1001))),
    "FX-FISHEYE-020": ("Convergence keyed to 200 at frame 4.",
                       case(convergence=keyed((0, 0), (4, 200)))),
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
        "instance_id": "fx-0-0", "type_id": "core.fisheye", "enabled": True,
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

    (OUT / "expected_fisheye.json").write_text(json.dumps(expected, indent=1) + "\n",
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

    # The curves meet the straight line at the middle and the rim, and undo each other.
    for c_ in (-1, -0.5, 0, 0.5, 1):
        assert curve(c_, 0) == 0 and abs(curve(c_, 1) - 1) < 1e-12
    assert abs(curve(1, curve(-1, 0.3)) - 0.3) < 1e-12
    assert curve(1, 0.5) < 0.5 < curve(-1, 0.5)

    assert near(c["FX-FISHEYE-002"]["0"], drawn)
    assert all(p == EMPTY for p in c["FX-FISHEYE-008"]["0"])
    one, three = c["FX-FISHEYE-001"]["0"], c["FX-FISHEYE-003"]["0"]
    assert one[at(0, 0)] == EMPTY == three[at(0, 0)] and one != three
    assert near([three[at(8, 5)]], [drawn[at(8, 5)]])
    four, five = c["FX-FISHEYE-004"]["0"], c["FX-FISHEYE-005"]["0"]
    assert four != five and not near(four, drawn) and not near(five, drawn)
    assert c["FX-FISHEYE-006"]["0"] != four
    assert c["FX-FISHEYE-007"]["0"][at(13, 5)] == EMPTY != c["FX-FISHEYE-007"]["0"][at(4, 5)]
    nine = c["FX-FISHEYE-009"]
    assert all(p == EMPTY for p in nine["0"]) and nine["2"] == one
    ten = c["FX-FISHEYE-010"]
    assert ten["0"] == five and ten["4"] == four and near(ten["2"], drawn)
    eleven = c["FX-FISHEYE-011"]
    assert eleven["0"] != eleven["2"] != eleven["4"]
    twelve = c["FX-FISHEYE-012"]["0"]
    assert all(twelve[at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(twelve[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    thirteen = c["FX-FISHEYE-013"]
    assert thirteen["2"] == thirteen["4"] == render(case(size=1000), 0)
    assert c["FX-FISHEYE-014"]["0"] != thirteen["4"]
    print("checked")


if __name__ == "__main__":
    main()
