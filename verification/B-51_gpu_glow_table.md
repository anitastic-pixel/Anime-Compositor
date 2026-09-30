# B-51: Glow on the GPU against the CPU

Written by `tests/b51_gpu_glow.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU, with the layer's last Glow done on the card. **The rule: no channel of any pixel more than 1 level of 255 apart** (D-108, proposed). A Glow at intensity 0 or with nothing that glows changes nothing, and one with invalid settings is reported and skipped, so none of these is left to the card; on those rows any difference is the card's layering, held to the same 1 level by D-100. On every row both paths must give the same warnings.

**338 of 338 checks pass.**

The worst comparison is "the reference shot with three Glows frame 0, Full": largest difference 1 of 255, pixels differing: 1981. Its pictures are in `verification/B-51 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

| Case | Glows left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---|---|
| fx_glow_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_004 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_004 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_004 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_004 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_004 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_004 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_004 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_004 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_004 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_004 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_008 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_008 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_008 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_008 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_008 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_008 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_008 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_008 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_008 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_008 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_010 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_010 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_010 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_010 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_010 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_010 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_010 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_010 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_017 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_017 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_018 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_glow_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_018 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_glow_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_019 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_020 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_020 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_020 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_020 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_020 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_020 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_021 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_021 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_021 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_021 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_021 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_glow_021 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_021 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_021 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_021 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_021 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_glow_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_028 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_028 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_028 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_028 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_028 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_028 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_028 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_028 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_028 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_028 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_029 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_029 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_029 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_029 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_029 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_029 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_029 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_029 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_029 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_029 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_030 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_030 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_030 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_030 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_030 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_030 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_030 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_030 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_030 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_030 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_031 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_031 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_031 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_031 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_031 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_031 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_031 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_031 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_031 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_031 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_032 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_032 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_032 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_032 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_032 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_032 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_032 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_032 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_032 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_032 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_033 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_033 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_033 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_033 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_033 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_033 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_033 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_033 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_033 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_glow_033 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| the reference shot with three Glows frame 0, Full | 3 | 1 | 1981 | none | PASS |
| the reference shot with three Glows frame 100, Full | 3 | 1 | 808 | none | PASS |
| the reference shot with three Glows frame 239, Full | 3 | 1 | 625 | none | PASS |
| the reference shot with three Glows frame 0, Draft | 3 | 1 | 107 | none | PASS |
| the reference shot with three Glows frame 100, Draft | 3 | 1 | 62 | none | PASS |
| the reference shot with three Glows frame 239, Draft | 3 | 1 | 50 | none | PASS |
| the reference shot with three Glows frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with three Glows frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
