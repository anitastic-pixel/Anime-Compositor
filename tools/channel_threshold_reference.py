"""Channel Threshold, worked a second way.

D-468 adds CycoreFX's CC Threshold as `core.channel_threshold`, a second name over Threshold's
engine (D-138), as Tritone is over Gradient Map (D-402), Levels (Individual Controls) over
Levels (D-383) and Split 2 over Split (D-394). Threshold's own file and fixtures do not change.
The controls are CC Threshold's: Threshold, Channel (Luminance, RGB, Saturation, Alpha), Invert
and Blend With Original.

    threshold            0 to 255, keyable, 128 when added (Threshold's start; ours)
    channel              "luminance", "rgb", "saturation" or "alpha", "luminance" when added
    invert               "off" or "on", "off" when added
    blend_with_original  0 to 100, keyable, 0 when added

The CycoreFX HD manual (1.8.1, page 124) says what each channel does: Luminance turns the
picture black and white by its luminance; RGB thresholds the red, green and blue separately;
Saturation turns pixels of saturation at or above the threshold white and the rest black; Alpha
thresholds the covering and leaves the colours. Invert "inverts the values of the channel
currently operating on"; Blend w. Original 100 is the layer as it was and 50 an even blend. It
does not publish how luminance or saturation is read, how the blend is mixed, or the defaults;
those below are ours and are logged as such.

The rule. At a pixel with covering a > 0, e its encoded straight colour (Threshold's), b its
straight linear colour, and o = 1 - blend / 100:

    decide(v) = 1 if 255 v + 1e-4 >= threshold else 0, flipped by Invert
    luminance   M = (d, d, d), d = decide(Y(e)), Y Threshold's encoded luma
    rgb         M = (decide(e.r), decide(e.g), decide(e.b))
    saturation  M = (d, d, d), d = decide(S(e)), S the HSL saturation (D-113's to_hsl)
        out = ((b + o (M - b)) a, a)
    alpha       t = decide(a), a' = a + o (t - a); out = (b a', a')

A pixel that does not show stays as it is, so the layer never grows. Luminance with Invert off
and Blend 0 is exactly Threshold at the same level; `check` asserts it against
tools/threshold_reference.py's own numbers.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is Threshold's drawing: bands of colour from black to white with a soft skin and a
soft line. The projects go into `Fixtures/channel_threshold`, the expected frames into
`Fixtures/channel_threshold/expected_channel_threshold.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/channel_threshold_reference.py
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

W, H = T.W, T.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "channel_threshold"
TOLERANCE = 2e-5
RANGES = {"threshold": (0, 255), "blend_with_original": (0, 100)}
CHANNELS = ("luminance", "rgb", "saturation", "alpha")


# --- the rule -------------------------------------------------------------------------------

def saturation(e):
    """The HSL saturation of an encoded colour, 0 to 1."""
    mx, mn = max(e), min(e)
    if mx == mn:
        return 0.0
    l = (mx + mn) / 2
    return (mx - mn) / (1 - abs(2 * l - 1))


def decide(v, level, invert):
    d = 1.0 if 255 * v + T.NUDGE >= level else 0.0
    return 1.0 - d if invert else d


def channel_threshold(pixels, levels, channel, inverts, blend):
    """`levels` and `inverts` are the red, green and blue ones (one value for all but RGB)."""
    o = 1 - blend / 100
    out = [R.working(p) for p in pixels]
    for w in out:
        a = w[3]
        if a <= 0:
            continue
        e = T.encoded(w)
        b = [min(1.0, max(0.0, v / a)) for v in w[:3]]
        if channel == "alpha":
            t = decide(a, levels[0], inverts[0])
            n = a + o * (t - a)
            w[:] = [c * n for c in b] + [n]
            continue
        if channel == "rgb":
            m = [decide(e[c], levels[c], inverts[c]) for c in range(3)]
        else:
            v = T.luma255(e) / 255 if channel == "luminance" else saturation(e)
            m = [decide(v, levels[0], inverts[0])] * 3
        w[:3] = [(b[c] + o * (m[c] - b[c])) * a for c in range(3)]
    return out


# --- the cases ------------------------------------------------------------------------------

def case(threshold=128, channel="luminance", invert="off", blend_with_original=0, shift=0):
    return {"drawing": "tones", "threshold": threshold, "channel": channel, "invert": invert,
            "blend_with_original": blend_with_original, "shift": shift}


def held(c, name, frame_no):
    lo, hi = RANGES[name]
    return min(hi, max(lo, value_at(c[name], frame_no)))


def render(c, frame_no):
    level = held(c, "threshold", frame_no)
    inv = c["invert"] == "on"
    pixels = [p for row in T.DRAWINGS[c["drawing"]] for p in row]
    return R.frame(channel_threshold(pixels, [level] * 3, c["channel"], [inv] * 3,
                                     held(c, "blend_with_original", frame_no)), c["shift"])


plain = T.plain


def project_json(fx, c):
    p = T.project_json(fx, {"drawing": c["drawing"], "level": 128, "shift": c["shift"]})
    p["compositions"][0]["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.channel_threshold", "enabled": True,
        "parameters": {"threshold": setting_json(c["threshold"]), "channel": c["channel"],
                       "invert": c["invert"],
                       "blend_with_original": setting_json(c["blend_with_original"])}}]
    return p


CASES = {
    "FX-CHTHRESH-001": ("As added: Threshold 128, Luminance, Invert off, Blend 0: exactly "
                        "Threshold at 128 (FX-THRESH-001), grey 128 and everything lighter "
                        "white, the rest black, each soft edge at its own covering.",
                        case(), [0]),
    "FX-CHTHRESH-002": ("Threshold 50: exactly Threshold at 50 (FX-THRESH-005), red white and "
                        "blue black.", case(threshold=50), [0]),
    "FX-CHTHRESH-003": ("Invert on: FX-CHTHRESH-001 turned over, what was white black and what "
                        "was black white.", case(invert="on"), [0]),
    "FX-CHTHRESH-004": ("RGB at 128: each channel on its own, so red stays pure red, green pure "
                        "green, blue pure blue, both skins white and grey 127 black.",
                        case(channel="rgb"), [0]),
    "FX-CHTHRESH-005": ("RGB at 200: the lit skin, red and green above 200 and blue below, "
                        "turns yellow; the shadow skin keeps only its red; the light grey "
                        "turns black.", case(channel="rgb", threshold=200), [0]),
    "FX-CHTHRESH-006": ("Saturation at 128: the pure colours and both skins white; the greys, "
                        "black, white and the dull line black.",
                        case(channel="saturation"), [0]),
    "FX-CHTHRESH-007": ("Saturation at 150: the shadow skin, saturation 136, turns black while "
                        "the lit skin, 193, stays white.",
                        case(channel="saturation", threshold=150), [0]),
    "FX-CHTHRESH-008": ("Alpha at 129: the two soft edges, covering 128, turn clear; every "
                        "opaque pixel keeps its colour and covering.",
                        case(channel="alpha", threshold=129), [0]),
    "FX-CHTHRESH-009": ("Alpha at 100: the soft edges turn opaque in their own colours; the "
                        "empty pixels stay empty.", case(channel="alpha", threshold=100), [0]),
    "FX-CHTHRESH-010": ("Alpha at 129, Invert on: the soft edges turn opaque and every opaque "
                        "pixel clear.", case(channel="alpha", threshold=129, invert="on"), [0]),
    "FX-CHTHRESH-011": ("Blend With Original 50: every pixel halfway, in linear light, between "
                        "the drawing and FX-CHTHRESH-001.",
                        case(blend_with_original=50), [0]),
    "FX-CHTHRESH-012": ("Blend With Original 100: the drawing exactly as it was.",
                        case(blend_with_original=100), [0]),
    "FX-CHTHRESH-013": ("RGB, Invert on, Blend 25: each channel turned over, 75 per cent of the "
                        "way from the drawing.",
                        case(channel="rgb", invert="on", blend_with_original=25), [0]),
    "FX-CHTHRESH-014": ("Alpha at 129, Blend 50: the soft edges at half their covering, the "
                        "rest as it was.",
                        case(channel="alpha", threshold=129, blend_with_original=50), [0]),
    "FX-CHTHRESH-015": ("Threshold keyed from 0 at frame 0 to 255 at frame 4, linear: frame 2 "
                        "is Threshold 127.5, grey 128 white and grey 127 black.",
                        case(threshold=keyed((0, 0), (4, 255))), [0, 2, 4]),
    "FX-CHTHRESH-016": ("Blend eased from 50 at frame 0 to 100 at frame 4 on a curve that "
                        "overshoots: at frame 2 it would pass 100, is held at 100, and the "
                        "frame is the drawing.",
                        case(blend_with_original=keyed((0, 50, OVERSHOOT), (4, 100))), [0, 2]),
    "FX-CHTHRESH-017": ("FX-CHTHRESH-004 moved three pixels right: the same, moved.",
                        case(channel="rgb", shift=3), [0, 3]),
}

INVALID = {
    "FX-CHTHRESH-018": ("Threshold 256, above 255.", case(threshold=256)),
    "FX-CHTHRESH-019": ("Threshold -1, below 0.", case(threshold=-1)),
    "FX-CHTHRESH-020": ("A channel written \"red\", not one of the four.", case(channel="red")),
    "FX-CHTHRESH-021": ("Invert written \"yes\", not \"on\" or \"off\".", case(invert="yes")),
    "FX-CHTHRESH-022": ("Blend With Original 101, above 100.", case(blend_with_original=101)),
    "FX-CHTHRESH-023": ("Blend With Original keyed to 150 at frame 4.",
                        case(blend_with_original=keyed((0, 0), (4, 150)))),
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
    (OUT / "expected_channel_threshold.json").write_text(json.dumps(expected, indent=1) + "\n",
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
    pixels = [p for row in T.DRAWINGS["tones"] for p in row]
    half = 128 / 255

    # Saturations the cases lean on, and no decided pixel near the cliff.
    sat = {q: 255 * saturation([v / 255 for v in q[:3]]) for q in col}
    assert abs(sat[T.SHADOW] - 136) < 1e-9
    assert 135 < sat[T.SHADOW] < 137 and 192 < sat[T.SKIN] < 194 and 40 < sat[T.LINE] < 42
    for q in (T.RED, T.GREEN, T.BLUE):
        assert sat[q] == 255
    for q in (T.BLACK, T.WHITE, T.GREY, T.DIM, T.DARK, T.LIGHT):
        assert sat[q] == 0

    # Luminance, no invert, no blend is Threshold, number for number.
    assert c["FX-CHTHRESH-001"]["0"] == T.render(T.case(), 0)
    assert c["FX-CHTHRESH-002"]["0"] == T.render(T.case(level=50), 0)
    one = c["FX-CHTHRESH-001"]["0"]
    three = c["FX-CHTHRESH-003"]["0"]
    for i in range(W * H):
        a = drawn[i][3]
        if a:
            assert near(three[i][:3], [a - v for v in one[i][:3]]), i
    four = c["FX-CHTHRESH-004"]["0"]
    assert near(px(four, T.RED), [1, 0, 0, 1]) and near(px(four, T.GREEN), [0, 1, 0, 1])
    assert near(px(four, T.BLUE), [0, 0, 1, 1]) and near(px(four, T.SKIN), [1, 1, 1, 1])
    assert near(px(four, T.SHADOW), [1, 1, 1, 1]) and near(px(four, T.DIM), [0, 0, 0, 1])
    assert near(px(four, T.GREY), [1, 1, 1, 1]) and near(px(four, T.SOFT_SKIN), [half] * 4)
    five = c["FX-CHTHRESH-005"]["0"]
    assert near(px(five, T.SKIN), [1, 1, 0, 1]) and near(px(five, T.SHADOW), [1, 0, 0, 1])
    assert near(px(five, T.LIGHT), [0, 0, 0, 1])
    six = c["FX-CHTHRESH-006"]["0"]
    for q in (T.RED, T.GREEN, T.BLUE, T.SHADOW, T.SKIN):
        assert near(px(six, q), [1, 1, 1, 1]), q
    for q in (T.BLACK, T.WHITE, T.GREY, T.LINE, T.LIGHT):
        assert near(px(six, q), [0, 0, 0, 1]), q
    seven = c["FX-CHTHRESH-007"]["0"]
    assert near(px(seven, T.SHADOW), [0, 0, 0, 1]) and near(px(seven, T.SKIN), [1, 1, 1, 1])
    eight = c["FX-CHTHRESH-008"]["0"]
    assert px(eight, T.SOFT_SKIN) == [0.0] * 4 and px(eight, T.SOFT_LINE) == [0.0] * 4
    for i in range(W * H):
        if drawn[i][3] == 1.0:
            assert near(eight[i], drawn[i])
    nine = c["FX-CHTHRESH-009"]["0"]
    assert near(px(nine, T.SOFT_SKIN), [srgb_to_linear(v / 255) for v in T.SKIN[:3]] + [1])
    assert all(nine[i] == [0.0] * 4 for i in range(W * H) if drawn[i][3] == 0)
    ten = c["FX-CHTHRESH-010"]["0"]
    assert near(px(ten, T.SOFT_LINE), [srgb_to_linear(v / 255) for v in T.LINE[:3]] + [1])
    assert all(ten[i] == [0.0] * 4 for i in range(W * H) if drawn[i][3] in (0, 1.0))
    assert like(c["FX-CHTHRESH-011"]["0"], [[(u + v) / 2 for u, v in zip(p, q)]
                                            for p, q in zip(drawn, one)])
    assert c["FX-CHTHRESH-012"]["0"] == drawn
    full = R.frame(channel_threshold(pixels, [128] * 3, "rgb", [True] * 3, 0), 0)
    assert like(c["FX-CHTHRESH-013"]["0"], [[u + 0.75 * (v - u) for u, v in zip(p, q)]
                                            for p, q in zip(drawn, full)])
    fourteen = c["FX-CHTHRESH-014"]["0"]
    assert near(px(fourteen, T.SOFT_SKIN)[3:], [half / 2])
    fifteen = c["FX-CHTHRESH-015"]
    assert fifteen["2"] == T.render(T.case(level=127.5), 0)
    sixteen = c["FX-CHTHRESH-016"]
    assert value_at(case(blend_with_original=keyed((0, 50, OVERSHOOT), (4, 100)))
                    ["blend_with_original"], 2) > 100
    assert sixteen["2"] == drawn and sixteen["0"] == c["FX-CHTHRESH-011"]["0"]
    moved = c["FX-CHTHRESH-017"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == four[at(0, y):at(W - 3, y)]

    # No decided value sits within 1e-5 of the level, and the empty pixels stay empty.
    for fx, (says, cc, frames) in CASES.items():
        base = plain(case(shift=cc["shift"]))
        for f in frames:
            level = held(cc, "threshold", f)
            for i, p in enumerate(base):
                if p[3] == 0:
                    assert c[fx][str(f)][i] == [0.0] * 4, (fx, i)
                    continue
                e = T.encoded(p)
                vs = {"luminance": [T.luma255(e) / 255], "rgb": e, "saturation": [saturation(e)],
                      "alpha": [p[3]]}[cc["channel"]]
                for v in vs:
                    assert abs(255 * v + T.NUDGE - level) >= T.CLIFF, (fx, f, i, v, level)
    print("checked")


if __name__ == "__main__":
    main()
