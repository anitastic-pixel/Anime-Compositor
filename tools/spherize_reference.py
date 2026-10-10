"""Spherize, worked a second way.

D-406 adds `core.spherize`, after After Effects' Spherize (Distort group): a round part of the
layer wrapped onto a sphere, so the middle swells toward the viewer and the picture crowds
toward the rim. Adobe's page says only that the effect "distorts a layer by wrapping a region of
the image onto a sphere" and gives no formula; the rule is this program's own, the same curve as
Fisheye's at convergence 100 (D-387), with the layer left as it is outside the sphere.

The settings. `radius`, 0 to 2500 pixels (After Effects' limit), 100 when added (ours), the
sphere's radius; `center`, two numbers, per cent of the picture as it reaches the effect, -1000
to 1000 each, (50, 50). Both keyable.

The rule. On the picture as it reaches the effect, w by h pixels, its centre point
c = (center / 100 w, center / 100 h) and R = radius. Each pixel, its centre p, v = p - c,
r = |v|. A pixel with r >= R, or every pixel when R is 0, is the picture's own. Inside, with
rho = r / R, the picture is read at

    c + v (2 asin(rho) / pi) / rho        (c itself when r = 0)

by document 21's bilinear sample (clear outside the picture): a point on a half sphere of radius
R seen from straight in front, at the arc length round the sphere from its top that the point
p's arc reaches, scaled so the rim reads the rim. The middle is enlarged pi / 2 times and the
picture crowds toward the rim; the edge of the sphere meets the layer outside it without a step.

Radius is a distance, scaled by a draft; the centre is a share.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size (Tiles' card,
D-404), unmoved unless the case says. The drawing goes into `Fixtures/spherize/media`, the
projects into `Fixtures/spherize`, and the expected frames into
`Fixtures/spherize/expected_spherize.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/spherize_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import tiles_reference as T  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402

W, H = T.W, T.H  # 16 by 10
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "spherize"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"radius": (0, 2500), "center": (-1000, 1000)}
NAMES = ("radius", "center")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def pixel(layer, i, j):
    if 0 <= i < layer["w"] and 0 <= j < layer["h"]:
        return layer["px"][j * layer["w"] + i]
    return EMPTY


def spherize(layer, s):
    w, h = layer["w"], layer["h"]
    c = (s["center"][0] / 100 * w, s["center"][1] / 100 * h)
    big = s["radius"]
    out = []
    for y in range(h):
        for x in range(w):
            vx, vy = x + 0.5 - c[0], y + 0.5 - c[1]
            r = math.sqrt(vx * vx + vy * vy)
            if big <= 0 or r >= big:
                out.append(list(pixel(layer, x, y)))
                continue
            rho = r / big
            k = 0.0 if r == 0 else (2 * math.asin(rho) / math.pi) / rho
            out.append(list(bilinear(layer, c[0] + k * vx, c[1] + k * vy)))
    return {"px": out, "left": layer["left"], "top": layer["top"], "w": w, "h": h}


# --- the cases ------------------------------------------------------------------------------

def case(**kw):
    c = {"drawing": "card", "radius": 100, "center": (50, 50), "shift": 0}
    c.update(kw)
    return c


def settings(c, frame_no):
    held = {}
    for k in NAMES:
        lo, hi = RANGES[k]
        v = value_at(c[k], frame_no)
        held[k] = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def placed(layer, shift):
    """The layer's buffer laid in the composition, moved `shift` pixels across."""
    out = []
    for y in range(H):
        for x in range(W):
            i, j = x - shift - layer["left"], y - layer["top"]
            out.append(list(pixel(layer, i, j)))
    return out


def render(c, frame_no):
    return placed(spherize(T.drawn_layer(c["drawing"]), settings(c, frame_no)), c["shift"])


