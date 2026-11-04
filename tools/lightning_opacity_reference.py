"""D-1051's Core Opacity and Glow Opacity for Lightning Bolt, worked a second way.

From wave 12's tutorial uGxruFKgt6U, step 5: After Effects' Advanced Lightning is set to Core
Radius 2.4, Core Opacity 75%, Glow Radius 50 and Glow Opacity 50%. `core.lightning_bolt` had one
Opacity for the whole bolt; it gains `core_opacity` and `glow_opacity`, 0 to 100 per cent, 100
when a file does not say, so a file from before draws exactly as it did. Core Radius and Glow
Radius are D-190's Width (the core's whole width, so twice the radius) and Glow (the glow's
reach), unchanged.

The rule, on top of D-190's (`tools/lightning_bolt_reference.py`), D-324's
(`tools/lightning_extras_reference.py`) and D-334's (`tools/lightning_core_reference.py`). At each
pixel D-190 finds the core k and the glow g (the largest of any segment) and lays on
k * core + (1 - k) * g * glow colour, alpha k + (1 - k) g, at the bolt's opacity. Now k is first
multiplied by core_opacity / 100 and g by glow_opacity / 100, and the same laying on follows. So a
core at 0 lets the glow show through where the core was, and both at 100 is D-190's frame number
for number.

**This file never runs the build's code path.** It works in double precision on lists.

Every case is D-190's 16 by 10 composition and drawing. The projects go into
`Fixtures/lightning_opacity`, the expected frames into
`Fixtures/lightning_opacity/expected_lightning_opacity.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/lightning_opacity_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import lightning_bolt_reference as LB  # noqa: E402
import lightning_extras_reference as LX  # noqa: E402
import lightning_core_reference as LC  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = LB.W, LB.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "lightning_opacity"
TOLERANCE = LB.TOLERANCE
NEW = ("core_opacity", "glow_opacity")


# --- the rule -------------------------------------------------------------------------------

def share(c, k, frame_no):
    """A new opacity at a frame, as a share of 1."""
    return min(100, max(0, value_at(c[k], frame_no))) / 100


def render(c, frame_no):
    pixels = [p for row in LX.DRAWINGS[c["drawing"]] for p in row]
    working = [R.working(p) for p in pixels]
    opacity, width, glow = (LX.held(c, k, frame_no) for k in ("opacity", "width", "glow"))
    if opacity == 0 or (width == 0 and glow == 0):
        return working
    C, G = LC.field([s[:7] for s in LX.segments(c, frame_no)], width, glow, c["core"])
    a, b = share(c, "core_opacity", frame_no), share(c, "glow_opacity", frame_no)
    core = [srgb_to_linear(v / 255) for v in R.hex_color(c["color"].lower())]
    halo = [srgb_to_linear(v / 255) for v in R.hex_color(c["glow_color"].lower())]
    s = opacity / 100
    out = []
    for o, k, g in zip(working, C, G):
        k, g = a * k, b * g
        lit = [k * core[ch] + (1 - k) * g * halo[ch] for ch in range(3)]
        out.append([o[ch] + s * lit[ch] for ch in range(3)] +
                   [o[3] + s * (k + (1 - k) * g) * (1 - o[3])])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(core_opacity=100, glow_opacity=100, core="hard", **settings):
    c = LC.case(core=core, **settings)
    c.update(core_opacity=core_opacity, glow_opacity=glow_opacity)
    return c


# D-334's straight line along y = 4.5, the middle of row 4, width 6 with no glow.
WIDE = LC.WIDE
# The same line, width 2 (the core covers y 3.5 to 5.5: row 4 whole, rows 3 and 5 half), glow 3.
GLOWING = {**LB.LINE, "width": 2, "glow": 3}

CASES = {
    "FX-LOPAC-001": ("D-334's wide line (width 6, no glow), core opacity 50: rows 2 to 6 half "
                     "lit and rows 1 and 7 a quarter, half of D-190's core.",
                     case(core_opacity=50, **WIDE), [0]),
    "FX-LOPAC-002": ("The line at width 2 with glow 3, glow opacity 50: the core as D-190's, "
                     "the glow round it half as strong.",
                     case(glow_opacity=50, **GLOWING), [0]),
    "FX-LOPAC-003": ("The same line, core opacity 0: no core, the glow full on the line where "
                     "the core was.", case(core_opacity=0, **GLOWING), [0]),
    "FX-LOPAC-004": ("The same line, glow opacity 0: only the core, as with glow 0.",
                     case(glow_opacity=0, **GLOWING), [0]),
    "FX-LOPAC-005": ("FX-BOLT-001's settings with both written at 100: exactly FX-BOLT-001.",
                     case(), [0, 2]),
    "FX-LOPAC-006": ("FX-BOLT-001's settings at the tutorial's core 75 and glow 50: the same "
                     "bolt, its core and glow both dimmer.",
                     case(core_opacity=75, glow_opacity=50), [0, 2]),
    "FX-LOPAC-007": ("The wide line with the soft core (D-334), core opacity 50: half of "
                     "FX-LCORE-001's covering, row by row.",
                     case(core="soft", core_opacity=50, **WIDE), [0]),
    "FX-LOPAC-008": ("The glowing line with core opacity keyed from 0 at frame 0 to 100 at "
                     "frame 4: frame 0 is FX-LOPAC-003, frame 4 is the plain line, frame 2 "
                     "between.", case(core_opacity=keyed((0, 0), (4, 100)), **GLOWING),
                     [0, 2, 4]),
}

INVALID = {
    "FX-LOPAC-009": ("A core opacity of 101, above 100.", case(core_opacity=101)),
    "FX-LOPAC-010": ("A glow opacity of -1, below 0.", case(glow_opacity=-1)),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = LC.project_json(fx, c)
    params = p["compositions"][0]["layers"][0]["effects"][0]["parameters"]
    for k in NEW:
        params[k] = setting_json(c[k])
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
    (OUT / "expected_lightning_opacity.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                         encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    night = render(case(opacity=0), 0)
    lit = lambda px, x, y: px[at(x, y)][3] - night[at(x, y)][3]  # noqa: E731
    plain = lambda **k: LX.render(LX.case(**k), 0)  # noqa: E731

    # 001: half D-190's hard core, down column 12 on the clear half.
    one = [round(lit(c["FX-LOPAC-001"]["0"], 12, y), 12) for y in range(H)]
    assert one == [0, 0.25, 0.5, 0.5, 0.5, 0.5, 0.5, 0.25, 0, 0], one
    # 002: the core rows as D-190's; past the core the glow is half of D-190's.
    full, two = plain(**GLOWING), c["FX-LOPAC-002"]["0"]
    assert two[at(12, 4)] == full[at(12, 4)]
    g = lambda px, y: lit(px, 12, y)  # noqa: E731
    assert g(full, 2) > 0 and abs(g(two, 2) - g(full, 2) / 2) < 1e-12
    # 003: no core, and the glow on the line is its full weight 1, in the glow colour.
    three = c["FX-LOPAC-003"]["0"]
    assert abs(lit(three, 12, 4) - 1) < 1e-12 and three[at(12, 4)][0] < three[at(12, 4)][2]
    # 004: as glow 0.
    assert c["FX-LOPAC-004"]["0"] == plain(**{**GLOWING, "glow": 0})
    # 005: both at 100 is D-190's frame exactly.
    assert all(c["FX-LOPAC-005"][str(f)] == LX.render(LX.case(), f) for f in (0, 2))
    # 006: never brighter than FX-BOLT-001, and different.
    hard, six = LX.render(LX.case(), 0), c["FX-LOPAC-006"]["0"]
    assert six != hard and all(a[3] <= b[3] + 1e-12 for a, b in zip(six, hard))
    # 007: half of FX-LCORE-001.
    soft = LC.render(LC.case(**WIDE), 0)
    assert all(abs(lit(c["FX-LOPAC-007"]["0"], 12, y) - lit(soft, 12, y) / 2) < 1e-12
               for y in range(H))
    # 008: its ends are 003 and the plain line.
    eight = c["FX-LOPAC-008"]
    assert eight["0"] == three and eight["4"] == full and eight["2"] not in (three, full)
    print("checked")


if __name__ == "__main__":
    main()
