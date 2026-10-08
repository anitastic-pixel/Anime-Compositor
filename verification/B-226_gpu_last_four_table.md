# B-226: the last four of P0-2 on the GPU against the CPU

Written by `tests/b226_gpu_last_four.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Block Dissolve, Gradient Wipe, Line Smoothing and Line Width, each the CPU's rule on the card (D-346). Block Dissolve's blocks are chosen by the CPU and handed to the card, so its seeds stay the CPU's (P0-23). Line Smoothing and Line Width begin their run on the card, so they are given the CPU's drawing, and the card is handed the CPU's encoded picture and chosen pixels, which they decide by exact comparisons of.

The cases: every fixture naming one of the four (82 files), at every frame it has; and the reference shot with each effect on its first three layers (the second after a Drop Shadow), 15 settings, at frames 0, 100 and 239. Each at Full and Draft.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU. **The rule: no channel of any pixel more than 1 level of 255 apart** (ADR-006, D-100), the same warnings on both, and on a reference shot row the effect in fact on the card, at Full the first layer's. 8 bpc and After Effects 32 bpc compositions give the card no effect (D-330, D-333); a few frames the card refuses whole (Float depth, an adjustment layer): those must be the CPU's picture exactly, with the card's message `GPU_PREVIEW_ON_CPU` its only extra warning.

**940 of 940 checks pass.**

The CPU drawing each plan made for the card draws the plan made for the CPU byte for byte in 30 of 30.

The worst comparison is "the reference shot with Gradient Wipe (2) frame 100, Full": largest difference 1 of 255, pixels differing: 20052. Its pictures are in `verification/B-226 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## Each effect

| Effect | Frames compared | Frames with an effect on the card | Frames the card refused | Largest difference (of 255) | Pass |
|---|---:|---:|---:|---:|---|
| Block Dissolve | 214 | 140 | 0 | 1 | 214 of 214 |
| Gradient Wipe | 304 | 192 | 0 | 1 | 304 of 304 |
| Line Smoothing | 178 | 136 | 0 | 1 | 178 of 178 |
| Line Width | 214 | 132 | 0 | 1 | 214 of 214 |

## Every frame

Effects left to the card on the first three layers.

