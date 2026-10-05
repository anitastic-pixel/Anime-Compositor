"""D-327's Fast Box Blur, worked a second way.

From P-26: tutorials 2 and 3 blur with After Effects' Fast Box Blur, which the replays stood
Gaussian Blur in for. After Effects' Fast Box Blur repeats a box blur (Blur Radius, Iterations,
default 3, Blur Dimensions, Repeat Edge Pixels): each pass averages the 2r + 1 pixels round each
one, so after n passes nothing lies further than n r pixels out. Tutorial 2 lifts its blurred
reflections 17 to 20 stops with Exposure, where a Gaussian's long faint tail shows and a box's
hard end does not.

`core.fast_box_blur`:

- `radius`, 0 to 500 pixels. A whole radius k is a box of the 2k + 1 pixels round each one; a
  part f past it, as 2.5, adds the next pixel on each side at weight f, so the box widens
  smoothly as the radius is keyed (this program's reading: After Effects takes a part of a
  pixel and says nothing of how).
- `iterations`, 1 to 50, its whole part counted: the box laid on that many times, the kernel
  being the box convolved with itself that many times, reaching iterations x ceil(radius).
- `dimensions`, "both", "horizontal" or "vertical", as Gaussian Blur's (D-303).
- `edges`, "transparent" or "repeat", as Gaussian Blur's (D-109): transparent reads nothing
  past the layer and grows it by the kernel's reach; repeat reads the edge pixel held, once
  for the whole kernel, and does not grow it.

Blurred premultiplied, as every blur here. Radius 0 changes nothing.

Every case is blurriness_reference's composition, 40 by 12, and its drawing `block`, a 4 by 4
orange square against the left edge. The projects go into `Fixtures/fast_box_blur`, the
expected frames into `Fixtures/fast_box_blur/expected_fast_box_blur.json`.

**This file never runs the build's code path.** It works in double precision on lists, from
the drawing's 8-bit values, laying the one-dimensional box on pass by pass (the build may use
the composed kernel) and summing two dimensions directly at each pixel.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/fast_box_blur_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import blurriness_reference as B  # noqa: E402
import smooth_reference as S  # noqa: E402
from fxkey_reference import setting_json  # noqa: E402

W, H, FRAMES = B.W, B.H, B.FRAMES
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "fast_box_blur"
TOLERANCE = 2e-5  # document 25's default for a filter


# --- the rule -------------------------------------------------------------------------------

def box(radius):
    """One pass: weights for -ceil(r)..ceil(r), the end ones at the radius's part past whole."""
    k = math.floor(radius)
    f = radius - k
    one = [1.0] * (2 * k + 1)
    if f > 0:
        one = [f] + one + [f]
    total = sum(one)
    return [v / total for v in one]


def taps(radius, iterations):
    """The box laid on `iterations` times, pass by pass, onto a single lit pixel."""
    n = math.floor(iterations)
    b = box(radius)
    h = len(b) // 2
    reach = n * h
    line = [0.0] * (2 * reach + 1)
    line[reach] = 1.0
    for _ in range(n):
        line = [sum(b[i + h] * line[x - i] for i in range(-h, h + 1) if 0 <= x - i < len(line))
                for x in range(len(line))]
    return line


def blur(px, radius, iterations, edges, dimensions):
    k = taps(radius, iterations)
    r = len(k) // 2
    still = [1.0 if i == r else 0.0 for i in range(2 * r + 1)]
    across = k if dimensions != "vertical" else still
    down = k if dimensions != "horizontal" else still

    def pixel(sx, sy):
        if edges == "repeat":
            sx, sy = min(max(sx, 0), W - 1), min(max(sy, 0), H - 1)
        elif not (0 <= sx < W and 0 <= sy < H):
            return None
        return px[sy * W + sx]

    out = []
    for y in range(H):
        for x in range(W):
            acc = [0.0] * 4
            for j in range(-r, r + 1):
                if down[j + r] == 0:
                    continue
                for i in range(-r, r + 1):
                    if across[i + r] == 0:
                        continue
                    p = pixel(x + i, y + j)
                    if p:
                        for c in range(4):
                            acc[c] += p[c] * across[i + r] * down[j + r]
            out.append(acc)
    return out


# --- the cases ------------------------------------------------------------------------------

def case(radius, iterations=3, edges=None, dimensions=None, stops=None, float_depth=False):
    return {"radius": radius, "iterations": iterations, "edges": edges,
            "dimensions": dimensions, "stops": stops, "float": float_depth}


def render(c):
    px = B.plain()
    px = blur(px, c["radius"], c["iterations"], c["edges"] or "transparent",
              c["dimensions"] or "both")
    return B.exposure(px, c["stops"]) if c["stops"] is not None else px


