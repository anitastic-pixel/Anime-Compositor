"""Line recolour, worked a second way.

D-91 adds `core.line_recolor`. It repaints the pixels of chosen colours, a drawing's black line
as a rule, in one new colour, keeping each pixel's own covering, so the soft edge of a line
stays soft in the new colour. The choice is D-87 and D-88's: a pixel is chosen when it shows and
each of its 8-bit red, green and blue is within the tolerance of one chosen colour's. It is this
program's own method; nothing is ported. Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says. The drawing goes into `Fixtures/recolor/media`, the projects into
`Fixtures/recolor`, and the expected frames into `Fixtures/recolor/expected_recolor.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/recolor_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402

W, H = 16, 10
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "recolor"
TOLERANCE = 2e-5  # document 25's default for a filter
MAX_TOLERANCE = 255


# --- the rule -------------------------------------------------------------------------------

def hex_color(s):
    return tuple(int(s[i:i + 2], 16) for i in (1, 3, 5))


def working(p):
    """Document 21's PNG reading: sRGB to linear, then premultiplied."""
    a = p[3] / 255
    return [srgb_to_linear(p[c] / 255) * a for c in range(3)] + [a]


def chosen(p, colors, tolerance):
    """D-87 and D-88: the pixel shows, and each of its 8-bit red, green and blue is within the
    tolerance of one chosen colour's."""
    return p[3] > 0 and any(all(abs(p[i] - t[i]) <= tolerance for i in range(3))
                            for t in (hex_color(s.lower()) for s in colors))


def recolor(pixels, colors, tolerance, new_color):
    """Each chosen pixel becomes the new colour at its own covering; every other is kept."""
    new = [srgb_to_linear(v / 255) for v in hex_color(new_color.lower())]
    return [[c * p[3] / 255 for c in new] + [p[3] / 255] if chosen(p, colors, tolerance)
            else working(p) for p in pixels]


def frame(layer, shift):
    """The layer's pixels as the composition's W by H frame, moved `shift` pixels right."""
    return [layer[y * W + x - shift] if 0 <= x - shift < W else [0.0] * 4
            for y in range(H) for x in range(W)]


# --- the drawing ----------------------------------------------------------------------------

LINE = (30, 26, 36, 255)        # the face drawing's line, #1e1a24
SOFT = (30, 26, 36, 128)        # the same line at half covering, its antialiased edge
TRACE = (200, 40, 40, 255)      # a red colour-trace line, #c82828
SKIN = (246, 214, 190, 255)     # #f6d6be
NONE = S.NONE
LINE_HEX, LINE_OFF_HEX, TRACE_HEX, SKIN_HEX = "#1e1a24", "#1e1a25", "#c82828", "#f6d6be"
NEAR_HEX = "#28242e"  # the line plus 10 on every channel


def face(x, y):
    """A box of line in columns 2 to 13 and rows 2 to 7, filled with skin, with a half-covering
    edge down its left side in column 1 and a red trace line across row 5, columns 4 to 11."""
    if not (1 <= x <= 13 and 2 <= y <= 7):
        return NONE
    if x == 1:
        return SOFT
    if x in (2, 13) or y in (2, 7):
        return LINE
    if y == 5 and 4 <= x <= 11:
        return TRACE
    return SKIN


