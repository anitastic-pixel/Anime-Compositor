"""Cross Blur, worked a second way.

`core.cross_blur`, modelled on CycoreFX's CC Cross Blur (its manual, and tutorials that use it
for light streaks and shine): the layer is blurred across only and, separately, down only, and
the two blurs are laid together with a mode. Nothing is ported; this is this program's own
reading. Document 21 is the rule in words; this file is the reference for the numbers document
25 pins against it.

The rule.

- `radius_x` and `radius_y`, 0 to 500 pixels each. Each is one pass of Fast Box Blur's box
  (D-327): a whole radius k averages the 2k + 1 pixels round each one, and a part f past it adds
  the next pixel on each side at weight f. Radius 0 leaves that direction untouched.
- `across` is the layer blurred across by radius_x only; `down` the layer blurred down by
  radius_y only. Both are premultiplied.
- `mode`, "blend", "add", "screen", "multiply", "lighten" or "darken". Blend is the plain mean
  of the two, (across + down) / 2, so a lit dot becomes a cross with arms half as strong. Every
  other mode lays `across` on `down` by document 21's general blend rule, Co = (1 - As) Cd +
  (1 - Ad) Cs + As Ad B(cs, cd), Ao = As + Ad - As Ad, with B as the composition's blend modes
  (Add held at 1, as Solid Composite's), and lighten and darken the larger and smaller straight
  colour. All six are symmetric, so which blur is laid on which does not matter.
- `edges`, "transparent" or "repeat", as Fast Box Blur's (D-109): transparent reads nothing past
  the layer and the layer grows by the larger reach; repeat reads the edge pixel held and the
  layer keeps its size. Words are exact.

Every case is directional blur's drawing, 16 by 10, in a composition the same size, unmoved
unless the case says. The projects go into `Fixtures/cross_blur`, the expected frames into
`Fixtures/cross_blur/expected_cross_blur.json`.

**This file never runs the build's code path.** It works in double precision on lists, from
the drawing's 8-bit values, summing each blur directly at each pixel.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/cross_blur_reference.py
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
import fast_box_blur_reference as F  # noqa: E402

W, H = D.W, D.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "cross_blur"
TOLERANCE = 2e-5  # document 25's default for a filter
RADIUS = (0, 500)
MODES = ("blend", "add", "screen", "multiply", "lighten", "darken")


# --- the rule -------------------------------------------------------------------------------

def mix(mode, s, d):
    """`s` laid on `d`, both premultiplied."""
    if mode == "blend":
        return [(a + b) / 2 for a, b in zip(s, d)]
    a_s, a_d = s[3], d[3]
    cs = [v / a_s if a_s else 0.0 for v in s[:3]]
    cd = [v / a_d if a_d else 0.0 for v in d[:3]]
    out = []
    for c in range(3):
        b = {"add": min(1.0, cs[c] + cd[c]), "screen": cs[c] + cd[c] - cs[c] * cd[c],
             "multiply": cs[c] * cd[c], "lighten": max(cs[c], cd[c]),
             "darken": min(cs[c], cd[c])}[mode]
        out.append((1 - a_s) * d[c] + (1 - a_d) * s[c] + a_s * a_d * b)
    return out + [a_s + a_d - a_s * a_d]


def cross(px, rx, ry, mode, edges, x, y):
    """The output at (x, y) of the drawing's own pixels, which may lie outside it."""
    if edges == "repeat" and not (0 <= x < W and 0 <= y < H):
        return [0.0] * 4

    def pixel(sx, sy):
        if edges == "repeat":
            sx, sy = min(max(sx, 0), W - 1), min(max(sy, 0), H - 1)
        elif not (0 <= sx < W and 0 <= sy < H):
            return [0.0] * 4
        return px[sy * W + sx]

    def one(radius, step):
        b = F.box(radius)
        h = len(b) // 2
        acc = [0.0] * 4
        for i in range(-h, h + 1):
            p = pixel(x + i * step[0], y + i * step[1])
            for c in range(4):
                acc[c] += b[i + h] * p[c]
        return acc

    return mix(mode, one(rx, (1, 0)), one(ry, (0, 1)))


# --- the cases ------------------------------------------------------------------------------

def case(rx=4, ry=4, mode=None, edges=None, shift=0):
    """`mode` and `edges` None: the file does not say, so blend and transparent."""
    return {"rx": rx, "ry": ry, "mode": mode, "edges": edges, "shift": shift}


def render(c, frame_no):
    px = [D.working(p) for row in D.DRAWINGS["bars"] for p in row]
    rx = RB.clamp(value_at(c["rx"], frame_no), RADIUS)
    ry = RB.clamp(value_at(c["ry"], frame_no), RADIUS)
    return [cross(px, rx, ry, c["mode"] or "blend", c["edges"] or "transparent",
                  x - c["shift"], y) for y in range(H) for x in range(W)]


def plain(c):
    return render(case(rx=0, ry=0, edges="repeat", shift=c["shift"]), 0)


