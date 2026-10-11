"""CC Glass, worked a second way.

D-317 added `core.glass`, After Effects' CC Glass (Stylize), reduced: a bump map bends the
picture and lights it from one side with Bevel Alpha's light. D-561 gives it the rest of CC
Glass's controls: Light Type (Distant or Point), Light Height, Light Position, and the shading,
Ambient, Diffuse, Specular, Roughness and Metal. What is known of CC Glass (CycoreFX's manual
names the controls; Adobe's own pages could not be read, every request refused) is: the Bump Map,
the layer itself at first, and its Property make a height map; Softness smooths it; Height sets
how steep it is; Displacement how far its slopes bend the picture; the light is the effect's own
(After Effects' light layers are not offered here), with Intensity, Color, Type, Height, Position
and Direction; the shading's Ambient, Diffuse, Specular, Roughness and Metal are as in CC
Blobbylize and CC Plastic. The lighting rule is Blobbylize's (D-379), as Plastic's (D-463) is,
and nothing is ported. The starting values are from memory of the effect's panel and could not
be checked against Adobe's pages, so they are this program's choice.

The rule, at the holder's frame, with its input O, linear and premultiplied:

1. The bump is D-317's: at each pixel Colorama's phase of `property` (intensity, luminance, red,
   green, blue or alpha) of q, the drawing's own pixel or the map's under it (D-189's layer
   setting, fitted to the drawing and lying on it, clear outside it), times q's covering unless
   the property is alpha; blurred by document 21's Gaussian at sigma `softness / 2` (none at 0),
   nothing outside the drawing, 0 beyond the blur's reach.
2. The slope n = k (gx, gy), g the bump's central differences, k = `height / 100` 1.25
   max(softness, 1).
3. Each pixel is O read bilinearly (document 21's sampler, clear outside) at its centre plus
   `displacement` n; call it P.
4. The light, by `light_type`:
   - `classic`, D-317's light, which a file written before D-561 (no `light_type`) keeps: with
     Bevel Alpha's u at `light_direction` and s = clamp(-(n . u), -1, 1), Bevel Alpha's light
     with I = `light_intensity / 100`: P + (Lc P.a - P) I s where s > 0, else P (1 + I s). At
     height 0, or with no displacement and no light, the output is O.
   - `distant` or `point`, Blobbylize's: N is (-nx, -ny, 1) made one long; Lv, toward the light,
     made one long, is (100 ux, 100 uy, `light_height`) for `distant`, or for `point` the light at
     `light_position` (per cent of the drawing's width and height) and `light_height` pixels
     above, Lv = (Lx - px, Ly - py, light_height) from the pixel's centre; at no length, none.
     nl = N.Lv; R = 2 nl N - Lv; the highlight Rz^(1 / roughness) where nl and Rz are above 0,
     else 0. With C = P's straight colour (0 where it is clear), I = `light_intensity / 100`, Lc
     the light colour in linear light and ambient, diffuse, specular and metal as shares of 100:
     each channel out = C (ambient + diffuse I Lc max(nl, 0)) + specular I (Lc + (C - Lc) metal)
     highlight, times P's covering, with that covering.
5. The layer does not grow. A draft scales `softness`, `displacement` and a point light's height.

`layer` D-189's word, ""; `fit` `stretch`, `center` or `tile`, `stretch`; `property` intensity,
luminance, red, green, blue or alpha, `alpha`; `softness` 0 to 100, 10; `height` -100 to 100, 50;
`displacement` -500 to 500, 100; `light_angle` (Light Direction) -3600 to 3600, -45; `light_color`
a colour, white; `light_intensity` 0 to 100, 100; `light_type` `classic`, `distant` or `point`,
`distant` when added and `classic` when the file has none; `light_height` -1000 to 1000, 25;
`light_position` -1000 to 1000 per cent each way, 50, 50; `ambient` 75, `diffuse` 90, `specular`
25 and `metal` 100, each 0 to 100; `roughness` 0.001 to 1, 0.1. Every number keyable. A file
without `light_type` is read as `classic`, the other seven new settings at their starting values;
a file with `light_type` must have all seven.

**This file never runs the build's code path.** It sums the two-dimensional kernel directly where
the build runs two passes, in double precision from the drawings' 8-bit values.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/glass/`: the
drawing `holder` at the top with the effect, and under it the map layers, every one switched
off. The expected pixels are in `Fixtures/glass/expected_glass.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/glass_reference.py
"""

import json
import sys
from math import sqrt
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from vector_blur_reference import phase as vphase, along  # noqa: E402
from blobbylize_reference import kernel, hex_linear, DRAWINGS, decoded  # noqa: E402
from bevel_reference import shade  # noqa: E402
import smooth_reference as S  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "glass"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
MAPS = ("dots", "ramp", "spot")


# --- the rule -------------------------------------------------------------------------------

