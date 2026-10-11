"""Depth of Field, worked a second way.

D-523 adds `core.depth_of_field`, Depth of Field, after After Effects' Depth of Field (3D
Channel), on EFFECTS.md row 377: a render with a depth pass is blurred more the further each
pixel lies from a chosen distance, as a camera focused there would blur it. The depth is read as
D-348 reads it (`tools/depth_channel_reference.py`): the layer's own EXR file, its Z channel or
one of the other depth names, never saved, said as `EFFECT_CHANNEL_MISSING` every frame when the
layer has none, and the effect then skipped.

Adobe's page names four settings and gives no formula: Focal Plane (the distance in focus),
Maximum Radius (the blur of what is furthest out of focus), Focal Plane Thickness (the depths
around the plane also in focus) and Focal Bias (the higher, the faster things leave focus). The
rule here is this program's own.

At composition frame n, with z the depth at a pixel of the layer's drawing:

1. beyond = max(0, |z - focal_plane| - focal_plane_thickness / 2): how far the pixel is outside
   the slab in focus, half the thickness on each side of the plane.
2. With k = focal_bias / 50, the fraction out of focus is f = 0 where beyond is 0 or k is 0, 1
   where k beyond >= |z|, and k beyond / |z| otherwise. At bias 50 this is a thin lens's blur
   circle, |z - focus| / z of its largest: what is twice as far as the plane is half blurred, what
   is very far nearly fully, what is nearer than half the plane's distance fully. It is the same
   for a depth in any unit, and for a depth written negative (some programs write the distance
   in front of the camera as -z).
3. The blur is D-359's Lens Blur with a blur map (`tools/lens_blur_map_reference.py`): radius
   `maximum_radius`, a circle iris (roundness 0, rotation 0, aspect 1), no highlights, edges
   repeated (the layer does not grow), the map the fractions f read by their alpha with focal
   distance 0. So f = 0 is the drawing's pixel, f = 1 the plain mean over the full radius's
   circle, and between the two nearest levels mixed.

`focal_plane` -1,000,000 to 1,000,000, 0 when added; `maximum_radius` 0 to 200 pixels, 10 when
added; `focal_plane_thickness` 0 to 1,000,000, 0 when added; `focal_bias` 0 to 100, 50 when added;
all keyable. The distances are in the depth's own units; only the radius is in pixels, and a
draft scales it as it scales Lens Blur's.

Sources read: Adobe's 3D Channel effects page (helpx.adobe.com, last updated 14 April 2024), its
Depth Of Field and 3D Channel Extract sections. The start values are this program's.

**This file never runs the build's code path.** It writes the EXR files with OpenEXR's own library
(3.4.15), reads them back with it, and works each expected pixel in double precision, the blur by
`tools/lens_blur_map_reference.py`'s direct sums.

Every case is a project of one composition 8 by 6 at 24 frames a second, five frames long, in
`Fixtures/depth_of_field/`, the scene the depth fixtures' (colour by column and row, depth
0.5 + 1.25 x + 0.75 y, 0.5 to 13). The expected pixels are
`Fixtures/depth_of_field/expected_depth_of_field.json`. The playtest uses the depth fixtures'
`Fixtures/depth_channel/sample/spheres.exr`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/depth_of_field_reference.py   (with OpenEXR 3.4.15 and numpy)
"""

import json
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
import depth_channel_reference as D  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import lens_blur_map_reference as M  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "depth_of_field"
D.OUT = OUT  # D's file helpers (write, read, picture, write_project) work in this folder
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = D.W, D.H, D.FRAMES
SETTINGS = ("focal_plane", "maximum_radius", "focal_plane_thickness", "focal_bias")
START = {"focal_plane": 0, "maximum_radius": 10, "focal_plane_thickness": 0, "focal_bias": 50}
RANGES = {"focal_plane": (-1e6, 1e6), "maximum_radius": (0, 200),
          "focal_plane_thickness": (0, 1e6), "focal_bias": (0, 100)}


# --- the files ------------------------------------------------------------------------------

def make_files():
    z = D.depth()
    D.write("media/scene.exr", dict(D.colour(), Z=z))
    D.write("media/negative.exr", dict(D.colour(), Z=-z))
    far = z.copy()
    far[5, 7], far[5, 6], far[5, 5] = np.inf, -np.inf, np.nan
    D.write("media/far.exr", dict(D.colour(), Z=far))
    D.write("media/plain.exr", D.colour())
    blender = {f"ViewLayer.Combined.{k}": v for k, v in D.colour().items()}
    blender["ViewLayer.Depth.Z"] = z
    D.write("media/blender.exr", blender)
    L.DRAWINGS["card"] = [[(36 * x, 200, 30 * y, 255) for x in range(W)] for y in range(H)]
    (OUT / "media" / "card.png").write_bytes(png(L.DRAWINGS["card"]))


