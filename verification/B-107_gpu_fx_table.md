# B-107: twenty-nine of the third batch on the GPU against the CPU

Written by `tests/b107_gpu_fx.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU, with the layer's last effect of the twenty-nine done on the card. **The rule: no channel of any pixel more than 1 level of 255 apart** (D-165). An effect that changes nothing or whose settings are invalid is not left to the card; on those rows any difference is the card's layering, held to the same 1 level by D-100. On every row both paths must give the same warnings, and the card must draw the frame itself, except a frame with an adjustment layer, which the CPU draws by B-44's rule: that one must be the CPU's picture exactly, the card's message `GPU_PREVIEW_ON_CPU` its only extra warning.

**6802 of 6802 checks pass.**

The worst comparison is "the reference shot with Black & White frame 0, Full": largest difference 1 of 255, pixels differing: 33060. Its pictures are in `verification/B-107 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## Each effect

Every fixture frame of the effect and the reference shot with it, at Full and Draft.

| Effect | Frames compared | Frames with it on the card | Largest difference (of 255) | Pass |
|---|---:|---:|---:|---|
| Invert | 206 | 130 | 1 | 206 of 206 |
| Brightness & Contrast | 226 | 150 | 1 | 226 of 226 |
| Black & White | 226 | 166 | 1 | 226 of 226 |
| Posterize | 186 | 126 | 1 | 186 of 186 |
| Threshold | 186 | 146 | 0 | 186 of 186 |
| Channel Mixer | 226 | 150 | 1 | 226 of 226 |
| Vibrance | 186 | 120 | 1 | 186 of 186 |
| Leave Color | 226 | 152 | 1 | 226 of 226 |
| Solarize | 166 | 126 | 1 | 166 of 166 |
| Halftone | 256 | 162 | 1 | 256 of 256 |
| Mosaic | 206 | 101 | 1 | 206 of 206 |
| Emboss | 266 | 186 | 1 | 266 of 266 |
| Find Edges | 196 | 120 | 1 | 196 of 196 |
| Sharpen | 186 | 100 | 1 | 186 of 186 |
| Diffusion | 186 | 102 | 1 | 186 of 186 |
| Wave Warp | 246 | 154 | 1 | 246 of 246 |
| Ripple | 266 | 174 | 1 | 266 of 266 |
| Twirl | 216 | 130 | 1 | 216 of 216 |
| Bulge | 236 | 150 | 1 | 236 of 236 |
| Mirror | 246 | 186 | 1 | 246 of 246 |
| Linear Wipe | 316 | 224 | 1 | 316 of 316 |
| Radial Wipe | 296 | 202 | 1 | 296 of 296 |
| Venetian Blinds | 256 | 172 | 1 | 256 of 256 |
| Iris Wipe | 276 | 172 | 1 | 276 of 276 |
| Simple Choker | 176 | 122 | 1 | 176 of 176 |
| Speed Lines | 296 | 216 | 1 | 296 of 296 |
| Cross Glare | 296 | 191 | 1 | 296 of 296 |
| Camera Shake | 216 | 132 | 1 | 216 of 216 |
| Rain | 276 | 94 | 1 | 276 of 276 |

## Every frame

