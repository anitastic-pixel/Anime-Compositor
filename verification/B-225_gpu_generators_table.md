# B-225: five generators on the GPU against the CPU

Written by `tests/b225_gpu_generators.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Beam, 4-Color Gradient, CC Light Sweep, Advanced Lightning and Bevel Edges, each the CPU's rule on the card (D-344); Radio Waves stays on the CPU (D-345). The bolt's segments are worked out by the CPU and handed to the card as a list, so its seeds stay the CPU's (P0-23).

The cases: every fixture naming one of the five (183 files), at every frame it has; and the reference shot with each effect on its first three layers (the second after a Drop Shadow), 19 settings, at frames 0, 100 and 239. Each at Full and Draft.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU. **The rule: no channel of any pixel more than 1 level of 255 apart** (ADR-006, D-100), the same warnings on both, and on a reference shot row the effect in fact on the card, at Full the first layer's. 8 bpc and After Effects 32 bpc compositions give the card no effect (D-330, D-333); a few frames the card refuses whole (Float depth, an adjustment layer): those must be the CPU's picture exactly, with the card's message `GPU_PREVIEW_ON_CPU` its only extra warning.

**1982 of 1982 checks pass.**

The CPU drawing each plan made for the card draws the plan made for the CPU byte for byte in 38 of 38.

The worst comparison is "the reference shot with CC Light Sweep (4) frame 0, Full": largest difference 1 of 255, pixels differing: 3847. Its pictures are in `verification/B-225 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## Each effect

| Effect | Frames compared | Frames with an effect on the card | Frames the card refused | Largest difference (of 255) | Pass |
|---|---:|---:|---:|---:|---|
| Beam | 308 | 208 | 0 | 1 | 308 of 308 |
| 4-Color Gradient | 284 | 184 | 0 | 1 | 284 of 284 |
| CC Light Sweep | 284 | 192 | 0 | 1 | 284 of 284 |
| Advanced Lightning | 940 | 688 | 0 | 1 | 940 of 940 |
| Bevel Edges | 128 | 88 | 0 | 1 | 128 of 128 |

## Every frame

Effects left to the card on the first three layers.

