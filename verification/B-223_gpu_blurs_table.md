# B-223: five blurs on the GPU against the CPU

Written by `tests/b223_gpu_blurs.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Fast Box Blur, Channel Blur, Compound Blur, Selective Color Blur and CC Vector Blur, each the CPU's rule on the card (D-342). Compound Blur and CC Vector Blur read another layer as a map, which the card is handed as a picture of its own. Selective Color Blur chooses pixels by an 8-bit rounding, so, like an HSV Key, it only begins a run.

The cases: every fixture naming one of the five (113 files), at every frame it has; and the reference shot with each effect on its first three layers (the second after a Drop Shadow), 17 settings, at frames 0, 100 and 239. Each at Full and Draft.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU. **The rule: no channel of any pixel more than 1 level of 255 apart** (ADR-006, D-100), the same warnings on both, and on a reference shot row the effect in fact on the card, at Full the first layer's blur. 8 bpc and After Effects 32 bpc compositions give the card no effect (D-330, D-333); a few frames the card refuses whole (Float depth, an adjustment layer): those must be the CPU's picture exactly, with the card's message `GPU_PREVIEW_ON_CPU` its only extra warning.

**1266 of 1266 checks pass.**

The CPU drawing each plan made for the card draws the plan made for the CPU byte for byte in 34 of 34.

The worst comparison is "the reference shot with CC Vector Blur (2) frame 0, Full": largest difference 1 of 255, pixels differing: 3783. Its pictures are in `verification/B-223 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## Each effect

| Effect | Frames compared | Frames with an effect on the card | Frames the card refused | Largest difference (of 255) | Pass |
|---|---:|---:|---:|---:|---|
| Fast Box Blur | 178 | 78 | 40 | 1 | 178 of 178 |
| Channel Blur | 164 | 112 | 0 | 1 | 164 of 164 |
| Compound Blur | 282 | 170 | 0 | 1 | 282 of 282 |
| Selective Color Blur | 302 | 210 | 0 | 1 | 302 of 302 |
| CC Vector Blur | 306 | 204 | 0 | 1 | 306 of 306 |

## Every frame

Effects left to the card on the first three layers.

