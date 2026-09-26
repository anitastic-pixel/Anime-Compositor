# B-47: Bloom on the GPU against the CPU

Written by `tests/b47_gpu_bloom.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU, with the layer's last Bloom done on the card. **The rule: no channel of any pixel more than 1 level of 255 apart** (D-104, proposed). A Bloom that lights nothing (intensity 0, or nothing as bright as the threshold) changes nothing, so it is not left to the card and the two pictures must be the same bytes. FX-BLOOM-020 to 028 each have a setting out of range: the bloom is left out with the warning `EFFECT_PARAMETER_INVALID`, nothing goes to the card, and the two pictures must be the same bytes. On every row both paths must give the same warnings.

**288 of 288 checks pass.**

The worst comparison is "the reference shot with three Blooms frame 0, Full": largest difference 1 of 255, pixels differing: 2640. Its pictures are in `verification/B-47 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

| Case | Blooms left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---|---|
| fx_bloom_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_bloom_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_bloom_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_bloom_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_bloom_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_bloom_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_bloom_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_bloom_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_bloom_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_bloom_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_bloom_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_004 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_bloom_004 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fx_bloom_004 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fx_bloom_004 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fx_bloom_004 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fx_bloom_004 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_bloom_004 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_bloom_004 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fx_bloom_004 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fx_bloom_004 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fx_bloom_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_017 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_bloom_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_017 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_bloom_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_bloom_019 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_bloom_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_028 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_028 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_028 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_028 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_028 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_028 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_028 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_028 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_028 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fx_bloom_028 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| the reference shot with three Blooms frame 0, Full | 3 | 1 | 2640 | none | PASS |
| the reference shot with three Blooms frame 100, Full | 3 | 1 | 956 | none | PASS |
| the reference shot with three Blooms frame 239, Full | 3 | 1 | 844 | none | PASS |
| the reference shot with three Blooms frame 0, Draft | 3 | 1 | 140 | none | PASS |
| the reference shot with three Blooms frame 100, Draft | 3 | 1 | 54 | none | PASS |
| the reference shot with three Blooms frame 239, Draft | 3 | 1 | 68 | none | PASS |
| the reference shot with three Blooms frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with three Blooms frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
