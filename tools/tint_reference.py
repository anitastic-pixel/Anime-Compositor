"""Tint, worked a second way.

D-401 gives `core.tint` After Effects' Tint settings: each pixel's lightness picks a colour
between two, Map Black To for black and Map White To for white, the colours between them in
between, and Amount to Tint is how far each pixel is moved toward that colour. `map_black_to`
and `map_white_to` are `#rrggbb` (#000000 and #ffffff when added, which turns the picture grey),
read in small letters, not keyable, as Gradient Map's colours are; `amount_to_tint` is 0 to 100
(100), keyable. After Effects' Swap Colors is a button in the panel that trades the two colours;
it is not a setting. Blend With Original is the Mix every effect has.

A Tint saved before D-401 (`color`, a linear RGB triple, and `amount`, 0 to 1) is read and drawn
as it always was, by document 21's older Tint rule, which FX-ADJ and FX-FXK pin; this file is
only the new settings.

Adobe does not publish how its Tint reads lightness or mixes the two colours. The rule below is
Gradient Map's (D-129) with its two ends only: the lightness is Gradient Map's, the colours are
mixed in encoded values, the result goes back to linear and is mixed with the pixel in linear
light. Nothing is ported. Document 21 is the rule in words; this file is the reference for the
numbers document 25 pins against it.

The rule. At a pixel with covering a > 0 and straight linear colour b:

    t = linear_to_srgb(clamp(0.2126 b.r + 0.7152 b.g + 0.0722 b.b, 0, 1))
    E = black + t (white - black)        per channel, each colour's 8-bit value / 255
    M = srgb_to_linear(E)
    out = ((b + amount / 100 (M - b)) a, a)

A pixel that does not show stays as it is; the covering never changes. Amount 0 is the input.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py): pure colours, greys,
a skin tone, orange and three warm tones, each column darker down the rows. The projects go into
`Fixtures/tint`, the expected frames into `Fixtures/tint/expected_tint.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/tint_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "tint"
AMOUNT = (0, 100)


# --- the rule -------------------------------------------------------------------------------

def lightness(b):
    """A straight linear colour's lightness, encoded, 0 to 1."""
    return S.linear_to_srgb(min(1.0, max(0.0, 0.2126 * b[0] + 0.7152 * b[1] + 0.0722 * b[2])))


def tint(px, black, white, amount):
    """`px` are 8-bit straight RGBA; `black` and `white` are #rrggbb."""
    lo, hi = ([v / 255 for v in R.hex_color(c.lower())] for c in (black, white))
    o = amount / 100
    out = []
    for p in px:
        if p[3] == 0:
            out.append(R.working(p))
            continue
        a = p[3] / 255
        b = [srgb_to_linear(c / 255) for c in p[:3]]
        t = lightness(b)
        m = [srgb_to_linear(u + t * (v - u)) for u, v in zip(lo, hi)]
        out.append([(b[c] + o * (m[c] - b[c])) * a for c in range(3)] + [a])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(map_black_to="#000000", map_white_to="#ffffff", amount_to_tint=100, shift=0):
    return {"map_black_to": map_black_to, "map_white_to": map_white_to,
            "amount_to_tint": amount_to_tint, "shift": shift}


