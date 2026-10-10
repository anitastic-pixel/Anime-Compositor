"""Ellipse, worked a second way.

D-415 adds `core.ellipse`, After Effects' Ellipse (Generate): "The Ellipse effect draws an
ellipse" (Adobe's After Effects user manual, page 514 of the copy on manualsdir.com, which says
nothing more). Its settings are After Effects': Center, Width, Height, Thickness, Softness, Inside
Color, Outside Color and Composite On Original. The manual gives no formula; the numbers below
are this program's own rule, nothing is ported. The outline is drawn as Beam's line is (D-207,
`tools/beam_reference.py`): Inside Color along its middle, Outside Color at its edges.

The rule, at a pixel of the layer's buffer whose centre is X = (x, y) in the drawing's own
pixels (the drawing's top-left corner (0, 0), however far an effect above grew the buffer):

1. C is the centre, in per cent of the drawing's own width and height; a = width / 2 and
   b = height / 2 the half axes. With (u, v) = X - C, k = sqrt(u^2 / a^2 + v^2 / b^2) and
   g = sqrt(u^2 / a^4 + v^2 / b^4), the distance to the outline is d = |k - 1| k / g, its first
   order (exact for a circle); at the centre, where g is 0, d = min(a, b).
2. As Beam's: r = thickness / 2, w = max(2 r softness / 100, 1), the covering
   c = clamp((min(d + w / 2, r) - max(d - w / 2, -r)) / w, 0, 1), and the colour
   L = (1 - q) inside + q outside, both linear, q = clamp(d / r, 0, 1) (1 when r = 0).
3. Composite On Original on: the layer's pixel times 1 - c plus (L c, c); off: (L c, c), the
   outline alone.

`center` -1000 to 1000 per cent, keyable, (50, 50) when added; `width` and `height` 1 to 10000
pixels, keyable, 200; `thickness` 0 to 10000 pixels, keyable, 8; `softness` 0 to 100 per cent,
keyable, 50; `inside_color` `#rrggbb`, white, and `outside_color`, #3c8cff, read in small letters;
`composite` `on` (when added) or `off`. Width, height and thickness are distances and a draft
halves them, width and height held at least 1. The layer never grows.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 by 10 holding Gradient's cel, the same size, unmoved unless the
case says. The expected frames are in `Fixtures/ellipse/expected_ellipse.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/ellipse_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as RC  # noqa: E402
import checkerboard_reference as K  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from gradient_reference import DRAWINGS  # noqa: E402

W, H = K.W, K.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "ellipse"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"center": (-1000, 1000), "width": (1, 10000), "height": (1, 10000),
          "thickness": (0, 10000), "softness": (0, 100)}
WORDS = ("inside_color", "outside_color", "composite")
NAMES = tuple(RANGES) + WORDS
VIOLET, BLUE, ORANGE = K.VIOLET, "#3c8cff", "#ff8000"


# --- the rule -------------------------------------------------------------------------------

def distance(u, v, a, b):
    g = math.sqrt(u * u / a ** 4 + v * v / b ** 4)
    if g == 0:
        return min(a, b)
    k = math.sqrt(u * u / (a * a) + v * v / (b * b))
    return abs(k - 1) * k / g


def outline(n, x, y):
    cx, cy = n["center"][0] / 100 * W, n["center"][1] / 100 * H
    d = distance(x - cx, y - cy, n["width"] / 2, n["height"] / 2)
    r = n["thickness"] / 2
    w = max(2 * r * n["softness"] / 100, 1)
    c = min(max((min(d + w / 2, r) - max(d - w / 2, -r)) / w, 0.0), 1.0)
    q = 1.0 if r == 0 else min(d / r, 1.0)
    return c, q


def ellipse(layer, n, inside, outside, composite):
    i_ = [srgb_to_linear(v / 255) for v in RC.hex_color(inside.lower())]
    o_ = [srgb_to_linear(v / 255) for v in RC.hex_color(outside.lower())]
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            c, q = outline(n, layer["left"] + i + 0.5, layer["top"] + j + 0.5)
            L = [(1 - q) * i_[ch] + q * o_[ch] for ch in range(3)]
            keep = 1 - c if composite == "on" else 0.0
            px.append([p[ch] * keep + L[ch] * c for ch in range(3)] + [p[3] * keep + c])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(center=(50, 50), width=200, height=200, thickness=8, softness=50,
         inside_color="#ffffff", outside_color=BLUE, composite="on", shift=0, tile=False):
    return {"drawing": "cel", "center": center, "width": width, "height": height,
            "thickness": thickness, "softness": softness, "inside_color": inside_color,
            "outside_color": outside_color, "composite": composite, "shift": shift, "tile": tile}


def ring(**kw):
    """The small frame's own ellipse: 12 by 8 about the middle, 2 thick, sharp."""
    return case(**dict(dict(width=12, height=8, thickness=2, softness=0), **kw))


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    if isinstance(v, (list, tuple)):
        return [min(hi, max(lo, e)) for e in v]
    return min(hi, max(lo, v))


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    out = ellipse(K.layer_of(c), n, c["inside_color"], c["outside_color"], c["composite"])
    return frame(out, c["shift"])


