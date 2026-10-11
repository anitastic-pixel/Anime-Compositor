"""3D Glasses, worked a second way.

D-506 adds 3D Glasses (`core.three_d_glasses`), after After Effects' 3D Glasses (Perspective,
EFFECTS.md row 335): a left and a right view, each a layer of the composition named by D-189's
layer setting (`tools/effect_layer_reference.py`) or the layer itself, are put together as one
stereo picture: an anaglyph for coloured glasses, interlaced rows, or the two side by side or one
over the other. Adobe's help names the settings (Left View, Right View, Scene Convergence,
Vertical Alignment, Units, Swap Left-Right, 3D View, Balance) and what each mode does, but
publishes no formula and no defaults we could read, so the rule below and the settings when added
are ours, the "rule ours" precedent.

At the holder's composition frame n, with its input O, linear and premultiplied, whose drawing's
own top-left pixel is at (ox, oy) however far earlier effects grew it, and whose drawing is its
own W by H:

1. Each view is read in the drawing's own space. A view with no layer named ("") is O itself:
   V(u, v) = O(u + ox, v + oy). A view naming a layer is D-189's map of it, fitted to W by H by
   `center` (After Effects asks for the two layers to be the same size): V(u, v) = map(u, v).
   Either is transparent outside its picture. If either view names a layer that is not in the
   composition (D-189's warning), the output is O.
2. With `swap_left_right` on, the two views change places.
3. The distances: `scene_convergence` c and `vertical_alignment` v are pixels with `units`
   `pixels`, or per cent of W (c) and of H (v) with `percent`.
4. A view is read at a point (s, t) by document 21's bilinear (pixel centres at halves,
   transparent outside). At output pixel (x, y), in the drawing's own space x = X - ox,
   y = Y - oy:
   - the anaglyphs and `interlace`: l = L(x + 1/2 - c/2, y + 1/2 + v/2) and
     r = R(x + 1/2 + c/2, y + 1/2 - v/2); the left view moves c/2 right and v/2 up, the right
     view c/2 left and v/2 down.
   - `side_by_side` (c and v unused): where 2x < W, L(2x + 1, y + 1/2); elsewhere
     R(2x + 1 - W, y + 1/2). Each view is squeezed to half the width, the mean of each two
     columns.
   - `over_under` likewise down the picture: where 2y < H, L(x + 1/2, 2y + 1); elsewhere
     R(x + 1/2, 2y + 1 - H).
5. `interlace`: rows with y even (the upper field) are l, odd rows r.
6. The balanced modes first draw the two views toward each other: m = (l + r) / 2, l' =
   m + b (l - m), r' = m + b (r - m), every channel, b the `balance`. At 0 the views are one and
   there is no depth; above 1 they are pushed apart, more saturated. The other modes take l and r
   as they are.
7. With Y(p) = 0.2126 p.r + 0.7152 p.g + 0.0722 p.b (linear, premultiplied) and A = the larger
   alpha of the two: `red_green` and `balanced_red_green` are (Y(r), Y(l), 0, A); `red_blue` and
   `balanced_red_blue` are (Y(r), Y(l), Y(l), A), the blue lens being cyan; and
   `balanced_colored_red_blue` is (r'.r, l'.g, l'.b, A), the original colours kept. Colour is
   then held at 0 or above and the alpha within 0 to 1.

`left_view` and `right_view` D-189's word, "" when added; `scene_convergence` and
`vertical_alignment` -1000 to 1000, 0 when added; `units` `pixels` when added or `percent`;
`swap_left_right` `off` when added or `on`; `three_d_view` one of `side_by_side`, `over_under`,
`interlace`, `red_green`, `red_blue`, `balanced_red_green`, `balanced_red_blue` and
`balanced_colored_red_blue`, the last when added; `balance` 0 to 10, 0.75 when added. The two
distances and the balance are keyable. As added, both views are the layer itself, so nothing
changes.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/three_d_glasses/`:
the view layers, every one switched off, and on top of them the layer `holder`, showing the
drawing `shot`, with the effect. The expected pixels are in
`Fixtures/three_d_glasses/expected_three_d_glasses.json`, with a project whose layer settings go
round in a circle.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/three_d_glasses_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import compound_blur_reference as K  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "three_d_glasses"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
GREY = (120, 130, 150, 255)
ORANGE = (240, 140, 40, 255)

PAIRS = ("red_green", "red_blue", "balanced_red_green", "balanced_red_blue",
         "balanced_colored_red_blue")
VIEWS = ("side_by_side", "over_under", "interlace") + PAIRS


def square(x0):
    """A grey card with an orange square 4 by 3, its left column at x0, rows 3 to 5, and a
    half-covering white dot at (12, 7)."""
    return [[ORANGE if x0 <= x < x0 + 4 and 3 <= y <= 5 else
             (255, 255, 255, 128) if (x, y) == (12, 7) else GREY for x in range(W)]
            for y in range(H)]


DRAWINGS = {
    # Red, blue and cream squares, the bottom right 4 by 3 clear.
    "shot": K.DRAWINGS["holder"],
    # The stereo pair: the square 2 pixels further right in the right eye's view.
    "left": square(5),
    "right": square(7),
    # The left view's middle 8 by 6, (4, 2) to (11, 7): centred on 16 by 10 it lies where it was cut.
    "small": [row[4:12] for row in square(5)[2:8]],
}


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


MAPS = {"left": "left", "right": "right", "small": "small", "late": "right"}


# --- the rule -------------------------------------------------------------------------------

def luma(p):
    return 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2]


def glasses(lv, rv, view, c, v, b):
    """Steps 3 to 7 on the two views lv and rv, each a picture W by H in the drawing's space."""
    out = []
    for y in range(H):
        for x in range(W):
            if view == "side_by_side":
                out.append(L.bilinear(lv, 2 * x + 1, y + 0.5) if 2 * x < W else
                           L.bilinear(rv, 2 * x + 1 - W, y + 0.5))
                continue
            if view == "over_under":
                out.append(L.bilinear(lv, x + 0.5, 2 * y + 1) if 2 * y < H else
                           L.bilinear(rv, x + 0.5, 2 * y + 1 - H))
                continue
            lp = L.bilinear(lv, x + 0.5 - c / 2, y + 0.5 + v / 2)
            rp = L.bilinear(rv, x + 0.5 + c / 2, y + 0.5 - v / 2)
            if view == "interlace":
                out.append(lp if y % 2 == 0 else rp)
                continue
            if view.startswith("balanced"):
                m = [(p + q) / 2 for p, q in zip(lp, rp)]
                lp = [mi + b * (p - mi) for mi, p in zip(m, lp)]
                rp = [mi + b * (q - mi) for mi, q in zip(m, rp)]
            a = max(lp[3], rp[3])
            if view.endswith("red_green"):
                px = [luma(rp), luma(lp), 0.0, a]
            elif view == "balanced_colored_red_blue":
                px = [rp[0], lp[1], lp[2], a]
            else:
                px = [luma(rp), luma(lp), luma(lp), a]
            out.append([max(0.0, px[0]), max(0.0, px[1]), max(0.0, px[2]),
                        min(1.0, max(0.0, px[3]))])
    return L.pic(W, H, out)


