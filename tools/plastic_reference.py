"""Plastic, worked a second way.

D-463 adds `core.plastic`, after After Effects' CC Plastic (Stylize): the layer given a lit,
shiny relief from a bump map read from a channel of a layer, its own or another (D-189's layer
setting), keeping its own colours and shape. What is known (the CycoreFX HD 1.8.1 manual's page
on CC Plastic; no tutorial found gives more) is: Bump Layer, the layer itself at first, and
Property, Red, Green, Blue, Alpha, Luminance or Lightness, make the height map, bright high and
dark low; Softness blurs it; Height sets how high it stands; Cut Min and Cut Max compress the
range of its values, Min the dark and Max the bright, and a Min above Max turns the bumps
inside out; the light is the effect's own, Distant or Point, with Intensity, Color, Height
(negative behind the layer), Position (point) and Direction (distant), and an Ambient Light
Color; the shading is Ambient, Diffuse, Dust ("very small particles covering the surface",
reflecting "in a different manner and more subtle than Diffuse"), Specular, Roughness (the
opposite of After Effects' Shininess) and Metal (100 the highlight in the layer's colour, 0 in
the light's, "like plastic"). The rule below is this program's own; the lighting is
Blobbylize's (D-379), and nothing is ported.

The rule, at the holder's frame, with its input O, linear and premultiplied:

1. The map is D-189's layer setting, fitted to the drawing's own W by H and lying on it, clear
   outside it; with the name "" (or a layer not in the composition, with the warning) it is O
   itself. F is at each pixel the map's `property` as CC Vector Blur reads it: red, green, blue,
   luminance or lightness of the straight encoded colour, times the covering; or the covering
   itself for `alpha`.
2. Bv is F through document 21's Gaussian at sigma `softness / 2`, nothing outside the drawing,
   worked out past its edges as far as the kernel reaches; at softness 0, Bv is F and 0 outside.
3. The cut: with lo = `cut_min / 100` and hi = `cut_max / 100`, b = (Bv - lo) / (hi - lo) held to
   0..1, so the values below lo lie flat at 0 and those above hi flat at 1; with lo above hi the
   bumps are turned inside out; with lo = hi, b is 1 where Bv is at least lo and 0 below.
4. The surface: N is (-k gx, -k gy, 1) made one long, g the central differences of b,
   `(b(x+1, y) - b(x-1, y)) / 2` and down the same, k = `height / 100` 1.25 max(softness, 1), as
   CC Glass slopes its bump. A negative height sinks the bright parts.
5. The light, one of the effect's own (After Effects' light layers are not offered): Lv, toward
   it, made one long, is (100 ux, 100 uy, `light_height`) for `distant`, (ux, uy) document 21's
   step at `light_direction` degrees clockwise from up; or for `point`, the light at
   `light_position` (per cent of the drawing's width and height) and `light_height` pixels
   above, Lv = (Lx - px, Ly - py, light_height) from the pixel's centre; at no length, none.
6. The shading: nl = N.Lv; diffuse max(nl, 0); dust, Lommel and Seeliger's law for a dusty
   surface seen from straight in front, nl / (nl + Nz) where nl is above 0, else 0; the
   reflection R = 2 nl N - Lv and the highlight Rz^(1 / roughness) where nl and Rz are above 0,
   else 0. With C the layer's straight colour (0 where it is clear), I = `light_intensity / 100`,
   Lc the light colour and Ac the ambient light colour in linear light, and ambient, diffuse,
   dust, specular and metal as shares of 100: each channel out = C (ambient Ac + diffuse I Lc
   diffuse-term + dust I Lc dust-term) + specular I (Lc + (C - Lc) metal) highlight.
7. The output is out times O's own covering, with that covering: the layer keeps its shape and
   does not grow. For a draft the softness is a distance, and so is the height of a point light.

`layer` D-189's word, ""; `fit` `stretch`, `center` or `tile`, `stretch`; `property` red, green,
blue, alpha, luminance or lightness, `alpha`; `softness` 0 to 100, 10; `height` -100 to 100, 50;
`cut_min` and `cut_max` 0 to 100, 0 and 100; `light_intensity` 0 to 400, 100; `light_color` a
colour, white; `light_type` `distant` or `point`, `distant`; `light_height` -1000 to 1000, 100;
`light_position` -1000 to 1000 per cent each way, 30, 30; `light_direction` -3600 to 3600, -45;
`ambient_light_color` a colour, white; `ambient` 25, `diffuse` 75, `dust` 0, `specular` 50 and
`metal` 0, each 0 to 100; `roughness` 0.001 to 1, 0.05. Every number keyable. The values when
added are chosen here (the light and shading Blobbylize's but Metal, 0 for plastic); the manual
gives none. Dust's law is this program's choice: the manual names no formula.

**This file never runs the build's code path.** It sums the two-dimensional kernel directly where
the build runs two passes, in double precision from the drawings' 8-bit values.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/plastic/`: the
drawing `holder` at the top with the effect, and under it the map layers, every one switched
off. The expected pixels are in `Fixtures/plastic/expected_plastic.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/plastic_reference.py
"""