# --- the rule -------------------------------------------------------------------------------

def fraction(z, focal, thickness, bias):
    """Steps 1 and 2."""
    beyond = max(0.0, abs(z - focal) - thickness / 2)
    k = bias / 50
    if beyond == 0 or k == 0:
        return 0.0
    if k * beyond >= abs(z):
        return 1.0
    return k * beyond / abs(z)


def settings(c, n):
    return {k: min(RANGES[k][1], max(RANGES[k][0], value_at(c[k], n))) for k in SETTINGS}


def effected(c, n):
    p = D.drawing(c)
    if c["before"] is not None:
        p = L.exposure(p, c["before"])
    if D.works(c):
        s = settings(c, n)
        pv = D.passes(c["file"], "depth")
        f = [fraction(pv[i][0], s["focal_plane"], s["focal_plane_thickness"], s["focal_bias"])
             for i in range(W * H)]
        m = L.pic(W, H, [[v] * 4 for v in f])
        blur = {**M.IRIS_START, "radius": s["maximum_radius"], "edges": "repeat",
                "channel": "alpha", "focal_distance": 0, "invert": "off"}
        out = M.mapped(p, m, blur)
        p = L.pic(W, H, [list(out[(x, y)]) for y in range(H) for x in range(W)])
    if c["after"] is not None:
        p = L.exposure(p, c["after"])
    return p


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.depth_of_field", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in SETTINGS}}


D.effected = effected  # D.render and D.plain draw through these
D.effect = effect


def case(**kw):
    s = {k: kw.pop(k) for k in list(kw) if k in SETTINGS}
    return D.case("dof", **kw, **{**START, **s})


# --- the cases ------------------------------------------------------------------------------

AT6 = dict(focal_plane=6, maximum_radius=3)
F0 = [0]
CASES = {
    "FX-DOF-001": ("As added: Focal Plane 0, so every depth is out of focus by its whole "
                   "distance, fully; the whole picture at the full radius 10, edges repeated.",
                   case(), [0, 4]),
    "FX-DOF-002": ("Focal Plane 6, radius 3: the diagonal at depth 6 sharp, the near top left "
                   "fully blurred, the far bottom right about half.", case(**AT6), F0),
    "FX-DOF-003": ("Focal Plane 6, Thickness 4: everything from 4 to 8 sharp.",
                   case(**AT6, focal_plane_thickness=4), F0),
    "FX-DOF-004": ("Focal Plane 6, Bias 100: out of focus twice as fast.",
                   case(**AT6, focal_bias=100), F0),
    "FX-DOF-005": ("Focal Plane 6, Bias 25: half as fast.", case(**AT6, focal_bias=25), F0),
    "FX-DOF-006": ("Bias 0: nothing leaves focus, the picture unchanged.",
                   case(**AT6, focal_bias=0), F0),
    "FX-DOF-007": ("Maximum Radius 0: the picture unchanged.",
                   case(focal_plane=6, maximum_radius=0), F0),
    "FX-DOF-008": ("Maximum Radius 2.5: three levels mixed.",
                   case(focal_plane=6, maximum_radius=2.5), F0),
    "FX-DOF-009": ("Focal Plane keyed from 0 at frame 0 to 13 at frame 4: the sharp band moves "
                   "from near to far.", case(focal_plane=keyed((0, 0), (4, 13)), maximum_radius=3),
                   [0, 2, 4]),
    "FX-DOF-010": ("Maximum Radius keyed from 0 at frame 0 to 4 at frame 4.",
                   case(focal_plane=6, maximum_radius=keyed((0, 0), (4, 4))), [0, 2, 4]),
    "FX-DOF-011": ("The depth written negative, Focal Plane -6: FX-DOF-002.",
                   case(focal_plane=-6, maximum_radius=3, file="negative.exr"), F0),
    "FX-DOF-012": ("Thickness 1,000,000: everything in focus, unchanged.",
                   case(**AT6, focal_plane_thickness=1e6), F0),
    "FX-DOF-013": ("Infinities and not-a-number in the depth: each read as D-348 reads it, all "
                   "three fully out of focus; the warning every frame.",
                   case(**AT6, file="far.exr"), F0),
    "FX-DOF-014": ("Blender's layout, the depth in ViewLayer.Depth.Z and the colour in "
                   "ViewLayer.Combined: FX-DOF-002.", case(**AT6, file="blender.exr"), F0),
    "FX-DOF-015": ("A file with no depth: unchanged, the warning every frame.",
                   case(**AT6, file="plain.exr"), F0),
    "FX-DOF-016": ("On a PNG drawing: unchanged, the warning.", case(**AT6, layer="card"), F0),
    "FX-DOF-017": ("On a solid: unchanged, the warning.", case(**AT6, layer="solid"), F0),
    "FX-DOF-018": ("On an adjustment layer above the EXR: no file of its own, unchanged, the "
                   "warning.", case(**AT6, on="adjust"), F0),
    "FX-DOF-019": ("The layer moved 2 right and 1 down: FX-DOF-002 moved.",
                   case(**AT6, move=(2, 1)), F0),
    "FX-DOF-020": ("An Exposure of +1 before it: the brighter picture blurred.",
                   case(**AT6, before=1), F0),
    "FX-DOF-021": ("An Exposure of -1 after it.", case(**AT6, after=-1), F0),
    "FX-DOF-022": ("Focal Plane 13, radius 3, Thickness 2: the far corner sharp, the near side "
                   "fully blurred.", case(focal_plane=13, maximum_radius=3,
                                          focal_plane_thickness=2), F0),
}

