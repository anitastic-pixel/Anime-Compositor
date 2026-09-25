"""Select colour, worked a second way.

D-93 adds `core.select_color`. It keeps the pixels of chosen colours and makes every other pixel
transparent, or the reverse: a drawing's line alone, to be treated on its own layer, or the
drawing without its line. The choice is D-87 and D-88's: a pixel is chosen when it shows and
each of its 8-bit red, green and blue is within the tolerance of one chosen colour's. `keep` is
"chosen" or "others". With no colour chosen it changes nothing, whichever it keeps. It is this
program's own method; nothing is ported. Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: line recolour's drawing, `tools/recolor_reference.py`'s face. The drawing goes
into `Fixtures/select_color/media`, the projects into `Fixtures/select_color`, and the expected
frames into `Fixtures/select_color/expected_select_color.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/select_color_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "select_color"
TOLERANCE = 2e-5  # document 25's default for a filter
MAX_TOLERANCE = 255
LINE_HEX, TRACE_HEX, NEAR_HEX = R.LINE_HEX, R.TRACE_HEX, R.NEAR_HEX


# --- the rule -------------------------------------------------------------------------------

def select(pixels, colors, tolerance, keep):
    """With no colour, every pixel is kept. Otherwise a pixel is kept when it is chosen and
    `keep` is "chosen", or not chosen and `keep` is "others"; one not kept becomes transparent."""
    if not colors:
        return [R.working(p) for p in pixels]
    return [R.working(p) if R.chosen(p, colors, tolerance) == (keep == "chosen") else [0.0] * 4
            for p in pixels]


# --- the cases ------------------------------------------------------------------------------

DRAWINGS = R.DRAWINGS


def case(colors=(LINE_HEX,), tolerance=0, keep="chosen", shift=0, name="face"):
    return {"drawing": name, "colors": list(colors), "tolerance": tolerance, "keep": keep,
            "shift": shift}


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    tolerance = min(MAX_TOLERANCE, max(0.0, value_at(c["tolerance"], frame_no)))
    return R.frame(select(pixels, c["colors"], tolerance, c["keep"]), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(colors=(), shift=c["shift"], name=c["drawing"]), 0)


CASES = {
    "FX-SELECT-001": ("The line chosen, keep chosen: only the box's line and its half-covering "
                      "edge are left, the edge still half covering; the skin and the trace line "
                      "become transparent.",
                      case(), [0]),
    "FX-SELECT-002": ("The line chosen, keep others: the line and its edge become transparent, "
                      "and the skin and the trace line are left.",
                      case(keep="others"), [0]),
    "FX-SELECT-003": ("No colour chosen, keep chosen: the drawing, untouched.",
                      case(colors=()), [0]),
    "FX-SELECT-004": ("No colour chosen, keep others: the drawing, untouched.",
                      case(colors=(), keep="others"), [0]),
    "FX-SELECT-005": ("The line and the trace line chosen, keep chosen: both lines are left, "
                      "and only the skin goes.",
                      case(colors=[LINE_HEX, TRACE_HEX]), [0]),
    "FX-SELECT-006": ("FX-SELECT-001 with the colour written in capitals: the same.",
                      case(colors=[LINE_HEX.upper()]), [0]),
    "FX-SELECT-007": ("#28242e chosen, 10 above the line on every channel, keep chosen, tolerance "
                      "keyed from 0 at frame 0 to 20 at frame 4, linear: frames 0 and 1 choose "
                      "nothing and so keep nothing, and the frame is empty; frame 2, at exactly "
                      "10, is FX-SELECT-001, and so is frame 4.",
                      case(colors=[NEAR_HEX], tolerance=keyed((0, 0), (4, 20))), [0, 1, 2, 4]),
    "FX-SELECT-008": ("FX-SELECT-001 moved three pixels right: the same, moved.",
                      case(shift=3), [0, 3]),
    "FX-SELECT-009": ("Tolerance 255, keep others: every pixel that shows is chosen, so the "
                      "frame is empty.",
                      case(tolerance=255, keep="others"), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-SELECT-010": ("Tolerance 256, above 255.", case(tolerance=256)),
    "FX-SELECT-011": ("Tolerance -1, below 0.", case(tolerance=-1)),
    "FX-SELECT-012": ("Tolerance keyed to 300 at frame 4.",
                      case(tolerance=keyed((0, 0), (4, 300)))),
    "FX-SELECT-013": ("Nine colours, one more than eight.",
                      case(colors=[f"#0000{i:02x}" for i in range(9)])),
    "FX-SELECT-014": ("A colour written \"#12345\", one digit short.",
                      case(colors=[LINE_HEX, "#12345"])),
    "FX-SELECT-015": ("Keep \"both\", which is not a choice.", case(keep="both")),
    "FX-SELECT-016": ("Keep \"Chosen\", in a capital, which is kept as written and is not the "
                      "word.", case(keep="Chosen")),
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
        "instance_id": "fx-0-0", "type_id": "core.select_color", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in ("colors", "tolerance", "keep")}}]
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

    (OUT / "expected_select_color.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    empty = [[0.0] * 4] * (W * H)
    at = lambda x, y: y * W + x  # noqa: E731
    one, two = c["FX-SELECT-001"]["0"], c["FX-SELECT-002"]["0"]

    # Keep chosen and keep others split the drawing between them, pixel for pixel.
    for i in range(W * H):
        assert (one[i] == drawn[i] and two[i] == [0.0] * 4) or \
               (two[i] == drawn[i] and one[i] == [0.0] * 4), i
    assert sum(p[3] > 0 for p in one) == 12 * 2 + 4 * 2 + 6
    assert one[at(1, 4)][3] == 128 / 255 and one[at(5, 4)] == [0.0] * 4
    assert two[at(5, 5)] == drawn[at(5, 5)] and two[at(2, 4)] == [0.0] * 4
    for fx in ("FX-SELECT-003", "FX-SELECT-004"):
        assert c[fx]["0"] == drawn, fx
    five = c["FX-SELECT-005"]["0"]
    assert five[at(5, 5)] == drawn[at(5, 5)] and five[at(5, 4)] == [0.0] * 4
    assert c["FX-SELECT-006"]["0"] == one
    seven = c["FX-SELECT-007"]
    assert seven["0"] == empty and seven["1"] == empty and seven["2"] == one and seven["4"] == one
    moved = c["FX-SELECT-008"]["0"]
    assert moved == c["FX-SELECT-008"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == one[at(0, y):at(W - 3, y)]
    assert c["FX-SELECT-009"]["0"] == empty


if __name__ == "__main__":
    main()
