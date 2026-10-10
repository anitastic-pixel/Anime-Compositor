"""Fractal, worked a second way.

D-416 adds `core.fractal`, After Effects' Fractal (Generate): it "renders the Mandelbrot or Julia
set, creating colorful textures ... the set is the area that is colored black. Any pixel outside
the set is colorized, depending on how close it is to the set" (Adobe's help page on the
Generate effects). Its settings are After Effects': Set Choice, Equation, the Mandelbrot and Julia
centres, magnifications and escape limits, Overlay, Transparency, Palette, Hue, Cycle Steps, Cycle
Offset, Edge Highlight, Oversample Method and Oversample Factor. Adobe gives no formula past "if
the path leads out of the bounded rectangle (-2, -2, 2, 2), it has gone into infinity ... the
starting-point color is based on how many line segments it takes to reach infinity. If the path
ends within the rectangle, it's colored black"; the numbers below are this program's own rule and
nothing is ported.

The rule, at a point X = (x, y) of the drawing's own pixels (its top-left corner (0, 0), however
far an effect above grew the buffer; a pixel's centre is its corner plus a half), the drawing w
by h pixels at the frame's scale (a draft halves both):

1. A view is a centre (cx, cy) and a magnification m: one pixel is u = 3 / (h 2^m) units, so the
   whole height spans 3 units at magnification 0, and X is the complex number
   p = (cx + (x - w / 2) u, cy - (y - h / 2) u), the imaginary part up.
2. z^n + c, n 2 to 6 by Equation, is worked by repeated multiplication, q = z then n - 1 times
   q = (q.re z.re - q.im z.im, q.re z.im + q.im z.re), then z = q + c, in that order.
3. The set: Mandelbrot takes z0 = 0 and c = p in the Mandelbrot view; Mandelbrot Over Julia the
   same with z0 the Julia centre; Julia takes z0 = p in the Julia view and c the Mandelbrot
   centre. The inverse ones work 1 / p = (p.re / d, -p.im / d), d = p.re p.re + p.im p.im, in
   place of p; 1 / 0 escapes at the first step. From k = 1 to the escape limit, z is stepped; the
   first k after which |z.re| > 2 or |z.im| > 2 is the escape count. A point that never escapes
   is inside the set.
4. The band: -1 inside; Lightness Gradient (k + offset) mod 8 steps; Hue Wheel (k + offset) mod
   steps; Black And White (k + offset) mod 2; Solid Color 0 outside.
5. The colour, worked encoded with the hue H in degrees, then decoded through the sRGB curve and
   premultiplied: inside, opaque black, or clear with Transparency; Lightness Gradient's band n
   is gradient g = n div steps, step j = n mod steps, hue H + 45 g, lightness (j + 1) /
   (steps + 1), saturation 1 (eight gradients from black to white through the hue, each 45
   degrees on); Hue Wheel's band n hue H + 360 n / steps, lightness 0.5, saturation 1; Black And
   White's band 0 black, 1 white; Solid Color the hue H at lightness 0.5 inside and clear
   outside, the other way round with Transparency. HSL is grade.rs's from_hls.
6. Oversampling, at full resolution only and with a factor f above 1: Brute Force works every
   pixel at the f by f points (corner + (a + 0.5) / f, corner + (b + 0.5) / f) and averages
   their colours (linear, premultiplied); Edge Detect does so only at pixels whose band differs
   from that of any of its four neighbours' centres (worked by the rule, inside the buffer or
   not), and keeps the centre's colour elsewhere.
7. Edge Highlight, when nothing is oversampled (a draft, or factor 1): a pixel whose band
   differs from its left or upper neighbour's becomes opaque white.
8. Overlay: the opposite set, worked at the pixel's centre in its own view with its own escape
   limit and never inverted (a Julia with c the Mandelbrot centre opposite the Mandelbrot
   choices, a plain Mandelbrot opposite the Julia ones); where it is inside, the pixel goes half
   way to opaque white, (p + 1) / 2. Then a cross at the drawing's pixel (floor(w / 2),
   floor(h / 2)), arms max(2, floor(h / 20)) long: first its black shadow one pixel right and
   down, then the cross in opaque white.

The layer is replaced across its whole buffer and never grows. `set_choice` `mandelbrot` (when
added), `mandelbrot_inverse`, `mandelbrot_over_julia`, `mandelbrot_inverse_over_julia`, `julia`
or `julia_inverse`; `equation` `z2` (when added) to `z6`; `mandelbrot_center` two numbers -10 to
10, keyable, (-0.75, 0) when added, `julia_center` the same, (0, 0); the magnifications -10 to
40, keyable, 0; the escape limits 1 to 10000, keyable, 100; `overlay`, `transparency` and
`edge_highlight` `off` (when added) or `on`; `palette` `lightness_gradient` (when added),
`hue_wheel`, `black_and_white` or `solid_color`; `hue` -36000 to 36000 degrees, keyable, 0;
`cycle_steps` 1 to 1000, keyable, 10; `cycle_offset` 0 to 1000, keyable, 0;
`oversample_method` `edge_detect` (when added) or `brute_force`; `oversample_factor` 1 to 8,
keyable, 2. The escape limits, the cycle steps and offset and the factor count their whole part,
as every count in this program does.

**This file never runs the build's code path.** It works in double precision on lists, in the
operation order above, which the build keeps.

Every case is a composition 16 by 10 holding Gradient's cel, the same size, unmoved unless the
case says. The expected frames are in `Fixtures/fractal/expected_fractal.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/fractal_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from gradient_reference import DRAWINGS  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "fractal"
TOLERANCE = 2e-5  # document 25's default for a filter
SETS = ("mandelbrot", "mandelbrot_inverse", "mandelbrot_over_julia",
        "mandelbrot_inverse_over_julia", "julia", "julia_inverse")
PALETTES = ("lightness_gradient", "hue_wheel", "black_and_white", "solid_color")
RANGES = {"mandelbrot_center": (-10, 10), "mandelbrot_magnification": (-10, 40),
          "julia_center": (-10, 10), "julia_magnification": (-10, 40), "hue": (-36000, 36000),
          "mandelbrot_escape_limit": (1, 10000), "julia_escape_limit": (1, 10000),
          "cycle_steps": (1, 1000), "cycle_offset": (0, 1000), "oversample_factor": (1, 8)}
COUNTS = ("mandelbrot_escape_limit", "julia_escape_limit", "cycle_steps", "cycle_offset",
          "oversample_factor")
WORDS = ("set_choice", "equation", "overlay", "transparency", "palette", "edge_highlight",
         "oversample_method")
NAMES = tuple(RANGES) + WORDS


# --- the rule -------------------------------------------------------------------------------

def from_hls(h, l, s):
    """grade.rs's from_hls, h in degrees 0 to 360."""
    chroma = (1 - abs(2 * l - 1)) * s
    hh = h / 60
    x = chroma * (1 - abs(hh % 2 - 1))
    rgb = [[chroma, x, 0], [x, chroma, 0], [0, chroma, x], [0, x, chroma], [x, 0, chroma],
           [chroma, 0, x]][min(math.floor(hh), 5)]
    return [v + l - chroma / 2 for v in rgb]


