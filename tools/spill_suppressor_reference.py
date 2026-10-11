"""Spill Suppressor, worked a second way.

D-548 adds After Effects' Spill Suppressor (Keying, an Obsolete effect) as
`core.spill_suppressor`, a second name over Screen Key's despill (D-540), as Levels (Individual
Controls) is over Levels (D-383) and Split 2 over Split (D-394). After Effects' help: it
"removes traces of the key color from an image with a screen that's already been keyed out",
the light reflected off the screen onto the subject's edges. Its controls are Color To
Suppress, Color Accuracy (Faster for blue, green or red, Better for other colours) and
Suppression. Adobe does not publish its rule, so the rule is ours (document 21):

`color_to_suppress` is `#rrggbb` in either case, not keyable; "#0000ff" when added, the
blue screen After Effects' older keyers start from (as Color Difference Key's and Linear Color
Key's key colours here). `suppression` is 0 to 100, 100 when added, keyable. There is no Color
Accuracy: the one rule is the careful one for every colour, as Color Difference Key (D-459) has
no Faster / More Accurate setting.

The rule is Screen Key's despill with its matte turned off (Screen Gain 0, so every covering is
kept), its despill bias half way and its despill the suppression. k is the colour's 8-bit values
over 255; its screen channel s is the largest of k's three (on a tie green, then blue, then red,
so a grey colour names green); o1 and o2 are the other two. At a pixel with covering a > 0, e is
its straight colour through the sRGB curve held inside 0 to 1:

    L = (e_o1 + e_o2) / 2
    e_s = e_s - (suppression / 100) max(0, e_s - L)
    out = (srgb_to_linear(e) a, a)

So only the leading channel of the colour matters, the other two channels are never touched,
and a pixel whose screen channel is no higher than the mean of the other two keeps its colour.
A pixel with a = 0 stays transparent.

**This file never runs the build's code path, and does not call Screen Key's reference.** It
works in double precision on lists, straight from the drawing's 8-bit values, where the build
works in single precision on its buffers.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py): black, white, grey,
yellow, cyan, green, magenta, red, blue, skin, orange and three warm tones, each column darker
down the rows, an empty column and a yellow one at half covering. The projects go into
`Fixtures/spill_suppressor`, the expected frames into
`Fixtures/spill_suppressor/expected_spill_suppressor.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/spill_suppressor_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "spill_suppressor"
SUPPRESSION = (0, 100)


# --- the rule -------------------------------------------------------------------------------

def leading(k):
    """The colour's screen channel: the largest, on a tie green, then blue, then red."""
    return max((1, 2, 0), key=lambda i: (k[i], -(1, 2, 0).index(i)))


def suppress(px, color, suppression):
    """`px` are 8-bit straight RGBA; `color` is #rrggbb."""
    s = leading([v / 255 for v in R.hex_color(color.lower())])
    o1, o2 = (i for i in range(3) if i != s)
    out = []
    for p in px:
        if p[3] == 0:
            out.append([0.0] * 4)
            continue
        a = p[3] / 255
        e = [v / 255 for v in p[:3]]
        e[s] -= suppression / 100 * max(0.0, e[s] - (e[o1] + e[o2]) / 2)
        out.append([srgb_to_linear(v) * a for v in e] + [a])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(color_to_suppress="#0000ff", suppression=100, shift=0):
    return {"color_to_suppress": color_to_suppress, "suppression": suppression, "shift": shift}


