"""Toner, worked a second way.

`core.toner`, modelled on CycoreFX's CC Toner (its manual): every pixel's lightness picks a
colour off a ramp of two, three or five tones, shadows at the dark end and highlights at the
light end, the way a duotone or a sepia print is made. Nothing is ported; this is this
program's own reading. Document 21 is the rule in words; this file is the reference for the
numbers document 25 pins against it.

The rule. Settings: `tones`, "duotone", "tritone" or "pentone", "tritone" when added; the word
is exact. `highlights`, `brights`, `midtones`, `darktones` and `shadows`, `#rrggbb`, "#ffffff",
"#e0cfb0", "#8c7355", "#46382a" and "#000000" when added (a warm sepia; CycoreFX's own starting
colours are not published, so these are ours). CC Toner's Blend w. Original is the Mix every
effect has (D-202): 100 less it.

- The ramp: duotone is shadows at 0 and highlights at 1; tritone shadows at 0, midtones at a
  half and highlights at 1; pentone shadows, darktones, midtones, brights and highlights at 0,
  a quarter, a half, three quarters and 1. A tone the choice leaves out is kept but not used.
- At a pixel with covering a > 0: its lightness t is the luma of its straight linear colour,
  0.2126 R + 0.7152 G + 0.0722 B held inside 0 to 1, through the sRGB curve, as Gradient Map's
  (D-129). Between the two stops either side of t, the colour goes straight from one to the
  other in encoded values, and comes back to linear at the pixel's own covering (document 21's
  shared colour rule). The covering is kept; a pixel that does not show stays as it is.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, where the build works on its single-precision
buffers.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py). The projects go into
`Fixtures/toner`, the expected frames into `Fixtures/toner/expected_toner.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/toner_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "toner"
NAMES = ("highlights", "brights", "midtones", "darktones", "shadows")
START = {"highlights": "#ffffff", "brights": "#e0cfb0", "midtones": "#8c7355",
         "darktones": "#46382a", "shadows": "#000000"}
STOPS = {"duotone": ("shadows", "highlights"),
         "tritone": ("shadows", "midtones", "highlights"),
         "pentone": ("shadows", "darktones", "midtones", "brights", "highlights")}


# --- the rule -------------------------------------------------------------------------------

def hex_colour(h):
    return [int(h[i:i + 2], 16) / 255 for i in (1, 3, 5)]


def lightness(p):
    lin = [srgb_to_linear(v) for v in B.encoded(p)]
    return S.linear_to_srgb(min(1.0, max(0.0, 0.2126 * lin[0] + 0.7152 * lin[1]
                                         + 0.0722 * lin[2])))


def ramp(t, colours):
    n = len(colours) - 1
    k = min(int(t * n), n - 1)  # the segment from stop k at k / n to stop k + 1
    s = t * n - k
    return [a + s * (b - a) for a, b in zip(colours[k], colours[k + 1])]


def toner(px, tones, colours):
    stops = [hex_colour(colours[n]) for n in STOPS[tones]]
    return [R.working(p) if p[3] == 0 else B.back(ramp(lightness(p), stops), p) for p in px]


# --- the cases ------------------------------------------------------------------------------

def case(tones="tritone", shift=0, **colours):
    c = {"tones": tones, "shift": shift}
    c.update(START)
    c.update(colours)
    return c


def render(c, frame_no):
    return R.frame(toner(B.pixels(), c["tones"], c), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {"tones": c["tones"]}
    params.update({n: c[n] for n in NAMES})
    return B.project_json(fx, "core.toner", params, c["shift"])


NIGHT = {"shadows": "#1b1464", "midtones": "#c0392b", "highlights": "#f9e79f"}
DUO = {"shadows": "#2b0a3d", "highlights": "#ffd166"}
FIVE = {"shadows": "#10002b", "darktones": "#3c096c", "midtones": "#9d4edd",
        "brights": "#ff9e00", "highlights": "#fff3b0"}

CASES = {
    "FX-TONER-001": ("Tritone, the colours as they start: black, a sepia brown and white, a "
                     "warm sepia print of the picture by its lightness.", case(), [0]),
    "FX-TONER-002": ("Duotone, as it starts: black to white, the picture turned to greys by its "
                     "lightness.", case("duotone"), [0]),
    "FX-TONER-003": ("Pentone, as it starts: black, dark brown, sepia, cream and white, a "
                     "sepia print with more steps.", case("pentone"), [0]),
    "FX-TONER-004": ("Tritone, navy, red and pale yellow: a night-to-sunset colouring.",
                     case(**NIGHT), [0]),
    "FX-TONER-005": ("Duotone, deep purple to warm yellow.", case("duotone", **DUO), [0]),
    "FX-TONER-006": ("Pentone, five colours from near-black purple through violet and orange "
                     "to pale yellow.", case("pentone", **FIVE), [0]),
    "FX-TONER-007": ("Tritone with brights and darktones set to loud green and blue: tritone "
                     "does not use them, so the same as FX-TONER-001.",
                     case(brights="#00ff00", darktones="#0000ff"), [0]),
    "FX-TONER-008": ("Duotone with midtones set to green: duotone does not use it, so the same "
                     "as FX-TONER-002.", case("duotone", midtones="#00ff00"), [0]),
    "FX-TONER-009": ("FX-TONER-004 moved three pixels right: the same, moved.",
                     case(shift=3, **NIGHT), [0, 3]),
}

INVALID = {
    "FX-TONER-010": ("Tones \"quadtone\", which is not a choice.", case("quadtone")),
    "FX-TONER-011": ("Tones \"Tritone\": the word is exact, so a capital is not it.",
                     case("Tritone")),
    "FX-TONER-012": ("Highlights \"#fff\", which is not #rrggbb.", case(highlights="#fff")),
    "FX-TONER-013": ("Midtones \"brown\", which is not a colour code.", case(midtones="brown")),
}


def main():
    expected = B.write_cases(OUT, "toner", CASES, INVALID, render, plain, file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    shows = [i for i in range(W * H) if px[i][3] > 0]
    enc = lambda p: [S.linear_to_srgb(u / p[3]) for u in p[:3]]  # noqa: E731

    # Duotone black to white is the lightness as a grey.
    two = c["FX-TONER-002"]["0"]
    for i in shows:
        assert near(enc(two[i]), [lightness(px[i])] * 3)
    # Black and white land on the end stops; a mid grey of lightness exactly a half would land
    # on the midtone. The ramp's own pieces: each stop at its place.
    one = c["FX-TONER-001"]["0"]
    assert near(enc(one[at(1, 0)]), [0, 0, 0]) and near(enc(one[at(2, 0)]), [1, 1, 1])
    for tones, names in STOPS.items():
        n = len(names) - 1
        cols = [hex_colour(START[k]) for k in names]
        for k in range(n + 1):
            assert near(ramp(k / n, cols), cols[k])
    # Pentone and tritone agree where pentone's extra stops sit on tritone's straight lines.
    mid = [(a + b) / 2 for a, b in zip(hex_colour("#000000"), hex_colour("#8c7355"))]
    assert near(ramp(0.25, [hex_colour("#000000"), hex_colour("#8c7355"),
                            hex_colour("#ffffff")]), mid)
    # Tones the choice leaves out are not used.
    assert like(c["FX-TONER-007"]["0"], one) and like(c["FX-TONER-008"]["0"], two)
    assert not like(c["FX-TONER-003"]["0"], one)
    # Every result is on the ramp: its colour lies between the two stops around t.
    four = c["FX-TONER-004"]["0"]
    stops = [hex_colour(NIGHT[k]) for k in STOPS["tritone"]]
    for i in shows:
        assert near(enc(four[i]), ramp(lightness(px[i]), stops))
    base, moved = c["FX-TONER-004"]["0"], c["FX-TONER-009"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == base[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert p[3] == drawn[i][3] or fx == "FX-TONER-009"
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
