"""Color Grade, worked a fourth way: Sharpen and a squarer vignette (D-637).

The fourth unit of `core.color_grade` (D-397, D-398, D-571), Lumetri Color's panel under our
own name. This unit adds the two things EFFECTS.md row 121 still listed as missing that have a
rule we can define: the Creative section's Sharpen, and the vignette's Roundness below 0, the
squarer vignette. Adobe publishes no maths and no range for either, so both rules are ours,
built from rules this program already has; document 14 records them as a deviation. Nothing is
ported. Lumetri's curve and wheel dragging and the key's eyedropper stay gaps (the owner's
choice, 2026-10-13).

The new setting, with where it starts (keyable):

    sharpen                        -100 to 100, 0

and `vignette_roundness`, which D-397 took from 0 to 100, now runs -100 to 100.

The rule. D-571's steps (tools/color_grade_3_reference.py) in its order, with one step more:

5a. Sharpen, when it is not 0, after the faded film and before the vibrance, as Lumetri's
    Creative section has it: the Sharpen effect's rule (tools/sharpen_reference.py) over the
    whole frame with amount `sharpen` and a blur of radius 1 pixel: e + s / 100 (e - eb), eb
    the encoded colour of the frame blurred over its covering, held inside 0 to 1. Below 0 it
    softens: at -100 each colour is its blur. The radius is fixed at 1 pixel of the layer as
    drawn (the draft keeps 1 pixel of the draft picture).

The vignette (step 7 of D-397) with roundness m below 0 keeps roundness 0's half-axes and
measures the distance by the p-norm, p = 2 / (1 + m): see tools/color_grade_reference.py.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, where the build works on its single-precision buffers.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py). The projects go into
`Fixtures/color_grade_4`, the expected frames into
`Fixtures/color_grade_4/expected_color_grade_4.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_grade_4_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import color_grade_reference as G  # noqa: E402
import color_grade_2_reference as C2  # noqa: E402
import color_grade_3_reference as C3  # noqa: E402
import sharpen_reference as SR  # noqa: E402


W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_grade_4"
SETTINGS = dict(C3.SETTINGS)
SETTINGS["sharpen"] = (-100, 100, 0)
LISTS, WORDS = C3.LISTS, C3.WORDS
NEW_KEYS = ["sharpen"]
clamp = C2.clamp


# --- the rule -------------------------------------------------------------------------------

def run(ws, px, frame_steps):
    """D-571's loop: each step on the encoded colours, back to linear at the covering."""
    for step in frame_steps:
        es = [None if p[3] == 0 else [S.linear_to_srgb(clamp(w[c] / w[3])) for c in range(3)]
              for p, w in zip(px, ws)]
        ws = [w if e is None else [srgb_to_linear(clamp(u)) * w[3] for u in e] + [w[3]]
              for w, e in zip(ws, step(es))]
    return ws


