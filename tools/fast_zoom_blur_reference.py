"""Fast Zoom Blur, worked a second way.

`core.fast_zoom_blur`, modelled on CycoreFX's CC Radial Fast Blur (its manual, and a tutorial
that sweeps light across text with it): quick fading streaks out from a centre, plain, or
keeping only what is brighter, or only what is darker, than the pixel. Nothing is ported; this
is this program's own reading. Document 21 is the rule in words; this file is the reference for
the numbers document 25 pins against it.

The rule. With c the centre in layer pixels, (center_x / 100 * width, center_y / 100 * height)
of the drawing's own size, as Radial Blur's (D-95), p the centre of the output pixel, d = p - c
and r = |d|: the path is r A / 100 and n = min(ceil(path) + 1, 256). With n = 1 the output is the
sample at p. Otherwise, for k = 0 to n - 1, v_k is document 21's bilinear sample of the layer at
c + (1 - A k / (100 (n - 1))) d, transparent outside the layer (so v_0 is the pixel itself and
the points run back toward the centre, and everything streaks outward), and f_k = (n - k) / n,
falling from 1 to 1 / n along the streak.

- `zoom` "standard": the mean of the v_k weighted by f_k, a streak fading as it runs out.
- `zoom` "brightest": channel by channel, red, green, blue and alpha alike, premultiplied, the
  largest of v_0 + f_k (v_k - v_0): the pixel kept, and lit further by any brighter point along
  the streak, by less the further back it lies. Light streaks; nothing darkens.
- `zoom` "darkest": the smallest of the same, so dark streaks and nothing brightens.
- `amount` A, 0 to 100, per cent of r. `center`, two numbers, per cent of the drawing's width
  and height, -1000 to 1000.

The layer does not grow; past it is transparent.

Every case is directional blur's drawing, 16 by 10, as Radial Blur's cases. The projects go into
`Fixtures/fast_zoom_blur`, the expected frames into
`Fixtures/fast_zoom_blur/expected_fast_zoom_blur.json`.

**This file never runs the build's code path.** It works in double precision on lists.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/fast_zoom_blur_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import directional_blur_reference as D  # noqa: E402
import radial_blur_reference as RB  # noqa: E402

W, H = D.W, D.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "fast_zoom_blur"
TOLERANCE = 2e-5  # document 25's default for a filter
AMOUNT = (0, 100)
CENTER = (-1000, 1000)
MOST = 256


# --- the rule -------------------------------------------------------------------------------

def streak(layer, amount, center, x, y):
    """The samples v_k along the pixel's streak, v_0 first."""
    cx, cy = center[0] / 100 * W, center[1] / 100 * H
    px, py = x + 0.5, y + 0.5
    dx, dy = px - cx, py - cy
    n = min(math.ceil(math.sqrt(dx * dx + dy * dy) * amount / 100) + 1, MOST)
    if n == 1:
        return [RB.bilinear(layer, px, py)]
    return [RB.bilinear(layer, cx + (1 - amount * k / (100 * (n - 1))) * dx,
                        cy + (1 - amount * k / (100 * (n - 1))) * dy) for k in range(n)]


def blurred(layer, zoom, amount, center, x, y):
    if not (0 <= x - layer["left"] < layer["w"] and 0 <= y - layer["top"] < layer["h"]):
        return [0.0] * 4
    v = streak(layer, amount, center, x, y)
    n = len(v)
    f = [(n - k) / n for k in range(n)]
    if zoom == "standard":
        return [sum(f[k] * v[k][i] for k in range(n)) / sum(f) for i in range(4)]
    pick = max if zoom == "brightest" else min
    return [pick(v[0][i] + f[k] * (v[k][i] - v[0][i]) for k in range(n)) for i in range(4)]


# --- the cases ------------------------------------------------------------------------------

def case(zoom=None, amount=50, center=(50, 50), shift=0):
    """`zoom` None: the file does not say, so standard."""
    return {"drawing": "bars", "zoom": zoom, "amount": amount, "center": center,
            "shift": shift, "before": None}


def render(c, frame_no):
    layer = RB.layer_of(c)
    amount = RB.clamp(value_at(c["amount"], frame_no), AMOUNT)
    center = [RB.clamp(v, CENTER) for v in value_at(c["center"], frame_no)]
    return [blurred(layer, c["zoom"] or "standard", amount, center, x - c["shift"], y)
            for y in range(H) for x in range(W)]


def plain(c):
    return render(case(amount=0, shift=c["shift"]), 0)


