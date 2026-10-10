"""Aerial Haze, worked a second way.

D-403 adds Aerial Haze (`core.aerial_haze`), PLUGINS.md's pick #8: distant things in a background
get less contrast and move toward the colour of the air. taka2's note on aerial perspective
(note.com/taka2composite, 2025-05-25) says to raise the black level rather than brighten, and to
lower the saturation and turn bluish, nearer the colour of the air. The rule below does all three
with one mix: each pixel moves toward the haze colour by the amount, in linear light, the
atmospheric model I = J t + A (1 - t) with t = 1 - amount. Blacks rise to the haze colour times
the amount, colours lose saturation toward it, and whites move toward it too. Evenly, or through
a matte: another layer of the composition, D-189's layer setting
(`tools/effect_layer_reference.py`), whose brightness says how far away each pixel is.

At the holder's composition frame n, with its input O, linear and premultiplied, whose drawing's
own top-left pixel is at (ox, oy) however far earlier effects grew it:

1. H is the haze colour, each channel's 8-bit value over 255 taken to linear by the sRGB curve.
2. With no layer named (""), v = 1 at every pixel: the haze is even.
3. With a layer named, its map M is D-189's, fitted to the drawing's own W by H by `fit`. Pixel
   (x, y) of O reads M(x - ox, y - oy), transparent outside M, and v is its picture luma, the
   sRGB encoding of clamp(0.2126 r + 0.7152 g + 0.0722 b, 0, 1), as Gradient Wipe reads its map,
   so a clear map reads as black and leaves no haze. A layer named that is not in the
   composition leaves the output O, with D-189's warning.
4. k = amount / 100 * v, and the output is (O.rgb + k (H O.a - O.rgb), O.a): the covering never
   changes, and a pixel that does not show stays clear.

`haze_color` `#rrggbb`, #b4c8dc when added (a pale sky blue of our own), read in small letters,
not keyable; `amount` 0 to 100, 30 when added, keyable; `layer` D-189's word, "" when added;
`fit` `stretch` when added, `center` or `tile`.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/aerial_haze/`:
Gradient Wipe's map layers, every one switched off, and on top of them the drawing `holder` with
the effect, so that only the holder is seen. The expected pixels are in
`Fixtures/aerial_haze/expected_aerial_haze.json`, with a project whose layer settings go round in
a circle.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/aerial_haze_reference.py
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
import displacement_map_reference as D  # noqa: E402
import recolor_reference as R  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "aerial_haze"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
SKY = "#b4c8dc"

DRAWINGS = {n: D.DRAWINGS[n] for n in ("holder", "ramp", "card", "veil")}
MAPS = {  # what each map layer's picture is at frame 0; `late` starts at frame 4
    "ramp": D.decoded("ramp"), "white": K.solid_pic([1.0] * 3), "black": K.solid_pic([0.0] * 3),
    "card": D.decoded("card"), "veil": D.decoded("veil"), "late": K.solid_pic([1.0] * 3),
}


# --- the rule -------------------------------------------------------------------------------

def haze(o, m, color, amount):
    """Steps 1 to 4 on the picture o; m is the map already fitted to it, "even" or None."""
    if m is None:
        return o
    h = [srgb_to_linear(c / 255) for c in R.hex_color(color.lower())]
    out = []
    for i, p in enumerate(o["px"]):
        v = 1.0 if m == "even" else K.luma(m["px"][i])
        k = amount / 100 * v
        out.append([p[c] + k * (h[c] * p[3] - p[c]) for c in range(3)] + [p[3]])
    return L.pic(o["w"], o["h"], out)


# --- the cases ------------------------------------------------------------------------------

def case(haze_color=SKY, amount=30, layer="", fit="stretch", shift=(0, 0), on="holder"):
    return {"haze_color": haze_color, "amount": amount, "layer": layer, "fit": fit,
            "shift": shift, "on": on}


def the_map(c, frame):
    if c["layer"] == "":
        return "even"
    if c["layer"] == "holder":
        return D.decoded("holder")  # steps 1 and 2 only: the drawing, no mask
    if c["layer"] not in MAPS:
        return None
    p = L.EMPTY if c["layer"] == "late" and frame < 4 else MAPS[c["layer"]]
    return L.fit(p, c["fit"], W, H)


def render(c, frame):
    """The composition's pixels at frame: the holder with its effect, moved by its shift; or,
    with the effect on an adjustment layer above, the frame below through it."""
    out = haze(D.decoded("holder"), the_map(c, frame), c["haze_color"], value_at(c["amount"], frame))
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = D.decoded("holder")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


CASES = {
    "FX-HAZE-001": ("As added: a pale sky blue, #b4c8dc, amount 30, no layer named: every pixel "
                    "that shows three tenths of the way to the sky blue, evenly; the clear corner "
                    "stays clear.", case(), (0,)),
    "FX-HAZE-002": ("Amount 0: nothing changes.", case(amount=0), (0,)),
    "FX-HAZE-003": ("Amount 100: every pixel that shows is the sky blue at its own covering.",
                    case(amount=100), (0,)),
    "FX-HAZE-004": ("White haze, #ffffff, amount 50: every channel halfway to white, the darkest "
                    "parts lifted most, as raising Levels' output black.",
                    case("#ffffff", 50), (0,)),
    "FX-HAZE-005": ("FX-HAZE-001 with its colour written in capitals, #B4C8DC: the same.",
                    case("#B4C8DC"), (0,)),
    "FX-HAZE-006": ("The grey ramp as the matte, amount 100: column 0 (black) untouched, column 15 "
                    "(white) the sky blue, the columns between hazed by the ramp's brightness. The "
                    "ramp layer is moved, scaled and switched off, none of which the matte reads.",
                    case(amount=100, layer="ramp"), (0,)),
    "FX-HAZE-007": ("The ramp, amount 60: as FX-HAZE-006 at six tenths.",
                    case(amount=60, layer="ramp"), (0,)),
    "FX-HAZE-008": ("A white solid as the matte, amount 30: the same as no matte, FX-HAZE-001.",
                    case(layer="white"), (0,)),
    "FX-HAZE-009": ("A black solid as the matte, amount 100: nothing changes.",
                    case(amount=100, layer="black"), (0,)),
    "FX-HAZE-010": ("A white matte clear at the top and more covering down each row, amount 100: a "
                    "clear matte reads as black, so row 0 is untouched and the haze grows "
                    "down the rows.", case(amount=100, layer="veil"), (0,)),
    "FX-HAZE-011": ("A 4 by 2 checker of white and black, centred, amount 100: outside it the "
                    "matte is clear, black, so only its four white pixels are hazed.",
                    case(amount=100, layer="card", fit="center"), (0,)),
    "FX-HAZE-012": ("The checker tiled, amount 100: hazed and untouched in a checker over the "
                    "whole layer.", case(amount=100, layer="card", fit="tile"), (0,)),
    "FX-HAZE-013": ("The checker stretched to 16 by 10, amount 100: soft squares of haze.",
                    case(amount=100, layer="card", fit="stretch"), (0,)),
    "FX-HAZE-014": ("The holder names itself, amount 100: its cream squares, the brightest, are "
                    "hazed most, its red and blue ones less, and the clear corner stays clear.",
                    case(amount=100, layer="holder"), (0,)),
    "FX-HAZE-015": ("Amount keyed from 0 at frame 0 to 100 at frame 4, no matte: frame 0 the "
                    "drawing, frame 2 halfway, frame 4 as FX-HAZE-003.",
                    case(amount=keyed((0, 0), (4, 100))), (0, 2, 4)),
    "FX-HAZE-016": ("A layer that is not in the composition, `gone`, amount 100: nothing changes, "
                    "and the warning every frame.", case(amount=100, layer="gone"), (0, 4)),
    "FX-HAZE-017": ("A white solid whose in point is frame 4 as the matte, amount 100: before it "
                    "the matte is empty, which reads as black, so at frame 0 nothing changes; at "
                    "frame 4 it is FX-HAZE-003.", case(amount=100, layer="late"), (0, 4)),
    "FX-HAZE-018": ("FX-HAZE-007 on the holder moved 2 right and 1 down: the same picture moved, "
                    "since the matte lies on the layer.",
                    case(amount=60, layer="ramp", shift=(2, 1)), (0,)),
    "FX-HAZE-019": ("The effect on an adjustment layer above the holder, the ramp as its matte, "
                    "amount 60: the matte lies on the frame, which here is the holder's own "
                    "rectangle, so this is FX-HAZE-007.",
                    case(amount=60, layer="ramp", on="adjust"), (0,)),
}

INVALID = {
    "FX-HAZE-020": ("Amount 101, above 100.", case(amount=101)),
    "FX-HAZE-021": ("Amount -1, below 0.", case(amount=-1)),
    "FX-HAZE-022": ("Amount keyed to 150 at frame 4.", case(amount=keyed((0, 30), (4, 150)))),
    "FX-HAZE-023": ("A haze colour written \"#12345\", five digits.", case("#12345")),
    "FX-HAZE-024": ("A fit written \"fill\".", case(layer="ramp", fit="fill")),
    "FX-HAZE-025": ("A layer written as the number 3, not a word.", case(layer=3)),
}

KEYS = ("haze_color", "amount", "layer", "fit")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.aerial_haze", "enabled": True,
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


# Layer settings that go round in a circle: a whole project, refused on opening.
LOADS = {
    "cycle_two.json": ("`a` and `b`, each the holder's drawing, each with an Aerial Haze whose "
                       "matte is the other: refused, `EFFECT_LAYER_CYCLE`.",
                       [drawing("a", effect("fx-a-0", case(layer="b"))),
                        drawing("b", effect("fx-b-0", case(layer="a")))]),
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

    (OUT / "expected_aerial_haze.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def near(a, b, eps=1e-9):
    return all(abs(x - y) <= eps for p, q in zip(a, b) for x, y in zip(p, q))


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    sky = [srgb_to_linear(v / 255) for v in R.hex_color(SKY)]
    shows = [i for i, p in enumerate(drawn) if p[3] > 0]
    assert shows and len(shows) < W * H, "the drawing has a clear corner"
    full = [[s * p[3] for s in sky] + [p[3]] for p in drawn]
    for fx in ("FX-HAZE-002", "FX-HAZE-009"):
        assert c[fx]["0"] == drawn, fx
    assert c["FX-HAZE-016"]["0"] == drawn and c["FX-HAZE-016"]["4"] == drawn
    assert c["FX-HAZE-017"]["0"] == drawn and near(c["FX-HAZE-017"]["4"], full)
    assert near(c["FX-HAZE-003"]["0"], full)
    # As added: three tenths of the way to the sky blue, the covering kept, clear stays clear.
    for p, q, f in zip(c["FX-HAZE-001"]["0"], drawn, full):
        assert p[3] == q[3] and all(abs(a - (b + 0.3 * (s - b))) < 1e-12 for a, b, s in zip(p, q, f))
        if q[3] == 0:
            assert p == q
    assert c["FX-HAZE-005"]["0"] == c["FX-HAZE-001"]["0"]
    assert near(c["FX-HAZE-008"]["0"], c["FX-HAZE-001"]["0"])
    # White at 50: blacks lifted to half the covering, every channel up, none down.
    for p, q in zip(c["FX-HAZE-004"]["0"], drawn):
        assert all(a >= b - 1e-12 for a, b in zip(p[:3], q[:3]))
        assert all(abs(a - (b + 0.5 * (q[3] - b))) < 1e-12 for a, b in zip(p[:3], q[:3]))
    # Less contrast: the spread of the drawing's lightness shrinks by the amount.
    lum = lambda p: 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2]  # noqa: E731
    spread = lambda px: max(lum(px[i]) for i in shows) - min(lum(px[i]) for i in shows)  # noqa: E731
    assert abs(spread(c["FX-HAZE-001"]["0"]) - 0.7 * spread(drawn)) < 1e-12
    # The ramp: column 0 untouched, column 15 the sky blue, between in order.
    col = lambda px, x: [px[y * W + x] for y in range(H)]  # noqa: E731
    assert col(c["FX-HAZE-006"]["0"], 0) == col(drawn, 0)
    assert near(col(c["FX-HAZE-006"]["0"], 15), col(full, 15))
    for x in range(1, 15):
        assert col(c["FX-HAZE-006"]["0"], x) != col(drawn, x), x
    # 60 against 100 through the ramp: the change at 0.6 of it.
    for p, q, r in zip(c["FX-HAZE-007"]["0"], c["FX-HAZE-006"]["0"], drawn):
        assert all(abs((a - b) - 0.6 * (d - b)) < 1e-12 for a, d, b in zip(p, q, r))
    # The veil: row 0 untouched, the rest hazed.
    assert c["FX-HAZE-010"]["0"][:W] == drawn[:W]
    assert all(c["FX-HAZE-010"]["0"][i] != drawn[i] for i in shows if i >= W)
    # The centred checker: only its four white pixels, (6, 4), (8, 4), (7, 5), (9, 5), hazed.
    keep = {(6, 4), (8, 4), (7, 5), (9, 5)}
    assert near(c["FX-HAZE-011"]["0"], [full[y * W + x] if (x, y) in keep else drawn[y * W + x]
                                        for y in range(H) for x in range(W)])
    assert near(c["FX-HAZE-012"]["0"], [full[y * W + x] if (x + y) % 2 == 0 else drawn[y * W + x]
                                        for y in range(H) for x in range(W)])
    assert c["FX-HAZE-013"]["0"] not in (drawn, c["FX-HAZE-012"]["0"])
    # Itself: cream hazed more than red and blue.
    for y in range(H):
        for x in range(W):
            i = y * W + x
            if drawn[i][3] == 0:
                assert c["FX-HAZE-014"]["0"][i] == drawn[i]
    # Keyed.
    assert c["FX-HAZE-015"]["0"] == drawn and near(c["FX-HAZE-015"]["4"], full)
    for p, q, f in zip(c["FX-HAZE-015"]["2"], drawn, full):
        assert all(abs(a - (b + 0.5 * (s - b))) < 1e-12 for a, b, s in zip(p, q, f))
    # Moved: the same picture, two right and one down; on an adjustment layer, the same as 007.
    moved = c["FX-HAZE-018"]["0"]
    assert all(moved[(y + 1) * W + x + 2] == c["FX-HAZE-007"]["0"][y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    assert near(c["FX-HAZE-019"]["0"], c["FX-HAZE-007"]["0"])
    print("checks passed")


if __name__ == "__main__":
    main()