| Case | Left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---|---:|---:|---|---|
| beam/fx_beam_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_018 frame 0, Full | 2 | 0 | 0 | none | PASS |
| beam/fx_beam_018 frame 1, Full | 2 | 0 | 0 | none | PASS |
| beam/fx_beam_018 frame 2, Full | 2 | 0 | 0 | none | PASS |
| beam/fx_beam_018 frame 3, Full | 2 | 0 | 0 | none | PASS |
| beam/fx_beam_018 frame 4, Full | 2 | 0 | 0 | none | PASS |
| beam/fx_beam_018 frame 0, Draft | 2 | 0 | 0 | none | PASS |
| beam/fx_beam_018 frame 1, Draft | 2 | 0 | 0 | none | PASS |
| beam/fx_beam_018 frame 2, Draft | 2 | 0 | 0 | none | PASS |
| beam/fx_beam_018 frame 3, Draft | 2 | 0 | 0 | none | PASS |
| beam/fx_beam_018 frame 4, Draft | 2 | 0 | 0 | none | PASS |
| beam/fx_beam_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_019 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| beam/fx_beam_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_028 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_028 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_028 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_028 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_028 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_028 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_028 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_028 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_028 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_028 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_029 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_029 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_029 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_029 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_029 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_029 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_029 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_029 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_029 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| beam/fx_beam_029 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_013 frame 0, Full | 0 | 0 | 0 | none | PASS |
| bevel/fx_bevel_013 frame 1, Full | 0 | 0 | 0 | none | PASS |
| bevel/fx_bevel_013 frame 2, Full | 0 | 0 | 0 | none | PASS |
| bevel/fx_bevel_013 frame 3, Full | 0 | 0 | 0 | none | PASS |
| bevel/fx_bevel_013 frame 4, Full | 0 | 0 | 0 | none | PASS |
| bevel/fx_bevel_013 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| bevel/fx_bevel_013 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| bevel/fx_bevel_013 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| bevel/fx_bevel_013 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| bevel/fx_bevel_013 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| bevel/fx_bevel_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| bevel/fx_bevel_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| bevel/fx_bevel_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_001 frame 0, Draft | 1 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_001 frame 1, Draft | 1 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_001 frame 2, Draft | 1 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_001 frame 3, Draft | 1 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_001 frame 4, Draft | 1 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_006 frame 0, Full | 0 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_006 frame 1, Full | 0 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_006 frame 2, Full | 0 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_006 frame 3, Full | 0 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_006 frame 4, Full | 0 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_006 frame 0, Draft | 0 | 1 | 3 | none | PASS |
| four_color_gradient/fx_4cg_006 frame 1, Draft | 0 | 1 | 3 | none | PASS |
| four_color_gradient/fx_4cg_006 frame 2, Draft | 0 | 1 | 3 | none | PASS |
| four_color_gradient/fx_4cg_006 frame 3, Draft | 0 | 1 | 3 | none | PASS |
| four_color_gradient/fx_4cg_006 frame 4, Draft | 0 | 1 | 3 | none | PASS |
| four_color_gradient/fx_4cg_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_010 frame 0, Draft | 1 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_010 frame 1, Draft | 1 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_010 frame 2, Draft | 1 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_010 frame 3, Draft | 1 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_010 frame 4, Draft | 1 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_013 frame 0, Draft | 1 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_014 frame 0, Draft | 1 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_016 frame 0, Full | 2 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_016 frame 1, Full | 2 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_016 frame 2, Full | 2 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_016 frame 3, Full | 2 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_016 frame 4, Full | 2 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_016 frame 0, Draft | 2 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_016 frame 1, Draft | 2 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_016 frame 2, Draft | 2 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_016 frame 3, Draft | 2 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_016 frame 4, Draft | 2 | 1 | 1 | none | PASS |
| four_color_gradient/fx_4cg_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| four_color_gradient/fx_4cg_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_018 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_018 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_018 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_018 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_018 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_019 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_019 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_019 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_019 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_019 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_020 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_020 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_020 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_020 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_020 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_021 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_021 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_021 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_021 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_021 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_022 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_022 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_022 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_022 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_022 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_023 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_023 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_023 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_023 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_023 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_024 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_024 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_024 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_024 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_024 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_025 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_025 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_025 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_025 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_025 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_026 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_026 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_026 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_026 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| four_color_gradient/fx_4cg_026 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_003 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_003 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_003 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_003 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_003 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_016 frame 0, Full | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_016 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_019 frame 0, Full | 2 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_019 frame 1, Full | 2 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_019 frame 2, Full | 2 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_019 frame 3, Full | 2 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_019 frame 4, Full | 2 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_019 frame 0, Draft | 2 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_019 frame 1, Draft | 2 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_019 frame 2, Draft | 2 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_019 frame 3, Draft | 2 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_019 frame 4, Draft | 2 | 0 | 0 | none | PASS |
| light_sweep/fx_sweep_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| light_sweep/fx_sweep_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_around/fx_lighta_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_013 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_013 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_013 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_013 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_013 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_around/fx_lighta_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_003 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_003 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_003 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_003 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_003 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_018 frame 0, Full | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_018 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_019 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_020 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_020 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_020 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_020 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_020 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_020 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_021 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_021 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_021 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_021 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_021 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_021 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_021 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_021 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_021 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_021 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_bolt/fx_bolt_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_028 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_028 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_028 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_028 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_028 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_028 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_028 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_028 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_028 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_028 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_029 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_029 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_029 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_029 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_029 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_029 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_029 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_029 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_029 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_029 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_030 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_030 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_030 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_030 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_030 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_030 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_030 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_030 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_030 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_030 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_031 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_031 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_031 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_031 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_031 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_031 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_031 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_031 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_031 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_031 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_032 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_032 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_032 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_032 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_032 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_032 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_032 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_032 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_032 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_bolt/fx_bolt_032 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_core/fx_lcore_006 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_006 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_006 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_006 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_006 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_006 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_006 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_006 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_006 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_006 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_007 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_007 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_007 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_007 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_007 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_007 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_007 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_007 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_007 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_core/fx_lcore_007 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_019 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_extras/fx_lightx_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_extras/fx_lightx_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_forks/fx_lfork_007 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_007 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_007 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_007 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_007 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_007 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_007 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_007 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_007 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_007 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_008 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_008 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_008 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_008 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_008 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_008 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_008 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_008 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_008 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_forks/fx_lfork_008 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_full_forks/fx_lfull_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| lightning_full_forks/fx_lfull_006 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_full_forks/fx_lfull_006 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_full_forks/fx_lfull_006 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_full_forks/fx_lfull_006 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_full_forks/fx_lfull_006 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_full_forks/fx_lfull_006 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_full_forks/fx_lfull_006 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_full_forks/fx_lfull_006 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_full_forks/fx_lfull_006 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| lightning_full_forks/fx_lfull_006 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| the reference shot with Beam (1) frame 0, Full | 1 / 2 / 1 | 1 | 3642 | none | PASS |
| the reference shot with Beam (1) frame 100, Full | 1 / 2 / 1 | 1 | 1368 | none | PASS |
| the reference shot with Beam (1) frame 239, Full | 1 / 2 / 1 | 1 | 1694 | none | PASS |
| the reference shot with Beam (1) frame 0, Draft | 1 / 2 / 1 | 1 | 121 | none | PASS |
| the reference shot with Beam (1) frame 100, Draft | 1 / 2 / 1 | 1 | 29 | none | PASS |
| the reference shot with Beam (1) frame 239, Draft | 1 / 2 / 1 | 1 | 44 | none | PASS |
| the reference shot with Beam (2) frame 0, Full | 1 / 2 / 1 | 1 | 111 | none | PASS |
| the reference shot with Beam (2) frame 100, Full | 1 / 2 / 1 | 1 | 169 | none | PASS |
| the reference shot with Beam (2) frame 239, Full | 1 / 2 / 1 | 1 | 72 | none | PASS |
| the reference shot with Beam (2) frame 0, Draft | 1 / 2 / 1 | 1 | 7 | none | PASS |
| the reference shot with Beam (2) frame 100, Draft | 1 / 2 / 1 | 1 | 7 | none | PASS |
| the reference shot with Beam (2) frame 239, Draft | 1 / 2 / 1 | 1 | 5 | none | PASS |
| the reference shot with Beam (3) frame 0, Full | 1 / 2 / 1 | 1 | 3771 | none | PASS |
| the reference shot with Beam (3) frame 100, Full | 1 / 2 / 1 | 1 | 1417 | none | PASS |
| the reference shot with Beam (3) frame 239, Full | 1 / 2 / 1 | 1 | 1760 | none | PASS |
| the reference shot with Beam (3) frame 0, Draft | 1 / 2 / 1 | 1 | 128 | none | PASS |
| the reference shot with Beam (3) frame 100, Draft | 1 / 2 / 1 | 1 | 32 | none | PASS |
| the reference shot with Beam (3) frame 239, Draft | 1 / 2 / 1 | 1 | 47 | none | PASS |
| the reference shot with 4-Color Gradient (1) frame 0, Full | 1 / 2 / 1 | 1 | 1051 | none | PASS |
| the reference shot with 4-Color Gradient (1) frame 100, Full | 1 / 2 / 1 | 1 | 1126 | none | PASS |
| the reference shot with 4-Color Gradient (1) frame 239, Full | 1 / 2 / 1 | 1 | 1129 | none | PASS |
| the reference shot with 4-Color Gradient (1) frame 0, Draft | 1 / 2 / 1 | 1 | 72 | none | PASS |
| the reference shot with 4-Color Gradient (1) frame 100, Draft | 1 / 2 / 1 | 1 | 73 | none | PASS |
| the reference shot with 4-Color Gradient (1) frame 239, Draft | 1 / 2 / 1 | 1 | 76 | none | PASS |
| the reference shot with 4-Color Gradient (2) frame 0, Full | 1 / 2 / 1 | 1 | 1155 | none | PASS |
| the reference shot with 4-Color Gradient (2) frame 100, Full | 1 / 2 / 1 | 1 | 1224 | none | PASS |
| the reference shot with 4-Color Gradient (2) frame 239, Full | 1 / 2 / 1 | 1 | 1064 | none | PASS |
| the reference shot with 4-Color Gradient (2) frame 0, Draft | 1 / 2 / 1 | 1 | 71 | none | PASS |
| the reference shot with 4-Color Gradient (2) frame 100, Draft | 1 / 2 / 1 | 1 | 70 | none | PASS |
| the reference shot with 4-Color Gradient (2) frame 239, Draft | 1 / 2 / 1 | 1 | 69 | none | PASS |
| the reference shot with 4-Color Gradient (3) frame 0, Full | 1 / 2 / 1 | 1 | 786 | none | PASS |
| the reference shot with 4-Color Gradient (3) frame 100, Full | 1 / 2 / 1 | 1 | 993 | none | PASS |
| the reference shot with 4-Color Gradient (3) frame 239, Full | 1 / 2 / 1 | 1 | 759 | none | PASS |
| the reference shot with 4-Color Gradient (3) frame 0, Draft | 1 / 2 / 1 | 1 | 51 | none | PASS |
| the reference shot with 4-Color Gradient (3) frame 100, Draft | 1 / 2 / 1 | 1 | 67 | none | PASS |
| the reference shot with 4-Color Gradient (3) frame 239, Draft | 1 / 2 / 1 | 1 | 47 | none | PASS |
| the reference shot with 4-Color Gradient (4) frame 0, Full | 1 / 2 / 1 | 1 | 840 | none | PASS |
| the reference shot with 4-Color Gradient (4) frame 100, Full | 1 / 2 / 1 | 1 | 979 | none | PASS |
| the reference shot with 4-Color Gradient (4) frame 239, Full | 1 / 2 / 1 | 1 | 920 | none | PASS |
| the reference shot with 4-Color Gradient (4) frame 0, Draft | 1 / 2 / 1 | 1 | 49 | none | PASS |
| the reference shot with 4-Color Gradient (4) frame 100, Draft | 1 / 2 / 1 | 1 | 68 | none | PASS |
| the reference shot with 4-Color Gradient (4) frame 239, Draft | 1 / 2 / 1 | 1 | 58 | none | PASS |
| the reference shot with CC Light Sweep (1) frame 0, Full | 1 / 2 / 1 | 1 | 3762 | none | PASS |
| the reference shot with CC Light Sweep (1) frame 100, Full | 1 / 2 / 1 | 1 | 1358 | none | PASS |
| the reference shot with CC Light Sweep (1) frame 239, Full | 1 / 2 / 1 | 1 | 1753 | none | PASS |
| the reference shot with CC Light Sweep (1) frame 0, Draft | 1 / 2 / 1 | 1 | 132 | none | PASS |
| the reference shot with CC Light Sweep (1) frame 100, Draft | 1 / 2 / 1 | 1 | 36 | none | PASS |
| the reference shot with CC Light Sweep (1) frame 239, Draft | 1 / 2 / 1 | 1 | 52 | none | PASS |
| the reference shot with CC Light Sweep (2) frame 0, Full | 1 / 2 / 1 | 1 | 3393 | none | PASS |
| the reference shot with CC Light Sweep (2) frame 100, Full | 1 / 2 / 1 | 1 | 1346 | none | PASS |
| the reference shot with CC Light Sweep (2) frame 239, Full | 1 / 2 / 1 | 1 | 1335 | none | PASS |
| the reference shot with CC Light Sweep (2) frame 0, Draft | 1 / 2 / 1 | 1 | 117 | none | PASS |
| the reference shot with CC Light Sweep (2) frame 100, Draft | 1 / 2 / 1 | 1 | 69 | none | PASS |
| the reference shot with CC Light Sweep (2) frame 239, Draft | 1 / 2 / 1 | 1 | 40 | none | PASS |
| the reference shot with CC Light Sweep (3) frame 0, Full | 1 / 2 / 1 | 1 | 3 | none | PASS |
| the reference shot with CC Light Sweep (3) frame 100, Full | 1 / 2 / 1 | 1 | 1 | none | PASS |
| the reference shot with CC Light Sweep (3) frame 239, Full | 1 / 2 / 1 | 1 | 2 | none | PASS |
| the reference shot with CC Light Sweep (3) frame 0, Draft | 1 / 2 / 1 | 1 | 2 | none | PASS |
| the reference shot with CC Light Sweep (3) frame 100, Draft | 1 / 2 / 1 | 1 | 2 | none | PASS |
| the reference shot with CC Light Sweep (3) frame 239, Draft | 1 / 2 / 1 | 1 | 2 | none | PASS |
| the reference shot with CC Light Sweep (4) frame 0, Full | 1 / 2 / 1 | 1 | 3847 | none | PASS |
| the reference shot with CC Light Sweep (4) frame 100, Full | 1 / 2 / 1 | 1 | 1115 | none | PASS |
| the reference shot with CC Light Sweep (4) frame 239, Full | 1 / 2 / 1 | 1 | 1886 | none | PASS |
| the reference shot with CC Light Sweep (4) frame 0, Draft | 1 / 2 / 1 | 1 | 159 | none | PASS |
| the reference shot with CC Light Sweep (4) frame 100, Draft | 1 / 2 / 1 | 1 | 41 | none | PASS |
| the reference shot with CC Light Sweep (4) frame 239, Draft | 1 / 2 / 1 | 1 | 122 | none | PASS |
| the reference shot with Advanced Lightning (1) frame 0, Full | 1 / 2 / 1 | 1 | 3741 | none | PASS |
| the reference shot with Advanced Lightning (1) frame 100, Full | 1 / 2 / 1 | 1 | 1417 | none | PASS |
| the reference shot with Advanced Lightning (1) frame 239, Full | 1 / 2 / 1 | 1 | 1768 | none | PASS |
| the reference shot with Advanced Lightning (1) frame 0, Draft | 1 / 2 / 1 | 1 | 125 | none | PASS |
| the reference shot with Advanced Lightning (1) frame 100, Draft | 1 / 2 / 1 | 1 | 32 | none | PASS |
| the reference shot with Advanced Lightning (1) frame 239, Draft | 1 / 2 / 1 | 1 | 48 | none | PASS |
| the reference shot with Advanced Lightning (2) frame 0, Full | 1 / 2 / 1 | 1 | 3480 | none | PASS |
| the reference shot with Advanced Lightning (2) frame 100, Full | 1 / 2 / 1 | 1 | 1416 | none | PASS |
| the reference shot with Advanced Lightning (2) frame 239, Full | 1 / 2 / 1 | 1 | 1524 | none | PASS |
| the reference shot with Advanced Lightning (2) frame 0, Draft | 1 / 2 / 1 | 1 | 121 | none | PASS |
| the reference shot with Advanced Lightning (2) frame 100, Draft | 1 / 2 / 1 | 1 | 32 | none | PASS |
| the reference shot with Advanced Lightning (2) frame 239, Draft | 1 / 2 / 1 | 1 | 48 | none | PASS |
| the reference shot with Advanced Lightning (3) frame 0, Full | 1 / 2 / 1 | 1 | 514 | none | PASS |
| the reference shot with Advanced Lightning (3) frame 100, Full | 1 / 2 / 1 | 1 | 624 | none | PASS |
| the reference shot with Advanced Lightning (3) frame 239, Full | 1 / 2 / 1 | 1 | 553 | none | PASS |
| the reference shot with Advanced Lightning (3) frame 0, Draft | 1 / 2 / 1 | 1 | 38 | none | PASS |
| the reference shot with Advanced Lightning (3) frame 100, Draft | 1 / 2 / 1 | 1 | 36 | none | PASS |
| the reference shot with Advanced Lightning (3) frame 239, Draft | 1 / 2 / 1 | 1 | 31 | none | PASS |
| the reference shot with Advanced Lightning (4) frame 0, Full | 1 / 2 / 1 | 1 | 3611 | none | PASS |
| the reference shot with Advanced Lightning (4) frame 100, Full | 1 / 2 / 1 | 1 | 1418 | none | PASS |
| the reference shot with Advanced Lightning (4) frame 239, Full | 1 / 2 / 1 | 1 | 1768 | none | PASS |
| the reference shot with Advanced Lightning (4) frame 0, Draft | 1 / 2 / 1 | 1 | 124 | none | PASS |
| the reference shot with Advanced Lightning (4) frame 100, Draft | 1 / 2 / 1 | 1 | 32 | none | PASS |
| the reference shot with Advanced Lightning (4) frame 239, Draft | 1 / 2 / 1 | 1 | 47 | none | PASS |
| the reference shot with Advanced Lightning (5) frame 0, Full | 1 / 1 / 1 | 1 | 3584 | none | PASS |
| the reference shot with Advanced Lightning (5) frame 100, Full | 1 / 1 / 1 | 1 | 1287 | none | PASS |
| the reference shot with Advanced Lightning (5) frame 239, Full | 1 / 1 / 1 | 1 | 1630 | none | PASS |
| the reference shot with Advanced Lightning (5) frame 0, Draft | 1 / 1 / 1 | 1 | 122 | none | PASS |
| the reference shot with Advanced Lightning (5) frame 100, Draft | 1 / 1 / 1 | 1 | 32 | none | PASS |
| the reference shot with Advanced Lightning (5) frame 239, Draft | 1 / 1 / 1 | 1 | 43 | none | PASS |
| the reference shot with Bevel Edges (1) frame 0, Full | 1 / 2 / 1 | 1 | 3783 | none | PASS |
| the reference shot with Bevel Edges (1) frame 100, Full | 1 / 2 / 1 | 1 | 1417 | none | PASS |
| the reference shot with Bevel Edges (1) frame 239, Full | 1 / 2 / 1 | 1 | 1772 | none | PASS |
| the reference shot with Bevel Edges (1) frame 0, Draft | 1 / 2 / 1 | 1 | 128 | none | PASS |
| the reference shot with Bevel Edges (1) frame 100, Draft | 1 / 2 / 1 | 1 | 32 | none | PASS |
| the reference shot with Bevel Edges (1) frame 239, Draft | 1 / 2 / 1 | 1 | 47 | none | PASS |
| the reference shot with Bevel Edges (2) frame 0, Full | 1 / 2 / 1 | 1 | 3783 | none | PASS |
| the reference shot with Bevel Edges (2) frame 100, Full | 1 / 2 / 1 | 1 | 1433 | none | PASS |
| the reference shot with Bevel Edges (2) frame 239, Full | 1 / 2 / 1 | 1 | 1772 | none | PASS |
| the reference shot with Bevel Edges (2) frame 0, Draft | 1 / 2 / 1 | 1 | 127 | none | PASS |
| the reference shot with Bevel Edges (2) frame 100, Draft | 1 / 2 / 1 | 1 | 32 | none | PASS |
| the reference shot with Bevel Edges (2) frame 239, Draft | 1 / 2 / 1 | 1 | 47 | none | PASS |
| the reference shot with Bevel Edges (3) frame 0, Full | 1 / 2 / 1 | 1 | 3783 | none | PASS |
| the reference shot with Bevel Edges (3) frame 100, Full | 1 / 2 / 1 | 1 | 1417 | none | PASS |
| the reference shot with Bevel Edges (3) frame 239, Full | 1 / 2 / 1 | 1 | 1772 | none | PASS |
| the reference shot with Bevel Edges (3) frame 0, Draft | 1 / 2 / 1 | 1 | 128 | none | PASS |
| the reference shot with Bevel Edges (3) frame 100, Draft | 1 / 2 / 1 | 1 | 32 | none | PASS |
| the reference shot with Bevel Edges (3) frame 239, Draft | 1 / 2 / 1 | 1 | 47 | none | PASS |
