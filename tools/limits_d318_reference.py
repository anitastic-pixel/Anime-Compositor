"""D-318's limit of exposure, worked a second way.

The owner's D-308 approval (2026-10-04) widens exposure's `stops` from D-90's -20..20 to
After Effects' -40..40. That supersedes three of `tools/limits_reference.py`'s cases, which are
kept as written in `Fixtures/limits/expected_limits.json` and marked superseded in document 25:
FX-LIMIT-004 (an ease past 20 is no longer held at 20), FX-LIMIT-006 and FX-LIMIT-007 (21 and -21
are ordinary values now). The cases below pin the new edges. Everything else is
`tools/limits_reference.py`'s, with the limit at 40; its cases are not written again.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/limits_d318_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import limits_reference as L  # noqa: E402
from adjust_reference import BG, decoded  # noqa: E402
from ease_reference import ease  # noqa: E402
from fxkey_reference import keyed, OVERSHOOT  # noqa: E402

L.STOPS_MAX = 40

CASES = {
    "FX-LIMIT-011": ("Exposure 40 stops, the top of D-318's range: every colour 2^40 times as "
                     "much, the coverage unchanged.",
                     {"layers": [BG, L.adjustment(("exposure", 40))]}, [0]),
    "FX-LIMIT-012": ("Exposure -40 stops, the bottom of D-318's range: every colour 2^40 times "
                     "less.",
                     {"layers": [BG, L.adjustment(("exposure", -40))]}, [0]),
    "FX-LIMIT-013": ("Exposure 21 stops, past D-90's old top: an ordinary value now, every "
                     "colour 2^21 times as much.",
                     {"layers": [BG, L.adjustment(("exposure", 21))]}, [0]),
    "FX-LIMIT-014": ("Exposure eased from 0 to 40 stops on the overshooting curve: past the "
                     "middle it would pass 40, and is held at 40.",
                     {"layers": [BG, L.adjustment(("exposure", keyed((0, 0, OVERSHOOT),
                                                                      (4, 40))))]}, [1, 2]),
}

INVALID = {
    "FX-LIMIT-015": ("Exposure 41 stops, one past D-318's top.", BG, ("exposure", 41)),
    "FX-LIMIT-016": ("Exposure -41 stops, one past D-318's bottom.", BG, ("exposure", -41)),
}


def main():
    expected = {"relative_tolerance": L.RELATIVE_TOLERANCE, "width": L.W, "height": L.H,
                "cases": {}}
    for fx, (says, case, frames) in CASES.items():
        rendered = {str(f): L.render(case, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": L.write(fx, case), "frames": rendered}
        print(f"{fx}: {says}")
    for fx, (says, drawing, effect) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        case = {"layers": [drawing, L.adjustment(effect)]}
        below = decoded(drawing["drawing"])
        expected["cases"][fx] = {"says": says, "project": L.write(fx, case),
                                 "frames": {"0": below, "4": below},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: {says}")
    (L.OUT / "expected_limits_d318.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")

    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    assert c["FX-LIMIT-011"]["0"][0] == [2 ** 40, 0, 0, 1]
    assert c["FX-LIMIT-012"]["0"][0] == [2 ** -40, 0, 0, 1]
    assert c["FX-LIMIT-013"]["0"][0] == [2 ** 21, 0, 0, 1]
    assert ease(OVERSHOOT, 0.5) > 1 and 40 * ease(OVERSHOOT, 0.25) < 40
    assert c["FX-LIMIT-014"]["2"] == c["FX-LIMIT-011"]["0"]
    assert c["FX-LIMIT-014"]["1"] != c["FX-LIMIT-011"]["0"]
    for fx in INVALID:
        assert c[fx]["0"] == decoded("bg")


if __name__ == "__main__":
    main()
