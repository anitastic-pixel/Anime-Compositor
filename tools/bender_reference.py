"""Bender, worked a second way.

D-378 adds `core.bender`, after CycoreFX's CC Bender: the picture pushed sideways between two
points, a Base that stays put and a Top, so a bottle or a building seems to sway or bend. Four
styles shape the push. CycoreFX's manual says what each style does in a sentence and publishes
no formula; the rule below is this program's own reading of it, and nothing is ported.

The rule. Bs and T are `base` and `top`, per cent of the drawing's own width and height as Radial
Blur's centre is; L = |T - Bs|, t = (T - Bs) / L the axis, n = t turned 90 degrees clockwise on
the screen (y down), so with the axis standing up, the Base below, n points right. A pixel's
centre P is s = (P - Bs).t / L along the axis, 0 at the Base and 1 at the Top. It takes document
21's bilinear sample of the drawing at P - d(s) n, transparent outside it, where A is `amount`
pixels, or with `adjust_to_distance` on, `amount` per cent of L, and d by `style`:

- `bend`: 0 before the Base, A s^2 from the Base to the Top, and on past the Top along the
  curve's tangent, A (2 s - 1): a smooth bend from the Base, the part past the Top carried over
  straight, and steeper as the points come together.
- `marilyn`: A sin^2(pi s) between the points, 0 outside: a smooth swell, narrower as the points
  come together.
- `sharp`: A (1 - |2 s - 1|) between the points, 0 outside: a triangle, its corner half way.
- `boxer`: 0 before the Base, A (3 s^2 - 2 s^3) between the points, A past the Top: everything
  past the Top shifted whole, as a boxer ducks a punch.

Every d meets 0 or its straight part without a step, so nothing is cut. Amount 0 is the drawing,
and so is Top = Base, which is no axis. The push is only across the axis, so it is undone
exactly. The layer does not grow: what is pushed past its edges is cut, as After Effects cuts it.
For a draft the amount is a distance unless Adjust To Distance is on.

`amount` -1000 to 1000, 20 when added; `style` `bend`, `marilyn`, `sharp` or `boxer`, `bend`;
`adjust_to_distance` `off` or `on`, `off`; `top` and `base` -1000 to 1000 per cent each way, 50, 0
and 50, 100 when added: the axis standing up, the drawing's full height. The numbers are
keyable. The values when added are chosen here.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/bender/media`, the projects into
`Fixtures/bender`, and the expected frames into `Fixtures/bender/expected_bender.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/bender_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "bender"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"amount": (-1000, 1000), "top": (-1000, 1000), "base": (-1000, 1000)}
NAMES = ("amount", "style", "adjust_to_distance", "top", "base")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def point(p):
    return p[0] / 100 * W, p[1] / 100 * H


def push(style, a, s):
    if style == "bend":
        return 0.0 if s < 0 else a * s * s if s <= 1 else a * (2 * s - 1)
    if style == "boxer":
        return 0.0 if s < 0 else a * (3 * s * s - 2 * s ** 3) if s <= 1 else a
    if not 0 <= s <= 1:
        return 0.0
    if style == "marilyn":
        return a * math.sin(math.pi * s) ** 2
    return a * (1 - abs(2 * s - 1))  # sharp


def source(s_, px, py):
    """Where the output point (px, py) reads the drawing."""
    bx, by = point(s_["base"])
    tx, ty = point(s_["top"])
    length = math.hypot(tx - bx, ty - by)
    if length == 0 or s_["amount"] == 0:
        return px, py
    ux, uy = (tx - bx) / length, (ty - by) / length
    nx, ny = -uy, ux
    a = s_["amount"] / 100 * length if s_["adjust_to_distance"] == "on" else s_["amount"]
    d = push(s_["style"], a, ((px - bx) * ux + (py - by) * uy) / length)
    return px - d * nx, py - d * ny


def bent(layer, s_, x, y):
    if not (0 <= x < layer["w"] and 0 <= y < layer["h"]):
        return EMPTY
    return bilinear(layer, *source(s_, x + 0.5, y + 0.5))


# --- the cases ------------------------------------------------------------------------------

def case(amount=20, style="bend", adjust="off", top=(50, 0), base=(50, 100), shift=0):
    return {"drawing": "stripes", "amount": amount, "style": style, "adjust_to_distance": adjust,
            "top": top, "base": base, "shift": shift}


def settings(c, frame_no):
    held = {}
    for k in NAMES:
        v = value_at(c[k], frame_no)
        if k in RANGES:
            lo, hi = RANGES[k]
            v = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
                 else min(hi, max(lo, v)))
        held[k] = v
    return held


def render(c, frame_no):
    layer = B.drawn_layer(c["drawing"])
    s_ = settings(c, frame_no)
    return [bent(layer, s_, x - c["shift"], y) for y in range(H) for x in range(W)]


def plain(c):
    layer = B.drawn_layer(c["drawing"])
    return [list(layer["px"][y * W + x - c["shift"]]) if 0 <= x - c["shift"] < W else EMPTY
            for y in range(H) for x in range(W)]


SHORT = {"top": (50, 30), "base": (50, 70)}  # an axis from row 7 up to row 3

CASES = {
    "FX-BENDER-001": ("The settings as they start: Amount 20, Bend, the axis from the middle of "
                      "the bottom edge to the middle of the top. Each row is pushed right by 20 "
                      "times the square of its height up the drawing, so the bottom rows barely "
                      "move and the top ones are pushed clean off the drawing: a hard bend.",
                      case(), [0]),
    "FX-BENDER-002": ("Amount 0: the drawing, untouched.", case(amount=0), [0]),
    "FX-BENDER-003": ("Bend, Amount 4: row 1, the stripes' top row, pushed 2.89 pixels right, "
                      "the band's rows 1.21 and 0.81, and row 8 0.09: the stripes curve over to "
                      "the right.",
                      case(amount=4), [0]),
    "FX-BENDER-004": ("Bend, Amount -4: the same push to the left.", case(amount=-4), [0]),
    "FX-BENDER-005": ("Marilyn, Amount 4: the middle rows swell 4 pixels right and the top and "
                      "bottom rows hardly move, a smooth bulge.", case(amount=4, style="marilyn"),
                      [0]),
    "FX-BENDER-006": ("Sharp, Amount 4: the same swell as a triangle, its point in the middle.",
                      case(amount=4, style="sharp"), [0]),
    "FX-BENDER-007": ("Boxer, Amount 4: a smooth S from the bottom row to the top, the top "
                      "pushed most nearly 4 pixels.", case(amount=4, style="boxer"), [0]),
    "FX-BENDER-008": ("Adjust To Distance on, Amount 40: 40 per cent of the 10-pixel axis, 4 "
                      "pixels: FX-BENDER-003 exactly.", case(amount=40, adjust="on"), [0]),
    "FX-BENDER-009": ("Bend, Amount 2, the axis from row 7 up to row 3: rows 7 to 9 below the "
                      "Base stay put, the rows to the Top curve, and the rows past it, 1 and 2, "
                      "carry on straight, slanting further, 2 (2 s - 1) pixels.",
                      case(amount=2, **SHORT), [0]),
    "FX-BENDER-010": ("Marilyn, Amount 2, the same short axis: only rows 3 to 6 swell; the rest "
                      "is untouched.", case(amount=2, style="marilyn", **SHORT), [0]),
    "FX-BENDER-011": ("Boxer, Amount 3, the axis lying across from the middle of the left edge to "
                      "the middle of the right: n points down, so the columns are pushed down, "
                      "the right ones 3 pixels.",
                      case(amount=3, style="boxer", top=(100, 50), base=(0, 50)), [0]),
    "FX-BENDER-012": ("Top and Base the same point: no axis, the drawing untouched.",
                      case(amount=4, top=(50, 50), base=(50, 50)), [0]),
    "FX-BENDER-013": ("Amount keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 the "
                      "drawing, frame 2 FX-BENDER-003.", case(amount=keyed((0, 0), (4, 8))),
                      [0, 2, 4]),
    "FX-BENDER-014": ("Bend, Amount 3, the Top keyed from 50, 0 at frame 0 to 50, 50 at frame 4: "
                      "the axis shortens and the same push bends harder.",
                      case(amount=3, top=keyed((0, (50, 0)), (4, (50, 50)))), [0, 2, 4]),
    "FX-BENDER-015": ("FX-BENDER-005 moved three pixels right: the same, moved; nothing grows, "
                      "and the three columns left of the drawing stay empty.",
                      case(amount=4, style="marilyn", shift=3), [0]),
    "FX-BENDER-016": ("Sharp, Amount 0.5: half a pixel at most, a blend of neighbouring "
                      "stripes rather than a move.", case(amount=0.5, style="sharp"), [0]),
    "FX-BENDER-017": ("Amount eased from 0 at frame 0 to 1000 at frame 4 on a curve that "
                      "overshoots: at frame 2 it would pass 1000 and is held there.",
                      case(amount=keyed((0, 0, OVERSHOOT), (4, 1000)), style="sharp"),
                      [0, 2, 4]),
}

INVALID = {
    "FX-BENDER-018": ("Amount 1001, above 1000.", case(amount=1001)),
    "FX-BENDER-019": ("Amount -1001, below -1000.", case(amount=-1001)),
    "FX-BENDER-020": ("A style written \"Bend\", with a capital.", case(style="Bend")),
    "FX-BENDER-021": ("Adjust To Distance written \"yes\".", case(adjust="yes")),
    "FX-BENDER-022": ("Top 50, 1001, past ten heights.", case(top=(50, 1001))),
    "FX-BENDER-023": ("Amount keyed to 2000 at frame 4.", case(amount=keyed((0, 0), (4, 2000)))),
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
        "instance_id": "fx-0-0", "type_id": "core.bender", "enabled": True,
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

    (OUT / "expected_bender.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")
    check(expected)


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

    # The styles meet their straight parts without a step.
    for style in ("bend", "marilyn", "sharp", "boxer"):
        for s in (0.0, 1.0):
            assert abs(push(style, 3, s - 1e-12) - push(style, 3, s + 1e-12)) < 1e-9, style
    assert push("bend", 4, 0.95) == 4 * 0.95 ** 2 and push("bend", 4, 1.5) == 8
    assert push("boxer", 4, 2) == 4 and push("marilyn", 4, 2) == 0 == push("sharp", 4, -1)

    assert c["FX-BENDER-002"]["0"] == drawn and c["FX-BENDER-012"]["0"] == drawn
    three = c["FX-BENDER-003"]["0"]
    assert c["FX-BENDER-008"]["0"] == three
    assert three != c["FX-BENDER-004"]["0"]
    for fx in ("FX-BENDER-001", "FX-BENDER-005", "FX-BENDER-006", "FX-BENDER-007",
               "FX-BENDER-009", "FX-BENDER-010", "FX-BENDER-011", "FX-BENDER-016"):
        assert c[fx]["0"] != drawn, fx
    nine, ten = c["FX-BENDER-009"]["0"], c["FX-BENDER-010"]["0"]
    assert all(nine[at(x, y)] == drawn[at(x, y)] for x in range(W) for y in (7, 8, 9))
    assert all(ten[at(x, y)] == drawn[at(x, y)] for x in range(W) for y in (0, 1, 2, 7, 8, 9))
    assert any(nine[at(x, 1)] != drawn[at(x, 1)] for x in range(W))
    thirteen = c["FX-BENDER-013"]
    assert thirteen["0"] == drawn and thirteen["2"] == three != thirteen["4"]
    fourteen = c["FX-BENDER-014"]
    assert fourteen["0"] == render(case(amount=3), 0) != fourteen["4"]
    fifteen, five = c["FX-BENDER-015"]["0"], c["FX-BENDER-005"]["0"]
    assert all(fifteen[at(x, y)] == five[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(fifteen[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    seventeen = c["FX-BENDER-017"]
    assert seventeen["2"] == seventeen["4"] == render(case(amount=1000, style="sharp"), 0)
    print("checked")


if __name__ == "__main__":
    main()
