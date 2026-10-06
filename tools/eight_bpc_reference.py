"""D-330's 8 bpc (After Effects) working depth, worked a second way.

From P-26's tutorial 2 replay. The tutorial is drawn in an 8 bpc project until its step F. Its
reflection copies keep only the bolt's tips, blur them wide with Fast Box Blur and lift them 9 to
17 stops with Exposure. After Effects in 8 bpc, with no linear working space, blurs the display
values and keeps each layer's pixels as whole numbers 0 to 255 between effects, so the blur's faint
outer edge becomes 0 and Exposure has nothing out there to brighten. This program works in linear
light and keeps every value, so the same edge lit a block 600 pixels wide
(`tools/d330_eight_bit_demo.py`).

A composition's working depth gains a third choice, saved as `eight_bpc: true`:

- After every effect on a layer, each pixel is held to what 8 bits keep: its alpha held to 0..1
  and rounded to a 255th; its straight colour held to 0..1, through the sRGB curve, rounded to a
  255th and back. A pixel whose alpha rounds to 0 is clear.
- The blurs that average neighbours, Gaussian Blur, Fast Box Blur, Directional Blur and Radial
  Blur, average the display values (the straight colour through the sRGB curve, premultiplied),
  as After Effects does without a linear working space; their result is taken back to linear
  light before the rounding.
- Every other effect works as in Display, Exposure in linear light as After Effects' CS3 manual
  (p.402) says. Blending is as in Display.
- A layer with no effects is not rounded (its drawing is already 8 bits). A composition inside
  another follows the outermost one's depth, as D-319.
- Not both: a file with `float_depth` and `eight_bpc` both true is refused when read.

Every case is blurriness_reference's composition, 40 by 12, and its drawing `block`, a 4 by 4
orange square against the left edge. The projects go into `Fixtures/eight_bpc`, the expected
frames into `Fixtures/eight_bpc/expected_eight_bpc.json`.

**This file never runs the build's code path.** It works in double precision on lists, from the
drawing's 8-bit values. Every value it rounds is checked to lie well clear of a rounding step,
so a build working in single precision cannot land on the other side of one.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/eight_bpc_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import blurriness_reference as B  # noqa: E402
import fast_box_blur_reference as F  # noqa: E402
import smooth_reference as S  # noqa: E402
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import setting_json  # noqa: E402

W, H, FRAMES = B.W, B.H, B.FRAMES
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "eight_bpc"
TOLERANCE = 2e-5  # document 25's default for a filter
CLEAR = 1e-3      # how far, in 255ths, every rounded value lies from a rounding step

near = []         # the closest any rounded value came to a step


def q(v):
    """A value 0..1 rounded to a 255th, noting how near it came to a step."""
    s = min(max(v, 0.0), 1.0) * 255
    near.append(abs(s - math.floor(s) - 0.5))
    return round(s) / 255


def eight(px):
    """D-330: each pixel held to 8 bits, in display values."""
    out = []
    for p in px:
        a = q(p[3])
        if a <= 0 or p[3] <= 0:
            out.append([0.0, 0.0, 0.0, 0.0])
            continue
        out.append([srgb_to_linear(q(S.linear_to_srgb(min(max(p[c] / p[3], 0.0), 1.0)))) * a
                    for c in range(3)] + [a])
    return out


def display(px):
    """Linear premultiplied to display premultiplied: the straight colour through the curve."""
    return [S.encoded(p) for p in px]


def linear(px):
    return [S.working(e) for e in px]


# --- the cases ------------------------------------------------------------------------------

def case(effects, eight_bpc=True):
    """`effects`: a list of ("fast_box", radius) | ("blurriness", number) | ("exposure", stops)."""
    return {"effects": effects, "eight": eight_bpc}


def render(c):
    px = B.plain()
    for kind, n in c["effects"]:
        if kind == "exposure":
            px = B.exposure(px, n)
        else:
            run = (lambda p: F.blur(p, n, 3, "transparent", "both")) if kind == "fast_box" else \
                  (lambda p: B.blur(p, n, "blurriness", "transparent", "both"))
            px = linear(run(display(px))) if c["eight"] else run(px)
        if c["eight"]:
            px = eight(px)
    return px


CASES = {
    "FX-8BPC-001": ("8 bpc: Fast Box Blur radius 2 (3 passes), then Exposure +6. The blur "
                    "averages display values and its faint edge rounds to 0, so the light ends "
                    "sooner than in Float (FX-FASTBOX-007), and nothing past white is kept.",
                    case([("fast_box", 2), ("exposure", 6)])),
    "FX-8BPC-002": ("8 bpc: Gaussian Blur, Blurriness 4, then Exposure +6. The long faint tail "
                    "that Float lights (FX-BLURRY-007) rounds to 0.",
                    case([("blurriness", 4), ("exposure", 6)])),
    "FX-8BPC-003": ("8 bpc: Exposure -1 alone, worked in linear light, then rounded to 8 bits.",
                    case([("exposure", -1)])),
    "FX-8BPC-004": ("8 bpc: Fast Box Blur radius 2 alone. Every value is a whole 255th in "
                    "display values.", case([("fast_box", 2)])),
    "FX-8BPC-005": ("8 bpc with no effect: the drawing untouched.", case([])),
}


# Files that are refused when read: each is FX-8BPC-001's project with one key changed.
LOADS = {
    "both_depths.json": ("`float_depth` and `eight_bpc` both true: refused, "
                         "`PROJECT_SCHEMA_INVALID`.", {"float_depth": True}),
    "eight_word.json": ("`eight_bpc` is the word \"yes\", not true or false: refused, "
                        "`PROJECT_SCHEMA_INVALID`.", {"eight_bpc": "yes"}),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = B.project_json(fx, B.case(1, units=None))
    effects = []
    for i, (kind, n) in enumerate(c["effects"]):
        if kind == "exposure":
            t, params = "core.exposure", {"stops": setting_json(n)}
        elif kind == "fast_box":
            t, params = "core.fast_box_blur", {"radius": setting_json(n),
                                               "iterations": setting_json(3)}
        else:
            t, params = "core.gaussian_blur", {"sigma_px": setting_json(n),
                                               "units": "blurriness"}
        effects.append({"instance_id": f"fx-0-{i}", "type_id": t, "enabled": True,
                        "parameters": params})
    comp = p["compositions"][0]
    comp["layers"][0]["effects"] = effects
    if c["eight"]:
        comp["eight_bpc"] = True
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "block.png").write_bytes(S.png(B.DRAWING))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c) in CASES.items():
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": render(c)}}
        print(f"{fx}: {says[:60]}")
    expected["loads"] = {}
    for name, (says, keys) in LOADS.items():
        p = project_json("FX-8BPC-001", CASES["FX-8BPC-001"][1])
        p["compositions"][0].update(keys)
        (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
        expected["loads"][name] = {"says": says, "refused": "PROJECT_SCHEMA_INVALID"}
    (OUT / "expected_eight_bpc.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    lit = lambda px: sum(1 for p in px if p[3] > 0)  # noqa: E731

    assert min(near) > CLEAR, min(near)
    # The light ends sooner than in Float, and is held to white.
    flt = F.render(F.case(2, stops=6, float_depth=True))
    one = c["FX-8BPC-001"]
    assert lit(one) < lit(flt), (lit(one), lit(flt))
    assert all(p[i] <= p[3] + 1e-12 for p in one for i in range(3))
    two = c["FX-8BPC-002"]
    gauss = B.render(B.case(4, stops=6, float_depth=True))
    assert lit(two) < lit(gauss) and gauss[at(8, 6)][3] > 0 and two[at(8, 6)] == [0.0] * 4
    # Every value of a rounded pixel is a whole 255th in display values.
    for px in (one, two, c["FX-8BPC-003"], c["FX-8BPC-004"]):
        for e in display(px):
            for v in [e[3]] + ([e[i] / e[3] for i in range(3)] if e[3] > 0 else []):
                assert abs(v * 255 - round(v * 255)) < 1e-9, v
    # Exposure -1 halves the linear light, then rounds: not the same as halving.
    three = c["FX-8BPC-003"]
    half = B.exposure(B.plain(), -1)
    assert three[at(1, 5)] != half[at(1, 5)] and abs(three[at(1, 5)][1] - half[at(1, 5)][1]) < 3e-3
    assert c["FX-8BPC-005"] == B.plain()


if __name__ == "__main__":
    main()
