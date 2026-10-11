"""IDentifier, worked a second way.

D-525 adds `core.identifier`, IDentifier, after After Effects' IDentifier (3D Channel, fnord
software's ProEXR plug-in that ships with After Effects), on EFFECTS.md row 381: a render's object
or material ids shown as a picture. The ids are read as D-349 reads them
(`tools/depth_channel_reference.py`): the layer's own EXR file, its object or material id channel
by name, never saved, said as `EFFECT_CHANNEL_MISSING` every frame when the layer has none, and the
effect then skipped. ID Key (`core.id_key`, after ID Matte) already keeps one id; IDentifier adds
the ways of looking at all of them.

fnord's manual says the default Colors display is a preview showing that the id channel is found
and that the objects have different ids, that the Luma and Alpha matte options are the ones used
in a comp, and that Raw shows the ids scaled to 8 bits so they can be sampled. It gives no colours
and no formula: the rule here is this program's own.

At composition frame n, with O the layer's picture (linear, premultiplied) whose drawing's own
top-left pixel is at (ox, oy), v the id at a pixel of the drawing and `id` the setting at n:

- "colors": k = floor(v + 0.5), the nearest whole id. Its hue is h = (k x 0.618033988749895) mod 1
  (the golden ratio's turn, so ids next to each other are far apart); the colour is HSV (h, 0.7,
  0.9) as Python's `colorsys.hsv_to_rgb` works it, each channel taken to linear by the sRGB curve.
  Opaque.
- "luma_matte": white where |v - id| < 0.5 (ID Key's test), black elsewhere. Opaque.
- "alpha_matte": O where |v - id| < 0.5, transparent elsewhere: the drawing with the matte as its
  alpha, ID Key with no feather.
- "raw": grey g = clamp(v / 255, 0, 1) as stored, taken to linear by the sRGB curve, so the 8-bit
  picture reads the id itself (ids past 255 white, below 0 black). Opaque.

Outside the ids (past the drawing) the result is transparent, as Pass Extract's is.

`aux_channel` "object_id" (when added) or "material_id"; `id` 0 to 1,000,000, 0 when added,
keyable; `output` "colors" (when added), "luma_matte", "alpha_matte" or "raw". Nothing is in
pixels, so a draft changes nothing but the pixels it draws.

Sources read: fnord software's ProEXR manual, version 2.6.6 (2 February 2026), its EXtractoR,
IDentifier and Cryptomatte sections; Adobe's 3D Channel effects page (helpx.adobe.com, last
updated 14 April 2024), its IDentifier and ID Matte sections; an OTOY forum thread
(render.otoy.com, viewtopic p=316850) where IDentifier said "No suitable integer channels
available" for an Octane object id pass stored as colour.

**This file never runs the build's code path.** It writes the EXR files with OpenEXR's own library
(3.4.15), reads them back with it, and works each expected pixel in double precision.

Every case is a project of one composition 8 by 6 at 24 frames a second, five frames long, in
`Fixtures/identifier/`, the depth fixtures' scene (colour by column and row, its top-left pixel
half covering and the next clear) with the depth fixtures' id files: `blender.exr` (object ids 0
to 5 in blocks three wide and three tall, material ids 1 to 3 on the diagonals with 2.4 and 2.6
between), `ids.exr` (a bare ObjectID, a materialID layer whose red is the id), and `big.exr`
(object ids 0 to 350 in steps of 50 by column, -3 at the bottom left, an infinity and a
not-a-number at the bottom right). The expected pixels are
`Fixtures/identifier/expected_identifier.json`. The playtest uses the depth fixtures'
`Fixtures/depth_channel/sample/spheres.exr`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/identifier_reference.py   (with OpenEXR 3.4.15 and numpy)
"""

