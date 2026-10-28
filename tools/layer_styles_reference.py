"""Layer Styles, Inner Shadow and Drop Shadow, worked a second way.

D-922 adds After Effects' first two Layer Styles, `core.style_inner_shadow` and
`core.style_drop_shadow`. Both read the layer's covering after its effects, whatever order the
file lists them in: a style always runs after every effect, Inner Shadows before Drop Shadows,
each kind in its listed order. Both share nine settings, After Effects' rows: `blend_mode`, one
of document 21's layer modes but the two stencils; `color`, `#rrggbb`; `opacity`, 0 to 100;
`use_global_light`, "on" or "off"; `angle`, -3600 to 3600 degrees counter-clockwise from the right,
where the light comes from; `distance`, 0 to 1000 pixels; `choke` (Inner Shadow) or `spread`
(Drop Shadow), 0 to 100 per cent; `size`, 0 to 250 pixels; and `noise`, 0 to 100 per cent. With
`use_global_light` "on" the angle is the composition's Global Light angle, `global_light_angle`
in the file, 120 when it is left out. It is this program's own method, modelled on After Effects'
and Photoshop's styles; nothing is ported. Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

The rule. The shadow moves `distance` pixels away from the light: the offset is
d * (-cos t, sin t) in pixel space, y down, exactly a whole step at a quarter turn. With
p = choke or spread / 100, the covering A is first eroded (Inner Shadow) or dilated (Drop Shadow)
by the disc of whole steps within size * p, the least or the most A under it, transparent outside
the layer, then blurred by document 21's Gaussian at sigma size * (1 - p) / 2. B is that, read
back by document 21's bilinear sample at P - offset, transparent outside.

Inner Shadow: k = (1 - B) * opacity / 100 * (1 - noise / 100 * rnd(x, y)), so outside the shape
it is 1. Its pixel S = (C * k, k), C the colour's linear value, is laid on the layer with
document 21's mode keeping the layer's alpha (D-828's "atop": Co = (1-k)*Cd + k*Ad*B(cs, cd),
Ao = Ad), Add held to 1. Drop Shadow: k = B * opacity / 100 * (1 - noise / 100 * rnd), and the
layer is laid over its shadow (C * k, k), so it shows only where the layer does not cover; it
is drawn so in every mode, a mode but Normal, or Multiply with black (the same pixels), said in a
note on each frame. The layer grows by ceil(distance), the disc's whole steps and the blur's
reach on every side; Inner Shadow never grows it.

rnd(x, y) is a hash of the layer's own whole pixel coordinates, the same every frame:
u = (x * 73856093) xor (y * 19349663), u ^= u >> 13, u *= 0x5bd1e995, u ^= u >> 15, all in 32 bits,
and rnd = (u & 0xffffff) / 2^24.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size: a plate of skin,
with room round it for a drop shadow to show. The drawing goes into `Fixtures/layer_styles/media`,
the projects into `Fixtures/layer_styles`, and the expected frames into
`Fixtures/layer_styles/expected_layer_styles.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/layer_styles_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop, srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
from edges_reference import gaussian  # noqa: E402
from layer_blend_modes_reference import mix, lin, enc  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "layer_styles"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"opacity": (0, 100), "angle": (-3600, 3600), "distance": (0, 1000),
          "spread": (0, 100), "size": (0, 250), "noise": (0, 100)}
EMPTY = [0.0] * 4
QUARTER = {0: (-1.0, 0.0), 90: (0.0, 1.0), 180: (1.0, 0.0), 270: (0.0, -1.0)}


# --- the rule -------------------------------------------------------------------------------

def offset(angle, distance):
    """Away from the light, in pixel space, y down; exact at quarter turns."""
    u = QUARTER.get(angle % 360)
    if u is None:
        t = math.radians(angle)
        u = (-math.cos(t), math.sin(t))
    return distance * u[0], distance * u[1]


def rnd(x, y):
    m = 0xffffffff
    u = ((x * 73856093) & m) ^ ((y * 19349663) & m)
    u ^= u >> 13
    u = (u * 0x5bd1e995) & m
    u ^= u >> 15
    return (u & 0xffffff) / 16777216


def disc(radius):
    r = math.floor(radius)
    return [(dx, dy) for dy in range(-r, r + 1) for dx in range(-r, r + 1)
            if dx * dx + dy * dy <= radius * radius]


def alpha_of(layer, x, y):
    lx, ly = x - layer["left"], y - layer["top"]
    if 0 <= lx < layer["w"] and 0 <= ly < layer["h"]:
        return layer["px"][ly * layer["w"] + lx][3]
    return 0.0


def shaped(layer, radius, grow):
    """The covering eroded (least under the disc, no growth) or, with `grow`, dilated (most,
    grown by the disc's whole steps), transparent outside the layer."""
    steps = disc(radius)
    g = math.floor(radius) if grow else 0
    pick = max if grow else min
    px = []
    for y in range(layer["top"] - g, layer["top"] + layer["h"] + g):
        for x in range(layer["left"] - g, layer["left"] + layer["w"] + g):
            px.append([0.0, 0.0, 0.0, pick(alpha_of(layer, x + dx, y + dy) for dx, dy in steps)])
    return {"px": px, "left": layer["left"] - g, "top": layer["top"] - g,
            "w": layer["w"] + 2 * g, "h": layer["h"] + 2 * g}


def covering(layer, s, inner):
    p = s["spread"] / 100
    return gaussian(shaped(layer, s["size"] * p, not inner), s["size"] * (1 - p) / 2,
                    "transparent")


def unpremultiply(p):
    return [p[c] / p[3] for c in range(3)] if p[3] > 0 else [0.0, 0.0, 0.0]


def blend(mode, cd, cs):
    """Document 21's B(cs, cd) on straight linear colours, Add held to 1."""
    if mode == "multiply":
        return [cs[c] * cd[c] for c in range(3)]
    if mode == "screen":
        return [cs[c] + cd[c] - cs[c] * cd[c] for c in range(3)]
    if mode == "add":
        return [min(1.0, cs[c] + cd[c]) for c in range(3)]
    return [lin(v) for v in mix(mode, [enc(v) for v in cd], [enc(v) for v in cs])]


def kept(mode, src, dst):
    """D-828's mode keeping the layer's alpha."""
    if mode == "normal":
        return [src[c] * dst[3] + dst[c] * (1 - src[3]) for c in range(3)] + [dst[3]]
    b = blend(mode, unpremultiply(dst), unpremultiply(src))
    return [(1 - src[3]) * dst[c] + src[3] * dst[3] * b[c] for c in range(3)] + [dst[3]]


def inner_shadow(layer, s):
    B = covering(layer, s, True)
    ox, oy = offset(s["angle"], s["distance"])
    c = [srgb_to_linear(v / 255) for v in R.hex_color(s["color"].lower())]
    out = []
    for y in range(layer["top"], layer["top"] + layer["h"]):
        for x in range(layer["left"], layer["left"] + layer["w"]):
            k = (1 - bilinear(B, x + 0.5 - ox, y + 0.5 - oy)[3]) * s["opacity"] / 100
            k *= 1 - s["noise"] / 100 * rnd(x, y)
            out.append(kept(s["blend_mode"], [c[0] * k, c[1] * k, c[2] * k, k],
                            layer["px"][(y - layer["top"]) * layer["w"] + x - layer["left"]]))
    return dict(layer, px=out)


def drop_shadow(layer, s):
    B = covering(layer, s, False)
    ox, oy = offset(s["angle"], s["distance"])
    c = [srgb_to_linear(v / 255) for v in R.hex_color(s["color"].lower())]
    p = s["spread"] / 100
    g = math.ceil(s["distance"]) + math.floor(s["size"] * p) + math.ceil(3 * s["size"] * (1 - p) / 2)
    out = []
    for y in range(layer["top"] - g, layer["top"] + layer["h"] + g):
        for x in range(layer["left"] - g, layer["left"] + layer["w"] + g):
            k = bilinear(B, x + 0.5 - ox, y + 0.5 - oy)[3] * s["opacity"] / 100
            k *= 1 - s["noise"] / 100 * rnd(x, y)
            d = pixel(layer, x, y)
            out.append([d[j] + [c[0] * k, c[1] * k, c[2] * k, k][j] * (1 - d[3]) for j in range(4)])
    return {"px": out, "left": layer["left"] - g, "top": layer["top"] - g,
            "w": layer["w"] + 2 * g, "h": layer["h"] + 2 * g}


def pixel(layer, x, y):
    lx, ly = x - layer["left"], y - layer["top"]
    if 0 <= lx < layer["w"] and 0 <= ly < layer["h"]:
        return layer["px"][ly * layer["w"] + lx]
    return EMPTY


# --- the drawing ----------------------------------------------------------------------------

SKIN, NONE = R.SKIN, R.NONE
HALF = SKIN[:3] + (128,)


def plate(x, y):
    """Skin in columns 3 to 11 and rows 2 to 7, and skin at half covering at (12, 4), its
    antialiased edge. The rest is empty."""
    if (x, y) == (12, 4):
        return HALF
    return SKIN if 3 <= x <= 11 and 2 <= y <= 7 else NONE


DRAWINGS = {"plate": [[plate(x, y) for x in range(W)] for y in range(H)]}


def drawn_layer():
    return {"px": [R.working(p) for row in DRAWINGS["plate"] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


def frame(layer):
    return [pixel(layer, x, y) for y in range(H) for x in range(W)]


# --- the cases ------------------------------------------------------------------------------

ORANGE, YELLOW, BLUE = "#ff8000", "#ffff00", "#2040a0"


def style(kind="inner", blend_mode="multiply", color="#000000", opacity=75, use_global_light="on",
          angle=120, distance=5, spread=0, size=5, noise=0):
    return {"kind": kind, "blend_mode": blend_mode, "color": color, "opacity": opacity,
            "use_global_light": use_global_light, "angle": angle, "distance": distance,
            "spread": spread, "size": size, "noise": noise}


def case(*styles, global_light=None):
    return {"styles": list(styles), "global_light": global_light}


def held(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    layer = drawn_layer()
    light = 120 if c["global_light"] is None else c["global_light"]
    order = [s for s in c["styles"] if s["kind"] == "inner"] + \
            [s for s in c["styles"] if s["kind"] == "drop"]
    for s in order:
        now = dict(s, **{k: held(value_at(s[k], frame_no), r) for k, r in RANGES.items()})
        if now["use_global_light"] == "on":
            now["angle"] = light
        layer = (inner_shadow if s["kind"] == "inner" else drop_shadow)(layer, now)
    return frame(layer)


def plain(c):
    return frame(drawn_layer())


NOTE = "PROJECT_FEATURE_UNSUPPORTED"
CASES = {
    "FX-LSTYLE-001": ("Inner Shadow at After Effects' defaults: Multiply, black, 75 per cent, "
                      "Global Light on at the composition's 120 (the file has no angle of its "
                      "own), distance 5, choke 0, size 5. The light comes from the upper left, so "
                      "the plate darkens inside its upper and left edges, softly; its lower right "
                      "corner stays near its own skin; its alpha never changes.",
                      case(style()), [0]),
    "FX-LSTYLE-002": ("Normal, #ff8000 at 100, Global Light off, angle 0 (light from the right), "
                      "distance 2, size 0: the plate's two right-hand columns, 10 and 11, are "
                      "exactly orange; every other skin pixel is unchanged; the half-covered pixel "
                      "at (12, 4) is orange at its own covering.",
                      case(style(blend_mode="normal", color=ORANGE, opacity=100,
                                 use_global_light="off", angle=0, distance=2, size=0)), [0]),
    "FX-LSTYLE-003": ("The tutorial's rim light (w12a 28:54): Linear Dodge (Add), #ff8000 at 50, "
                      "Global Light off, angle 361 (1x+1.0), distance 2, size 2: an orange glow "
                      "added inside the right-hand edge, brighter than the skin, held to 1.",
                      case(style(blend_mode="add", color=ORANGE, opacity=50,
                                 use_global_light="off", angle=361, distance=2, size=2)), [0]),
    "FX-LSTYLE-004": ("w12c's (video 2, 5:23): Color Dodge, #ffff00 at 75, Global Light off, angle "
                      "178, distance 2, size 1: the skin brightened toward yellow inside its left "
                      "edge, on the encoded colours.",
                      case(style(blend_mode="color_dodge", color=YELLOW, opacity=75,
                                 use_global_light="off", angle=178, distance=2, size=1)), [0]),
    "FX-LSTYLE-005": ("Choke 50, size 4, distance 0, Normal black at 100: the covering is eroded "
                      "by the disc of radius 2 and blurred at sigma 1, so a dark band runs inside "
                      "every edge, darkest at the edge.",
                      case(style(blend_mode="normal", opacity=100, use_global_light="off",
                                 distance=0, spread=50, size=4)), [0]),
    "FX-LSTYLE-006": ("Noise 50, Normal black at 100, Global Light off, angle 90 (light from "
                      "above), distance 1, size 0: the plate's top row is shadowed, each pixel by "
                      "between half and all of it, the same grain every time.",
                      case(style(blend_mode="normal", opacity=100, use_global_light="off",
                                 angle=90, distance=1, size=0, noise=50)), [0]),
    "FX-LSTYLE-007": ("Global Light on with the composition's angle 90 in the file, the style's "
                      "own angle 0 left unused: FX-LSTYLE-008 exactly.",
                      case(style(blend_mode="normal", opacity=100, angle=0, distance=2, size=0),
                           global_light=90), [0]),
    "FX-LSTYLE-008": ("Global Light off, angle 90, distance 2, size 0, Normal black at 100: the "
                      "plate's top two rows are black.",
                      case(style(blend_mode="normal", opacity=100, use_global_light="off",
                                 angle=90, distance=2, size=0)), [0]),
    "FX-LSTYLE-009": ("Distance keyed from 0 at frame 0 to 4 at frame 4, linear, angle 0, Normal "
                      "black at 100, size 0: frame 0 is the plate unchanged but for its "
                      "half-covered pixel's outside, frame 2 has two black right-hand columns, "
                      "frame 4 four.",
                      case(style(blend_mode="normal", opacity=100, use_global_light="off",
                                 angle=0, distance=keyed((0, 0), (4, 4)), size=0)), [0, 2, 4]),
    "FX-LSTYLE-010": ("Opacity 0: the plate untouched.",
                      case(style(opacity=0)), [0]),
    "FX-LSTYLE-011": ("Drop Shadow at After Effects' defaults: Multiply, black, 75, Global Light "
                      "on at 120, distance 5, spread 0, size 5: a soft shadow down and to the "
                      "right of the plate, behind it. Multiply with black is Normal's pixels "
                      "exactly, so nothing is noted.",
                      case(style(kind="drop")), [0]),
    "FX-LSTYLE-012": ("Drop Shadow, Normal, #2040a0 at 100, Global Light off, angle 90, "
                      "distance 2, size 0: the plate's shape moved two pixels down, blue, in rows "
                      "8 and 9 below it; the plate itself unchanged.",
                      case(style(kind="drop", blend_mode="normal", color=BLUE, opacity=100,
                                 use_global_light="off", angle=90, distance=2, size=0)), [0]),
    "FX-LSTYLE-013": ("Drop Shadow, spread 50, size 4, distance 0, Normal black at 100: the "
                      "covering dilated by the disc of radius 2 and blurred at sigma 1, a dark "
                      "halo round the plate.",
                      case(style(kind="drop", blend_mode="normal", opacity=100,
                                 use_global_light="off", distance=0, spread=50, size=4)), [0]),
    "FX-LSTYLE-014": ("Both, listed Drop Shadow first: the Inner Shadow is drawn first, from the "
                      "plate's own covering, then the Drop Shadow behind the result.",
                      case(style(kind="drop", blend_mode="normal", opacity=100,
                                 use_global_light="off", angle=90, distance=2, size=0),
                           style(blend_mode="normal", color=ORANGE, opacity=100,
                                 use_global_light="off", angle=0, distance=2, size=0)), [0]),
    "FX-LSTYLE-015": ("Drop Shadow in Screen, white at 100: drawn as Normal's would be, behind the "
                      "plate, with a note on each frame that its mode is not mixed with what is "
                      "beneath.",
                      case(style(kind="drop", blend_mode="screen", color="#ffffff", opacity=100,
                                 use_global_light="off", angle=90, distance=2, size=0)), [0]),
}
FRAME_NOTES = {"FX-LSTYLE-015": NOTE}

# Outside the contract in a file. D-46: the file is read, the style is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-LSTYLE-016": ("Opacity 101, above 100.", case(style(opacity=101))),
    "FX-LSTYLE-017": ("Size 251, above 250.", case(style(size=251))),
    "FX-LSTYLE-018": ("Choke 101, above 100.", case(style(spread=101))),
    "FX-LSTYLE-019": ("Noise -1, below 0.", case(style(noise=-1))),
    "FX-LSTYLE-020": ("A colour written \"black\".", case(style(color="black"))),
    "FX-LSTYLE-021": ("Use Global Light written \"yes\".", case(style(use_global_light="yes"))),
    "FX-LSTYLE-022": ("Blend mode \"dissolve\", which this program does not draw.",
                      case(style(blend_mode="dissolve"))),
    "FX-LSTYLE-023": ("Blend mode \"stencil_alpha\", a stencil, not a style's mode.",
                      case(style(kind="drop", blend_mode="stencil_alpha"))),
}


# --- the project files ----------------------------------------------------------------------

def style_json(i, s):
    inner = s["kind"] == "inner"
    p = {"blend_mode": s["blend_mode"], "color": s["color"], "opacity": setting_json(s["opacity"]),
         "use_global_light": s["use_global_light"], "angle": setting_json(s["angle"]),
         "distance": setting_json(s["distance"]),
         ("choke" if inner else "spread"): setting_json(s["spread"]),
         "size": setting_json(s["size"]), "noise": setting_json(s["noise"])}
    return {"instance_id": f"fx-0-{i}",
            "type_id": "core.style_inner_shadow" if inner else "core.style_drop_shadow",
            "enabled": True, "parameters": p}


def project_json(fx, c):
    p = S.project_json(fx, {"drawing": "plate", "shift": 0, "softness": 0, "threshold": 0})
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2, H / 2])
    comp["layers"][0]["effects"] = [style_json(i, s) for i, s in enumerate(c["styles"])]
    if c["global_light"] is not None:
        comp["global_light_angle"] = c["global_light"]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "plate.png").write_bytes(S.png(DRAWINGS["plate"]))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        if fx in FRAME_NOTES:
            expected["cases"][fx]["frame_warning"] = FRAME_NOTES[fx]
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the style is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_layer_styles.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(None)
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for p, q in zip(a, b))  # noqa: E731
    orange = [srgb_to_linear(v / 255) for v in R.hex_color(ORANGE)]

    assert offset(120, 2) == (2 * -math.cos(math.radians(120)), 2 * math.sin(math.radians(120)))
    assert offset(0, 2) == (-2.0, 0.0) and offset(-270, 1) == (0.0, 1.0)
    assert disc(0) == [(0, 0)] and len(disc(2)) == 13
    assert 0 <= rnd(-3, 7) < 1 and rnd(5, 5) == rnd(5, 5)

    for name, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert -1e-12 <= p[3] <= 1 + 1e-12, (name, p)
                inner_only = name not in ("FX-LSTYLE-011", "FX-LSTYLE-012", "FX-LSTYLE-013",
                                          "FX-LSTYLE-014", "FX-LSTYLE-015")
                if inner_only:
                    assert p[3] == drawn[i][3], name  # an Inner Shadow keeps the alpha

    first = c["FX-LSTYLE-001"]["0"]
    assert first[at(3, 2)][0] < first[at(11, 7)][0] < drawn[at(11, 7)][0] + 1e-12
    two = c["FX-LSTYLE-002"]["0"]
    for y in range(2, 8):
        for x in range(3, 12):
            want = orange + [1.0] if x >= 10 else drawn[at(x, y)]
            if (x, y) == (10, 4):  # its light is read at (12, 4), half covered
                k = 1 - 128 / 255
                want = [orange[i] * k + drawn[at(x, y)][i] * (1 - k) for i in range(3)] + [1.0]
            assert near(two[at(x, y)], want), (x, y)
    assert near(two[at(12, 4)], [v * 128 / 255 for v in orange] + [128 / 255])
    three = c["FX-LSTYLE-003"]["0"]
    assert three[at(11, 4)][0] > drawn[at(11, 4)][0] and three[at(11, 4)][0] <= 1
    four = c["FX-LSTYLE-004"]["0"]
    assert four[at(3, 4)][1] > drawn[at(3, 4)][1] + 0.05 and four[at(3, 4)][2] == drawn[at(3, 4)][2]
    assert near(four[at(7, 4)], drawn[at(7, 4)], 1e-3)  # yellow has no blue to dodge with
    five = c["FX-LSTYLE-005"]["0"]
    assert five[at(3, 4)][0] < five[at(4, 4)][0] < five[at(6, 4)][0]
    six = c["FX-LSTYLE-006"]["0"]
    assert len({round(six[at(x, 2)][0], 9) for x in range(3, 12)}) > 3
    assert all(six[at(x, 3)] == drawn[at(x, 3)] for x in range(3, 12))
    assert c["FX-LSTYLE-007"]["0"] == c["FX-LSTYLE-008"]["0"]
    eight = c["FX-LSTYLE-008"]["0"]
    assert all(near(eight[at(x, y)], [0, 0, 0, 1]) for x in range(3, 12) for y in (2, 3))
    assert all(eight[at(x, 4)] == drawn[at(x, 4)] for x in range(3, 12))
    nine = c["FX-LSTYLE-009"]
    assert all(nine["0"][i] == drawn[i] for i in range(W * H) if i != at(12, 4))
    assert all(near(nine["2"][at(x, 5)], [0, 0, 0, 1]) == (x >= 10) for x in range(3, 12))
    assert all(near(nine["4"][at(x, 5)], [0, 0, 0, 1]) == (x >= 8) for x in range(3, 12))
    assert c["FX-LSTYLE-010"]["0"] == drawn
    eleven = c["FX-LSTYLE-011"]["0"]
    assert eleven[at(13, 9)][3] > 0.3 > 0.01 > eleven[at(1, 0)][3]
    assert all(eleven[i] == drawn[i] for i in range(W * H) if drawn[i][3] == 1)
    twelve = c["FX-LSTYLE-012"]["0"]
    blue = [srgb_to_linear(v / 255) for v in R.hex_color(BLUE)]
    assert all(near(twelve[at(x, y)], blue + [1.0]) for x in range(3, 12) for y in (8, 9))
    assert twelve[at(3, 1)] == EMPTY
    thirteen = c["FX-LSTYLE-013"]["0"]
    assert thirteen[at(2, 4)][3] > thirteen[at(1, 4)][3] > thirteen[at(0, 0)][3]
    fourteen = c["FX-LSTYLE-014"]["0"]
    assert near(fourteen[at(11, 4)], orange + [1.0]) and near(fourteen[at(5, 9)], [0, 0, 0, 1])
    assert c["FX-LSTYLE-015"]["0"][at(5, 9)] == [1.0, 1.0, 1.0, 1.0]
    print("checked")


if __name__ == "__main__":
    main()