CASES = {
    "FX-FASTZOOM-001": ("Standard, amount 50 (the default), about the middle: every edge "
                        "streaks outward, fading as it runs.", case(), [0]),
    "FX-FASTZOOM-002": ("Brightest: only light streaks; the skin block streaks outward over "
                        "the clear pixels round it, the dark line, with only clear pixels "
                        "behind it, is kept, and nothing grows darker anywhere.",
                        case("brightest"), [0]),
    "FX-FASTZOOM-003": ("Darkest: only dark streaks; the line down the left edge, with clear "
                        "pixels behind it, fades to a quarter, the skin block, whose streaks "
                        "run back over itself, is kept, and nothing grows lighter.",
                        case("darkest"), [0]),
    "FX-FASTZOOM-004": ("Amount 0: the drawing, untouched.", case(amount=0), [0]),
    "FX-FASTZOOM-005": ("Standard, amount 100, the most: each pixel reads all the way back to "
                        "the centre.", case(amount=100), [0]),
    "FX-FASTZOOM-006": ("Brightest about centre 25, 50, the point (4, 5): the block streaks "
                        "right, away from it.", case("brightest", center=(25, 50)), [0]),
    "FX-FASTZOOM-007": ("Amount keyed from 0 at frame 0 to 80 at frame 4, brightest, linear.",
                        case("brightest", keyed((0, 0), (4, 80))), [0, 2, 4]),
    "FX-FASTZOOM-008": ("Centre keyed from 0, 50 at frame 0 to 100, 50 at frame 4, brightest "
                        "amount 80, as a light passing along a line of text.",
                        case("brightest", 80, keyed((0, (0, 50)), (4, (100, 50)))), [0, 2, 4]),
    "FX-FASTZOOM-009": ("Standard, moved three pixels right: the centre moves with the drawing, "
                        "and nothing is drawn left of it.", case(shift=3), [0]),
}

INVALID = {
    "FX-FASTZOOM-010": ("Amount 101, above 100.", case(amount=101)),
    "FX-FASTZOOM-011": ("Amount -1, below 0.", case(amount=-1)),
    "FX-FASTZOOM-012": ("Zoom \"bright\", which is not one.", case("bright")),
    "FX-FASTZOOM-013": ("Zoom \"Standard\": the word is exact.", case("Standard")),
    "FX-FASTZOOM-014": ("Centre 1001, 50, past ten widths.", case(center=(1001, 50))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = RB.project_json(fx, RB.case(shift=c["shift"]))
    params = {"amount": setting_json(c["amount"]), "center": setting_json(c["center"])}
    if c["zoom"] is not None:
        params["zoom"] = c["zoom"]
    p["compositions"][0]["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.fast_zoom_blur", "enabled": True,
        "parameters": params}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in D.DRAWINGS.items():
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
    (OUT / "expected_fast_zoom_blur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b: all(abs(p - q) < 1e-9 for u, v in zip(a, b)  # noqa: E731
                            for p, q in zip(u, v))
    one, bright, dark = c["FX-FASTZOOM-001"], c["FX-FASTZOOM-002"], c["FX-FASTZOOM-003"]
    assert one != drawn and bright != one and dark != one and bright != dark
    # Brightest never darkens and darkest never brightens, channel by channel.
    assert all(bright[i][k] >= drawn[i][k] - 1e-12 and dark[i][k] <= drawn[i][k] + 1e-12
               for i in range(W * H) for k in range(4))
    # Brightest: the block's streak lights the clear pixels outside it; the dark line, with only
    # clear pixels behind it, is kept as it is.
    assert bright[at(4, 4)][3] > 0.5 and near([bright[at(0, 4)]], [drawn[at(0, 4)]])
    # Darkest: the line, with clear pixels behind it, fades to a quarter; the block, whose
    # streaks run back over itself, is kept.
    assert dark[at(0, 4)][3] < 0.3 and near([dark[at(5, 3)], dark[at(9, 6)]],
                                             [drawn[at(5, 3)], drawn[at(9, 6)]])
    assert c["FX-FASTZOOM-004"] == drawn
    assert c["FX-FASTZOOM-005"][at(13, 8)][3] > one[at(13, 8)][3]
    six = c["FX-FASTZOOM-006"]
    assert six[at(12, 5)][3] > drawn[at(12, 5)][3] and six[at(2, 4)][3] < 1e-9
    seven = expected["cases"]["FX-FASTZOOM-007"]["frames"]
    assert seven["0"] == drawn and near(seven["2"], render(case("brightest", 40), 0))
    eight = expected["cases"]["FX-FASTZOOM-008"]["frames"]
    assert near(eight["2"], render(case("brightest", 80, (50, 50)), 0))
    assert eight["0"] != eight["4"]
    nine = c["FX-FASTZOOM-009"]
    assert all(nine[at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    assert all(near([nine[at(x, y)]], [one[at(x - 3, y)]]) for x in range(3, W) for y in range(H))
    for name, px in c.items():
        for p in px:
            assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                      for v in p[:3]), (name, p)


if __name__ == "__main__":
    main()
