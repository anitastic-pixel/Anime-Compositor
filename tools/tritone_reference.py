"""Tritone, worked a second way.

D-402 adds After Effects' Tritone as `core.tritone`, a second name over Gradient Map's engine
(D-129), as Levels (Individual Controls) is over Levels (D-383) and Split 2 over Split (D-394):
each pixel's lightness picks a colour on a ramp from Shadows through Midtones to Highlights, and
Blend With Original is how much of the drawing is kept. `highlights`, `midtones` and `shadows`
are `#rrggbb` (#ffffff, #8c7355 and #000000 when added: white, a sepia brown of our own, black),
read in small letters, not keyable, as Gradient Map's colours are; `blend_with_original` is 0 to
100 (0), keyable.

Adobe does not publish how its Tritone reads lightness or mixes the colours. The rule below is
Gradient Map's with its midpoint at the middle: the lightness is Gradient Map's, the colours are
mixed in encoded values, the result goes back to linear and is mixed with the pixel in linear
light. Nothing is ported, and this file does not call Gradient Map's reference. Document 21 is
the rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. At a pixel with covering a > 0 and straight linear colour b:

    t = linear_to_srgb(clamp(0.2126 b.r + 0.7152 b.g + 0.0722 b.b, 0, 1))
    E = shadows + 2 t (midtones - shadows)               if t <= 1/2
    E = midtones + (2 t - 1) (highlights - midtones)     otherwise
        per channel, each colour's 8-bit value / 255
    M = srgb_to_linear(E)
    out = ((b + (1 - blend / 100) (M - b)) a, a)

A pixel that does not show stays as it is; the covering never changes. Blend 100 is the input.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py): pure colours, greys,
a skin tone, orange and three warm tones, each column darker down the rows. The projects go into
`Fixtures/tritone`, the expected frames into `Fixtures/tritone/expected_tritone.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/tritone_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "tritone"
BLEND = (0, 100)


# --- the rule -------------------------------------------------------------------------------

def lightness(b):
    """A straight linear colour's lightness, encoded, 0 to 1."""
    return S.linear_to_srgb(min(1.0, max(0.0, 0.2126 * b[0] + 0.7152 * b[1] + 0.0722 * b[2])))


def ramp(t, lo, mid, hi):
    """The encoded colour at lightness `t` on the three stops."""
    if t <= 0.5:
        return [u + 2 * t * (v - u) for u, v in zip(lo, mid)]
    return [u + (2 * t - 1) * (v - u) for u, v in zip(mid, hi)]


def tritone(px, highlights, midtones, shadows, blend):
    """`px` are 8-bit straight RGBA; the colours are #rrggbb."""
    hi, mid, lo = ([v / 255 for v in R.hex_color(c.lower())] for c in (highlights, midtones, shadows))
    o = 1 - blend / 100
    out = []
    for p in px:
        if p[3] == 0:
            out.append(R.working(p))
            continue
        a = p[3] / 255
        b = [srgb_to_linear(c / 255) for c in p[:3]]
        m = [srgb_to_linear(e) for e in ramp(lightness(b), lo, mid, hi)]
        out.append([(b[c] + o * (m[c] - b[c])) * a for c in range(3)] + [a])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(highlights="#ffffff", midtones="#8c7355", shadows="#000000", blend_with_original=0, shift=0):
    return {"highlights": highlights, "midtones": midtones, "shadows": shadows,
            "blend_with_original": blend_with_original, "shift": shift}


