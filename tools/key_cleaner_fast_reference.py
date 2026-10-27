"""Key Cleaner's Fast quality, worked a second way.

D-912 (wave 11, pick 9, second step): Key Cleaner gets D-903's setting Quality, "exact" (as it
has always been drawn, and the default, never saved) or "fast", which the user chooses and which
exports draw too. Key Cleaner is Refine Soft Matte's fit (D-503), so Fast is exactly D-903's
fast guided filter (He and Sun, "Fast Guided Filter", 2015) in place of the guided filter, and
nothing else changes. Document 21 is the rule in words; this file is the reference for the
numbers document 25 pins against it (FX-KCLEAN-019 to 023). Everything else is
`key_cleaner_reference.py`'s and `matte_refine_fast_reference.py`'s, which this file imports and
does not change.

**This file never runs the build's code path**, as `key_cleaner_reference.py` says of itself.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/key_cleaner_fast_reference.py
"""

import json
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
import key_cleaner_reference as K  # noqa: E402
import matte_refine_fast_reference as F  # noqa: E402

MR = K.MR
EXPECTED = K.OUT / "expected_key_cleaner_fast.json"


def render(c, n, P=None):
    """Key Cleaner with the guided filter worked the fast way; nothing else of it changes."""
    MR.guided = F.guided_fast
    try:
        return K.render(c, n, P)
    finally:
        MR.guided = F.GUIDED


def fast(c):
    return dict(c, quality="fast")


CASES = {
    "FX-KCLEAN-019": ("FX-KCLEAN-001 (Key Cleaner as added, Additional Edge Radius 10) at Quality "
                      "Fast: the fit worked on blocks 2 by 2 at radius 5 and read back smoothly, "
                      "so the covering is close to FX-KCLEAN-001's and not the same.",
                      fast(K.cleaner())),
    "FX-KCLEAN-020": ("FX-KCLEAN-002 (Additional Edge Radius 4) at Quality Fast: below radius 8 "
                      "Fast is the exact filter, so FX-KCLEAN-002's frame.", fast(K.cleaner(4))),
    "FX-KCLEAN-021": ("Additional Edge Radius 12, Alpha Contrast 50, at Quality Fast: blocks 3 "
                      "by 3 at radius 4, the last row of blocks one pixel tall (16 rows), then "
                      "steepened.", fast(K.cleaner(12, 50))),
    "FX-KCLEAN-022": ("Additional Edge Radius 16, Strength 50, at Quality Fast: blocks 4 by 4 "
                      "(the most) at radius 4, then halfway back to the drawing.",
                      fast(K.cleaner(16, strength=50))),
}

INVALID = {
    "FX-KCLEAN-023": ("Key Cleaner, quality written \"medium\".",
                      dict(K.cleaner(), quality="medium")),
}


def write(fx, c):
    j = K.project_json(fx, c)
    j["compositions"][0]["layers"][0]["effects"][0]["parameters"]["quality"] = c["quality"]
    name = f"{fx.lower().replace('-', '_')}.json"
    (K.OUT / name).write_text(json.dumps(j, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    expected = {"tolerance": K.TOLERANCE, "width": K.W, "height": K.H, "cases": {}}
    rounded = K.drawn().astype(np.float32).astype(np.float64)
    for fx, (says, c) in sorted(CASES.items()):
        got = render(c, 0)
        moved_by = float(np.abs(render(c, 0, rounded) - got).max())
        assert moved_by < K.TOLERANCE / 10, (fx, moved_by)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": MR.as_list(got)}}
        exact = K.render(c, 0)
        print(f"{fx}: largest difference from exact {float(np.abs(got - exact).max()):.4f}")
    for fx, (says, c) in sorted(INVALID.items()):
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = K.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    EXPECTED.write_text(json.dumps(expected, indent=1) + "\n", encoding="utf-8")
    check(expected)


def check(expected):
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    exact = json.loads((K.OUT / "expected_key_cleaner.json").read_text(encoding="utf-8"))
    e = {fx: v["frames"]["0"] for fx, v in exact["cases"].items()}
    # Below radius 8 Fast is the exact filter, to the last digit.
    assert c["FX-KCLEAN-020"] == e["FX-KCLEAN-002"]
    # Fast is not exact where it works on blocks, and stays near it.
    alpha = lambda f: np.array([p[3] for p in f]).reshape(K.H, K.W)  # noqa: E731
    d = np.abs(alpha(c["FX-KCLEAN-019"]) - alpha(e["FX-KCLEAN-001"]))
    assert 1e-3 < d.max() < 0.5, d.max()
    for fx in ("FX-KCLEAN-021", "FX-KCLEAN-022"):
        assert c[fx] != MR.as_list(K.render(CASES[fx][1], 0)), fx
    # Strength 50 is halfway between the drawing and the full-strength Fast frame.
    full = MR.as_list(render(fast(K.cleaner(16)), 0))
    flat = np.array(MR.as_list(K.drawn()))
    assert np.allclose(np.array(c["FX-KCLEAN-022"]), (flat + np.array(full)) / 2, atol=1e-12)
    for name, px in c.items():
        for q in px:
            assert -1e-12 <= q[3] <= 1 + 1e-12 and all(-1e-12 <= v <= q[3] + 1e-12
                                                      for v in q[:3]), (name, q)
    print("checked")


if __name__ == "__main__":
    main()