import json
import sys
from math import sqrt
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from vector_blur_reference import phase, along  # noqa: E402
from blobbylize_reference import kernel, hex_linear, DRAWINGS, decoded  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "plastic"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
MAPS = ("dots", "ramp", "spot")


# --- the rule -------------------------------------------------------------------------------

def plastic(o, m, s):
    w, h = o["w"], o["h"]
    f = [phase(s["property"], p) for p in m["px"]]
    r, one = kernel(s["softness"] / 2)
    lo, hi = s["cut_min"] / 100, s["cut_max"] / 100

    def bv(x, y):
        if r == 0:
            return f[y * w + x] if 0 <= x < w and 0 <= y < h else 0.0
        return sum(f[j * w + i] * one[i - x + r] * one[j - y + r]
                   for j in range(max(0, y - r), min(h, y + r + 1))
                   for i in range(max(0, x - r), min(w, x + r + 1)))

    def b(x, y):
        v = bv(x, y)
        if hi == lo:
            return 1.0 if v >= lo else 0.0
        return min(max((v - lo) / (hi - lo), 0.0), 1.0)

    k = s["height"] / 100 * 1.25 * max(s["softness"], 1.0)
    lc, ac = hex_linear(s["light_color"]), hex_linear(s["ambient_light_color"])
    inten = s["light_intensity"] / 100
    amb, dif, dust, spc, metal = (s[n] / 100 for n in ("ambient", "diffuse", "dust", "specular",
                                                       "metal"))
    out = []
    for y in range(h):
        for x in range(w):
            p = o["px"][y * w + x]
            a = p[3]
            if a <= 0:
                out.append([0.0] * 4)
                continue
            col = [p[i] / a for i in range(3)]
            gx, gy = (b(x + 1, y) - b(x - 1, y)) / 2, (b(x, y + 1) - b(x, y - 1)) / 2
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
            dusty = nl / (nl + nn[2]) if nl > 0 else 0.0
            rz = 2 * nl * nn[2] - lv[2]
            shine = rz ** (1 / s["roughness"]) if nl > 0 and rz > 0 else 0.0
            lit = [col[i] * (amb * ac[i] + dif * inten * lc[i] * max(nl, 0.0)
                             + dust * inten * lc[i] * dusty)
                   + spc * inten * (lc[i] + (col[i] - lc[i]) * metal) * shine for i in range(3)]
            out.append([v * a for v in lit] + [a])
    return L.pic(w, h, out)


# --- the cases ------------------------------------------------------------------------------

NAMES = ("layer", "fit", "property", "softness", "height", "cut_min", "cut_max",
         "light_intensity", "light_color", "light_type", "light_height", "light_position",
         "light_direction", "ambient_light_color", "ambient", "diffuse", "dust", "specular",
         "roughness", "metal")
FLAT = {"ambient": 100, "diffuse": 0, "specular": 0}  # no light: the layer as it was


