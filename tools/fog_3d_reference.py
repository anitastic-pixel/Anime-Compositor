"""Fog 3D, worked a second way.

D-524 adds `core.fog_3d`, Fog 3D, after After Effects' Fog 3D (3D Channel), on EFFECTS.md row
379: a render with a depth pass is laid over with fog that thickens with distance, as though
the air between the camera and each pixel scattered light. The depth is read as D-348 reads it
(`tools/depth_channel_reference.py`): the layer's own EXR file, its Z channel or one of the other
depth names, never saved, said as `EFFECT_CHANNEL_MISSING` every frame when the layer has none,
and the effect then skipped.

Adobe's page names Fog Start Depth (where the scattering begins), Fog End Depth (where it is
greatest), Scattering Density (the higher, the denser the fog soon after its start), Foggy
Background (on: fog fills the back of the scene; off: the back stays clear for compositing),
Gradient Layer (a grey layer whose brightness sets the fog's density, at least the scene's size)
and Layer Contribution (how much the gradient counts). The effect's panel also has Fog Color and
Fog Opacity. Adobe gives no formula: the rule here is this program's own.

At composition frame n, with O the layer's picture (linear, premultiplied) whose drawing's own
top-left pixel is at (ox, oy), and z the depth at a pixel of the drawing:

1. t = clamp((z - start) / (end - start), 0, 1), or, with the two depths equal, 1 where
   z >= start and 0 nearer: how far through the fog the pixel lies.
2. With d = scattering_density / 10, the share of fog is f = t at d = 0, and otherwise
   f = (1 - e^(-d t)) / (1 - e^(-d)): 0 at the start, 1 at the end, and the higher the density,
   the sooner it thickens.
3. With no gradient layer named (""), g = 1. With one named, its map M is D-189's
   (`tools/effect_layer_reference.py`), centred on the drawing's own size; the drawing's pixel
   reads M under it, transparent outside M, and L is its picture luma (the sRGB encoding of
   clamp(0.2126 r + 0.7152 g + 0.0722 b, 0, 1), as Aerial Haze reads its matte); with
   c = layer_contribution / 100, g = 1 - c + c L. A layer named that is not in the composition
   leaves O, with D-189's warning.
4. With o = fog_opacity / 100: the fog on the pixel F = f g o, and the fog behind it
   B = g o with Foggy Background on, 0 off.
5. C is the fog colour, each channel's 8-bit value over 255 taken to linear by the sRGB curve.
   The output is (O.rgb + F (C O.a - O.rgb) + (1 - O.a) B C, O.a + (1 - O.a) B): the pixel moves
   toward the fog colour as Aerial Haze moves it, and what does not cover the pixel lets the fog
   behind show through. Outside the drawing nothing changes.

`fog_color` `#rrggbb`, #ffffff when added, read in small letters, not keyable; `fog_start_depth`
and `fog_end_depth` -1,000,000 to 1,000,000, 0 and 10 when added; `fog_opacity` 0 to 100, 100
when added; `scattering_density` 0 to 100, 50 when added; `foggy_background` "on" (when added) or
"off"; `gradient_layer` D-189's word, "" when added; `layer_contribution` 0 to 100, 100 when added.
The numbers are keyable. The depths are in the depth's own units; nothing is in pixels, so a
draft changes nothing but the pixels it draws.

Sources read: Adobe's 3D Channel effects page (helpx.adobe.com, last updated 14 April 2024), its
"About 3D Channel effects" and Fog 3D sections. The start values are this program's.

**This file never runs the build's code path.** It writes the EXR files with OpenEXR's own library
(3.4.15), reads them back with it, and works each expected pixel in double precision.

Every case is a project of one composition 8 by 6 at 24 frames a second, five frames long, in
`Fixtures/fog_3d/`, the depth fixtures' scene (colour by column and row, its top-left pixel half
covering and the next clear; depth 0.5 + 1.25 x + 0.75 y, 0.5 to 13), with a grey ramp layer
`grad` (black at the left to white at the right) switched off beneath it for the gradient. The
expected pixels are `Fixtures/fog_3d/expected_fog_3d.json`. The playtest uses the depth fixtures'
`Fixtures/depth_channel/sample/spheres.exr`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/fog_3d_reference.py   (with OpenEXR 3.4.15 and numpy)
"""

