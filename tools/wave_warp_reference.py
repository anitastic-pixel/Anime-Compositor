"""Wave warp, worked a second way.

D-149 adds `core.wave_warp`, a regular wave running across a drawing: a flag in the wind, a
reflection in water, a wobble on a title. The wave runs along `direction` and pushes each pixel
across it, to one side and the other, by up to `height` pixels, one whole wave every `width`
pixels. `shape` "sine" gives a smooth wave, "triangle" a zigzag of straight sides; `phase` slides
the wave along, and `speed` adds that many degrees of phase every frame, so with speed not 0 the
wave travels from frame to frame. With `edges` "transparent" the layer grows by the height on
every side so a picture pushed past its edge is kept, and a push that reaches past the picture
reads emptiness; with "repeat" it does not grow and a push past the edge reads the nearest edge
pixel. It is this program's own method, modelled on After Effects' Wave Warp in spirit and not
claimed to match it; nothing is ported. Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

D-745 adds two of After Effects' choices, keeping every case above as it was. `shape` "circle"
bends the wave into half circles: each half wave is the top or the bottom of a circle, so the
wave rises straight up from the middle line and is flat at its crest. `pinning` holds edges of
the drawing still: "none" (the start, and what a file without it reads), "all_edges",
"left_edge", "top_edge", "right_edge", "bottom_edge", "horizontal_edges" (top and bottom) or
"vertical_edges" (left and right). Each pinned edge scales the push down to nothing at that edge,
growing in a straight line to the whole push a quarter of the drawing's width (left, right) or
height (top, bottom) in from it; two pinned edges multiply. Width keeps its meaning, one wave
every `width` pixels, about twice After Effects' Wave Width for the same look.

D-755 adds After Effects' remaining wave types, keeping every shape above as it was. With u the
place in the whole wave, u = a / 2 pi - floor(a / 2 pi), from 0 to 1: "square" is 1 for u under
a half and -1 from a half on; "sawtooth" 2u - 1, rising straight and dropping at the wave's
start; "semicircle" 1 - 2 sqrt(1 - (2u - 1)^2), one round arch a wave reaching -1 at its middle,
the arches meeting in points at 1. Each is placed against the sine as After Effects' stills show
it (Motion Academy, all at the composition's frame 0: the square's halves on the sine's halves,
the sawtooth's drop where the sine rises through nothing, the semicircle's points there too).
"uncircle", which no tutorial shows, is the semicircle turned over: 2 sqrt(1 - (2u - 1)^2) - 1.
After Effects' "noise" and "smooth_noise" are random; ours are fixed, the same at every frame for
the same settings, and travel with the phase as the others do: a level r(kind, j) from -1 to 1 at
each knot j, j = floor(n) with n = 16 a / 2 pi (noise, kind 1, straight lines between knots) or
n = 2 a / 2 pi (smooth noise, kind 2, joined by f^2 (3 - 2 f), f = n - j). r(kind, j) =
mix(mix(kind) xor j) / 2^32 * 2 - 1 in 32-bit whole numbers, mix being Scribble's 32-bit mix
x ^= x >> 16, x *= 0x7feb352d, x ^= x >> 15, x *= 0x846ca68b, x ^= x >> 16, with j taken as its
32-bit two's complement.

The rule. With height 0 the output is the input and nothing grows. Otherwise, with
"transparent", the input grows by g = ceil(height) transparent pixels on every side; with
"repeat", g = 0. t = u(direction) = (sin, -cos) of the direction, exact at quarter turns, and
n = (-t.y, t.x), a quarter turn clockwise from t. Each output pixel, centre (X, Y) in the
drawing's own space (a grown pixel has negative coordinates), takes s = t.x X + t.y Y,
phi = radians(phase + speed * frame), frame the composition frame, a hidden value, and
w = sin(2 pi s / width + phi) for "sine" or (2 / pi) asin(sin(2 pi s / width + phi)) for
"triangle"; for "circle", with a = 2 pi s / width + phi, m = floor(a / pi) and u = a / pi - m,
w = sqrt(1 - (2u - 1)^2), negative where m is odd. The pin p is 1 with pinning "none"; else the
product, over the pinned edges, of clamp(4 d / size, 0, 1), d the distance of (X, Y) in from
that edge of the drawing (X, Y, its width - X, its height - Y) and size the drawing's width or
height across it. The output is document 21's bilinear sample of the input at
(X, Y) - height p w n, transparent outside it, or with "repeat" D-109's held sample, the point
first held inside the input's pixel centres.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says, drawn below. The drawing goes into `Fixtures/wave_warp/media`, the projects into
`Fixtures/wave_warp`, and the expected frames into `Fixtures/wave_warp/expected_wave_warp.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/wave_warp_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
from drop_shadow_reference import unit  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "wave_warp"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"height": (0, 1000), "width": (1, 10000), "direction": (-3600, 3600),
          "speed": (-360, 360), "phase": (-100000, 100000)}
NUMBERS = ("height", "width", "direction", "speed", "phase")
NAMES = ("shape",) + NUMBERS + ("edges",)
EMPTY = [0.0] * 4
# D-745: the edges each pinning holds, l t r b.
PINS = {"none": "", "all_edges": "ltrb", "left_edge": "l", "top_edge": "t", "right_edge": "r",
        "bottom_edge": "b", "horizontal_edges": "tb", "vertical_edges": "lr"}


# --- the rule -------------------------------------------------------------------------------

def growth(height, edges):
    return math.ceil(height) if edges == "transparent" and height > 0 else 0


def wave(shape, width, direction, speed, phase, frame_no, X, Y):
    """w at the point (X, Y) of the drawing's own space, in -1..1."""
    t = unit(direction)
    a = 2 * math.pi * (t[0] * X + t[1] * Y) / width + math.radians(phase + speed * frame_no)
    if shape == "circle":
        m = math.floor(a / math.pi)
        u = a / math.pi - m
        v = math.sqrt(max(0.0, 1 - (2 * u - 1) ** 2))
        return -v if m % 2 else v
    if shape in MORE:
        return more(shape, a)
    return math.sin(a) if shape == "sine" else 2 / math.pi * math.asin(math.sin(a))


