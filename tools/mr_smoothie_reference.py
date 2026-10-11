"""Mr. Smoothie, worked a second way.

D-462 adds `core.mr_smoothie`, after After Effects' CC Mr. Smoothie (Stylize): the layer
repainted in flowing bands of colour, a gradient sampled from the layer between two points laid
over the shapes of a channel of a layer, its own or another (D-189's layer setting). What is
known (the CycoreFX HD 1.8.1 manual's page on CC Mr. Smoothie and a Creative Cow thread on its
Color Loop; no tutorial found says more) is: Flow Layer chooses the layer whose pattern the
gradient is mapped onto, the layer itself at first; Property chooses its channel, Red, Green,
Blue, Alpha, Luminance, Lightness, Hue or Saturation, Luminance at first; Smoothness softens the
pattern; Sample A and Sample B are the two points of the layer the gradient's colours are
sampled between; Phase, in degrees, turns the gradient round so that it starts and ends at
other colours; and Color Loop, AB, BA, ABA or BAB, sets the gradient's order. The rule below is
this program's own, and nothing is ported.

The rule, at the holder's frame, with its input O, linear and premultiplied, W by H, its corner
at o in the buffer an earlier effect may have grown:

1. The map is D-189's layer setting, fitted to the drawing's own W by H and lying on it, clear
   outside it; with the name "" (or a layer not in the composition, with the warning) it is O
   itself. F is at each pixel the map's `property` as CC Vector Blur reads it: red, green,
   blue, luminance, lightness, hue (over 360) or saturation of the straight encoded colour,
   times the covering; or the covering itself for `alpha`.
2. Bv is F through document 21's Gaussian at sigma `smoothness / 2`, nothing outside the
   drawing; at smoothness 0, Bv is F.
3. v = Bv held to 0..1, and t = v + `phase` / 360. Only a t below 0 or above 1 is wrapped,
   t - floor(t), so at Phase 0 the channel's 0 and 1 are the gradient's two ends.
4. The place along the gradient, s: AB, t; BA, 1 - t; ABA, 1 - |2 t - 1|; BAB, |2 t - 1|. So AB
   runs from A to B, BA back, and ABA and BAB go there and back, with no jump where t wraps.
5. A and B are `sample_a` and `sample_b`, per cent of the drawing's width and height, plus o.
   The colour is document 21's bilinear sample of O at P = A + s (B - A), P first held inside
   the buffer's pixel centres, made straight (its colour over its covering; black where it has
   none).
6. The output is that colour times O's own covering at the pixel: the layer keeps its shape and
   only its colours change. The layer does not grow. For a draft the smoothness is a distance.

AB and BA jump from one end to the other where t wraps: `check` asserts that no pixel of those
cases has a wrapped t within 1e-6 of a whole number.

`layer` D-189's word, ""; `fit` `stretch`, `center` or `tile`, `stretch`; `property` one of the
eight words above, `luminance`; `smoothness` 0 to 100 pixels, 5; `sample_a` and `sample_b`
-1000 to 1000 per cent each way, 25, 50 and 75, 50; `phase` -3600 to 3600 degrees, 0;
`color_loop` `ab`, `ba`, `aba` or `bab`, `aba`. Every number keyable. Words are kept as written,
so a wrong one is reported. Apart from Property, the values when added are chosen here; the
manual gives none. After Effects' Blend w. Original is not in the manual and is not a setting;
the effect's own mix (D-137) does it.

**This file never runs the build's code path.** It sums the two-dimensional kernel directly where
the build runs two passes, in double precision from the drawings' 8-bit values.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/mr_smoothie/`: the
drawing `holder` at the top with the effect, and under it the map layers, every one switched
off. The expected pixels are in `Fixtures/mr_smoothie/expected_mr_smoothie.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/mr_smoothie_reference.py
"""