# --- the cases ------------------------------------------------------------------------------

def case(view="balanced_colored_red_blue", left="", right="", conv=0, vert=0, units="pixels",
         swap="off", balance=0.75, shift=(0, 0), on="holder"):
    return {"left_view": left, "right_view": right, "scene_convergence": conv,
            "vertical_alignment": vert, "units": units, "swap_left_right": swap,
            "three_d_view": view, "balance": balance, "shift": shift, "on": on}


def view_picture(name, frame):
    """Step 1 for one view: the picture, or None for a layer not in the composition."""
    if name in ("", "holder"):
        return decoded("shot")  # no effects before this one, so the input is the drawing
    if name not in MAPS:
        return None
    p = L.EMPTY if name == "late" and frame < 4 else decoded(MAPS[name])
    return L.fit(p, "center", W, H)


def worked(c, frame):
    o = decoded("shot")
    lv, rv = view_picture(c["left_view"], frame), view_picture(c["right_view"], frame)
    if lv is None or rv is None:
        return o
    if c["swap_left_right"] == "on":
        lv, rv = rv, lv
    conv, vert = value_at(c["scene_convergence"], frame), value_at(c["vertical_alignment"], frame)
    if c["units"] == "percent":
        conv, vert = conv / 100 * W, vert / 100 * H
    return glasses(lv, rv, c["three_d_view"], conv, vert, value_at(c["balance"], frame))


def render(c, frame):
    """The composition's pixels at frame: the holder with its effect, moved by its shift; or,
    with the effect on an adjustment layer above, the frame below through it."""
    out = worked(c, frame)
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded("shot")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


def pair(view, **kw):
    return case(view, "left", "right", **kw)


