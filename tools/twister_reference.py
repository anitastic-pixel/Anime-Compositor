"""Twister, worked a second way.

D-517 adds Twister (`core.twister`), after CycoreFX's CC Twister (Transition, EFFECTS.md row
279), under a name of our own: a transition in which the layer twists round a line, like a ribbon
turned over a strip at a time, to show its back. The CycoreFX HD 1.8.1 manual names the controls
(Completion; Backside, the layer seen on the back, the layer itself twisting into itself, none
twisting into nothingness, a layer of another size revealed centred; Shading, light and shadow on
the twist; Center, the point of origin of the wipe; Axis, the direction of twisting, in degrees)
but publishes no formula and no settings when added, so the rule below and those settings are
ours, the "rule ours" precedent.

At the holder's composition frame n, with its input O, linear and premultiplied, whose drawing's
own top-left pixel is at (ox, oy) however far earlier effects grew it, and whose drawing is its
own W by H:

1. The back B: with `backside` "" nothing (transparent); otherwise D-189's map of the named layer
   (`tools/effect_layer_reference.py`) fitted to W by H by `center`, lying on the drawing at its
   corner, transparent outside it. If it names a layer that is not in the composition (D-189's
   warning), the output is O.
2. In the drawing's own W by H space, C = (center x / 100 W, center y / 100 H), a = `axis` in
   degrees, d = (cos a, sin a) and n = (-sin a, cos a). At a pixel centre P = (x + 1/2, y + 1/2),
   u = (P - C) . d and v = (P - C) . n. U is the largest |u| of the drawing's four corners, and
   t = (u + U) / (2 U), 0 at the side the twist starts from and 1 at the far side.
3. With c = completion / 100, the strip at P is turned by theta = pi clamp(2 c - t, 0, 1) round
   the line through C along d, so s = |cos theta| is how much of its width faces the viewer. The
   point read is Q = C + u d + (v / s) n.
4. Where cos theta > 0 the front shows: O's bilinear read (document 21's, pixel centres at halves,
   transparent outside) at Q + (ox, oy). Where cos theta < 0 the back shows: B's bilinear read at
   Q (not mirrored); clear with `backside` "". Where s is 0 the strip is edge on: clear.
5. `shading` "on" multiplies the colour (not the covering) by s: the strip darkens as it turns
   edge on and comes back to full on the far face.
6. The layer does not grow. Nothing is a distance, so a draft changes nothing.

`completion` 0 to 100, 0 when added; `backside` D-189's word, "" when added (nothing); `shading`
"on" (when added) or "off", exact; `center` x then y, -1000 to 1000 per cent of the drawing's
width and height, [50, 50] when added; `axis` -3600 to 3600 degrees, 0 when added. Completion,
center and axis are keyable. As added, completion 0, nothing changes.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values. No case puts a pixel centre where |cos theta| is below 1e-5 (checked below), so
single precision reads stay inside the tolerance.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/twister/`: the map
layers, every one switched off, and on top of them the drawing `holder` with the effect. The
expected pixels are in `Fixtures/twister/expected_twister.json`, with a project whose layer
settings go round in a circle.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/twister_reference.py
"""

import json
import sys
from math import cos, pi, radians, sin
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import displacement_map_reference as D  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "twister"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
CLEAR = [0.0, 0.0, 0.0, 0.0]
EDGE = 1e-5

DRAWINGS = {
    "holder": D.DRAWINGS["holder"],
    "paint": D.DRAWINGS["paint"],
    # The paint's middle 8 by 6, (4, 2) to (11, 7): centred on 16 by 10 it lies where it was cut.
    "small": [row[4:12] for row in D.DRAWINGS["paint"][2:8]],
}


def decoded(name):
    if name in D.DRAWINGS:
        return D.decoded(name)
    from adjust_reference import srgb_to_linear
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


