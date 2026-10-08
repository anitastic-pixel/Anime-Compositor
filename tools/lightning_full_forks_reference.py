"""D-339's full-width long forks for Lightning Bolt, worked a second way.

From P-26's tutorial 2. D-338's long forks start at half the main bolt's weight where they leave
it, as D-324's forks do; the tutorial's strands are nearly as thick as the main bolt. `forks`
gains a third word, "full": D-338's long forks exactly, the same ways and lengths, but each
starting at the main bolt's full weight wM where it leaves it, and ending at wM k_d. Breaking's
forks already start at wM, so for it "full" is "long". "short" and "long" are unchanged, and
`tools/lightning_forks_reference.py`'s FX-LFORK cases keep their values.

**This file never runs the build's code path.** It works in double precision on lists, through
`tools/lightning_forks_reference.py`'s `bolt`.

The projects go into `Fixtures/lightning_full_forks`, the expected frames into
`Fixtures/lightning_full_forks/expected_lightning_full_forks.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/lightning_full_forks_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import lightning_extras_reference as LX  # noqa: E402
import lightning_forks_reference as LF  # noqa: E402
import smooth_reference as S  # noqa: E402

W, H = LF.W, LF.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "lightning_full_forks"

# FX-LFORK-001's bolt, two pixels wide so the forks' weight shows.
DOWN = {**LF.DOWN, "width": 2}

CASES = {
    "FX-LFULL-001": ("FX-LFORK-001's bolt at width 2 with forks Full: the same seven long forks "
                     "running to the bottom edge, each starting as wide as the main bolt where it "
                     "leaves it, where Long starts them at half.",
                     LF.case(forks="full", **DOWN), [0, 1]),
    "FX-LFULL-002": ("FX-LFULL-001 with forks Long, for comparison: the same strands, half as "
                     "wide.", LF.case(forks="long", **DOWN), [0]),
    "FX-LFULL-003": ("FX-LFORK-004 with forks Full: over the ground drawing, Alpha Obstacle 50, "
                     "width 1, the strands stop where they reach the ground, and rows 8 and 9 are "
                     "untouched.",
                     LF.case(forks="full", drawing="ground", obstacle=50, **LF.DOWN), [0, 1]),
    "FX-LFULL-004": ("FX-BOLT-001's settings with forks Full: the same main bolt as FX-BOLT-001, "
                     "the first forks long and as wide as the main bolt.",
                     LF.case(forks="full"), [0, 2]),
    "FX-LFULL-005": ("FX-LFULL-001 as Breaking, whose forks already start at the main bolt's "
                     "weight: Full draws exactly what Long does.",
                     LF.case(forks="full", kind="breaking", **DOWN), [0]),
}

INVALID = {
    "FX-LFULL-006": ("Forks \"Full\": the word is exact, so a capital is not it.",
                     LF.case(forks="Full")),
}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(LF.project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in LX.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected = {"tolerance": LF.TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {str(f): LF.render(c, f) for f in frames}}
        print(f"{fx}: {says[:60]}")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = LF.render(LF.case(opacity=0), 0)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_lightning_full_forks.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                            encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    full_c, long_c = CASES["FX-LFULL-001"][1], CASES["FX-LFULL-002"][1]
    for f in (0, 1):
        bf, bl = [], []
        full = LF.segments(full_c, f, born=bf)
        long_ = LF.segments(long_c, f, born=bl)
        # The same strands in the same places; only weights differ.
        assert [s[:2] for s in full] == [s[:2] for s in long_]
        fl = [s for s in bf if s[3] > 0]
        ll = [s for s in bl if s[3] > 0]
        assert len(fl) == 7 and [s[:2] for s in fl] == [s[:2] for s in ll]
        main = {s[0]: s[2] for s in full if s[4] == 0}
        for P, Q, wp, wq, *_ in fl:
            assert main[P] == wp and wq == wp
        for a, b in zip(fl, ll):
            assert abs(a[2] - 2 * b[2]) < 1e-12
        # The main bolt and the short forks are untouched.
        assert [s for s in full if s[4] == 0] == [s for s in long_ if s[4] == 0]
    assert c["FX-LFULL-001"]["0"] != c["FX-LFULL-002"]["0"]
    lit = lambda px: sum(p[3] for p in px)  # noqa: E731
    assert lit(c["FX-LFULL-001"]["0"]) > lit(c["FX-LFULL-002"]["0"])
    # Long at width 2 is D-338's rule: FX-LFORK-001's forks, the width doubled.
    assert LF.segments(long_c, 0) == LF.segments(LF.CASES["FX-LFORK-001"][1], 0)
    # 003: every strand stops at the ground.
    ground = LF.render(LF.case(drawing="ground", opacity=0), 0)
    for px in c["FX-LFULL-003"].values():
        assert all(px[at(x, y)] == ground[at(x, y)] for x in range(W) for y in (8, 9))
        assert sum(px[at(x, 7)] != ground[at(x, 7)] for x in range(W)) >= 3
    assert c["FX-LFULL-003"]["0"] != LF.render(LF.CASES["FX-LFORK-004"][1], 0)
    # 004: FX-BOLT-001's main bolt.
    four = LF.segments(CASES["FX-LFULL-004"][1], 0)
    six = LF.segments(LF.CASES["FX-LFORK-006"][1], 0)
    assert [s for s in four if s[4] == 0] == [s for s in six if s[4] == 0]
    assert c["FX-LFULL-004"]["0"] != LF.render(LF.CASES["FX-LFORK-006"][1], 0)
    # 005: for Breaking, Full is Long.
    assert c["FX-LFULL-005"]["0"] == LF.render(LF.case(forks="long", kind="breaking", **DOWN), 0)
    print("checked")


if __name__ == "__main__":
    main()
