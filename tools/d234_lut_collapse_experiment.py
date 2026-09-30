"""D-234 experiment: a chain of per-pixel colour effects collapsed into one 3D lookup table.

A proposal, not a rule. Nothing here is used by the build, and no fixture is touched.

The exact chain is Curves, Levels, Hue/Saturation, Brightness/Contrast and Tint in that order,
each worked as its reference in tools/ works it: a pixel that shows is taken to its encoded
straight colour, graded, held inside 0..1 and brought back to linear at its own covering. The
round trip between two effects is the identity on a held value, so the whole chain is one
function of the encoded straight colour (Tint works in linear light inside it, as effects.rs
does). The collapse samples that function on an N by N by N grid and reads it back with
cube_lut_reference's trilinear rule. Both are compared as the viewer shows them: 8-bit sRGB.

The vectorised effects are checked against the reference tools on random colours before any
number is printed (the `check` step), so the numbers are the references' own rule.

    python tools/d234_lut_collapse_experiment.py
"""

import math
import sys
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw
from scipy.interpolate import CubicSpline

TOOLS = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOLS))
import curves_reference as C  # noqa: E402
import levels_reference as L  # noqa: E402
import hue_saturation_reference as HS  # noqa: E402
import brightness_contrast_reference as BC  # noqa: E402
import cube_lut_reference as CL  # noqa: E402
import smooth_reference as S  # noqa: E402
from adjust_reference import srgb_to_linear  # noqa: E402

ROOT = TOOLS.parent
FRAME = ROOT / "verification" / "B-05a_reference_frame.png"
OUT = ROOT / "verification" / "D-234 proposal"
GRIDS = (17, 33, 65)

# Two grades: one an artist might leave on a shot, one pushed hard.
CHAINS = {
    "gentle": dict(
        master=[[0, 0], [64, 52], [192, 205], [255, 255]], red=[[0, 0], [128, 136], [255, 255]],
        levels=(8, 245, 1.1, 0, 255), hue=(8, 15, 0), bc=(0, 10),
        tint=([1.0, 0.6, 0.25], 0.12)),
    "strong": dict(
        master=[[0, 0], [48, 20], [128, 150], [200, 240], [255, 255]],
        red=[[0, 0], [100, 80], [255, 255]], levels=(20, 230, 0.7, 10, 250),
        hue=(40, 60, -10), bc=(10, 40), tint=([0.2, 0.4, 1.0], 0.3)),
}


# --- the exact chain, vectorised --------------------------------------------------------------

def to_lin(e):
    return np.where(e <= 0.04045, e / 12.92, ((e + 0.055) / 1.055) ** 2.4)


def to_enc(c):
    c = np.clip(c, 0, 1)
    return np.where(c <= 0.0031308, 12.92 * c, 1.055 * c ** (1 / 2.4) - 0.055)


def spline(points):
    """The natural cubic spline Curves draws, flat outside its end points."""
    xs, ys = [p[0] for p in points], [p[1] for p in points]
    if len(xs) == 2:
        return lambda x: np.interp(x, xs, ys)
    s = CubicSpline(xs, ys, bc_type="natural")
    return lambda x: s(np.clip(x, xs[0], xs[-1]))


def curves(e, master, red):
    m = spline(master)
    fns = [spline(red), spline([[0, 0], [255, 255]]), spline([[0, 0], [255, 255]])]
    return np.stack([np.clip(np.clip(m(np.clip(fns[i](e[..., i] * 255), 0, 255)), 0, 255)
                             / 255, 0, 1) for i in range(3)], -1)


def levels(e, ib, iw, gamma, ob, ow):
    v = np.clip((e * 255 - ib) / (iw - ib), 0, 1) ** (1 / gamma)
    return np.clip((ob + v * (ow - ob)) / 255, 0, 1)