CASES = {
    "FX-GLASSES-001": ("As added: both views the layer itself, balanced colored red blue, so "
                       "nothing changes.", case(), (0,)),
    "FX-GLASSES-002": ("The stereo pair as red blue: the right view's grey in red, the left's "
                       "in green and blue; the orange square red-fringed on its right, cyan on "
                       "its left.", pair("red_blue"), (0,)),
    "FX-GLASSES-003": ("The pair as red green: the left view's grey in green only, blue empty.",
                       pair("red_green"), (0,)),
    "FX-GLASSES-004": ("The pair as balanced red blue at 0.75: the fringes a quarter softer than "
                       "FX-GLASSES-002's.", pair("balanced_red_blue"), (0,)),
    "FX-GLASSES-005": ("The pair as balanced red green at 0.75.", pair("balanced_red_green"),
                       (0,)),
    "FX-GLASSES-006": ("The pair as balanced colored red blue at 0.75: the colours kept where "
                       "the views agree, red and cyan fringes where the square differs.",
                       pair("balanced_colored_red_blue"), (0,)),
    "FX-GLASSES-007": ("The pair side by side: the left view squeezed into the left half, the "
                       "right view into the right half.", pair("side_by_side"), (0,)),
    "FX-GLASSES-008": ("The pair over under: the left view squeezed into the top half, the right "
                       "into the bottom.", pair("over_under"), (0,)),
    "FX-GLASSES-009": ("The pair interlaced: even rows from the left view, odd rows from the "
                       "right.", pair("interlace"), (0,)),
    "FX-GLASSES-010": ("Red blue with scene convergence 2: the left view 1 pixel right, the right "
                       "view 1 pixel left, so the squares lie 0 apart and line up.",
                       pair("red_blue", conv=2), (0,)),
    "FX-GLASSES-011": ("Scene convergence 3: each view moved a pixel and a half, read between "
                       "pixels.", pair("red_blue", conv=3), (0,)),
    "FX-GLASSES-012": ("Vertical alignment 2: the left view 1 pixel up, the right 1 pixel down.",
                       pair("red_blue", vert=2), (0,)),
    "FX-GLASSES-013": ("In per cent: convergence 12.5 of 16 is 2 and alignment 20 of 10 is 2, so "
                       "this is convergence 2 and alignment 2 in pixels.",
                       pair("red_blue", conv=12.5, vert=20, units="percent"), (0,)),
    "FX-GLASSES-014": ("FX-GLASSES-013 written in pixels.", pair("red_blue", conv=2, vert=2),
                       (0,)),
    "FX-GLASSES-015": ("FX-GLASSES-002 with left and right swapped.",
                       pair("red_blue", swap="on"), (0,)),
    "FX-GLASSES-016": ("Balanced colored at balance 0: the two views made one, the mean of the "
                       "two, no depth.", pair("balanced_colored_red_blue", balance=0), (0,)),
    "FX-GLASSES-017": ("Balanced colored at balance 2: the views pushed apart, the fringes "
                       "stronger, colour held at 0.", pair("balanced_colored_red_blue", balance=2),
                       (0,)),
    "FX-GLASSES-018": ("Convergence keyed from 0 at frame 0 to 4 at frame 4: frame 0 is "
                       "FX-GLASSES-002, frame 2 FX-GLASSES-010.",
                       pair("red_blue", conv=keyed((0, 0), (4, 4))), (0, 2, 4)),
    "FX-GLASSES-019": ("A left view that is not in the composition, `gone`: nothing changes, and "
                       "the warning every frame.", case("red_blue", "gone", "right"), (0, 4)),
    "FX-GLASSES-020": ("The right view with its in point at frame 4: before it the right view is "
                       "empty, so red is empty; at frame 4 it is FX-GLASSES-002.",
                       case("red_blue", "left", "late"), (0, 4)),
    "FX-GLASSES-021": ("FX-GLASSES-006 on the holder moved 2 right and 1 down: the same picture "
                       "moved, since the views lie on the layer.",
                       pair("balanced_colored_red_blue", shift=(2, 1)), (0,)),
    "FX-GLASSES-022": ("The effect on an adjustment layer above the holder, the pair named: the "
                       "views lie on the frame, the composition's size, so this is "
                       "FX-GLASSES-006.", pair("balanced_colored_red_blue", on="adjust"), (0,)),
    "FX-GLASSES-023": ("Only the left view named, the right the layer itself, red blue.",
                       case("red_blue", "left", ""), (0,)),
    "FX-GLASSES-024": ("The left view's middle 8 by 6 as the left view, centred: it lies where it "
                       "was cut, clear around it.", case("red_blue", "small", "right"), (0,)),
    "FX-GLASSES-025": ("Side by side with convergence 5 and alignment 3: both unused, so this is "
                       "FX-GLASSES-007.", pair("side_by_side", conv=5, vert=3), (0,)),
    "FX-GLASSES-026": ("The layer itself as both views, red blue, convergence 2: the shot as a "
                       "grey anaglyph, its edges fringed.", case("red_blue", conv=2), (0,)),
}

