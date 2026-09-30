# B-156: adjustment layers on the GPU against the CPU

Written by `tests/b156_gpu_adjust.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

An adjustment layer runs its effects on the whole frame drawn beneath it and mixes the result back by what it covers (D-66). Each of the 64 effects the card can draw is put alone on an adjustment layer over the reference shot. Then runs of three: over the whole frame, over part of it (scaled, turned and at 60% opacity, so its edges cover part of a pixel), and between the second and third layers, so the two above it are drawn onto the adjusted frame. Bloom, Glow, Paraffin and Kira-kira look at their drawing before the card is asked, and an HSV Key only begins a run (D-224), so an adjustment layer with one stays the CPU's; so does one with a Kaleidoscope, which the card does not draw (D-240).

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU. **The rule: drawn where the table says; on the card no channel of any pixel more than 1 level of 255 apart; kept on the CPU the CPU's frame byte for byte**; the same warnings on both.

**32 of 308 checks pass.**

The worst comparison drawn on the card is "Radial Blur alone, over the whole frame frame 0, Full": largest difference 0 of 255, pixels differing: 0. Its pictures are in `verification/B-156 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## By kind

| Kind | Frames compared | Largest difference (of 255) | Pass |
|---|---:|---:|---|
| a run of three the card draws | 20 | 0 | 0 of 20 |
| kept on the CPU | 32 | 0 | 32 of 32 |
| one effect the card draws | 256 | 0 | 0 of 256 |

## Every frame

