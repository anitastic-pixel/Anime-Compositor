"""Compound Blur, worked a second way.

D-191 proposes Compound Blur (`core.compound_blur`): a layer blurred more where another layer of
the same composition, its map, is brighter. The map is D-189's: the named layer's picture fitted
to the holder, `tools/effect_layer_reference.py`. This file pins what is done with it.

At the holder's composition frame n, with its input O, linear and premultiplied, whose drawing's
own top-left pixel is at (ox, oy) however far earlier effects grew it:

1. With no map (an empty name, a missing layer) or `max_blur` 0, the output is O.
2. The map M is D-189's, fitted to the drawing's own W by H. Pixel (x, y) of O reads
   M(x - ox, y - oy), transparent outside M. v is its picture luma, the sRGB encoding of
   0.2126 r + 0.7152 g + 0.0722 b held within 0 and 1, as Find Edges reads a picture; transparent
   is 0. With `invert` on, v is 1 - v.
3. S = max_blur / 3, and the six levels are s_0 = 0 and s_j = S 2^(j - 5) for j = 1 to 5: S/16,
   S/8, S/4, S/2 and S. B_0 is O, and B_j is O through document 21's Gaussian at sigma s_j, read
   at O's own pixels: with `edges` `transparent` nothing outside O, with `repeat` O's outer
   pixels held. The layer does not grow.
4. At each pixel s = v S. With j the level where s_j <= s <= s_(j + 1) and
   t = (s - s_j) / (s_(j + 1) - s_j), the output is B_j + t (B_(j + 1) - B_j), all four numbers.
5. For a draft, `max_blur` is a distance and the map is made at the holder's size (D-189 step 3).

`max_blur` 0 to 500, keyable, 20 when added; `layer` D-189's word, "" when added; `fit` `stretch`
when added (After Effects' Stretch Map to Fit is on when added), `center` or `tile`; `invert`
`off` or `on`; `edges` `transparent` or `repeat`, as Lens Blur's.

**This file never runs the build's code path.** It blurs with the two-dimensional kernel summed
directly where the build runs two passes, in double precision from the drawings' 8-bit values.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/compound_blur/`:
the drawing `holder` at the top with the effect, and under it the map layers, every one switched
off, so that only the holder is seen. The expected pixels are in
`Fixtures/compound_blur/expected_compound_blur.json`, with four projects whose layer settings go
round in a circle, or seem to.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/compound_blur_reference.py
"""

import json
import sys
from math import ceil, exp
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import smooth_reference as S  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "compound_blur"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
CLEAR = [0.0, 0.0, 0.0, 0.0]

RED, BLUE, CREAM, NONE = (220, 40, 40, 255), (40, 80, 220, 255), (240, 240, 200, 255), (0, 0, 0, 0)