def point(x, y, center, mag):
    u = 3 / (H * 2 ** mag)
    return (center[0] + (x - W / 2) * u, center[1] - (y - H / 2) * u)


def escape(z, c, power, limit):
    """Step 3's count, or -1 inside."""
    zr, zi = z
    for k in range(1, limit + 1):
        qr, qi = zr, zi
        for _ in range(power - 1):
            qr, qi = qr * zr - qi * zi, qr * zi + qi * zr
        zr, zi = qr + c[0], qi + c[1]
        if abs(zr) > 2 or abs(zi) > 2:
            return k
    return -1


def invert(p):
    d = p[0] * p[0] + p[1] * p[1]
    return None if d == 0 else (p[0] / d, -p[1] / d)


def count(x, y, n, w):
    """Step 3 at the drawing point (x, y)."""
    power = int(w["equation"][1])
    choice = w["set_choice"]
    if choice.startswith("julia"):
        p = point(x, y, n["julia_center"], n["julia_magnification"])
        if choice == "julia_inverse":
            p = invert(p)
            if p is None:
                return 1
        return escape(p, n["mandelbrot_center"], power, n["julia_escape_limit"])
    p = point(x, y, n["mandelbrot_center"], n["mandelbrot_magnification"])
    if "inverse" in choice:
        p = invert(p)
        if p is None:
            return 1
    z0 = n["julia_center"] if "over_julia" in choice else (0.0, 0.0)
    return escape(z0, p, power, n["mandelbrot_escape_limit"])


