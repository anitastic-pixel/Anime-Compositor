"""Find Edges, worked a second way.

D-146 adds `core.find_edges`. It turns a drawing into a sketch of its own edges, as a line test
pulls the lines out of a painted cel: where the picture is flat it goes white, and where it
changes sharply from one pixel to the next it goes dark, so every line, every boundary between
two colours and the drawing's own outline against transparency comes out as a dark line on
white. `invert`, "off" or "on" (off), turns that around to light lines on black. `amount`, 0 to
100 (100), is how far each pixel goes from its own colour to the sketch: 100 all the way, 50 half,
0 not at all. The covering is kept, so a soft edge stays soft, and a pixel that does not show
stays as it is. It is this program's own method, modelled on After Effects' Find Edges; nothing
is ported. Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it.

The rule. At amount 0 the output is the input, exactly. The picture luma of a premultiplied
pixel p is y(p) = linear_to_srgb(clamp(0.2126 p.r + 0.7152 p.g + 0.0722 p.b, 0, 1)), the colour
over black, so a transparent pixel's luma is 0. Yp(x, y) is the luma of the buffer's pixel
(clamp(x, 0, w - 1), clamp(y, 0, h - 1)): past the buffer's border the border pixel repeats.
Sobel: gx = Yp(x+1, y-1) + 2 Yp(x+1, y) + Yp(x+1, y+1) - Yp(x-1, y-1) - 2 Yp(x-1, y) -
Yp(x-1, y+1), gy the same with rows for columns (below minus above), m = min(1, sqrt(gx^2 +
gy^2) / 2), and v = 1 - m (off) or m (on). At a pixel with covering a > 0, with
e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) its encoded straight colour, each channel
e'_c = e_c + amount / 100 (v - e_c), and the output is (srgb_to_linear(clamp(e'_c, 0, 1)) * a,
a). A pixel with a = 0 stays as it is. The layer does not grow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: bands of colour with a line, a shade and a soft edge, drawn below, or a flat skin
card that fills the frame. The drawings go into `Fixtures/find_edges/media`, the projects into
`Fixtures/find_edges`, and the expected frames into `Fixtures/find_edges/expected_find_edges.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/find_edges_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "find_edges"
TOLERANCE = 2e-5  # document 25's default for a filter
NAMES = ("invert", "amount")


# --- the rule -------------------------------------------------------------------------------

def luma(w):
    """The picture luma of a working pixel: its colour over black, encoded."""
    return S.linear_to_srgb(min(1.0, max(0.0, 0.2126 * w[0] + 0.7152 * w[1] + 0.0722 * w[2])))


def magnitude(lum, x, y, w, h):
    """m at (x, y) from the lumas `lum`, a w by h buffer, its border repeated."""
    Y = lambda i, j: lum[min(max(j, 0), h - 1) * w + min(max(i, 0), w - 1)]  # noqa: E731
    gx = ((Y(x + 1, y - 1) + 2 * Y(x + 1, y) + Y(x + 1, y + 1))
          - (Y(x - 1, y - 1) + 2 * Y(x - 1, y) + Y(x - 1, y + 1)))
    gy = ((Y(x - 1, y + 1) + 2 * Y(x, y + 1) + Y(x + 1, y + 1))
          - (Y(x - 1, y - 1) + 2 * Y(x, y - 1) + Y(x + 1, y - 1)))
    return min(1.0, math.sqrt(gx * gx + gy * gy) / 2)


def find_edges(pixels, invert, amount, w=W, h=H):
    """The drawing's 8-bit pixels (w by h, row by row), through Find Edges, as working values."""
    src = [R.working(p) for p in pixels]
    t = amount / 100
    if t == 0:
        return src  # the build exits early
    lum = [luma(p) for p in src]
    out = [list(p) for p in src]
    for y in range(h):
        for x in range(w):
            o = out[y * w + x]
            a = o[3]
            if a <= 0:
                continue
            m = magnitude(lum, x, y, w, h)
            v = m if invert == "on" else 1 - m
            for c in range(3):
                e = S.linear_to_srgb(min(1.0, max(0.0, o[c] / a)))
                o[c] = srgb_to_linear(min(1.0, max(0.0, e + t * (v - e)))) * a
    return out


