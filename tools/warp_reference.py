"""Warp, worked a second way.

D-604 adds `core.warp`, after After Effects' Warp (Distort): fifteen ready-made bends, the same
names Illustrator's and Photoshop's warps have, with Bend and two Distortions. After Effects
names each style and publishes no formula; the shapes below are this program's own, and nothing
is ported.

The rule. The drawing, W by H, is laid on u and v, each -1/2 to 1/2 (x = (u + 1/2) W,
y = (v + 1/2) H). b = `bend` / 100, h = `horizontal_distortion` / 100,
d = `vertical_distortion` / 100, e = 1 - 4 u^2 (1 down the middle, 0 at the sides) and
f = 1 - 4 v^2 (1 across the middle, 0 at the top and bottom). With `warp_axis` `vertical`, u and
v change places before the style and back after it, so the bend runs down the drawing.

Each style takes (u, v) to (u', v'):

- `arc`: the drawing bent round a circle, the middle line's arc a quarter turn at Bend 100:
  T = b pi / 2, R = 1 / T, a = u T, r = R - v, u' = r sin a, v' = R - r cos a (b 0: unmoved).
- `arc_lower`: v' = v + b e (v + 1/2) / 2: the bottom edge bowed, the top kept.
- `arc_upper`: v' = v - b e (1/2 - v) / 2: the top edge bowed, the bottom kept.
- `arch`: v' = v - b e / 2: lifted in the middle, the sides kept upright.
- `bulge`: v' = v (1 + b e / 2): top and bottom bowed out together.
- `shell_lower`: as `arc_lower`, and u' = u (1 - b (v + 1/2) / 2): narrowed toward the bottom.
- `shell_upper`: as `arc_upper`, and u' = u (1 - b (1/2 - v) / 2): narrowed toward the top.
- `flag`: v' = v + b sin(2 pi u) / 4: one whole wave across, every row alike.
- `wave`: v' = v + b sin(2 pi u + pi (v + 1/2)) / 4: the wave half a turn on from top to bottom.
- `fish`: v' = v (1 + b sin(2 pi u) / 2): thick at one end, thin at the other, one way then the
  other.
- `rise`: v' = v - b sin(pi u) / 2: the right side raised, the left lowered, smoothly.
- `fisheye`: s = 1 + b (1 - 2 (u^2 + v^2)) / 2, u' = u s, v' = v s: swollen from the middle,
  the corners kept.
- `inflate`: u' = u (1 + b f / 2), v' = v (1 + b e / 2): swollen both ways.
- `squeeze`: u' = u (1 - b f / 2), v' = v (1 + b e / 2): drawn in across, swollen up and down.
- `twist`: p = b pi (1 - 2 (u^2 + v^2)) / 2, (u', v') = (u cos p - v sin p, u sin p + v cos p):
  turned most in the middle, the corners kept.

A coordinate a style leaves out stays as it was. Then the distortions, from (u', v') together:
u'' = u' (1 + d v'), v'' = v' (1 + h u'): Horizontal Distortion makes the right side taller
than the left (shorter below 0), Vertical Distortion the bottom wider than the top.

The drawing is cut into a G by G grid, G = 64, each grid point (a, c) at u = a / G - 1/2,
v = c / G - 1/2 moved as above to x = (u'' + 1/2) W, y = (v'' + 1/2) H, and drawn as
`tools/mesh_reference.py` (D-601) draws a forward mesh: outside the bent drawing clear, where it
folds the part later in the grid on top. Nothing grows: what is bent past the edges is cut off.
With Bend 0 and both Distortions 0, every style leaves the drawing untouched.

`warp_style` one of the fifteen, `arc` when added; `warp_axis` `horizontal` or `vertical`,
`horizontal`; `bend` -100 to 100, 50; `horizontal_distortion` and `vertical_distortion` -100 to
100, 0. The numbers are keyable. Everything is a share of the drawing, so a draft needs no
change.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/warp/media`, the projects into
`Fixtures/warp`, and the expected frames into `Fixtures/warp/expected_warp.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/warp_reference.py
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
from drop_shadow_reference import frame  # noqa: E402
import mesh_reference as M  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "warp"
TOLERANCE = 2e-5  # document 25's default for a filter
G = 64
STYLES = ("arc", "arc_lower", "arc_upper", "arch", "bulge", "shell_lower", "shell_upper", "flag",
          "wave", "fish", "rise", "fisheye", "inflate", "squeeze", "twist")
NAMES = ("warp_style", "warp_axis", "bend", "horizontal_distortion", "vertical_distortion")
RANGES = {"bend": (-100, 100), "horizontal_distortion": (-100, 100),
          "vertical_distortion": (-100, 100)}


# --- the rule -------------------------------------------------------------------------------

def styled(style, b, u, v):
    e, f = 1 - 4 * u * u, 1 - 4 * v * v
    if style == "arc":
        if b == 0:
            return u, v
        t = b * math.pi / 2
        big = 1 / t
        a, r = u * t, big - v
        return r * math.sin(a), big - r * math.cos(a)
    if style == "arc_lower":
        return u, v + b * e * (v + 0.5) / 2
    if style == "arc_upper":
        return u, v - b * e * (0.5 - v) / 2
    if style == "arch":
        return u, v - b * e / 2
    if style == "bulge":
        return u, v * (1 + b * e / 2)
    if style == "shell_lower":
        return u * (1 - b * (v + 0.5) / 2), v + b * e * (v + 0.5) / 2
    if style == "shell_upper":
        return u * (1 - b * (0.5 - v) / 2), v - b * e * (0.5 - v) / 2
    if style == "flag":
        return u, v + b * math.sin(2 * math.pi * u) / 4
    if style == "wave":
        return u, v + b * math.sin(2 * math.pi * u + math.pi * (v + 0.5)) / 4
    if style == "fish":
        return u, v * (1 + b * math.sin(2 * math.pi * u) / 2)
    if style == "rise":
        return u, v - b * math.sin(math.pi * u) / 2
    if style == "fisheye":
        s = 1 + b * (1 - 2 * (u * u + v * v)) / 2
        return u * s, v * s
    if style == "inflate":
        return u * (1 + b * f / 2), v * (1 + b * e / 2)
    if style == "squeeze":
        return u * (1 - b * f / 2), v * (1 + b * e / 2)
    p = b * math.pi * (1 - 2 * (u * u + v * v)) / 2  # twist
    return u * math.cos(p) - v * math.sin(p), u * math.sin(p) + v * math.cos(p)


def moved(s_, u, v):
    b = s_["bend"] / 100
    h, d = s_["horizontal_distortion"] / 100, s_["vertical_distortion"] / 100
    if s_["warp_axis"] == "vertical":
        q, p = styled(s_["warp_style"], b, v, u)
    else:
        p, q = styled(s_["warp_style"], b, u, v)
    return p * (1 + d * q), q * (1 + h * p)


def warp(layer, s_):
    if s_["bend"] == 0 and s_["horizontal_distortion"] == 0 and s_["vertical_distortion"] == 0:
        return layer
    points = []
    for c in range(G + 1):
        for a in range(G + 1):
            p, q = moved(s_, a / G - 0.5, c / G - 0.5)
            points.append(((p + 0.5) * W, (q + 0.5) * H))
    return M.draw_mesh(layer, points, G, G, W, H)


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, warp_style="arc", warp_axis="horizontal", bend=50, horizontal_distortion=0,
         vertical_distortion=0):
    return {"drawing": "stripes", "shift": shift, "warp_style": warp_style, "warp_axis": warp_axis,
            "bend": bend, "horizontal_distortion": horizontal_distortion,
            "vertical_distortion": vertical_distortion}


def settings(c, frame_no):
    held = {k: c[k] for k in ("warp_style", "warp_axis")}
    for k, (lo, hi) in RANGES.items():
        held[k] = min(hi, max(lo, value_at(c[k], frame_no)))
    return held


def render(c, frame_no):
    return frame(warp(B.drawn_layer(c["drawing"]), settings(c, frame_no)), c["shift"])


def plain(c):
    return frame(B.drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-WARP-001": ("The settings as they start: Arc, Bend 50: the drawing bent round a circle, "
                    "its middle line an eighth of a turn, the sides drooping.", case(), [0]),
    "FX-WARP-002": ("Bend 0, both Distortions 0: the drawing, untouched.", case(bend=0), [0]),
}
SAYS = {
    "arc_lower": "Arc Lower, Bend 50: the bottom edge bowed down, the top kept.",
    "arc_upper": "Arc Upper, Bend 50: the top edge bowed up, the bottom kept.",
    "arch": "Arch, Bend 50: lifted in the middle, the sides kept upright, clear below the middle.",
    "bulge": "Bulge, Bend 50: the top and bottom bowed out together, cut off at the edges.",
    "shell_lower": "Shell Lower, Bend 50: the bottom bowed down and narrowed toward it.",
    "shell_upper": "Shell Upper, Bend 50: the top bowed up and narrowed toward it.",
    "flag": "Flag, Bend 50: one whole wave across, every row alike.",
    "wave": "Wave, Bend 50: the wave half a turn on from the top to the bottom.",
    "fish": "Fish, Bend 50: thick at one end and thin at the other.",
    "rise": "Rise, Bend 50: the right side raised and the left lowered, smoothly.",
    "fisheye": "Fisheye, Bend 50: swollen from the middle, the corners kept.",
    "inflate": "Inflate, Bend 50: swollen both ways, the corners kept.",
    "squeeze": "Squeeze, Bend 50: drawn in across and swollen up and down.",
    "twist": "Twist, Bend 50: turned most in the middle, the corners kept.",
}
for n, style in enumerate(STYLES[1:], 3):
    CASES[f"FX-WARP-{n:03}"] = (SAYS[style], case(warp_style=style), [0])
CASES.update({
    "FX-WARP-017": ("Arc at Bend -50: bent the other way, the sides lifted, the mirror image of "
                    "FX-WARP-001 top for bottom.", case(bend=-50), [0]),
    "FX-WARP-018": ("Flag with Warp Axis vertical: the wave runs down the drawing, the columns "
                    "sway.", case(warp_style="flag", warp_axis="vertical"), [0]),
    "FX-WARP-019": ("Bend 0, Horizontal Distortion 60: no bend, the right side taller than the "
                    "left, cut off where it passes the edges.",
                    case(bend=0, horizontal_distortion=60), [0]),
    "FX-WARP-020": ("Bend 0, Vertical Distortion -60: the bottom narrower than the top.",
                    case(bend=0, vertical_distortion=-60), [0]),
    "FX-WARP-021": ("Bulge, Bend 40, with both Distortions at 30.",
                    case(warp_style="bulge", bend=40, horizontal_distortion=30,
                         vertical_distortion=30), [0]),
    "FX-WARP-022": ("Bend keyed from 0 at frame 0 to 100 at frame 4, linear, Twist: frame 0 the "
                    "drawing, frame 4 a whole Twist, frame 2 between.",
                    case(warp_style="twist", bend=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-WARP-023": ("Bend keyed from 60 at frame 0 to 100 at frame 4, eased past its end: frame "
                    "2 would pass 100 and is held there, the same as frame 4.",
                    case(bend=keyed((0, 60, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-WARP-024": ("FX-WARP-001 with the layer moved three pixels right: the bent drawing moves "
                    "with it.", case(3), [0]),
    "FX-WARP-025": ("Twist at Bend 100 with Warp Axis vertical: u and v change places, so the "
                    "turn runs the other way round from FX-WARP-022's frame 4.",
                    case(warp_style="twist", bend=100, warp_axis="vertical"), [0]),
})

INVALID = {
    "FX-WARP-026": ("Warp Style \"spiral\", not one of the fifteen.", case(warp_style="spiral")),
    "FX-WARP-027": ("Warp Axis \"diagonal\".", case(warp_axis="diagonal")),
    "FX-WARP-028": ("Bend 101, above 100.", case(bend=101)),
    "FX-WARP-029": ("Horizontal Distortion -101, below -100.", case(horizontal_distortion=-101)),
    "FX-WARP-030": ("Vertical Distortion keyed to 150 at frame 4.",
                    case(vertical_distortion=keyed((0, 0), (4, 150)))),
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
        "instance_id": "fx-0-0", "type_id": "core.warp", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_bytes((json.dumps(project_json(fx, c), indent=2) + "\n").encode("utf-8"))
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

    (OUT / "expected_warp.json").write_bytes((json.dumps(expected, indent=1) + "\n").encode("utf-8"))
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    layer = B.drawn_layer("stripes")
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda a, b, e=1e-9: all(near(p, q, e) for p, q in zip(a, b))  # noqa: E731

    # Every style at Bend 0 leaves every grid point where it was.
    for style in STYLES:
        for axis in ("horizontal", "vertical"):
            s_ = {"warp_style": style, "warp_axis": axis, "bend": 0, "horizontal_distortion": 0,
                  "vertical_distortion": 0}
            for u, v in ((-0.5, -0.5), (0.1, -0.3), (0.5, 0.5), (0.25, 0.0)):
                assert near(moved(s_, u, v), (u, v), 1e-12), (style, axis)
    # The corners stay put for the styles that say so.
    for style in ("fisheye", "twist", "inflate", "squeeze"):
        s_ = {"warp_style": style, "warp_axis": "horizontal", "bend": 70,
              "horizontal_distortion": 0, "vertical_distortion": 0}
        for u, v in ((-0.5, -0.5), (0.5, -0.5), (-0.5, 0.5), (0.5, 0.5)):
            assert near(moved(s_, u, v), (u, v), 1e-12), style
    # Arc keeps the middle line's middle and bends the edges' ends down at a positive bend.
    s_ = case()
    assert near(moved(settings(s_, 0), 0, 0), (0, 0), 1e-12)
    assert moved(settings(s_, 0), 0.5, 0)[1] > 0.05

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    assert c["FX-WARP-002"]["0"] == drawn
    looks = [c[f"FX-WARP-{n:03}"]["0"] for n in range(3, 17)] + [c["FX-WARP-001"]["0"]]
    for i, a in enumerate(looks):
        assert not same(a, drawn), i
        for b in looks[i + 1:]:
            assert not same(a, b), i
    assert not same(c["FX-WARP-017"]["0"], c["FX-WARP-001"]["0"])
    assert not same(c["FX-WARP-018"]["0"], c[f"FX-WARP-{STYLES.index('flag') + 2:03}"]["0"])
    assert not same(c["FX-WARP-019"]["0"], drawn) and not same(c["FX-WARP-020"]["0"], drawn)
    twist = c["FX-WARP-022"]
    assert twist["0"] == drawn and not same(twist["2"], drawn) and not same(twist["2"], twist["4"])
    held = c["FX-WARP-023"]
    assert value_at(keyed((0, 60, OVERSHOOT), (4, 100)), 2) > 100 and held["2"] == held["4"]
    assert same(c["FX-WARP-024"]["0"], frame(warp(layer, settings(case(), 0)), 3))
    assert not same(c["FX-WARP-025"]["0"], twist["4"])
    print("checked")


if __name__ == "__main__":
    main()
