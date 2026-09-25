"""The limits of exposure and the blur, worked a second way.

D-90 gives two settings that had no upper end one: a Gaussian blur's `sigma_px` runs from 0 to
500, and exposure's `stops` from -20 to 20. A value on the limit is an ordinary value and renders
as document 21 says. A value past it is D-46's invalid parameter: the file is read, the effect is
kept exactly as written and left out of every frame with `EFFECT_PARAMETER_INVALID`, and a
command that would set one is refused. A keyed setting whose ease overshoots between two keys that
are in range is held at the limit, as D-68 holds a tint amount at 1.

**This file never runs the build's code path.** It renders each tiny frame pixel by pixel through
`tools/adjust_reference.py` and `tools/fxkey_reference.py`, and blurs with the two-dimensional
kernel summed directly, where the build runs two one-dimensional passes. The kernel is summed only
over the pixels inside the frame, because at sigma 500 it reaches 1,500 pixels past a frame six
wide and everything out there is transparent.

Every case is a composition 6 pixels by 2 and five frames long. The projects and drawings go into
`Fixtures/limits`, and the expected frames into `Fixtures/limits/expected_limits.json`. At 20
stops a pixel is a million and at sigma 500 it is a millionth, so a sample agrees when it is
within 1e-4 of the expected value's own size, and exactly when the expected value is 0.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/limits_reference.py
"""

import json
import sys
from math import ceil, exp
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import adjust_reference as adj  # noqa: E402
import fxkey_reference as fxk  # noqa: E402
from adjust_reference import W, H, BG, DOT, decoded, fmt  # noqa: E402
from ease_reference import ease  # noqa: E402
from fxkey_reference import keyed, value_at, OVERSHOOT  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "limits"
RELATIVE_TOLERANCE = 1e-4
SIGMA_MAX = 500
STOPS_MAX = 20


def blur(frame, sigma):
    """adjust_reference's blur, summed only over the frame's own pixels."""
    r = ceil(3 * sigma)
    if r == 0:
        return [list(p) for p in frame]
    total = sum(exp(-(d * d) / (2 * sigma * sigma)) for d in range(-r, r + 1))
    w = lambda d: exp(-(d * d) / (2 * sigma * sigma)) / total  # noqa: E731
    out = []
    for y in range(H):
        for x in range(W):
            acc = [0.0] * 4
            for sy in range(H):
                for sx in range(W):
                    k = w(sx - x) * w(sy - y)
                    for c in range(4):
                        acc[c] += frame[sy * W + sx][c] * k
            out.append(acc)
    return out


def in_range(kind, args):
    """D-90 on top of D-68: the worked-out value is held inside the setting's range."""
    if kind == "blur":
        return [min(SIGMA_MAX, max(0.0, args[0]))]
    if kind == "exposure":
        return [min(STOPS_MAX, max(-STOPS_MAX, args[0]))]
    return args


def render(case, frame_no):
    """One drawing and one adjustment layer above it, both the composition's size."""
    drawing, layer = case["layers"]
    frame = decoded(drawing["drawing"])
    for kind, *args in layer["effects"]:
        args = in_range(kind, [value_at(a, frame_no) for a in args])
        frame = blur(frame, *args) if kind == "blur" else adj.EFFECTS[kind](frame, *args)
    return frame


def adjustment(*effects):
    return {"id": "adj", "kind": "adjustment", "effects": list(effects)}


CASES = {
    "FX-LIMIT-001": ("Exposure 20 stops, the top of its range: every colour 2^20 times as much, "
                     "the coverage unchanged.",
                     {"layers": [BG, adjustment(("exposure", 20))]}, [0]),
    "FX-LIMIT-002": ("Exposure -20 stops, the bottom of its range: every colour 2^20 times "
                     "less.",
                     {"layers": [BG, adjustment(("exposure", -20))]}, [0]),
    "FX-LIMIT-003": ("A Gaussian blur of sigma 500, the top of its range: one pixel spread "
                     "almost evenly over the frame and far past it.",
                     {"layers": [DOT, adjustment(("blur", 500))]}, [0]),
    "FX-LIMIT-004": ("Exposure eased from 0 to 20 stops on the overshooting curve: past the "
                     "middle it would pass 20, and is held at 20.",
                     {"layers": [BG, adjustment(("exposure", keyed((0, 0, OVERSHOOT),
                                                                    (4, 20))))]}, [1, 2]),
    "FX-LIMIT-005": ("A blur eased from 0 to 500 on the overshooting curve: held at 500.",
                     {"layers": [DOT, adjustment(("blur", keyed((0, 0, OVERSHOOT),
                                                                (4, 500))))]}, [2]),
}

