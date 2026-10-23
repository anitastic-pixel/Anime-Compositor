"""Cell Pattern, worked a second way.

D-209 adds `core.cell_pattern`: a pattern of cells, like bubbles, crystals, stained glass or
cracked plates, laid over each pixel that shows, in two colours, black and white as it starts,
for a texture, a background or a magic surface on a cel. It is After Effects' Cell Pattern in
purpose, and this program's own rule. Nothing is ported. Document 21 is the rule in words; this
file is the reference for the numbers document 25 pins against it.

The cells. `size` is how many pixels a cell spans. Pixel (X, Y) of the drawing's own space, the
top-left corner (0, 0), sits at p = ((X + 0.5) / size, (Y + 0.5) / size) among the cells, and
cell (m, n) holds one point. With U Noise's number in -1..1 (D-119, `tools/noise_reference.py`),
U_c = U(seed, m, n, 0, c): r = disperse / 2 * (U_0 + 1) / 2, phi = pi * U_1, s = 1 when U_2 >= 0
and -1 otherwise, the cell's grey g = (U_3 + 1) / 2, and the point is (m + 0.5 + r cos(phi + s t),
n + 0.5 + r sin(phi + s t)), t the evolution in radians: disperse 0 sets every point in its
cell's middle, 1.5 lets it wander three quarters of a cell, and turning the evolution sends each
point round its own circle, some one way and some the other, one turn of 360 degrees bringing
it back. The nearest and second nearest points to p, F1 and F2 apart, are sought among the 5 by
5 cells round the one p falls in, row by row from the top and left to right, a nearer point
taking the lead only when strictly nearer; the grey is the nearest's.

The rule. k = 2 F1 / (F1 + F2), 0 at a point and 1 where two cells meet, and 0 when F1 + F2 is
0. The value v by `pattern`: "bubbles", sqrt(1 - k^2), round balls; "crystals", 1 - k, faceted
cones with dark seams; "plates", min(1, 4 (1 - k)), flat plates with dark seams; and
"static_plates", g, each cell one flat grey. `invert` "on" makes it 1 - v. Then v = clamp(0.5 +
(v - 0.5) contrast / 100, 0, 1), and the colour and its mixing are Fractal Noise's (D-128): G =
srgb_to_linear(dark + v (light - dark)) per channel, the colours' 8-bit values over 255; with b
the straight linear colour, f = G (normal), b G (multiply), 1 - (1 - b)(1 - G) (screen) or b + G
(add) by `blend`, and out = ((b + op (f - b)) a, a) with op = opacity / 100. Every pixel keeps its
own covering, pixels that do not show stay as they are, and opacity 0 leaves the layer as it is.
The seed's whole part is counted. A draft scales the size, held at 1, as Fractal Noise's is.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build writes single precision into its buffers.

D-762 adds After Effects' Offset, Overflow, Evolution Options and Tiling Options, each drawing as
D-209 when the file leaves it out. `offset`, x then y in pixels (absent 0, 0), slides the cells:
p = ((X + 0.5 - offset_x) / size, (Y + 0.5 - offset_y) / size). `overflow` after the contrast:
"clip" (absent) holds v inside 0..1 as before, "soft_clamp" eases it, 0.5 + 0.5 tanh(2 (v - 0.5)),
and "wrap_back" folds it back at 0 and 1 (Fractal Noise's, D-647). `cycle_evolution` "on" with
`cycle` revolutions (1..1000, whole part, absent 1) makes the evolution loop every 360 N degrees,
N the whole cycle: the evolution is taken modulo 360 N, and each point turns K / N times as
fast, K = min(N, 1 + floor((U_4 + 1) / 2 N)); so 360 N degrees is the start again, and some
points turn more than once in a cycle. `tiling` "on" with `tiles_horizontal` and
`tiles_vertical` (1..1000, whole parts, absent 10) gives cell (m, n) the numbers of cell
(m mod Th, n mod Tv), so the pattern repeats every Th cells across and Tv down. Contrast's top
is 10000, as After Effects lets it be typed.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says: Noise's card (`tools/noise_reference.py`). The drawing goes into
`Fixtures/cell_pattern/media`, the projects into `Fixtures/cell_pattern`, and the expected frames
into `Fixtures/cell_pattern/expected_cell_pattern.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/cell_pattern_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from noise_reference import u as U  # noqa: E402
import noise_reference as N  # noqa: E402
from gradient_reference import BLENDS  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "cell_pattern"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"contrast": (0, 10000), "disperse": (0, 1.5), "size": (1, 1000),
          "evolution": (-100000, 100000), "seed": (0, 100000), "opacity": (0, 100)}
PATTERNS = ("bubbles", "crystals", "plates", "static_plates")
WORDS = ("pattern", "invert", "dark_color", "light_color", "blend")
NAMES = ("pattern", "invert") + tuple(RANGES) + WORDS[2:]
# D-762: written only when a case sets them; a file without one draws as it says.
MORE_RANGES = {"offset": (-100000, 100000), "cycle": (1, 1000), "tiles_horizontal": (1, 1000),
               "tiles_vertical": (1, 1000)}
MORE_WORDS = {"overflow": ("clip", "soft_clamp", "wrap_back"), "cycle_evolution": ("off", "on"),
              "tiling": ("off", "on")}
ABSENT = {"offset": (0, 0), "overflow": "clip", "cycle_evolution": "off", "cycle": 1,
          "tiling": "off", "tiles_horizontal": 10, "tiles_vertical": 10}
DRAWINGS = {"card": N.DRAWINGS["card"]}


# --- the rule -------------------------------------------------------------------------------

def point(seed, m, n, disperse, e, cycle=None, tiles=None):
    """Cell (m, n)'s point among the cells, and its grey; `e` the evolution in degrees."""
    hm, hn = (m % tiles[0], n % tiles[1]) if tiles else (m, n)
    u = [U(seed, hm, hn, 0, c) for c in range(4)]
    r = disperse / 2 * (u[0] + 1) / 2
    if cycle:
        k = min(cycle, 1 + math.floor((U(seed, hm, hn, 0, 4) + 1) / 2 * cycle))
        t = math.radians(e % (360 * cycle)) * k / cycle
    else:
        t = math.radians(e)
    a = math.pi * u[1] + (1 if u[2] >= 0 else -1) * t
    return m + 0.5 + r * math.cos(a), n + 0.5 + r * math.sin(a), (u[3] + 1) / 2


