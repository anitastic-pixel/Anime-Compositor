# B-775 (D-895): Lens Blur by Fourier transform on the viewer's card

Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory. The card's Lens Blur with transparent edges and no blur map worked by Fourier transform at every radius, against the CPU's exact gather, both as the page receives them (8-bit straight sRGB). The bar (wave 8, D-894): within 1 level of 255 on at least 99.9% of the pixels a viewer can see and within 2 everywhere; a pixel fully transparent on both sides is not graded, and must look the same over black and over white.

## The reference shot at frame 100, D-222's Lens Blur on its first three layers

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius 25, Full | 3 Lens Blurs by transform; 1 frames, 2073600 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| radius 25, Draft | 3 Lens Blurs by transform; 1 frames, 129600 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| radius 50, Full | 3 Lens Blurs by transform; 1 frames, 2073600 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| radius 50, Draft | 3 Lens Blurs by transform; 1 frames, 129600 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| radius 100, Full | 3 Lens Blurs by transform; 1 frames, 2073600 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| radius 100, Draft | 3 Lens Blurs by transform; 1 frames, 129600 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| radius 200, Full | 3 Lens Blurs by transform; 1 frames, 2073600 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| radius 200, Draft | 3 Lens Blurs by transform; 1 frames, 129600 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |

## Every Lens Blur fixture, first and middle frames, Full and Draft

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lens_001 | 4 Lens Blurs by transform; 4 frames, 344 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_002 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_003 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_004 | 2 Lens Blurs by transform; 4 frames, 118 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 110 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_005 | 2 Lens Blurs by transform; 4 frames, 138 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 100 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_006 | 2 Lens Blurs by transform; 4 frames, 222 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 74 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_007 | 4 Lens Blurs by transform; 4 frames, 312 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 22 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_008 | 0 Lens Blurs by transform; 4 frames, 344 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_009 | 2 Lens Blurs by transform; 4 frames, 344 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_010 | 1 Lens Blurs by transform; 4 frames, 124 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 32 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_011 | 1 Lens Blurs by transform; 4 frames, 88 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 55 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_012 | 2 Lens Blurs by transform; 4 frames, 150 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 76 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_013 | 0 Lens Blurs by transform; 4 frames, 134 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_014 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_015 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_016 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_017 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_018 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_019 | 4 Lens Blurs by transform; 4 frames, 250 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 48 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_020 | 2 Lens Blurs by transform; 4 frames, 238 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 40 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_021 | 4 Lens Blurs by transform; 4 frames, 296 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 30 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_022 | 4 Lens Blurs by transform; 4 frames, 302 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 20 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_023 | 4 Lens Blurs by transform; 4 frames, 246 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 30 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_024 | 2 Lens Blurs by transform; 4 frames, 248 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 42 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_025 | 4 Lens Blurs by transform; 4 frames, 312 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 22 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_026 | 4 Lens Blurs by transform; 4 frames, 298 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 22 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_027 | 2 Lens Blurs by transform; 4 frames, 234 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 48 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_028 | 2 Lens Blurs by transform; 4 frames, 234 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 48 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_029 | 4 Lens Blurs by transform; 4 frames, 312 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 18 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_030 | 4 Lens Blurs by transform; 4 frames, 312 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 22 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_031 | 4 Lens Blurs by transform; 4 frames, 253 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 40 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_032 | 4 Lens Blurs by transform; 4 frames, 312 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 12 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_033 | 0 Lens Blurs by transform; 4 frames, 344 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_034 | 2 Lens Blurs by transform; 4 frames, 168 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 104 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_035 | 4 Lens Blurs by transform; 4 frames, 344 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_036 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_037 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_038 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_039 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_040 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_041 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_042 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_043 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_044 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_045 | 4 Lens Blurs by transform; 4 frames, 312 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 22 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_046 | 0 Lens Blurs by transform; 4 frames, 228 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_047 | 0 Lens Blurs by transform; 4 frames, 232 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_048 | 0 Lens Blurs by transform; 4 frames, 168 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_049 | 0 Lens Blurs by transform; 4 frames, 232 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_050 | 0 Lens Blurs by transform; 4 frames, 312 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_051 | 0 Lens Blurs by transform; 4 frames, 232 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_052 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_053 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_054 | 0 Lens Blurs by transform; 4 frames, 264 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_055 | 0 Lens Blurs by transform; 4 frames, 344 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_056 | 0 Lens Blurs by transform; 4 frames, 198 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_057 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_058 | 0 Lens Blurs by transform; 4 frames, 194 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_059 | 0 Lens Blurs by transform; 4 frames, 280 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_060 | 0 Lens Blurs by transform; 4 frames, 188 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_061 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_062 | 0 Lens Blurs by transform; 4 frames, 74 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_063 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_064 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_065 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_066 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_067 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_068 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| fx_lens_069 | 0 Lens Blurs by transform; 4 frames, 58 pixels seen: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; 0 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |
| all 69 fixtures together | 78 Lens Blurs by transform; 276 frames, 11845 pixels seen: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 1037 fully transparent pixels with different hidden colours not graded, largest difference over black or white 0 | yes |

## The switch

| Check | The build's answer | Matches |
| --- | --- | --- |
| as shipped (from radius 100), radius 50, Full | 0 Lens Blurs by transform (expected 0); largest 1 | yes |
| as shipped (from radius 100), radius 100, Full | 3 Lens Blurs by transform (expected 3); largest 1 | yes |
| as shipped (from radius 100), radius 200, Full | 3 Lens Blurs by transform (expected 3); largest 1 | yes |
| as shipped (from radius 100), radius 200, Draft | 0 Lens Blurs by transform (expected 0); largest 1 | yes |
| switched off, radius 200, Full | 0 Lens Blurs by transform; the exact gather, largest difference 1 | yes |

## Result

83 of 83 checks pass.