| Case | Effects left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---|---|
| fx_invert_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_invert_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_invert_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_invert_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_invert_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_invert_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_invert_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_invert_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_invert_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_invert_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_invert_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_invert_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_invert_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_011 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_invert_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_011 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_invert_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_012 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_invert_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_012 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_invert_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_013 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_invert_013 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_invert_013 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_invert_013 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_invert_013 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_invert_013 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_invert_013 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_invert_013 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_invert_013 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_invert_013 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_invert_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_invert_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_invert_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_invert_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_bricon_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_bricon_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_bricon_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_bricon_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_bricon_001 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_bricon_001 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_bricon_001 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_bricon_001 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_bricon_001 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_bricon_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_012 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_bricon_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_012 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_bricon_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_013 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_bricon_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_013 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_bricon_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_014 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_bricon_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_014 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_bricon_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bricon_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bricon_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bricon_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bw_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bw_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_017 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_017 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_017 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_017 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_017 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_018 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_018 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_018 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_018 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_018 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_019 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_019 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_019 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_019 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_019 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_020 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_020 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_020 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_020 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_020 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_021 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_021 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_021 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_021 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_021 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_022 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_022 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_022 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_022 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bw_022 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_009 frame 4, Full | 1 | 1 | 22 | none | PASS |
| fx_poster_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_009 frame 4, Draft | 1 | 1 | 1 | none | PASS |
| fx_poster_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_poster_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_poster_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_013 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_013 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_013 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_013 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_013 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_014 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_014 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_014 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_014 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_014 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_015 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_015 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_015 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_015 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_015 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_016 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_016 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_016 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_016 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_016 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_017 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_017 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_017 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_017 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_017 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_018 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_018 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_018 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_018 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_poster_018 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_thresh_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_thresh_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_thresh_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_mixer_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_mixer_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_mixer_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_mixer_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_mixer_001 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_mixer_001 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_mixer_001 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_mixer_001 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_mixer_001 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_mixer_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_005 frame 0, Full | 1 | 1 | 66 | none | PASS |
| fx_mixer_005 frame 1, Full | 1 | 1 | 66 | none | PASS |
| fx_mixer_005 frame 2, Full | 1 | 1 | 66 | none | PASS |
| fx_mixer_005 frame 3, Full | 1 | 1 | 66 | none | PASS |
| fx_mixer_005 frame 4, Full | 1 | 1 | 66 | none | PASS |
| fx_mixer_005 frame 0, Draft | 1 | 1 | 2 | none | PASS |
| fx_mixer_005 frame 1, Draft | 1 | 1 | 2 | none | PASS |
| fx_mixer_005 frame 2, Draft | 1 | 1 | 2 | none | PASS |
| fx_mixer_005 frame 3, Draft | 1 | 1 | 2 | none | PASS |
| fx_mixer_005 frame 4, Draft | 1 | 1 | 2 | none | PASS |
| fx_mixer_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_012 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_mixer_012 frame 1, Full | 1 | 1 | 4 | none | PASS |
| fx_mixer_012 frame 2, Full | 1 | 1 | 12 | none | PASS |
| fx_mixer_012 frame 3, Full | 1 | 1 | 4 | none | PASS |
| fx_mixer_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_012 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_mixer_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_012 frame 2, Draft | 1 | 1 | 1 | none | PASS |
| fx_mixer_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_013 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_mixer_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_013 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_mixer_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_014 frame 1, Full | 1 | 1 | 42 | none | PASS |
| fx_mixer_014 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_mixer_014 frame 3, Full | 1 | 1 | 42 | none | PASS |
| fx_mixer_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_014 frame 1, Draft | 1 | 1 | 1 | none | PASS |
| fx_mixer_014 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_mixer_014 frame 3, Draft | 1 | 1 | 1 | none | PASS |
| fx_mixer_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mixer_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mixer_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_017 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_017 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_017 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_017 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_017 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_018 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_018 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_018 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_018 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_018 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_019 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_019 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_019 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_019 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_019 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_020 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_020 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_020 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_020 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_020 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_021 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_021 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_021 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_021 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_021 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_022 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_022 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_022 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_022 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mixer_022 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_vibrance_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_vibrance_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_vibrance_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_vibrance_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_vibrance_001 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_vibrance_001 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_vibrance_001 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_vibrance_001 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_vibrance_001 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_vibrance_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_vibrance_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_vibrance_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_011 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_vibrance_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_011 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_vibrance_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_012 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_vibrance_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_012 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_vibrance_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vibrance_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vibrance_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vibrance_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_leave_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_leave_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_leave_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_leave_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_leave_002 frame 0, Draft | 0 | 1 | 3 | none | PASS |
| fx_leave_002 frame 1, Draft | 0 | 1 | 3 | none | PASS |
| fx_leave_002 frame 2, Draft | 0 | 1 | 3 | none | PASS |
| fx_leave_002 frame 3, Draft | 0 | 1 | 3 | none | PASS |
| fx_leave_002 frame 4, Draft | 0 | 1 | 3 | none | PASS |
| fx_leave_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_014 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_leave_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_014 frame 0, Draft | 0 | 1 | 3 | none | PASS |
| fx_leave_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_015 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_leave_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_015 frame 0, Draft | 0 | 1 | 3 | none | PASS |
| fx_leave_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_leave_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_leave_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_017 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_017 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_017 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_017 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_017 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_018 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_018 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_018 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_018 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_018 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_019 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_019 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_019 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_019 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_019 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_020 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_020 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_020 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_020 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_020 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_021 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_021 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_021 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_021 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_021 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_022 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_022 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_022 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_022 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_leave_022 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_solar_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_solar_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_013 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_013 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_013 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_013 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_013 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_solar_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_halftone_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_halftone_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_halftone_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_halftone_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_halftone_003 frame 0, Draft | 0 | 1 | 1 | none | PASS |
| fx_halftone_003 frame 1, Draft | 0 | 1 | 1 | none | PASS |
| fx_halftone_003 frame 2, Draft | 0 | 1 | 1 | none | PASS |
| fx_halftone_003 frame 3, Draft | 0 | 1 | 1 | none | PASS |
| fx_halftone_003 frame 4, Draft | 0 | 1 | 1 | none | PASS |
| fx_halftone_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_011 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_halftone_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_011 frame 0, Draft | 0 | 1 | 1 | none | PASS |
| fx_halftone_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_014 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_halftone_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_014 frame 0, Draft | 0 | 1 | 1 | none | PASS |
| fx_halftone_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_halftone_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_halftone_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_018 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_018 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_018 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_018 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_018 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_019 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_019 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_019 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_019 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_019 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_020 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_020 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_020 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_020 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_020 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_021 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_021 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_021 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_021 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_021 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_022 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_022 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_022 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_022 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_022 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_023 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_023 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_023 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_023 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_023 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_024 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_024 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_024 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_024 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_024 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_025 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_025 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_025 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_025 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_halftone_025 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_mosaic_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_mosaic_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_mosaic_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_mosaic_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_mosaic_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_003 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_003 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_003 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_003 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_003 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_004 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_004 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_004 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_004 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_004 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_005 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_005 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_005 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_005 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_005 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_006 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_006 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_006 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_006 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_006 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_010 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_010 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_011 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_mosaic_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_011 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_012 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_012 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_012 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_012 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_012 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_013 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_013 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_013 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_013 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_013 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_mosaic_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mosaic_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mosaic_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mosaic_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_001 frame 0, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_001 frame 1, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_001 frame 2, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_001 frame 3, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_001 frame 4, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_009 frame 0, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_009 frame 1, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_009 frame 2, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_009 frame 3, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_009 frame 4, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_010 frame 0, Full | 1 | 1 | 14 | none | PASS |
| fx_emboss_010 frame 1, Full | 1 | 1 | 14 | none | PASS |
| fx_emboss_010 frame 2, Full | 1 | 1 | 14 | none | PASS |
| fx_emboss_010 frame 3, Full | 1 | 1 | 14 | none | PASS |
| fx_emboss_010 frame 4, Full | 1 | 1 | 14 | none | PASS |
| fx_emboss_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_013 frame 3, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_014 frame 2, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_015 frame 0, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_015 frame 1, Full | 1 | 1 | 14 | none | PASS |
| fx_emboss_015 frame 2, Full | 1 | 1 | 14 | none | PASS |
| fx_emboss_015 frame 3, Full | 1 | 1 | 14 | none | PASS |
| fx_emboss_015 frame 4, Full | 1 | 1 | 14 | none | PASS |
| fx_emboss_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_016 frame 0, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_016 frame 1, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_016 frame 2, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_016 frame 3, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_016 frame 4, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_017 frame 0, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_017 frame 1, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_017 frame 2, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_017 frame 3, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_017 frame 4, Full | 1 | 1 | 2 | none | PASS |
| fx_emboss_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_emboss_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_emboss_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_019 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_019 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_019 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_019 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_019 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_020 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_020 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_020 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_020 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_020 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_021 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_021 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_021 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_021 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_021 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_022 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_022 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_022 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_022 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_022 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_023 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_023 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_023 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_023 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_023 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_024 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_024 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_024 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_024 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_024 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_025 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_025 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_025 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_025 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_025 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_026 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_026 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_026 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_026 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_emboss_026 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_findedges_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_findedges_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_findedges_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_findedges_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_findedges_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_findedges_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_findedges_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_findedges_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_findedges_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_findedges_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_004 frame 0, Full | 1 | 1 | 2 | none | PASS |
| fx_findedges_004 frame 1, Full | 1 | 1 | 2 | none | PASS |
| fx_findedges_004 frame 2, Full | 1 | 1 | 2 | none | PASS |
| fx_findedges_004 frame 3, Full | 1 | 1 | 2 | none | PASS |
| fx_findedges_004 frame 4, Full | 1 | 1 | 2 | none | PASS |
| fx_findedges_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_007 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_findedges_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_007 frame 2, Full | 1 | 1 | 2 | none | PASS |
| fx_findedges_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_007 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_findedges_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_008 frame 3, Full | 1 | 1 | 2 | none | PASS |
| fx_findedges_008 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_findedges_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_008 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_findedges_009 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_findedges_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_009 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_findedges_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_findedges_010 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_findedges_010 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_findedges_010 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_findedges_010 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_findedges_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_findedges_010 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_findedges_010 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_findedges_010 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_findedges_010 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_findedges_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_findedges_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_findedges_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_findedges_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_sharpen_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_sharpen_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_sharpen_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_sharpen_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_sharpen_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_sharpen_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_sharpen_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_sharpen_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_sharpen_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_sharpen_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_sharpen_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_sharpen_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_sharpen_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_sharpen_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_sharpen_003 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_sharpen_003 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_sharpen_003 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_sharpen_003 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_sharpen_003 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_sharpen_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_008 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_sharpen_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_008 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_sharpen_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_009 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_sharpen_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_009 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_sharpen_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_sharpen_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_sharpen_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_sharpen_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_sharpen_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_013 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_013 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_013 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_013 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_013 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_sharpen_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_diffuse_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_diffuse_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_diffuse_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_diffuse_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_diffuse_003 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_diffuse_003 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_diffuse_003 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_diffuse_003 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_diffuse_003 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_diffuse_004 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_diffuse_004 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_diffuse_004 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_diffuse_004 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_diffuse_004 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_diffuse_004 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_diffuse_004 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_diffuse_004 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_diffuse_004 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_diffuse_004 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_diffuse_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_diffuse_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_diffuse_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_011 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_diffuse_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_011 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_diffuse_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_diffuse_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_diffuse_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_013 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_013 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_013 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_013 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_013 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_diffuse_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_wave_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_wave_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_wave_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_wave_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_wave_002 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_wave_002 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_wave_002 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_wave_002 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_wave_002 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_wave_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_012 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_wave_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_012 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_wave_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wave_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wave_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_017 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_017 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_017 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_017 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_017 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_018 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_018 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_018 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_018 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_018 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_019 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_019 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_019 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_019 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_019 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_020 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_020 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_020 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_020 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_020 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_021 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_021 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_021 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_021 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_021 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_022 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_022 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_022 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_022 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_022 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_023 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_023 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_023 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_023 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_023 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_024 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_024 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_024 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_024 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wave_024 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_ripple_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_ripple_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_ripple_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_ripple_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_ripple_002 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_ripple_002 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_ripple_002 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_ripple_002 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_ripple_002 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_ripple_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_013 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_ripple_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_013 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_ripple_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_ripple_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_ripple_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_019 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_019 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_019 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_019 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_019 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_020 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_020 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_020 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_020 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_020 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_021 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_021 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_021 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_021 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_021 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_022 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_022 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_022 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_022 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_022 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_023 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_023 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_023 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_023 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_023 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_024 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_024 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_024 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_024 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_024 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_025 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_025 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_025 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_025 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_025 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_026 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_026 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_026 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_026 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_ripple_026 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_twirl_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_twirl_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_twirl_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_twirl_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_twirl_003 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_twirl_003 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_twirl_003 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_twirl_003 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_twirl_003 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_twirl_004 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_twirl_004 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_twirl_004 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_twirl_004 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_twirl_004 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_twirl_004 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_twirl_004 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_twirl_004 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_twirl_004 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_twirl_004 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_twirl_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_twirl_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_twirl_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_011 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_twirl_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_011 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_twirl_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_013 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_twirl_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_013 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_twirl_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_twirl_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_015 frame 0, Full | 1 | 1 | 6 | none | PASS |
| fx_twirl_015 frame 1, Full | 1 | 1 | 6 | none | PASS |
| fx_twirl_015 frame 2, Full | 1 | 1 | 6 | none | PASS |
| fx_twirl_015 frame 3, Full | 1 | 1 | 6 | none | PASS |
| fx_twirl_015 frame 4, Full | 1 | 1 | 6 | none | PASS |
| fx_twirl_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_twirl_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_twirl_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_bulge_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_bulge_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_bulge_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_bulge_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_bulge_002 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_bulge_002 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_bulge_002 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_bulge_002 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_bulge_002 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_bulge_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_bulge_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_bulge_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_bulge_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_bulge_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_bulge_003 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_bulge_003 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_bulge_003 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_bulge_003 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_bulge_003 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_bulge_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_011 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_bulge_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_011 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_bulge_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_012 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_bulge_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_012 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_bulge_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_014 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_bulge_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_014 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_bulge_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bulge_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bulge_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_018 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_018 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_018 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_018 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_018 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_019 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_019 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_019 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_019 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_019 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_020 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_020 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_020 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_020 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_020 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_021 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_021 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_021 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_021 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_021 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_022 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_022 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_022 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_022 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_022 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_023 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_023 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_023 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_023 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bulge_023 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_mirror_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_mirror_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_mirror_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_lwipe_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_lwipe_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_lwipe_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_lwipe_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_lwipe_001 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_lwipe_001 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_lwipe_001 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_lwipe_001 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_lwipe_001 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_lwipe_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_014 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_lwipe_014 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_lwipe_014 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_lwipe_014 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_lwipe_014 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_lwipe_014 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_lwipe_014 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_lwipe_014 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_lwipe_014 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_lwipe_014 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_lwipe_015 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_lwipe_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_015 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_lwipe_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_019 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_020 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_020 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_020 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_020 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_020 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_020 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_021 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_021 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_021 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_021 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_021 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_021 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_021 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_021 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_021 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_021 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_022 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_022 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_022 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_022 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_022 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_022 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_022 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_022 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_022 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_022 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_023 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_023 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_023 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_023 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_023 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_023 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_023 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_023 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_023 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_023 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_024 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_024 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_024 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_024 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_024 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lwipe_024 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_024 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_024 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_024 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_024 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lwipe_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_028 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_028 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_028 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_028 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_028 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_028 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_028 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_028 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_028 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_028 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_029 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_029 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_029 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_029 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_029 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_029 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_029 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_029 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_029 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_029 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_030 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_030 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_030 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_030 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_030 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_030 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_030 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_030 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_030 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_030 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_031 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_031 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_031 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_031 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_031 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_031 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_031 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_031 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_031 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lwipe_031 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_rwipe_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_rwipe_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_rwipe_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_rwipe_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_rwipe_001 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_rwipe_001 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_rwipe_001 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_rwipe_001 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_rwipe_001 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_rwipe_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_014 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_rwipe_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_014 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_rwipe_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_019 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_rwipe_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_019 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_rwipe_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_020 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_020 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_020 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_020 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_020 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_020 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_021 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_021 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_021 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_021 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_021 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rwipe_021 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_021 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_021 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_021 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_021 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rwipe_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_028 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_028 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_028 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_028 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_028 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_028 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_028 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_028 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_028 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_028 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_029 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_029 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_029 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_029 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_029 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_029 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_029 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_029 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_029 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rwipe_029 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_blinds_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_blinds_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_blinds_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_blinds_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_blinds_001 frame 0, Draft | 0 | 1 | 4 | none | PASS |
| fx_blinds_001 frame 1, Draft | 0 | 1 | 4 | none | PASS |
| fx_blinds_001 frame 2, Draft | 0 | 1 | 4 | none | PASS |
| fx_blinds_001 frame 3, Draft | 0 | 1 | 4 | none | PASS |
| fx_blinds_001 frame 4, Draft | 0 | 1 | 4 | none | PASS |
| fx_blinds_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_014 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_blinds_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_014 frame 0, Draft | 0 | 1 | 4 | none | PASS |
| fx_blinds_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_016 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_blinds_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_016 frame 0, Draft | 0 | 1 | 4 | none | PASS |
| fx_blinds_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_blinds_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_blinds_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_019 frame 0, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_019 frame 1, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_019 frame 2, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_019 frame 3, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_019 frame 4, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_020 frame 0, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_020 frame 1, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_020 frame 2, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_020 frame 3, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_020 frame 4, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_021 frame 0, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_021 frame 1, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_021 frame 2, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_021 frame 3, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_021 frame 4, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_022 frame 0, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_022 frame 1, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_022 frame 2, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_022 frame 3, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_022 frame 4, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_023 frame 0, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_023 frame 1, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_023 frame 2, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_023 frame 3, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_023 frame 4, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_024 frame 0, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_024 frame 1, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_024 frame 2, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_024 frame 3, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_024 frame 4, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_025 frame 0, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_025 frame 1, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_025 frame 2, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_025 frame 3, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_blinds_025 frame 4, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_iris_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_iris_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_iris_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_iris_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_iris_001 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_iris_001 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_iris_001 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_iris_001 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_iris_001 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_iris_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_011 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_iris_011 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_iris_011 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_iris_011 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_iris_011 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_iris_011 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_iris_011 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_iris_011 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_iris_011 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_iris_011 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_iris_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_015 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_iris_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_015 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_iris_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_018 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_iris_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_018 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_iris_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_iris_019 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_iris_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_020 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_020 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_020 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_020 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_020 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_021 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_021 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_021 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_021 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_021 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_022 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_022 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_022 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_022 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_022 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_023 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_023 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_023 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_023 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_023 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_024 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_024 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_024 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_024 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_024 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_025 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_025 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_025 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_025 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_025 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_026 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_026 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_026 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_026 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_026 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_027 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_027 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_027 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_027 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_iris_027 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_choke_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_choke_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_choke_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_choke_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_choke_001 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_choke_001 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_choke_001 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_choke_001 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_choke_001 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_choke_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_009 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_choke_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_009 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_choke_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_choke_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_choke_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_choke_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_choke_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_choke_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_019 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_020 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_020 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_020 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_020 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_020 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_020 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_021 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_021 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_021 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_021 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_021 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_speed_021 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_021 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_021 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_021 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_021 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_speed_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_028 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_028 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_028 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_028 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_028 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_028 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_028 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_028 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_028 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_028 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_029 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_029 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_029 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_029 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_029 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_029 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_029 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_029 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_029 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_speed_029 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_glare_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_glare_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_glare_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_glare_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_glare_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_glare_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_glare_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_glare_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_glare_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_glare_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_glare_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_glare_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_glare_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_glare_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_glare_003 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_glare_003 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_glare_003 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_glare_003 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_glare_003 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_glare_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_018 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_glare_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_018 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_glare_018 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_glare_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_019 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_020 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_glare_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_020 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_glare_020 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_020 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_020 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_020 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_021 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_021 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_021 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_021 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_021 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glare_021 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_021 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_021 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_021 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_021 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glare_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_028 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_028 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_028 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_028 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_028 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_028 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_028 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_028 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_028 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_028 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_029 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_029 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_029 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_029 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_029 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_029 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_029 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_029 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_029 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glare_029 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_shake_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_shake_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_shake_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_shake_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_shake_002 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_shake_002 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_shake_002 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_shake_002 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_shake_002 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_shake_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_011 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_shake_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_011 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_shake_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_012 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_shake_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_012 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_shake_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shake_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shake_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_015 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_015 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_015 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_015 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_015 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_016 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_016 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_016 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_016 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_016 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_017 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_017 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_017 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_017 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_017 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_018 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_018 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_018 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_018 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_018 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_019 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_019 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_019 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_019 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_019 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_020 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_020 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_020 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_020 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_020 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_021 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_021 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_021 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_021 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shake_021 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rain_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rain_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rain_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rain_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rain_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_002 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_002 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_002 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_002 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_002 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_rain_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_rain_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_rain_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_rain_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_rain_003 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_003 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_003 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_003 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_003 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_004 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_004 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_004 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_004 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_004 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_005 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_005 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_005 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_005 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_005 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_006 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_rain_006 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_rain_006 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_rain_006 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_rain_006 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_rain_006 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_006 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_006 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_006 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_006 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_007 frame 2, Full | 1 | 1 | 1 | none | PASS |
| fx_rain_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_007 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_007 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_007 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_007 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_007 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_008 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_008 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_008 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_008 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_008 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_009 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_009 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_009 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_009 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_009 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_010 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_010 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_010 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_010 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_010 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_011 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_011 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_011 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_011 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_011 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_012 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_012 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_012 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_012 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_012 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_013 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_013 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_013 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_013 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_013 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_016 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_rain_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_017 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_rain_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rain_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rain_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| the reference shot with Invert frame 0, Full | 3 | 1 | 208 | none | PASS |
| the reference shot with Invert frame 100, Full | 3 | 1 | 230 | none | PASS |
| the reference shot with Invert frame 239, Full | 3 | 1 | 144 | none | PASS |
| the reference shot with Invert frame 0, Draft | 3 | 1 | 1132 | none | PASS |
| the reference shot with Invert frame 100, Draft | 3 | 1 | 898 | none | PASS |
| the reference shot with Invert frame 239, Draft | 3 | 1 | 1148 | none | PASS |
| the reference shot with Brightness & Contrast frame 0, Full | 3 | 1 | 6447 | none | PASS |
| the reference shot with Brightness & Contrast frame 100, Full | 3 | 1 | 6554 | none | PASS |
| the reference shot with Brightness & Contrast frame 239, Full | 3 | 1 | 6345 | none | PASS |
| the reference shot with Brightness & Contrast frame 0, Draft | 3 | 1 | 32 | none | PASS |
| the reference shot with Brightness & Contrast frame 100, Draft | 3 | 1 | 31 | none | PASS |
| the reference shot with Brightness & Contrast frame 239, Draft | 3 | 1 | 31 | none | PASS |
| the reference shot with Black & White frame 0, Full | 3 | 1 | 33060 | none | PASS |
| the reference shot with Black & White frame 100, Full | 3 | 1 | 30045 | none | PASS |
| the reference shot with Black & White frame 239, Full | 3 | 1 | 31298 | none | PASS |
| the reference shot with Black & White frame 0, Draft | 3 | 1 | 418 | none | PASS |
| the reference shot with Black & White frame 100, Draft | 3 | 1 | 252 | none | PASS |
| the reference shot with Black & White frame 239, Draft | 3 | 1 | 334 | none | PASS |
| the reference shot with Posterize frame 0, Full | 3 | 0 | 0 | none | PASS |
| the reference shot with Posterize frame 100, Full | 3 | 1 | 1 | none | PASS |
| the reference shot with Posterize frame 239, Full | 3 | 1 | 1 | none | PASS |
| the reference shot with Posterize frame 0, Draft | 3 | 0 | 0 | none | PASS |
| the reference shot with Posterize frame 100, Draft | 3 | 0 | 0 | none | PASS |
| the reference shot with Posterize frame 239, Draft | 3 | 0 | 0 | none | PASS |
| the reference shot with Threshold frame 0, Full | 3 | 0 | 0 | none | PASS |
| the reference shot with Threshold frame 100, Full | 3 | 0 | 0 | none | PASS |
| the reference shot with Threshold frame 239, Full | 3 | 0 | 0 | none | PASS |
| the reference shot with Threshold frame 0, Draft | 3 | 0 | 0 | none | PASS |
| the reference shot with Threshold frame 100, Draft | 3 | 0 | 0 | none | PASS |
| the reference shot with Threshold frame 239, Draft | 3 | 0 | 0 | none | PASS |
| the reference shot with Channel Mixer frame 0, Full | 3 | 1 | 4924 | none | PASS |
| the reference shot with Channel Mixer frame 100, Full | 3 | 1 | 3121 | none | PASS |
| the reference shot with Channel Mixer frame 239, Full | 3 | 1 | 3895 | none | PASS |
| the reference shot with Channel Mixer frame 0, Draft | 3 | 1 | 145 | none | PASS |
| the reference shot with Channel Mixer frame 100, Draft | 3 | 1 | 56 | none | PASS |
| the reference shot with Channel Mixer frame 239, Draft | 3 | 1 | 107 | none | PASS |
| the reference shot with Vibrance frame 0, Full | 3 | 1 | 2754 | none | PASS |
| the reference shot with Vibrance frame 100, Full | 3 | 1 | 673 | none | PASS |
| the reference shot with Vibrance frame 239, Full | 3 | 1 | 942 | none | PASS |
| the reference shot with Vibrance frame 0, Draft | 3 | 1 | 116 | none | PASS |
| the reference shot with Vibrance frame 100, Draft | 3 | 1 | 39 | none | PASS |
| the reference shot with Vibrance frame 239, Draft | 3 | 1 | 40 | none | PASS |
| the reference shot with Leave Color frame 0, Full | 3 | 1 | 3577 | none | PASS |
| the reference shot with Leave Color frame 100, Full | 3 | 1 | 1471 | none | PASS |
| the reference shot with Leave Color frame 239, Full | 3 | 1 | 1762 | none | PASS |
| the reference shot with Leave Color frame 0, Draft | 3 | 1 | 147 | none | PASS |
| the reference shot with Leave Color frame 100, Draft | 3 | 1 | 74 | none | PASS |
| the reference shot with Leave Color frame 239, Draft | 3 | 1 | 83 | none | PASS |
| the reference shot with Solarize frame 0, Full | 3 | 1 | 5716 | none | PASS |
| the reference shot with Solarize frame 100, Full | 3 | 1 | 4221 | none | PASS |
| the reference shot with Solarize frame 239, Full | 3 | 1 | 3979 | none | PASS |
| the reference shot with Solarize frame 0, Draft | 3 | 1 | 135 | none | PASS |
| the reference shot with Solarize frame 100, Draft | 3 | 1 | 89 | none | PASS |
| the reference shot with Solarize frame 239, Draft | 3 | 1 | 62 | none | PASS |
| the reference shot with Halftone frame 0, Full | 3 | 0 | 0 | none | PASS |
| the reference shot with Halftone frame 100, Full | 3 | 0 | 0 | none | PASS |
| the reference shot with Halftone frame 239, Full | 3 | 0 | 0 | none | PASS |
| the reference shot with Halftone frame 0, Draft | 3 | 0 | 0 | none | PASS |
| the reference shot with Halftone frame 100, Draft | 3 | 0 | 0 | none | PASS |
| the reference shot with Halftone frame 239, Draft | 3 | 0 | 0 | none | PASS |
| the reference shot with Mosaic frame 0, Full | 3 | 1 | 510 | none | PASS |
| the reference shot with Mosaic frame 100, Full | 3 | 1 | 1200 | none | PASS |
| the reference shot with Mosaic frame 239, Full | 3 | 1 | 380 | none | PASS |
| the reference shot with Mosaic frame 0, Draft | 3 | 1 | 70 | none | PASS |
| the reference shot with Mosaic frame 100, Draft | 3 | 1 | 39 | none | PASS |
| the reference shot with Mosaic frame 239, Draft | 3 | 1 | 50 | none | PASS |
| the reference shot with Emboss frame 0, Full | 3 | 1 | 12645 | none | PASS |
| the reference shot with Emboss frame 100, Full | 3 | 1 | 13148 | none | PASS |
| the reference shot with Emboss frame 239, Full | 3 | 1 | 12500 | none | PASS |
| the reference shot with Emboss frame 0, Draft | 3 | 1 | 276 | none | PASS |
| the reference shot with Emboss frame 100, Draft | 3 | 1 | 247 | none | PASS |
| the reference shot with Emboss frame 239, Draft | 3 | 1 | 235 | none | PASS |
| the reference shot with Find Edges frame 0, Full | 3 | 1 | 692 | none | PASS |
| the reference shot with Find Edges frame 100, Full | 3 | 1 | 839 | none | PASS |
| the reference shot with Find Edges frame 239, Full | 3 | 1 | 748 | none | PASS |
| the reference shot with Find Edges frame 0, Draft | 3 | 1 | 70 | none | PASS |
| the reference shot with Find Edges frame 100, Draft | 3 | 1 | 56 | none | PASS |
| the reference shot with Find Edges frame 239, Draft | 3 | 1 | 68 | none | PASS |
| the reference shot with Sharpen frame 0, Full | 3 | 1 | 1171 | none | PASS |
| the reference shot with Sharpen frame 100, Full | 3 | 1 | 841 | none | PASS |
| the reference shot with Sharpen frame 239, Full | 3 | 1 | 720 | none | PASS |
| the reference shot with Sharpen frame 0, Draft | 3 | 1 | 193 | none | PASS |
| the reference shot with Sharpen frame 100, Draft | 3 | 1 | 135 | none | PASS |
| the reference shot with Sharpen frame 239, Draft | 3 | 1 | 133 | none | PASS |
| the reference shot with Diffusion frame 0, Full | 3 | 1 | 869 | none | PASS |
| the reference shot with Diffusion frame 100, Full | 3 | 1 | 861 | none | PASS |
| the reference shot with Diffusion frame 239, Full | 3 | 1 | 726 | none | PASS |
| the reference shot with Diffusion frame 0, Draft | 3 | 1 | 69 | none | PASS |
| the reference shot with Diffusion frame 100, Draft | 3 | 1 | 59 | none | PASS |
| the reference shot with Diffusion frame 239, Draft | 3 | 1 | 55 | none | PASS |
| the reference shot with Wave Warp frame 0, Full | 3 | 1 | 2137 | none | PASS |
| the reference shot with Wave Warp frame 100, Full | 3 | 1 | 953 | none | PASS |
| the reference shot with Wave Warp frame 239, Full | 3 | 1 | 910 | none | PASS |
| the reference shot with Wave Warp frame 0, Draft | 3 | 1 | 86 | none | PASS |
| the reference shot with Wave Warp frame 100, Draft | 3 | 1 | 60 | none | PASS |
| the reference shot with Wave Warp frame 239, Draft | 3 | 1 | 62 | none | PASS |
| the reference shot with Ripple frame 0, Full | 3 | 1 | 1981 | none | PASS |
| the reference shot with Ripple frame 100, Full | 3 | 1 | 960 | none | PASS |
| the reference shot with Ripple frame 239, Full | 3 | 1 | 776 | none | PASS |
| the reference shot with Ripple frame 0, Draft | 3 | 1 | 75 | none | PASS |
| the reference shot with Ripple frame 100, Draft | 3 | 1 | 50 | none | PASS |
| the reference shot with Ripple frame 239, Draft | 3 | 1 | 47 | none | PASS |
| the reference shot with Twirl frame 0, Full | 3 | 1 | 3733 | none | PASS |
| the reference shot with Twirl frame 100, Full | 3 | 1 | 1411 | none | PASS |
| the reference shot with Twirl frame 239, Full | 3 | 1 | 1722 | none | PASS |
| the reference shot with Twirl frame 0, Draft | 3 | 1 | 126 | none | PASS |
| the reference shot with Twirl frame 100, Draft | 3 | 1 | 32 | none | PASS |
| the reference shot with Twirl frame 239, Draft | 3 | 1 | 45 | none | PASS |
| the reference shot with Bulge frame 0, Full | 3 | 1 | 3737 | none | PASS |
| the reference shot with Bulge frame 100, Full | 3 | 1 | 1410 | none | PASS |
| the reference shot with Bulge frame 239, Full | 3 | 1 | 1726 | none | PASS |
| the reference shot with Bulge frame 0, Draft | 3 | 1 | 130 | none | PASS |
| the reference shot with Bulge frame 100, Draft | 3 | 1 | 34 | none | PASS |
| the reference shot with Bulge frame 239, Draft | 3 | 1 | 49 | none | PASS |
| the reference shot with Mirror frame 0, Full | 3 | 1 | 5632 | none | PASS |
| the reference shot with Mirror frame 100, Full | 3 | 1 | 10051 | none | PASS |
| the reference shot with Mirror frame 239, Full | 3 | 1 | 6004 | none | PASS |
| the reference shot with Mirror frame 0, Draft | 3 | 1 | 210 | none | PASS |
| the reference shot with Mirror frame 100, Draft | 3 | 1 | 422 | none | PASS |
| the reference shot with Mirror frame 239, Draft | 3 | 1 | 235 | none | PASS |
| the reference shot with Linear Wipe frame 0, Full | 3 | 1 | 2830 | none | PASS |
| the reference shot with Linear Wipe frame 100, Full | 3 | 0 | 0 | none | PASS |
| the reference shot with Linear Wipe frame 239, Full | 3 | 1 | 538 | none | PASS |
| the reference shot with Linear Wipe frame 0, Draft | 3 | 1 | 105 | none | PASS |
| the reference shot with Linear Wipe frame 100, Draft | 3 | 0 | 0 | none | PASS |
| the reference shot with Linear Wipe frame 239, Draft | 3 | 1 | 17 | none | PASS |
| the reference shot with Radial Wipe frame 0, Full | 3 | 1 | 953 | none | PASS |
| the reference shot with Radial Wipe frame 100, Full | 3 | 1 | 1417 | none | PASS |
| the reference shot with Radial Wipe frame 239, Full | 3 | 1 | 1234 | none | PASS |
| the reference shot with Radial Wipe frame 0, Draft | 3 | 1 | 23 | none | PASS |
| the reference shot with Radial Wipe frame 100, Draft | 3 | 1 | 32 | none | PASS |
| the reference shot with Radial Wipe frame 239, Draft | 3 | 1 | 30 | none | PASS |
| the reference shot with Venetian Blinds frame 0, Full | 3 | 1 | 1877 | none | PASS |
| the reference shot with Venetian Blinds frame 100, Full | 3 | 1 | 722 | none | PASS |
| the reference shot with Venetian Blinds frame 239, Full | 3 | 1 | 930 | none | PASS |
| the reference shot with Venetian Blinds frame 0, Draft | 3 | 1 | 79 | none | PASS |
| the reference shot with Venetian Blinds frame 100, Draft | 3 | 1 | 15 | none | PASS |
| the reference shot with Venetian Blinds frame 239, Draft | 3 | 1 | 29 | none | PASS |
| the reference shot with Iris Wipe frame 0, Full | 3 | 1 | 3782 | none | PASS |
| the reference shot with Iris Wipe frame 100, Full | 3 | 1 | 1417 | none | PASS |
| the reference shot with Iris Wipe frame 239, Full | 3 | 1 | 1772 | none | PASS |
| the reference shot with Iris Wipe frame 0, Draft | 3 | 1 | 128 | none | PASS |
| the reference shot with Iris Wipe frame 100, Draft | 3 | 1 | 32 | none | PASS |
| the reference shot with Iris Wipe frame 239, Draft | 3 | 1 | 47 | none | PASS |
| the reference shot with Simple Choker frame 0, Full | 3 | 1 | 3781 | none | PASS |
| the reference shot with Simple Choker frame 100, Full | 3 | 1 | 1417 | none | PASS |
| the reference shot with Simple Choker frame 239, Full | 3 | 1 | 1772 | none | PASS |
| the reference shot with Simple Choker frame 0, Draft | 3 | 1 | 128 | none | PASS |
| the reference shot with Simple Choker frame 100, Draft | 3 | 1 | 32 | none | PASS |
| the reference shot with Simple Choker frame 239, Draft | 3 | 1 | 47 | none | PASS |
| the reference shot with Speed Lines frame 0, Full | 3 | 1 | 2850 | none | PASS |
| the reference shot with Speed Lines frame 100, Full | 3 | 1 | 990 | none | PASS |
| the reference shot with Speed Lines frame 239, Full | 3 | 1 | 1445 | none | PASS |
| the reference shot with Speed Lines frame 0, Draft | 3 | 1 | 94 | none | PASS |
| the reference shot with Speed Lines frame 100, Draft | 3 | 1 | 48 | none | PASS |
| the reference shot with Speed Lines frame 239, Draft | 3 | 1 | 70 | none | PASS |
| the reference shot with Cross Glare frame 0, Full | 3 | 1 | 2201 | none | PASS |
| the reference shot with Cross Glare frame 100, Full | 3 | 1 | 733 | none | PASS |
| the reference shot with Cross Glare frame 239, Full | 3 | 1 | 623 | none | PASS |
| the reference shot with Cross Glare frame 0, Draft | 3 | 1 | 100 | none | PASS |
| the reference shot with Cross Glare frame 100, Draft | 3 | 1 | 48 | none | PASS |
| the reference shot with Cross Glare frame 239, Draft | 3 | 1 | 27 | none | PASS |
| the reference shot with Camera Shake frame 0, Full | 3 | 1 | 1735 | none | PASS |
| the reference shot with Camera Shake frame 100, Full | 3 | 1 | 906 | none | PASS |
| the reference shot with Camera Shake frame 239, Full | 3 | 1 | 777 | none | PASS |
| the reference shot with Camera Shake frame 0, Draft | 3 | 1 | 58 | none | PASS |
| the reference shot with Camera Shake frame 100, Draft | 3 | 1 | 58 | none | PASS |
| the reference shot with Camera Shake frame 239, Draft | 3 | 1 | 67 | none | PASS |
| the reference shot with Rain frame 0, Full | 3 | 1 | 3721 | none | PASS |
| the reference shot with Rain frame 100, Full | 3 | 1 | 1414 | none | PASS |
| the reference shot with Rain frame 239, Full | 3 | 1 | 1759 | none | PASS |
| the reference shot with Rain frame 0, Draft | 3 | 1 | 124 | none | PASS |
| the reference shot with Rain frame 100, Draft | 3 | 1 | 39 | none | PASS |
| the reference shot with Rain frame 239, Draft | 3 | 1 | 49 | none | PASS |
| the reference shot with Invert frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Invert frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Brightness & Contrast frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Brightness & Contrast frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Black & White frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Black & White frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Posterize frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Posterize frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Threshold frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Threshold frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Channel Mixer frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Channel Mixer frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Vibrance frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Vibrance frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Leave Color frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Leave Color frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Solarize frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Solarize frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Halftone frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Halftone frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Mosaic frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Mosaic frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Emboss frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Emboss frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Find Edges frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Find Edges frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Sharpen frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Sharpen frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Diffusion frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Diffusion frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Wave Warp frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Wave Warp frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Ripple frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Ripple frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Twirl frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Twirl frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Bulge frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Bulge frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Mirror frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Mirror frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Linear Wipe frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Linear Wipe frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Radial Wipe frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Radial Wipe frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Venetian Blinds frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Venetian Blinds frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Iris Wipe frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Iris Wipe frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Simple Choker frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Simple Choker frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Speed Lines frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Speed Lines frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Cross Glare frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Cross Glare frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Camera Shake frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Camera Shake frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Rain frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Rain frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
