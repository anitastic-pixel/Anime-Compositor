"""Gamma/Pedestal/Gain, worked a second way.

`core.gamma_pedestal_gain`, modelled on After Effects' Gamma/Pedestal/Gain: one stretch that
lifts the dark values of every channel, then for red, green and blue apart a curve with three
handles, the gamma bending its middle, the pedestal setting the lowest value the channel can
reach and the gain the highest. Nothing is ported; Adobe does not publish its method, so the
rule below is ours (D-382). Document 21 is the rule in words; this file is the reference for the
numbers document 25 pins against it.

The rule. Settings: `black_stretch`, 1 to 4, 1; for each of red, green and blue a `_gamma`,
0.1 to 10, 1; a `_pedestal`, -1 to 1, 0; and a `_gain`, 0 to 4, 1. All ten keyable.

- At a pixel with covering a > 0, e its straight colour through the sRGB curve held inside 0
  to 1, and s the black stretch: each channel first x = s e / (1 + (s - 1) e), which keeps 0
  at 0 and 1 at 1 and lifts the dark values more the larger s is.
- Then each channel c: pedestal_c + (gain_c - pedestal_c) x^(1 / gamma_c), a gamma above 1
  brightening the middle as Levels' does; a pedestal above the gain turns the channel over.
- The result held inside 0 to 1 (the effect works on 8-bit values, as After Effects' does),
  back to linear at the pixel's covering (document 21's shared colour rule). The covering is
  never changed. A pixel with a = 0 is left as it is; when every setting is at its start the
  layer is left exactly as it is.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, where the build works on its single-precision buffers.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py): pure colours, greys,
a skin tone, orange and three warm tones. The projects go into `Fixtures/gamma_pedestal_gain`,
the expected frames into `Fixtures/gamma_pedestal_gain/expected_gamma_pedestal_gain.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/gamma_pedestal_gain_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "gamma_pedestal_gain"
CHANNELS = ("red", "green", "blue")
NUMBERS = {"black_stretch": (1, 4)}
for _c in CHANNELS:
    NUMBERS |= {f"{_c}_gamma": (0.1, 10), f"{_c}_pedestal": (-1, 1), f"{_c}_gain": (0, 4)}
START = {k: (0 if k.endswith("pedestal") else 1) for k in NUMBERS}


# --- the rule -------------------------------------------------------------------------------

def clamp(v):
    return min(1.0, max(0.0, v))


def gpg(px, s):
    if all(s[k] == v for k, v in START.items()):
        return [R.working(p) for p in px]
    st = s["black_stretch"]
    out = []
    for p in px:
        if p[3] == 0:
            out.append(R.working(p))
            continue
        e = B.encoded(p)
        o = []
        for c, v in zip(CHANNELS, e):
            x = st * v / (1 + (st - 1) * v)
            ped, gain = s[f"{c}_pedestal"], s[f"{c}_gain"]
            o.append(clamp(ped + (gain - ped) * x ** (1 / s[f"{c}_gamma"])))
        out.append(B.back(o, p))
    return out


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, **kw):
    c = dict(START)
    for k, v in kw.items():
        if k in CHANNELS + ("all",):
            continue
        c[k] = v
    for part in ("gamma", "pedestal", "gain"):  # all=dict(gamma=..) sets the three channels
        if part in kw.get("all", {}):
            for ch in CHANNELS:
                c[f"{ch}_{part}"] = kw["all"][part]
    c["shift"] = shift
    return c


def render(c, frame_no):
    s = dict(c)
    for k, (lo, hi) in NUMBERS.items():
        s[k] = min(hi, max(lo, value_at(c[k], frame_no)))
    return R.frame(gpg(B.pixels(), s), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {k: setting_json(c[k]) for k in NUMBERS}
    return B.project_json(fx, "core.gamma_pedestal_gain", params, c["shift"])


CASES = {
    "FX-GPG-001": ("The settings as they start: black stretch 1, every gamma and gain 1, every "
                   "pedestal 0, so the drawing, untouched.", case(), [0]),
    "FX-GPG-002": ("Black stretch 2: every dark value lifted, the warm shadow and the dim rows "
                   "most; black and full values stay.", case(black_stretch=2), [0]),
    "FX-GPG-003": ("Black stretch 4, as far as it goes: the darks lifted much more.",
                   case(black_stretch=4), [0]),
    "FX-GPG-004": ("Red gamma 2: the middle of the red channel brighter, so the skin, grey and "
                   "warm tones go redder; green and blue untouched, and 0 and full red stay.",
                   case(red_gamma=2), [0]),
    "FX-GPG-005": ("Every gamma 0.5: the middle of every channel darker, the colours deeper.",
                   case(all={"gamma": 0.5}), [0]),
    "FX-GPG-006": ("Every pedestal 0.2: black becomes a grey of 0.2 (about 51 of 255), white "
                   "stays white, everything between lifted a little less the brighter it is.",
                   case(all={"pedestal": 0.2}), [0]),
    "FX-GPG-007": ("Every gain 0.5: white becomes a grey of 0.5, black stays black, everything "
                   "halved in its encoded values.", case(all={"gain": 0.5}), [0]),
    "FX-GPG-008": ("Every pedestal and gain 0.5: every colour the same grey of 0.5, the "
                   "covering kept.", case(all={"pedestal": 0.5, "gain": 0.5}), [0]),
    "FX-GPG-009": ("Every pedestal 1 and gain 0: the channel turned over, so the drawing's "
                   "negative: black white, red cyan, yellow blue.",
                   case(all={"pedestal": 1, "gain": 0}), [0]),
    "FX-GPG-010": ("Every pedestal -0.5: the lower half of each channel held at 0, the rest "
                   "stretched down to it; white stays white.", case(all={"pedestal": -0.5}),
                   [0]),
    "FX-GPG-011": ("Every gain 2: each channel doubled, everything above half held at full.",
                   case(all={"gain": 2}), [0]),
    "FX-GPG-012": ("Black stretch 1.5, red gain 1.2, green pedestal 0.1 and blue gamma 1.5 "
                   "together, each channel its own curve.",
                   case(black_stretch=1.5, red_gain=1.2, green_pedestal=0.1, blue_gamma=1.5),
                   [0]),
    "FX-GPG-013": ("Red gain keyed from 1 at frame 0 to 0 at frame 4, linear: frame 0 "
                   "untouched, frame 2 the red channel halved, frame 4 no red at all.",
                   case(red_gain=keyed((0, 1), (4, 0))), [0, 2, 4]),
    "FX-GPG-014": ("FX-GPG-012 moved three pixels right: the same, moved.",
                   case(black_stretch=1.5, red_gain=1.2, green_pedestal=0.1, blue_gamma=1.5,
                        shift=3), [0, 3]),
}

INVALID = {
    "FX-GPG-015": ("Black stretch 0.9, below 1.", case(black_stretch=0.9)),
    "FX-GPG-016": ("Black stretch 4.1, above 4.", case(black_stretch=4.1)),
    "FX-GPG-017": ("Green gamma 0.09, below 0.1.", case(green_gamma=0.09)),
    "FX-GPG-018": ("Blue gamma 10.1, above 10.", case(blue_gamma=10.1)),
    "FX-GPG-019": ("Red pedestal 1.1, above 1.", case(red_pedestal=1.1)),
    "FX-GPG-020": ("Green gain -0.1, below 0.", case(green_gain=-0.1)),
    "FX-GPG-021": ("Blue gain 4.1, above 4.", case(blue_gain=4.1)),
}


def main():
    expected = B.write_cases(OUT, "gamma_pedestal_gain", CASES, INVALID, render, plain,
                             file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    enc = lambda p: [S.linear_to_srgb(u / p[3]) for u in p[:3]]  # noqa: E731

    assert c["FX-GPG-001"]["0"] == drawn
    two, four = c["FX-GPG-002"]["0"], c["FX-GPG-003"]["0"]
    for x in (1, 2, 8):  # black, white and full red keep their 0 and 1 under any stretch
        assert near(two[at(x, 0)], drawn[at(x, 0)]) and near(four[at(x, 0)], drawn[at(x, 0)])
    assert enc(drawn[at(12, 0)])[0] < enc(two[at(12, 0)])[0] < enc(four[at(12, 0)])[0]
    r = c["FX-GPG-004"]["0"]
    for i, p in enumerate(px):
        if p[3]:
            e, d, o = B.encoded(p), enc(drawn[i]), enc(r[i])
            assert near(o[1:], d[1:], 1e-7) and (o[0] > d[0] or e[0] in (0, 1))
    assert near(enc(c["FX-GPG-006"]["0"][at(1, 0)]), [0.2] * 3)
    assert near(enc(c["FX-GPG-006"]["0"][at(2, 0)]), [1] * 3)
    assert near(enc(c["FX-GPG-007"]["0"][at(2, 0)]), [0.5] * 3)
    assert near(enc(c["FX-GPG-007"]["0"][at(1, 0)]), [0] * 3)
    neg = c["FX-GPG-009"]["0"]
    for i, p in enumerate(px):
        if p[3]:
            assert near(enc(c["FX-GPG-008"]["0"][i]), [0.5] * 3, 1e-7)
            assert near(enc(neg[i]), [1 - v for v in B.encoded(p)], 1e-7)
    assert near(enc(c["FX-GPG-010"]["0"][at(3, 9)]), [0] * 3)  # grey 36/255, under half
    assert near(enc(c["FX-GPG-011"]["0"][at(3, 0)]), [1] * 3)  # grey 128/255, over half
    thirteen = c["FX-GPG-013"]
    assert thirteen["0"] == drawn
    assert near(enc(thirteen["2"][at(8, 0)]), [0.5, 0, 0])
    assert near(enc(thirteen["4"][at(10, 0)])[:1], [0])
    twelve, moved = c["FX-GPG-012"]["0"], c["FX-GPG-014"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == twelve[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert p[3] == drawn[i][3] or fx == "FX-GPG-014"
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
