# B-224: Displacement Map and CC Glass on the GPU against the CPU

Written by `tests/b224_gpu_maps.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Displacement Map and CC Glass, each the CPU's rule on the card (D-343). Both read another layer as a map, which the card is handed as a picture of its own.

The cases: every fixture naming one of the two (88 files), at every frame it has; and the reference shot with each effect on its first three layers (the second after a Drop Shadow), 11 settings, at frames 0, 100 and 239. Each at Full and Draft.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU. **The rule: no channel of any pixel more than 1 level of 255 apart** (ADR-006, D-100), the same warnings on both, and on a reference shot row the effect in fact on the card, at Full the first layer's. 8 bpc and After Effects 32 bpc compositions give the card no effect (D-330, D-333); a few frames the card refuses whole (Float depth, an adjustment layer): those must be the CPU's picture exactly, with the card's message `GPU_PREVIEW_ON_CPU` its only extra warning.

**968 of 968 checks pass.**

The CPU drawing each plan made for the card draws the plan made for the CPU byte for byte in 22 of 22.

The worst comparison is "the reference shot with Displacement Map (5) frame 0, Full": largest difference 1 of 255, pixels differing: 3907. Its pictures are in `verification/B-224 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## Each effect

| Effect | Frames compared | Frames with an effect on the card | Frames the card refused | Largest difference (of 255) | Pass |
|---|---:|---:|---:|---:|---|
| Displacement Map | 530 | 350 | 0 | 1 | 530 of 530 |
| CC Glass | 416 | 326 | 0 | 1 | 416 of 416 |

## Every frame

Effects left to the card on the first three layers.

