"""CC Spotlight's Gel renders, worked a second way.

D-519 builds the four Gel renders and the Gel Layer of `core.cc_spotlight` (D-507, EFFECTS.md row
341), after CycoreFX's CC Spotlight (Perspective). The CycoreFX HD 1.8.1 manual says: Gel Only
"clears the source layer and renders the spotlight using the Gel Layer as a gel", like a slide
projector; Gel Add "projects the Gel Layer onto the source layer"; Gel Add+ "also adds light to
transparent areas"; Gel Shadow "renders a shadow but lets light through the projected gel"; and
Gel Layer is the layer used as the gel. It publishes no formula, so the rule below is ours (the
"rule ours" precedent, document 21, D-507).

The light, its cone and its share f at a pixel are D-507's (`tools/cc_spotlight_reference.py`).
Added to them:

1. The gel G: with `gel_layer` "" no gel (clear); otherwise D-189's map of the named layer
   (`tools/effect_layer_reference.py`) fitted to the drawing's W by H by `center`. If it names a
   layer that is not in the composition (D-189's warning), the output is the input. Only a Gel
   render reads the gel layer: a Light render ignores it, says nothing and is never in a cycle.
2. The gel is carried across the cone. With the light at F, the axis A = T - F and its unit a,
   e1 is the drawing's x direction (1, 0, 0) with its part along a taken off, made a unit (the
   y direction (0, 1, 0) instead when that leaves less than 1e-9), and e2 = a x e1, so a light
   straight down shows the gel upright and not mirrored. At the pixel's centre P, D = P - F and
   the gel point is g = (D . e1, D . e2) / ((D . a) tan o'), o' the half opening held at most 89
   degrees, so the cone's edge is |g| = 1 (at o below 89 degrees). With M the larger of W and H,
   the gel is read bilinearly (document 21's, pixel centres at halves, transparent outside) at
   (W / 2 + g1 M / 2, H / 2 + g2 M / 2): the gel's longer side spans the cone.
3. The light let through, per channel, is the gel over white: T = (1 - ga) + gc, (gc, ga) the
   gel's premultiplied read. Clear gel lets all the light through; covering gel lets its colour.
4. Each Gel render is its Light render with the colour C replaced by C T: gel_only is
   light_only's (C T k, min(k, 1)), gel_add light_add's, gel_add_plus light_add_plus's, and
   gel_shadow light_shadow's c ((1 - f)(1 - min(i, 1)) + f C T). Where f is 0 the gel is not read.

`gel_layer` is D-189's word, "" when added; a file that leaves it out reads it as "" (D-360's
precedent), so every CC Spotlight saved before D-519 opens as it was. With "" a Gel render is its
Light render. The layer never grows; a draft changes nothing but D-189's map.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/cc_spotlight_gel/`:
the gel layers, every one switched off, and on top of them the drawing `holder` with the effect.
The expected pixels are in `Fixtures/cc_spotlight_gel/expected_cc_spotlight_gel.json`, with a
project whose gel layers go round in a circle.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/cc_spotlight_gel_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import displacement_map_reference as D  # noqa: E402
import cc_spotlight_reference as SP  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "cc_spotlight_gel"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
assert (SP.W, SP.H) == (W, H)
CLEAR = [0.0, 0.0, 0.0, 0.0]

SLIDE = (20, 40, 160, 255)  # a deep blue
DRAWINGS = {
    "holder": D.DRAWINGS["holder"],
    "paint": D.DRAWINGS["paint"],
    # The paint's middle 8 by 6: centred on 16 by 10 it lies where it was cut.
    "small": [row[4:12] for row in D.DRAWINGS["paint"][2:8]],
    # Clear, with a blue cross through the middle and a half-covering red square at top left.
    "slide": [[SLIDE if x in (7, 8) or y in (4, 5) else
               ((220, 30, 30, 128) if x < 4 and y < 3 else (0, 0, 0, 0)) for x in range(W)]
              for y in range(H)],
}
GELS = {"paint": "paint", "small": "small", "slide": "slide", "late": "slide"}
GEL_RENDERS = ("gel_only", "gel_add", "gel_add_plus", "gel_shadow")


def decoded(name):
    if name in D.DRAWINGS:
        return D.decoded(name)
    from adjust_reference import srgb_to_linear
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


# --- the rule -------------------------------------------------------------------------------

def unit(v):
    n = math.sqrt(sum(c * c for c in v))
    return [c / n for c in v]


def gel_point(k, x, y):
    """Step 2: where the gel is read for pixel (x, y), or None when there is no axis."""
    f = (k["from"][0] / 100 * W, k["from"][1] / 100 * H, -k["height"] / 100 * W)
    t = (k["to"][0] / 100 * W, k["to"][1] / 100 * H, 0.0)
    a = [t[i] - f[i] for i in range(3)]
    if a == [0.0, 0.0, 0.0]:
        return None
    a = unit(a)
    e1 = [1.0 - a[0] * a[0], -a[0] * a[1], -a[0] * a[2]]
    if math.sqrt(sum(c * c for c in e1)) < 1e-9:
        e1 = [-a[1] * a[0], 1.0 - a[1] * a[1], -a[1] * a[2]]
    e1 = unit(e1)
    e2 = [a[1] * e1[2] - a[2] * e1[1], a[2] * e1[0] - a[0] * e1[2], a[0] * e1[1] - a[1] * e1[0]]
    d = [x + 0.5 - f[0], y + 0.5 - f[1], -f[2]]
    o = min(math.radians(k["cone_angle"]) / 2, math.radians(89))
    s = sum(d[i] * a[i] for i in range(3)) * math.tan(o)
    g1, g2 = sum(d[i] * e1[i] for i in range(3)) / s, sum(d[i] * e2[i] for i in range(3)) / s
    m = max(W, H)
    return W / 2 + g1 * m / 2, H / 2 + g2 * m / 2


def spotlight(px, k, gel):
    """D-507's renders, with the colour C replaced by C T for a Gel render."""
    C, i = SP.colour(k["color"]), k["intensity"] / 100
    light = k["render"].replace("gel_only", "light_only").replace("gel_", "light_")
    out = []
    for n, w in enumerate(px):
        x, y = n % W, n // W
        f = SP.share(k, x, y)
        q = f * i
        cc = C
        if f > 0 and k["render"] in GEL_RENDERS and gel is not None:
            gc = L.bilinear(gel, *gel_point(k, x, y))
            cc = [C[j] * ((1 - gc[3]) + gc[j]) for j in range(3)]
        c, a = w[:3], w[3]
        if light == "light_only":
            out.append([v * q for v in cc] + [min(q, 1.0)])
        elif light == "light_add":
            out.append([c[j] + cc[j] * q * a for j in range(3)] + [a])
        elif light == "light_add_plus":
            out.append([c[j] + cc[j] * q for j in range(3)] + [a + (1 - a) * min(q, 1.0)])
        else:
            out.append([c[j] * ((1 - f) * (1 - min(i, 1.0)) + f * cc[j]) for j in range(3)] + [a])
    return out


