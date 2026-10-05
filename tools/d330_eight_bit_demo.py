"""D-330's pictures: why tutorial 2's reflections come out as a big block here.

A proposal's demonstration, not a fixture. Tutorial 2, step E, is drawn in 8 bpc. Its Lightning
Diff copy keeps only the bolt's tips (a 70% Linear Wipe), lays Fast Box Blur on them (horizontal
75.2, vertical 87.3, 3 passes) and then Exposure +17.33. Three tips, 9 by 6 pixels, in the
tutorial's blue #096bf1, worked three ways:

- ours: this program today. The blur works on linear light and nothing is rounded between
  effects, so the blur's faintest edge, times 2^17.33, lights a block 600 pixels wide.
- ae_8bpc: as After Effects is believed to work in an 8 bpc project: the blur on display
  (sRGB-encoded) values, the result rounded to 8 bits after each effect, and Exposure worked in
  linear light (the CS3 manual, p.402). The faint edge rounds to 0 and only a small spot lights.
- ae_32bpc: the same without the rounding, to show the rounding is what makes the difference.

Lightning Spec 2 (6.3, 34.3, Exposure +9.33) is drawn the same three ways below it.

    python tools/d330_eight_bit_demo.py
"""
from pathlib import Path

import numpy as np
from PIL import Image

OUT = Path(__file__).resolve().parent.parent / "verification" / "D-330 pictures"
W, H = 1920, 1080


def srgb_to_linear(e):
    e = np.clip(e, 0, None)
    return np.where(e <= 0.04045, e / 12.92, ((e + 0.055) / 1.055) ** 2.4)


def linear_to_srgb(v):
    v = np.clip(v, 0, None)
    return np.where(v <= 0.0031308, v * 12.92, 1.055 * v ** (1 / 2.4) - 0.055)


def box_pass(a, r, axis):
    """One box of radius r (D-327's: the part past whole weights the next pixel each side)."""
    k, f = int(np.floor(r)), r - np.floor(r)
    a = np.moveaxis(a, axis, -1)
    pad = k + 1
    p = np.pad(a, [(0, 0)] * (a.ndim - 1) + [(pad, pad)])
    c = np.concatenate([np.zeros(a.shape[:-1] + (1,)), np.cumsum(p, axis=-1)], axis=-1)
    i = np.arange(a.shape[-1]) + pad
    s = c[..., i + k + 1] - c[..., i - k] + f * (p[..., i - k - 1] + p[..., i + k + 1])
    return np.moveaxis(s / (2 * k + 1 + 2 * f), -1, axis)


def fast_box(a, r, axis, rounded=False):
    for _ in range(3):
        a = box_pass(a, r, axis)
    return q8(a) if rounded else a


def q8(a):
    return np.round(np.clip(a, 0, 1) * 255) / 255


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    blue = np.array([0x09, 0x6B, 0xF1]) / 255.0
    tips = np.zeros((H, W))
    for x in (520, 600, 680):
        tips[756:762, x - 4:x + 5] = 1.0
    tiles, counts = [], {}
    for name, (h, v, stops) in {"diff": (75.2, 87.3, 17.33), "spec": (6.3, 34.3, 9.33)}.items():
        gain = 2.0 ** stops
        enc = tips[..., None] * blue
        ways = {
            "ours": fast_box(fast_box(tips[..., None] * srgb_to_linear(blue), h, 1), v, 0) * gain,
            "ae_8bpc": srgb_to_linear(fast_box(fast_box(enc, h, 1, True), v, 0, True)) * gain,
            "ae_32bpc": srgb_to_linear(fast_box(fast_box(enc, h, 1), v, 0)) * gain,
        }
        for way, light in ways.items():
            shown = (linear_to_srgb(np.clip(light, 0, 1)) * 255).round().astype(np.uint8)[600:1080, 200:1000]
            Image.fromarray(shown).save(OUT / f"{name}_{way}.png")
            tiles.append(shown)
            counts[(name, way)] = int((light.max(axis=2) >= 1 / 255).sum())
            print(f"{name} {way}: {counts[(name, way)]} pixels show")
    sheet = np.concatenate([np.concatenate(tiles[:3], axis=1), np.concatenate(tiles[3:], axis=1)], axis=0)
    Image.fromarray(sheet).save(OUT / "sheet.png")
    # The claim the pictures make: rounding to 8 bits shrinks the Diff glow to a small part.
    assert counts[("diff", "ae_8bpc")] * 10 < counts[("diff", "ours")]
    assert counts[("diff", "ae_32bpc")] * 2 > counts[("diff", "ours")]


if __name__ == "__main__":
    main()
