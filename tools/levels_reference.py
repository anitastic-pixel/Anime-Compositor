"""Levels, worked a second way.

D-112 adds `core.levels`. It sets a drawing's darkest and lightest tones and bends the middle
between them, as the input and output levels of a photograph are set. On the 0 to 255 scale of
a colour as a drawing program holds it, each channel of a pixel that shows is taken from the
input range `input_black` to `input_white` to 0..1, held there; raised to the power
1 / `gamma`, so a gamma above 1 lightens the middle; and laid on the output range
`output_black` to `output_white`. An input white below the black, or an output black above the
white, inverts; an input white equal to the black is a threshold. The colour comes back to
linear at the pixel's own covering. A pixel that does not show is left as it is. It is this
program's own method, modelled on After Effects' Levels; nothing is ported. Document 21 is the
rule in words; this file is the reference for the numbers document 25 pins against it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: Curves' colour chart (`tools/curves_reference.py`). The drawing goes into
`Fixtures/levels/media`, the projects into `Fixtures/levels`, and the expected frames into
`Fixtures/levels/expected_levels.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/levels_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import curves_reference as C  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "levels"
TOLERANCE = 2e-5  # document 25's default for a filter
NAMES = ("input_black", "input_white", "gamma", "output_black", "output_white")
RANGES = {"input_black": (0, 255), "input_white": (0, 255), "gamma": (0.1, 10),
          "output_black": (0, 255), "output_white": (0, 255)}


# --- the rule -------------------------------------------------------------------------------

def level(x, input_black, input_white, gamma, output_black, output_white):
    """One channel's 0..255 value through the levels, to 0..255."""
    if input_white == input_black:
        v = 1.0 if x >= input_black else 0.0
    else:
        v = C.clamp((x - input_black) / (input_white - input_black), 0, 1)
    v = v ** (1 / gamma)
    return output_black + v * (output_white - output_black)


def levels(pixels, *settings):
    return C.grade(pixels, [lambda x: level(x, *settings)] * 3)


# --- the drawing ----------------------------------------------------------------------------

DRAWINGS = C.DRAWINGS  # Curves' colour chart, written again into this effect's own media


# --- the cases ------------------------------------------------------------------------------