INVALID = {
    "FX-DOF-023": ("Maximum Radius 201, above 200.", case(maximum_radius=201)),
    "FX-DOF-024": ("Maximum Radius -1, below 0.", case(maximum_radius=-1)),
    "FX-DOF-025": ("Focal Bias 101, above 100.", case(focal_bias=101)),
    "FX-DOF-026": ("Focal Plane Thickness -1, below 0.", case(focal_plane_thickness=-1)),
    "FX-DOF-027": ("Focal Plane keyed to 2,000,000 at frame 4.",
                   case(focal_plane=keyed((0, 0), (4, 2e6)))),
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
    (OUT / "expected_depth_of_field.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    scene = D.plain(case(), 0)
    near = lambda a, b, e=1e-9: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                    for p, q in zip(u, v))
    at = lambda px, x, y: px[y * W + x]  # noqa: E731
    assert fraction(12, 6, 0, 50) == 0.5 and fraction(2, 6, 0, 50) == 1.0
    assert fraction(6, 6, 0, 50) == 0.0 and fraction(7, 6, 4, 100) == 0.0
    assert fraction(D.BIG, 6, 0, 50) == 1.0 and fraction(0.0, 6, 0, 50) == 1.0
    two = c["FX-DOF-002"]["0"]
    z = lambda x, y: 0.5 + 1.25 * x + 0.75 * y  # noqa: E731
    for y in range(H):
        for x in range(W):
            if fraction(z(x, y), 6, 0, 50) == 0:
                assert at(two, x, y) == at(scene, x, y)
    assert two != scene and c["FX-DOF-001"]["0"] != two
    three = c["FX-DOF-003"]["0"]
    for y in range(H):
        for x in range(W):
            if 4 <= z(x, y) <= 8:
                assert at(three, x, y) == at(scene, x, y), (x, y)
    assert c["FX-DOF-004"]["0"] not in (two, scene) and c["FX-DOF-005"]["0"] not in (two, scene)
    for fx in ("FX-DOF-006", "FX-DOF-007", "FX-DOF-012", "FX-DOF-015", "FX-DOF-018"):
        assert c[fx]["0"] == scene, fx
    assert near(c["FX-DOF-009"]["0"], D.render(case(maximum_radius=3), 0))
    assert c["FX-DOF-010"]["0"] == scene
    assert near(c["FX-DOF-010"]["4"], D.render(case(focal_plane=6, maximum_radius=4), 0))
    assert near(c["FX-DOF-011"]["0"], two) and near(c["FX-DOF-014"]["0"], two)
    assert expected["cases"]["FX-DOF-013"]["frame_warning"] == "MEDIA_EXR_ADJUSTED"
    assert all(c["FX-DOF-019"]["0"][(y + 1) * W + x + 2] == two[y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    assert c["FX-DOF-001"]["0"] == c["FX-DOF-001"]["4"]
    assert c["FX-DOF-022"]["0"] not in (two, scene)
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(v >= -1e-12 for v in p[:3]), (name, p)
    print("checks passed")


if __name__ == "__main__":
    main()
