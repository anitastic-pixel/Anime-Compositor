"""Shadow/Highlight's Midtone Contrast and Black and White Clip, worked a second way.

D-400 added `core.shadow_highlight`, After Effects' Shadow/Highlight, without three of its
settings: Midtone Contrast, Black Clip and White Clip (gaps in document 14).
`tools/shadow_highlight_reference.py` is D-400's rule and is not changed. D-563 adds the three.
Adobe does not publish what they do, and no page was read anew for D-563. What is known is
only what the panel and the manuals of the two programs that have the effect say in words,
as recalled: Midtone Contrast is the contrast of the middle tones, 0 leaving them alone; the
clips say how much of the darkest and lightest parts are clipped to the new extreme shadow and
highlight colours in the picture, a larger value giving more contrast. The rule below is this
program's own; nothing is ported.

The settings, each keyable: `midtone_contrast` -100 to 100; `black_clip` and `white_clip` 0 to
50 per cent. All three are 0 when the effect is added, and a file without them reads 0, so a
file written before D-563 draws as it did (FX-SHCLIP-001). After Effects' panel starts both
clips at 0.01 per cent (as recalled, unconfirmed); 0 is chosen here because a clip reads the
whole picture, which keeps the effect off the card (document 14 records it). Auto Amounts,
Temporal Smoothing and Scene Detect stay gaps.

The rule, after D-400's (its highlights and then its shadows, on l, p, q, document 21):

1. Midtone Contrast, m = midtone contrast / 100, on the lightness of each pixel with covering:
   l' = l + m l (1 - l) (2 l - 1), Color Grade's contrast curve (D-397). At m = 1 the middle is
   half as steep again and the ends flat; at m = -1 the middle half as steep; 0 and 1 stay, so
   black and white stay. The chroma follows the lightness: where l' < l, p and q are times
   l' / l; where l' > l, times (1 - l') / (1 - l); so a colour taken to black is black and one
   taken to white is white.
2. The clips, when either is above 0. A histogram of 256 bins of l' over every pixel with
   covering a > 0, each adding a to bin floor(255 l' + 1/2) (held inside 0 to 255), as
   Stretch Levels counts (D-351). The black bin kb is the lowest whose running total from the
   bottom is above `black_clip` per cent of the whole, the white bin kw the highest whose
   running total from the top is above `white_clip` per cent (`frame_stats::clip_points`); lo
   and hi the same at 0 per cent, the darkest and lightest bins in use. Where kw > kb, l' is
   taken along the line through (kb / 255, lo / 255) and (kw / 255, hi / 255) and held inside 0
   to 1: the clipped share is pushed to, and past, the picture's own darkest and lightest, and
   with the picture's black and white in use, to black and white. Where kw <= kb, nothing.
   The chroma follows the lightness as in step 1. The count is of the whole layer, so moving the layer moves the result.
3. Back to linear as D-400, each channel held inside 0 to 1, at the pixel's covering. With both
   amounts, the midtone contrast and both clips 0 the layer is left exactly as it is.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values. Each case is checked to keep every l' at least 1e-4 / 255 from a
bin's edge and every running total away from its clip, so a single-precision blur in the build
cannot move a pixel to another bin.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py), as D-400's are. The
projects go into `Fixtures/shadow_highlight_clip`, the expected frames into
`Fixtures/shadow_highlight_clip/expected_shadow_highlight_clip.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/shadow_highlight_clip_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import shadow_highlight_reference as SH  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "shadow_highlight_clip"
NEW = ("midtone_contrast", "black_clip", "white_clip")
NAMES = SH.NAMES + NEW
START = dict(SH.START, midtone_contrast=0, black_clip=0, white_clip=0)
RANGE = dict(SH.RANGE, midtone_contrast=(-100, 100), black_clip=(0, 50), white_clip=(0, 50))


def bin_of(v):
    return min(255, max(0, math.floor(v * 255 + 0.5)))


def clip_points(h, black, white):
    """frame_stats::clip_points, checking no running total lies on its clip."""
    t = sum(h)
    lo, hi = t * black / 100, t * white / 100
    kb = kw = None
    run = 0.0
    for k in range(256):
        run += h[k]
        assert abs(run - lo) > 1e-6 * t or (run == 0 and lo == 0), "a running total on its clip"
        if kb is None and run > lo:
            kb = k
    run = 0.0
    for k in range(255, -1, -1):
        run += h[k]
        assert abs(run - hi) > 1e-6 * t or (run == 0 and hi == 0), "a running total on its clip"
        if kw is None and run > hi:
            kw = k
    return (255 if kb is None else kb), (0 if kw is None else kw)


def follow(c, l2):
    """l, p, q with the lightness l2, the chroma following it toward black or white."""
    l, p, q = c
    k = l2 / l if l2 < l else (1 - l2) / (1 - l) if l2 > l else 1.0
    return [l2, p * k, q * k]


def rule(px, v):
    m = v["midtone_contrast"] / 100
    black, white = v["black_clip"], v["white_clip"]
    amounts = v["shadow_amount"] != 0 or v["highlight_amount"] != 0
    if not amounts and m == 0 and black == 0 and white == 0:
        return [R.working(p) for p in px]
    straight = [[srgb_to_linear(c / 255) for c in p[:3]] for p in px]
    labs = [SH.lab(b) if p[3] else [0.0, 0.0, 0.0] for b, p in zip(straight, px)]
    plane = [[labs[y * W + x][0] for x in range(W)] for y in range(H)]
    if amounts:
        ls, lh = SH.held_blur(plane, v["shadow_radius"]), SH.held_blur(plane, v["highlight_radius"])
    else:
        ls = lh = plane  # no step runs with both amounts 0, so what they hold is never read
    s, h = 2 * v["shadow_amount"] / 100, 2 * v["highlight_amount"] / 100
    cs = min(1 - v["shadow_tonal_width"] / 100, 0.99)
    ch = min(1 - v["highlight_tonal_width"] / 100, 0.99)
    k = v["color_correction"] / 100
    lpq = []
    for i, p in enumerate(px):
        if p[3] == 0:
            lpq.append(None)
            continue
        y, x = divmod(i, W)
        l, pp, qq = labs[i]
        l, pp, qq = SH.overlay(l, pp, qq, 1 - lh[y][x], h,
                               lambda t: min(1, max(0, 1 - t / (1 - ch))), False, k)
        l, pp, qq = SH.overlay(l, pp, qq, 1 - ls[y][x], s,
                               lambda t: min(1, max(0, t / (1 - cs) - cs / (1 - cs))), True, k)
        lpq.append(follow([l, pp, qq], l + m * l * (1 - l) * (2 * l - 1)))
    if black > 0 or white > 0:
        hist = [0.0] * 256
        for p, c in zip(px, lpq):
            if c is not None:
                edge = abs((c[0] * 255) % 1 - 0.5)
                assert edge > 1e-4 or c[0] in (0.0, 1.0), ("l on a bin's edge", c[0])
                hist[bin_of(c[0])] += p[3] / 255
        kb, kw = clip_points(hist, black, white)
        lo, hi = clip_points(hist, 0, 0)
        if kw > kb:
            for i, c in enumerate(lpq):
                if c is not None:
                    t = lo / 255 + (c[0] - kb / 255) * (hi - lo) / (kw - kb)
                    lpq[i] = follow(c, min(1.0, max(0.0, t)))
    out = []
    for p, c in zip(px, lpq):
        if c is None:
            out.append(R.working(p))
            continue
        a = p[3] / 255
        out.append([min(1.0, max(0.0, u)) * a for u in SH.unlab(*c)] + [a])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, written=NEW, **settings):
    c = dict(START)
    c.update(settings)
    c["shift"] = shift
    c["written"] = written
    return c


def render(c, frame_no):
    v = {n: min(RANGE[n][1], max(RANGE[n][0], value_at(c[n], frame_no))) for n in NAMES}
    return R.frame(rule(B.pixels(), v), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    names = SH.NAMES + tuple(n for n in NEW if n in c["written"])
    return B.project_json(fx, "core.shadow_highlight", {n: setting_json(c[n]) for n in names},
                          c["shift"])


BOTH = dict(shadow_amount=70, highlight_amount=30, midtone_contrast=40, black_clip=10,
            white_clip=10)

CASES = {
    "FX-SHCLIP-001": ("A file written before D-563, without the three settings, at "
                      "Shadow/Highlight's starting settings: drawn exactly as FX-SHHI-001.",
                      case(written=()), [0]),
    "FX-SHCLIP-002": ("The three written at 0: the same as FX-SHCLIP-001.", case(), [0]),
    "FX-SHCLIP-003": ("Midtone contrast 50 after the starting shadow lift: lightnesses above "
                      "the middle lighter, below it darker; black and white stay.",
                      case(midtone_contrast=50), [0]),
    "FX-SHCLIP-004": ("Midtone contrast 100: further than FX-SHCLIP-003.",
                      case(midtone_contrast=100), [0]),
    "FX-SHCLIP-005": ("Midtone contrast -50: the middle tones drawn together toward the "
                      "middle.", case(midtone_contrast=-50), [0]),
    "FX-SHCLIP-006": ("Midtone contrast -100: further than FX-SHCLIP-005.",
                      case(midtone_contrast=-100), [0]),
    "FX-SHCLIP-007": ("Midtone contrast 60 with both amounts 0: it works alone, with no lift.",
                      case(shadow_amount=0, midtone_contrast=60), [0]),
    "FX-SHCLIP-008": ("Black clip 20: the darkest fifth of the picture pushed to black.",
                      case(black_clip=20), [0]),
    "FX-SHCLIP-009": ("White clip 10: the lightest tenth pushed to white.",
                      case(white_clip=10), [0]),
    "FX-SHCLIP-010": ("Black clip 15 and white clip 15 together: both ends.",
                      case(black_clip=15, white_clip=15), [0]),
    "FX-SHCLIP-011": ("Both clips 12 with both amounts 0: the drawing stretched by its own "
                      "count alone.", case(shadow_amount=0, black_clip=12, white_clip=12), [0]),
    "FX-SHCLIP-012": ("Both clips 0.01, After Effects' starting numbers as recalled: the "
                      "drawing's own black and white are more than 0.01 per cent of it, so "
                      "nothing is clipped and it is drawn as FX-SHCLIP-001.",
                      case(black_clip=0.01, white_clip=0.01), [0]),
    "FX-SHCLIP-013": ("Shadow 70, highlight 30, midtone contrast 40, both clips 10: every step "
                      "in turn.", case(**BOTH), [0]),
    "FX-SHCLIP-014": ("Midtone contrast keyed from -100 at frame 0 to 100 at frame 4, linear: "
                      "frame 0 as FX-SHCLIP-006, frame 2 as FX-SHCLIP-002, frame 4 as "
                      "FX-SHCLIP-004.", case(midtone_contrast=keyed((0, -100), (4, 100))),
                      [0, 2, 4]),
    "FX-SHCLIP-015": ("Black clip keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 as "
                      "FX-SHCLIP-002, frame 2 as FX-SHCLIP-008, frame 4 further.",
                      case(black_clip=keyed((0, 0), (4, 40))), [0, 2, 4]),
    "FX-SHCLIP-016": ("FX-SHCLIP-013 moved three pixels right: the count is of the whole "
                      "layer, so the same, moved.", case(shift=3, **BOTH), [0, 3]),
    "FX-SHCLIP-017": ("Both clips 50: the black bin is at or above the white bin, so the clips "
                      "do nothing and it is drawn as FX-SHCLIP-001.",
                      case(black_clip=50, white_clip=50), [0]),
}

INVALID = {
    "FX-SHCLIP-018": ("Midtone contrast 101, above 100.", case(midtone_contrast=101)),
    "FX-SHCLIP-019": ("Midtone contrast -101, below -100.", case(midtone_contrast=-101)),
    "FX-SHCLIP-020": ("Black clip -1, below 0.", case(black_clip=-1)),
    "FX-SHCLIP-021": ("White clip 51, above 50.", case(white_clip=51)),
}


def main():
    expected = B.write_cases(OUT, "shadow_highlight_clip", CASES, INVALID, render, plain,
                             file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    old = json.loads((SH.OUT / "expected_shadow_highlight.json").read_text(encoding="utf-8"))
    near = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    luma = lambda p: 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2]  # noqa: E731
    px = B.pixels()
    covered = [i for i in range(W * H) if px[i][3]]
    drawn = plain(case())

    one = c["FX-SHCLIP-001"]["0"]
    assert like(one, old["cases"]["FX-SHHI-001"]["frames"]["0"])
    assert "midtone_contrast" not in json.dumps(
        json.loads((OUT / "fx_shclip_001.json").read_text(encoding="utf-8")))
    for fx in ("FX-SHCLIP-002", "FX-SHCLIP-012", "FX-SHCLIP-017"):
        assert like(c[fx]["0"], one), fx
    # Black and white stay under midtone contrast.
    for fx in ("FX-SHCLIP-003", "FX-SHCLIP-004", "FX-SHCLIP-005", "FX-SHCLIP-006"):
        assert near(c[fx]["0"][at(1, 0)], drawn[at(1, 0)], 1e-9)
        assert near(c[fx]["0"][at(2, 0)], drawn[at(2, 0)], 1e-9)
    # Midtone contrast spreads lightness from the middle, or draws it in; the colour held inside
    # 0 to 1 can move a pure colour's lightness a little (red at -50: 4e-4), hence 1e-3.
    mid = lambda p: SH.lab([v / p[3] for v in p[:3]])[0] if p[3] else 0.5  # noqa: E731
    for i in covered:
        l1 = mid(one[i])
        for more, less in (("FX-SHCLIP-003", "FX-SHCLIP-005"), ("FX-SHCLIP-004", "FX-SHCLIP-006")):
            lm, ll = mid(c[more]["0"][i]), mid(c[less]["0"][i])
            if l1 > 0.5 + 1e-6:
                assert lm >= l1 - 1e-3 and ll <= l1 + 1e-3
            elif l1 < 0.5 - 1e-6:
                assert lm <= l1 + 1e-3 and ll >= l1 - 1e-3
        assert abs(mid(c["FX-SHCLIP-004"]["0"][i]) - 0.5) >= abs(mid(c["FX-SHCLIP-003"]["0"][i]) - 0.5) - 1e-3
    assert not like(c["FX-SHCLIP-007"]["0"], drawn)
    # The clips push more of the picture to black, or to white.
    blacks = lambda f: sum(1 for i in covered if luma(f[i]) < 1e-9)  # noqa: E731
    whites = lambda f: sum(1 for i in covered if luma(f[i]) > f[i][3] - 1e-9)  # noqa: E731
    assert blacks(c["FX-SHCLIP-008"]["0"]) > blacks(one)
    assert whites(c["FX-SHCLIP-009"]["0"]) > whites(one)
    ten = c["FX-SHCLIP-010"]["0"]
    assert blacks(ten) > blacks(one) and whites(ten) > whites(one)
    assert not like(c["FX-SHCLIP-011"]["0"], drawn)
    for i in covered:
        assert luma(c["FX-SHCLIP-008"]["0"][i]) <= luma(one[i]) + 1e-9
        assert luma(c["FX-SHCLIP-009"]["0"][i]) >= luma(one[i]) - 1e-9
    k14 = c["FX-SHCLIP-014"]
    assert like(k14["0"], c["FX-SHCLIP-006"]["0"]) and like(k14["2"], c["FX-SHCLIP-002"]["0"])
    assert like(k14["4"], c["FX-SHCLIP-004"]["0"])
    k15 = c["FX-SHCLIP-015"]
    assert like(k15["0"], c["FX-SHCLIP-002"]["0"]) and like(k15["2"], c["FX-SHCLIP-008"]["0"])
    assert blacks(k15["4"]) > blacks(k15["2"])
    moved, thirteen = c["FX-SHCLIP-016"]["0"], c["FX-SHCLIP-013"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == thirteen[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        for fr in frames.values():
            for i, p in enumerate(fr):
                assert p[3] == drawn[i][3] or fx == "FX-SHCLIP-016"
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
