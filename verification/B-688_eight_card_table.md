# B-688 (D-808): 8 bpc (After Effects) frames on the graphics card

Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory. Every fixture project with each composition set to 8 bpc whose frame leaves an effect to the card, and every fixture saved in 8 bpc as saved, drawn by the CPU and by the card at its first and middle frames, Full and Draft. D-808's bar for each effect over all its fixtures' pixels: within 1 level of 255 on at least 99.9% and within 2 everywhere. A frame the card refuses must be the CPU's exactly.

## Each effect the card drew in 8 bpc, over all its fixtures

| Check | The build's answer | Matches |
| --- | --- | --- |
| (layers only, no effect on the card) | 466 fixtures, 953 frames, 64844 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| 3D Glasses | 24 fixtures, 96 frames, 8256 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| 4-Color Gradient | 18 fixtures, 70 frames, 6020 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Add Grain | 27 fixtures, 106 frames, 9116 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Advanced Spill Suppressor | 16 fixtures, 62 frames, 5332 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Aerial Haze | 16 fixtures, 62 frames, 5332 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Arbitrary Map | 11 fixtures, 44 frames, 3784 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Audio Spectrum | 31 fixtures, 124 frames, 10664 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Audio Waveform | 30 fixtures, 120 frames, 10320 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Beam | 19 fixtures, 76 frames, 6536 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Bend It | 16 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Bender | 15 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Bevel Alpha | 8 fixtures, 28 frames, 2408 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Bevel Edges | 7 fixtures, 28 frames, 2408 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Bezier Warp | 12 fixtures, 46 frames, 3956 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Bilateral Blur | 9 fixtures, 28 frames, 2556 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Black & White | 16 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Blobbylize | 19 fixtures, 76 frames, 6536 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Block Dissolve | 12 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Block Load | 15 fixtures, 60 frames, 5160 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Brightness & Contrast | 15 fixtures, 54 frames, 4644 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Broadcast Safe | 10 fixtures, 40 frames, 3440 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Brush Strokes | 20 fixtures, 80 frames, 6880 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Bubbles | 24 fixtures, 94 frames, 76704 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Bulge | 15 fixtures, 54 frames, 4644 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Burn Film | 13 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| CC Ball Action | 37 fixtures, 76 frames, 11864 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| CC Cylinder | 18 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| CC Environment | 19 fixtures, 76 frames, 6536 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| CC Glass | 20 fixtures, 80 frames, 6880 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| CC Simple Wire Removal | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| CC Sphere | 17 fixtures, 68 frames, 5848 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| CC Spotlight | 35 fixtures, 140 frames, 12040 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| CC Vector Blur | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| CC Vignette | 16 fixtures, 58 frames, 4988 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Camera Shake | 13 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Card Wipe | 50 fixtures, 200 frames, 17200 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Cartoon | 23 fixtures, 92 frames, 7912 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Cell Pattern | 37 fixtures, 148 frames, 12728 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Change Color | 13 fixtures, 50 frames, 4300 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Change to Color | 19 fixtures, 76 frames, 6536 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Channel Blur | 17 fixtures, 64 frames, 16320 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Channel Mixer | 15 fixtures, 54 frames, 4644 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Channel Threshold | 16 fixtures, 62 frames, 5332 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Channel Wipe | 21 fixtures, 82 frames, 7052 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Checkerboard | 23 fixtures, 92 frames, 7912 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Chromatic Aberration | 27 fixtures, 106 frames, 9116 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Circle | 28 fixtures, 112 frames, 9632 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Color Balance | 13 fixtures, 46 frames, 3956 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Color Balance (HLS) | 10 fixtures, 38 frames, 3268 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Color Difference Key | 17 fixtures, 68 frames, 5848 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Color Grade | 68 fixtures, 266 frames, 22876 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Color Link | 17 fixtures, 66 frames, 5676 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Color Lookup | 7 fixtures, 28 frames, 2408 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Color Neutralizer | 11 fixtures, 44 frames, 3784 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Color Offset | 11 fixtures, 42 frames, 3612 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Color Range | 12 fixtures, 48 frames, 4128 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Colorama | 23 fixtures, 92 frames, 7912 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Colour Key | 13 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Compound Blur | 15 fixtures, 58 frames, 4988 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Contour Paint | 31 fixtures, 124 frames, 10664 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Corner Pin | 14 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Cross Blur | 12 fixtures, 48 frames, 4128 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Cross Glare | 33 fixtures, 128 frames, 11008 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Cryptomatte | 16 fixtures, 64 frames, 1664 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Curl Noise | 62 fixtures, 248 frames, 21328 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Curves | 13 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Deband | 18 fixtures, 70 frames, 6020 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Depth Key | 10 fixtures, 40 frames, 1040 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Depth of Field | 17 fixtures, 68 frames, 1768 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Detail-preserving Upscale | 13 fixtures, 48 frames, 16320 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Difference Matte | 14 fixtures, 56 frames, 4816 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Diffusion | 19 fixtures, 72 frames, 6192 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Directional | 26 fixtures, 102 frames, 8772 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Displacement Map | 32 fixtures, 128 frames, 11008 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Distance Gradation | 14 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Drizzle | 24 fixtures, 94 frames, 76704 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Dust & Scratches | 9 fixtures, 18 frames, 2732 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Ellipse | 18 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Emboss | 18 fixtures, 72 frames, 6192 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Energy Stroke | 46 fixtures, 174 frames, 15704 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Exposure Flicker | 14 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Extract | 17 fixtures, 68 frames, 5848 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Eyedropper Fill | 22 fixtures, 86 frames, 7396 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Fast Blur (Legacy) | 6 fixtures, 22 frames, 5610 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Fast Box Blur | 9 fixtures, 36 frames, 9180 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Find Edges | 12 fixtures, 44 frames, 3784 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Fisheye | 14 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Flow Motion | 14 fixtures, 52 frames, 4472 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Foam | 41 fixtures, 162 frames, 132192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Fog 3D | 18 fixtures, 70 frames, 1820 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Fractal | 30 fixtures, 120 frames, 10320 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Fractal Noise | 32 fixtures, 126 frames, 10836 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Gamma/Pedestal/Gain | 13 fixtures, 50 frames, 4300 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Gaussian | 12 fixtures, 44 frames, 8020 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Glass Melt | 20 fixtures, 78 frames, 6708 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Glow Wipe | 15 fixtures, 58 frames, 4988 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Glue Gun | 16 fixtures, 64 frames, 5504 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Gradient | 14 fixtures, 56 frames, 4816 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Gradient Map | 15 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Gradient Wipe | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Grid | 26 fixtures, 104 frames, 8944 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Griddler | 14 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| HSV Key | 10 fixtures, 40 frames, 3440 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Hair | 20 fixtures, 80 frames, 65280 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Halation | 13 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Halftone | 16 fixtures, 60 frames, 5160 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Hex Tile | 14 fixtures, 33 frames, 4540 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Hue/Saturation | 17 fixtures, 64 frames, 5504 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| ID Key | 11 fixtures, 44 frames, 1144 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| IDentifier | 15 fixtures, 60 frames, 1560 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Invert | 20 fixtures, 74 frames, 6364 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Iris Wipe | 17 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Kernel | 11 fixtures, 42 frames, 3612 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Kira-kira | 18 fixtures, 68 frames, 5848 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Leave Color | 15 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Lens Blur | 48 fixtures, 192 frames, 16512 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Lens Flare | 20 fixtures, 80 frames, 6880 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Levels | 19 fixtures, 70 frames, 6020 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Levels (Individual Controls) | 8 fixtures, 30 frames, 2580 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Light Burst | 20 fixtures, 78 frames, 6708 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Light Rays | 41 fixtures, 160 frames, 13760 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Light Sweep | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Lightning Bolt | 66 fixtures, 262 frames, 22532 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Line Blur | 9 fixtures, 34 frames, 2924 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Line Recolour | 10 fixtures, 40 frames, 3440 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Line Smoothing | 12 fixtures, 46 frames, 2346 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Line Width | 11 fixtures, 42 frames, 3612 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Linear Color Key | 13 fixtures, 52 frames, 4472 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Linear Wipe | 21 fixtures, 82 frames, 7052 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Liquify | 30 fixtures, 118 frames, 10148 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Magnify | 23 fixtures, 92 frames, 7912 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Mask Smear | 11 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Match Grain | 15 fixtures, 29 frames, 4640 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Matte Choker | 7 fixtures, 28 frames, 5712 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Median | 6 fixtures, 11 frames, 1612 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Mercury | 26 fixtures, 102 frames, 83232 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Mesh Warp | 19 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Mirror | 18 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Mosaic | 27 fixtures, 82 frames, 8236 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Motion Tile | 46 fixtures, 178 frames, 15456 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Mr. Smoothie | 18 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Noise | 11 fixtures, 42 frames, 3612 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Noise Alpha | 21 fixtures, 82 frames, 7052 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Noise HLS | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Noise HLS Auto | 14 fixtures, 54 frames, 4644 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Offset | 17 fixtures, 62 frames, 5332 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Optics Compensation | 13 fixtures, 50 frames, 4300 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Outline | 12 fixtures, 46 frames, 3956 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Page Turn | 18 fixtures, 70 frames, 6020 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Paint Bucket | 30 fixtures, 120 frames, 10320 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Paraffin | 15 fixtures, 58 frames, 4988 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Particle Emitter | 73 fixtures, 288 frames, 24768 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Particle Grid | 72 fixtures, 286 frames, 24596 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Particle Plane | 85 fixtures, 336 frames, 28896 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Particle Playground | 122 fixtures, 454 frames, 41412 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Particle Space | 83 fixtures, 330 frames, 28380 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Pass Extract | 37 fixtures, 148 frames, 3848 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Path Stroke | 36 fixtures, 144 frames, 12384 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Photo Filter | 13 fixtures, 50 frames, 4300 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Plastic | 22 fixtures, 88 frames, 7568 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Polar Coordinates | 16 fixtures, 60 frames, 5160 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Polygon Shards | 25 fixtures, 96 frames, 79776 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Posterize | 12 fixtures, 48 frames, 4128 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Power Pin | 17 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Push Iris | 10 fixtures, 38 frames, 3268 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Radial | 39 fixtures, 156 frames, 13416 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Radial Shadow | 15 fixtures, 60 frames, 5160 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Radial Wipe | 20 fixtures, 76 frames, 6536 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Radio Waves | 26 fixtures, 104 frames, 8944 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Rain | 17 fixtures, 34 frames, 5144 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Remove Grain | 13 fixtures, 50 frames, 4300 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Repeat Tile | 24 fixtures, 94 frames, 8084 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Reshape | 24 fixtures, 84 frames, 7224 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Rim Light | 20 fixtures, 78 frames, 6708 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Ripple | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Ripple Pulse | 9 fixtures, 22 frames, 1892 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Roughen Edges | 14 fixtures, 54 frames, 4644 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Scatter | 12 fixtures, 40 frames, 3884 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Scatterize | 17 fixtures, 68 frames, 55488 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Screen Key | 12 fixtures, 48 frames, 4128 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Scribble | 48 fixtures, 98 frames, 15384 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Select Colour | 7 fixtures, 28 frames, 2408 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Selective Color | 17 fixtures, 66 frames, 5676 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Selective Colour Blur | 20 fixtures, 78 frames, 3978 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Shadow/Highlight | 27 fixtures, 104 frames, 8944 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Sharpen | 13 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Shift Channels | 15 fixtures, 60 frames, 5160 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Simple Choker | 12 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Slant | 14 fixtures, 54 frames, 4644 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Smart Blur | 7 fixtures, 14 frames, 2092 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Smear | 10 fixtures, 34 frames, 2924 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Snowfall | 17 fixtures, 34 frames, 5144 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Soft Physical Glow | 71 fixtures, 284 frames, 24424 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Solarize | 12 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Speed Lines | 21 fixtures, 84 frames, 7224 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Spherize | 10 fixtures, 36 frames, 3096 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Spill Suppressor | 10 fixtures, 40 frames, 3440 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Split | 10 fixtures, 34 frames, 2924 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Split 2 | 12 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Star Burst | 18 fixtures, 70 frames, 57120 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Stretch Wipe | 11 fixtures, 42 frames, 3612 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Stripe Sweep | 13 fixtures, 32 frames, 4084 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Strobe Light | 22 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Teeth Wipe | 15 fixtures, 54 frames, 4940 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Texturize | 11 fixtures, 44 frames, 3784 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Threads | 19 fixtures, 70 frames, 6464 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Threshold | 14 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Threshold RGB | 9 fixtures, 34 frames, 2924 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Tile Wipe | 15 fixtures, 58 frames, 4988 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Tiles | 13 fixtures, 46 frames, 3956 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Tint | 10 fixtures, 38 frames, 3268 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Toner | 9 fixtures, 36 frames, 3096 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Transform | 18 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Tritone | 8 fixtures, 30 frames, 2580 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Turbulent Displace | 52 fixtures, 204 frames, 17544 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Turbulent Noise | 82 fixtures, 326 frames, 28036 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Twirl | 17 fixtures, 60 frames, 5160 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Twister | 18 fixtures, 70 frames, 6020 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Unmult | 14 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Vegas | 33 fixtures, 132 frames, 11352 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Venetian Blinds | 17 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Vibrance | 12 fixtures, 42 frames, 3612 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Vignette | 17 fixtures, 64 frames, 5504 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Warp | 24 fixtures, 94 frames, 8084 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Warp Blend | 24 fixtures, 94 frames, 8084 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Wave Warp | 35 fixtures, 138 frames, 11868 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Write-on | 26 fixtures, 100 frames, 8600 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |

## Projects saved in 8 bpc before B-688 open and draw as saved

| Check | The build's answer | Matches |
| --- | --- | --- |
| display_mix/fx_mix8_001 | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |
| display_mix/fx_mix8_003 | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |
| display_mix/fx_mix8_005 | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |
| display_mix/fx_mix8_007 | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |
| display_mix/fx_mix8_009 | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |
| display_mix/fx_mix8_011 | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |
| eight_bpc/both_depths | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |
| eight_bpc/fx_8bpc_001 | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |
| eight_bpc/fx_8bpc_002 | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |
| eight_bpc/fx_8bpc_003 | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |
| eight_bpc/fx_8bpc_004 | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |
| eight_bpc/fx_8bpc_005 | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |
| exposure_ae/fx_expae_008 | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |
| exposure_ae/fx_expae_009 | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_006 | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_016 | opens; the card's frame within 1 level of the CPU's, or the CPU's exactly | yes |

## All the frames

| Check | The build's answer | Matches |
| --- | --- | --- |
| 4711 fixture projects | 18822 frames on the card; 8 the card refused, 8 of them the CPU's exactly | yes |

## Result

245 of 245 checks pass.
