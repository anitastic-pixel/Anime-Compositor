"""D-333's pictures: tutorial 2's glowing block after step F, worked three ways.

A proposal's demonstration, not a fixture. After step F the tutorial is in 32 bpc. Its Lightning
Diff copy keeps the bottom 30% of the bolt (a Linear Wipe at 70%), blurs it with Fast Box Blur
(101.2 across, 121.3 down, 3 passes), puts it on black (Solid Composite), lifts it +20.49 stops
with Exposure, and fades the top again (Linear Wipe 69%, feather 32.2). The bolt is this
program's own frame 20 of the tutorial 2 replay, over black (`bolt_f20.png`).

- ours: this program today. Float works in linear light, so the blur averages linear light and
  Exposure multiplies it by 2^20.49 (about 1.4 million). The blur's faint edge turns white out to
  its full reach: a block.
- ae_srgb: After Effects' default 32 bpc, which has no linear working space: the blur averages
  display values, and Exposure (CS3 manual p.402: "performing calculations in a linear color
  space") converts with the sRGB curve, multiplies, converts back. Still a block, because the
  sRGB curve is a straight line near black and keeps faint values faint rather than crushing them.
- ae_gamma22: the same, but Exposure converts with a plain 2.2 power curve, which crushes faint
  values (a value of 0.001 becomes 0.0000003). Only a soft pool lights, as in the tutorial.

Which curve After Effects uses is not stated in any source found; that is the open question.

    python tools/d333_ae_32bpc_demo.py
"""
import sys
from pathlib import Path

import numpy as np
from PIL import Image

sys.path.insert(0, str(Path(__file__).resolve().parent))
from d330_eight_bit_demo import fast_box, linear_to_srgb, srgb_to_linear  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "verification" / "D-333 pictures"


def wipe(h, completion, feather):
    """Linear Wipe at 180 degrees: transparent above `completion` of the height, a ramp of `feather`."""
    y = np.arange(h)[:, None, None] + 0.5
    edge = completion / 100 * h
    if feather <= 0:
        return (y >= edge).astype(float)
    return np.clip((y - edge) / feather + 0.5, 0, 1)


def main():
    bolt = np.asarray(Image.open(OUT / "bolt_f20.png").convert("RGB")) / 255.0  # display values, over black
    h = bolt.shape[0]
    tips = bolt * wipe(h, 70, 0)
    gain = 2.0 ** 20.49
    blurred_display = fast_box(fast_box(tips, 101.2, 1), 121.3, 0)
    blurred_linear = fast_box(fast_box(srgb_to_linear(tips), 101.2, 1), 121.3, 0)
    ways = {
        "ours": blurred_linear * gain,
        "ae_srgb": srgb_to_linear(blurred_display) * gain,
        "ae_gamma22": np.clip(blurred_display, 0, None) ** 2.2 * gain,
    }
    fade = wipe(h, 69, 32.2)
    tiles, lit = [], {}
    for name, light in ways.items():
        light = light * fade
        shown = (linear_to_srgb(np.clip(light, 0, 1)) * 255).round().astype(np.uint8)
        Image.fromarray(shown).save(OUT / f"diff_{name}.png")
        lit[name] = int((light.max(axis=2) >= 0.25).sum())
        print(f"{name}: {lit[name]} pixels brighter than a quarter")
        tiles.append(shown[600:1080, 300:1300])
    Image.fromarray(np.concatenate(tiles, axis=1)).save(OUT / "sheet.png")
    # The claims the pictures make: the sRGB curve keeps the block, the 2.2 curve shrinks it.
    assert lit["ae_srgb"] * 2 > lit["ours"]
    assert lit["ae_gamma22"] * 4 < lit["ours"]


if __name__ == "__main__":
    main()
