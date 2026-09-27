"""Lens blur, worked a second way.

D-116 adds `core.lens_blur`. It blurs a layer as an out-of-focus lens does: each pixel becomes
the plain average of every pixel whose centre is within `radius` of its own, a flat disc rather
than Gaussian Blur's bell, so a small bright spot spreads into an even round patch with a firm
edge. Exactly: the mean over the whole-pixel offsets (dx, dy) with dx^2 + dy^2 <= radius^2, N
of them, of the premultiplied pixels there, red, green, blue and alpha alike. With `edges`
"transparent" an offset outside the layer reads transparent black and the layer grows by
ceil(radius) on every side; with "repeat" the offset's column and row are each held inside the
layer, and it does not grow. A radius below 1 takes the pixel alone, N = 1, and changes
nothing. It is this program's own method, modelled on After Effects' Camera Lens Blur; nothing
is ported. Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it.

D-121 gives the disc an iris, as After Effects' Camera Lens Blur has: `iris`, a circle or a
polygon of 3 to 10 blades standing on a side; `roundness`, 0 to 100, mixing the polygon toward
the circle; `rotation`, degrees clockwise from up; and `aspect`, the iris's width over its
height, its area kept. A step is on the iris when `measure` puts it within the radius, with a
billionth of a pixel of slack so a step exactly on an edge counts on every machine. Each pixel
spreads over the iris, so an odd iris, a triangle, gathers turned half round. And highlights:
a pixel whose brightest straight channel is at or above `highlight_threshold` per cent has its
colour multiplied by 1 + `highlight_gain` before the average, and a result lit past white is
held at white. At their start values, circle, 0, 0, 1, 0 and 100, and in a file without them,
the rule is D-116's, and FX-LENS-001 to 018 are the same numbers, to the byte.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawings' 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: directional blur's `bars`, a line down the left edge, a block of skin and a red
pixel at half covering with room round them, or the edges fixtures' `plate`, a picture that
fills the layer to its edges. The drawings go into `Fixtures/lens_blur/media`, the projects
into `Fixtures/lens_blur`, and the expected frames into
`Fixtures/lens_blur/expected_lens_blur.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/lens_blur_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import directional_blur_reference as D  # noqa: E402
import edges_reference as E  # noqa: E402

W, H = D.W, D.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "lens_blur"
TOLERANCE = 2e-5  # document 25's default for a filter
RADIUS = (0, 200)


# --- the rule -------------------------------------------------------------------------------

# D-121: the iris. Its shapes, by the number of blades; "circle" has none.
IRIS = {"circle": 0, "triangle": 3, "square": 4, "pentagon": 5, "hexagon": 6, "heptagon": 7,
        "octagon": 8, "nonagon": 9, "decagon": 10}
ROUNDNESS = (0, 100)
ROTATION = (-3600, 3600)
ASPECT = (0.1, 10)
GAIN = (0, 100)
THRESHOLD = (0, 100)
SLACK = 1e-9  # a step on the iris's edge counts, however the sines round


def stretch(aspect):
    """How much wider than a circle the iris is drawn: sqrt(aspect) across, 1 / that down."""
    return math.sqrt(aspect)


def measure(dx, dy, iris="circle", roundness=0, rotation=0, aspect=1):
    """How far the step (dx, dy) is out on the iris, in the radius's pixels: the iris of radius
    r holds the steps whose measure is at most r. The step is turned back by the rotation
    (clockwise from up, y down), squeezed back by the aspect, and then measured on the upright
    shape: the plain distance for a circle; for n blades the furthest the step goes toward any
    of the n sides, over the distance from the middle to a side over the distance to a corner,
    so each corner is at the radius, the shape standing on a side. Roundness mixes the
    polygon's measure with the circle's, all the circle at 100."""
    t = math.radians(rotation)
    c, s = math.cos(t), math.sin(t)
    x, y = dx * c + dy * s, -dx * s + dy * c
    k = stretch(aspect)
    u, v = x / k, y * k
    round_ = math.sqrt(u * u + v * v)
    n = IRIS[iris]
    if n == 0:
        return round_
    poly = max(-u * math.sin(2 * math.pi * j / n) + v * math.cos(2 * math.pi * j / n)
               for j in range(n)) / math.cos(math.pi / n)
    w = roundness / 100
    return (1 - w) * poly + w * round_


def reach(radius, aspect=1):
    """The furthest the iris goes across or down, in whole pixels or more."""
    k = stretch(aspect)
    return radius * max(k, 1 / k)


def offsets(radius, iris="circle", roundness=0, rotation=0, aspect=1):
    """The whole-pixel steps the iris holds: the pixel alone below 1."""
    if radius < 1:
        return [(0, 0)]
    r = math.floor(reach(radius + SLACK, aspect))
    return [(dx, dy) for dy in range(-r, r + 1) for dx in range(-r, r + 1)
            if measure(dx, dy, iris, roundness, rotation, aspect) <= radius + SLACK]


def growth(radius, edges, aspect=1):
    return 0 if edges == "repeat" else math.ceil(reach(radius, aspect))


def lit(p, gain, threshold):
    """D-121's highlight: a pixel whose brightest straight channel is at or above the threshold
    has its colour, not its covering, multiplied by 1 + gain before the average."""
    if gain == 0 or p[3] <= 0 or max(p[:3]) / p[3] < threshold / 100:
        return p
    return [v * (1 + gain) for v in p[:3]] + [p[3]]


def blurred(layer, radius, edges, x, y, iris="circle", roundness=0, rotation=0, aspect=1,
            gain=0, threshold=100):
    """The output at the pixel (x, y) of layer space: nothing outside the output's rectangle,
    the layer's own grown by `growth`."""
    g = growth(radius, edges, aspect)
    left, top, w, h = layer["left"], layer["top"], layer["w"], layer["h"]
    if not (left - g <= x < left + w + g and top - g <= y < top + h + g):
        return [0.0] * 4
    at = offsets(radius, iris, roundness, rotation, aspect)
    total = [0.0] * 4
    for dx, dy in reversed(at):  # the pixels read top to bottom, left to right, as D-116's
        # Each pixel spreads over the iris, so a pixel gathers from the iris turned half round.
        sx, sy = x - dx - left, y - dy - top
        if edges == "repeat":
            sx, sy = min(max(sx, 0), w - 1), min(max(sy, 0), h - 1)
        elif not (0 <= sx < w and 0 <= sy < h):
            continue
        p = lit(layer["px"][sy * w + sx], gain, threshold)
        for i in range(4):
            total[i] += p[i]
    out = [v / len(at) for v in total]
    if gain:
        # A disc lit past white is white: each colour held at or below the covering.
        out = [min(v, out[3]) for v in out[:3]] + [out[3]]
    return out