def render(c, frame_no):
    amount = min(AMOUNT[1], max(AMOUNT[0], value_at(c["amount_to_tint"], frame_no)))
    return R.frame(tint(B.pixels(), c["map_black_to"], c["map_white_to"], amount), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    return B.project_json(fx, "core.tint", {
        "map_black_to": c["map_black_to"], "map_white_to": c["map_white_to"],
        "amount_to_tint": setting_json(c["amount_to_tint"])}, c["shift"])


DUO = {"map_black_to": "#1a2a6c", "map_white_to": "#fdbb2d"}
SWAPPED = {"map_black_to": DUO["map_white_to"], "map_white_to": DUO["map_black_to"]}

CASES = {
    "FX-TINT-001": ("The settings as they start: black to #000000, white to #ffffff, amount "
                    "100. Every pixel that shows turns grey by its lightness, red, green and "
                    "blue alike; black stays black and white white; the yellow at half covering "
                    "takes the same grey as the yellow, at its own covering.", case(), [0]),
    "FX-TINT-002": ("Amount 0: the drawing exactly as it is.", case(amount_to_tint=0), [0]),
    "FX-TINT-003": ("Amount 50: every pixel halfway, in linear light, between the drawing and "
                    "FX-TINT-001.", case(amount_to_tint=50), [0]),
    "FX-TINT-004": ("Black to #1a2a6c, a navy, and white to #fdbb2d, a gold: a two-colour "
                    "picture; the black pixels turn exactly #1a2a6c and the white #fdbb2d, the "
                    "rest between by their lightness.", case(**DUO), [0]),
    "FX-TINT-005": ("Black to #ffffff and white to #000000, the two swapped from as they start "
                    "(Swap Colors): a negative in grey; black turns white and white black.",
                    case(map_black_to="#ffffff", map_white_to="#000000"), [0]),
    "FX-TINT-006": ("FX-TINT-004's colours swapped: black to the gold and white to the navy.",
                    case(**SWAPPED), [0]),
    "FX-TINT-007": ("Both colours #6450a0: every pixel that shows is #6450a0 at its own "
                    "covering, whatever its lightness.",
                    case(map_black_to="#6450a0", map_white_to="#6450a0"), [0]),
    "FX-TINT-008": ("FX-TINT-004 with its colours written in capitals: the same.",
                    case(**{k: v.upper() for k, v in DUO.items()}), [0]),
    "FX-TINT-009": ("FX-TINT-004 at amount 30: each pixel 30 per cent of the way, in linear "
                    "light, toward its colour.", case(amount_to_tint=30, **DUO), [0]),
    "FX-TINT-010": ("Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the "
                    "drawing, frame 2 is FX-TINT-003, frame 4 is FX-TINT-001.",
                    case(amount_to_tint=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-TINT-011": ("FX-TINT-004 moved three pixels right: the same, moved.",
                    case(shift=3, **DUO), [0, 3]),
}

INVALID = {
    "FX-TINT-012": ("Amount 101, above 100.", case(amount_to_tint=101)),
    "FX-TINT-013": ("Amount -1, below 0.", case(amount_to_tint=-1)),
    "FX-TINT-014": ("Amount keyed to 150 at frame 4.",
                    case(amount_to_tint=keyed((0, 50), (4, 150)))),
    "FX-TINT-015": ("Map Black To written \"#12345\", one digit short.",
                    case(map_black_to="#12345")),
    "FX-TINT-016": ("Map White To written \"white\", a name, not #rrggbb.",
                    case(map_white_to="white")),
}


def main():
    expected = B.write_cases(OUT, "tint", CASES, INVALID, render, plain, file_json)
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

    assert abs(lightness([1.0] * 3) - 1) < 1e-15 and lightness([0.0] * 3) == 0.0
    one = c["FX-TINT-001"]["0"]
    for i in shows:
        assert near(one[i][:3], [one[i][0]] * 3), i                       # grey
    assert near(one[black], [0, 0, 0, 1]) and near(one[white], [1, 1, 1, 1])
    assert near(straight(one[at(15, 3)]), straight(one[at(4, 3)]))     # half-covered yellow
    assert c["FX-TINT-002"]["0"] == drawn
    assert like(c["FX-TINT-003"]["0"], [[(u + v) / 2 for u, v in zip(p, q)]
                                        for p, q in zip(drawn, one)])
    four = c["FX-TINT-004"]["0"]
    assert near(four[black], lin("#1a2a6c") + [1]) and near(four[white], lin("#fdbb2d") + [1])
    five = c["FX-TINT-005"]["0"]
    assert near(five[black], [1, 1, 1, 1]) and near(five[white], [0, 0, 0, 1])
    # Swapped, each pixel's encoded grey is one less the unswapped one's.
    enc = lambda p: S.linear_to_srgb(p[0] / p[3])  # noqa: E731
    assert all(abs(enc(five[i]) - (1 - enc(one[i]))) < 1e-12 for i in shows)
    six = c["FX-TINT-006"]["0"]
    assert near(six[black], lin("#fdbb2d") + [1]) and near(six[white], lin("#1a2a6c") + [1])
    assert all(near(straight(c["FX-TINT-007"]["0"][i]), lin("#6450a0")) for i in shows)
    assert c["FX-TINT-008"]["0"] == four
    nine = c["FX-TINT-009"]["0"]
    assert like(nine, [[u + 0.3 * (v - u) for u, v in zip(p, q)] for p, q in zip(drawn, four)])
    ten = c["FX-TINT-010"]
    assert ten["0"] == drawn and like(ten["2"], c["FX-TINT-003"]["0"]) and ten["4"] == one
    moved = c["FX-TINT-011"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == four[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-TINT-011" else 0))
        for fr in frames.values():
            for i, p in enumerate(fr):
                assert p[3] == base[i][3], (fx, i)
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), (fx, i)
    print("checked")


if __name__ == "__main__":
    main()