def color_grade(px, v, look, lists, words):
    early = G.steps(dict(v, vibrance=0, creative_saturation=100, shadow_tint_amount=0,
                         highlight_tint_amount=0), look)
    ws = run([R.working(p) for p in px], px,
             [lambda es, f=f: [None if e is None else f(e) for e in es] for f in early])
    if v["sharpen"] != 0:
        ws = SR.sharpen(ws, v["sharpen"], 1.0, (W, H))
    late = C3.steps(dict(v, temperature=0, tint=0, exposure=0, contrast=0, highlights=0,
                         shadows=0, whites=0, blacks=0, saturation=100, faded_film=0),
                    False, lists, words)
    ws = run(ws, px, late)
    if v["vignette_amount"] != 0 and words["key_view"] != "mask":
        for i, p in enumerate(px):
            if p[3] == 0:
                continue
            o, g = G.vignette(i % W, i // W, v)
            w = ws[i]
            a = w[3]
            ws[i] = [(w[c] / a + o * (g - w[c] / a)) * a for c in range(3)] + [a]
    return ws


# --- the cases ------------------------------------------------------------------------------

def case(look="", shift=0, old=False, **given):
    for k in given:
        assert k in SETTINGS or k in LISTS or k in WORDS, k
    return {"look": look, "shift": shift, "old": old, "set": given}


def values(c, frame_no):
    return {k: min(high, max(low, value_at(c["set"].get(k, start), frame_no)))
            for k, (low, high, start) in SETTINGS.items()}


def render(c, frame_no):
    lists = {k: c["set"].get(k, d) for k, d in LISTS.items()}
    words = {k: c["set"].get(k, d) for k, d in WORDS.items()}
    return R.frame(color_grade(B.pixels(), values(c, frame_no), c["look"] == "asset-look",
                               lists, words), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {"look": c["look"]}
    for k, (_, _, start) in SETTINGS.items():
        if not (c["old"] and k in NEW_KEYS):
            params[k] = setting_json(c["set"].get(k, start))
    for k, d in list(LISTS.items()) + list(WORDS.items()):
        params[k] = c["set"].get(k, d)
    p = B.project_json(fx, "core.color_grade", params, c["shift"])
    p["assets"].append({"id": "asset-look", "kind": "lut", "name": "look",
                        "path": "luts/look.cube"})
    return p


SQUARE = dict(vignette_amount=-3, vignette_roundness=-100, vignette_feather=20)

CASES = {
    "FX-GRADE4-001": ("Every setting as added, sharpen written at 0: the drawing exactly as it "
                      "is.", case(), [0]),
    "FX-GRADE4-002": ("Sharpen 100: each colour pushed away from its 1-pixel blur, so the edges "
                      "between the colour columns harder; flat runs untouched.",
                      case(sharpen=100), [0]),
    "FX-GRADE4-003": ("Sharpen 50: half FX-GRADE4-002's push.", case(sharpen=50), [0]),
    "FX-GRADE4-004": ("Sharpen -100: each colour its 1-pixel blur, the drawing softened.",
                      case(sharpen=-100), [0]),
    "FX-GRADE4-005": ("Sharpen -40: four tenths of the way to the blur.", case(sharpen=-40),
                      [0]),
    "FX-GRADE4-006": ("Sharpen keyed from -100 at frame 0 to 100 at frame 4, linear: the blur "
                      "at frame 0, the drawing at frame 2, FX-GRADE4-002 at frame 4.",
                      case(sharpen=keyed((0, -100), (4, 100))), [0, 2, 4]),
    "FX-GRADE4-007": ("Faded film 40, sharpen 80, vibrance 50: the sharpen works on the faded "
                      "colours and the vibrance on the sharpened ones, in that order.",
                      case(faded_film=40, sharpen=80, vibrance=50), [0]),
    "FX-GRADE4-008": ("Sharpen 60 moved three pixels right: the same, moved.",
                      case(shift=3, sharpen=60), [0]),
    "FX-GRADE4-009": ("A dark vignette (amount -3, feather 20) at roundness -100: a rectangle "
                      "the layer's shape, so the corners darken no more than the edges' "
                      "middles.", case(**SQUARE), [0]),
    "FX-GRADE4-010": ("The same at roundness -50: between the rectangle and roundness 0's "
                      "ellipse, the corners a little darker than the edges' middles.",
                      case(**dict(SQUARE, vignette_roundness=-50)), [0]),
    "FX-GRADE4-011": ("A light vignette (amount 2, midpoint 20, feather 10) at roundness -20: "
                      "just squarer than roundness 0.",
                      case(vignette_amount=2, vignette_midpoint=20, vignette_feather=10,
                           vignette_roundness=-20), [0]),
    "FX-GRADE4-012": ("FX-GRADE4-009's vignette with roundness keyed from -100 at frame 0 to "
                      "100 at frame 4, linear: the rectangle, roundness 0's ellipse, a circle.",
                      case(**dict(SQUARE, vignette_roundness=keyed((0, -100), (4, 100)))),
                      [0, 2, 4]),
    "FX-GRADE4-013": ("A whole grade with sharpen 30 and roundness -60, every section in its "
                      "order.",
                      case("asset-look", curve_master=C2.S_CURVE, sharpen=30,
                           vignette_roundness=-60, hue_hue=[[30, -10], [200, 20]],
                           shadow_wheel_hue=200, shadow_wheel_amount=30,
                           **G.EVERYTHING), [0]),
    "FX-GRADE4-014": ("A D-571 file without sharpen, a strong vignette: D-571's own picture.",
                      case(old=True, vignette_amount=-3, vignette_roundness=40), [0]),
}

INVALID = {
    "FX-GRADE4-015": ("Sharpen 101, above 100.", case(sharpen=101)),
    "FX-GRADE4-016": ("Sharpen -101, below -100.", case(sharpen=-101)),
}


def main():
    (OUT / "luts").mkdir(parents=True, exist_ok=True)
    (OUT / "luts" / "look.cube").write_bytes(G.look_text().encode("utf-8"))
    expected = B.write_cases(OUT, "color_grade_4", CASES, INVALID, render, plain, file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g, e=1e-9: all(near(p, q, e) for p, q in zip(f, g))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    assert c["FX-GRADE4-001"]["0"] == drawn
    for k in ("002", "003", "004", "005", "009", "010", "011"):
        assert not like(c[f"FX-GRADE4-{k}"]["0"], drawn), k
    # sharpen 0 is D-571 exactly, whatever else is set
    for given in (dict(faded_film=40, vibrance=50), dict(G.EVERYTHING, curve_master=C2.S_CURVE),
                  dict(vignette_amount=-3, vignette_roundness=40)):
        assert like(render(case(**given), 0), C3.render(C3.case(**given), 0))
    assert like(c["FX-GRADE4-014"]["0"], C3.render(C3.case(vignette_amount=-3,
                                                            vignette_roundness=40), 0))
    keyed_ = c["FX-GRADE4-006"]
    assert like(keyed_["0"], c["FX-GRADE4-004"]["0"]) and keyed_["2"] == drawn
    assert like(keyed_["4"], c["FX-GRADE4-002"]["0"])
    assert not like(c["FX-GRADE4-007"]["0"], render(case(faded_film=40, vibrance=50), 0))
    moved, still = c["FX-GRADE4-008"]["0"], render(case(sharpen=60), 0)
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == still[at(0, y):at(W - 3, y)]
    # roundness -100: the share depends on the larger of |dx| and |dy| only
    base = values(case(**SQUARE), 0)

    def share(x, y, m):
        return G.vignette(x, y, dict(base, vignette_roundness=m))[0]
    assert abs(share(0, 0, -100) - share(0, 4, -100)) < 1e-12  # corner as dark as left middle
    assert share(0, 0, -50) > share(0, 4, -50)
    rk = c["FX-GRADE4-012"]
    assert like(rk["0"], c["FX-GRADE4-009"]["0"])
    assert like(rk["2"], C3.render(C3.case(**dict(SQUARE, vignette_roundness=0)), 0))
    assert like(rk["4"], C3.render(C3.case(**dict(SQUARE, vignette_roundness=100)), 0))
    for fx, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert p[3] == drawn[i][3] or fx == "FX-GRADE4-008"
                assert all(-1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
