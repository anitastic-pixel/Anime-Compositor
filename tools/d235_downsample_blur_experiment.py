"""D-235 experiment: large Gaussian Blur, Glow and Bloom worked at a reduced size and enlarged.

A proposal, not a rule. Nothing here is used by the build, and no fixture is touched.

The exact blur is document 21's: weights exp(-d^2 / 2 sigma^2) cut at ceil(3 sigma) and
normalised, outside the layer transparent, on premultiplied linear values. It is summed here by
FFT, which equals the direct sum to about 1e-12 (checked against adjust_reference.blur first).
Gaussian Blur's setting is sigma itself; Glow's and Bloom's radius is 3 sigma, and Bloom
averages four blurs at sigma * 1, 1/2, 1/4, 1/8 (their references in tools/).

The shortcut, for a factor f: average each f by f block into one pixel, blur that smaller
picture at a smaller sigma, and enlarge it back by bilinear sampling from pixel centres. The
shrinking and the enlarging blur a little themselves, (f^2 - 1) / 12 + f^2 / 6 of variance,
so the small blur's sigma is taken as sqrt(sigma^2 - that) / f. A blur whose small sigma would
fall under MIN_SMALL_SIGMA is worked at a smaller factor (halved until it holds), down to exact:
the "guarded" run. The "plain" run (MIN_SMALL_SIGMA 0) always uses the factor asked for; the guarded one keeps it at GUARD.

Both are compared as the viewer shows them: the frame over black, 8-bit sRGB.

    python tools/d235_downsample_blur_experiment.py
"""

import math
import sys
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw
from scipy.signal import fftconvolve

TOOLS = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOLS))
import adjust_reference as A  # noqa: E402

ROOT = TOOLS.parent
FRAME = ROOT / "verification" / "B-05a_reference_frame.png"
OUT = ROOT / "verification" / "D-235 proposal"
RADII = (20, 50, 100, 200)
FACTORS = (2, 4, 8)
MIN_SMALL_SIGMA = 0.0  # set by main: 0 for the plain run, GUARD for the guarded one
GUARD = 6.0  # the smallest tried that kept every case here within 1 level (2 and 4 did not)
GLOW_THRESHOLD, BLOOM_THRESHOLD = 60, 80  # the effects' own defaults, intensity 1


# --- the exact rule ---------------------------------------------------------------------------

def weights(sigma):
    r = math.ceil(3 * sigma) if sigma > 0 else 0
    if not r:
        return np.ones(1)
    one = np.exp(-np.arange(-r, r + 1) ** 2 / (2 * sigma * sigma))
    return one / one.sum()


def blur(img, sigma):
    """Separable, zero outside, the same size as `img` (H, W, 4)."""
    k = weights(sigma)
    if k.size == 1:
        return img.copy()
    out = fftconvolve(img, k[None, :, None], mode="same", axes=1)
    return fftconvolve(out, k[:, None, None], mode="same", axes=0)


# --- the shortcut -----------------------------------------------------------------------------

def factor_for(sigma, f):
    """The largest factor up to f that keeps the small sigma at MIN_SMALL_SIGMA or more."""
    while f > 1 and small_sigma(sigma, f) < MIN_SMALL_SIGMA:
        f //= 2
    return f


def small_sigma(sigma, f):
    extra = (f * f - 1) / 12 + f * f / 6
    return math.sqrt(max(0.0, sigma * sigma - extra)) / f


def shortcut(img, sigma, f):
    f = factor_for(sigma, f)
    if f == 1:
        return blur(img, sigma)
    h, w = img.shape[:2]
    pad = math.ceil(3 * sigma / f + 2) * f      # room for the blur to spread past the frame
    big = np.zeros((h + 2 * pad, w + 2 * pad, 4))
    big[pad:pad + h, pad:pad + w] = img
    H, W = big.shape[0] // f, big.shape[1] // f
    small = big[:H * f, :W * f].reshape(H, f, W, f, 4).mean((1, 3))
    small = blur(small, small_sigma(sigma, f))
    # Bilinear from pixel centres: full pixel x sits at (x + 0.5) / f - 0.5 in the small one.
    ys = (np.arange(h) + pad + 0.5) / f - 0.5
    xs = (np.arange(w) + pad + 0.5) / f - 0.5
    y0, x0 = np.floor(ys).astype(int), np.floor(xs).astype(int)
    fy, fx = (ys - y0)[:, None, None], (xs - x0)[None, :, None]
    a, b = small[y0][:, x0], small[y0][:, x0 + 1]
    c, d = small[y0 + 1][:, x0], small[y0 + 1][:, x0 + 1]
    return (a * (1 - fx) + b * fx) * (1 - fy) + (c * (1 - fx) + d * fx) * fy


# --- the three effects ------------------------------------------------------------------------

def light(px8, work, threshold):
    """Glow's bright test: the brightest 8-bit channel at least threshold% of 255."""
    lit = (px8[..., 3] > 0) & (px8[..., :3].max(-1) * 100 >= threshold * 255)
    return work * lit[..., None]


def gaussian(px8, work, size, fn):
    return fn(work, size)


def glow(px8, work, size, fn):
    g = fn(light(px8, work, GLOW_THRESHOLD), size / 3)
    return added(work, g)