def overflowed(v, how):
    if how == "soft_clamp":
        return 0.5 + 0.5 * math.tanh(2 * (v - 0.5))
    if how == "wrap_back":
        t = v % 2
        return t if t <= 1 else 2 - t
    return min(1.0, max(0.0, v))


def value(n, pattern, invert, X, Y, w=None):
    w = w or ABSENT
    px = (X + 0.5 - n["offset"][0]) / n["size"]
    py = (Y + 0.5 - n["offset"][1]) / n["size"]
    I, J = math.floor(px), math.floor(py)
    cycle = math.floor(n["cycle"]) if w["cycle_evolution"] == "on" else None
    tiles = ((math.floor(n["tiles_horizontal"]), math.floor(n["tiles_vertical"]))
             if w["tiling"] == "on" else None)
    f1 = f2 = math.inf
    g = 0.0
    for nn in range(J - 2, J + 3):
        for m in range(I - 2, I + 3):
            qx, qy, grey = point(n["seed"], m, nn, n["disperse"], n["evolution"], cycle, tiles)
            d = (px - qx) * (px - qx) + (py - qy) * (py - qy)
            if d < f1:
                f1, f2, g = d, f1, grey
            elif d < f2:
                f2 = d
    f1, f2 = math.sqrt(f1), math.sqrt(f2)
    k = 2 * f1 / (f1 + f2) if f1 + f2 > 0 else 0.0
    v = {"bubbles": math.sqrt(1 - k * k), "crystals": 1 - k, "plates": min(1.0, 4 * (1 - k)),
         "static_plates": g}[pattern]
    if invert == "on":
        v = 1 - v
    return overflowed(0.5 + (v - 0.5) * n["contrast"] / 100, w["overflow"])


