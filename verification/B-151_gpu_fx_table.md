# B-151: ten of the fourth batch on the GPU against the CPU

Written by `tests/b151_gpu_fx.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU, with the layer's last effect, one of the ten, done on the card. **The rule: no channel of any pixel more than 1 level of 255 apart** (D-217). An effect that changes nothing or whose settings are invalid is not left to the card; on those rows any difference is the card's layering, held to the same 1 level by D-100. On a reference shot row the effect must in fact be left to the card. On every row both paths must give the same warnings, and the card must draw the frame itself, except a frame with an adjustment layer, which the CPU draws by B-44's rule: that one must be the CPU's picture exactly, the card's message `GPU_PREVIEW_ON_CPU` its only extra warning.

**2170 of 2170 checks pass.**

The worst comparison is "the reference shot with Median frame 0, Full": largest difference 1 of 255, pixels differing: 3814. Its pictures are in `verification/B-151 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## Each effect

Every fixture frame of the effect and the reference shot with it, at Full and Draft.

| Effect | Frames compared | Frames with it on the card | Largest difference (of 255) | Pass |
|---|---:|---:|---:|---|
| Median | 116 | 39 | 1 | 116 of 116 |
| Smart Blur | 126 | 44 | 1 | 126 of 126 |
| Roughen Edges | 246 | 144 | 1 | 246 of 246 |
| Radial Shadow | 256 | 156 | 1 | 256 of 256 |
| Bevel Alpha | 156 | 82 | 1 | 156 of 156 |
| Snowfall | 296 | 94 | 1 | 296 of 296 |
| Cell Pattern | 336 | 216 | 1 | 336 of 336 |
| Polar Coordinates | 226 | 162 | 1 | 226 of 226 |
| Optics Compensation | 206 | 134 | 1 | 206 of 206 |
| Corner Pin | 186 | 142 | 1 | 186 of 186 |

## Every frame

