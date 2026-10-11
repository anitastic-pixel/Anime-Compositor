"""Gaussian Blur (Legacy), worked a second way.

D-472 adds After Effects' Gaussian Blur (Legacy), in its Obsolete folder, as
`core.gaussian_blur_legacy`, a second name over Gaussian Blur's engine in Blurriness (D-321), as
D-383 and D-394 ("both names, one engine"). Gaussian Blur's own file and fixtures do not change.

What is published:

- After Effects CC 2017 added a new Gaussian Blur "meant to replace both Fast Blur and Gaussian
  Blur"; the old one was renamed Gaussian Blur (Legacy) and moved to Obsolete (Stu Maschwitz,
  "Five Blurs", prolost.com). Adobe's UpgradeLegacyBlurs script, shipped with After Effects,
  turns a Gaussian Blur (Legacy) into a Gaussian Blur (Adobe community answers, "Missing Gaussian
  Blur effect in After Effects CC 2015"); users there report the two do not match exactly.
- Maschwitz, "A Tale of Three Blurs" (2006), advised never to use the old Gaussian Blur for what
  it lacked: it has only Blurriness and Blur Dimensions, no Repeat Edge Pixels.

So `core.gaussian_blur_legacy`:

    blurriness    0 to 500, keyable, 0 when added; Gaussian Blur's Blurriness: sigma 0.3 x it,
                  the kernel reaching ceil(6.5 sigma) pixels (D-321)
    dimensions    "both", "horizontal" or "vertical", as Gaussian Blur's (D-303)

drawn as Gaussian Blur in Blurriness with those dimensions and its edges always transparent (the
layer grows by the reach). The old effect's own sigma is not published: Gaussian Blur's
Blurriness rule is taken, as Adobe's upgrade script takes it; the range's top, 500, is Gaussian
Blur's.

**This file never runs the build's code path.** It works in double precision on lists through
`tools/blurriness_reference.py`'s blur, from the drawing's 8-bit values.

Every case is blurriness_reference's composition, 40 by 12, and its drawing `block`, a 4 by 4
orange square against the left edge. The projects go into `Fixtures/gaussian_blur_legacy`, the
expected frames into `Fixtures/gaussian_blur_legacy/expected_gaussian_blur_legacy.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/gaussian_blur_legacy_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import blurriness_reference as B  # noqa: E402
import smooth_reference as S  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402

W, H, FRAMES = B.W, B.H, B.FRAMES
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "gaussian_blur_legacy"
TOLERANCE = 2e-5


def case(blurriness, dimensions=None, stops=None, float_depth=False):
    return {"blurriness": blurriness, "dimensions": dimensions, "stops": stops,
            "float": float_depth}


def render(c, frame=0):
    b = value_at(c["blurriness"], frame)
    px = B.blur(B.plain(), b, "blurriness", "transparent", c["dimensions"] or "both")
    return B.exposure(px, c["stops"]) if c["stops"] is not None else px


CASES = {
    "FX-GAUSSLEG-001": ("Blurriness 10: Gaussian Blur's Blurriness 10, a Gaussian of sigma 3 "
                        "reaching 20 pixels, FX-BLURRY-001's frame.", case(10), [0]),
    "FX-GAUSSLEG-002": ("Blurriness 4: sigma 1.2, reaching 8 pixels; column 11, eight right of "
                        "the square, is lit and column 12 is clear.", case(4), [0]),
    "FX-GAUSSLEG-003": ("Blurriness 10, Blur Dimensions horizontal: spread across only.",
                        case(10, dimensions="horizontal"), [0]),
    "FX-GAUSSLEG-004": ("Blurriness 10, Blur Dimensions vertical: spread down only.",
                        case(10, dimensions="vertical"), [0]),
    "FX-GAUSSLEG-005": ("Blurriness 0: the drawing untouched.", case(0), [0]),
    "FX-GAUSSLEG-006": ("Blurriness keyed from 0 at frame 0 to 20 at frame 4, linear: frame 2 is "
                        "Blurriness 10, FX-GAUSSLEG-001's picture.",
                        case(keyed((0, 0), (4, 20))), [0, 2, 4]),
    "FX-GAUSSLEG-007": ("A Float composition: Blurriness 4, then Exposure +6. The light falls "
                        "away smoothly for eight pixels right of the square and no further.",
                        case(4, stops=6, float_depth=True), [0]),
}

INVALID = {
    "FX-GAUSSLEG-008": ("Blurriness -1, below 0.", case(-1)),
    "FX-GAUSSLEG-009": ("Blurriness 501, above 500.", case(501)),
    "FX-GAUSSLEG-010": ("Blur Dimensions \"Both\": the word is exact, so a capital is not it.",
                        case(10, dimensions="Both")),
    "FX-GAUSSLEG-011": ("Blurriness keyed to 600 at frame 4.", case(keyed((0, 10), (4, 600)))),
}


def project_json(fx, c):
    p = B.project_json(fx, B.case(1, units=None, stops=c["stops"], float_depth=c["float"]))
    params = {"blurriness": setting_json(c["blurriness"])}
    if c["dimensions"] is not None:
        params["dimensions"] = c["dimensions"]
    p["compositions"][0]["layers"][0]["effects"][0] = {
        "instance_id": "fx-0-0", "type_id": "core.gaussian_blur_legacy", "enabled": True,
        "parameters": params}
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "block.png").write_bytes(S.png(B.DRAWING))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {str(f): render(c, f) for f in frames}}
        print(f"{fx}: {says[:60]}")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": B.plain(), "4": B.plain()},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_gaussian_blur_legacy.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                            encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    one = c["FX-GAUSSLEG-001"]["0"]
    # Gaussian Blur's own case in Blurriness, number for number.
    assert one == B.render(B.case(10))
    two = c["FX-GAUSSLEG-002"]["0"]
    assert two[at(11, 6)][3] > 0 and all(two[at(x, y)] == [0.0] * 4
                                         for x in range(12, W) for y in range(H))
    three = c["FX-GAUSSLEG-003"]["0"]
    assert all(three[at(x, y)] == [0.0] * 4 for x in range(W) for y in (0, 1, 2, 3, 8, 9, 10, 11))
    four = c["FX-GAUSSLEG-004"]["0"]
    assert all(four[at(x, y)] == [0.0] * 4 for x in range(4, W) for y in range(H))
    assert four[at(0, 0)][3] > 0
    assert c["FX-GAUSSLEG-005"]["0"] == B.plain()
    six = c["FX-GAUSSLEG-006"]
    assert six["0"] == B.plain() and six["2"] == one and six["4"] == render(case(20))
    seven = c["FX-GAUSSLEG-007"]["0"]
    assert seven[at(1, 6)][0] > 1 and seven[at(11, 6)][0] > 0 and seven[at(12, 6)] == [0.0] * 4
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12, (name, p)
    print("checked")


if __name__ == "__main__":
    main()
