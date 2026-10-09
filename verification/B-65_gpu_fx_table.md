# B-65: the batch of ten on the GPU against the CPU

Written by `tests/b65_gpu_fx.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU, with the layer's last effect of the ten done on the card. **The rule: no channel of any pixel more than 1 level of 255 apart** (D-122). An effect that changes nothing or whose settings are invalid, and a Levels that is a threshold, is not left to the card; on those rows any difference is the card's layering, held to the same 1 level by D-100. On every row both paths must give the same warnings.

**2710 of 2710 checks pass.**

The worst comparison is "the reference shot with Hue/Saturation frame 100, Full": largest difference 1 of 255, pixels differing: 79669. Its pictures are in `verification/B-65 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## Each effect

Every fixture frame of the effect and the reference shot with it, at Full and Draft.

| Effect | Frames compared | Frames with it on the card | Largest difference (of 255) | Pass |
|---|---:|---:|---:|---|
| Curves | 206 | 136 | 1 | 206 of 206 |
| Levels | 246 | 150 | 1 | 246 of 246 |
| Hue/Saturation | 236 | 162 | 1 | 236 of 236 |
| Gradient | 226 | 146 | 1 | 226 of 226 |
| Drop Shadow | 236 | 176 | 1 | 236 of 236 |
| Lens Blur | 696 | 486 | 1 | 696 of 696 |
| Rim Light | 286 | 204 | 1 | 286 of 286 |
| Outline | 206 | 124 | 1 | 206 of 206 |
| Noise | 186 | 114 | 1 | 186 of 186 |
| Chromatic Aberration | 166 | 104 | 1 | 166 of 166 |

## Every frame