import json
import sys
from math import exp
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import depth_channel_reference as D  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import compound_blur_reference as K  # noqa: E402
import recolor_reference as R  # noqa: E402
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "fog_3d"
D.OUT = OUT  # D's file helpers (write, read, picture, write_project) work in this folder
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = D.W, D.H, D.FRAMES
NUMBERS = ("fog_start_depth", "fog_end_depth", "fog_opacity", "scattering_density",
           "layer_contribution")
WORDS = ("fog_color", "foggy_background", "gradient_layer")
START = {"fog_color": "#ffffff", "fog_start_depth": 0, "fog_end_depth": 10, "fog_opacity": 100,
         "scattering_density": 50, "foggy_background": "on", "gradient_layer": "",
         "layer_contribution": 100}
RANGES = {"fog_start_depth": (-1e6, 1e6), "fog_end_depth": (-1e6, 1e6), "fog_opacity": (0, 100),
          "scattering_density": (0, 100), "layer_contribution": (0, 100)}


# --- the files ------------------------------------------------------------------------------

def make_files():
    z = D.depth()
    D.write("media/scene.exr", dict(D.colour(), Z=z))
    far = z.copy()
    far[5, 7], far[5, 6], far[5, 5] = D.np.inf, -D.np.inf, D.np.nan
    D.write("media/far.exr", dict(D.colour(), Z=far))
    D.write("media/plain.exr", D.colour())
    blender = {f"ViewLayer.Combined.{k}": v for k, v in D.colour().items()}
    blender["ViewLayer.Depth.Z"] = z
    D.write("media/blender.exr", blender)
    L.DRAWINGS["card"] = [[(36 * x, 200, 30 * y, 255) for x in range(W)] for y in range(H)]
    (OUT / "media" / "card.png").write_bytes(png(L.DRAWINGS["card"]))
    L.DRAWINGS["grad"] = [[(round(255 * x / (W - 1)),) * 3 + (255,) for x in range(W)]
                          for _ in range(H)]
    (OUT / "media" / "grad.png").write_bytes(png(L.DRAWINGS["grad"]))


# --- the rule -------------------------------------------------------------------------------

def through(z, start, end):
    """Step 1."""
    if end == start:
        return 1.0 if z >= start else 0.0
    return min(1.0, max(0.0, (z - start) / (end - start)))


def share(t, density):
    """Step 2."""
    d = density / 10
    return t if d == 0 else (1 - exp(-d * t)) / (1 - exp(-d))


def settings(c, n):
    s = {k: min(RANGES[k][1], max(RANGES[k][0], value_at(c[k], n))) for k in NUMBERS}
    return {**s, **{k: c[k] for k in WORDS}}


def gradient(c):
    """Step 3's map, already on the drawing, or "even"; None for a layer not there."""
    if c["gradient_layer"] == "":
        return "even"
    if c["gradient_layer"] != "grad":
        return None
    return L.fit(L.decoded("grad"), "center", W, H)


def fog(p, z, m, s):
    """Steps 1 to 5 on the drawing p, with its depths z and the gradient map m."""
    col = [srgb_to_linear(v / 255) for v in R.hex_color(s["fog_color"].lower())]
    o = s["fog_opacity"] / 100
    c = s["layer_contribution"] / 100
    out = []
    for i, q in enumerate(p["px"]):
        g = 1.0 if m == "even" else 1 - c + c * K.luma(m["px"][i])
        f = share(through(z[i], s["fog_start_depth"], s["fog_end_depth"]), s["scattering_density"])
        big_f, big_b = f * g * o, (g * o if s["foggy_background"] == "on" else 0.0)
        a = q[3]
        out.append([q[k] + big_f * (col[k] * a - q[k]) + (1 - a) * big_b * col[k] for k in range(3)]
                   + [a + (1 - a) * big_b])
    return L.pic(W, H, out)


def effected(c, n):
    p = D.drawing(c)
    if c["before"] is not None:
        p = L.exposure(p, c["before"])
    m = gradient(c)
    if D.works(c) and m is not None:
        pv = D.passes(c["file"], "depth")
        p = fog(p, [pv[i][0] for i in range(W * H)], m, settings(c, n))
    if c["after"] is not None:
        p = L.exposure(p, c["after"])
    return p


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.fog_3d", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in NUMBERS + WORDS}}


ORIGINAL_PROJECT = D.project_json


