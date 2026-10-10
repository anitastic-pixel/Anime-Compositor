"""Transform, worked a second way.

D-408 adds `core.transform`, After Effects' Transform effect: the layer's anchor point, position,
scale, skew, rotation and opacity again, as an effect, inside the layer, with its own motion blur.
Adobe's page on the Distort effects says the settings "function much like the layer transform
properties", that the effect "is relative to the input layer", and that its shutter angle is only
"relevant" when "motion blur [is] enabled for the layer and composition". It says no more about
Skew than "Skew amount" and "The axis about which skew occurs". The matrix and the skew are those
of Skia's Skottie, which plays After Effects' exported animations: modules/skottie/src/Transform.cpp
(TransformAdapter2D::totalMatrix, skew held to After Effects' limit of 85 degrees) and
modules/skottie/src/effects/TransformEffect.cpp (in uniform mode the scale is Scale Height alone;
opacity a multiplier). Nothing is ported: the rule is in words in document 21 and worked here.

The rule. W and H are the drawing's own size and o its place in the input buffer, which an earlier
effect may have grown. Points are in per cent of the drawing's own width and height, so A = (a_x /
100 W, a_y / 100 H) is the anchor point and P likewise the position, both in the drawing's own
space; at their starting place, the middle, with scale 100, skew 0, rotation 0 and opacity 100,
the output is the input. With Uniform Scale on, the scale across is Scale Height, Scale Width
being kept but not read. The drawing's point p goes to

    M p = T(P) R(rotation) K(skew, skew axis) S(scale width / 100, scale height / 100) T(-A) p

where R(r) turns clockwise on the screen (y down), [[cos r, -sin r], [sin r, cos r]], and K is a
shear along the skew axis: R(axis) [[1, -tan(skew)], [0, 1]] R(-axis) (no shear at skew 0). The
output is the input's size and place: the layer never grows, so what the effect moves off it is
cut off, as in After Effects. The output at a pixel with centre q, in the drawing's own space, is
the input sampled at M^-1 q, clear when M cannot be undone (a scale of 0), then each of its four
numbers times opacity / 100.

Sampling "bilinear" is document 21's bilinear sample, transparent outside the input. "bicubic"
is the 4 by 4 cubic convolution of Keys with a = -0.5 (Catmull-Rom), from pixel centres,
transparent outside the input; its lobes can go below 0 or a covering above 1, so after it each
number is held at 0 or above and the covering at 1 or below. At a whole-pixel place both give the
pixel itself.

Motion blur. Only when the layer's motion blur switch is on and the composition's motion blur is
enabled (D-188's shutter). With Use Composition's Shutter Angle on, the moments are the
composition's (its angle, phase and samples); off, the effect's own Shutter Angle, centred on the
frame (phase minus half the angle), with the composition's samples; an angle of 0 is no blur. The
settings are read at each moment (keys at that moment, as D-188 reads a layer's), the drawing is
transformed at each, and the pictures are summed in order and divided by their number once. The
layer's own motion blur, of its own transform, is drawn after, as it always is.

Settings: `anchor_point` and `position` [x, y], -1000 to 1000 per cent, [50, 50]; `uniform_scale`
"on" or "off", "on"; `scale_height` and `scale_width` -10000 to 10000 per cent, 100;
`skew` -85 to 85 degrees, 0; `skew_axis` and `rotation` -3600 to 3600 degrees, 0; `opacity` 0 to
100, 100; `use_composition_shutter_angle` "on" or "off", "on"; `shutter_angle` 0 to 360 degrees,
0; `sampling` "bilinear" or "bicubic", "bilinear". The numbers are keyable, the words not.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, and undoes M by a general 3 by 3 inverse, where the build works
in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says: Wave Warp's stripes, imported from `tools/wave_warp_reference.py`. The drawing goes into
`Fixtures/transform/media`, the projects into `Fixtures/transform`, and the expected frames into
`Fixtures/transform/expected_transform.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/transform_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from motion_blur_reference import sample_times, shutter  # noqa: E402
from wave_warp_reference import DRAWINGS  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "transform"
TOLERANCE = 2e-5  # document 25's default for a filter
EMPTY = [0.0] * 4
POINTS = ("anchor_point", "position")
NUMBERS = ("scale_height", "scale_width", "skew", "skew_axis", "rotation", "opacity")
WORDS = ("uniform_scale", "use_composition_shutter_angle", "sampling")
RANGES = {"anchor_point": (-1000, 1000), "position": (-1000, 1000),
          "scale_height": (-10000, 10000), "scale_width": (-10000, 10000), "skew": (-85, 85),
          "skew_axis": (-3600, 3600), "rotation": (-3600, 3600), "opacity": (0, 100),
          "shutter_angle": (0, 360)}
KEYS = ("anchor_point", "position", "uniform_scale", "scale_height", "scale_width", "skew",
        "skew_axis", "rotation", "opacity", "use_composition_shutter_angle", "shutter_angle",
        "sampling")
ON = shutter(180, -90, 4)  # the composition's shutter in the blurred cases


# --- the rule -------------------------------------------------------------------------------

def mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def translate(x, y):
    return [[1, 0, x], [0, 1, y], [0, 0, 1]]


def turn(deg):
    r = math.radians(deg)
    return [[math.cos(r), -math.sin(r), 0], [math.sin(r), math.cos(r), 0], [0, 0, 1]]


def matrix(s, dw, dh):
    """M, the drawing's point to its new place, both in the drawing's own space."""
    ax, ay = s["anchor_point"][0] / 100 * dw, s["anchor_point"][1] / 100 * dh
    px, py = s["position"][0] / 100 * dw, s["position"][1] / 100 * dh
    sy = s["scale_height"] / 100
    sx = sy if s["uniform_scale"] == "on" else s["scale_width"] / 100
    shear = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    if s["skew"] != 0:
        shear = mul(mul(turn(s["skew_axis"]), [[1, -math.tan(math.radians(s["skew"])), 0],
                                               [0, 1, 0], [0, 0, 1]]), turn(-s["skew_axis"]))
    m = translate(px, py)
    for step in (turn(s["rotation"]), shear, [[sx, 0, 0], [0, sy, 0], [0, 0, 1]],
                 translate(-ax, -ay)):
        m = mul(m, step)
    return m


def inverse(m):
    """The general 3 by 3 inverse, or None when m cannot be undone."""
    (a, b, c), (d, e, f), (g, h, i) = m
    det = a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    if det == 0 or not math.isfinite(det):
        return None
    adj = [[e * i - f * h, c * h - b * i, b * f - c * e],
           [f * g - d * i, a * i - c * g, c * d - a * f],
           [d * h - e * g, b * g - a * h, a * e - b * d]]
    return [[v / det for v in row] for row in adj]


def keys_weight(d, a=-0.5):
    d = abs(d)
    if d <= 1:
        return (a + 2) * d ** 3 - (a + 3) * d ** 2 + 1
    if d < 2:
        return a * d ** 3 - 5 * a * d ** 2 + 8 * a * d - 4 * a
    return 0.0


def bicubic(layer, x, y):
    fx, fy = x - 0.5, y - 0.5
    x0, y0 = math.floor(fx), math.floor(fy)
    out = [0.0] * 4
    for j in range(-1, 3):
        wy = keys_weight(fy - (y0 + j))
        for i in range(-1, 3):
            wx = keys_weight(fx - (x0 + i))
            sx, sy = x0 + i - layer["left"], y0 + j - layer["top"]
            if 0 <= sx < layer["w"] and 0 <= sy < layer["h"] and wx * wy:
                p = layer["px"][sy * layer["w"] + sx]
                for c in range(4):
                    out[c] += p[c] * wx * wy
    out = [max(0.0, v) for v in out]
    out[3] = min(1.0, out[3])
    return out


def drawn_once(layer, s, dw, dh):
    inv = inverse(matrix(s, dw, dh))
    k = s["opacity"] / 100
    sample = bicubic if s["sampling"] == "bicubic" else bilinear
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            if inv is None:
                px.append(EMPTY)
                continue
            qx, qy = layer["left"] + i + 0.5, layer["top"] + j + 0.5
            sx = inv[0][0] * qx + inv[0][1] * qy + inv[0][2]
            sy = inv[1][0] * qx + inv[1][1] * qy + inv[1][2]
            px.append([v * k for v in sample(layer, sx, sy)])
    return dict(layer, px=px)


def transform(layer, moments, dw, dh):
    """The layer drawn at each moment's settings, summed in order and divided once."""
    pictures = [drawn_once(layer, s, dw, dh) for s in moments]
    n = len(pictures)
    px = [[sum(p["px"][i][c] for p in pictures) / n for c in range(4)]
          for i in range(len(layer["px"]))]
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, tile=False, blur=None, layer_blur=False, **settings):
    c = {"drawing": "stripes", "shift": shift, "tile": tile, "blur": blur,
         "layer_blur": layer_blur, "anchor_point": (50, 50), "position": (50, 50),
         "uniform_scale": "on", "scale_height": 100, "scale_width": 100, "skew": 0,
         "skew_axis": 0, "rotation": 0, "opacity": 100, "use_composition_shutter_angle": "on",
         "shutter_angle": 0, "sampling": "bilinear"}
    c.update(settings)
    return c


