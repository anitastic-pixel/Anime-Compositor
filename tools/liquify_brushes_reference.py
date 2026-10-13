"""Liquify's other brushes, freeze areas and earlier strokes, worked a second way.

D-700 closes the gaps D-605's Liquify (`tools/liquify_reference.py`) left against After Effects'
Liquify (Distort): the brushes it lacked, areas frozen against the brush, and more than one
stroke. After Effects names its tools and publishes no formula; the rule below is this
program's own, and nothing is ported. Every D-605 setting, case and expected value stands: with
the new settings as they start, the rule below is D-605's.

The new settings.

- `tool` takes six more words after D-605's six, numbered on from them: `turbulence` (6),
  `reflection` (7), `clone` (8), `reconstruction` (9), `freeze_area` (10) and `thaw_area` (11).
  D-605's six are 0 to 5 in its order: `warp`, `twirl_clockwise`, `twirl_counterclockwise`,
  `pucker`, `bloat`, `shift_pixels`. (The bare word `freeze` stays refused, as FX-LIQUIFY-016
  has it.)
- `turbulent_jitter`, how hard Turbulence scrambles, 1 to 100, 70 when added; keyable.
- `clone_offset`, where Clone copies from, x then y in pixels from the brush, -10000 to 10000,
  (0, 0) when added; keyable.
- `reconstruction_mode`, what Reconstruction brushes back toward: `revert` (when added),
  `displace`, `amplitwist` or `affine`.
- `freeze_area_mask`, a mask whose inside the brush never moves, 0 to 1000, the mask numbered
  so (1 the first, its floor taken), 0 for none (when added); keyable. The mask is chosen as
  Reshape's are (D-606): one of the layer's masks at the frame, flattened as document 19 does
  it, counting when it is on and has two points or more; a mask named and not found draws the
  drawing as it is and says EFFECT_PATH_MISSING every frame.
- `strokes`, the strokes brushed before the one `stroke` holds, oldest first: none when added,
  or for each a list of numbers: its tool's number, its brush size, brush pressure, turbulent
  jitter, clone offset x and y, its reconstruction mode's number (0 `revert`, 1 `displace`,
  2 `amplitwist`, 3 `affine`), then x then y for each of its points, one point or more, per
  cent of the drawing. Each number is held to its setting's range above (a tool's and a mode's
  numbers whole); a list breaking one leaves the effect out with EFFECT_PARAMETER_INVALID. Not
  keyable. The last stroke, `stroke` and the settings beside it, stays keyable as D-605 has it.

The rule. The grid, the dabs along a stroke, f and the steps of D-605's six tools are D-605's.
Each corner p of the grid carries a step S(p) and a freeze Z(p), both 0 to begin with. The
strokes are brushed in turn, `strokes` in order and then `stroke` (none when it has no points),
each with its own settings. As a stroke begins, the frozen share at p is F(p) = 1 when p lies
inside the freeze area mask (ADR-016's even-odd rule) and Z(p) otherwise.

- D-605's six tools and Turbulence add every dab's step, times (1 - F(p)), to S(p).
  Turbulence's step is a f J R (n(p / R, 0), n(p / R, 1)), a = 1/10, J = jitter / 100: the
  value noise n(x, y, k) blends the lattice values v = 2 h - 1 at the four whole corners round
  (x, y) with weights s(u) = u^2 (3 - 2 u) on the fractions u across and down, h(i, j, k) the
  32-bit hash of the corner: h = (73856093 i) xor (19349663 j) xor (83492791 k), each product
  taken modulo 2^32; h = h xor (h >> 13); h = (h 1540483477) mod 2^32; h = h xor (h >> 15);
  h / 2^32.
- Reflection, Clone and Reconstruction pull S(p) toward a step T(p), dab by dab in order:
  S(p) = S(p) + g (T(p) - S(p)), g = f (1 - F(p)).
  - Reflection: the drawing on the left of the way the dab moves, mirrored onto its right
    (y runs down, so on a stroke going right the right is below). With u = d / |d| and
    q = p - c, a corner where u_x q_y - u_y q_x > 0 takes T = c + 2 (q . u) u - q - p, the
    step that reads it from its mirror across the stroke's line; other corners, and a dab
    that does not move (d = 0), are left as they are.
  - Clone: T(p) = S(p + o), o `clone_offset`, read as D-601 reads a field (the four corners
    round it blended, held in the grid) from the steps as they were before the dab: the
    distortion copied from o away.
  - Reconstruction: back toward the distortion as the stroke found it at its first point a.
    With A = S(a) and G the change of S across and down at a (S(a +- (W / gx, 0)) and
    S(a +- (0, H / gy)), differences over twice the cell), both read as Clone reads from the
    steps as the stroke began: `revert` T = 0; `displace` T = A; `affine` T = A + G (p - a);
    `amplitwist` T = A + (C - I)(p - a), C the turn and even scale nearest I + G
    (C = [[al, -be], [be, al]], al = (M_xx + M_yy) / 2, be = (M_yx - M_xy) / 2, M = I + G).
- Freeze Area adds f (1 - Z(p)) to Z(p) for each dab; Thaw Area takes f Z(p) from it. Neither
  moves the drawing.

The frame is then drawn from S as D-605 draws it. With no points in `stroke` and no `strokes`,
or `distortion_percentage` 0, the drawing is untouched.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/liquify_brushes/media`, the
projects into `Fixtures/liquify_brushes`, and the expected frames into
`Fixtures/liquify_brushes/expected_liquify_brushes.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/liquify_brushes_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, OVERSHOOT  # noqa: E402
import bulge_reference as B  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
import mesh_reference as M  # noqa: E402
import liquify_reference as L  # noqa: E402
import reshape_reference as RS  # noqa: E402
from mask_reference import flatten, points_at, points, point_inside  # noqa: E402

W, H = L.W, L.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "liquify_brushes"
TOLERANCE = 2e-5  # document 25's default for a filter
TOOLS = L.TOOLS + ("turbulence", "reflection", "clone", "reconstruction", "freeze_area",
                   "thaw_area")
MODES = ("revert", "displace", "amplitwist", "affine")
NAMES = L.NAMES + ("turbulent_jitter", "clone_offset", "reconstruction_mode",
                   "freeze_area_mask", "strokes")
WORDS = ("strokes",)
RANGES = dict(L.RANGES, turbulent_jitter=(1, 100), clone_offset=(-10000, 10000),
              freeze_area_mask=(0, 1000))
MISSING_PATH = "missing"


# --- the rule -------------------------------------------------------------------------------

def hash01(i, j, k):
    m = 0xFFFFFFFF
    h = ((73856093 * i) & m) ^ ((19349663 * j) & m) ^ ((83492791 * k) & m)
    h ^= h >> 13
    h = (h * 1540483477) & m
    h ^= h >> 15
    return h / 4294967296.0


def noise(x, y, k):
    i, j = math.floor(x), math.floor(y)
    u, v = x - i, y - j
    su, sv = u * u * (3 - 2 * u), v * v * (3 - 2 * v)
    val = lambda a, b: 2 * hash01(a, b, k) - 1  # noqa: E731
    top = val(i, j) + su * (val(i + 1, j) - val(i, j))
    bottom = val(i, j + 1) + su * (val(i + 1, j + 1) - val(i, j + 1))
    return top + sv * (bottom - top)


def stroke_list(s_):
    """Every stroke as (settings, points), oldest first."""
    out = []
    for e in s_["strokes"]:
        out.append(({"tool": TOOLS[int(e[0])], "brush_size": e[1], "brush_pressure": e[2],
                     "turbulent_jitter": e[3], "clone_offset": (e[4], e[5]),
                     "reconstruction_mode": MODES[int(e[6])]}, list(e[7:])))
    if s_["stroke"]:
        out.append((s_, list(s_["stroke"])))
    return out


def field(s_, mask):
    gx, gy = min(256, math.ceil(W / 2)), min(256, math.ceil(H / 2))
    corners = [(i * W / gx, j * H / gy) for j in range(gy + 1) for i in range(gx + 1)]
    S = [(0.0, 0.0)] * len(corners)
    Z = [0.0] * len(corners)
    inside = [mask is not None and point_inside(mask, x, y) for x, y in corners]
    read = lambda field, x, y: M.field_at(field, gx, gy, W, H, x, y)  # noqa: E731
    for st, pts in stroke_list(s_):
        tool, reach, pressure = st["tool"], st["brush_size"] / 2, st["brush_pressure"] / 100
        landed = L.dabs(pts, reach)
        F = [1.0 if inside[n] else Z[n] for n in range(len(corners))]
        fall = lambda p, c: (lambda t: pressure * (1 - t * t) ** 2 if t < 1 else 0.0)(  # noqa: E731
            math.hypot(p[0] - c[0], p[1] - c[1]) / reach)
        if tool in L.TOOLS or tool == "turbulence":
            jitter = st["turbulent_jitter"] / 100
            for n, p in enumerate(corners):
                sx, sy = S[n]
                for c, d in landed:
                    f = fall(p, c)
                    if f == 0:
                        continue
                    if tool == "turbulence":
                        a = 0.1 * f * jitter * reach
                        stp = (a * noise(p[0] / reach, p[1] / reach, 0),
                               a * noise(p[0] / reach, p[1] / reach, 1))
                    else:
                        stp = L.step(tool, f, p, c, d)
                    sx += (1 - F[n]) * stp[0]
                    sy += (1 - F[n]) * stp[1]
                S[n] = (sx, sy)
        elif tool in ("freeze_area", "thaw_area"):
            for c, _ in landed:
                for n, p in enumerate(corners):
                    f = fall(p, c)
                    Z[n] = Z[n] + f * (1 - Z[n]) if tool == "freeze_area" else Z[n] - f * Z[n]
        else:
            if tool == "reconstruction":
                a = (pts[0] / 100 * W, pts[1] / 100 * H)
                A = read(S, *a)
                hx, hy = W / gx, H / gy
                gxv = [(read(S, a[0] + hx, a[1])[k] - read(S, a[0] - hx, a[1])[k]) / (2 * hx)
                       for k in range(2)]
                gyv = [(read(S, a[0], a[1] + hy)[k] - read(S, a[0], a[1] - hy)[k]) / (2 * hy)
                       for k in range(2)]
                mode = st["reconstruction_mode"]
                if mode == "amplitwist":
                    al = (1 + gxv[0] + 1 + gyv[1]) / 2
                    be = (gxv[1] - gyv[0]) / 2
                    lin = ((al - 1, -be), (be, al - 1))
                else:
                    lin = ((gxv[0], gyv[0]), (gxv[1], gyv[1]))
                target = lambda p: (0.0, 0.0) if mode == "revert" else A if mode == "displace" else (  # noqa: E731
                    A[0] + lin[0][0] * (p[0] - a[0]) + lin[0][1] * (p[1] - a[1]),
                    A[1] + lin[1][0] * (p[0] - a[0]) + lin[1][1] * (p[1] - a[1]))
            for c, d in landed:
                before = list(S)
                for n, p in enumerate(corners):
                    f = fall(p, c)
                    if f == 0:
                        continue
                    if tool == "reflection":
                        ln = math.hypot(d[0], d[1])
                        if ln == 0:
                            continue
                        u = (d[0] / ln, d[1] / ln)
                        q = (p[0] - c[0], p[1] - c[1])
                        if u[0] * q[1] - u[1] * q[0] <= 0:
                            continue
                        dot = q[0] * u[0] + q[1] * u[1]
                        T = (c[0] + 2 * dot * u[0] - q[0] - p[0], c[1] + 2 * dot * u[1] - q[1] - p[1])
                    elif tool == "clone":
                        o = st["clone_offset"]
                        T = read(before, p[0] + o[0], p[1] + o[1])
                    else:
                        T = target(p)
                    g = f * (1 - F[n])
                    S[n] = (S[n][0] + g * (T[0] - S[n][0]), S[n][1] + g * (T[1] - S[n][1]))
    return S, gx, gy


def liquify(layer, s_, mask=None):
    if (not s_["stroke"] and not s_["strokes"]) or s_["distortion_percentage"] == 0:
        return layer
    steps, gx, gy = field(s_, mask)
    return M.draw_field(layer, steps, gx, gy, W, H, s_["distortion_percentage"] / 100,
                        tuple(s_["distortion_mesh_offset"]))


# --- the cases ------------------------------------------------------------------------------

ACROSS, DOT = L.ACROSS, L.DOT
DIAGONAL = [25, 20, 75, 80]
SQUARE = points([4, 2], [12, 2], [12, 8], [4, 8])


def entry(tool, brush_size, brush_pressure, pts, jitter=70, clone=(0, 0), mode="revert"):
    return [TOOLS.index(tool), brush_size, brush_pressure, jitter, clone[0], clone[1],
            MODES.index(mode)] + list(pts)


PUSH = entry("warp", 8, 50, ACROSS)
TWIRL = entry("twirl_clockwise", 10, 100, DOT)
SHEAR = entry("shift_pixels", 8, 60, [50, 10, 50, 90])


def case(shift=0, tool="warp", brush_size=8, brush_pressure=50, stroke=ACROSS,
         distortion_percentage=100, distortion_mesh_offset=(0, 0), turbulent_jitter=70,
         clone_offset=(0, 0), reconstruction_mode="revert", freeze_area_mask=0, strokes=(),
         masks=()):
    c = L.case(shift, tool, brush_size, brush_pressure, stroke, distortion_percentage,
               distortion_mesh_offset)
    return dict(c, turbulent_jitter=turbulent_jitter,
                clone_offset=list(clone_offset) if isinstance(clone_offset, tuple) else clone_offset,
                reconstruction_mode=reconstruction_mode, freeze_area_mask=freeze_area_mask,
                strokes=[list(e) for e in strokes], masks=list(masks))


def settings(c, frame_no):
    held = {"tool": c["tool"], "reconstruction_mode": c["reconstruction_mode"],
            "strokes": c["strokes"]}
    for k, (lo, hi) in RANGES.items():
        v = value_at(c[k], frame_no)
        held[k] = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def chosen(c, frame_no):
    """The freeze area mask's outline; None for 0; MISSING_PATH when it is not there."""
    i = math.floor(settings(c, frame_no)["freeze_area_mask"]) - 1
    if i < 0:
        return None
    if i >= len(c["masks"]):
        return MISSING_PATH
    m = c["masks"][i]
    pts = points_at(m, frame_no)
    if not m.get("enabled", True) or len(pts) < 2:
        return MISSING_PATH
    return flatten(pts)