import colorsys
import json
import sys
from math import floor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import depth_channel_reference as D  # noqa: E402
import effect_layer_reference as L  # noqa: E402
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "identifier"
D.OUT = OUT  # D's file helpers (write, read, picture, write_project) work in this folder
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = D.W, D.H, D.FRAMES
GOLDEN = 0.618033988749895
OUTPUTS = ("colors", "luma_matte", "alpha_matte", "raw")
SETTINGS = ("aux_channel", "id", "output")


# --- the files ------------------------------------------------------------------------------

def make_files():
    np = D.np
    y, x = np.mgrid[0:H, 0:W].astype(np.float64)
    D.write("media/scene.exr", dict(D.colour(), Z=D.depth()))
    blender = {f"ViewLayer.Combined.{k}": v for k, v in D.colour().items()}
    blender.update({"ViewLayer.Depth.Z": D.depth(), "ViewLayer.IndexOB.X": D.objects(),
                    "ViewLayer.IndexMA.X": D.materials()})
    D.write("media/blender.exr", blender)
    D.write("media/ids.exr", dict(D.colour(), ObjectID=(x % 4).astype("f4"),
                                  **{"materialID.R": (y % 2 + 7).astype("f4"),
                                     "materialID.G": np.full((H, W), 100, "f4"),
                                     "materialID.B": np.full((H, W), 200, "f4")}))
    big = (50 * x).astype("f4")
    big[5, 0], big[5, 7], big[4, 7] = -3, np.nan, np.inf
    D.write("media/big.exr", dict(D.colour(), ObjectID=big))
    L.DRAWINGS["card"] = [[(36 * x, 200, 30 * y, 255) for x in range(W)] for y in range(H)]
    (OUT / "media" / "card.png").write_bytes(png(L.DRAWINGS["card"]))


# --- the rule -------------------------------------------------------------------------------

def colour_of(v):
    h = (floor(v + 0.5) * GOLDEN) % 1.0
    return [srgb_to_linear(c) for c in colorsys.hsv_to_rgb(h, 0.7, 0.9)]


def hit(v, at):
    return abs(v - at) < 0.5


def shown(p, v, at, output):
    """One pixel of the drawing p (premultiplied) with its id v."""
    if output == "colors":
        return colour_of(v) + [1.0]
    if output == "luma_matte":
        return [1.0 if hit(v, at) else 0.0] * 3 + [1.0]
    if output == "alpha_matte":
        return list(p) if hit(v, at) else [0.0, 0.0, 0.0, 0.0]
    g = srgb_to_linear(min(1.0, max(0.0, v / 255)))
    return [g, g, g, 1.0]


def effected(c, n):
    p = D.drawing(c)
    if c["before"] is not None:
        p = L.exposure(p, c["before"])
    if D.works(c):
        pv = D.passes(c["file"], c["pass"])
        at = min(1e6, max(0.0, value_at(c["id"], n)))
        p = L.pic(W, H, [shown(p["px"][i], pv[i][0], at, c["output"]) for i in range(W * H)])
    if c["after"] is not None:
        p = L.exposure(p, c["after"])
    return p


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.identifier", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in SETTINGS}}


D.effected = effected  # D.render and D.plain draw through these
D.effect = effect


def case(aux_channel="object_id", id=0, output="colors", file="blender.exr", **kw):
    return D.case("identifier", aux_channel=aux_channel, id=id, output=output, file=file,
                  **{"pass": aux_channel}, **kw)


# --- the cases ------------------------------------------------------------------------------

