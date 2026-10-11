"""Cryptomatte, worked a second way.

D-522 adds `core.cryptomatte`, Cryptomatte, after After Effects' Cryptomatte (3D Channel), on
EFFECTS.md row 375. A render that saves Cryptomatte passes writes, for every pixel, a short list
of the objects seen in it (each as a number made from the object's name) with how much of the
pixel each covers. Typing an object's name picks it, and the coverage is its matte: soft at
antialiased, blurred or see-through edges, as an ID matte cannot be.

**The file**, by Psyop's Cryptomatte specification 1.2.0. A Cryptomatte layer named T (its "type
name", such as `CryptoObject`) is the channels `T00.r`, `T00.g`, `T00.b`, `T00.a`, `T01.r` and
so on. In level kk, red is the id of rank 2kk and green its coverage, blue the id of rank 2kk+1
and alpha its coverage. The channels `T.r`, `T.g`, `T.b` (no digits) are the old preview, and
are not read.

- A name is split at its last dot into a prefix and a part, the part's case ignored. A layer is
  a prefix ending in `00`, with at least one character before it, holding a part `r`; T is the
  prefix without the `00`. The layers are taken in the file's order of channel names (an EXR
  file keeps them sorted).
- `layer`, the effect's layer name: `""` takes the first; otherwise the first layer whose T, or
  whose T's last dotted word, equals it, case ignored (so `CryptoObject` finds Blender's
  `ViewLayer.CryptoObject`).
- The levels are 00, 01, 02 and on, while `T<kk>` has a part `r`. Each level gives the rank red
  with green, and, when it has both, blue with alpha. A level missing its green gives no rank.
- A sample is read as single precision. A coverage that is not finite is read as D-62 reads one
  (not-a-number 0, an infinity the largest number of its sign); an id that is not finite is read
  as 0, no object. Each frame that reads such a number says `MEDIA_EXR_ADJUSTED`
  (`non_finite: n`, over the rank channels).

**A name's id.** The name's UTF-8 bytes hashed by MurmurHash3_x86_32 with seed 0, giving a
32-bit number h; where h's bits 23 to 30 (a single-precision exponent) are all 0 or all 1, bit 23
is flipped; the id is h's 32 bits read as a single-precision number. An id equals a pixel's when
the two single-precision numbers are equal. Psyop's test value: `torus` hashes to 4053562365, the
id -1.54943624832e+30.

**The settings.** `layer`, any text, `""` when added; `selection`, the names picked, separated by
commas, each trimmed of spaces at its ends, empty ones ignored, `""` when added; `output`,
`colors`, `matte` or `keyed`, `colors` when added. None is keyable.

**The rule**, at each pixel of the layer's drawing, with its ranks (id_k, c_k) and S the ids of
the selection:

- m = sum of c_k over the ranks whose id_k is in S, held in 0 to 1.
- The colour of an id v whose bits are b: (b mod 256, floor(b / 256) mod 256, floor(b / 65536)
  mod 128) over (255, 255, 127), the low bits of its number, which a hash scatters; an id in S
  is white (1, 1, 1). P = sum of c_k times the colour of id_k.
- `colors`: the pixel is (P, 1), opaque, whatever the input there was: every object a flat
  colour, the picked ones white, their edges mixed by coverage. `matte`: (m, m, m, 1). `keyed`:
  the input O times m, all four numbers. Outside the drawing every output is transparent.

The ids are the layer's own EXR file at that frame (D-348's route); a layer with no such file,
or a file without the layer named, has none: the effect is skipped, its settings kept, the
picture unchanged, and `EFFECT_CHANNEL_MISSING` is said every frame. The file is never changed.
Nothing has a distance in it; a draft reads the matte or the colours at the file's size and
stretches them to the draft's, as D-189 stretches a map.

**Ours, not Adobe's.** Adobe documents no settings for its Cryptomatte effect (fnord's, bundled
with After Effects); there it is picked by clicking in the composition. The three settings, the
colour of an id, the clamp of m and the names typed are this program's. The manifest (the names
a file lists) is not read: a name is hashed as typed.

**This file never runs the build's code path.** It writes the EXR files with OpenEXR's own
library (3.4.15), hashes the names with its own MurmurHash3 checked against Psyop's test value,
reads the files back with OpenEXR, and works each expected pixel in double precision.

Every case is a project of one composition 8 by 6 at 24 frames a second, five frames long, in
`Fixtures/cryptomatte/`; the expected pixels are `Fixtures/cryptomatte/expected_cryptomatte.json`.
`Fixtures/cryptomatte/sample/balls.exr` is a bigger picture for the playtest: three balls over a
floor, 320 by 180, with object and material Cryptomattes.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/cryptomatte_reference.py   (with OpenEXR 3.4.15 and numpy)
"""