def holder_px(x, y):
    """Two-pixel squares of red, blue and cream, the bottom right corner 4 by 3 clear."""
    if x >= 12 and y >= 7:
        return NONE
    return (RED, BLUE, CREAM)[(x // 2 + y // 2) % 3]


DRAWINGS = {
    "holder": [[holder_px(x, y) for x in range(W)] for y in range(H)],
    # Black at the left to white at the right, one step of 17 a column.
    "ramp": [[(17 * x, 17 * x, 17 * x, 255) for x in range(W)] for y in range(H)],
    # A 4 by 2 checker of white and black, white at its top left.
    "card": [[(255, 255, 255, 255) if (x + y) % 2 == 0 else (0, 0, 0, 255) for x in range(4)]
             for y in range(2)],
}


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


def solid_pic(color):
    return L.pic(W, H, [list(color) + [1.0] for _ in range(W * H)])


GREY = [0.25, 0.25, 0.25]  # linear, as a solid's colour is written
MAPS = {  # what each map layer's picture is at frame 0; `late` starts at frame 4
    "ramp": decoded("ramp"), "white": solid_pic([1.0] * 3), "black": solid_pic([0.0] * 3),
    "grey": solid_pic(GREY), "card": decoded("card"), "late": solid_pic([1.0] * 3),
}


# --- the rule -------------------------------------------------------------------------------

def luma(p):
    return S.linear_to_srgb(min(1.0, max(0.0, 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2])))


def gaussian(p, sigma, edges):
    """Document 21's Gaussian at sigma, the kernel summed in two dimensions, read at p's own
    pixels: transparent outside p, or its outer pixels held."""
    r = ceil(3 * sigma)
    if r == 0:
        return p
    one = [exp(-(k * k) / (2 * sigma * sigma)) for k in range(-r, r + 1)]
    total = sum(one)
    one = [v / total for v in one]
    w, h = p["w"], p["h"]
    out = []
    for y in range(h):
        for x in range(w):
            acc = [0.0] * 4
            for j in range(-r, r + 1):
                for i in range(-r, r + 1):
                    if edges == "repeat":
                        s = L.at(p, min(max(x + i, 0), w - 1), min(max(y + j, 0), h - 1))
                    else:
                        s = L.at(p, x + i, y + j)
                    k = one[i + r] * one[j + r]
                    for c in range(4):
                        acc[c] += s[c] * k
            out.append(acc)
    return L.pic(w, h, out)


def compound_blur(o, m, max_blur, invert, edges):
    """Steps 1 to 4 on the picture o, the map m already fitted to it, or None."""
    if m is None or max_blur == 0:
        return o
    big = max_blur / 3
    levels = [0.0] + [big * 2 ** (j - 5) for j in range(1, 6)]
    blurred = [o] + [gaussian(o, s, edges) for s in levels[1:]]
    out = []
    for i in range(o["w"] * o["h"]):
        v = luma(m["px"][i])
        s = (1 - v if invert == "on" else v) * big
        j = max(k for k in range(5) if levels[k] <= s)
        t = (s - levels[j]) / (levels[j + 1] - levels[j])
        a, b = blurred[j]["px"][i], blurred[j + 1]["px"][i]
        out.append([a[c] + t * (b[c] - a[c]) for c in range(4)])
    return L.pic(o["w"], o["h"], out)


# --- the cases ------------------------------------------------------------------------------

def case(layer="", fit="stretch", max_blur=20, invert="off", edges="transparent", shift=(0, 0),
         on="holder"):
    return {"layer": layer, "fit": fit, "max_blur": max_blur, "invert": invert, "edges": edges,
            "shift": shift, "on": on}


def the_map(c, frame):
    if c["layer"] == "":
        return None
    if c["layer"] == "holder":
        return decoded("holder")  # steps 1 and 2 only: the drawing, no mask
    if c["layer"] not in MAPS:
        return None
    p = L.EMPTY if c["layer"] == "late" and frame < 4 else MAPS[c["layer"]]
    return L.fit(p, c["fit"], W, H)


def render(c, frame):
    """The composition's pixels at frame: the holder with its effect, moved by its shift; or,
    with the effect on an adjustment layer above, the frame below through it."""
    o = decoded("holder")
    out = compound_blur(o, the_map(c, frame), value_at(c["max_blur"], frame), c["invert"],
                        c["edges"])
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded("holder")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


CASES = {
    "FX-CBLUR-001": ("As added: no layer named, so nothing is blurred.", case(), (0,)),
    "FX-CBLUR-002": ("The ramp as the map, Maximum Blur 6: sharp at the left, blurred more and "
                     "more to the right. The ramp layer is moved, scaled and switched off, none of "
                     "which the map reads.", case("ramp", max_blur=6), (0,)),
    "FX-CBLUR-003": ("A white solid as the map, Maximum Blur 6: every pixel blurred the most, "
                     "document 21's Gaussian at sigma 2.", case("white", max_blur=6), (0,)),
    "FX-CBLUR-004": ("A black solid as the map: nothing is blurred.", case("black", max_blur=6),
                     (0,)),
    "FX-CBLUR-005": ("The black solid inverted: every pixel blurred the most, as FX-CBLUR-003.",
                     case("black", max_blur=6, invert="on"), (0,)),
    "FX-CBLUR-006": ("The ramp inverted: blurred at the left, sharp at the right.",
                     case("ramp", max_blur=6, invert="on"), (0,)),
    "FX-CBLUR-007": ("A grey solid, linear 0.25, luma 0.537: between the two largest levels, "
                     "everywhere the same.", case("grey", max_blur=6), (0,)),
    "FX-CBLUR-008": ("FX-CBLUR-002 with the edges repeated: the outer pixels are held rather "
                     "than fading into clear.", case("ramp", max_blur=6, edges="repeat"), (0,)),
    "FX-CBLUR-009": ("The ramp with Maximum Blur 0: nothing is blurred.",
                     case("ramp", max_blur=0), (0,)),
    "FX-CBLUR-010": ("A 4 by 2 checker as the map, centred: only the eight pixels it covers, "
                     "columns 6 to 9 of rows 4 and 5, are read; the white ones blurred.",
                     case("card", "center", max_blur=6), (0,)),
    "FX-CBLUR-011": ("The checker tiled: its pattern repeated over the whole layer.",
                     case("card", "tile", max_blur=6), (0,)),
    "FX-CBLUR-012": ("The checker stretched to 16 by 10, softened between its squares.",
                     case("card", "stretch", max_blur=6), (0,)),
    "FX-CBLUR-013": ("The holder names itself: its own brightness is the map; the cream squares "
                     "blurred most, the clear corner not at all.", case("holder", max_blur=6),
                     (0,)),
    "FX-CBLUR-014": ("Maximum Blur keyed from 0 at frame 0 to 12 at frame 4 with the ramp: frame "
                     "0 is untouched and frame 2 is FX-CBLUR-002.",
                     case("ramp", max_blur=keyed((0, 0), (4, 12))), (0, 2, 4)),
    "FX-CBLUR-015": ("A layer that is not in the composition, `gone`: nothing is blurred, and "
                     "the warning every frame.", case("gone", max_blur=6), (0, 4)),
    "FX-CBLUR-016": ("A white solid whose in point is frame 4: an empty map before it, so "
                     "nothing is blurred at frame 0 and all of it at frame 4.",
                     case("late", max_blur=6), (0, 4)),
    "FX-CBLUR-017": ("FX-CBLUR-016 inverted: all of it at frame 0, as FX-CBLUR-003, and nothing "
                     "at frame 4.", case("late", max_blur=6, invert="on"), (0, 4)),
    "FX-CBLUR-018": ("FX-CBLUR-002 on the holder moved 2 right and 1 down: the same picture "
                     "moved, since the map lies on the layer.",
                     case("ramp", max_blur=6, shift=(2, 1)), (0,)),
    "FX-CBLUR-019": ("The effect on an adjustment layer above the holder, the ramp as its map: "
                     "the map lies on the frame, which here is the holder's own rectangle, so "
                     "this is FX-CBLUR-002.", case("ramp", max_blur=6, on="adjust"), (0,)),
}

INVALID = {
    "FX-CBLUR-020": ("Maximum Blur 501, above 500.", case("ramp", max_blur=501)),
    "FX-CBLUR-021": ("Maximum Blur -1, below 0.", case("ramp", max_blur=-1)),
    "FX-CBLUR-022": ("Maximum Blur keyed to 600 at frame 4.",
                     case("ramp", max_blur=keyed((0, 6), (4, 600)))),
    "FX-CBLUR-023": ("A fit written \"fill\".", case("ramp", "fill", max_blur=6)),
    "FX-CBLUR-024": ("Invert written \"yes\".", case("ramp", max_blur=6, invert="yes")),
    "FX-CBLUR-025": ("Edges written \"wrap\".", case("ramp", max_blur=6, edges="wrap")),
    "FX-CBLUR-026": ("A layer written as the number 3, not a word.", case(3, max_blur=6)),
}


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.compound_blur", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in ("layer", "fit", "max_blur", "invert",
                                                          "edges")}}


