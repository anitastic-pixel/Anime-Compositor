"""Blobbylize, worked a second way.

D-379 adds `core.blobbylize`, after CycoreFX's CC Blobbylize: the layer cut out by a soft, glossy
blob shape read from a channel of a layer, its own or another (D-189's layer setting), and lit.
CycoreFX's manual says what each control does in a sentence and publishes no formula; the rule
below is this program's own reading of it, and nothing is ported.

At the holder's frame, with its input O, linear and premultiplied:

1. The map is D-189's layer setting, fitted to the drawing's own W by H and lying on it, clear
   outside it; with the name "" (or a layer not in the composition, with the warning) it is O
   itself. F, the blob's height, is at each pixel the map's `property` as CC Vector Blur reads
   it: red, green, blue, luminance or lightness of the straight encoded colour, times the
   covering; or the covering itself for `alpha`.
2. Bv is F through document 21's Gaussian at sigma `softness / 2`, nothing outside the drawing,
   worked out past its edges as far as the kernel reaches; at softness 0, Bv is F and nothing
   outside. Ob is O through the same Gaussian, all four channels (O itself at softness 0).
3. The colour: K = O + (1 - O.a) Ob, the layer over its own blur, and C = K.rgb / K.a, straight
   (0 where K.a is 0). So where the layer is solid it keeps its own colour, sharp, and where the
   blob reaches past it the colour is the layer's spread from nearby.
4. The covering: with c = `cut_away / 100`, a = (Bv - c) / (1 - c) held to 0..1; at c = 1, 0.
   Cut Away 0 keeps the blob's values as they are; 100 cuts everything away.
5. The surface: N is (-k gx, -k gy, 1) made one long, g the central differences of Bv,
   `(Bv(x+1, y) - Bv(x-1, y)) / 2` and down the same, k = 1.25 max(softness, 1), as CC Glass at
   height 100 slopes its bump.
6. The light, one of the effect's own (After Effects' light layers are not offered): Lv, toward
   it, made one long, is (100 ux, 100 uy, `light_height`) for `distant`, (ux, uy) document 21's
   step at `light_direction` degrees clockwise from up; or for `point`, the light at
   `light_position` (per cent of the drawing's width and height) and `light_height` pixels
   above, Lv = (Lx - px, Ly - py, light_height) from the pixel's centre; at no length, none.
   A negative height is behind the layer.
7. The shading, Phong's: nl = N.Lv, diffuse max(nl, 0), the reflection R = 2 nl N - Lv and the
   highlight Rz^(1 / roughness) where nl and Rz are above 0, else 0. With I = `light_intensity /
   100`, Lc the light colour in linear light and ambient, diffuse, specular and metal as shares
   of 100: each channel out = C (ambient + diffuse I Lc diffuse-term) + specular I (Lc + (C - Lc)
   metal) highlight, so at metal 100 the highlight is the layer's colour and at 0 the light's.
8. The output is out times a, with covering a. The layer does not grow. For a draft the softness
   is a distance, and so is the height of a point light.

`layer` D-189's word, ""; `fit` `stretch`, `center` or `tile`, `stretch`; `property` red, green,
blue, alpha, luminance or lightness, `alpha`; `softness` 0 to 100, 10; `cut_away` 0 to 100, 0;
`light_intensity` 0 to 400, 100; `light_color` a colour, white; `light_type` `distant` or `point`,
`distant`; `light_height` -1000 to 1000, 100; `light_position` -1000 to 1000 per cent each way,
30, 30; `light_direction` -3600 to 3600, -45; `ambient` 25, `diffuse` 75, `specular` 50 and
`metal` 100, each 0 to 100; `roughness` 0.001 to 1, 0.05. Every number keyable. The values when
added are chosen here; the manual gives none.

**This file never runs the build's code path.** It sums the two-dimensional kernel directly where
the build runs two passes, in double precision from the drawings' 8-bit values.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/blobbylize/`: the
drawing `holder` at the top with the effect, and under it the map layers, every one switched
off. The expected pixels are in `Fixtures/blobbylize/expected_blobbylize.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/blobbylize_reference.py
"""

import json
import sys
from math import ceil, exp, sqrt
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from vector_blur_reference import phase, along  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "blobbylize"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
NONE = (0, 0, 0, 0)
RED, BLUE, CREAM = (220, 40, 40, 255), (40, 80, 220, 255), (240, 240, 200, 255)
WHITE, BLACK = (255, 255, 255, 255), (0, 0, 0, 255)