def bump_phase(prop, p):
    """Step 1 before the blur: CC Vector Blur's read but for intensity, Colorama's mean of the
    encoded channels, times the covering."""
    if prop != "intensity":
        return vphase(prop, p)
    a = p[3]
    if a <= 0:
        return 0.0
    return sum(S.linear_to_srgb(min(max(p[c] / a, 0.0), 1.0)) for c in range(3)) / 3 * a


def glass(o, m, s):
    w, h = o["w"], o["h"]
    f = [bump_phase(s["property"], p) for p in m["px"]]
    r, one = kernel(s["softness"] / 2)

    def bv(x, y):
        if r == 0:
            return f[y * w + x] if 0 <= x < w and 0 <= y < h else 0.0
        return sum(f[j * w + i] * one[i - x + r] * one[j - y + r]
                   for j in range(max(0, y - r), min(h, y + r + 1))
                   for i in range(max(0, x - r), min(w, x + r + 1)))

    k = s["height"] / 100 * 1.25 * max(s["softness"], 1.0)
    lc = hex_linear(s["light_color"])
    inten = s["light_intensity"] / 100
    kind = s["light_type"]
    if kind == "classic" and (s["height"] == 0 or (s["displacement"] == 0 and inten <= 0)):
        return o
    ux, uy = along(s["light_angle"])
    amb, dif, spc, metal = (s[n] / 100 for n in ("ambient", "diffuse", "specular", "metal"))
    out = []
    for y in range(h):
        for x in range(w):
            nx, ny = (bv(x + 1, y) - bv(x - 1, y)) / 2 * k, (bv(x, y + 1) - bv(x, y - 1)) / 2 * k
            d = s["displacement"]
            p = L.bilinear(o, x + 0.5 + d * nx, y + 0.5 + d * ny) if d != 0 else list(L.at(o, x, y))
            a = p[3]
            if kind == "classic":
                if inten > 0:
                    p = shade(p, min(max(-(nx * ux + ny * uy), -1.0), 1.0), lc, inten)
                out.append(list(p))
                continue
            if a <= 0:
                out.append([0.0] * 4)
                continue
            col = [p[i] / a for i in range(3)]
            nn = [-nx, -ny, 1.0]
            ln = sqrt(sum(v * v for v in nn))
            nn = [v / ln for v in nn]
            if kind == "point":
                lx, ly = s["light_position"][0] / 100 * w, s["light_position"][1] / 100 * h
                lv = [lx - (x + 0.5), ly - (y + 0.5), s["light_height"]]
            else:
                lv = [100 * ux, 100 * uy, s["light_height"]]
            ll = sqrt(sum(v * v for v in lv))
            lv = [v / ll for v in lv] if ll > 0 else [0.0] * 3
            nl = sum(nn[i] * lv[i] for i in range(3))
            rz = 2 * nl * nn[2] - lv[2]
            shine = rz ** (1 / s["roughness"]) if nl > 0 and rz > 0 else 0.0
            lit = [col[i] * (amb + dif * inten * lc[i] * max(nl, 0.0))
                   + spc * inten * (lc[i] + (col[i] - lc[i]) * metal) * shine for i in range(3)]
            out.append([v * a for v in lit] + [a])
    return L.pic(w, h, out)


# --- the cases ------------------------------------------------------------------------------

OLD = ("layer", "fit", "property", "softness", "height", "displacement", "light_angle",
       "light_color", "light_intensity")
NEW = ("light_type", "light_height", "light_position", "ambient", "diffuse", "specular",
       "roughness", "metal")
NAMES = OLD + NEW
FLAT = {"ambient": 100, "diffuse": 0, "specular": 0, "displacement": 0}  # the layer as it was


def case(holder="shapes", shift=(0, 0), old=False, **kw):
    c = {"layer": "", "fit": "stretch", "property": "alpha", "softness": 10, "height": 50,
         "displacement": 100, "light_angle": -45, "light_color": "#ffffff",
         "light_intensity": 100, "light_type": "distant", "light_height": 25,
         "light_position": (50, 50), "ambient": 75, "diffuse": 90, "specular": 25,
         "roughness": 0.1, "metal": 100}
    if old:
        c["light_type"] = "classic"  # how the build reads a file without it
    c.update(kw)
    c["holder"], c["shift"], c["old"] = holder, shift, old
    return c


def the_map(c):
    if c["layer"] in MAPS:
        return L.fit(decoded(c["layer"]), c["fit"], W, H)
    return decoded(c["holder"])  # "" and a layer not there: the layer itself


RANGES = {"softness": (0, 100), "height": (-100, 100), "displacement": (-500, 500),
          "light_angle": (-3600, 3600), "light_intensity": (0, 100),
          "light_height": (-1000, 1000), "ambient": (0, 100), "diffuse": (0, 100),
          "specular": (0, 100), "roughness": (0.001, 1), "metal": (0, 100)}