def plain(c):
    return placed(T.drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-SPHERIZE-001": ("The settings as they start: a sphere of radius 100 round the middle, "
                        "far wider than the 16 by 10 drawing, so every pixel is near its top "
                        "and the whole drawing is enlarged about pi / 2 times round (8, 5).",
                        case(), [0, 4]),
    "FX-SPHERIZE-002": ("Radius 0: no sphere, the drawing as it is.", case(radius=0), [0]),
    "FX-SPHERIZE-003": ("Radius 5 round the middle: the middle swells, the picture crowds "
                        "toward the rim, and the corners outside the circle are the drawing's "
                        "own.", case(radius=5), [0]),
    "FX-SPHERIZE-004": ("Radius 5 round 25, 50 (4, 5): the sphere on the left, cut by the "
                        "drawing's edge.", case(radius=5, center=(25, 50)), [0]),
    "FX-SPHERIZE-005": ("Radius 2500, the most: rho under 0.004 everywhere, so the drawing is "
                        "enlarged pi / 2 times round the middle, smoothly.",
                        case(radius=2500), [0]),
    "FX-SPHERIZE-006": ("Radius 12 round -25, 50 (-4, 5), off the left edge: only the right "
                        "part of the sphere lies on the drawing, its rim near column 7.",
                        case(radius=12, center=(-25, 50)), [0]),
    "FX-SPHERIZE-007": ("Radius 3: a small sphere, nine or so pixels round the middle changed.",
                        case(radius=3), [0]),
    "FX-SPHERIZE-008": ("Radius keyed from 0 at frame 0 to 8 at frame 4, linear: nothing at "
                        "frame 0, 4 at frame 2, 8 at frame 4.",
                        case(radius=keyed((0, 0), (4, 8))), [0, 2, 4]),
    "FX-SPHERIZE-009": ("Centre keyed from 25, 50 at frame 0 to 75, 50 at frame 4, radius 4: "
                        "the sphere slides across.",
                        case(center=keyed((0, (25, 50)), (4, (75, 50))), radius=4), [0, 2, 4]),
    "FX-SPHERIZE-010": ("Radius 5 with the layer moved 3 pixels right: FX-SPHERIZE-003 moved; "
                        "columns 0 to 2 empty.", case(radius=5, shift=3), [0]),
    "FX-SPHERIZE-011": ("Radius eased from 5 at frame 0 to 0 at frame 4 on a curve that "
                        "overshoots: at frame 2 it would pass below 0 and is held there, so "
                        "frames 2 and 4 show the drawing as it is.",
                        case(radius=keyed((0, 5, OVERSHOOT), (4, 0))), [0, 2, 4]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-SPHERIZE-012": ("Radius -1, below 0.", case(radius=-1)),
    "FX-SPHERIZE-013": ("Radius 2501, above 2500.", case(radius=2501)),
    "FX-SPHERIZE-014": ("Centre at 1001, 50, past ten widths.", case(center=(1001, 50))),
    "FX-SPHERIZE-015": ("Centre at 50, -1001.", case(center=(50, -1001))),
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
        "instance_id": "fx-0-0", "type_id": "core.spherize", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in T.DRAWINGS.items():
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

    (OUT / "expected_spherize.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def near(a, b, tol=1e-12):
    return all(abs(u - v) < tol for p, q in zip(a, b) for u, v in zip(p, q))


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    layer = T.drawn_layer("card")
    at = lambda x, y: y * W + x  # noqa: E731
    one = lambda fx: c[fx]["0"]  # noqa: E731

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    first = one("FX-SPHERIZE-001")
    assert first == c["FX-SPHERIZE-001"]["4"] and first != drawn
    assert one("FX-SPHERIZE-002") == drawn
    three = one("FX-SPHERIZE-003")
    # (0, 0) is 8.6 from (8, 5), outside radius 5; (3, 5) is 4.5 in, (2, 5) 5.5 out.
    assert three[at(0, 0)] == drawn[at(0, 0)] and three[at(2, 5)] == drawn[at(2, 5)]
    assert all(three[at(x, y)] == drawn[at(x, y)] for x in range(W) for y in range(H)
               if (x + 0.5 - 8) ** 2 + (y + 0.5 - 5) ** 2 >= 25) and three != drawn
    # (7, 4): v = (-0.5, -0.5), rho = 0.1414, read at c + v (2 asin(rho) / pi) / rho.
    rho = math.sqrt(0.5) / 5
    k = 2 * math.asin(rho) / math.pi / rho
    assert near([three[at(7, 4)]], [bilinear(layer, 8 - 0.5 * k, 5 - 0.5 * k)])
    # At the most radius the read is c + v 2 / pi to within rho^2 / 6 of v.
    five = one("FX-SPHERIZE-005")
    assert near(five, [list(bilinear(layer, 8 + (x + 0.5 - 8) * 2 / math.pi,
                                     5 + (y + 0.5 - 5) * 2 / math.pi))
                       for y in range(H) for x in range(W)], 1e-5)
    six = one("FX-SPHERIZE-006")
    assert all(six[at(x, y)] == drawn[at(x, y)] for x in range(8, W) for y in range(H))
    assert six != drawn
    seven = one("FX-SPHERIZE-007")
    changed = sum(seven[i] != drawn[i] for i in range(W * H))
    assert 0 < changed <= 32
    eight = c["FX-SPHERIZE-008"]
    assert eight["0"] == drawn and len({json.dumps(eight[f]) for f in ("0", "2", "4")}) == 3
    nine = c["FX-SPHERIZE-009"]
    assert len({json.dumps(nine[f]) for f in ("0", "2", "4")}) == 3
    ten = one("FX-SPHERIZE-010")
    assert all(ten[at(x, y)] == three[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(ten[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    raw = value_at(CASES["FX-SPHERIZE-011"][1]["radius"], 2)
    eleven = c["FX-SPHERIZE-011"]
    assert raw < 0 and eleven["0"] == three and eleven["2"] == eleven["4"] == drawn
    print("checked")


if __name__ == "__main__":
    main()
