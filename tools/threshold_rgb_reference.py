"""Threshold RGB, worked a second way.

D-469 adds CycoreFX's CC Threshold RGB as `core.threshold_rgb`, a second name over Threshold's
engine (D-138) beside Channel Threshold (D-468), as Tritone is over Gradient Map (D-402) and
Split 2 over Split (D-394). Threshold's own file and fixtures do not change. The controls are CC
Threshold RGB's: a threshold and an Invert for each of red, green and blue, and Blend With
Original.

    red_threshold, green_threshold, blue_threshold   0 to 255, keyable, 128 when added (ours)
    invert_red, invert_green, invert_blue            "off" or "on", "off" when added
    blend_with_original                              0 to 100, keyable, 0 when added

The CycoreFX HD manual (1.8.1, page 125): each channel brighter than its threshold goes to its
maximum and darker to black; each Invert "inverts the values of its respective channel"; Blend
w. Original 100 is the layer as it was. The rule is Channel Threshold's RGB channel
(tools/channel_threshold_reference.py) with a level and an Invert per channel; `check` asserts
the two agree number for number where their settings meet.

    decide_c = 1 if 255 e_c + 1e-4 >= threshold_c else 0, flipped by that channel's Invert
    out = ((b + o (M - b)) a, a),  M = (decide_r, decide_g, decide_b),  o = 1 - blend / 100

at a pixel with covering a > 0, e its encoded straight colour and b its straight linear colour.
A pixel that does not show stays as it is; the covering never changes.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is Threshold's drawing. The projects go into `Fixtures/threshold_rgb`, the expected
frames into `Fixtures/threshold_rgb/expected_threshold_rgb.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/threshold_rgb_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import threshold_reference as T  # noqa: E402
import channel_threshold_reference as C  # noqa: E402

W, H = T.W, T.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "threshold_rgb"
TOLERANCE = 2e-5
LEVELS = ("red_threshold", "green_threshold", "blue_threshold")
INVERTS = ("invert_red", "invert_green", "invert_blue")
RANGES = {**{k: (0, 255) for k in LEVELS}, "blend_with_original": (0, 100)}


def case(red_threshold=128, green_threshold=128, blue_threshold=128, invert_red="off",
         invert_green="off", invert_blue="off", blend_with_original=0, shift=0):
    return {"drawing": "tones", "red_threshold": red_threshold, "green_threshold": green_threshold,
            "blue_threshold": blue_threshold, "invert_red": invert_red,
            "invert_green": invert_green, "invert_blue": invert_blue,
            "blend_with_original": blend_with_original, "shift": shift}


def held(c, name, frame_no):
    lo, hi = RANGES[name]
    return min(hi, max(lo, value_at(c[name], frame_no)))


def threshold_rgb(pixels, levels, inverts, blend):
    return C.channel_threshold(pixels, levels, "rgb", inverts, blend)


def render(c, frame_no):
    pixels = [p for row in T.DRAWINGS[c["drawing"]] for p in row]
    return R.frame(threshold_rgb(pixels, [held(c, k, frame_no) for k in LEVELS],
                                 [c[k] == "on" for k in INVERTS],
                                 held(c, "blend_with_original", frame_no)), c["shift"])


plain = T.plain


def project_json(fx, c):
    p = T.project_json(fx, {"drawing": c["drawing"], "level": 128, "shift": c["shift"]})
    params = {k: setting_json(c[k]) for k in LEVELS}
    params.update({k: c[k] for k in INVERTS})
    params["blend_with_original"] = setting_json(c["blend_with_original"])
    p["compositions"][0]["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.threshold_rgb", "enabled": True,
        "parameters": params}]
    return p


CASES = {
    "FX-THRESHRGB-001": ("As added: every threshold 128, no Invert, Blend 0: each channel on "
                         "its own, red stays pure red, green pure green, both skins white, grey "
                         "127 black; the same as Channel Threshold's RGB at 128 "
                         "(FX-CHTHRESH-004).", case(), [0]),
    "FX-THRESHRGB-002": ("Red 230, green 128, blue 50: the lit skin (246, 214, 190) stays "
                         "white, the shadow skin (220, 160, 140) loses its red and turns cyan, "
                         "the light grey (192) loses its red too.",
                         case(red_threshold=230, blue_threshold=50), [0]),
    "FX-THRESHRGB-003": ("Invert Green only: green turned over in every pixel, so pure green "
                         "turns black, pure red yellow and both skins magenta.",
                         case(invert_green="on"), [0]),
    "FX-THRESHRGB-004": ("All three inverted: FX-THRESHRGB-001 turned over, white black and "
                         "pure red cyan.",
                         case(invert_red="on", invert_green="on", invert_blue="on"), [0]),
    "FX-THRESHRGB-005": ("Blend With Original 50: every pixel halfway, in linear light, between "
                         "the drawing and FX-THRESHRGB-001.", case(blend_with_original=50), [0]),
    "FX-THRESHRGB-006": ("Red 0, green 255, blue 128: red full in every shown pixel, green "
                         "only where it is full (green and white), blue as at 128.",
                         case(red_threshold=0, green_threshold=255), [0]),
    "FX-THRESHRGB-007": ("Red keyed from 0 at frame 0 to 255 at frame 4, linear: frame 2 is "
                         "red 127.5.", case(red_threshold=keyed((0, 0), (4, 255))), [0, 2, 4]),
    "FX-THRESHRGB-008": ("Blend eased from 50 at frame 0 to 100 at frame 4 on a curve that "
                         "overshoots: at frame 2 it is held at 100, the drawing.",
                         case(blend_with_original=keyed((0, 50, OVERSHOOT), (4, 100))), [0, 2]),
    "FX-THRESHRGB-009": ("FX-THRESHRGB-002 moved three pixels right: the same, moved.",
                         case(red_threshold=230, blue_threshold=50, shift=3), [0, 3]),
}

INVALID = {
    "FX-THRESHRGB-010": ("Red Threshold 256, above 255.", case(red_threshold=256)),
    "FX-THRESHRGB-011": ("Green Threshold -1, below 0.", case(green_threshold=-1)),
    "FX-THRESHRGB-012": ("Blue Threshold keyed to 300 at frame 4.",
                         case(blue_threshold=keyed((0, 128), (4, 300)))),
    "FX-THRESHRGB-013": ("Invert Red written \"true\", not \"on\" or \"off\".",
                         case(invert_red="true")),
    "FX-THRESHRGB-014": ("Blend With Original -1, below 0.", case(blend_with_original=-1)),
}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in T.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_threshold_rgb.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    col = {q: x for x, q in enumerate(T.BANDS) if q != T.NONE}
    px = lambda f, q: f[at(col[q], 4)]  # noqa: E731

    one = c["FX-THRESHRGB-001"]["0"]
    assert one == C.render(C.case(channel="rgb"), 0)
    assert near(px(one, T.RED), [1, 0, 0, 1]) and near(px(one, T.SKIN), [1, 1, 1, 1])
    assert near(px(one, T.DIM), [0, 0, 0, 1])
    two = c["FX-THRESHRGB-002"]["0"]
    assert near(px(two, T.SKIN), [1, 1, 1, 1]) and near(px(two, T.SHADOW), [0, 1, 1, 1])
    assert near(px(two, T.LIGHT), [0, 1, 1, 1])
    three = c["FX-THRESHRGB-003"]["0"]
    assert near(px(three, T.GREEN), [0, 0, 0, 1]) and near(px(three, T.RED), [1, 1, 0, 1])
    assert near(px(three, T.SKIN), [1, 0, 1, 1]) and near(px(three, T.SHADOW), [1, 0, 1, 1])
    four = c["FX-THRESHRGB-004"]["0"]
    assert four == C.render(C.case(channel="rgb", invert="on"), 0)
    assert near(px(four, T.WHITE), [0, 0, 0, 1]) and near(px(four, T.RED), [0, 1, 1, 1])
    assert like(c["FX-THRESHRGB-005"]["0"], [[(u + v) / 2 for u, v in zip(p, q)]
                                             for p, q in zip(drawn, one)])
    six = c["FX-THRESHRGB-006"]["0"]
    assert near(px(six, T.BLACK), [1, 0, 0, 1]) and near(px(six, T.GREEN), [1, 1, 0, 1])
    assert near(px(six, T.WHITE), [1, 1, 1, 1]) and near(px(six, T.SKIN), [1, 0, 1, 1])
    seven = c["FX-THRESHRGB-007"]
    assert seven["2"] == render(case(red_threshold=127.5), 0)
    eight = c["FX-THRESHRGB-008"]
    assert value_at(case(blend_with_original=keyed((0, 50, OVERSHOOT), (4, 100)))
                    ["blend_with_original"], 2) > 100
    assert eight["2"] == drawn and eight["0"] == c["FX-THRESHRGB-005"]["0"]
    moved = c["FX-THRESHRGB-009"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == two[at(0, y):at(W - 3, y)]
    for fx, (says, cc, frames) in CASES.items():
        base = plain(case(shift=cc["shift"]))
        for f in frames:
            levels = [held(cc, k, f) for k in LEVELS]
            for i, p in enumerate(base):
                assert c[fx][str(f)][i][3] == p[3], (fx, i)
                if p[3]:
                    for v, level in zip(T.encoded(p), levels):
                        assert abs(255 * v + T.NUDGE - level) >= T.CLIFF, (fx, f, i, v, level)
    print("checked")


if __name__ == "__main__":
    main()