| Case | Left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---|---:|---:|---|---|
| block_dissolve/fx_bdissolve_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_001 frame 0, Draft | 0 | 1 | 4 | none | PASS |
| block_dissolve/fx_bdissolve_001 frame 1, Draft | 0 | 1 | 4 | none | PASS |
| block_dissolve/fx_bdissolve_001 frame 2, Draft | 0 | 1 | 4 | none | PASS |
| block_dissolve/fx_bdissolve_001 frame 3, Draft | 0 | 1 | 4 | none | PASS |
| block_dissolve/fx_bdissolve_001 frame 4, Draft | 0 | 1 | 4 | none | PASS |
| block_dissolve/fx_bdissolve_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_009 frame 0, Full | 0 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_009 frame 0, Draft | 0 | 1 | 4 | none | PASS |
| block_dissolve/fx_bdissolve_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_010 frame 0, Draft | 0 | 1 | 4 | none | PASS |
| block_dissolve/fx_bdissolve_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| block_dissolve/fx_bdissolve_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_014 frame 0, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_014 frame 1, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_014 frame 2, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_014 frame 3, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_014 frame 4, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_015 frame 0, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_015 frame 1, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_015 frame 2, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_015 frame 3, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_015 frame 4, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_016 frame 0, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_016 frame 1, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_016 frame 2, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_016 frame 3, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_016 frame 4, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_017 frame 0, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_017 frame 1, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_017 frame 2, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_017 frame 3, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_017 frame 4, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_018 frame 0, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_018 frame 1, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_018 frame 2, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_018 frame 3, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_018 frame 4, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_019 frame 0, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_019 frame 1, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_019 frame 2, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_019 frame 3, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| block_dissolve/fx_bdissolve_019 frame 4, Draft | 0 | 1 | 4 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_001 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_001 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_001 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_001 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_001 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_005 frame 0, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_005 frame 1, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_005 frame 2, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_005 frame 3, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_005 frame 4, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_005 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_005 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_005 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_005 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_005 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_017 frame 0, Full | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_017 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_019 frame 0, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| gradient_wipe/fx_gwipe_019 frame 1, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| gradient_wipe/fx_gwipe_019 frame 2, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| gradient_wipe/fx_gwipe_019 frame 3, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| gradient_wipe/fx_gwipe_019 frame 4, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| gradient_wipe/fx_gwipe_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| gradient_wipe/fx_gwipe_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| gradient_wipe/fx_gwipe_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| gradient_wipe/fx_gwipe_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| gradient_wipe/fx_gwipe_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| gradient_wipe/fx_gwipe_020 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_020 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_020 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_020 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_020 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_020 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_021 frame 0, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_021 frame 1, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_021 frame 2, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_021 frame 3, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_021 frame 4, Full | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_021 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_021 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_021 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_021 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_021 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_022 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_022 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_022 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_022 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_022 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_022 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_022 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_022 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_022 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_022 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| gradient_wipe/fx_gwipe_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_028 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_028 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_028 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_028 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_028 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_028 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_028 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_028 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_028 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| gradient_wipe/fx_gwipe_028 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_004 frame 0, Full | 0 | 0 | 0 | none | PASS |
| line_width/fx_width_004 frame 1, Full | 0 | 0 | 0 | none | PASS |
| line_width/fx_width_004 frame 2, Full | 0 | 0 | 0 | none | PASS |
| line_width/fx_width_004 frame 3, Full | 0 | 0 | 0 | none | PASS |
| line_width/fx_width_004 frame 4, Full | 0 | 0 | 0 | none | PASS |
| line_width/fx_width_004 frame 0, Draft | 0 | 1 | 1 | none | PASS |
| line_width/fx_width_004 frame 1, Draft | 0 | 1 | 1 | none | PASS |
| line_width/fx_width_004 frame 2, Draft | 0 | 1 | 1 | none | PASS |
| line_width/fx_width_004 frame 3, Draft | 0 | 1 | 1 | none | PASS |
| line_width/fx_width_004 frame 4, Draft | 0 | 1 | 1 | none | PASS |
| line_width/fx_width_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_008 frame 0, Full | 0 | 0 | 0 | none | PASS |
| line_width/fx_width_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_008 frame 0, Draft | 0 | 1 | 1 | none | PASS |
| line_width/fx_width_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| line_width/fx_width_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_013 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_013 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_013 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_013 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_013 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_014 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_014 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_014 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_014 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_014 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_015 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_015 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_015 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_015 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_015 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_016 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_016 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_016 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_016 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_016 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_017 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_017 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_017 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_017 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_017 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_018 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_018 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_018 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_018 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_018 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_019 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_019 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_019 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_019 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| line_width/fx_width_019 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| smooth/fx_smooth_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| smooth/fx_smooth_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| smooth/fx_smooth_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| smooth/fx_smooth_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| smooth/fx_smooth_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| smooth/fx_smooth_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| smooth/fx_smooth_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| smooth/fx_smooth_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| smooth/fx_smooth_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| smooth/fx_smooth_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_012 frame 0, Full | 0 | 0 | 0 | none | PASS |
| smooth/fx_smooth_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_012 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| smooth/fx_smooth_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| smooth/fx_smooth_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| smooth/fx_smooth_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| the reference shot with Block Dissolve (1) frame 0, Full | 1 / 2 / 1 | 1 | 2531 | none | PASS |
| the reference shot with Block Dissolve (1) frame 100, Full | 1 / 2 / 1 | 1 | 857 | none | PASS |
| the reference shot with Block Dissolve (1) frame 239, Full | 1 / 2 / 1 | 1 | 1152 | none | PASS |
| the reference shot with Block Dissolve (1) frame 0, Draft | 1 / 2 / 1 | 1 | 79 | none | PASS |
| the reference shot with Block Dissolve (1) frame 100, Draft | 1 / 2 / 1 | 1 | 16 | none | PASS |
| the reference shot with Block Dissolve (1) frame 239, Draft | 1 / 2 / 1 | 1 | 28 | none | PASS |
| the reference shot with Block Dissolve (2) frame 0, Full | 1 / 2 / 1 | 1 | 1401 | none | PASS |
| the reference shot with Block Dissolve (2) frame 100, Full | 1 / 2 / 1 | 1 | 730 | none | PASS |
| the reference shot with Block Dissolve (2) frame 239, Full | 1 / 2 / 1 | 1 | 915 | none | PASS |
| the reference shot with Block Dissolve (2) frame 0, Draft | 1 / 2 / 1 | 1 | 124 | none | PASS |
| the reference shot with Block Dissolve (2) frame 100, Draft | 1 / 2 / 1 | 1 | 95 | none | PASS |
| the reference shot with Block Dissolve (2) frame 239, Draft | 1 / 2 / 1 | 1 | 102 | none | PASS |
| the reference shot with Block Dissolve (3) frame 0, Full | 1 / 2 / 1 | 1 | 1703 | none | PASS |
| the reference shot with Block Dissolve (3) frame 100, Full | 1 / 2 / 1 | 1 | 1228 | none | PASS |
| the reference shot with Block Dissolve (3) frame 239, Full | 1 / 2 / 1 | 1 | 1979 | none | PASS |
| the reference shot with Block Dissolve (3) frame 0, Draft | 1 / 2 / 1 | 1 | 94 | none | PASS |
| the reference shot with Block Dissolve (3) frame 100, Draft | 1 / 2 / 1 | 1 | 84 | none | PASS |
| the reference shot with Block Dissolve (3) frame 239, Draft | 1 / 2 / 1 | 1 | 91 | none | PASS |
| the reference shot with Block Dissolve (4) frame 0, Full | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Block Dissolve (4) frame 100, Full | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Block Dissolve (4) frame 239, Full | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Block Dissolve (4) frame 0, Draft | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Block Dissolve (4) frame 100, Draft | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Block Dissolve (4) frame 239, Draft | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Gradient Wipe (1) frame 0, Full | 1 / 2 / 1 | 1 | 3781 | none | PASS |
| the reference shot with Gradient Wipe (1) frame 100, Full | 1 / 2 / 1 | 1 | 1417 | none | PASS |
| the reference shot with Gradient Wipe (1) frame 239, Full | 1 / 2 / 1 | 1 | 1772 | none | PASS |
| the reference shot with Gradient Wipe (1) frame 0, Draft | 1 / 2 / 1 | 1 | 128 | none | PASS |
| the reference shot with Gradient Wipe (1) frame 100, Draft | 1 / 2 / 1 | 1 | 32 | none | PASS |
| the reference shot with Gradient Wipe (1) frame 239, Draft | 1 / 2 / 1 | 1 | 47 | none | PASS |
| the reference shot with Gradient Wipe (2) frame 0, Full | 1 / 2 / 1 | 1 | 2824 | none | PASS |
| the reference shot with Gradient Wipe (2) frame 100, Full | 1 / 2 / 1 | 1 | 20052 | none | PASS |
| the reference shot with Gradient Wipe (2) frame 239, Full | 1 / 2 / 1 | 1 | 3496 | none | PASS |
| the reference shot with Gradient Wipe (2) frame 0, Draft | 1 / 2 / 1 | 1 | 70 | none | PASS |
| the reference shot with Gradient Wipe (2) frame 100, Draft | 1 / 2 / 1 | 1 | 541 | none | PASS |
| the reference shot with Gradient Wipe (2) frame 239, Draft | 1 / 2 / 1 | 1 | 63 | none | PASS |
| the reference shot with Gradient Wipe (3) frame 0, Full | 1 / 2 / 1 | 1 | 5058 | none | PASS |
| the reference shot with Gradient Wipe (3) frame 100, Full | 1 / 2 / 1 | 1 | 1829 | none | PASS |
| the reference shot with Gradient Wipe (3) frame 239, Full | 1 / 2 / 1 | 1 | 2783 | none | PASS |
| the reference shot with Gradient Wipe (3) frame 0, Draft | 1 / 2 / 1 | 1 | 165 | none | PASS |
| the reference shot with Gradient Wipe (3) frame 100, Draft | 1 / 2 / 1 | 1 | 67 | none | PASS |
| the reference shot with Gradient Wipe (3) frame 239, Draft | 1 / 2 / 1 | 1 | 78 | none | PASS |
| the reference shot with Gradient Wipe (4) frame 0, Full | 1 / 2 / 1 | 1 | 3781 | none | PASS |
| the reference shot with Gradient Wipe (4) frame 100, Full | 1 / 2 / 1 | 1 | 1417 | none | PASS |
| the reference shot with Gradient Wipe (4) frame 239, Full | 1 / 2 / 1 | 1 | 1772 | none | PASS |
| the reference shot with Gradient Wipe (4) frame 0, Draft | 1 / 2 / 1 | 1 | 128 | none | PASS |
| the reference shot with Gradient Wipe (4) frame 100, Draft | 1 / 2 / 1 | 1 | 32 | none | PASS |
| the reference shot with Gradient Wipe (4) frame 239, Draft | 1 / 2 / 1 | 1 | 47 | none | PASS |
| the reference shot with Line Smoothing (1) frame 0, Full | 1 / 1 / 1 | 1 | 16869 | none | PASS |
| the reference shot with Line Smoothing (1) frame 100, Full | 1 / 1 / 1 | 1 | 11007 | none | PASS |
| the reference shot with Line Smoothing (1) frame 239, Full | 1 / 1 / 1 | 1 | 13622 | none | PASS |
| the reference shot with Line Smoothing (1) frame 0, Draft | 1 / 1 / 1 | 1 | 138 | none | PASS |
| the reference shot with Line Smoothing (1) frame 100, Draft | 1 / 1 / 1 | 1 | 60 | none | PASS |
| the reference shot with Line Smoothing (1) frame 239, Draft | 1 / 1 / 1 | 1 | 69 | none | PASS |
| the reference shot with Line Smoothing (2) frame 0, Full | 1 / 1 / 1 | 1 | 8264 | none | PASS |
| the reference shot with Line Smoothing (2) frame 100, Full | 1 / 1 / 1 | 1 | 7254 | none | PASS |
| the reference shot with Line Smoothing (2) frame 239, Full | 1 / 1 / 1 | 1 | 6823 | none | PASS |
| the reference shot with Line Smoothing (2) frame 0, Draft | 1 / 1 / 1 | 1 | 151 | none | PASS |
| the reference shot with Line Smoothing (2) frame 100, Draft | 1 / 1 / 1 | 1 | 62 | none | PASS |
| the reference shot with Line Smoothing (2) frame 239, Draft | 1 / 1 / 1 | 1 | 78 | none | PASS |
| the reference shot with Line Smoothing (3) frame 0, Full | 1 / 1 / 1 | 1 | 6170 | none | PASS |
| the reference shot with Line Smoothing (3) frame 100, Full | 1 / 1 / 1 | 1 | 3168 | none | PASS |
| the reference shot with Line Smoothing (3) frame 239, Full | 1 / 1 / 1 | 1 | 3961 | none | PASS |
| the reference shot with Line Smoothing (3) frame 0, Draft | 1 / 1 / 1 | 1 | 130 | none | PASS |
| the reference shot with Line Smoothing (3) frame 100, Draft | 1 / 1 / 1 | 1 | 45 | none | PASS |
| the reference shot with Line Smoothing (3) frame 239, Draft | 1 / 1 / 1 | 1 | 54 | none | PASS |
| the reference shot with Line Width (1) frame 0, Full | 1 / 1 / 1 | 1 | 3781 | none | PASS |
| the reference shot with Line Width (1) frame 100, Full | 1 / 1 / 1 | 1 | 1417 | none | PASS |
| the reference shot with Line Width (1) frame 239, Full | 1 / 1 / 1 | 1 | 1772 | none | PASS |
| the reference shot with Line Width (1) frame 0, Draft | 1 / 1 / 1 | 1 | 128 | none | PASS |
| the reference shot with Line Width (1) frame 100, Draft | 1 / 1 / 1 | 1 | 32 | none | PASS |
| the reference shot with Line Width (1) frame 239, Draft | 1 / 1 / 1 | 1 | 47 | none | PASS |
| the reference shot with Line Width (2) frame 0, Full | 1 / 1 / 1 | 1 | 3781 | none | PASS |
| the reference shot with Line Width (2) frame 100, Full | 1 / 1 / 1 | 1 | 1420 | none | PASS |
| the reference shot with Line Width (2) frame 239, Full | 1 / 1 / 1 | 1 | 1772 | none | PASS |
| the reference shot with Line Width (2) frame 0, Draft | 1 / 1 / 1 | 1 | 128 | none | PASS |
| the reference shot with Line Width (2) frame 100, Draft | 1 / 1 / 1 | 1 | 32 | none | PASS |
| the reference shot with Line Width (2) frame 239, Draft | 1 / 1 / 1 | 1 | 47 | none | PASS |
| the reference shot with Line Width (3) frame 0, Full | 1 / 1 / 1 | 1 | 3678 | none | PASS |
| the reference shot with Line Width (3) frame 100, Full | 1 / 1 / 1 | 1 | 1397 | none | PASS |
| the reference shot with Line Width (3) frame 239, Full | 1 / 1 / 1 | 1 | 1676 | none | PASS |
| the reference shot with Line Width (3) frame 0, Draft | 1 / 1 / 1 | 1 | 128 | none | PASS |
| the reference shot with Line Width (3) frame 100, Draft | 1 / 1 / 1 | 1 | 32 | none | PASS |
| the reference shot with Line Width (3) frame 239, Draft | 1 / 1 / 1 | 1 | 47 | none | PASS |
| the reference shot with Line Width (4) frame 0, Full | 1 / 1 / 1 | 1 | 4250 | none | PASS |
| the reference shot with Line Width (4) frame 100, Full | 1 / 1 / 1 | 1 | 1550 | none | PASS |
| the reference shot with Line Width (4) frame 239, Full | 1 / 1 / 1 | 1 | 2123 | none | PASS |
| the reference shot with Line Width (4) frame 0, Draft | 1 / 1 / 1 | 1 | 128 | none | PASS |
| the reference shot with Line Width (4) frame 100, Draft | 1 / 1 / 1 | 1 | 32 | none | PASS |
| the reference shot with Line Width (4) frame 239, Draft | 1 / 1 / 1 | 1 | 47 | none | PASS |