# --- the drawings ---------------------------------------------------------------------------

DRAWINGS = {"bars": D.DRAWINGS["bars"], "plate": E.DRAWINGS["plate"]}


# --- the cases ------------------------------------------------------------------------------

# D-121's settings as they start, which a file without them reads.
IRIS_START = {"iris": "circle", "roundness": 0, "rotation": 0, "aspect": 1,
              "highlight_gain": 0, "highlight_threshold": 100}
RANGES = {"radius": RADIUS, "roundness": ROUNDNESS, "rotation": ROTATION, "aspect": ASPECT,
          "highlight_gain": GAIN, "highlight_threshold": THRESHOLD}


def case(radius=10, edges="transparent", shift=0, drawing="bars", **iris):
    """A case; D-121's settings, when given, are all written to its file, and only then."""
    assert set(iris) <= set(IRIS_START)
    c = {"drawing": drawing, "radius": radius, "edges": edges, "shift": shift}
    if iris:
        c["iris"] = {**IRIS_START, **iris}
    return c


def settings(c, frame_no):
    """The case's settings at a frame, each number held inside its range."""
    s = {"radius": c["radius"], **c.get("iris", IRIS_START)}
    return {k: min(RANGES[k][1], max(RANGES[k][0], value_at(v, frame_no))) if k in RANGES else v
            for k, v in s.items()}