def case(input_black=0, input_white=255, gamma=1, output_black=0, output_white=255, shift=0):
    return {"drawing": "chart", "input_black": input_black, "input_white": input_white,
            "gamma": gamma, "output_black": output_black, "output_white": output_white,
            "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(levels(pixels, *(held(c, k, frame_no) for k in NAMES)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return R.frame([R.working(p) for row in DRAWINGS[c["drawing"]] for p in row], c["shift"])


ALL = dict(input_black=32, input_white=224, gamma=1.5, output_black=16, output_white=240)

CASES = {
    "FX-LEVELS-001": ("Every setting its default, 0, 255, gamma 1, 0, 255: the drawing, "
                      "within the tolerance.",
                      case(), [0]),
    "FX-LEVELS-002": ("Input black 64: the ramp's greys at or below 64 become black, and every "
                      "colour darkens toward it; white stays.",
                      case(input_black=64), [0]),
    "FX-LEVELS-003": ("Input white 192: the ramp's greys at or above 192 become white, and every "
                      "colour lightens toward it; black stays.",
                      case(input_white=192), [0]),
    "FX-LEVELS-004": ("Gamma 2: the middle lightens, the ramp's 128 to about 180; black and "
                      "white stay.",
                      case(gamma=2), [0]),
    "FX-LEVELS-005": ("Gamma 0.5: the middle darkens, the ramp's 128 to about 64; black and "
                      "white stay.",
                      case(gamma=0.5), [0]),
    "FX-LEVELS-006": ("Output black 64: black becomes 64, every colour rises toward white a "
                      "little; white stays.",
                      case(output_black=64), [0]),
    "FX-LEVELS-007": ("Output white 192: white becomes 192, every colour falls toward it; "
                      "black stays.",
                      case(output_white=192), [0]),
    "FX-LEVELS-008": ("All five at once: input 32 to 224, gamma 1.5, output 16 to 240.",
                      case(**ALL), [0]),
    "FX-LEVELS-009": ("Input black 255 and input white 0, the white below the black: every "
                      "colour is inverted; the empty pixels stay empty.",
                      case(input_black=255, input_white=0), [0]),
    "FX-LEVELS-010": ("Output black 255 and output white 0: every colour is inverted, the same "
                      "as FX-LEVELS-009.",
                      case(output_black=255, output_white=0), [0]),
    "FX-LEVELS-011": ("Input black and white both 120: a threshold; each channel at or above "
                      "120 becomes 255 and each below becomes 0.",
                      case(input_black=120, input_white=120), [0]),
    "FX-LEVELS-012": ("Output black and white both 255: every pixel that shows becomes white at "
                      "its own covering, the soft ones still soft; the empty pixels stay empty.",
                      case(output_black=255), [0]),
    "FX-LEVELS-013": ("Gamma keyed from 1 at frame 0 to 3 at frame 4, linear: frame 0 is the "
                      "drawing, frame 2 is gamma 2, FX-LEVELS-004, and frame 4 is gamma 3.",
                      case(gamma=keyed((0, 1), (4, 3))), [0, 2, 4]),
    "FX-LEVELS-014": ("Gamma keyed from 1 at frame 0 to 10 at frame 4 on an ease that runs "
                      "past its end: frame 2 works out above 10 and is held at 10, the same as "
                      "frame 4.",
                      case(gamma=keyed((0, 1, OVERSHOOT), (4, 10))), [0, 2, 4]),
    "FX-LEVELS-015": ("Output white keyed from 255 at frame 0 to 128 at frame 4, linear: white "
                      "falls to 191.5 at frame 2 and to 128 at frame 4.",
                      case(output_white=keyed((0, 255), (4, 128))), [0, 2, 4]),
    "FX-LEVELS-016": ("Gamma 0.1, the least: everything but white falls almost to black.",
                      case(gamma=0.1), [0]),
    "FX-LEVELS-017": ("FX-LEVELS-008 moved three pixels right: the same, moved.",
                      case(shift=3, **ALL), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-LEVELS-018": ("Input black 256, above 255.", case(input_black=256)),
    "FX-LEVELS-019": ("Input white -1, below 0.", case(input_white=-1)),
    "FX-LEVELS-020": ("Gamma 0.05, below 0.1.", case(gamma=0.05)),
    "FX-LEVELS-021": ("Gamma 11, above 10.", case(gamma=11)),
    "FX-LEVELS-022": ("Output black -1, below 0.", case(output_black=-1)),
    "FX-LEVELS-023": ("Output white 256, above 255.", case(output_white=256)),
    "FX-LEVELS-024": ("Input black keyed to 300 at frame 4.",
                      case(input_black=keyed((0, 0), (4, 300)))),
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
        "instance_id": "fx-0-0", "type_id": "core.levels", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
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

    (OUT / "expected_levels.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    frames = {fx: v["frames"] for fx, v in expected["cases"].items()}
    c = {fx: f["0"] for fx, f in frames.items()}
    drawn = plain(case())
    art = DRAWINGS["chart"]
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda f, g: all(abs(p[i] - q[i]) < 1e-12  # noqa: E731
                             for p, q in zip(f, g) for i in range(4))
    showing = [(x, y) for y in range(H) for x in range(W) if art[y][x][3] > 0]
    empty = [(x, y) for y in range(H) for x in range(W) if art[y][x][3] == 0]

    def enc(f, x, y):
        p = f[at(x, y)]
        return [S.linear_to_srgb(p[i] / p[3]) * 255 for i in range(3)]

    def want(f, fn):
        """Every pixel that shows is the drawing's own with each 8-bit channel through fn, at
        its own covering; every empty one stays empty."""
        for x, y in showing:
            p, q = art[y][x], f[at(x, y)]
            assert abs(q[3] - p[3] / 255) < 1e-15, (x, y)
            got = enc(f, x, y)
            assert all(abs(got[i] - fn(p[i])) < 1e-9 for i in range(3)), (x, y, got)
        for x, y in empty:
            assert f[at(x, y)] == [0.0] * 4

    grey = lambda f, v: enc(f, 1 + C.RAMP.index(v), 3)[0]  # noqa: E731

    # The rule's own pieces.
    assert level(0, 0, 255, 1, 0, 255) == 0 and level(255, 0, 255, 1, 0, 255) == 1 * 255
    assert level(10, 64, 192, 1, 0, 255) == 0 and level(200, 64, 192, 1, 0, 255) == 255
    assert level(128, 120, 120, 1, 0, 255) == 255 and level(119.9, 120, 120, 1, 0, 255) == 0
    assert abs(level(64, 255, 0, 1, 0, 255) - 191) < 1e-12
    # No channel of the drawing lies near the threshold, so the build's single precision can't
    # land a channel on the other side of it.
    assert all(abs(v - 120) > 1 for row in art for p in row if p[3] for v in p[:3])

    assert close(c["FX-LEVELS-001"], drawn)
    want(c["FX-LEVELS-002"], lambda v: max(0, (v - 64) / 191) * 255)
    assert all(grey(c["FX-LEVELS-002"], v) < 1e-9 for v in C.RAMP if v <= 64)
    want(c["FX-LEVELS-003"], lambda v: min(1, v / 192) * 255)
    assert all(abs(grey(c["FX-LEVELS-003"], v) - 255) < 1e-9 for v in C.RAMP if v >= 192)
    want(c["FX-LEVELS-004"], lambda v: (v / 255) ** 0.5 * 255)
    assert 180 < grey(c["FX-LEVELS-004"], 128) < 181
    want(c["FX-LEVELS-005"], lambda v: (v / 255) ** 2 * 255)
    assert 64 < grey(c["FX-LEVELS-005"], 128) < 65
    want(c["FX-LEVELS-006"], lambda v: 64 + v / 255 * 191)
    want(c["FX-LEVELS-007"], lambda v: v / 255 * 192)
    want(c["FX-LEVELS-008"], lambda v: 16 + min(1, max(0, (v - 32) / 192)) ** (1 / 1.5) * 224)
    want(c["FX-LEVELS-009"], lambda v: 255 - v)
    assert close(c["FX-LEVELS-009"], c["FX-LEVELS-010"])
    want(c["FX-LEVELS-011"], lambda v: 255 if v >= 120 else 0)
    want(c["FX-LEVELS-012"], lambda v: 255)
    assert abs(c["FX-LEVELS-012"][at(1, 7)][3] - 32 / 255) < 1e-15
    k = frames["FX-LEVELS-013"]
    assert close(k["0"], drawn) and k["2"] == c["FX-LEVELS-004"]
    want(k["4"], lambda v: (v / 255) ** (1 / 3) * 255)
    e = frames["FX-LEVELS-014"]
    assert value_at(case(gamma=keyed((0, 1, OVERSHOOT), (4, 10)))["gamma"], 2) > 10
    assert close(e["0"], drawn) and e["2"] == e["4"]
    want(e["4"], lambda v: (v / 255) ** 0.1 * 255)
    o = frames["FX-LEVELS-015"]
    assert close(o["0"], drawn)
    want(o["2"], lambda v: v / 255 * 191.5)
    want(o["4"], lambda v: v / 255 * 128)
    want(c["FX-LEVELS-016"], lambda v: (v / 255) ** 10 * 255)
    assert grey(c["FX-LEVELS-016"], 192) < 15 and abs(grey(c["FX-LEVELS-016"], 255) - 255) < 1e-9
    moved = frames["FX-LEVELS-017"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == c["FX-LEVELS-008"][at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