def plain(c):
    return frame(K.layer_of(c), c["shift"])


CASES = {
    "FX-ELLIPSE-001": ("The settings as they start: centre in the middle, 200 by 200, thickness 8, "
                       "softness 50, white inside, blue outside, composited: the outline lies far "
                       "outside the small frame, the cel as it was.", case(), [0]),
    "FX-ELLIPSE-002": ("12 by 8, thickness 2, softness 0: a sharp outline 2 pixels thick round the "
                       "middle over the cel, white along its middle, blue at its edges.", ring(),
                       [0]),
    "FX-ELLIPSE-003": ("FX-ELLIPSE-002 with Composite On Original off: the outline alone, the rest "
                       "clear.", ring(composite="off"), [0]),
    "FX-ELLIPSE-004": ("FX-ELLIPSE-003 at softness 100: the whole width of the outline ramps.",
                       ring(composite="off", softness=100), [0]),
    "FX-ELLIPSE-005": ("10 by 10, thickness 2, alone: a circle of radius 5, the distance exact.",
                       ring(width=10, height=10, composite="off"), [0]),
    "FX-ELLIPSE-006": ("Thickness 4, softness 50, alone.",
                       ring(thickness=4, softness=50, composite="off"), [0]),
    "FX-ELLIPSE-007": ("Orange inside, violet outside, thickness 4, alone: orange along the middle, "
                       "violet at the edges.", ring(thickness=4, inside_color=ORANGE,
                                                    outside_color=VIOLET, composite="off"), [0]),
    "FX-ELLIPSE-008": ("Thickness 0: nothing drawn, the cel as it was.", ring(thickness=0), [0]),
    "FX-ELLIPSE-009": ("8 by 6 alone, the centre keyed from (50, 50) at frame 0 to (25, 50) at "
                       "frame 4, linear: the outline slides left, 1 pixel a frame.",
                       ring(width=8, height=6, composite="off",
                            center=keyed((0, (50, 50)), (4, (25, 50)))), [0, 2, 4]),
    "FX-ELLIPSE-010": ("Width keyed from 8 at frame 0 to 16 at frame 4, linear, alone: frame 2 is "
                       "12 across.", ring(width=keyed((0, 8), (4, 16)), composite="off"),
                       [0, 2, 4]),
    "FX-ELLIPSE-011": ("FX-ELLIPSE-002 moved three pixels right: the outline moves with the "
                       "layer.", ring(shift=3), [0]),
    "FX-ELLIPSE-012": ("After a Motion Tile that grows the layer: the centre is the drawing's own, "
                       "so the frame is FX-ELLIPSE-002's.", ring(tile=True), [0]),
    "FX-ELLIPSE-013": ("FX-ELLIPSE-007 with its colours in capitals, #FF8000 and #6450A0: the "
                       "same.", ring(thickness=4, inside_color="#FF8000", outside_color="#6450A0",
                                     composite="off"), [0]),
    "FX-ELLIPSE-014": ("20 by 10, the centre at (-25, 50) per cent, outside the drawing, alone: "
                       "the outline's right end, at x 6, reaches into the frame's left side.",
                       ring(width=20, height=10, center=(-25, 50), composite="off"), [0]),
    "FX-ELLIPSE-015": ("Thickness keyed from 2 at frame 0 to 0 at frame 4 past its end by an ease: "
                       "held at 0, frame 4 the cel as it was.",
                       ring(thickness=keyed((0, 2, OVERSHOOT), (4, 0))), [0, 4]),
    "FX-ELLIPSE-016": ("4 by 10, tall, alone: the outline narrow across, long down.",
                       ring(width=4, height=10, composite="off"), [0]),
    "FX-ELLIPSE-017": ("Thickness 1, softness 0, alone: a hairline, each pixel covered by how "
                       "much of it lies within half a pixel of the outline.",
                       ring(thickness=1, composite="off"), [0]),
    "FX-ELLIPSE-018": ("Width 1, height 1, thickness 2, alone: a dot at the middle.",
                       ring(width=1, height=1, composite="off"), [0]),
}

