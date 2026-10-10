"""Checkerboard, worked a second way.

D-413 adds `core.checkerboard`, After Effects' Checkerboard (Generate): "a checkerboard pattern
of rectangles, half of which are transparent" (Adobe's help, read through its After Effects 7
manual page on Checkerboard). Its settings are After Effects': Anchor ("the point of origin of the
checkerboard pattern. Moving this point offsets the pattern"), Size From (Corner Point: "the
dimensions of each rectangle are the dimensions of the rectangle with opposite corners defined by
the Anchor and Corner points"; Width Slider: "the rectangles are squares"; Width & Height
Sliders), Corner, Width, Height, Feather Width and Height ("thickness of the edge feather within
the checkerboard pattern"), Color, Opacity and Blending Mode. The numbers below are this
program's own rule; nothing is ported.

The rule, at a pixel of the layer's buffer whose centre is X = (x, y) in the drawing's own
pixels (the drawing's top-left corner (0, 0), however far an effect above grew the buffer):

1. A is the anchor and K the corner, each in per cent of the drawing's own width and height.
   The cell is w by h pixels: Corner Point, w = |K.x - A.x| and h = |K.y - A.y|, each held at
   least 1; Width Slider, w = h = width; Width & Height Sliders, w = width and h = height.
2. Along x: u = (x - A.x) / w, i = floor(u), the distance to the nearer cell edge
   d = w min(u - i, i + 1 - u), the sign s = +1 when i is even and -1 when odd, and
   f_x = s min(d / (r_x / 2), 1) with r_x = max(feather_width, 1): a straight ramp r_x pixels
   wide centred on each edge, 1 pixel when there is no feather, which is a pixel's exact
   covering of a straight edge. f_y the same down, with h and feather_height. The covering is
   c = (1 + f_x f_y) / 2: 1 inside the cell that starts at the anchor and every second one from
   it, 0 in the others, a half on an edge.
3. The pattern is S = (C c o, c o), premultiplied, C the colour in linear light (its 8-bit
   values / 255 through the sRGB curve) and o = opacity / 100.
4. The layer's pixel D (premultiplied linear) becomes, by `blending_mode`: `none`, S alone, so
   the layer is replaced by the pattern; `normal`, S + D (1 - S.a); `stencil_alpha`, D S.a;
   `multiply`, `screen`, `add`, `overlay` and `soft_light`, document 21's layer blend of S onto
   D, Co = (1 - As) Cd + (1 - Ad) Cs + As Ad B(cs, cd), Ao = As + Ad - As Ad, with B cs cd,
   cs + cd - cs cd, min(cs + cd, 1), and overlay and soft light on the encoded straight
   colours held to 0..1 (the colour beneath first), as a layer's blend mode lays a layer
   (`src/composite.rs`). These are the blend modes the program already has; After Effects'
   others (Hard Light, Color Dodge, Color Burn, Darken, Lighten, Difference, Exclusion, Hue,
   Saturation, Color, Luminosity, Silhouette Alpha) are not offered.

`anchor` and `corner` -1000 to 1000 per cent, keyable, (50, 50) and (60, 60) when added;
`size_from` `corner_point`, `width_slider` (when added) or `width_and_height_sliders`; `width`
and `height` 1 to 10000 pixels, keyable, 64 when added; `feather_width` and `feather_height` 0
to 10000 pixels, keyable, 0 when added; `color` `#rrggbb`, white when added, read in small
letters; `opacity` 0 to 100, keyable, 100 when added; `blending_mode` as above, `none` when
added. Width, height and the feathers are distances: a draft halves them. The layer never grows.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 by 10 holding Gradient's cel, the same size, unmoved unless the
case says. The expected frames are in `Fixtures/checkerboard/expected_checkerboard.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/checkerboard_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "checkerboard"
TOLERANCE = 2e-5  # document 25's default for a filter
MODES = ("none", "normal", "add", "multiply", "screen", "overlay", "soft_light", "stencil_alpha")
RANGES = {"anchor": (-1000, 1000), "corner": (-1000, 1000), "width": (1, 10000),
          "height": (1, 10000), "feather_width": (0, 10000), "feather_height": (0, 10000),
          "opacity": (0, 100)}
WORDS = ("size_from", "color", "blending_mode")
NAMES = tuple(RANGES) + WORDS
VIOLET = "#6450a0"


# --- the laying, shared by Circle (tools/circle_reference.py) -------------------------------

def to_srgb(v):
    v = min(max(v, 0.0), 1.0)
    return 12.92 * v if v <= 0.0031308 else 1.055 * v ** (1 / 2.4) - 0.055


def mixer(mode, b, c):
    """grade::mixer's overlay and soft light, b the colour beneath, c the one laid on."""
    if mode == "overlay":
        return 2 * b * c if b <= 0.5 else 1 - 2 * (1 - b) * (1 - c)
    if c <= 0.5:
        return b - (1 - 2 * c) * b * (1 - b)
    d = ((16 * b - 12) * b + 4) * b if b <= 0.25 else math.sqrt(b)
    return b + (2 * c - 1) * (d - b)


