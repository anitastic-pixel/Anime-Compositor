"""Chromatic aberration, worked a second way.

D-120 adds `core.chromatic_aberration`. It splits a layer's red and blue apart about a centre,
as a cheap lens does: red is drawn a little larger and blue a little smaller than the green,
so edges away from the centre gain a red fringe on one side and a blue one on the other, the
further out the wider. `amount` is 0 to 100 (3), a distance: how many pixels the red and the
blue each move at the drawing's corner, half its diagonal from the middle. `center` is two
numbers, per cent of the drawing's width and height, 50, 50 its middle, as Radial Blur's centre
is. It is this program's own method, modelled on After Effects' lens effects (its Optics
Compensation and the channel offsets of its colour split); nothing is ported. Document 21 is
the rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. With c the centre in layer pixels, (center_x / 100 * W0, center_y / 100 * H0) of the
drawing's own size W0 by H0, R = sqrt(W0^2 + H0^2) / 2 and k = amount / R: at the pixel whose
centre is P, d = P - c; red is the red of document 21's bilinear sample of the layer at
c + d (1 - k), blue is the blue of the sample at c + d (1 + k), green is the pixel's own, and
the covering is the largest of the two samples' coverings and the pixel's own. All are
premultiplied. The layer does not grow: a fringe that would fall outside the layer is cut, and
a sample outside it reads transparent. Amount 0 changes nothing.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a figure, drawn below. The drawing goes into
`Fixtures/chromatic_aberration/media`, the projects into `Fixtures/chromatic_aberration`, and
the expected frames into `Fixtures/chromatic_aberration/expected_chromatic_aberration.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/chromatic_aberration_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "chromatic_aberration"
TOLERANCE = 2e-5  # document 25's default for a filter
AMOUNT = (0, 100)
CENTER = (-1000, 1000)


# --- the rule -------------------------------------------------------------------------------

def aberration(layer, amount, center, size=(W, H)):
    """The layer's pixels, split. `size` is the drawing's own W0 by H0."""
    w0, h0 = size
    cx, cy = center[0] / 100 * w0, center[1] / 100 * h0
    k = amount / (math.sqrt(w0 * w0 + h0 * h0) / 2)
    out = []
    for i, p in enumerate(layer["px"]):
        dx = layer["left"] + i % layer["w"] + 0.5 - cx
        dy = layer["top"] + i // layer["w"] + 0.5 - cy
        red = bilinear(layer, cx + dx * (1 - k), cy + dy * (1 - k))
        blue = bilinear(layer, cx + dx * (1 + k), cy + dy * (1 + k))
        out.append([red[0], p[1], blue[2], max(red[3], p[3], blue[3])])
    return out


# --- the drawing ----------------------------------------------------------------------------

WHITE = (255, 255, 255, 255)
LINE = R.LINE                         # #1e1a24
SKIN = R.SKIN                         # #f6d6be
SOFT = SKIN[:3] + (128,)              # the skin at half covering, a soft edge
NONE = S.NONE


def figure(x, y):
    """A white block in columns 0 to 5 and rows 2 to 7, against the drawing's left edge, closed
    by a line down column 6; a skin block in columns 9 to 12 and rows 3 to 6 with a
    half-covering edge down column 13. Everything else is empty."""
    if 2 <= y <= 7 and x <= 5:
        return WHITE
    if 2 <= y <= 7 and x == 6:
        return LINE
    if 3 <= y <= 6 and 9 <= x <= 12:
        return SKIN
    if 3 <= y <= 6 and x == 13:
        return SOFT
    return NONE


DRAWINGS = {"figure": [[figure(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(amount=3, center=(50, 50), shift=0):
    return {"drawing": "figure", "amount": amount, "center": center, "shift": shift}


def clamp(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    amount = clamp(value_at(c["amount"], frame_no), AMOUNT)
    center = [clamp(v, CENTER) for v in value_at(c["center"], frame_no)]
    return R.frame(aberration(layer, amount, center), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(amount=0, shift=c["shift"]), 0)


CASES = {
    "FX-CHROMA-001": ("The settings as they start, amount 3 about the middle: red fringes "
                      "outward past the blocks' outer edges, where the red sample alone covers "
                      "and the pixel takes its covering, and blue fringes inward; the white "
                      "block's left column, on the layer's edge, loses its blue, whose sample "
                      "falls outside the layer, and turns yellow, and nothing is drawn beyond "
                      "the layer.",
                      case(), [0]),
    "FX-CHROMA-002": ("Amount 0: the drawing, untouched.",
                      case(amount=0), [0]),
    "FX-CHROMA-003": ("Amount 6: wider fringes than FX-CHROMA-001; the half-covering edge "
                      "keeps at least its own covering.",
                      case(amount=6), [0]),
    "FX-CHROMA-004": ("Centre 0, 50, the middle of the left edge: the split runs away from "
                      "there, so red spreads right past both blocks' right sides, and the "
                      "white block's left column, beside the centre, keeps its blue, whose "
                      "sample now lands inside the layer.",
                      case(center=(0, 50)), [0]),
    "FX-CHROMA-005": ("Amount keyed from 0 at frame 0 to 12 at frame 4, linear: frame 0 "
                      "untouched, frame 2 is FX-CHROMA-003, frame 4 splits at 12, past the "
                      "drawing's half diagonal, 9.43, so its red is turned over as "
                      "FX-CHROMA-008's is.",
                      case(amount=keyed((0, 0), (4, 12))), [0, 2, 4]),
    "FX-CHROMA-006": ("Centre keyed from 50, 50 at frame 0 to 0, 50 at frame 4: frame 0 is "
                      "FX-CHROMA-001, frame 2 splits about 25, 50, frame 4 is FX-CHROMA-004.",
                      case(center=keyed((0, (50, 50)), (4, (0, 50)))), [0, 2, 4]),
    "FX-CHROMA-007": ("FX-CHROMA-001 moved three pixels right: the same, moved; the three "
                      "columns left of the drawing stay empty, as the layer does not grow.",
                      case(shift=3), [0, 3]),
    "FX-CHROMA-008": ("Amount 100, the most: k is past 10, so the red picture is turned over "
                      "through the centre and shrunk nearly tenfold and the blue one grown "
                      "nearly twelvefold, and on a drawing this small both samples fall off it "
                      "for every pixel: each keeps only its own green, at its own covering, as "
                      "in FX-CHROMA-009.",
                      case(amount=100), [0]),
    "FX-CHROMA-009": ("Centre -1000, -1000, far off the top left: both samples fall off the "
                      "layer for every pixel, so each pixel keeps only its own green, at its "
                      "own covering.",
                      case(center=(-1000, -1000)), [0]),
    "FX-CHROMA-010": ("Amount 1: fringes a pixel or less wide at the blocks' edges; the "
                      "insides of both blocks, whose samples all land inside them, keep their "
                      "colour.",
                      case(amount=1), [0]),
    "FX-CHROMA-011": ("Centre 100, 100, the bottom right corner: the red picture is drawn "
                      "larger away from that corner, up and left, so red reaches rows 0 and 1 "
                      "above the blocks; the blue picture is drawn smaller toward it, so blue "
                      "reaches row 8 below the white block; and the white block's left column "
                      "loses its blue.",
                      case(center=(100, 100)), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-CHROMA-012": ("Amount 101, above 100.", case(amount=101)),
    "FX-CHROMA-013": ("Amount -1, below 0.", case(amount=-1)),
    "FX-CHROMA-014": ("Amount keyed to 150 at frame 4.", case(amount=keyed((0, 3), (4, 150)))),
    "FX-CHROMA-015": ("Centre 1001, 50, past ten widths.", case(center=(1001, 50))),
    "FX-CHROMA-016": ("Centre 50, -1001, past ten heights.", case(center=(50, -1001))),
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
        "instance_id": "fx-0-0", "type_id": "core.chromatic_aberration", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in ("amount", "center")}}]
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

    (OUT / "expected_chromatic_aberration.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                            encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b: all(abs(p - q) < 1e-12 for u, v in zip(a, b)  # noqa: E731
                            for p, q in zip(u, v))
    spread = lambda f: sum(f[i] != drawn[i] for i in range(W * H))  # noqa: E731
    one = c["FX-CHROMA-001"]["0"]

    # The rule's own pieces: at amount 0 both samples are the pixel's own centre.
    layer = {"px": drawn, "left": 0, "top": 0, "w": W, "h": H}
    assert aberration(layer, 0, (50, 50)) == drawn
    assert c["FX-CHROMA-002"]["0"] == drawn
    for fx, frames in c.items():
        for px in frames.values():
            for i, p in enumerate(px):
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (fx, i)
    # Green is always the pixel's own, moved as the case moves it.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-CHROMA-007" else 0))
        for px in frames.values():
            assert all(px[i][1] == base[i][1] for i in range(W * H)), fx
    # Red outward: above the white block's top, row 1 was empty and now has red alone, with the
    # red sample's covering; below the skin block too.
    for x, y in ((2, 1), (11, 7)):
        p = one[at(x, y)]
        assert drawn[at(x, y)][3] == 0 and p[0] > 0 and p[1] == 0 and p[2] == 0 and p[3] >= p[0]
    # Blue inward: the white block's outer row loses some blue, its sample reaching the empty row.
    assert one[at(2, 2)][2] < drawn[at(2, 2)][2] and one[at(2, 2)][0] == drawn[at(2, 2)][0]
    # The left edge: blue's sample falls outside the layer, and the column turns yellow.
    for y in range(3, 7):
        assert one[at(0, y)][:2] == [1.0, 1.0] and one[at(0, y)][2] == 0 and one[at(0, y)][3] == 1
    # The covering is the largest of the three: never less than the pixel's own.
    for fx in ("FX-CHROMA-001", "FX-CHROMA-003", "FX-CHROMA-008"):
        assert all(c[fx]["0"][i][3] >= drawn[i][3] for i in range(W * H))
    three = c["FX-CHROMA-003"]["0"]
    assert spread(three) > spread(one) and three[at(13, 4)][3] >= drawn[at(13, 4)][3]
    # About the left edge's middle: red runs right, and the left column keeps its blue.
    four = c["FX-CHROMA-004"]["0"]
    for y in range(3, 7):
        assert abs(four[at(0, y)][2] - 1) < 1e-12 and one[at(0, y)][2] == 0
    for x in (7, 14):
        assert drawn[at(x, 4)][3] == 0 and four[at(x, 4)][0] > 0.5
    five, six = c["FX-CHROMA-005"], c["FX-CHROMA-006"]
    assert five["0"] == drawn and five["2"] == three
    assert 12 / (math.sqrt(W * W + H * H) / 2) > 1  # frame 4's red sample is past the centre
    assert near(five["4"], render(case(amount=12), 0))
    assert six["0"] == one and six["4"] == four
    assert near(six["2"], render(case(center=(25, 50)), 0))
    moved = c["FX-CHROMA-007"]
    assert moved["0"] == moved["3"]
    assert all(moved["0"][at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved["0"][at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    # The most: k past 10, and the frame is FX-CHROMA-009's: green alone.
    assert 100 / (math.sqrt(W * W + H * H) / 2) > 10
    nine = c["FX-CHROMA-009"]["0"]
    assert c["FX-CHROMA-008"]["0"] == nine
    assert all(nine[i] == [0.0, drawn[i][1], 0.0, drawn[i][3]] for i in range(W * H))
    # A pixel's worth: the insides keep their colour, the edges change.
    ten = c["FX-CHROMA-010"]["0"]
    for x, y in ((2, 4), (3, 5), (10, 4), (11, 5)):
        assert near([ten[at(x, y)]], [drawn[at(x, y)]]), (x, y)
    assert ten[at(2, 1)][0] > 0 and ten[at(7, 4)] != drawn[at(7, 4)]
    # About the bottom right corner: red above the blocks, blue below, the left column blueless.
    eleven = c["FX-CHROMA-011"]["0"]
    for x, y in ((0, 0), (7, 1), (10, 2)):
        assert drawn[at(x, y)][3] == 0 and eleven[at(x, y)][0] > 0.5, (x, y)
    for x in range(4, 8):
        assert drawn[at(x, 8)][3] == 0 and eleven[at(x, 8)][2] > 0.4 and eleven[at(x, 8)][0] == 0
    assert eleven[at(0, 5)][2] == 0 and eleven[at(0, 5)][3] == 1
    print("checked")


if __name__ == "__main__":
    main()
