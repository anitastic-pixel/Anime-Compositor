"""Turbulent Noise's Saturation, Color, Luminosity, None, Stencil Alpha and Stencil Luma, worked a second way.

D-730 adds six blending modes to `core.turbulent_noise` (D-445, D-567, D-647; the noise itself is
`tools/turbulent_more_reference.py`'s, reused unchanged). Turbulent Noise draws black to white,
so the noise is a grey g (linear) at each pixel, and b is the layer's straight colour (linear).

Sources. Adobe's help page on the Noise & Grain effects, read through its search snippets (the
page refuses automated reading): Fractal Noise's blending modes "are identical to the ones in the
Modes column in the Timeline panel", except that None "renders the fractal noise only and does not
composite on the original layer". Turbulent Noise is a second name over Fractal Noise here (D-445).
So:

- `saturation`, `color`, `luminosity`: the W3C Compositing and Blending Level 1 non-separable
  modes, backdrop the layer, source the noise, with its Lum (0.3, 0.59, 0.11), ClipColor, SetLum,
  Sat and SetSat, then mixed in at the opacity as every other mode of this effect is,
  b + o (B - b). This is our basis, not a match to After Effects, whose formula is unpublished.
  With grey noise, Saturation and Color both give the layer's own grey at its own Lum.
- `none`: the noise alone, replacing the layer at every pixel, transparent ones too, as an opaque
  layer at the opacity: (g o, g o, g o, o) premultiplied (D-413's None for Checkerboard).
- `stencil_alpha`, `stencil_luma`: this program's own Timeline modes (D-301, document 21), the
  noise the opaque layer above at the opacity: the layer kept by o, or by the encoded luma of the
  noise's (g o) (its colour over black, encoded). Clear pixels stay clear.
- Opacity 0 with None or a stencil clears the layer rather than leaving it, as D-413's patterns.

Not built: Hue (FX-TURBNOISE-060 pins it refused; the owner's fixture decision), Overlay
(FX-TURBNOISE-032), and Silhouette Alpha and Silhouette Luma (the program has no Timeline rule for
them to follow). Those two are refused here with a warning.
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import turbulent_noise_reference as T  # noqa: E402
import turbulent_more_reference as M  # noqa: E402

W, H = T.W, T.H
OUT = T.OUT
TOLERANCE = 2e-5
FINE = T.FINE


# --- the W3C non-separable modes ------------------------------------------------------------

def lum(c):
    return 0.3 * c[0] + 0.59 * c[1] + 0.11 * c[2]


def clip_color(c):
    l = lum(c)
    n, x = min(c), max(c)
    if n < 0 and l > n:
        c = [l + (v - l) * l / (l - n) for v in c]
    if x > 1 and x > l:
        c = [l + (v - l) * (1 - l) / (x - l) for v in c]
    return c


def set_lum(c, l):
    d = l - lum(c)
    return clip_color([v + d for v in c])


def sat(c):
    return max(c) - min(c)


def set_sat(c, s):
    n, x = min(c), max(c)
    return [(v - n) * s / (x - n) if x > n else 0.0 for v in c]


WHOLE = {
    "saturation": lambda b, s: set_lum(set_sat(b, sat(s)), lum(b)),
    "color": lambda b, s: set_lum(s, lum(b)),
    "luminosity": lambda b, s: set_lum(b, lum(s)),
}


# --- the rule -------------------------------------------------------------------------------

def turbulent_noise(pixels, c, n):
    blend, op = c["blend"], n["opacity"] / 100
    out = []
    for i, p in enumerate(pixels):
        if p[3] == 0 and blend != "none":
            out.append(R.working(p))
            continue
        g = srgb_to_linear(M.value(c, n, i % W, i // W))
        a = p[3] / 255
        dst = R.working(p)
        if blend == "none":
            out.append([g * op] * 3 + [op])
        elif blend == "stencil_alpha":
            out.append([v * op for v in dst])
        elif blend == "stencil_luma":
            k = S.linear_to_srgb(min(1.0, max(0.0, g * op)))
            out.append([v * k for v in dst])
        else:
            b = [srgb_to_linear(p[ch] / 255) for ch in range(3)]
            m = WHOLE[blend](b, [g, g, g])
            out.append([(b[ch] + op * (m[ch] - b[ch])) * a for ch in range(3)] + [a])
    return out


def render(c, frame_no):
    pixels = [p for row in T.DRAWINGS[c["drawing"]] for p in row]
    n = {k: M.held(c, k, frame_no) for k in M.RANGES}
    return R.frame(turbulent_noise(pixels, c, n), c["shift"])


# --- the cases ------------------------------------------------------------------------------

case = M.case

CASES = {
    "FX-TURBNOISE-061": ("Size 4, blending mode saturation: the noise is grey, so the card takes "
                         "its saturation, none, and goes to its own grey.",
                         case(blend="saturation", **FINE), [0]),
    "FX-TURBNOISE-062": ("Size 4, blending mode color: the card's lightness in the noise's "
                         "colour, grey, so the same as FX-TURBNOISE-061.",
                         case(blend="color", **FINE), [0]),
    "FX-TURBNOISE-063": ("Size 4, blending mode luminosity: the card's colours at the clouds' "
                         "lightness.", case(blend="luminosity", **FINE), [0]),
    "FX-TURBNOISE-064": ("Size 4, blending mode luminosity, opacity 50: halfway from the card to "
                         "FX-TURBNOISE-063.",
                         case(blend="luminosity", opacity=50, **FINE), [0]),
    "FX-TURBNOISE-065": ("Size 4, blending mode none: the clouds alone, the card gone, filling "
                         "the clear corner too.", case(blend="none", **FINE), [0]),
    "FX-TURBNOISE-066": ("Size 4, blending mode none, opacity 60: the clouds alone at 60% "
                         "everywhere.", case(blend="none", opacity=60, **FINE), [0]),
    "FX-TURBNOISE-067": ("Size 4, blending mode stencil_alpha: the noise is opaque, so the card "
                         "is kept whole, the plain card.",
                         case(blend="stencil_alpha", **FINE), [0]),
    "FX-TURBNOISE-068": ("Size 4, blending mode stencil_alpha, opacity 40: the card kept at 40%.",
                         case(blend="stencil_alpha", opacity=40, **FINE), [0]),
    "FX-TURBNOISE-069": ("Size 4, blending mode stencil_luma: the card kept where the clouds are "
                         "light and cut away where they are dark.",
                         case(blend="stencil_luma", **FINE), [0]),
    "FX-TURBNOISE-070": ("Size 4, blending mode stencil_luma, opacity 50: the clouds at half, so "
                         "the card is kept less.",
                         case(blend="stencil_luma", opacity=50, **FINE), [0]),
    "FX-TURBNOISE-071": ("Size 4, blending mode none, opacity 0: nothing drawn and the card "
                         "gone, a clear frame, not the card left as it was.",
                         case(blend="none", opacity=0, **FINE), [0]),
    "FX-TURBNOISE-072": ("Size 4, blending mode stencil_luma, opacity keyed from 100 at frame 0 "
                         "to 0 at frame 4, linear: FX-TURBNOISE-069 fading to a clear frame.",
                         case(blend="stencil_luma", opacity=M.keyed((0, 100), (4, 0)), **FINE),
                         [0, 2, 4]),
}

# D-46: the file is read, the effect is kept as written and left out of every frame, with
# EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-TURBNOISE-073": ("Blending mode \"silhouette_alpha\", which is not built.",
                         case(blend="silhouette_alpha")),
    "FX-TURBNOISE-074": ("Blending mode \"silhouette_luma\", which is not built.",
                         case(blend="silhouette_luma")),
}


def main():
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": M.write(fx, c), "frames": rendered}
        before = T.plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = T.plain(c)
        expected["cases"][fx] = {"says": says, "project": M.write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_turbulent_blends.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                        encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    drawn = T.plain(case(**FINE))
    shown = [i for i in range(W * H) if drawn[i][3] > 0]
    clear = [i for i in range(W * H) if drawn[i][3] == 0]
    assert shown and clear
    near = lambda a, b, e=1e-12: all(abs(x - y) <= e for p, q in zip(a, b) for x, y in zip(p, q))  # noqa: E731

    # The W3C helpers on known colours: SetLum keeps the asked lightness, ClipColor holds 0..1.
    for col in ([0.9, 0.1, 0.4], [0.2, 0.8, 0.05], [1.0, 1.0, 0.0]):
        for l in (0.0, 0.25, 0.5, 0.95):
            r = set_lum(col, l)
            assert abs(lum(r) - l) < 1e-12 and all(-1e-12 <= v <= 1 + 1e-12 for v in r)
    assert near([set_sat([0.2, 0.6, 0.4], 0.8)], [[0.0, 0.8, 0.4]])

    # Saturation and Color agree on grey noise: the card's own grey at its own Lum.
    assert near(c["FX-TURBNOISE-061"], c["FX-TURBNOISE-062"])
    for i in shown:
        p = c["FX-TURBNOISE-061"][i]
        assert abs(p[0] - p[1]) < 1e-12 and abs(p[1] - p[2]) < 1e-12
    # Luminosity changes the card; at 50% it is halfway.
    lu, half = c["FX-TURBNOISE-063"], c["FX-TURBNOISE-064"]
    assert lu != drawn
    assert near(half, [[(x + y) / 2 for x, y in zip(p, q)] for p, q in zip(drawn, lu)], 1e-12)
    # None: the noise alone everywhere, grey, opaque; at 60% everything is 0.6 of it.
    none, sixty = c["FX-TURBNOISE-065"], c["FX-TURBNOISE-066"]
    assert all(p[3] == 1.0 and p[0] == p[1] == p[2] for p in none)
    assert near(sixty, [[v * 0.6 for v in p] for p in none], 1e-12)
    assert any(none[i][0] > 0 for i in clear)
    # Stencil Alpha: whole at 100%, 40% at 40%.
    assert c["FX-TURBNOISE-067"] == drawn
    assert near(c["FX-TURBNOISE-068"], [[v * 0.4 for v in p] for p in drawn], 1e-12)
    # Stencil Luma keeps a share of the card set by the clouds, never adds.
    sl = c["FX-TURBNOISE-069"]
    for i in range(W * H):
        k = sl[i][3] / drawn[i][3] if drawn[i][3] else 0.0
        assert 0 <= k <= 1 + 1e-12
        assert near([sl[i]], [[v * k for v in drawn[i]]], 1e-12)
    assert len({round(sl[i][3] / drawn[i][3], 9) for i in shown}) > 5
    assert all(sum(c["FX-TURBNOISE-070"][i]) <= sum(sl[i]) + 1e-12 for i in shown)
    # Opacity 0 with None: a clear frame.
    assert all(p == [0.0] * 4 for p in c["FX-TURBNOISE-071"])
    keyed = expected["cases"]["FX-TURBNOISE-072"]["frames"]
    assert keyed["0"] == sl and all(p == [0.0] * 4 for p in keyed["4"])
    assert keyed["2"] not in (keyed["0"], keyed["4"])
    # Every case but None leaves clear pixels clear; all stay premultiplied.
    for fx, v in expected["cases"].items():
        for px in v["frames"].values():
            for i in range(W * H):
                if drawn[i][3] == 0 and fx not in ("FX-TURBNOISE-065", "FX-TURBNOISE-066"):
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= x <= px[i][3] + 1e-12 for x in px[i][:3]), (fx, i)
    assert len({str(c[f"FX-TURBNOISE-0{k}"]) for k in (61, 63, 65, 67, 68, 69)}) == 6
    print("checked")


if __name__ == "__main__":
    main()