def settings(c, frame):
    """Each setting at the frame, a keyed number held to its range as the build holds it."""
    s = {k: value_at(c[k], frame) for k in NAMES}
    for k, (lo, hi) in RANGES.items():
        s[k] = min(hi, max(lo, s[k]))
    return s


def render(c, frame):
    out = glass(decoded(c["holder"]), the_map(c), settings(c, frame))
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded(c["holder"])
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


STILL = dict(displacement=0, softness=4)  # no bending, a narrow rim: the light alone
RAMP = dict(holder="photo", layer="ramp", property="luminance", softness=0, height=100,
            displacement=3)
CASES = {
    "FX-GLASS-001": ("As added: the layer's own covering as the bump, Softness 10, Height 50, "
                     "Displacement 100, a white distant light from the top left at height 25, "
                     "Ambient 75, Diffuse 90, Specular 25, Roughness 0.1, Metal 100: the two "
                     "blocks bent outward by their soft edges and lit, lighter toward the top "
                     "left.", case(), (0,)),
    "FX-GLASS-002": ("A file written before D-561, without the eight new settings, Displacement "
                     "20 and Light Intensity 70: read as the classic light, so it draws as CC "
                     "Glass drew before, D-317's bend and Bevel Alpha's light.",
                     case(old=True, displacement=20, light_intensity=70), (0,)),
    "FX-GLASS-003": ("The same settings with the light type written `classic`: the same as "
                     "FX-GLASS-002.", case(light_type="classic", displacement=20,
                                           light_intensity=70), (0,)),
    "FX-GLASS-004": ("Ambient 100, Diffuse 0, Specular 0, Displacement 0: nothing is bent or "
                     "lit, so the drawing comes back untouched.", case(**FLAT), (0,)),
    "FX-GLASS-005": ("Height 0: a flat surface, nothing bent, every covered pixel its colour "
                     "times Ambient plus Diffuse times 0.24, and the faintest highlight in its "
                     "own colour.", case(height=0), (0,)),
    "FX-GLASS-006": ("Softness 4, Displacement 0: the light alone, lighter on the top and left "
                     "edges of each block, darker on the bottom and right.", case(**STILL), (0,)),
    "FX-GLASS-007": ("The light's Direction 135, from the bottom right: the other edges light.",
                     case(light_angle=135, **STILL), (0,)),
    "FX-GLASS-008": ("A point light at 25, 30 per cent, 10 pixels up: brightest near (4, 3), "
                     "falling away across the blocks.",
                     case(light_type="point", light_position=(25, 30), light_height=10,
                          **STILL), (0,)),
    "FX-GLASS-009": ("The light's Height -50, behind the layer: the flat middles face away from "
                     "it, so only Ambient lights them, at 0.75 of their colour.",
                     case(light_height=-50, **STILL), (0,)),
    "FX-GLASS-010": ("Specular 100, Roughness 0.5, Metal 0, an orange light, Ambient and "
                     "Diffuse 0, light Height 40: a broad highlight in the light's own orange on "
                     "the blue block too.", case(specular=100, roughness=0.5, ambient=0, diffuse=0, metal=0,
                                  light_color="#ff8000", light_height=40, **STILL), (0,)),
    "FX-GLASS-011": ("The same with Metal 100: the highlight takes each block's own colour, so "
                     "the blue block's shine is blue.",
                     case(specular=100, roughness=0.5, ambient=0, diffuse=0, metal=100,
                          light_color="#ff8000", light_height=40, **STILL), (0,)),
    "FX-GLASS-012": ("Light Intensity 0, Displacement 20: the blocks bent, every covered pixel "
                     "at Ambient, 0.75, of the colour bent onto it.",
                     case(light_intensity=0, displacement=20), (0,)),
    "FX-GLASS-013": ("The ramp's luminance as the bump on the photo, Softness 0, Height 100, "
                     "Displacement 3: the squares pushed sideways along the even slope, lit "
                     "alike.", case(**RAMP), (0,)),
    "FX-GLASS-014": ("The dots layer as the bump, its covering, Softness 2, Displacement 10, on "
                     "the solid photo: the photo bent and lit round two squares.",
                     case(holder="photo", layer="dots", softness=2, displacement=10), (0,)),
    "FX-GLASS-015": ("The 8 by 5 spot stretched to the drawing's size, Property Red, Softness "
                     "2, Displacement 10, on the photo: an oval lens in the middle.",
                     case(holder="photo", layer="spot", property="red", softness=2,
                          displacement=10), (0,)),
    "FX-GLASS-016": ("Property Intensity on the photo itself, Softness 2, Height -50, "
                     "Displacement 10: the cream squares sunk below the red and blue.",
                     case(holder="photo", property="intensity", softness=2, height=-50,
                          displacement=10), (0,)),
    "FX-GLASS-017": ("A layer that is not in the composition, `gone`: the layer itself is the "
                     "bump, as FX-GLASS-001, and the warning every frame.",
                     case(layer="gone"), (0, 4)),
    "FX-GLASS-018": ("Softness keyed from 0 at frame 0 to 8 at frame 4, linear, Displacement 0: "
                     "frame 2 is FX-GLASS-006.",
                     case(softness=keyed((0, 0), (4, 8)), displacement=0), (0, 2, 4)),
    "FX-GLASS-019": ("FX-GLASS-006 on the holder moved 2 right and 1 down: the same, moved.",
                     case(shift=(2, 1), **STILL), (0,)),
    "FX-GLASS-020": ("Ambient eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                     "overshoots: at frame 2 it would pass 100, is held at 100, as at frame 4.",
                     case(ambient=keyed((0, 0, OVERSHOOT), (4, 100)), **STILL), (0, 2, 4)),
}

