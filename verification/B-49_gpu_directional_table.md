# B-49: Directional Blur on the GPU against the CPU

Written by `tests/b49_gpu_directional.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU, with the layer's last Directional Blur done on the card. **The rule: no channel of any pixel more than 1 level of 255 apart** (D-106, proposed). A blur of length 0 changes nothing, so it is not left to the card and the two pictures must be the same bytes. FX-DIRBLUR-012 to 015 each have a setting out of range: the blur is left out with the warning `EFFECT_PARAMETER_INVALID`, nothing goes to the card, and the two pictures must be the same bytes. On every row both paths must give the same warnings.

**158 of 158 checks pass.**

The worst comparison is "the reference shot with three Directional Blurs frame 100, Full": largest difference 1 of 255, pixels differing: 14674. Its pictures are in `verification/B-49 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

| Case | Blurs left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---|---|
| fx_dirblur_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_002 frame 0, Full | 1 | 1 | 1 | none | PASS |
| fx_dirblur_002 frame 1, Full | 1 | 1 | 1 | none | PASS |
| fx_dirblur_002 frame 2, Full | 1 | 1 | 1 | none | PASS |
| fx_dirblur_002 frame 3, Full | 1 | 1 | 1 | none | PASS |
| fx_dirblur_002 frame 4, Full | 1 | 1 | 1 | none | PASS |
| fx_dirblur_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_003 frame 0, Full | 1 | 1 | 1 | none | PASS |
| fx_dirblur_003 frame 1, Full | 1 | 1 | 1 | none | PASS |
| fx_dirblur_003 frame 2, Full | 1 | 1 | 1 | none | PASS |
| fx_dirblur_003 frame 3, Full | 1 | 1 | 1 | none | PASS |
| fx_dirblur_003 frame 4, Full | 1 | 1 | 1 | none | PASS |
| fx_dirblur_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_005 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_dirblur_005 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_dirblur_005 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_dirblur_005 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_dirblur_005 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_dirblur_005 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_dirblur_005 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_dirblur_005 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_dirblur_005 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_dirblur_005 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_dirblur_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_dirblur_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_010 frame 2, Full | 1 | 1 | 1 | none | PASS |
| fx_dirblur_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_dirblur_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_011 frame 1, Full | 1 | 1 | 1 | none | PASS |
| fx_dirblur_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_dirblur_011 frame 3, Full | 1 | 1 | 2 | none | PASS |
| fx_dirblur_011 frame 4, Full | 1 | 1 | 1 | none | PASS |
| fx_dirblur_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_dirblur_012 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_012 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_012 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_012 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_012 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_012 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_012 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_012 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_012 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_012 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_013 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_013 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_013 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_013 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_013 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_dirblur_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| the reference shot with three Directional Blurs frame 0, Full | 3 | 1 | 14135 | none | PASS |
| the reference shot with three Directional Blurs frame 100, Full | 3 | 1 | 14674 | none | PASS |
| the reference shot with three Directional Blurs frame 239, Full | 3 | 1 | 14008 | none | PASS |
| the reference shot with three Directional Blurs frame 0, Draft | 3 | 1 | 2338 | none | PASS |
| the reference shot with three Directional Blurs frame 100, Draft | 3 | 1 | 2292 | none | PASS |
| the reference shot with three Directional Blurs frame 239, Draft | 3 | 1 | 2285 | none | PASS |
| the reference shot with three Directional Blurs frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with three Directional Blurs frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
