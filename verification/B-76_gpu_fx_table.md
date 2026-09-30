# B-76: the second batch of ten on the GPU against the CPU

Written by `tests/b76_gpu_fx.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU, with the layer's last effect of the nine done on the card and its Light Wraps wherever they are. **The rule: no channel of any pixel more than 1 level of 255 apart** (D-133). An effect that changes nothing or whose settings are invalid is not left to the card; on those rows any difference is the card's layering, held to the same 1 level by D-100. On every row both paths must give the same warnings, and the card must draw the frame itself, except a frame with an adjustment layer, which the CPU draws by B-44's rule: that one must be the CPU's picture exactly, the card's message `GPU_PREVIEW_ON_CPU` its only extra warning.

**2460 of 2460 checks pass.**

The worst comparison is "the reference shot with Vignette frame 0, Full": largest difference 1 of 255, pixels differing: 3937. Its pictures are in `verification/B-76 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## Each effect

Every fixture frame of the effect and the reference shot with it, at Full and Draft.

| Effect | Frames compared | Frames with it on the card | Largest difference (of 255) | Pass |
|---|---:|---:|---:|---|
| Distance Gradation | 246 | 142 | 1 | 246 of 246 |
| Light Rays | 246 | 164 | 1 | 246 of 246 |
| Exposure Flicker | 226 | 142 | 1 | 226 of 226 |
| Vignette | 256 | 172 | 1 | 256 of 256 |
| Turbulent Displace | 266 | 174 | 1 | 266 of 266 |
| Fractal Noise | 286 | 204 | 1 | 286 of 286 |
| Gradient Map | 236 | 152 | 1 | 236 of 236 |
| Color Balance | 196 | 130 | 1 | 196 of 196 |
| Offset | 236 | 170 | 1 | 236 of 236 |
| Light Wrap | 246 | 166 | 1 | 246 of 246 |

## Every frame