F0 = [0]
CASES = {
    "FX-IDENT-001": ("As added: the object ids 0 to 5 as six colours, one to each block of three "
                     "by three, opaque.", case(), [0, 4]),
    "FX-IDENT-002": ("The material ids as colours: 1 to 3 on the diagonals, 2.4 coloured as 2 "
                     "and 2.6 as 3.", case(aux_channel="material_id"), F0),
    "FX-IDENT-003": ("Luma matte of object 4: the bottom-middle block white, the rest black.",
                     case(id=4, output="luma_matte"), F0),
    "FX-IDENT-004": ("Alpha matte of object 4: the scene kept in the bottom-middle block, "
                     "transparent elsewhere.", case(id=4, output="alpha_matte"), F0),
    "FX-IDENT-005": ("Raw object ids: greys whose 8-bit values are 0 to 5.",
                     case(output="raw"), F0),
    "FX-IDENT-006": ("Raw material ids: 2.4 and 2.6 as they are, not rounded.",
                     case(aux_channel="material_id", output="raw"), F0),
    "FX-IDENT-007": ("Luma matte of material 2: the 2 diagonal and the 2.4 pixel, not the 2.6.",
                     case(aux_channel="material_id", id=2, output="luma_matte"), F0),
    "FX-IDENT-008": ("The id keyed from 0 at frame 0 to 4 at frame 4, luma matte: block 0, 2, "
                     "then 4.", case(id=keyed((0, 0), (4, 4)), output="luma_matte"), [0, 2, 4]),
    "FX-IDENT-009": ("A bare ObjectID channel, 0 to 3 by column: four colours.",
                     case(file="ids.exr"), F0),
    "FX-IDENT-010": ("materialID.R (7 or 8 by row), not its G (100) or B (200): luma matte of 8, "
                     "the odd rows.", case(aux_channel="material_id", id=8, output="luma_matte",
                                           file="ids.exr"), F0),
    "FX-IDENT-011": ("Ids from 0 to 350 by column, -3, an infinity and not-a-number, raw: white "
                     "past 255, black at -3, the infinity white and the not-a-number black; the "
                     "warning every frame.", case(output="raw", file="big.exr"), F0),
    "FX-IDENT-012": ("The same ids as colours: each its own colour, the infinity too.",
                     case(file="big.exr"), F0),
    "FX-IDENT-013": ("A file with no ids: unchanged, the warning every frame.",
                     case(file="scene.exr"), F0),
    "FX-IDENT-014": ("On a PNG drawing: unchanged, the warning.", case(layer="card"), F0),
    "FX-IDENT-015": ("On an adjustment layer above the EXR: no file of its own, unchanged, the "
                     "warning.", case(on="adjust"), F0),
    "FX-IDENT-016": ("The layer moved 2 right and 1 down: FX-IDENT-001 moved.",
                     case(move=(2, 1)), F0),
    "FX-IDENT-017": ("An Exposure of +1 before it, alpha matte of object 0: the brighter scene "
                     "kept in the top-left block.", case(output="alpha_matte", before=1), F0),
    "FX-IDENT-018": ("An Exposure of -1 after it, colours: each colour halved.",
                     case(after=-1), F0),
    "FX-IDENT-019": ("Alpha matte of object 9, which no pixel has: all transparent.",
                     case(id=9, output="alpha_matte"), F0),
}

INVALID = {
    "FX-IDENT-020": ("Output written \"matte\".", case(output="matte")),
    "FX-IDENT-021": ("Aux channel written \"depth\".", case(aux_channel="depth")),
    "FX-IDENT-022": ("Id -1, below 0.", case(id=-1)),
    "FX-IDENT-023": ("Id keyed to 1,000,001 at frame 4.", case(id=keyed((0, 0), (4, 1000001)))),
}


def warning_at_frame(c):
    if c["on"] == "adjust" or c["layer"] != "scene" or D.passes(c["file"], c["pass"]) is None:
        return "EFFECT_CHANNEL_MISSING"
    if D.non_finite(c["file"], c["pass"]):
        return "MEDIA_EXR_ADJUSTED"
    return None