# --- the cases ------------------------------------------------------------------------------

NUMBERS = SP.NUMBERS
KEYS = ("from", "to", "height", "cone_angle", "edge_softness", "color", "intensity", "render",
        "gel_layer")


def case(render="gel_add", gel="paint", color="#ffffff", shift=(0, 0), on="holder", **numbers):
    c = {"render": render, "gel_layer": gel, "color": color, "shift": shift, "on": on,
         "from": [50, 50], "to": [50, 50], "height": 50, "cone_angle": 90, "edge_softness": 20,
         "intensity": 100}
    for n, v in numbers.items():
        assert n in NUMBERS, n
        c[n] = v
    return c


def settings(c, frame):
    k = {n: value_at(c[n], frame) for n in NUMBERS}
    k.update(color=c["color"], render=c["render"])
    return k


def gel_picture(name, frame):
    if name == "":
        return None
    if name == "holder":
        return decoded("holder")
    p = L.EMPTY if name == "late" and frame < 4 else decoded(GELS[name])
    return L.fit(p, "center", W, H)


def worked(c, frame):
    o = decoded("holder")
    px = [list(L.at(o, x, y)) for y in range(H) for x in range(W)]
    if c["render"] in GEL_RENDERS and c["gel_layer"] not in set(GELS) | {"", "holder"}:
        return px
    gel = gel_picture(c["gel_layer"], frame) if c["render"] in GEL_RENDERS else None
    return spotlight(px, settings(c, frame), gel)


def render(c, frame):
    out = L.pic(W, H, worked(c, frame))
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded("holder")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