import json
import struct
import sys
from pathlib import Path

import numpy as np
import OpenEXR

sys.path.insert(0, str(Path(__file__).resolve().parent))
import depth_channel_reference as D  # noqa: E402
import effect_layer_reference as L  # noqa: E402
from motion_blur_reference import png  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "cryptomatte"
D.OUT = OUT  # D's file helpers (write, read, picture) work in this folder
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = D.W, D.H, D.FRAMES
CLEAR = D.CLEAR
OUTPUTS = ("colors", "matte", "keyed")


# --- the id of a name -----------------------------------------------------------------------

def murmur3_32(data, seed=0):
    """MurmurHash3_x86_32, as Austin Appleby published it."""
    c1, c2, m = 0xCC9E2D51, 0x1B873593, 0xFFFFFFFF
    rotl = lambda x, r: ((x << r) | (x >> (32 - r))) & m  # noqa: E731
    h = seed
    n = len(data) // 4 * 4
    for i in range(0, n, 4):
        k = int.from_bytes(data[i:i + 4], "little")
        k = rotl(k * c1 & m, 15) * c2 & m
        h = (rotl(h ^ k, 13) * 5 + 0xE6546B64) & m
    k = 0
    tail = data[n:]
    for i in reversed(range(len(tail))):
        k = (k << 8) | tail[i]
    if tail:
        h ^= rotl(k * c1 & m, 15) * c2 & m
    h ^= len(data)
    h ^= h >> 16
    h = h * 0x85EBCA6B & m
    h ^= h >> 13
    h = h * 0xC2B2AE35 & m
    return h ^ (h >> 16)


def name_id(name):
    """The single-precision id of `name`, as a float64 holding it exactly."""
    h = murmur3_32(name.encode("utf-8"))
    if (h >> 23) & 255 in (0, 255):
        h ^= 1 << 23
    return struct.unpack("<f", struct.pack("<I", h))[0]


def bits(v):
    return struct.unpack("<I", struct.pack("<f", v))[0]


def id_colour(v):
    b = bits(v)
    return [(b & 255) / 255, ((b >> 8) & 255) / 255, ((b >> 16) & 127) / 127]


# --- the files ------------------------------------------------------------------------------

def object_cover(x, y):
    """The objects in pixel (x, y) with their coverage, most first."""
    special = {
        (3, 2): [("ball", 0.6), ("floor", 0.4)],
        (4, 4): [("Kugel_ü", 0.5), ("floor", 0.3), ("big box", 0.2)],
        (2, 5): [("floor", 0.4), ("ball", 0.3), ("big box", 0.2), ("Kugel_ü", 0.1)],
        (0, 5): [],  # nothing: the sky
        (7, 0): [("big box", 0.5)],  # half covered, half empty
    }
    if (x, y) in special:
        return special[(x, y)]
    if 1 <= x <= 3 and 1 <= y <= 3:
        return [("ball", 1.0)]
    if x >= 5 and y <= 2:
        return [("big box", 1.0)]
    if x >= 4 and y >= 3:
        return [("Kugel_ü", 1.0)]
    return [("floor", 1.0)]


