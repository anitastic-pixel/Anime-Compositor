"""Gradient Wipe, worked a second way.

D-194 proposes Gradient Wipe (`core.gradient_wipe`): a transition that wipes a layer away in the
order of another layer's brightness, the darkest parts first. The other layer is its map, D-189's:
the named layer's picture fitted to the holder, `tools/effect_layer_reference.py`. This file pins
what is done with it.

At the holder's composition frame n, with its input O, linear and premultiplied, whose drawing's
own top-left pixel is at (ox, oy) however far earlier effects grew it:

1. With no map (an empty name, a missing layer) or `completion` 0, the output is O. With
   `completion` 100 every pixel is (0, 0, 0, 0).
2. The map M is D-189's, fitted to the drawing's own W by H. Pixel (x, y) of O reads
   M(x - ox, y - oy), transparent outside M, and v is its picture luma, the sRGB encoding of
   clamp(0.2126 r + 0.7152 g + 0.0722 b, 0, 1), as Compound Blur reads a map, so a clear map
   reads as black. With `invert` `on`, v is 1 - v.
3. c = completion / 100, s = softness / 100 and E = -s / 2 + c (1 + s). k = clamp((v - E) / s +
   0.5, 0, 1) when s > 0; otherwise k = 1 where v >= E and 0 elsewhere. The output is O k, all four
   numbers. So E runs from below the darkest to above the brightest as completion goes from 0 to
   100, and the softness is how far below E a pixel starts to go: at completion 50 and softness
   100, k is v itself.
4. For a draft the map is made at the holder's size (D-189 step 3); no setting is a distance.

`layer` D-189's word, "" when added; `fit` `stretch` when added, `center` or `tile`;
`completion` and `softness`, 0 to 100, 0 when added, keyable; `invert` `off` or `on`.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/gradient_wipe/`:
the map layers, every one switched off, and on top of them the drawing `holder` with the effect,
so that only the holder is seen. The expected pixels are in
`Fixtures/gradient_wipe/expected_gradient_wipe.json`, with two projects whose layer settings go
round in a circle.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/gradient_wipe_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import compound_blur_reference as K  # noqa: E402
import displacement_map_reference as D  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "gradient_wipe"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
CLEAR = [0.0, 0.0, 0.0, 0.0]
CLIFF = 1e-5  # no pixel decided by a hard edge lies this near it

DRAWINGS = {n: D.DRAWINGS[n] for n in ("holder", "ramp", "card", "veil")}
MAPS = {  # what each map layer's picture is at frame 0; `late` starts at frame 4
    "ramp": D.decoded("ramp"), "white": K.solid_pic([1.0] * 3), "black": K.solid_pic([0.0] * 3),
    "card": D.decoded("card"), "veil": D.decoded("veil"), "late": K.solid_pic([1.0] * 3),
}


# --- the rule -------------------------------------------------------------------------------

def wipe(o, m, completion, softness, invert):
    """Steps 1 to 3 on the picture o, the map m already fitted to it, or None."""
    if m is None or completion == 0:
        return o
    if completion == 100:
        return L.pic(o["w"], o["h"], [list(CLEAR) for _ in range(o["w"] * o["h"])])
    c, s = completion / 100, softness / 100
    edge = -s / 2 + c * (1 + s)
    out = []
    for i, p in enumerate(o["px"]):
        v = K.luma(m["px"][i])
        if invert == "on":
            v = 1 - v
        if s > 0:
            k = min(1.0, max(0.0, (v - edge) / s + 0.5))
        else:
            assert abs(v - edge) > CLIFF, ("a pixel on the cliff", i, v, edge)
            k = 1.0 if v >= edge else 0.0
        out.append([q * k for q in p])
    return L.pic(o["w"], o["h"], out)


# --- the cases ------------------------------------------------------------------------------

def case(layer="", fit="stretch", completion=0, softness=0, invert="off", shift=(0, 0),
         on="holder"):
    return {"layer": layer, "fit": fit, "completion": completion, "softness": softness,
            "invert": invert, "shift": shift, "on": on}


def the_map(c, frame):
    if c["layer"] == "":
        return None
    if c["layer"] == "holder":
        return D.decoded("holder")  # steps 1 and 2 only: the drawing, no mask
    if c["layer"] not in MAPS:
        return None
    p = L.EMPTY if c["layer"] == "late" and frame < 4 else MAPS[c["layer"]]
    return L.fit(p, c["fit"], W, H)


def render(c, frame):
    """The composition's pixels at frame: the holder with its effect, moved by its shift; or,
    with the effect on an adjustment layer above, the frame below through it."""
    out = wipe(D.decoded("holder"), the_map(c, frame), value_at(c["completion"], frame),
               value_at(c["softness"], frame), c["invert"])
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = D.decoded("holder")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


CASES = {
    "FX-GWIPE-001": ("As added: no layer named and completion 0, so nothing changes.", case(), (0,)),
    "FX-GWIPE-002": ("No layer named, completion 50: with no map nothing changes.",
                     case(completion=50), (0,)),
    "FX-GWIPE-003": ("The grey ramp as the map, completion 50, softness 0: its eight dark columns, "
                     "0 to 7, are wiped away and the eight light ones kept whole. The ramp layer is "
                     "moved, scaled and switched off, none of which the map reads.",
                     case("ramp", completion=50), (0,)),
    "FX-GWIPE-004": ("The ramp, completion 50, softness 100: each pixel kept by the ramp's "
                     "brightness there, column 0 gone, column 15 whole, the rest between.",
                     case("ramp", completion=50, softness=100), (0,)),
    "FX-GWIPE-005": ("The ramp, completion 0, softness 50: nothing changes.",
                     case("ramp", softness=50), (0,)),
    "FX-GWIPE-006": ("The ramp, completion 100, softness 50: every pixel clear.",
                     case("ramp", completion=100, softness=50), (0,)),
    "FX-GWIPE-007": ("The ramp, completion 100, softness 0: every pixel clear, the white column "
                     "too.", case("ramp", completion=100), (0,)),
    "FX-GWIPE-008": ("FX-GWIPE-003 with Invert on: the eight light columns go, the dark ones stay.",
                     case("ramp", completion=50, invert="on"), (0,)),
    "FX-GWIPE-009": ("The ramp, completion 30, softness 20: columns 0 to 2 gone, 6 and on whole, "
                     "3 to 5 fading in between.", case("ramp", completion=30, softness=20), (0,)),
    "FX-GWIPE-010": ("A white solid, completion 99: nothing is dark enough to go.",
                     case("white", completion=99), (0,)),
    "FX-GWIPE-011": ("A black solid, completion 1: everything goes at once.",
                     case("black", completion=1), (0,)),
    "FX-GWIPE-012": ("A white map clear at the top and more covering down each row, completion "
                     "50: a clear map reads as black, so rows 0 and 1 go and the rest stay.",
                     case("veil", completion=50), (0,)),
    "FX-GWIPE-013": ("A 4 by 2 checker of white and black, centred, completion 50: outside it the "
                     "map is clear, black, so only its four white pixels are kept.",
                     case("card", "center", completion=50), (0,)),
    "FX-GWIPE-014": ("The checker tiled: kept and gone in a checker over the whole layer.",
                     case("card", "tile", completion=50), (0,)),
    "FX-GWIPE-015": ("The checker stretched to 16 by 10, completion 50, softness 30: soft squares.",
                     case("card", "stretch", completion=50, softness=30), (0,)),
    "FX-GWIPE-016": ("The holder names itself, completion 50: its red and blue squares, darker "
                     "than half, go, its cream ones stay, and the clear corner stays clear.",
                     case("holder", completion=50), (0,)),
    "FX-GWIPE-017": ("Completion keyed from 0 at frame 0 to 100 at frame 4, the ramp: frame 0 "
                     "whole, frame 2 as FX-GWIPE-003, frame 4 clear.",
                     case("ramp", completion=keyed((0, 0), (4, 100))), (0, 2, 4)),
    "FX-GWIPE-018": ("Softness keyed from 0 at frame 0 to 100 at frame 4, completion 50, the ramp: "
                     "frame 0 as FX-GWIPE-003, frame 4 as FX-GWIPE-004.",
                     case("ramp", completion=50, softness=keyed((0, 0), (4, 100))), (0, 2, 4)),
    "FX-GWIPE-019": ("A layer that is not in the composition, `gone`, completion 50: nothing "
                     "changes, and the warning every frame.", case("gone", completion=50), (0, 4)),
    "FX-GWIPE-020": ("A white solid whose in point is frame 4, completion 50: before it the map is "
                     "empty, which reads as black, so at frame 0 everything is gone; at frame 4 "
                     "nothing is.", case("late", completion=50), (0, 4)),
    "FX-GWIPE-021": ("FX-GWIPE-003 on the holder moved 2 right and 1 down: the same picture "
                     "moved, since the map lies on the layer.",
                     case("ramp", completion=50, shift=(2, 1)), (0,)),
    "FX-GWIPE-022": ("The effect on an adjustment layer above the holder, the ramp as its map: "
                     "the map lies on the frame, which here is the holder's own rectangle, so "
                     "this is FX-GWIPE-003.", case("ramp", completion=50, on="adjust"), (0,)),
}

INVALID = {
    "FX-GWIPE-023": ("Completion 101, above 100.", case("ramp", completion=101)),
    "FX-GWIPE-024": ("Softness -1, below 0.", case("ramp", softness=-1)),
    "FX-GWIPE-025": ("Completion keyed to 200 at frame 4.",
                     case("ramp", completion=keyed((0, 30), (4, 200)))),
    "FX-GWIPE-026": ("A fit written \"fill\".", case("ramp", "fill", completion=50)),
    "FX-GWIPE-027": ("Invert written \"yes\".", case("ramp", completion=50, invert="yes")),
    "FX-GWIPE-028": ("A layer written as the number 3, not a word.", case(3, completion=50)),
}

KEYS = ("layer", "fit", "completion", "softness", "invert")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.gradient_wipe", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in KEYS}}


def map_layers():
    return [L.raster("ramp", "asset-ramp", position=(3, 2), scale=(50, 50), enabled=False,
                     out_frame=FRAMES),
            L.solid("white", W, H, [1.0] * 3, enabled=False, out_frame=FRAMES),
            L.solid("black", W, H, [0.0] * 3, enabled=False, out_frame=FRAMES),
            L.raster("card", "asset-card", enabled=False, out_frame=FRAMES),
            L.raster("veil", "asset-veil", enabled=False, out_frame=FRAMES),
            L.solid("late", W, H, [1.0] * 3, enabled=False, in_frame=4, out_frame=FRAMES)]


def project(pid, layers):
    return {"schema_version": 0, "project_id": pid,
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, FRAMES, layers)]}


def project_json(fx, c):
    """Layers are drawn first to last, so the holder is written after the map layers and an
    adjustment layer after the holder."""
    holder = L.raster("holder", "asset-holder", position=c["shift"], out_frame=FRAMES)
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
    lay = L.raster(id, "asset-holder", out_frame=FRAMES)
    lay["effects"] = [fx]
    return lay


# Layer settings that go round in a circle. Each is a whole project, refused on opening.
LOADS = {
    "cycle_two.json": ("`a` and `b`, each the holder's drawing, each with a Gradient Wipe naming "
                       "the other: refused, `EFFECT_LAYER_CYCLE`.",
                       [drawing("a", effect("fx-a-0", case("b", completion=50))),
                        drawing("b", effect("fx-b-0", case("a", completion=50)))]),
    "cycle_mixed.json": ("`a`'s Gradient Wipe names `b`, and `b`'s Displacement Map names `a`: a "
                         "circle through two kinds of effect, refused the same.",
                         [drawing("a", effect("fx-a-0", case("b", completion=50))),
                          drawing("b", D.effect("fx-b-0", D.case("a", max_h=2, max_v=2)))]),
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
        write(name, project("proj-" + name[:-5].replace("_", "-"), map_layers() + layers))
        expected["loads"][name] = {"says": says, "refused": "EFFECT_LAYER_CYCLE"}

    (OUT / "expected_gradient_wipe.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    gone = [CLEAR] * (W * H)
    col = lambda px, x: [px[y * W + x] for y in range(H)]  # noqa: E731
    for fx in ("FX-GWIPE-001", "FX-GWIPE-002", "FX-GWIPE-005", "FX-GWIPE-010"):
        assert c[fx]["0"] == drawn, fx
    assert c["FX-GWIPE-019"]["0"] == drawn and c["FX-GWIPE-019"]["4"] == drawn
    for fx in ("FX-GWIPE-006", "FX-GWIPE-007", "FX-GWIPE-011"):
        assert c[fx]["0"] == gone, fx
    assert c["FX-GWIPE-020"]["0"] == gone and c["FX-GWIPE-020"]["4"] == drawn
    # The ramp at 50: the dark half gone, the light half whole; inverted, the other way round.
    for x in range(W):
        half = col(drawn, x) if x >= 8 else col(gone, x)
        assert col(c["FX-GWIPE-003"]["0"], x) == half, x
        other = col(drawn, x) if x < 8 else col(gone, x)
        assert col(c["FX-GWIPE-008"]["0"], x) == other, x
    # Softness 100 at 50: kept by the ramp's own brightness, 17 x of 255.
    for x in range(W):
        for y in range(H):
            p, q = c["FX-GWIPE-004"]["0"][y * W + x], drawn[y * W + x]
            assert all(abs(a - b * 17 * x / 255) < 1e-9 for a, b in zip(p, q)), (x, y)
    # Completion 30, softness 20: 0 to 2 gone, 6 on whole, 3 to 5 between.
    for x in range(W):
        got, whole = col(c["FX-GWIPE-009"]["0"], x), col(drawn, x)
        if x <= 2:
            assert got == col(gone, x), x
        elif x >= 6:
            assert got == whole, x
        else:
            assert got != whole and got != col(gone, x), x
    # The veil: rows 0 and 1 go, the rest stay.
    assert c["FX-GWIPE-012"]["0"] == gone[:2 * W] + drawn[2 * W:]
    # The centred checker: only its four white pixels, (6, 4), (8, 4), (7, 5), (9, 5), kept.
    keep = {(6, 4), (8, 4), (7, 5), (9, 5)}
    assert c["FX-GWIPE-013"]["0"] == [drawn[y * W + x] if (x, y) in keep else CLEAR
                                      for y in range(H) for x in range(W)]
    assert c["FX-GWIPE-014"]["0"] == [drawn[y * W + x] if (x + y) % 2 == 0 else CLEAR
                                      for y in range(H) for x in range(W)]
    assert c["FX-GWIPE-015"]["0"] not in (drawn, gone, c["FX-GWIPE-014"]["0"])
    # Itself: red and blue go, cream stays.
    for y in range(H):
        for x in range(W):
            kept = K.holder_px(x, y) == K.CREAM
            assert c["FX-GWIPE-016"]["0"][y * W + x] == (drawn[y * W + x] if kept else CLEAR)
    # Keyed.
    assert c["FX-GWIPE-017"]["0"] == drawn and c["FX-GWIPE-017"]["2"] == c["FX-GWIPE-003"]["0"]
    assert c["FX-GWIPE-017"]["4"] == gone
    assert c["FX-GWIPE-018"]["0"] == c["FX-GWIPE-003"]["0"]
    assert c["FX-GWIPE-018"]["4"] == c["FX-GWIPE-004"]["0"]
    assert c["FX-GWIPE-018"]["2"] not in (c["FX-GWIPE-003"]["0"], c["FX-GWIPE-004"]["0"])
    # Moved: the same picture, two right and one down; on an adjustment layer, the same as 003.
    moved = c["FX-GWIPE-021"]["0"]
    assert all(moved[(y + 1) * W + x + 2] == c["FX-GWIPE-003"]["0"][y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    assert c["FX-GWIPE-022"]["0"] == c["FX-GWIPE-003"]["0"]
    # A wipe only takes away: every number between 0 and the drawing's own (021 is moved).
    for fx, v in c.items():
        for px in v.values():
            for p, q in zip(px, drawn):
                assert fx == "FX-GWIPE-021" or all(-1e-12 <= a <= b + 1e-12 for a, b in zip(p, q))
    print("checks passed")


if __name__ == "__main__":
    main()
