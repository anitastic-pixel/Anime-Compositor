# B-754 (D-874): Float and 32 bpc (After Effects) frames on the graphics card

Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory. Every fixture project with each composition set to Float, and again to 32 bpc (After Effects), and every fixture saved in either as saved, drawn by the CPU and by the card at its first and middle frames, Full and Draft. D-874's bar for each effect over all its fixtures' pixels: at 8 bits within 1 level of 255 on at least 99.9% and within 2 everywhere; in floats (the card's own linear frame before it is turned to 8 bits) within 1e-4 on solid areas (slope under 1e-3 a pixel), and no edge moved by more than 0.05 px (the difference over the slope where it is at least 0.01). A frame the card refuses must be the CPU's exactly.

## Each effect the card drew in Float, over all its fixtures

| Check | The build's answer | Matches |
| --- | --- | --- |
| (layers only, no effect on the card) | 4180 fixtures, 15773 frames, 97077364 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 9.60e-6, solid 1.79e-7, edge shift 0.0000 px | yes |
| 3D Glasses | 24 fixtures, 96 frames, 8256 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| 4-Color Gradient | 18 fixtures, 70 frames, 6020 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 1.16e-10, edge shift 0.0000 px | yes |
| Add Grain | 27 fixtures, 106 frames, 9116 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Advanced Spill Suppressor | 16 fixtures, 62 frames, 5332 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Aerial Haze | 16 fixtures, 62 frames, 5332 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Arbitrary Map | 11 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Audio Spectrum | 31 fixtures, 124 frames, 10664 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Audio Waveform | 30 fixtures, 120 frames, 10320 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Beam | 19 fixtures, 76 frames, 6536 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Bend It | 16 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Bender | 15 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Bevel Alpha | 8 fixtures, 28 frames, 2408 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Bevel Edges | 7 fixtures, 28 frames, 2408 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Bezier Warp | 12 fixtures, 46 frames, 3956 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 1.49e-8, edge shift 0.0000 px | yes |
| Bilateral Blur | 9 fixtures, 28 frames, 2556 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 7.15e-7, solid 7.15e-7, edge shift 0.0000 px | yes |
| Black & White | 16 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Blobbylize | 19 fixtures, 76 frames, 6536 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Block Dissolve | 12 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Block Load | 15 fixtures, 60 frames, 5160 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 3.73e-7, solid 1.79e-7, edge shift 0.0000 px | yes |
| Bloom | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.77e-7, solid 9.31e-10, edge shift 0.0000 px | yes |
| Brightness & Contrast | 15 fixtures, 54 frames, 4644 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Broadcast Safe | 10 fixtures, 40 frames, 3440 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Brush Strokes | 20 fixtures, 80 frames, 6880 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Bubbles | 24 fixtures, 94 frames, 76704 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.49e-8, solid 3.73e-9, edge shift 0.0000 px | yes |
| Bulge | 15 fixtures, 54 frames, 4644 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 1.49e-8, edge shift 0.0000 px | yes |
| Burn Film | 13 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.45e-7, solid 3.54e-8, edge shift 0.0000 px | yes |
| CC Ball Action | 37 fixtures, 76 frames, 11864 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.68e-39, solid 4.91e-39, edge shift 0.0000 px | yes |
| CC Cylinder | 18 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.88e-6, solid 2.78e-15, edge shift 0.0000 px | yes |
| CC Environment | 19 fixtures, 76 frames, 6536 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| CC Glass | 20 fixtures, 80 frames, 6880 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.71e-6, solid 2.38e-7, edge shift 0.0000 px | yes |
| CC Simple Wire Removal | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| CC Sphere | 17 fixtures, 68 frames, 5848 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.31e-6, solid 5.96e-8, edge shift 0.0000 px | yes |
| CC Spotlight | 35 fixtures, 140 frames, 12040 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.79e-6, solid 2.79e-9, edge shift 0.0000 px | yes |
| CC Vector Blur | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.02e-5, solid 0.00e0, edge shift 0.0003 px | yes |
| CC Vignette | 16 fixtures, 58 frames, 4988 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.17e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Camera Shake | 13 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 3.73e-9, edge shift 0.0000 px | yes |
| Card Wipe | 50 fixtures, 200 frames, 17200 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.43e-6, solid 2.98e-8, edge shift 0.0001 px | yes |
| Cartoon | 23 fixtures, 92 frames, 7912 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.25e-6, solid 1.25e-6, edge shift 0.0000 px | yes |
| Cell Pattern | 37 fixtures, 148 frames, 12728 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Change Color | 13 fixtures, 50 frames, 4300 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 8.59e-18, solid 2.15e-18, edge shift 0.0000 px | yes |
| Change to Color | 19 fixtures, 76 frames, 6536 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 8.59e-18, solid 0.00e0, edge shift 0.0000 px | yes |
| Channel Blur | 17 fixtures, 64 frames, 16320 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.79e-7, solid 3.73e-9, edge shift 0.0000 px | yes |
| Channel Mixer | 15 fixtures, 54 frames, 4644 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.81e-18, solid 4.81e-18, edge shift 0.0000 px | yes |
| Channel Threshold | 16 fixtures, 62 frames, 5332 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Channel Wipe | 21 fixtures, 82 frames, 7052 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Checkerboard | 23 fixtures, 92 frames, 7912 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Chromatic Aberration | 27 fixtures, 106 frames, 9116 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.73e-6, solid 1.19e-7, edge shift 0.0000 px | yes |
| Circle | 28 fixtures, 112 frames, 9632 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Color Balance | 13 fixtures, 46 frames, 3956 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Color Balance (HLS) | 10 fixtures, 38 frames, 3268 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.15e-18, solid 2.15e-18, edge shift 0.0000 px | yes |
| Color Difference Key | 17 fixtures, 68 frames, 5848 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 3.58e-7, solid 2.79e-9, edge shift 0.0000 px | yes |
| Color Grade | 68 fixtures, 266 frames, 22876 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.07e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Color Link | 17 fixtures, 66 frames, 5676 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Color Lookup | 7 fixtures, 28 frames, 2408 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Color Neutralizer | 11 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.08e-18, solid 0.00e0, edge shift 0.0000 px | yes |
| Color Offset | 11 fixtures, 42 frames, 3612 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Color Range | 12 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.55e-15, solid 5.55e-15, edge shift 0.0000 px | yes |
| Colorama | 23 fixtures, 92 frames, 7912 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.11e-16, solid 8.59e-17, edge shift 0.0000 px | yes |
| Colour Key | 13 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Compound Blur | 16 fixtures, 62 frames, 5332 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 1.19e-7, edge shift 0.0000 px | yes |
| Contour Paint | 31 fixtures, 124 frames, 10664 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 3.73e-9, edge shift 0.0000 px | yes |
| Corner Pin | 14 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Cross Blur | 12 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Cross Glare | 33 fixtures, 128 frames, 11008 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 9.54e-7, solid 4.77e-7, edge shift 0.0000 px | yes |
| Cryptomatte | 17 fixtures, 68 frames, 1768 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Curl Noise | 62 fixtures, 248 frames, 21328 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 6.68e-6, solid 7.45e-9, edge shift 0.0000 px | yes |
| Curves | 13 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 8.63e-18, solid 0.00e0, edge shift 0.0000 px | yes |
| Deband | 18 fixtures, 70 frames, 6020 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.38e-7, solid 1.79e-7, edge shift 0.0000 px | yes |
| Depth Key | 10 fixtures, 40 frames, 1040 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Depth of Field | 18 fixtures, 72 frames, 1872 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Detail-preserving Upscale | 13 fixtures, 48 frames, 16320 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.77e-7, solid 2.98e-7, edge shift 0.0000 px | yes |
| Difference Matte | 14 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.43e-6, solid 0.00e0, edge shift 0.0000 px | yes |
| Diffusion | 19 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.49e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Directional | 26 fixtures, 102 frames, 8772 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 7.15e-7, solid 5.36e-7, edge shift 0.0000 px | yes |
| Displacement Map | 32 fixtures, 128 frames, 11008 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Distance Gradation | 14 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Drizzle | 24 fixtures, 94 frames, 76704 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-7, solid 3.73e-9, edge shift 0.0000 px | yes |
| Drop Shadow | 35 fixtures, 140 frames, 12040 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.34e-7, solid 5.33e-7, edge shift 0.0000 px | yes |
| Dust & Scratches | 9 fixtures, 18 frames, 2732 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Ellipse | 18 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Emboss | 18 fixtures, 72 frames, 6192 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.94e-7, solid 8.94e-8, edge shift 0.0000 px | yes |
| Energy Stroke | 46 fixtures, 174 frames, 15704 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 2.33e-10, edge shift 0.0000 px | yes |
| Exposure | 59 fixtures, 232 frames, 32708 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.91e-6, solid 1.19e-7, edge shift 0.0000 px | yes |
| Exposure Flicker | 14 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Extract | 17 fixtures, 68 frames, 5848 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.76e-16, solid 0.00e0, edge shift 0.0000 px | yes |
| Eyedropper Fill | 22 fixtures, 86 frames, 7396 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Fast Blur (Legacy) | 7 fixtures, 26 frames, 6630 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.91e-6, solid 1.16e-10, edge shift 0.0000 px | yes |
| Fast Box Blur | 14 fixtures, 56 frames, 14280 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.91e-6, solid 3.73e-9, edge shift 0.0000 px | yes |
| Fill | 28 fixtures, 110 frames, 9460 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 2.91e-11, edge shift 0.0000 px | yes |
| Find Edges | 12 fixtures, 44 frames, 3784 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 8.59e-18, solid 8.59e-18, edge shift 0.0000 px | yes |
| Fisheye | 14 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Flow Motion | 14 fixtures, 52 frames, 4472 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Foam | 41 fixtures, 162 frames, 132192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Fog 3D | 19 fixtures, 74 frames, 1924 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Fractal | 30 fixtures, 120 frames, 10320 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Fractal Noise | 32 fixtures, 126 frames, 10836 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Gamma/Pedestal/Gain | 13 fixtures, 50 frames, 4300 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.38e-7, solid 1.75e-10, edge shift 0.0000 px | yes |
| Gaussian | 17 fixtures, 64 frames, 12204 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.91e-6, solid 3.73e-9, edge shift 0.0000 px | yes |
| Glass Melt | 20 fixtures, 78 frames, 6708 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.85e-6, solid 5.96e-8, edge shift 0.0000 px | yes |
| Glow | 31 fixtures, 120 frames, 10320 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.38e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Glow Wipe | 15 fixtures, 58 frames, 4988 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Glue Gun | 16 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.38e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Gradient | 14 fixtures, 56 frames, 4816 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Gradient Map | 15 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Gradient Wipe | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.49e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Grid | 26 fixtures, 104 frames, 8944 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Griddler | 14 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| HSV Key | 10 fixtures, 40 frames, 3440 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Hair | 20 fixtures, 80 frames, 65280 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.92e-6, solid 8.94e-8, edge shift 0.0002 px | yes |
| Halation | 13 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 4.66e-10, edge shift 0.0000 px | yes |
| Halftone | 16 fixtures, 60 frames, 5160 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Hex Tile | 14 fixtures, 33 frames, 4540 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Hue/Saturation | 17 fixtures, 64 frames, 5504 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| ID Key | 11 fixtures, 44 frames, 1144 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| IDentifier | 16 fixtures, 64 frames, 1664 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Invert | 21 fixtures, 78 frames, 6708 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Iris Wipe | 17 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Kernel | 11 fixtures, 42 frames, 3612 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Kira-kira | 18 fixtures, 68 frames, 5848 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 1.19e-7, edge shift 0.0000 px | yes |
| Leave Color | 15 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Lens Blur | 48 fixtures, 192 frames, 16512 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Lens Flare | 20 fixtures, 80 frames, 6880 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.43e-6, solid 8.94e-8, edge shift 0.0000 px | yes |
| Levels | 19 fixtures, 70 frames, 6020 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.47e-7, solid 2.68e-7, edge shift 0.0000 px | yes |
| Levels (Individual Controls) | 8 fixtures, 30 frames, 2580 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.38e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Light Burst | 20 fixtures, 78 frames, 6708 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.91e-6, solid 2.98e-8, edge shift 0.0000 px | yes |
| Light Rays | 41 fixtures, 160 frames, 13760 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.86e-6, solid 1.19e-7, edge shift 0.0000 px | yes |
| Light Sweep | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Lightning Bolt | 66 fixtures, 262 frames, 22532 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Line Blur | 9 fixtures, 34 frames, 2924 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.79e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Line Recolour | 10 fixtures, 40 frames, 3440 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Line Smoothing | 12 fixtures, 46 frames, 2346 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.38e-7, solid 1.19e-7, edge shift 0.0000 px | yes |
| Line Width | 11 fixtures, 42 frames, 3612 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Linear Color Key | 13 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Linear Wipe | 21 fixtures, 82 frames, 7052 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Liquify | 30 fixtures, 118 frames, 10148 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 3.73e-9, edge shift 0.0000 px | yes |
| Magnify | 23 fixtures, 92 frames, 7912 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 5.96e-8, edge shift 0.0000 px | yes |
| Mask Smear | 11 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Match Grain | 15 fixtures, 29 frames, 4640 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Matte Choker | 7 fixtures, 28 frames, 5712 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Median | 6 fixtures, 11 frames, 1612 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Mercury | 26 fixtures, 102 frames, 83232 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Mesh Warp | 19 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 1.49e-8, edge shift 0.0000 px | yes |
| Mirror | 18 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Mosaic | 27 fixtures, 82 frames, 8236 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.86e-7, solid 1.86e-7, edge shift 0.0000 px | yes |
| Motion Tile | 47 fixtures, 182 frames, 15800 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 6.56e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Mr. Smoothie | 18 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.79e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Noise | 11 fixtures, 42 frames, 3612 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Noise Alpha | 21 fixtures, 82 frames, 7052 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Noise HLS | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Noise HLS Auto | 14 fixtures, 54 frames, 4644 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Offset | 17 fixtures, 62 frames, 5332 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Optics Compensation | 13 fixtures, 50 frames, 4300 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Outline | 12 fixtures, 46 frames, 3956 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Page Turn | 18 fixtures, 70 frames, 6020 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Paint Bucket | 30 fixtures, 120 frames, 10320 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 8.01e-8, solid 5.96e-8, edge shift 0.0000 px | yes |
| Paraffin | 15 fixtures, 58 frames, 4988 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Particle Emitter | 73 fixtures, 288 frames, 24768 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Particle Grid | 72 fixtures, 286 frames, 24596 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Particle Plane | 85 fixtures, 336 frames, 28896 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Particle Playground | 122 fixtures, 454 frames, 41412 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 3.28e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Particle Space | 85 fixtures, 338 frames, 29068 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Pass Extract | 39 fixtures, 156 frames, 4056 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Path Stroke | 36 fixtures, 144 frames, 12384 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Photo Filter | 13 fixtures, 50 frames, 4300 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Plastic | 22 fixtures, 88 frames, 7568 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-7, solid 9.31e-10, edge shift 0.0000 px | yes |
| Polar Coordinates | 16 fixtures, 60 frames, 5160 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Polygon Shards | 25 fixtures, 96 frames, 79776 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Posterize | 12 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Power Pin | 17 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Push Iris | 10 fixtures, 38 frames, 3268 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Radial | 39 fixtures, 156 frames, 13416 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 3.87e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Radial Shadow | 15 fixtures, 60 frames, 5160 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Radial Wipe | 20 fixtures, 76 frames, 6536 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Radio Waves | 26 fixtures, 104 frames, 8944 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.44e-16, solid 0.00e0, edge shift 0.0000 px | yes |
| Rain | 17 fixtures, 34 frames, 5144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Remove Grain | 13 fixtures, 50 frames, 4300 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 7.15e-7, solid 5.96e-7, edge shift 0.0000 px | yes |
| Repeat Tile | 24 fixtures, 94 frames, 8084 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Reshape | 24 fixtures, 84 frames, 7224 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Rim Light | 20 fixtures, 78 frames, 6708 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Ripple | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Ripple Pulse | 9 fixtures, 22 frames, 1892 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Roughen Edges | 14 fixtures, 54 frames, 4644 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Scatter | 12 fixtures, 40 frames, 3884 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Scatterize | 17 fixtures, 68 frames, 55488 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.91e-6, solid 1.31e-6, edge shift 0.0000 px | yes |
| Screen Key | 12 fixtures, 48 frames, 4128 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-8, solid 9.31e-10, edge shift 0.0000 px | yes |
| Scribble | 48 fixtures, 98 frames, 15384 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Select Colour | 7 fixtures, 28 frames, 2408 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Selective Color | 17 fixtures, 66 frames, 5676 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Selective Colour Blur | 20 fixtures, 78 frames, 3978 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-7, solid 1.79e-7, edge shift 0.0000 px | yes |
| Shadow/Highlight | 27 fixtures, 104 frames, 8944 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-6, solid 2.05e-8, edge shift 0.0000 px | yes |
| Sharpen | 13 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.07e-6, solid 3.58e-7, edge shift 0.0000 px | yes |
| Shift Channels | 15 fixtures, 60 frames, 5160 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Simple Choker | 12 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Slant | 14 fixtures, 54 frames, 4644 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Smart Blur | 7 fixtures, 14 frames, 2092 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Smear | 10 fixtures, 34 frames, 2924 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Snowfall | 17 fixtures, 34 frames, 5144 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Soft Physical Glow | 71 fixtures, 284 frames, 24424 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.38e-7, solid 1.86e-9, edge shift 0.0000 px | yes |
| Solarize | 12 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Solid Composite | 18 fixtures, 72 frames, 8896 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.77e-7, solid 1.86e-9, edge shift 0.0000 px | yes |
| Speed Lines | 21 fixtures, 84 frames, 7224 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Spherize | 10 fixtures, 36 frames, 3096 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 1.19e-7, edge shift 0.0000 px | yes |
| Spill Suppressor | 10 fixtures, 40 frames, 3440 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Split | 10 fixtures, 34 frames, 2924 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 7.45e-9, edge shift 0.0000 px | yes |
| Split 2 | 12 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Star Burst | 18 fixtures, 70 frames, 57120 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.66e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Stretch Wipe | 11 fixtures, 42 frames, 3612 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Stripe Sweep | 13 fixtures, 32 frames, 4084 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Strobe Light | 22 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.49e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Teeth Wipe | 15 fixtures, 54 frames, 4940 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Texturize | 11 fixtures, 44 frames, 3784 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Threads | 19 fixtures, 70 frames, 6464 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Threshold | 14 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Threshold RGB | 9 fixtures, 34 frames, 2924 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Tile Wipe | 15 fixtures, 58 frames, 4988 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Tiles | 13 fixtures, 46 frames, 3956 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Tint | 10 fixtures, 38 frames, 3268 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Toner | 9 fixtures, 36 frames, 3096 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Transform | 18 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Tritone | 8 fixtures, 30 frames, 2580 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Turbulent Displace | 52 fixtures, 204 frames, 17544 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.13e-6, solid 1.13e-6, edge shift 0.0000 px | yes |
| Turbulent Noise | 82 fixtures, 326 frames, 28036 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Twirl | 17 fixtures, 60 frames, 5160 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 6.56e-7, solid 4.77e-7, edge shift 0.0000 px | yes |
| Twister | 18 fixtures, 70 frames, 6020 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Unmult | 14 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Vegas | 33 fixtures, 132 frames, 11352 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Venetian Blinds | 17 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Vibrance | 12 fixtures, 42 frames, 3612 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 3.21e-19, solid 0.00e0, edge shift 0.0000 px | yes |
| Vignette | 17 fixtures, 64 frames, 5504 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 7.15e-7, solid 5.36e-7, edge shift 0.0000 px | yes |
| Warp | 24 fixtures, 94 frames, 8084 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 3.73e-9, edge shift 0.0000 px | yes |
| Warp Blend | 24 fixtures, 94 frames, 8084 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.88e-6, solid 2.38e-7, edge shift 0.0000 px | yes |
| Wave Warp | 35 fixtures, 138 frames, 11868 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Write-on | 26 fixtures, 100 frames, 8600 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |

## All the frames in Float

| Check | The build's answer | Matches |
| --- | --- | --- |
| 8619 fixture projects | 34302 frames on the card; 118 the card refused, 118 of them the CPU's exactly | yes |
| Refused (20 frames) | The CPU drew this frame: a layer has a blend mode the GPU does not draw. | yes |
| Refused (4 frames) | The CPU drew this frame: it has a Light Wrap with a Mix below 100, which the GPU does not draw yet. | yes |
| Refused (10 frames) | The CPU drew this frame: it has motion blur or frame blending and no effect for the GPU, which the CPU draws faster. | yes |
| Refused (84 frames) | The CPU drew this frame: its adjustment layer has an effect the GPU does not draw there. | yes |

## Each effect the card drew in 32 bpc (After Effects), over all its fixtures

| Check | The build's answer | Matches |
| --- | --- | --- |
| (layers only, no effect on the card) | 4358 fixtures, 16499 frames, 97168126 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 9.60e-6, solid 1.79e-7, edge shift 0.0000 px | yes |
| 3D Glasses | 24 fixtures, 96 frames, 8256 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| 4-Color Gradient | 18 fixtures, 70 frames, 6020 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 1.16e-10, edge shift 0.0000 px | yes |
| Add Grain | 27 fixtures, 106 frames, 9116 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Advanced Spill Suppressor | 16 fixtures, 62 frames, 5332 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Aerial Haze | 16 fixtures, 62 frames, 5332 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Arbitrary Map | 11 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Audio Spectrum | 31 fixtures, 124 frames, 10664 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Audio Waveform | 30 fixtures, 120 frames, 10320 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Beam | 19 fixtures, 76 frames, 6536 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Bend It | 16 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Bender | 15 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Bevel Alpha | 8 fixtures, 28 frames, 2408 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Bevel Edges | 7 fixtures, 28 frames, 2408 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Bezier Warp | 12 fixtures, 46 frames, 3956 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 1.49e-8, edge shift 0.0000 px | yes |
| Bilateral Blur | 9 fixtures, 28 frames, 2556 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 7.15e-7, solid 7.15e-7, edge shift 0.0000 px | yes |
| Black & White | 16 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Blobbylize | 19 fixtures, 76 frames, 6536 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Block Dissolve | 12 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Block Load | 15 fixtures, 60 frames, 5160 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Bloom | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.77e-7, solid 9.31e-10, edge shift 0.0000 px | yes |
| Brightness & Contrast | 15 fixtures, 54 frames, 4644 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Broadcast Safe | 10 fixtures, 40 frames, 3440 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Brush Strokes | 20 fixtures, 80 frames, 6880 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Bubbles | 24 fixtures, 94 frames, 76704 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.49e-8, solid 3.73e-9, edge shift 0.0000 px | yes |
| Bulge | 15 fixtures, 54 frames, 4644 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 1.49e-8, edge shift 0.0000 px | yes |
| Burn Film | 13 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| CC Ball Action | 37 fixtures, 76 frames, 11864 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.68e-39, solid 4.91e-39, edge shift 0.0000 px | yes |
| CC Cylinder | 18 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.88e-6, solid 2.78e-15, edge shift 0.0000 px | yes |
| CC Environment | 19 fixtures, 76 frames, 6536 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| CC Glass | 20 fixtures, 80 frames, 6880 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.71e-6, solid 2.38e-7, edge shift 0.0000 px | yes |
| CC Simple Wire Removal | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| CC Sphere | 17 fixtures, 68 frames, 5848 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.31e-6, solid 5.96e-8, edge shift 0.0000 px | yes |
| CC Spotlight | 35 fixtures, 140 frames, 12040 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.79e-6, solid 2.79e-9, edge shift 0.0000 px | yes |
| CC Vector Blur | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.02e-5, solid 0.00e0, edge shift 0.0003 px | yes |
| CC Vignette | 16 fixtures, 58 frames, 4988 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Camera Shake | 13 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 3.73e-9, edge shift 0.0000 px | yes |
| Card Wipe | 50 fixtures, 200 frames, 17200 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.43e-6, solid 2.98e-8, edge shift 0.0001 px | yes |
| Cartoon | 23 fixtures, 92 frames, 7912 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.25e-6, solid 1.25e-6, edge shift 0.0000 px | yes |
| Cell Pattern | 37 fixtures, 148 frames, 12728 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Change Color | 13 fixtures, 50 frames, 4300 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 8.59e-18, solid 2.15e-18, edge shift 0.0000 px | yes |
| Change to Color | 19 fixtures, 76 frames, 6536 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 8.59e-18, solid 0.00e0, edge shift 0.0000 px | yes |
| Channel Blur | 17 fixtures, 64 frames, 16320 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.79e-7, solid 3.73e-9, edge shift 0.0000 px | yes |
| Channel Mixer | 15 fixtures, 54 frames, 4644 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.81e-18, solid 4.81e-18, edge shift 0.0000 px | yes |
| Channel Threshold | 16 fixtures, 62 frames, 5332 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Channel Wipe | 21 fixtures, 82 frames, 7052 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Checkerboard | 23 fixtures, 92 frames, 7912 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Chromatic Aberration | 27 fixtures, 106 frames, 9116 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.73e-6, solid 1.19e-7, edge shift 0.0000 px | yes |
| Circle | 28 fixtures, 112 frames, 9632 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Color Balance | 13 fixtures, 46 frames, 3956 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Color Balance (HLS) | 10 fixtures, 38 frames, 3268 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.15e-18, solid 2.15e-18, edge shift 0.0000 px | yes |
| Color Difference Key | 17 fixtures, 68 frames, 5848 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 3.58e-7, solid 2.79e-9, edge shift 0.0000 px | yes |
| Color Grade | 68 fixtures, 266 frames, 22876 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.07e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Color Link | 17 fixtures, 66 frames, 5676 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Color Lookup | 7 fixtures, 28 frames, 2408 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Color Neutralizer | 11 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.08e-18, solid 0.00e0, edge shift 0.0000 px | yes |
| Color Offset | 11 fixtures, 42 frames, 3612 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Color Range | 12 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.55e-15, solid 5.55e-15, edge shift 0.0000 px | yes |
| Colorama | 23 fixtures, 92 frames, 7912 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.11e-16, solid 8.59e-17, edge shift 0.0000 px | yes |
| Colour Key | 13 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Compound Blur | 16 fixtures, 62 frames, 5332 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 1.19e-7, edge shift 0.0000 px | yes |
| Contour Paint | 31 fixtures, 124 frames, 10664 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 3.73e-9, edge shift 0.0000 px | yes |
| Corner Pin | 14 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Cross Glare | 33 fixtures, 128 frames, 11008 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 9.54e-7, solid 4.77e-7, edge shift 0.0000 px | yes |
| Cryptomatte | 16 fixtures, 64 frames, 1664 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Curl Noise | 62 fixtures, 248 frames, 21328 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 6.68e-6, solid 7.45e-9, edge shift 0.0000 px | yes |
| Curves | 13 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 8.63e-18, solid 0.00e0, edge shift 0.0000 px | yes |
| Deband | 18 fixtures, 70 frames, 6020 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.38e-7, solid 1.79e-7, edge shift 0.0000 px | yes |
| Depth Key | 10 fixtures, 40 frames, 1040 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Depth of Field | 17 fixtures, 68 frames, 1768 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Detail-preserving Upscale | 13 fixtures, 48 frames, 16320 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.77e-7, solid 2.98e-7, edge shift 0.0000 px | yes |
| Difference Matte | 14 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.43e-6, solid 0.00e0, edge shift 0.0000 px | yes |
| Diffusion | 19 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.49e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Displacement Map | 32 fixtures, 128 frames, 11008 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Distance Gradation | 14 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Drizzle | 24 fixtures, 94 frames, 76704 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-7, solid 3.73e-9, edge shift 0.0000 px | yes |
| Drop Shadow | 35 fixtures, 140 frames, 12040 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.34e-7, solid 5.33e-7, edge shift 0.0000 px | yes |
| Dust & Scratches | 9 fixtures, 18 frames, 2732 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Ellipse | 18 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Emboss | 18 fixtures, 72 frames, 6192 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.94e-7, solid 8.94e-8, edge shift 0.0000 px | yes |
| Energy Stroke | 46 fixtures, 174 frames, 15704 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 2.33e-10, edge shift 0.0000 px | yes |
| Exposure Flicker | 14 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Extract | 17 fixtures, 68 frames, 5848 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.76e-16, solid 0.00e0, edge shift 0.0000 px | yes |
| Eyedropper Fill | 22 fixtures, 86 frames, 7396 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Fill | 28 fixtures, 110 frames, 9460 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 2.91e-11, edge shift 0.0000 px | yes |
| Find Edges | 12 fixtures, 44 frames, 3784 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 8.59e-18, solid 8.59e-18, edge shift 0.0000 px | yes |
| Fisheye | 14 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Flow Motion | 14 fixtures, 52 frames, 4472 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Foam | 41 fixtures, 162 frames, 132192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Fog 3D | 18 fixtures, 70 frames, 1820 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Fractal | 30 fixtures, 120 frames, 10320 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Fractal Noise | 32 fixtures, 126 frames, 10836 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Gamma/Pedestal/Gain | 13 fixtures, 50 frames, 4300 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.38e-7, solid 1.75e-10, edge shift 0.0000 px | yes |
| Glass Melt | 20 fixtures, 78 frames, 6708 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.85e-6, solid 5.96e-8, edge shift 0.0000 px | yes |
| Glow Wipe | 15 fixtures, 58 frames, 4988 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Glue Gun | 16 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.38e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Gradient | 14 fixtures, 56 frames, 4816 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Gradient Map | 15 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Gradient Wipe | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.49e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Grid | 26 fixtures, 104 frames, 8944 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Griddler | 14 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| HSV Key | 10 fixtures, 40 frames, 3440 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Hair | 20 fixtures, 80 frames, 65280 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.92e-6, solid 8.94e-8, edge shift 0.0002 px | yes |
| Halation | 13 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 4.66e-10, edge shift 0.0000 px | yes |
| Halftone | 16 fixtures, 60 frames, 5160 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Hex Tile | 14 fixtures, 33 frames, 4540 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Hue/Saturation | 17 fixtures, 64 frames, 5504 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| ID Key | 11 fixtures, 44 frames, 1144 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| IDentifier | 15 fixtures, 60 frames, 1560 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Invert | 20 fixtures, 74 frames, 6364 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Iris Wipe | 17 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Kernel | 11 fixtures, 42 frames, 3612 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Kira-kira | 18 fixtures, 68 frames, 5848 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 1.19e-7, edge shift 0.0000 px | yes |
| Leave Color | 15 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Lens Blur | 48 fixtures, 192 frames, 16512 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Lens Flare | 20 fixtures, 80 frames, 6880 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.43e-6, solid 8.94e-8, edge shift 0.0000 px | yes |
| Levels | 19 fixtures, 70 frames, 6020 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.47e-7, solid 2.68e-7, edge shift 0.0000 px | yes |
| Levels (Individual Controls) | 8 fixtures, 30 frames, 2580 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.38e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Light Burst | 20 fixtures, 78 frames, 6708 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.91e-6, solid 2.98e-8, edge shift 0.0000 px | yes |
| Light Rays | 41 fixtures, 160 frames, 13760 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.86e-6, solid 1.19e-7, edge shift 0.0000 px | yes |
| Light Sweep | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Lightning Bolt | 66 fixtures, 262 frames, 22532 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Line Blur | 9 fixtures, 34 frames, 2924 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.79e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Line Recolour | 10 fixtures, 40 frames, 3440 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Line Smoothing | 12 fixtures, 46 frames, 2346 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.38e-7, solid 1.19e-7, edge shift 0.0000 px | yes |
| Line Width | 11 fixtures, 42 frames, 3612 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Linear Color Key | 13 fixtures, 52 frames, 4472 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Linear Wipe | 21 fixtures, 82 frames, 7052 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Liquify | 30 fixtures, 118 frames, 10148 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 3.73e-9, edge shift 0.0000 px | yes |
| Magnify | 23 fixtures, 92 frames, 7912 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 5.96e-8, edge shift 0.0000 px | yes |
| Mask Smear | 11 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Match Grain | 15 fixtures, 29 frames, 4640 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Matte Choker | 7 fixtures, 28 frames, 5712 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Median | 6 fixtures, 11 frames, 1612 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Mercury | 26 fixtures, 102 frames, 83232 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Mesh Warp | 19 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 1.49e-8, edge shift 0.0000 px | yes |
| Mirror | 18 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Mosaic | 27 fixtures, 82 frames, 8236 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Motion Tile | 47 fixtures, 182 frames, 15800 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 6.56e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Mr. Smoothie | 18 fixtures, 72 frames, 6192 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.79e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Noise | 11 fixtures, 42 frames, 3612 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Noise Alpha | 21 fixtures, 82 frames, 7052 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Noise HLS | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Noise HLS Auto | 14 fixtures, 54 frames, 4644 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Offset | 17 fixtures, 62 frames, 5332 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Optics Compensation | 13 fixtures, 50 frames, 4300 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Outline | 12 fixtures, 46 frames, 3956 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Page Turn | 18 fixtures, 70 frames, 6020 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Paint Bucket | 30 fixtures, 120 frames, 10320 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 8.01e-8, solid 5.96e-8, edge shift 0.0000 px | yes |
| Paraffin | 15 fixtures, 58 frames, 4988 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Particle Emitter | 73 fixtures, 288 frames, 24768 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Particle Grid | 72 fixtures, 286 frames, 24596 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Particle Plane | 85 fixtures, 336 frames, 28896 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Particle Playground | 122 fixtures, 454 frames, 41412 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 3.28e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Particle Space | 85 fixtures, 338 frames, 29068 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Pass Extract | 37 fixtures, 148 frames, 3848 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Path Stroke | 36 fixtures, 144 frames, 12384 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Photo Filter | 13 fixtures, 50 frames, 4300 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Plastic | 22 fixtures, 88 frames, 7568 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-7, solid 9.31e-10, edge shift 0.0000 px | yes |
| Polar Coordinates | 16 fixtures, 60 frames, 5160 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Polygon Shards | 25 fixtures, 96 frames, 79776 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Posterize | 12 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Power Pin | 17 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Push Iris | 10 fixtures, 38 frames, 3268 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Radial Shadow | 15 fixtures, 60 frames, 5160 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Radial Wipe | 20 fixtures, 76 frames, 6536 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Radio Waves | 26 fixtures, 104 frames, 8944 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 4.44e-16, solid 0.00e0, edge shift 0.0000 px | yes |
| Rain | 17 fixtures, 34 frames, 5144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Remove Grain | 13 fixtures, 50 frames, 4300 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 7.15e-7, solid 5.96e-7, edge shift 0.0000 px | yes |
| Repeat Tile | 24 fixtures, 94 frames, 8084 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Reshape | 24 fixtures, 84 frames, 7224 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Rim Light | 20 fixtures, 78 frames, 6708 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Ripple | 17 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Ripple Pulse | 9 fixtures, 22 frames, 1892 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Roughen Edges | 14 fixtures, 54 frames, 4644 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Scatter | 12 fixtures, 40 frames, 3884 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Scatterize | 17 fixtures, 68 frames, 55488 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.91e-6, solid 1.31e-6, edge shift 0.0000 px | yes |
| Screen Key | 12 fixtures, 48 frames, 4128 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-8, solid 9.31e-10, edge shift 0.0000 px | yes |
| Scribble | 48 fixtures, 98 frames, 15384 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Select Colour | 7 fixtures, 28 frames, 2408 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Selective Color | 17 fixtures, 66 frames, 5676 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Selective Colour Blur | 20 fixtures, 78 frames, 3978 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-7, solid 1.79e-7, edge shift 0.0000 px | yes |
| Shadow/Highlight | 27 fixtures, 104 frames, 8944 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-6, solid 2.05e-8, edge shift 0.0000 px | yes |
| Sharpen | 13 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.07e-6, solid 3.58e-7, edge shift 0.0000 px | yes |
| Shift Channels | 15 fixtures, 60 frames, 5160 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Simple Choker | 12 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Slant | 14 fixtures, 54 frames, 4644 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Smart Blur | 7 fixtures, 14 frames, 2092 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Smear | 10 fixtures, 34 frames, 2924 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Snowfall | 17 fixtures, 34 frames, 5144 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Soft Physical Glow | 71 fixtures, 284 frames, 24424 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.38e-7, solid 1.86e-9, edge shift 0.0000 px | yes |
| Solarize | 12 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Speed Lines | 21 fixtures, 84 frames, 7224 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Spherize | 10 fixtures, 36 frames, 3096 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 1.19e-7, edge shift 0.0000 px | yes |
| Spill Suppressor | 10 fixtures, 40 frames, 3440 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Split | 10 fixtures, 34 frames, 2924 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 7.45e-9, edge shift 0.0000 px | yes |
| Split 2 | 12 fixtures, 44 frames, 3784 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Star Burst | 18 fixtures, 70 frames, 57120 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.66e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Stretch Wipe | 11 fixtures, 42 frames, 3612 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Stripe Sweep | 13 fixtures, 32 frames, 4084 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Strobe Light | 22 fixtures, 48 frames, 4128 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.49e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Teeth Wipe | 15 fixtures, 54 frames, 4940 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Texturize | 11 fixtures, 44 frames, 3784 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Threads | 19 fixtures, 70 frames, 6464 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 2.98e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Threshold | 14 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Threshold RGB | 9 fixtures, 34 frames, 2924 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Tile Wipe | 15 fixtures, 58 frames, 4988 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Tiles | 13 fixtures, 46 frames, 3956 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 5.96e-8, edge shift 0.0000 px | yes |
| Tint | 10 fixtures, 38 frames, 3268 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Toner | 9 fixtures, 36 frames, 3096 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Transform | 18 fixtures, 66 frames, 5676 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Tritone | 8 fixtures, 30 frames, 2580 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Turbulent Displace | 52 fixtures, 204 frames, 17544 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.13e-6, solid 1.13e-6, edge shift 0.0000 px | yes |
| Turbulent Noise | 82 fixtures, 326 frames, 28036 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 5.96e-8, solid 0.00e0, edge shift 0.0000 px | yes |
| Twirl | 17 fixtures, 60 frames, 5160 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 1.19e-7, edge shift 0.0000 px | yes |
| Twister | 18 fixtures, 70 frames, 6020 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Unmult | 14 fixtures, 56 frames, 4816 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Vegas | 33 fixtures, 132 frames, 11352 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Venetian Blinds | 17 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Vibrance | 12 fixtures, 42 frames, 3612 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 3.21e-19, solid 0.00e0, edge shift 0.0000 px | yes |
| Vignette | 17 fixtures, 64 frames, 5504 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |
| Warp | 24 fixtures, 94 frames, 8084 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 3.73e-9, edge shift 0.0000 px | yes |
| Warp Blend | 24 fixtures, 94 frames, 8084 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.88e-6, solid 2.38e-7, edge shift 0.0000 px | yes |
| Wave Warp | 35 fixtures, 138 frames, 11868 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; float largest 1.19e-7, solid 0.00e0, edge shift 0.0000 px | yes |
| Write-on | 26 fixtures, 100 frames, 8600 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; float largest 0.00e0, solid 0.00e0, edge shift 0.0000 px | yes |

## All the frames in 32 bpc (After Effects)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 8620 fixture projects | 34306 frames on the card; 118 the card refused, 118 of them the CPU's exactly | yes |
| Refused (20 frames) | The CPU drew this frame: a layer has a blend mode the GPU does not draw. | yes |
| Refused (4 frames) | The CPU drew this frame: it has a Light Wrap with a Mix below 100, which the GPU does not draw yet. | yes |
| Refused (10 frames) | The CPU drew this frame: it has motion blur or frame blending and no effect for the GPU, which the CPU draws faster. | yes |
| Refused (84 frames) | The CPU drew this frame: its adjustment layer has an effect the GPU does not draw there. | yes |

## Projects saved in Float or 32 bpc (After Effects) before B-754 open and draw as saved

| Check | The build's answer | Matches |
| --- | --- | --- |
| blurriness/fx_blurry_007 (Float) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| curl_noise/fx_curl_024 (Float) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| fast_blur_legacy/fx_fastleg_008 (Float) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| fast_box_blur/fx_fastbox_007 (Float) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| float_depth/fx_blend_addf_001 (Float) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| float_depth/fx_blend_addf_003 (Float) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| float_depth/fx_blend_scrf_001 (Float) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| float_depth/fx_blend_scrf_002 (Float) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| float_depth/fx_fnoise_hdr_001 (Float) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| float_depth/fx_fnoise_hdr_003 (Float) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| gaussian_blur_legacy/fx_gaussleg_007 (Float) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_007 (Float) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| ae_32bpc/fx_ae32_001 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| ae_32bpc/fx_ae32_002 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| ae_32bpc/fx_ae32_003 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| ae_32bpc/fx_ae32_004 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| ae_32bpc/fx_ae32_005 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| ae_32bpc/fx_ae32_006 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| exposure_ae/fx_expae_005 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| exposure_ae/fx_expae_006 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| exposure_ae/fx_expae_011 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_001 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_002 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_003 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_004 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_005 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_008 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_009 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_010 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_011 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_012 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_013 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_014 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |
| glow_display/fx_gldisp_015 (32 bpc (After Effects)) | opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly | yes |

## Values past white and below black survive the card as the CPU keeps them

| Check | The build's answer | Matches |
| --- | --- | --- |
| normal over the pushed card, opacity 0.5, Fractal Noise none, Full | CPU -0.4460 to 3.8652, card -0.4460 to 3.8652; largest difference 1.32e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| normal over the pushed card, opacity 0.5, Fractal Noise none, Draft | CPU 0.0195 to 3.4380, card 0.0195 to 3.4380; largest difference 1.12e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| normal over the pushed card, opacity 0.75, Fractal Noise screen, Full | CPU -0.4460 to 5.7228, card -0.4460 to 5.7228; largest difference 2.38e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| normal over the pushed card, opacity 0.75, Fractal Noise screen, Draft | CPU 0.0293 to 4.1218, card 0.0293 to 4.1218; largest difference 1.58e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| normal over the pushed card, opacity 1, Fractal Noise add, Full | CPU -0.4460 to 9.5720, card -0.4460 to 9.5720; largest difference 1.19e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| normal over the pushed card, opacity 1, Fractal Noise add, Draft | CPU 0.0391 to 8.2829, card 0.0391 to 8.2829; largest difference 9.61e-8 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| normal: the CPU's frames went past white and below black | -0.4460 to 9.5720 | yes |
| multiply over the pushed card, opacity 0.5, Fractal Noise none, Full | CPU -2.0932 to 1.0000, card -2.0932 to 1.0000; largest difference 2.38e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| multiply over the pushed card, opacity 0.5, Fractal Noise none, Draft | CPU -1.0980 to 1.0000, card -1.0980 to 1.0000; largest difference 1.19e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| multiply over the pushed card, opacity 0.75, Fractal Noise screen, Full | CPU -2.8710 to 1.0000, card -2.8710 to 1.0000; largest difference 2.36e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| multiply over the pushed card, opacity 0.75, Fractal Noise screen, Draft | CPU -1.6094 to 1.0000, card -1.6094 to 1.0000; largest difference 1.48e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| multiply over the pushed card, opacity 1, Fractal Noise add, Full | CPU -4.6090 to 1.5123, card -4.6090 to 1.5123; largest difference 1.90e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| multiply over the pushed card, opacity 1, Fractal Noise add, Draft | CPU -2.7397 to 1.0000, card -2.7397 to 1.0000; largest difference 2.15e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| multiply: the CPU's frames went past white and below black | -4.6090 to 1.5123 | yes |
| screen over the pushed card, opacity 0.5, Fractal Noise none, Full | CPU -0.4460 to 5.1138, card -0.4460 to 5.1138; largest difference 2.18e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| screen over the pushed card, opacity 0.5, Fractal Noise none, Draft | CPU 0.0195 to 4.2664, card 0.0195 to 4.2664; largest difference 2.38e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| screen over the pushed card, opacity 0.75, Fractal Noise screen, Full | CPU -0.4460 to 7.7795, card -0.4460 to 7.7795; largest difference 3.58e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| screen over the pushed card, opacity 0.75, Fractal Noise screen, Draft | CPU 0.0293 to 5.0490, card 0.0293 to 5.0490; largest difference 2.38e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| screen over the pushed card, opacity 1, Fractal Noise add, Full | CPU -0.4460 to 13.3906, card -0.4460 to 13.3906; largest difference 1.86e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| screen over the pushed card, opacity 1, Fractal Noise add, Draft | CPU 0.0391 to 10.2464, card 0.0391 to 10.2464; largest difference 2.08e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| screen: the CPU's frames went past white and below black | -0.4460 to 13.3906 | yes |
| add over the pushed card, opacity 0.5, Fractal Noise none, Full | CPU -0.4460 to 3.7304, card -0.4460 to 3.7304; largest difference 1.53e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| add over the pushed card, opacity 0.5, Fractal Noise none, Draft | CPU 0.0195 to 3.3031, card 0.0195 to 3.3031; largest difference 2.35e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| add over the pushed card, opacity 0.75, Fractal Noise screen, Full | CPU -0.4460 to 5.5206, card -0.4460 to 5.5206; largest difference 3.25e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| add over the pushed card, opacity 0.75, Fractal Noise screen, Draft | CPU 0.0010 to 3.9196, card 0.0010 to 3.9196; largest difference 2.32e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| add over the pushed card, opacity 1, Fractal Noise add, Full | CPU -0.4460 to 9.3024, card -0.4460 to 9.3024; largest difference 1.97e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| add over the pushed card, opacity 1, Fractal Noise add, Draft | CPU 0.0391 to 8.0133, card 0.0391 to 8.0133; largest difference 1.32e-7 of the value (or of 1 under 1), solid 0.00e0, edge shift 0.0000 px, 0 levels at 8 bits | yes |
| add: the CPU's frames went past white and below black | -0.4460 to 9.3024 | yes |

## Result

531 of 531 checks pass.
