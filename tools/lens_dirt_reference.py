"""Lens Dirt, worked a second way.

D-545 is pick #14 in `docs/effects/EFFECTS.md` (row 261), "Lens Dirt": a dirt texture from
another layer, lit where the glow or flare is bright. PLUGINS.md settles it as "a 'dirt texture
from a layer' setting on Soft Physical Glow and Lens Flare", with Deep Glow 2's Lens Dirt
Texture controls: Texture, Opacity, Inherit Glow Color, Matte Exposure, Matte Gamma and Repeat
Edges. The plugin publishes what each does in a sentence and no formula; the rule below is our
own reading of those sentences. No plugin code or presets are used.

Six new settings on Soft Physical Glow (`core.glow` with `falloff` "physical", D-353) and on
Lens Flare (`core.lens_flare`, D-426), the same on both:

* `dirt_layer`, D-189's layer setting, "" (none, as added): the dirt picture.
* `dirt_opacity`, 0 to 100 per cent, default 100: how strongly the dirt is laid on.
* `dirt_inherit`, 0 to 100 per cent, default 0: how far the dirt takes the light's colour.
* `dirt_exposure`, -10 to 10 stops, default 0: the matte's exposure.
* `dirt_gamma`, 0.1 to 10, default 1: the matte's gamma.
* `dirt_repeat`, "on" (as added) or "off": whether a dirt picture smaller than the layer
  mirror-repeats.

The four numbers are keyable, held inside their ranges at each frame. A file without them reads
"", 100, 0, 0, 1, "on", and they are written only when changed or in the file already, so every
earlier file draws the same numbers.

The rule. G is the light the effect adds, each channel, linear: Soft Physical Glow's glow (the
levels' sum, tinted when Tint is on) times Exposure, before Screen holds it at 1; Lens Flare's
light L (its parts' sum times Brightness).

1. The dirt picture D: the named layer's picture as D-189 makes it (steps 1 to 3), whole, not
   fitted to the holder (the fit word `whole`), w by h, linear premultiplied. It is centred on
   the layer: the layer's pixel (x, y), the layer W by H as the effect receives it, reads D at
   (x - floor((W - w) / 2), y - floor((H - h) / 2)). With Repeat Edges on, a place outside D is
   mirrored back into it (period 2w across: ..., 1, 0 | 0, 1, ..., w - 1 | w - 1, w - 2, ...),
   so a small picture covers the whole layer and the glow's grown ring; off, D is clear
   outside itself.
2. The matte: m_c = (1 - i) Y(G) + i G_c, i = `dirt_inherit` / 100 and Y the Rec. 709
   brightness 0.2126 G_r + 0.7152 G_g + 0.0722 G_b. At inherit 0 the matte is grey, so the dirt
   keeps its own colour; at 100 it is the light channel by channel, so the dirt takes the
   light's colour, tint included.
3. Exposure and gamma act on the matte only: m'_c = clamp(m_c 2^E, 0, 1)^(1 / gamma), E =
   `dirt_exposure`. The matte is held at 1, so the dirt is never brighter than itself.
4. The matted dirt goes on top of the light: G'_c = G_c + o m'_c D_c, o = `dirt_opacity` / 100
   (D premultiplied, so its covering is already in it).
5. G' takes G's place: Soft Physical Glow's finish from there (Screen's hold at 1, Unmult's
   covering, the untouched layer at Source Opacity); Lens Flare's lay-on (the light added,
   the covering lifted by its brightest channel, Blend With Original).

With no layer named, or Opacity 0, G' is G, the effect's earlier rule exactly. A layer named
that is not in the composition leaves the layer drawn without the effect, with D-189's
EFFECT_LAYER_MISSING. A layer named whose picture is empty (outside its in and out) adds no dirt.

**This file never runs the build's code path.** It works in double precision with numpy (the
glow on D-353's padded planes of cells, `tools/spectral_glow_reference.py`'s glow sum; the flare
by `tools/lens_flare_reference.py`'s parts), where the build works in single precision.

Every case is a composition 16 pixels by 10. The glow cases hold `tools/soft_glow_reference.py`'s
drawings, the flare cases `tools/lens_flare_reference.py`'s photo; each project also holds the
dirt drawings as layers switched off (D-189 reads them all the same). The drawings go into
`Fixtures/lens_dirt/media`, the projects into `Fixtures/lens_dirt`, and the expected frames into
`Fixtures/lens_dirt/expected_lens_dirt.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/lens_dirt_reference.py
"""

