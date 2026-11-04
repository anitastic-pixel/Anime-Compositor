"""Color Grade, worked a sixth way: a switch per section and the white balance eyedropper (D-1009).

The sixth unit of `core.color_grade` (D-397, D-398, D-571, D-637, D-1007), Lumetri Color's panel
under our own name. Lumetri has a tick beside each section's name that turns the section off,
and an eyedropper (WB Selector) beside the white balance that makes a picked colour neutral.
Adobe publishes no maths for either, so the rules below are ours; document 14 records them.
Nothing is ported.

Six new words (not keyable), each "on" or "off", "on" when the effect is added and when a file
from before has none:

    basic_on        Basic Correction: temperature, tint, exposure, contrast, highlights,
                    shadows, whites, blacks, saturation
    creative_on     Creative: the look and look_intensity, faded_film, sharpen, vibrance,
                    creative_saturation, the two tints and tint_balance
    curves_on       Curves: the four RGB curves, hue_saturation and D-571's four curves
    wheels_on       Color Wheels: the three wheels and their lightness
    secondary_on    HSL Secondary: the key, its correction and wheels, key_denoise, key_blur,
                    key_invert, key_view and secondary_wheels
    vignette_on     Vignette: vignette_amount, _midpoint, _roundness, _feather

The rule, the switches. A section switched off is drawn as if each of its settings were at its
start: no look, the curves straight, Show Key "off" (so the vignette is drawn again). Its
settings are kept as written; switching it back on draws them.

The rule, the white balance eyedropper (the page only; it writes temperature and tint the way
typing them would). The picked 8-bit colour, made linear (r, g, b), each above 0:

    temperature = round(temperature + 100 log2(b / r)),  held to -100..100
    tint        = round(tint + 200 log2(g / sqrt(r b))), held to -100..100

D-397's gains are 2^(T/200), 2^(-t/200), 2^(-T/200) in linear light, so these are the change
that makes (r, g, b) grey; added to the settings the picture already shows, it is exact when the
white balance is all that has touched the colour (the tone terms are the same on each channel,
so a grey stays grey) and close otherwise. A colour with a channel at 0 changes nothing.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works on its single-precision buffers.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_grade_6_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import color_grade_2_reference as C2  # noqa: E402
import color_grade_4_reference as C4  # noqa: E402
import color_grade_5_reference as C5  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_grade_6"
SETTINGS, LISTS, WORDS = C4.SETTINGS, C4.LISTS, C4.WORDS
NAMES = list(SETTINGS)
SECTIONS = {
    "basic_on": (NAMES[0:9], [], []),
    "creative_on": (NAMES[9:18] + ["sharpen"], [], []),
    "curves_on": ([], list(LISTS), []),
    "wheels_on": (NAMES[22:31], [], []),
    "secondary_on": (NAMES[31:54], [], ["key_invert", "key_view", "secondary_wheels"]),
    "vignette_on": (NAMES[18:22], [], []),
}
assert NAMES[8] == "saturation" and NAMES[17] == "tint_balance" and NAMES[53] == "key_blur"
assert NAMES[22] == "shadow_wheel_hue" and NAMES[30] == "highlight_lightness"
assert NAMES[18] == "vignette_amount" and NAMES[21] == "vignette_feather"


def switched(v, look, lists, words, on):
    """The settings drawn: each section switched off at its starts."""
    v, lists, words = dict(v), dict(lists), dict(words)
    for section, (numbers, curves, said) in SECTIONS.items():
        if on.get(section, "on") == "off":
            v.update({k: SETTINGS[k][2] for k in numbers})
            lists.update({k: LISTS[k] for k in curves})
            words.update({k: WORDS[k] for k in said})
            look = look and section != "creative_on"
    return v, look, lists, words


def case(look="", old=False, **given):
    for k in given:
        assert k in SETTINGS or k in LISTS or k in WORDS or k in SECTIONS, k
    return {"look": look, "shift": 0, "old": old, "set": given}


def render(c, frame_no):
    lists = {k: c["set"].get(k, d) for k, d in LISTS.items()}
    words = {k: c["set"].get(k, d) for k, d in WORDS.items()}
    v, look, lists, words = switched(C4.values(c, frame_no), c["look"] == "asset-look", lists,
                                     words, c["set"])
    return C4.R.frame(C5.color_grade(B.pixels(), v, look, lists, words), 0)


def file_json(fx, c):
    plain_set = {k: x for k, x in c["set"].items() if k not in SECTIONS}
    p = C4.file_json(fx, dict(c, set=plain_set, old=False))
    params = p["compositions"][0]["layers"][0]["effects"][0]["parameters"]
    if not c["old"]:
        for s in SECTIONS:
            params[s] = c["set"].get(s, "on")
    return p


WHOLE = dict(C4.G.EVERYTHING, curve_master=C2.S_CURVE, sharpen=30, hue_hue=[[30, -10], [200, 20]],
             shadow_wheel_hue=200, shadow_wheel_amount=30, midtone_lightness=-10,
             key_hue=0, key_hue_range=20, secondary_saturation=0)
OFF = {s: "off" for s in SECTIONS}

CASES = {
    "FX-GRADE6-001": ("A whole grade, every section on and written: D-1007's own picture.",
                      case("asset-look", **WHOLE), [0]),
    "FX-GRADE6-002": ("The same with Basic Correction off: no white balance, tone or "
                      "saturation; the rest as FX-GRADE6-001.",
                      case("asset-look", basic_on="off", **WHOLE), [0]),
    "FX-GRADE6-003": ("Creative off: no look, faded film, sharpen, vibrance or tints.",
                      case("asset-look", creative_on="off", **WHOLE), [0]),
    "FX-GRADE6-004": ("Curves off: the S curve and the hue-versus-hue curve not drawn.",
                      case("asset-look", curves_on="off", **WHOLE), [0]),
    "FX-GRADE6-005": ("Color Wheels off: the shadows' blue push and the midtones' darkening "
                      "not drawn.", case("asset-look", wheels_on="off", **WHOLE), [0]),
    "FX-GRADE6-006": ("HSL Secondary off: the reds keep their colour.",
                      case("asset-look", secondary_on="off", **WHOLE), [0]),
    "FX-GRADE6-007": ("Vignette off: the corners no darker.",
                      case("asset-look", vignette_on="off", **WHOLE), [0]),
    "FX-GRADE6-008": ("Every section off: the drawing exactly as it is.",
                      case("asset-look", **dict(WHOLE, **OFF)), [0]),
    "FX-GRADE6-009": ("Show Key Color/Gray with HSL Secondary off: the view goes with the "
                      "section, and the vignette is drawn again.",
                      case(key_view="color_gray", key_hue=0, key_hue_range=20,
                           vignette_amount=-3, secondary_on="off"), [0]),
    "FX-GRADE6-010": ("A D-1007 file with no switches: every section on, FX-GRADE6-001's "
                      "picture.", case("asset-look", old=True, **WHOLE), [0]),
}

INVALID = {
    "FX-GRADE6-011": ("Curves' switch \"yes\", not \"on\" or \"off\".",
                      case(curves_on="yes")),
}


def white_balance(t, tint, hex_):
    r, g, b = (srgb_to_linear(int(hex_[i:i + 2], 16) / 255) for i in (1, 3, 5))
    if min(r, g, b) <= 0:
        return {"temperature": t, "tint": tint}
    held = lambda x: max(-100, min(100, math.floor(x + 0.5)))  # noqa: E731
    return {"temperature": held(t + 100 * math.log2(b / r)),
            "tint": held(tint + 200 * math.log2(g / math.sqrt(r * b)))}


PICKS = [
    ("A grey: nothing to correct.", 0, 0, "#808080"),
    ("A warm white (a tungsten-lit wall): cooler, temperature down.", 0, 0, "#ffe0c0"),
    ("A cold white (shade under a blue sky): warmer, temperature up.", 0, 0, "#c8d8ff"),
    ("A greenish grey (fluorescent light): tint towards magenta, up.", 0, 0, "#90a890"),
    ("A magenta grey: tint down.", 0, 0, "#a890a8"),
    ("The warm white picked again after the first pick (temperature -20, tint -6 already "
     "set, the picture showing them): the change adds to them.", -20, -6, "#f8f0e8"),
    ("A strong orange: held to -100.", 0, 0, "#ff6010"),
    ("A colour with no blue: nothing changes.", 10, 5, "#ff8000"),
]


def main():
    (OUT / "luts").mkdir(parents=True, exist_ok=True)
    (OUT / "luts" / "look.cube").write_bytes(C4.G.look_text().encode("utf-8"))
    expected = B.write_cases(OUT, "color_grade_6", CASES, INVALID, render, B.plain, file_json)
    picks = {"rule": "D-1009", "cases": [
        {"says": s, "picked": hx, "from": {"temperature": t, "tint": tn},
         "to": white_balance(t, tn, hx)} for s, t, tn, hx in PICKS]}
    (OUT / "expected_white_balance.json").write_text(json.dumps(picks, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected, picks)


def check(expected, picks):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g, e=1e-9: all(near(p, q, e) for p, q in zip(f, g))  # noqa: E731
    drawn = B.plain(case())
    whole = C5.render(C5.case("asset-look", **WHOLE), 0)
    assert like(c["FX-GRADE6-001"], whole) and like(c["FX-GRADE6-010"], whole)
    assert like(c["FX-GRADE6-008"], drawn)
    for n, section in enumerate(SECTIONS, 2):
        fx = f"FX-GRADE6-{n:03}"
        numbers, curves, said = SECTIONS[section]
        reset = {k: x for k, x in WHOLE.items() if k not in numbers + curves + said}
        assert like(c[fx], C5.render(C5.case("" if section == "creative_on" else "asset-look",
                                             **reset), 0)), fx
        assert not like(c[fx], whole), fx
    assert like(c["FX-GRADE6-009"], C5.render(C5.case(vignette_amount=-3), 0))
    # the eyedropper: the change makes the picked colour grey under D-397's gains
    for p in picks["cases"]:
        t, tn = p["to"]["temperature"], p["to"]["tint"]
        dt = t - p["from"]["temperature"]
        dn = tn - p["from"]["tint"]
        lin = [srgb_to_linear(int(p["picked"][i:i + 2], 16) / 255) for i in (1, 3, 5)]
        if abs(t) < 100 and abs(tn) < 100 and min(lin) > 0:
            gains = [2 ** (dt / 200), 2 ** (-dn / 200), 2 ** (-dt / 200)]
            out = [x * k for x, k in zip(lin, gains)]
            assert max(out) / min(out) < 1.01, p["says"]
    to = [p["to"] for p in picks["cases"]]
    assert to[0] == {"temperature": 0, "tint": 0}
    assert to[1]["temperature"] < 0 < to[2]["temperature"] and to[3]["tint"] > 0 > to[4]["tint"]
    assert to[6]["temperature"] == -100 and to[7] == {"temperature": 10, "tint": 5}
    print("checked")


if __name__ == "__main__":
    main()
