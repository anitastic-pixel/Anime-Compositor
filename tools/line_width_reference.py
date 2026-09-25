"""Line width, worked a second way.

D-94 adds `core.line_width`. It thickens or thins a drawing's lines by a number of pixels, as a
thicker or thinner pen would have drawn them. `based_on` is "shape" or "colors". On the shape, a
positive width gives each pixel the covering and colour of the most covered pixel within the
width, and a negative width lowers each pixel's covering to the least covered within the width,
its own colour kept. On colours, a positive width paints the pixels near a chosen colour with
it, and a negative width replaces the chosen pixels near anything else with what is next to
them. "Within the width" is the disc `dx^2 + dy^2 <= width^2`; ties go to the nearest pixel, then
the one above, then the one to the left. Outside the drawing is transparent. The choice is D-87
and D-88's. It is this program's own method; nothing is ported. Document 21 is the rule in
words; this file is the reference for the numbers document 25 pins against it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: line recolour's drawing, `tools/recolor_reference.py`'s face. The rule is worked
at every pixel of the composition, inside the drawing or not, which is what the build's grown
bounds hold. The drawing goes into `Fixtures/line_width/media`, the projects into
`Fixtures/line_width`, and the expected frames into
`Fixtures/line_width/expected_line_width.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/line_width_reference.py
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

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "line_width"
TOLERANCE = 2e-5  # document 25's default for a filter
LINE_HEX, TRACE_HEX = R.LINE_HEX, R.TRACE_HEX
CLEAR = (0, 0, 0, 0)


# --- the rule -------------------------------------------------------------------------------

def disc(width):
    """The offsets within `|width|`, nearest first, then the one above, then the one left."""
    r = math.floor(abs(width))
    return sorted(((dx, dy) for dy in range(-r, r + 1) for dx in range(-r, r + 1)
                   if dx * dx + dy * dy <= width * width),
                  key=lambda o: (o[0] ** 2 + o[1] ** 2, o[1], o[0]))


def widened(pixel, x, y, width, based_on, colors, tolerance):
    """The output at (x, y), in working values. `pixel(x, y)` is the drawing's 8-bit pixel,
    transparent outside it."""
    own = pixel(x, y)
    if width == 0:
        return R.working(own)
    offsets = disc(width)
    near = [pixel(x + dx, y + dy) for dx, dy in offsets]
    if based_on == "shape":
        if width > 0:
            # The most covered; `max` keeps the first of equals, the nearest.
            return R.working(max(near, key=lambda p: p[3]))
        least = min(p[3] for p in near)
        w = R.working(own)
        return [c * least / own[3] for c in w[:3]] + [least / 255] if own[3] else w
    if not colors:
        return R.working(own)
    pick = R.chosen(own, colors, tolerance)
    wanted = width > 0
    if pick == wanted:
        return R.working(own)
    for p in near:
        if R.chosen(p, colors, tolerance) == wanted:
            return R.working(p)
    return R.working(own)


# --- the cases ------------------------------------------------------------------------------

DRAWINGS = R.DRAWINGS


def case(width=1, based_on="shape", colors=(LINE_HEX,), tolerance=0, shift=0, name="face"):
    return {"drawing": name, "width": width, "based_on": based_on, "colors": list(colors),
            "tolerance": tolerance, "shift": shift}


def render(c, frame_no):
    rows = DRAWINGS[c["drawing"]]

    def pixel(x, y):
        return rows[y][x] if 0 <= x < W and 0 <= y < H else CLEAR

    width = min(20.0, max(-20.0, value_at(c["width"], frame_no)))
    tolerance = min(255.0, max(0.0, value_at(c["tolerance"], frame_no)))
    return [widened(pixel, x - c["shift"], y, width, c["based_on"], c["colors"], tolerance)
            for y in range(H) for x in range(W)]


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(width=0, shift=c["shift"], name=c["drawing"]), 0)


CASES = {
    "FX-WIDTH-001": ("Shape, width 1: the drawing grows one pixel up, down, left and right. The "
                     "half-covering edge in column 1 becomes solid line, and a new half-covering "
                     "edge appears in column 0; the corners stay square-cut.",
                     case(), [0]),
    "FX-WIDTH-002": ("Shape, width 1.5: the diagonal neighbours count too, so the corners fill "
                     "out as well.",
                     case(width=1.5), [0]),
    "FX-WIDTH-003": ("Shape, width -1: the drawing shrinks one pixel. The box's outer line "
                     "touches the transparent outside and goes; the line in column 2 touches "
                     "the half-covering edge and becomes half covering, still the line's colour.",
                     case(width=-1), [0]),
    "FX-WIDTH-004": ("Width 0: the drawing, untouched.",
                     case(width=0), [0]),
    "FX-WIDTH-005": ("Colours, the line chosen, width 1: the line spreads one pixel into the "
                     "skin inside and the transparent outside, filling the skin between the red "
                     "trace line's ends and the box; the trace line itself is untouched.",
                     case(based_on="colors"), [0]),
    "FX-WIDTH-006": ("Colours, the trace line chosen, width -1: a one-pixel line thinned by one "
                     "is gone; each of its pixels takes the skin above it, which wins the tie "
                     "with the skin below.",
                     case(width=-1, based_on="colors", colors=[TRACE_HEX]), [0]),
    "FX-WIDTH-007": ("Colours, the trace line chosen, width 2: the red line grows two pixels up "
                     "and down and sideways, painting over the skin and the box's line alike.",
                     case(width=2, based_on="colors", colors=[TRACE_HEX]), [0]),
    "FX-WIDTH-008": ("Shape, width keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is "
                     "the drawing, frame 1 is FX-WIDTH-001, and frames 2 and 4 grow two and "
                     "four pixels.",
                     case(width=keyed((0, 0), (4, 4))), [0, 1, 2, 4]),
    "FX-WIDTH-009": ("FX-WIDTH-001 moved three pixels right: the same, moved.",
                     case(shift=3), [0, 3]),
    "FX-WIDTH-010": ("Colours with no colour chosen, width 3: the drawing, untouched.",
                     case(width=3, based_on="colors", colors=()), [0]),
    "FX-WIDTH-011": ("Shape with the line listed as a colour: the colours are kept but not "
                     "used, and the frame is FX-WIDTH-001.",
                     case(colors=[LINE_HEX]), [0]),
    "FX-WIDTH-012": ("Shape, width -20: the drawing, six pixels tall, is gone.",
                     case(width=-20), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-WIDTH-013": ("Width 21, above 20.", case(width=21)),
    "FX-WIDTH-014": ("Width -21, below -20.", case(width=-21)),
    "FX-WIDTH-015": ("Width keyed to 25 at frame 4.", case(width=keyed((0, 0), (4, 25)))),
    "FX-WIDTH-016": ("Tolerance 256, above 255.", case(tolerance=256)),
    "FX-WIDTH-017": ("Based on \"line\", which is not a choice.", case(based_on="line")),
    "FX-WIDTH-018": ("Nine colours, one more than eight.",
                     case(colors=[f"#0000{i:02x}" for i in range(9)])),
    "FX-WIDTH-019": ("A colour written \"#12345\", one digit short.",
                     case(colors=[LINE_HEX, "#12345"])),
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
        "instance_id": "fx-0-0", "type_id": "core.line_width", "enabled": True,
        "parameters": {k: setting_json(c[k])
                       for k in ("width", "based_on", "colors", "tolerance")}}]
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

    (OUT / "expected_line_width.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    line, soft = R.working(R.LINE), R.working(R.SOFT)
    skin, trace = R.working(R.SKIN), R.working(R.TRACE)
    clear = [0.0] * 4
    shows = lambda f: {i for i, p in enumerate(f) if p[3] > 0}  # noqa: E731

    one = c["FX-WIDTH-001"]["0"]
    assert one[at(1, 4)] == line and one[at(0, 4)] == soft
    assert one[at(5, 1)] == line and one[at(5, 8)] == line and one[at(14, 4)] == line
    assert one[at(1, 1)] == soft and one[at(0, 1)] == clear  # square-cut corner
    assert one[at(5, 4)] == skin  # the inside keeps its own
    one_five = c["FX-WIDTH-002"]["0"]
    assert one_five[at(0, 1)] == soft and one_five[at(14, 1)] == line
    three = c["FX-WIDTH-003"]["0"]
    assert three[at(1, 4)] == clear and three[at(5, 2)] == clear
    assert three[at(2, 4)][3] == 128 / 255
    assert all(abs(three[at(2, 4)][i] - line[i] * 128 / 255) < 1e-15 for i in range(3))
    assert three[at(5, 4)] == skin
    assert c["FX-WIDTH-004"]["0"] == drawn
    five = c["FX-WIDTH-005"]["0"]
    assert five[at(5, 3)] == line and five[at(5, 1)] == line and five[at(5, 4)] == skin
    assert five[at(3, 5)] == line and five[at(12, 5)] == line
    assert all(five[at(x, 5)] == trace for x in range(4, 12))
    assert five[at(0, 4)] == soft  # the nearest chosen is the half-covering edge
    six = c["FX-WIDTH-006"]["0"]
    assert all(six[at(x, 5)] == skin for x in range(4, 12))
    assert shows(six) == shows(drawn)
    seven = c["FX-WIDTH-007"]["0"]
    assert all(seven[at(x, y)] == trace for x in range(4, 12) for y in range(3, 8))
    assert seven[at(2, 5)] == trace and seven[at(1, 5)] == soft and seven[at(5, 2)] == line
    eight = c["FX-WIDTH-008"]
    assert eight["0"] == drawn and eight["1"] == one
    assert eight["2"][at(5, 0)] == line and eight["2"][at(15, 4)] == line
    assert eight["4"][at(5, 0)] == line and eight["4"][at(5, 9)] == line
    moved = c["FX-WIDTH-009"]["0"]
    assert moved == c["FX-WIDTH-009"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == one[at(0, y):at(W - 3, y)]
    assert c["FX-WIDTH-010"]["0"] == drawn
    assert c["FX-WIDTH-011"]["0"] == one
    assert shows(c["FX-WIDTH-012"]["0"]) == set()


if __name__ == "__main__":
    main()