BACKS = {"paint": "paint", "small": "small", "late": "paint"}
NEAREST = [1.0]  # the smallest |cos theta| met at a pixel centre, over every case


# --- the rule -------------------------------------------------------------------------------

def twist(o, back, completion, shading, center, axis):
    """Steps 2 to 5 on the input o with the back picture (None: nothing), W by H."""
    w, ht = o["w"], o["h"]
    cx, cy = center[0] / 100 * w, center[1] / 100 * ht
    a = radians(axis)
    dx, dy = cos(a), sin(a)
    nx, ny = -dy, dx
    big = max(abs((px - cx) * dx + (py - cy) * dy) for px in (0, w) for py in (0, ht))
    c = completion / 100
    out = []
    for y in range(ht):
        for x in range(w):
            px, py = x + 0.5 - cx, y + 0.5 - cy
            u, v = px * dx + py * dy, px * nx + py * ny
            t = (u + big) / (2 * big)
            k = cos(pi * min(1.0, max(0.0, 2 * c - t)))
            s = abs(k)
            if 0 < 2 * c - t < 1:
                NEAREST[0] = min(NEAREST[0], s)
            if s == 0:
                out.append(CLEAR)
                continue
            qx, qy = cx + u * dx + v / s * nx, cy + u * dy + v / s * ny
            if k > 0:
                p = list(L.bilinear(o, qx, qy))
            elif back is not None:
                p = list(L.bilinear(back, qx, qy))
            else:
                p = CLEAR
            if shading == "on":
                p = [p[0] * s, p[1] * s, p[2] * s, p[3]]
            out.append(p)
    return L.pic(w, ht, out)


# --- the cases ------------------------------------------------------------------------------

def case(completion=0, backside="", shading="on", center=(50, 50), axis=0, shift=(0, 0),
         on="holder"):
    return {"completion": completion, "backside": backside, "shading": shading,
            "center": center, "axis": axis, "shift": shift, "on": on}


def back_picture(name, frame):
    if name == "":
        return None
    if name == "holder":
        return decoded("holder")
    p = L.EMPTY if name == "late" and frame < 4 else decoded(BACKS[name])
    return L.fit(p, "center", W, H)


def worked(c, frame):
    o = decoded("holder")
    if c["backside"] not in set(BACKS) | {"", "holder"}:
        return o
    return twist(o, back_picture(c["backside"], frame), value_at(c["completion"], frame),
                 c["shading"], value_at(c["center"], frame), value_at(c["axis"], frame))


def render(c, frame):
    """The composition's pixels at frame: the holder with its effect, moved by its shift; or,
    with the effect on an adjustment layer above, the frame below through it."""
    out = worked(c, frame)
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded("holder")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


