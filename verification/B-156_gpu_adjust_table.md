# B-156: adjustment layers on the GPU against the CPU

Written by `tests/b156_gpu_adjust.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

An adjustment layer runs its effects on the whole frame drawn beneath it and mixes the result back by what it covers (D-66). Each of the 64 effects the card can draw is put alone on an adjustment layer over the reference shot. Then runs of three: over the whole frame, over part of it (scaled, turned and at 60% opacity, so its edges cover part of a pixel), and between the second and third layers, so the two above it are drawn onto the adjusted frame. Bloom, Glow, Paraffin and Kira-kira look at their drawing before the card is asked, and an HSV Key only begins a run (D-224), so an adjustment layer with one stays the CPU's; so does one with a Kaleidoscope, which the card does not draw (D-240).

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU. **The rule: drawn where the table says; on the card no channel of any pixel more than 1 level of 255 apart; kept on the CPU the CPU's frame byte for byte**; the same warnings on both.

**308 of 308 checks pass.**

The worst comparison drawn on the card is "Hue/Saturation alone, over the whole frame frame 100, Full": largest difference 1 of 255, pixels differing: 78242. Its pictures are in `verification/B-156 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## By kind

| Kind | Frames compared | Largest difference (of 255) | Pass |
|---|---:|---:|---|
| a run of three the card draws | 20 | 1 | 20 of 20 |
| kept on the CPU | 32 | 0 | 32 of 32 |
| one effect the card draws | 256 | 1 | 256 of 256 |

## Every frame