def material_cover(x, y):
    if x == 3:
        return [("steel", 0.75), ("rubber", 0.25)]
    return [("steel", 1.0)] if x < 3 else [("rubber", 1.0)]


def rank_planes(cover, levels, w=W, h=H):
    """The 4 * levels planes (id, coverage, id, coverage per level) of `cover`."""
    planes = np.zeros((levels * 4, h, w), np.float32)
    for y in range(h):
        for x in range(w):
            for k, (name, c) in enumerate(cover(x, y)[:levels * 2]):
                planes[2 * k, y, x] = name_id(name)
                planes[2 * k + 1, y, x] = c
    return planes


def crypto_channels(prefix, planes, parts="RGBA"):
    out = {}
    for i in range(planes.shape[0]):
        out[f"{prefix}{i // 4:02d}.{parts[i % 4]}"] = planes[i]
    return out


def manifest(key, typename, names):
    hexes = {n: f"{bits(name_id(n)):08x}" for n in names}
    return {f"cryptomatte/{key}/name": typename, f"cryptomatte/{key}/hash": "MurmurHash3_32",
            f"cryptomatte/{key}/conversion": "uint32_to_float32",
            f"cryptomatte/{key}/manifest": json.dumps(hexes, ensure_ascii=False)}


def write_with(rel, channels, attributes, w=W, h=H):
    header = {"compression": OpenEXR.ZIP_COMPRESSION, "type": OpenEXR.scanlineimage,
              "dataWindow": D.box(0, 0, w - 1, h - 1), "displayWindow": D.box(0, 0, w - 1, h - 1),
              "lineOrder": OpenEXR.INCREASING_Y, "pixelAspectRatio": 1.0}
    header.update(attributes)
    path = OUT / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    OpenEXR.File(header, {k: np.ascontiguousarray(v) for k, v in channels.items()}).write(str(path))


OBJECTS = ["ball", "floor", "big box", "Kugel_ü"]


def make_files():
    obj = crypto_channels("CryptoObject", rank_planes(object_cover, 2))
    mat = crypto_channels("CryptoMaterial", rank_planes(material_cover, 1))
    attrs = dict(manifest("d593dd7", "CryptoObject", OBJECTS),
                 **manifest("be93ba3", "CryptoMaterial", ["steel", "rubber"]))
    write_with("media/crypto.exr", dict(D.colour(), **obj, **mat), attrs)
    # Blender's layout: the colour in ViewLayer.Combined, one level, parts in lower case.
    blender = {f"ViewLayer.Combined.{k}": v for k, v in D.colour().items()}
    blender.update(crypto_channels("ViewLayer.CryptoObject", rank_planes(object_cover, 1), "rgba"))
    write_with("media/blender.exr", blender, manifest("a1b2c3d", "ViewLayer.CryptoObject", OBJECTS))
    # One rank only (no blue or alpha), lower-case type name, a coverage that is not a number.
    odd = rank_planes(lambda x, y: [("rock", 1.0)] if x % 2 == 0 else [], 1)
    odd[1, 5, 6] = np.nan
    write_with("media/odd.exr", dict(D.colour(), **{"crypto_asset00.R": odd[0],
                                                    "crypto_asset00.G": odd[1]}), {})
    write_with("media/plain.exr", D.colour(), {})
    L.DRAWINGS["card"] = [[(36 * x, 200, 30 * y, 255) for x in range(W)] for y in range(H)]
    (OUT / "media" / "card.png").write_bytes(png(L.DRAWINGS["card"]))
    balls()


# --- reading them back ----------------------------------------------------------------------

def layers(names):
    found = []
    for n in names:
        prefix, _, part = n.rpartition(".")
        if part.lower() == "r" and len(prefix) > 2 and prefix.endswith("00") \
                and prefix[:-2] not in found:
            found.append(prefix[:-2])
    return found


