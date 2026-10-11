"""Halation, worked a second way.

D-544 is pick #13 in `docs/effects/EFFECTS.md` (row 260), "Halation": warm red-orange light
bleeding from bright edges, as film does when light passes through the emulsion, bounces off the
film's base and comes back to expose the red layer around the highlight. PLUGINS.md settles that
it is an effect of its own ("Build separately, small, on the shared soft-glow engine", the
ranked list's "13 | Halation | Separate"). The rule below is our own; no plugin code or presets
are used.

`core.halation`, "Halation", has five settings:

* `threshold`, 0 to 100, default 60, and `threshold_smooth`, 0 to 100, default 50: which light
  makes halation, tested on its Rec. 709 brightness (Soft Physical Glow's Luminance mode).
* `radius`, 0 to 2,000 pixels, default 25: how far the halation spreads.
* `amount`, 0 to 400 per cent, default 100: how strong it is.
* `color`, a colour `#rrggbb`, default `#ff4020` (red-orange): its colour.

The rule, on D-353's engine (`tools/soft_glow_reference.py`):

1. The light: D-353's stage 1 in Luminance mode, with no Saturation Bias.
2. The spread: D-353's levels at Radius (Aspect Ratio 1, no angle), the light's own share
   included, so the spread of an even light is that light.
3. The halation at a pixel: h = max(0, Y(spread) - Y(light at the pixel)), Y the Rec. 709
   brightness, the light 0 outside the layer. Inside a bright area the spread is no brighter
   than the light, so nothing is added there; the halation lies only where the light spreads
   onto something darker than itself. That is what makes it hug the bright edges and is what
   sets it apart from a tinted glow.
4. Laid on: h x the colour (linear) x Amount / 100, laid with the layer as Soft Physical Glow
   lays its glow with Blend Mode Screen, Unmult on and Source Opacity 100: the halation held at
   1, added to the layer and held at 1; its covering its brightest channel.

The layer grows by the spread's reach, so no halation is cut off (no silent crop).

**This file never runs the build's code path.** It works in double precision with numpy, on
D-353's padded planes of cells (`tools/spectral_glow_reference.py`'s glow sum, untinted); the
build works in single precision, pixel by pixel.

Every case is a composition 16 pixels by 10 holding one drawing the same size, the drawings of
`tools/soft_glow_reference.py`. The drawings go into `Fixtures/halation/media`, the projects into
`Fixtures/halation`, and the expected frames into `Fixtures/halation/expected_halation.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/halation_reference.py
"""

import json
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import soft_glow_reference as G  # noqa: E402
import glow_tint_reference as T  # noqa: E402
import spectral_glow_reference as SP  # noqa: E402

W, H = G.W, G.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "halation"
TOLERANCE = 2e-5  # document 25's default for a filter
MAX_RADIUS, MAX_AMOUNT = 2000, 400
RED_ORANGE = "#ff4020"
PARAMS = ("threshold", "threshold_smooth", "radius", "amount", "color")


# --- the rule -------------------------------------------------------------------------------

def luma(rgb):
    return 0.2126 * rgb[..., 0] + 0.7152 * rgb[..., 1] + 0.0722 * rgb[..., 2]


def engine(c):
    """The settings D-353's stages read."""
    return {"threshold_mode": "luminance", "threshold": c["threshold"],
            "threshold_smooth": c["threshold_smooth"], "saturation_bias": 0,
            "radius": c["radius"], "aspect_ratio": 1, "aspect_angle": 0, "tint": "off",
            "inner_color": "#ffffff", "outer_color": "#ffffff", "tint_amount": 0}


