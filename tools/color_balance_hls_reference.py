"""Color Balance (HLS), worked a second way.

`core.color_balance_hls`, modelled on After Effects' Color Balance (HLS): every colour of the
layer is turned round the colour wheel, made lighter or darker, and made stronger or greyer by
fixed amounts. It is a separate effect from Hue/Saturation (D-113), whose lightness pushes toward
black or white by a share of what is left; this one adds its amounts straight onto the colour's
hue, lightness and saturation. Nothing is ported; Adobe does not publish its method, so the rule
below is ours (D-374). Document 21 is the rule in words; this file is the reference for the
numbers document 25 pins against it.

The rule. Settings: `hue`, -3600 to 3600 degrees, 0; `lightness` and `saturation`, -100 to
100, 0. All three keyable.

- At a pixel with covering a > 0, e its straight colour through the sRGB curve held inside 0
  to 1, and (h, l, s) its HLS: l = (max + min) / 2, s = (max - min) / (1 - |2 l - 1|), h the
  hue in degrees; a grey (max = min) has no hue and saturation 0.
- h' = h + hue; l' = l + lightness / 100 and s' = s + saturation / 100, each held inside 0 to
  1. A grey keeps saturation 0 whatever the saturation setting, having no hue to show.
- The colour of (h', l', s'), held inside 0 to 1, back to linear at the pixel's covering
  (document 21's shared colour rule). A pixel with a = 0 is left as it is; when all three
  settings are 0 the layer is left exactly as it is.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, with Python's own colorsys for HLS, where the build
works on its single-precision buffers.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py): pure colours, greys,
a skin tone, orange and three warm tones. The projects go into `Fixtures/color_balance_hls`, the
expected frames into `Fixtures/color_balance_hls/expected_color_balance_hls.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_balance_hls_reference.py
"""

import colorsys
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_balance_hls"
NUMBERS = {"hue": (-3600, 3600), "lightness": (-100, 100), "saturation": (-100, 100)}


# --- the rule -------------------------------------------------------------------------------

def clamp(v):
    return min(1.0, max(0.0, v))


def balance(px, s):
    if (s["hue"], s["lightness"], s["saturation"]) == (0, 0, 0):
        return [R.working(p) for p in px]
    out = []
    for p in px:
        if p[3] == 0:
            out.append(R.working(p))
            continue
        e = B.encoded(p)
        h, l, sat = colorsys.rgb_to_hls(*e)
        grey = max(e) == min(e)
        h = (h * 360 + s["hue"]) % 360 / 360
        l = clamp(l + s["lightness"] / 100)
        sat = 0.0 if grey else clamp(sat + s["saturation"] / 100)
        out.append(B.back(list(colorsys.hls_to_rgb(h, l, sat)), p))
    return out


# --- the cases ------------------------------------------------------------------------------

def case(hue=0, light=0, sat=0, shift=0):
    return {"hue": hue, "lightness": light, "saturation": sat, "shift": shift}