def cell_pattern(layer, n, c):
    op = n["opacity"] / 100
    if op == 0:
        return layer
    dark = [v / 255 for v in R.hex_color(c["dark_color"].lower())]
    light = [v / 255 for v in R.hex_color(c["light_color"].lower())]
    mix = BLENDS[c["blend"]]
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            a = p[3]
            if a <= 0:
                px.append(p)
                continue
            v = value(n, c["pattern"], c["invert"], layer["left"] + i, layer["top"] + j,
                      {**ABSENT, **c["more"]})
            G = [srgb_to_linear(d + v * (e - d)) for d, e in zip(dark, light)]
            b = [p[ch] / a for ch in range(3)]
            px.append([(b[ch] + op * (mix(b[ch], G[ch]) - b[ch])) * a for ch in range(3)] + [a])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(pattern="bubbles", invert="off", contrast=100, disperse=1, size=4, evolution=0, seed=0,
         dark_color="#000000", light_color="#ffffff", opacity=100, blend="normal", shift=0,
         tile=False, **more):
    assert set(more) <= set(ABSENT), more
    return {"drawing": "card", "pattern": pattern, "invert": invert, "contrast": contrast,
            "disperse": disperse, "size": size, "evolution": evolution, "seed": seed,
            "dark_color": dark_color, "light_color": light_color, "opacity": opacity,
            "blend": blend, "shift": shift, "tile": tile, "more": more}


