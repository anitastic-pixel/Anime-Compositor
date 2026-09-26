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

def offsets(radius):
    """The whole-pixel offsets within the radius: the pixel alone below 1."""
    if radius < 1:
        return [(0, 0)]
    r = math.floor(radius)
    return [(dx, dy) for dy in range(-r, r + 1) for dx in range(-r, r + 1)
            if dx * dx + dy * dy <= radius * radius]


def growth(radius, edges):
    return 0 if edges == "repeat" else math.ceil(radius)


def blurred(layer, radius, edges, x, y):
    """The output at the pixel (x, y) of layer space: nothing outside the output's rectangle,
    the layer's own grown by `growth`."""
    g = growth(radius, edges)
    left, top, w, h = layer["left"], layer["top"], layer["w"], layer["h"]
    if not (left - g <= x < left + w + g and top - g <= y < top + h + g):
        return [0.0] * 4
    at = offsets(radius)
    total = [0.0] * 4
    for dx, dy in at:
        sx, sy = x + dx - left, y + dy - top
        if edges == "repeat":
            sx, sy = min(max(sx, 0), w - 1), min(max(sy, 0), h - 1)
        elif not (0 <= sx < w and 0 <= sy < h):
            continue
        p = layer["px"][sy * w + sx]
        for i in range(4):
            total[i] += p[i]
    return [v / len(at) for v in total]


# --- the drawings ---------------------------------------------------------------------------

DRAWINGS = {"bars": D.DRAWINGS["bars"], "plate": E.DRAWINGS["plate"]}


# --- the cases ------------------------------------------------------------------------------

def case(radius=10, edges="transparent", shift=0, drawing="bars"):
    return {"drawing": drawing, "radius": radius, "edges": edges, "shift": shift}


def layer_of(c):
    return {"px": [D.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


def render(c, frame_no):
    layer = layer_of(c)
    radius = min(RADIUS[1], max(RADIUS[0], value_at(c["radius"], frame_no)))
    return [blurred(layer, radius, c["edges"], x - c["shift"], y)
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
        "parameters": {k: setting_json(c[k]) for k in ("radius", "edges")}}]
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
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