def band(k, n, w):
    if k < 0:
        return -1
    steps, off = n["cycle_steps"], n["cycle_offset"]
    return {"lightness_gradient": (k + off) % (8 * steps), "hue_wheel": (k + off) % steps,
            "black_and_white": (k + off) % 2, "solid_color": 0}[w["palette"]]


def color(b, n, w):
    clear_inside = w["transparency"] == "on"
    hue = n["hue"]
    if w["palette"] == "solid_color":
        if (b < 0) == clear_inside:
            return [0.0] * 4
        e = from_hls(hue % 360, 0.5, 1)
    elif b < 0:
        return [0.0] * 4 if clear_inside else [0.0, 0.0, 0.0, 1.0]
    elif w["palette"] == "lightness_gradient":
        steps = n["cycle_steps"]
        e = from_hls((hue + 45 * (b // steps)) % 360, ((b % steps) + 1) / (steps + 1), 1)
    elif w["palette"] == "hue_wheel":
        e = from_hls((hue + 360 * b / n["cycle_steps"]) % 360, 0.5, 1)
    else:
        e = [float(b)] * 3
    return [srgb_to_linear(v) for v in e] + [1.0]


def fractal(layer, n, w):
    """Every pixel of the buffer, at full resolution."""
    f = n["oversample_factor"]
    brute = w["oversample_method"] == "brute_force"
    at = lambda x, y: band(count(x, y, n, w), n, w)  # noqa: E731
    cx, cy, arm = W // 2, H // 2, max(2, H // 20)
    cross = {(cx + d, cy) for d in range(-arm, arm + 1)} | {(cx, cy + d) for d in range(-arm, arm + 1)}
    shadow = {(x + 1, y + 1) for x, y in cross}
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            x0, y0 = layer["left"] + i, layer["top"] + j
            x, y = x0 + 0.5, y0 + 0.5
            b = at(x, y)
            near = (at(x - 1, y), at(x + 1, y), at(x, y - 1), at(x, y + 1))
            if f > 1 and (brute or any(v != b for v in near)):
                total = [0.0] * 4
                for bb in range(f):
                    for aa in range(f):
                        s = color(at(x0 + (aa + 0.5) / f, y0 + (bb + 0.5) / f), n, w)
                        total = [t + v for t, v in zip(total, s)]
                p = [t / (f * f) for t in total]
            else:
                p = color(b, n, w)
                if f == 1 and w["edge_highlight"] == "on" and (near[0] != b or near[2] != b):
                    p = [1.0] * 4
            if w["overlay"] == "on":
                if w["set_choice"].startswith("julia"):
                    q = point(x, y, n["mandelbrot_center"], n["mandelbrot_magnification"])
                    k = escape((0.0, 0.0), q, int(w["equation"][1]), n["mandelbrot_escape_limit"])
                else:
                    q = point(x, y, n["julia_center"], n["julia_magnification"])
                    k = escape(q, n["mandelbrot_center"], int(w["equation"][1]),
                               n["julia_escape_limit"])
                if k < 0:
                    p = [(v + 1) / 2 for v in p]
                if (x0, y0) in cross:
                    p = [1.0] * 4
                elif (x0, y0) in shadow:
                    p = [0.0, 0.0, 0.0, 1.0]
            px.append(p)
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(set_choice="mandelbrot", equation="z2", mandelbrot_center=(-0.75, 0),
         mandelbrot_magnification=0, mandelbrot_escape_limit=100, julia_center=(0, 0),
         julia_magnification=0, julia_escape_limit=100, overlay="off", transparency="off",
         palette="lightness_gradient", hue=0, cycle_steps=10, cycle_offset=0,
         edge_highlight="off", oversample_method="edge_detect", oversample_factor=2, shift=0,
         tile=False):
    c = dict(locals())
    c["drawing"] = "cel"
    return c


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    if isinstance(v, (list, tuple)):
        return [min(hi, max(lo, e)) for e in v]
    return min(hi, max(lo, v))


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    return motion_tile(layer, 300, 300, "on") if c["tile"] else layer


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    n.update({k: math.floor(n[k]) for k in COUNTS})
    return frame(fractal(layer_of(c), n, {k: c[k] for k in WORDS}), c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


DEEP = (-0.743643887037151, 0.131825904205330)

CASES = {
    "FX-FRACTALSET-001": ("The settings as they start: the Mandelbrot set from -3.15 to 1.65 across "
                       "and -1.5 to 1.5 up, black inside, the bands of Lightness Gradient's "
                       "first gradient (red) outside, oversampled 2 by 2 where the bands change; "
                       "the cel itself is gone.", case(), [0]),
    "FX-FRACTALSET-002": ("Brute Force on Solid Color at magnification 2, (-0.75, 0.1): every "
                       "pixel worked 2 by 2, so two pixels whose centre and four neighbours are "
                       "all outside, but which a filament of the set crosses, turn partly red; "
                       "Edge Detect misses them. (On the settings as they start every pixel of "
                       "this small frame is on a band change, so there the two methods agree.)",
                       case(oversample_method="brute_force", palette="solid_color",
                            mandelbrot_center=(-0.75, 0.1), mandelbrot_magnification=2), [0]),
    "FX-FRACTALSET-003": ("Oversample Factor 1: one point a pixel, hard band edges.",
                       case(oversample_factor=1), [0]),
    "FX-FRACTALSET-004": ("Factor 1 with Edge Highlight: pixels where the band changes from the "
                       "left or above are white.", case(oversample_factor=1, edge_highlight="on"),
                       [0]),
    "FX-FRACTALSET-005": ("Edge Highlight with factor 2: oversampling wins, the frame is "
                       "FX-FRACTALSET-001's.", case(edge_highlight="on"), [0]),
    "FX-FRACTALSET-006": ("Transparency: inside the set is clear.", case(transparency="on"), [0]),
    "FX-FRACTALSET-007": ("Hue Wheel: the bands go round the wheel in 10 steps.",
                       case(palette="hue_wheel"), [0]),
    "FX-FRACTALSET-008": ("Black And White: the bands alternate.", case(palette="black_and_white"),
                       [0]),
    "FX-FRACTALSET-009": ("Solid Color: the set in red, outside clear.", case(palette="solid_color"),
                       [0]),
    "FX-FRACTALSET-010": ("Solid Color with Transparency: the set clear, outside red.",
                       case(palette="solid_color", transparency="on"), [0]),
    "FX-FRACTALSET-011": ("Hue 120: the gradients start green.", case(hue=120), [0]),
    "FX-FRACTALSET-012": ("Cycle Steps 3, Cycle Offset 2: shorter gradients, started two on.",
                       case(cycle_steps=3, cycle_offset=2), [0]),
    "FX-FRACTALSET-013": ("Julia: the Julia set of c = -0.75 (the Mandelbrot centre), at the Julia "
                       "view (0, 0).", case(set_choice="julia"), [0]),
    "FX-FRACTALSET-014": ("Julia Inverse.", case(set_choice="julia_inverse"), [0]),
    "FX-FRACTALSET-015": ("Mandelbrot Inverse, centred on 0 so the middle pixel's corner is the "
                       "point 1 / 0.", case(set_choice="mandelbrot_inverse",
                                            mandelbrot_center=(0, 0)), [0]),
    "FX-FRACTALSET-016": ("Mandelbrot Over Julia, the Julia centre (0.3, 0.2): z starts there.",
                       case(set_choice="mandelbrot_over_julia", julia_center=(0.3, 0.2)), [0]),
    "FX-FRACTALSET-017": ("Mandelbrot Inverse Over Julia, the same Julia centre.",
                       case(set_choice="mandelbrot_inverse_over_julia", julia_center=(0.3, 0.2),
                            mandelbrot_center=(0, 0)), [0]),
    "FX-FRACTALSET-018": ("Equation z^3 + c.", case(equation="z3", mandelbrot_center=(0, 0)), [0]),
    "FX-FRACTALSET-019": ("Equation z^6 + c.", case(equation="z6", mandelbrot_center=(0, 0)), [0]),
    "FX-FRACTALSET-020": ("Overlay on the Mandelbrot: the Julia set ghosted half way to white and "
                       "the white cross with its black shadow at pixel (8, 5).",
                       case(overlay="on"), [0]),
    "FX-FRACTALSET-021": ("Overlay on the Julia: the Mandelbrot ghosted.",
                       case(set_choice="julia", overlay="on"), [0]),
    "FX-FRACTALSET-022": ("Magnification 2 at (-0.75, 0.1): four times closer, on the neck.",
                       case(mandelbrot_magnification=2, mandelbrot_center=(-0.75, 0.1)), [0]),
    "FX-FRACTALSET-023": ("Escape Limit 5: far more of the plane counts as inside.",
                       case(mandelbrot_escape_limit=5), [0]),
    "FX-FRACTALSET-024": ("Magnification keyed from 0 at frame 0 to 4 at frame 4, linear: frame 2 "
                       "is magnification 2.",
                       case(mandelbrot_magnification=keyed((0, 0), (4, 4))), [0, 2, 4]),
    "FX-FRACTALSET-025": ("Hue keyed from 0 at frame 0 to 360 at frame 4: frame 4 is frame 0, "
                       "frame 2 hue 180.", case(hue=keyed((0, 0), (4, 360))), [0, 2, 4]),
    "FX-FRACTALSET-026": ("FX-FRACTALSET-003 moved three pixels right: the set moves with the layer.",
                       case(oversample_factor=1, shift=3), [0]),
    "FX-FRACTALSET-027": ("After a Motion Tile that grows the layer: the view is the drawing's own, "
                       "so the frame is FX-FRACTALSET-001's.", case(tile=True), [0]),
    "FX-FRACTALSET-028": ("A deep zoom, magnification 40 at (-0.743643887037151, 0.131825904205330) "
                       "with Escape Limit 2000: a pixel is 2.7e-13 units, which double "
                       "precision still separates.",
                       case(mandelbrot_center=DEEP, mandelbrot_magnification=40,
                            mandelbrot_escape_limit=2000), [0]),
    "FX-FRACTALSET-029": ("Magnification keyed from 0 at frame 0 to 40 at frame 4 past its end by "
                       "an ease: held at 40, frame 4 is magnification 40.",
                       case(mandelbrot_magnification=keyed((0, 0, OVERSHOOT), (4, 40))), [4]),
    "FX-FRACTALSET-030": ("Cycle Steps 3.7 and Escape Limit 100.9: their whole parts count, so the "
                       "frame is Cycle Steps 3's with Escape Limit 100.",
                       case(cycle_steps=3.7, mandelbrot_escape_limit=100.9), [0]),
}

INVALID = {
    "FX-FRACTALSET-031": ("Set Choice \"burning_ship\", not one of its six words.",
                       case(set_choice="burning_ship")),
    "FX-FRACTALSET-032": ("Equation \"z7\".", case(equation="z7")),
    "FX-FRACTALSET-033": ("Escape Limit 0, below 1.", case(mandelbrot_escape_limit=0)),
    "FX-FRACTALSET-034": ("Cycle Offset 1001, above 1000.", case(cycle_offset=1001)),
    "FX-FRACTALSET-035": ("Magnification 41, above 40.", case(mandelbrot_magnification=41)),
    "FX-FRACTALSET-036": ("Julia centre 11 across, above 10.", case(julia_center=(11, 0))),
    "FX-FRACTALSET-037": ("Palette \"rainbow\".", case(palette="rainbow")),
    "FX-FRACTALSET-038": ("Cycle Steps 0, below 1.", case(cycle_steps=0)),
    "FX-FRACTALSET-039": ("Oversample Factor 9, above 8.", case(oversample_factor=9)),
    "FX-FRACTALSET-040": ("Hue keyed to 36001 at frame 4, above 36000.",
                       case(hue=keyed((0, 0), (4, 36001)))),
    "FX-FRACTALSET-041": ("Oversample Method \"fast\".", case(oversample_method="fast")),
    "FX-FRACTALSET-042": ("Transparency \"yes\", not off or on.", case(transparency="yes")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    effects = []
    if c["tile"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.motion_tile", "enabled": True,
                        "parameters": {"output_width": 300, "output_height": 300,
                                       "mirror": "on"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.fractal",
                    "enabled": True,
                    "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                   for k in NAMES}})
    comp["layers"][0]["effects"] = effects
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
    (OUT / "expected_fractal.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    white, black, clear = [1.0] * 4, [0.0, 0.0, 0.0, 1.0], [0.0] * 4
    art = plain(case())
    one, three = c["FX-FRACTALSET-001"]["0"], c["FX-FRACTALSET-003"]["0"]
    assert one != art and all(p[3] == 1 for p in one)
    # (-0.75, 0) sits in the main bulb: the pixels about the middle are black; the corners escape.
    assert near(three[at(8, 5)], black) and three[at(0, 0)] != black
    # Escape at the first step: the corner (-3.15, 1.5) is past 2 already once stepped.
    assert escape((0.0, 0.0), point(0.5, 0.5, (-0.75, 0), 0), 2, 100) == 1
    assert one == c["FX-FRACTALSET-005"]["0"] == c["FX-FRACTALSET-027"]["0"]
    edge = render(case(palette="solid_color", mandelbrot_center=(-0.75, 0.1),
                       mandelbrot_magnification=2), 0)
    assert sum(a != b for a, b in zip(c["FX-FRACTALSET-002"]["0"], edge)) == 2
    assert render(case(oversample_method="brute_force"), 0) == one
    # Oversampling only moves pixels on a band change; elsewhere 001 is 003.
    assert sum(one[i] != three[i] for i in range(W * H)) > 0
    four = c["FX-FRACTALSET-004"]["0"]
    assert any(near(p, white) for p in four)
    assert all(near(four[i], three[i]) or near(four[i], white) for i in range(W * H))
    six = c["FX-FRACTALSET-006"]["0"]
    assert any(near(p, black) for p in one)
    assert all(near(six[i], clear) == near(one[i], black) for i in range(W * H))
    bw = c["FX-FRACTALSET-008"]["0"]
    assert all(near(p, black) or near(p, white) or 0 < p[0] < 1 for p in bw)
    red = [1.0, 0.0, 0.0, 1.0]
    nine, ten = c["FX-FRACTALSET-009"]["0"], c["FX-FRACTALSET-010"]["0"]
    assert near(nine[at(8, 5)], red) and near(nine[at(0, 0)], clear)
    assert near(ten[at(8, 5)], clear) and near(ten[at(0, 0)], red)
    assert c["FX-FRACTALSET-011"]["0"] != one
    # 1 / 0 escapes at once: Mandelbrot Inverse centred on 0 puts 0 at the corner of (8, 5);
    # no pixel centre lands on it, so check the rule directly.
    assert invert((0.0, 0.0)) is None
    assert c["FX-FRACTALSET-016"]["0"] != one and c["FX-FRACTALSET-013"]["0"] != one
    twenty = c["FX-FRACTALSET-020"]["0"]
    assert near(twenty[at(8, 5)], white) and near(twenty[at(10, 5)], white)
    assert near(twenty[at(8, 3)], white) and near(twenty[at(11, 6)], black)
    assert near(twenty[at(9, 7)], black)
    assert any(twenty[i] != one[i] for i in range(W * H) if abs(i % W - 8) > 3)
    assert c["FX-FRACTALSET-021"]["0"] != c["FX-FRACTALSET-013"]["0"]
    k24 = c["FX-FRACTALSET-024"]
    assert k24["0"] == one
    assert k24["2"] == render(case(mandelbrot_magnification=2), 0)
    k25 = c["FX-FRACTALSET-025"]
    assert k25["0"] == k25["4"] == one or all(near(a, b) for a, b in zip(k25["4"], one))
    assert k25["2"] == render(case(hue=180), 0)
    moved, = (c["FX-FRACTALSET-026"]["0"],)
    assert all(moved[at(x, y)] == three[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved[at(x, y)] == clear for x in range(3) for y in range(H))
    deep = c["FX-FRACTALSET-028"]["0"]
    assert len({tuple(p) for p in deep}) > 4
    assert c["FX-FRACTALSET-029"]["4"] == render(case(mandelbrot_magnification=40), 0)
    assert c["FX-FRACTALSET-023"]["0"].count(black) > three.count(black)
    assert c["FX-FRACTALSET-030"]["0"] == render(case(cycle_steps=3), 0)
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


if __name__ == "__main__":
    main()