import json
import sys
from math import floor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from vector_blur_reference import phase  # noqa: E402
from blobbylize_reference import kernel  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "mr_smoothie"
TOLERANCE = 2e-5  # document 25's default for a filter
CLIFF = 1e-6
W, H, FRAMES = 16, 10, 5
NONE = (0, 0, 0, 0)
RED, BLUE, CREAM = (220, 40, 40, 255), (40, 80, 220, 255), (240, 240, 200, 255)
WHITE, BLACK = (255, 255, 255, 255), (0, 0, 0, 255)


def shapes_px(x, y):
    """A red block, columns 1 to 5 and rows 2 to 7, and a blue one, columns 9 to 11 and rows 3
    to 6, clear round them."""
    if 1 <= x <= 5 and 2 <= y <= 7:
        return RED
    if 9 <= x <= 11 and 3 <= y <= 6:
        return BLUE
    return NONE


def dots_px(x, y):
    """Clear, with two white squares three pixels across, at columns 2 to 4, rows 2 to 4, and
    columns 10 to 12, rows 5 to 7."""
    if (2 <= x <= 4 and 2 <= y <= 4) or (10 <= x <= 12 and 5 <= y <= 7):
        return WHITE
    return NONE


DRAWINGS = {
    # Red rising and green falling to the right, blue rising down: every pixel its own colour.
    "hues": [[(17 * x, 255 - 17 * x, 60 + 15 * y, 255) for x in range(W)] for y in range(H)],
    "shapes": [[shapes_px(x, y) for x in range(W)] for y in range(H)],
    # Two-pixel squares of red, blue and cream, solid all over.
    "photo": [[(RED, BLUE, CREAM)[(x // 2 + y // 2) % 3] for x in range(W)] for y in range(H)],
    "dots": [[dots_px(x, y) for x in range(W)] for y in range(H)],
    # Black at the left to white at the right, one step of 17 a column.
    "ramp": [[(17 * x, 17 * x, 17 * x, 255) for x in range(W)] for y in range(H)],
    # 8 by 5: a red disc 2 pixels round the middle on black.
    "spot": [[(255, 0, 0, 255) if (x + 0.5 - 4) ** 2 + (y + 0.5 - 2.5) ** 2 <= 4 else BLACK
              for x in range(8)] for y in range(5)],
}


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


MAPS = ("dots", "ramp", "spot", "photo")
LOOPS = ("ab", "ba", "aba", "bab")


# --- the rule -------------------------------------------------------------------------------

def place(t, loop):
    """Steps 3 and 4: the wrapped t and the place s along the gradient."""
    wrapped = t < 0 or t > 1
    if wrapped:
        t = t - floor(t)
    s = {"ab": t, "ba": 1 - t, "aba": 1 - abs(2 * t - 1), "bab": abs(2 * t - 1)}[loop]
    return s, wrapped, t


def mr_smoothie(o, m, s, cliffs=None):
    w, h = o["w"], o["h"]
    f = [phase(s["property"], p) for p in m["px"]]
    r, one = kernel(s["smoothness"] / 2)

    def bv(x, y):
        if r == 0:
            return f[y * w + x]
        return sum(f[j * w + i] * one[i - x + r] * one[j - y + r]
                   for j in range(max(0, y - r), min(h, y + r + 1))
                   for i in range(max(0, x - r), min(w, x + r + 1)))

    ax, ay = s["sample_a"][0] / 100 * w, s["sample_a"][1] / 100 * h
    bx, by = s["sample_b"][0] / 100 * w, s["sample_b"][1] / 100 * h
    out = []
    for y in range(h):
        for x in range(w):
            v = min(max(bv(x, y), 0.0), 1.0)
            k, wrapped, t = place(v + s["phase"] / 360, s["color_loop"])
            if cliffs is not None and wrapped and s["color_loop"] in ("ab", "ba"):
                cliffs.append(min(t, 1 - t))
            px, py = ax + k * (bx - ax), ay + k * (by - ay)
            q = L.bilinear(o, min(max(px, 0.5), w - 0.5), min(max(py, 0.5), h - 0.5))
            col = [q[i] / q[3] for i in range(3)] if q[3] > 0 else [0.0] * 3
            a = o["px"][y * w + x][3]
            out.append([c * a for c in col] + [a])
    return L.pic(w, h, out)


# --- the cases ------------------------------------------------------------------------------

NAMES = ("layer", "fit", "property", "smoothness", "sample_a", "sample_b", "phase", "color_loop")


def case(holder="hues", shift=(0, 0), **kw):
    c = {"layer": "", "fit": "stretch", "property": "luminance", "smoothness": 5,
         "sample_a": (25, 50), "sample_b": (75, 50), "phase": 0, "color_loop": "aba"}
    c.update(kw)
    c["holder"], c["shift"] = holder, shift
    return c


def the_map(c):
    if c["layer"] in MAPS:
        return L.fit(decoded(c["layer"]), c["fit"], W, H)
    return decoded(c["holder"])  # "" and a layer not there: the layer itself


def settings(c, frame):
    return {k: value_at(c[k], frame) for k in NAMES}


def render(c, frame, cliffs=None):
    out = mr_smoothie(decoded(c["holder"]), the_map(c), settings(c, frame), cliffs)
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded(c["holder"])
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


RAMP = dict(layer="ramp", smoothness=0)
CASES = {
    "FX-SMOOTHIE-001": ("As added: the layer's own luminance, Smoothness 5, Sample A at 25, 50 "
                        "per cent and B at 75, 50, Phase 0, Color Loop ABA: the layer repainted "
                        "in soft bands of the colours between the two points.", case(), (0,)),
    "FX-SMOOTHIE-002": ("The ramp as the flow layer, Smoothness 0, AB: each column takes the "
                        "colour as far from A toward B as the ramp is bright there, A at the "
                        "black left, B at the white right.", case(color_loop="ab", **RAMP), (0,)),
    "FX-SMOOTHIE-003": ("The same, BA: the gradient the other way, B at the left.",
                        case(color_loop="ba", **RAMP), (0,)),
    "FX-SMOOTHIE-004": ("The same, ABA: A at both sides, B in the middle.",
                        case(color_loop="aba", **RAMP), (0,)),
    "FX-SMOOTHIE-005": ("The same, BAB: B at both sides, A in the middle.",
                        case(color_loop="bab", **RAMP), (0,)),
    "FX-SMOOTHIE-006": ("FX-SMOOTHIE-002 with Phase 90: the gradient moved a quarter along, the "
                        "brightest columns wrapping round to start again near A.",
                        case(color_loop="ab", phase=90, **RAMP), (0,)),
    "FX-SMOOTHIE-007": ("FX-SMOOTHIE-002 with Sample A and B swapped: FX-SMOOTHIE-003.",
                        case(color_loop="ab", sample_a=(75, 50), sample_b=(25, 50), **RAMP), (0,)),
    "FX-SMOOTHIE-008": ("Sample A and B both at 25, 50 on the two blocks: every covered pixel "
                        "the red found there, the blue block turned red, the clear round them "
                        "kept clear.", case(holder="shapes", sample_a=(25, 50), sample_b=(25, 50)),
                        (0,)),
    "FX-SMOOTHIE-009": ("The two blocks' own luminance, Smoothness 4, AB: the blocks recoloured, "
                        "their coverings exactly as they were.",
                        case(holder="shapes", smoothness=4, color_loop="ab"), (0,)),
    "FX-SMOOTHIE-010": ("Property Hue, Smoothness 0: each pixel placed along the gradient by its "
                        "own hue.", case(property="hue", smoothness=0), (0,)),
    "FX-SMOOTHIE-011": ("Property Saturation of the photo, Smoothness 2, BAB.",
                        case(layer="photo", property="saturation", smoothness=2, color_loop="bab"),
                        (0,)),
    "FX-SMOOTHIE-012": ("The dots layer's covering, Smoothness 2, AB: B's colour in two soft "
                        "round patches where the dots are, A's elsewhere.",
                        case(layer="dots", property="alpha", smoothness=2, color_loop="ab"), (0,)),
    "FX-SMOOTHIE-013": ("The 8 by 5 spot, centred and clear round it, Property Red, Smoothness "
                        "2: a round patch from further along the gradient in the middle.",
                        case(layer="spot", fit="center", property="red", smoothness=2), (0,)),
    "FX-SMOOTHIE-014": ("A layer that is not in the composition, `gone`: the layer itself is the "
                        "flow layer, as FX-SMOOTHIE-001, and the warning every frame.",
                        case(layer="gone"), (0, 4)),
    "FX-SMOOTHIE-015": ("Smoothness keyed from 0 at frame 0 to 8 at frame 4, linear: frame 2 is "
                        "Smoothness 4.", case(smoothness=keyed((0, 0), (4, 8))), (0, 2, 4)),
    "FX-SMOOTHIE-016": ("FX-SMOOTHIE-009 on the holder moved 2 right and 1 down: the same, moved.",
                        case(holder="shapes", smoothness=4, color_loop="ab", shift=(2, 1)), (0,)),
    "FX-SMOOTHIE-017": ("Sample A keyed from 0, 50 at frame 0 to 100, 50 at frame 4, linear: "
                        "frame 2 has A at 50, 50.",
                        case(sample_a=keyed((0, (0, 50)), (4, (100, 50)))), (0, 2, 4)),
    "FX-SMOOTHIE-018": ("Phase keyed from 0 at frame 0 to 180 at frame 4 on the ramp, AB, "
                        "linear: frame 0 is FX-SMOOTHIE-002 and frame 2 FX-SMOOTHIE-006.",
                        case(color_loop="ab", phase=keyed((0, 0), (4, 180)), **RAMP), (0, 2, 4)),
}

INVALID = {
    "FX-SMOOTHIE-019": ("Smoothness 101, above 100.", case(smoothness=101)),
    "FX-SMOOTHIE-020": ("A property written \"value\", which Mr. Smoothie does not read.",
                        case(property="value")),
    "FX-SMOOTHIE-021": ("A colour loop written \"abab\".", case(color_loop="abab")),
    "FX-SMOOTHIE-022": ("A layer written as the number 3, not a word.", case(layer=3)),
    "FX-SMOOTHIE-023": ("Sample B at 50, 1001, past ten heights.", case(sample_b=(50, 1001))),
    "FX-SMOOTHIE-024": ("Phase 3601, above 3600.", case(phase=3601)),
    "FX-SMOOTHIE-025": ("A fit written \"fill\".", case(fit="fill")),
    "FX-SMOOTHIE-026": ("Smoothness keyed to 150 at frame 4.",
                        case(smoothness=keyed((0, 5), (4, 150)))),
}


def effect(c):
    return {"instance_id": "fx-1", "type_id": "core.mr_smoothie", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in NAMES}}


def project_json(fx, c):
    holder = L.raster("holder", "asset-" + c["holder"], position=c["shift"], out_frame=FRAMES)
    holder["effects"] = [effect(c)]
    maps = [L.raster(n, "asset-" + n, enabled=False, out_frame=FRAMES) for n in MAPS]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, FRAMES, [holder] + maps)]}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    cliffs = []
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f, cliffs) for f in frames}
        entry = {"says": says, "project": write(fx, c), "frames": rendered}
        if c["layer"] == "gone":
            entry["warning"] = "EFFECT_LAYER_MISSING"
        expected["cases"][fx] = entry
        before = plain(c)
        print(f"{fx}: " + ", ".join(f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} "
                                    "changed" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_mr_smoothie.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected, cliffs)