INVALID = {
    "FX-GLASSES-027": ("Scene convergence 1001, above 1000.", pair("red_blue", conv=1001)),
    "FX-GLASSES-028": ("Scene convergence -1001, below -1000.", pair("red_blue", conv=-1001)),
    "FX-GLASSES-029": ("Vertical alignment 1001, above 1000.", pair("red_blue", vert=1001)),
    "FX-GLASSES-030": ("Balance 11, above 10.", pair("red_blue", balance=11)),
    "FX-GLASSES-031": ("Balance -1, below 0.", pair("red_blue", balance=-1)),
    "FX-GLASSES-032": ("A 3D view written \"anaglyph\".", pair("anaglyph")),
    "FX-GLASSES-033": ("Units written \"inches\".", pair("red_blue", units="inches")),
    "FX-GLASSES-034": ("Swap written \"yes\".", pair("red_blue", swap="yes")),
    "FX-GLASSES-035": ("A left view written as the number 3, not a word.",
                       case("red_blue", 3, "right")),
    "FX-GLASSES-036": ("A right view written as the number 3, not a word.",
                       case("red_blue", "left", 3)),
    "FX-GLASSES-037": ("Balance keyed to 20 at frame 4.",
                       pair("red_blue", balance=keyed((0, 0.75), (4, 20)))),
}

KEYS = ("left_view", "right_view", "scene_convergence", "vertical_alignment", "units",
        "swap_left_right", "three_d_view", "balance")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.three_d_glasses", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in KEYS}}


def map_layers():
    return [L.raster(n, "asset-" + n, enabled=False, out_frame=FRAMES)
            for n in ("left", "right", "small")] + [
        L.raster("late", "asset-right", enabled=False, in_frame=4, out_frame=FRAMES)]


def project(pid, layers):
    return {"schema_version": 0, "project_id": pid,
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, FRAMES, layers)]}


def project_json(fx, c):
    """Layers are drawn first to last, so the holder is written after the views and an
    adjustment layer after the holder."""
    holder = L.raster("holder", "asset-shot", position=c["shift"], out_frame=FRAMES)
    layers = map_layers() + [holder]
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = [effect("fx-1", c)]
        layers.append(adjust)
    else:
        holder["effects"] = [effect("fx-1", c)]
    return project("proj-" + fx.lower(), layers)


def write(name, p):
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def drawing(id, fx):
    lay = L.raster(id, "asset-shot", out_frame=FRAMES)
    lay["effects"] = [fx]
    return lay