CASES = {
    "FX-FASTBOX-001": ("Radius 4, iterations 3 (After Effects' default): the box laid on three "
                       "times, reaching 12 pixels; column 15, twelve right of the square, is "
                       "lit and column 16 is clear.", case(4)),
    "FX-FASTBOX-002": ("Radius 4, iterations 1: one plain box of 9 pixels, flat-topped, ending "
                       "hard 4 pixels right of the square.", case(4, iterations=1)),
    "FX-FASTBOX-003": ("Radius 2.5, iterations 1: the box of 5 pixels and half of the next on "
                       "each side.", case(2.5, iterations=1)),
    "FX-FASTBOX-004": ("FX-FASTBOX-001 with edges repeat: the square's orange is read past the "
                       "left edge, and the layer does not grow.", case(4, edges="repeat")),
    "FX-FASTBOX-005": ("FX-FASTBOX-001 with Blur Dimensions horizontal: spread across only.",
                       case(4, dimensions="horizontal")),
    "FX-FASTBOX-006": ("Radius 0: the drawing untouched.", case(0)),
    "FX-FASTBOX-007": ("A Float composition: radius 2, iterations 3, then Exposure +6. The light "
                       "falls away to nothing 6 pixels right of the square and no further, "
                       "where a Gaussian's tail goes on.", case(2, stops=6, float_depth=True)),
    "FX-FASTBOX-008": ("Iterations 2.7: its whole part, 2, counted.", case(4, iterations=2.7)),
}

INVALID = {
    "FX-FASTBOX-009": ("Iterations 0, below 1.", case(4, iterations=0)),
    "FX-FASTBOX-010": ("Radius 501, above 500.", case(501)),
    "FX-FASTBOX-011": ("Blur Dimensions \"Both\": the word is exact, so a capital is not it.",
                       case(4, dimensions="Both")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = B.project_json(fx, B.case(1, units=None, stops=c["stops"], float_depth=c["float"]))
    params = {"radius": setting_json(c["radius"]), "iterations": setting_json(c["iterations"])}
    for word in ("edges", "dimensions"):
        if c[word] is not None:
            params[word] = c[word]
    p["compositions"][0]["layers"][0]["effects"][0] = {
        "instance_id": "fx-0-0", "type_id": "core.fast_box_blur", "enabled": True,
        "parameters": params}
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
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": B.plain(), "4": B.plain()},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_fast_box_blur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731

    # The composed kernel: sums to one, symmetric, reaching iterations x ceil(radius).
    for r, n in ((4, 3), (2.5, 1), (2.5, 3), (0, 3), (1, 50)):
        k = taps(r, n)
        assert abs(sum(k) - 1) < 1e-12 and k == k[::-1] and len(k) == 2 * n * math.ceil(r) + 1
    assert taps(4, 1) == [1 / 9] * 9
    # Radius whole: the half-weight ends vanish, so 2.0 is 2 exactly.
    assert box(2.0) == [0.2] * 5 and abs(box(2.5)[0] - 0.5 / 6) < 1e-15
    one = c["FX-FASTBOX-001"]
    # Square ends at column 3: 3 + 12 = 15 is the last lit column, 16 clear, on every row.
    assert one[at(15, 6)][3] > 0 and all(one[at(x, y)] == [0.0] * 4
                                         for x in range(16, W) for y in range(H))
    # One box: flat, the square's 4 columns over 9, from column 0 to 3 + 4 = 7, then nothing.
    two = c["FX-FASTBOX-002"]
    assert two[at(7, 6)][3] > 0 and two[at(8, 6)] == [0.0] * 4
    # Row 6 at columns 3 to 7 sees 4, 4, 3, 2, 1 of the square's columns, over 9 down too.
    edge = 4 / 9  # rows 2 to 9 reach the square's 4 rows; row 6 sees all 4 of 9
    for x, seen in zip(range(3, 8), (4, 4, 3, 2, 1)):
        assert abs(two[at(x, 6)][3] - seen / 9 * edge) < 1e-12, x
    three = c["FX-FASTBOX-003"]
    assert three[at(6, 6)][3] > 0 and three[at(7, 6)] == [0.0] * 4
    assert c["FX-FASTBOX-004"][at(0, 6)][3] > one[at(0, 6)][3] + 0.05
    five = c["FX-FASTBOX-005"]
    assert all(five[at(x, y)] == [0.0] * 4 for x in range(W) for y in (0, 1, 2, 3, 8, 9, 10, 11))
    assert five[at(6, 6)][3] > 0.01
    assert c["FX-FASTBOX-006"] == B.plain()
    # Float, Exposure +6: past white on the square, lit 6 out (column 9), dead at 10, where
    # Blurriness at the same softness (sigma 2) would still be lit.
    seven = c["FX-FASTBOX-007"]
    assert seven[at(1, 6)][0] > 1 and seven[at(9, 6)][0] > 0 and seven[at(10, 6)] == [0.0] * 4
    gauss = B.exposure(B.blur(B.plain(), 2 / 0.3, "blurriness", "transparent", "both"), 6)
    assert gauss[at(10, 6)][0] > 1e-3
    assert c["FX-FASTBOX-008"] == render(case(4, iterations=2))
    for name, px in c.items():
        for p in px:
            assert -1e-12 <= p[3] <= 1 + 1e-12, (name, p)


if __name__ == "__main__":
    main()
