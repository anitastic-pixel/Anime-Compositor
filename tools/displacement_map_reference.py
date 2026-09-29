"""Displacement Map, worked a second way.

D-193 proposes Displacement Map (`core.displacement_map`): a layer's pixels moved sideways and
up or down by how bright a chosen channel of another layer of the same composition, its map, is.
The map is D-189's: the named layer's picture fitted to the holder,
`tools/effect_layer_reference.py`. This file pins what is done with it.

At the holder's composition frame n, with its input O, linear and premultiplied, whose drawing's
own top-left pixel is at (ox, oy) however far earlier effects grew it:

1. With no map (an empty name, a missing layer), the output is O.
2. The map M is D-189's, fitted to the drawing's own W by H. Pixel (x, y) of O reads
   m = M(x - ox, y - oy), transparent outside M, with covering a.
3. Each direction's channel word gives v in 0..1: `full` 1, `off` 0.5, `alpha` a. Every other
   word reads m's straight colour c = linear_to_srgb(clamp(m.rgb / a, 0, 1)), laid over mid grey
   by its covering, v = 0.5 + a (k - 0.5), so a clear map moves nothing: `red`, `green` and
   `blue` its channels; `luminance` linear_to_srgb(0.2126 r + 0.7152 g + 0.0722 b) of the
   straight linear colour; `hue` (degrees / 360), `saturation` and `lightness` Hue/Saturation's
   HSL of c (D-113). Where a is 0, every such word gives 0.5.
4. dx = (2 v_h - 1) max_horizontal and dy = (2 v_v - 1) max_vertical. The output pixel is O
   read bilinearly at (x + 0.5 + dx, y + 0.5 + dy), document 21's resampling, transparent
   outside O; with `wrap` `on`, the place and the four pixels it reads wrap round O's width and
   height. So white in the chosen channel reads the picture from the right and below, and the
   picture seems to move left and up; black the other way; mid grey not at all. The layer does
   not grow.
5. For a draft, both maxima are distances and the map is made at the holder's size (D-189
   step 3).

`layer` D-189's word, "" when added; `fit` `stretch` when added, `center` or `tile`;
`horizontal` `red` and `vertical` `green` when added, each one of red, green, blue, alpha,
luminance, hue, lightness, saturation, full, off; `max_horizontal` and `max_vertical`, -1000 to
1000 pixels, 5 when added, keyable; `wrap` `off` or `on`.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values.

Every case is a project of one composition 16 by 10, five frames, in
`Fixtures/displacement_map/`: the map layers, every one switched off, and on top of them the
drawing `holder` with the effect, so that only the holder is seen. The expected pixels are in
`Fixtures/displacement_map/expected_displacement_map.json`, with two projects whose layer
settings go round in a circle.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/displacement_map_reference.py
"""

import json
import sys
from math import floor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from hue_saturation_reference import to_hsl  # noqa: E402
import smooth_reference as S  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import compound_blur_reference as K  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "displacement_map"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
CLEAR = [0.0, 0.0, 0.0, 0.0]
WORDS = ("red", "green", "blue", "alpha", "luminance", "hue", "lightness", "saturation", "full",
         "off")

DRAWINGS = {
    "holder": K.DRAWINGS["holder"],
    "ramp": K.DRAWINGS["ramp"],
    "card": K.DRAWINGS["card"],
    # Greens and blues, never red the largest, so no pixel's hue is near 0 or 360.
    "paint": [[(40 + 8 * y, 30 + 14 * x, 200 - 12 * x + 3 * y, 255) for x in range(W)]
              for y in range(H)],
    # White, clear at the top and more covering down each row.
    "veil": [[(255, 255, 255, 28 * y) for x in range(W)] for y in range(H)],
}


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


MAPS = {  # what each map layer's picture is at frame 0; `late` starts at frame 4
    "ramp": decoded("ramp"), "white": K.solid_pic([1.0] * 3), "black": K.solid_pic([0.0] * 3),
    "card": decoded("card"), "paint": decoded("paint"), "veil": decoded("veil"),
    "late": K.solid_pic([1.0] * 3),
}


# --- the rule -------------------------------------------------------------------------------

def channel(word, m):
    a = m[3]
    if word == "full":
        return 1.0
    if word == "off":
        return 0.5
    if word == "alpha":
        return a
    if a <= 0:
        return 0.5
    lin = [min(max(m[c] / a, 0.0), 1.0) for c in range(3)]
    c = [S.linear_to_srgb(v) for v in lin]
    h, s, l = to_hsl(*c)
    k = {"red": c[0], "green": c[1], "blue": c[2], "hue": h / 360, "saturation": s,
         "lightness": l,
         "luminance": S.linear_to_srgb(0.2126 * lin[0] + 0.7152 * lin[1] + 0.0722 * lin[2])}[word]
    return 0.5 + a * (k - 0.5)