def layer_of(c):
    return {"px": [D.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


def render(c, frame_no):
    layer = layer_of(c)
    s = settings(c, frame_no)
    return [blurred(layer, s["radius"], c["edges"], x - c["shift"], y, s["iris"],
                    s["roundness"], s["rotation"], s["aspect"], s["highlight_gain"],
                    s["highlight_threshold"])
            for y in range(H) for x in range(W)]


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(radius=0, shift=c["shift"], drawing=c["drawing"]), 0)


CASES = {
    "FX-LENS-001": ("The settings as they start, radius 10, edges transparent: every pixel is "
                    "the mean of the 317 within ten pixels of it, so the line, the block and "
                    "the red pixel spread into one faint even haze over the whole frame.",
                    case(), [0]),
    "FX-LENS-002": ("Radius 0: the drawing, untouched.",
                    case(radius=0), [0]),
    "FX-LENS-003": ("Radius 0.9, under one pixel: each pixel takes itself alone, and the "
                    "drawing is untouched.",
                    case(radius=0.9), [0]),
    "FX-LENS-004": ("Radius 1: each pixel is the mean of five, itself and the four beside, "
                    "above and below it, a plus: the block's edge pixels lose a fifth of themselves "
                    "for each empty neighbour, its middle stays skin, and the pixel diagonally "
                    "off its corner stays empty.",
                    case(radius=1), [0]),
    "FX-LENS-005": ("Radius 1.5: the mean of the nine pixels of the 3 by 3 square about each "
                    "pixel, so a diagonal neighbour of the block now takes a ninth of it.",
                    case(radius=1.5), [0]),
    "FX-LENS-006": ("Radius 2.5: the mean of 21 pixels, the 5 by 5 square without its four "
                    "corners.",
                    case(radius=2.5), [0]),
    "FX-LENS-007": ("Radius 4: the mean of 49 pixels; the red pixel at half covering spreads "
                    "into an even round patch nine pixels across, each pixel of it 1/49 of it, "
                    "and the block into a rounded haze.",
                    case(radius=4), [0]),
    "FX-LENS-008": ("Radius 2.5, edges repeat, on a picture that fills the layer: every pixel "
                    "stays fully covered, the edges as solid as the middle.",
                    case(radius=2.5, edges="repeat", drawing="plate"), [0]),
    "FX-LENS-009": ("The same with edges transparent: the edges fade, the corners most, and the "
                    "middle, more than 2.5 pixels from every edge, is FX-LENS-008's.",
                    case(radius=2.5, drawing="plate"), [0]),
    "FX-LENS-010": ("Radius keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 untouched, "
                    "frame 2 at radius 2, the mean of 13, and frame 4 FX-LENS-007.",
                    case(radius=keyed((0, 0), (4, 4))), [0, 2, 4]),
    "FX-LENS-011": ("Radius eased from 1 at frame 0 to 0 at frame 4 on a curve that overshoots: "
                    "at frame 2 the radius would be below 0, is held at 0, and the drawing is "
                    "untouched; frame 0 is FX-LENS-004.",
                    case(radius=keyed((0, 1, OVERSHOOT), (4, 0))), [0, 2]),
    "FX-LENS-012": ("Radius 1.5, moved three pixels right: the layer grew two pixels on every "
                    "side, so the line on the drawing's left edge spreads into the column left "
                    "of it, which shows; nothing reaches the two columns before that.",
                    case(radius=1.5, shift=3), [0, 3]),
    "FX-LENS-013": ("Radius 1.5, edges repeat, on the bars moved three pixels right: the layer "
                    "does not grow, so the three columns left of it stay empty, and the line on "
                    "its edge keeps more of itself than FX-LENS-012's.",
                    case(radius=1.5, edges="repeat", shift=3), [0]),
    # D-121: the iris's shape, turn, roundness and aspect, and the highlights.
    "FX-LENS-019": ("Iris triangle, radius 4: the iris holds 25 steps, a triangle standing on "
                    "its side with its point up; the red pixel spreads into that triangle, each "
                    "pixel of it 1/25 of the red, its point four pixels above the red pixel and "
                    "its base two below.",
                    case(radius=4, iris="triangle"), [0]),
    "FX-LENS-020": ("Iris square, radius 3: the square standing on a side, its corners three "
                    "pixels from the middle, so the 5 by 5 square of 25 steps.",
                    case(radius=3, iris="square"), [0]),
    "FX-LENS-021": ("Iris hexagon, radius 4: 43 steps, flat along the top and bottom, pointed "
                    "at the sides.",
                    case(radius=4, iris="hexagon"), [0]),
    "FX-LENS-022": ("Iris decagon, radius 4: 47 steps, nearly FX-LENS-007's circle of 49.",
                    case(radius=4, iris="decagon"), [0]),
    "FX-LENS-023": ("Iris triangle, radius 4, rotation 180: FX-LENS-019's triangle upside down, "
                    "its point four pixels below the red pixel.",
                    case(radius=4, iris="triangle", rotation=180), [0]),
    "FX-LENS-024": ("Iris square, radius 3, rotation 45: the square turned onto its corner, a "
                    "diamond of 25 steps reaching three pixels up, down, left and right.",
                    case(radius=3, iris="square", rotation=45), [0]),
    "FX-LENS-025": ("Iris triangle, radius 4, roundness 100: all round, FX-LENS-007 exactly.",
                    case(radius=4, iris="triangle", roundness=100), [0]),
    "FX-LENS-026": ("Iris square, radius 4, roundness 50: halfway between the square, 25 "
                    "steps at this radius, and FX-LENS-007's circle of 49: 45 steps, the "
                    "square's sides bowed out to three pixels from the middle, and the circle's "
                    "four furthest steps, four pixels straight out, not reached.",
                    case(radius=4, iris="square", roundness=50), [0]),
    "FX-LENS-027": ("Round iris, radius 3, aspect 2: an oval twice as wide as it is tall, 29 "
                    "steps, reaching four pixels left and right and two up and down.",
                    case(radius=3, aspect=2), [0]),
    "FX-LENS-028": ("Round iris, radius 3, aspect 0.5, rotation 90: an oval twice as tall as "
                    "wide, turned a quarter, FX-LENS-027 exactly.",
                    case(radius=3, aspect=0.5, rotation=90), [0]),
    "FX-LENS-029": ("Radius 4, highlight gain 3, threshold 80: the skin, its brightest channel "
                    "0.92, is lit to four times before the average, so the block's haze is "
                    "brighter and near it held at white; the red, at 0.58, is under the "
                    "threshold, and its patch where the block does not reach is FX-LENS-007's.",
                    case(radius=4, highlight_gain=3, highlight_threshold=80), [0]),
    "FX-LENS-030": ("Radius 4, highlight gain 0.5, threshold 50: the red is lit too, to one "
                    "and a half times, and its patch where the block does not reach is one and "
                    "a half times FX-LENS-007's in colour, its covering the same.",
                    case(radius=4, highlight_gain=0.5, highlight_threshold=50), [0]),
    "FX-LENS-031": ("Iris triangle, radius 4, rotation keyed from 0 at frame 0 to 60 at frame 4, "
                    "linear: frame 0 is FX-LENS-019, frame 2 is turned 30, and frame 4, turned "
                    "60, stands on its point, FX-LENS-023.",
                    case(radius=4, iris="triangle", rotation=keyed((0, 0), (4, 60))), [0, 2, 4]),
    "FX-LENS-032": ("Radius 4, threshold 80, highlight gain keyed from 0 at frame 0 to 3 at "
                    "frame 4, linear: frame 0 is FX-LENS-007, frame 4 FX-LENS-029.",
                    case(radius=4, highlight_gain=keyed((0, 0), (4, 3)), highlight_threshold=80),
                    [0, 2, 4]),
    "FX-LENS-033": ("Iris hexagon, radius 2.5, edges repeat, on the picture that fills the "
                    "layer: every pixel stays fully covered.",
                    case(radius=2.5, edges="repeat", drawing="plate", iris="hexagon"), [0]),
    "FX-LENS-034": ("Round iris, radius 1.5, aspect 2, moved three pixels right: the iris is "
                    "a line of five across with one above and one below the middle; the layer "
                    "grew three pixels, 1.5 times the square root of 2 rounded up, so the line "
                    "on the drawing's left edge spreads into the two columns left of it, and "
                    "the column before those stays empty.",
                    case(radius=1.5, aspect=2, shift=3), [0]),
    "FX-LENS-035": ("Every D-121 setting written at its start value: circle, roundness 0, "
                    "rotation 0, aspect 1, highlight gain 0, threshold 100; FX-LENS-001 "
                    "exactly.",
                    case(iris="circle"), [0]),
    "FX-LENS-036": ("Iris decagon, radius 0.9, turned 17 and stretched to aspect 3: under one "
                    "pixel each pixel takes itself alone, and the drawing is untouched.",
                    case(radius=0.9, iris="decagon", rotation=17, aspect=3), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-LENS-014": ("Radius 201, above 200.", case(radius=201)),
    "FX-LENS-015": ("Radius -1, below 0.", case(radius=-1)),
    "FX-LENS-016": ("Radius keyed to 250 at frame 4.", case(radius=keyed((0, 0), (4, 250)))),
    "FX-LENS-017": ("Edges \"wrap\", which is not a way of treating edges.",
                    case(edges="wrap")),
    "FX-LENS-018": ("Edges \"Repeat\": the word is exact, so a capital is not it.",
                    case(edges="Repeat")),
    "FX-LENS-037": ("Iris \"star\", which is not a shape of iris.", case(iris="star")),
    "FX-LENS-038": ("Iris \"Hexagon\": the word is exact, so a capital is not it.",
                    case(iris="Hexagon")),
    "FX-LENS-039": ("Roundness 101, above 100.", case(roundness=101)),
    "FX-LENS-040": ("Rotation 3601, above 3600.", case(rotation=3601)),
    "FX-LENS-041": ("Aspect 0.05, below 0.1.", case(aspect=0.05)),
    "FX-LENS-042": ("Aspect keyed to 20 at frame 4.", case(aspect=keyed((0, 1), (4, 20)))),
    "FX-LENS-043": ("Highlight gain -1, below 0.", case(highlight_gain=-1)),
    "FX-LENS-044": ("Highlight threshold 101, above 100.", case(highlight_threshold=101)),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.lens_blur", "enabled": True,
        "parameters": {k: setting_json(v) for k, v in
                       {"radius": c["radius"], "edges": c["edges"], **c.get("iris", {})}.items()}}]
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

    (OUT / "expected_lens_blur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b: all(abs(p - q) < 1e-12 for u, v in zip(a, b)  # noqa: E731
                            for p, q in zip(u, v))
    solid = lambda px: all(abs(p[3] - 1) < 1e-12 for p in px)  # noqa: E731
    red = D.working(D.SOFT)
    skin = D.working(D.SKIN)

    # The rule's own pieces: the disc's sizes.
    assert [len(offsets(r)) for r in (0, 0.9, 1, 1.5, 2, 2.5, 4, 10)] == \
        [1, 1, 5, 9, 13, 21, 49, 317]
    assert set(offsets(1)) == {(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)}
    assert (2, 2) not in offsets(2.5) and (2, 1) in offsets(2.5)
    assert growth(0.9, "transparent") == 1 and growth(4, "repeat") == 0

    one = c["FX-LENS-001"]["0"]
    assert all(0 < p[3] < 0.2 for p in one)  # every pixel of the frame touched, none strong
    assert c["FX-LENS-002"]["0"] == drawn and c["FX-LENS-003"]["0"] == drawn
    # Radius 1: the block's middle edge pixel (6, 3) has one empty neighbour, its corner (5, 3)
    # two; the pixel diagonally off the corner, (4, 2), takes nothing.
    four = c["FX-LENS-004"]["0"]
    assert near([four[at(7, 4)]], [skin]) and near([four[at(6, 3)]], [[v * 4 / 5 for v in skin]])
    assert near([four[at(5, 3)]], [[v * 3 / 5 for v in skin]]) and four[at(4, 2)] == [0.0] * 4
    five = c["FX-LENS-005"]["0"]
    assert near([five[at(4, 2)]], [[v / 9 for v in skin]])
    # Radius 2.5: (3, 1), two off the block's corner on the diagonal, takes nothing (2^2 + 2^2 is
    # past 6.25), where radius 4 reaches it; (4, 2) takes three block pixels of 21.
    six = c["FX-LENS-006"]["0"]
    assert six[at(3, 1)] == [0.0] * 4 and near([six[at(4, 2)]], [[v * 3 / 21 for v in skin]])
    seven = c["FX-LENS-007"]["0"]
    assert seven[at(3, 1)][3] > 0
    # The red pixel's patch: 1/49 of it at (12, 0), four above it, and (15, 1), where 3^2 + 3^2 is
    # past 16, takes none of it (and nothing else reaches there).
    assert near([seven[at(12, 0)]], [[v / 49 for v in red]])
    assert seven[at(15, 1)] == [0.0] * 4
    # Repeat: solid; the middle as with transparent.
    eight, nine = c["FX-LENS-008"]["0"], c["FX-LENS-009"]["0"]
    assert solid(eight) and not solid(nine) and nine[at(0, 0)][3] < nine[at(0, 5)][3] < 1
    assert all(near([eight[at(x, y)]], [nine[at(x, y)]])
               for x in range(3, W - 3) for y in range(3, H - 3))
    ten = c["FX-LENS-010"]
    assert ten["0"] == drawn and near(ten["4"], seven)
    assert near(ten["2"], render(case(radius=2), 0)) and ten["2"] != six
    eleven = c["FX-LENS-011"]
    assert ease(OVERSHOOT, 0.5) > 1 and eleven["2"] == drawn and near(eleven["0"], four)
    # Moved: column 2 is the grown column left of the line, 3/9 of the line on row 4; columns 0
    # and 1 are empty.
    twelve = c["FX-LENS-012"]
    assert twelve["0"] == twelve["3"]
    line = D.working(D.LINE)
    assert near([twelve["0"][at(2, 4)]], [[v * 3 / 9 for v in line]])
    assert all(twelve["0"][at(x, y)] == [0.0] * 4 for x in (0, 1) for y in range(H))
    assert all(twelve["0"][at(x, y)] == five[at(x - 3, y)] for x in range(3, W) for y in range(H))
    thirteen = c["FX-LENS-013"]["0"]
    assert all(thirteen[at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    assert thirteen[at(3, 4)][3] > twelve["0"][at(3, 4)][3] + 0.1

    # D-121. The irises' sizes and shapes.
    size = lambda r, **k: len(offsets(r, **k))  # noqa: E731
    assert [size(4, iris="triangle"), size(3, iris="square"), size(4, iris="hexagon"),
            size(4, iris="decagon"), size(4, iris="square", roundness=50),
            size(3, aspect=2)] == [25, 25, 43, 47, 45, 29]
    assert set(offsets(3, iris="square")) == {(x, y) for x in range(-2, 3) for y in range(-2, 3)}
    assert set(offsets(3, iris="square", rotation=45)) == \
        {(x, y) for x in range(-3, 4) for y in range(-3, 4) if abs(x) + abs(y) <= 3}
    tri = offsets(4, iris="triangle")
    assert min(y for _, y in tri) == -4 and max(y for _, y in tri) == 2 and (0, -4) in tri
    assert set(offsets(4, iris="triangle", roundness=100)) == set(offsets(4))
    oval = offsets(3, aspect=2)
    assert max(x for x, _ in oval) == 4 and max(y for _, y in oval) == 2
    assert set(oval) == set(offsets(3, aspect=0.5, rotation=90))
    assert set(offsets(1.5, aspect=2)) == \
        {(-2, 0), (-1, 0), (0, 0), (1, 0), (2, 0), (0, 1), (0, -1)}
    assert growth(1.5, "transparent", 2) == 3 and growth(3, "repeat", 2) == 0
    assert offsets(0.9, iris="decagon", rotation=17, aspect=3) == [(0, 0)]
    # The triangle: the red pixel's own patch, 1/25 of it, point up four above, base two below.
    t19, t23 = c["FX-LENS-019"]["0"], c["FX-LENS-023"]["0"]
    for x, y in ((12, 0), (15, 6)):
        assert near([t19[at(x, y)]], [[v / 25 for v in red]])
    assert t19[at(15, 7)] == [0.0] * 4
    # Upside down: the point four below, nothing above it.
    assert near([t23[at(12, 8)]], [[v / 25 for v in red]]) and t23[at(12, 0)] == [0.0] * 4
    assert t19 != t23 and c["FX-LENS-024"]["0"] != c["FX-LENS-020"]["0"]
    assert near(c["FX-LENS-025"]["0"], seven)
    assert near(c["FX-LENS-027"]["0"], c["FX-LENS-028"]["0"])
    # The highlights: the skin lit and held at white, the red not, then lit too.
    lit80, lit50 = c["FX-LENS-029"]["0"], c["FX-LENS-030"]["0"]
    assert lit(red, 3, 80) == red and lit(red, 0.5, 50) != red
    assert lit(skin, 3, 80)[0] == 4 * skin[0]
    assert near([lit80[at(12, 0)]], [seven[at(12, 0)]])
    assert near([lit50[at(12, 0)]], [[v * 1.5 / 49 for v in red[:3]] + [red[3] / 49]])
    assert any(abs(p[0] - p[3]) < 1e-12 and p[3] > 0 for p in lit80)
    assert all(p[0] >= q[0] - 1e-12 for p, q in zip(lit80, seven))
    assert sum(p[0] for p in lit80) > sum(p[0] for p in seven) + 1
    t31 = c["FX-LENS-031"]
    assert t31["0"] == t19 and near(t31["4"], t23) and t31["2"] not in (t19, t23)
    t32 = c["FX-LENS-032"]
    assert t32["0"] == seven and near(t32["4"], lit80)
    assert solid(c["FX-LENS-033"]["0"])
    t34 = c["FX-LENS-034"]["0"]
    assert all(t34[at(0, y)] == [0.0] * 4 for y in range(H))
    assert near([t34[at(1, 4)]], [[v / 7 for v in line]]) and t34[at(2, 4)][3] > 0
    assert c["FX-LENS-035"]["0"] == one and c["FX-LENS-036"]["0"] == drawn

    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