def chosen_layer(names, wanted):
    found = layers(names)
    if wanted == "":
        return found[0] if found else None
    for t in found:
        if t.lower() == wanted.lower() or t.rpartition(".")[2].lower() == wanted.lower():
            return t
    return None


def ranks(rel, wanted):
    """Each pixel's ranks [(id, coverage)], and how many rank numbers were not finite; None
    where the file has no such layer."""
    c = D.read(rel)
    names = sorted(c)
    t = chosen_layer(names, wanted)
    if t is None:
        return None
    part = lambda kk, p: next((n for n in names if n.rpartition(".")[0] == f"{t}{kk:02d}"  # noqa: E731
                               and n.rpartition(".")[2].lower() == p), None)
    pairs = []
    kk = 0
    while part(kk, "r"):
        if part(kk, "g"):
            pairs.append((part(kk, "r"), part(kk, "g")))
        if part(kk, "b") and part(kk, "a"):
            pairs.append((part(kk, "b"), part(kk, "a")))
        kk += 1
    used = {n for pair in pairs for n in pair}
    bad = sum(1 for n in used for v in c[n].flat if not np.isfinite(v))
    px = []
    for y in range(H):
        for x in range(W):
            row = []
            for i, cv in pairs:
                v = c[i][y, x]
                row.append((float(v) if np.isfinite(v) else 0.0, D.finite(c[cv][y, x])))
            px.append(row)
    return px, bad


# --- the rule -------------------------------------------------------------------------------

def selected(selection):
    return {name_id(n.strip()) for n in selection.split(",") if n.strip()}


def matte(rs, s):
    return min(1.0, max(0.0, sum(c for v, c in rs if v in s)))


def colours(rs, s):
    p = [0.0, 0.0, 0.0]
    for v, c in rs:
        col = [1.0, 1.0, 1.0] if v in s else id_colour(v)
        p = [a + c * b for a, b in zip(p, col)]
    return p


def works(c):
    return c.get("works", True) and c["layer"] == "scene" and c["on"] == "scene" and \
        ranks(c["file"], c["crypto_layer"]) is not None


def effected(c, n):
    p = D.drawing(c)
    if c["before"] is not None:
        p = L.exposure(p, c["before"])
    if not works(c):
        return p
    rs, _ = ranks(c["file"], c["crypto_layer"])
    s = selected(c["selection"])
    if c["output"] == "colors":
        px = [colours(rs[i], s) + [1.0] for i in range(W * H)]
    elif c["output"] == "matte":
        px = [[matte(rs[i], s)] * 3 + [1.0] for i in range(W * H)]
    else:
        px = [[v * matte(rs[i], s) for v in p["px"][i]] for i in range(W * H)]
    out = L.pic(W, H, px)
    if c["after"] is not None:
        out = L.exposure(out, c["after"])
    return out


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.cryptomatte", "enabled": True,
            "parameters": {"layer": c["crypto_layer"], "selection": c["selection"],
                           "output": c["output"]}}


D.effected = effected  # D.render and D.plain draw through these
D.effect = effect


def case(output="colors", layer_="CryptoObject", selection="", **kw):
    c = D.case("crypto", **{"file": kw.pop("file", "crypto.exr")}, **kw)
    c.update({"output": output, "crypto_layer": layer_, "selection": selection})
    return c


def warning_at_frame(c):
    if c["on"] == "adjust" or c["layer"] != "scene" or \
            ranks(c["file"], c["crypto_layer"]) is None:
        return "EFFECT_CHANNEL_MISSING"
    if ranks(c["file"], c["crypto_layer"])[1]:
        return "MEDIA_EXR_ADJUSTED"
    return None


# --- the cases ------------------------------------------------------------------------------