def render(c, frame_no):
    blend = min(BLEND[1], max(BLEND[0], value_at(c["blend_with_original"], frame_no)))
    return R.frame(tritone(B.pixels(), c["highlights"], c["midtones"], c["shadows"], blend), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    return B.project_json(fx, "core.tritone", {
        "highlights": c["highlights"], "midtones": c["midtones"], "shadows": c["shadows"],
        "blend_with_original": setting_json(c["blend_with_original"])}, c["shift"])


NIGHT = {"highlights": "#fdf3a7", "midtones": "#c0392b", "shadows": "#1a2a6c"}

CASES = {
    "FX-TRITONE-001": ("The settings as they start: shadows #000000, midtones #8c7355, a sepia "
                       "brown, highlights #ffffff, blend 0. A sepia print of the drawing by its "
                       "lightness; black stays black and white white; the yellow at half "
                       "covering takes the same colour as the yellow, at its own covering.",
                       case(), [0]),
    "FX-TRITONE-002": ("Blend With Original 100: the drawing exactly as it is.",
                       case(blend_with_original=100), [0]),
    "FX-TRITONE-003": ("Blend With Original 50: every pixel halfway, in linear light, between "
                       "the drawing and FX-TRITONE-001.", case(blend_with_original=50), [0]),
    "FX-TRITONE-004": ("Shadows #1a2a6c, a navy, midtones #c0392b, a red, highlights #fdf3a7, a "
                       "pale yellow: a night-to-sunset colouring; black turns exactly the navy "
                       "and white the pale yellow.", case(**NIGHT), [0]),
    "FX-TRITONE-005": ("All three #6450a0: every pixel that shows is #6450a0 at its own "
                       "covering, whatever its lightness.",
                       case(highlights="#6450a0", midtones="#6450a0", shadows="#6450a0"), [0]),
    "FX-TRITONE-006": ("FX-TRITONE-004 with its colours written in capitals: the same.",
                       case(**{k: v.upper() for k, v in NIGHT.items()}), [0]),
    "FX-TRITONE-007": ("FX-TRITONE-004 at blend 70: each pixel 30 per cent of the way, in "
                       "linear light, toward its colour.",
                       case(blend_with_original=70, **NIGHT), [0]),
    "FX-TRITONE-008": ("Blend keyed from 100 at frame 0 to 0 at frame 4, linear: frame 0 is the "
                       "drawing, frame 2 is FX-TRITONE-003, frame 4 is FX-TRITONE-001.",
                       case(blend_with_original=keyed((0, 100), (4, 0))), [0, 2, 4]),
    "FX-TRITONE-009": ("FX-TRITONE-004 moved three pixels right: the same, moved.",
                       case(shift=3, **NIGHT), [0, 3]),
}

INVALID = {
    "FX-TRITONE-010": ("Blend With Original 101, above 100.", case(blend_with_original=101)),
    "FX-TRITONE-011": ("Blend With Original -1, below 0.", case(blend_with_original=-1)),
    "FX-TRITONE-012": ("Blend With Original keyed to 150 at frame 4.",
                       case(blend_with_original=keyed((0, 50), (4, 150)))),
    "FX-TRITONE-013": ("Highlights written \"#12345\", one digit short.",
                       case(highlights="#12345")),
    "FX-TRITONE-014": ("Midtones written \"brown\", a name, not #rrggbb.", case(midtones="brown")),
    "FX-TRITONE-015": ("Shadows written \"#00000g\", not a hex digit.", case(shadows="#00000g")),
}


def main():
    expected = B.write_cases(OUT, "tritone", CASES, INVALID, render, plain, file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    lin = lambda hx: [srgb_to_linear(v / 255) for v in R.hex_color(hx.lower())]  # noqa: E731
    shows = [i for i in range(W * H) if px[i][3]]
    black, white = at(1, 0), at(2, 0)
    straight = lambda p: [v / p[3] for v in p[:3]]  # noqa: E731

    # The ramp's three stops: 0, the middle and 1.
    lo, mid, hi = [0.1, 0.2, 0.3], [0.5, 0.4, 0.6], [0.9, 0.8, 0.7]
    assert ramp(0, lo, mid, hi) == lo and ramp(0.5, lo, mid, hi) == mid and near(ramp(1, lo, mid, hi), hi)
    assert near(ramp(0.25, lo, mid, hi), [0.3, 0.3, 0.45]) and near(ramp(0.75, lo, mid, hi), [0.7, 0.6, 0.65])
    one = c["FX-TRITONE-001"]["0"]
    assert near(one[black], [0, 0, 0, 1]) and near(one[white], [1, 1, 1, 1])
    assert near(straight(one[at(15, 3)]), straight(one[at(4, 3)]))      # half-covered yellow
    assert c["FX-TRITONE-002"]["0"] == drawn
    assert like(c["FX-TRITONE-003"]["0"], [[(u + v) / 2 for u, v in zip(p, q)]
                                           for p, q in zip(drawn, one)])
    four = c["FX-TRITONE-004"]["0"]
    assert near(four[black], lin(NIGHT["shadows"]) + [1]) and near(four[white], lin(NIGHT["highlights"]) + [1])
    assert all(near(straight(c["FX-TRITONE-005"]["0"][i]), lin("#6450a0")) for i in shows)
    assert c["FX-TRITONE-006"]["0"] == four
    seven = c["FX-TRITONE-007"]["0"]
    assert like(seven, [[u + 0.3 * (v - u) for u, v in zip(p, q)] for p, q in zip(drawn, four)])
    eight = c["FX-TRITONE-008"]
    assert eight["0"] == drawn and like(eight["2"], c["FX-TRITONE-003"]["0"]) and eight["4"] == one
    moved = c["FX-TRITONE-009"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == four[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-TRITONE-009" else 0))
        for fr in frames.values():
            for i, p in enumerate(fr):
                assert p[3] == base[i][3], (fx, i)
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), (fx, i)
    print("checked")


if __name__ == "__main__":
    main()