def project_json(fx, c):
    """The depth fixtures' project with the ramp `grad` beneath the scene, switched off."""
    p = ORIGINAL_PROJECT(fx, c)
    comp = p["compositions"][0]
    comp["layers"].append(L.raster("grad", "asset-grad", enabled=False, out_frame=FRAMES))
    comp["layer_order"].append("grad")
    p["assets"].append(L.still("grad", "media/grad.png"))
    return p


D.effected = effected  # D.render and D.plain draw through these
D.effect = effect
D.project_json = project_json


def case(**kw):
    s = {k: kw.pop(k) for k in list(kw) if k in START}
    return D.case("fog", **kw, **{**START, **s})


# --- the cases ------------------------------------------------------------------------------

F0 = [0]
RAMP = dict(gradient_layer="grad")
CASES = {
    "FX-FOG-001": ("As added: white fog from depth 0 to 10, density 50, opacity 100, Foggy "
                   "Background on: the near top left lightly fogged, everything past depth 10 "
                   "white; the clear pixel filled with white fog, the half-covering one half.",
                   case(), [0, 4]),
    "FX-FOG-002": ("Density 0: the fog grows evenly from depth 0 to 10.",
                   case(scattering_density=0), F0),
    "FX-FOG-003": ("Density 100: thick almost at once.", case(scattering_density=100), F0),
    "FX-FOG-004": ("Start 4, End 12, density 0: nothing nearer than 4 fogged, the far corner "
                   "past 12 all fog.", case(fog_start_depth=4, fog_end_depth=12,
                                            scattering_density=0), F0),
    "FX-FOG-005": ("Start and End both 6: a cut, no fog nearer than 6, all fog from 6 on.",
                   case(fog_start_depth=6, fog_end_depth=6), F0),
    "FX-FOG-006": ("Opacity 50: half of FX-FOG-002's fog, the clear pixel half fog.",
                   case(scattering_density=0, fog_opacity=50), F0),
    "FX-FOG-007": ("Opacity 0: nothing changes.", case(fog_opacity=0), F0),
    "FX-FOG-008": ("Foggy Background off: the clear pixel stays clear and the half-covering one "
                   "keeps its covering.", case(foggy_background="off"), F0),
    "FX-FOG-009": ("Fog colour #4060a0, a dull blue, density 0.",
                   case(fog_color="#4060a0", scattering_density=0), F0),
    "FX-FOG-010": ("The ramp as the gradient, contribution 100, density 0: no fog in column 0 "
                   "(black), the full fog in column 7 (white).",
                   case(**RAMP, scattering_density=0), F0),
    "FX-FOG-011": ("The ramp, contribution 50: column 0 half fogged.",
                   case(**RAMP, scattering_density=0, layer_contribution=50), F0),
    "FX-FOG-012": ("The ramp, contribution 0: FX-FOG-002.",
                   case(**RAMP, scattering_density=0, layer_contribution=0), F0),
    "FX-FOG-013": ("A gradient layer named that is not in the composition: nothing changes, with "
                   "D-189's warning.", case(gradient_layer="gone"), F0),
    "FX-FOG-014": ("End keyed from 1 at frame 0 to 20 at frame 4: the fog draws back.",
                   case(fog_end_depth=keyed((0, 1), (4, 20))), [0, 2, 4]),
    "FX-FOG-015": ("Opacity keyed from 0 at frame 0 to 100 at frame 4.",
                   case(fog_opacity=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-FOG-016": ("Start 10, End 2: the other way round, fog thickest nearest.",
                   case(fog_start_depth=10, fog_end_depth=2, scattering_density=0), F0),
    "FX-FOG-017": ("Infinities and not-a-number in the depth: each read as D-348 reads it; the "
                   "warning every frame.", case(file="far.exr"), F0),
    "FX-FOG-018": ("Blender's layout, the depth in ViewLayer.Depth.Z: FX-FOG-001.",
                   case(file="blender.exr"), F0),
    "FX-FOG-019": ("A file with no depth: unchanged, the warning every frame.",
                   case(file="plain.exr"), F0),
    "FX-FOG-020": ("On a PNG drawing: unchanged, the warning.", case(layer="card"), F0),
    "FX-FOG-021": ("On an adjustment layer above the EXR: no file of its own, unchanged, the "
                   "warning.", case(on="adjust"), F0),
    "FX-FOG-022": ("The layer moved 2 right and 1 down, the ramp as the gradient: FX-FOG-010 "
                   "moved, the ramp with it.", case(**RAMP, scattering_density=0, move=(2, 1)),
                   F0),
    "FX-FOG-023": ("An Exposure of +1 before it.", case(before=1), F0),
    "FX-FOG-024": ("An Exposure of -1 after it.", case(after=-1), F0),
}

INVALID = {
    "FX-FOG-025": ("Fog Opacity 101, above 100.", case(fog_opacity=101)),
    "FX-FOG-026": ("Scattering Density -1, below 0.", case(scattering_density=-1)),
    "FX-FOG-027": ("Foggy Background written \"yes\".", case(foggy_background="yes")),
    "FX-FOG-028": ("Fog colour written \"white\".", case(fog_color="white")),
    "FX-FOG-029": ("Fog End Depth keyed to 2,000,000 at frame 4.",
                   case(fog_end_depth=keyed((0, 0), (4, 2e6)))),
    "FX-FOG-030": ("Layer Contribution 101, above 100.", case(layer_contribution=101)),
}


def warning_at_frame(c):
    if c["on"] == "adjust" or c["layer"] != "scene" or D.passes(c["file"], "depth") is None:
        return "EFFECT_CHANNEL_MISSING"
    if D.non_finite(c["file"], "depth"):
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
        if c["gradient_layer"] == "gone":
            entry["warning"] = "EFFECT_LAYER_MISSING"
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
    (OUT / "expected_fog_3d.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    scene = D.plain(case(), 0)
    near = lambda a, b, e=1e-9: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                    for p, q in zip(u, v))
    at = lambda px, x, y: px[y * W + x]  # noqa: E731
    z = lambda x, y: 0.5 + 1.25 * x + 0.75 * y  # noqa: E731
    assert share(0, 50) == 0 and abs(share(1, 50) - 1) < 1e-15 and share(0.5, 0) == 0.5
    assert share(0.2, 100) > share(0.2, 50) > share(0.2, 0)
    assert through(3, 6, 6) == 0 and through(6, 6, 6) == 1 and through(8, 10, 2) == 0.25
    one = c["FX-FOG-001"]["0"]
    for y in range(H):
        for x in range(W):
            if z(x, y) >= 10:
                assert near([at(one, x, y)], [[1.0, 1.0, 1.0, 1.0]]), (x, y)
    assert near([at(one, 1, 0)], [[1.0, 1.0, 1.0, 1.0]])  # the clear pixel, all fog behind
    assert c["FX-FOG-001"]["0"] == c["FX-FOG-001"]["4"]
    four = c["FX-FOG-004"]["0"]
    for y in range(H):
        for x in range(W):
            if z(x, y) <= 4 and (x, y) not in ((0, 0), (1, 0)):
                assert at(four, x, y) == at(scene, x, y), (x, y)
    for fx in ("FX-FOG-007", "FX-FOG-013", "FX-FOG-019", "FX-FOG-021"):
        assert c[fx]["0"] == scene, fx
    eight = c["FX-FOG-008"]["0"]
    assert at(eight, 1, 0) == [0.0, 0.0, 0.0, 0.0] and at(eight, 0, 0)[3] == at(scene, 0, 0)[3]
    two, six = c["FX-FOG-002"]["0"], c["FX-FOG-006"]["0"]
    for y in range(H):
        for x in range(2, W):
            for k in range(3):
                assert abs((at(two, x, y)[k] + at(scene, x, y)[k]) / 2 - at(six, x, y)[k]) < 1e-9
    ten = c["FX-FOG-010"]["0"]
    for y in range(1, H):
        assert at(ten, 0, y) == at(scene, 0, y) and near([at(ten, W - 1, y)], [at(two, W - 1, y)])
    assert near(c["FX-FOG-012"]["0"], two)
    assert c["FX-FOG-011"]["0"] not in (ten, two)
    assert near(c["FX-FOG-014"]["4"], D.render(case(fog_end_depth=20), 0))
    assert c["FX-FOG-015"]["0"] == scene and near(c["FX-FOG-015"]["4"], one)
    assert near(c["FX-FOG-018"]["0"], one)
    assert expected["cases"]["FX-FOG-017"]["frame_warning"] == "MEDIA_EXR_ADJUSTED"
    assert all(c["FX-FOG-022"]["0"][(y + 1) * W + x + 2] == ten[y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    sixteen = c["FX-FOG-016"]["0"]
    assert at(sixteen, 7, 5) == at(scene, 7, 5) and at(sixteen, 0, 1) != at(scene, 0, 1)
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(v >= -1e-12 for v in p[:3]), (name, p)
    print("checks passed")


if __name__ == "__main__":
    main()