def halation_layer(px, c, xs, ys):
    e = engine(c)
    lit = G.light(px, e)
    spread = SP.glow_sum(px, e, xs, ys, c["radius"])
    here = np.zeros((len(ys), len(xs), 3))
    for jy, y in enumerate(ys):
        for ix, x in enumerate(xs):
            if 0 <= x < W and 0 <= y < H:
                here[jy, ix] = lit[y, x]
    h = np.maximum(0.0, luma(spread) - luma(here))
    g = np.minimum(h[..., None] * T.linear(c["color"]) * (c["amount"] / 100), 1.0)
    ga = np.minimum(1.0, g.max(axis=-1))
    out = np.zeros((len(ys), len(xs), 4))
    for jy, y in enumerate(ys):
        for ix, x in enumerate(xs):
            s = px[y, x] if (0 <= x < W and 0 <= y < H) else np.zeros(4)
            out[jy, ix, :3] = np.minimum(g[jy, ix] + s[:3], 1.0)
            out[jy, ix, 3] = min(1.0, ga[jy, ix] + s[3])
    return out


def settings_at(c, frame_no):
    out = dict(c)
    for k, top in (("threshold", 100), ("threshold_smooth", 100), ("radius", MAX_RADIUS),
                   ("amount", MAX_AMOUNT)):
        out[k] = min(top, max(0, float(value_at(c[k], frame_no))))
    return out


def render(c, frame_no):
    s = settings_at(c, frame_no)
    out = halation_layer(G.pixels(c["drawing"]), s, [x - c["shift"] for x in range(W)],
                         list(range(H)))
    return [[float(v) for v in out[y, x]] for y in range(H) for x in range(W)]


# --- the cases ------------------------------------------------------------------------------

def case(name="patches", threshold=60, threshold_smooth=50, radius=12, amount=100,
         color=RED_ORANGE, shift=0):
    return {"drawing": name, "threshold": threshold, "threshold_smooth": threshold_smooth,
            "radius": radius, "amount": amount, "color": color, "shift": shift}