def render(c, frame_no):
    mask = chosen(c, frame_no)
    layer = B.drawn_layer(c["drawing"])
    if mask != MISSING_PATH:
        layer = liquify(layer, settings(c, frame_no), mask)
    return frame(layer, c["shift"])


def plain(c):
    return frame(B.drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-LIQUIFY-BRUSHES-001": ("Turbulence, brush 8, pressure 50, jitter 70, stroked across the "
                               "middle: the stripes there scrambled smoothly.",
                               case(tool="turbulence"), [0]),
    "FX-LIQUIFY-BRUSHES-002": ("FX-LIQUIFY-BRUSHES-001 at jitter 20: scrambled less.",
                               case(tool="turbulence", turbulent_jitter=20), [0]),
    "FX-LIQUIFY-BRUSHES-003": ("Reflection, brush 8, pressure 100, stroked from the top left "
                               "to the bottom right: below the line, the stripes above it "
                               "mirrored.",
                               case(tool="reflection", brush_pressure=100, stroke=DIAGONAL), [0]),
    "FX-LIQUIFY-BRUSHES-004": ("A Warp across the middle first, then Clone, brush 6, pressure "
                               "100, one dab near the top with Clone Offset 3 pixels down: the "
                               "push copied up there.",
                               case(tool="clone", brush_size=6, brush_pressure=100,
                                    stroke=[50, 20], clone_offset=(0, 3), strokes=[PUSH]), [0]),
    "FX-LIQUIFY-BRUSHES-005": ("A Warp across the middle first, then Reconstruction (Revert), "
                               "brush 6, pressure 70, one dab in the middle: the push mostly "
                               "undone there.",
                               case(tool="reconstruction", brush_size=6, brush_pressure=70,
                                    stroke=DOT, strokes=[PUSH]), [0]),
    "FX-LIQUIFY-BRUSHES-006": ("A Twirl in the middle and a Shift Pixels down it first, then "
                               "Reconstruction (Displace) brushed from above the middle to the "
                               "right: the displacement where it starts carried along.",
                               case(tool="reconstruction", brush_size=6, brush_pressure=100,
                                    stroke=[50, 30, 85, 30], reconstruction_mode="displace",
                                    strokes=[TWIRL, SHEAR]), [0]),
    "FX-LIQUIFY-BRUSHES-007": ("FX-LIQUIFY-BRUSHES-006 with Amplitwist: the turn and swell "
                               "there carried along too.",
                               case(tool="reconstruction", brush_size=6, brush_pressure=100,
                                    stroke=[50, 30, 85, 30], reconstruction_mode="amplitwist",
                                    strokes=[TWIRL, SHEAR]), [0]),
    "FX-LIQUIFY-BRUSHES-008": ("FX-LIQUIFY-BRUSHES-006 with Affine: the whole local "
                               "distortion, slant and all.",
                               case(tool="reconstruction", brush_size=6, brush_pressure=100,
                                    stroke=[50, 30, 85, 30], reconstruction_mode="affine",
                                    strokes=[TWIRL, SHEAR]), [0]),
    "FX-LIQUIFY-BRUSHES-009": ("Freeze Area, brush 6, pressure 100, one dab in the middle, "
                               "then FX-LIQUIFY-002's Warp: the push held back in the middle.",
                               case(strokes=[entry("freeze_area", 6, 100, DOT)]), [0]),
    "FX-LIQUIFY-BRUSHES-010": ("FX-LIQUIFY-BRUSHES-009 with a Thaw Area at pressure 50 over "
                               "the freeze before the Warp: held back half as much.",
                               case(strokes=[entry("freeze_area", 6, 100, DOT),
                                             entry("thaw_area", 6, 50, DOT)]), [0]),
    "FX-LIQUIFY-BRUSHES-011": ("FX-LIQUIFY-002's Warp with Freeze Area Mask 1, a box from "
                               "(4, 2) to (12, 8): the drawing well inside it does not move.",
                               case(freeze_area_mask=1, masks=[RS.none(SQUARE)]), [0]),
    "FX-LIQUIFY-BRUSHES-012": ("FX-LIQUIFY-BRUSHES-011 with the layer moved three pixels "
                               "right: the mask is the layer's, so all of it moves with it.",
                               case(3, freeze_area_mask=1, masks=[RS.none(SQUARE)]), [0]),
    "FX-LIQUIFY-BRUSHES-013": ("Two strokes: a Warp across the middle, then Pucker, brush 10, "
                               "pressure 100, one dab in the middle: both drawn.",
                               case(tool="pucker", brush_size=10, brush_pressure=100,
                                    stroke=DOT, strokes=[PUSH]), [0]),
    "FX-LIQUIFY-BRUSHES-014": ("FX-LIQUIFY-002's Warp as an earlier stroke and no stroke "
                               "after it: FX-LIQUIFY-002's frame.",
                               case(stroke=[], strokes=[PUSH]), [0]),
    "FX-LIQUIFY-BRUSHES-015": ("Turbulence with the jitter keyed from 10 at frame 0 to 100 at "
                               "frame 4, linear: scrambled more as it goes.",
                               case(tool="turbulence", turbulent_jitter=keyed((0, 10), (4, 100))),
                               [0, 2, 4]),
    "FX-LIQUIFY-BRUSHES-016": ("FX-LIQUIFY-BRUSHES-004 with the Clone Offset keyed from 3 "
                               "pixels down at frame 0 to 3 pixels up at frame 4: frame 0 "
                               "FX-LIQUIFY-BRUSHES-004's, then copied from above instead.",
                               case(tool="clone", brush_size=6, brush_pressure=100,
                                    stroke=[50, 20], strokes=[PUSH],
                                    clone_offset=keyed((0, [0, 3]), (4, [0, -3]))), [0, 4]),
}

