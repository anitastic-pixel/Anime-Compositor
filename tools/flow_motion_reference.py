"""Flow Motion, worked a second way.

D-385 adds `core.flow_motion`, after CycoreFX's CC Flo Motion: the picture drawn into two knots,
or blown out of them, as a vortex or a black hole draws in what is round it. CycoreFX's manual
says what each control does in a sentence and publishes no formula; the rule below is this
program's own reading of it, and nothing is ported.

The rule. K1 and K2 are `knot_1` and `knot_2`, per cent of the drawing's own width and height as
Radial Blur's centre is; D is the drawing's own diagonal in pixels. Each knot has a reach
sigma = D x 0.01 x 2^(0.7 `falloff`): falloff 0 holds the pull to a hundredth of the diagonal
round the knot, 5 to about a ninth, 10 to more than the whole drawing, nearly no falloff. Each
knot's strength is a = `amount` / 10, or `amount` / 200 with `finer_controls` on, so 20 then is
1 now. At a pixel's centre P, for each knot, d = P - K, r = |d|, g = sigma^2 / (sigma^2 + r^2),
1 at the knot and falling away, and the knot's scale m = 1 + a g when a >= 0, 1 / (1 - a g) when
a < 0. The pixel reads the drawing at

    S = P + (P - K1)(m1 - 1) + (P - K2)(m2 - 1),

so a knot with a positive amount reads from farther out, drawing the picture in towards it, and
a negative one reads from nearer, blowing the picture out of it. m stays above 0, so nothing is
turned inside out. With `tile_edges` on, S reads document 21's bilinear sample of the drawing
repeated round itself, every other copy turned over (a pixel column i outside reads column j,
j = i mod 2w, 2w - 1 - j when j >= w; the same for rows), so the drawing's own edges never show;
off, it is transparent outside the drawing. `antialiasing` low takes one point, the pixel's
centre; medium 2 by 2 and high 4 by 4 evenly spaced points across the pixel, averaged, which
smooths the fine copies a strong pull draws round a knot.

Amount 0 at both knots is the drawing, whatever the antialiasing. The layer does not grow. Nothing here is a distance in
pixels, so a draft needs no change.

`knot_1` and `knot_2` -1000 to 1000 per cent each way, 25, 50 and 75, 50 when added; `amount_1`
and `amount_2` -1000 to 1000, 10 and -10; `falloff` 0 to 10, 5; `tile_edges` `off` or `on`, `on`;
`finer_controls` `off` or `on`, `off`; `antialiasing` `low`, `medium` or `high`, `low`. The
numbers are keyable. The values when added are chosen here.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/flow_motion/media`, the projects
into `Fixtures/flow_motion`, and the expected frames into
`Fixtures/flow_motion/expected_flow_motion.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/flow_motion_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "flow_motion"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"knot_1": (-1000, 1000), "amount_1": (-1000, 1000), "knot_2": (-1000, 1000),
          "amount_2": (-1000, 1000), "falloff": (0, 10)}
NAMES = ("knot_1", "amount_1", "knot_2", "amount_2", "falloff", "tile_edges", "finer_controls",
         "antialiasing")
POINTS = {"low": 1, "medium": 2, "high": 4}
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def point(p):
    return p[0] / 100 * W, p[1] / 100 * H


def reach(falloff):
    return math.hypot(W, H) * 0.01 * 2 ** (0.7 * falloff)


def scale(a, g):
    return 1 + a * g if a >= 0 else 1 / (1 - a * g)


def source(s_, px, py):
    """Where the point (px, py) reads the drawing."""
    sigma2 = reach(s_["falloff"]) ** 2
    per = 200 if s_["finer_controls"] == "on" else 10
    sx, sy = px, py
    for k in (1, 2):
        a = s_[f"amount_{k}"] / per
        if a == 0:
            continue
        kx, ky = point(s_[f"knot_{k}"])
        dx, dy = px - kx, py - ky
        m = scale(a, sigma2 / (sigma2 + dx * dx + dy * dy))
        sx += dx * (m - 1)
        sy += dy * (m - 1)
    return sx, sy


def fold(i, n):
    j = i % (2 * n)
    return 2 * n - 1 - j if j >= n else j


def mirrored(layer, x, y):
    """Document 21's bilinear sample of the drawing repeated round itself, every other copy
    turned over."""
    fx, fy = x - 0.5, y - 0.5
    x0, y0 = math.floor(fx), math.floor(fy)
    ux, uy = fx - x0, fy - y0
    out = [0.0] * 4
    for dy, wy in ((0, 1 - uy), (1, uy)):
        for dx, wx in ((0, 1 - ux), (1, ux)):
            if wx * wy:
                p = layer["px"][fold(y0 + dy, layer["h"]) * layer["w"] + fold(x0 + dx, layer["w"])]
                for i in range(4):
                    out[i] += p[i] * wx * wy
    return out


def flowed(layer, s_, x, y):
    if not (0 <= x < layer["w"] and 0 <= y < layer["h"]):
        return EMPTY
    if s_["amount_1"] == 0 and s_["amount_2"] == 0:
        return list(layer["px"][y * layer["w"] + x])
    n = POINTS[s_["antialiasing"]]
    sample = mirrored if s_["tile_edges"] == "on" else bilinear
    out = [0.0] * 4
    for j in range(n):
        for i in range(n):
            p = sample(layer, *source(s_, x + (i + 0.5) / n, y + (j + 0.5) / n))
            for c in range(4):
                out[c] += p[c]
    return [v / (n * n) for v in out]


# --- the cases ------------------------------------------------------------------------------

def case(knot_1=(25, 50), amount_1=10, knot_2=(75, 50), amount_2=-10, falloff=5, tile="on",
         finer="off", aa="low", shift=0):
    return {"drawing": "stripes", "knot_1": knot_1, "amount_1": amount_1, "knot_2": knot_2,
            "amount_2": amount_2, "falloff": falloff, "tile_edges": tile,
            "finer_controls": finer, "antialiasing": aa, "shift": shift}


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
    return [flowed(layer, s_, x - c["shift"], y) for y in range(H) for x in range(W)]


def plain(c):
    layer = B.drawn_layer(c["drawing"])
    return [list(layer["px"][y * W + x - c["shift"]]) if 0 <= x - c["shift"] < W else EMPTY
            for y in range(H) for x in range(W)]


ONE = {"amount_2": 0, "tile": "off"}  # the first knot alone, nothing tiled
MID = {"knot_1": (50, 50)}

CASES = {
    "FX-FLOW-001": ("The settings as they start: knot 1 at a quarter of the way across, half way "
                    "down, drawing in by 10; knot 2 at three quarters, blowing out by 10; falloff "
                    "5, a reach of about 2 pixels here; Tile Edges on. Round the left knot the "
                    "stripes are drawn in and narrow, round the right they swell.", case(), [0]),
    "FX-FLOW-002": ("Both amounts 0: the drawing, untouched.", case(amount_1=0, amount_2=0), [0]),
    "FX-FLOW-003": ("Knot 1 alone, in the middle, drawing in by 10, Tile Edges off: the middle "
                    "reads from twice as far out, so the band and the stripes are pulled in "
                    "to half size there.", case(**ONE, **MID), [0]),
    "FX-FLOW-004": ("Knot 1 alone, in the middle, blowing out by 10: the middle reads from half "
                    "as far out, swelling to twice the size.",
                    case(amount_1=-10, **ONE, **MID), [0]),
    "FX-FLOW-005": ("Knot 1 alone, in the middle, drawing in by 30, falloff 10, Tile Edges on: "
                    "the whole drawing reads from three to four times as far out, shrunk, and "
                    "its copies, turned over by turns, fill the rest.",
                    case(amount_1=30, amount_2=0, falloff=10, **MID), [0]),
    "FX-FLOW-006": ("FX-FLOW-005 with Tile Edges off: the shrunk drawing alone, transparent round "
                    "it.", case(amount_1=30, amount_2=0, falloff=10, tile="off", **MID), [0]),
    "FX-FLOW-007": ("Finer Controls on, Amount 200: 20 times finer, the same as 10 without: "
                    "FX-FLOW-003 exactly.", case(amount_1=200, finer="on", **ONE, **MID), [0]),
    "FX-FLOW-008": ("Falloff 0: the pull held to a fifth of a pixel round the knot: the four "
                    "pixels round it move about a twentieth of a pixel, the rest far less.",
                    case(falloff=0, **ONE, **MID), [0]),
    "FX-FLOW-009": ("FX-FLOW-005 with Antialiasing medium: each pixel averages 2 by 2 points, "
                    "the copies softer.",
                    case(amount_1=30, amount_2=0, falloff=10, aa="medium", **MID), [0]),
    "FX-FLOW-010": ("FX-FLOW-005 with Antialiasing high: 4 by 4 points a pixel.",
                    case(amount_1=30, amount_2=0, falloff=10, aa="high", **MID), [0]),
    "FX-FLOW-011": ("Knot 2 alone at the top left corner, blowing out by 40, falloff 7: the "
                    "corner swells across the drawing.",
                    case(amount_1=0, knot_2=(0, 0), amount_2=-40, falloff=7), [0]),
    "FX-FLOW-012": ("Amount 1 keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 the "
                    "drawing, frame 2 FX-FLOW-003.",
                    case(amount_1=keyed((0, 0), (4, 20)), **ONE, **MID), [0, 2, 4]),
    "FX-FLOW-013": ("Knot 1 keyed from 25, 50 at frame 0 to 75, 50 at frame 4: the pull slides "
                    "across.", case(knot_1=keyed((0, (25, 50)), (4, (75, 50))), **ONE),
                    [0, 2, 4]),
    "FX-FLOW-014": ("FX-FLOW-003 moved three pixels right: the same, moved; nothing grows, and "
                    "the three columns left of the drawing stay empty.",
                    case(shift=3, **ONE, **MID), [0]),
    "FX-FLOW-015": ("Amount 1 eased from 0 at frame 0 to 1000 at frame 4 on a curve that "
                    "overshoots: at frame 2 it would pass 1000 and is held there.",
                    case(amount_1=keyed((0, 0, OVERSHOOT), (4, 1000)), **ONE, **MID),
                    [0, 2, 4]),
}

INVALID = {
    "FX-FLOW-016": ("Amount 1 1001, above 1000.", case(amount_1=1001)),
    "FX-FLOW-017": ("Amount 2 -1001, below -1000.", case(amount_2=-1001)),
    "FX-FLOW-018": ("Falloff 11, above 10.", case(falloff=11)),
    "FX-FLOW-019": ("Falloff -1, below 0.", case(falloff=-1)),
    "FX-FLOW-020": ("Tile Edges written \"yes\".", case(tile="yes")),
    "FX-FLOW-021": ("Finer Controls written \"On\", with a capital.", case(finer="On")),
    "FX-FLOW-022": ("Antialiasing written \"best\".", case(aa="best")),
    "FX-FLOW-023": ("Knot 1 at 50, 1001, past ten heights.", case(knot_1=(50, 1001))),
    "FX-FLOW-024": ("Amount 1 keyed to 2000 at frame 4.", case(amount_1=keyed((0, 0), (4, 2000)))),
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
        "instance_id": "fx-0-0", "type_id": "core.flow_motion", "enabled": True,
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

    (OUT / "expected_flow_motion.json").write_text(json.dumps(expected, indent=1) + "\n",
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

    # The fold repeats the drawing turned over, and the scale never reaches 0.
    assert [fold(i, 3) for i in range(-4, 8)] == [2, 2, 1, 0, 0, 1, 2, 2, 1, 0, 0, 1]
    assert scale(1, 1) == 2 and scale(-1, 1) == 0.5 and scale(-100, 1) > 0
    # Tiling changes nothing inside the drawing: a mirrored sample there is the plain one.
    layer = B.drawn_layer("stripes")
    assert mirrored(layer, 3.3, 4.7) == bilinear(layer, 3.3, 4.7)

    assert c["FX-FLOW-002"]["0"] == drawn
    three = c["FX-FLOW-003"]["0"]
    assert c["FX-FLOW-007"]["0"] == three != c["FX-FLOW-004"]["0"]
    for fx in ("FX-FLOW-001", "FX-FLOW-003", "FX-FLOW-004", "FX-FLOW-005", "FX-FLOW-008",
               "FX-FLOW-011"):
        assert c[fx]["0"] != drawn, fx
    five, six = c["FX-FLOW-005"]["0"], c["FX-FLOW-006"]["0"]
    assert six[at(0, 0)] == EMPTY != five[at(0, 0)]
    assert c["FX-FLOW-009"]["0"] != five != c["FX-FLOW-010"]["0"]
    twelve = c["FX-FLOW-012"]
    assert twelve["0"] == drawn and twelve["2"] == three != twelve["4"]
    thirteen = c["FX-FLOW-013"]
    assert thirteen["0"] != thirteen["2"] != thirteen["4"]
    fourteen = c["FX-FLOW-014"]["0"]
    assert all(fourteen[at(x, y)] == three[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(fourteen[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    fifteen = c["FX-FLOW-015"]
    assert fifteen["2"] == fifteen["4"] == render(case(amount_1=1000, **ONE, **MID), 0)
    print("checked")


if __name__ == "__main__":
    main()