def map_layers():
    return [L.raster("ramp", "asset-ramp", position=(3, 2), scale=(50, 50), enabled=False,
                     out_frame=FRAMES),
            L.solid("white", W, H, [1.0] * 3, enabled=False, out_frame=FRAMES),
            L.solid("black", W, H, [0.0] * 3, enabled=False, out_frame=FRAMES),
            L.solid("grey", W, H, GREY, enabled=False, out_frame=FRAMES),
            L.raster("card", "asset-card", enabled=False, out_frame=FRAMES),
            L.solid("late", W, H, [1.0] * 3, enabled=False, in_frame=4, out_frame=FRAMES)]


def project(pid, layers):
    return {"schema_version": 0, "project_id": pid,
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, FRAMES, layers)]}


def project_json(fx, c):
    holder = L.raster("holder", "asset-holder", position=c["shift"], out_frame=FRAMES)
    layers = [holder] + map_layers()
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = [effect("fx-1", c)]
        layers = [adjust] + layers
    else:
        holder["effects"] = [effect("fx-1", c)]
    return project("proj-" + fx.lower(), layers)


def write(name, p):
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def drawing(id, *effects):
    lay = L.raster(id, "asset-holder", out_frame=FRAMES)
    lay["effects"] = [dict(effect(f"fx-{id}-{k}", case(named, max_blur=m)), enabled=on)
                      for k, (named, m, on) in enumerate(effects)]
    return lay


# Layer settings that go round, or seem to. Each is a whole project; the first three are refused.
LOADS = {
    "cycle_two.json": ("`a` names `b` and `b` names `a`: refused, `EFFECT_LAYER_CYCLE`.",
                       [drawing("a", ("b", 6, True)), drawing("b", ("a", 6, True))]),
    "cycle_three.json": ("`a` names `b`, `b` names `c`, `c` names `a`: refused.",
                         [drawing("a", ("b", 6, True)), drawing("b", ("c", 6, True)),
                          drawing("c", ("a", 6, True))]),
    "cycle_off.json": ("`a` names `b` and `b` names `a` by an effect switched off: still refused, "
                       "since switching it on would close the circle.",
                       [drawing("a", ("b", 6, True)), drawing("b", ("a", 6, False))]),
}