def main():
    make_files()
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): D.render(c, f) for f in frames}
        entry = {"says": says, "project": D.write_project(fx, c), "frames": rendered}
        said = warning_at_frame(c)
        if said:
            entry["frame_warning"] = said
        expected["cases"][fx] = entry
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != D.plain(c, int(f))[i] for i in range(W * H))} changed"
            for f, px in rendered.items()) + (f", {said}" if said else ""))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": D.write_project(fx, c),
                                 "frames": {"0": D.plain(c, 0), "4": D.plain(c, 4)},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_identifier.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda px, x, y: px[y * W + x]  # noqa: E731
    scene = D.plain(case(), 0)
    block = lambda x, y: x // 3 + 3 * (y // 3)  # noqa: E731
    one = c["FX-IDENT-001"]["0"]
    assert one == c["FX-IDENT-001"]["4"]
    for y in range(H):
        for x in range(W):
            assert at(one, x, y) == colour_of(block(x, y)) + [1.0]
    assert len({tuple(colour_of(k)) for k in range(6)}) == 6
    two = c["FX-IDENT-002"]["0"]
    assert at(two, 5, 2) == colour_of(2) + [1.0] and at(two, 5, 3) == colour_of(3) + [1.0]
    three, four = c["FX-IDENT-003"]["0"], c["FX-IDENT-004"]["0"]
    for y in range(H):
        for x in range(W):
            inside = block(x, y) == 4
            assert at(three, x, y) == ([1.0] * 4 if inside else [0.0, 0.0, 0.0, 1.0])
            assert at(four, x, y) == (at(scene, x, y) if inside else [0.0] * 4)
    five = c["FX-IDENT-005"]["0"]
    for k in range(6):
        g = at(five, 3 * (k % 3), 3 * (k // 3))[0]
        enc = 12.92 * g if g <= 0.0031308 else 1.055 * g ** (1 / 2.4) - 0.055
        assert abs(enc * 255 - k) < 1e-9, (k, enc)
    six = c["FX-IDENT-006"]["0"]
    assert abs(at(six, 5, 2)[0] - srgb_to_linear(float(D.np.float32(2.4)) / 255)) < 1e-12
    seven = c["FX-IDENT-007"]["0"]
    assert at(seven, 5, 2) == [1.0] * 4 and at(seven, 5, 3) == [0.0, 0.0, 0.0, 1.0]
    eight = c["FX-IDENT-008"]
    assert eight["0"] == D.render(case(id=0, output="luma_matte"), 0)
    assert eight["2"] == D.render(case(id=2, output="luma_matte"), 0)
    assert eight["4"] == three
    assert at(c["FX-IDENT-009"]["0"], 4, 0) == at(c["FX-IDENT-009"]["0"], 0, 3)
    ten = c["FX-IDENT-010"]["0"]
    assert all(at(ten, x, y)[0] == (1.0 if y % 2 else 0.0) for y in range(H) for x in range(W))
    eleven = c["FX-IDENT-011"]["0"]
    assert at(eleven, 6, 0) == [1.0] * 4 and at(eleven, 0, 5) == [0.0, 0.0, 0.0, 1.0]
    assert at(eleven, 7, 4) == [1.0] * 4 and at(eleven, 7, 5) == [0.0, 0.0, 0.0, 1.0]
    assert expected["cases"]["FX-IDENT-011"]["frame_warning"] == "MEDIA_EXR_ADJUSTED"
    for fx in ("FX-IDENT-013", "FX-IDENT-014", "FX-IDENT-015"):
        assert expected["cases"][fx]["frame_warning"] == "EFFECT_CHANNEL_MISSING"
    assert c["FX-IDENT-013"]["0"] == scene and c["FX-IDENT-015"]["0"] == scene
    assert all(c["FX-IDENT-016"]["0"][(y + 1) * W + x + 2] == one[y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    assert at(c["FX-IDENT-016"]["0"], 0, 0) == [0.0] * 4
    seventeen = c["FX-IDENT-017"]["0"]
    assert at(seventeen, 1, 1) != at(scene, 1, 1) and at(seventeen, 4, 1) == [0.0] * 4
    assert all(abs(a - b / 2) < 1e-12 for a, b in zip(at(c["FX-IDENT-018"]["0"], 2, 2)[:3],
                                                       at(one, 2, 2)[:3]))
    assert all(p == [0.0] * 4 for p in c["FX-IDENT-019"]["0"])
    print("checks passed")


if __name__ == "__main__":
    main()