import copy
import json
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import soft_glow_reference as G  # noqa: E402
import glow_tint_reference as T  # noqa: E402
import spectral_glow_reference as SP  # noqa: E402
import lens_flare_reference as F  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import blobbylize_reference as B  # noqa: E402

W, H = G.W, G.H
assert (F.W, F.H) == (W, H)
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "lens_dirt"
TOLERANCE = 2e-5  # document 25's default for a filter
DIRT_RANGES = {"dirt_opacity": (0, 100), "dirt_inherit": (0, 100), "dirt_exposure": (-10, 10),
               "dirt_gamma": (0.1, 10)}
DIRT = ("dirt_layer", "dirt_opacity", "dirt_inherit", "dirt_exposure", "dirt_gamma",
        "dirt_repeat")
GLOW_PARAMS = G.PARAMS + ("tint", "inner_color", "outer_color", "tint_amount")


# --- the dirt drawings ----------------------------------------------------------------------

def smudge(x, y):
    """6 by 4: warm to cool smudges, one pixel half covering and one clear."""
    if (x, y) == (4, 1):
        return (0, 0, 0, 0)
    return (200 - 20 * x + 5 * y, 120 + 10 * x + 15 * y, 60 + 25 * x, 128 if (x, y) == (1, 2) else 255)


def specks(x, y):
    """20 by 12, bigger than the layer: specks on a faint grey, the edge columns red."""
    if x in (0, 19):
        return (255, 0, 0, 255)
    if (x * 7 + y * 3) % 5 == 0:
        return (240, 230, 210, 255)
    return (60, 60, 60, 200)


DIRT_DRAWINGS = {
    "smudge": [[smudge(x, y) for x in range(6)] for y in range(4)],
    "specks": [[specks(x, y) for x in range(20)] for y in range(12)],
}


def dirt_pixels(name):
    return np.array([[G.working(p) for p in row] for row in DIRT_DRAWINGS[name]])


# --- the rule -------------------------------------------------------------------------------

def mirror(s, n):
    m = s % (2 * n)
    return m if m < n else 2 * n - 1 - m


def dirt_at(d, x, y, repeat):
    """Step 1: the dirt picture d (h x w x 4) at the layer's pixel (x, y), centred on it."""
    h, w = d.shape[:2]
    sx, sy = x - (W - w) // 2, y - (H - h) // 2
    if repeat == "on":
        sx, sy = mirror(sx, w), mirror(sy, h)
    if 0 <= sx < w and 0 <= sy < h:
        return d[sy, sx, :3]
    return np.zeros(3)


def dirtied(g, d, s):
    """Steps 2 to 4: the light g (3 channels) with the matted dirt d added."""
    i = s["dirt_inherit"] / 100
    y = 0.2126 * g[0] + 0.7152 * g[1] + 0.0722 * g[2]
    m = np.clip(((1 - i) * y + i * g) * 2.0 ** s["dirt_exposure"], 0.0, 1.0) ** (1 / s["dirt_gamma"])
    return g + s["dirt_opacity"] / 100 * m * d


def glow_layer(px, c, xs, ys, dirt):
    """Soft Physical Glow (D-353 with D-542's tint) with the dirt in G's place."""
    g = SP.glow_sum(px, c, xs, ys, c["radius"]) * c["exposure"]
    if dirt is not None:
        for jy, y in enumerate(ys):
            for ix, x in enumerate(xs):
                g[jy, ix] = dirtied(g[jy, ix], dirt_at(dirt, x, y, c["dirt_repeat"]), c)
    screen = c["operation"] == "screen"
    if screen:
        g = np.minimum(g, 1.0)
    ga = np.minimum(1.0, g.max(axis=-1)) if c["unmult"] == "on" else np.ones(g.shape[:2])
    o = c["source_opacity"] / 100
    out = np.zeros((len(ys), len(xs), 4))
    for jy, y in enumerate(ys):
        for ix, x in enumerate(xs):
            s = px[y, x] if (0 <= x < W and 0 <= y < H) else np.zeros(4)
            rgb = g[jy, ix] + o * s[:3]
            if screen:
                rgb = np.minimum(rgb, 1.0)
            out[jy, ix, :3] = rgb
            out[jy, ix, 3] = min(1.0, ga[jy, ix] + o * s[3])
    return out