INVALID = {
    "FX-ELLIPSE-019": ("Width 0, below 1.", case(width=0)),
    "FX-ELLIPSE-020": ("Height 10001, above 10000.", case(height=10001)),
    "FX-ELLIPSE-021": ("Thickness -1, below 0.", case(thickness=-1)),
    "FX-ELLIPSE-022": ("Softness 101, above 100.", case(softness=101)),
    "FX-ELLIPSE-023": ("Centre 1001 per cent across, above 1000.", case(center=(1001, 50))),
    "FX-ELLIPSE-024": ("Composite \"yes\", not on or off.", case(composite="yes")),
    "FX-ELLIPSE-025": ("Inside colour \"#12345\", not six hex digits.", case(inside_color="#12345")),
    "FX-ELLIPSE-026": ("Outside colour \"red\", not #rrggbb.", case(outside_color="red")),
    "FX-ELLIPSE-027": ("Width keyed to 10001 at frame 4, above 10000.",
                       case(width=keyed((0, 200), (4, 10001)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = K.project_json(fx, dict(K.case(), shift=c["shift"], tile=c["tile"]))
    effects = p["compositions"][0]["layers"][0]["effects"]
    effects[-1] = {"instance_id": effects[-1]["instance_id"], "type_id": "core.ellipse",
                   "enabled": True,
                   "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                  for k in NAMES}}
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
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
    (OUT / "expected_ellipse.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    clear = [0.0, 0.0, 0.0, 0.0]
    blue = [srgb_to_linear(v / 255) for v in (0x3c, 0x8c, 0xff)]
    art = plain(case())
    assert c["FX-ELLIPSE-001"]["0"] == art == c["FX-ELLIPSE-008"]["0"]
    two, alone = c["FX-ELLIPSE-002"]["0"], c["FX-ELLIPSE-003"]["0"]
    # composited is the cel under the outline alone
    assert all(near(two[i], [art[i][k] * (1 - alone[i][3]) + alone[i][k] for k in range(4)])
               for i in range(W * H))
    assert two != art and any(p == clear for p in alone) and any(p[3] == 1 for p in alone)
    # on the axes the distance is exact: (14.5, 5.5) is 0.5 past the right end (14, 5), d 0.52
    for x, y in ((14, 5), (8, 1), (13, 5)):
        u, v = x + 0.5 - 8, y + 0.5 - 5
        d = distance(u, v, 6, 4)
        cov = min(max(min(d + 0.5, 1) - max(d - 0.5, -1), 0), 1)
        q = min(d, 1)
        assert near(alone[at(x, y)], [((1 - q) + q * blue[k]) * cov for k in range(3)] + [cov])
    assert near(alone[at(8, 5)], clear) and near(alone[at(0, 0)], clear)
    soft = c["FX-ELLIPSE-004"]["0"]
    assert soft != alone and all(p[3] <= 1 for p in soft)
    five = c["FX-ELLIPSE-005"]["0"]
    for y in range(H):
        for x in range(W):
            d = abs(math.hypot(x + 0.5 - 8, y + 0.5 - 5) - 5)
            cov = min(max(min(d + 0.5, 1) - max(d - 0.5, -1), 0), 1)
            assert abs(five[at(x, y)][3] - cov) < 1e-12, (x, y)
    seven = c["FX-ELLIPSE-007"]["0"]
    assert c["FX-ELLIPSE-013"]["0"] == seven
    mid = seven[at(14, 5)]  # d 0.52 of r 2: mostly orange
    assert abs(mid[3] - 1) < 1e-12 and mid[0] > mid[2]
    slide = c["FX-ELLIPSE-009"]
    assert all(slide["4"][at(x, y)] == slide["0"][at(x + 4, y)] for x in range(W - 4)
               for y in range(H))
    grow = c["FX-ELLIPSE-010"]
    assert grow["2"] == alone and grow["0"] == render(ring(width=8, composite="off"), 0)
    moved = c["FX-ELLIPSE-011"]["0"]
    assert all(moved[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert c["FX-ELLIPSE-012"]["0"] == two
    far = c["FX-ELLIPSE-014"]["0"]
    assert far[at(5, 5)][3] > 0 and near(far[at(0, 5)], clear)
    assert near(far[at(W - 1, 5)], clear)
    ease = c["FX-ELLIPSE-015"]
    assert ease["0"] == two and ease["4"] == art
    tall = c["FX-ELLIPSE-016"]["0"]
    assert tall[at(8, 0)][3] > 0 and near(tall[at(1, 5)], clear)
    hair = c["FX-ELLIPSE-017"]["0"]
    assert all(0 <= p[3] <= 1 for p in hair) and any(p[3] > 0 for p in hair)
    dot = c["FX-ELLIPSE-018"]["0"]
    assert dot[at(7, 4)][3] > 0 and near(dot[at(0, 0)], clear)
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


if __name__ == "__main__":
    main()
