"""Line blur, worked a second way.

D-183 adds `core.line_blur`. It softens a drawing's lines along their own length and never across
them, so jagged steps smooth into slopes while a line keeps its thickness and its ends. It is
inspired by OpenToonz's Line Blur, not a port: this program's own, lighter rule.

The rule. A pixel's "ink" is its covering less its light, `alpha - (0.2126 R + 0.7152 G +
0.0722 B)` on its working (linear, premultiplied) values, held within 0 and 1: a dark line on
nothing and a dark line on white paper both have ink. The slopes of the covering and of the ink
at each pixel are central differences each way; the structure tensor adds, for both, slope x
squared, x times y, and y squared, and is smoothed by the 5 by 5 binomial kernel, 1 4 6 4 1 each
way over 256. So a line is found by its covering on an empty layer, whatever its colour, and by
its darkness on an opaque one. From the smoothed tensor (a, b, c), `r = sqrt((a - c)^2 + 4 b^2)`;
where r is 0 the pixel is left as it is. Otherwise the line runs along the tensor's weaker
direction, the unit vector of `(b, -(a - c + r) / 2)` when a >= c and of `(-(c - a + r) / 2, b)`
when a < c. The blur is a 1-D Gaussian along that direction, sigma = length / 2, weight
`exp(-k^2 / (2 sigma^2))` at whole steps k = 1 to ceil(length) each way, each tap a bilinear
sample of the working values, transparent outside the drawing, the pixel itself weighing 1. Each
side stops at the first tap whose covering is below 1/256, where the line has ended, so a blur
never eats a line from its ends. The sum is divided by the weights'. The pixel moves towards the
blur by strength / 100 times the tensor's coherence `r / (a + c)`, so straight runs follow their
line fully and corners, where the direction is uncertain, move less. With "lines only" that
amount is also multiplied by the pixel's own ink, so pale fills and the empty outside are left
alone. Length 0 or strength 0 is the drawing untouched. The tensor is 0 more than 3 pixels from
anything drawn, so the effect grows the layer by 3 pixels whatever the length.

Document 21 is the rule in words; this file is the reference for the numbers document 25 pins
against it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a short bar, a stepped diagonal, or `tools/recolor_reference.py`'s face. The rule
is worked at every pixel of the composition, inside the drawing or not, which is what the
build's grown bounds hold. The drawings go into `Fixtures/line_blur/media`, the projects into
`Fixtures/line_blur`, and the expected frames into `Fixtures/line_blur/expected_line_blur.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/line_blur_reference.py
"""

import json
import math
import sys
from functools import lru_cache
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "line_blur"
TOLERANCE = 2e-5  # document 25's default for a filter
MAX_LENGTH = 50
GROW = 3
LUMA = (0.2126, 0.7152, 0.0722)
SMOOTH = (1, 4, 6, 4, 1)
STOP = 1 / 256
MARGIN = [1.0]  # the nearest any tap's covering came to STOP, checked below
CLEAR = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def line_blur(pixel, length, strength, lines_only):
    """`pixel(x, y)` is the drawing's working value, transparent outside it. Returns the output
    at (x, y) in the drawing's own pixels."""

    @lru_cache(None)
    def ink(x, y):
        p = pixel(x, y)
        return min(1.0, max(0.0, p[3] - sum(k * v for k, v in zip(LUMA, p))))

    def cover(x, y):
        return pixel(x, y)[3]

    @lru_cache(None)
    def slopes(x, y):
        return [((f(x + 1, y) - f(x - 1, y)) / 2, (f(x, y + 1) - f(x, y - 1)) / 2)
                for f in (cover, ink)]

    def tensor(x, y):
        a = b = c = 0.0
        for j, wy in enumerate(SMOOTH):
            for i, wx in enumerate(SMOOTH):
                w = wx * wy / 256
                for gx, gy in slopes(x + i - 2, y + j - 2):
                    a += w * gx * gx
                    b += w * gx * gy
                    c += w * gy * gy
        return a, b, c

    def sample(px, py):
        x0, y0 = math.floor(px), math.floor(py)
        fx, fy = px - x0, py - y0
        p00, p10 = pixel(x0, y0), pixel(x0 + 1, y0)
        p01, p11 = pixel(x0, y0 + 1), pixel(x0 + 1, y0 + 1)
        return [(p00[i] * (1 - fx) + p10[i] * fx) * (1 - fy)
                + (p01[i] * (1 - fx) + p11[i] * fx) * fy for i in range(4)]

    def out(x, y):
        own = pixel(x, y)
        if length == 0 or strength == 0:
            return own
        a, b, c = tensor(x, y)
        r = math.sqrt((a - c) ** 2 + 4 * b * b)
        if r == 0:
            return own
        tx, ty = (b, -(a - c + r) / 2) if a >= c else (-(c - a + r) / 2, b)
        n = math.sqrt(tx * tx + ty * ty)
        tx, ty = tx / n, ty / n
        sigma = length / 2
        acc, total = list(own), 1.0
        for side in (1, -1):
            for k in range(1, math.ceil(length) + 1):
                s = sample(x + side * k * tx, y + side * k * ty)
                MARGIN[0] = min(MARGIN[0], abs(s[3] - STOP))
                if s[3] < STOP:
                    break
                w = math.exp(-k * k / (2 * sigma * sigma))
                acc = [v + w * q for v, q in zip(acc, s)]
                total += w
        amount = strength / 100 * r / (a + c) * (ink(x, y) if lines_only else 1)
        return [o + (v / total - o) * amount for o, v in zip(own, acc)]

    return out


