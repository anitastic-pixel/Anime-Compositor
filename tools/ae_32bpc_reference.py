"""D-333's 32 bpc (After Effects) working depth, worked a second way.

From P-26's tutorial 2 replay. After its step F the tutorial is in After Effects' 32 bpc, which by
default has no linear working space. Its Lightning Diff copy blurs the bolt's tips wide with Fast
Box Blur, puts them on black with Solid Composite and lifts them +20.49 stops with Exposure. This
program's Float works in linear light, so the blur's faint edge, times about 1.4 million, lit a
block 600 pixels wide where the tutorial shows a soft pool (`tools/d333_ae_32bpc_demo.py`).

A composition's working depth gains a fourth choice, saved as `ae_32bpc: true` beside
`float_depth: true`. It is Float, with two differences in a layer's effects:

- The blurs that average neighbours, Gaussian Blur, Fast Box Blur, Directional Blur and Radial
  Blur, average the display values (the straight colour, held at 0 below and not held above,
  through the sRGB curve, premultiplied); their result is taken back to linear light the same way.
- Exposure works as After Effects' CS3 manual (p.402) says, "in a linear color space", taking the
  display value there and back with a plain 2.2 power curve (Adobe's community: non-linear
  blending uses "a 2.2 gamma"). The display value is multiplied by `(2^stops)^(1/2.2)`: going to
  linear with `d^2.2`, multiplying by `2^stops` and coming back with `^(1/2.2)` is that. A pixel
  with no alpha is multiplied as in Float.

No value is rounded and none is held at white, as in Float. Every other effect, and blending, is
as in Float. A composition inside another follows the outermost one's depth, as D-319. A file with
`ae_32bpc` true and `float_depth` not true is refused when read.

Which curve After Effects really uses is not stated in any source found. The 2.2 curve is this
program's best reading, chosen because it alone turns tutorial 2's block into its pool.

Every case is blurriness_reference's composition, 40 by 12, and its drawing `block`, a 4 by 4
orange square against the left edge. The projects go into `Fixtures/ae_32bpc`, the expected
frames into `Fixtures/ae_32bpc/expected_ae_32bpc.json`.

**This file never runs the build's code path.** It works in double precision on lists, from the
drawing's 8-bit values.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/ae_32bpc_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import blurriness_reference as B  # noqa: E402
import fast_box_blur_reference as F  # noqa: E402
import smooth_reference as S  # noqa: E402
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import setting_json  # noqa: E402

W, H, FRAMES = B.W, B.H, B.FRAMES
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "ae_32bpc"
TOLERANCE = 2e-5  # document 25's default for a filter


def display(px):
    """Linear premultiplied to display premultiplied, held at 0 below, not above."""
    return [[S.linear_to_srgb(max(p[i] / p[3], 0.0)) * p[3] for i in range(3)] + [p[3]]
            if p[3] > 0 else [0.0] * 4 for p in px]


def linear(px):
    return [[srgb_to_linear(max(e[i] / e[3], 0.0)) * e[3] for i in range(3)] + [e[3]]
            if e[3] > 0 else [0.0] * 4 for e in px]


def exposure(px, stops, ae):
    """Float's Exposure, or D-333's: the display value multiplied by (2^stops)^(1/2.2)."""
    if not ae:
        return B.exposure(px, stops)
    k = (2 ** stops) ** (1 / 2.2)
    return [[srgb_to_linear(S.linear_to_srgb(max(p[i] / p[3], 0.0)) * k) * p[3] for i in range(3)]
            + [p[3]] if p[3] > 0 else [p[i] * 2 ** stops for i in range(3)] + [p[3]] for p in px]


def on_black(px):
    """Solid Composite, black, Normal, both opacities 100: the layer over opaque black."""
    return [p[:3] + [1.0] for p in px]


# --- the cases ------------------------------------------------------------------------------

def case(effects, ae=True):
    """`effects`: a list of ("fast_box", radius) | ("blurriness", number) | ("exposure", stops)
    | ("black", None)."""
    return {"effects": effects, "ae": ae}


def render(c):
    px = B.plain()
    for kind, n in c["effects"]:
        if kind == "exposure":
            px = exposure(px, n, c["ae"])
        elif kind == "black":
            px = on_black(px)
        else:
            run = (lambda p: F.blur(p, n, 3, "transparent", "both")) if kind == "fast_box" else \
                  (lambda p: B.blur(p, n, "blurriness", "transparent", "both"))
            px = linear(run(display(px))) if c["ae"] else run(px)
    return px


TUTORIAL = [("fast_box", 2), ("black", None), ("exposure", 4)]
GAUSS = [("blurriness", 4), ("black", None), ("exposure", 4)]

CASES = {
    "FX-AE32-001": ("32 bpc (After Effects): Fast Box Blur radius 2 (3 passes), Solid Composite "
                    "on black, then Exposure +4, tutorial 2's reflection in small. The blur "
                    "averages display values and Exposure lifts them through the 2.2 curve, so "
                    "the faint edge stays dim where Float lights it; light past white is kept.",
                    case(TUTORIAL)),
    "FX-AE32-002": ("32 bpc (After Effects): Gaussian Blur, Blurriness 4, Solid Composite on "
                    "black, then Exposure +4. The long faint tail stays dim.", case(GAUSS)),
    "FX-AE32-003": ("32 bpc (After Effects): Exposure -1 alone, the display value times "
                    "2^(-1/2.2).", case([("exposure", -1)])),
    "FX-AE32-004": ("32 bpc (After Effects): Exposure +2, then Fast Box Blur radius 2. The blur "
                    "averages display values past white without holding them.",
                    case([("exposure", 2), ("fast_box", 2)])),
    "FX-AE32-005": ("32 bpc (After Effects): Solid Composite on black, then Fast Box Blur radius "
                    "2. The orange fades to black through display values.",
                    case([("black", None), ("fast_box", 2)])),
    "FX-AE32-006": ("32 bpc (After Effects) with no effect: the drawing untouched.", case([])),
}


# Files that are refused when read: each is FX-AE32-001's project with keys changed.
LOADS = {
    "ae32_alone.json": ("`ae_32bpc` true without `float_depth`: refused, "
                        "`PROJECT_SCHEMA_INVALID`.", {"float_depth": None}),
    "ae32_word.json": ("`ae_32bpc` is the word \"yes\", not true or false: refused, "
                       "`PROJECT_SCHEMA_INVALID`.", {"ae_32bpc": "yes"}),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = B.project_json(fx, B.case(1, units=None))
    effects = []
    for i, (kind, n) in enumerate(c["effects"]):
        if kind == "exposure":
            t, params = "core.exposure", {"stops": setting_json(n)}
        elif kind == "black":
            t, params = "core.solid_composite", {"source_opacity": 100, "color": "#000000",
                                                 "opacity": 100, "blend": "normal"}
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
    comp["float_depth"] = True
    if c["ae"]:
        comp["ae_32bpc"] = True
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
        p = project_json("FX-AE32-001", CASES["FX-AE32-001"][1])
        comp = p["compositions"][0]
        for k, v in keys.items():
            if v is None:
                comp.pop(k)
            else:
                comp[k] = v
        (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
        expected["loads"][name] = {"says": says, "refused": "PROJECT_SCHEMA_INVALID"}
    (OUT / "expected_ae_32bpc.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    white = lambda px: sum(1 for p in px if p[3] > 0 and p[0] / p[3] >= 1)  # noqa: E731

    for fx, chain, edge in (("FX-AE32-001", TUTORIAL, 9), ("FX-AE32-002", GAUSS, 8)):
        ae, flt = c[fx], render(case(chain, ae=False))
        # No more pixels are past white than in Float and the faint edge is under half as bright,
        # though the middle rises higher (the 2.2 curve against the sRGB curve's steeper one):
        # nothing is held at white.
        assert 0 < white(ae) <= white(flt), (fx, white(ae), white(flt))
        assert ae[at(edge, 6)][0] < flt[at(edge, 6)][0] / 2, (fx, ae[at(edge, 6)], flt[at(edge, 6)])
        assert ae[at(1, 6)][0] > flt[at(1, 6)][0]
        assert ae[at(1, 6)][0] > 1, ae[at(1, 6)]
        print(f"{fx}: {white(ae)} pixels past white, {white(flt)} in Float")
    # Exposure -1 is the display value times 2^(-1/2.2), not half the light.
    three, px = c["FX-AE32-003"], B.plain()
    r = px[at(1, 5)][0]
    assert abs(three[at(1, 5)][0] - srgb_to_linear(S.linear_to_srgb(r) * 2 ** (-1 / 2.2))) < 1e-12
    assert abs(three[at(1, 5)][0] - r / 2) > 5e-3
    # The blur keeps light past white, and averages it differently from Float.
    four = c["FX-AE32-004"]
    assert four[at(1, 6)][0] / four[at(1, 6)][3] > 1
    assert abs(four[at(5, 6)][0] - render(case([("exposure", 2), ("fast_box", 2)], ae=False))[at(5, 6)][0]) > 1e-3
    # Fading to black through display values: the same alpha, a different colour.
    five, lin = c["FX-AE32-005"], render(case([("black", None), ("fast_box", 2)], ae=False))
    assert all(abs(a[3] - b[3]) < 1e-12 for a, b in zip(five, lin))
    assert abs(five[at(5, 6)][0] - lin[at(5, 6)][0]) > 1e-3
    assert c["FX-AE32-006"] == B.plain()


if __name__ == "__main__":
    main()