def case(holder="shapes", shift=(0, 0), **kw):
    c = {"layer": "", "fit": "stretch", "property": "alpha", "softness": 10, "height": 50,
         "cut_min": 0, "cut_max": 100, "light_intensity": 100, "light_color": "#ffffff",
         "light_type": "distant", "light_height": 100, "light_position": (30, 30),
         "light_direction": -45, "ambient_light_color": "#ffffff", "ambient": 25, "diffuse": 75,
         "dust": 0, "specular": 50, "roughness": 0.05, "metal": 0}
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
    out = plastic(decoded(c["holder"]), the_map(c), settings(c, frame))
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded(c["holder"])
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


SOFT4 = dict(softness=4)
RAMP = dict(holder="photo", layer="ramp", property="luminance", softness=0, height=100)
CASES = {
    "FX-PLASTIC-001": ("As added: the layer's own covering as the bump, Softness 10, Height 50, "
                       "Cut 0 to 100, a white distant light from the top left at height 100, "
                       "Ambient 25, Diffuse 75, Dust 0, Specular 50, Roughness 0.05, Metal 0: "
                       "the two blocks keep their colours and shapes, their edges lit as if "
                       "raised, lighter facing the top left.", case(), (0,)),
    "FX-PLASTIC-002": ("Ambient 100, Diffuse 0, Specular 0, a white ambient light: nothing is "
                       "lit, so the drawing comes back untouched.", case(**FLAT), (0,)),
    "FX-PLASTIC-003": ("Height 0: a flat surface, every covered pixel its colour times Ambient "
                       "plus Diffuse times 0.71, and the faintest highlight.",
                       case(height=0), (0,)),
    "FX-PLASTIC-004": ("Softness 4: a narrower, steeper rim round each block, lighter on the top "
                       "and left edges, darker on the bottom and right.", case(**SOFT4), (0,)),
    "FX-PLASTIC-005": ("The light's Direction 135, from the bottom right: the other edges light.",
                       case(light_direction=135, **SOFT4), (0,)),
    "FX-PLASTIC-006": ("A point light at 25, 30 per cent, 10 pixels up: brightest near (4, 3), "
                       "falling away across the blocks.",
                       case(light_type="point", light_position=(25, 30), light_height=10,
                            **SOFT4), (0,)),
    "FX-PLASTIC-007": ("The light's Height -50, behind the layer: the flat middles face away "
                       "from it, so only Ambient lights them.", case(light_height=-50, **SOFT4),
                       (0,)),
    "FX-PLASTIC-008": ("Specular 100, Roughness 0.5, Metal 0, an orange light, Diffuse 0, light "
                       "Height 40: a broad highlight in the light's own orange on the blue "
                       "block too.", case(specular=100, roughness=0.5, diffuse=0,
                                          light_color="#ff8000", light_height=40, **SOFT4), (0,)),
    "FX-PLASTIC-009": ("The same with Metal 100: the highlight takes each block's own colour, "
                       "so the blue block's shine is blue.",
                       case(specular=100, roughness=0.5, diffuse=0, metal=100,
                            light_color="#ff8000", light_height=40, **SOFT4), (0,)),
    "FX-PLASTIC-010": ("A blue ambient light, Ambient 100, Diffuse 0, Specular 0: every colour "
                       "multiplied by the blue, the red block near black.",
                       case(ambient_light_color="#4080ff", **FLAT), (0,)),
    "FX-PLASTIC-011": ("Dust 100 alone (Ambient, Diffuse and Specular 0), Height 0: a softer "
                       "light than Diffuse's, every covered pixel at 0.41 of its colour where "
                       "Diffuse would give 0.71.",
                       case(dust=100, ambient=0, diffuse=0, specular=0, height=0), (0,)),
    "FX-PLASTIC-012": ("The ramp's luminance as the bump on the photo, Softness 0, Height 100: "
                       "one even slope rising to the right, lit alike in every column but the "
                       "two at the sides.", case(**RAMP), (0,)),
    "FX-PLASTIC-013": ("The same, Cut Min 30 and Cut Max 70: the darkest and brightest columns "
                       "lie flat, the slope steeper between.",
                       case(cut_min=30, cut_max=70, **RAMP), (0,)),
    "FX-PLASTIC-014": ("The same, Cut Min 70 and Cut Max 30: the slope turned inside out, "
                       "falling to the right.", case(cut_min=70, cut_max=30, **RAMP), (0,)),
    "FX-PLASTIC-015": ("Cut Min and Cut Max both 50: a single step where the ramp passes 50 per "
                       "cent, flat either side.", case(cut_min=50, cut_max=50, **RAMP), (0,)),
    "FX-PLASTIC-016": ("The dots layer as the bump, its covering, Softness 2, on the solid "
                       "photo: two round lit bumps where the dots are, the photo whole.",
                       case(holder="photo", layer="dots", softness=2), (0,)),
    "FX-PLASTIC-017": ("The 8 by 5 spot stretched to the drawing's size, Property Red, Softness "
                       "2, on the photo: an oval bump in the middle.",
                       case(holder="photo", layer="spot", property="red", softness=2), (0,)),
    "FX-PLASTIC-018": ("A layer that is not in the composition, `gone`: the layer itself is the "
                       "bump, as FX-PLASTIC-001, and the warning every frame.",
                       case(layer="gone"), (0, 4)),
    "FX-PLASTIC-019": ("Softness keyed from 0 at frame 0 to 8 at frame 4, linear: frame 2 is "
                       "FX-PLASTIC-004.", case(softness=keyed((0, 0), (4, 8))), (0, 2, 4)),
    "FX-PLASTIC-020": ("FX-PLASTIC-004 on the holder moved 2 right and 1 down: the same, moved.",
                       case(shift=(2, 1), **SOFT4), (0,)),
    "FX-PLASTIC-021": ("Cut Min eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                       "overshoots: at frame 2 it would pass 100, is held at 100, and the bump "
                       "is a step at full covering, as at frame 4.",
                       case(cut_min=keyed((0, 0, OVERSHOOT), (4, 100)), **SOFT4), (0, 2, 4)),
    "FX-PLASTIC-022": ("Property Lightness on the photo itself, Softness 2, Height -50: the "
                       "cream squares sunk below the red and blue.",
                       case(holder="photo", property="lightness", softness=2, height=-50), (0,)),
}

