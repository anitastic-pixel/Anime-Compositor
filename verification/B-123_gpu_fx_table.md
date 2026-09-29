# B-123: the fourth batch's five on the GPU against the CPU

Written by `tests/b123_gpu_fx.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU, with the layer's last effect, one of the five, done on the card. **The rule: no channel of any pixel more than 1 level of 255 apart** (D-187). An effect that changes nothing or whose settings are invalid is not left to the card; on those rows any difference is the card's layering, held to the same 1 level by D-100. On every row both paths must give the same warnings, and the card must draw the frame itself, except a frame with an adjustment layer, which the CPU draws by B-44's rule: that one must be the CPU's picture exactly, the card's message `GPU_PREVIEW_ON_CPU` its only extra warning.

**1010 of 1010 checks pass.**

The worst comparison is "the reference shot with Kira-kira frame 0, Full": largest difference 1 of 255, pixels differing: 3783. Its pictures are in `verification/B-123 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## Each effect

Every fixture frame of the effect and the reference shot with it, at Full and Draft.

| Effect | Frames compared | Frames with it on the card | Largest difference (of 255) | Pass |
|---|---:|---:|---:|---|
| Color Lookup | 136 | 76 | 1 | 136 of 136 |
| Line Blur | 166 | 94 | 1 | 166 of 166 |
| HSV Key | 166 | 106 | 1 | 166 of 166 |
| Paraffin | 236 | 154 | 1 | 236 of 236 |
| Kira-kira | 296 | 182 | 1 | 296 of 296 |

## Every frame