# --- the drawings ---------------------------------------------------------------------------

LINE = R.LINE                    # #1e1a24, the line
TRACE = R.TRACE                  # #c82828, a red colour-trace line
SKIN = R.SKIN                    # #f6d6be
SHADE = (220, 160, 140, 255)     # #dca08c, the skin's shadow tone
BLUE = (58, 111, 216, 255)       # #3a6fd8, the ball's blue band
WHITE = (255, 255, 255, 255)
SOFT = (246, 214, 190, 128)      # the skin at half covering, a soft edge
FAINT = (246, 214, 190, 64)      # the skin at a quarter covering, the edge's outer ring
NONE = S.NONE


def bands(x, y):
    """Rows 0 and 9 and column 0 are empty. Between, rows 1 to 8: skin in columns 1 to 5, the
    line in column 6, skin in columns 7 to 9 of rows 1 to 4 over its shade in rows 5 to 8, the
    red trace in column 10, blue in 11 to 13, white in 14, and in column 15 the skin at half
    covering in rows 1 to 4 and at a quarter in rows 5 to 8."""
    if y in (0, 9) or x == 0:
        return NONE
    if x <= 5:
        return SKIN
    if x == 6:
        return LINE
    if x <= 9:
        return SKIN if y <= 4 else SHADE
    if x == 10:
        return TRACE
    if x <= 13:
        return BLUE
    if x == 14:
        return WHITE
    return SOFT if y <= 4 else FAINT