def wrapped(p, sx, sy):
    """Document 21's bilinear read, the place and its four pixels wrapped round the picture."""
    w, h = p["w"], p["h"]
    fx, fy = sx - 0.5, sy - 0.5
    x0, y0 = floor(fx), floor(fy)
    ux, uy = fx - x0, fy - y0
    out = [0.0] * 4
    for dy, wy in ((0, 1 - uy), (1, uy)):
        for dx, wx in ((0, 1 - ux), (1, ux)):
            s = L.at(p, (x0 + dx) % w, (y0 + dy) % h)
            for c in range(4):
                out[c] += s[c] * wx * wy
    return out


def displace(o, m, horizontal, max_h, vertical, max_v, wrap):
    """Steps 1 to 4 on the picture o, the map m already fitted to it, or None."""
    if m is None:
        return o
    out = []
    for y in range(o["h"]):
        for x in range(o["w"]):
            p = m["px"][y * o["w"] + x]
            dx = (2 * channel(horizontal, p) - 1) * max_h
            dy = (2 * channel(vertical, p) - 1) * max_v
            read = wrapped if wrap == "on" else L.bilinear
            out.append(read(o, x + 0.5 + dx, y + 0.5 + dy))
    return L.pic(o["w"], o["h"], out)


# --- the cases ------------------------------------------------------------------------------

def case(layer="", fit="stretch", horizontal="red", max_h=5, vertical="green", max_v=5,
         wrap="off", shift=(0, 0), on="holder"):
    return {"layer": layer, "fit": fit, "horizontal": horizontal, "max_horizontal": max_h,
            "vertical": vertical, "max_vertical": max_v, "wrap": wrap, "shift": shift, "on": on}


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
    out = displace(decoded("holder"), the_map(c, frame), c["horizontal"],
                   value_at(c["max_horizontal"], frame), c["vertical"],
                   value_at(c["max_vertical"], frame), c["wrap"])
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded("holder")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


CASES = {
    "FX-DMAP-001": ("As added: no layer named, so nothing moves.", case(), (0,)),
    "FX-DMAP-002": ("The grey ramp as the map, red across and green down, both at most 2: "
                    "column 0, black, reads 2 left and 2 up, column 15, white, 2 right and 2 "
                    "down, and between them in step. The ramp layer is moved, scaled and switched "
                    "off, none of which the map reads.", case("ramp", max_h=2, max_v=2), (0,)),
    "FX-DMAP-003": ("A white solid: every pixel reads the one 2 right and 2 below, so the picture "
                    "moves 2 left and 2 up and clear comes in at the right and the bottom.",
                    case("white", max_h=2, max_v=2), (0,)),
    "FX-DMAP-004": ("A black solid: the picture moves 2 right and 2 down.",
                    case("black", max_h=2, max_v=2), (0,)),
    "FX-DMAP-005": ("The white solid with both maxima -2: as FX-DMAP-004.",
                    case("white", max_h=-2, max_v=-2), (0,)),
    "FX-DMAP-006": ("The black solid, across Full and down Off, both at most 2: the picture "
                    "moves 2 left, whatever the map, and not up or down.",
                    case("black", horizontal="full", max_h=2, vertical="off", max_v=2), (0,)),
    "FX-DMAP-007": ("The ramp, both directions Off: nothing moves.",
                    case("ramp", horizontal="off", vertical="off", max_h=2, max_v=2), (0,)),
    "FX-DMAP-008": ("A painted map of greens and blues, blue across at most 2.5 and luminance "
                    "down at most 1.5.", case("paint", horizontal="blue", max_h=2.5,
                                              vertical="luminance", max_v=1.5), (0,)),
    "FX-DMAP-009": ("The painted map, hue across and saturation down, both at most 2.",
                    case("paint", horizontal="hue", max_h=2, vertical="saturation", max_v=2),
                    (0,)),
    "FX-DMAP-010": ("The painted map, lightness across at most -2 and red down at most 1.5.",
                    case("paint", horizontal="lightness", max_h=-2, vertical="red", max_v=1.5),
                    (0,)),
    "FX-DMAP-011": ("A white map clear at the top and more covering down each row, red and "
                    "green, at most 2: row 0 does not move, since a clear map moves nothing, and "
                    "each row lower moves more, up and to the left.",
                    case("veil", max_h=2, max_v=2), (0,)),
    "FX-DMAP-012": ("The same map, alpha both ways: row 0, clear, reads 2 left and 2 up, so it "
                    "is clear itself; the bottom row, nearly covered, reads nearly 2 right and 2 "
                    "down.", case("veil", horizontal="alpha", max_h=2, vertical="alpha", max_v=2),
                    (0,)),
    "FX-DMAP-013": ("FX-DMAP-003 with Wrap on: the picture moves 2 left and 2 up, and what "
                    "leaves at the left and the top comes back at the right and the bottom.",
                    case("white", max_h=2, max_v=2, wrap="on"), (0,)),
    "FX-DMAP-014": ("A 4 by 2 checker of white and black as the map, centred, at most 2: only "
                    "the eight pixels it covers, columns 6 to 9 of rows 4 and 5, move.",
                    case("card", "center", max_h=2, max_v=2), (0,)),
    "FX-DMAP-015": ("The checker tiled: its pattern repeated over the whole layer.",
                    case("card", "tile", max_h=2, max_v=2), (0,)),
    "FX-DMAP-016": ("The checker stretched to 16 by 10, softened between its squares.",
                    case("card", "stretch", max_h=2, max_v=2), (0,)),
    "FX-DMAP-017": ("The holder names itself, at most 2: its own colours move it; the clear "
                    "corner, a clear map, does not move.", case("holder", max_h=2, max_v=2),
                    (0,)),
    "FX-DMAP-018": ("Across keyed from 0 at frame 0 to 4 at frame 4, down 0, with the ramp: "
                    "frame 0 does not move, frame 2 moves across as FX-DMAP-002 does.",
                    case("ramp", max_h=keyed((0, 0), (4, 4)), max_v=0), (0, 2, 4)),
    "FX-DMAP-019": ("A layer that is not in the composition, `gone`: nothing moves, and the "
                    "warning every frame.", case("gone", max_h=2, max_v=2), (0, 4)),
    "FX-DMAP-020": ("A white solid whose in point is frame 4: an empty map before it, so nothing "
                    "moves at frame 0, and at frame 4 the picture moves as in FX-DMAP-003.",
                    case("late", max_h=2, max_v=2), (0, 4)),
    "FX-DMAP-021": ("FX-DMAP-002 on the holder moved 2 right and 1 down: the same picture "
                    "moved, since the map lies on the layer.",
                    case("ramp", max_h=2, max_v=2, shift=(2, 1)), (0,)),
    "FX-DMAP-022": ("The effect on an adjustment layer above the holder, the ramp as its map: "
                    "the map lies on the frame, which here is the holder's own rectangle, so "
                    "this is FX-DMAP-002.", case("ramp", max_h=2, max_v=2, on="adjust"), (0,)),
    "FX-DMAP-023": ("FX-DMAP-002 with Wrap on: the same in the middle, and at the edges what "
                    "is read from beyond one side comes from the other.",
                    case("ramp", max_h=2, max_v=2, wrap="on"), (0,)),
}