def lay(d, cov, color, o, mode):
    """Step 3 and 4: the pattern's covering `cov` in `color` (linear) at `o` on the pixel d."""
    s = [v * cov * o for v in color] + [cov * o]
    if mode == "none":
        return s
    if mode == "normal":
        return [s[k] + d[k] * (1 - s[3]) for k in range(4)]
    if mode == "stencil_alpha":
        return [v * s[3] for v in d]
    a_s, a_d = s[3], d[3]
    cs = [v / a_s if a_s > 0 else 0.0 for v in s[:3]]
    cd = [v / a_d if a_d > 0 else 0.0 for v in d[:3]]
    out = []
    for k in range(3):
        if mode == "multiply":
            b = cs[k] * cd[k]
        elif mode == "screen":
            b = cs[k] + cd[k] - cs[k] * cd[k]
        elif mode == "add":
            b = min(cs[k] + cd[k], 1.0)
        else:
            b = srgb_to_linear(mixer(mode, to_srgb(cd[k]), to_srgb(cs[k])))
        out.append((1 - a_s) * d[k] + (1 - a_d) * s[k] + a_s * a_d * b)
    return out + [a_s + a_d - a_s * a_d]


def generate(layer, coverage, color, opacity, mode):
    """Every pixel of the buffer, its centre in the drawing's own pixels."""
    c = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    o = opacity / 100
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            px.append(lay(p, coverage(layer["left"] + i + 0.5, layer["top"] + j + 0.5), c, o, mode))
    return dict(layer, px=px)


# --- the rule -------------------------------------------------------------------------------

def axis(x, a, w, feather):
    u = (x - a) / w
    i = math.floor(u)
    d = w * min(u - i, i + 1 - u)
    s = 1 if i % 2 == 0 else -1
    return s * min(d / (max(feather, 1) / 2), 1)


def checker(n, size_from):
    ax, ay = n["anchor"][0] / 100 * W, n["anchor"][1] / 100 * H
    if size_from == "corner_point":
        kx, ky = n["corner"][0] / 100 * W, n["corner"][1] / 100 * H
        w, h = max(abs(kx - ax), 1), max(abs(ky - ay), 1)
    elif size_from == "width_slider":
        w = h = n["width"]
    else:
        w, h = n["width"], n["height"]
    return lambda x, y: (1 + axis(x, ax, w, n["feather_width"]) * axis(y, ay, h, n["feather_height"])) / 2


# --- the cases ------------------------------------------------------------------------------