MISSING = {
    "FX-LIQUIFY-BRUSHES-017": ("Freeze Area Mask 2, of one.",
                               case(freeze_area_mask=2, masks=[RS.none(SQUARE)])),
}

INVALID = {
    "FX-LIQUIFY-BRUSHES-018": ("Tool \"smudge\", not one of the twelve.", case(tool="smudge")),
    "FX-LIQUIFY-BRUSHES-019": ("Turbulent Jitter 0, below 1.", case(turbulent_jitter=0)),
    "FX-LIQUIFY-BRUSHES-020": ("Reconstruction Mode \"smooth\", not one of the four.",
                               case(reconstruction_mode="smooth")),
    "FX-LIQUIFY-BRUSHES-021": ("An earlier stroke with tool number 12, past the last.",
                               case(strokes=[[12] + PUSH[1:]])),
    "FX-LIQUIFY-BRUSHES-022": ("An earlier stroke with half a point over.",
                               case(strokes=[PUSH[:-1]])),
    "FX-LIQUIFY-BRUSHES-023": ("An earlier stroke with a point 300 per cent across.",
                               case(strokes=[entry("warp", 8, 50, [25, 50, 300, 50])])),
    "FX-LIQUIFY-BRUSHES-024": ("An earlier stroke with brush size 0.",
                               case(strokes=[entry("warp", 0, 50, ACROSS)])),
    "FX-LIQUIFY-BRUSHES-025": ("An earlier stroke with no points.",
                               case(strokes=[PUSH[:7]])),
    "FX-LIQUIFY-BRUSHES-026": ("Freeze Area Mask 1001, above 1000.",
                               case(freeze_area_mask=1001)),
    "FX-LIQUIFY-BRUSHES-027": ("Clone Offset 10001 pixels across, past 10000.",
                               case(clone_offset=(10001, 0))),
    "FX-LIQUIFY-BRUSHES-028": ("An earlier stroke with reconstruction mode number 1.5, not "
                               "whole.", case(strokes=[PUSH[:6] + [1.5] + PUSH[7:]])),
}