| Case | Must be drawn on | Drawn on | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---|---|---:|---:|---|---|
| Radial Blur alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Radial Blur alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Radial Blur alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Radial Blur alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Bloom alone, over the whole frame frame 0, Full | CPU | CPU | 0 | 0 | none | PASS |
| Bloom alone, over the whole frame frame 100, Full | CPU | CPU | 0 | 0 | none | PASS |
| Bloom alone, over the whole frame frame 0, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Bloom alone, over the whole frame frame 100, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Directional Blur alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Directional Blur alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Directional Blur alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Directional Blur alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Gaussian Blur alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Gaussian Blur alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Gaussian Blur alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Gaussian Blur alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Glow alone, over the whole frame frame 0, Full | CPU | CPU | 0 | 0 | none | PASS |
| Glow alone, over the whole frame frame 100, Full | CPU | CPU | 0 | 0 | none | PASS |
| Glow alone, over the whole frame frame 0, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Glow alone, over the whole frame frame 100, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Curves alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Curves alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Curves alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Curves alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Hue/Saturation alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Hue/Saturation alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Hue/Saturation alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Hue/Saturation alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Gradient alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Gradient alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Gradient alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Gradient alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Drop Shadow alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Drop Shadow alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Drop Shadow alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Drop Shadow alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Lens Blur alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Lens Blur alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Lens Blur alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Lens Blur alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Rim Light alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Rim Light alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Rim Light alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Rim Light alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Outline alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Outline alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Outline alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Outline alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Noise alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Noise alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Noise alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Noise alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Chromatic Aberration alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Chromatic Aberration alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Chromatic Aberration alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Chromatic Aberration alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Distance Gradation alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Distance Gradation alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Distance Gradation alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Distance Gradation alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Light Rays alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Light Rays alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Light Rays alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Light Rays alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Exposure Flicker alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Exposure Flicker alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Exposure Flicker alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Exposure Flicker alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Vignette alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Vignette alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Vignette alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Vignette alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Turbulent Displace alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Turbulent Displace alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Turbulent Displace alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Turbulent Displace alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Fractal Noise alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Fractal Noise alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Fractal Noise alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Fractal Noise alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Gradient Map alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Gradient Map alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Gradient Map alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Gradient Map alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Color Balance alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Color Balance alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Color Balance alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Color Balance alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Offset alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Offset alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Offset alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Offset alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Invert alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Invert alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Invert alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Invert alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Brightness & Contrast alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Brightness & Contrast alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Brightness & Contrast alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Brightness & Contrast alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Black & White alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Black & White alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Black & White alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Black & White alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Posterize alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Posterize alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Posterize alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Posterize alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Threshold alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Threshold alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Threshold alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Threshold alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Channel Mixer alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Channel Mixer alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Channel Mixer alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Channel Mixer alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Vibrance alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Vibrance alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Vibrance alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Vibrance alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Leave Color alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Leave Color alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Leave Color alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Leave Color alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Solarize alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Solarize alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Solarize alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Solarize alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Halftone alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Halftone alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Halftone alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Halftone alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Mosaic alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Mosaic alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Mosaic alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Mosaic alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Emboss alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Emboss alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Emboss alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Emboss alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Find Edges alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Find Edges alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Find Edges alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Find Edges alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Sharpen alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Sharpen alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Sharpen alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Sharpen alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Diffusion alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Diffusion alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Diffusion alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Diffusion alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Wave Warp alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Wave Warp alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Wave Warp alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Wave Warp alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Ripple alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Ripple alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Ripple alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Ripple alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Twirl alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Twirl alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Twirl alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Twirl alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Bulge alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Bulge alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Bulge alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Bulge alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Mirror alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Mirror alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Mirror alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Mirror alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Linear Wipe alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Linear Wipe alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Linear Wipe alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Linear Wipe alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Radial Wipe alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Radial Wipe alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Radial Wipe alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Radial Wipe alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Venetian Blinds alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Venetian Blinds alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Venetian Blinds alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Venetian Blinds alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Iris Wipe alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Iris Wipe alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Iris Wipe alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Iris Wipe alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Simple Choker alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Simple Choker alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Simple Choker alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Simple Choker alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Speed Lines alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Speed Lines alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Speed Lines alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Speed Lines alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Cross Glare alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Cross Glare alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Cross Glare alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Cross Glare alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Camera Shake alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Camera Shake alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Camera Shake alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Camera Shake alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Rain alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Rain alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Rain alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Rain alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Motion Tile alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Motion Tile alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Motion Tile alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Motion Tile alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Color Lookup alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Color Lookup alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Color Lookup alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Color Lookup alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Line Blur alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Line Blur alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Line Blur alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Line Blur alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| HSV Key alone, over the whole frame frame 0, Full | CPU | CPU | 0 | 0 | none | PASS |
| HSV Key alone, over the whole frame frame 100, Full | CPU | CPU | 0 | 0 | none | PASS |
| HSV Key alone, over the whole frame frame 0, Draft | CPU | CPU | 0 | 0 | none | PASS |
| HSV Key alone, over the whole frame frame 100, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Paraffin alone, over the whole frame frame 0, Full | CPU | CPU | 0 | 0 | none | PASS |
| Paraffin alone, over the whole frame frame 100, Full | CPU | CPU | 0 | 0 | none | PASS |
| Paraffin alone, over the whole frame frame 0, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Paraffin alone, over the whole frame frame 100, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Kira-kira alone, over the whole frame frame 0, Full | CPU | CPU | 0 | 0 | none | PASS |
| Kira-kira alone, over the whole frame frame 100, Full | CPU | CPU | 0 | 0 | none | PASS |
| Kira-kira alone, over the whole frame frame 0, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Kira-kira alone, over the whole frame frame 100, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Median alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Median alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Median alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Median alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Smart Blur alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Smart Blur alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Smart Blur alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Smart Blur alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Roughen Edges alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Roughen Edges alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Roughen Edges alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Roughen Edges alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Radial Shadow alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Radial Shadow alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Radial Shadow alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Radial Shadow alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Bevel Alpha alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Bevel Alpha alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Bevel Alpha alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Bevel Alpha alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Snowfall alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Snowfall alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Snowfall alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Snowfall alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Cell Pattern alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Cell Pattern alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Cell Pattern alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Cell Pattern alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Polar Coordinates alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Polar Coordinates alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Polar Coordinates alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Polar Coordinates alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Optics Compensation alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Optics Compensation alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Optics Compensation alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Optics Compensation alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Corner Pin alone, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Corner Pin alone, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Corner Pin alone, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Corner Pin alone, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels, Gaussian Blur and Hue/Saturation, over the whole frame frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels, Gaussian Blur and Hue/Saturation, over the whole frame frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels, Gaussian Blur and Hue/Saturation, over the whole frame frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels, Gaussian Blur and Hue/Saturation, over the whole frame frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels, Gaussian Blur and Hue/Saturation, over part of it frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels, Gaussian Blur and Hue/Saturation, over part of it frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels, Gaussian Blur and Hue/Saturation, over part of it frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels, Gaussian Blur and Hue/Saturation, over part of it frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels, Gaussian Blur and Hue/Saturation, between the second and third layers frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels, Gaussian Blur and Hue/Saturation, between the second and third layers frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels, Gaussian Blur and Hue/Saturation, between the second and third layers frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Levels, Gaussian Blur and Hue/Saturation, between the second and third layers frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Camera Shake, Motion Tile and Corner Pin, which grow by the frame's size, over part of it frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Camera Shake, Motion Tile and Corner Pin, which grow by the frame's size, over part of it frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Camera Shake, Motion Tile and Corner Pin, which grow by the frame's size, over part of it frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Camera Shake, Motion Tile and Corner Pin, which grow by the frame's size, over part of it frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Directional Blur, Curves and a moving Noise, between the second and third layers frame 0, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Directional Blur, Curves and a moving Noise, between the second and third layers frame 100, Full | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Directional Blur, Curves and a moving Noise, between the second and third layers frame 0, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Directional Blur, Curves and a moving Noise, between the second and third layers frame 100, Draft | GPU | CPU | 0 | 0 | none | FAIL: the CPU drew it |
| Kaleidoscope alone, over the whole frame frame 0, Full | CPU | CPU | 0 | 0 | none | PASS |
| Kaleidoscope alone, over the whole frame frame 100, Full | CPU | CPU | 0 | 0 | none | PASS |
| Kaleidoscope alone, over the whole frame frame 0, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Kaleidoscope alone, over the whole frame frame 100, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Levels, then Kaleidoscope, then Curves, over part of it frame 0, Full | CPU | CPU | 0 | 0 | none | PASS |
| Levels, then Kaleidoscope, then Curves, over part of it frame 100, Full | CPU | CPU | 0 | 0 | none | PASS |
| Levels, then Kaleidoscope, then Curves, over part of it frame 0, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Levels, then Kaleidoscope, then Curves, over part of it frame 100, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Levels, then a Glow, over part of it frame 0, Full | CPU | CPU | 0 | 0 | none | PASS |
| Levels, then a Glow, over part of it frame 100, Full | CPU | CPU | 0 | 0 | none | PASS |
| Levels, then a Glow, over part of it frame 0, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Levels, then a Glow, over part of it frame 100, Draft | CPU | CPU | 0 | 0 | none | PASS |