F04 = [0, 4]
CASES = {
    "FX-CRYPTO-001": ("Colors with the layer name empty: the first layer in the file's order, "
                      "CryptoMaterial (sorted before CryptoObject): two flat colours, column 3 "
                      "mixed three to one.", case(layer_=""), F04),
    "FX-CRYPTO-002": ("Colors of CryptoObject, nothing picked: four flat colours, the sky black, "
                      "the edge pixels mixed by coverage; opaque.", case(), F04),
    "FX-CRYPTO-003": ("Colors of CryptoObject, ball picked: the ball white.",
                      case(selection="ball"), F04),
    "FX-CRYPTO-004": ("Matte of ball: white on the ball, 0.6 at (3, 2), 0.3 at (2, 5).",
                      case("matte", selection="ball"), F04),
    "FX-CRYPTO-005": ("Matte of ball and floor together: the coverages added.",
                      case("matte", selection="ball, floor"), F04),
    "FX-CRYPTO-006": ("Keyed on ball: the picture kept on the ball, soft at its edge.",
                      case("keyed", selection="ball"), F04),
    "FX-CRYPTO-007": ("Keyed on a name with a space and one beyond ASCII (UTF-8): big box and "
                      "Kugel_ü, extra spaces and an empty name around them.",
                      case("keyed", selection="  big box ,, Kugel_ü "), F04),
    "FX-CRYPTO-008": ("Keyed on a name the file lacks: all transparent.",
                      case("keyed", selection="teapot"), F04),
    "FX-CRYPTO-009": ("Keyed with nothing picked: all transparent.", case("keyed"), F04),
    "FX-CRYPTO-010": ("Matte of steel from CryptoMaterial: columns 0 to 2 white, column 3 at "
                      "0.75.", case("matte", layer_="CryptoMaterial", selection="steel"), F04),
    "FX-CRYPTO-011": ("The layer name in another case finds the same layer: as FX-CRYPTO-004.",
                      case("matte", layer_="cRYPTOoBJECT", selection="ball"), F04),
    "FX-CRYPTO-012": ("Blender's layout, found by the last dotted word: one level only, so the "
                      "third and fourth objects in a pixel are not there.",
                      case("matte", selection="ball", file="blender.exr"), F04),
    "FX-CRYPTO-013": ("Blender's layout by its whole name, keyed on Kugel_ü.",
                      case("keyed", layer_="ViewLayer.CryptoObject", selection="Kugel_ü",
                           file="blender.exr"), F04),
    "FX-CRYPTO-014": ("A layer name the file lacks: unchanged, said each frame.",
                      case("matte", layer_="CryptoAsset", selection="ball"), F04),
    "FX-CRYPTO-015": ("An EXR with no Cryptomatte: unchanged, said each frame.",
                      case("matte", layer_="", selection="ball", file="plain.exr"), F04),
    "FX-CRYPTO-016": ("On a PNG layer: unchanged, said each frame.",
                      case("matte", selection="ball", layer="card"), F04),
    "FX-CRYPTO-017": ("On a solid: unchanged, said each frame.",
                      case("matte", selection="ball", layer="solid"), F04),
    "FX-CRYPTO-018": ("One rank (no blue or alpha), a lower-case type name, a coverage that is "
                      "not a number (read as 0, said each frame): rock on the even columns.",
                      case("matte", layer_="crypto_asset", selection="rock", file="odd.exr"), F04),
    "FX-CRYPTO-019": ("The layer moved 2 right and 1 down: the matte moves with it.",
                      case("keyed", selection="ball", move=(2, 1)), F04),
    "FX-CRYPTO-020": ("After an Exposure of -1: the darker picture keyed.",
                      case("keyed", selection="big box", before=-1), F04),
    "FX-CRYPTO-021": ("On an adjustment layer above: no file of its own, so unchanged, said each "
                      "frame.", case("keyed", selection="ball", on="adjust"), F04),
    "FX-CRYPTO-022": ("Colors with the floor and Kugel_ü picked, then an Exposure of +1.",
                      case(selection="floor,Kugel_ü", after=1), F04),
}

INVALID = {
    "FX-CRYPTO-023": ("Cryptomatte's output written \"preview\".", case("preview")),
}


