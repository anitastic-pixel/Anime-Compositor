"""Levels with a set for each channel, worked a second way.

D-383, the owner's "Both names, one engine": After Effects' Levels has a Channel menu (RGB, Red,
Green, Blue, Alpha) and each choice has its own five settings; Levels (Individual Controls) is
the same effect with all of them shown at once. Here `core.levels` gains the menu, `channel`,
and `core.levels_individual` is the second name over the same rule. Nothing is ported; Adobe
does not publish the order its channels are applied in, so the order below is ours, the same as
Curves' (D-111). Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it.

The rule. Five sets of five settings, in the order RGB, red, green, blue, alpha. The RGB set is
D-112's `input_black`, `input_white`, `gamma`, `output_black`, `output_white`; each other set is
the same five names after `red_`, `green_`, `blue_` or `alpha_`. Inputs and outputs 0 to 255,
gamma 0.1 to 10; starts 0, 255, 1, 0, 255, which change nothing. All 25 keyable. `channel`
(`core.levels` only) is which set the controls show, and never changes the picture.

- One set on a value x on the 0 to 255 scale: from the input range to 0..1, held there (an
  input white equal to its black is a threshold, 1 at or above it), raised to 1 / gamma and laid
  on the output range, as D-112.
- At a pixel with covering a > 0, e its straight colour through the sRGB curve held inside 0 to
  1: each of red, green and blue on 255 e through its own set, then through the RGB set; a set
  at its start is skipped. The result over 255, held inside 0 to 1, back to linear at the
  pixel's covering (document 21's shared colour rule). A pixel with a = 0 keeps its colour.
- Then, when the alpha set is not at its start, the covering 255 a through the alpha set, over
  255, and the straight colour kept at the new covering, as Curves' alpha (D-302): a pixel that
  did not show and now does is black.
- Every set at its start leaves the layer exactly as it is.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, where the build works on its single-precision buffers.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py): pure colours, greys,
a skin tone, orange, three warm tones, an empty column and yellow at half covering. The projects
go into `Fixtures/levels_individual`, the expected frames into
`Fixtures/levels_individual/expected_levels_individual.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/levels_individual_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "levels_individual"
SETS = ("", "red_", "green_", "blue_", "alpha_")
FIVE = ("input_black", "input_white", "gamma", "output_black", "output_white")
PLAIN = (0, 255, 1, 0, 255)
NUMBERS = {}
for _s in SETS:
    NUMBERS |= {f"{_s}{n}": ((0.1, 10) if n == "gamma" else (0, 255)) for n in FIVE}
START = {f"{s}{n}": v for s in SETS for n, v in zip(FIVE, PLAIN)}


# --- the rule -------------------------------------------------------------------------------

def level(s, x):
    ib, iw, gamma, ob, ow = s
    if iw == ib:
        v = 1.0 if x >= ib else 0.0
    else:
        v = min(1.0, max(0.0, (x - ib) / (iw - ib)))
    return ob + v ** (1 / gamma) * (ow - ob)


def levels(px, s):
    sets = [tuple(s[f"{p}{n}"] for n in FIVE) for p in SETS]
    rgb, own, alpha = sets[0], sets[1:4], sets[4]
    out = []
    for p in px:
        if p[3] == 0 or all(t == PLAIN for t in sets[:4]):
            out.append(R.working(p))
            continue
        o = []
        for c, v in enumerate(B.encoded(p)):
            x = v * 255
            if own[c] != PLAIN:
                x = level(own[c], x)
            if rgb != PLAIN:
                x = level(rgb, x)
            o.append(x / 255)
        out.append(B.back(o, p))
    if alpha != PLAIN:
        for i, q in enumerate(out):
            a = q[3]
            to = level(alpha, a * 255) / 255
            out[i] = [(u / a * to if a > 0 else 0.0) for u in q[:3]] + [to]
    return out


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, kind="levels_individual", channel="rgb", only_rgb=False, **kw):
    c = dict(START)
    c.update(kw)
    c |= {"shift": shift, "kind": kind, "channel": channel, "only_rgb": only_rgb}
    return c


def render(c, frame_no):
    s = dict(c)
    for k, (lo, hi) in NUMBERS.items():
        s[k] = min(hi, max(lo, value_at(c[k], frame_no)))
    return R.frame(levels(B.pixels(), s), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    names = list(NUMBERS)[:5] if c["only_rgb"] else list(NUMBERS)
    params = {k: setting_json(c[k]) for k in names}
    if c["kind"] == "levels":
        params = {"channel": c["channel"]} | params
    return B.project_json(fx, f"core.{c['kind']}", params, c["shift"])


ALL = dict(input_black=10, gamma=1.2, red_input_white=230, red_output_black=20, green_gamma=0.7,
           green_output_white=240, blue_input_black=30, blue_gamma=1.5, alpha_output_white=200)

CASES = {
    "FX-LVLIC-001": ("Levels (Individual Controls) with every setting as it starts: the drawing, "
                     "untouched.", case(), [0]),
    "FX-LVLIC-002": ("Red input white 192: red brightened, every red at or above 192 full; "
                     "green and blue untouched.", case(red_input_white=192), [0]),
    "FX-LVLIC-003": ("Green gamma 2: the middle of the green channel brighter; 0 and full green "
                     "stay; red and blue untouched.", case(green_gamma=2), [0]),
    "FX-LVLIC-004": ("Blue output 64 to 192: blue squeezed into that range, so no blue below 64 "
                     "or above 192; red and green untouched.",
                     case(blue_output_black=64, blue_output_white=192), [0]),
    "FX-LVLIC-005": ("The order: Red input white 192, then the RGB input black 32 on top. Red is "
                     "stretched by its own set first and then by the RGB set; green and blue by "
                     "the RGB set alone.", case(red_input_white=192, input_black=32), [0]),
    "FX-LVLIC-006": ("Red output black 255 and white 0: the red channel turned over, green and "
                     "blue untouched.", case(red_output_black=255, red_output_white=0), [0]),
    "FX-LVLIC-007": ("Alpha output white 128: every covering halved, the colours kept; the empty "
                     "column stays empty.", case(alpha_output_white=128), [0]),
    "FX-LVLIC-008": ("Alpha output black 64: every covering lifted to at least 64 of 255, so the "
                     "empty column shows as black at a quarter covering.",
                     case(alpha_output_black=64), [0]),
    "FX-LVLIC-009": ("Alpha input white 128: the half-covered yellow becomes fully covered; the "
                     "rest, already full, stays.", case(alpha_input_white=128), [0]),
    "FX-LVLIC-010": ("Blue input black and white both 140: a threshold on blue, 0 below 140 and "
                     "full at or above it.", case(blue_input_black=140, blue_input_white=140),
                     [0]),
    "FX-LVLIC-011": ("Levels with its Channel menu on Red and Red input white 192: exactly "
                     "FX-LVLIC-002, the same rule under the other name.",
                     case(kind="levels", channel="red", red_input_white=192), [0]),
    "FX-LVLIC-012": ("Levels with its Channel menu on Alpha, and only the RGB five written, input "
                     "white 200: the menu changes nothing and the four sets not written start "
                     "plain, so this draws as D-112's Levels with input white 200.",
                     case(kind="levels", channel="alpha", only_rgb=True, input_white=200), [0]),
    "FX-LVLIC-013": ("Red gamma keyed from 1 at frame 0 to 3 at frame 4, linear: frame 0 "
                     "untouched, frames 2 and 4 the middle reds brighter and brighter.",
                     case(red_gamma=keyed((0, 1), (4, 3))), [0, 2, 4]),
    "FX-LVLIC-014": ("Every set at once: RGB input black 10 and gamma 1.2, red input white 230 "
                     "and output black 20, green gamma 0.7 and output white 240, blue input "
                     "black 30 and gamma 1.5, alpha output white 200.", case(**ALL), [0]),
    "FX-LVLIC-015": ("FX-LVLIC-014 moved three pixels right: the same, moved.",
                     case(shift=3, **ALL), [0, 3]),
}

INVALID = {
    "FX-LVLIC-016": ("Red input black 256, above 255.", case(red_input_black=256)),
    "FX-LVLIC-017": ("Green gamma 0.05, below 0.1.", case(green_gamma=0.05)),
    "FX-LVLIC-018": ("Blue output white -1, below 0.", case(blue_output_white=-1)),
    "FX-LVLIC-019": ("Alpha gamma 11, above 10.", case(alpha_gamma=11)),
    "FX-LVLIC-020": ("Levels with the channel \"luma\", which is not one of rgb, red, green, "
                     "blue or alpha.", case(kind="levels", channel="luma")),
}


def main():
    expected = B.write_cases(OUT, "levels_individual", CASES, INVALID, render, plain,
                             file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    enc = lambda p: [S.linear_to_srgb(u / p[3]) * 255 for u in p[:3]]  # noqa: E731

    assert c["FX-LVLIC-001"]["0"] == drawn
    for fx, ch in (("FX-LVLIC-002", 0), ("FX-LVLIC-003", 1), ("FX-LVLIC-004", 2),
                   ("FX-LVLIC-006", 0)):
        f = c[fx]["0"]
        for i, p in enumerate(px):
            if p[3]:
                o, d = enc(f[i]), enc(drawn[i])
                others = [k for k in range(3) if k != ch]
                assert near([o[k] for k in others], [d[k] for k in others], 1e-6), fx
    two = c["FX-LVLIC-002"]["0"]
    assert near(enc(two[at(14, 1)])[:1], [255], 1e-6)  # 240 * 0.92 = 221, over 192
    four = c["FX-LVLIC-004"]["0"]
    assert all(64 - 1e-6 <= enc(f)[2] <= 192 + 1e-6 for f, p in zip(four, px) if p[3])
    five = c["FX-LVLIC-005"]["0"]
    p = px[at(13, 0)]  # warm midtone 150, 128, 100
    want = [(150 * 255 / 192 - 32) * 255 / 223, (128 - 32) * 255 / 223, (100 - 32) * 255 / 223]
    assert near(enc(five[at(13, 0)]), want, 1e-6), (enc(five[at(13, 0)]), want, p)
    six = c["FX-LVLIC-006"]["0"]
    assert near(enc(six[at(8, 0)]), [0, 0, 0], 1e-6) and near(enc(six[at(1, 0)]), [255, 0, 0], 1e-6)
    seven, eight, nine = (c[f"FX-LVLIC-00{k}"]["0"] for k in (7, 8, 9))
    for i, p in enumerate(px):
        a = p[3] / 255
        assert abs(seven[i][3] - a * 128 / 255) < 1e-12
        assert abs(eight[i][3] - (64 + a * 191) / 255) < 1e-12
        assert abs(nine[i][3] - min(1.0, a * 255 / 128)) < 1e-12
        if p[3]:
            assert near(enc(seven[i]), enc(drawn[i]), 1e-6)
            assert near(enc(eight[i]), enc(drawn[i]), 1e-6)
    assert near(eight[at(0, 0)], [0, 0, 0, 64 / 255])
    assert nine[at(15, 0)][3] == 1.0
    blues = {v[2] for v in px}
    assert all(abs(b - 140) > 1 for b in blues), sorted(blues)  # no blue sits on the threshold
    ten = c["FX-LVLIC-010"]["0"]
    for i, p in enumerate(px):
        if p[3]:
            assert near(enc(ten[i])[2:], [255 if p[2] >= 140 else 0], 1e-6)
    assert c["FX-LVLIC-011"]["0"] == two
    twelve = c["FX-LVLIC-012"]["0"]
    for i, p in enumerate(px):
        if p[3]:
            assert near(enc(twelve[i]), [min(255, v * 255 / 200) for v in p[:3]], 1e-6)
    thirteen = c["FX-LVLIC-013"]
    assert thirteen["0"] == drawn
    r = lambda f: enc(f[at(13, 0)])[0]  # noqa: E731
    assert 150 < r(thirteen["2"]) < r(thirteen["4"])
    fourteen, moved = c["FX-LVLIC-014"]["0"], c["FX-LVLIC-015"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == fourteen[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        for f in frames.values():
            for q in f:
                assert all(-1e-12 <= v <= q[3] + 1e-12 for v in q[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