# D-755: the remaining wave types.
MORE = ("square", "sawtooth", "semicircle", "uncircle", "noise", "smooth_noise")
M32 = 0xFFFFFFFF


def mix(x):
    """Scribble's 32-bit mix."""
    x ^= x >> 16
    x = (x * 0x7FEB352D) & M32
    x ^= x >> 15
    x = (x * 0x846CA68B) & M32
    return x ^ (x >> 16)


def knot(kind, j):
    """The noise level at knot j, -1 to 1."""
    return mix(mix(kind) ^ (int(j) & M32)) / 4294967296.0 * 2 - 1


def more(shape, a):
    q = a / (2 * math.pi)
    u = q - math.floor(q)
    arch = math.sqrt(max(0.0, 1 - (2 * u - 1) ** 2))
    if shape == "square":
        return 1.0 if u < 0.5 else -1.0
    if shape == "sawtooth":
        return 2 * u - 1
    if shape == "semicircle":
        return 1 - 2 * arch
    if shape == "uncircle":
        return 2 * arch - 1
    per, kind = (16, 1) if shape == "noise" else (2, 2)
    n = q * per
    j = math.floor(n)
    f = n - j
    p, nx = knot(kind, j), knot(kind, j + 1)
    return p + (nx - p) * f if shape == "noise" else p + (nx - p) * f * f * (3 - 2 * f)


def pin(pinning, w, h, X, Y):
    """D-745: the share of the push kept at (X, Y) of a drawing w by h."""
    p = 1.0
    for e in PINS[pinning]:
        d, size = {"l": (X, w), "t": (Y, h), "r": (w - X, w), "b": (h - Y, h)}[e]
        p *= min(1.0, max(0.0, 4 * d / size))
    return p


def push(shape, height, width, direction, speed, phase, frame_no, X, Y, p=1.0):
    """How far the sample point lies from (X, Y): -height p w n."""
    t = unit(direction)
    k = -height * p * wave(shape, width, direction, speed, phase, frame_no, X, Y)
    return k * -t[1], k * t[0]


def warped(layer, shape, height, width, direction, speed, phase, edges, frame_no, x, y,
           pinning="none"):
    """The output at pixel (x, y) of the drawing's own space; empty outside the grown layer."""
    if height == 0:
        return layer["px"][y * layer["w"] + x] if 0 <= x < layer["w"] and 0 <= y < layer["h"] \
            else EMPTY
    g = growth(height, edges)
    if not (-g <= x < layer["w"] + g and -g <= y < layer["h"] + g):
        return EMPTY
    X, Y = x + 0.5, y + 0.5
    p = pin(pinning, layer["w"], layer["h"], X, Y)
    dx, dy = push(shape, height, width, direction, speed, phase, frame_no, X, Y, p)
    return bilinear(layer, X + dx, Y + dy, edges)