| Case | Effects left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---|---|
| fx_median_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_median_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_median_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_median_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_median_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_median_001 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_001 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_001 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_001 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_001 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_median_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_median_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_median_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_median_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_median_002 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_002 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_002 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_002 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_002 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_median_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_median_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_median_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_median_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_median_003 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_003 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_003 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_003 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_003 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_median_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_median_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_median_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_median_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_median_004 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_004 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_004 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_004 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_004 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_005 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_median_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_median_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_median_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_median_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_median_005 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_005 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_005 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_005 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_median_006 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_median_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_median_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_median_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_median_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_median_006 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_median_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_median_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_median_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_median_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_median_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_median_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_median_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_median_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_median_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_median_007 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_median_007 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_median_007 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_median_007 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_median_007 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_median_008 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_008 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_008 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_008 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_008 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_008 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_008 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_008 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_008 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_008 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_009 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_009 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_009 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_009 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_009 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_009 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_009 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_009 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_009 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_009 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_010 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_010 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_010 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_010 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_010 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_010 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_010 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_010 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_010 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_010 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_011 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_011 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_011 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_011 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_011 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_011 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_011 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_011 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_011 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_median_011 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_001 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_001 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_001 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_001 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_001 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_smart_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_smart_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_smart_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_smart_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_smart_002 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_002 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_002 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_002 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_002 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_003 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_003 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_003 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_003 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_003 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_004 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_004 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_004 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_004 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_004 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_005 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_005 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_005 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_005 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_005 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_006 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_006 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_006 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_006 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_006 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_007 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_smart_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_007 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_smart_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_smart_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_smart_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_smart_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_smart_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_smart_008 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_smart_008 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_smart_008 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_smart_008 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_smart_008 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_smart_009 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_009 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_009 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_009 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_009 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_009 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_009 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_009 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_009 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_009 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_010 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_010 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_010 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_010 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_010 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_010 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_010 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_010 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_010 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_010 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_011 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_011 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_011 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_011 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_011 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_011 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_011 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_011 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_011 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_011 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_012 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_012 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_012 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_012 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_012 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_012 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_012 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_012 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_012 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_smart_012 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_rough_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_rough_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_rough_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_rough_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_rough_002 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_rough_002 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_rough_002 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_rough_002 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_rough_002 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_rough_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_012 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_rough_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_012 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_rough_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rough_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rough_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_016 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_016 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_016 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_016 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_016 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_017 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_017 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_017 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_017 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_017 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_018 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_018 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_018 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_018 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_018 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_019 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_019 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_019 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_019 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_019 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_020 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_020 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_020 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_020 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_020 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_021 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_021 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_021 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_021 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_021 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_022 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_022 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_022 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_022 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_022 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_023 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_023 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_023 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_023 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_023 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_024 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_024 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_024 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_024 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rough_024 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_rshadow_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_rshadow_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_rshadow_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_bevel_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_bevel_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_bevel_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_bevel_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_bevel_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_bevel_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_bevel_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_bevel_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_bevel_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_bevel_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_bevel_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_bevel_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_bevel_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_bevel_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_bevel_003 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_bevel_003 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_bevel_003 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_bevel_003 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_bevel_003 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_bevel_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_008 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_bevel_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_008 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_bevel_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_009 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_bevel_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_009 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_bevel_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bevel_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bevel_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bevel_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_snow_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_snow_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_snow_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_snow_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_snow_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_002 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_002 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_002 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_002 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_002 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_003 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_003 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_003 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_003 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_003 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_004 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_004 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_004 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_004 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_004 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_005 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_005 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_005 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_005 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_005 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_006 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_006 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_006 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_006 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_006 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_006 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_006 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_006 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_006 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_006 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_007 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_007 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_007 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_007 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_007 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_007 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_007 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_007 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_007 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_007 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_008 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_008 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_008 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_008 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_008 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_009 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_009 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_009 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_009 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_009 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_010 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_010 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_010 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_010 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_010 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_011 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_011 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_011 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_011 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_011 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_012 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_012 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_012 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_012 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_012 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_013 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_013 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_013 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_013 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_013 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_017 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_018 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_snow_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_020 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_snow_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_028 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_028 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_028 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_028 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_028 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_028 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_028 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_028 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_028 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_028 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_029 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_029 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_029 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_029 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_029 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_029 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_029 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_029 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_029 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_snow_029 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_016 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_cell_016 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_cell_016 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_cell_016 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_cell_016 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_cell_016 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_cell_016 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_cell_016 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_cell_016 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_cell_016 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_cell_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_019 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_020 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_020 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_020 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_020 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_020 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_020 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_021 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_021 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_021 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_021 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_021 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_021 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_021 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_021 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_021 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_021 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_022 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_022 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_022 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_022 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_022 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_cell_022 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_022 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_022 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_022 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_022 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_cell_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_028 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_028 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_028 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_028 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_028 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_028 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_028 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_028 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_028 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_028 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_029 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_029 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_029 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_029 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_029 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_029 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_029 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_029 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_029 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_029 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_030 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_030 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_030 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_030 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_030 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_030 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_030 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_030 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_030 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_030 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_031 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_031 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_031 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_031 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_031 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_031 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_031 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_031 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_031 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_031 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_032 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_032 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_032 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_032 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_032 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_032 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_032 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_032 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_032 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_032 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_033 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_033 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_033 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_033 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_033 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_033 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_033 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_033 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_033 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_cell_033 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_polar_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_polar_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_polar_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_polar_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_polar_002 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_polar_002 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_polar_002 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_polar_002 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_polar_002 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_polar_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_006 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_polar_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_006 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_polar_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_007 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_polar_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_007 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_polar_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_012 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_012 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_012 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_012 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_012 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_012 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_012 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_012 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_012 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_012 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_013 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_013 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_013 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_013 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_013 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_014 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_014 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_014 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_014 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_014 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_015 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_015 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_015 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_015 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_015 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_019 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_020 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_020 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_020 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_020 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_020 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_020 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_021 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_021 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_021 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_021 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_021 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_polar_021 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_021 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_021 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_021 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_021 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_polar_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_022 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_022 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_022 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_022 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_polar_022 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_optics_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_optics_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_optics_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_optics_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_optics_002 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_optics_002 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_optics_002 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_optics_002 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_optics_002 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_optics_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_optics_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_010 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_optics_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_optics_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_optics_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_015 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_015 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_015 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_015 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_015 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_016 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_016 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_016 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_016 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_016 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_017 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_017 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_017 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_017 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_017 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_018 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_018 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_018 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_018 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_018 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_019 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_019 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_019 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_019 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_019 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_020 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_020 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_020 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_020 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_optics_020 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_pin_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_pin_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_pin_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_pin_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_pin_001 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_pin_001 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_pin_001 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_pin_001 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_pin_001 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_pin_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_011 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_pin_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_011 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_pin_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_012 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_pin_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_012 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_pin_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_pin_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_pin_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_016 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_016 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_016 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_016 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_016 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_017 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_017 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_017 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_017 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_017 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_018 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_018 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_018 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_018 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_pin_018 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| the reference shot with Median frame 0, Full | 3 | 1 | 3814 | none | PASS |
| the reference shot with Median frame 100, Full | 3 | 1 | 1731 | none | PASS |
| the reference shot with Median frame 239, Full | 3 | 1 | 1825 | none | PASS |
| the reference shot with Median frame 0, Draft | 3 | 1 | 132 | none | PASS |
| the reference shot with Median frame 100, Draft | 3 | 1 | 24 | none | PASS |
| the reference shot with Median frame 239, Draft | 3 | 1 | 47 | none | PASS |
| the reference shot with Smart Blur frame 0, Full | 3 | 1 | 973 | none | PASS |
| the reference shot with Smart Blur frame 100, Full | 3 | 1 | 1075 | none | PASS |
| the reference shot with Smart Blur frame 239, Full | 3 | 1 | 760 | none | PASS |
| the reference shot with Smart Blur frame 0, Draft | 3 | 1 | 73 | none | PASS |
| the reference shot with Smart Blur frame 100, Draft | 3 | 1 | 39 | none | PASS |
| the reference shot with Smart Blur frame 239, Draft | 3 | 1 | 54 | none | PASS |
| the reference shot with Roughen Edges frame 0, Full | 3 | 1 | 3781 | none | PASS |
| the reference shot with Roughen Edges frame 100, Full | 3 | 1 | 1418 | none | PASS |
| the reference shot with Roughen Edges frame 239, Full | 3 | 1 | 1772 | none | PASS |
| the reference shot with Roughen Edges frame 0, Draft | 3 | 1 | 128 | none | PASS |
| the reference shot with Roughen Edges frame 100, Draft | 3 | 1 | 33 | none | PASS |
| the reference shot with Roughen Edges frame 239, Draft | 3 | 1 | 47 | none | PASS |
| the reference shot with Radial Shadow frame 0, Full | 3 | 1 | 3785 | none | PASS |
| the reference shot with Radial Shadow frame 100, Full | 3 | 1 | 1417 | none | PASS |
| the reference shot with Radial Shadow frame 239, Full | 3 | 1 | 1773 | none | PASS |
| the reference shot with Radial Shadow frame 0, Draft | 3 | 1 | 129 | none | PASS |
| the reference shot with Radial Shadow frame 100, Draft | 3 | 1 | 33 | none | PASS |
| the reference shot with Radial Shadow frame 239, Draft | 3 | 1 | 47 | none | PASS |
| the reference shot with Bevel Alpha frame 0, Full | 3 | 1 | 3781 | none | PASS |
| the reference shot with Bevel Alpha frame 100, Full | 3 | 1 | 1418 | none | PASS |
| the reference shot with Bevel Alpha frame 239, Full | 3 | 1 | 1773 | none | PASS |
| the reference shot with Bevel Alpha frame 0, Draft | 3 | 1 | 130 | none | PASS |
| the reference shot with Bevel Alpha frame 100, Draft | 3 | 1 | 34 | none | PASS |
| the reference shot with Bevel Alpha frame 239, Draft | 3 | 1 | 49 | none | PASS |
| the reference shot with Snowfall frame 0, Full | 3 | 1 | 3711 | none | PASS |
| the reference shot with Snowfall frame 100, Full | 3 | 1 | 1404 | none | PASS |
| the reference shot with Snowfall frame 239, Full | 3 | 1 | 1774 | none | PASS |
| the reference shot with Snowfall frame 0, Draft | 3 | 1 | 130 | none | PASS |
| the reference shot with Snowfall frame 100, Draft | 3 | 1 | 33 | none | PASS |
| the reference shot with Snowfall frame 239, Draft | 3 | 1 | 48 | none | PASS |
| the reference shot with Cell Pattern frame 0, Full | 3 | 1 | 729 | none | PASS |
| the reference shot with Cell Pattern frame 100, Full | 3 | 1 | 839 | none | PASS |
| the reference shot with Cell Pattern frame 239, Full | 3 | 1 | 657 | none | PASS |
| the reference shot with Cell Pattern frame 0, Draft | 3 | 1 | 46 | none | PASS |
| the reference shot with Cell Pattern frame 100, Draft | 3 | 1 | 61 | none | PASS |
| the reference shot with Cell Pattern frame 239, Draft | 3 | 1 | 38 | none | PASS |
| the reference shot with Polar Coordinates frame 0, Full | 3 | 1 | 430 | none | PASS |
| the reference shot with Polar Coordinates frame 100, Full | 3 | 1 | 2524 | none | PASS |
| the reference shot with Polar Coordinates frame 239, Full | 3 | 1 | 525 | none | PASS |
| the reference shot with Polar Coordinates frame 0, Draft | 3 | 1 | 34 | none | PASS |
| the reference shot with Polar Coordinates frame 100, Draft | 3 | 1 | 81 | none | PASS |
| the reference shot with Polar Coordinates frame 239, Draft | 3 | 1 | 40 | none | PASS |
| the reference shot with Optics Compensation frame 0, Full | 3 | 1 | 1939 | none | PASS |
| the reference shot with Optics Compensation frame 100, Full | 3 | 1 | 992 | none | PASS |
| the reference shot with Optics Compensation frame 239, Full | 3 | 1 | 848 | none | PASS |
| the reference shot with Optics Compensation frame 0, Draft | 3 | 1 | 72 | none | PASS |
| the reference shot with Optics Compensation frame 100, Draft | 3 | 1 | 48 | none | PASS |
| the reference shot with Optics Compensation frame 239, Draft | 3 | 1 | 45 | none | PASS |
| the reference shot with Corner Pin frame 0, Full | 3 | 1 | 1186 | none | PASS |
| the reference shot with Corner Pin frame 100, Full | 3 | 1 | 811 | none | PASS |
| the reference shot with Corner Pin frame 239, Full | 3 | 1 | 787 | none | PASS |
| the reference shot with Corner Pin frame 0, Draft | 3 | 1 | 49 | none | PASS |
| the reference shot with Corner Pin frame 100, Draft | 3 | 1 | 58 | none | PASS |
| the reference shot with Corner Pin frame 239, Draft | 3 | 1 | 48 | none | PASS |
| the reference shot with Median frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Median frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Smart Blur frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Smart Blur frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Roughen Edges frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Roughen Edges frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Radial Shadow frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Radial Shadow frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Bevel Alpha frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Bevel Alpha frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Snowfall frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Snowfall frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Cell Pattern frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Cell Pattern frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Polar Coordinates frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Polar Coordinates frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Optics Compensation frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Optics Compensation frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Corner Pin frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Corner Pin frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