def render(c, frame_no):
    s = dict(c)
    for k, (lo, hi) in NUMBERS.items():
        s[k] = min(hi, max(lo, value_at(c[k], frame_no)))
    return R.frame(balance(B.pixels(), s), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {k: setting_json(c[k]) for k in NUMBERS}
    return B.project_json(fx, "core.color_balance_hls", params, c["shift"])


CASES = {
    "FX-HLSBAL-001": ("The settings as they start: all three 0, so the drawing, untouched.",
                      case(), [0]),
    "FX-HLSBAL-002": ("Hue 120: every colour a third of the way round the wheel, red to green, "
                      "green to blue, the skin and warm tones to greens; black, white and grey, "
                      "having no hue, stay.", case(hue=120), [0]),
    "FX-HLSBAL-003": ("Hue 480, a turn and a third: the same as FX-HLSBAL-002.",
                      case(hue=480), [0]),
    "FX-HLSBAL-004": ("Hue -90: every colour a quarter turn the other way.", case(hue=-90), [0]),
    "FX-HLSBAL-005": ("Lightness 30: every colour, greys and black among them, 0.3 lighter in "
                      "HLS, the light ones held at white.", case(light=30), [0]),
    "FX-HLSBAL-006": ("Lightness -40: every colour 0.4 darker, the dark ones held at black.",
                      case(light=-40), [0]),
    "FX-HLSBAL-007": ("Saturation -100: every colour grey at its own HLS lightness.",
                      case(sat=-100), [0]),
    "FX-HLSBAL-008": ("Saturation 50: the skin and warm tones much stronger, the pure colours "
                      "already at full strength unchanged, the greys still grey.",
                      case(sat=50), [0]),
    "FX-HLSBAL-009": ("Hue 60, lightness -20, saturation 20 together.",
                      case(hue=60, light=-20, sat=20), [0]),
    "FX-HLSBAL-010": ("Hue keyed from 0 at frame 0 to 240 at frame 4, linear: frame 0 "
                      "untouched, frame 2 FX-HLSBAL-002, frame 4 red turned blue.",
                      case(hue=keyed((0, 0), (4, 240))), [0, 2, 4]),
    "FX-HLSBAL-011": ("FX-HLSBAL-009 moved three pixels right: the same, moved.",
                      case(hue=60, light=-20, sat=20, shift=3), [0, 3]),
}

INVALID = {
    "FX-HLSBAL-012": ("Hue 3601, above 3600.", case(hue=3601)),
    "FX-HLSBAL-013": ("Lightness 101, above 100.", case(light=101)),
    "FX-HLSBAL-014": ("Saturation -101, below -100.", case(sat=-101)),
}


def main():
    expected = B.write_cases(OUT, "color_balance_hls", CASES, INVALID, render, plain, file_json)
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
    grey = lambda e: max(e) - min(e) < 1e-9  # noqa: E731

    assert c["FX-HLSBAL-001"]["0"] == drawn
    two = c["FX-HLSBAL-002"]["0"]
    assert like(two, c["FX-HLSBAL-003"]["0"])
    assert near(enc(two[at(8, 0)]), [0, 1, 0]) and near(enc(two[at(6, 0)]), [0, 0, 1])
    assert near(enc(c["FX-HLSBAL-004"]["0"][at(8, 0)]), [0.5, 0, 1])  # red to violet
    for x in (1, 2, 3):  # no hue: black, white and grey keep their colour under a hue turn
        assert all(near(two[at(x, y)], drawn[at(x, y)]) for y in range(H))
    five, six = c["FX-HLSBAL-005"]["0"], c["FX-HLSBAL-006"]["0"]
    assert near(enc(five[at(1, 0)]), [0.3] * 3) and near(enc(five[at(2, 0)]), [1] * 3)
    assert near(enc(six[at(1, 0)]), [0] * 3) and near(enc(six[at(2, 0)]), [0.6] * 3)
    for i, p in enumerate(px):
        if p[3]:
            l0 = colorsys.rgb_to_hls(*B.encoded(p))[1]
            assert abs(colorsys.rgb_to_hls(*enc(five[i]))[1] - min(1, l0 + 0.3)) < 1e-9
            assert grey(enc(c["FX-HLSBAL-007"]["0"][i]))
    eight = c["FX-HLSBAL-008"]["0"]
    assert near(eight[at(8, 0)], drawn[at(8, 0)]) and near(eight[at(3, 0)], drawn[at(3, 0)])
    s0 = colorsys.rgb_to_hls(*B.encoded(px[at(10, 0)]))[2]
    assert abs(colorsys.rgb_to_hls(*enc(eight[at(10, 0)]))[2] - min(1, s0 + 0.5)) < 1e-9
    ten = c["FX-HLSBAL-010"]
    assert ten["0"] == drawn and like(ten["2"], two)
    assert near(enc(ten["4"][at(8, 0)]), [0, 0, 1])
    nine, moved = c["FX-HLSBAL-009"]["0"], c["FX-HLSBAL-011"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == nine[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert p[3] == drawn[i][3] or fx == "FX-HLSBAL-011"
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
