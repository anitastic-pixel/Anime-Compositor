"""D-326's Fractal Noise brightness past -200..200, worked a second way.

Tutorial 3 (P-26) keys Fractal Noise's Brightness to -204, which After Effects takes when it is
typed in, past the slider's -200. D-318 held this build to -200..200, so the key was set to -200.
The owner asked on 2026-10-04 for what D-308 left unbuilt; D-326 widens `brightness` to
-1000..1000 (this program's choice, beside `contrast`'s 0..1000: no After Effects limit was found).
That supersedes FX-FRACTAL-033 (brightness -201), kept as written in
`Fixtures/fractal_noise/expected_fractal_noise_d318.json` and marked superseded in document 25.
The cases below pin the new edges, with `tools/fractal_noise_reference.py`'s field and rule; its
own cases are not written again.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/fractal_noise_d326_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import fractal_noise_reference as F  # noqa: E402

F.RANGES["complexity"] = (1, 20)
F.RANGES["brightness"] = (-1000, 1000)
FINE = {"size": 4}

CASES = {
    "FX-FRACTAL-034": ("Size 4, contrast 1000, brightness -204, tutorial 3's typed value: a "
                       "little darker than brightness -200, the brightest peaks still showing.",
                       F.case(contrast=1000, brightness=-204, **FINE), [0]),
    "FX-FRACTAL-035": ("Size 4, brightness 1000: every value held at the light colour.",
                       F.case(brightness=1000, **FINE), [0]),
    "FX-FRACTAL-036": ("Size 4, contrast 1000, brightness -1000: every value held at the dark "
                       "colour.",
                       F.case(contrast=1000, brightness=-1000, **FINE), [0]),
}

INVALID = {
    "FX-FRACTAL-037": ("Brightness -1001, below -1000.", F.case(brightness=-1001)),
    "FX-FRACTAL-038": ("Brightness 1001, above 1000.", F.case(brightness=1001)),
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
    (F.OUT / "expected_fractal_noise_d326.json").write_text(
        json.dumps(expected, indent=1) + "\n", encoding="utf-8")

    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    at_200 = F.render(F.case(contrast=1000, brightness=-200, **FINE), 0)
    dark = F.render(F.case(brightness=-100, contrast=0, **FINE), 0)
    light = F.render(F.case(brightness=100, contrast=0, **FINE), 0)
    four = c["FX-FRACTAL-034"]
    assert four != at_200, "-204 is not -200"
    assert all(p[0] <= q[0] + 1e-12 for p, q in zip(four, at_200)), "-204 is never brighter"
    assert four != dark, "the brightest peaks still show"
    assert c["FX-FRACTAL-035"] == light, "brightness 1000 is all light"
    assert c["FX-FRACTAL-036"] == dark, "brightness -1000 is all dark"
    print("differing pixels from -200:", sum(p != q for p, q in zip(four, at_200)), "of", len(four))


if __name__ == "__main__":
    main()