CASES = {
    "FX-TWIST-001": ("As added: completion 0, so nothing changes.", case(), (0,)),
    "FX-TWIST-002": ("Completion 100 with nothing on the back: every pixel clear.",
                     case(100), (0,)),
    "FX-TWIST-003": ("Completion 100 with the paint on the back: the paint exactly, shading on "
                     "but every strip flat again.", case(100, "paint"), (0,)),
    "FX-TWIST-004": ("Completion 25, the paint on the back, shading off: the left half twisting "
                     "(the far left nearly edge on), the right half untouched.",
                     case(25, "paint", "off"), (0,)),
    "FX-TWIST-005": ("FX-TWIST-004 with shading on: the twisting strips darker the more they "
                     "have turned.", case(25, "paint"), (0,)),
    "FX-TWIST-006": ("Completion 50, the paint on the back: the left half shows the paint, the "
                     "right half the holder, squeezed toward the middle row where they turn.",
                     case(50, "paint"), (0,)),
    "FX-TWIST-007": ("Completion 60, axis 90: the twist runs downward, the top rows over first.",
                     case(60, "paint", axis=90), (0,)),
    "FX-TWIST-008": ("Completion 45, axis 30, Center at 40, 60: a slanted twist off the middle.",
                     case(45, "paint", center=(40, 60), axis=30), (0,)),
    "FX-TWIST-009": ("Completion 70, axis -135, shading off: the twist from the bottom right.",
                     case(70, "paint", "off", axis=-135), (0,)),
    "FX-TWIST-010": ("Completion 50, nothing on the back: where the paint was, clear.",
                     case(50), (0,)),
    "FX-TWIST-011": ("The holder on its own back, completion 50: the layer twists into itself.",
                     case(50, "holder"), (0,)),
    "FX-TWIST-012": ("The paint's middle 8 by 6 on the back, centred: it lies where it was cut, "
                     "clear round it.", case(50, "small"), (0,)),
    "FX-TWIST-013": ("Completion keyed from 0 at frame 0 to 100 at frame 4: frame 0 the holder, "
                     "frame 2 FX-TWIST-006, frame 4 the paint.",
                     case(keyed((0, 0), (4, 100)), "paint"), (0, 2, 4)),
    "FX-TWIST-014": ("Axis keyed from 0 at frame 0 to 90 at frame 4, completion 60: frame 4 is "
                     "FX-TWIST-007.", case(60, "paint", axis=keyed((0, 0), (4, 90))), (0, 2, 4)),
    "FX-TWIST-015": ("Center keyed from 50, 50 at frame 0 to 40, 60 at frame 4, completion 45, "
                     "axis 30: frame 4 is FX-TWIST-008.",
                     case(45, "paint", center=keyed((0, (50, 50)), (4, (40, 60))), axis=30),
                     (0, 4)),
    "FX-TWIST-016": ("A back layer that is not in the composition, `gone`: nothing changes, and "
                     "the warning every frame.", case(50, "gone"), (0, 4)),
    "FX-TWIST-017": ("The paint with its in point at frame 4 on the back: before it the back is "
                     "clear; at frame 4 FX-TWIST-006.", case(50, "late"), (0, 4)),
    "FX-TWIST-018": ("FX-TWIST-006 on the holder moved 2 right and 1 down: the same picture "
                     "moved, since the back lies on the layer.",
                     case(50, "paint", shift=(2, 1)), (0,)),
    "FX-TWIST-019": ("The effect on an adjustment layer above the holder: the back lies on the "
                     "frame, which here is the holder's own rectangle, so this is FX-TWIST-006.",
                     case(50, "paint", on="adjust"), (0,)),
    "FX-TWIST-020": ("Completion 50, Center at -200, 50: the line far off to the left, so the "
                     "twist reaches across the drawing at once, every strip partly turned.",
                     case(50, "paint", center=(-200, 50)), (0,)),
    "FX-TWIST-021": ("Axis 360: the same as axis 0, FX-TWIST-006.",
                     case(50, "paint", axis=360), (0,)),
}

INVALID = {
    "FX-TWIST-022": ("Completion 101, above 100.", case(101, "paint")),
    "FX-TWIST-023": ("Completion -1, below 0.", case(-1, "paint")),
    "FX-TWIST-024": ("Axis 3601, above 3600.", case(50, "paint", axis=3601)),
    "FX-TWIST-025": ("Axis -3601, below -3600.", case(50, "paint", axis=-3601)),
    "FX-TWIST-026": ("Center x 1001, above 1000.", case(50, "paint", center=(1001, 50))),
    "FX-TWIST-027": ("Center y -1001, below -1000.", case(50, "paint", center=(50, -1001))),
    "FX-TWIST-028": ("Shading \"maybe\", not on or off.", case(50, "paint", "maybe")),
    "FX-TWIST-029": ("A back layer written as the number 3, not a word.", case(50, 3)),
    "FX-TWIST-030": ("Completion keyed to 200 at frame 4.",
                     case(keyed((0, 30), (4, 200)), "paint")),
}

