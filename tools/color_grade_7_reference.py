"""Color Grade, worked a seventh way: HSL Secondary's Sharpen (D-1010).

The seventh unit of `core.color_grade` (D-397, D-398, D-571, D-637, D-1007, D-1009), Lumetri
Color's panel under our own name. Lumetri's HSL Secondary has a Sharpen slider in its
correction, sharpening only what the key takes. Adobe publishes no maths, so the rule is ours,
built from D-637's Creative Sharpen; document 14 records it. Nothing is ported.

The new setting, with where it starts (keyable), in the HSL Secondary section (D-1009's
`secondary_on` switches it with the rest):

    secondary_sharpen              -100 to 100, 0

The rule. After HSL Secondary's correction (step 11 of tools/color_grade_2_reference.py, as
D-571 refines its key) and before the vignette, when secondary_sharpen s is not 0 and Show Key
is "off": the Sharpen effect's rule (tools/sharpen_reference.py) over the whole frame with
amount s and a blur of radius 1 pixel, as D-637's, mixed in by the key: w + m (sh - w) on each
premultiplied linear channel, w the frame after the correction, sh it sharpened, m the key the
correction used (inverted with key_invert, refined by key_denoise and key_blur), as Show Key
White/Black shows it. Below 0 it softens what the key takes.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, where the build works on its single-precision buffers.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_grade_7_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import color_grade_reference as G  # noqa: E402
import color_grade_2_reference as C2  # noqa: E402
import color_grade_4_reference as C4  # noqa: E402
import color_grade_5_reference as C5  # noqa: E402
import color_grade_6_reference as C6  # noqa: E402
import sharpen_reference as SR  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_grade_7"
LISTS, WORDS = C4.LISTS, C4.WORDS
NEW = ("secondary_sharpen", (-100, 100, 0))
clamp = C2.clamp


def color_grade(px, v, look, lists, words):
    s = v["secondary_sharpen"]
    v = {k: x for k, x in v.items() if k != NEW[0]}
    if s == 0 or words["key_view"] != "off":
        return C5.color_grade(px, v, look, lists, words)
    ws = C4.color_grade(px, dict(v, vignette_amount=0), look, lists, words)
    grey = C4.color_grade(px, v, look, lists, dict(words, key_view="mask"))
    sh = SR.sharpen(ws, s, 1.0, (W, H))
    out = []
    for i, (p, w, q, k) in enumerate(zip(px, ws, sh, grey)):
        if p[3] == 0:
            out.append(w)
            continue
        m = S.linear_to_srgb(clamp(k[0] / k[3]))
        w = [a + m * (b - a) for a, b in zip(w, q)]
        if v["vignette_amount"] != 0:
            o, g = G.vignette(i % W, i // W, v)
            a = w[3]
            w = [(w[c] / a + o * (g - w[c] / a)) * a for c in range(3)] + [a]
        out.append(w)
    return out


def case(look="", old=False, **given):
    return C6.case(look, old, **{k: x for k, x in given.items() if k != NEW[0]}) | {
        "sharpen2": given.get(NEW[0], 0), "old2": old}


def values(c, frame_no):
    low, high, _ = NEW[1]
    v = C4.values(c, frame_no)
    v[NEW[0]] = min(high, max(low, value_at(c["sharpen2"], frame_no)))
    return v


def render(c, frame_no):
    lists = {k: c["set"].get(k, d) for k, d in LISTS.items()}
    words = {k: c["set"].get(k, d) for k, d in WORDS.items()}
    v, look, lists, words = C6.switched(values(c, frame_no), c["look"] == "asset-look", lists,
                                        words, c["set"])
    if c["set"].get("secondary_on", "on") == "off":
        v[NEW[0]] = 0
    return C4.R.frame(color_grade(B.pixels(), v, look, lists, words), 0)


def file_json(fx, c):
    p = C6.file_json(fx, dict(c, old=True))
    params = p["compositions"][0]["layers"][0]["effects"][0]["parameters"]
    if not c["old2"]:
        params[NEW[0]] = setting_json(c["sharpen2"])
    for s in C6.SECTIONS:
        if s in c["set"]:
            params[s] = c["set"][s]
    return p


REDS = dict(key_hue=0, key_hue_range=20)
CASES = {
    "FX-GRADE7-001": ("Secondary sharpen 100 on a key of the reds: the reds' edges harder, the "
                      "rest as drawn.", case(secondary_sharpen=100, **REDS), [0]),
    "FX-GRADE7-002": ("Secondary sharpen -100 on the reds: the reds softened.",
                      case(secondary_sharpen=-100, **REDS), [0]),
    "FX-GRADE7-003": ("Secondary sharpen 100 with the key as added (every colour): the "
                      "Creative sharpen 100's picture, FX-GRADE4-002.",
                      case(secondary_sharpen=100), [0]),
    "FX-GRADE7-004": ("The reds' key inverted: every colour but the reds sharpened.",
                      case(secondary_sharpen=100, key_invert="on", **REDS), [0]),
    "FX-GRADE7-005": ("With a correction (secondary saturation 0) and a vignette: the reds "
                      "grey and sharpened, then the corners darkened.",
                      case(secondary_sharpen=100, secondary_saturation=0, vignette_amount=-3,
                           **REDS), [0]),
    "FX-GRADE7-006": ("Keyed from -100 at frame 0 to 100 at frame 4.",
                      case(secondary_sharpen=keyed((0, -100), (4, 100)), **REDS), [0, 2, 4]),
    "FX-GRADE7-007": ("HSL Secondary switched off: the drawing as it is.",
                      case(secondary_sharpen=100, secondary_on="off", **REDS), [0]),
    "FX-GRADE7-008": ("Show Key White/Black: the key, unchanged by the sharpen.",
                      case(secondary_sharpen=100, key_view="mask", **REDS), [0]),
    "FX-GRADE7-009": ("A D-1009 file without the setting, the reds made grey: D-1009's own "
                      "picture.", case(old=True, secondary_saturation=0, **REDS), [0]),
}
INVALID = {
    "FX-GRADE7-010": ("Secondary sharpen 101, above 100.", case(secondary_sharpen=101)),
}


def main():
    (OUT / "luts").mkdir(parents=True, exist_ok=True)
    (OUT / "luts" / "look.cube").write_bytes(C4.G.look_text().encode("utf-8"))
    expected = B.write_cases(OUT, "color_grade_7", CASES, INVALID, render, B.plain, file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    near = lambda p, q: all(abs(u - v) < 1e-9 for u, v in zip(p, q))  # noqa: E731
    drawn = B.plain(case())
    mask = C5.render(C5.case(key_view="mask", **REDS), 0)
    kept = [i for i, k in enumerate(mask) if k[0] == 0]
    keyed_px = [i for i, k in enumerate(mask) if k[3] > 0 and abs(k[0] - k[3]) < 1e-12]
    assert kept and keyed_px
    for fx in ("FX-GRADE7-001", "FX-GRADE7-002"):
        assert all(near(c[fx][i], drawn[i]) for i in kept), fx
        assert any(not near(c[fx][i], drawn[i]) for i in keyed_px), fx
    assert all(near(p, q) for p, q in zip(c["FX-GRADE7-003"], C4.render(C4.case(sharpen=100), 0)))
    assert all(near(c["FX-GRADE7-004"][i], drawn[i]) for i in keyed_px)
    assert any(not near(c["FX-GRADE7-004"][i], drawn[i]) for i in kept)
    assert all(near(p, q) for p, q in zip(c["FX-GRADE7-007"], drawn))
    assert all(near(p, q) for p, q in zip(c["FX-GRADE7-008"], mask))
    assert all(near(p, q) for p, q in zip(
        c["FX-GRADE7-009"], C5.render(C5.case(secondary_saturation=0, **REDS), 0)))
    f6 = expected["cases"]["FX-GRADE7-006"]["frames"]
    assert all(near(p, q) for p, q in zip(f6["0"], c["FX-GRADE7-002"]))
    assert all(near(p, q) for p, q in zip(f6["4"], c["FX-GRADE7-001"]))
    assert all(near(p, q) for p, q in zip(f6["2"], drawn))
    print("checked")


if __name__ == "__main__":
    main()