# --- the drawing ----------------------------------------------------------------------------

LINE, SKIN, SOFT_SKIN, NONE = R.LINE, R.SKIN, (246, 214, 190, 128), S.NONE
BAND = (58, 111, 216, 255)  # #3a6fd8, the ball's blue band


def stripes(x, y):
    """Rows 0 to 8, columns 0 to 13: upright stripes two pixels wide, skin and line by turns,
    crossed by a blue band in rows 4 and 5, so a push either way shows; the stripes touch the
    drawing's top and left edges. Column 14 is the skin at half covering; column 15 and row 9
    are empty."""
    if x == 15 or y == 9:
        return NONE
    if x == 14:
        return SOFT_SKIN
    if y in (4, 5):
        return BAND
    return LINE if (x // 2) % 2 else SKIN


DRAWINGS = {"stripes": [[stripes(x, y) for x in range(W)] for y in range(H)]}


def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


# --- the cases ------------------------------------------------------------------------------

def case(shape="sine", height=10, width=40, direction=90, speed=0, phase=0,
         edges="transparent", shift=0, pinning=None):
    """`pinning` None: the file has none, as every file before D-745, and reads as "none"."""
    return {"drawing": "stripes", "shape": shape, "height": height, "width": width,
            "direction": direction, "speed": speed, "phase": phase, "edges": edges,
            "shift": shift, "pinning": pinning}


def names(c):
    return NAMES + (("pinning",) if c["pinning"] is not None else ())


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    layer = drawn_layer(c["drawing"])
    n = [held(c, k, frame_no) for k in NUMBERS]
    return [warped(layer, c["shape"], *n, c["edges"], frame_no, x - c["shift"], y,
                   c["pinning"] or "none")
            for y in range(H) for x in range(W)]


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(height=0, shift=c["shift"]), 0)


WAVE = {"height": 2, "width": 8}  # two pixels each way, a wave every eight: the band waves