INVALID = {
    "FX-DMAP-024": ("Maximum across 1001, above 1000.", case("ramp", max_h=1001)),
    "FX-DMAP-025": ("Maximum down -1001, below -1000.", case("ramp", max_v=-1001)),
    "FX-DMAP-026": ("Maximum across keyed to 2000 at frame 4.",
                    case("ramp", max_h=keyed((0, 3), (4, 2000)))),
    "FX-DMAP-027": ("Across written \"half\", After Effects' Half, which is left out.",
                    case("ramp", horizontal="half")),
    "FX-DMAP-028": ("Down written \"purple\".", case("ramp", vertical="purple")),
    "FX-DMAP-029": ("A fit written \"fill\".", case("ramp", "fill")),
    "FX-DMAP-030": ("Wrap written \"yes\".", case("ramp", wrap="yes")),
    "FX-DMAP-031": ("A layer written as the number 3, not a word.", case(3)),
}

KEYS = ("layer", "fit", "horizontal", "max_horizontal", "vertical", "max_vertical", "wrap")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.displacement_map", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in KEYS}}


def map_layers():
    return [L.raster("ramp", "asset-ramp", position=(3, 2), scale=(50, 50), enabled=False,
                     out_frame=FRAMES),
            L.solid("white", W, H, [1.0] * 3, enabled=False, out_frame=FRAMES),
            L.solid("black", W, H, [0.0] * 3, enabled=False, out_frame=FRAMES),
            L.raster("card", "asset-card", enabled=False, out_frame=FRAMES),
            L.raster("paint", "asset-paint", enabled=False, out_frame=FRAMES),
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


def compound_blur(fid, named):
    return {"instance_id": fid, "type_id": "core.compound_blur", "enabled": True,
            "parameters": {"layer": named, "fit": "stretch", "max_blur": 6, "invert": "off",
                           "edges": "transparent"}}


# Layer settings that go round in a circle. Each is a whole project, refused on opening.
LOADS = {
    "cycle_two.json": ("`a` and `b`, each the holder's drawing, each with a Displacement Map "
                       "naming the other: refused, `EFFECT_LAYER_CYCLE`.",
                       [drawing("a", effect("fx-a-0", case("b", max_h=2, max_v=2))),
                        drawing("b", effect("fx-b-0", case("a", max_h=2, max_v=2)))]),
    "cycle_mixed.json": ("`a`'s Displacement Map names `b`, and `b`'s Compound Blur names `a`: "
                         "a circle through two kinds of effect, refused the same.",
                         [drawing("a", effect("fx-a-0", case("b", max_h=2, max_v=2))),
                          drawing("b", compound_blur("fx-b-0", "a"))]),
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

    (OUT / "expected_displacement_map.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                        encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    o = decoded("holder")
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    shifted = lambda dx, dy, wrap=False: [  # noqa: E731
        list(L.at(o, (x + dx) % W, (y + dy) % H) if wrap else L.at(o, x + dx, y + dy))
        for y in range(H) for x in range(W)]
    for fx in ("FX-DMAP-001", "FX-DMAP-007", "FX-DMAP-019"):
        assert c[fx]["0"] == drawn, fx
    assert c["FX-DMAP-019"]["4"] == drawn and c["FX-DMAP-020"]["0"] == drawn
    assert c["FX-DMAP-018"]["0"] == drawn
    assert near(c["FX-DMAP-003"]["0"], shifted(2, 2))
    assert near(c["FX-DMAP-004"]["0"], shifted(-2, -2))
    assert near(c["FX-DMAP-005"]["0"], c["FX-DMAP-004"]["0"])
    assert near(c["FX-DMAP-006"]["0"], shifted(2, 0))
    assert near(c["FX-DMAP-013"]["0"], shifted(2, 2, wrap=True))
    assert near(c["FX-DMAP-020"]["4"], c["FX-DMAP-003"]["0"])
    assert c["FX-DMAP-022"]["0"] == c["FX-DMAP-002"]["0"]
    # Frame 2 of the keyed case moves across by the ramp as 002 does, and not down.
    ramp_across = render(case("ramp", max_h=2, max_v=0), 0)
    assert c["FX-DMAP-018"]["2"] == ramp_across and ramp_across != c["FX-DMAP-002"]["0"]
    # The ramp: column 0 reads 2 left and 2 up, column 15 two right and two down.
    col = lambda px, x: [px[y * W + x] for y in range(H)]  # noqa: E731
    assert near(col(c["FX-DMAP-002"]["0"], 0), col(shifted(-2, -2), 0), 1e-9)
    assert near(col(c["FX-DMAP-002"]["0"], 15), col(shifted(2, 2), 15), 1e-9)
    # Wrap changes only what is read from beyond an edge: the middle of the ramp case is 002's.
    for y in range(3, 7):
        for x in range(5, 11):
            assert c["FX-DMAP-023"]["0"][y * W + x] == c["FX-DMAP-002"]["0"][y * W + x], (x, y)
    assert c["FX-DMAP-023"]["0"] != c["FX-DMAP-002"]["0"]
    # A clear map moves nothing; clear under alpha reads 2 left and 2 up, off the picture.
    assert c["FX-DMAP-011"]["0"][:W] == drawn[:W]
    assert all(p == CLEAR for p in c["FX-DMAP-012"]["0"][:W])
    # The centred checker: only its eight pixels move.
    for y in range(H):
        for x in range(W):
            if not (6 <= x < 10 and 4 <= y < 6):
                assert c["FX-DMAP-014"]["0"][y * W + x] == drawn[y * W + x], (x, y)
    assert c["FX-DMAP-015"]["0"] != c["FX-DMAP-016"]["0"]
    # Itself: the clear corner, a clear map, reads itself, clear.
    assert all(c["FX-DMAP-017"]["0"][y * W + x] == CLEAR for y in range(7, H) for x in range(12, W))
    # Moved: the same picture, two right and one down.
    moved = c["FX-DMAP-021"]["0"]
    assert all(moved[(y + 1) * W + x + 2] == c["FX-DMAP-002"]["0"][y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    # The painted cases each move something, and differently.
    paints = [c[f"FX-DMAP-0{n}"]["0"] for n in ("08", "09", "10")]
    assert all(p != drawn for p in paints) and len({json.dumps(p) for p in paints}) == 3
    # No painted pixel's hue is near where it wraps from 360 back to 0.
    for row in DRAWINGS["paint"]:
        for r, g, b, _ in row:
            h = to_hsl(r / 255, g / 255, b / 255)[0]
            assert 30 < h < 330, (r, g, b, h)
    # Bilinear reads mix pixels and never make light: every covering within 0 and 1.
    assert all(-1e-12 <= p[3] <= 1 + 1e-12 for v in c.values() for px in v.values() for p in px)
    print("checks passed")


if __name__ == "__main__":
    main()