CASES = {
    "FX-HALO-001": ("Halation as added (threshold 60, smooth 50, amount 100, red-orange "
                    "#ff4020), radius 12: the yellow patch, the only light bright enough, gets a "
                    "red-orange halo round it, strongest at its edge; the yellow itself is "
                    "untouched.",
                    case(), [0]),
    "FX-HALO-002": ("Threshold 0: every pixel's light counts, so each patch spreads onto "
                    "anything darker beside it: halation round all three patches and on the "
                    "darker patches' near edges.",
                    case(threshold=0), [0]),
    "FX-HALO-003": ("Radius 0: nothing spreads, so there is no halation: the drawing as it is.",
                    case(radius=0), [0]),
    "FX-HALO-004": ("Radius 100: the halation spread wide and faint.",
                    case(radius=100), [0]),
    "FX-HALO-005": ("Amount 300: three times as strong as FX-HALO-001 off the drawing.",
                    case(amount=300), [0]),
    "FX-HALO-006": ("Amount 0: no halation: the drawing as it is.",
                    case(amount=0), [0]),
    "FX-HALO-007": ("Colour white: the same halo, colourless.",
                    case(color="#ffffff"), [0]),
    "FX-HALO-008": ("Amount keyed from 0 at frame 0 to 200 at frame 4, linear: frame 0 is the "
                    "drawing, frame 2 is FX-HALO-001.",
                    case(amount=keyed((0, 0), (4, 200))), [0, 2, 4]),
    "FX-HALO-009": ("The grey drawing: the grey patch is bright enough to give some halation "
                    "too.",
                    case("grey"), [0]),
    "FX-HALO-010": ("The patches half covering: the light counts at half, and the halation "
                    "outside the drawing covers by its brightest channel.",
                    case("faint", threshold=30), [0]),
    "FX-HALO-011": ("Moved three pixels right: the halation follows, and none is cut off at the "
                    "layer's edge.",
                    case(shift=3), [0]),
    "FX-HALO-012": ("Threshold 30, smooth 0: a hard step: the yellow glows fully and nothing "
                    "else.",
                    case(threshold=30, threshold_smooth=0), [0]),
    "FX-HALO-013": ("Radius 2.25: one level, half in: a thin halo one pixel wide.",
                    case(radius=2.25), [0]),
    "FX-HALO-014": ("Threshold 90, smooth 50: the yellow only partly bright enough, a weaker "
                    "halo.",
                    case(threshold=90), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-HALO-015": ("Threshold 101, above 100.", case(threshold=101)),
    "FX-HALO-016": ("Radius -1, below 0.", case(radius=-1)),
    "FX-HALO-017": ("Colour \"orange\", not a colour #rrggbb.", case(color="orange")),
    "FX-HALO-018": ("Amount keyed to 500 at frame 4, above 400.",
                    case(amount=keyed((0, 100), (4, 500)))),
}


# --- the project files ----------------------------------------------------------------------

def write(fx, c):
    p = G.project_json(fx, G.case(c["drawing"], shift=c["shift"]))
    p["compositions"][0]["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.halation", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in PARAMS}}]
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, px in G.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(px))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        base = G.plain(c["drawing"], c["shift"])
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(max(abs(a - b) for a, b in zip(px[i], base[i])) > 1e-9 for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        base = G.plain(c["drawing"])
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": base, "4": base},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_halation.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(a - b) <= e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g, e=1e-12: all(close(p, q, e) for p, q in zip(f, g))  # noqa: E731
    drawn = G.plain("patches")

    one = c["FX-HALO-001"]["0"]
    # The yellow (columns 2 to 4, rows 3 to 6) is untouched; the halo lies outside it.
    for y in range(3, 7):
        for x in range(2, 5):
            assert one[at(x, y)] == drawn[at(x, y)]
    # Beside the yellow, on nothing: red-orange, red above green above blue, strongest nearest.
    p, q = one[at(5, 4)], one[at(6, 4)]
    assert p[0] > p[1] > p[2] > 0 and q[0] < p[0]
    # Above the yellow too (it spreads every way).
    assert one[at(3, 2)][0] > 0
    # Far off, at column 15, it is faint.
    assert one[at(15, 4)][0] < 0.05
    # 003, 006 and 008's frame 0 are the drawing.
    assert same(c["FX-HALO-003"]["0"], drawn, 0)
    assert same(c["FX-HALO-006"]["0"], drawn, 0)
    assert same(c["FX-HALO-008"]["0"], drawn, 0)
    assert same(c["FX-HALO-008"]["2"], one, 1e-12)
    # 002: the gap between the brown and purple patches, which the brown lights only with
    # threshold 0, gets halation.
    assert c["FX-HALO-002"]["0"][at(9, 4)][0] > one[at(9, 4)][0]
    assert c["FX-HALO-002"]["0"][at(9, 4)][0] > 0
    # 005: three times 001 off the drawing.
    for x in (5, 6, 9):
        assert close(c["FX-HALO-005"]["0"][at(x, 4)][:3], [3 * v for v in one[at(x, 4)][:3]], 1e-12)
    # 007: white halation is grey off the drawing.
    p = c["FX-HALO-007"]["0"][at(5, 4)]
    assert abs(p[0] - p[1]) < 1e-12 and abs(p[1] - p[2]) < 1e-12 and p[0] > 0
    # 004: wider and fainter at the yellow's edge than 001, further out stronger.
    assert c["FX-HALO-004"]["0"][at(5, 4)][0] < one[at(5, 4)][0]
    assert c["FX-HALO-004"]["0"][at(15, 9)][0] > one[at(15, 9)][0]
    # 009: the grey patch gives halation that the brown did not: beside it, more light.
    assert c["FX-HALO-009"]["0"][at(9, 2)][0] > one[at(9, 2)][0]
    # 011: moved three right, the halation is the same picture moved.
    for y in range(H):
        for x in range(3, W):
            assert close(c["FX-HALO-011"]["0"][at(x, y)], one[at(x - 3, y)], 1e-12)
    # 014: weaker than 001.
    assert c["FX-HALO-014"]["0"][at(5, 4)][0] < one[at(5, 4)][0]
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(-1e-12 <= v <= 1 for v in p[:3]), (name, p)


if __name__ == "__main__":
    main()