| Case | Effects left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---|---|
| fx_lut_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_007 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_lut_007 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_lut_007 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_lut_007 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_lut_007 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_lut_007 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_lut_007 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_lut_007 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_lut_007 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_lut_007 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_lut_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lut_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lut_009 frame 0, Full | 0 | 0 | 0 | MEDIA_MISSING, on both | PASS |
| fx_lut_009 frame 1, Full | 0 | 0 | 0 | MEDIA_MISSING, on both | PASS |
| fx_lut_009 frame 2, Full | 0 | 0 | 0 | MEDIA_MISSING, on both | PASS |
| fx_lut_009 frame 3, Full | 0 | 0 | 0 | MEDIA_MISSING, on both | PASS |
| fx_lut_009 frame 4, Full | 0 | 0 | 0 | MEDIA_MISSING, on both | PASS |
| fx_lut_009 frame 0, Draft | 0 | 0 | 0 | MEDIA_MISSING, on both | PASS |
| fx_lut_009 frame 1, Draft | 0 | 0 | 0 | MEDIA_MISSING, on both | PASS |
| fx_lut_009 frame 2, Draft | 0 | 0 | 0 | MEDIA_MISSING, on both | PASS |
| fx_lut_009 frame 3, Draft | 0 | 0 | 0 | MEDIA_MISSING, on both | PASS |
| fx_lut_009 frame 4, Draft | 0 | 0 | 0 | MEDIA_MISSING, on both | PASS |
| fx_lut_010 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_010 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_010 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_010 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_010 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_010 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_010 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_010 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_010 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_010 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_011 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_011 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_011 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_011 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_011 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_011 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_011 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_011 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_011 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_011 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lut_012 frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_lut_012 frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_lut_012 frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_lut_012 frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_lut_012 frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_lut_012 frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_lut_012 frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_lut_012 frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_lut_012 frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_lut_012 frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: an adjustment layer, so the CPU drew it (B-44) |
| fx_lut_013 frame 0, Full | 0 | 0 | 0 | MEDIA_DECODE_FAILED, on both | PASS |
| fx_lut_013 frame 1, Full | 0 | 0 | 0 | MEDIA_DECODE_FAILED, on both | PASS |
| fx_lut_013 frame 2, Full | 0 | 0 | 0 | MEDIA_DECODE_FAILED, on both | PASS |
| fx_lut_013 frame 3, Full | 0 | 0 | 0 | MEDIA_DECODE_FAILED, on both | PASS |
| fx_lut_013 frame 4, Full | 0 | 0 | 0 | MEDIA_DECODE_FAILED, on both | PASS |
| fx_lut_013 frame 0, Draft | 0 | 0 | 0 | MEDIA_DECODE_FAILED, on both | PASS |
| fx_lut_013 frame 1, Draft | 0 | 0 | 0 | MEDIA_DECODE_FAILED, on both | PASS |
| fx_lut_013 frame 2, Draft | 0 | 0 | 0 | MEDIA_DECODE_FAILED, on both | PASS |
| fx_lut_013 frame 3, Draft | 0 | 0 | 0 | MEDIA_DECODE_FAILED, on both | PASS |
| fx_lut_013 frame 4, Draft | 0 | 0 | 0 | MEDIA_DECODE_FAILED, on both | PASS |
| fx_lblur_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_006 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_lblur_006 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_lblur_006 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_lblur_006 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_lblur_006 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_lblur_006 frame 0, Draft | 0 | 1 | 1 | none | PASS |
| fx_lblur_006 frame 1, Draft | 0 | 1 | 1 | none | PASS |
| fx_lblur_006 frame 2, Draft | 0 | 1 | 1 | none | PASS |
| fx_lblur_006 frame 3, Draft | 0 | 1 | 1 | none | PASS |
| fx_lblur_006 frame 4, Draft | 0 | 1 | 1 | none | PASS |
| fx_lblur_007 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_lblur_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_007 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_lblur_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_lblur_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_lblur_011 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_011 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_011 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_011 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_011 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_011 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_011 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_011 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_011 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_011 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_012 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_012 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_012 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_012 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_012 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_012 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_012 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_012 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_012 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_012 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_013 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_013 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_013 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_013 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_013 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_014 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_014 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_014 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_014 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_014 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_015 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_015 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_015 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_015 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_015 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_016 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_016 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_016 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_016 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_lblur_016 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_hsv_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_hsv_011 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_011 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_011 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_011 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_011 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_011 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_011 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_011 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_011 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_011 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_012 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_012 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_012 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_012 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_012 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_012 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_012 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_012 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_012 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_012 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_013 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_013 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_013 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_013 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_013 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_hsv_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_para_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_para_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_para_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_para_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_para_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_002 frame 0, Full | 1 | 1 | 3 | none | PASS |
| fx_para_002 frame 1, Full | 1 | 1 | 3 | none | PASS |
| fx_para_002 frame 2, Full | 1 | 1 | 3 | none | PASS |
| fx_para_002 frame 3, Full | 1 | 1 | 3 | none | PASS |
| fx_para_002 frame 4, Full | 1 | 1 | 3 | none | PASS |
| fx_para_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_003 frame 0, Full | 1 | 1 | 3 | none | PASS |
| fx_para_003 frame 1, Full | 1 | 1 | 3 | none | PASS |
| fx_para_003 frame 2, Full | 1 | 1 | 3 | none | PASS |
| fx_para_003 frame 3, Full | 1 | 1 | 3 | none | PASS |
| fx_para_003 frame 4, Full | 1 | 1 | 3 | none | PASS |
| fx_para_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_para_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_para_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_para_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_para_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_para_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_005 frame 0, Full | 1 | 1 | 3 | none | PASS |
| fx_para_005 frame 1, Full | 1 | 1 | 3 | none | PASS |
| fx_para_005 frame 2, Full | 1 | 1 | 3 | none | PASS |
| fx_para_005 frame 3, Full | 1 | 1 | 3 | none | PASS |
| fx_para_005 frame 4, Full | 1 | 1 | 3 | none | PASS |
| fx_para_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_para_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_para_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_para_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_para_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_para_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_para_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_para_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_para_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_para_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_para_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_para_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_para_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_para_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_para_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_para_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_para_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_para_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_para_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_para_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_para_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_para_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_para_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_para_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_para_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_para_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_para_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_para_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_para_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_para_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_para_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_012 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_para_012 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_para_012 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_para_012 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_para_012 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_para_012 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_para_012 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_para_012 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_para_012 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_para_012 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_para_013 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_para_013 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_para_013 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_para_013 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_para_013 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_para_013 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_para_013 frame 1, Draft | 0 | 1 | 2 | none | PASS |
| fx_para_013 frame 2, Draft | 0 | 1 | 2 | none | PASS |
| fx_para_013 frame 3, Draft | 0 | 1 | 2 | none | PASS |
| fx_para_013 frame 4, Draft | 0 | 1 | 2 | none | PASS |
| fx_para_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_para_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_para_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_para_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_para_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_para_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_015 frame 0, Full | 1 | 1 | 3 | none | PASS |
| fx_para_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_para_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_para_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_para_015 frame 4, Full | 1 | 1 | 3 | none | PASS |
| fx_para_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_016 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_para_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_para_016 frame 2, Full | 1 | 1 | 3 | none | PASS |
| fx_para_016 frame 3, Full | 1 | 1 | 3 | none | PASS |
| fx_para_016 frame 4, Full | 1 | 1 | 3 | none | PASS |
| fx_para_016 frame 0, Draft | 0 | 1 | 2 | none | PASS |
| fx_para_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_para_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_para_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_para_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_para_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_para_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_para_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_018 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_018 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_018 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_018 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_018 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_019 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_019 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_019 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_019 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_019 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_020 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_020 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_020 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_020 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_020 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_021 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_021 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_021 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_021 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_021 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_022 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_022 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_022 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_022 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_022 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_023 frame 0, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_023 frame 1, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_023 frame 2, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_023 frame 3, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_para_023 frame 4, Draft | 0 | 1 | 2 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_003 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_003 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_003 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_003 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_003 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_004 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_004 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_004 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_004 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_004 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_004 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_004 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_004 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_004 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_004 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_019 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_019 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_020 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_kira_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_020 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_kira_020 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_020 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_020 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_020 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_021 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_021 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_021 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_021 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_021 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_kira_021 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_021 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_021 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_021 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_021 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_kira_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_028 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_028 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_028 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_028 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_028 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_028 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_028 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_028 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_028 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_028 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_029 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_029 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_029 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_029 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_029 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_029 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_029 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_029 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_029 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_kira_029 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| the reference shot with Color Lookup frame 0, Full | 3 | 1 | 817 | none | PASS |
| the reference shot with Color Lookup frame 100, Full | 3 | 1 | 2234 | none | PASS |
| the reference shot with Color Lookup frame 239, Full | 3 | 1 | 890 | none | PASS |
| the reference shot with Color Lookup frame 0, Draft | 3 | 1 | 65 | none | PASS |
| the reference shot with Color Lookup frame 100, Draft | 3 | 1 | 54 | none | PASS |
| the reference shot with Color Lookup frame 239, Draft | 3 | 1 | 55 | none | PASS |
| the reference shot with Line Blur frame 0, Full | 3 | 1 | 1096 | none | PASS |
| the reference shot with Line Blur frame 100, Full | 3 | 1 | 937 | none | PASS |
| the reference shot with Line Blur frame 239, Full | 3 | 1 | 754 | none | PASS |
| the reference shot with Line Blur frame 0, Draft | 3 | 1 | 64 | none | PASS |
| the reference shot with Line Blur frame 100, Draft | 3 | 1 | 71 | none | PASS |
| the reference shot with Line Blur frame 239, Draft | 3 | 1 | 52 | none | PASS |
| the reference shot with HSV Key frame 0, Full | 3 | 1 | 1444 | none | PASS |
| the reference shot with HSV Key frame 100, Full | 3 | 1 | 1247 | none | PASS |
| the reference shot with HSV Key frame 239, Full | 3 | 1 | 1273 | none | PASS |
| the reference shot with HSV Key frame 0, Draft | 3 | 1 | 34 | none | PASS |
| the reference shot with HSV Key frame 100, Draft | 3 | 1 | 30 | none | PASS |
| the reference shot with HSV Key frame 239, Draft | 3 | 1 | 31 | none | PASS |
| the reference shot with Paraffin frame 0, Full | 3 | 1 | 863 | none | PASS |
| the reference shot with Paraffin frame 100, Full | 3 | 1 | 881 | none | PASS |
| the reference shot with Paraffin frame 239, Full | 3 | 1 | 625 | none | PASS |
| the reference shot with Paraffin frame 0, Draft | 3 | 1 | 55 | none | PASS |
| the reference shot with Paraffin frame 100, Draft | 3 | 1 | 41 | none | PASS |
| the reference shot with Paraffin frame 239, Draft | 3 | 1 | 44 | none | PASS |
| the reference shot with Kira-kira frame 0, Full | 3 | 1 | 3783 | none | PASS |
| the reference shot with Kira-kira frame 100, Full | 3 | 1 | 1418 | none | PASS |
| the reference shot with Kira-kira frame 239, Full | 3 | 1 | 1753 | none | PASS |
| the reference shot with Kira-kira frame 0, Draft | 3 | 1 | 128 | none | PASS |
| the reference shot with Kira-kira frame 100, Draft | 3 | 1 | 31 | none | PASS |
| the reference shot with Kira-kira frame 239, Draft | 3 | 1 | 48 | none | PASS |
| the reference shot with Color Lookup frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Color Lookup frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Line Blur frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Line Blur frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with HSV Key frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with HSV Key frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Paraffin frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Paraffin frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Kira-kira frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Kira-kira frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
