"""Griddler, worked a second way.

D-386 adds `core.griddler`, after CycoreFX's CC Griddler: the picture cut into square tiles, each
tile's own piece scaled and turned in its place, a mosaic of little squares. CycoreFX's manual
says what each control does in a sentence and publishes no formula; the rule below is this
program's own reading of it, and nothing is ported.

The rule. The tiles are T pixels square, T = `tile_size` per cent of the drawing's own width,
laid from its top left corner, so the last column and row of tiles may be cut short. A pixel's
centre P falls in the tile with centre C. Its offset q = P - C is turned back by `rotation`
degrees (clockwise on the screen for a positive angle, as After Effects turns a layer) and
divided by hs = `horizontal_scale` / 100 across and vs = `vertical_scale` / 100 down, giving q';
the pixel takes document 21's bilinear sample of the drawing at C + q', transparent outside the
drawing. So each tile shows its own piece of the picture grown, shrunk, turned or, with a
negative scale, turned over, about the tile's centre. With `cut_tiles` on, a q' outside the
tile's own square, |q'| more than T / 2 across or down, is transparent: each tile draws only its
own piece, and a shrunk or turned tile leaves gaps. Off, the piece runs on into the picture round
it, so every tile is filled, a view of the picture round its centre. A scale of 0 draws nothing.
Each pixel is in one tile only, so a grown tile is cut at its own edges, and an edge falls where
the pixel's centre decides, without smoothing.

Scales 100 and rotation 0 are the drawing. The layer does not grow. The tile is a share of the
width, so a draft needs no change.

`horizontal_scale` and `vertical_scale` -1000 to 1000 per cent, 80 each when added; `tile_size`
0.1 to 100 per cent of the width, 10; `rotation` -3600 to 3600 degrees, 0; `cut_tiles` `off` or
`on`, `on`. The numbers are keyable. The values when added are chosen here.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/griddler/media`, the projects into
`Fixtures/griddler`, and the expected frames into `Fixtures/griddler/expected_griddler.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/griddler_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import bulge_reference as B  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "griddler"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"horizontal_scale": (-1000, 1000), "vertical_scale": (-1000, 1000),
          "tile_size": (0.1, 100), "rotation": (-3600, 3600)}
NAMES = ("horizontal_scale", "vertical_scale", "tile_size", "rotation", "cut_tiles")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def source(s_, px, py):
    """Where the point (px, py) reads the drawing, or None for nothing."""
    hs, vs = s_["horizontal_scale"] / 100, s_["vertical_scale"] / 100
    if hs == 0 or vs == 0:
        return None
    t = s_["tile_size"] / 100 * W
    cx = (math.floor(px / t) + 0.5) * t
    cy = (math.floor(py / t) + 0.5) * t
    qx, qy = px - cx, py - cy
    a = math.radians(s_["rotation"])
    sin, cos = math.sin(a), math.cos(a)
    ux = (qx * cos + qy * sin) / hs
    uy = (qy * cos - qx * sin) / vs
    if s_["cut_tiles"] == "on" and (abs(ux) > t / 2 or abs(uy) > t / 2):
        return None
    return cx + ux, cy + uy


def gridded(layer, s_, x, y):
    if not (0 <= x < layer["w"] and 0 <= y < layer["h"]):
        return EMPTY
    at = source(s_, x + 0.5, y + 0.5)
    return EMPTY if at is None else bilinear(layer, *at)


# --- the cases ------------------------------------------------------------------------------

def case(hs=80, vs=80, tile=10, rotation=0, cut="on", shift=0):
    return {"drawing": "stripes", "horizontal_scale": hs, "vertical_scale": vs,
            "tile_size": tile, "rotation": rotation, "cut_tiles": cut, "shift": shift}


def settings(c, frame_no):
    held = {}
    for k in NAMES:
        v = value_at(c[k], frame_no)
        if k in RANGES:
            lo, hi = RANGES[k]
            v = min(hi, max(lo, v))
        held[k] = v
    return held


def render(c, frame_no):
    layer = B.drawn_layer(c["drawing"])
    s_ = settings(c, frame_no)
    return [gridded(layer, s_, x - c["shift"], y) for y in range(H) for x in range(W)]


def plain(c):
    layer = B.drawn_layer(c["drawing"])
    return [list(layer["px"][y * W + x - c["shift"]]) if 0 <= x - c["shift"] < W else EMPTY
            for y in range(H) for x in range(W)]


FOUR = {"tile": 25}  # tiles 4 pixels square: four across, two and a half down

CASES = {
    "FX-GRIDDLER-001": ("The settings as they start: tiles a tenth of the width, 1.6 pixels "
                        "here, each its own piece at 80 per cent, cut: a fine grid of gaps "
                        "through the drawing.", case(), [0]),
    "FX-GRIDDLER-002": ("Scales 100, rotation 0: the drawing, untouched.",
                        case(hs=100, vs=100, **FOUR), [0]),
    "FX-GRIDDLER-003": ("Tiles 4 pixels, scales 50, cut: each tile's own piece at half size in "
                        "its middle 2 pixels, the ring round it transparent.",
                        case(hs=50, vs=50, **FOUR), [0]),
    "FX-GRIDDLER-004": ("FX-GRIDDLER-003 with Cut Tiles off: each tile filled with the 8 "
                        "pixels round its centre at half size.",
                        case(hs=50, vs=50, cut="off", **FOUR), [0]),
    "FX-GRIDDLER-005": ("Tiles 4 pixels, scales 100, rotation 45, cut: each tile's piece "
                        "turned an eighth, its corners cut off at the tile's edges and the "
                        "tile's own corners empty.", case(hs=100, vs=100, rotation=45, **FOUR),
                        [0]),
    "FX-GRIDDLER-006": ("FX-GRIDDLER-005 with Cut Tiles off: the tile's corners filled from the "
                        "picture round it, turned with it.",
                        case(hs=100, vs=100, rotation=45, cut="off", **FOUR), [0]),
    "FX-GRIDDLER-007": ("Tiles 4 pixels, horizontal scale -100: each tile's piece turned over "
                        "left to right, so each stripe pair swaps within its tile.",
                        case(hs=-100, vs=100, **FOUR), [0]),
    "FX-GRIDDLER-008": ("Tiles 4 pixels, vertical scale -100: each tile turned over top to "
                        "bottom; the band in rows 4 and 5 moves to rows 6 and 7.",
                        case(hs=100, vs=-100, **FOUR), [0]),
    "FX-GRIDDLER-009": ("Tiles 4 pixels, horizontal scale 200: each tile's middle 2 columns "
                        "stretched across it.", case(hs=200, vs=100, **FOUR), [0]),
    "FX-GRIDDLER-010": ("Tiles 8 pixels, rotation 90: each tile turned a quarter about its "
                        "centre, its pixels landing on pixels, so the stripes lie across.",
                        case(hs=100, vs=100, tile=50, rotation=90), [0]),
    "FX-GRIDDLER-011": ("Horizontal scale 0: nothing drawn.", case(hs=0, **FOUR), [0]),
    "FX-GRIDDLER-012": ("Rotation keyed from 0 at frame 0 to 90 at frame 4, linear, tiles 8 "
                        "pixels: frame 0 the drawing, frame 4 FX-GRIDDLER-010.",
                        case(hs=100, vs=100, tile=50, rotation=keyed((0, 0), (4, 90))),
                        [0, 2, 4]),
    "FX-GRIDDLER-013": ("Tile size keyed from 25 at frame 0 to 50 at frame 4, scales 50: the "
                        "tiles grow.", case(hs=50, vs=50, tile=keyed((0, 25), (4, 50))),
                        [0, 2, 4]),
    "FX-GRIDDLER-014": ("FX-GRIDDLER-003 moved three pixels right: the same, moved; the tiles "
                        "go with the layer, and the three columns left of it stay empty.",
                        case(hs=50, vs=50, shift=3, **FOUR), [0]),
    "FX-GRIDDLER-015": ("Horizontal scale eased from 100 at frame 0 to 1000 at frame 4 on a "
                        "curve that overshoots: at frame 2 it would pass 1000 and is held "
                        "there.", case(hs=keyed((0, 100, OVERSHOOT), (4, 1000)), vs=100,
                                       **FOUR), [0, 2, 4]),
}

INVALID = {
    "FX-GRIDDLER-016": ("Horizontal scale 1001, above 1000.", case(hs=1001)),
    "FX-GRIDDLER-017": ("Vertical scale -1001, below -1000.", case(vs=-1001)),
    "FX-GRIDDLER-018": ("Tile size 0, below 0.1.", case(tile=0)),
    "FX-GRIDDLER-019": ("Tile size 101, above 100.", case(tile=101)),
    "FX-GRIDDLER-020": ("Rotation 3601, past ten turns.", case(rotation=3601)),
    "FX-GRIDDLER-021": ("Cut Tiles written \"yes\".", case(cut="yes")),
    "FX-GRIDDLER-022": ("Tile size keyed to 200 at frame 4.", case(tile=keyed((0, 10), (4, 200)))),
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
        "instance_id": "fx-0-0", "type_id": "core.griddler", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in B.DRAWINGS.items():
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

    (OUT / "expected_griddler.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def near(a, b):
    return all(abs(u - v) < 1e-9 for p, q in zip(a, b) for u, v in zip(p, q))


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    assert c["FX-GRIDDLER-002"]["0"] == drawn
    assert all(p == EMPTY for p in c["FX-GRIDDLER-011"]["0"])
    three, four = c["FX-GRIDDLER-003"]["0"], c["FX-GRIDDLER-004"]["0"]
    # Cut, each 4-pixel tile keeps only its middle 2 by 2; off, every pixel of the drawing is
    # drawn.
    for y in range(8):
        for x in range(16):
            if x % 4 in (0, 3) or y % 4 in (0, 3):
                assert three[at(x, y)] == EMPTY, (x, y)
    assert three != four and four[at(4, 2)] != EMPTY
    assert c["FX-GRIDDLER-005"]["0"][at(0, 4)] == EMPTY != c["FX-GRIDDLER-006"]["0"][at(0, 4)]
    seven, eight = c["FX-GRIDDLER-007"]["0"], c["FX-GRIDDLER-008"]["0"]
    assert near([seven[at(1, 2)]], [drawn[at(2, 2)]]) and near([seven[at(2, 2)]], [drawn[at(1, 2)]])
    assert near([eight[at(3, 6)]], [drawn[at(3, 5)]]) and near([eight[at(3, 7)]], [drawn[at(3, 4)]])
    ten = c["FX-GRIDDLER-010"]["0"]
    # A quarter turn about (4, 4) clockwise: the pixel at (x, y) in the first tile shows the one
    # at (y, 7 - x).
    for y in range(8):
        for x in range(8):
            assert near([ten[at(x, y)]], [drawn[at(y, 7 - x)]]), (x, y)
    twelve = c["FX-GRIDDLER-012"]
    assert twelve["0"] == render(case(hs=100, vs=100, tile=50), 0)
    assert near(twelve["0"], drawn) and twelve["4"] == ten != twelve["2"]
    thirteen = c["FX-GRIDDLER-013"]
    assert thirteen["0"] == three != thirteen["4"]
    fourteen = c["FX-GRIDDLER-014"]["0"]
    assert all(fourteen[at(x, y)] == three[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(fourteen[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    fifteen = c["FX-GRIDDLER-015"]
    assert fifteen["2"] == fifteen["4"] == render(case(hs=1000, vs=100, **FOUR), 0)
    print("checked")


if __name__ == "__main__":
    main()