INVALID = {
    "FX-PLASTIC-023": ("Softness 101, above 100.", case(softness=101)),
    "FX-PLASTIC-024": ("Height 101, above 100.", case(height=101)),
    "FX-PLASTIC-025": ("Cut Min -1, below 0.", case(cut_min=-1)),
    "FX-PLASTIC-026": ("A property written \"hue\", which Plastic does not read.",
                       case(property="hue")),
    "FX-PLASTIC-027": ("A light type written \"spot\".", case(light_type="spot")),
    "FX-PLASTIC-028": ("Roughness 0, below 0.001.", case(roughness=0)),
    "FX-PLASTIC-029": ("A layer written as the number 3, not a word.", case(layer=3)),
    "FX-PLASTIC-030": ("An ambient light colour written \"blue\", not a colour.",
                       case(ambient_light_color="blue")),
    "FX-PLASTIC-031": ("Dust keyed to 150 at frame 4.", case(dust=keyed((0, 0), (4, 150)))),
}


def effect(c):
    return {"instance_id": "fx-1", "type_id": "core.plastic", "enabled": True,
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
    (OUT / "expected_plastic.json").write_text(json.dumps(expected, indent=1) + "\n",
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
    straight = lambda p: [v / p[3] for v in p[:3]] if p[3] > 0 else [0.0] * 3  # noqa: E731
    lum = lambda p: sum(straight(p))  # noqa: E731
    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(u >= -1e-12 for u in p[:3]), (fx, p)
        if fx in INVALID:
            base = plain(INVALID[fx][1])
            assert all(px == base for px in frames.values())
        elif "2" not in frames and "4" not in frames:
            # Every lit case keeps the holder's covering exactly.
            base = plain(CASES[fx][1])
            assert all(p[3] == q[3] for p, q in zip(frames["0"], base)), fx
    assert near(c["FX-PLASTIC-002"]["0"], drawn)
    red = straight(drawn[at(3, 4)])
    three = c["FX-PLASTIC-003"]["0"]
    lz = 1 / sqrt(2)
    want = [v * (0.25 + 0.75 * lz) + 0.5 * lz ** 20 for v in red]  # Metal 0: a white shine
    assert near([straight(three[at(3, 4)])], [want], 1e-9)
    one, four, five = (c[f"FX-PLASTIC-00{i}"]["0"] for i in (1, 4, 5))
    assert one != drawn and four != one
    # Top left edge lighter than bottom right with the light at the top left; turned round with it.
    assert lum(four[at(1, 2)]) > lum(four[at(5, 7)]) and lum(five[at(1, 2)]) < lum(five[at(5, 7)])
    assert c["FX-PLASTIC-006"]["0"] not in (four, five)
    seven = c["FX-PLASTIC-007"]["0"]
    assert near([straight(seven[at(3, 4)])], [[0.25 * v for v in red]], 1e-6)
    eight, nine = c["FX-PLASTIC-008"]["0"], c["FX-PLASTIC-009"]["0"]
    blue = (10, 4)
    assert eight[at(*blue)][0] > eight[at(*blue)][2]  # plastic: the orange light's shine
    assert nine[at(*blue)][2] > nine[at(*blue)][0]  # metal: the blue block's own
    ten = c["FX-PLASTIC-010"]["0"]
    tint = hex_linear("#4080ff")
    assert near([straight(ten[at(3, 4)])], [[red[i] * tint[i] for i in range(3)]], 1e-9)
    eleven = c["FX-PLASTIC-011"]["0"]
    assert near([straight(eleven[at(3, 4)])], [[v * lz / (lz + 1) for v in red]], 1e-6)
    twelve, thirteen, fourteen, fifteen = (c[f"FX-PLASTIC-0{i}"]["0"] for i in (12, 13, 14, 15))
    # Row 4, away from the top and bottom, where the clear outside tips the edge rows.
    ph = lambda x: straight(photo[at(x, 4)])  # noqa: E731
    ratio = lambda img, x: [straight(img[at(x, 4)])[i] / ph(x)[i] for i in range(3)]  # noqa: E731
    # One even slope: the same light at columns 3 and 9 (their colours differ); cut: flat ends.
    assert near([ratio(twelve, 3)], [ratio(twelve, 9)], 1e-9)
    # A flat pixel: its colour times Ambient plus Diffuse times 0.71, and the white shine.
    flat = lambda x: [v * (0.25 + 0.75 * lz) + 0.5 * lz ** 20 for v in ph(x)]  # noqa: E731
    lit = lambda img, x: straight(img[at(x, 4)])  # noqa: E731
    for x in (1, 14):
        assert near([lit(thirteen, x)], [flat(x)], 1e-9)
    assert not near([lit(thirteen, 7)], [flat(7)], 1e-6)
    # Inside out: the slope faces the other way, so it lights differently from FX-PLASTIC-012.
    assert not near([ratio(twelve, 7)], [ratio(fourteen, 7)], 1e-6)
    for x in (2, 13):
        assert near([lit(fifteen, x)], [flat(x)], 1e-9)
    assert not near([lit(fifteen, 7)], [flat(7)], 1e-6)  # the step, columns 7 and 8
    sixteen = c["FX-PLASTIC-016"]["0"]
    assert sixteen != photo
    seventeen = c["FX-PLASTIC-017"]["0"]
    assert seventeen != photo
    assert c["FX-PLASTIC-018"]["0"] == c["FX-PLASTIC-001"]["0"] == c["FX-PLASTIC-018"]["4"]
    nineteen = c["FX-PLASTIC-019"]
    assert nineteen["2"] == four and nineteen["0"] != four
    twenty = c["FX-PLASTIC-020"]["0"]
    assert all(twenty[at(x, y)] == four[at(x - 2, y - 1)] for x in range(2, W) for y in range(1, H))
    twentyone = c["FX-PLASTIC-021"]
    assert twentyone["2"] == twentyone["4"] and twentyone["0"] != twentyone["2"]
    assert c["FX-PLASTIC-022"]["0"] != photo
    print("checked")


if __name__ == "__main__":
    main()
