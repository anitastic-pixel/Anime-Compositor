"""Fast Blur (Legacy), worked a second way.

D-471 adds After Effects' Fast Blur (Legacy), in its Obsolete folder, as `core.fast_blur_legacy`,
a second name over Fast Box Blur's engine (D-327), as D-383 and D-394 ("both names, one engine").
Fast Box Blur's own file and fixtures do not change.

What is published:

- After Effects' Fast Blur was renamed Fast Blur (Legacy) and moved to Obsolete when Box Blur
  became Fast Box Blur; Adobe's UpgradeLegacyBlurs script, shipped with After Effects, turns a
  Fast Blur into a Fast Box Blur (Adobe community answers, "Alternative to Fast Blur").
- Stu Maschwitz, "A Tale of Three Blurs" (prolost.com, 2006): "At three iterations, Box Blur is
  identical in quality to Fast Blur", but "Box Blur's main slider works differently than Fast
  Blur's". His "Five Blurs" post: "a Fast Blur (Legacy) with a Blur Radius of 100 is a perfect
  visual match for a Fast Box Blur with a radius of 37".
- The controls: Blurriness, Blur Dimensions and Repeat Edge Pixels.

So `core.fast_blur_legacy`:

    blurriness    0 to 1000, keyable, 0 when added (After Effects' start); radius = 0.37 x it
    dimensions    "both", "horizontal" or "vertical", as Gaussian Blur's (D-303)
    edges         "transparent" or "repeat" (Repeat Edge Pixels), as Gaussian Blur's (D-109)

drawn as Fast Box Blur at radius 0.37 x Blurriness and 3 iterations, with the same dimensions
and edges. The factor 0.37 is Maschwitz's visual match, not Adobe's; the range's top, 1000
(radius 370, inside Fast Box Blur's 500), is ours.

**This file never runs the build's code path.** It works in double precision on lists through
`tools/fast_box_blur_reference.py`'s box, pass by pass, from the drawing's 8-bit values.

Every case is blurriness_reference's composition, 40 by 12, and its drawing `block`, a 4 by 4
orange square against the left edge. The projects go into `Fixtures/fast_blur_legacy`, the
expected frames into `Fixtures/fast_blur_legacy/expected_fast_blur_legacy.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/fast_blur_legacy_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import blurriness_reference as B  # noqa: E402
import fast_box_blur_reference as F  # noqa: E402
import smooth_reference as S  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402

W, H, FRAMES = B.W, B.H, B.FRAMES
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "fast_blur_legacy"
TOLERANCE = 2e-5
FACTOR = 0.37  # Maschwitz: Fast Blur (Legacy) 100 matches Fast Box Blur 37
ITERATIONS = 3  # "At three iterations, Box Blur is identical in quality to Fast Blur"


def case(blurriness, edges=None, dimensions=None, stops=None, float_depth=False):
    return {"blurriness": blurriness, "edges": edges, "dimensions": dimensions,
            "stops": stops, "float": float_depth}


def render(c, frame=0):
    b = value_at(c["blurriness"], frame)
    px = F.blur(B.plain(), FACTOR * b, ITERATIONS, c["edges"] or "transparent",
                c["dimensions"] or "both")
    return B.exposure(px, c["stops"]) if c["stops"] is not None else px


CASES = {
    "FX-FASTLEG-001": ("Blurriness 10: Fast Box Blur at radius 3.7 and 3 iterations, reaching 12 "
                       "pixels; column 15, twelve right of the square, is lit and column 16 is "
                       "clear.", case(10), [0]),
    "FX-FASTLEG-002": ("Blurriness 5: radius 1.85, the box of 3 pixels and 0.85 of the next on "
                       "each side, laid on three times, reaching 6 pixels.", case(5), [0]),
    "FX-FASTLEG-003": ("FX-FASTLEG-001 with Repeat Edge Pixels: the square's orange is read past "
                       "the left edge, and the layer does not grow.", case(10, edges="repeat"),
                       [0]),
    "FX-FASTLEG-004": ("FX-FASTLEG-001 with Blur Dimensions horizontal: spread across only.",
                       case(10, dimensions="horizontal"), [0]),
    "FX-FASTLEG-005": ("FX-FASTLEG-001 with Blur Dimensions vertical: spread down only.",
                       case(10, dimensions="vertical"), [0]),
    "FX-FASTLEG-006": ("Blurriness 0: the drawing untouched.", case(0), [0]),
    "FX-FASTLEG-007": ("Blurriness keyed from 0 at frame 0 to 20 at frame 4, linear: frame 2 is "
                       "Blurriness 10, FX-FASTLEG-001's picture.",
                       case(keyed((0, 0), (4, 20))), [0, 2, 4]),
    "FX-FASTLEG-008": ("A Float composition: Blurriness 5, then Exposure +6. The light falls away "
                       "to nothing 6 pixels right of the square and no further.",
                       case(5, stops=6, float_depth=True), [0]),
}

INVALID = {
    "FX-FASTLEG-009": ("Blurriness -1, below 0.", case(-1)),
    "FX-FASTLEG-010": ("Blurriness 1001, above 1000.", case(1001)),
    "FX-FASTLEG-011": ("Blur Dimensions \"Both\": the word is exact, so a capital is not it.",
                       case(10, dimensions="Both")),
    "FX-FASTLEG-012": ("Repeat Edge Pixels \"on\": the word is \"transparent\" or \"repeat\".",
                       case(10, edges="on")),
    "FX-FASTLEG-013": ("Blurriness keyed to 1200 at frame 4.", case(keyed((0, 10), (4, 1200)))),
}


def project_json(fx, c):
    p = B.project_json(fx, B.case(1, units=None, stops=c["stops"], float_depth=c["float"]))
    params = {"blurriness": setting_json(c["blurriness"])}
    for word in ("edges", "dimensions"):
        if c[word] is not None:
            params[word] = c[word]
    p["compositions"][0]["layers"][0]["effects"][0] = {
        "instance_id": "fx-0-0", "type_id": "core.fast_blur_legacy", "enabled": True,
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
    (OUT / "expected_fast_blur_legacy.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                        encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    one = c["FX-FASTLEG-001"]["0"]
    # Fast Box Blur's own case at the same radius, number for number.
    assert one == F.render(F.case(3.7))
    assert one[at(15, 6)][3] > 0 and all(one[at(x, y)] == [0.0] * 4
                                         for x in range(16, W) for y in range(H))
    two = c["FX-FASTLEG-002"]["0"]
    assert two[at(9, 6)][3] > 0 and two[at(10, 6)] == [0.0] * 4
    assert c["FX-FASTLEG-003"]["0"][at(0, 6)][3] > one[at(0, 6)][3] + 0.05
    four = c["FX-FASTLEG-004"]["0"]
    assert all(four[at(x, y)] == [0.0] * 4 for x in range(W) for y in (0, 1, 2, 3, 8, 9, 10, 11))
    five = c["FX-FASTLEG-005"]["0"]
    assert all(five[at(x, y)] == [0.0] * 4 for x in range(4, W) for y in range(H))
    assert five[at(0, 0)][3] > 0
    assert c["FX-FASTLEG-006"]["0"] == B.plain()
    seven = c["FX-FASTLEG-007"]
    assert seven["0"] == B.plain() and seven["2"] == one and seven["4"] == render(case(20))
    eight = c["FX-FASTLEG-008"]["0"]
    assert eight[at(1, 6)][0] > 1 and eight[at(9, 6)][0] > 0 and eight[at(10, 6)] == [0.0] * 4
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12, (name, p)
    print("checked")


if __name__ == "__main__":
    main()