DRAWINGS = {"face": [[face(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(colors=(LINE_HEX,), tolerance=0, new_color="#ff0000", shift=0, name="face"):
    return {"drawing": name, "colors": list(colors), "tolerance": tolerance,
            "new_color": new_color, "shift": shift}


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    tolerance = min(MAX_TOLERANCE, max(0.0, value_at(c["tolerance"], frame_no)))
    return frame(recolor(pixels, c["colors"], tolerance, c["new_color"]), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(colors=(), shift=c["shift"], name=c["drawing"]), 0)


CASES = {
    "FX-RECOLOR-001": ("The line chosen, new colour #ff0000: the box's line and its half-covering "
                       "edge turn red, the edge still half covering; the skin and the trace line "
                       "are untouched.",
                       case(), [0]),
    "FX-RECOLOR-002": ("No colour chosen: the drawing, untouched.",
                       case(colors=()), [0]),
    "FX-RECOLOR-003": ("The line chosen one step off in blue, tolerance 0: nothing is chosen, and "
                       "the drawing is untouched.",
                       case(colors=[LINE_OFF_HEX]), [0]),
    "FX-RECOLOR-004": ("The same at tolerance 1: the line is chosen again, and this is "
                       "FX-RECOLOR-001.",
                       case(colors=[LINE_OFF_HEX], tolerance=1), [0]),
    "FX-RECOLOR-005": ("The line and the trace line chosen, new colour #3060ff: both turn blue.",
                       case(colors=[LINE_HEX, TRACE_HEX], new_color="#3060ff"), [0]),
    "FX-RECOLOR-006": ("FX-RECOLOR-001 with both colours written in capitals: the same.",
                       case(colors=[LINE_HEX.upper()], new_color="#FF0000"), [0]),
    "FX-RECOLOR-007": ("#28242e chosen, 10 above the line on every channel, tolerance keyed from 0 "
                       "at frame 0 to 20 at frame 4, linear: frames 0 and 1, at 0 and 5, choose "
                       "nothing; frame 2, at exactly 10, is FX-RECOLOR-001, and so is frame 4.",
                       case(colors=[NEAR_HEX], tolerance=keyed((0, 0), (4, 20))), [0, 1, 2, 4]),
    "FX-RECOLOR-008": ("FX-RECOLOR-001 moved three pixels right: the same, moved.",
                       case(shift=3), [0, 3]),
    "FX-RECOLOR-009": ("The new colour is the line's own: the drawing, untouched.",
                       case(new_color=LINE_HEX), [0]),
    "FX-RECOLOR-010": ("The new colour is the skin's: the line turns to skin and vanishes into "
                       "it, and its edge is skin at half covering.",
                       case(new_color=SKIN_HEX), [0]),
    "FX-RECOLOR-011": ("Tolerance 255: every pixel that shows is chosen and turns red; the "
                       "empty ones stay empty.",
                       case(tolerance=255), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-RECOLOR-012": ("Tolerance 256, above 255.", case(tolerance=256)),
    "FX-RECOLOR-013": ("Tolerance -1, below 0.", case(tolerance=-1)),
    "FX-RECOLOR-014": ("Tolerance keyed to 300 at frame 4.",
                       case(tolerance=keyed((0, 0), (4, 300)))),
    "FX-RECOLOR-015": ("Nine colours, one more than eight.",
                       case(colors=[f"#0000{i:02x}" for i in range(9)])),
    "FX-RECOLOR-016": ("A colour written \"#12345\", one digit short.",
                       case(colors=[LINE_HEX, "#12345"])),
    "FX-RECOLOR-017": ("A new colour written \"red\".", case(new_color="red")),
    "FX-RECOLOR-018": ("A new colour left empty, which this effect does not allow.",
                       case(new_color="")),
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
        "instance_id": "fx-0-0", "type_id": "core.line_recolor", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in ("colors", "tolerance", "new_color")}}]
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

    (OUT / "expected_recolor.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    one = c["FX-RECOLOR-001"]["0"]
    red = srgb_to_linear(1.0)

    assert one[at(2, 4)] == [red, 0.0, 0.0, 1.0]
    assert one[at(1, 4)] == [red * 128 / 255, 0.0, 0.0, 128 / 255]  # the soft edge, still soft
    assert one[at(5, 4)] == drawn[at(5, 4)] and one[at(5, 5)] == drawn[at(5, 5)]
    assert sum(one[i] != drawn[i] for i in range(W * H)) == 12 * 2 + 4 * 2 + 6
    for fx in ("FX-RECOLOR-002", "FX-RECOLOR-003", "FX-RECOLOR-009"):
        assert c[fx]["0"] == drawn, fx
    assert c["FX-RECOLOR-004"]["0"] == one and c["FX-RECOLOR-006"]["0"] == one
    five = c["FX-RECOLOR-005"]["0"]
    assert five[at(5, 5)] != drawn[at(5, 5)] and five[at(5, 5)][:3] == five[at(2, 4)][:3]
    seven = c["FX-RECOLOR-007"]
    assert seven["0"] == drawn and seven["1"] == drawn and seven["2"] == one and seven["4"] == one
    moved = c["FX-RECOLOR-008"]["0"]
    assert moved == c["FX-RECOLOR-008"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == one[at(0, y):at(W - 3, y)]
    ten = c["FX-RECOLOR-010"]["0"]
    assert ten[at(2, 4)] == drawn[at(5, 4)]
    assert ten[at(1, 4)][3] == 128 / 255
    eleven = c["FX-RECOLOR-011"]["0"]
    for i in range(W * H):
        assert eleven[i] == ([red * drawn[i][3], 0.0, 0.0, drawn[i][3]] if drawn[i][3] else
                             [0.0] * 4)
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(0 <= v <= p[3] for v in p[:3]), (name, p)


if __name__ == "__main__":
    main()