def held(c, k, frame_no):
    lo, hi = {**RANGES, **MORE_RANGES}[k]
    v = value_at(c[k] if k in RANGES else c["more"].get(k, ABSENT[k]), frame_no)
    if isinstance(v, (list, tuple)):
        return [min(hi, max(lo, x)) for x in v]
    v = min(hi, max(lo, v))
    return math.floor(v) if k == "seed" else v


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    return motion_tile(layer, 300, 300, "on") if c["tile"] else layer


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in {**RANGES, **MORE_RANGES}}
    return frame(cell_pattern(layer_of(c), n, c), c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


CASES = {
    "FX-CELL-001": ("Bubbles, four pixels a cell, the other settings as they start: invert off, "
                    "contrast 100, disperse 1, evolution 0, seed 0, black to white, opacity 100, "
                    "normal. The card is covered by grey balls, white at each cell's point and "
                    "dark where cells meet, the drawing gone under them, every pixel at its own "
                    "covering and the empty pixels empty.", case(), [0]),
    "FX-CELL-002": ("Crystals: faceted cones, darker than the bubbles away from the points.",
                    case(pattern="crystals"), [0]),
    "FX-CELL-003": ("Plates: flat white plates, dark only close to the seams.",
                    case(pattern="plates"), [0]),
    "FX-CELL-004": ("Static plates: each cell one flat grey of its own.",
                    case(pattern="static_plates"), [0]),
    "FX-CELL-005": ("Invert on: FX-CELL-001 turned over, dark balls on light seams.",
                    case(invert="on"), [0]),
    "FX-CELL-006": ("Contrast 300: the greys pushed out towards black and white.",
                    case(contrast=300), [0]),
    "FX-CELL-007": ("Contrast 0: every pixel that shows the middle grey, halfway from black to "
                    "white.", case(contrast=0), [0]),
    "FX-CELL-008": ("Disperse 0: every point in its cell's middle, so the balls sit in a square "
                    "grid, the pattern repeating every four pixels.", case(disperse=0), [0]),
    "FX-CELL-009": ("Disperse 0 and evolution 90: a point in its cell's middle has no circle to "
                    "go round, so the frame is FX-CELL-008's.",
                    case(disperse=0, evolution=90), [0]),
    "FX-CELL-010": ("Evolution 90: the points turned a quarter of the way round their circles, "
                    "the balls moved.", case(evolution=90), [0]),
    "FX-CELL-011": ("Seed 7: other points, other balls.", case(seed=7), [0]),
    "FX-CELL-012": ("Seed 7.9: the whole part counted, so the frame is FX-CELL-011's.",
                    case(seed=7.9), [0]),
    "FX-CELL-013": ("Size 1000: one cell far larger than the card, so the card sees one soft "
                    "part of a ball.", case(size=1000), [0]),
    "FX-CELL-014": ("Colours #203070 to #FFD060, the second in capitals: navy seams and gold "
                    "balls.", case(dark_color="#203070", light_color="#FFD060"), [0]),
    "FX-CELL-015": ("Opacity 50: the balls laid on at half strength over the card.",
                    case(opacity=50), [0]),
    "FX-CELL-016": ("Opacity 0: the drawing, untouched.", case(opacity=0), [0]),
    "FX-CELL-017": ("Blend multiply: the card darkened by the pattern, the line still dark.",
                    case(blend="multiply"), [0]),
    "FX-CELL-018": ("Blend add at opacity 30: the card lightened by the pattern.",
                    case(blend="add", opacity=30), [0]),
    "FX-CELL-019": ("Evolution keyed from 0 at frame 0 to 360 at frame 4, linear: the balls "
                    "move round and are back where they began at frame 4; frame 0 is "
                    "FX-CELL-001.", case(evolution=keyed((0, 0), (4, 360))), [0, 2, 4]),
    "FX-CELL-020": ("Size keyed from 4 at frame 0 to 1000 at frame 4, eased past its end: frame 2 "
                    "would pass 1000, is held at 1000, and is size 1000.",
                    case(size=keyed((0, 4, OVERSHOOT), (4, 1000))), [0, 2]),
    "FX-CELL-021": ("FX-CELL-001 moved three pixels right: the pattern moves with the drawing.",
                    case(shift=3), [0]),
    "FX-CELL-022": ("After a Motion Tile that grows the layer: the pattern is fixed to the "
                    "drawing's own space, so the frame is FX-CELL-001's.", case(tile=True), [0]),
    # D-762: Offset, Overflow, Evolution Options and Tiling Options.
    "FX-CELL-034": ("Offset 2, 0: the cells slid two pixels right over the unmoved drawing; "
                    "each shown pixel's colour is FX-CELL-001's two pixels to its left.",
                    case(offset=(2, 0)), [0]),
    "FX-CELL-035": ("Offset 0, 0 written: FX-CELL-001 exactly.", case(offset=(0, 0)), [0]),
    "FX-CELL-036": ("Offset -1.5, 2.5: the cells slid a pixel and a half left and two and a half "
                    "down, between pixels.", case(offset=(-1.5, 2.5)), [0]),
    "FX-CELL-037": ("Offset keyed from 0, 0 at frame 0 to 4, 0 at frame 4, linear: frame 0 is "
                    "FX-CELL-001 and frame 4 the cells four pixels right, one whole cell.",
                    case(offset=keyed((0, (0, 0)), (4, (4, 0)))), [0, 2, 4]),
    "FX-CELL-038": ("Overflow clip written, contrast 300: FX-CELL-006 exactly.",
                    case(contrast=300, overflow="clip"), [0]),
    "FX-CELL-039": ("Overflow soft clamp, contrast 300: the greys past black and white eased "
                    "back inside, so no pixel is pure black or white and the order of the greys "
                    "is kept.", case(contrast=300, overflow="soft_clamp"), [0]),
    "FX-CELL-040": ("Overflow wrap back, contrast 300: the greys past white fold back down and "
                    "those past black fold back up, so the brightest balls get dark middles.",
                    case(contrast=300, overflow="wrap_back"), [0]),
    "FX-CELL-041": ("Contrast 10000, the new top, clipped: almost every pixel pure black or "
                    "white.", case(contrast=10000), [0]),
    "FX-CELL-042": ("Contrast 10000 with wrap back: the greys folded many times, rings in the "
                    "balls.", case(contrast=10000, overflow="wrap_back"), [0]),
    "FX-CELL-043": ("Cycle Evolution on, cycle 1, evolution 90: one revolution a cycle turns "
                    "every point as before, so this is FX-CELL-010 exactly.",
                    case(evolution=90, cycle_evolution="on", cycle=1), [0]),
    "FX-CELL-044": ("Cycle Evolution on, cycle 2, evolution keyed from 0 at frame 0 to 720 at "
                    "frame 4, linear: frame 0 is FX-CELL-001, frame 2 (360 degrees) is not, and "
                    "frame 4, the end of the two-revolution cycle, is FX-CELL-001 again exactly: "
                    "the loop closes.",
                    case(cycle_evolution="on", cycle=2, evolution=keyed((0, 0), (4, 720))),
                    [0, 2, 4]),
    "FX-CELL-045": ("Cycle Evolution off, cycle 3 written, evolution 90: the cycle is not used, "
                    "so this is FX-CELL-010.", case(evolution=90, cycle=3), [0]),
    "FX-CELL-046": ("Cycle Evolution on, cycle 2.7, evolution 720: the whole part, 2, is "
                    "counted, so 720 degrees is a whole cycle and this is FX-CELL-001.",
                    case(evolution=720, cycle_evolution="on", cycle=2.7), [0]),
    "FX-CELL-047": ("Tiling on, 2 by 2 tiles: the cells repeat every two across and two down, "
                    "so the pattern repeats every eight pixels.",
                    case(tiling="on", tiles_horizontal=2, tiles_vertical=2), [0]),
    "FX-CELL-048": ("Tiling off, 2 by 2 tiles written: FX-CELL-001.",
                    case(tiling="off", tiles_horizontal=2, tiles_vertical=2), [0]),
    "FX-CELL-049": ("Every new setting written at the value a file without it means: offset 0, "
                    "0, clip, cycle off, cycle 1, tiling off, 10 by 10 tiles: FX-CELL-001 "
                    "exactly.", case(**ABSENT), [0]),
}

INVALID = {
    "FX-CELL-023": ("Size 0, below 1.", case(size=0)),
    "FX-CELL-024": ("Size 1001, above 1000.", case(size=1001)),
    "FX-CELL-025": ("Disperse 1.6, above 1.5.", case(disperse=1.6)),
    "FX-CELL-026": ("Contrast -1, below 0.", case(contrast=-1)),
    "FX-CELL-027": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-CELL-028": ("Seed -1, below 0.", case(seed=-1)),
    "FX-CELL-029": ("Pattern \"tubular\", which is not \"bubbles\", \"crystals\", \"plates\" or "
                    "\"static_plates\".", case(pattern="tubular")),
    "FX-CELL-030": ("Invert \"yes\", which is not \"on\" or \"off\".", case(invert="yes")),
    "FX-CELL-031": ("Light colour \"#12345\", not six hex digits.",
                    case(light_color="#12345")),
    "FX-CELL-032": ("Blend \"darken\", which is not \"normal\", \"multiply\", \"screen\" or "
                    "\"add\".", case(blend="darken")),
    "FX-CELL-033": ("Disperse keyed to 1.6 at frame 4, above 1.5.",
                    case(disperse=keyed((0, 1), (4, 1.6)))),
    "FX-CELL-050": ("Contrast 10001, above 10000.", case(contrast=10001)),
    "FX-CELL-051": ("Overflow \"allow_hdr\", which is not \"clip\", \"soft_clamp\" or "
                    "\"wrap_back\".", case(overflow="allow_hdr")),
    "FX-CELL-052": ("Cycle Evolution \"yes\", which is not \"on\" or \"off\".",
                    case(cycle_evolution="yes")),
    "FX-CELL-053": ("Cycle 0, below 1.", case(cycle_evolution="on", cycle=0)),
    "FX-CELL-054": ("Cycle 1001, above 1000.", case(cycle=1001)),
    "FX-CELL-055": ("Tiling \"yes\", which is not \"on\" or \"off\".", case(tiling="yes")),
    "FX-CELL-056": ("Tiles Horizontal 0, below 1.", case(tiling="on", tiles_horizontal=0)),
    "FX-CELL-057": ("Tiles Vertical 1001, above 1000.", case(tiles_vertical=1001)),
    "FX-CELL-058": ("Offset 100001, 0, above 100000.", case(offset=(100001, 0))),
    "FX-CELL-059": ("Offset keyed to -100001, 0 at frame 4, below -100000.",
                    case(offset=keyed((0, (0, 0)), (4, (-100001, 0))))),
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
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.cell_pattern",
                    "enabled": True,
                    "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                   for k in NAMES}})
    effects[-1]["parameters"].update({k: (v if k in MORE_WORDS else setting_json(v))
                                      for k, v in c["more"].items()})
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
    # D-762: in number order, so 001 to 033 keep their place in the file.
    expected["cases"] = dict(sorted(expected["cases"].items()))
    (OUT / "expected_cell_pattern.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    art = plain(case())
    empty = [i for i in range(W * H) if art[i][3] == 0]
    shows = [i for i in range(W * H) if art[i][3] > 0]
    assert empty and shows
    grey = lambda px, i: px[i][1] / px[i][3]  # noqa: E731  the green, straight
    mean = lambda px: sum(grey(px, i) for i in shows) / len(shows)  # noqa: E731

    # Every covering kept, and the empty pixels left empty, in every case.
    for fx, frames in c.items():
        if fx == "FX-CELL-021":
            continue
        for px in frames.values():
            assert all(px[i][3] == art[i][3] for i in range(W * H)), fx
            assert all(px[i] == art[i] for i in empty), fx

    one = c["FX-CELL-001"]["0"]
    # Black to white at normal: every pixel is grey, the three channels alike, and the greys
    # vary across the card, some near white and some dark.
    assert all(near(one[i][:3], [one[i][1]] * 3, 1e-12) for i in shows)
    assert max(grey(one, i) for i in shows) > 0.6 and min(grey(one, i) for i in shows) < 0.1
    # Crystals are 1 - k, below the bubbles' sqrt(1 - k^2) everywhere; plates are above both.
    cry, pla = c["FX-CELL-002"]["0"], c["FX-CELL-003"]["0"]
    assert all(grey(cry, i) <= grey(one, i) + 1e-12 for i in shows)
    assert all(grey(pla, i) >= grey(cry, i) - 1e-12 for i in shows)
    assert sum(grey(pla, i) > 1 - 1e-9 for i in shows) > len(shows) // 3
    # Static plates: few greys, each a cell's.
    flat = c["FX-CELL-004"]["0"]
    assert 2 < len({round(grey(flat, i), 12) for i in shows}) < len(shows) // 2
    # Invert: 1 - v, in encoded terms, through the curve.
    inv = c["FX-CELL-005"]["0"]
    linear_to_srgb = lambda l: 12.92 * l if l <= 0.0031308 else 1.055 * l ** (1 / 2.4) - 0.055  # noqa
    assert all(abs(linear_to_srgb(grey(inv, i)) + linear_to_srgb(grey(one, i)) - 1) < 1e-9
               for i in shows)
    hard, mid = c["FX-CELL-006"]["0"], c["FX-CELL-007"]["0"]
    assert sum(min(grey(hard, i), 1 - grey(hard, i)) < 1e-9 for i in shows) > sum(
        min(grey(one, i), 1 - grey(one, i)) < 1e-9 for i in shows)
    assert all(abs(grey(mid, i) - srgb_to_linear(0.5)) < 1e-12 for i in shows)
    # Disperse 0: a square grid, the same every four pixels; evolution cannot move it.
    grid = c["FX-CELL-008"]["0"]
    assert all(near(grid[at(x, y)], grid[at(x + 4, y)]) for x in range(1, 11) for y in range(9)
               if art[at(x, y)][3] == art[at(x + 4, y)][3] == 1)
    assert c["FX-CELL-009"]["0"] == grid
    assert c["FX-CELL-010"]["0"] != one and c["FX-CELL-011"]["0"] != one
    assert c["FX-CELL-012"]["0"] == c["FX-CELL-011"]["0"]
    big = c["FX-CELL-013"]["0"]
    assert max(grey(big, i) for i in shows) - min(grey(big, i) for i in shows) < 0.05
    gold = c["FX-CELL-014"]["0"]
    hi = max(shows, key=lambda i: grey(one, i))
    lo = min(shows, key=lambda i: grey(one, i))
    assert gold[hi][0] > gold[hi][2] and gold[lo][2] > gold[lo][0]  # gold balls, navy seams
    half = c["FX-CELL-015"]["0"]
    for i in shows:
        assert near(half[i], [(art[i][k] + one[i][k]) / 2 for k in range(3)] + [art[i][3]], 1e-12)
    assert c["FX-CELL-016"]["0"] == art
    mult, add = c["FX-CELL-017"]["0"], c["FX-CELL-018"]["0"]
    for i in shows:
        assert all(mult[i][k] <= art[i][k] + 1e-12 for k in range(3))
        assert all(add[i][k] >= art[i][k] - 1e-12 for k in range(3))
    nineteen = c["FX-CELL-019"]
    assert nineteen["0"] == one and nineteen["2"] != one
    assert all(near(nineteen["4"][i], one[i], 1e-9) for i in range(W * H))
    twenty = c["FX-CELL-020"]
    assert ease(OVERSHOOT, 0.5) > 1 and twenty["0"] == one
    assert twenty["2"] == render(case(size=1000), 2)
    moved = c["FX-CELL-021"]["0"]
    assert all(moved[at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert c["FX-CELL-022"]["0"] == one
    assert mean(one) != mean(inv)
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art

    # D-762. Offset: at normal and opacity 100 a shown pixel's colour is the pattern's alone.
    slid = c["FX-CELL-034"]["0"]
    assert all(near([v / slid[at(x, y)][3] for v in slid[at(x, y)][:3]],
                    [v / one[at(x - 2, y)][3] for v in one[at(x - 2, y)][:3]], 1e-9)
               for x in range(2, W) for y in range(H)
               if art[at(x, y)][3] > 0 and art[at(x - 2, y)][3] > 0)
    assert slid != one and c["FX-CELL-035"]["0"] == one
    assert c["FX-CELL-036"]["0"] not in (one, slid)
    keyed_off = c["FX-CELL-037"]
    assert keyed_off["0"] == one and keyed_off["4"] == render(case(offset=(4, 0)), 0)
    assert keyed_off["2"] == render(case(offset=(2, 0)), 0)
    # Overflow: clip is D-209's; soft clamp keeps every grey strictly inside and in order;
    # wrap back differs where the contrast pushed past white or black.
    assert c["FX-CELL-038"]["0"] == hard
    soft, wrap = c["FX-CELL-039"]["0"], c["FX-CELL-040"]["0"]
    assert all(0 < grey(soft, i) < 1 for i in shows)
    assert all((grey(soft, i) - grey(soft, j)) * (grey(one, i) - grey(one, j)) >= -1e-12
               for i in shows for j in shows)
    assert wrap != hard and any(abs(grey(wrap, i) - grey(hard, i)) > 0.1 for i in shows)
    assert all(grey(wrap, i) == grey(hard, i) for i in shows if 0 < grey(hard, i) < 1)
    top = c["FX-CELL-041"]["0"]
    assert sum(min(grey(top, i), 1 - grey(top, i)) < 1e-9 for i in shows) > 0.9 * len(shows)
    assert c["FX-CELL-042"]["0"] != top
    # Cycle: one revolution is the old turn; the loop closes exactly; off ignores the cycle.
    assert c["FX-CELL-043"]["0"] == c["FX-CELL-010"]["0"]
    loop = c["FX-CELL-044"]
    assert loop["0"] == one and loop["4"] == one and loop["2"] != one
    assert c["FX-CELL-045"]["0"] == c["FX-CELL-010"]["0"]
    assert c["FX-CELL-046"]["0"] == one
    # Tiling: the pattern repeats every two cells, eight pixels.
    tiled = c["FX-CELL-047"]["0"]
    pairs = [(at(x, y), at(x + 8, y)) for x in range(W - 8) for y in range(H)
             if art[at(x, y)][3] > 0 and art[at(x + 8, y)][3] > 0]
    assert pairs and all(near([v / tiled[i][3] for v in tiled[i][:3]],
                              [v / tiled[j][3] for v in tiled[j][:3]], 1e-9) for i, j in pairs)
    assert tiled != one and c["FX-CELL-048"]["0"] == one and c["FX-CELL-049"]["0"] == one
    print("checked")


if __name__ == "__main__":
    main()
