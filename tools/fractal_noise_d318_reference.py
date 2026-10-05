"""D-318's Fractal Noise ranges, worked a second way.

The owner's D-308 approval (2026-10-04) widens `complexity` from D-128's 1..8 to After Effects'
1..20 and `brightness` from -100..100 to -200..200. That supersedes FX-FRACTAL-022 (complexity 9)
and FX-FRACTAL-024 (brightness -101), which are kept as written in
`Fixtures/fractal_noise/expected_fractal_noise.json` and marked superseded in document 25. The
cases below pin the new edges, with `tools/fractal_noise_reference.py`'s field and rule; its own
cases are not written again.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/fractal_noise_d318_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import fractal_noise_reference as F  # noqa: E402

F.RANGES["complexity"] = (1, 20)
F.RANGES["brightness"] = (-200, 200)
FINE = {"size": 4}

CASES = {
    "FX-FRACTAL-029": ("Size 4, complexity 20: twenty octaves, finer detail on top of "
                       "FX-FRACTAL-004's eight.",
                       F.case(complexity=20, **FINE), [0]),
    "FX-FRACTAL-030": ("Size 4, contrast 600, brightness -154: the clouds pushed apart and "
                       "darkened, most of the card held at the dark colour, the brightest "
                       "peaks still showing.",
                       F.case(contrast=600, brightness=-154, **FINE), [0]),
    "FX-FRACTAL-031": ("Size 4, brightness 200: every value held at the light colour.",
                       F.case(brightness=200, **FINE), [0]),
}

INVALID = {
    "FX-FRACTAL-032": ("Complexity 21, above 20.", F.case(complexity=21)),
    "FX-FRACTAL-033": ("Brightness -201, below -200.", F.case(brightness=-201)),
}


def main():
    expected = {"tolerance": F.TOLERANCE, "width": F.W, "height": F.H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): F.render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": F.write(fx, c), "frames": rendered}
        print(f"{fx}: {says}")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = F.plain(c)
        expected["cases"][fx] = {"says": says, "project": F.write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (F.OUT / "expected_fractal_noise_d318.json").write_text(
        json.dumps(expected, indent=1) + "\n", encoding="utf-8")

    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    eight = F.render(F.case(complexity=8, **FINE), 0)
    assert c["FX-FRACTAL-029"]["0"] != eight, "twenty octaves are not eight"
    assert max(abs(a - b) for p, q in zip(c["FX-FRACTAL-029"]["0"], eight)
               for a, b in zip(p, q)) < 0.05, "the octaves past eight are faint"
    light = F.render(F.case(brightness=100, contrast=0, **FINE), 0)
    assert c["FX-FRACTAL-031"]["0"] == light, "brightness 200 is all light"
    dark = F.render(F.case(brightness=-100, contrast=0, **FINE), 0)
    thirty = c["FX-FRACTAL-030"]["0"]
    assert thirty != dark and sum(p == q for p, q in zip(thirty, dark)) > len(dark) // 2


if __name__ == "__main__":
    main()
