"""D-334's soft core for Lightning Bolt, worked a second way.

From P-26's tutorial 2. After Effects' Advanced Lightning draws its core bright in the middle of
the bolt and fading to its edge; D-190's core covers its whole width evenly, so tutorial 2's bolt
read as a flat ribbon with a hard edge. `core.lightning_bolt` gains `core`: "hard", what a file
without it means, D-190's core exactly; or "soft".

The rule, on top of D-190's (`tools/lightning_bolt_reference.py`) and D-324's
(`tools/lightning_extras_reference.py`). A segment's core at a pixel centre is the share of the
pixel's width across the line that the core covers, d being the distance to the line and
half = w * width / 2 the core's half width there. Hard covers that share evenly:
clamp(min(d + 0.5, half) - max(d - 0.5, -half), 0, 1). Soft weighs each point of the share by
1 - |u| / half, u being its distance from the line, so the core is
rise(d + 0.5) - rise(d - 0.5), with rise(u) = sign(u) (a - a^2 / (2 half)) and a = min(|u|, half):
1 - 1 / (4 half) on the line of a wide core, falling to 0 at its edge. A core of no width is 0.
The glow, and how core and glow are laid on, are D-190's.

**This file never runs the build's code path.** It works in double precision on lists.

Every case is D-190's 16 by 10 composition and drawing. The projects go into
`Fixtures/lightning_core`, the expected frames into
`Fixtures/lightning_core/expected_lightning_core.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/lightning_core_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
import lightning_bolt_reference as LB  # noqa: E402
import lightning_extras_reference as LX  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = LB.W, LB.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "lightning_core"
TOLERANCE = LB.TOLERANCE


# --- the rule -------------------------------------------------------------------------------

def soft(d, half):
    """The soft core at distance d from the line of a core half wide."""
    if half <= 0:
        return 0.0

    def rise(u):
        a = min(abs(u), half)
        return math.copysign(a - a * a / (2 * half), u) if u else 0.0
    return rise(d + 0.5) - rise(d - 0.5)


def light(seg, width, glow, X, Y, core):
    """A segment's (core, glow) at the point (X, Y): LB.light, or the soft core."""
    k, g = LB.light(seg, width, glow, X, Y)
    if core == "hard":
        return k, g
    P, Q, wp, wq = seg[:4]
    dx, dy = Q[0] - P[0], Q[1] - P[1]
    L2 = dx * dx + dy * dy
    t = 0.0 if L2 == 0 else min(1.0, max(0.0, ((X - P[0]) * dx + (Y - P[1]) * dy) / L2))
    d = math.hypot(X - P[0] - t * dx, Y - P[1] - t * dy)
    return soft(d, (wp + t * (wq - wp)) * width / 2), g


def field(segs, width, glow, core):
    """LB.field with the core chosen."""
    C, G = [0.0] * (W * H), [0.0] * (W * H)
    for seg in segs:
        e = LB.reach(seg, width, glow)
        (px, py), (qx, qy) = seg[0], seg[1]
        for y in range(max(0, math.floor(min(py, qy) - e)), min(H, math.ceil(max(py, qy) + e) + 1)):
            for x in range(max(0, math.floor(min(px, qx) - e)), min(W, math.ceil(max(px, qx) + e) + 1)):
                k, g = light(seg, width, glow, x + 0.5, y + 0.5, core)
                i = y * W + x
                C[i], G[i] = max(C[i], k), max(G[i], g)
    return C, G


def render(c, frame_no):
    pixels = [p for row in LX.DRAWINGS[c["drawing"]] for p in row]
    working = [R.working(p) for p in pixels]
    opacity, width, glow = (LX.held(c, k, frame_no) for k in ("opacity", "width", "glow"))
    if opacity == 0 or (width == 0 and glow == 0):
        return working
    C, G = field([s[:7] for s in LX.segments(c, frame_no)], width, glow, c["core"])
    core = [srgb_to_linear(v / 255) for v in R.hex_color(c["color"].lower())]
    halo = [srgb_to_linear(v / 255) for v in R.hex_color(c["glow_color"].lower())]
    s = opacity / 100
    out = []
    for o, k, g in zip(working, C, G):
        lit = [k * core[ch] + (1 - k) * g * halo[ch] for ch in range(3)]
        out.append([o[ch] + s * lit[ch] for ch in range(3)] +
                   [o[3] + s * (k + (1 - k) * g) * (1 - o[3])])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(core="soft", **settings):
    c = LX.case(**settings)
    c["core"] = core
    return c


