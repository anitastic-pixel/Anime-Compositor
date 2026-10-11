"""Strobe Light, worked a second way.

D-466 adds `core.strobe_light`, after After Effects' Strobe Light (Stylize): at regular or random
moments the layer is worked on with a colour, or made clear, for a short while. What is known
(the "Strobe Light--AE" page of Adobe Premiere 6.0's help, which carries After Effects' effect,
and After Effects' own control names) is: Strobe Color is the colour worked with; Blend With
Original at 0 shows the strobe at full strength and higher values weaken it; Strobe Duration is
how long a strobe lasts, in seconds; Strobe Period is the time from one strobe's start to the
next's, and a period below the duration strobes all the time; Random Strobe Probability is the
chance that any given frame strobes; Strobe chooses Operates On Color Only, which works the
colour channels, or Makes Layer Transparent; Strobe Operator is the arithmetic used on the
colour, Copy as it starts; and Random Seed varies the random strobes. The operator list past
Copy, what each does, the ranges and the starting values past Copy are this program's own, and
nothing is ported.

The rule. The input as it reaches the effect is w by h pixels, premultiplied, in the working
(linear) space.

1. The time is t, the layer's key time from its in point in seconds, and the count is k =
   floor of the same in frames. The strobe is on when the period P is at most the duration D,
   or when t - P floor(t / P) < D; or when the random draw for k is below the probability: the
   hash of (k, s), s the seed's floor, each as 32 unsigned bits, h = k 0x9E3779B1 xor s
   0x85EBCA77, h = h xor (h >> 15), h = h 0x2C1B3C6D, h = h xor (h >> 12), h = h 0x297A2D39, h = h
   xor (h >> 15), all kept to 32 bits, is on when h / 2^32 < probability / 100.
2. When it is off, the output is the input.
3. When it is on, with b = blend_with_original / 100: Makes Layer Transparent scales every
   channel of every pixel by b, so the layer fades to clear. Operates On Color Only takes each
   pixel's straight colour c (its colour over its alpha; a clear pixel stays clear) and the
   strobe colour s, in the working space, works r per channel, holds it in 0 to 1, and gives
   (1 - b) r + b c times the pixel's own alpha, the alpha kept:
   Copy s; Add c + s; Subtract c - s; Multiply c s; Difference |c - s|; And, Or and Xor the
   bitwise operation on floor(255 c + 1/2) and floor(255 s + 1/2), c and s held in 0 to 1
   first, over 255; Minimum and Maximum the smaller
   and the larger, channel by channel; Darken and Lighten the whole colour, c or s, whose
   brightness 0.2126 r + 0.7152 g + 0.0722 b is lower (Darken) or higher (Lighten), c on a tie,
   then held in 0 to 1 too.

Settings: `strobe_color` a colour, white; `blend_with_original` 0 to 100, 0; `strobe_duration`
0 to 100 seconds, 0.1; `strobe_period` 0 to 100 seconds, 1; `random_strobe_probability` 0 to
100 per cent, 0; `strobe` "color_only" or "transparent", "color_only"; `strobe_operator` one of
the twelve words below, "copy"; `random_seed` 0 to 100000, its whole part counted, 0. Words are
kept as written, so a wrong one is reported. The numbers can be keyed.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10, 24 frames a second and 30 frames long, holding
Motion Tile's drawing, 6 pixels by 4, its top left corner at (5, 3), unmoved unless the case
says. The drawing goes into `Fixtures/strobe_light/media`, the projects into
`Fixtures/strobe_light`, and the expected frames into
`Fixtures/strobe_light/expected_strobe_light.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/strobe_light_reference.py
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
from drop_shadow_reference import drop_shadow, frame  # noqa: E402
from motion_tile_reference import DRAWINGS, DW, DH, SHADOW, drawn_layer  # noqa: E402
from blobbylize_reference import hex_linear  # noqa: E402

W, H = R.W, R.H
FPS, LONG = 24, 30
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "strobe_light"
TOLERANCE = 2e-5  # document 25's default for a filter
OPERATORS = ("copy", "add", "subtract", "multiply", "difference", "and", "or", "xor",
             "minimum", "maximum", "darken", "lighten")
STROBES = ("color_only", "transparent")
WORDS = ("strobe_color", "strobe", "strobe_operator")
RANGES = {"blend_with_original": (0, 100), "strobe_duration": (0, 100),
          "strobe_period": (0, 100), "random_strobe_probability": (0, 100),
          "random_seed": (0, 100000)}
NAMES = ("strobe_color", "blend_with_original", "strobe_duration", "strobe_period",
         "random_strobe_probability", "strobe", "strobe_operator", "random_seed")
M32 = 0xFFFFFFFF


# --- the rule -------------------------------------------------------------------------------

def draw(k, seed):
    h = ((k & M32) * 0x9E3779B1 ^ (seed & M32) * 0x85EBCA77) & M32
    h ^= h >> 15
    h = (h * 0x2C1B3C6D) & M32
    h ^= h >> 12
    h = (h * 0x297A2D39) & M32
    h ^= h >> 15
    return h


def is_on(s, t, k):
    p, d = s["strobe_period"], s["strobe_duration"]
    if p <= d or t - p * math.floor(t / p) < d:
        return True
    return draw(k, math.floor(s["random_seed"])) / 2 ** 32 \
        < s["random_strobe_probability"] / 100


def luma(c):
    return 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2]


def level(v):
    """A channel as a whole number of 255, held in 0 to 1 first and rounded half up."""
    return math.floor(255 * min(max(v, 0.0), 1.0) + 0.5)


def operate(op, c, s):
    if op == "darken":
        r = s if luma(s) < luma(c) else c
    elif op == "lighten":
        r = s if luma(s) > luma(c) else c
    elif op in ("and", "or", "xor"):
        f = {"and": lambda x, y: x & y, "or": lambda x, y: x | y, "xor": lambda x, y: x ^ y}[op]
        r = [f(level(a), level(b)) / 255 for a, b in zip(c, s)]
    else:
        f = {"copy": lambda a, b: b, "add": lambda a, b: a + b, "subtract": lambda a, b: a - b,
             "multiply": lambda a, b: a * b, "difference": lambda a, b: abs(a - b),
             "minimum": min, "maximum": max}[op]
        r = [f(a, b) for a, b in zip(c, s)]
    return [min(max(v, 0.0), 1.0) for v in r]


def strobe_light(layer, s, t, k):
    if not is_on(s, t, k):
        return layer
    b = s["blend_with_original"] / 100
    if s["strobe"] == "transparent":
        return {**layer, "px": [[v * b for v in p] for p in layer["px"]]}
    col = hex_linear(s["strobe_color"])
    out = []
    for p in layer["px"]:
        a = p[3]
        if a <= 0:
            out.append(list(p))
            continue
        c = [v / a for v in p[:3]]
        r = operate(s["strobe_operator"], c, col)
        out.append([((1 - b) * x + b * y) * a for x, y in zip(r, c)] + [a])
    return {**layer, "px": out}


# --- the cases ------------------------------------------------------------------------------

def case(strobe_color="#ffffff", blend_with_original=0, strobe_duration=0.1, strobe_period=1,
         random_strobe_probability=0, strobe="color_only", strobe_operator="copy", random_seed=0,
         shift=0, shadow=False, in_frame=0):
    c = dict(locals())
    c["drawing"] = "patch"
    return c


def numbers(c, frame_no):
    return {k: (c[k] if k in WORDS else min(RANGES[k][1], max(RANGES[k][0],
                                                              value_at(c[k], frame_no))))
            for k in NAMES}


def layer_of(c):
    layer = drawn_layer(c["drawing"])
    if c["shadow"]:
        layer = drop_shadow(layer, SHADOW["color"], SHADOW["opacity"], SHADOW["direction"],
                            SHADOW["distance"], SHADOW["softness"])
    return layer


def render(c, frame_no):
    if frame_no < c["in_frame"]:
        return [[0.0] * 4 for _ in range(W * H)]
    t = (frame_no - c["in_frame"]) / FPS
    return frame(strobe_light(layer_of(c), numbers(c, frame_no), t, frame_no - c["in_frame"]),
                 c["shift"])


def plain(c):
    return frame(drawn_layer(c["drawing"]), c["shift"])


PURPLE = "#8040c0"
CASES = {
    "FX-STROBE-001": ("The settings as they start: white, Copy, a strobe 0.1 seconds long every "
                      "second. Frames 0, 1 and 2 (0 to 0.083 seconds) are white where the drawing "
                      "is; frame 3 (0.125) is the drawing; frame 24 (1 second) is white again.",
                      case(), [0, 2, 3, 24]),
}
for _n, _op in enumerate(OPERATORS[1:], start=2):
    CASES[f"FX-STROBE-{_n:03d}"] = (f"Strobe Operator {_op.title()} with {PURPLE}, frame 0.",
                                    case(PURPLE, strobe_operator=_op), [0])
CASES.update({
    "FX-STROBE-013": ("Copy in red with Blend With Original 50: halfway between the drawing and "
                      "red.", case("#ff0000", blend_with_original=50), [0]),
    "FX-STROBE-014": ("Makes Layer Transparent: frame 0 is empty, frame 3 the drawing.",
                      case(strobe="transparent"), [0, 3]),
    "FX-STROBE-015": ("Makes Layer Transparent with Blend With Original 25: the drawing at a "
                      "quarter strength.", case(strobe="transparent", blend_with_original=25),
                      [0]),
    "FX-STROBE-016": ("Strobe Period 0.05, below the 0.1 duration: on at every frame.",
                      case(strobe_period=0.05), [0, 3, 13]),
    "FX-STROBE-017": ("Strobe Duration 0: never on.", case(strobe_duration=0), [0, 1]),
    "FX-STROBE-018": ("Random Strobe Probability 50, seed 3, no regular strobe (duration 0, "
                      "period 100): some frames strobe and some do not.",
                      case(strobe_duration=0, strobe_period=100, random_strobe_probability=50,
                           random_seed=3), list(range(10))),
    "FX-STROBE-019": ("Random Strobe Probability 50, seed 4: a different choice of frames from "
                      "FX-STROBE-018.",
                      case(strobe_duration=0, strobe_period=100, random_strobe_probability=50,
                           random_seed=4), list(range(10))),
    "FX-STROBE-020": ("Random Strobe Probability 100: every frame strobes.",
                      case(strobe_duration=0, strobe_period=100, random_strobe_probability=100),
                      [0, 3, 7]),
    "FX-STROBE-021": ("Blend With Original keyed from 0 at frame 0 to 100 at frame 4, linear, "
                      "Strobe Period 0.01 so every frame strobes: frame 2 is half strength, frame "
                      "4 the drawing.",
                      case(blend_with_original=keyed((0, 0), (4, 100)), strobe_period=0.01),
                      [0, 2, 4]),
    "FX-STROBE-022": ("FX-STROBE-002 moved three pixels right.",
                      case(PURPLE, strobe_operator="add", shift=3), [0]),
    "FX-STROBE-023": ("After a drop shadow that grows the layer to 10 by 8, Copy in purple: the "
                      "shadow is purple too, at its own alpha.", case(PURPLE, shadow=True), [0]),
    "FX-STROBE-024": ("The layer starting at frame 2: its time counts from there, so frame 2 "
                      "and 4 strobe and frame 5 does not.", case(in_frame=2), [2, 4, 5]),
})

INVALID = {
    "FX-STROBE-025": ("Blend With Original 101, above 100.", case(blend_with_original=101)),
    "FX-STROBE-026": ("Strobe Duration -1, below 0.", case(strobe_duration=-1)),
    "FX-STROBE-027": ("Strobe Operator \"screen\", which is not one.",
                      case(strobe_operator="screen")),
    "FX-STROBE-028": ("Strobe \"flash\", which is not one.", case(strobe="flash")),
    "FX-STROBE-029": ("Strobe Color \"white\", which is not a colour.",
                      case(strobe_color="white")),
    "FX-STROBE-030": ("Random Strobe Probability keyed to 150 at frame 4, above 100.",
                      case(random_strobe_probability=keyed((0, 0), (4, 150)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    assert comp["frame_rate"] == {"numerator": FPS, "denominator": 1}
    comp["duration_frames"] = LONG
    comp["work_area"]["end_frame_exclusive"] = LONG
    layer = comp["layers"][0]
    layer["in_frame"], layer["out_frame"] = c["in_frame"], LONG
    t = layer["transform"]
    t["anchor"] = prop([DW / 2, DH / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    effects = []
    if c["shadow"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.drop_shadow", "enabled": True,
                        "parameters": dict(SHADOW)})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.strobe_light",
                    "enabled": True,
                    "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                   for k in NAMES}})
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
    (OUT / "expected_strobe_light.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    one = lambda fx: c[fx]["0"]  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    layer = drawn_layer("patch")
    art = plain(case())
    LEFT, TOP = layer["left"], layer["top"]
    inside = [at(LEFT + x, TOP + y) for y in range(DH) for x in range(DW)]
    empty = [0.0] * 4
    close = lambda p, q: all(abs(a - b) < 1e-12 for a, b in zip(p, q))  # noqa: E731

    for fx, frames in c.items():
        for px in frames.values():
            assert all(0 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3])
                       for p in px), fx
    k1 = c["FX-STROBE-001"]
    white = [[layer["px"][i][3]] * 3 + [layer["px"][i][3]] for i in range(DW * DH)]
    assert all(close(k1["0"][j], white[i]) for i, j in enumerate(inside))
    assert k1["2"] == k1["0"] and k1["24"] == k1["0"] and k1["3"] == art
    # Every operator draws something different, and none is the drawing.
    ops = [one(f"FX-STROBE-{n:03d}") for n in range(1, 13)]
    assert len({json.dumps(px) for px in ops}) == 12 and art not in ops
    # Spot checks on one pixel against the operators' definitions.
    p0 = layer["px"][0]
    cs = [v / p0[3] for v in p0[:3]]
    pu = hex_linear(PURPLE)
    j0 = inside[0]
    want = {"add": [min(a + b, 1) for a, b in zip(cs, pu)],
            "multiply": [a * b for a, b in zip(cs, pu)],
            "minimum": [min(a, b) for a, b in zip(cs, pu)],
            "maximum": [max(a, b) for a, b in zip(cs, pu)]}
    for n, op in enumerate(OPERATORS[1:], start=2):
        if op in want:
            assert close(one(f"FX-STROBE-{n:03d}")[j0], [v * p0[3] for v in want[op]] + [p0[3]])
    assert operate("and", [1.0, 0.0, 0.5], [0.5, 1.0, 1.0]) == [128 / 255, 0.0, 128 / 255]
    assert operate("darken", [1, 1, 1], [0, 0, 0]) == [0, 0, 0]
    assert operate("lighten", [1, 1, 1], [0, 0, 0]) == [1, 1, 1]
    red = one("FX-STROBE-013")
    assert close(red[j0], [(0.5 * a + 0.5 * b) * p0[3] for a, b in zip([1, 0, 0], cs)] + [p0[3]])
    k14 = c["FX-STROBE-014"]
    assert all(p == empty for p in k14["0"]) and k14["3"] == art
    assert all(close(one("FX-STROBE-015")[j], [v * 0.25 for v in art[j]]) for j in range(W * H))
    k16 = c["FX-STROBE-016"]
    assert k16["0"] == k16["3"] == k16["13"] == k1["0"]
    assert c["FX-STROBE-017"]["0"] == c["FX-STROBE-017"]["1"] == art
    for fx in ("FX-STROBE-018", "FX-STROBE-019"):
        on = [f for f, px in c[fx].items() if px != art]
        assert 2 <= len(on) <= 8 and all(c[fx][f] == k1["0"] for f in on), fx
    on18 = {f for f, px in c["FX-STROBE-018"].items() if px != art}
    on19 = {f for f, px in c["FX-STROBE-019"].items() if px != art}
    assert on18 != on19
    assert all(px == k1["0"] for px in c["FX-STROBE-020"].values())
    k21 = c["FX-STROBE-021"]
    assert k21["0"] == k1["0"] and k21["4"] == art
    assert all(close(k21["2"][j], [(a + b) / 2 for a, b in zip(k1["0"][j], art[j])])
               for j in range(W * H))
    moved, two = one("FX-STROBE-022"), one("FX-STROBE-002")
    assert all(moved[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    shadowed = layer_of(case(shadow=True))
    assert (shadowed["left"], shadowed["top"], shadowed["w"], shadowed["h"]) == (3, 1, 10, 8)
    sh = one("FX-STROBE-023")
    assert all(p[3] == 0 or close([v / p[3] for v in p[:3]], pu) for p in sh)
    k24 = c["FX-STROBE-024"]
    assert k24["2"] == k24["4"] == k1["0"] and k24["5"] == art
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


if __name__ == "__main__":
    main()
