# B-46: Radial Blur on the GPU against the CPU

Written by `tests/b46_gpu_radial.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU, with the layer's last Radial Blur done on the card. **The rule: no channel of any pixel more than 1 level of 255 apart** (D-103, proposed), and the blur must in fact have been left to the card. FX-RADIAL-013 to 018 each have a setting out of range: the blur is left out with the warning `EFFECT_PARAMETER_INVALID`, nothing goes to the card, and the two pictures must be the same bytes. On every row both paths must give the same warnings.

**188 of 188 checks pass.**

The worst comparison is "the reference shot with three Radial Blurs frame 0, Full": largest difference 1 of 255, pixels differing: 20539. Its pictures are in `verification/B-46 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

| Case | Blurs left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---|---|
| fx_radial_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_005 frame 0, Draft | 1 | 1 | 1 | none | PASS |
| fx_radial_005 frame 1, Draft | 1 | 1 | 1 | none | PASS |
| fx_radial_005 frame 2, Draft | 1 | 1 | 1 | none | PASS |
| fx_radial_005 frame 3, Draft | 1 | 1 | 1 | none | PASS |
| fx_radial_005 frame 4, Draft | 1 | 1 | 1 | none | PASS |
| fx_radial_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_009 frame 2, Full | 1 | 1 | 1 | none | PASS |
| fx_radial_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_radial_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_012 frame 0, Full | 1 | 1 | 3 | none | PASS |
| fx_radial_012 frame 1, Full | 1 | 1 | 3 | none | PASS |
| fx_radial_012 frame 2, Full | 1 | 1 | 3 | none | PASS |
| fx_radial_012 frame 3, Full | 1 | 1 | 3 | none | PASS |
| fx_radial_012 frame 4, Full | 1 | 1 | 3 | none | PASS |
| fx_radial_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_radial_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_013 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_013 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_013 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_013 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_013 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_radial_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| the reference shot with three Radial Blurs frame 0, Full | 3 | 1 | 20539 | none | PASS |
| the reference shot with three Radial Blurs frame 100, Full | 3 | 1 | 19707 | none | PASS |
| the reference shot with three Radial Blurs frame 239, Full | 3 | 1 | 20356 | none | PASS |
| the reference shot with three Radial Blurs frame 0, Draft | 3 | 1 | 1029 | none | PASS |
| the reference shot with three Radial Blurs frame 100, Draft | 3 | 1 | 1023 | none | PASS |
| the reference shot with three Radial Blurs frame 239, Draft | 3 | 1 | 1018 | none | PASS |
| the reference shot with three Radial Blurs frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with three Radial Blurs frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