# --- the project files ----------------------------------------------------------------------

def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    p = RS.project_json(fx, c, type_id="core.liquify", names=NAMES, words=WORDS)
    (OUT / name).write_bytes((json.dumps(p, indent=2) + "\n").encode("utf-8"))
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in B.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(L.S.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in MISSING.items():
        says += (" Nothing to freeze by: the layer is drawn without the effect, which is kept as "
                 "written, and EFFECT_PATH_MISSING is said every frame.")
        before = plain(c)
        assert chosen(c, 0) == MISSING_PATH and render(c, 0) == before
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "frame_warning": "EFFECT_PATH_MISSING"}
        print(f"{fx}: no path")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_liquify_brushes.json").write_bytes(
        (json.dumps(expected, indent=1) + "\n").encode("utf-8"))
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda a, b, e=1e-9: all(near(p, q, e) for p, q in zip(a, b))  # noqa: E731
    at = lambda px, x, y: px[y * W + x]  # noqa: E731
    n = lambda k: c[f"FX-LIQUIFY-BRUSHES-{k:03}"]  # noqa: E731
    old = json.loads((OUT.parent / "liquify" / "expected_liquify.json").read_text("utf-8"))
    push = old["cases"]["FX-LIQUIFY-002"]["frames"]["0"]

    # The hash and the noise: fixed values, whole corners give their lattice values.
    assert hash01(0, 0, 0) == 0.0 and 0 <= hash01(-3, 5, 1) < 1
    assert abs(noise(2, 3, 1) - (2 * hash01(2, 3, 1) - 1)) < 1e-15

    # With the new settings as they start, D-605's rule: every D-605 case drawn the same here.
    for fx, (_, oc, frames) in L.CASES.items():
        nc = dict(oc, turbulent_jitter=70, clone_offset=[0, 0], reconstruction_mode="revert",
                  freeze_area_mask=0, strokes=[], masks=[])
        for f in frames:
            assert same(render(nc, f), old["cases"][fx]["frames"][str(f)]), (fx, f)

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx] or "frame_warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    looks = [n(k)["0"] for k in range(1, 14)]
    for i, a in enumerate(looks):
        assert not same(a, drawn), i + 1
        for j, b in enumerate(looks[i + 1:]):
            if (i + 1, i + 2 + j) != (11, 12):
                assert not same(a, b), (i + 1, i + 2 + j)
    assert not same(n(4)["0"], push) and not same(n(5)["0"], push)
    assert same(n(14)["0"], push)
    # Freeze: the stripes beside the middle move less than under the bare push; the mask holds them still.
    moved = lambda px: sum(abs(a - b) for x in range(5, 12) for y in (3, 6)  # noqa: E731
                           for a, b in zip(at(px, x, y), at(drawn, x, y)))
    assert moved(n(9)["0"]) < moved(n(10)["0"]) < moved(push)
    # The field is frozen at the grid's corners inside the mask: the pixels whose four corners
    # are all inside (columns 4 to 9, rows 2 to 5) keep the drawing's own.
    for x in range(4, 10):
        for y in range(2, 6):
            assert near(at(n(11)["0"], x, y), at(drawn, x, y)), (x, y)
    s12 = frame(liquify(B.drawn_layer("stripes"), settings(case(freeze_area_mask=1), 0),
                        flatten(points_at(RS.none(SQUARE), 0))), 3)
    assert same(n(12)["0"], s12)
    keyed_on = n(15)
    assert not same(keyed_on["0"], keyed_on["4"]) and not same(keyed_on["2"], keyed_on["4"])
    assert same(n(16)["0"], n(4)["0"]) and not same(n(16)["4"], n(4)["0"])
    assert value_at(keyed((0, 20, OVERSHOOT), (4, 100)), 2) > 100
    print("checked")


if __name__ == "__main__":
    main()