| Case | Must be drawn on | Drawn on | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---|---|---:|---:|---|---|
| Radial Blur alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 205 | none | PASS |
| Radial Blur alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 186 | none | PASS |
| Radial Blur alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 7 | none | PASS |
| Radial Blur alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 11 | none | PASS |
| Bloom alone, over the whole frame frame 0, Full | CPU | CPU | 0 | 0 | none | PASS |
| Bloom alone, over the whole frame frame 100, Full | CPU | CPU | 0 | 0 | none | PASS |
| Bloom alone, over the whole frame frame 0, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Bloom alone, over the whole frame frame 100, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Directional Blur alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 27 | none | PASS |
| Directional Blur alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 24 | none | PASS |
| Directional Blur alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 3 | none | PASS |
| Directional Blur alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Gaussian Blur alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 25 | none | PASS |
| Gaussian Blur alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 22 | none | PASS |
| Gaussian Blur alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Gaussian Blur alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 4 | none | PASS |
| Glow alone, over the whole frame frame 0, Full | CPU | CPU | 0 | 0 | none | PASS |
| Glow alone, over the whole frame frame 100, Full | CPU | CPU | 0 | 0 | none | PASS |
| Glow alone, over the whole frame frame 0, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Glow alone, over the whole frame frame 100, Draft | CPU | CPU | 0 | 0 | none | PASS |
| Curves alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Curves alone, over the whole frame frame 100, Full | GPU | GPU | 0 | 0 | none | PASS |
| Curves alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Curves alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Levels alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Levels alone, over the whole frame frame 100, Full | GPU | GPU | 0 | 0 | none | PASS |
| Levels alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 5 | none | PASS |
| Levels alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 5 | none | PASS |
| Hue/Saturation alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 65550 | none | PASS |
| Hue/Saturation alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 78242 | none | PASS |
| Hue/Saturation alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 3842 | none | PASS |
| Hue/Saturation alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 4601 | none | PASS |
| Gradient alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 33 | none | PASS |
| Gradient alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 28 | none | PASS |
| Gradient alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Gradient alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Drop Shadow alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Drop Shadow alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 1 | none | PASS |
| Drop Shadow alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Drop Shadow alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Lens Blur alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 24 | none | PASS |
| Lens Blur alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 26 | none | PASS |
| Lens Blur alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Lens Blur alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Rim Light alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Rim Light alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 3 | none | PASS |
| Rim Light alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Rim Light alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Outline alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Outline alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 1 | none | PASS |
| Outline alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Outline alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Noise alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 30 | none | PASS |
| Noise alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 27 | none | PASS |
| Noise alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 5 | none | PASS |
| Noise alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Chromatic Aberration alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 11 | none | PASS |
| Chromatic Aberration alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 11 | none | PASS |
| Chromatic Aberration alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Chromatic Aberration alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Distance Gradation alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Distance Gradation alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 1 | none | PASS |
| Distance Gradation alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Distance Gradation alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Light Rays alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 115 | none | PASS |
| Light Rays alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 108 | none | PASS |
| Light Rays alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 5 | none | PASS |
| Light Rays alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 4 | none | PASS |
| Exposure Flicker alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Exposure Flicker alone, over the whole frame frame 100, Full | GPU | GPU | 0 | 0 | none | PASS |
| Exposure Flicker alone, over the whole frame frame 0, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Exposure Flicker alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 4 | none | PASS |
| Vignette alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 25 | none | PASS |
| Vignette alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 26 | none | PASS |
| Vignette alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 5 | none | PASS |
| Vignette alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 5 | none | PASS |
| Turbulent Displace alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 23 | none | PASS |
| Turbulent Displace alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 23 | none | PASS |
| Turbulent Displace alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Turbulent Displace alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 3 | none | PASS |
| Fractal Noise alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 38 | none | PASS |
| Fractal Noise alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 38 | none | PASS |
| Fractal Noise alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 6 | none | PASS |
| Fractal Noise alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 3 | none | PASS |
| Gradient Map alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 16 | none | PASS |
| Gradient Map alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 12 | none | PASS |
| Gradient Map alone, over the whole frame frame 0, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Gradient Map alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Color Balance alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 134 | none | PASS |
| Color Balance alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 834 | none | PASS |
| Color Balance alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 3 | none | PASS |
| Color Balance alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 4 | none | PASS |
| Offset alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 6 | none | PASS |
| Offset alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 5 | none | PASS |
| Offset alone, over the whole frame frame 0, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Offset alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Invert alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 1 | none | PASS |
| Invert alone, over the whole frame frame 100, Full | GPU | GPU | 0 | 0 | none | PASS |
| Invert alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 999 | none | PASS |
| Invert alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 753 | none | PASS |
| Brightness & Contrast alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 6254 | none | PASS |
| Brightness & Contrast alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 6386 | none | PASS |
| Brightness & Contrast alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 12 | none | PASS |
| Brightness & Contrast alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 15 | none | PASS |
| Black & White alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 29288 | none | PASS |
| Black & White alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 28726 | none | PASS |
| Black & White alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 289 | none | PASS |
| Black & White alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 266 | none | PASS |
| Posterize alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Posterize alone, over the whole frame frame 100, Full | GPU | GPU | 0 | 0 | none | PASS |
| Posterize alone, over the whole frame frame 0, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Posterize alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Threshold alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Threshold alone, over the whole frame frame 100, Full | GPU | GPU | 0 | 0 | none | PASS |
| Threshold alone, over the whole frame frame 0, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Threshold alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Channel Mixer alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Channel Mixer alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 1 | none | PASS |
| Channel Mixer alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Channel Mixer alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Vibrance alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 3 | none | PASS |
| Vibrance alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 2 | none | PASS |
| Vibrance alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Vibrance alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Leave Color alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 627 | none | PASS |
| Leave Color alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 622 | none | PASS |
| Leave Color alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 17 | none | PASS |
| Leave Color alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 15 | none | PASS |
| Solarize alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Solarize alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 1 | none | PASS |
| Solarize alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Solarize alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Halftone alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Halftone alone, over the whole frame frame 100, Full | GPU | GPU | 0 | 0 | none | PASS |
| Halftone alone, over the whole frame frame 0, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Halftone alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Mosaic alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Mosaic alone, over the whole frame frame 100, Full | GPU | GPU | 0 | 0 | none | PASS |
| Mosaic alone, over the whole frame frame 0, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Mosaic alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Emboss alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 14793 | none | PASS |
| Emboss alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 11795 | none | PASS |
| Emboss alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 285 | none | PASS |
| Emboss alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 346 | none | PASS |
| Find Edges alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 10 | none | PASS |
| Find Edges alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 10 | none | PASS |
| Find Edges alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 4 | none | PASS |
| Find Edges alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Sharpen alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 17 | none | PASS |
| Sharpen alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 17 | none | PASS |
| Sharpen alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 64 | none | PASS |
| Sharpen alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 65 | none | PASS |
| Diffusion alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 24 | none | PASS |
| Diffusion alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 21 | none | PASS |
| Diffusion alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 3 | none | PASS |
| Diffusion alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Wave Warp alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 5 | none | PASS |
| Wave Warp alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 40 | none | PASS |
| Wave Warp alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 7 | none | PASS |
| Wave Warp alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 15 | none | PASS |
| Ripple alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 24 | none | PASS |
| Ripple alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 12 | none | PASS |
| Ripple alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 3 | none | PASS |
| Ripple alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Twirl alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Twirl alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 1 | none | PASS |
| Twirl alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Twirl alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Bulge alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Bulge alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 1 | none | PASS |
| Bulge alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Bulge alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Mirror alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Mirror alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 2 | none | PASS |
| Mirror alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Mirror alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Linear Wipe alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Linear Wipe alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 1 | none | PASS |
| Linear Wipe alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Linear Wipe alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Radial Wipe alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Radial Wipe alone, over the whole frame frame 100, Full | GPU | GPU | 0 | 0 | none | PASS |
| Radial Wipe alone, over the whole frame frame 0, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Radial Wipe alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Venetian Blinds alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Venetian Blinds alone, over the whole frame frame 100, Full | GPU | GPU | 0 | 0 | none | PASS |
| Venetian Blinds alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Venetian Blinds alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Iris Wipe alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Iris Wipe alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 1 | none | PASS |
| Iris Wipe alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Iris Wipe alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Simple Choker alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Simple Choker alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 1 | none | PASS |
| Simple Choker alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Simple Choker alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Speed Lines alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 1 | none | PASS |
| Speed Lines alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 1 | none | PASS |
| Speed Lines alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Speed Lines alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Cross Glare alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 15 | none | PASS |
| Cross Glare alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 11 | none | PASS |
| Cross Glare alone, over the whole frame frame 0, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Cross Glare alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Camera Shake alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 9 | none | PASS |
| Camera Shake alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 13 | none | PASS |
| Camera Shake alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Camera Shake alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 5 | none | PASS |
| Rain alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 1 | none | PASS |
| Rain alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 2 | none | PASS |
| Rain alone, over the whole frame frame 0, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Rain alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Motion Tile alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Motion Tile alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 1 | none | PASS |
| Motion Tile alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Motion Tile alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Color Lookup alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 4 | none | PASS |
| Color Lookup alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 8 | none | PASS |
| Color Lookup alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Color Lookup alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Line Blur alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 23 | none | PASS |
| Line Blur alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 18 | none | PASS |
| Line Blur alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 5 | none | PASS |
| Line Blur alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 5 | none | PASS |
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
| Median alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Median alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 2 | none | PASS |
| Median alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Median alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Smart Blur alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 16 | none | PASS |
| Smart Blur alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 14 | none | PASS |
| Smart Blur alone, over the whole frame frame 0, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Smart Blur alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Roughen Edges alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Roughen Edges alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 1 | none | PASS |
| Roughen Edges alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Roughen Edges alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Radial Shadow alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Radial Shadow alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 1 | none | PASS |
| Radial Shadow alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Radial Shadow alone, over the whole frame frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
| Bevel Alpha alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Bevel Alpha alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 1 | none | PASS |
| Bevel Alpha alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 3 | none | PASS |
| Bevel Alpha alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Snowfall alone, over the whole frame frame 0, Full | GPU | GPU | 0 | 0 | none | PASS |
| Snowfall alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 2 | none | PASS |
| Snowfall alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Snowfall alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Cell Pattern alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 25 | none | PASS |
| Cell Pattern alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 26 | none | PASS |
| Cell Pattern alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 3 | none | PASS |
| Cell Pattern alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Polar Coordinates alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 24 | none | PASS |
| Polar Coordinates alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 19 | none | PASS |
| Polar Coordinates alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 3 | none | PASS |
| Polar Coordinates alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 6 | none | PASS |
| Optics Compensation alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 18 | none | PASS |
| Optics Compensation alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 21 | none | PASS |
| Optics Compensation alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 7 | none | PASS |
| Optics Compensation alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 7 | none | PASS |
| Corner Pin alone, over the whole frame frame 0, Full | GPU | GPU | 1 | 17 | none | PASS |
| Corner Pin alone, over the whole frame frame 100, Full | GPU | GPU | 1 | 17 | none | PASS |
| Corner Pin alone, over the whole frame frame 0, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Corner Pin alone, over the whole frame frame 100, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Levels, Gaussian Blur and Hue/Saturation, over the whole frame frame 0, Full | GPU | GPU | 1 | 289 | none | PASS |
| Levels, Gaussian Blur and Hue/Saturation, over the whole frame frame 100, Full | GPU | GPU | 1 | 65 | none | PASS |
| Levels, Gaussian Blur and Hue/Saturation, over the whole frame frame 0, Draft | GPU | GPU | 1 | 155 | none | PASS |
| Levels, Gaussian Blur and Hue/Saturation, over the whole frame frame 100, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Levels, Gaussian Blur and Hue/Saturation, over part of it frame 0, Full | GPU | GPU | 1 | 18 | none | PASS |
| Levels, Gaussian Blur and Hue/Saturation, over part of it frame 100, Full | GPU | GPU | 1 | 15 | none | PASS |
| Levels, Gaussian Blur and Hue/Saturation, over part of it frame 0, Draft | GPU | GPU | 1 | 3 | none | PASS |
| Levels, Gaussian Blur and Hue/Saturation, over part of it frame 100, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Levels, Gaussian Blur and Hue/Saturation, between the second and third layers frame 0, Full | GPU | GPU | 1 | 21 | none | PASS |
| Levels, Gaussian Blur and Hue/Saturation, between the second and third layers frame 100, Full | GPU | GPU | 1 | 17 | none | PASS |
| Levels, Gaussian Blur and Hue/Saturation, between the second and third layers frame 0, Draft | GPU | GPU | 1 | 3 | none | PASS |
| Levels, Gaussian Blur and Hue/Saturation, between the second and third layers frame 100, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Camera Shake, Motion Tile and Corner Pin, which grow by the frame's size, over part of it frame 0, Full | GPU | GPU | 1 | 10 | none | PASS |
| Camera Shake, Motion Tile and Corner Pin, which grow by the frame's size, over part of it frame 100, Full | GPU | GPU | 1 | 11 | none | PASS |
| Camera Shake, Motion Tile and Corner Pin, which grow by the frame's size, over part of it frame 0, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Camera Shake, Motion Tile and Corner Pin, which grow by the frame's size, over part of it frame 100, Draft | GPU | GPU | 1 | 1 | none | PASS |
| Directional Blur, Curves and a moving Noise, between the second and third layers frame 0, Full | GPU | GPU | 1 | 17 | none | PASS |
| Directional Blur, Curves and a moving Noise, between the second and third layers frame 100, Full | GPU | GPU | 1 | 27 | none | PASS |
| Directional Blur, Curves and a moving Noise, between the second and third layers frame 0, Draft | GPU | GPU | 1 | 2 | none | PASS |
| Directional Blur, Curves and a moving Noise, between the second and third layers frame 100, Draft | GPU | GPU | 0 | 0 | none | PASS |
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