def held(c, k, t):
    lo, hi = RANGES[k]
    v = value_at(c[k], t)
    if isinstance(v, (tuple, list)):
        return tuple(min(hi, max(lo, x)) for x in v)
    return min(hi, max(lo, v))


def settings_at(c, t):
    s = {k: held(c, k, t) for k in POINTS + NUMBERS}
    s.update({k: c[k] for k in WORDS})
    return s


def moments(c, n):
    """The moments frame n's Transform is drawn at: the frame itself unless blurred."""
    blur = c["blur"]
    if not (c["layer_blur"] and blur and blur["enabled"]):
        return [n]
    if c["use_composition_shutter_angle"] == "off":
        a = held(c, "shutter_angle", n)
        blur = shutter(a, -a / 2, blur["samples"])
    return sample_times(n, blur)


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    return motion_tile(layer, 300, 300, "off") if c["tile"] else layer


def render(c, n):
    out = transform(layer_of(c), [settings_at(c, t) for t in moments(c, n)], W, H)
    return frame(out, c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return frame(layer_of(case()), c["shift"])


SLIDE = keyed((0, (50, 50)), (4, (100, 50)))  # two pixels right a frame
TURN = keyed((0, 0), (4, 90))

CASES = {
    "FX-XFORM-001": ("As added: the anchor point and position at the middle, scale 100, no skew "
                     "or rotation, opacity 100: the drawing, untouched.", case(), [0]),
    "FX-XFORM-002": ("Position (75, 50): the drawing four pixels right inside its layer; its "
                     "left four columns clear and its right four cut off, the layer never "
                     "growing.", case(position=(75, 50)), [0]),
    "FX-XFORM-003": ("Position (53.125, 50): half a pixel right, each pixel the even mix of "
                     "itself and the one to its left.", case(position=(53.125, 50)), [0]),
    "FX-XFORM-004": ("Scale Height 50 with Uniform Scale on, Scale Width 200 not read: the "
                     "drawing at half size round the middle, in columns 4 to 11 and rows 2 to 7.",
                     case(scale_height=50, scale_width=200), [0]),
    "FX-XFORM-005": ("Uniform Scale off, Scale Width 50, Scale Height 100: squeezed to half its "
                     "width round the middle, its full height kept.",
                     case(uniform_scale="off", scale_width=50), [0]),
    "FX-XFORM-006": ("Rotation 90: a quarter turn clockwise round the middle; column x of the "
                     "frame is row 12 - x of the drawing, cut to the layer.",
                     case(rotation=90), [0]),
    "FX-XFORM-007": ("Rotation 30, bilinear sampling: turned a twelfth, soft at the stripes' "
                     "edges.", case(rotation=30), [0]),
    "FX-XFORM-008": ("Rotation 30, bicubic sampling: the same turn, the stripes' edges crisper "
                     "than FX-XFORM-007.", case(rotation=30, sampling="bicubic"), [0]),
    "FX-XFORM-009": ("Skew 30 along axis 0: the rows slide sideways, the top ones right and the "
                     "lower ones left, round the middle row; the stripes lean.",
                     case(skew=30), [0]),
    "FX-XFORM-010": ("Skew 30 along axis 90: the columns slide up and down instead; the blue "
                     "band leans.", case(skew=30, skew_axis=90), [0]),
    "FX-XFORM-011": ("Anchor point and position (0, 0), rotation 45: an eighth turn round the "
                     "drawing's top left corner, half of it swung off the layer.",
                     case(anchor_point=(0, 0), position=(0, 0), rotation=45), [0]),
    "FX-XFORM-012": ("Opacity 50: every pixel at half its covering.", case(opacity=50), [0]),
    "FX-XFORM-013": ("Scale Height -100, uniform: turned over both ways, the same as rotation "
                     "180.", case(scale_height=-100), [0]),
    "FX-XFORM-014": ("Scale Height 0: nothing left, the frame clear.", case(scale_height=0),
                     [0]),
    "FX-XFORM-015": ("Position keyed from (50, 50) at frame 0 to (100, 50) at frame 4, linear, "
                     "no motion blur: two pixels further right each frame.",
                     case(position=SLIDE), [0, 2, 4]),
    "FX-XFORM-016": ("FX-XFORM-015 with the layer's motion blur switch on and the composition's "
                     "blur enabled at 180 degrees, phase -90, 4 samples: frame 2 is the mean of "
                     "the slide at 1.8125, 1.9375, 2.0625 and 2.1875, a streak a pixel long.",
                     case(position=SLIDE, blur=ON, layer_blur=True), [0, 2, 4]),
    "FX-XFORM-017": ("FX-XFORM-016 with the layer's switch off: no blur, as FX-XFORM-015.",
                     case(position=SLIDE, blur=ON), [2]),
    "FX-XFORM-018": ("FX-XFORM-016 with the composition's blur not enabled: no blur.",
                     case(position=SLIDE, blur=shutter(180, -90, 4, False), layer_blur=True),
                     [2]),
    "FX-XFORM-019": ("FX-XFORM-016 with Use Composition's Shutter Angle off and Shutter Angle "
                     "360, centred on the frame: the moments 1.625, 1.875, 2.125 and 2.375, at "
                     "twice the spacing round 2, a streak two pixels long.",
                     case(position=SLIDE, blur=ON, layer_blur=True,
                          use_composition_shutter_angle="off", shutter_angle=360), [2]),
    "FX-XFORM-020": ("FX-XFORM-019 with Shutter Angle 0: no blur, as FX-XFORM-015.",
                     case(position=SLIDE, blur=ON, layer_blur=True,
                          use_composition_shutter_angle="off", shutter_angle=0), [2]),
    "FX-XFORM-021": ("Rotation 30, not keyed, with the layer's and composition's blur on: "
                     "nothing moves through the shutter, so no blur, as FX-XFORM-007.",
                     case(rotation=30, blur=ON, layer_blur=True), [0]),
    "FX-XFORM-022": ("Rotation keyed from 0 at frame 0 to 90 at frame 4 with the blur on, "
                     "bicubic: frame 2 is the mean of four turns round 45 degrees, a spin "
                     "blur.", case(rotation=TURN, blur=ON, layer_blur=True, sampling="bicubic"),
                     [2]),
    "FX-XFORM-023": ("Position (75, 50) on the layer moved three pixels right: the effect works "
                     "inside the layer, so FX-XFORM-002 moved three more.",
                     case(3, position=(75, 50)), [0]),
    "FX-XFORM-024": ("Motion Tile at 300% by 300% first, then rotation 90: the points are the "
                     "drawing's own, so the tiles turn round the drawing's middle and fill the "
                     "frame.", case(tile=True, rotation=90), [0]),
    "FX-XFORM-025": ("Bicubic sampling with the settings as added: the drawing, untouched.",
                     case(sampling="bicubic"), [0]),
}

INVALID = {
    "FX-XFORM-026": ("Skew 86, above 85.", case(skew=86)),
    "FX-XFORM-027": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-XFORM-028": ("Position keyed to (1200, 50) at frame 4.",
                     case(position=keyed((0, (50, 50)), (4, (1200, 50))))),
    "FX-XFORM-029": ("Shutter Angle 361, above 360.", case(shutter_angle=361)),
    "FX-XFORM-030": ("Uniform Scale written \"yes\".", case(uniform_scale="yes")),
    "FX-XFORM-031": ("Sampling written \"nearest\".", case(sampling="nearest")),
    "FX-XFORM-032": ("Use Composition's Shutter Angle written \"maybe\".",
                     case(use_composition_shutter_angle="maybe")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    if c["blur"] is not None:
        comp["motion_blur"] = c["blur"]
    layer = comp["layers"][0]
    if c["layer_blur"]:
        layer["motion_blur"] = True
    t = layer["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    effects = []
    if c["tile"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.motion_tile", "enabled": True,
                        "parameters": {"output_width": 300, "output_height": 300,
                                       "mirror": "off"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.transform",
                    "enabled": True, "parameters": {k: setting_json(c[k]) for k in KEYS}})
    layer["effects"] = effects
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_transform.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda a, b, e=1e-9: all(near(p, q, e) for p, q in zip(a, b))  # noqa: E731
    clear = lambda p: all(abs(v) < 1e-12 for v in p)  # noqa: E731

    # The rule's own pieces: the matrix at the start is the identity, and undoes.
    m = matrix(settings_at(case(), 0), W, H)
    assert all(abs(m[i][j] - (i == j)) < 1e-12 for i in range(3) for j in range(3))
    for s in (case(rotation=30, skew=20, skew_axis=40, scale_height=70, uniform_scale="off",
                   scale_width=-120, anchor_point=(10, 90), position=(60, 20)),):
        m = matrix(settings_at(s, 0), W, H)
        i = inverse(m)
        assert all(abs(sum(m[a][k] * i[k][b] for k in range(3)) - (a == b)) < 1e-9
                   for a in range(3) for b in range(3))
    assert inverse(matrix(settings_at(case(scale_height=0), 0), W, H)) is None
    assert abs(keys_weight(0) - 1) < 1e-15 and keys_weight(1) == 0 and keys_weight(2) == 0
    assert abs(sum(keys_weight(d) for d in (1.3, 0.3, 0.7, 1.7)) - 1) < 1e-12

    assert c["FX-XFORM-001"]["0"] == drawn
    assert same(c["FX-XFORM-025"]["0"], drawn)
    two = c["FX-XFORM-002"]["0"]
    for y in range(H):
        for x in range(W):
            assert near(two[at(x, y)], drawn[at(x - 4, y)] if x >= 4 else EMPTY), (x, y)
    three = c["FX-XFORM-003"]["0"]
    for y in range(H):
        for x in range(W):
            left = drawn[at(x - 1, y)] if x >= 1 else EMPTY
            assert near(three[at(x, y)], [(a + b) / 2 for a, b in zip(drawn[at(x, y)], left)])
    four = c["FX-XFORM-004"]["0"]
    for y in range(H):
        for x in range(W):
            assert clear(four[at(x, y)]) or (4 <= x <= 11 and 2 <= y <= 7), (x, y)
    assert c["FX-XFORM-005"]["0"] != four
    six = c["FX-XFORM-006"]["0"]
    for y in range(H):
        for x in range(W):
            sx, sy = y + 3, 12 - x
            want = drawn[at(sx, sy)] if 0 <= sx < W and 0 <= sy < H else EMPTY
            assert near(six[at(x, y)], want, 1e-9), (x, y)
    assert c["FX-XFORM-007"]["0"] != c["FX-XFORM-008"]["0"]
    nine, ten = c["FX-XFORM-009"]["0"], c["FX-XFORM-010"]["0"]
    assert nine != drawn and ten != drawn and nine != ten
    # Axis 0: rows only slide, so the middle row (y = 5, centre 5.5, 0.5 below the anchor)
    # moves left by tan(30) / 2 of a pixel and keeps its blue.
    assert all(n[2] >= n[0] for n in (nine[at(x, 5)] for x in range(3, 12)))
    eleven = c["FX-XFORM-011"]["0"]
    # Clockwise round the top left corner: the top edge swings down to the diagonal, the left
    # edge off the layer, so above the diagonal is clear and below it drawn.
    assert clear(eleven[at(9, 0)]) and clear(eleven[at(1, 0)])
    assert not clear(eleven[at(0, 9)]) and not clear(eleven[at(0, 4)])
    twelve = c["FX-XFORM-012"]["0"]
    assert same(twelve, [[v / 2 for v in p] for p in drawn])
    thirteen = c["FX-XFORM-013"]["0"]
    assert same(thirteen, render(case(rotation=180), 0), 1e-9)
    assert all(clear(p) for p in c["FX-XFORM-014"]["0"])
    fifteen = c["FX-XFORM-015"]
    assert fifteen["0"] == drawn
    assert same(fifteen["2"], render(case(position=(75, 50)), 0))
    assert same(fifteen["2"], two)
    sixteen = c["FX-XFORM-016"]
    assert moments(case(position=SLIDE, blur=ON, layer_blur=True), 2) == [1.8125, 1.9375, 2.0625,
                                                                         2.1875]
    assert sixteen["2"] != fifteen["2"]
    mean = transform(layer_of(case()), [settings_at(case(position=SLIDE), t)
                                        for t in (1.8125, 1.9375, 2.0625, 2.1875)], W, H)
    assert same(sixteen["2"], frame(mean, 0))
    for fx in ("FX-XFORM-017", "FX-XFORM-018", "FX-XFORM-020"):
        assert same(c[fx]["2"], fifteen["2"]), fx
    nineteen = c["FX-XFORM-019"]["2"]
    assert moments(case(position=SLIDE, blur=ON, layer_blur=True,
                        use_composition_shutter_angle="off", shutter_angle=360), 2) == [
        1.625, 1.875, 2.125, 2.375]
    assert nineteen != sixteen["2"] and nineteen != fifteen["2"]
    assert same(c["FX-XFORM-021"]["0"], c["FX-XFORM-007"]["0"])
    assert c["FX-XFORM-022"]["2"] != render(case(rotation=45, sampling="bicubic"), 0)
    twentythree = c["FX-XFORM-023"]["0"]
    for y in range(H):
        for x in range(W):
            assert near(twentythree[at(x, y)], two[at(x - 3, y)] if x >= 3 else EMPTY)
    # The tiles fill the corners FX-XFORM-006 leaves clear.
    tiled = c["FX-XFORM-024"]["0"]
    assert sum(not clear(p) for p in tiled) > sum(not clear(p) for p in six)
    assert clear(six[at(0, 0)]) and not clear(tiled[at(0, 0)])
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(v >= -1e-12 for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