def adjust_chain():
    a = drawing("a", ("b", 6, True))
    b = L.adjustment("b", out_frame=FRAMES)
    b["effects"] = [effect("fx-b-0", case("a", max_blur=0))]
    return [b, a]


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}, "loads": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write(f"{fx.lower().replace('-', '_')}.json",
                                                project_json(fx, c)), "frames": rendered}
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
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)),
            "frames": {"0": before, "4": before}, "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    for name, (says, layers) in LOADS.items():
        write(name, project("proj-" + name[:-5], layers + map_layers()))
        expected["loads"][name] = {"says": says, "refused": "EFFECT_LAYER_CYCLE"}
    write("chain_adjustment.json", project("proj-chain-adjustment", adjust_chain() + map_layers()))
    expected["loads"]["chain_adjustment.json"] = {
        "says": "`a`, a drawing, names `b`, an adjustment layer, whose Compound Blur, Maximum Blur 0, "
                "names `a`: not a circle, since an adjustment layer's map is its white rectangle and "
                "its effects are not run for it. It opens, and `a` is blurred the most everywhere, "
                "as FX-CBLUR-003.",
        "frames": {"0": render(case("white", max_blur=6), 0)}}

    (OUT / "expected_compound_blur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    full = [list(L.at(gaussian(decoded("holder"), 2, "transparent"), x, y))
            for y in range(H) for x in range(W)]
    for fx in ("FX-CBLUR-001", "FX-CBLUR-004", "FX-CBLUR-009", "FX-CBLUR-015", "FX-CBLUR-016"):
        assert c[fx]["0"] == drawn, fx
    assert near(c["FX-CBLUR-003"]["0"], full)
    for fx in ("FX-CBLUR-005", "FX-CBLUR-017"):
        assert near(c[fx]["0"], full), fx
    assert near(c["FX-CBLUR-016"]["4"], full) and near(c["FX-CBLUR-017"]["4"], drawn)
    assert c["FX-CBLUR-014"]["0"] == drawn and c["FX-CBLUR-014"]["2"] == c["FX-CBLUR-002"]["0"]
    assert c["FX-CBLUR-019"]["0"] == c["FX-CBLUR-002"]["0"]
    assert expected["loads"]["chain_adjustment.json"]["frames"]["0"] == c["FX-CBLUR-003"]["0"]
    col = lambda px, x: [px[y * W + x] for y in range(H)]  # noqa: E731
    # The ramp: column 0 is black, so sharp, column 15 white, so the full blur; inverted, the other
    # way round. Repeated edges change only what the blur reads from outside the layer.
    assert col(c["FX-CBLUR-002"]["0"], 0) == col(drawn, 0)
    assert near(col(c["FX-CBLUR-002"]["0"], 15), col(full, 15))
    assert near(col(c["FX-CBLUR-006"]["0"], 15), col(drawn, 15))
    assert near(col(c["FX-CBLUR-006"]["0"], 0), col(full, 0))
    assert c["FX-CBLUR-008"]["0"] != c["FX-CBLUR-002"]["0"]
    assert col(c["FX-CBLUR-008"]["0"], 0) == col(drawn, 0)
    # Grey: between the plain picture and the full blur, and the map's luma 0.537.
    assert abs(luma(GREY + [1.0]) - 0.5371) < 1e-4
    # The centred checker: only its eight pixels read, so the rest untouched.
    for y in range(H):
        for x in range(W):
            if not (6 <= x < 10 and 4 <= y < 6):
                assert c["FX-CBLUR-010"]["0"][y * W + x] == drawn[y * W + x], (x, y)
    assert c["FX-CBLUR-011"]["0"] != c["FX-CBLUR-012"]["0"]
    # The self map: the clear corner stays clear and untouched, for its luma is 0.
    assert all(c["FX-CBLUR-013"]["0"][y * W + x] == CLEAR for y in range(7, H) for x in range(12, W))
    # Moved: the same picture, two right and one down.
    moved = c["FX-CBLUR-018"]["0"]
    assert all(moved[(y + 1) * W + x + 2] == c["FX-CBLUR-002"]["0"][y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    # Blurring moves light and never makes it: every case's covering stays within 0 and 1.
    assert all(-1e-12 <= p[3] <= 1 + 1e-12 for v in c.values() for px in v.values() for p in px)
    print("checks passed")


if __name__ == "__main__":
    main()