| Case | Left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---|---:|---:|---|---|
| displacement_map/fx_dmap_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_001 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_001 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_001 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_001 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_001 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_019 frame 0, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| displacement_map/fx_dmap_019 frame 1, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| displacement_map/fx_dmap_019 frame 2, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| displacement_map/fx_dmap_019 frame 3, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| displacement_map/fx_dmap_019 frame 4, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| displacement_map/fx_dmap_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| displacement_map/fx_dmap_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| displacement_map/fx_dmap_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| displacement_map/fx_dmap_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| displacement_map/fx_dmap_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| displacement_map/fx_dmap_020 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_020 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_020 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_020 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_020 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_020 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_021 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_021 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_021 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_021 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_021 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_021 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_021 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_021 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_021 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_021 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_022 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_022 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_022 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_022 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_022 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_022 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_022 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_022 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_022 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_022 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_023 frame 0, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_023 frame 1, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_023 frame 2, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_023 frame 3, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_023 frame 4, Full | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_023 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_023 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_023 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_023 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_023 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| displacement_map/fx_dmap_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_028 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_028 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_028 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_028 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_028 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_028 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_028 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_028 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_028 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_028 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_029 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_029 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_029 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_029 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_029 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_029 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_029 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_029 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_029 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_029 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_030 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_030 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_030 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_030 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_030 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_030 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_030 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_030 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_030 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_030 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_031 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_031 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_031 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_031 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_031 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_031 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_031 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_031 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_031 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| displacement_map/fx_dmap_031 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_017 frame 0, Full | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| glass/fx_glass_017 frame 1, Full | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| glass/fx_glass_017 frame 2, Full | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| glass/fx_glass_017 frame 3, Full | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| glass/fx_glass_017 frame 4, Full | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| glass/fx_glass_017 frame 0, Draft | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| glass/fx_glass_017 frame 1, Draft | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| glass/fx_glass_017 frame 2, Draft | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| glass/fx_glass_017 frame 3, Draft | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| glass/fx_glass_017 frame 4, Draft | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| glass/fx_glass_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_019 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_020 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_020 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_020 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_020 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_020 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_020 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_020 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_020 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_020 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_020 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_028 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_028 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_028 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_028 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_028 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_028 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_028 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_028 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_028 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_028 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_029 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_029 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_029 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_029 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_029 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_029 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_029 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_029 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_029 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_029 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_030 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_030 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_030 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_030 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_030 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_030 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_030 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_030 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_030 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_030 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_031 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_031 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_031 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_031 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_031 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_031 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_031 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_031 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_031 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_031 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_032 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_032 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_032 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_032 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_032 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_032 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_032 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_032 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_032 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_032 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_033 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_033 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_033 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_033 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_033 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_033 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_033 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_033 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_033 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_033 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_034 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_034 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_034 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_034 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_034 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_034 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_034 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_034 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_034 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_034 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_035 frame 0, Full | 0 | 0 | 0 | none | PASS |
| glass/fx_glass_035 frame 1, Full | 0 | 0 | 0 | none | PASS |
| glass/fx_glass_035 frame 2, Full | 0 | 0 | 0 | none | PASS |
| glass/fx_glass_035 frame 3, Full | 0 | 0 | 0 | none | PASS |
| glass/fx_glass_035 frame 4, Full | 0 | 0 | 0 | none | PASS |
| glass/fx_glass_035 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| glass/fx_glass_035 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| glass/fx_glass_035 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| glass/fx_glass_035 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| glass/fx_glass_035 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| glass/fx_glass_036 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_036 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_036 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_036 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_036 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_036 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_036 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_036 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_036 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_036 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_037 frame 0, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_037 frame 1, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_037 frame 2, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_037 frame 3, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_037 frame 4, Full | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_037 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_037 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_037 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_037 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_037 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| glass/fx_glass_038 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_038 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_038 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_038 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_038 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_038 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_038 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_038 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_038 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| glass/fx_glass_038 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_010 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_010 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_010 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_010 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_010 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_010 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_010 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_010 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_010 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_010 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_014 frame 0, Full | 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_014 frame 1, Full | 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_014 frame 2, Full | 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_014 frame 3, Full | 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_014 frame 4, Full | 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_014 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_014 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_014 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_014 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_014 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| map_chromatic/fx_mapchroma_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| map_chromatic/fx_mapchroma_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| the reference shot with Displacement Map (1) frame 0, Full | 1 / 2 / 1 | 1 | 1963 | none | PASS |
| the reference shot with Displacement Map (1) frame 100, Full | 1 / 2 / 1 | 1 | 803 | none | PASS |
| the reference shot with Displacement Map (1) frame 239, Full | 1 / 2 / 1 | 1 | 832 | none | PASS |
| the reference shot with Displacement Map (1) frame 0, Draft | 1 / 2 / 1 | 1 | 70 | none | PASS |
| the reference shot with Displacement Map (1) frame 100, Draft | 1 / 2 / 1 | 1 | 51 | none | PASS |
| the reference shot with Displacement Map (1) frame 239, Draft | 1 / 2 / 1 | 1 | 48 | none | PASS |
| the reference shot with Displacement Map (2) frame 0, Full | 1 / 2 / 1 | 1 | 1706 | none | PASS |
| the reference shot with Displacement Map (2) frame 100, Full | 1 / 2 / 1 | 1 | 695 | none | PASS |
| the reference shot with Displacement Map (2) frame 239, Full | 1 / 2 / 1 | 1 | 697 | none | PASS |
| the reference shot with Displacement Map (2) frame 0, Draft | 1 / 2 / 1 | 1 | 74 | none | PASS |
| the reference shot with Displacement Map (2) frame 100, Draft | 1 / 2 / 1 | 1 | 62 | none | PASS |
| the reference shot with Displacement Map (2) frame 239, Draft | 1 / 2 / 1 | 1 | 49 | none | PASS |
| the reference shot with Displacement Map (3) frame 0, Full | 1 / 2 / 1 | 1 | 1762 | none | PASS |
| the reference shot with Displacement Map (3) frame 100, Full | 1 / 2 / 1 | 1 | 836 | none | PASS |
| the reference shot with Displacement Map (3) frame 239, Full | 1 / 2 / 1 | 1 | 769 | none | PASS |
| the reference shot with Displacement Map (3) frame 0, Draft | 1 / 2 / 1 | 1 | 64 | none | PASS |
| the reference shot with Displacement Map (3) frame 100, Draft | 1 / 2 / 1 | 1 | 59 | none | PASS |
| the reference shot with Displacement Map (3) frame 239, Draft | 1 / 2 / 1 | 1 | 42 | none | PASS |
| the reference shot with Displacement Map (4) frame 0, Full | 1 / 2 / 1 | 1 | 2605 | none | PASS |
| the reference shot with Displacement Map (4) frame 100, Full | 1 / 2 / 1 | 1 | 892 | none | PASS |
| the reference shot with Displacement Map (4) frame 239, Full | 1 / 2 / 1 | 1 | 1055 | none | PASS |
| the reference shot with Displacement Map (4) frame 0, Draft | 1 / 2 / 1 | 1 | 75 | none | PASS |
| the reference shot with Displacement Map (4) frame 100, Draft | 1 / 2 / 1 | 1 | 38 | none | PASS |
| the reference shot with Displacement Map (4) frame 239, Draft | 1 / 2 / 1 | 1 | 41 | none | PASS |
| the reference shot with Displacement Map (5) frame 0, Full | 1 / 2 / 1 | 1 | 3907 | none | PASS |
| the reference shot with Displacement Map (5) frame 100, Full | 1 / 2 / 1 | 1 | 1416 | none | PASS |
| the reference shot with Displacement Map (5) frame 239, Full | 1 / 2 / 1 | 1 | 1830 | none | PASS |
| the reference shot with Displacement Map (5) frame 0, Draft | 1 / 2 / 1 | 1 | 92 | none | PASS |
| the reference shot with Displacement Map (5) frame 100, Draft | 1 / 2 / 1 | 1 | 41 | none | PASS |
| the reference shot with Displacement Map (5) frame 239, Draft | 1 / 2 / 1 | 1 | 47 | none | PASS |
| the reference shot with CC Glass (1) frame 0, Full | 1 / 2 / 1 | 1 | 995 | none | PASS |
| the reference shot with CC Glass (1) frame 100, Full | 1 / 2 / 1 | 1 | 969 | none | PASS |
| the reference shot with CC Glass (1) frame 239, Full | 1 / 2 / 1 | 1 | 771 | none | PASS |
| the reference shot with CC Glass (1) frame 0, Draft | 1 / 2 / 1 | 1 | 64 | none | PASS |
| the reference shot with CC Glass (1) frame 100, Draft | 1 / 2 / 1 | 1 | 59 | none | PASS |
| the reference shot with CC Glass (1) frame 239, Draft | 1 / 2 / 1 | 1 | 46 | none | PASS |
| the reference shot with CC Glass (2) frame 0, Full | 1 / 2 / 1 | 1 | 3783 | none | PASS |
| the reference shot with CC Glass (2) frame 100, Full | 1 / 2 / 1 | 1 | 1419 | none | PASS |
| the reference shot with CC Glass (2) frame 239, Full | 1 / 2 / 1 | 1 | 1773 | none | PASS |
| the reference shot with CC Glass (2) frame 0, Draft | 1 / 2 / 1 | 1 | 128 | none | PASS |
| the reference shot with CC Glass (2) frame 100, Draft | 1 / 2 / 1 | 1 | 32 | none | PASS |
| the reference shot with CC Glass (2) frame 239, Draft | 1 / 2 / 1 | 1 | 47 | none | PASS |
| the reference shot with CC Glass (3) frame 0, Full | 1 / 2 / 1 | 1 | 1983 | none | PASS |
| the reference shot with CC Glass (3) frame 100, Full | 1 / 2 / 1 | 1 | 1002 | none | PASS |
| the reference shot with CC Glass (3) frame 239, Full | 1 / 2 / 1 | 1 | 909 | none | PASS |
| the reference shot with CC Glass (3) frame 0, Draft | 1 / 2 / 1 | 1 | 81 | none | PASS |
| the reference shot with CC Glass (3) frame 100, Draft | 1 / 2 / 1 | 1 | 58 | none | PASS |
| the reference shot with CC Glass (3) frame 239, Draft | 1 / 2 / 1 | 1 | 45 | none | PASS |
| the reference shot with CC Glass (4) frame 0, Full | 1 / 2 / 1 | 1 | 3411 | none | PASS |
| the reference shot with CC Glass (4) frame 100, Full | 1 / 2 / 1 | 1 | 1334 | none | PASS |
| the reference shot with CC Glass (4) frame 239, Full | 1 / 2 / 1 | 1 | 1546 | none | PASS |
| the reference shot with CC Glass (4) frame 0, Draft | 1 / 2 / 1 | 1 | 114 | none | PASS |
| the reference shot with CC Glass (4) frame 100, Draft | 1 / 2 / 1 | 1 | 42 | none | PASS |
| the reference shot with CC Glass (4) frame 239, Draft | 1 / 2 / 1 | 1 | 50 | none | PASS |
| the reference shot with CC Glass (5) frame 0, Full | 1 / 2 / 1 | 1 | 2973 | none | PASS |
| the reference shot with CC Glass (5) frame 100, Full | 1 / 2 / 1 | 1 | 1252 | none | PASS |
| the reference shot with CC Glass (5) frame 239, Full | 1 / 2 / 1 | 1 | 1361 | none | PASS |
| the reference shot with CC Glass (5) frame 0, Draft | 1 / 2 / 1 | 1 | 112 | none | PASS |
| the reference shot with CC Glass (5) frame 100, Draft | 1 / 2 / 1 | 1 | 34 | none | PASS |
| the reference shot with CC Glass (5) frame 239, Draft | 1 / 2 / 1 | 1 | 46 | none | PASS |
| the reference shot with CC Glass (6) frame 0, Full | 1 / 2 / 1 | 1 | 3594 | none | PASS |
| the reference shot with CC Glass (6) frame 100, Full | 1 / 2 / 1 | 1 | 1368 | none | PASS |
| the reference shot with CC Glass (6) frame 239, Full | 1 / 2 / 1 | 1 | 1682 | none | PASS |
| the reference shot with CC Glass (6) frame 0, Draft | 1 / 2 / 1 | 1 | 125 | none | PASS |
| the reference shot with CC Glass (6) frame 100, Draft | 1 / 2 / 1 | 1 | 33 | none | PASS |
| the reference shot with CC Glass (6) frame 239, Draft | 1 / 2 / 1 | 1 | 54 | none | PASS |