def check(expected, cliffs):
    """The claims the cases make, checked on the numbers just worked."""
    assert not cliffs or min(cliffs) > CLIFF, min(cliffs)
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    straight = lambda p: [v / p[3] for v in p[:3]] if p[3] > 0 else [0.0] * 3  # noqa: E731
    hues = decoded("hues")
    a_col = straight(L.bilinear(hues, 4.0, 5.0))
    b_col = straight(L.bilinear(hues, 12.0, 5.0))
    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(u >= -1e-12 for u in p[:3]), (fx, p)
        if fx in INVALID:
            base = plain(INVALID[fx][1])
            assert all(px == base for px in frames.values())
    two, three, four, five = (c[f"FX-SMOOTHIE-00{i}"]["0"] for i in (2, 3, 4, 5))
    # The ramp's black left column is A under AB, its white right column B; reversed under BA.
    for y in range(H):
        assert near([two[at(0, y)][:3]], [a_col], 1e-9) and near([two[at(15, y)][:3]], [b_col], 1e-9)
        assert near([three[at(0, y)][:3]], [b_col], 1e-9) and near([three[at(15, y)][:3]], [a_col], 1e-9)
        assert near([four[at(0, y)][:3]], [a_col], 1e-9) and near([four[at(15, y)][:3]], [a_col], 1e-9)
        assert near([five[at(0, y)][:3]], [b_col], 1e-9) and near([five[at(15, y)][:3]], [b_col], 1e-9)
    assert near(c["FX-SMOOTHIE-007"]["0"], three, 1e-12)
    six = c["FX-SMOOTHIE-006"]["0"]
    # Column 12 wraps round to near A; column 11, just short of the wrap, is near B.
    gap = lambda p, q: sum((u - v) ** 2 for u, v in zip(p[:3], q))  # noqa: E731
    assert six != two and gap(six[at(12, 0)], a_col) < gap(six[at(12, 0)], b_col)
    assert gap(six[at(11, 0)], b_col) < gap(six[at(11, 0)], a_col)
    shapes = plain(case(holder="shapes"))
    eight, nine = c["FX-SMOOTHIE-008"]["0"], c["FX-SMOOTHIE-009"]["0"]
    red = shapes[at(3, 4)]
    for i, p in enumerate(shapes):
        assert eight[i][3] == p[3] and nine[i][3] == p[3]
        if p[3] > 0:
            assert near([eight[i]], [red], 1e-9)
    assert nine != shapes
    assert c["FX-SMOOTHIE-010"]["0"] != c["FX-SMOOTHIE-001"]["0"]
    twelve = c["FX-SMOOTHIE-012"]["0"]
    assert gap(twelve[at(3, 3)], b_col) < gap(twelve[at(3, 3)], a_col)
    assert near([twelve[at(15, 0)][:3]], [a_col], 1e-9)
    assert c["FX-SMOOTHIE-013"]["0"][at(8, 5)] != c["FX-SMOOTHIE-013"]["0"][at(0, 0)]
    assert c["FX-SMOOTHIE-014"]["0"] == c["FX-SMOOTHIE-001"]["0"] == c["FX-SMOOTHIE-014"]["4"]
    fifteen = c["FX-SMOOTHIE-015"]
    assert fifteen["2"] == render(case(smoothness=4), 0) and fifteen["0"] != fifteen["2"]
    sixteen = c["FX-SMOOTHIE-016"]["0"]
    assert all(sixteen[at(x, y)] == nine[at(x - 2, y - 1)] for x in range(2, W) for y in range(1, H))
    seventeen = c["FX-SMOOTHIE-017"]
    assert seventeen["2"] == render(case(sample_a=(50, 50)), 0)
    eighteen = c["FX-SMOOTHIE-018"]
    assert eighteen["0"] == two and eighteen["2"] == six and eighteen["4"] != six
    print("checked")


if __name__ == "__main__":
    main()
