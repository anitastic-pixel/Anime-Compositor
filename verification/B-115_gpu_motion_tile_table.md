# B-115: Motion Tile on the GPU against the CPU

Written by `tests/b115_gpu_motion_tile.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU, with the layer's last effect, a Motion Tile, done on the card. **The rule: no channel of any pixel more than 1 level of 255 apart** (D-178). An effect that changes nothing or whose settings are invalid is not left to the card; on those rows any difference is the card's layering, held to the same 1 level by D-100. On every row both paths must give the same warnings, and the card must draw the frame itself, except a frame with an adjustment layer, which the CPU draws by B-44's rule: that one must be the CPU's picture exactly, the card's message `GPU_PREVIEW_ON_CPU` its only extra warning.

**238 of 238 checks pass.**

The worst comparison is "the reference shot with Motion Tile frame 0, Full": largest difference 1 of 255, pixels differing: 3783. Its pictures are in `verification/B-115 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## Each effect

Every fixture frame of the effect and the reference shot with it, at Full and Draft.

| Effect | Frames compared | Frames with it on the card | Largest difference (of 255) | Pass |
|---|---:|---:|---:|---|
| Motion Tile | 236 | 142 | 1 | 236 of 236 |

## Every frame

| Case | Effects left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---|---|
| fx_tile_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_tile_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_tile_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_tile_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_tile_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_tile_001 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_tile_001 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_tile_001 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_tile_001 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_tile_001 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_tile_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_tile_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_tile_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_tile_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_tile_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_tile_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_tile_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_tile_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_tile_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_tile_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_tile_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_013 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_tile_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_013 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_tile_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_014 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_tile_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_014 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_tile_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_tile_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_tile_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_tile_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| the reference shot with Motion Tile frame 0, Full | 3 | 1 | 3783 | none | PASS |
| the reference shot with Motion Tile frame 100, Full | 3 | 1 | 1417 | none | PASS |
| the reference shot with Motion Tile frame 239, Full | 3 | 1 | 1772 | none | PASS |
| the reference shot with Motion Tile frame 0, Draft | 3 | 1 | 128 | none | PASS |
| the reference shot with Motion Tile frame 100, Draft | 3 | 1 | 32 | none | PASS |
| the reference shot with Motion Tile frame 239, Draft | 3 | 1 | 47 | none | PASS |
| the reference shot with Motion Tile frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with Motion Tile frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