# --- the drawings ---------------------------------------------------------------------------

LINE, NONE = R.LINE, R.NONE


def bar(x, y):
    """A one-pixel line across row 4, columns 4 to 11."""
    return LINE if y == 4 and 4 <= x <= 11 else NONE


def stairs(x, y):
    """A one-pixel diagonal drawn without smoothing: three pixels across, then one down."""
    return LINE if y == 2 + x // 3 else NONE


DRAWINGS = {"bar": [[bar(x, y) for x in range(W)] for y in range(H)],
            "stairs": [[stairs(x, y) for x in range(W)] for y in range(H)],
            "face": R.DRAWINGS["face"]}


# --- the cases ------------------------------------------------------------------------------

def case(length=3, strength=100, lines_only=False, shift=0, name="face"):
    return {"drawing": name, "length": length, "strength": strength, "lines_only": lines_only,
            "shift": shift}


def render(c, frame_no):
    rows = DRAWINGS[c["drawing"]]

    def pixel(x, y):
        return R.working(rows[y][x]) if 0 <= x < W and 0 <= y < H else CLEAR

    length = min(MAX_LENGTH, max(0.0, value_at(c["length"], frame_no)))
    strength = min(100.0, max(0.0, value_at(c["strength"], frame_no)))
    out = line_blur(pixel, length, strength, c["lines_only"])
    return [out(x - c["shift"], y) for y in range(H) for x in range(W)]


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(length=0, shift=c["shift"], name=c["drawing"]), 0)