KEYS = ("completion", "backside", "shading", "center", "axis")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.twister", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in KEYS}}


def map_layers():
    return [L.raster("paint", "asset-paint", enabled=False, out_frame=FRAMES),
            L.raster("small", "asset-small", enabled=False, out_frame=FRAMES),
            L.raster("late", "asset-paint", enabled=False, in_frame=4, out_frame=FRAMES)]


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


LOADS = {
    "cycle_two.json": ("`a` and `b`, each the holder's drawing, each with a Twister whose back "
                       "layer is the other: refused, `EFFECT_LAYER_CYCLE`.",
                       [drawing("a", effect("fx-a-0", case(50, "b"))),
                        drawing("b", effect("fx-b-0", case(50, "a")))]),
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
        if c["backside"] == "gone":
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

    (OUT / "expected_twister.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def near(a, b, eps=1e-12):
    return all(abs(x - y) <= eps for p, q in zip(a, b) for x, y in zip(p, q))


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    paint = [list(L.at(decoded("paint"), x, y)) for y in range(H) for x in range(W)]
    gone = [CLEAR] * (W * H)
    col = lambda px, x: [px[y * W + x] for y in range(H)]  # noqa: E731
    assert NEAREST[0] >= EDGE, NEAREST[0]
    assert c["FX-TWIST-001"]["0"] == drawn
    assert near(c["FX-TWIST-002"]["0"], gone)
    assert near(c["FX-TWIST-003"]["0"], paint)
    # Completion 25, axis 0: t = (x + 1/2) / 16, theta = pi (1/2 - t), so columns 8 to 15 are
    # untouched and columns 0 to 7 turned but still facing front.
    r = c["FX-TWIST-004"]["0"]
    for x in range(8, W):
        assert col(r, x) == col(drawn, x), x
    assert col(r, 0) != col(drawn, 0)
    # Shading only darkens: the same coverings, colour no brighter.
    for p, q in zip(c["FX-TWIST-005"]["0"], r):
        assert abs(p[3] - q[3]) <= 1e-12 and all(p[i] <= q[i] + 1e-12 for i in range(3))
    # Completion 50: theta = pi (1 - t), so columns 0 to 7 show the back, 8 to 15 the front.
    h6 = c["FX-TWIST-006"]["0"]
    # Nothing on the back: the back half clear.
    for x in range(8):
        assert col(c["FX-TWIST-010"]["0"], x) == [CLEAR] * H, x
    for x in range(8, W):
        assert col(c["FX-TWIST-010"]["0"], x) == col(h6, x), x
    # Keyed frames agree with the fixed cases.
    assert c["FX-TWIST-013"]["0"] == drawn
    assert near(c["FX-TWIST-013"]["2"], h6)
    assert near(c["FX-TWIST-013"]["4"], paint)
    assert near(c["FX-TWIST-014"]["4"], c["FX-TWIST-007"]["0"])
    assert near(c["FX-TWIST-015"]["4"], c["FX-TWIST-008"]["0"])
    assert c["FX-TWIST-016"]["0"] == drawn and c["FX-TWIST-016"]["4"] == drawn
    assert near(c["FX-TWIST-017"]["0"], c["FX-TWIST-010"]["0"])
    assert c["FX-TWIST-017"]["4"] == h6
    # The small back: clear at the corner pixel's back side.
    assert c["FX-TWIST-012"]["0"][0] == CLEAR
    # Moved and adjusted.
    moved = c["FX-TWIST-018"]["0"]
    assert all(moved[(y + 1) * W + x + 2] == h6[y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    assert c["FX-TWIST-019"]["0"] == h6
    assert near(c["FX-TWIST-021"]["0"], h6, 1e-9)
    # Far-off line: every column turned, none untouched.
    for x in range(W):
        assert col(c["FX-TWIST-020"]["0"], x) != col(drawn, x), x
    print("checks passed")


if __name__ == "__main__":
    main()