def case(anchor=(50, 50), size_from="width_slider", corner=(60, 60), width=64, height=64,
         feather_width=0, feather_height=0, color="#ffffff", opacity=100, blending_mode="none",
         shift=0, tile=False):
    return {"drawing": "cel", "anchor": anchor, "size_from": size_from, "corner": corner,
            "width": width, "height": height, "feather_width": feather_width,
            "feather_height": feather_height, "color": color, "opacity": opacity,
            "blending_mode": blending_mode, "shift": shift, "tile": tile}


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
    out = generate(layer_of(c), checker(n, c["size_from"]), c["color"], n["opacity"],
                   c["blending_mode"])
    return frame(out, c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


CASES = {
    "FX-CHECK-001": ("The settings as they start: anchor in the middle, (50, 50) per cent, Width "
                     "Slider 64, white, opacity 100, blending mode none: the cell that starts at "
                     "the anchor runs off the bottom-right, so the top-left and bottom-right "
                     "quarters are white and the other two clear; the cel itself is gone.",
                     case(), [0]),
    "FX-CHECK-002": ("Width 4: white squares 4 pixels across in a checker, the anchor (8, 5) on "
                     "a corner of them, the others clear.", case(width=4), [0]),
    "FX-CHECK-003": ("Width 4 with the anchor at (53.125, 55) per cent, (8.5, 5.5) pixels: the "
                     "edges run through pixel centres, so the pixels on them are half white.",
                     case(anchor=(53.125, 55), width=4), [0]),
    "FX-CHECK-004": ("Width & Height Sliders, width 4 and height 2: rectangles 4 across and 2 "
                     "down.", case(size_from="width_and_height_sliders", width=4, height=2), [0]),
    "FX-CHECK-005": ("Corner Point, the corner at (75, 70) per cent, (12, 7) pixels: rectangles "
                     "4 by 2, FX-CHECK-004's.", case(size_from="corner_point", corner=(75, 70)),
                     [0]),
    "FX-CHECK-006": ("Corner Point with the corner on the anchor: each cell held at 1 pixel, a "
                     "checker of single pixels.", case(size_from="corner_point", corner=(50, 50)),
                     [0]),
    "FX-CHECK-007": ("Width 4, feather width 2: the upright edges ramp over 2 pixels, the level "
                     "ones stay sharp.", case(width=4, feather_width=2), [0]),
    "FX-CHECK-008": ("Width 4, both feathers 4: no pixel wholly white or wholly clear, a soft "
                     "weave.", case(width=4, feather_width=4, feather_height=4), [0]),
    "FX-CHECK-009": ("Width 4, orange #ff8000 at opacity 50: orange squares at half covering.",
                     case(width=4, color="#ff8000", opacity=50), [0]),
    "FX-CHECK-010": ("Width 4, normal: white squares over the cel, the cel showing in the clear "
                     "ones.", case(width=4, blending_mode="normal"), [0]),
    "FX-CHECK-011": ("Width 4, violet #6450a0, multiply: the cel darkened and tinted in the "
                     "squares, as it was in the others.",
                     case(width=4, color=VIOLET, blending_mode="multiply"), [0]),
    "FX-CHECK-012": ("Width 4, violet, screen: the cel lightened in the squares.",
                     case(width=4, color=VIOLET, blending_mode="screen"), [0]),
    "FX-CHECK-013": ("Width 4, violet, add: the violet added in the squares.",
                     case(width=4, color=VIOLET, blending_mode="add"), [0]),
    "FX-CHECK-014": ("Width 4, violet, overlay.",
                     case(width=4, color=VIOLET, blending_mode="overlay"), [0]),
    "FX-CHECK-015": ("Width 4, violet, soft light.",
                     case(width=4, color=VIOLET, blending_mode="soft_light"), [0]),
    "FX-CHECK-016": ("Width 4, stencil alpha: the cel shows only through the squares.",
                     case(width=4, blending_mode="stencil_alpha"), [0]),
    "FX-CHECK-017": ("Width 4, normal at opacity 0: the cel exactly as it was.",
                     case(width=4, opacity=0, blending_mode="normal"), [0]),
    "FX-CHECK-018": ("Width 4, the anchor keyed from (50, 50) at frame 0 to (75, 50) at frame "
                     "4, linear: the squares slide right, 1 pixel a frame; frame 4, moved one "
                     "square, is the clear and white swapped.",
                     case(width=4, anchor=keyed((0, (50, 50)), (4, (75, 50)))), [0, 2, 4]),
    "FX-CHECK-019": ("Width keyed from 2 at frame 0 to 8 at frame 4, linear: the squares "
                     "grow; frame 2 is width 5.",
                     case(width=keyed((0, 2), (4, 8))), [0, 2, 4]),
    "FX-CHECK-020": ("FX-CHECK-010 moved three pixels right: the squares move with the layer.",
                     case(width=4, blending_mode="normal", shift=3), [0]),
    "FX-CHECK-021": ("After a Motion Tile that grows the layer: the anchor is the drawing's own, "
                     "so the frame is FX-CHECK-002's.", case(width=4, tile=True), [0]),
    "FX-CHECK-022": ("FX-CHECK-009 with its colour in capitals, #FF8000: the same.",
                     case(width=4, color="#FF8000", opacity=50), [0]),
    "FX-CHECK-023": ("Width 3, the anchor at (-100, -100) per cent, outside the drawing: the "
                     "checker still lies across the whole layer.",
                     case(anchor=(-100, -100), width=3), [0]),
    "FX-CHECK-024": ("Opacity keyed from 100 at frame 0 to 0 at frame 4 past its end by an ease, "
                     "width 4, none: held at 0, frame 4 is clear everywhere.",
                     case(width=4, opacity=keyed((0, 100, OVERSHOOT), (4, 0))), [0, 4]),
}

INVALID = {
    "FX-CHECK-025": ("Width 0, below 1.", case(width=0)),
    "FX-CHECK-026": ("Height 10001, above 10000.", case(height=10001)),
    "FX-CHECK-027": ("Feather width -1, below 0.", case(feather_width=-1)),
    "FX-CHECK-028": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-CHECK-029": ("Anchor 1001 per cent across, above 1000.", case(anchor=(1001, 50))),
    "FX-CHECK-030": ("Size From \"corner\", not one of its three words.", case(size_from="corner")),
    "FX-CHECK-031": ("Blending mode \"darken\", which this program does not have.",
                     case(blending_mode="darken")),
    "FX-CHECK-032": ("Colour \"#12345\", not six hex digits.", case(color="#12345")),
    "FX-CHECK-033": ("Width keyed to 10001 at frame 4, above 10000.",
                     case(width=keyed((0, 64), (4, 10001)))),
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
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.checkerboard",
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
    (OUT / "expected_checkerboard.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    white, clear = [1.0, 1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 0.0]
    art = plain(case())
    shows = [i for i in range(W * H) if art[i][3] > 0]
    one = c["FX-CHECK-001"]["0"]
    for y in range(H):
        for x in range(W):
            want = white if (x < 8) == (y < 5) else clear
            assert near(one[at(x, y)], want), (x, y)
    two = c["FX-CHECK-002"]["0"]
    for y in range(H):
        for x in range(W):
            on = (math.floor((x - 8) / 4) + math.floor((y - 5) / 4)) % 2 == 0
            assert near(two[at(x, y)], white if on else clear), (x, y)
    # Half a pixel along: the pixels whose centres lie on an edge are half covered.
    three = c["FX-CHECK-003"]["0"]
    assert abs(three[at(8, 2)][3] - 0.5) < 1e-12 and abs(three[at(4, 7)][3] - 0.5) < 1e-12
    assert near(three[at(8, 5)], [0.5] * 4) and near(three[at(9, 6)], white)
    assert c["FX-CHECK-005"]["0"] == c["FX-CHECK-004"]["0"]
    four = c["FX-CHECK-004"]["0"]
    for y in range(H):
        for x in range(W):
            on = (math.floor((x - 8) / 4) + math.floor((y - 5) / 2)) % 2 == 0
            assert near(four[at(x, y)], white if on else clear)
    six = c["FX-CHECK-006"]["0"]
    assert all(near(six[at(x, y)], white if (x + y) % 2 == 1 else clear)
               for x in range(W) for y in range(H))
    # Feather width 2: the pixels either side of an upright edge a quarter and three quarters.
    seven = c["FX-CHECK-007"]["0"]
    assert abs(seven[at(7, 6)][3] - 0.25) < 1e-12 and abs(seven[at(8, 6)][3] - 0.75) < 1e-12
    assert near(seven[at(9, 6)], white) and near(seven[at(9, 4)], clear)
    eight = c["FX-CHECK-008"]["0"]
    assert all(0 < p[3] < 1 for p in eight)
    orange = [srgb_to_linear(v / 255) * 0.5 for v in (255, 128, 0)] + [0.5]
    nine = c["FX-CHECK-009"]["0"]
    assert all(near(nine[i], orange if two[i][3] == 1 else clear) for i in range(W * H))
    assert c["FX-CHECK-022"]["0"] == nine
    ten = c["FX-CHECK-010"]["0"]
    assert all(near(ten[i], white if two[i][3] == 1 else art[i]) for i in range(W * H))
    mult, scr, add = (c[f"FX-CHECK-01{k}"]["0"] for k in (1, 2, 3))
    for i in shows:
        if two[i][3] == 1 and art[i][3] == 1:
            assert all(mult[i][k] <= art[i][k] + 1e-12 for k in range(3))
            assert all(scr[i][k] >= art[i][k] - 1e-12 for k in range(3))
        elif two[i][3] == 0:
            assert near(mult[i], art[i]) and near(scr[i], art[i]) and near(add[i], art[i])
    for fx in ("FX-CHECK-014", "FX-CHECK-015"):
        assert c[fx]["0"] != art and all(near(c[fx]["0"][i], art[i]) for i in range(W * H)
                                         if two[i][3] == 0), fx
    sixteen = c["FX-CHECK-016"]["0"]
    assert all(near(sixteen[i], art[i] if two[i][3] == 1 else clear) for i in range(W * H))
    assert c["FX-CHECK-017"]["0"] == art
    eighteen = c["FX-CHECK-018"]
    assert eighteen["0"] == two
    assert all(near(eighteen["4"][i], clear if two[i][3] == 1 else white) for i in range(W * H))
    nineteen = c["FX-CHECK-019"]
    assert nineteen["0"] == render(case(width=2), 0) and nineteen["4"] == render(case(width=8), 0)
    assert nineteen["2"] == render(case(width=5), 0)
    moved = c["FX-CHECK-020"]["0"]
    assert all(moved[at(x, y)] == ten[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert c["FX-CHECK-021"]["0"] == two
    far = c["FX-CHECK-023"]["0"]
    assert any(p[3] == 1 for p in far) and any(p[3] == 0 for p in far)
    assert ease_ok(c["FX-CHECK-024"])
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


def ease_ok(frames):
    return frames["0"] == render(case(width=4), 0) and all(p == [0.0] * 4 for p in frames["4"])


if __name__ == "__main__":
    main()
