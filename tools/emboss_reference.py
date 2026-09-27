"""Emboss, worked a second way.

D-145 adds `core.emboss`. It makes a drawing look pressed into metal or paper: each pixel looks
a little way ahead and a little way behind along one direction, and where the picture is brighter
ahead than behind the pixel turns lighter, where darker it turns darker, and where both are alike
it turns a flat middle grey. So the flat parts of a drawing go grey and its edges stand up as
raised or sunken ridges, lit from one side. `direction` is in degrees, clockwise from up (135,
down and to the right, as it starts); `relief` 0 to 100, starting at 1, is how many pixels ahead
and behind it looks; `contrast` 0 to 1000, starting at 100, is how strongly a difference shows;
`mode` "grey" gives the grey relief alone, and "color" lays the same relief over the drawing's
own colours. The covering is kept, so a soft edge stays soft, and a pixel that does not show
stays as it is. It is this program's own method, modelled on After Effects' Emboss (grey) and
Color Emboss (color) in spirit and not claimed to match them; nothing is ported. Document 21 is
the rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. d = relief u(direction), u(θ) = (sin θ, -cos θ), exact at whole quarter turns. At a
pixel with covering a > 0, with P its centre in the layer's own space and each sample document
21's bilinear sample of the input with the point first held inside the input's pixel centres
(D-109's held sample): v = 0.5 + (y(sample at P + d) - y(sample at P - d)) contrast / 100, with
y(p) = linear_to_srgb(clamp(0.2126 p.r + 0.7152 p.g + 0.0722 p.b, 0, 1)) the picture luma of a
premultiplied pixel, its colour over black, so a drawing's outline against transparency is an
edge. Grey: every encoded channel e'_c = v; color: e'_c = e_c + v - 0.5, with
e = linear_to_srgb(clamp(p.rgb / a, 0, 1)); the output is (srgb_to_linear(clamp(e'_c, 0, 1)) * a,
a). Relief 0 or contrast 0 gives v = 0.5: flat grey in grey mode, the input in color mode. A
pixel with a = 0 stays as it is. The layer does not grow; a draft scales relief.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says, drawn below. The drawing goes into `Fixtures/emboss/media`, the projects into
`Fixtures/emboss`, and the expected frames into `Fixtures/emboss/expected_emboss.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/emboss_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
from drop_shadow_reference import unit  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "emboss"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"direction": (-3600, 3600), "relief": (0, 100), "contrast": (0, 1000)}
NAMES = ("direction", "relief", "contrast", "mode")
MODES = ("grey", "color")


# --- the rule -------------------------------------------------------------------------------

def luma(p):
    """The picture luma of a premultiplied pixel: its colour over black, encoded."""
    return S.linear_to_srgb(min(1.0, max(0.0, 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2])))


def relief_value(layer, direction, relief, contrast, x, y):
    """v at pixel (x, y) of the layer's own space."""
    ux, uy = unit(direction)
    dx, dy = relief * ux, relief * uy
    px, py = x + 0.5, y + 0.5
    ahead = luma(bilinear(layer, px + dx, py + dy, "repeat"))
    behind = luma(bilinear(layer, px - dx, py - dy, "repeat"))
    return 0.5 + (ahead - behind) * contrast / 100


def embossed(e, v, mode):
    """One encoded straight colour, embossed (before the clamp)."""
    return [v] * 3 if mode == "grey" else [c + v - 0.5 for c in e]


def emboss(layer, direction, relief, contrast, mode):
    out = []
    for y in range(layer["h"]):
        for x in range(layer["w"]):
            p = layer["px"][y * layer["w"] + x]
            a = p[3]
            if a <= 0:
                out.append(list(p))
                continue
            e = [S.linear_to_srgb(min(1.0, max(0.0, c / a))) for c in p[:3]]
            v = relief_value(layer, direction, relief, contrast, x, y)
            out.append([srgb_to_linear(min(1.0, max(0.0, c))) * a
                        for c in embossed(e, v, mode)] + [a])
    return out


# --- the drawing ----------------------------------------------------------------------------

LINE, SKIN, NONE = R.LINE, R.SKIN, S.NONE
SOFT_SKIN = (246, 214, 190, 128)  # the skin at half covering, a soft edge
BAND = (58, 111, 216, 255)        # #3a6fd8, the ball's blue band