CASES = {
    "FX-WAVE-001": ("The settings as they start: shape sine, height 10, width 40, direction 90, "
                    "speed 0, phase 0, edges transparent. The wave runs to the right and pushes "
                    "each column down by 10 sin(2 pi X / 40), more than the drawing's height "
                    "across its middle: column 0 is pushed down by 0.8 pixels, column 3 by 5.2, "
                    "columns 7 to 12 by more than 9, so their rows 0 to 8 are empty and row 9 "
                    "keeps only part of the drawing's top row, and column 15 by 6.5; the frames are "
                    "the same, the wave standing still at speed 0.",
                    case(), [0, 4]),
    "FX-WAVE-002": ("Height 0: the drawing, untouched, and nothing grows.",
                    case(height=0), [0, 2]),
    "FX-WAVE-003": ("Height 2, width 8: a wave every eight pixels, each column pushed down or "
                    "up by 2 sin(2 pi X / 8), so the blue band waves: columns 1 and 2 carry it "
                    "down into rows 6 and 7 and columns 5 and 6 up into rows 2 and 3. Where a "
                    "column is pushed down its top pixel reads past the drawing's top edge and "
                    "loses covering, and row 9 takes the drawing's bottom row. The frames are "
                    "the same.",
                    case(**WAVE), [0, 2, 4]),
    "FX-WAVE-004": ("FX-WAVE-003 with shape triangle: the zigzag pushes each pixel no further "
                    "than the sine does, so the band's waves are flatter.",
                    case(shape="triangle", **WAVE), [0]),
    "FX-WAVE-005": ("Direction 0: the wave runs up the drawing and pushes each row left or "
                    "right, so the upright stripes wave while the band, pushed along itself, "
                    "stays blue from column 3 to column 11.",
                    case(direction=0, **WAVE), [0]),
    "FX-WAVE-006": ("Direction 45: the wave runs up and to the right and pushes along the "
                    "other diagonal, so both the stripes and the band bend.",
                    case(direction=45, **WAVE), [0]),
    "FX-WAVE-007": ("Direction 450, a whole turn past 90: FX-WAVE-003 exactly.",
                    case(direction=450, **WAVE), [0]),
    "FX-WAVE-008": ("Direction 270 at phase 0: the wave runs left and pushes the other way, "
                    "and a sine run backwards is the same sine turned over, so it is "
                    "FX-WAVE-003.",
                    case(direction=270, **WAVE), [0]),
    "FX-WAVE-009": ("Phase 90: the wave slid a quarter of the way along, so each column is "
                    "pushed by 2 cos(2 pi X / 8) and the band waves two pixels further left "
                    "than FX-WAVE-003's.",
                    case(phase=90, **WAVE), [0]),
    "FX-WAVE-010": ("Speed 45: the wave travels. Frame 0 is FX-WAVE-003, frame 2 is phase 90, "
                    "FX-WAVE-009, and frame 4 is phase 180, where every push is FX-WAVE-003's "
                    "the other way.",
                    case(speed=45, **WAVE), [0, 2, 4]),
    "FX-WAVE-011": ("FX-WAVE-003 with edges repeat: the layer does not grow, and a push past "
                    "the drawing's top edge reads the drawing's top row, so where FX-WAVE-003's "
                    "top pixels lose covering these keep it; every other pixel is "
                    "FX-WAVE-003's.",
                    case(edges="repeat", **WAVE), [0]),
    "FX-WAVE-012": ("Width 8, height keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 "
                    "is the drawing, frame 2 is height 2, FX-WAVE-003, and frame 4 height 4.",
                    case(height=keyed((0, 0), (4, 4)), width=8), [0, 2, 4]),
    "FX-WAVE-013": ("Height 2, width 8, speed keyed from 0 at frame 0 to 360 at frame 4, eased "
                    "past its end (477 at frame 2): frame 0 is FX-WAVE-003, and frame 2 is held "
                    "at 360, so it is speed 360's frame 2.",
                    case(speed=keyed((0, 0, OVERSHOOT), (4, 360)), **WAVE), [0, 2, 4]),
    "FX-WAVE-014": ("FX-WAVE-005 moved three pixels right: the wave is worked in the drawing's "
                    "own space, so it moves with it; the layer grew by 2, and the two columns "
                    "left of the drawing show the grown pixels, into which the stripes are "
                    "pushed, while the column left of those stays empty.",
                    case(direction=0, shift=3, **WAVE), [0]),
    "FX-WAVE-015": ("FX-WAVE-014 with edges repeat: nothing grows, so the three columns left "
                    "of the drawing stay empty.",
                    case(direction=0, edges="repeat", shift=3, **WAVE), [0]),
    "FX-WAVE-016": ("Height 2.5, width 8, direction 0, moved three pixels right: the layer "
                    "grows by 3, the height rounded up, and the stripes pushed furthest left "
                    "reach into the third column left of the drawing.",
                    case(height=2.5, width=8, direction=0, shift=3), [0]),
    # D-745.
    "FX-WAVE-025": ("FX-WAVE-003 with shape circle: each half wave is half a circle, so the "
                    "push rises steeply from nothing where the sine crosses the middle and "
                    "stays near its full 2 pixels across most of each half wave.",
                    case(shape="circle", **WAVE), [0]),
    "FX-WAVE-026": ("FX-WAVE-025 at direction 0: the half circles run up the drawing and push "
                    "each row left or right.",
                    case(shape="circle", direction=0, **WAVE), [0]),
    "FX-WAVE-027": ("FX-WAVE-003 with pinning all_edges: the push fades to nothing at every "
                    "edge of the drawing, reaching its whole size 4 pixels in from the left "
                    "and right and 2.5 from the top and bottom, so the drawing's outline "
                    "stays put and only its middle waves.",
                    case(pinning="all_edges", **WAVE), [0]),
    "FX-WAVE-028": ("Pinning left_edge: column 0 is pushed an eighth as far as in "
                    "FX-WAVE-003, column 1 three eighths, column 2 five, column 3 seven, and "
                    "columns 4 on are FX-WAVE-003's.",
                    case(pinning="left_edge", **WAVE), [0]),
    "FX-WAVE-029": ("Pinning top_edge: row 0 is pushed a fifth as far as in FX-WAVE-003, row 1 "
                    "three fifths, and rows 2 on are FX-WAVE-003's.",
                    case(pinning="top_edge", **WAVE), [0]),
    "FX-WAVE-030": ("Pinning right_edge: the drawing's right edge, past column 15, holds still; "
                    "columns 12 to 15 are pushed less, and columns 0 to 11 are FX-WAVE-003's.",
                    case(pinning="right_edge", **WAVE), [0]),
    "FX-WAVE-031": ("Pinning bottom_edge: rows 8 and 9 are pushed less, and rows 0 to 7 are "
                    "FX-WAVE-003's.",
                    case(pinning="bottom_edge", **WAVE), [0]),
    "FX-WAVE-032": ("Pinning horizontal_edges, the top and the bottom: FX-WAVE-029 and "
                    "FX-WAVE-031 together.",
                    case(pinning="horizontal_edges", **WAVE), [0]),
    "FX-WAVE-033": ("Pinning vertical_edges, the left and the right: FX-WAVE-028 and "
                    "FX-WAVE-030 together.",
                    case(pinning="vertical_edges", **WAVE), [0]),
    "FX-WAVE-034": ("Pinning none written in the file: FX-WAVE-003 exactly.",
                    case(pinning="none", **WAVE), [0]),
    "FX-WAVE-035": ("FX-WAVE-014 with pinning all_edges: the drawing moved three pixels right, "
                    "its edges still held where the drawing is, so the grown columns left of "
                    "it stay empty.",
                    case(direction=0, shift=3, pinning="all_edges", **WAVE), [0]),
    "FX-WAVE-036": ("FX-WAVE-025 with pinning all_edges and speed 45: the half circles travel "
                    "while the edges stay put; frame 2 is a quarter wave on.",
                    case(shape="circle", speed=45, pinning="all_edges", **WAVE), [0, 2]),
    # D-755.
    "FX-WAVE-039": ("FX-WAVE-003 with shape square: columns 0 to 3 are pushed down the whole 2 "
                    "pixels and columns 4 to 7 up, and again from column 8, so the band steps "
                    "down and up in blocks four pixels wide.",
                    case(shape="square", **WAVE), [0]),
    "FX-WAVE-040": ("FX-WAVE-003 with shape sawtooth: the push climbs evenly across each wave of "
                    "eight columns, from 1.75 pixels one way at column 0 to 1.75 the other at "
                    "column 7, then drops back at column 8.",
                    case(shape="sawtooth", **WAVE), [0]),
    "FX-WAVE-041": ("FX-WAVE-003 with shape semicircle: one round arch a wave, pushing up most "
                    "at columns 3 and 4 and 11 and 12, and the arches meeting in points at "
                    "columns 0, 7, 8 and 15, all but unmoved.",
                    case(shape="semicircle", **WAVE), [0]),
    "FX-WAVE-042": ("FX-WAVE-041 with shape uncircle: every push the other way.",
                    case(shape="uncircle", **WAVE), [0]),
    "FX-WAVE-043": ("Shape noise, height 2, width 40: a fixed uneven push, a new level every "
                    "two and a half pixels with straight lines between.",
                    case(shape="noise", height=2, width=40), [0]),
    "FX-WAVE-044": ("Shape smooth_noise, height 2, width 8: a fixed uneven push, a new level "
                    "every four pixels joined smoothly.",
                    case(shape="smooth_noise", **WAVE), [0]),
    "FX-WAVE-045": ("FX-WAVE-043 with speed 45: the noise travels, the same at every frame for "
                    "the same settings; frame 2 is phase 90, the pattern a quarter of 40 pixels "
                    "on.",
                    case(shape="noise", height=2, width=40, speed=45), [0, 2]),
    "FX-WAVE-046": ("FX-WAVE-041 with pinning all_edges and direction 0: the arches run up the "
                    "drawing, pushing each row left or right, held still at its edges.",
                    case(shape="semicircle", direction=0, pinning="all_edges", **WAVE), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-WAVE-017": ("Height 1001, above 1000.", case(height=1001)),
    "FX-WAVE-018": ("Width 0, below 1.", case(width=0)),
    "FX-WAVE-019": ("Direction 3601, above 3600.", case(direction=3601)),
    "FX-WAVE-020": ("Speed -361, below -360.", case(speed=-361)),
    "FX-WAVE-021": ("Phase 100001, above 100000.", case(phase=100001)),
    "FX-WAVE-022": ("Height keyed to 1500 at frame 4.", case(height=keyed((0, 10), (4, 1500)))),
    # D-755: square became a choice, so the word is now one that never was.
    "FX-WAVE-023": ("Shape \"hexagon\", which is not a choice.", case(shape="hexagon")),
    "FX-WAVE-024": ("Edges \"Repeat\": the word is exact, so a capital is not the choice.",
                    case(edges="Repeat")),
    "FX-WAVE-037": ("Pinning \"center\", which is not a choice.", case(pinning="center")),
    "FX-WAVE-038": ("Pinning \"All Edges\", the name on the panel, not the word.",
                    case(pinning="All Edges")),
    # D-755.
    "FX-WAVE-047": ("Shape \"Smooth Noise\", the name on the panel, not the word.",
                    case(shape="Smooth Noise")),
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
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.wave_warp", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in names(c)}}]
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

    (OUT / "expected_wave_warp.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    blue = lambda p: p[2] > p[0] + 0.1  # noqa: E731  the band, not skin or line
    band = R.working(BAND)

    # The rule's own pieces.
    assert growth(10, "transparent") == 10 and growth(2.5, "transparent") == 3
    assert growth(10, "repeat") == 0 and growth(0, "transparent") == 0
    assert unit(90) == (1.0, 0.0) and unit(450) == unit(90) and unit(0) == (0.0, -1.0)
    assert abs(wave("sine", 8, 90, 0, 0, 0, 2, 7) - 1) < 1e-15  # a quarter wave: the crest
    assert abs(wave("triangle", 8, 90, 0, 0, 0, 1, 7) - 0.5) < 1e-15  # half way up a side
    assert abs(wave("triangle", 8, 90, 0, 0, 0, 2, 7) - 1) < 1e-7  # the same crest
    assert push("sine", 2, 8, 90, 0, 0, 0, 2, 7) == (0.0, -2.0)  # direction 90: pushed down
    dx, dy = push("sine", 2, 8, 0, 0, 0, 0, 7, 6)  # direction 0: across, s = -Y
    assert dy == 0 and abs(dx + 2) < 1e-15
    for X in [x + 0.5 for x in range(-3, W + 3)]:
        for Y in [y + 0.5 for y in range(-3, H + 3)]:
            s = push("sine", 2, 8, 90, 0, 0, 0, X, Y)
            tr = push("triangle", 2, 8, 90, 0, 0, 0, X, Y)
            assert abs(tr[1]) <= abs(s[1]) + 1e-12 and tr[0] == s[0] == 0
            back = push("sine", 2, 8, 90, 0, 180, 0, X, Y)
            assert abs(back[1] + s[1]) < 1e-12  # phase 180: every push the other way

    one = c["FX-WAVE-001"]
    assert one["0"] == one["4"] != drawn
    down = [-push("sine", 10, 40, 90, 0, 0, 0, x + 0.5, 0)[1] for x in range(W)]
    assert abs(down[0] - 0.785) < 1e-3 and abs(down[3] - 5.225) < 1e-3
    assert abs(down[15] - 6.494) < 1e-3 and all(d > 9 for d in down[7:13])
    for x in range(7, 13):
        assert all(one["0"][at(x, y)] == EMPTY for y in range(9))
        assert 0 < one["0"][at(x, 9)][3] < 1 - 1e-3  # part of row 0
    assert c["FX-WAVE-002"]["0"] == drawn == c["FX-WAVE-002"]["2"]
    three = c["FX-WAVE-003"]
    f3 = three["0"]
    assert f3 == three["2"] == three["4"] != drawn
    for x in (1, 2):
        assert blue(f3[at(x, 6)]) and blue(f3[at(x, 7)]) and not blue(f3[at(x, 4)])
        assert f3[at(x, 0)][3] < 1 - 1e-3  # pushed down: reads past the top edge
    for x in (5, 6):
        assert blue(f3[at(x, 2)]) and blue(f3[at(x, 3)]) and not blue(f3[at(x, 5)])
    assert any(f3[at(x, 9)][3] > 0.5 for x in range(W))  # the bottom row carried into row 9
    assert c["FX-WAVE-004"]["0"] != f3
    five = c["FX-WAVE-005"]["0"]
    for x in range(3, 12):
        for y in (4, 5):
            assert near([five[at(x, y)]], [band], 1e-9), (x, y)
    assert five != f3 and five != drawn
    assert c["FX-WAVE-006"]["0"] not in (f3, five, drawn)
    assert c["FX-WAVE-007"]["0"] == f3
    assert near(c["FX-WAVE-008"]["0"], f3)
    nine = c["FX-WAVE-009"]["0"]
    assert nine != f3
    for x in (3, 4):  # cos: pushed up at X 2..6, most at 4: the band rises two pixels sooner
        assert blue(nine[at(x, 2)]) and blue(nine[at(x, 3)])
    ten = c["FX-WAVE-010"]
    assert ten["0"] == f3 and ten["2"] == nine
    assert ten["4"] == render(case(phase=180, **WAVE), 0) != f3
    eleven = c["FX-WAVE-011"]["0"]
    layer = drawn_layer("stripes")
    same = differ = 0
    for y in range(H):
        for x in range(W):
            px, py = push("sine", 2, 8, 90, 0, 0, 0, x + 0.5, y + 0.5)
            sy = y + 0.5 + py
            if 0.5 <= sy <= H - 0.5:
                assert near([eleven[at(x, y)]], [f3[at(x, y)]]), (x, y)
                same += 1
            elif sy < 0.5 and x < 15:
                assert eleven[at(x, y)][3] > f3[at(x, y)][3] + 1e-6, (x, y)
                differ += 1
    assert same > 100 and differ > 0
    assert all(abs(eleven[at(x, 0)][3] - layer["px"][at(x, 0)][3]) < 1e-12 for x in range(W))
    twelve = c["FX-WAVE-012"]
    assert twelve["0"] == drawn and twelve["2"] == f3
    assert twelve["4"] == render(case(height=4, width=8), 0) != f3
    thirteen = c["FX-WAVE-013"]
    assert value_at(keyed((0, 0, OVERSHOOT), (4, 360)), 2) > 360
    assert thirteen["0"] == f3
    assert thirteen["2"] == render(case(speed=360, **WAVE), 2)
    assert thirteen["4"] == render(case(speed=360, **WAVE), 4)
    fourteen, fifteen = c["FX-WAVE-014"]["0"], c["FX-WAVE-015"]["0"]
    repeat0 = render(case(direction=0, edges="repeat", **WAVE), 0)
    for x in range(3, W):
        for y in range(H):
            assert fourteen[at(x, y)] == five[at(x - 3, y)]
            assert fifteen[at(x, y)] == repeat0[at(x - 3, y)]
    assert all(fourteen[at(0, y)] == EMPTY for y in range(H))
    assert any(fourteen[at(x, y)][3] > 0 for x in (1, 2) for y in range(H))  # grown, shown
    assert all(fifteen[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    assert any(c["FX-WAVE-016"]["0"][at(0, y)][3] > 0.1 for y in range(H))
    # D-745: the circle and the pins.
    for a in (0.3, 1.0, 2.0, 2.9, 4.0, 5.5, -1.0):
        X = a * 8 / (2 * math.pi)  # width 8, direction 90: a = 2 pi X / 8
        v, sv = wave("circle", 8, 90, 0, 0, 0, X, 0), math.sin(a)
        assert abs(v) >= abs(sv) - 1e-12 and (v > 0) == (sv > 0), a  # rounder, same side
        assert abs(v * v + (2 * (a / math.pi - math.floor(a / math.pi)) - 1) ** 2 - 1) < 1e-12
    assert abs(wave("circle", 8, 90, 0, 0, 0, 2, 0) - 1) < 1e-12  # the crest where the sine's is
    assert wave("circle", 8, 90, 0, 0, 0, 0, 0) == 0
    assert pin("none", W, H, 0.5, 0.5) == 1
    assert pin("left_edge", W, H, 0.5, 5) == 0.125 and pin("left_edge", W, H, 3.5, 5) == 0.875
    assert pin("left_edge", W, H, 4.5, 5) == 1 and pin("left_edge", W, H, -0.5, 5) == 0
    assert pin("top_edge", W, H, 8, 0.5) == 0.2 and pin("top_edge", W, H, 8, 1.5) == 0.6
    assert pin("right_edge", W, H, 15.5, 5) == 0.125 and pin("bottom_edge", W, H, 8, 9.5) == 0.2
    assert pin("all_edges", W, H, 0.5, 0.5) == 0.125 * 0.2
    for X in [x + 0.5 for x in range(W)]:
        for Y in [y + 0.5 for y in range(H)]:
            assert pin("horizontal_edges", W, H, X, Y) == \
                pin("top_edge", W, H, X, Y) * pin("bottom_edge", W, H, X, Y)
            assert pin("vertical_edges", W, H, X, Y) == \
                pin("left_edge", W, H, X, Y) * pin("right_edge", W, H, X, Y)
    assert c["FX-WAVE-025"]["0"] not in (f3, drawn)
    assert c["FX-WAVE-026"]["0"] not in (five, drawn)
    allp = c["FX-WAVE-027"]["0"]
    assert allp not in (f3, drawn)
    left = c["FX-WAVE-028"]["0"]
    assert all(left[at(x, y)] == f3[at(x, y)] for x in range(4, W) for y in range(H))
    assert any(left[at(x, y)] != f3[at(x, y)] for x in range(4) for y in range(H))
    top = c["FX-WAVE-029"]["0"]
    assert all(top[at(x, y)] == f3[at(x, y)] for x in range(W) for y in range(3, H))
    right = c["FX-WAVE-030"]["0"]
    assert all(right[at(x, y)] == f3[at(x, y)] for x in range(12) for y in range(H))
    bottom = c["FX-WAVE-031"]["0"]
    assert all(bottom[at(x, y)] == f3[at(x, y)] for x in range(W) for y in range(7))
    assert c["FX-WAVE-032"]["0"] not in (top, bottom, f3)
    assert c["FX-WAVE-033"]["0"] not in (left, right, f3)
    assert c["FX-WAVE-034"]["0"] == f3
    t35 = c["FX-WAVE-035"]["0"]
    assert all(t35[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    assert t35 != fourteen
    t36 = c["FX-WAVE-036"]
    assert t36["0"] != t36["2"]
    # D-755: the remaining shapes, against the sine as After Effects' stills place them.
    for x in range(16):
        a = 2 * math.pi * (x + 0.5) / 8
        sq = wave("square", 8, 90, 0, 0, 0, x + 0.5, 0)
        assert sq == (1.0 if math.sin(a) > 0 else -1.0)  # the square's halves on the sine's
        saw = wave("sawtooth", 8, 90, 0, 0, 0, x + 0.5, 0)
        assert abs(saw - (((x % 8) + 0.5) / 4 - 1)) < 1e-12
        semi = wave("semicircle", 8, 90, 0, 0, 0, x + 0.5, 0)
        assert semi == -wave("uncircle", 8, 90, 0, 0, 0, x + 0.5, 0)
    assert wave("semicircle", 8, 90, 0, 0, 0, 0, 0) == 1  # its point where the sine rises
    assert abs(wave("semicircle", 8, 90, 0, 0, 0, 4, 0) + 1) < 1e-12  # its arch's crest
    assert abs(wave("sawtooth", 8, 90, 0, 0, 0, 4, 0)) < 1e-12  # through nothing mid-wave
    assert mix(0) == 0 and knot(1, 0) != knot(2, 0)
    for X in [i * 0.37 - 20 for i in range(200)]:
        for sh in ("noise", "smooth_noise"):
            v = wave(sh, 8, 90, 0, 0, 0, X, 0)
            assert -1 <= v <= 1
            d = abs(wave(sh, 8, 90, 0, 0, 0, X + 1e-9, 0) - v)
            assert d < 1e-6, (sh, X)  # no jumps: the knots are joined
    knots = [wave("noise", 40, 90, 0, 0, 0, 2.5 * j, 0) for j in range(-8, 8)]
    assert all(abs(k - knot(1, j)) < 1e-12 for k, j in zip(knots, range(-8, 8)))
    assert len({round(k, 6) for k in knots}) == len(knots)  # a new level at every knot
    f39 = c["FX-WAVE-039"]["0"]
    for x in (0, 1, 2, 3):
        assert blue(f39[at(x, 6)]) and blue(f39[at(x, 7)]) and not blue(f39[at(x, 4)])
    for x in (4, 5, 6, 7):
        assert blue(f39[at(x, 2)]) and blue(f39[at(x, 3)]) and not blue(f39[at(x, 5)])
    for fx in ("FX-WAVE-040", "FX-WAVE-041", "FX-WAVE-042", "FX-WAVE-043", "FX-WAVE-044",
               "FX-WAVE-046"):
        assert c[fx]["0"] not in (f3, drawn, f39), fx
    assert c["FX-WAVE-041"]["0"] != c["FX-WAVE-042"]["0"]
    t45 = c["FX-WAVE-045"]
    assert t45["0"] == c["FX-WAVE-043"]["0"] != t45["2"]
    assert t45["2"] == render(case(shape="noise", height=2, width=40, phase=90), 0)
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