CASES = {
    "FX-SPOTGEL-001": ("Render gel only, the paint as the gel, the light straight down on the "
                       "middle, opening 90 degrees: the drawing is gone and the paint is "
                       "projected in a round pool, its long side across the pool, fading at "
                       "the edge.", case("gel_only"), [0]),
    "FX-SPOTGEL-002": ("Render gel add: the paint's light added to the drawing where it is "
                       "covered; the transparent corner at the bottom right stays transparent.",
                       case(), [0]),
    "FX-SPOTGEL-003": ("Render gel add plus, the light straight down on the transparent corner "
                       "at the bottom right: the paint's light falls on the transparent part too.",
                       case("gel_add_plus", to=[85, 85], **{"from": [85, 85]}), [0]),
    "FX-SPOTGEL-004": ("Render gel shadow, the slide as the gel: outside the cone black, inside "
                       "the drawing as it was where the slide is clear and tinted blue under its "
                       "cross.", case("gel_shadow", "slide"), [0]),
    "FX-SPOTGEL-005": ("Render gel add with no gel layer: exactly light add.",
                       case("gel_add", ""), [0]),
    "FX-SPOTGEL-006": ("Render light add with the paint named as the gel: a Light render "
                       "ignores the gel, so exactly light add.", case("light_add"), [0]),
    "FX-SPOTGEL-007": ("Gel add, the small paint (8 by 6) as the gel: centred on the drawing, so "
                       "its light is the paint's in the middle and white around it.",
                       case("gel_add", "small"), [0]),
    "FX-SPOTGEL-008": ("Gel add, the gel a layer that starts at frame 4: plain white light "
                       "before, the slide from frame 4.", case("gel_add", "late"), [0, 4]),
    "FX-SPOTGEL-009": ("Gel add, the gel layer \"gone\" not in the composition: the drawing "
                       "unchanged, warned every frame.", case("gel_add", "gone"), [0, 2]),
    "FX-SPOTGEL-010": ("Gel only, the light low at the top left, height 20, opening 60: the "
                       "paint is drawn out into a long oval.",
                       case("gel_only", height=20, cone_angle=60, **{"from": [10, 10]}), [0]),
    "FX-SPOTGEL-011": ("Gel add, an orange light at intensity 250 through the slide: orange "
                       "light past 1 through the clear parts, darker under the cross.",
                       case("gel_add", "slide", "#ff8000", intensity=250), [0]),
    "FX-SPOTGEL-012": ("Gel shadow at intensity 50 through the paint: outside the cone half as "
                       "dark, inside tinted by the paint.", case("gel_shadow", intensity=50), [0]),
    "FX-SPOTGEL-013": ("Gel only, the holder itself as the gel: its own drawing projected onto "
                       "the cleared layer.", case("gel_only", "holder"), [0]),
    "FX-SPOTGEL-014": ("Gel add, opening 180: the gel's spread is held at 89 degrees, so its "
                       "middle colours the whole lit drawing.", case(cone_angle=180), [0]),
    "FX-SPOTGEL-015": ("Gel add with From keyed from [30, 50] at frame 0 to [70, 50] at frame "
                       "4, linear: the gel slides and stretches as the light swings.",
                       case(**{"from": keyed((0, (30, 50)), (4, (70, 50)))}), [0, 2, 4]),
    "FX-SPOTGEL-016": ("FX-SPOTGEL-004 on an adjustment layer above the drawing: the same.",
                       case("gel_shadow", "slide", on="adjust"), [0]),
    "FX-SPOTGEL-017": ("FX-SPOTGEL-002 moved three pixels right: the same, moved, light and "
                       "all.", case(shift=(3, 0)), [0]),
    "FX-SPOTGEL-018": ("Gel add, the light along the drawing (height 0) from the left middle "
                       "to the right middle: the axis lies along x, so the gel's across is the "
                       "drawing's y.", case(height=0, **{"from": [-50, 50]}, to=[100, 50],
                                            cone_angle=60), [0]),
}

INVALID = {
    "FX-SPOTGEL-019": ("A gel layer written as the number 3, not a word.", case("gel_add", 3)),
    "FX-SPOTGEL-020": ("Render \"Gel_Add\": the word is exact, so a capital is not it.",
                       case("Gel_Add")),
    "FX-SPOTGEL-021": ("Render \"gel_glow\": not one of CC Spotlight's renders.",
                       case("gel_glow")),
}


def effect(fid, c):
    params = {k: setting_json(c[k]) for k in KEYS if k in NUMBERS}
    params.update({k: c[k] for k in ("color", "render", "gel_layer")})
    return {"instance_id": fid, "type_id": "core.cc_spotlight", "enabled": True,
            "parameters": params}


def gel_layers():
    return [L.raster("paint", "asset-paint", enabled=False, out_frame=FRAMES),
            L.raster("small", "asset-small", enabled=False, out_frame=FRAMES),
            L.raster("slide", "asset-slide", enabled=False, out_frame=FRAMES),
            L.raster("late", "asset-slide", enabled=False, in_frame=4, out_frame=FRAMES)]


def project(pid, layers):
    return {"schema_version": 0, "project_id": pid,
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, FRAMES, layers)]}