def bloom(px8, work, size, fn):
    src = light(px8, work, BLOOM_THRESHOLD)
    halo = sum(fn(src, size / 3 * s) for s in (1, 0.5, 0.25, 0.125)) / 4
    return added(work, halo)


def added(o, g):
    out = o + g
    out[..., 3] = np.minimum(1, o[..., 3] + g[..., 3])
    return out


EFFECTS = {"Gaussian Blur": (gaussian, "sigma"), "Glow": (glow, "radius"),
           "Bloom": (bloom, "radius")}


# --- the measure ------------------------------------------------------------------------------

def shown(work):
    """Over black, as the viewer shows it: premultiplied linear, held, to 8-bit sRGB."""
    c = np.clip(work[..., :3], 0, 1)
    e = np.where(c <= 0.0031308, 12.92 * c, 1.055 * c ** (1 / 2.4) - 0.055)
    return np.round(e * 255).astype(int)


def measure(a8, b8):
    d = np.abs(a8 - b8)
    mse = np.mean(d.astype(float) ** 2)
    return (int(d.max()), 100 * np.mean(d.max(-1) > 1),
            math.inf if mse == 0 else 10 * math.log10(255 ** 2 / mse))


def taps(sigma, f, pixels):
    """Pixel operations, as multiply-adds per channel: an estimate, not a timing."""
    exact = pixels * 2 * (2 * math.ceil(3 * sigma) + 1)
    f = factor_for(sigma, f)
    if f == 1:
        return exact, exact
    s = small_sigma(sigma, f)
    # shrink: each pixel read once; small blur: pixels / f^2 of it; enlarge: 4 taps a pixel.
    return exact, pixels + pixels / (f * f) * 2 * (2 * math.ceil(3 * s) + 1) + 4 * pixels


def check():
    """The FFT blur is document 21's sum: adjust_reference.blur on its 16 by 10 frame."""
    rng = np.random.default_rng(235)
    img = rng.random((A.H, A.W, 4))
    for sigma in (0.7, 2.5):
        want = np.array(A.blur([list(p) for p in img.reshape(-1, 4)], sigma)).reshape(img.shape)
        assert np.abs(blur(img, sigma) - want).max() < 1e-12


def label(img, text):
    d = ImageDraw.Draw(img)
    d.rectangle([0, 0, 7 * len(text) + 12, 22], fill=(0, 0, 0))
    d.text((6, 5), text, fill=(255, 255, 255))
    return img


def picture(exact8, approx8, name, amp=16):
    diff = np.clip(np.abs(exact8 - approx8) * amp, 0, 255).astype(np.uint8)
    sheet = Image.new("RGB", (2880, 540))
    for i, (x, t) in enumerate(((exact8, "exact"), (approx8, name),
                                (diff, f"difference x{amp}"))):
        p = Image.fromarray(x.astype(np.uint8)).resize((960, 540), Image.LANCZOS)
        sheet.paste(label(p, t), (960 * i, 0))
    return sheet


def main():
    check()
    print("check: the FFT blur agrees with adjust_reference.blur")
    OUT.mkdir(parents=True, exist_ok=True)
    px8 = np.asarray(Image.open(FRAME).convert("RGBA")).astype(float)
    a = px8[..., 3:] / 255
    work = np.concatenate([A_lin(px8[..., :3] / 255) * a, a], -1)
    pixels = work.shape[0] * work.shape[1]
    print(f"frame {FRAME.name}, {work.shape[1]} by {work.shape[0]}")
    exact = {}
    global MIN_SMALL_SIGMA
    for MIN_SMALL_SIGMA, run in ((0.0, "plain"), (GUARD, "guarded")):
        print(f"--- {run}: small sigma kept at {MIN_SMALL_SIGMA} or more")
        each(px8, work, pixels, exact, run)


def each(px8, work, pixels, exact, run):
    for ename, (fx, unit) in EFFECTS.items():
        for size in RADII:
            if (ename, size) not in exact:
                exact[ename, size] = shown(fx(px8, work, size, blur))
            exact8 = exact[ename, size]
            for f in FACTORS:
                approx8 = shown(fx(px8, work, size, lambda im, s: shortcut(im, s, f)))
                worst, over1, psnr = measure(exact8, approx8)
                sigmas = [size] if unit == "sigma" else \
                    [size / 3] if ename == "Glow" else [size / 3 * s for s in (1, .5, .25, .125)]
                ops = [taps(s, f, pixels) for s in sigmas]
                speed = sum(e for e, _ in ops) / sum(s for _, s in ops)
                used = "/".join(str(factor_for(s, f)) for s in sigmas)
                print(f"{ename:13} {unit} {size:3} factor {f} (used {used:>7}): worst {worst:2} "
                      f"levels, {over1:7.4f}% off by >1, PSNR {psnr:5.1f} dB, "
                      f"estimated {speed:5.1f}x fewer operations")
                if (size, f) in PICTURES[run]:
                    picture(exact8, approx8, f"{ename} {unit} {size} at 1/{f} size, {run}").save(
                        OUT / f"{ename.split()[0].lower()}_{size}_factor{f}_{run}.png")


PICTURES = {"plain": ((20, 8), (50, 2), (100, 4), (200, 8)), "guarded": ((200, 8),)}


def A_lin(e):
    return np.where(e <= 0.04045, e / 12.92, ((e + 0.055) / 1.055) ** 2.4)


if __name__ == "__main__":
    main()