def plate(x, y):
    """Rows 1 to 8, columns 0 to 13: skin, crossed by a blue band in rows 4 and 5, with a dark
    square of the line colour in columns 6 to 9, rows 2 to 7, over the band; the skin and the
    band touch the drawing's left edge. Column 14 is the skin at half covering; column 15 and
    rows 0 and 9 are empty."""
    if x == 15 or y in (0, 9):
        return NONE
    if x == 14:
        return SOFT_SKIN
    if 6 <= x <= 9 and 2 <= y <= 7:
        return LINE
    if y in (4, 5):
        return BAND
    return SKIN


DRAWINGS = {"plate": [[plate(x, y) for x in range(W)] for y in range(H)]}


def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


# --- the cases ------------------------------------------------------------------------------

def case(direction=135, relief=1, contrast=100, mode="grey", shift=0):
    return {"drawing": "plate", "direction": direction, "relief": relief, "contrast": contrast,
            "mode": mode, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    layer = drawn_layer(c["drawing"])
    n = [held(c, k, frame_no) for k in NAMES[:-1]]
    return R.frame(emboss(layer, *n, c["mode"]), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return R.frame(drawn_layer(c["drawing"])["px"], c["shift"])


CASES = {
    "FX-EMBOSS-001": ("The settings as they start: direction 135, relief 1, contrast 100, mode "
                      "grey. Every pixel that shows turns a grey, lighter where the picture "
                      "ahead of it (down and to the right) is brighter than behind it and "
                      "darker where it is darker: the flat skin and the middle of the square "
                      "stay a middle grey (0.5 as written). The drawing, brighter than the "
                      "emptiness around it, stands up: its top row is light, and its bottom "
                      "row and its soft right-hand column dark. The dark square and the band "
                      "sink in: their top edges go dark and their bottom edges light, and the "
                      "square's left edge dark and its right edge light. The soft column is "
                      "grey at its half covering; the empty pixels stay empty.",
                      case(), [0]),
    "FX-EMBOSS-002": ("Relief 0: each pixel looks nowhere, so every pixel that shows is the "
                      "flat middle grey (0.5 as written) at its own covering.",
                      case(relief=0), [0]),
    "FX-EMBOSS-003": ("Relief 0, mode color: the flat relief laid over the colours changes "
                      "nothing: the drawing, within a rounding step.",
                      case(relief=0, mode="color"), [0]),
    "FX-EMBOSS-004": ("Contrast 0: no difference shows, so it is FX-EMBOSS-002, flat grey.",
                      case(contrast=0), [0]),
    "FX-EMBOSS-005": ("Mode color at the start settings: the same relief as FX-EMBOSS-001 laid "
                      "over the drawing's own colours, each channel as written moved by "
                      "v - 0.5, so the flat skin and the middle of the square keep their "
                      "colours and the edges are lit or shaded.",
                      case(mode="color"), [0]),
    "FX-EMBOSS-006": ("Direction 90, relief 1: each pixel looks exactly one pixel right and "
                      "one left, v = 0.5 + y(right) - y(left), so only upright edges show: "
                      "the band, running level from the drawing's left edge, vanishes into "
                      "the grey except beside the square, whose left side goes dark and right "
                      "side light, and the drawing's soft right-hand edge goes dark. The first "
                      "column's look to the left is held at the drawing's own first column, "
                      "not read as empty, so it stays the flat grey.",
                      case(direction=90), [0]),
    "FX-EMBOSS-007": ("Direction 270, relief 1: lit from the other side, FX-EMBOSS-006 turned "
                      "inside out: wherever neither is held at black or white, each pixel's "
                      "grey is 1 less FX-EMBOSS-006's.",
                      case(direction=270), [0]),
    "FX-EMBOSS-008": ("Direction 0, relief 1: each pixel looks one pixel up and one down, so "
                      "only level edges show: the square's upright sides and the drawing's "
                      "soft right-hand edge vanish, the band and the square's top and bottom "
                      "stand out, the top row, looking up at the empty row, goes dark, and the "
                      "bottom row, looking down at it, light.",
                      case(direction=0), [0]),
    "FX-EMBOSS-009": ("Contrast 300: each difference three times as strong, the strongest "
                      "edges held at black or white.",
                      case(contrast=300), [0]),
    "FX-EMBOSS-010": ("Contrast 1000: ten times as strong: every edge pixel but the eight "
                      "faintest is black or white, and the flat parts stay the middle grey.",
                      case(contrast=1000), [0]),
    "FX-EMBOSS-011": ("Relief 2.5: each pixel looks two and a half pixels ahead and behind, "
                      "so the ridges are wider and almost nothing stays flat: the skin rows "
                      "just inside the top and bottom and the whole square now reach an edge. "
                      "Only the soft column's top pixel is the middle grey, both its looks "
                      "landing on emptiness.",
                      case(relief=2.5), [0]),
    "FX-EMBOSS-012": ("Relief 100, direction 270, contrast 50: both looks fall far outside "
                      "the drawing and are held at its edges, ahead at its first column and "
                      "behind at its empty last one, so every pixel of a row that shows is one "
                      "grey, 0.5 + half the first column's luma: lighter in the skin's rows "
                      "than in the band's.",
                      case(direction=270, relief=100, contrast=50), [0]),
    "FX-EMBOSS-013": ("Direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 "
                      "is FX-EMBOSS-008, frame 2 is direction 90, FX-EMBOSS-006, and frame 4 "
                      "direction 180, lit from below.",
                      case(direction=keyed((0, 0), (4, 180))), [0, 2, 4]),
    "FX-EMBOSS-014": ("Relief keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is "
                      "FX-EMBOSS-002, flat grey, frame 2 is relief 1, FX-EMBOSS-001, and frame "
                      "4 relief 2.",
                      case(relief=keyed((0, 0), (4, 2))), [0, 2, 4]),
    "FX-EMBOSS-015": ("Contrast eased from 100 at frame 0 to 1000 at frame 4 on a curve that "
                      "overshoots: frame 0 is FX-EMBOSS-001, and at frame 2 it has gone past "
                      "1000 and is held there, so frames 2 and 4 are both FX-EMBOSS-010.",
                      case(contrast=keyed((0, 100, OVERSHOOT), (4, 1000))), [0, 2, 4]),
    "FX-EMBOSS-016": ("Direction 495, one turn past 135: FX-EMBOSS-001.",
                      case(direction=495), [0]),
    "FX-EMBOSS-017": ("Direction -225, the same way as 135: FX-EMBOSS-001.",
                      case(direction=-225), [0]),
    "FX-EMBOSS-018": ("FX-EMBOSS-005 moved three pixels right: the relief is worked in the "
                      "drawing's own space, so it moves with it, and nothing grows: the three "
                      "columns left of the drawing stay empty.",
                      case(mode="color", shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-EMBOSS-019": ("Relief 101, above 100.", case(relief=101)),
    "FX-EMBOSS-020": ("Relief -1, below 0.", case(relief=-1)),
    "FX-EMBOSS-021": ("Contrast 1001, above 1000.", case(contrast=1001)),
    "FX-EMBOSS-022": ("Contrast -1, below 0.", case(contrast=-1)),
    "FX-EMBOSS-023": ("Direction 3601, above 3600.", case(direction=3601)),
    "FX-EMBOSS-024": ("Mode \"gray\", which is not a choice: the word is \"grey\".",
                      case(mode="gray")),
    "FX-EMBOSS-025": ("Mode \"Grey\": the word is exact, so a capital is not the choice.",
                      case(mode="Grey")),
    "FX-EMBOSS-026": ("Relief keyed to 150 at frame 4.", case(relief=keyed((0, 1), (4, 150)))),
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
        "instance_id": "fx-0-0", "type_id": "core.emboss", "enabled": True,
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

    (OUT / "expected_emboss.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    layer = drawn_layer("plate")
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g, e=1e-9: all(near(p, q, e) for p, q in zip(f, g))  # noqa: E731
    shows = [(x, y) for y in range(H) for x in range(W) if drawn[at(x, y)][3] > 0]
    Y = lambda x, y: luma(drawn[at(x, y)])  # noqa: E731
    enc = lambda p: [S.linear_to_srgb(min(1.0, max(0.0, v / p[3]))) for v in p[:3]]  # noqa: E731
    back = lambda e, a: [srgb_to_linear(min(1.0, max(0.0, v))) * a  # noqa: E731
                         for v in e] + [a]

    def grey(f, x, y):
        """The encoded grey a grey-mode pixel was given (all three channels alike)."""
        p = f[at(x, y)]
        e = enc(p)
        assert abs(e[0] - e[1]) < 1e-9 and abs(e[1] - e[2]) < 1e-9, (x, y)
        return e[0]

    def flat(f, x, y):
        return near(f[at(x, y)], back([0.5] * 3, drawn[at(x, y)][3]))

    # The rule's own pieces.
    assert unit(135) == unit(495) == unit(-225) and unit(90) == (1, 0) and unit(0) == (0, -1)
    assert unit(270) == (-1, 0) and unit(180) == (0, 1)
    assert luma([0.0] * 4) == 0 and abs(luma([1.0] * 4) - 1) < 1e-12
    assert embossed([0.2, 0.4, 0.6], 0.7, "grey") == [0.7] * 3
    assert near(embossed([0.2, 0.4, 0.6], 0.7, "color"), [0.4, 0.6, 0.8], 1e-15)
    # The held sample: a look past the left edge reads the first column, not emptiness.
    assert bilinear(layer, -0.5, 2.5, "repeat") == layer["px"][at(0, 2)]
    assert bilinear(layer, -0.5, 2.5) == [0.0] * 4
    ys, yb, yl, yh = Y(0, 1), Y(0, 4), Y(6, 2), Y(14, 1)
    assert yl < yb < yh < ys  # the line, the band, the soft skin, the skin

    # Every case keeps every pixel's covering and leaves the empty pixels empty.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-EMBOSS-018" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)

    one = c["FX-EMBOSS-001"]["0"]
    for x, y in shows:
        want = min(1, max(0, relief_value(layer, 135, 1, 100, x, y)))
        assert abs(grey(one, x, y) - want) < 1e-9, (x, y)
    # The flat skin and the middle of the square stay the middle grey.
    for xy in ((2, 2), (3, 7), (7, 4), (8, 3), (12, 2), (11, 7)):
        assert flat(one, *xy), xy
    # The drawing stands up: top row light, bottom row and soft column dark.
    assert all(grey(one, x, 1) > 0.8 for x in (0, 1, 2, 3, 4, 10, 11, 12))
    assert all(grey(one, x, 8) < 0.2 for x in (0, 1, 2, 3, 4, 5, 11, 12, 13))
    assert all(grey(one, 14, y) < 0.45 for y in range(1, 9))
    assert all(one[at(14, y)][3] == 128 / 255 for y in range(1, 9))
    # The band and the square sink in: top edges dark, bottom edges light, the square's left
    # edge dark and its right edge light.
    for x in range(5):
        assert grey(one, x, 3) < 0.3 and grey(one, x, 4) < 0.3, x
        assert grey(one, x, 5) > 0.7 and grey(one, x, 6) > 0.7, x
    for xy in ((5, 2), (6, 2), (7, 2), (6, 3)):
        assert grey(one, *xy) < 0.2, xy
    for xy in ((9, 3), (9, 5), (9, 6), (9, 7), (7, 7), (8, 7)):
        assert grey(one, *xy) > 0.8, xy

    two = c["FX-EMBOSS-002"]["0"]
    assert all(flat(two, *xy) for xy in shows)
    assert like(c["FX-EMBOSS-003"]["0"], drawn, 1e-12)
    assert like(c["FX-EMBOSS-004"]["0"], two, 1e-12)

    five = c["FX-EMBOSS-005"]["0"]
    for x, y in shows:
        v = relief_value(layer, 135, 1, 100, x, y)
        p = drawn[at(x, y)]
        assert near(five[at(x, y)], back([e + v - 0.5 for e in enc(p)], p[3])), (x, y)
    for xy in ((2, 2), (3, 7), (7, 4), (8, 3), (12, 2), (11, 7)):
        assert near(five[at(*xy)], drawn[at(*xy)]), xy
    assert sum(not near(five[at(*xy)], drawn[at(*xy)]) for xy in shows) > 90

    six = c["FX-EMBOSS-006"]["0"]
    for x, y in shows:
        want = 0.5 + Y(min(x + 1, W - 1), y) - Y(max(x - 1, 0), y)
        assert abs(grey(six, x, y) - min(1, max(0, want))) < 1e-9, (x, y)
    for y in range(1, 9):  # the first column is held, so reads itself: flat
        assert flat(six, 0, y)
    for x in (0, 1, 2, 3, 4, 11, 12):  # the band vanishes but beside the square
        assert flat(six, x, 4) and flat(six, x, 5)
    assert all(grey(six, 5, y) < 0.2 and grey(six, 10, y) > 0.8 for y in range(2, 8))
    assert all(grey(six, 14, y) < 0.1 for y in range(1, 9))
    seven = c["FX-EMBOSS-007"]["0"]
    both = 0
    for x, y in shows:
        g6, g7 = grey(six, x, y), grey(seven, x, y)
        if 0 < g6 < 1 and 0 < g7 < 1:
            assert abs(g6 + g7 - 1) < 1e-9, (x, y)
            both += 1
    assert both > 90
    eight = c["FX-EMBOSS-008"]["0"]
    for x, y in shows:
        want = 0.5 + Y(x, y - 1) - Y(x, y + 1)
        assert abs(grey(eight, x, y) - min(1, max(0, want))) < 1e-9, (x, y)
    for y in range(1, 9):  # no upright edge: each side of the square as its neighbour
        assert near(eight[at(5, y)], eight[at(4, y)]) and near(eight[at(6, y)], eight[at(7, y)])
        assert near(eight[at(9, y)], eight[at(8, y)]) and near(eight[at(10, y)], eight[at(11, y)])
    for y in range(2, 8):
        assert flat(eight, 14, y)  # the soft column above and below the same: flat
    assert all(grey(eight, x, 1) < 0.45 for x in range(15))  # top row looks up at nothing
    assert all(grey(eight, x, 8) > 0.55 for x in range(15))  # bottom row looks down at it
    assert grey(eight, 2, 3) > 0.8 and grey(eight, 2, 6) < 0.2  # the band stands out
    assert grey(eight, 7, 2) > 0.8 and grey(eight, 7, 7) < 0.2  # the square, top and bottom

    nine, ten = c["FX-EMBOSS-009"]["0"], c["FX-EMBOSS-010"]["0"]
    k9 = k10 = faint = 0
    for x, y in shows:
        v = relief_value(layer, 135, 1, 100, x, y) - 0.5
        g9, g10 = grey(nine, x, y), grey(ten, x, y)
        assert abs(g9 - min(1, max(0, 0.5 + 3 * v))) < 1e-9, (x, y)
        assert abs(g10 - min(1, max(0, 0.5 + 10 * v))) < 1e-9, (x, y)
        k9 += g9 < 1e-9 or g9 > 1 - 1e-9
        if abs(v) >= 0.05:
            assert g10 < 1e-9 or g10 > 1 - 1e-9, (x, y)
            k10 += 1
        elif abs(v) > 1e-9:
            faint += 1
        else:
            assert flat(ten, x, y)
    assert k9 > 60 and k10 == 88 and faint == 8

    eleven = c["FX-EMBOSS-011"]["0"]
    for x, y in shows:
        assert abs(grey(eleven, x, y) -
                   min(1, max(0, relief_value(layer, 135, 2.5, 100, x, y)))) < 1e-9
    assert [xy for xy in shows if flat(eleven, *xy)] == [(14, 1)]
    assert sum(flat(one, *xy) for xy in shows) > 20
    assert all(flat(one, x, 2) and flat(one, x, 7) for x in (0, 1, 2, 3, 4, 11, 12))

    twelve = c["FX-EMBOSS-012"]["0"]
    for x, y in shows:
        assert abs(grey(twelve, x, y) - (0.5 + 0.5 * Y(0, y))) < 1e-9, (x, y)
    assert abs(grey(twelve, 7, 2) - (0.5 + 0.5 * ys)) < 1e-9
    assert abs(grey(twelve, 7, 4) - (0.5 + 0.5 * yb)) < 1e-9 and yb < ys

    thirteen = c["FX-EMBOSS-013"]
    assert like(thirteen["0"], eight) and like(thirteen["2"], six)
    assert like(thirteen["4"], render(case(direction=180), 0)) and thirteen["4"] != eight
    fourteen = c["FX-EMBOSS-014"]
    assert like(fourteen["0"], two) and like(fourteen["2"], one)
    assert like(fourteen["4"], render(case(relief=2), 0)) and fourteen["4"] != one
    fifteen = c["FX-EMBOSS-015"]
    assert ease(OVERSHOOT, 0.5) > 1 and value_at(keyed((0, 100, OVERSHOOT), (4, 1000)), 2) > 1000
    assert fifteen["0"] == one and fifteen["2"] == ten and fifteen["4"] == ten
    assert c["FX-EMBOSS-016"]["0"] == one and c["FX-EMBOSS-017"]["0"] == one
    moved = c["FX-EMBOSS-018"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == five[at(0, y):at(W - 3, y)]
        assert moved["0"][at(0, y):at(3, y)] == [[0.0] * 4] * 3
    print("checked")


if __name__ == "__main__":
    main()