def project_json(fx, c):
    holder = L.raster("holder", "asset-holder", position=c["shift"], out_frame=FRAMES)
    layers = gel_layers() + [holder]
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
    "cycle_two.json": ("`a` and `b`, each the holder's drawing, each with a CC Spotlight "
                       "rendering gel add whose gel layer is the other: refused, "
                       "`EFFECT_LAYER_CYCLE`.",
                       [drawing("a", effect("fx-a-0", case("gel_add", "b"))),
                        drawing("b", effect("fx-b-0", case("gel_add", "a")))], "EFFECT_LAYER_CYCLE"),
    "light_names_other.json": ("The same two layers with Light renders: a Light render never "
                               "reads its gel layer, so there is no cycle and the file opens.",
                               [drawing("a", effect("fx-a-0", case("light_add", "b"))),
                                drawing("b", effect("fx-b-0", case("light_add", "a")))], None),
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
        if c["gel_layer"] == "gone":
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
    for name, (says, layers, refused) in LOADS.items():
        write(name, project("proj-" + name[:-5].replace("_", "-"), gel_layers() + layers))
        expected["loads"][name] = {"says": says, "refused": refused}

    (OUT / "expected_cc_spotlight_gel.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                         encoding="utf-8")
    check(expected)


def near(a, b, eps=1e-12):
    return all(abs(x - y) <= eps for p, q in zip(a, b) for x, y in zip(p, q))


def light(render, **kw):
    """The same case worked by D-507's own tool, the gel left out."""
    c = case(render, "", **kw)
    o = decoded("holder")
    px = [list(L.at(o, x, y)) for y in range(H) for x in range(W)]
    return SP.spotlight(px, settings(c, 0))


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    assert near(c["FX-SPOTGEL-005"]["0"], light("light_add"))
    assert near(c["FX-SPOTGEL-006"]["0"], light("light_add"))
    assert c["FX-SPOTGEL-002"]["0"] != c["FX-SPOTGEL-005"]["0"]
    # a light straight down shows the gel upright: the pixel at the middle reads the gel there
    k = settings(case(), 0)
    assert near([gel_point(k, 7, 4)], [(7.5, 4.5)])
    gx, gy = gel_point(k, 11, 4)
    assert gx > 7.5 and abs(gy - 4.5) < 1e-12
    gx, gy = gel_point(k, 7, 7)
    assert gy > 4.5 and abs(gx - 7.5) < 1e-12
    # the cone's edge is |g| = 1: a point on the circle of the cone reads the gel's long side end
    o = math.radians(45)
    r = 0.5 * W * math.tan(o)
    assert abs(r - 8) < 1e-12
    # gel add leaves transparent pixels transparent; gel only clears the drawing
    assert all(p == q for p, q in zip(c["FX-SPOTGEL-002"]["0"], drawn) if q[3] == 0)
    lit = [SP.share(k, n % W, n // W) for n in range(W * H)]
    assert all(near([p], [[0, 0, 0, 0]]) for p, f in zip(c["FX-SPOTGEL-001"]["0"], lit) if f == 0)
    three = c["FX-SPOTGEL-003"]["0"]
    assert any(q[3] == 0 and p[3] > 0.5 for p, q in zip(three, drawn))
    # gel shadow through the slide: clear parts of the slide light the drawing as light shadow
    four, ls = c["FX-SPOTGEL-004"]["0"], light("light_shadow")
    ks = settings(case("gel_shadow", "slide"), 0)
    slide = gel_picture("slide", 0)
    clear = [n for n in range(W * H) if lit[n] > 0
             and L.bilinear(slide, *gel_point(ks, n % W, n // W))[3] == 0]
    blue = [n for n in range(W * H) if lit[n] == 1
            and L.bilinear(slide, *gel_point(ks, n % W, n // W))[3] == 1]
    assert clear and blue
    assert all(near([four[n]], [ls[n]]) for n in clear)
    assert all(four[n][2] >= four[n][0] for n in blue)
    # the late gel: plain light before frame 4, then the slide
    assert near(c["FX-SPOTGEL-008"]["0"], light("light_add"))
    assert c["FX-SPOTGEL-008"]["4"] == render(case("gel_add", "slide"), 0)
    assert c["FX-SPOTGEL-009"]["0"] == drawn and c["FX-SPOTGEL-009"]["2"] == drawn
    assert c["FX-SPOTGEL-015"]["2"] == render(case(), 0)
    assert c["FX-SPOTGEL-016"]["0"] == c["FX-SPOTGEL-004"]["0"]
    moved, two = c["FX-SPOTGEL-017"]["0"], c["FX-SPOTGEL-002"]["0"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == two[y * W:y * W + W - 3]
    assert any(max(p[:3]) > 1 for p in c["FX-SPOTGEL-011"]["0"])
    for fx in ("FX-SPOTGEL-007", "FX-SPOTGEL-010", "FX-SPOTGEL-013", "FX-SPOTGEL-014",
               "FX-SPOTGEL-018"):
        assert c[fx]["0"] != drawn, fx
    print("checked")


if __name__ == "__main__":
    main()
