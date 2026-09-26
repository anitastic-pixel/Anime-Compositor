"""Outline, worked a second way.

D-118 adds `core.outline`. It draws a band of `color` round the drawing's shape, `width` pixels
wide, softened by `softness`, at `opacity` per cent, behind the drawing, as an anime cel's
outer line or a sticker's border is. The band at a pixel is the greatest covering the drawing
has within `width` of it, counted on whole-pixel steps (a step (dx, dy) counts when
dx^2 + dy^2 <= width^2), so a width of 1 reaches the four pixels beside a pixel but not the
corners, and a width of 2.5 reaches two pixels straight and diagonally one-and-two, but not
three. The band is then blurred by document 21's Gaussian at sigma softness / 3. It goes behind
the drawing: where the drawing covers fully, it does not show; where the drawing covers part of
a pixel, it shows through the rest. The layer grows by the width rounded up and the blur's
reach, so the band round the drawing's edge is kept. It is this program's own method, modelled
on After Effects' Stroke and Photoshop's Stroke layer style; nothing is ported. Document 21 is
the rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. Width 0 changes nothing and does not grow the layer. Otherwise, with n = ceil(width),
D(P) = the greatest covering of the drawing at P + (dx, dy) over the whole steps with
dx^2 + dy^2 <= width^2, transparent outside the drawing, worked on the drawing grown by n; D is
then the Gaussian blur at s = softness / 3, growing by r = ceil(3 s) more (D itself at softness
0). With o = opacity / 100 and C the colour's linear value, the output is
I + (C * D * o, D * o) * (1 - I.a), I the drawing's pixel, transparent outside it. The layer
grows by n + r on every side.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: drop shadow's card, with room round it for the band to show in the frame. The
drawing goes into `Fixtures/outline/media`, the projects into `Fixtures/outline`, and the
expected frames into `Fixtures/outline/expected_outline.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/outline_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop, srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from edges_reference import gaussian  # noqa: E402
import drop_shadow_reference as DS  # noqa: E402
from drop_shadow_reference import pixel, frame, drawn_layer, EMPTY  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "outline"
TOLERANCE = 2e-5  # document 25's default for a filter
WIDTH, SOFTNESS, OPACITY = (0, 100), (0, 100), (0, 100)


# --- the rule -------------------------------------------------------------------------------

def steps(width):
    """The whole steps within `width` of a pixel, itself included."""
    n = math.ceil(width)
    return [(dx, dy) for dy in range(-n, n + 1) for dx in range(-n, n + 1)
            if dx * dx + dy * dy <= width * width]


def outline(layer, color, width, softness, opacity):
    """The layer with its band behind it, grown by ceil(width) + the blur's reach."""
    if width == 0:
        return layer
    n = math.ceil(width)
    near = steps(width)
    left, top = layer["left"] - n, layer["top"] - n
    w, h = layer["w"] + 2 * n, layer["h"] + 2 * n
    band = [[0.0, 0.0, 0.0, max(pixel(layer, x + dx, y + dy)[3] for dx, dy in near)]
            for y in range(top, top + h) for x in range(left, left + w)]
    D = gaussian({"px": band, "left": left, "top": top, "w": w, "h": h}, softness / 3,
                 "transparent")
    c = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    o = opacity / 100
    out = []
    for y in range(D["top"], D["top"] + D["h"]):
        for x in range(D["left"], D["left"] + D["w"]):
            d = pixel(D, x, y)[3] * o
            i = pixel(layer, x, y)
            ring = [c[0] * d, c[1] * d, c[2] * d, d]
            out.append([i[j] + ring[j] * (1 - i[3]) for j in range(4)])
    return dict(D, px=out)


# --- the drawing ----------------------------------------------------------------------------

LINE, SOFT, SKIN = R.LINE, R.SOFT, R.SKIN
DRAWINGS = {"card": DS.DRAWINGS["card"]}


# --- the cases ------------------------------------------------------------------------------

RED_HEX = R.TRACE_HEX  # #c82828


def case(color="#ffffff", width=3, softness=0, opacity=100, shift=0):
    return {"drawing": "card", "color": color, "width": width, "softness": softness,
            "opacity": opacity, "shift": shift}