| Case | Effects left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---|---|
| fx_distgrad_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_005 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_005 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_005 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_005 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_005 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_005 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_005 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_005 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_005 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_005 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_006 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_006 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_006 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_006 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_006 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_006 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_006 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_006 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_006 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_006 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_007 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_007 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_007 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_007 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_007 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_007 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_007 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_007 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_007 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_007 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_015 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_015 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_016 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_distgrad_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_016 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_distgrad_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_distgrad_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_distgrad_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_018 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_018 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_018 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_018 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_018 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_019 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_019 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_019 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_019 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_019 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_020 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_020 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_020 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_020 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_020 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_021 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_021 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_021 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_021 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_021 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_022 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_022 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_022 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_022 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_022 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_023 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_023 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_023 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_023 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_023 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_024 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_024 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_024 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_024 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_distgrad_024 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_rays_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_rays_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_rays_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_rays_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_rays_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_rays_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_rays_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_rays_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_rays_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_rays_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_016 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_rays_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_016 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_rays_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rays_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rays_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rays_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_flicker_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_flicker_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_flicker_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_flicker_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_flicker_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_flicker_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_flicker_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_flicker_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_flicker_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_flicker_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_011 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_flicker_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_011 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_flicker_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_012 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_flicker_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_012 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_flicker_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_flicker_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_flicker_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_flicker_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_vignette_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_vignette_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_vignette_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_vignette_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_vignette_003 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_vignette_003 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_vignette_003 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_vignette_003 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_vignette_003 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_vignette_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_011 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_vignette_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_011 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_vignette_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_014 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_vignette_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_014 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_vignette_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_vignette_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_018 frame 0, Full | 1 | 1 | 4 | none | PASS |
| fx_vignette_018 frame 1, Full | 1 | 1 | 4 | none | PASS |
| fx_vignette_018 frame 2, Full | 1 | 1 | 4 | none | PASS |
| fx_vignette_018 frame 3, Full | 1 | 1 | 4 | none | PASS |
| fx_vignette_018 frame 4, Full | 1 | 1 | 4 | none | PASS |
| fx_vignette_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_vignette_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_vignette_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_turb_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_turb_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_turb_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_turb_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_turb_002 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_turb_002 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_turb_002 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_turb_002 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_turb_002 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_turb_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_015 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_turb_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_015 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_turb_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_turb_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_turb_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_019 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_019 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_019 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_019 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_019 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_020 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_020 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_020 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_020 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_020 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_021 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_021 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_021 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_021 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_021 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_022 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_022 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_022 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_022 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_022 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_023 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_023 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_023 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_023 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_023 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_024 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_024 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_024 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_024 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_024 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_025 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_025 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_025 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_025 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_025 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_026 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_026 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_026 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_026 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_turb_026 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_019 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_fractal_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_019 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_fractal_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_020 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fractal_020 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_020 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_020 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_020 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_020 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_fractal_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_028 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_028 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_028 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_028 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_028 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_028 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_028 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_028 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_028 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_fractal_028 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_gradmap_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_gradmap_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_gradmap_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_gradmap_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_gradmap_002 frame 0, Draft | 0 | 1 | 3 | none | PASS |
| fx_gradmap_002 frame 1, Draft | 0 | 1 | 3 | none | PASS |
| fx_gradmap_002 frame 2, Draft | 0 | 1 | 3 | none | PASS |
| fx_gradmap_002 frame 3, Draft | 0 | 1 | 3 | none | PASS |
| fx_gradmap_002 frame 4, Draft | 0 | 1 | 3 | none | PASS |
| fx_gradmap_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_013 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_gradmap_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_013 frame 0, Draft | 0 | 1 | 3 | none | PASS |
| fx_gradmap_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_014 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_gradmap_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_014 frame 0, Draft | 0 | 1 | 3 | none | PASS |
| fx_gradmap_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_gradmap_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_gradmap_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_017 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_017 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_017 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_017 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_017 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_018 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_018 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_018 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_018 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_018 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_019 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_019 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_019 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_019 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_019 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_020 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_020 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_020 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_020 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_020 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_021 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_021 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_021 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_021 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_021 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_022 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_022 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_022 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_022 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_022 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_023 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_023 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_023 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_023 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_gradmap_023 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_balance_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_balance_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_balance_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_balance_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_balance_001 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_balance_001 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_balance_001 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_balance_001 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_balance_001 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_balance_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_008 frame 0, Full | 1 | 1 | 16 | none | PASS |
| fx_balance_008 frame 1, Full | 1 | 1 | 16 | none | PASS |
| fx_balance_008 frame 2, Full | 1 | 1 | 16 | none | PASS |
| fx_balance_008 frame 3, Full | 1 | 1 | 16 | none | PASS |
| fx_balance_008 frame 4, Full | 1 | 1 | 16 | none | PASS |
| fx_balance_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_011 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_balance_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_011 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_balance_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_012 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_balance_012 frame 3, Full | 1 | 1 | 16 | none | PASS |
| fx_balance_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_012 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_balance_012 frame 3, Draft | 1 | 1 | 2 | none | PASS |
| fx_balance_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_013 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_balance_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_013 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_balance_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_balance_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_balance_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_balance_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_offset_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_offset_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_offset_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_offset_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_offset_001 frame 0, Draft | 0 | 1 | 1 | none | PASS |
| fx_offset_001 frame 1, Draft | 0 | 1 | 1 | none | PASS |
| fx_offset_001 frame 2, Draft | 0 | 1 | 1 | none | PASS |
| fx_offset_001 frame 3, Draft | 0 | 1 | 1 | none | PASS |
| fx_offset_001 frame 4, Draft | 0 | 1 | 1 | none | PASS |
| fx_offset_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_013 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_offset_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_013 frame 0, Draft | 0 | 1 | 1 | none | PASS |
| fx_offset_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_014 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_offset_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_014 frame 0, Draft | 0 | 1 | 1 | none | PASS |
| fx_offset_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_015 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_offset_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_015 frame 0, Draft | 0 | 1 | 1 | none | PASS |
| fx_offset_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_offset_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_offset_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_019 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_019 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_019 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_019 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_019 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_020 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_020 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_020 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_020 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_020 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_021 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_021 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_021 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_021 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_021 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_022 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_022 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_022 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_022 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_022 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_023 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_023 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_023 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_023 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_offset_023 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_006 frame 0, Draft | 1 | 1 | 1 | none | PASS |
| fx_wrap_006 frame 1, Draft | 1 | 1 | 1 | none | PASS |
| fx_wrap_006 frame 2, Draft | 1 | 1 | 1 | none | PASS |
| fx_wrap_006 frame 3, Draft | 1 | 1 | 1 | none | PASS |
| fx_wrap_006 frame 4, Draft | 1 | 1 | 1 | none | PASS |
| fx_wrap_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_010 frame 0, Full | 1 | 1 | 2 | none | PASS |
| fx_wrap_010 frame 1, Full | 1 | 1 | 2 | none | PASS |
| fx_wrap_010 frame 2, Full | 1 | 1 | 2 | none | PASS |
| fx_wrap_010 frame 3, Full | 1 | 1 | 2 | none | PASS |
| fx_wrap_010 frame 4, Full | 1 | 1 | 2 | none | PASS |
| fx_wrap_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_015 frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_wrap_015 frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_wrap_015 frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_wrap_015 frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_wrap_015 frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_wrap_015 frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_wrap_015 frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_wrap_015 frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_wrap_015 frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_wrap_015 frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_wrap_016 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_wrap_016 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_wrap_016 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_wrap_016 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_wrap_016 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_wrap_016 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_wrap_016 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_wrap_016 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_wrap_016 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_wrap_016 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_wrap_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_017 frame 3, Full | 1 | 1 | 2 | none | PASS |
| fx_wrap_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_017 frame 0, Draft | 1 | 1 | 1 | none | PASS |
| fx_wrap_017 frame 1, Draft | 1 | 1 | 1 | none | PASS |
| fx_wrap_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_017 frame 4, Draft | 1 | 1 | 1 | none | PASS |
| fx_wrap_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_wrap_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_wrap_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_wrap_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| the reference shot with Distance Gradation frame 0, Full | 3 | 1 | 3782 | none | PASS |
| the reference shot with Distance Gradation frame 100, Full | 3 | 1 | 1417 | none | PASS |
| the reference shot with Distance Gradation frame 239, Full | 3 | 1 | 1772 | none | PASS |
| the reference shot with Distance Gradation frame 0, Draft | 3 | 1 | 129 | none | PASS |
| the reference shot with Distance Gradation frame 100, Draft | 3 | 1 | 33 | none | PASS |
| the reference shot with Distance Gradation frame 239, Draft | 3 | 1 | 48 | none | PASS |
| the reference shot with Light Rays frame 0, Full | 3 | 1 | 3141 | none | PASS |
| the reference shot with Light Rays frame 100, Full | 3 | 1 | 1059 | none | PASS |
| the reference shot with Light Rays frame 239, Full | 3 | 1 | 1377 | none | PASS |
| the reference shot with Light Rays frame 0, Draft | 3 | 1 | 123 | none | PASS |
| the reference shot with Light Rays frame 100, Draft | 3 | 1 | 67 | none | PASS |
| the reference shot with Light Rays frame 239, Draft | 3 | 1 | 62 | none | PASS |
| the reference shot with Exposure Flicker frame 0, Full | 3 | 1 | 1 | none | PASS |
| the reference shot with Exposure Flicker frame 100, Full | 3 | 1 | 1605 | none | PASS |
| the reference shot with Exposure Flicker frame 239, Full | 3 | 1 | 1 | none | PASS |
| the reference shot with Exposure Flicker frame 0, Draft | 3 | 1 | 26 | none | PASS |
| the reference shot with Exposure Flicker frame 100, Draft | 3 | 1 | 48 | none | PASS |
| the reference shot with Exposure Flicker frame 239, Draft | 3 | 1 | 36 | none | PASS |
| the reference shot with Vignette frame 0, Full | 3 | 1 | 3937 | none | PASS |
| the reference shot with Vignette frame 100, Full | 3 | 1 | 1449 | none | PASS |
| the reference shot with Vignette frame 239, Full | 3 | 1 | 1878 | none | PASS |
| the reference shot with Vignette frame 0, Draft | 3 | 1 | 134 | none | PASS |
| the reference shot with Vignette frame 100, Draft | 3 | 1 | 37 | none | PASS |
| the reference shot with Vignette frame 239, Draft | 3 | 1 | 58 | none | PASS |
| the reference shot with Turbulent Displace frame 0, Full | 3 | 1 | 1733 | none | PASS |
| the reference shot with Turbulent Displace frame 100, Full | 3 | 1 | 912 | none | PASS |
| the reference shot with Turbulent Displace frame 239, Full | 3 | 1 | 844 | none | PASS |
| the reference shot with Turbulent Displace frame 0, Draft | 3 | 1 | 67 | none | PASS |
| the reference shot with Turbulent Displace frame 100, Draft | 3 | 1 | 77 | none | PASS |
| the reference shot with Turbulent Displace frame 239, Draft | 3 | 1 | 44 | none | PASS |
| the reference shot with Fractal Noise frame 0, Full | 3 | 1 | 937 | none | PASS |
| the reference shot with Fractal Noise frame 100, Full | 3 | 1 | 1016 | none | PASS |
| the reference shot with Fractal Noise frame 239, Full | 3 | 1 | 764 | none | PASS |
| the reference shot with Fractal Noise frame 0, Draft | 3 | 1 | 65 | none | PASS |
| the reference shot with Fractal Noise frame 100, Draft | 3 | 1 | 66 | none | PASS |
| the reference shot with Fractal Noise frame 239, Draft | 3 | 1 | 52 | none | PASS |
| the reference shot with Gradient Map frame 0, Full | 3 | 1 | 947 | none | PASS |
| the reference shot with Gradient Map frame 100, Full | 3 | 1 | 928 | none | PASS |
| the reference shot with Gradient Map frame 239, Full | 3 | 1 | 1015 | none | PASS |
| the reference shot with Gradient Map frame 0, Draft | 3 | 1 | 59 | none | PASS |
| the reference shot with Gradient Map frame 100, Draft | 3 | 1 | 65 | none | PASS |
| the reference shot with Gradient Map frame 239, Draft | 3 | 1 | 69 | none | PASS |
| the reference shot with Color Balance frame 0, Full | 3 | 1 | 914 | none | PASS |
| the reference shot with Color Balance frame 100, Full | 3 | 1 | 1782 | none | PASS |
| the reference shot with Color Balance frame 239, Full | 3 | 1 | 1329 | none | PASS |
| the reference shot with Color Balance frame 0, Draft | 3 | 1 | 54 | none | PASS |
| the reference shot with Color Balance frame 100, Draft | 3 | 1 | 65 | none | PASS |
| the reference shot with Color Balance frame 239, Draft | 3 | 1 | 63 | none | PASS |
| the reference shot with Offset frame 0, Full | 3 | 1 | 2151 | none | PASS |
| the reference shot with Offset frame 100, Full | 3 | 1 | 621 | none | PASS |
| the reference shot with Offset frame 239, Full | 3 | 1 | 757 | none | PASS |
| the reference shot with Offset frame 0, Draft | 3 | 1 | 72 | none | PASS |
| the reference shot with Offset frame 100, Draft | 3 | 1 | 51 | none | PASS |
| the reference shot with Offset frame 239, Draft | 3 | 1 | 48 | none | PASS |
| the reference shot with Light Wrap frame 0, Full | 3 | 1 | 3781 | none | PASS |
| the reference shot with Light Wrap frame 100, Full | 3 | 1 | 1417 | none | PASS |
| the reference shot with Light Wrap frame 239, Full | 3 | 1 | 1774 | none | PASS |
| the reference shot with Light Wrap frame 0, Draft | 3 | 1 | 128 | none | PASS |
| the reference shot with Light Wrap frame 100, Draft | 3 | 1 | 32 | none | PASS |
| the reference shot with Light Wrap frame 239, Draft | 3 | 1 | 47 | none | PASS |
| the reference shot with Distance Gradation frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Distance Gradation frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Light Rays frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Light Rays frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Exposure Flicker frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Exposure Flicker frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Vignette frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Vignette frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Turbulent Displace frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Turbulent Displace frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Fractal Noise frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Fractal Noise frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Gradient Map frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Gradient Map frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Color Balance frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Color Balance frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Offset frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Offset frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Light Wrap frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Light Wrap frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