def shapes_px(x, y):
    """A red block, columns 1 to 5 and rows 2 to 7, and a blue one, columns 9 to 11 and rows 3
    to 6, three clear columns between them, clear round them."""
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
    "shapes": [[shapes_px(x, y) for x in range(W)] for y in range(H)],
    # Two-pixel squares of red, blue and cream, solid all over.
    "photo": [[(RED, BLUE, CREAM)[(x // 2 + y // 2) % 3] for x in range(W)] for y in range(H)],
    "dots": [[dots_px(x, y) for x in range(W)] for y in range(H)],
    # Black at the left to white at the right, one step of 17 a column.
    "ramp": [[(17 * x, 17 * x, 17 * x, 255) for x in range(W)] for y in range(H)],
    # 8 by 5: a red disc 2 pixels round the middle on black; stretched to the drawing's size.
    "spot": [[(255, 0, 0, 255) if (x + 0.5 - 4) ** 2 + (y + 0.5 - 2.5) ** 2 <= 4 else BLACK
              for x in range(8)] for y in range(5)],
}


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


MAPS = ("dots", "ramp", "spot")


def hex_linear(h):
    return [srgb_to_linear(int(h[i:i + 2], 16) / 255) for i in (1, 3, 5)]


# --- the rule -------------------------------------------------------------------------------

def kernel(sigma):
    r = ceil(3 * sigma) if sigma > 0 else 0
    if r == 0:
        return 0, [1.0]
    one = [exp(-(k * k) / (2 * sigma * sigma)) for k in range(-r, r + 1)]
    return r, [v / sum(one) for v in one]


def blobbylize(o, m, s):
    w, h = o["w"], o["h"]
    f = [phase(s["property"], p) for p in m["px"]]
    r, one = kernel(s["softness"] / 2)

    def bv(x, y):
        if r == 0:
            return f[y * w + x] if 0 <= x < w and 0 <= y < h else 0.0
        return sum(f[j * w + i] * one[i - x + r] * one[j - y + r]
                   for j in range(max(0, y - r), min(h, y + r + 1))
                   for i in range(max(0, x - r), min(w, x + r + 1)))

    ob = L.blur(o, s["softness"] / 2) if r else o
    c = s["cut_away"] / 100
    k = 1.25 * max(s["softness"], 1.0)
    lc = hex_linear(s["light_color"])
    inten = s["light_intensity"] / 100
    amb, dif, spc, metal = (s[n] / 100 for n in ("ambient", "diffuse", "specular", "metal"))
    out = []
    for y in range(h):
        for x in range(w):
            b = bv(x, y)
            a = 0.0 if c >= 1 else min(max((b - c) / (1 - c), 0.0), 1.0)
            if a == 0:
                out.append([0.0] * 4)
                continue
            p, q = o["px"][y * w + x], ob["px"][y * w + x]
            kk = [p[i] + (1 - p[3]) * q[i] for i in range(4)]
            col = [kk[i] / kk[3] for i in range(3)] if kk[3] > 0 else [0.0] * 3
            gx, gy = (bv(x + 1, y) - bv(x - 1, y)) / 2, (bv(x, y + 1) - bv(x, y - 1)) / 2
            nn = [-k * gx, -k * gy, 1.0]
            ln = sqrt(sum(v * v for v in nn))
            nn = [v / ln for v in nn]
            if s["light_type"] == "point":
                lx, ly = s["light_position"][0] / 100 * w, s["light_position"][1] / 100 * h
                lv = [lx - (x + 0.5), ly - (y + 0.5), s["light_height"]]
            else:
                ux, uy = along(s["light_direction"])
                lv = [100 * ux, 100 * uy, s["light_height"]]
            ll = sqrt(sum(v * v for v in lv))
            lv = [v / ll for v in lv] if ll > 0 else [0.0] * 3
            nl = sum(nn[i] * lv[i] for i in range(3))
            rz = 2 * nl * nn[2] - lv[2]
            hi = rz ** (1 / s["roughness"]) if nl > 0 and rz > 0 else 0.0
            shaded = [col[i] * (amb + dif * inten * lc[i] * max(nl, 0.0))
                      + spc * inten * (lc[i] + (col[i] - lc[i]) * metal) * hi for i in range(3)]
            out.append([v * a for v in shaded] + [a])
    return L.pic(w, h, out)


# --- the cases ------------------------------------------------------------------------------

NAMES = ("layer", "fit", "property", "softness", "cut_away", "light_intensity", "light_color",
         "light_type", "light_height", "light_position", "light_direction", "ambient",
         "diffuse", "specular", "roughness", "metal")
FLAT = {"ambient": 100, "diffuse": 0, "specular": 0}  # no light: the blob's shape and colour


def case(holder="shapes", shift=(0, 0), **kw):
    c = {"layer": "", "fit": "stretch", "property": "alpha", "softness": 10, "cut_away": 0,
         "light_intensity": 100, "light_color": "#ffffff", "light_type": "distant",
         "light_height": 100, "light_position": (30, 30), "light_direction": -45,
         "ambient": 25, "diffuse": 75, "specular": 50, "roughness": 0.05, "metal": 100}
    c.update(kw)
    c["holder"], c["shift"] = holder, shift
    return c


def the_map(c):
    if c["layer"] in MAPS:
        return L.fit(decoded(c["layer"]), c["fit"], W, H)
    return decoded(c["holder"])  # "" and a layer not there: the layer itself


def settings(c, frame):
    return {k: value_at(c[k], frame) for k in NAMES}


def render(c, frame):
    out = blobbylize(decoded(c["holder"]), the_map(c), settings(c, frame))
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded(c["holder"])
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


SOFT4 = dict(softness=4)
CASES = {
    "FX-BLOB-001": ("As added: the layer's own covering as the blob, Softness 10, Cut Away 0, a "
                    "white distant light from the top left at height 100, Ambient 25, Diffuse "
                    "75, Specular 50, Roughness 0.05, Metal 100: the two blocks melt into one "
                    "soft, lit blob, lighter on its upper left slopes.", case(), (0,)),
    "FX-BLOB-002": ("Softness 0, Ambient 100, Diffuse 0, Specular 0: the blob is the covering "
                    "itself and nothing is lit, so the drawing comes back untouched.",
                    case(softness=0, **FLAT), (0,)),
    "FX-BLOB-003": ("Softness 4, unlit: the blocks' edges go soft and the three clear columns "
                    "between them fill part way, their colour spread from the blocks, red into "
                    "blue.", case(**SOFT4, **FLAT), (0,)),
    "FX-BLOB-004": ("The same, Cut Away 30: the softest 30 per cent cut away and the rest "
                    "stretched back to full, a firmer blob, the gap thinner.",
                    case(cut_away=30, **SOFT4, **FLAT), (0,)),
    "FX-BLOB-005": ("Cut Away 100: everything cut away, an empty frame.",
                    case(cut_away=100, **SOFT4), (0,)),
    "FX-BLOB-006": ("Softness 4, lit as added: lighter on the slopes that face the top left, "
                    "darker on those facing away, the flat middles at Ambient plus Diffuse "
                    "times 0.71.", case(**SOFT4), (0,)),
    "FX-BLOB-007": ("The light's Direction 135, from the bottom right: the other slopes light.",
                    case(light_direction=135, **SOFT4), (0,)),
    "FX-BLOB-008": ("A point light at 25, 30 per cent, 10 pixels up: brightest near (4, 3), "
                    "falling away across the blocks.",
                    case(light_type="point", light_position=(25, 30), light_height=10, **SOFT4),
                    (0,)),
    "FX-BLOB-009": ("The light's Height -50, behind the layer: the flat middles face away from "
                    "it, so only Ambient lights them; a slope tipped far enough toward it "
                    "catches a little.", case(light_height=-50, **SOFT4), (0,)),
    "FX-BLOB-010": ("Specular 100, Roughness 0.5, Metal 0, an orange light, Diffuse 0: a broad "
                    "highlight in the light's own orange, not the blocks' colour.",
                    case(specular=100, roughness=0.5, metal=0, diffuse=0, light_color="#ff8000",
                         light_height=40, **SOFT4), (0,)),
    "FX-BLOB-011": ("An orange light at Intensity 150, Diffuse 100: the blocks tinted warm and "
                    "brighter.", case(light_color="#ff8000", light_intensity=150, diffuse=100,
                                      **SOFT4), (0,)),
    "FX-BLOB-012": ("The dots layer as the blob, its covering, Softness 2, on the solid photo: "
                    "the photo cut down to two soft round blobs where the dots are.",
                    case(holder="photo", layer="dots", softness=2), (0,)),
    "FX-BLOB-013": ("The ramp as the blob, Property Luminance, Softness 0, unlit, on the photo: "
                    "the photo fading in from clear at the left to solid at the right.",
                    case(holder="photo", layer="ramp", property="luminance", softness=0, **FLAT),
                    (0,)),
    "FX-BLOB-014": ("The 8 by 5 spot stretched to the drawing's size, Property Red, Softness 2, "
                    "on the photo: an oval blob in the middle.",
                    case(holder="photo", layer="spot", property="red", softness=2), (0,)),
    "FX-BLOB-015": ("A layer that is not in the composition, `gone`: the layer itself is the "
                    "blob, as FX-BLOB-001, and the warning every frame.", case(layer="gone"),
                    (0, 4)),
    "FX-BLOB-016": ("Softness keyed from 0 at frame 0 to 8 at frame 4, linear: frame 2 is "
                    "FX-BLOB-006.", case(softness=keyed((0, 0), (4, 8))), (0, 2, 4)),
    "FX-BLOB-017": ("FX-BLOB-006 on the holder moved 2 right and 1 down: the same, moved.",
                    case(shift=(2, 1), **SOFT4), (0,)),
    "FX-BLOB-018": ("Cut Away eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                    "overshoots: at frame 2 it would pass 100, is held at 100, and the frame is "
                    "empty, as frame 4 is.",
                    case(cut_away=keyed((0, 0, OVERSHOOT), (4, 100)), **SOFT4), (0, 2, 4)),
    "FX-BLOB-019": ("Property Lightness on the photo itself, Softness 2, Cut Away 20, lit: the "
                    "cream squares stand up as blobs, the red and blue lower.",
                    case(holder="photo", property="lightness", softness=2, cut_away=20), (0,)),
}

INVALID = {
    "FX-BLOB-020": ("Softness 101, above 100.", case(softness=101)),
    "FX-BLOB-021": ("Cut Away -1, below 0.", case(cut_away=-1)),
    "FX-BLOB-022": ("A property written \"hue\", which Blobbylize does not read.",
                    case(property="hue")),
    "FX-BLOB-023": ("A light type written \"spot\".", case(light_type="spot")),
    "FX-BLOB-024": ("Roughness 0, below 0.001.", case(roughness=0)),
    "FX-BLOB-025": ("A layer written as the number 3, not a word.", case(layer=3)),
    "FX-BLOB-026": ("A light position 50, 1001, past ten heights.",
                    case(light_position=(50, 1001))),
    "FX-BLOB-027": ("Metal keyed to 150 at frame 4.", case(metal=keyed((0, 100), (4, 150)))),
}


def effect(c):
    return {"instance_id": "fx-1", "type_id": "core.blobbylize", "enabled": True,
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
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
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
    (OUT / "expected_blobbylize.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    photo = plain(case(holder="photo"))
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    clear = [0.0] * 4
    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(u >= -1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx] and fx in INVALID:
            base = plain(INVALID[fx][1])
            assert all(px == base for px in frames.values())
    assert near(c["FX-BLOB-002"]["0"], drawn)
    three, four = c["FX-BLOB-003"]["0"], c["FX-BLOB-004"]["0"]
    # The gap fills part way, and less with the cut; its colour lies between red and blue.
    gap = three[at(7, 5)]
    assert 0.05 < gap[3] < 0.95 and drawn[at(7, 5)] == clear
    assert four[at(7, 5)][3] < gap[3]
    assert gap[0] > 0 and gap[2] > 0
    # Inside a block the colour is the block's own, unlit.
    red = drawn[at(3, 4)]
    assert near([[v / three[at(3, 4)][3] for v in three[at(3, 4)][:3]]], [red[:3]], 1e-9)
    assert all(p == clear for p in c["FX-BLOB-005"]["0"])
    six, seven = c["FX-BLOB-006"]["0"], c["FX-BLOB-007"]["0"]
    assert six != three and seven != six
    # Upper left slope lighter than lower right with the light at the top left; turned round
    # with it.
    lum = lambda p: sum(p[:3]) / p[3] if p[3] > 0 else 0  # noqa: E731
    assert lum(six[at(1, 2)]) > lum(six[at(5, 7)]) and lum(seven[at(1, 2)]) < lum(seven[at(5, 7)])
    assert c["FX-BLOB-008"]["0"] not in (six, seven)
    nine = c["FX-BLOB-009"]["0"]
    flat = nine[at(3, 4)]
    assert near([[v / flat[3] for v in flat[:3]]], [[0.25 * v for v in red[:3]]], 1e-6)
    ten = c["FX-BLOB-010"]["0"]
    assert ten[at(2, 3)][0] > 4 * ten[at(2, 3)][2]  # the highlight is the light's orange
    twelve = c["FX-BLOB-012"]["0"]
    assert twelve[at(3, 3)][3] > 0.5 and twelve[at(7, 1)][3] < 0.05
    thirteen = c["FX-BLOB-013"]["0"]
    for y in range(H):
        assert thirteen[at(0, y)] == clear and abs(thirteen[at(15, y)][3] - 1) < 1e-12
    assert c["FX-BLOB-014"]["0"][at(8, 5)][3] > c["FX-BLOB-014"]["0"][at(1, 1)][3]
    assert c["FX-BLOB-015"]["0"] == c["FX-BLOB-001"]["0"] == c["FX-BLOB-015"]["4"]
    sixteen = c["FX-BLOB-016"]
    assert sixteen["2"] == six and sixteen["0"] != six
    seventeen = c["FX-BLOB-017"]["0"]
    assert all(seventeen[at(x, y)] == six[at(x - 2, y - 1)] for x in range(2, W) for y in range(1, H))
    eighteen = c["FX-BLOB-018"]
    assert all(p == clear for f in ("2", "4") for p in eighteen[f]) and eighteen["0"] != eighteen["2"]
    nineteen = c["FX-BLOB-019"]["0"]
    assert nineteen != photo
    print("checked")


if __name__ == "__main__":
    main()