def hue_saturation(e, hue, sat, light):
    r, g, b = e[..., 0], e[..., 1], e[..., 2]
    mx, mn = e.max(-1), e.min(-1)
    c = mx - mn
    l = (mx + mn) / 2
    safe = np.where(c == 0, 1, c)
    s = np.where(c == 0, 0, c / np.where(c == 0, 1, 1 - np.abs(2 * l - 1)))
    h = np.where(mx == r, 60 * (((g - b) / safe) % 6),
                 np.where(mx == g, 60 * ((b - r) / safe + 2), 60 * ((r - g) / safe + 4)))
    h = np.where(c == 0, 0, h)
    h = (h + hue) % 360
    s = np.clip(s * (1 + sat / 100), 0, 1)
    l = l + (1 - l) * light / 100 if light >= 0 else l * (1 + light / 100)
    C_ = (1 - np.abs(2 * l - 1)) * s
    X = C_ * (1 - np.abs((h / 60) % 2 - 1))
    m = l - C_ / 2
    k = (h // 60).astype(int) % 6
    z = np.zeros_like(C_)
    r1 = np.choose(k, [C_, X, z, z, X, C_])
    g1 = np.choose(k, [X, C_, C_, X, z, z])
    b1 = np.choose(k, [z, z, X, C_, C_, X])
    return np.clip(np.stack([r1 + m, g1 + m, b1 + m], -1), 0, 1)


def brightness_contrast(e, brightness, contrast):
    return np.clip((e - 0.5) * BC.slope(contrast) + 0.5 + brightness / 255, 0, 1)


def tint(e, color, amount):
    lin = to_lin(e)
    return to_enc(lin + (np.array(color) - lin) * amount)


STEPS = {
    "Curves": lambda e, k: curves(e, k["master"], k["red"]),
    "Levels": lambda e, k: levels(e, *k["levels"]),
    "Hue/Saturation": lambda e, k: hue_saturation(e, *k["hue"]),
    "Brightness/Contrast": lambda e, k: brightness_contrast(e, *k["bc"]),
    "Tint": lambda e, k: tint(e, *k["tint"]),
}


def chain(e, k, steps=tuple(STEPS)):
    for name in steps:
        e = STEPS[name](e, k)
    return e


# --- the collapse -----------------------------------------------------------------------------

def build(n, k, steps=tuple(STEPS)):
    """The grid, flat r + g*n + b*n*n as a .cube table is."""
    g = np.linspace(0, 1, n)
    b_, g_, r_ = np.meshgrid(g, g, g, indexing="ij")
    return chain(np.stack([r_, g_, b_], -1).reshape(-1, 3), k, steps)


def lookup(table, n, e):
    """cube_lut_reference.lookup's trilinear rule, vectorised (domain 0..1)."""
    u = np.clip(e, 0, 1) * (n - 1)
    i = np.minimum(np.floor(u), n - 2).astype(int)
    f = u - i
    out = np.zeros_like(e)
    for db in (0, 1):
        for dg in (0, 1):
            for dr in (0, 1):
                w = ((f[..., 0] if dr else 1 - f[..., 0]) * (f[..., 1] if dg else 1 - f[..., 1])
                     * (f[..., 2] if db else 1 - f[..., 2]))
                idx = (i[..., 0] + dr) + (i[..., 1] + dg) * n + (i[..., 2] + db) * n * n
                out += w[..., None] * table[idx]
    return out


# --- the check against the reference tools ---------------------------------------------------

def check():
    rng = np.random.default_rng(234)
    e = rng.random((400, 3))
    e[:20] = np.round(e[:20])            # corners, greys and ties in the hue rule
    e[20:40, 1] = e[20:40, 0]
    for k in CHAINS.values():
        ref = []
        for p in e:
            m = C.spline(k["master"])
            fns = [C.spline(k["red"]), C.spline(C.DEFAULT), C.spline(C.DEFAULT)]
            v = [C.clamp(C.clamp(m(C.clamp(fns[i](p[i] * 255), 0, 255)), 0, 255) / 255, 0, 1)
                 for i in range(3)]
            v = [C.clamp(L.level(x * 255, *k["levels"]) / 255, 0, 1) for x in v]
            v = [C.clamp(x, 0, 1) for x in HS.grade(v, *k["hue"])]
            v = [C.clamp(x, 0, 1) for x in BC.grade(v, *k["bc"])]
            color, t = k["tint"]
            lin = [srgb_to_linear(x) for x in v]
            v = [S.linear_to_srgb(min(1, max(0, x + (color[i] - x) * t)))
                 for i, x in enumerate(lin)]
            ref.append(v)
        got = chain(e, k)
        assert np.abs(got - np.array(ref)).max() < 1e-9, np.abs(got - np.array(ref)).max()
        n = 5
        table = build(n, k)
        cube = (3, n, [0.0] * 3, [1.0] * 3, [list(t) for t in table])
        want = np.array([CL.lookup(cube, list(p)) for p in e[:50]])
        assert np.abs(lookup(table, n, e[:50]) - want).max() < 1e-12


# --- the measure ------------------------------------------------------------------------------

def to8(e):
    return np.round(np.clip(e, 0, 1) * 255).astype(int)


def measure(a8, b8):
    d = np.abs(a8 - b8)
    mse = np.mean(d.astype(float) ** 2)
    return dict(worst=int(d.max()), over1=100 * np.mean(d.max(-1) > 1),
                psnr=math.inf if mse == 0 else 10 * math.log10(255 ** 2 / mse))


def every_colour(k, table, n, steps=tuple(STEPS)):
    """The worst level over all 16,777,216 8-bit colours, in slabs of blue."""
    worst, over1, total = 0, 0, 0
    v = np.arange(256) / 255
    g_, r_ = np.meshgrid(v, v, indexing="ij")
    for b in range(0, 256, 16):
        e = np.stack([np.broadcast_to(r_, (16, 256, 256)), np.broadcast_to(g_, (16, 256, 256)),
                      np.broadcast_to(v[b:b + 16, None, None], (16, 256, 256))], -1)
        d = np.abs(to8(chain(e, k, steps)) - to8(lookup(table, n, e))).max(-1)
        worst, over1, total = max(worst, int(d.max())), over1 + int((d > 1).sum()), total + d.size
    return worst, 100 * over1 / total


def label(img, text):
    d = ImageDraw.Draw(img)
    d.rectangle([0, 0, 8 * len(text) + 12, 22], fill=(0, 0, 0))
    d.text((6, 5), text, fill=(255, 255, 255))
    return img


def picture(name, exact8, approx8, amp):
    diff = np.clip(np.abs(exact8 - approx8) * amp, 0, 255).astype(np.uint8)
    panels = [label(Image.fromarray(x.astype(np.uint8)).resize((960, 540), Image.LANCZOS), t)
              for x, t in ((exact8, "exact chain"), (approx8, name),
                           (diff, f"difference x{amp}"))]
    sheet = Image.new("RGB", (2880, 540))
    for i, p in enumerate(panels):
        sheet.paste(p, (960 * i, 0))
    return sheet


def main():
    check()
    print("check: vectorised chain and lookup agree with the reference tools")
    OUT.mkdir(parents=True, exist_ok=True)
    src = np.asarray(Image.open(FRAME).convert("RGB")).astype(float) / 255
    for cname, k in CHAINS.items():
        exact8 = to8(chain(src, k))
        for n in GRIDS:
            table = build(n, k)
            approx8 = to8(lookup(table, n, src))
            m = measure(exact8, approx8)
            worst_all, over1_all = every_colour(k, table, n)
            print(f"{cname:6} grid {n:2}: frame worst {m['worst']:2} levels, "
                  f"{m['over1']:.4f}% of pixels off by >1, PSNR {m['psnr']:.1f} dB; "
                  f"all 16.7M colours worst {worst_all}, {over1_all:.4f}% off by >1")
            picture(f"{n}-point table", exact8, approx8, 32).save(
                OUT / f"{cname}_grid{n}_side_by_side.png")
    # Which effect the error comes from: each alone through its own table.
    for cname, k in CHAINS.items():
        for name in STEPS:
            row = []
            for n in (33, 65):
                w, o = every_colour(k, build(n, k, (name,)), n, (name,))
                row.append(f"grid {n}: worst {w}, {o:.4f}% off by >1")
            print(f"{cname:6} {name:19} alone, all colours: " + "; ".join(row))
        rest = tuple(s for s in STEPS if s != "Hue/Saturation")
        row = []
        for n in GRIDS:
            w, o = every_colour(k, build(n, k, rest), n, rest)
            row.append(f"grid {n}: worst {w}, {o:.4f}% off by >1")
        print(f"{cname:6} chain without Hue/Saturation, all colours: " + "; ".join(row))
    Image.fromarray((src * 255).round().astype(np.uint8)).resize((960, 540)).save(
        OUT / "untouched_frame.png")


if __name__ == "__main__":
    main()