def render(c, frame_no):
    v = min(SUPPRESSION[1], max(SUPPRESSION[0], value_at(c["suppression"], frame_no)))
    return R.frame(suppress(B.pixels(), c["color_to_suppress"], v), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    return B.project_json(fx, "core.spill_suppressor", {
        "color_to_suppress": c["color_to_suppress"],
        "suppression": setting_json(c["suppression"])}, c["shift"])


CASES = {
    "FX-SSUP-001": ("The settings as they start: Color To Suppress blue, Suppression 100. Each "
                    "pixel's blue is held no higher than the mean of its red and green: pure "
                    "blue turns black, cyan and magenta keep half their blue, the greys, white, "
                    "yellow, green, red, skin and the warm tones are untouched; every covering "
                    "is kept.", case(), [0]),
    "FX-SSUP-002": ("Color To Suppress green: green turns black, yellow and cyan keep half "
                    "their green; blue, magenta, red, the greys and skin are untouched.",
                    case(color_to_suppress="#00ff00"), [0]),
    "FX-SSUP-003": ("Color To Suppress green, Suppression 50: each green half way, in encoded "
                    "values, between the drawing's and FX-SSUP-002's.",
                    case(color_to_suppress="#00ff00", suppression=50), [0]),
    "FX-SSUP-004": ("Suppression 0: the drawing exactly as it is.", case(suppression=0), [0]),
    "FX-SSUP-005": ("Color To Suppress picked from a green screen, (40, 180, 60): green leads "
                    "it, so it is FX-SSUP-002 exactly.", case(color_to_suppress="#28b43c"), [0]),
    "FX-SSUP-006": ("Color To Suppress red: red turns black, yellow and magenta keep half their "
                    "red; skin and orange, whose red leads, lose some of it.",
                    case(color_to_suppress="#ff0000"), [0]),
    "FX-SSUP-007": ("Color To Suppress #808080, a grey: no channel leads, so green is taken, "
                    "as Screen Key takes it: FX-SSUP-002 exactly.",
                    case(color_to_suppress="#808080"), [0]),
    "FX-SSUP-008": ("FX-SSUP-002 with its colour written in capitals, #00FF00: the same.",
                    case(color_to_suppress="#00FF00"), [0]),
    "FX-SSUP-009": ("Suppression keyed from 0 at frame 0 to 100 at frame 4, linear, green: frame "
                    "0 is the drawing, frame 2 is FX-SSUP-003, frame 4 is FX-SSUP-002.",
                    case(color_to_suppress="#00ff00", suppression=keyed((0, 0), (4, 100))),
                    [0, 2, 4]),
    "FX-SSUP-010": ("FX-SSUP-002 moved three pixels right: the same, moved.",
                    case(color_to_suppress="#00ff00", shift=3), [0, 3]),
}

INVALID = {
    "FX-SSUP-011": ("Suppression 101, above 100.", case(suppression=101)),
    "FX-SSUP-012": ("Suppression -1, below 0.", case(suppression=-1)),
    "FX-SSUP-013": ("Suppression keyed to 150 at frame 4.",
                    case(suppression=keyed((0, 50), (4, 150)))),
    "FX-SSUP-014": ("Color To Suppress written \"blue\", a name, not #rrggbb.",
                    case(color_to_suppress="blue")),
    "FX-SSUP-015": ("Color To Suppress written \"#00ff0\", one digit short.",
                    case(color_to_suppress="#00ff0")),
}


def main():
    expected = B.write_cases(OUT, "spill_suppressor", CASES, INVALID, render, plain, file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    same = lambda f, x: all(near(f[at(x, y)], drawn[at(x, y)]) for y in range(H))  # noqa: E731
    enc = lambda q: [B.S.linear_to_srgb(v / q[3]) for v in q[:3]]  # noqa: E731

    assert leading([0, 0, 1]) == 2 and leading([0.5, 0.5, 0.5]) == 1 and leading([1, 0, 1]) == 2
    one = c["FX-SSUP-001"]["0"]
    assert near(one[at(9, 0)], [0, 0, 0, 1])                                # blue to black
    assert abs(enc(one[at(5, 0)])[2] - 0.5) < 1e-9                          # cyan's blue half
    assert abs(enc(one[at(7, 0)])[2] - 0.5) < 1e-9                          # magenta's blue half
    for x in (1, 2, 3, 4, 6, 8, 10, 11, 12, 13, 14, 15):
        assert same(one, x), x
    two = c["FX-SSUP-002"]["0"]
    assert near(two[at(6, 0)], [0, 0, 0, 1])
    assert abs(enc(two[at(4, 0)])[1] - 0.5) < 1e-9 and abs(enc(two[at(5, 0)])[1] - 0.5) < 1e-9
    for x in (1, 2, 3, 7, 8, 9, 10):
        assert same(two, x), x
    three = c["FX-SSUP-003"]["0"]
    for i, p in enumerate(px):
        if p[3]:
            assert abs(enc(three[i])[1] - (enc(drawn[i])[1] + enc(two[i])[1]) / 2) < 1e-9, i
    assert c["FX-SSUP-004"]["0"] == drawn
    assert c["FX-SSUP-005"]["0"] == two and c["FX-SSUP-007"]["0"] == two
    assert c["FX-SSUP-008"]["0"] == two
    six = c["FX-SSUP-006"]["0"]
    assert near(six[at(8, 0)], [0, 0, 0, 1]) and abs(enc(six[at(4, 0)])[0] - 0.5) < 1e-9
    assert enc(six[at(10, 0)])[0] < enc(drawn[at(10, 0)])[0]
    nine = c["FX-SSUP-009"]
    assert nine["0"] == drawn and like(nine["2"], three) and nine["4"] == two
    moved = c["FX-SSUP-010"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == two[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-SSUP-010" else 0))
        for fr in frames.values():
            for i, p in enumerate(fr):
                assert p[3] == base[i][3], (fx, i)
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), (fx, i)
    print("checked")


if __name__ == "__main__":
    main()