| Case | Left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---|---:|---:|---|---|
| ae_32bpc/fx_ae32_001 frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001 frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001 frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001 frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001 frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001 frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001 frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001 frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001 frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001 frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004 frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004 frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004 frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004 frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004 frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004 frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004 frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004 frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004 frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004 frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005 frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005 frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005 frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005 frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005 frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005 frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005 frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005 frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005 frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005 frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| channel_blur/fx_chblur_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_008 frame 0, Full | 0 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_008 frame 1, Full | 0 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_008 frame 2, Full | 0 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_008 frame 3, Full | 0 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_008 frame 4, Full | 0 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_008 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_008 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_008 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_008 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_008 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| channel_blur/fx_chblur_011 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_011 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_011 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_011 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_011 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_011 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_011 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_011 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_011 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_011 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_012 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_012 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_012 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_012 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_012 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_012 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_012 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_012 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_012 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_012 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_013 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_013 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_013 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_013 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_013 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| channel_blur/fx_chblur_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/chain_adjustment frame 0, Full | 0 / 1 | 0 | 0 | none | PASS |
| compound_blur/chain_adjustment frame 1, Full | 0 / 1 | 0 | 0 | none | PASS |
| compound_blur/chain_adjustment frame 2, Full | 0 / 1 | 0 | 0 | none | PASS |
| compound_blur/chain_adjustment frame 3, Full | 0 / 1 | 0 | 0 | none | PASS |
| compound_blur/chain_adjustment frame 4, Full | 0 / 1 | 0 | 0 | none | PASS |
| compound_blur/chain_adjustment frame 0, Draft | 0 / 1 | 0 | 0 | none | PASS |
| compound_blur/chain_adjustment frame 1, Draft | 0 / 1 | 0 | 0 | none | PASS |
| compound_blur/chain_adjustment frame 2, Draft | 0 / 1 | 0 | 0 | none | PASS |
| compound_blur/chain_adjustment frame 3, Draft | 0 / 1 | 0 | 0 | none | PASS |
| compound_blur/chain_adjustment frame 4, Draft | 0 / 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_001 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_001 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_001 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_001 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_001 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_009 frame 0, Full | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_009 frame 1, Full | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_009 frame 2, Full | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_009 frame 3, Full | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_009 frame 4, Full | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_009 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_009 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_009 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_009 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_009 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_014 frame 0, Full | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_014 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_015 frame 0, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| compound_blur/fx_cblur_015 frame 1, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| compound_blur/fx_cblur_015 frame 2, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| compound_blur/fx_cblur_015 frame 3, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| compound_blur/fx_cblur_015 frame 4, Full | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| compound_blur/fx_cblur_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| compound_blur/fx_cblur_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| compound_blur/fx_cblur_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| compound_blur/fx_cblur_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| compound_blur/fx_cblur_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| compound_blur/fx_cblur_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_019 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_019 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_019 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_019 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_019 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_019 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_019 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_019 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_019 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_019 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| compound_blur/fx_cblur_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| compound_blur/fx_cblur_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| eight_bpc/fx_8bpc_001 frame 0, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001 frame 1, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001 frame 2, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001 frame 3, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001 frame 4, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_004 frame 0, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_004 frame 1, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_004 frame 2, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_004 frame 3, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_004 frame 4, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_004 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_004 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_004 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_004 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_004 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_006 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_006 frame 1, Full | 0 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_006 frame 2, Full | 0 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_006 frame 3, Full | 0 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_006 frame 4, Full | 0 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_006 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_006 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_006 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_006 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_006 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_007 frame 0, Full | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 1, Full | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 2, Full | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 3, Full | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 4, Full | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 0, Draft | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 1, Draft | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 2, Draft | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 3, Draft | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 4, Draft | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fast_box_blur/fx_fastbox_009 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_009 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_009 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_009 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_009 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_009 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_009 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_009 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_009 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_009 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_010 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_010 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_010 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_010 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_010 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_010 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_010 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_010 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_010 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_010 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_011 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_011 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_011 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_011 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_011 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_011 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_011 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_011 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_011 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_011 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_009 frame 0, Full | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_009 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_014 frame 0, Full | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_014 frame 1, Full | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_014 frame 2, Full | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_014 frame 3, Full | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_014 frame 4, Full | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_014 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_014 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_014 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_014 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_014 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| selblur/fx_selblur_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_025 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_025 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_025 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_025 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_025 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_025 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_025 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_025 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_025 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_025 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_026 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_026 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_026 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_026 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_026 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_026 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_026 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_026 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_026 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_026 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_027 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_027 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_027 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_027 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_027 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_027 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_027 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_027 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_027 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_027 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_028 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_028 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_028 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_028 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_028 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_028 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_028 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_028 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_028 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_028 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_029 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_029 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_029 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_029 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_029 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_029 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_029 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_029 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_029 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_029 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_030 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_030 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_030 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_030 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_030 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_030 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_030 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_030 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_030 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_030 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_031 frame 0, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_031 frame 1, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_031 frame 2, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_031 frame 3, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_031 frame 4, Full | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_031 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_031 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_031 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_031 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_031 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| selblur/fx_selblur_032 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_032 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_032 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_032 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_032 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_032 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_032 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_032 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_032 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_032 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_033 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_033 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_033 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_033 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_033 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_033 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_033 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_033 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_033 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| selblur/fx_selblur_033 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_016 frame 0, Full | 0 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_016 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_017 frame 0, Full | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| vector_blur/fx_vblur_017 frame 1, Full | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| vector_blur/fx_vblur_017 frame 2, Full | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| vector_blur/fx_vblur_017 frame 3, Full | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| vector_blur/fx_vblur_017 frame 4, Full | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| vector_blur/fx_vblur_017 frame 0, Draft | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| vector_blur/fx_vblur_017 frame 1, Draft | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| vector_blur/fx_vblur_017 frame 2, Draft | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| vector_blur/fx_vblur_017 frame 3, Draft | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| vector_blur/fx_vblur_017 frame 4, Draft | 1 | 0 | 0 | EFFECT_LAYER_MISSING, on both | PASS |
| vector_blur/fx_vblur_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| vector_blur/fx_vblur_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_027 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_027 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_027 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_027 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| vector_blur/fx_vblur_027 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| the reference shot with Fast Box Blur (1) frame 0, Full | 1 / 2 / 1 | 1 | 895 | none | PASS |
| the reference shot with Fast Box Blur (1) frame 100, Full | 1 / 2 / 1 | 1 | 1050 | none | PASS |
| the reference shot with Fast Box Blur (1) frame 239, Full | 1 / 2 / 1 | 1 | 819 | none | PASS |
| the reference shot with Fast Box Blur (1) frame 0, Draft | 1 / 2 / 1 | 1 | 60 | none | PASS |
| the reference shot with Fast Box Blur (1) frame 100, Draft | 1 / 2 / 1 | 1 | 73 | none | PASS |
| the reference shot with Fast Box Blur (1) frame 239, Draft | 1 / 2 / 1 | 1 | 48 | none | PASS |
| the reference shot with Fast Box Blur (2) frame 0, Full | 1 / 2 / 1 | 1 | 915 | none | PASS |
| the reference shot with Fast Box Blur (2) frame 100, Full | 1 / 2 / 1 | 1 | 1063 | none | PASS |
| the reference shot with Fast Box Blur (2) frame 239, Full | 1 / 2 / 1 | 1 | 813 | none | PASS |
| the reference shot with Fast Box Blur (2) frame 0, Draft | 1 / 2 / 1 | 1 | 76 | none | PASS |
| the reference shot with Fast Box Blur (2) frame 100, Draft | 1 / 2 / 1 | 1 | 61 | none | PASS |
| the reference shot with Fast Box Blur (2) frame 239, Draft | 1 / 2 / 1 | 1 | 58 | none | PASS |
| the reference shot with Fast Box Blur (3) frame 0, Full | 1 / 2 / 1 | 1 | 1165 | none | PASS |
| the reference shot with Fast Box Blur (3) frame 100, Full | 1 / 2 / 1 | 1 | 1021 | none | PASS |
| the reference shot with Fast Box Blur (3) frame 239, Full | 1 / 2 / 1 | 1 | 854 | none | PASS |
| the reference shot with Fast Box Blur (3) frame 0, Draft | 1 / 2 / 1 | 1 | 76 | none | PASS |
| the reference shot with Fast Box Blur (3) frame 100, Draft | 1 / 2 / 1 | 1 | 77 | none | PASS |
| the reference shot with Fast Box Blur (3) frame 239, Draft | 1 / 2 / 1 | 1 | 64 | none | PASS |
| the reference shot with Channel Blur (1) frame 0, Full | 1 / 2 / 1 | 1 | 2802 | none | PASS |
| the reference shot with Channel Blur (1) frame 100, Full | 1 / 2 / 1 | 1 | 348 | none | PASS |
| the reference shot with Channel Blur (1) frame 239, Full | 1 / 2 / 1 | 1 | 849 | none | PASS |
| the reference shot with Channel Blur (1) frame 0, Draft | 1 / 2 / 1 | 1 | 152 | none | PASS |
| the reference shot with Channel Blur (1) frame 100, Draft | 1 / 2 / 1 | 1 | 75 | none | PASS |
| the reference shot with Channel Blur (1) frame 239, Draft | 1 / 2 / 1 | 1 | 81 | none | PASS |
| the reference shot with Channel Blur (2) frame 0, Full | 1 / 2 / 1 | 1 | 922 | none | PASS |
| the reference shot with Channel Blur (2) frame 100, Full | 1 / 2 / 1 | 1 | 899 | none | PASS |
| the reference shot with Channel Blur (2) frame 239, Full | 1 / 2 / 1 | 1 | 728 | none | PASS |
| the reference shot with Channel Blur (2) frame 0, Draft | 1 / 2 / 1 | 1 | 49 | none | PASS |
| the reference shot with Channel Blur (2) frame 100, Draft | 1 / 2 / 1 | 1 | 60 | none | PASS |
| the reference shot with Channel Blur (2) frame 239, Draft | 1 / 2 / 1 | 1 | 42 | none | PASS |
| the reference shot with Channel Blur (3) frame 0, Full | 1 / 2 / 1 | 1 | 926 | none | PASS |
| the reference shot with Channel Blur (3) frame 100, Full | 1 / 2 / 1 | 1 | 1027 | none | PASS |
| the reference shot with Channel Blur (3) frame 239, Full | 1 / 2 / 1 | 1 | 828 | none | PASS |
| the reference shot with Channel Blur (3) frame 0, Draft | 1 / 2 / 1 | 1 | 56 | none | PASS |
| the reference shot with Channel Blur (3) frame 100, Draft | 1 / 2 / 1 | 1 | 80 | none | PASS |
| the reference shot with Channel Blur (3) frame 239, Draft | 1 / 2 / 1 | 1 | 50 | none | PASS |
| the reference shot with Channel Blur (4) frame 0, Full | 1 / 2 / 1 | 1 | 884 | none | PASS |
| the reference shot with Channel Blur (4) frame 100, Full | 1 / 2 / 1 | 1 | 1048 | none | PASS |
| the reference shot with Channel Blur (4) frame 239, Full | 1 / 2 / 1 | 1 | 823 | none | PASS |
| the reference shot with Channel Blur (4) frame 0, Draft | 1 / 2 / 1 | 1 | 62 | none | PASS |
| the reference shot with Channel Blur (4) frame 100, Draft | 1 / 2 / 1 | 1 | 60 | none | PASS |
| the reference shot with Channel Blur (4) frame 239, Draft | 1 / 2 / 1 | 1 | 53 | none | PASS |
| the reference shot with Compound Blur (1) frame 0, Full | 1 / 2 / 1 | 1 | 839 | none | PASS |
| the reference shot with Compound Blur (1) frame 100, Full | 1 / 2 / 1 | 1 | 924 | none | PASS |
| the reference shot with Compound Blur (1) frame 239, Full | 1 / 2 / 1 | 1 | 755 | none | PASS |
| the reference shot with Compound Blur (1) frame 0, Draft | 1 / 2 / 1 | 1 | 51 | none | PASS |
| the reference shot with Compound Blur (1) frame 100, Draft | 1 / 2 / 1 | 1 | 64 | none | PASS |
| the reference shot with Compound Blur (1) frame 239, Draft | 1 / 2 / 1 | 1 | 49 | none | PASS |
| the reference shot with Compound Blur (2) frame 0, Full | 1 / 2 / 1 | 1 | 941 | none | PASS |
| the reference shot with Compound Blur (2) frame 100, Full | 1 / 2 / 1 | 1 | 926 | none | PASS |
| the reference shot with Compound Blur (2) frame 239, Full | 1 / 2 / 1 | 1 | 694 | none | PASS |
| the reference shot with Compound Blur (2) frame 0, Draft | 1 / 2 / 1 | 1 | 66 | none | PASS |
| the reference shot with Compound Blur (2) frame 100, Draft | 1 / 2 / 1 | 1 | 58 | none | PASS |
| the reference shot with Compound Blur (2) frame 239, Draft | 1 / 2 / 1 | 1 | 49 | none | PASS |
| the reference shot with Selective Color Blur (1) frame 0, Full | 1 / 1 / 1 | 1 | 3782 | none | PASS |
| the reference shot with Selective Color Blur (1) frame 100, Full | 1 / 1 / 1 | 1 | 1417 | none | PASS |
| the reference shot with Selective Color Blur (1) frame 239, Full | 1 / 1 / 1 | 1 | 1773 | none | PASS |
| the reference shot with Selective Color Blur (1) frame 0, Draft | 1 / 1 / 1 | 1 | 128 | none | PASS |
| the reference shot with Selective Color Blur (1) frame 100, Draft | 1 / 1 / 1 | 1 | 32 | none | PASS |
| the reference shot with Selective Color Blur (1) frame 239, Draft | 1 / 1 / 1 | 1 | 47 | none | PASS |
| the reference shot with Selective Color Blur (2) frame 0, Full | 1 / 1 / 1 | 1 | 3782 | none | PASS |
| the reference shot with Selective Color Blur (2) frame 100, Full | 1 / 1 / 1 | 1 | 1417 | none | PASS |
| the reference shot with Selective Color Blur (2) frame 239, Full | 1 / 1 / 1 | 1 | 1772 | none | PASS |
| the reference shot with Selective Color Blur (2) frame 0, Draft | 1 / 1 / 1 | 1 | 128 | none | PASS |
| the reference shot with Selective Color Blur (2) frame 100, Draft | 1 / 1 / 1 | 1 | 32 | none | PASS |
| the reference shot with Selective Color Blur (2) frame 239, Draft | 1 / 1 / 1 | 1 | 47 | none | PASS |
| the reference shot with CC Vector Blur (1) frame 0, Full | 1 / 2 / 1 | 1 | 2147 | none | PASS |
| the reference shot with CC Vector Blur (1) frame 100, Full | 1 / 2 / 1 | 1 | 985 | none | PASS |
| the reference shot with CC Vector Blur (1) frame 239, Full | 1 / 2 / 1 | 1 | 811 | none | PASS |
| the reference shot with CC Vector Blur (1) frame 0, Draft | 1 / 2 / 1 | 1 | 107 | none | PASS |
| the reference shot with CC Vector Blur (1) frame 100, Draft | 1 / 2 / 1 | 1 | 69 | none | PASS |
| the reference shot with CC Vector Blur (1) frame 239, Draft | 1 / 2 / 1 | 1 | 54 | none | PASS |
| the reference shot with CC Vector Blur (2) frame 0, Full | 1 / 2 / 1 | 1 | 3783 | none | PASS |
| the reference shot with CC Vector Blur (2) frame 100, Full | 1 / 2 / 1 | 1 | 1419 | none | PASS |
| the reference shot with CC Vector Blur (2) frame 239, Full | 1 / 2 / 1 | 1 | 1773 | none | PASS |
| the reference shot with CC Vector Blur (2) frame 0, Draft | 1 / 2 / 1 | 1 | 128 | none | PASS |
| the reference shot with CC Vector Blur (2) frame 100, Draft | 1 / 2 / 1 | 1 | 32 | none | PASS |
| the reference shot with CC Vector Blur (2) frame 239, Draft | 1 / 2 / 1 | 1 | 47 | none | PASS |
| the reference shot with CC Vector Blur (3) frame 0, Full | 1 / 2 / 1 | 1 | 1258 | none | PASS |
| the reference shot with CC Vector Blur (3) frame 100, Full | 1 / 2 / 1 | 1 | 1019 | none | PASS |
| the reference shot with CC Vector Blur (3) frame 239, Full | 1 / 2 / 1 | 1 | 871 | none | PASS |
| the reference shot with CC Vector Blur (3) frame 0, Draft | 1 / 2 / 1 | 1 | 68 | none | PASS |
| the reference shot with CC Vector Blur (3) frame 100, Draft | 1 / 2 / 1 | 1 | 65 | none | PASS |
| the reference shot with CC Vector Blur (3) frame 239, Draft | 1 / 2 / 1 | 1 | 47 | none | PASS |
| the reference shot with CC Vector Blur (4) frame 0, Full | 1 / 2 / 1 | 1 | 1195 | none | PASS |
| the reference shot with CC Vector Blur (4) frame 100, Full | 1 / 2 / 1 | 1 | 961 | none | PASS |
| the reference shot with CC Vector Blur (4) frame 239, Full | 1 / 2 / 1 | 1 | 827 | none | PASS |
| the reference shot with CC Vector Blur (4) frame 0, Draft | 1 / 2 / 1 | 1 | 58 | none | PASS |
| the reference shot with CC Vector Blur (4) frame 100, Draft | 1 / 2 / 1 | 1 | 54 | none | PASS |
| the reference shot with CC Vector Blur (4) frame 239, Draft | 1 / 2 / 1 | 1 | 48 | none | PASS |
| the reference shot with CC Vector Blur (5) frame 0, Full | 1 / 2 / 1 | 1 | 1526 | none | PASS |
| the reference shot with CC Vector Blur (5) frame 100, Full | 1 / 2 / 1 | 1 | 879 | none | PASS |
| the reference shot with CC Vector Blur (5) frame 239, Full | 1 / 2 / 1 | 1 | 732 | none | PASS |
| the reference shot with CC Vector Blur (5) frame 0, Draft | 1 / 2 / 1 | 1 | 77 | none | PASS |
| the reference shot with CC Vector Blur (5) frame 100, Draft | 1 / 2 / 1 | 1 | 65 | none | PASS |
| the reference shot with CC Vector Blur (5) frame 239, Draft | 1 / 2 / 1 | 1 | 41 | none | PASS |
| the reference shot with CC Vector Blur (6) frame 0, Full | 1 / 2 / 1 | 1 | 3459 | none | PASS |
| the reference shot with CC Vector Blur (6) frame 100, Full | 1 / 2 / 1 | 1 | 1352 | none | PASS |
| the reference shot with CC Vector Blur (6) frame 239, Full | 1 / 2 / 1 | 1 | 1552 | none | PASS |
| the reference shot with CC Vector Blur (6) frame 0, Draft | 1 / 2 / 1 | 1 | 112 | none | PASS |
| the reference shot with CC Vector Blur (6) frame 100, Draft | 1 / 2 / 1 | 1 | 33 | none | PASS |
| the reference shot with CC Vector Blur (6) frame 239, Draft | 1 / 2 / 1 | 1 | 51 | none | PASS |
