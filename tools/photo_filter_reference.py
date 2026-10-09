"""Photo Filter, worked a second way.

`core.photo_filter`, modelled on After Effects' Photo Filter (the same as Photoshop's): the
picture as if shot through a coloured glass filter, the filter picked from a list or given as
a colour, its Density saying how strongly it tints, and Preserve Luminosity keeping each
pixel's brightness while the colour changes. Nothing is ported; Adobe does not publish its
method, so the rule below is ours (D-384). Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

The rule. Settings: `filter`, one of the words in FILTERS or "custom", "warming_85" when added
(After Effects' own start, Warming Filter (85)); the word is exact. `color`, `#rrggbb`, used
only when the filter is "custom", "#ec8a00" when added. `density`, 0 to 100, 25 when added,
keyable. `preserve_luminosity`, "on" or "off", "on" when added.

- The filter's colour F is the chosen preset's, or `color` when the filter is "custom". The
  presets' colours are those fmwconcepts' colorfilter script gives for Photoshop's Photo Filter
  ("simulates the Photoshop Photo Filter function"); Aspose's documentation agrees on Warming
  (85) as (236, 138, 0). Photoshop's other presets (Warming LBA, Cooling LBB, Red, Orange,
  Yellow, Green, Cyan, Blue, Violet, Magenta, Deep Red, Deep Blue, Deep Emerald, Deep Yellow)
  have no published colour we found, so they are left out; "custom" covers them.
- At a pixel with covering a > 0, e its straight colour through the sRGB curve held inside 0
  to 1 and d the density over 100: f = e (1 - d + d F) per channel, the filter laid over by
  multiplying, as glass in front of the lens passes only its own colour.
- With Preserve Luminosity on, the result is scaled so its brightness is the pixel's own: with
  L(c) = 0.2126 c_r + 0.7152 c_g + 0.0722 c_b on the encoded values, f L(e) / L(f) when L(f) > 0
  (when L(f) is 0 the filter took everything, and f, black, is kept).
- The result held inside 0 to 1 (the effect works on 8-bit values, as After Effects' does),
  back to linear at the pixel's covering (document 21's shared colour rule). The covering is
  never changed. A pixel with a = 0 is left as it is; with density 0 the layer is left exactly
  as it is.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, where the build works on its single-precision buffers.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py): pure colours, greys,
a skin tone, orange and three warm tones. The projects go into `Fixtures/photo_filter`, the
expected frames into `Fixtures/photo_filter/expected_photo_filter.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/photo_filter_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "photo_filter"
FILTERS = {"warming_85": "#ec8a00", "warming_81": "#ebb113", "cooling_80": "#006dff",
           "cooling_82": "#00b5ff", "sepia": "#ac7a33", "underwater": "#00c2b1"}


# --- the rule -------------------------------------------------------------------------------

def hex_colour(h):
    return [int(h[i:i + 2], 16) / 255 for i in (1, 3, 5)]


def luma(c):
    return 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2]


def photo_filter(px, filt, color, density, keep):
    if density == 0:
        return [R.working(p) for p in px]
    F = hex_colour(color if filt == "custom" else FILTERS[filt])
    d = density / 100
    out = []
    for p in px:
        if p[3] == 0:
            out.append(R.working(p))
            continue
        e = B.encoded(p)
        f = [v * (1 - d + d * k) for v, k in zip(e, F)]
        if keep and luma(f) > 0:
            f = [v * luma(e) / luma(f) for v in f]
        out.append(B.back(f, p))  # held inside 0 to 1 there
    return out


# --- the cases ------------------------------------------------------------------------------

def case(filt="warming_85", color="#ec8a00", density=25, keep="on", shift=0):
    return {"filter": filt, "color": color, "density": density, "keep": keep, "shift": shift}


def render(c, frame_no):
    d = min(100, max(0, value_at(c["density"], frame_no)))
    return R.frame(photo_filter(B.pixels(), c["filter"], c["color"], d, c["keep"] == "on"),
                   c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    return B.project_json(fx, "core.photo_filter", {
        "filter": c["filter"], "color": c["color"], "density": setting_json(c["density"]),
        "preserve_luminosity": c["keep"]}, c["shift"])


CASES = {
    "FX-PFILT-001": ("The settings as they start: Warming Filter (85), density 25, preserve "
                     "luminosity on: every colour a little warmer, its brightness kept; black "
                     "stays black, white turns a pale warm white.", case(), [0]),
    "FX-PFILT-002": ("The same with preserve luminosity off: warmer and a little darker, since "
                     "the orange glass holds back some of the green and blue.",
                     case(keep="off"), [0]),
    "FX-PFILT-003": ("Density 100, preserve luminosity on: the full filter, the brightness kept; "
                     "pure blue, which the orange glass stops entirely, goes black.",
                     case(density=100), [0]),
    "FX-PFILT-004": ("Density 100, preserve luminosity off: each channel multiplied by the "
                     "filter's colour, white becomes the filter's orange itself.",
                     case(density=100, keep="off"), [0]),
    "FX-PFILT-005": ("Density 0: no filter, the drawing exactly as it is.", case(density=0),
                     [0]),
    "FX-PFILT-006": ("Warming Filter (81), density 50: a yellower warming.",
                     case("warming_81", density=50), [0]),
    "FX-PFILT-007": ("Cooling Filter (80), density 50: everything bluer, the brightness kept.",
                     case("cooling_80", density=50), [0]),
    "FX-PFILT-008": ("Cooling Filter (82), density 50: a paler, cyan cooling.",
                     case("cooling_82", density=50), [0]),
    "FX-PFILT-009": ("Sepia, density 60: a brown tint, the brightness kept.",
                     case("sepia", density=60), [0]),
    "FX-PFILT-010": ("Underwater, density 60: a green-blue tint, the reds pulled down.",
                     case("underwater", density=60), [0]),
    "FX-PFILT-011": ("Custom magenta #ff00ff, density 50, preserve luminosity off: the green "
                     "channel halved, red and blue kept.", case("custom", "#ff00ff", 50, "off"),
                     [0]),
    "FX-PFILT-012": ("Warming Filter (85) with the colour set to magenta: the colour is used "
                     "only for a custom filter, so the same as FX-PFILT-001.",
                     case(color="#ff00ff"), [0]),
    "FX-PFILT-013": ("Density keyed from 0 at frame 0 to 100 at frame 4, linear, preserve "
                     "luminosity off: frame 0 untouched, frame 2 halfway, frame 4 the full "
                     "filter as FX-PFILT-004.", case(density=keyed((0, 0), (4, 100)), keep="off"),
                     [0, 2, 4]),
    "FX-PFILT-014": ("FX-PFILT-011 moved three pixels right: the same, moved.",
                     case("custom", "#ff00ff", 50, "off", shift=3), [0, 3]),
}

INVALID = {
    "FX-PFILT-015": ("Density -1, below 0.", case(density=-1)),
    "FX-PFILT-016": ("Density 101, above 100.", case(density=101)),
    "FX-PFILT-017": ("Filter \"warming\", which is not a choice.", case("warming")),
    "FX-PFILT-018": ("Filter \"Warming_85\": the word is exact, so a capital is not it.",
                     case("Warming_85")),
    "FX-PFILT-019": ("Color \"#fff\", which is not #rrggbb.", case("custom", "#fff")),
    "FX-PFILT-020": ("Preserve luminosity \"yes\", which is not \"on\" or \"off\".",
                     case(keep="yes")),
}


def main():
    expected = B.write_cases(OUT, "photo_filter", CASES, INVALID, render, plain, file_json)
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
    orange = hex_colour(FILTERS["warming_85"])

    assert c["FX-PFILT-005"]["0"] == drawn
    one = c["FX-PFILT-001"]["0"]
    assert near(enc(one[at(1, 0)]), [0, 0, 0])
    for i in (at(3, 0), at(12, 0), at(13, 0)):  # brightness kept, unless a channel was held
        assert abs(luma(enc(one[i])) - luma(B.encoded(px[i]))) < 1e-7
        assert enc(one[i])[0] > B.encoded(px[i])[0]  # warmer
    assert near(enc(one[at(2, 0)])[:1], [1]) and enc(one[at(2, 0)])[2] < 0.95  # white: pale warm white
    off = c["FX-PFILT-002"]["0"]
    assert luma(enc(off[at(3, 0)])) < luma(B.encoded(px[at(3, 0)]))
    assert near(enc(c["FX-PFILT-003"]["0"][at(9, 0)]), [0, 0, 0])  # blue stopped
    assert near(enc(c["FX-PFILT-004"]["0"][at(2, 0)]), orange)  # white becomes the orange
    seven = c["FX-PFILT-007"]["0"]
    assert enc(seven[at(3, 0)])[2] > enc(seven[at(3, 0)])[0]  # grey goes blue
    eleven = c["FX-PFILT-011"]["0"]
    assert near(enc(eleven[at(2, 0)]), [1, 0.5, 1])
    assert like(c["FX-PFILT-012"]["0"], one)
    thirteen = c["FX-PFILT-013"]
    assert thirteen["0"] == drawn and like(thirteen["4"], c["FX-PFILT-004"]["0"])
    assert near(enc(thirteen["2"][at(2, 0)]), [0.5 + 0.5 * v for v in orange])
    moved = c["FX-PFILT-014"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == eleven[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert p[3] == drawn[i][3] or fx == "FX-PFILT-014"
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