def main():
    assert murmur3_32(b"torus") == 4053562365
    assert abs(name_id("torus") / -1.54943624832e+30 - 1) < 1e-11
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
    (OUT / "expected_cryptomatte.json").write_text(
        json.dumps(expected, indent=1, ensure_ascii=False) + "\n", encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    scene = D.plain(case(), 0)
    near = lambda a, b: all(abs(p - q) < 1e-6 for u, v in zip(a, b)  # noqa: E731
                            for p, q in zip(u, v))
    at = lambda fx, x, y: c[fx][y * W + x]  # noqa: E731
    steel, rubber = id_colour(name_id("steel")), id_colour(name_id("rubber"))
    assert near([at("FX-CRYPTO-001", 0, 0)[:3]], [steel])
    assert near([at("FX-CRYPTO-001", 3, 0)[:3]], [[0.75 * a + 0.25 * b for a, b in zip(steel, rubber)]])
    assert all(p[3] == 1 for p in c["FX-CRYPTO-002"])
    assert at("FX-CRYPTO-002", 0, 5) == [0.0, 0.0, 0.0, 1.0]
    assert at("FX-CRYPTO-003", 2, 2) == [1.0, 1.0, 1.0, 1.0]
    assert at("FX-CRYPTO-003", 0, 0) == at("FX-CRYPTO-002", 0, 0)
    m4 = [p[0] for p in c["FX-CRYPTO-004"]]
    assert m4[2 * W + 2] == 1 and abs(m4[2 * W + 3] - 0.6) < 1e-7 and abs(m4[5 * W + 2] - 0.3) < 1e-7
    assert m4[0] == 0 and abs(sum(m4) - (9 - 1 + 0.6 + 0.3)) < 1e-6
    m5 = [p[0] for p in c["FX-CRYPTO-005"]]
    assert abs(m5[5 * W + 2] - 0.7) < 1e-7 and abs(m5[4 * W + 4] - 0.3) < 1e-7 and m5[0] == 1
    for i in range(W * H):
        assert near([c["FX-CRYPTO-006"][i]], [[v * m4[i] for v in scene[i]]])
    assert near(c["FX-CRYPTO-011"], c["FX-CRYPTO-004"])
    assert all(p == CLEAR for fx in ("FX-CRYPTO-008", "FX-CRYPTO-009") for p in c[fx])
    assert at("FX-CRYPTO-007", 2, 2) == CLEAR
    assert near([at("FX-CRYPTO-007", 4, 4)], [[v * 0.7 for v in scene[4 * W + 4]]])
    assert near([at("FX-CRYPTO-007", 2, 5)], [[v * 0.3 for v in scene[5 * W + 2]]])
    assert near([at("FX-CRYPTO-007", 6, 1)], [scene[W + 6]]) and near([at("FX-CRYPTO-007", 7, 0)], [[v * 0.5 for v in scene[7]]])
    m10 = [p[0] for p in c["FX-CRYPTO-010"]]
    assert m10[0] == 1 and m10[3] == 0.75 and m10[4] == 0
    m12 = [p[0] for p in c["FX-CRYPTO-012"]]
    # One level keeps two objects a pixel: the ball is second at (2, 5), so its matte is
    # FX-CRYPTO-004's; Kugel_ü, fourth there, is gone (FX-CRYPTO-013).
    assert abs(m12[5 * W + 2] - 0.3) < 1e-7 and near(c["FX-CRYPTO-012"], c["FX-CRYPTO-004"])
    assert near([at("FX-CRYPTO-013", 2, 5)], [CLEAR]) and near([at("FX-CRYPTO-013", 5, 5)], [scene[5 * W + 5]])
    for fx in ("FX-CRYPTO-014", "FX-CRYPTO-015", "FX-CRYPTO-021"):
        assert c[fx] == scene, fx
    assert c["FX-CRYPTO-017"] == [D.SOLID + [1.0]] * (W * H)
    m18 = [p[0] for p in c["FX-CRYPTO-018"]]
    assert all(m18[i] == (1.0 if i % W % 2 == 0 else 0.0) for i in range(W * H) if i != 5 * W + 6)
    assert m18[5 * W + 6] == 0.0
    assert expected["cases"]["FX-CRYPTO-018"]["frame_warning"] == "MEDIA_EXR_ADJUSTED"
    assert all(c["FX-CRYPTO-019"][(y + 1) * W + x + 2] == c["FX-CRYPTO-006"][y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    assert near([at("FX-CRYPTO-020", 6, 1)], [[v * (0.5 if k < 3 else 1) for k, v in enumerate(scene[W + 6])]])
    assert at("FX-CRYPTO-022", 0, 0)[:3] == [2.0, 2.0, 2.0]
    for fx, v in expected["cases"].items():
        if "4" in v["frames"]:
            assert v["frames"]["4"] == v["frames"]["0"], fx  # nothing moves in time
    print("checks passed")


# --- the playtest's picture -----------------------------------------------------------------

def balls():
    """Three balls over a two-tone floor, 320 by 180, drawn 4 by 4 times a pixel, so the
    Cryptomatte coverage is soft at every edge; object names `red ball`, `blue ball`, `yellow
    ball`, `floor`, `sky`; material names `paint` (red and blue), `gold`, `tile`, `air`."""
    w, h, n = 320, 180, 4
    discs = [("red ball", "paint", (90, 100), 45, (0.9, 0.25, 0.2)),
             ("blue ball", "paint", (170, 95), 38, (0.2, 0.6, 0.9)),
             ("yellow ball", "gold", (240, 105), 50, (0.95, 0.8, 0.2))]
    col = np.zeros((4, h, w))
    obj_cover, mat_cover = {}, {}
    for y in range(h):
        for x in range(w):
            objs, mats, rgb = {}, {}, np.zeros(3)
            for j in range(n):
                for i in range(n):
                    px, py = x + (i + 0.5) / n, y + (j + 0.5) / n
                    hit = None
                    for name, mat, (cx, cy), r, c in discs:  # later balls are in front
                        if (px - cx) ** 2 + (py - cy) ** 2 <= r * r:
                            shade = 0.35 + 0.65 * max(0.0, 1 - ((px - cx + r / 3) ** 2
                                                                + (py - cy + r / 3) ** 2) / (r * r))
                            hit = (name, mat, np.array(c) * shade)
                    if hit is None:
                        if py > 120:
                            tile = (int(px // 40) + int(py // 20)) % 2
                            hit = ("floor", "tile", np.array([0.5, 0.5, 0.45] if tile else [0.3, 0.3, 0.28]))
                        else:
                            hit = ("sky", "air", np.array([0.55, 0.7, 0.9]) * (0.6 + py / 300))
                    objs[hit[0]] = objs.get(hit[0], 0) + 1 / (n * n)
                    mats[hit[1]] = mats.get(hit[1], 0) + 1 / (n * n)
                    rgb += hit[2] / (n * n)
            col[:3, y, x], col[3, y, x] = rgb, 1.0
            obj_cover[(x, y)] = sorted(objs.items(), key=lambda kv: -kv[1])
            mat_cover[(x, y)] = sorted(mats.items(), key=lambda kv: -kv[1])
    channels = {k: col[i].astype("f2") for i, k in enumerate("RGBA")}
    channels.update(crypto_channels("CryptoObject", rank_planes(lambda x, y: obj_cover[(x, y)], 2, w, h)))
    channels.update(crypto_channels("CryptoMaterial", rank_planes(lambda x, y: mat_cover[(x, y)], 2, w, h)))
    attrs = dict(manifest("d593dd7", "CryptoObject", ["red ball", "blue ball", "yellow ball", "floor", "sky"]),
                 **manifest("be93ba3", "CryptoMaterial", ["paint", "gold", "tile", "air"]))
    write_with("sample/balls.exr", channels, attrs, w, h)


if __name__ == "__main__":
    main()
