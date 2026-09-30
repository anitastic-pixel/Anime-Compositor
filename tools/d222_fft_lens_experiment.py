"""Would a single-precision FFT Lens Blur stay within 1 level of the exact one?
A 1024x640 frame: an opaque cel shape with detail and a few highlights on transparency,
blurred by a disc iris, the exact answer in double precision against scipy's float32 FFT.
Compared as the page receives it: 8-bit straight sRGB, every channel."""
import numpy as np
import scipy.fft as F

rng = np.random.default_rng(3)
H, W = 640, 1024


def srgb8(p):
    a = p[..., 3]
    c = np.where(a[..., None] > 0, p[..., :3] / np.maximum(a, 1e-300)[..., None], 0)
    c = np.clip(c, 0, 1)
    s = np.where(c <= 0.0031308, 12.92 * c, 1.055 * c ** (1 / 2.4) - 0.055)
    return np.concatenate([np.round(s * 255), np.round(np.clip(a, 0, 1) * 255)[..., None]], -1).astype(int)


img = np.zeros((H, W, 4))
yy, xx = np.mgrid[:H, :W]
shape = ((xx - 500) ** 2 / 300 ** 2 + (yy - 320) ** 2 / 200 ** 2) < 1
img[shape, 3] = 1
img[shape, :3] = rng.random((shape.sum(), 3)) * 0.6
for _ in range(20):  # highlights, gain 3
    x, y = rng.integers(250, 750), rng.integers(150, 490)
    img[y - 2:y + 3, x - 2:x + 3, :3] = 3.0

for R in (25, 100, 200):
    ky, kx = np.mgrid[-R:R + 1, -R:R + 1]
    k = (kx ** 2 + ky ** 2 <= R * R).astype(float)
    k /= k.sum()
    Hp, Wp = F.next_fast_len(H + 2 * R), F.next_fast_len(W + 2 * R)
    Hp, Wp = 1 << (Hp - 1).bit_length(), 1 << (Wp - 1).bit_length()
    out = {}
    for dt in (np.float64, np.float32):
        K = F.rfft2(k.astype(dt), (Hp, Wp))
        res = []
        for c in range(4):
            I = F.rfft2(img[..., c].astype(dt), (Hp, Wp))
            res.append(F.irfft2(I * K, (Hp, Wp))[:H + 2 * R, :W + 2 * R])
        out[dt] = np.stack(res, -1).astype(np.float64)
    ex, f32 = out[np.float64], out[np.float32]
    ex[np.abs(ex) < 1e-12] = 0  # the exact answer's empty pixels are empty
    for label, a in (("raw", f32), ("floor 1e-5", np.where(np.abs(f32[..., 3:]) < 1e-5, 0, f32))):
        d = np.abs(srgb8(a) - srgb8(ex)).max(-1)
        vis = srgb8(ex)[..., 3] > 0
        print(f"R {R} {label}: worst {d.max()} levels, {np.count_nonzero(d > 1)} pixels over 1; "
              f"worst where alpha shows {d[vis].max()}; float error {np.abs(f32 - ex).max():.1e}")

from PIL import Image
out_dir = r"I:/AI Coding/Claude/TotallyNotAfterEffects/verification/B-164 pictures"
for name, a in ((" exact", ex), (" single-precision FFT", f32)):
    s = srgb8(a)
    Image.fromarray(s[..., :3].astype(np.uint8)).save(f"{out_dir}/lens 200 hidden colour{name}.png")
    seen = np.round(s[..., :3] * s[..., 3:] / 255 + 128 * (1 - s[..., 3:] / 255)).astype(np.uint8)
    Image.fromarray(seen).save(f"{out_dir}/lens 200 as seen over grey{name}.png")