CASES = {
    "FX-CROSS-001": ("Radius X 4, Radius Y 4, mode blend (the default): every edge streaks "
                     "across and down, four pixels, at half strength, and not diagonally, so "
                     "the half-covered red pixel becomes a plus sign.", case(), [0]),
    "FX-CROSS-002": ("Radius X 6, Radius Y 0, blend: the across blur laid half and half on the "
                     "drawing itself, a soft streak across over a sharp copy.",
                     case(rx=6, ry=0), [0]),
    "FX-CROSS-003": ("Mode add: the two blurs added, held at 1, so where they cross they are "
                     "brighter than either.", case(mode="add"), [0]),
    "FX-CROSS-004": ("Mode screen.", case(mode="screen"), [0]),
    "FX-CROSS-005": ("Mode multiply: only where both blurs reach does colour stay as it is; "
                     "the plus sign's arms keep the colour of the one blur there.",
                     case(mode="multiply"), [0]),
    "FX-CROSS-006": ("Mode lighten: the lighter colour of the two blurs.",
                     case(mode="lighten"), [0]),
    "FX-CROSS-007": ("Mode darken: the darker colour of the two blurs.",
                     case(mode="darken"), [0]),
    "FX-CROSS-008": ("Edges repeat: past the edge the edge pixel is read, so the line down the "
                     "left edge keeps its strength there, and the layer does not grow.",
                     case(edges="repeat"), [0]),
    "FX-CROSS-009": ("Radius X 2.5, Radius Y 1: the box of 5 and half of the next pixel each "
                     "side, across; 3 down.", case(rx=2.5, ry=1), [0]),
    "FX-CROSS-010": ("Radius X 0, Radius Y 0: the drawing untouched.", case(rx=0, ry=0), [0]),
    "FX-CROSS-011": ("Radius X keyed from 0 at frame 0 to 8 at frame 4, linear, Radius Y 2: "
                     "frame 0 blurs down only, half and half with the drawing, frame 2 is "
                     "Radius X 4.", case(rx=keyed((0, 0), (4, 8)), ry=2), [0, 2, 4]),
    "FX-CROSS-012": ("Moved three pixels right, edges transparent: the layer grows, so the "
                     "line's streak reaches the three columns left of the drawing.",
                     case(shift=3), [0]),
    "FX-CROSS-013": ("Moved three pixels right, edges repeat: nothing left of the drawing.",
                     case(shift=3, edges="repeat"), [0]),
}

INVALID = {
    "FX-CROSS-014": ("Radius X 501, above 500.", case(rx=501)),
    "FX-CROSS-015": ("Radius Y -1, below 0.", case(ry=-1)),
    "FX-CROSS-016": ("Mode \"overlay\", which is not one.", case(mode="overlay")),
    "FX-CROSS-017": ("Mode \"Add\": the word is exact, so a capital is not it.",
                     case(mode="Add")),
    "FX-CROSS-018": ("Edges \"Repeat\": the word is exact.", case(edges="Repeat")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = RB.project_json(fx, RB.case(shift=c["shift"]))
    params = {"radius_x": setting_json(c["rx"]), "radius_y": setting_json(c["ry"])}
    for word in ("mode", "edges"):
        if c[word] is not None:
            params[word] = c[word]
    p["compositions"][0]["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.cross_blur", "enabled": True,
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
    (OUT / "expected_cross_blur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b: all(abs(p - q) < 1e-12 for u, v in zip(a, b)  # noqa: E731
                            for p, q in zip(u, v))
    one = c["FX-CROSS-001"]["0"]
    # The red pixel at (12, 4), alone: a plus sign. Its arms are lit, its diagonals are not
    # (the skin block is three columns away, past (13, 5)'s reach across only by row 5).
    assert one[at(12, 0)][3] > 0 and one[at(15, 4)][3] > 0
    assert one[at(14, 2)] == [0.0] * 4 and one[at(15, 8)] == [0.0] * 4
    # Blend: at the red pixel the mean of the two blurs. Across, its row reads it and two skin
    # pixels, (8, 4) and (9, 4), a ninth each; down, its column reads it alone.
    red, skin = drawn[at(12, 4)], drawn[at(7, 4)]
    assert near([one[at(12, 4)]], [[((r + 2 * k) / 9 + r / 9) / 2 for r, k in zip(red, skin)]])
    # Radius Y 0: the drawing itself is the down blur, so half of it shows through, sharp.
    two = c["FX-CROSS-002"]["0"]
    assert near([two[at(12, 4)]], [[((r + 4 * k) / 13 + r) / 2 for r, k in zip(red, skin)]])
    assert two[at(12, 2)] == [0.0] * 4
    # The modes differ, and at every pixel, colour by colour, add is at least screen, screen
    # at least lighten, lighten at least darken, darken at least multiply.
    frames = {m: c[f"FX-CROSS-00{i}"]["0"] for i, m in
              zip((1, 3, 4, 5, 6, 7), ("blend",) + MODES[1:])}
    for m in MODES:
        for n in MODES:
            assert m == n or frames[m] != frames[n], (m, n)
    order = ("add", "screen", "lighten", "darken", "multiply")
    for a, b in zip(order, order[1:]):
        assert all(frames[a][i][k] >= frames[b][i][k] - 1e-12
                   for i in range(W * H) for k in range(3)), (a, b)
    # Repeat: the line's own column keeps more of the line than transparent's.
    assert c["FX-CROSS-008"]["0"][at(0, 4)][3] > one[at(0, 4)][3]
    assert c["FX-CROSS-010"]["0"] == drawn
    eleven = c["FX-CROSS-011"]
    assert near(eleven["0"], render(case(rx=0, ry=2), 0))
    assert near(eleven["2"], render(case(rx=4, ry=2), 0))
    assert near(eleven["4"], render(case(rx=8, ry=2), 0))
    # Moved: transparent grows into the columns left of the drawing; repeat does not.
    twelve, thirteen = c["FX-CROSS-012"]["0"], c["FX-CROSS-013"]["0"]
    assert all(twelve[at(x, 4)][3] > 0 for x in range(3))
    assert all(thirteen[at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    assert all(near([twelve[at(x, y)]], [one[at(x - 3, y)]]) for x in range(3, W) for y in range(H))
    for name, frames_ in c.items():
        for px in frames_.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)


if __name__ == "__main__":
    main()