def flare_layer(o, s, dirt):
    """Lens Flare (D-426) with the dirt in L's place."""
    w, h = o["w"], o["h"]
    keep = 1 - s["blend_with_original"] / 100
    out = []
    for y in range(h):
        for x in range(w):
            p = o["px"][y * w + x]
            lt = np.array(F.light(w, h, s, x, y))
            if dirt is not None:
                lt = dirtied(lt, dirt_at(dirt, x, y, s["dirt_repeat"]), s)
            f = [p[k] + lt[k] for k in range(3)] + [p[3] + (1 - p[3]) * min(1.0, max(lt))]
            out.append([float(p[k] + keep * (f[k] - p[k])) for k in range(4)])
    return L.pic(w, h, out)


def held(c, frame_no):
    out = dict(c)
    for k, (lo, hi) in DIRT_RANGES.items():
        out[k] = min(hi, max(lo, float(value_at(c[k], frame_no))))
    return out


def the_dirt(c):
    """None for no layer or one not in the composition (the latter is skipped before this)."""
    return dirt_pixels(c["dirt_layer"]) if c["dirt_layer"] in DIRT_DRAWINGS else None


def render(c, frame_no):
    if c["dirt_layer"] not in ("",) + tuple(DIRT_DRAWINGS):
        return plain(c)  # D-189: a layer not in the composition, drawn without the effect
    if c["effect"] == "glow":
        s = G.settings_at(held(c, frame_no), frame_no)
        out = glow_layer(G.pixels(c["drawing"]), s, [x - c["shift"] for x in range(W)],
                         list(range(H)), the_dirt(c))
        return [[float(v) for v in out[y, x]] for y in range(H) for x in range(W)]
    s = held(c, frame_no)
    s.update(F.settings(c, frame_no))
    out = flare_layer(B.decoded(c["holder"]), s, the_dirt(c))
    return [list(L.at(out, x, y)) for y in range(H) for x in range(W)]


def plain(c):
    if c["effect"] == "glow":
        return G.plain(c["drawing"], c["shift"])
    o = B.decoded(c["holder"])
    return [list(L.at(o, x, y)) for y in range(H) for x in range(W)]


# --- the cases ------------------------------------------------------------------------------

def dirt_settings(dirt_layer="smudge", dirt_opacity=100, dirt_inherit=0, dirt_exposure=0,
                  dirt_gamma=1, dirt_repeat="on"):
    return {"dirt_layer": dirt_layer, "dirt_opacity": dirt_opacity, "dirt_inherit": dirt_inherit,
            "dirt_exposure": dirt_exposure, "dirt_gamma": dirt_gamma, "dirt_repeat": dirt_repeat}


def glow(name="patches", radius=12, exposure=1, operation="screen", tint="off", shift=0, **dirt):
    c = G.case(name, radius=radius, exposure=exposure, operation=operation, shift=shift)
    c.update({"tint": tint, "inner_color": "#ffffff", "outer_color": "#4080ff",
              "tint_amount": 100, "spectral": "off"})
    c.update(dirt_settings(**dirt))
    c["effect"] = "glow"
    return c


def flare(flare_brightness=100, blend_with_original=0, **dirt):
    c = F.case(holder="photo", flare_brightness=flare_brightness,
               blend_with_original=blend_with_original)
    c.update(dirt_settings(**dirt))
    c["effect"] = "flare"
    return c


