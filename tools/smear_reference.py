"""Smear, worked a second way.

D-392 adds `core.smear`, after CycoreFX's CC Smear: the picture at one point dragged along a
line to another, like a thumb through wet paint, the drag fading out sideways over a radius.
CycoreFX's notes say what each control does in a sentence and publish no formula; the rule
below is this program's own reading of them, and nothing is ported.

The rule. F is `from` and T is `to`, per cent of the drawing's own width and height as Radial
Blur's centre is. The drag is V = (T - F) `reach` / 100, so at reach 200 it goes twice as far
as T and at a negative reach the other way. R is `radius` in pixels. A pixel's centre P lies
s = clamp((P - F) . V / |V|^2, 0, 1) of the way along the drag, nearest the point
Q = F + s V, rho = |P - Q| from it; its weight is w = (1 - (rho / R)^2)^2 inside the radius and 0
past it, so the drag fades smoothly to nothing sideways and round both ends. It reads the
drawing at

    P - w s V,

document 21's bilinear sample, transparent outside the drawing: on the line itself every point
reads F, so the picture at F is drawn out along the whole drag, and its far end lands on
F + V. Radius 0, reach 0 or F on T leaves the picture as it was. The layer does not grow. In a
draft the radius, in pixels, is scaled with the picture.

`from` -1000 to 1000 per cent each way, 40, 50 when added; `to` the same, 60, 50; `reach` -1000
to 1000 per cent, 100; `radius` 0 to 1000 pixels, 70. The numbers are keyable. The values when
added are chosen here.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/smear/media`, the projects into
`Fixtures/smear`, and the expected frames into `Fixtures/smear/expected_smear.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/smear_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "smear"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"from": (-1000, 1000), "to": (-1000, 1000), "reach": (-1000, 1000),
          "radius": (0, 1000)}
NAMES = ("from", "to", "reach", "radius")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def source(s_, px, py):
    """Where the pixel centre (px, py) reads the drawing."""
    fx, fy = s_["from"][0] / 100 * W, s_["from"][1] / 100 * H
    tx, ty = s_["to"][0] / 100 * W, s_["to"][1] / 100 * H
    vx, vy = (tx - fx) * s_["reach"] / 100, (ty - fy) * s_["reach"] / 100
    v2 = vx * vx + vy * vy
    radius = s_["radius"]
    if radius <= 0 or v2 == 0:
        return px, py
    s = min(1.0, max(0.0, ((px - fx) * vx + (py - fy) * vy) / v2))
    rho = math.hypot(px - fx - s * vx, py - fy - s * vy)
    if rho >= radius:
        return px, py
    w = (1 - (rho / radius) ** 2) ** 2
    return px - w * s * vx, py - w * s * vy


def seen(layer, s_, x, y):
    if not (0 <= x < layer["w"] and 0 <= y < layer["h"]):
        return EMPTY
    return bilinear(layer, *source(s_, x + 0.5, y + 0.5))


# --- the cases ------------------------------------------------------------------------------

def case(frm=(40, 50), to=(60, 50), reach=100, radius=70, shift=0):
    return {"drawing": "stripes", "from": frm, "to": to, "reach": reach, "radius": radius,
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


ACROSS = {"frm": (25, 50), "to": (75, 50), "radius": 4}

CASES = {
    "FX-SMEAR-001": ("The settings as they start: from 40, 50 to 60, 50, radius 70, wider "
                     "than the drawing: the picture at 6.4, 5 drawn out 3.2 pixels to the "
                     "right, everything right of it pushed along.", case(), [0]),
    "FX-SMEAR-002": ("From 25, 50 to 75, 50, radius 4: a band 4 pixels each side of the "
                     "middle row dragged 8 pixels right, the stripes at 4, 5 drawn out along "
                     "it, the top and bottom rows untouched.", case(**ACROSS), [0]),
    "FX-SMEAR-003": ("FX-SMEAR-002 at reach 200: dragged twice as far.",
                     case(reach=200, **ACROSS), [0]),
    "FX-SMEAR-004": ("FX-SMEAR-002 at reach -100: dragged 8 pixels left from 4, 5.",
                     case(reach=-100, **ACROSS), [0]),
    "FX-SMEAR-005": ("FX-SMEAR-002 at reach 0: the drawing, untouched.",
                     case(reach=0, **ACROSS), [0]),
    "FX-SMEAR-006": ("Radius 0: the drawing, untouched.", case(radius=0), [0]),
    "FX-SMEAR-007": ("From 50, 20 to 50, 80, radius 5: dragged down the middle column, the "
                     "blue band pulled down.", case(frm=(50, 20), to=(50, 80), radius=5), [0]),
    "FX-SMEAR-008": ("From and to both at 50, 50: no drag, the drawing untouched.",
                     case(frm=(50, 50), to=(50, 50)), [0]),
    "FX-SMEAR-009": ("Radius keyed from 0 at frame 0 to 8 at frame 4, linear, from 25, 50 to "
                     "75, 50: frame 0 the drawing, frame 2 FX-SMEAR-002.",
                     case(frm=(25, 50), to=(75, 50), radius=keyed((0, 0), (4, 8))), [0, 2, 4]),
    "FX-SMEAR-010": ("To keyed from 25, 50 at frame 0 to 75, 50 at frame 4, from 25, 50, "
                     "radius 4: frame 0 no drag, frame 4 FX-SMEAR-002: the drag grows out.",
                     case(frm=(25, 50), to=keyed((0, (25, 50)), (4, (75, 50))), radius=4),
                     [0, 2, 4]),
    "FX-SMEAR-011": ("FX-SMEAR-002 moved three pixels right: the same, moved; nothing grows.",
                     case(shift=3, **ACROSS), [0]),
    "FX-SMEAR-012": ("Radius eased from 0 at frame 0 to 1000 at frame 4 on a curve that "
                     "overshoots: at frame 2 it would pass 1000 and is held there.",
                     case(radius=keyed((0, 0, OVERSHOOT), (4, 1000))), [0, 2, 4]),
    "FX-SMEAR-013": ("From 25, 50 to 30, 50, reach 1000, radius 3: the 0.8 pixel step ten "
                     "times over, 8 pixels, the same drag as FX-SMEAR-002 but narrower.",
                     case(frm=(25, 50), to=(30, 50), reach=1000, radius=3), [0]),
}

INVALID = {
    "FX-SMEAR-014": ("Radius 1001, above 1000.", case(radius=1001)),
    "FX-SMEAR-015": ("Radius -1, below 0.", case(radius=-1)),
    "FX-SMEAR-016": ("Reach 1001, above 1000.", case(reach=1001)),
    "FX-SMEAR-017": ("Reach -1001, below -1000.", case(reach=-1001)),
    "FX-SMEAR-018": ("From at 1001, 50, past ten widths.", case(frm=(1001, 50))),
    "FX-SMEAR-019": ("To at 50, -1001, past ten heights.", case(to=(50, -1001))),
    "FX-SMEAR-020": ("Reach keyed to 2000 at frame 4.", case(reach=keyed((0, 0), (4, 2000)))),
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
        "instance_id": "fx-0-0", "type_id": "core.smear", "enabled": True,
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

    (OUT / "expected_smear.json").write_text(json.dumps(expected, indent=1) + "\n",
                                             encoding="utf-8")
    check(expected)


def near(a, b):
    return all(abs(u - v) < 1e-9 for p, q in zip(a, b) for u, v in zip(p, q))


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    layer = B.drawn_layer("stripes")
    at = lambda x, y: y * W + x  # noqa: E731

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    # The weight is 1 on the line and 0 at the radius; on the line every point reads F.
    s_ = {"from": (25, 50), "to": (75, 50), "reach": 100, "radius": 4}
    for px in (4.5, 7.5, 11.5):
        assert near([source(s_, px, 5.0)], [(4.0, 5.0)])
    assert source(s_, 8.5, 9.0) == (8.5, 9.0)
    assert not near(c["FX-SMEAR-001"]["0"], drawn)
    two = c["FX-SMEAR-002"]["0"]
    assert not near(two, drawn) and all(two[at(x, 0)] == drawn[at(x, 0)] for x in range(W))
    assert all(two[at(x, 9)] == drawn[at(x, 9)] for x in range(W))
    assert all(two[at(x, y)] == drawn[at(x, y)] for x in range(4) for y in range(H))
    assert c["FX-SMEAR-003"]["0"] != two and c["FX-SMEAR-004"]["0"] != two
    for fx in ("FX-SMEAR-005", "FX-SMEAR-006", "FX-SMEAR-008"):
        assert c[fx]["0"] == drawn, fx
    seven = c["FX-SMEAR-007"]["0"]
    assert not near(seven, drawn) and seven[at(0, 5)] == drawn[at(0, 5)]
    nine = c["FX-SMEAR-009"]
    assert nine["0"] == drawn and nine["2"] == two
    ten = c["FX-SMEAR-010"]
    assert ten["0"] == drawn and ten["4"] == two and ten["2"] not in (drawn, two)
    eleven = c["FX-SMEAR-011"]["0"]
    assert all(eleven[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(eleven[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    twelve = c["FX-SMEAR-012"]
    assert twelve["0"] == drawn and twelve["2"] == twelve["4"] == render(case(radius=1000), 0)
    thirteen = c["FX-SMEAR-013"]["0"]
    assert thirteen != two and near([thirteen[at(8, 5)]], [two[at(8, 5)]])
    assert near([thirteen[at(8, 5)]], [bilinear(layer, 4.0, 5.0)])
    print("checked")


if __name__ == "__main__":
    main()
