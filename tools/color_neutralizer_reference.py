"""Color Neutralizer, worked a second way.

`core.color_neutralizer`, modelled on CycoreFX's CC Color Neutralizer (its manual): a tint is
taken out of the shadows, the midtones and the highlights separately, keeping the picture's
lightness, by naming the colour each should have been grey, and fine-tuned with red, green and
blue added at each. Nothing is ported; this is this program's own reading. Document 21 is the
rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. Settings: `shadows_unbalance`, `midtones_unbalance` and `highlights_unbalance`,
`#rrggbb`, the colours to make neutral, "#000000", "#808080" and "#ffffff" when added (each
already grey, so nothing changes); `shadows`, `midtones` and `highlights`, each three numbers,
red, green and blue, -255 to 255 levels added, 0 when added; `pinning`, 0 to 100, 0; and
`black_point` and `white_point`, 0 to 255, 0 and 255, which say what counts as shadow and
highlight. After Effects' Blend w. Original is the Mix every effect has (D-202): 100 less it.

- For each of the three references r, with u its unbalance colour encoded (8-bit over 255) and
  t_r its three numbers, the correction there is d_r = (luma(u) - u) + t_r / 255, channel by
  channel, where luma(e) = 0.2126 eR + 0.7152 eG + 0.0722 eB: the picked colour turned to the
  grey of its own lightness, plus what the numbers add. (CycoreFX links the colour and the
  numbers, picking a colour sets them; here they add.)
- At a pixel with covering a > 0, e its straight colour through the sRGB curve held inside 0 to
  1, and v = 255 luma(e). Its place between black and white, t, is (v - black_point) /
  (white_point - black_point) held inside 0 to 1 when white_point is above black_point;
  otherwise t is 0 below black_point and 1 from it up.
- The correction at t: from d_shadows at 0 straight to d_midtones at a half, then straight to
  d_highlights at 1.
- Pinning: with p = pinning / 200, the correction's weight is 1 when p is 0, otherwise
  min(t, 1 - t) / p held inside 0 to 1, so black and white are pinned and the correction comes
  in over the first and last p of the way.
- e + weight x correction, held inside 0 to 1, comes back to linear at the pixel's covering
  (document 21's shared colour rule). The space is the encoded colour, as every colour effect's
  (a deviation from linear light recorded in document 21). When all three corrections are 0 the
  layer is left exactly as it is.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, where the build works on its single-precision
buffers. Where white_point is not above black_point it asserts that no pixel's lightness lies
within a thousandth of a level of black_point.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py), whose columns 12 to
14 are a warm shadow, midtone and highlight. The projects go into `Fixtures/color_neutralizer`,
the expected frames into `Fixtures/color_neutralizer/expected_color_neutralizer.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_neutralizer_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_neutralizer"
NEUTRAL = ("#000000", "#808080", "#ffffff")
SHADOW, MID, HIGH = "#3c2d1e", "#968064", "#f0e6c8"  # columns 12 to 14, row 0


def luma(e):
    return 0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2]


def hex_colour(h):
    return [int(h[i:i + 2], 16) / 255 for i in (1, 3, 5)]


# --- the rule -------------------------------------------------------------------------------

def corrections(unbalance, numbers):
    out = []
    for h, t in zip(unbalance, numbers):
        u = hex_colour(h)
        out.append([luma(u) - u[c] + t[c] / 255 for c in range(3)])
    return out


def place(v, black, white):
    if white > black:
        return min(1.0, max(0.0, (v - black) / (white - black)))
    assert abs(v - black) > 1e-3, "a pixel's lightness lies on the step"
    return 0.0 if v < black else 1.0


def neutralize(px, unbalance, numbers, pinning, black, white):
    d = corrections(unbalance, numbers)
    if all(v == 0 for r in d for v in r):
        return [R.working(p) for p in px]
    out = []
    for p in px:
        if p[3] == 0:
            out.append(R.working(p))
            continue
        e = B.encoded(p)
        t = place(255 * luma(e), black, white)
        if t <= 0.5:
            k = [d[0][c] + (d[1][c] - d[0][c]) * 2 * t for c in range(3)]
        else:
            k = [d[1][c] + (d[2][c] - d[1][c]) * (2 * t - 1) for c in range(3)]
        w = 1.0 if pinning == 0 else min(1.0, max(0.0, min(t, 1 - t) / (pinning / 200)))
        out.append(B.back([e[c] + w * k[c] for c in range(3)], p))
    return out


# --- the cases ------------------------------------------------------------------------------

def case(unbalance=NEUTRAL, numbers=([0, 0, 0],) * 3, pinning=0, black=0, white=255,
         shift=0):
    return {"unbalance": list(unbalance), "numbers": [list(t) for t in numbers],
            "pinning": pinning, "black": black, "white": white, "shift": shift}


def held(c, k, frame_no, lo, hi):
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    return R.frame(neutralize(B.pixels(), c["unbalance"], c["numbers"],
                              held(c, "pinning", frame_no, 0, 100),
                              held(c, "black", frame_no, 0, 255),
                              held(c, "white", frame_no, 0, 255)), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    names = ("shadows", "midtones", "highlights")
    params = {f"{n}_unbalance": u for n, u in zip(names, c["unbalance"])}
    params.update({n: t for n, t in zip(names, c["numbers"])})
    params.update({"pinning": setting_json(c["pinning"]),
                   "black_point": setting_json(c["black"]),
                   "white_point": setting_json(c["white"])})
    return B.project_json(fx, "core.color_neutralizer", params, c["shift"])


ALL = (SHADOW, MID, HIGH)
TUNE = ([20, 0, -20], [0, 10, 0], [-30, 0, 30])

CASES = {
    "FX-NEUTRAL-001": ("The settings as they start, black, grey and white to neutralize and "
                       "nothing added: the drawing, untouched.", case(), [0]),
    "FX-NEUTRAL-002": ("Shadows unbalance the warm shadow #3c2d1e: its cast, turned the other "
                       "way, is added in full at black and fades out toward the midtones; the "
                       "warm shadow itself, under a fifth of the way up, loses about two thirds "
                       "of its cast, its lightness kept; middle grey and up are untouched.",
                       case(unbalance=(SHADOW, NEUTRAL[1], NEUTRAL[2])), [0]),
    "FX-NEUTRAL-003": ("Midtones unbalance the warm midtone #968064: the cast taken out most at "
                       "middle lightness, fading to nothing at black and at white.",
                       case(unbalance=(NEUTRAL[0], MID, NEUTRAL[2])), [0]),
    "FX-NEUTRAL-004": ("Highlights unbalance the warm highlight #f0e6c8: the cast taken out of "
                       "the light colours, white itself turned bluish.",
                       case(unbalance=(NEUTRAL[0], NEUTRAL[1], HIGH)), [0]),
    "FX-NEUTRAL-005": ("All three: the warm shadow, midtone and highlight themselves (row 0) "
                       "come out near grey, the cast gone, their lightness kept; their darker "
                       "copies further down, whose cast is smaller than the correction at "
                       "their lightness, are pushed past grey toward blue.",
                       case(unbalance=ALL), [0]),
    "FX-NEUTRAL-006": ("Numbers only: shadows red 20 and blue -20, midtones green 10, highlights "
                       "red -30 and blue 30, levels added and faded between.",
                       case(numbers=TUNE), [0]),
    "FX-NEUTRAL-007": ("All three colours with pinning 50: black and white are pinned, and the "
                       "correction comes in over the first and last quarter of the way.",
                       case(unbalance=ALL, pinning=50), [0]),
    "FX-NEUTRAL-008": ("Pinning 100: only middle lightness gets the whole correction; the rest "
                       "less, the nearer black or white.",
                       case(unbalance=ALL, pinning=100), [0]),
    "FX-NEUTRAL-009": ("Black point 40 and white point 200 with all three colours: everything "
                       "below 40 counts as shadow and above 200 as highlight.",
                       case(unbalance=ALL, black=40, white=200), [0]),
    "FX-NEUTRAL-010": ("Black point 120 and white point 100, white not above black, with the "
                       "numbers: lightness below 120 takes the shadows' numbers only, from 120 "
                       "up the highlights' only.", case(numbers=TUNE, black=120, white=100), [0]),
    "FX-NEUTRAL-011": ("Pinning keyed from 0 at frame 0 to 100 at frame 4, all three colours: "
                       "frame 0 is FX-NEUTRAL-005, frame 2 pinning 50, FX-NEUTRAL-007, frame 4 "
                       "FX-NEUTRAL-008.",
                       case(unbalance=ALL, pinning=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-NEUTRAL-012": ("FX-NEUTRAL-005 moved three pixels right: the same, moved.",
                       case(unbalance=ALL, shift=3), [0, 3]),
}

INVALID = {
    "FX-NEUTRAL-013": ("Shadows unbalance \"#3c2d1\", five digits, not a colour.",
                       case(unbalance=("#3c2d1", NEUTRAL[1], NEUTRAL[2]))),
    "FX-NEUTRAL-014": ("Midtones two numbers, not three.",
                       case(numbers=([0, 0, 0], [0, 0], [0, 0, 0]))),
    "FX-NEUTRAL-015": ("Highlights blue 256, above 255.",
                       case(numbers=([0, 0, 0], [0, 0, 0], [0, 0, 256]))),
    "FX-NEUTRAL-016": ("Pinning 101, above 100.", case(pinning=101)),
    "FX-NEUTRAL-017": ("Black point -1, below 0.", case(black=-1)),
    "FX-NEUTRAL-018": ("White point 256, above 255.", case(white=256)),
}


def main():
    expected = B.write_cases(OUT, "color_neutralizer", CASES, INVALID, render, plain, file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    enc = lambda f, i: [S.linear_to_srgb(v / f[i][3]) for v in f[i][:3]]  # noqa: E731
    spread = lambda e: max(e) - min(e)  # noqa: E731

    assert hex_colour(SHADOW) == B.encoded(px[at(12, 0)])
    assert hex_colour(MID) == B.encoded(px[at(13, 0)])
    assert hex_colour(HIGH) == B.encoded(px[at(14, 0)])
    assert corrections(NEUTRAL, ([0] * 3,) * 3) == [[0.0] * 3] * 3
    assert c["FX-NEUTRAL-001"]["0"] == drawn
    # One reference alone: that colour itself comes out grey, at its own lightness.
    two, three, four = (c[f"FX-NEUTRAL-00{i}"]["0"] for i in (2, 3, 4))
    # One reference alone, at its place: the warm shadow sits under a fifth of the way up, so
    # takes about two thirds of the shadows' correction; its lightness is kept exactly.
    sh, hi = enc(two, at(12, 0)), enc(four, at(14, 0))
    assert spread(sh) < spread(hex_colour(SHADOW)) / 2.5
    assert spread(hi) < spread(hex_colour(HIGH)) / 4
    assert abs(luma(sh) - luma(hex_colour(SHADOW))) < 1e-9
    assert abs(luma(hi) - luma(hex_colour(HIGH))) < 1e-9
    assert near(two[at(2, 0)], drawn[at(2, 0)]) and near(three[at(2, 0)], drawn[at(2, 0)])
    assert near(three[at(1, 0)], drawn[at(1, 0)]) and near(four[at(1, 0)], drawn[at(1, 0)])
    w = enc(four, at(2, 0))
    assert abs(w[2] - 1) < 1e-9 and w[0] < w[2] - 0.03  # white turned bluish
    # All three: the three warm colours themselves come out near grey; their darker copies,
    # whose cast is smaller, are pushed past grey toward blue.
    five = c["FX-NEUTRAL-005"]["0"]
    for x in (12, 13, 14):
        assert spread(enc(five, at(x, 0))) < spread(B.encoded(px[at(x, 0)])) / 4, x
        e = enc(five, at(x, 9))
        assert e[2] > e[0], x
    # Pinning: black and white untouched; middle grey the same as without pinning.
    seven, eight = c["FX-NEUTRAL-007"]["0"], c["FX-NEUTRAL-008"]["0"]
    for f in (seven, eight):
        assert near(f[at(1, 0)], drawn[at(1, 0)]) and near(f[at(2, 0)], drawn[at(2, 0)])
    assert not near(five[at(1, 0)], drawn[at(1, 0)]) and not near(five[at(2, 0)], drawn[at(2, 0)])
    assert near(seven[at(3, 0)], five[at(3, 0)])
    # The step: below 120 the shadows' numbers alone, from 120 up the highlights'.
    ten = c["FX-NEUTRAL-010"]["0"]
    for i in range(W * H):
        if px[i][3] == 0:
            continue
        e = B.encoded(px[i])
        t = TUNE[0] if 255 * luma(e) < 120 else TUNE[2]
        assert near(ten[i], B.back([e[k] + t[k] / 255 for k in range(3)], px[i]))
    eleven = c["FX-NEUTRAL-011"]
    assert like(eleven["0"], five) and like(eleven["2"], seven) and like(eleven["4"], eight)
    moved = c["FX-NEUTRAL-012"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == five[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
