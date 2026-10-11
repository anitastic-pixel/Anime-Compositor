"""Texturize, worked a second way.

D-467 adds `core.texturize`, after After Effects' Texturize (Stylize): another layer's picture
pressed into the layer as a relief, so the layer looks printed on that texture. What is known
(Adobe's After Effects help, Stylize effects, Texturize; a Creative COW thread, "Texturize effect
alternative for depth in texture"; Mike Murphy's tutorial "After Effects: How To Use Texturize
Effect") is: Texture Layer names the layer whose picture is the texture, and the tutorial hides
that layer's eye so that only the relief shows; Light Direction is the angle the light strikes
it from; Texture Contrast is how strong the relief is; Texture Placement is Tile Texture, Center
Texture or Stretch Texture To Fit; the texture's own colours are not laid on the layer, only its
lights and darks as bumps. The rule below is this program's own, Emboss's colour relief (D-145)
read from the texture instead of the layer itself, and nothing is ported.

The rule, at the holder's frame, with its input O, linear and premultiplied:

1. The texture is D-189's layer setting, fitted to the drawing's own W by H by `fit` (`tile`,
   `center` or `stretch`, After Effects' three placements) and lying on it. With the name ""
   (no texture, as the effect is added) or a layer not in the composition (with the warning),
   nothing happens; nor with Texture Contrast 0.
2. t is at each pixel the texture's luminance as CC Vector Blur reads it: the encoded luminance
   of its straight colour, times its covering; 0 where the texture is clear or not there.
3. d is one pixel along u(`light_direction`), u(θ) = (sin θ, -cos θ), degrees clockwise from up,
   exact at whole quarter turns. At a pixel with covering a > 0 and centre P, each sample of t
   document 21's bilinear sample with the point first held inside the pixel centres (Emboss's
   held sample): v = `texture_contrast` (t(P + d) - t(P - d)) / 2, so the side of each bright
   part facing along d turns lighter and the far side darker.
4. Each encoded straight channel e = linear_to_srgb(clamp(O.rgb / a, 0, 1)) becomes e + v, and
   the output is (srgb_to_linear(clamp(e + v, 0, 1)) a, a): the layer keeps its colours, its
   covering and its size, and a pixel with a = 0 stays as it is.

`layer` D-189's word, ""; `fit` `tile`, `center` or `stretch`, `tile`; `light_direction` -3600
to 3600, 135; `texture_contrast` 0 to 2, 1. Every number keyable. Adobe's help gives no values;
the ones when added and the contrast's range are chosen here, as is the half in step 3.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawings' 8-bit values, where the build keeps the texture's luminance in single
precision.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/texturize/`: the
drawing `holder` at the top with the effect, and under it the texture layers, every one switched
off. The expected pixels are in `Fixtures/texturize/expected_texturize.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/texturize_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from vector_blur_reference import phase, along  # noqa: E402
from blobbylize_reference import DRAWINGS, decoded  # noqa: E402
from adjust_reference import srgb_to_linear  # noqa: E402
import smooth_reference as S  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "texturize"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
MAPS = ("dots", "ramp", "spot")


# --- the rule -------------------------------------------------------------------------------

def texturize(o, t, s):
    """O textured by T (both the drawing's size), or O itself without a texture."""
    w, h = o["w"], o["h"]
    if t is None or s["texture_contrast"] == 0:
        return o
    height = L.pic(w, h, [[0.0, 0.0, 0.0, phase("luminance", p)] for p in t["px"]])
    ux, uy = along(s["light_direction"])
    hold = lambda x, y: (min(max(x, 0.5), w - 0.5), min(max(y, 0.5), h - 0.5))  # noqa: E731
    out = []
    for y in range(h):
        for x in range(w):
            p = o["px"][y * w + x]
            a = p[3]
            if a <= 0:
                out.append(list(p))
                continue
            px, py = x + 0.5, y + 0.5
            ahead = L.bilinear(height, *hold(px + ux, py + uy))[3]
            behind = L.bilinear(height, *hold(px - ux, py - uy))[3]
            v = s["texture_contrast"] * (ahead - behind) / 2
            e = [S.linear_to_srgb(min(max(p[c] / a, 0.0), 1.0)) for c in range(3)]
            out.append([srgb_to_linear(min(max(c + v, 0.0), 1.0)) * a for c in e] + [a])
    return L.pic(w, h, out)


# --- the cases ------------------------------------------------------------------------------

NAMES = ("layer", "fit", "light_direction", "texture_contrast")


def case(holder="photo", shift=(0, 0), **kw):
    c = {"layer": "dots", "fit": "tile", "light_direction": 135, "texture_contrast": 1}
    c.update(kw)
    c["holder"], c["shift"] = holder, shift
    return c


def the_texture(c):
    return L.fit(decoded(c["layer"]), c["fit"], W, H) if c["layer"] in MAPS else None


RANGES = {"light_direction": (-3600, 3600), "texture_contrast": (0, 2)}


def settings(c, frame):
    """Each number held inside its range after it is worked out (document 20)."""
    s = {k: value_at(c[k], frame) for k in NAMES}
    for k, (lo, hi) in RANGES.items():
        s[k] = min(hi, max(lo, s[k]))
    return s


def render(c, frame):
    out = texturize(decoded(c["holder"]), the_texture(c), settings(c, frame))
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded(c["holder"])
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


RAMP = dict(layer="ramp", light_direction=90)
CASES = {
    "FX-TEXTURIZE-001": ("As added: no Texture Layer, so nothing happens and the photo comes "
                         "back untouched.", case(layer=""), (0,)),
    "FX-TEXTURIZE-002": ("The dots layer as the texture, Light Direction 135, Texture Contrast "
                         "1, Tile: the two white squares pressed into the photo, their top and "
                         "left edges lighter, their bottom and right edges darker, the rest as "
                         "it was.", case(), (0,)),
    "FX-TEXTURIZE-003": ("Light Direction -45: turned round, the top and left edges darker.",
                         case(light_direction=-45), (0,)),
    "FX-TEXTURIZE-004": ("Texture Contrast 0: nothing happens.", case(texture_contrast=0), (0,)),
    "FX-TEXTURIZE-005": ("Texture Contrast 2: twice as strong, the lightest edges clipped to "
                         "white.", case(texture_contrast=2), (0,)),
    "FX-TEXTURIZE-006": ("The ramp, black at the left to white at the right, as the texture, "
                         "Light Direction 90: one even slope, every column but the two at the "
                         "sides 17 levels of 255 lighter, those two 8.5.", case(**RAMP), (0,)),
    "FX-TEXTURIZE-007": ("The 8 by 5 spot as the texture, Tile: four red discs pressed in, two "
                         "across and two down.", case(layer="spot"), (0,)),
    "FX-TEXTURIZE-008": ("The spot, Center: one disc in the middle, the photo untouched round "
                         "it.", case(layer="spot", fit="center"), (0,)),
    "FX-TEXTURIZE-009": ("The spot, Stretch: one oval filling the middle.",
                         case(layer="spot", fit="stretch"), (0,)),
    "FX-TEXTURIZE-010": ("The dots on the two blocks with clear round them: the blocks pressed "
                         "where the squares cross them, every clear pixel still clear and the "
                         "covering kept exactly.", case(holder="shapes"), (0,)),
    "FX-TEXTURIZE-011": ("FX-TEXTURIZE-002 on the holder moved 2 right and 1 down: the same, "
                         "moved, the texture lying on the drawing.", case(shift=(2, 1)), (0,)),
    "FX-TEXTURIZE-012": ("A Texture Layer that is not in the composition, `gone`: nothing "
                         "happens, and the warning every frame.", case(layer="gone"), (0, 4)),
    "FX-TEXTURIZE-013": ("Light Direction keyed from 0 at frame 0 to 180 at frame 4, linear: "
                         "frame 2 is Light Direction 90.",
                         case(light_direction=keyed((0, 0), (4, 180))), (0, 2, 4)),
    "FX-TEXTURIZE-014": ("Texture Contrast eased from 1 at frame 0 to 2 at frame 4 on a curve "
                         "that overshoots: at frame 2 it would pass 2, is held at 2, and the "
                         "frame is FX-TEXTURIZE-005.",
                         case(texture_contrast=keyed((0, 1, OVERSHOOT), (4, 2))), (0, 2, 4)),
}

INVALID = {
    "FX-TEXTURIZE-015": ("Texture Contrast 2.5, above 2.", case(texture_contrast=2.5)),
    "FX-TEXTURIZE-016": ("Texture Contrast -0.5, below 0.", case(texture_contrast=-0.5)),
    "FX-TEXTURIZE-017": ("A placement written \"mirror\".", case(fit="mirror")),
    "FX-TEXTURIZE-018": ("A Texture Layer written as the number 3, not a word.", case(layer=3)),
    "FX-TEXTURIZE-019": ("Light Direction 4000, past 3600.", case(light_direction=4000)),
    "FX-TEXTURIZE-020": ("Texture Contrast keyed to 3 at frame 4.",
                         case(texture_contrast=keyed((0, 1), (4, 3)))),
}


def effect(c):
    return {"instance_id": "fx-1", "type_id": "core.texturize", "enabled": True,
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
    (OUT / "expected_texturize.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    photo = plain(case())
    shapes = plain(case(holder="shapes"))
    at = lambda x, y: y * W + x  # noqa: E731
    enc = lambda p: [S.linear_to_srgb(v / p[3]) for v in p[:3]]  # noqa: E731
    lum = lambda p: sum(enc(p))  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(u >= -1e-12 for u in p[:3]), (fx, p)
    for fx in ("001", "004", "012"):
        assert all(px == photo for px in c["FX-TEXTURIZE-" + fx].values()), fx
    two, three, five = (c[f"FX-TEXTURIZE-00{i}"]["0"] for i in (2, 3, 5))
    # The first square, columns 2 to 4 and rows 2 to 4: its top left corner lighter, its bottom
    # right darker, its middle and the far corner of the frame untouched.
    assert lum(two[at(2, 2)]) > lum(photo[at(2, 2)]) and lum(two[at(4, 4)]) < lum(photo[at(4, 4)])
    assert lum(three[at(2, 2)]) < lum(photo[at(2, 2)]) and lum(three[at(4, 4)]) > lum(photo[at(4, 4)])
    assert two[at(3, 3)] == photo[at(3, 3)] or near([two[at(3, 3)]], [photo[at(3, 3)]])
    assert near([two[at(15, 0)]], [photo[at(15, 0)]])
    # Twice the contrast: twice as far from the photo where nothing clips.
    for x, y in ((4, 4), (5, 5)):
        d2 = [a - b for a, b in zip(enc(two[at(x, y)]), enc(photo[at(x, y)]))]
        d5 = [a - b for a, b in zip(enc(five[at(x, y)]), enc(photo[at(x, y)]))]
        assert all(abs(q - 2 * p) < 1e-9 for p, q in zip(d2, d5) if 0 < enc(five[at(x, y)])[0] < 1)
    assert any(enc(p) == [1.0, 1.0, 1.0] or max(enc(p)) > 1 - 1e-12 for p in five)
    six = c["FX-TEXTURIZE-006"]["0"]
    for y in range(H):
        for x in range(W):
            want = 17 / 255 if 0 < x < W - 1 else 8.5 / 255
            # The cream's 240 clips at 255.
            got, was = enc(six[at(x, y)]), enc(photo[at(x, y)])
            assert all(abs(g - min(e + want, 1.0)) < 1e-9 for g, e in zip(got, was)), (x, y)
    seven, eight, nine = (c[f"FX-TEXTURIZE-00{i}"]["0"] for i in (7, 8, 9))
    assert seven != eight != nine != seven
    # Tile: the same relief every 8 across and 5 down, away from the frame's edges, in every
    # channel neither clipped.
    for y in range(1, 4):
        for x in range(1, 7):
            one, other = ([(g, e) for g, e in zip(enc(seven[at(i, j)]), enc(photo[at(i, j)]))]
                          for i, j in ((x, y), (x + 8, y + 5)))
            for (g, e), (h, f) in zip(one, other):
                if 1e-9 < g < 1 - 1e-9 and 1e-9 < h < 1 - 1e-9:
                    assert abs((g - e) - (h - f)) < 1e-9, (x, y)
    # Center: the spot's 8 by 5 lies at columns 4 to 11, rows 2 to 6; outside, untouched.
    for y in range(H):
        for x in range(W):
            if not (3 <= x <= 12 and 1 <= y <= 7):
                assert near([eight[at(x, y)]], [photo[at(x, y)]]), (x, y)
    ten = c["FX-TEXTURIZE-010"]["0"]
    assert all(p[3] == q[3] for p, q in zip(ten, shapes))
    assert all(p == q for p, q in zip(ten, shapes) if q[3] == 0) and ten != shapes
    eleven = c["FX-TEXTURIZE-011"]["0"]
    assert all(eleven[at(x, y)] == two[at(x - 2, y - 1)] for x in range(2, W) for y in range(1, H))
    thirteen = c["FX-TEXTURIZE-013"]
    assert thirteen["2"] == render(case(light_direction=90), 0)
    assert thirteen["0"] != thirteen["2"] != thirteen["4"]
    fourteen = c["FX-TEXTURIZE-014"]
    assert fourteen["2"] == five == fourteen["4"] and fourteen["0"] == two
    print("checked")


if __name__ == "__main__":
    main()