CASES = {
    "FX-LBLUR-001": ("The bar, length 3: a clean straight line has nothing to smooth, and every "
                     "pixel stays as it was, to within rounding.",
                     case(name="bar"), [0]),
    "FX-LBLUR-002": ("The stepped diagonal, length 4: the steps soften into a slope. The empty "
                     "pixel in the crook of each step takes some ink, the step pixels give some "
                     "up, and nothing more than one pixel from the line changes much.",
                     case(length=4, name="stairs"), [0]),
    "FX-LBLUR-003": ("The face, length 3: the box's straight sides, the flat skin and the red "
                     "line's middle stay as they are; the red line's ends taper into the skin, "
                     "and the box's inside corners round off, the skin in each taking some of "
                     "the line.",
                     case(), [0]),
    "FX-LBLUR-004": ("FX-LBLUR-003 at strength 50: every pixel halfway between the drawing and "
                     "FX-LBLUR-003.",
                     case(strength=50), [0]),
    "FX-LBLUR-005": ("FX-LBLUR-003 with lines only: each pixel changes by FX-LBLUR-003's change "
                     "times its own ink, so the empty outside stays empty and the skin moves "
                     "less than the lines.",
                     case(lines_only=True), [0]),
    "FX-LBLUR-006": ("Length 0: the drawing, untouched.",
                     case(length=0), [0]),
    "FX-LBLUR-007": ("The stepped diagonal with the length keyed from 0 at frame 0 to 4 at frame "
                     "4, linear: frame 0 is the drawing, frame 2 softens it less, and frame 4 is "
                     "FX-LBLUR-002.",
                     case(length=keyed((0, 0), (4, 4)), name="stairs"), [0, 2, 4]),
    "FX-LBLUR-008": ("FX-LBLUR-002 moved three pixels right: the same, moved.",
                     case(length=4, name="stairs", shift=3), [0, 3]),
    "FX-LBLUR-009": ("Strength 0: the drawing, untouched.",
                     case(strength=0), [0]),
    "FX-LBLUR-010": ("The bar, length 50, the most: each side of the blur stops where the line "
                     "ends, so the bar is still as it was, to within rounding.",
                     case(length=50, name="bar"), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-LBLUR-011": ("Length 51, above 50.", case(length=51)),
    "FX-LBLUR-012": ("Length -1, below 0.", case(length=-1)),
    "FX-LBLUR-013": ("Length keyed to 60 at frame 4.", case(length=keyed((0, 3), (4, 60)))),
    "FX-LBLUR-014": ("Strength 101, above 100.", case(strength=101)),
    "FX-LBLUR-015": ("Strength -1, below 0.", case(strength=-1)),
    "FX-LBLUR-016": ("Lines only written \"yes\", not true or false.", case(lines_only="yes")),
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
        "instance_id": "fx-0-0", "type_id": "core.line_blur", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in ("length", "strength", "lines_only")}}]
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

    (OUT / "expected_line_blur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q: all(abs(u - v) < 1e-12 for u, v in zip(p, q))  # noqa: E731
    change = lambda p, q: sum(abs(u - v) for u, v in zip(p, q))  # noqa: E731
    assert MARGIN[0] > 1e-5, "a tap's covering came too near the stopping point"

    bar_drawn = plain(case(name="bar"))
    for fx in ("FX-LBLUR-001", "FX-LBLUR-010"):
        assert all(near(p, q) for p, q in zip(c[fx]["0"], bar_drawn))

    stairs_drawn = plain(case(name="stairs"))
    two = c["FX-LBLUR-002"]["0"]
    for x, y in ((3, 2), (6, 3), (9, 4), (12, 5)):  # the crook of each step
        assert stairs_drawn[at(x, y)] == CLEAR and two[at(x, y)][3] > 0.1
        assert two[at(x, y + 1)][3] < 0.9  # the step pixel beside it
    for y in range(H):
        for x in range(W):
            if all(stairs_drawn[at(u, v)] == CLEAR for u in range(x - 1, x + 2)
                   for v in range(y - 1, y + 2) if 0 <= u < W and 0 <= v < H):
                assert two[at(x, y)][3] < 0.03

    face = plain(case())
    three = c["FX-LBLUR-003"]["0"]
    for x, y in ((7, 2), (7, 4), (7, 5), (8, 5)):
        assert near(three[at(x, y)], face[at(x, y)])
    assert change(three[at(4, 5)], face[at(4, 5)]) > 0.05
    assert change(three[at(11, 5)], face[at(11, 5)]) > 0.05
    for x, y in ((3, 3), (12, 3), (3, 6), (12, 6)):  # skin in the inside corners
        assert three[at(x, y)][1] < face[at(x, y)][1] - 0.05
    four = c["FX-LBLUR-004"]["0"]
    assert all(near(p, [(u + v) / 2 for u, v in zip(q, r)]) for p, q, r in zip(four, face, three))
    five = c["FX-LBLUR-005"]["0"]
    for p, q, r in zip(five, face, three):
        own = min(1.0, max(0.0, q[3] - sum(k * v for k, v in zip(LUMA, q))))
        assert near(p, [u + (v - u) * own for u, v in zip(q, r)])
    assert all(p == CLEAR for p, q in zip(five, face) if q == CLEAR)
    assert c["FX-LBLUR-006"]["0"] == face and c["FX-LBLUR-009"]["0"] == face

    seven = c["FX-LBLUR-007"]
    assert seven["0"] == stairs_drawn and seven["4"] == two
    assert 0 < change(seven["2"][at(3, 2)], CLEAR) < change(two[at(3, 2)], CLEAR)

    eight = c["FX-LBLUR-008"]["0"]
    assert eight == c["FX-LBLUR-008"]["3"]
    for y in range(H):
        assert eight[at(3, y):at(0, y + 1)] == two[at(0, y):at(W - 3, y)]


if __name__ == "__main__":
    main()