# Each is outside its range. D-46: the file is read, the effect is kept as written and left out
# of every frame, with EFFECT_PARAMETER_INVALID. The frame is the drawing's.
INVALID = {
    "FX-LIMIT-006": ("Exposure 21 stops, one past the top.", BG, ("exposure", 21)),
    "FX-LIMIT-007": ("Exposure -21 stops, one past the bottom.", BG, ("exposure", -21)),
    "FX-LIMIT-008": ("Exposure 128 stops over a drawing that is mostly transparent: past the "
                     "limit, so left out, where before it made every transparent pixel "
                     "not-a-number.", DOT, ("exposure", 128)),
    "FX-LIMIT-009": ("A Gaussian blur of sigma 501, one past the top.", DOT, ("blur", 501)),
    "FX-LIMIT-010": ("A blur keyed from 0 at frame 0 to 600 at frame 4: one key past the top, so "
                     "left out of every frame, frame 0 as well.",
                     DOT, ("blur", keyed((0, 0), (4, 600)))),
}


def write(fx, case):
    project = f"{fx.lower().replace('-', '_')}.json"
    (OUT / project).write_text(json.dumps(fxk.project_json(fx, case), indent=2) + "\n",
                               encoding="utf-8")
    return project


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name in ("bg", "dot"):
        (OUT / "media" / f"{name}.png").write_bytes(adj.png(adj.DRAWINGS[name]))

    expected = {"relative_tolerance": RELATIVE_TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, case, frames) in CASES.items():
        rendered = {str(f): render(case, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, case), "frames": rendered}

        print(f"{fx}: {says}\n")
        print("| frame, row | " + " | ".join(f"x = {x}" for x in range(W)) + " |")
        print("| --- " * (W + 1) + "|")
        for f, px in rendered.items():
            for y in range(H):
                cells = (" ".join(fmt(v) for v in px[y * W + x]) for x in range(W))
                print(f"| {f}, {y} | " + " | ".join(cells) + " |")
        print()

    for fx, (says, drawing, effect) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        case = {"layers": [drawing, adjustment(effect)]}
        below = decoded(drawing["drawing"])
        expected["cases"][fx] = {"says": says, "project": write(fx, case),
                                 "frames": {"0": below, "4": below},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: {says} Frames 0 and 4 are the {drawing['drawing']} drawing, unchanged.\n")

    (OUT / "expected_limits.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")

    # The claims the cases are there to make, checked on the numbers just worked.
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    bg, dot = decoded("bg"), decoded("dot")
    assert c["FX-LIMIT-001"]["0"][0] == [2 ** 20, 0, 0, 1]
    assert c["FX-LIMIT-002"]["0"][0] == [2 ** -20, 0, 0, 1]
    three = c["FX-LIMIT-003"]["0"]
    assert min(p[3] for p in three) > 0.99 * max(p[3] for p in three), "not almost even"
    assert sum(p[3] for p in three) < 1e-5, "the frame kept what spread past it"
    assert ease(OVERSHOOT, 0.5) > 1 and 20 * ease(OVERSHOOT, 0.25) < 20
    assert c["FX-LIMIT-004"]["2"] == c["FX-LIMIT-001"]["0"]
    assert c["FX-LIMIT-004"]["1"] != c["FX-LIMIT-001"]["0"]
    assert c["FX-LIMIT-005"]["2"] == three
    for fx in INVALID:
        assert c[fx]["0"] in (bg, dot)


if __name__ == "__main__":
    main()