DRAWINGS = {"bands": [[bands(x, y) for x in range(W)] for y in range(H)],
            "card": [[SKIN] * W for _ in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(invert="off", amount=100, shift=0, drawing="bands"):
    return {"drawing": drawing, "invert": invert, "amount": amount, "shift": shift}


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    amount = min(100, max(0, value_at(c["amount"], frame_no)))
    return R.frame(find_edges(pixels, c["invert"], amount), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(amount=0, shift=c["shift"], drawing=c["drawing"]), 0)


CASES = {
    "FX-FINDEDGES-001": ("Invert off, amount 100, the settings as they start: dark lines on "
                         "white. Where the picture is flat, inside the skin and the blue, the "
                         "pixel turns pure white; the drawing's own outline against the empty "
                         "rows and column turns black; the line, one pixel wide in column 6, "
                         "comes out as two black lines either side of it, its own middle white in "
                         "rows 2 and 3, where the skin either side matches (the rule looks at a "
                         "pixel's neighbours, not the pixel), and grey lower down, where the "
                         "shade begins on one side; the skin meets its shade in a grey edge; the "
                         "blue beside the red trace barely darkens; every shown pixel turns a "
                         "grey, at its own covering, the soft edge keeping its covering, and the "
                         "empty pixels stay empty.",
                         case(), [0]),
    "FX-FINDEDGES-002": ("Amount 0: the drawing, untouched.",
                         case(amount=0), [0]),
    "FX-FINDEDGES-003": ("Invert on, amount 100: light lines on black, FX-FINDEDGES-001 turned "
                         "round: every shown pixel's grey is 1 minus FX-FINDEDGES-001's, so the "
                         "flat skin and blue turn black and the lines and the outline white or "
                         "nearly white.",
                         case(invert="on"), [0]),
    "FX-FINDEDGES-004": ("Amount 50: every shown pixel goes half way from its own colour to "
                         "FX-FINDEDGES-001's grey, in encoded values: the flat skin half way to "
                         "white, the line half way to black.",
                         case(amount=50), [0]),
    "FX-FINDEDGES-005": ("Invert on, amount 50: half way from each pixel's colour to "
                         "FX-FINDEDGES-003's grey.",
                         case(invert="on", amount=50), [0]),
    "FX-FINDEDGES-006": ("Amount 25: a quarter of the way to FX-FINDEDGES-001's grey; the "
                         "drawing's colours still show.",
                         case(amount=25), [0]),
    "FX-FINDEDGES-007": ("Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 "
                         "untouched, frame 2 FX-FINDEDGES-004 and frame 4 FX-FINDEDGES-001.",
                         case(amount=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-FINDEDGES-008": ("Invert on, amount keyed from 100 at frame 0 to 0 at frame 4, linear: "
                         "frame 0 FX-FINDEDGES-003, frame 2 FX-FINDEDGES-005 and frame 4 "
                         "untouched.",
                         case(invert="on", amount=keyed((0, 100), (4, 0))), [0, 2, 4]),
    "FX-FINDEDGES-009": ("Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                         "overshoots: at frame 2 it would pass 100, is held at 100, and is "
                         "FX-FINDEDGES-001, as is frame 4; frame 0 is the drawing.",
                         case(amount=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-FINDEDGES-010": ("Invert on, amount 0: the drawing, untouched.",
                         case(invert="on", amount=0), [0]),
    "FX-FINDEDGES-011": ("A flat skin card that fills the whole drawing, invert off: nothing "
                         "changes from one pixel to the next, and past the drawing's border its "
                         "border pixels repeat, so the border is no edge: every pixel turns pure "
                         "white.",
                         case(drawing="card"), [0]),
    "FX-FINDEDGES-012": ("The same card, invert on: every pixel turns pure black.",
                         case(drawing="card", invert="on"), [0]),
    "FX-FINDEDGES-013": ("FX-FINDEDGES-001 moved three pixels right: the same, moved; the three "
                         "columns on the left, outside the layer, stay empty.",
                         case(shift=3), [0, 3]),
    "FX-FINDEDGES-014": ("FX-FINDEDGES-003 moved three pixels right: the same, moved; the columns "
                         "outside the layer stay empty, not black.",
                         case(invert="on", shift=3), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-FINDEDGES-015": ("Amount 101, above 100.", case(amount=101)),
    "FX-FINDEDGES-016": ("Amount -1, below 0.", case(amount=-1)),
    "FX-FINDEDGES-017": ("Invert \"yes\", which is not a choice.", case(invert="yes")),
    "FX-FINDEDGES-018": ("Invert \"ON\", in capitals, which is kept as written and is not the "
                         "word.", case(invert="ON")),
    "FX-FINDEDGES-019": ("Amount keyed to 150 at frame 4.",
                         case(amount=keyed((0, 0), (4, 150)))),
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
        "instance_id": "fx-0-0", "type_id": "core.find_edges", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
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

    (OUT / "expected_find_edges.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    src = [p for row in DRAWINGS["bands"] for p in row]
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    shown = [i for i in range(W * H) if src[i][3] > 0]
    enc = lambda p: [S.linear_to_srgb(u / p[3]) for u in p[:3]]  # noqa: E731

    # The rule's pieces, worked by hand from the 8-bit values: the luma of a colour over black,
    # and m at three pixels. Skin over its shade (column 8, rows 4 and 5): the columns either
    # side match, gx = 0, and gy = 4 (y(shade) - y(skin)), so m = 2 |y(skin) - y(shade)|,
    # below 1. The line is far darker than the skin: m is held at 1.
    y8 = lambda p: S.linear_to_srgb(sum(k * srgb_to_linear(p[i] / 255) * p[3] / 255  # noqa: E731
                                        for i, k in enumerate((0.2126, 0.7152, 0.0722))))
    lum = [luma(R.working(p)) for p in src]
    assert all(abs(lum[i] - y8(src[i])) < 1e-12 for i in range(W * H))
    assert lum[at(0, 4)] == 0.0 and lum[at(15, 2)] < lum[at(3, 2)]  # soft skin is darker
    m_shade = 2 * (y8(SKIN) - y8(SHADE))
    assert 0.2 < m_shade < 0.5
    for y in (4, 5):
        assert abs(magnitude(lum, 8, y, W, H) - m_shade) < 1e-12
    assert magnitude(lum, 5, 3, W, H) == 1.0 and magnitude(lum, 7, 3, W, H) == 1.0
    assert magnitude(lum, 6, 3, W, H) == 0.0 and magnitude(lum, 3, 3, W, H) == 0.0
    assert 0 < magnitude(lum, 6, 6, W, H) < 1 and 0 < magnitude(lum, 11, 4, W, H) < 0.1
    # The border repeats: a pixel on the card's border has no edge; the empty row 0 is an edge
    # to the drawing's row 1.
    card = [R.working(p) for row in DRAWINGS["card"] for p in row]
    card_lum = [luma(p) for p in card]
    assert all(magnitude(card_lum, x, y, W, H) == 0.0 for y in range(H) for x in range(W))
    assert magnitude(lum, 3, 1, W, H) == 1.0

    # Every case keeps every pixel's covering and leaves the empty pixels empty.
    for fx, frames in c.items():
        n = int(fx[-3:])
        base = plain(case(shift=3 if n in (13, 14) else 0,
                          drawing="card" if n in (11, 12) else "bands"))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= u <= px[i][3] + 1e-12 for u in px[i][:3]), (fx, i)

    # 001: every shown pixel a grey 1 - m at its own covering; flat pixels white; the one-pixel
    # line black either side and white in its own middle where the skin either side matches;
    # the skin over its shade a grey 1 - m_shade; the blue beside the trace barely darker; the
    # soft edge keeping its covering.
    one = c["FX-FINDEDGES-001"]["0"]
    for i in shown:
        e = enc(one[i])
        assert near(e, [e[0]] * 3)
        assert abs(e[0] - (1 - magnitude(lum, i % W, i // W, W, H))) < 1e-9
    for x, y in ((3, 3), (3, 4), (12, 3), (12, 6), (6, 2), (6, 3)):
        assert near(one[at(x, y)], [1, 1, 1, 1]), (x, y)
    for x, y in ((3, 1), (3, 8), (1, 4), (5, 4), (7, 4)):  # the outline and the line's sides
        assert near(one[at(x, y)], [0, 0, 0, 1]), (x, y)
    for y in range(4, 8):  # the line's middle grey where the shade begins on one side
        assert 0.5 < enc(one[at(6, y)])[0] < 1
    for y in (4, 5):
        assert near(enc(one[at(8, y)]), [1 - m_shade] * 3)
    for y in range(2, 8):
        assert 0.9 < enc(one[at(11, y)])[0] < 1
    assert one[at(15, 2)][3] == 128 / 255 and one[at(15, 6)][3] == 64 / 255
    assert c["FX-FINDEDGES-002"]["0"] == drawn and c["FX-FINDEDGES-010"]["0"] == drawn

    # 003 is 001 turned round; 004 to 006 are part of the way from the colour to the grey.
    three = c["FX-FINDEDGES-003"]["0"]
    for i in shown:
        assert near(enc(three[i]), [1 - u for u in enc(one[i])])
    assert near(three[at(3, 3)], [0, 0, 0, 1]) and near(three[at(5, 4)], [1, 1, 1, 1])
    for fx, full, t in (("FX-FINDEDGES-004", one, 0.5), ("FX-FINDEDGES-005", three, 0.5),
                        ("FX-FINDEDGES-006", one, 0.25)):
        px = c[fx]["0"]
        for i in shown:
            want = [e + t * (v - e) for e, v in zip(enc(drawn[i]), enc(full[i]))]
            assert near(enc(px[i]), want), (fx, i)

    seven = c["FX-FINDEDGES-007"]
    assert seven["0"] == drawn and seven["2"] == c["FX-FINDEDGES-004"]["0"] and seven["4"] == one
    eight = c["FX-FINDEDGES-008"]
    assert eight["0"] == three and eight["2"] == c["FX-FINDEDGES-005"]["0"] and eight["4"] == drawn
    nine = c["FX-FINDEDGES-009"]
    assert ease(OVERSHOOT, 0.5) > 1 and value_at(keyed((0, 0, OVERSHOOT), (4, 100)), 2) > 100
    assert nine["0"] == drawn and nine["2"] == one and nine["4"] == one

    assert all(near(p, [1, 1, 1, 1]) for p in c["FX-FINDEDGES-011"]["0"])
    assert all(near(p, [0, 0, 0, 1]) for p in c["FX-FINDEDGES-012"]["0"])

    moved = c["FX-FINDEDGES-013"]["0"]
    assert moved == c["FX-FINDEDGES-013"]["3"]
    shifted = c["FX-FINDEDGES-014"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == one[at(0, y):at(W - 3, y)]
        assert shifted[at(3, y):at(0, y + 1)] == three[at(0, y):at(W - 3, y)]
        assert all(shifted[at(x, y)] == [0.0] * 4 for x in range(3))
    print("checked")


if __name__ == "__main__":
    main()