# A straight line along y = 4.5, the middle of row 4, its ends outside the frame.
WIDE = {**LB.LINE, "width": 6}

CASES = {
    "FX-LCORE-001": ("A straight line along the middle of row 4, width 6, no glow, core Soft: "
                     "brightest on row 4 (1 - 1/12 of the colour), fading row by row to nothing "
                     "past rows 1 and 7.", case(**WIDE), [0]),
    "FX-LCORE-002": ("FX-LCORE-001 with core written \"hard\": rows 2 to 6 fully lit, rows 1 and "
                     "7 half, as D-190's core.", case(core="hard", **WIDE), [0]),
    "FX-LCORE-003": ("FX-BOLT-001's settings with core Soft: the same bolt and glow, the core "
                     "dimmer toward its edge.", case(), [0, 2]),
    "FX-LCORE-004": ("The line at width 1, core Soft: row 4 half lit, as a core half a pixel "
                     "wide on each side weighs its middle pixel.",
                     case(**{**WIDE, "width": 1}), [0]),
    "FX-LCORE-005": ("The line at width 0 with glow 3, core Soft: no core, only D-190's glow.",
                     case(**{**WIDE, "width": 0, "glow": 3}), [0]),
}

INVALID = {
    "FX-LCORE-006": ("A core \"Soft\": the word is exact, so a capital is not it.",
                     case(core="Soft")),
    "FX-LCORE-007": ("A core \"fuzzy\", which is not one.", case(core="fuzzy")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = LX.project_json(fx, c)
    p["compositions"][0]["layers"][0]["effects"][0]["parameters"]["core"] = c["core"]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in LX.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {str(f): render(c, f) for f in frames}}
        print(f"{fx}: {says[:60]}")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = render(case(opacity=0), 0)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_lightning_core.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    night = render(case(opacity=0), 0)
    lit = lambda px, x, y: px[at(x, y)][3] - night[at(x, y)][3]  # noqa: E731

    # The soft weighting integrates to the hard one's area times a half, and peaks at 1 - 1/(4 half).
    assert abs(soft(0, 3) - (1 - 1 / 12)) < 1e-12 and soft(3.5, 3) == 0 and soft(0, 0) == 0
    assert abs(sum(soft(d + 0.5, 3) for d in range(-10, 10)) - 3) < 1e-12
    # 001: on the clear right half (column 12) the covering falls row by row from row 4.
    one = [lit(c["FX-LCORE-001"]["0"], 12, y) for y in range(H)]
    assert abs(one[4] - (1 - 1 / 12)) < 1e-12
    assert one[4] > one[3] > one[2] > one[1] > 0 and one[0] == 0 and one[3] == one[5]
    # 002: the hard core, D-190's rule exactly.
    two = c["FX-LCORE-002"]["0"]
    assert two == LX.render(LX.case(**WIDE), 0)
    assert [round(lit(two, 12, y), 12) for y in range(H)] == [0, 0.5, 1, 1, 1, 1, 1, 0.5, 0, 0]
    # 003: the same segments as FX-BOLT-001, and dimmer than it, never brighter.
    hard = LX.render(LX.case(), 0)
    three = c["FX-LCORE-003"]["0"]
    assert three != hard and all(a[0] <= b[0] + 1e-12 for a, b in zip(three, hard))
    # 004: half lit on row 4.
    assert abs(lit(c["FX-LCORE-004"]["0"], 12, 4) - 0.5) < 1e-12
    # 005: no core, the glow as the hard core's.
    assert c["FX-LCORE-005"]["0"] == LX.render(LX.case(**{**WIDE, "width": 0, "glow": 3}), 0)
    print("checked")


if __name__ == "__main__":
    main()