| Case | Effects left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---|---|
| fx_curves_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_curves_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_curves_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_curves_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_curves_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_curves_001 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_curves_001 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_curves_001 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_curves_001 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_curves_001 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_curves_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_curves_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_curves_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_curves_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_levels_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_levels_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_levels_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_levels_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_levels_001 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_levels_001 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_levels_001 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_levels_001 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_levels_001 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_levels_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_003 frame 0, Full | 1 | 1 | 1 | none | PASS |
| fx_levels_003 frame 1, Full | 1 | 1 | 1 | none | PASS |
| fx_levels_003 frame 2, Full | 1 | 1 | 1 | none | PASS |
| fx_levels_003 frame 3, Full | 1 | 1 | 1 | none | PASS |
| fx_levels_003 frame 4, Full | 1 | 1 | 1 | none | PASS |
| fx_levels_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_011 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_levels_011 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_levels_011 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_levels_011 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_levels_011 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_levels_011 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_levels_011 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_levels_011 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_levels_011 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_levels_011 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_levels_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_013 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_levels_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_013 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_levels_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_014 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_levels_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_014 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_levels_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_015 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_levels_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_015 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_levels_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_levels_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_levels_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_levels_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_huesat_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_huesat_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_huesat_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_huesat_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_huesat_001 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_huesat_001 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_huesat_001 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_huesat_001 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_huesat_001 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_huesat_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_009 frame 0, Full | 1 | 1 | 16 | none | PASS |
| fx_huesat_009 frame 1, Full | 1 | 1 | 16 | none | PASS |
| fx_huesat_009 frame 2, Full | 1 | 1 | 16 | none | PASS |
| fx_huesat_009 frame 3, Full | 1 | 1 | 16 | none | PASS |
| fx_huesat_009 frame 4, Full | 1 | 1 | 16 | none | PASS |
| fx_huesat_009 frame 0, Draft | 1 | 1 | 2 | none | PASS |
| fx_huesat_009 frame 1, Draft | 1 | 1 | 2 | none | PASS |
| fx_huesat_009 frame 2, Draft | 1 | 1 | 2 | none | PASS |
| fx_huesat_009 frame 3, Draft | 1 | 1 | 2 | none | PASS |
| fx_huesat_009 frame 4, Draft | 1 | 1 | 2 | none | PASS |
| fx_huesat_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_015 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_huesat_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_015 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_huesat_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_016 frame 1, Full | 1 | 1 | 16 | none | PASS |
| fx_huesat_016 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_huesat_016 frame 3, Full | 1 | 1 | 16 | none | PASS |
| fx_huesat_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_016 frame 1, Draft | 1 | 1 | 2 | none | PASS |
| fx_huesat_016 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_huesat_016 frame 3, Draft | 1 | 1 | 2 | none | PASS |
| fx_huesat_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_huesat_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_huesat_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_huesat_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_002 frame 0, Draft | 1 | 1 | 4 | none | PASS |
| fx_grad_002 frame 1, Draft | 1 | 1 | 4 | none | PASS |
| fx_grad_002 frame 2, Draft | 1 | 1 | 4 | none | PASS |
| fx_grad_002 frame 3, Draft | 1 | 1 | 4 | none | PASS |
| fx_grad_002 frame 4, Draft | 1 | 1 | 4 | none | PASS |
| fx_grad_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_008 frame 0, Draft | 1 | 1 | 4 | none | PASS |
| fx_grad_008 frame 1, Draft | 1 | 1 | 4 | none | PASS |
| fx_grad_008 frame 2, Draft | 1 | 1 | 4 | none | PASS |
| fx_grad_008 frame 3, Draft | 1 | 1 | 4 | none | PASS |
| fx_grad_008 frame 4, Draft | 1 | 1 | 4 | none | PASS |
| fx_grad_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_010 frame 2, Full | 1 | 1 | 55 | none | PASS |
| fx_grad_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_010 frame 0, Draft | 1 | 1 | 4 | none | PASS |
| fx_grad_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_012 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_grad_012 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_grad_012 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_grad_012 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_grad_012 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_grad_012 frame 0, Draft | 0 | 1 | 3 | none | PASS |
| fx_grad_012 frame 1, Draft | 0 | 1 | 3 | none | PASS |
| fx_grad_012 frame 2, Draft | 0 | 1 | 3 | none | PASS |
| fx_grad_012 frame 3, Draft | 0 | 1 | 3 | none | PASS |
| fx_grad_012 frame 4, Draft | 0 | 1 | 3 | none | PASS |
| fx_grad_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_grad_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_grad_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_016 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_016 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_016 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_016 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_016 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_017 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_017 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_017 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_017 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_017 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_018 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_018 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_018 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_018 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_018 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_019 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_019 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_019 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_019 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_019 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_020 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_020 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_020 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_020 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_020 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_021 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_021 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_021 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_021 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_021 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_022 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_022 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_022 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_022 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_grad_022 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_shadow_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_shadow_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_shadow_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_019 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_020 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_020 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_020 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_020 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_020 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_020 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_021 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_021 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_021 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_021 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_021 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_021 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_021 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_021 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_021 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_021 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_022 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_022 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_022 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_022 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_022 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_022 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_022 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_022 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_022 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_022 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_023 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_023 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_023 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_023 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_023 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_023 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_023 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_023 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_023 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_023 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_024 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_024 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_024 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_024 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_024 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_024 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_024 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_024 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_024 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_024 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_025 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_025 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_025 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_025 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_025 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_025 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_025 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_025 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_025 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_025 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_026 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_026 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_026 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_026 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_026 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_026 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_026 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_026 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_026 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_026 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_027 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_027 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_027 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_027 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_027 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_027 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_027 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_027 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_027 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_027 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_028 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_028 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_028 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_028 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_028 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_028 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_028 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_028 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_028 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_028 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_029 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_029 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_029 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_029 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_029 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_029 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_029 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_029 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_029 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_029 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_030 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_030 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_030 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_030 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_030 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_030 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_030 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_030 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_030 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_030 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_031 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_031 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_031 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_031 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_031 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_031 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_031 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_031 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_031 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_031 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_032 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_032 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_032 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_032 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_032 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_032 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_032 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_032 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_032 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_032 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_033 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_033 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_033 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_033 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_033 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_033 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_033 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_033 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_033 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_033 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_034 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_034 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_034 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_034 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_034 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_034 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_034 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_034 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_034 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_034 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_035 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_035 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_035 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_035 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_035 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_035 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_035 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_035 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_035 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_035 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_036 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_036 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_036 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_036 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_036 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_036 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_036 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_036 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_036 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_036 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_037 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_037 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_037 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_037 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_037 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_037 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_037 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_037 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_037 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_037 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_038 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_038 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_038 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_038 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_038 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_038 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_038 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_038 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_038 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_038 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_039 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_039 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_039 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_039 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_039 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_039 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_039 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_039 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_039 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_039 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_040 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_040 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_040 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_040 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_040 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_040 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_040 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_040 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_040 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_040 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_041 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_041 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_041 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_041 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_041 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_041 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_041 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_041 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_041 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_041 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_042 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_042 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_042 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_042 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_042 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_042 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_042 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_042 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_042 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_042 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_043 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_043 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_043 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_043 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_043 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_043 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_043 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_043 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_043 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_043 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_044 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_044 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_044 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_044 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_044 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_044 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_044 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_044 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_044 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_044 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_045 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_045 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_045 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_045 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_045 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_045 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_045 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_045 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_045 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_045 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_046 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_046 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_046 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_046 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_046 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_046 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_046 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_046 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_046 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_046 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_047 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_047 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_047 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_047 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_047 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_047 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_047 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_047 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_047 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_047 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_048 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_048 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_048 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_048 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_048 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_048 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_048 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_048 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_048 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_048 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_049 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_049 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_049 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_049 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_049 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_049 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_049 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_049 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_049 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_049 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_050 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_050 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_050 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_050 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_050 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_050 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_050 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_050 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_050 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_050 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_051 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_051 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_051 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_051 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_051 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_051 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_051 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_051 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_051 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_051 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_052 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_052 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_052 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_052 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_052 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_052 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_052 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_052 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_052 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_052 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_053 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_053 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_053 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_053 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_053 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_053 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_053 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_053 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_053 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_053 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_054 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_054 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_054 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_054 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_054 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_054 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_054 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_054 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_054 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_054 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_055 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_055 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_055 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_055 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_055 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_055 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_055 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_055 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_055 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_055 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_056 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_056 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_056 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_056 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_056 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_056 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_056 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_056 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_056 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_056 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_057 frame 0, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| fx_lens_057 frame 1, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| fx_lens_057 frame 2, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| fx_lens_057 frame 3, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| fx_lens_057 frame 4, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| fx_lens_057 frame 0, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| fx_lens_057 frame 1, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| fx_lens_057 frame 2, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| fx_lens_057 frame 3, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| fx_lens_057 frame 4, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| fx_lens_058 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_058 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_058 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_058 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_058 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_058 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_058 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_058 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_058 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_058 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_059 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_059 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_059 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_059 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_059 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_059 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_059 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_059 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_059 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_059 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_060 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_060 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_060 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_060 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_060 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_060 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_060 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_060 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_060 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_060 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_061 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_061 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_061 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_061 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_061 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_061 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_061 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_061 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_061 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_061 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_062 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_062 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_062 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_062 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_062 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lens_062 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_062 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_062 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_062 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_062 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lens_063 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_063 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_063 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_063 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_063 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_063 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_063 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_063 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_063 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_063 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_064 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_064 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_064 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_064 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_064 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_064 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_064 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_064 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_064 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_064 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_065 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_065 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_065 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_065 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_065 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_065 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_065 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_065 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_065 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_065 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_066 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_066 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_066 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_066 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_066 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_066 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_066 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_066 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_066 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_066 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_067 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_067 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_067 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_067 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_067 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_067 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_067 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_067 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_067 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_067 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_068 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_068 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_068 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_068 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_068 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_068 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_068 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_068 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_068 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_068 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_069 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_069 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_069 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_069 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_069 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_069 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_069 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_069 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_069 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lens_069 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_rim_010 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_rim_010 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_rim_010 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_rim_010 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_rim_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_rim_010 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_rim_010 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_rim_010 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_rim_010 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_rim_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_019 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_rim_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_019 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_rim_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_020 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_020 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_020 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_020 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_020 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_020 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_021 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_021 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_021 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_021 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_021 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rim_021 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_021 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_021 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_021 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_021 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rim_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_028 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_028 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_028 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_028 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_028 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_028 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_028 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_028 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_028 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rim_028 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_outline_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_outline_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_outline_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_outline_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_outline_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_outline_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_outline_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_outline_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_outline_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_outline_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_outline_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_outline_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_outline_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_outline_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_outline_003 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_outline_003 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_outline_003 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_outline_003 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_outline_003 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_outline_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_outline_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_outline_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_outline_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_outline_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_outline_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_008 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_noise_008 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_noise_008 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_noise_008 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_noise_008 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_noise_008 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_noise_008 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_noise_008 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_noise_008 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_noise_008 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_noise_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_noise_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_noise_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_noise_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_noise_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_013 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_013 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_013 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_013 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_013 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_noise_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_chroma_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_chroma_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_chroma_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_chroma_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_chroma_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_chroma_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_chroma_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_chroma_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_chroma_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_chroma_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_005 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_chroma_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_005 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_chroma_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_chroma_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_chroma_012 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_012 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_012 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_012 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_012 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_012 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_012 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_012 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_012 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_012 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_013 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_013 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_013 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_013 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_013 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_chroma_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| the reference shot with Curves frame 0, Full | 3 | 1 | 694 | none | PASS |
| the reference shot with Curves frame 100, Full | 3 | 0 | 0 | none | PASS |
| the reference shot with Curves frame 239, Full | 3 | 1 | 262 | none | PASS |
| the reference shot with Curves frame 0, Draft | 3 | 1 | 48 | none | PASS |
| the reference shot with Curves frame 100, Draft | 3 | 1 | 42 | none | PASS |
| the reference shot with Curves frame 239, Draft | 3 | 1 | 37 | none | PASS |
| the reference shot with Levels frame 0, Full | 3 | 1 | 1 | none | PASS |
| the reference shot with Levels frame 100, Full | 3 | 1 | 1 | none | PASS |
| the reference shot with Levels frame 239, Full | 3 | 1 | 1 | none | PASS |
| the reference shot with Levels frame 0, Draft | 3 | 1 | 41 | none | PASS |
| the reference shot with Levels frame 100, Draft | 3 | 1 | 56 | none | PASS |
| the reference shot with Levels frame 239, Draft | 3 | 1 | 46 | none | PASS |
| the reference shot with Hue/Saturation frame 0, Full | 3 | 1 | 79182 | none | PASS |
| the reference shot with Hue/Saturation frame 100, Full | 3 | 1 | 79669 | none | PASS |
| the reference shot with Hue/Saturation frame 239, Full | 3 | 1 | 77408 | none | PASS |
| the reference shot with Hue/Saturation frame 0, Draft | 3 | 1 | 4720 | none | PASS |
| the reference shot with Hue/Saturation frame 100, Draft | 3 | 1 | 4721 | none | PASS |
| the reference shot with Hue/Saturation frame 239, Draft | 3 | 1 | 4619 | none | PASS |
| the reference shot with Gradient frame 0, Full | 3 | 1 | 1306 | none | PASS |
| the reference shot with Gradient frame 100, Full | 3 | 1 | 1324 | none | PASS |
| the reference shot with Gradient frame 239, Full | 3 | 1 | 1372 | none | PASS |
| the reference shot with Gradient frame 0, Draft | 3 | 1 | 77 | none | PASS |
| the reference shot with Gradient frame 100, Draft | 3 | 1 | 92 | none | PASS |
| the reference shot with Gradient frame 239, Draft | 3 | 1 | 83 | none | PASS |
| the reference shot with Drop Shadow frame 0, Full | 3 | 1 | 3782 | none | PASS |
| the reference shot with Drop Shadow frame 100, Full | 3 | 1 | 1465 | none | PASS |
| the reference shot with Drop Shadow frame 239, Full | 3 | 1 | 1773 | none | PASS |
| the reference shot with Drop Shadow frame 0, Draft | 3 | 1 | 128 | none | PASS |
| the reference shot with Drop Shadow frame 100, Draft | 3 | 1 | 32 | none | PASS |
| the reference shot with Drop Shadow frame 239, Draft | 3 | 1 | 48 | none | PASS |
| the reference shot with Lens Blur frame 0, Full | 3 | 1 | 761 | none | PASS |
| the reference shot with Lens Blur frame 100, Full | 3 | 1 | 954 | none | PASS |
| the reference shot with Lens Blur frame 239, Full | 3 | 1 | 643 | none | PASS |
| the reference shot with Lens Blur frame 0, Draft | 3 | 1 | 54 | none | PASS |
| the reference shot with Lens Blur frame 100, Draft | 3 | 1 | 82 | none | PASS |
| the reference shot with Lens Blur frame 239, Draft | 3 | 1 | 52 | none | PASS |
| the reference shot with Rim Light frame 0, Full | 3 | 1 | 3781 | none | PASS |
| the reference shot with Rim Light frame 100, Full | 3 | 1 | 1417 | none | PASS |
| the reference shot with Rim Light frame 239, Full | 3 | 1 | 1776 | none | PASS |
| the reference shot with Rim Light frame 0, Draft | 3 | 1 | 128 | none | PASS |
| the reference shot with Rim Light frame 100, Draft | 3 | 1 | 33 | none | PASS |
| the reference shot with Rim Light frame 239, Draft | 3 | 1 | 47 | none | PASS |
| the reference shot with Outline frame 0, Full | 3 | 1 | 3781 | none | PASS |
| the reference shot with Outline frame 100, Full | 3 | 1 | 1418 | none | PASS |
| the reference shot with Outline frame 239, Full | 3 | 1 | 1773 | none | PASS |
| the reference shot with Outline frame 0, Draft | 3 | 1 | 128 | none | PASS |
| the reference shot with Outline frame 100, Draft | 3 | 1 | 32 | none | PASS |
| the reference shot with Outline frame 239, Draft | 3 | 1 | 47 | none | PASS |
| the reference shot with Noise frame 0, Full | 3 | 1 | 961 | none | PASS |
| the reference shot with Noise frame 100, Full | 3 | 1 | 1048 | none | PASS |
| the reference shot with Noise frame 239, Full | 3 | 1 | 944 | none | PASS |
| the reference shot with Noise frame 0, Draft | 3 | 1 | 65 | none | PASS |
| the reference shot with Noise frame 100, Draft | 3 | 1 | 67 | none | PASS |
| the reference shot with Noise frame 239, Draft | 3 | 1 | 69 | none | PASS |
| the reference shot with Chromatic Aberration frame 0, Full | 3 | 1 | 2862 | none | PASS |
| the reference shot with Chromatic Aberration frame 100, Full | 3 | 1 | 1975 | none | PASS |
| the reference shot with Chromatic Aberration frame 239, Full | 3 | 1 | 1754 | none | PASS |
| the reference shot with Chromatic Aberration frame 0, Draft | 3 | 1 | 65 | none | PASS |
| the reference shot with Chromatic Aberration frame 100, Draft | 3 | 1 | 54 | none | PASS |
| the reference shot with Chromatic Aberration frame 239, Draft | 3 | 1 | 56 | none | PASS |
| the reference shot with Curves frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Curves frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Levels frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Levels frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Hue/Saturation frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Hue/Saturation frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Gradient frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Gradient frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Drop Shadow frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Drop Shadow frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Lens Blur frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Lens Blur frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Rim Light frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Rim Light frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Outline frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Outline frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Noise frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Noise frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Chromatic Aberration frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Chromatic Aberration frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