CASES = {
    "FX-DIRT-001": ("Soft Physical Glow, radius 12, with the 6 by 4 smudge as its dirt, every "
                    "dirt setting as added (opacity 100, inherit 0, exposure 0, gamma 1, repeat "
                    "on): the smudge, mirror-repeated over the whole grown layer, shows where "
                    "the glow is bright, in its own colours, brightest beside the yellow patch.",
                    glow(), [0]),
    "FX-DIRT-002": ("Repeat Edges off: the smudge only in the middle, columns 5 to 10 and rows "
                    "3 to 6; elsewhere FX-SGLOW-001's glow.",
                    glow(dirt_repeat="off"), [0]),
    "FX-DIRT-003": ("Inherit Glow Color 100: the matte is the glow channel by channel, so the "
                    "dirt takes the glow's warm colour.",
                    glow(dirt_inherit=100), [0]),
    "FX-DIRT-004": ("Inherit Glow Color 50: halfway between FX-DIRT-001's matte and "
                    "FX-DIRT-003's.",
                    glow(dirt_inherit=50), [0]),
    "FX-DIRT-005": ("Matte Exposure 2: the matte four times as bright, held at 1, so the dirt "
                    "shows further out from the patches.",
                    glow(dirt_exposure=2), [0]),
    "FX-DIRT-006": ("Matte Exposure -2: the matte a quarter as bright, the dirt fainter.",
                    glow(dirt_exposure=-2), [0]),
    "FX-DIRT-007": ("Matte Gamma 2.2: the matte's faint parts raised, the dirt reaching further "
                    "into the glow's tail.",
                    glow(dirt_gamma=2.2), [0]),
    "FX-DIRT-008": ("Matte Gamma 0.5: the matte squared, the dirt only where the glow is "
                    "strongest.",
                    glow(dirt_gamma=0.5), [0]),
    "FX-DIRT-009": ("Opacity 0: no dirt, FX-SGLOW-001's glow exactly.",
                    glow(dirt_opacity=0), [0]),
    "FX-DIRT-010": ("Opacity 50: half FX-DIRT-001's dirt.",
                    glow(dirt_opacity=50), [0]),
    "FX-DIRT-011": ("Opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the "
                    "glow alone, frame 2 FX-DIRT-010, frame 4 FX-DIRT-001.",
                    glow(dirt_opacity=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-DIRT-012": ("The 20 by 12 specks, bigger than the layer: centred, its two red edge "
                    "columns fall in the grown ring outside the drawing, past the comp's view; "
                    "the specks show where the glow is bright.",
                    glow(dirt_layer="specks"), [0]),
    "FX-DIRT-013": ("Moved three pixels right: the glow and its dirt move together, the dirt "
                    "lying on the layer.",
                    glow(shift=3), [0]),
    "FX-DIRT-014": ("Tint core to edge (white to blue) with Inherit 100: the dirt takes the "
                    "tinted glow's colour, bluer away from the patches.",
                    glow(tint="core_to_edge", dirt_inherit=100), [0]),
    "FX-DIRT-015": ("Blend Mode Add, Exposure 3: the glow not held at 1, the matte held at 1, "
                    "so the dirt is never brighter than itself.",
                    glow(operation="add", exposure=3), [0]),
    "FX-DIRT-016": ("A dirt layer that is not in the composition, `gone`: the layer drawn "
                    "without the effect, and the warning every frame.",
                    glow(dirt_layer="gone"), [0, 4]),
    "FX-DIRT-017": ("Lens Flare as added on the photo with the smudge as its dirt: the smudge "
                    "lit where the flare is bright, round its core at (4.8, 3) and on its "
                    "ghosts.",
                    flare(), [0]),
    "FX-DIRT-018": ("Lens Flare, Inherit Glow Color 100: the dirt takes the flare's colours.",
                    flare(dirt_inherit=100), [0]),
    "FX-DIRT-019": ("Lens Flare, Blend With Original 50: halfway between FX-DIRT-017 and the "
                    "photo.",
                    flare(blend_with_original=50), [0]),
    "FX-DIRT-020": ("Lens Flare, Brightness 0: no light, so no matte and no dirt: the photo "
                    "untouched.",
                    flare(flare_brightness=0), [0]),
    "FX-DIRT-021": ("Lens Flare, Repeat Edges off, Matte Exposure 3: the dirt only in the "
                    "middle, strong.",
                    flare(dirt_repeat="off", dirt_exposure=3), [0]),
    "FX-DIRT-022": ("Lens Flare, no dirt layer named: FX-FLARE-001 exactly.",
                    flare(dirt_layer=""), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-DIRT-023": ("Soft Physical Glow, dirt opacity 101, above 100.", glow(dirt_opacity=101)),
    "FX-DIRT-024": ("Soft Physical Glow, dirt inherit -1, below 0.", glow(dirt_inherit=-1)),
    "FX-DIRT-025": ("Soft Physical Glow, dirt exposure 11, above 10.", glow(dirt_exposure=11)),
    "FX-DIRT-026": ("Soft Physical Glow, dirt gamma 0.05, below 0.1.", glow(dirt_gamma=0.05)),
    "FX-DIRT-027": ("Soft Physical Glow, dirt repeat written \"yes\".", glow(dirt_repeat="yes")),
    "FX-DIRT-028": ("Lens Flare, dirt opacity -1, below 0.", flare(dirt_opacity=-1)),
    "FX-DIRT-029": ("Lens Flare, dirt gamma keyed to 11 at frame 4, above 10.",
                    flare(dirt_gamma=keyed((0, 1), (4, 11)))),
}


# --- the project files ----------------------------------------------------------------------

def dirt_json(c):
    return {k: c[k] if k in ("dirt_layer", "dirt_repeat") else setting_json(c[k]) for k in DIRT}


def dirt_layers(template, frames):
    out = []
    for name in DIRT_DRAWINGS:
        if template is None:
            out.append(L.raster(name, "asset-" + name, out_frame=frames, enabled=False))
        else:
            layer = copy.deepcopy(template)
            layer.update({"id": name, "name": name, "asset_id": "asset-" + name,
                          "enabled": False, "effects": []})
            t = layer["transform"]
            t["anchor"]["base"] = [len(DIRT_DRAWINGS[name][0]) / 2, len(DIRT_DRAWINGS[name]) / 2]
            t["position"]["base"] = [W / 2, H / 2]
            out.append(layer)
    return out


def project_json(fx, c):
    if c["effect"] == "glow":
        p = G.project_json(fx, c)
        comp = p["compositions"][0]
        params = comp["layers"][0]["effects"][0]["parameters"]
        for k in ("tint", "inner_color", "outer_color"):
            if c[k] != {"tint": "off", "inner_color": "#ffffff", "outer_color": "#4080ff"}[k]:
                params[k] = c[k]
        params.update(dirt_json(c))
        extra = dirt_layers(comp["layers"][0], comp["duration_frames"])
        comp["layers"] += extra
        comp["layer_order"] += [layer["id"] for layer in extra]
    else:
        p = F.project_json(fx, c)
        p["assets"] = [a for a in p["assets"] if a["id"] == "asset-photo"]
        comp = p["compositions"][0]
        comp["layers"][0]["effects"][0]["parameters"].update(dirt_json(c))
        extra = dirt_layers(None, F.FRAMES)
        comp["layers"] += extra
        comp["layer_order"] += [layer["id"] for layer in extra]
    p["assets"] += [L.still(n, f"media/{n}.png") for n in DIRT_DRAWINGS]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name in ("patches",):
        (OUT / "media" / f"{name}.png").write_bytes(S.png(G.DRAWINGS[name]))
    (OUT / "media" / "photo.png").write_bytes(F.png(B.DRAWINGS["photo"]))
    for name, px in DIRT_DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(px))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write(fx, c), "frames": rendered}
        if c["dirt_layer"] == "gone":
            entry["warning"] = "EFFECT_LAYER_MISSING"
        expected["cases"][fx] = entry
        base = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(max(abs(a - b) for a, b in zip(px[i], base[i])) > 1e-9 for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        base = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": base, "4": base},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_lens_dirt.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(a - b) <= e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g, e=1e-12: all(close(p, q, e) for p, q in zip(f, g))  # noqa: E731

    # The mirror: period 2n, the edge pixel repeated.
    assert [mirror(s, 3) for s in range(-4, 8)] == [2, 2, 1, 0, 0, 1, 2, 2, 1, 0, 0, 1]

    bare = render(glow(dirt_layer=""), 0)
    one = c["FX-DIRT-001"]["0"]
    # The dirt only adds; with Screen nothing passes 1.
    assert all(p[k] >= q[k] - 1e-15 for p, q in zip(one, bare) for k in range(4))
    assert any(p[0] > q[0] + 1e-3 for p, q in zip(one, bare))
    # 009 is the glow without dirt; 022 is FX-FLARE-001.
    assert same(c["FX-DIRT-009"]["0"], bare, 0)
    flare_one = json.loads((OUT.parent / "lens_flare" / "expected_lens_flare.json")
                           .read_text(encoding="utf-8"))["cases"]["FX-FLARE-001"]["frames"]["0"]
    assert same(c["FX-DIRT-022"]["0"], flare_one, 1e-12)
    # And the glow without dirt is FX-SGLOW-001.
    sglow = json.loads((OUT.parent / "soft_glow" / "expected_soft_glow.json")
                       .read_text(encoding="utf-8"))["cases"]["FX-SGLOW-001"]["frames"]["0"]
    assert same(bare, sglow, 1e-12)
    # 002: outside the middle 6 by 4 the glow alone; inside, FX-DIRT-001.
    two = c["FX-DIRT-002"]["0"]
    for y in range(H):
        for x in range(W):
            middle = 5 <= x <= 10 and 3 <= y <= 6
            assert close(two[at(x, y)], (one if middle else bare)[at(x, y)], 1e-12), (x, y)
    # 001 repeats: column 4 reads the smudge's column 0 mirrored (sx = -1 -> 0), as column 5.
    # 010: half 001's dirt where nothing is held (off the patches, below 1).
    ten = c["FX-DIRT-010"]["0"]
    for i in range(W * H):
        if max(one[i][:3]) < 0.999:
            for k in range(3):
                assert abs((ten[i][k] - bare[i][k]) - (one[i][k] - bare[i][k]) / 2) < 1e-12
    k = c["FX-DIRT-011"]
    assert same(k["0"], bare, 1e-12) and same(k["2"], ten, 1e-12) and same(k["4"], one, 1e-12)
    # 005 brighter dirt than 001, 006 fainter; 007 brighter, 008 fainter.
    gain = lambda f: sum(p[0] - q[0] for p, q in zip(f, bare))  # noqa: E731
    assert gain(c["FX-DIRT-005"]["0"]) > gain(one) > gain(c["FX-DIRT-006"]["0"]) > 0
    assert gain(c["FX-DIRT-007"]["0"]) > gain(one) > gain(c["FX-DIRT-008"]["0"]) > 0
    # 003 differs from 001 and 004 lies between them.
    three, four = c["FX-DIRT-003"]["0"], c["FX-DIRT-004"]["0"]
    assert not same(three, one, 1e-6) and not same(four, one, 1e-6) and not same(four, three, 1e-6)
    # 013: the same picture moved three right.
    for y in range(H):
        for x in range(3, W):
            assert close(c["FX-DIRT-013"]["0"][at(x, y)], one[at(x - 3, y)], 1e-12)
    # 015: the matte is held, so the dirt adds at most the dirt itself.
    sm = dirt_pixels("smudge")
    add3 = render(glow(operation="add", exposure=3, dirt_layer=""), 0)
    for y in range(H):
        for x in range(W):
            d = dirt_at(sm, x, y, "on")
            for k in range(3):
                assert c["FX-DIRT-015"]["0"][at(x, y)][k] - add3[at(x, y)][k] <= d[k] + 1e-12
    # 016: drawn without the effect.
    assert same(c["FX-DIRT-016"]["0"], G.plain("patches"), 0)
    # Flare: 020 is the photo; 019 halfway; 017 lit beyond FX-FLARE-001.
    photo = plain(flare())
    assert same(c["FX-DIRT-020"]["0"], photo, 0)
    seventeen = c["FX-DIRT-017"]["0"]
    assert all(abs(c["FX-DIRT-019"]["0"][i][k] - (seventeen[i][k] + photo[i][k]) / 2) < 1e-12
               for i in range(W * H) for k in range(4))
    assert gain(seventeen) > 0 and not same(seventeen, flare_one, 1e-6)
    assert all(p[k] >= q[k] - 1e-15 for p, q in zip(seventeen, flare_one) for k in range(4))
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(v >= -1e-12 for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