# Layer settings that go round in a circle: a whole project, refused on opening.
LOADS = {
    "cycle_two.json": ("`a` and `b`, each the shot, each with 3D Glasses whose left view is the "
                       "other: refused, `EFFECT_LAYER_CYCLE`.",
                       [drawing("a", effect("fx-a-0", case("red_blue", "b"))),
                        drawing("b", effect("fx-b-0", case("red_blue", "a")))]),
}


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}, "loads": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write(f"{fx.lower().replace('-', '_')}.json",
                                                project_json(fx, c)), "frames": rendered}
        if "gone" in (c["left_view"], c["right_view"]):
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
        write(name, project("proj-" + name[:-5].replace("_", "-"), map_layers() + layers))
        expected["loads"][name] = {"says": says, "refused": "EFFECT_LAYER_CYCLE"}

    (OUT / "expected_three_d_glasses.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                       encoding="utf-8")
    check(expected)


def near(a, b, eps=1e-12):
    return all(abs(x - y) <= eps for p, q in zip(a, b) for x, y in zip(p, q))


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    lv, rv = decoded("left")["px"], decoded("right")["px"]
    # As added, and a missing view: nothing changes, exactly.
    assert c["FX-GLASSES-001"]["0"] == drawn
    assert c["FX-GLASSES-019"]["0"] == drawn and c["FX-GLASSES-019"]["4"] == drawn
    # Red blue: red the right view's luma, green and blue the left's, opaque.
    for i, p in enumerate(c["FX-GLASSES-002"]["0"]):
        assert near([p], [[luma(rv[i]), luma(lv[i]), luma(lv[i]), max(lv[i][3], rv[i][3])]]), i
    for i, p in enumerate(c["FX-GLASSES-003"]["0"]):
        assert near([p], [[luma(rv[i]), luma(lv[i]), 0.0, max(lv[i][3], rv[i][3])]]), i
    # Where the views agree (the grey) every anaglyph is that grey's luma, balanced or not.
    grey = 0
    for i in range(W * H):
        if lv[i] == rv[i]:
            grey += 1
            y = luma(lv[i])
            for fx in ("FX-GLASSES-002", "FX-GLASSES-004"):
                assert near([c[fx]["0"][i]], [[y, y, y, lv[i][3]]]), (fx, i)
            assert near([c["FX-GLASSES-006"]["0"][i]], [lv[i]]), i
    assert grey > W * H // 2
    # Balance 0: both views the mean, so the colored mode shows the mean picture.
    for i, p in enumerate(c["FX-GLASSES-016"]["0"]):
        m = [(a + b) / 2 for a, b in zip(lv[i], rv[i])]
        assert near([p], [m]), i
    # Balance 2 pushes past: somewhere a colour is held at 0.
    assert any(v == 0.0 for p in c["FX-GLASSES-017"]["0"] for v in p[:3])
    # Side by side: the left half is the left view's columns 2x and 2x + 1 averaged.
    for y in range(H):
        for x in range(W // 2):
            want = [(a + b) / 2 for a, b in zip(lv[y * W + 2 * x], lv[y * W + 2 * x + 1])]
            assert near([c["FX-GLASSES-007"]["0"][y * W + x]], [want]), (x, y)
            want = [(a + b) / 2 for a, b in zip(rv[y * W + 2 * x], rv[y * W + 2 * x + 1])]
            assert near([c["FX-GLASSES-007"]["0"][y * W + x + W // 2]], [want]), (x, y)
    assert c["FX-GLASSES-025"]["0"] == c["FX-GLASSES-007"]["0"]
    # Over under: the top half is the left view's rows 2y and 2y + 1 averaged.
    for y in range(H // 2):
        for x in range(W):
            want = [(a + b) / 2 for a, b in zip(lv[2 * y * W + x], lv[(2 * y + 1) * W + x])]
            assert near([c["FX-GLASSES-008"]["0"][y * W + x]], [want]), (x, y)
    # Interlace: even rows the left view, odd the right, untouched.
    for i, p in enumerate(c["FX-GLASSES-009"]["0"]):
        assert p == (lv[i] if (i // W) % 2 == 0 else rv[i]), i
    # Convergence 2: the left read 1 to the left, the right 1 to the right, so both squares lie
    # at columns 6 to 9 and the square's middle is the plain orange luma in all three channels.
    conv = c["FX-GLASSES-010"]["0"]
    y_orange = luma(lv[3 * W + 5])
    for x in range(6, 10):
        assert near([conv[4 * W + x]], [[y_orange, y_orange, y_orange, 1.0]]), x
    # Per cent and pixels agree; the keyed frames agree with the fixed ones.
    assert near(c["FX-GLASSES-013"]["0"], c["FX-GLASSES-014"]["0"])
    assert c["FX-GLASSES-018"]["0"] == c["FX-GLASSES-002"]["0"]
    assert near(c["FX-GLASSES-018"]["2"], c["FX-GLASSES-010"]["0"])
    # Swapped: red now the left view's luma.
    for i, p in enumerate(c["FX-GLASSES-015"]["0"]):
        assert near([p], [[luma(lv[i]), luma(rv[i]), luma(rv[i]), max(lv[i][3], rv[i][3])]]), i
    # Late: before its in point red is empty; at it, FX-GLASSES-002.
    assert all(p[0] == 0.0 for p in c["FX-GLASSES-020"]["0"])
    assert c["FX-GLASSES-020"]["4"] == c["FX-GLASSES-002"]["0"]
    # On an adjustment layer: the same as on the holder.
    assert c["FX-GLASSES-022"]["0"] == c["FX-GLASSES-006"]["0"]
    # Moved: the same picture, two right and one down.
    moved = c["FX-GLASSES-021"]["0"]
    assert all(moved[(y + 1) * W + x + 2] == c["FX-GLASSES-006"]["0"][y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    # The small left view: clear around its 8 by 6, so green is empty there.
    small = c["FX-GLASSES-024"]["0"]
    assert small[0][1] == 0.0 and small[0][0] > 0.0
    print("checks passed")


if __name__ == "__main__":
    main()
