"""D-313's Channel Blur, worked a second way.

After Effects' Channel Blur blurs a layer's red, green, blue and alpha each by its own
Blurriness, with Edge Behavior (Repeat Edge Pixels) and Blur Dimensions (both, horizontal,
vertical). `core.channel_blur` has the same six settings: `red_blurriness`,
`green_blurriness`, `blue_blurriness` and `alpha_blurriness`, each 0 to 500, a Gaussian sigma
in pixels as Gaussian Blur's sigma units are (document 21); `edges`, "transparent" or
"repeat"; `dimensions`, "both", "horizontal" or "vertical".

The rule, document 21's words. With G_s the Gaussian blur at sigma s with those edges and
dimensions: A = G_alpha(p). For each colour c whose sigma differs from the alpha's,
B = G_c(p), and the output's channel c is B_c / B_a * A_a where B_a > 0, else A_c. A colour
whose sigma is the alpha's is A_c, so four the same are Gaussian Blur. Blurred premultiplied,
as every blur here: a colour is blurred with its own covering and laid back inside the alpha's.

Every case is a composition 40 by 12 holding one drawing the same size, `pair`: an orange
square #ff8000 from (0, 4) to (3, 7), against the left edge, a green square #28c850 touching it
from (4, 4) to (7, 7), and a blue square #3c78f0 at half covering from (26, 4) to (29, 7), clear
round them. The drawing goes into
`Fixtures/channel_blur/media`, the projects into `Fixtures/channel_blur`, and the expected
frames into `Fixtures/channel_blur/expected_channel_blur.json`.

**This file never runs the build's code path.** It works in double precision on lists, from the
drawing's 8-bit values, summing each two-dimensional kernel directly at each pixel
(`blurriness_reference.blur`, in sigma units), where the build blurs in single precision one
axis at a time on a buffer grown by the kernel.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/channel_blur_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import blurriness_reference as B  # noqa: E402
import directional_blur_reference as D  # noqa: E402
import smooth_reference as S  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "channel_blur"
TOLERANCE = 2e-5  # document 25's default for a filter
NAMES = ("red_blurriness", "green_blurriness", "blue_blurriness", "alpha_blurriness")
ORANGE, GREEN, BLUE = (255, 128, 0, 255), (40, 200, 80, 255), (60, 120, 240, 128)
NONE = (0, 0, 0, 0)


# --- the rule -------------------------------------------------------------------------------

def channel_blur(px, sigmas, edges, dimensions):
    """`sigmas` red, green, blue, alpha."""
    A = B.blur(px, sigmas[3], "sigma", edges, dimensions)
    out = [list(p) for p in A]
    for c in range(3):
        if sigmas[c] == sigmas[3]:
            continue
        G = B.blur(px, sigmas[c], "sigma", edges, dimensions)
        for o, g in zip(out, G):
            if g[3] > 0:
                o[c] = g[c] / g[3] * o[3]
    return out


# --- the drawing and the cases --------------------------------------------------------------

def paint(x, y):
    if 4 <= y <= 7 and x <= 3:
        return ORANGE
    if 4 <= y <= 7 and x <= 7:
        return GREEN
    if 4 <= y <= 7 and 26 <= x <= 29:
        return BLUE
    return NONE


DRAWING = [[paint(x, y) for x in range(W)] for y in range(H)]


def plain():
    return [D.working(p) for row in DRAWING for p in row]


def case(r=0, g=0, b=0, a=0, edges=None, dimensions="both"):
    """`edges` None: the file does not say, transparent."""
    return {"red_blurriness": r, "green_blurriness": g, "blue_blurriness": b,
            "alpha_blurriness": a, "edges": edges, "dimensions": dimensions}


def render(c, frame_no=0):
    sigmas = [value_at(c[k], frame_no) for k in NAMES]
    return channel_blur(plain(), sigmas, c["edges"] or "transparent", c["dimensions"])


CASES = {
    "FX-CHBLUR-001": ("Red Blurriness 3, the rest 0: only red is blurred, and laid back inside "
                      "the drawing's own covering, so green, blue and every covering are the "
                      "drawing's exactly and the empty pixels stay empty. Red softens across the "
                      "join of orange and green, the orange side losing red and the green side "
                      "gaining it; a square of one colour, as the half-covered blue one, has no "
                      "other red within reach and stays as it is, its edge against the clear "
                      "being no edge to a blur divided by its own covering.", case(r=3), [0]),
    "FX-CHBLUR-002": ("Alpha Blurriness 3, the rest 0: the covering spreads as Gaussian Blur "
                      "spreads it, and every pixel the drawing covered keeps its own straight "
                      "colour; past the drawing the spread takes the colour the blurred covering "
                      "carries, green beside green and blue beside blue, never a dark rim.",
                      case(a=3), [0]),
    "FX-CHBLUR-003": ("All four 3: Gaussian Blur at sigma 3, every channel blurred together.",
                      case(3, 3, 3, 3), [0]),
    "FX-CHBLUR-004": ("Red 2, green 0, blue 5, alpha 1: each colour spread by its own amount "
                      "and laid inside a covering spread by one pixel. Where orange meets green, "
                      "red and blue soften across the join, blue the widest, so the orange takes "
                      "a little blue and loses some red, while green stays as sharp as drawn.",
                      case(2, 0, 5, 1), [0]),
    "FX-CHBLUR-005": ("FX-CHBLUR-004 with Repeat Edge Pixels: past the left edge the orange "
                      "square's own pixels are read, so its left column keeps its covering "
                      "and its orange; the layer does not grow.",
                      case(2, 0, 5, 1, edges="repeat"), [0]),
    "FX-CHBLUR-006": ("FX-CHBLUR-004 with Blur Dimensions horizontal: spread across only, the "
                      "rows above and below the squares still empty.",
                      case(2, 0, 5, 1, dimensions="horizontal"), [0]),
    "FX-CHBLUR-007": ("Red 4 and alpha 2 with Blur Dimensions vertical and Repeat Edge Pixels: "
                      "spread down only, the columns between the green and blue squares still empty.",
                      case(r=4, a=2, edges="repeat", dimensions="vertical"), [0]),
    "FX-CHBLUR-008": ("All four 0, as it starts: the drawing untouched.", case(), [0]),
    "FX-CHBLUR-009": ("Green 1.5 and alpha 0.5: a part of a pixel, each Gaussian cut at three "
                      "of its sigmas, reaching 5 pixels and 2.", case(g=1.5, a=0.5), [0]),
    "FX-CHBLUR-010": ("Red Blurriness keyed from 0 at frame 0 to 6 at frame 4, linear: frame 0 "
                      "untouched, frame 2 red 3, FX-CHBLUR-001, and frame 4 red 6.",
                      case(r=keyed((0, 0), (4, 6))), [0, 2, 4]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-CHBLUR-011": ("Red Blurriness 501, above 500.", case(r=501)),
    "FX-CHBLUR-012": ("Alpha Blurriness -1, below 0.", case(a=-1)),
    "FX-CHBLUR-013": ("Edges \"Repeat\": the word is exact, so a capital is not it.",
                      case(r=3, edges="Repeat")),
    "FX-CHBLUR-014": ("Blur Dimensions \"diagonal\", which is not one.",
                      case(r=3, dimensions="diagonal")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = B.project_json(fx, B.case(1, units=None))
    p["assets"][0].update({"id": "asset-pair", "name": "pair", "path": "media/pair.png"})
    layer = p["compositions"][0]["layers"][0]
    layer["asset_id"] = "asset-pair"
    params = {k: setting_json(c[k]) for k in NAMES}
    if c["edges"] is not None:
        params["edges"] = c["edges"]
    params["dimensions"] = c["dimensions"]
    layer["effects"] = [{"instance_id": "fx-0-0", "type_id": "core.channel_blur",
                         "enabled": True, "parameters": params}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "pair.png").write_bytes(S.png(DRAWING))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {str(f): render(c, f) for f in frames}}
        print(f"{fx}: {says[:60]}")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": plain(), "4": plain()},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_channel_blur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    drawn = plain()
    straight = lambda p: [v / p[3] for v in p[:3]]  # noqa: E731

    # 001: green, blue and alpha exactly the drawing's; red softens across the join only.
    one = c["FX-CHBLUR-001"]["0"]
    assert all(one[i][1:] == drawn[i][1:] for i in range(W * H))
    assert all(one[i] == [0.0] * 4 for i in range(W * H) if drawn[i][3] == 0)
    assert one[at(3, 5)][0] < one[at(0, 5)][0] < drawn[at(0, 5)][0]
    assert one[at(4, 5)][0] > one[at(7, 5)][0] > drawn[at(7, 5)][0]
    assert all(near(one[at(x, y)], drawn[at(x, y)], 1e-9) for x in range(26, 30) for y in range(4, 8))
    # 002: the drawing's own straight colours where it covered; one colour past it.
    two = c["FX-CHBLUR-002"]["0"]
    assert all(near(straight(two[i]), straight(drawn[i]), 1e-9) for i in range(W * H) if drawn[i][3] > 0)
    assert near(straight(two[at(13, 5)]), straight(drawn[at(5, 5)]), 1e-9)  # green past it
    assert near(straight(two[at(21, 5)]), straight(drawn[at(27, 5)]), 1e-9)  # blue past it
    assert two[at(13, 5)][3] > 0 and two[at(21, 5)][3] > 0
    alpha_blur = B.blur(drawn, 3, "sigma", "transparent", "both")
    assert all(abs(two[i][3] - alpha_blur[i][3]) < 1e-15 for i in range(W * H))
    # 003: Gaussian Blur itself.
    assert c["FX-CHBLUR-003"]["0"] == alpha_blur
    # 004: green as drawn where drawn; blue reaches into the orange, red is pulled down.
    four = c["FX-CHBLUR-004"]["0"]
    assert all(abs(straight(four[i])[1] - straight(drawn[i])[1]) < 1e-9 for i in range(W * H) if drawn[i][3] > 0)
    assert straight(four[at(1, 5)])[2] > 1e-4 and straight(four[at(1, 5)])[0] < straight(drawn[at(1, 5)])[0] - 1e-4
    # 005: repeat holds the left edge whole; 004's left column is thinner.
    five = c["FX-CHBLUR-005"]["0"]
    assert five[at(0, 5)][3] > four[at(0, 5)][3] + 0.1
    # 006 and 007: one direction only.
    six, seven = c["FX-CHBLUR-006"]["0"], c["FX-CHBLUR-007"]["0"]
    assert all(six[at(x, y)] == [0.0] * 4 for x in range(W) for y in (0, 1, 2, 3, 8, 9, 10, 11))
    assert six[at(6, 5)][3] > 0
    assert all(seven[at(x, y)] == [0.0] * 4 for x in range(8, 26) for y in range(H))
    assert seven[at(1, 2)][3] > 0
    assert c["FX-CHBLUR-008"]["0"] == drawn
    # 009: the cut kernels' lengths.
    assert len(B.taps(1.5, "sigma")) == 11 and len(B.taps(0.5, "sigma")) == 5
    ten = c["FX-CHBLUR-010"]
    assert ten["0"] == drawn and ten["2"] == one and ten["4"] == render(case(r=6))
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12, (name, p)
                assert all(-1e-12 <= v <= p[3] + 1e-9 for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