INVALID = {
    "FX-GLASS-021": ("A light type written \"spot\".", case(light_type="spot")),
    "FX-GLASS-022": ("Roughness 0, below 0.001.", case(roughness=0)),
    "FX-GLASS-023": ("Ambient 101, above 100.", case(ambient=101)),
    "FX-GLASS-024": ("Light Height 1001, above 1000.", case(light_height=1001)),
    "FX-GLASS-025": ("Specular -1, below 0.", case(specular=-1)),
    "FX-GLASS-026": ("Light Position 50, -1001, beyond -1000.",
                     case(light_position=(50, -1001))),
    "FX-GLASS-027": ("Metal keyed to 150 at frame 4.", case(metal=keyed((0, 0), (4, 150)))),
}


def effect(c):
    names = OLD if c["old"] else NAMES
    return {"instance_id": "fx-1", "type_id": "core.glass", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in names}}


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
    (OUT / "expected_glass.json").write_text(json.dumps(expected, indent=1) + "\n",
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
    one = c["FX-GLASS-001"]["0"]
    assert one != drawn
    assert c["FX-GLASS-002"]["0"] == c["FX-GLASS-003"]["0"] != drawn
    assert c["FX-GLASS-002"]["0"] != c["FX-GLASS-001"]["0"]
    assert near(c["FX-GLASS-004"]["0"], drawn)
    red = straight(drawn[at(3, 4)])
    lz = 25 / sqrt(100 ** 2 + 25 ** 2)
    want = [v * (0.75 + 0.9 * lz) + 0.25 * v * lz ** 10 for v in red]  # Metal 100: its own shine
    assert near([straight(c["FX-GLASS-005"]["0"][at(3, 4)])], [want], 1e-9)
    six, seven = c["FX-GLASS-006"]["0"], c["FX-GLASS-007"]["0"]
    # Top left edge lighter than bottom right with the light at the top left; turned round with it.
    assert lum(six[at(1, 2)]) > lum(six[at(5, 7)]) and lum(seven[at(1, 2)]) < lum(seven[at(5, 7)])
    assert all(p[3] == q[3] for p, q in zip(six, drawn))  # no displacement: the covering kept
    assert c["FX-GLASS-008"]["0"] not in (six, seven)
    nine = c["FX-GLASS-009"]["0"]
    assert near([straight(nine[at(3, 4)])], [[0.75 * v for v in red]], 1e-6)
    ten, eleven = c["FX-GLASS-010"]["0"], c["FX-GLASS-011"]["0"]
    blue = (10, 4)
    assert ten[at(*blue)][0] > ten[at(*blue)][2]  # plastic: the orange light's shine
    assert eleven[at(*blue)][2] > eleven[at(*blue)][0]  # metal: the blue block's own
    twelve = c["FX-GLASS-012"]["0"]
    bent = render(case(light_intensity=0, displacement=20, ambient=100), 0)
    assert near([straight(p) for p in twelve], [[0.75 * v for v in straight(q)] for q in bent], 1e-9)
    assert bent != drawn  # the bend itself moved the blocks
    for fx in ("FX-GLASS-013", "FX-GLASS-014", "FX-GLASS-015", "FX-GLASS-016"):
        assert c[fx]["0"] != photo, fx
    assert c["FX-GLASS-017"]["0"] == one == c["FX-GLASS-017"]["4"]
    eighteen = c["FX-GLASS-018"]
    assert eighteen["2"] == six and eighteen["0"] != six
    nineteen = c["FX-GLASS-019"]["0"]
    assert all(nineteen[at(x, y)] == six[at(x - 2, y - 1)] for x in range(2, W) for y in range(1, H))
    twenty = c["FX-GLASS-020"]
    assert twenty["2"] == twenty["4"] and twenty["0"] != twenty["2"]
    print("checked")


if __name__ == "__main__":
    main()