def held(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    at = lambda k, r: held(value_at(c[k], frame_no), r)  # noqa: E731
    return frame(outline(drawn_layer(c["drawing"]), c["color"], at("width", WIDTH),
                         at("softness", SOFTNESS), at("opacity", OPACITY)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return frame(drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-OUTLINE-001": ("The defaults: white, width 3, softness 0, opacity 100. A white band "
                       "three pixels wide round the card, its corners rounded (a pixel three "
                       "across and one down from a corner is outside it), filling rows 0 to 9; "
                       "the card itself, fully covered, is unchanged. The soft pixel takes the "
                       "white through its uncovered share and is fully covered; past it, "
                       "where only the soft pixel is within reach, as at (13, 4), the band is "
                       "only as strong as it, 128/255.",
                       case(), [0]),
    "FX-OUTLINE-002": ("Width 0: the drawing, untouched, and the layer does not grow.",
                       case(width=0), [0]),
    "FX-OUTLINE-003": ("Width 0, softness 10: still nothing.",
                       case(width=0, softness=10), [0]),
    "FX-OUTLINE-004": ("Width 1: a one-pixel band beside the card's four sides, with its four "
                       "corners empty, since a diagonal step is 1.41 away; the soft pixel is "
                       "filled to full covering, and the pixel past it, (11, 4), takes the "
                       "band at 128/255.",
                       case(width=1), [0]),
    "FX-OUTLINE-005": ("Width 2.5: steps of (2, 1) count, 2.24 away, and (2, 2) and (3, 0) do "
                       "not, 2.83 and 3 away; the layer grows by 3, but its outermost ring "
                       "stays empty.",
                       case(width=2.5), [0]),
    "FX-OUTLINE-006": ("Width 2, softness 3: the band of width 2, blurred at sigma 1, soft "
                       "at its outer edge and reaching three pixels further.",
                       case(width=2, softness=3), [0]),
    "FX-OUTLINE-007": ("Opacity 40: FX-OUTLINE-001's band at 40 per cent of its covering.",
                       case(opacity=40), [0]),
    "FX-OUTLINE-008": ("Colour #c82828, width 1: a red band; the soft pixel becomes half line "
                       "and half red, fully covered.",
                       case(color=RED_HEX, width=1), [0]),
    "FX-OUTLINE-009": ("FX-OUTLINE-008 with the colour written in capitals: the same.",
                       case(color=RED_HEX.upper(), width=1), [0]),
    "FX-OUTLINE-010": ("Width keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the "
                       "drawing, untouched; frame 2, at 1, is FX-OUTLINE-004; frame 4 is width "
                       "2, where the step (1, 1), 1.41 away, counts, and (2, 1), 2.24 away, "
                       "does not, so its corners are cut on the slant.",
                       case(width=keyed((0, 0), (4, 2))), [0, 2, 4]),
    "FX-OUTLINE-011": ("Opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end "
                       "(132.5 at frame 2): frame 0 is the drawing; frame 2 is held at 100 and "
                       "is FX-OUTLINE-001, as is frame 4.",
                       case(opacity=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-OUTLINE-012": ("Width 0.5: only the step (0, 0) counts, so the band is the drawing's "
                       "own covering and shows only through the soft pixel, whose covering "
                       "goes from 0.502 to 0.752; the layer grows by 1 with nothing in it.",
                       case(width=0.5), [0]),
    "FX-OUTLINE-013": ("Width 5: the band would reach rows -2 to 11 and is cut off by the "
                       "frame's top and bottom edges; it runs along rows 0 and 9 from column 1 "
                       "to 13, and reaches column 0 in rows 3 to 6.",
                       case(width=5), [0]),
    "FX-OUTLINE-014": ("FX-OUTLINE-001 moved three pixels right: the same, moved, and the band "
                       "past the soft pixel, which would fall in column 16, is cut off by the "
                       "frame's edge.",
                       case(shift=3), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-OUTLINE-015": ("Width 101, above 100.", case(width=101)),
    "FX-OUTLINE-016": ("Width -1, below 0.", case(width=-1)),
    "FX-OUTLINE-017": ("Softness 101, above 100.", case(softness=101)),
    "FX-OUTLINE-018": ("Opacity -1, below 0.", case(opacity=-1)),
    "FX-OUTLINE-019": ("A colour written \"#gggggg\", not hexadecimal.", case(color="#gggggg")),
    "FX-OUTLINE-020": ("Width keyed to 150 at frame 4.", case(width=keyed((0, 0), (4, 150)))),
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
        "instance_id": "fx-0-0", "type_id": "core.outline", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in ("color", "width", "softness", "opacity")}}]
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

    (OUT / "expected_outline.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    white, red = [1.0, 1.0, 1.0, 1.0], [srgb_to_linear(v / 255) for v in R.hex_color(RED_HEX)]
    soft, a = (10, 4), SOFT[3] / 255
    # The card's pixels, and each empty pixel's nearest step to one of them.
    card = [(x, y) for y in range(H) for x in range(W) if (x, y) != soft and drawn[at(x, y)][3]]
    reach = lambda x, y: min((x - u) ** 2 + (y - v) ** 2 for u, v in card)  # noqa: E731

    # The rule's own pieces.
    assert len(steps(1)) == 5 and len(steps(1.5)) == 9 and steps(0.5) == [(0, 0)]
    assert (2, 1) in steps(2.5) and (2, 2) not in steps(2.5) and (3, 0) not in steps(2.5)
    grown = outline(drawn_layer("card"), "#ffffff", 2.5, 3, 100)
    assert (grown["left"], grown["top"], grown["w"]) == (-6, -6, W + 12)
    assert outline(drawn_layer("card"), "#ffffff", 0, 10, 100) == drawn_layer("card")
    ring = outline(drawn_layer("card"), "#ffffff", 2.5, 0, 100)
    assert all(p == EMPTY for p in ring["px"][:ring["w"]])  # the outermost ring stays empty

    for name, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
                if name != "FX-OUTLINE-014" and drawn[i][3] == 1:
                    assert p == drawn[i], name  # the card, fully covered, never changes

    def band(f, width, color=white, k=1.0):
        """Every empty pixel within the width of the card is the colour at k, the rest empty
        (the pixels near the soft pixel aside)."""
        for y in range(H):
            for x in range(W):
                if drawn[at(x, y)][3] == 0 and (x - soft[0]) ** 2 + (y - soft[1]) ** 2 > width ** 2:
                    want = [v * k for v in color] if reach(x, y) <= width ** 2 else EMPTY
                    assert near(f[at(x, y)], want), (x, y)
        return True

    first = c["FX-OUTLINE-001"]["0"]
    assert band(first, 3) and first[at(2, 4)] == white and first[at(2, 2)] == EMPTY
    assert all(first[at(7, y)] != EMPTY for y in range(H))  # rows 0 to 9
    assert near(first[at(*soft)], [v + 1 - a for v in drawn[at(*soft)][:3]] + [1.0])
    assert near(first[at(13, 4)], [a] * 4) and first[at(14, 4)] == EMPTY
    assert c["FX-OUTLINE-002"]["0"] == drawn and c["FX-OUTLINE-003"]["0"] == drawn
    four = c["FX-OUTLINE-004"]["0"]
    assert band(four, 1) and four[at(4, 2)] == EMPTY and four[at(4, 3)] == white
    assert abs(four[at(*soft)][3] - 1) < 1e-12 and near(four[at(11, 4)], [a] * 4)
    five = c["FX-OUTLINE-005"]["0"]
    assert band(five, 2.5) and five[at(3, 2)] == white and five[at(3, 1)] == EMPTY
    assert five[at(2, 4)] == EMPTY and five[at(3, 4)] == white
    six = c["FX-OUTLINE-006"]["0"]
    assert 0 < six[at(0, 4)][3] < six[at(1, 4)][3] < six[at(2, 4)][3] < 1
    assert c["FX-OUTLINE-010"]["4"] != six and c["FX-OUTLINE-010"]["4"][at(1, 4)] == EMPTY
    seven = c["FX-OUTLINE-007"]["0"]
    assert band(seven, 3, k=0.4) and seven[at(*soft)][3] == a + 0.4 * (1 - a)
    eight = c["FX-OUTLINE-008"]["0"]
    assert band(eight, 1, red + [1.0])
    assert near(eight[at(*soft)], [s + r * (1 - a) for s, r in zip(drawn[at(*soft)], red)]
                + [1.0])
    assert c["FX-OUTLINE-009"]["0"] == eight
    ten = c["FX-OUTLINE-010"]
    assert ten["0"] == drawn and ten["2"] == four and band(ten["4"], 2)
    assert ten["4"][at(3, 2)] == EMPTY and ten["4"][at(3, 3)] == white
    eleven = c["FX-OUTLINE-011"]
    assert eleven["0"] == drawn and eleven["2"] == first == eleven["4"]
    twelve = c["FX-OUTLINE-012"]["0"]
    assert [i for i in range(W * H) if twelve[i] != drawn[i]] == [at(*soft)]
    assert twelve[at(*soft)][3] == a + a * (1 - a)
    thirteen = c["FX-OUTLINE-013"]["0"]
    assert band(thirteen, 5) and thirteen[at(7, 0)] == white == thirteen[at(7, 9)]
    assert [y for y in range(H) if thirteen[at(0, y)] == white] == [3, 4, 5, 6]
    for y in (0, 9):
        assert [x for x in range(W) if thirteen[at(x, y)] != EMPTY] == list(range(1, 14))
    moved = c["FX-OUTLINE-014"]["0"]
    assert all(moved[at(x, y)] == first[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    assert first[at(13, 4)] != EMPTY  # 13 + 3 = 16 is cut off
    print("checked")


if __name__ == "__main__":
    main()
