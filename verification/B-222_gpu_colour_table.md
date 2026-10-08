# B-222: ten colour effects on the GPU against the CPU

Written by `tests/b222_gpu_colour.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Exposure, Tint, Shift Channels, Solid Composite, Change to Color, Color Key, Select Color, Line Recolor, Colorama and Extract each read only the pixel they write, so the card draws them, and inside a run of colour effects drawn in one pass (B-172) (D-341). Color Key, Select Color and Line Recolor choose pixels by an 8-bit rounding, so, like an HSV Key, they only begin a run. A Colorama that adds another layer's phase stays the CPU's.

The cases: every fixture naming one of the ten (188 files), at every frame it has, alone and with each instance given neighbours (a Hue/Saturation before, except before the three that only begin a run, and a Levels after); and the reference shot with each effect on its first three layers (the second after a Drop Shadow), 23 settings, alone and with the same neighbours, at frames 0, 100 and 239. Each at Full and Draft.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU. **The rule: no channel of any pixel more than 1 level of 255 apart** (ADR-006, D-100), the same warnings on both, and on a reference shot row the effect in fact on the card, at Full the first layer's whole run. Most fixtures are 8 bpc or After Effects 32 bpc compositions, which the card is given no effect in (D-330, D-333); a few the card refuses whole (Float depth, an adjustment layer): those must be the CPU's picture exactly, with the card's message `GPU_PREVIEW_ON_CPU` its only extra warning.

**4086 of 4086 checks pass.**

The reference shot's runs drawn in one pass match the same runs drawn a pass each, byte for byte and in fewer passes, in 46 of 46 (frame 100, Full and Draft). The CPU drawing each plan made for the card draws the plan made for the CPU byte for byte in 92 of 92.

The worst comparison is "the reference shot with Shift Channels (2) frame 239, Full": largest difference 1 of 255, pixels differing: 40469. Its pictures are in `verification/B-222 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## Each effect

| Effect | Frames compared | Frames with an effect on the card | Frames the card refused | Largest difference (of 255) | Pass |
|---|---:|---:|---:|---:|---|
| Exposure | 1364 | 384 | 320 | 1 | 1364 of 1364 |
| Tint | 84 | 12 | 20 | 1 | 84 of 84 |
| Shift Channels | 36 | 36 | 0 | 1 | 36 of 36 |
| Solid Composite | 108 | 48 | 60 | 1 | 108 of 108 |
| Change to Color | 564 | 484 | 0 | 1 | 564 of 564 |
| Color Key | 484 | 384 | 0 | 1 | 484 of 484 |
| Select Color | 344 | 254 | 0 | 1 | 344 of 344 |
| Line Recolor | 372 | 292 | 0 | 1 | 372 of 372 |
| Colorama | 48 | 48 | 0 | 1 | 48 of 48 |
| Extract | 544 | 454 | 0 | 1 | 544 of 544 |

## Every frame

Effects left to the card on the first three layers.

| Case | Left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---|---:|---:|---|---|
| adjust/fx_adj_001 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_001 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_001 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_001 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_001 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_001 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_001, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_001, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_001, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_001, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_001, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_001, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_002 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_002 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_002 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_002 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_002 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_002 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_002, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_002, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_002, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_002, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_002, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_002, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_003 frame 0, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_003 frame 1, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_003 frame 2, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_003 frame 0, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_003 frame 1, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_003 frame 2, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_003, in a run frame 0, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_003, in a run frame 1, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_003, in a run frame 2, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_003, in a run frame 0, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_003, in a run frame 1, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_003, in a run frame 2, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_004 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_004 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_004 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_004 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_004 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_004 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_004, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_004, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_004, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_004, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_004, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_004, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_005 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_005 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_005 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_005 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_005 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_005 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_005, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_005, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_005, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_005, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_005, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_005, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_006 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_006 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_006 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_006 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_006 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_006 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_006, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_006, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_006, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_006, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_006, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_006, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_008 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_008 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_008 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_008 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_008 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_008 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_008, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_008, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_008, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_008, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_008, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_008, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_009 frame 0, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_009 frame 1, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_009 frame 2, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_009 frame 0, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_009 frame 1, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_009 frame 2, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_009, in a run frame 0, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_009, in a run frame 1, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_009, in a run frame 2, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_009, in a run frame 0, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_009, in a run frame 1, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_009, in a run frame 2, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_010 frame 0, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_010 frame 1, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_010 frame 2, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_010 frame 0, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_010 frame 1, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_010 frame 2, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_010, in a run frame 0, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_010, in a run frame 1, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_010, in a run frame 2, Full | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_010, in a run frame 0, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_010, in a run frame 1, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_010, in a run frame 2, Draft | 0 / 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_011 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_011 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_011 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_011 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_011 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_011 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_011, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_011, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_011, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_011, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_011, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_011, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_012 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_012 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_012 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_012 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_012 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_012 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_012, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_012, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_012, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_012, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_012, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_012, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_013 frame 0, Full | 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_013 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_013 frame 2, Full | 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_013 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_013 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_013 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_013, in a run frame 0, Full | 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_013, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_013, in a run frame 2, Full | 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_013, in a run frame 0, Draft | 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_013, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| adjust/fx_adj_013, in a run frame 2, Draft | 0 | 0 | 0 | none | PASS |
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
| ae_32bpc/fx_ae32_001, in a run frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001, in a run frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001, in a run frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001, in a run frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001, in a run frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001, in a run frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001, in a run frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001, in a run frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001, in a run frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_001, in a run frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002 frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002 frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002 frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002 frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002 frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002 frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002 frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002 frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002 frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002 frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002, in a run frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002, in a run frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002, in a run frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002, in a run frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002, in a run frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002, in a run frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002, in a run frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002, in a run frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002, in a run frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_002, in a run frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003 frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003 frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003 frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003 frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003 frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003 frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003 frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003 frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003 frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003 frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003, in a run frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003, in a run frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003, in a run frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003, in a run frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003, in a run frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003, in a run frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003, in a run frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003, in a run frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003, in a run frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_003, in a run frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
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
| ae_32bpc/fx_ae32_004, in a run frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004, in a run frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004, in a run frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004, in a run frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004, in a run frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004, in a run frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004, in a run frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004, in a run frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004, in a run frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_004, in a run frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
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
| ae_32bpc/fx_ae32_005, in a run frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005, in a run frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005, in a run frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005, in a run frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005, in a run frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005, in a run frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005, in a run frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005, in a run frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005, in a run frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| ae_32bpc/fx_ae32_005, in a run frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007 frame 0, Full | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007 frame 1, Full | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007 frame 2, Full | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007 frame 3, Full | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007 frame 4, Full | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007 frame 0, Draft | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007 frame 1, Draft | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007 frame 2, Draft | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007 frame 3, Draft | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007 frame 4, Draft | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007, in a run frame 0, Full | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007, in a run frame 1, Full | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007, in a run frame 2, Full | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007, in a run frame 3, Full | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007, in a run frame 4, Full | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007, in a run frame 0, Draft | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007, in a run frame 1, Draft | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007, in a run frame 2, Draft | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007, in a run frame 3, Draft | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| blurriness/fx_blurry_007, in a run frame 4, Draft | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| change_to_color/fx_ctc_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_001, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_002, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_003, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_004, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_005, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_006, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_007, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_008, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_009, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_010, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_011, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_012, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_013, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_014, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_015, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_016, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_017, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_018, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019 frame 0, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019 frame 1, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019 frame 2, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019 frame 3, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019 frame 4, Full | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_019, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| change_to_color/fx_ctc_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_020, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_021, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_022, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_023, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_024, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_025, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_026, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027 frame 0, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027 frame 1, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027 frame 2, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027 frame 3, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027 frame 4, Draft | 0 | 1 | 3 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| change_to_color/fx_ctc_027, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_001, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_001, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_001, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_001, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_001, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_001, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_001, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_001, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_001, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_001, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_002, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_002, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_002, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_002, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_002, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_002, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_002, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_002, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_002, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_002, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_003, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_003, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_003, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_003, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_003, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_003, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_003, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_003, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_003, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_003, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_004, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_004, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_004, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_004, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_004, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_004, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_004, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_004, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_004, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_004, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_005, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_005, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_005, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_005, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_005, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_005, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_005, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_005, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_005, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_005, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_006 frame 0, Full | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_006 frame 1, Full | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_006 frame 2, Full | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_006 frame 3, Full | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_006 frame 4, Full | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_006 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_006 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_006 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_006 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_006 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_006, in a run frame 0, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_006, in a run frame 1, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_006, in a run frame 2, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_006, in a run frame 3, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_006, in a run frame 4, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_006, in a run frame 0, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_006, in a run frame 1, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_006, in a run frame 2, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_006, in a run frame 3, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_006, in a run frame 4, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_007 frame 0, Full | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_007 frame 1, Full | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_007 frame 2, Full | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_007 frame 3, Full | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_007 frame 4, Full | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_007 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_007 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_007 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_007 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_007 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| color_key/fx_key_007, in a run frame 0, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_007, in a run frame 1, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_007, in a run frame 2, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_007, in a run frame 3, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_007, in a run frame 4, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_007, in a run frame 0, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_007, in a run frame 1, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_007, in a run frame 2, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_007, in a run frame 3, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_007, in a run frame 4, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_008, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_008, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_008, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_008, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_008, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_008, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_008, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_008, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_008, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_008, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_009, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_009, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_009, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_009, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_009, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_009, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_009, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_009, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_009, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_009, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_010, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_010, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_010, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_010, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_010, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_010, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_010, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_010, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_010, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_010, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_011, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_011, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_011, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_011, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_011, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_011, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_011, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_011, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_011, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_011, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_012, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_012, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_012, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_012, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_012, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_012, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_012, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_012, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_012, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_012, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_013, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_013, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_013, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_013, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_013, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_013, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_013, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_013, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_013, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_013, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_014, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_014, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_014, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_014, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_014, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_014, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_014, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_014, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_014, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_014, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| color_key/fx_key_015, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_015, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_015, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_015, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_015, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_015, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_015, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_015, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_015, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_015, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| color_key/fx_key_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_016, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_017, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_018, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_019, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_020, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_021, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_022, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| color_key/fx_key_023, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| echo/fx_echo_017 frame 0, Full | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017 frame 1, Full | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017 frame 2, Full | 0 | 1 | 3 | none | PASS |
| echo/fx_echo_017 frame 3, Full | 0 | 1 | 3 | none | PASS |
| echo/fx_echo_017 frame 4, Full | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017 frame 5, Full | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017 frame 6, Full | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017 frame 7, Full | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017 frame 5, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017 frame 6, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017 frame 7, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017, in a run frame 0, Full | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017, in a run frame 1, Full | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017, in a run frame 2, Full | 0 | 1 | 3 | none | PASS |
| echo/fx_echo_017, in a run frame 3, Full | 0 | 1 | 3 | none | PASS |
| echo/fx_echo_017, in a run frame 4, Full | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017, in a run frame 5, Full | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017, in a run frame 6, Full | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017, in a run frame 7, Full | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017, in a run frame 0, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017, in a run frame 1, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017, in a run frame 2, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017, in a run frame 3, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017, in a run frame 4, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017, in a run frame 5, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017, in a run frame 6, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_017, in a run frame 7, Draft | 0 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 0, Full | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 1, Full | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 2, Full | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 3, Full | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 4, Full | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 5, Full | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 6, Full | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 7, Full | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 5, Draft | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 6, Draft | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018 frame 7, Draft | 1 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 5, Full | 3 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 6, Full | 3 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 7, Full | 3 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 5, Draft | 3 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 6, Draft | 3 | 0 | 0 | none | PASS |
| echo/fx_echo_018, in a run frame 7, Draft | 3 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_009, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010, in a run frame 0, Full | 5 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010, in a run frame 1, Full | 5 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010, in a run frame 2, Full | 5 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010, in a run frame 3, Full | 5 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010, in a run frame 4, Full | 5 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010, in a run frame 0, Draft | 5 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010, in a run frame 1, Draft | 5 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010, in a run frame 2, Draft | 5 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010, in a run frame 3, Draft | 5 | 0 | 0 | none | PASS |
| effect_mix/fx_mix_010, in a run frame 4, Draft | 5 | 0 | 0 | none | PASS |
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
| eight_bpc/fx_8bpc_001, in a run frame 0, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001, in a run frame 1, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001, in a run frame 2, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001, in a run frame 3, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001, in a run frame 4, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001, in a run frame 0, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001, in a run frame 1, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001, in a run frame 2, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001, in a run frame 3, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_001, in a run frame 4, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002, in a run frame 0, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002, in a run frame 1, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002, in a run frame 2, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002, in a run frame 3, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002, in a run frame 4, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002, in a run frame 0, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002, in a run frame 1, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002, in a run frame 2, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002, in a run frame 3, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_002, in a run frame 4, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003, in a run frame 0, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003, in a run frame 1, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003, in a run frame 2, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003, in a run frame 3, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003, in a run frame 4, Full | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003, in a run frame 0, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003, in a run frame 1, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003, in a run frame 2, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003, in a run frame 3, Draft | 0 | 0 | 0 | none | PASS |
| eight_bpc/fx_8bpc_003, in a run frame 4, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_001, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_002, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_003, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_004, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_005 frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005 frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005 frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005 frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005 frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005 frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005 frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005 frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005 frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005 frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005, in a run frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005, in a run frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005, in a run frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005, in a run frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005, in a run frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005, in a run frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005, in a run frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005, in a run frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005, in a run frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_005, in a run frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006 frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006 frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006 frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006 frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006 frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006 frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006 frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006 frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006 frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006 frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006, in a run frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006, in a run frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006, in a run frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006, in a run frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006, in a run frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006, in a run frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006, in a run frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006, in a run frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006, in a run frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_006, in a run frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_007, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008 frame 0, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008 frame 1, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008 frame 2, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008 frame 3, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008 frame 4, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008, in a run frame 0, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008, in a run frame 1, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008, in a run frame 2, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008, in a run frame 3, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008, in a run frame 4, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008, in a run frame 0, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008, in a run frame 1, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008, in a run frame 2, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008, in a run frame 3, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_008, in a run frame 4, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009 frame 0, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009 frame 1, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009 frame 2, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009 frame 3, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009 frame 4, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009 frame 3, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009, in a run frame 0, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009, in a run frame 1, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009, in a run frame 2, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009, in a run frame 3, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009, in a run frame 4, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009, in a run frame 0, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009, in a run frame 1, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009, in a run frame 2, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009, in a run frame 3, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_009, in a run frame 4, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_010, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| exposure_ae/fx_expae_011 frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011 frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011 frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011 frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011 frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011 frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011 frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011 frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011 frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011 frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011, in a run frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011, in a run frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011, in a run frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011, in a run frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011, in a run frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011, in a run frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011, in a run frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011, in a run frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011, in a run frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_011, in a run frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| exposure_ae/fx_expae_012 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_012, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_013, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_014, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_015, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| exposure_ae/fx_expae_016, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_001, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_001, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_001, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_001, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_001, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_001, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_001, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_001, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_001, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_001, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_002, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_002, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_002, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_002, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_002, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_002, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_002, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_002, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_002, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_002, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_003, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_003, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_003, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_003, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_003, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_003, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_003, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_003, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_003, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_003, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_004, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_004, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_004, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_004, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_004, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_004, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_004, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_004, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_004, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_004, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_005, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_005, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_005, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_005, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_005, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_005, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_005, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_005, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_005, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_005, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_006, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_006, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_006, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_006, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_006, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_006, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_006, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_006, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_006, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_006, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_007, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_007, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_007, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_007, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_007, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_007, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_007, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_007, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_007, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_007, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_008, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_008, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_008, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_008, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_008, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_008, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_008, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_008, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_008, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_008, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_009, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_009, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_009, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_009, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_009, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_009, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_009, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_009, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_009, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_009, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_010, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_010, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_010, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_010, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_010, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_010, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_010, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_010, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_010, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_010, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_011, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_011, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_011, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_011, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_011, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_011, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_011, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_011, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_011, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_011, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_012 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_012 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_012 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_012 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_012 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_012 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_012 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_012 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_012 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_012 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_012, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_012, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_012, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_012, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_012, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_012, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_012, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_012, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_012, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_012, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_013 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_013 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_013 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_013 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_013 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_013 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_013 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_013 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_013 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_013 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_013, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_013, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_013, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_013, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_013, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_013, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_013, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_013, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_013, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_013, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_014 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_014 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_014 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_014 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_014 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_014 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_014 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_014 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_014 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_014 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_014, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_014, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_014, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_014, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_014, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_014, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_014, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_014, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_014, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_014, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_015 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_015 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_015 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_015 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_015 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_015 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_015 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_015 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_015 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_015 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_015, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_015, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_015, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_015, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_015, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_015, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_015, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_015, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_015, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_015, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_016 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_016 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_016 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_016 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_016 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_016 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_016 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_016 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_016 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_016 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_016, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_016, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_016, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_016, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_016, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_016, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_016, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_016, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_016, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_016, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_017 frame 0, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_017 frame 1, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_017 frame 2, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_017 frame 3, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_017 frame 4, Full | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_017 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_017 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_017 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_017 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_017 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| extract/fx_extract_017, in a run frame 0, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_017, in a run frame 1, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_017, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_017, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_017, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_017, in a run frame 0, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_017, in a run frame 1, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_017, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_017, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_017, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| extract/fx_extract_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_018, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_019, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_020, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_021, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_022, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_023, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_024, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_025, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026 frame 0, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026 frame 1, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026 frame 2, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026 frame 3, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026 frame 4, Draft | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| extract/fx_extract_026, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| fast_box_blur/fx_fastbox_007 frame 0, Full | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 1, Full | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 2, Full | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 3, Full | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 4, Full | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 0, Draft | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 1, Draft | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 2, Draft | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 3, Draft | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007 frame 4, Draft | 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007, in a run frame 0, Full | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007, in a run frame 1, Full | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007, in a run frame 2, Full | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007, in a run frame 3, Full | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007, in a run frame 4, Full | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007, in a run frame 0, Draft | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007, in a run frame 1, Draft | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007, in a run frame 2, Draft | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007, in a run frame 3, Draft | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fast_box_blur/fx_fastbox_007, in a run frame 4, Draft | 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001 frame 0, Full | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001 frame 1, Full | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001 frame 2, Full | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001 frame 3, Full | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001 frame 4, Full | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001 frame 0, Draft | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001 frame 1, Draft | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001 frame 2, Draft | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001 frame 3, Draft | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001 frame 4, Draft | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001, in a run frame 0, Full | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001, in a run frame 1, Full | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001, in a run frame 2, Full | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001, in a run frame 3, Full | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001, in a run frame 4, Full | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001, in a run frame 0, Draft | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001, in a run frame 1, Draft | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001, in a run frame 2, Draft | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001, in a run frame 3, Draft | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_001, in a run frame 4, Draft | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_addf_002 frame 0, Full | 0 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002 frame 1, Full | 0 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002 frame 2, Full | 0 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002 frame 3, Full | 0 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002 frame 4, Full | 0 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002 frame 0, Draft | 0 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002 frame 1, Draft | 0 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002 frame 2, Draft | 0 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002 frame 3, Draft | 0 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002 frame 4, Draft | 0 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002, in a run frame 0, Full | 0 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002, in a run frame 1, Full | 0 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002, in a run frame 2, Full | 0 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002, in a run frame 3, Full | 0 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002, in a run frame 4, Full | 0 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002, in a run frame 0, Draft | 0 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002, in a run frame 1, Draft | 0 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002, in a run frame 2, Draft | 0 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002, in a run frame 3, Draft | 0 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_addf_002, in a run frame 4, Draft | 0 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_001 frame 0, Full | 1 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001 frame 1, Full | 1 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001 frame 2, Full | 1 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001 frame 3, Full | 1 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001 frame 4, Full | 1 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001 frame 0, Draft | 1 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001 frame 1, Draft | 1 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001 frame 2, Draft | 1 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001 frame 3, Draft | 1 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001 frame 4, Draft | 1 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001, in a run frame 0, Full | 3 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001, in a run frame 1, Full | 3 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001, in a run frame 2, Full | 3 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001, in a run frame 3, Full | 3 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001, in a run frame 4, Full | 3 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001, in a run frame 0, Draft | 3 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001, in a run frame 1, Draft | 3 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001, in a run frame 2, Draft | 3 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001, in a run frame 3, Draft | 3 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_001, in a run frame 4, Draft | 3 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002 frame 0, Full | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002 frame 1, Full | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002 frame 2, Full | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002 frame 3, Full | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002 frame 4, Full | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002 frame 0, Draft | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002 frame 1, Draft | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002 frame 2, Draft | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002 frame 3, Draft | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002 frame 4, Draft | 0 / 1 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002, in a run frame 0, Full | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002, in a run frame 1, Full | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002, in a run frame 2, Full | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002, in a run frame 3, Full | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002, in a run frame 4, Full | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002, in a run frame 0, Draft | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002, in a run frame 1, Draft | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002, in a run frame 2, Draft | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002, in a run frame 3, Draft | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_002, in a run frame 4, Draft | 0 / 3 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_blend_scrf_003 frame 0, Full | 1 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003 frame 1, Full | 1 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003 frame 2, Full | 1 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003 frame 3, Full | 1 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003 frame 4, Full | 1 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003 frame 0, Draft | 1 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003 frame 1, Draft | 1 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003 frame 2, Draft | 1 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003 frame 3, Draft | 1 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003 frame 4, Draft | 1 / 1 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003, in a run frame 0, Full | 3 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003, in a run frame 1, Full | 3 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003, in a run frame 2, Full | 3 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003, in a run frame 3, Full | 3 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003, in a run frame 4, Full | 3 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003, in a run frame 0, Draft | 3 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003, in a run frame 1, Draft | 3 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003, in a run frame 2, Draft | 3 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003, in a run frame 3, Draft | 3 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_blend_scrf_003, in a run frame 4, Draft | 3 / 3 | 0 | 0 | none | PASS |
| float_depth/fx_fnoise_hdr_003 frame 0, Full | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003 frame 1, Full | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003 frame 2, Full | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003 frame 3, Full | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003 frame 4, Full | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003 frame 0, Draft | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003 frame 1, Draft | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003 frame 2, Draft | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003 frame 3, Draft | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003 frame 4, Draft | 2 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003, in a run frame 0, Full | 4 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003, in a run frame 1, Full | 4 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003, in a run frame 2, Full | 4 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003, in a run frame 3, Full | 4 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003, in a run frame 4, Full | 4 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003, in a run frame 0, Draft | 4 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003, in a run frame 1, Draft | 4 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003, in a run frame 2, Draft | 4 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003, in a run frame 3, Draft | 4 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| float_depth/fx_fnoise_hdr_003, in a run frame 4, Draft | 4 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_001 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001, in a run frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001, in a run frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001, in a run frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_001, in a run frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002, in a run frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002, in a run frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002, in a run frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_002, in a run frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003, in a run frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003, in a run frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003, in a run frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_003, in a run frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004, in a run frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004, in a run frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004, in a run frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_004, in a run frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005, in a run frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005, in a run frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005, in a run frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_005, in a run frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008, in a run frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008, in a run frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008, in a run frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_008, in a run frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| fxkey/fx_fxk_009 frame 0, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009 frame 1, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009 frame 2, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009 frame 3, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009 frame 4, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009 frame 0, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009 frame 1, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009 frame 2, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009 frame 3, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009 frame 4, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009, in a run frame 0, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009, in a run frame 1, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009, in a run frame 2, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009, in a run frame 3, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009, in a run frame 4, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009, in a run frame 0, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009, in a run frame 1, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009, in a run frame 2, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009, in a run frame 3, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| fxkey/fx_fxk_009, in a run frame 4, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002 frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002 frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002 frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002 frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002 frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002 frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002 frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002 frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002 frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002 frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002, in a run frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002, in a run frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002, in a run frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002, in a run frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002, in a run frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002, in a run frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002, in a run frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002, in a run frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002, in a run frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_002, in a run frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008 frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008 frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008 frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008 frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008 frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008 frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008 frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008 frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008 frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008 frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008, in a run frame 0, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008, in a run frame 1, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008, in a run frame 2, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008, in a run frame 3, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008, in a run frame 4, Full | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008, in a run frame 0, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008, in a run frame 1, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008, in a run frame 2, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008, in a run frame 3, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| glow_display/fx_gldisp_008, in a run frame 4, Draft | 0 | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| layer_map/layer_map frame 0, Full | 0 / 0 / 0 | 0 | 0 | COMPOSITION_REFERENCE_MISSING, MEDIA_MISSING, on both | PASS |
| layer_map/layer_map frame 1, Full | 0 / 0 / 0 | 0 | 0 | COMPOSITION_REFERENCE_MISSING, MEDIA_MISSING, on both | PASS |
| layer_map/layer_map frame 2, Full | 0 / 0 / 0 | 0 | 0 | COMPOSITION_REFERENCE_MISSING, MEDIA_MISSING, on both | PASS |
| layer_map/layer_map frame 0, Draft | 0 / 0 / 0 | 0 | 0 | COMPOSITION_REFERENCE_MISSING, MEDIA_MISSING, on both | PASS |
| layer_map/layer_map frame 1, Draft | 0 / 0 / 0 | 0 | 0 | COMPOSITION_REFERENCE_MISSING, MEDIA_MISSING, on both | PASS |
| layer_map/layer_map frame 2, Draft | 0 / 0 / 0 | 0 | 0 | COMPOSITION_REFERENCE_MISSING, MEDIA_MISSING, on both | PASS |
| layer_map/layer_map, in a run frame 0, Full | 0 / 0 / 0 | 0 | 0 | COMPOSITION_REFERENCE_MISSING, MEDIA_MISSING, on both | PASS |
| layer_map/layer_map, in a run frame 1, Full | 0 / 0 / 0 | 0 | 0 | COMPOSITION_REFERENCE_MISSING, MEDIA_MISSING, on both | PASS |
| layer_map/layer_map, in a run frame 2, Full | 0 / 0 / 0 | 0 | 0 | COMPOSITION_REFERENCE_MISSING, MEDIA_MISSING, on both | PASS |
| layer_map/layer_map, in a run frame 0, Draft | 0 / 0 / 0 | 0 | 0 | COMPOSITION_REFERENCE_MISSING, MEDIA_MISSING, on both | PASS |
| layer_map/layer_map, in a run frame 1, Draft | 0 / 0 / 0 | 0 | 0 | COMPOSITION_REFERENCE_MISSING, MEDIA_MISSING, on both | PASS |
| layer_map/layer_map, in a run frame 2, Draft | 0 / 0 / 0 | 0 | 0 | COMPOSITION_REFERENCE_MISSING, MEDIA_MISSING, on both | PASS |
| light_wrap/fx_wrap_013 frame 0, Full | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013 frame 1, Full | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013 frame 2, Full | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013 frame 3, Full | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013 frame 4, Full | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013 frame 0, Draft | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013 frame 1, Draft | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013 frame 2, Draft | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013 frame 3, Draft | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013 frame 4, Draft | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013, in a run frame 0, Full | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013, in a run frame 1, Full | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013, in a run frame 2, Full | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013, in a run frame 3, Full | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013, in a run frame 4, Full | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013, in a run frame 0, Draft | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013, in a run frame 1, Draft | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013, in a run frame 2, Draft | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013, in a run frame 3, Draft | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_013, in a run frame 4, Draft | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014 frame 0, Full | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014 frame 1, Full | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014 frame 2, Full | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014 frame 3, Full | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014 frame 4, Full | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014 frame 0, Draft | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014 frame 1, Draft | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014 frame 2, Draft | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014 frame 3, Draft | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014 frame 4, Draft | 0 / 1 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014, in a run frame 0, Full | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014, in a run frame 1, Full | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014, in a run frame 2, Full | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014, in a run frame 3, Full | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014, in a run frame 4, Full | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014, in a run frame 0, Draft | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014, in a run frame 1, Draft | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014, in a run frame 2, Draft | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014, in a run frame 3, Draft | 0 / 3 | 0 | 0 | none | PASS |
| light_wrap/fx_wrap_014, in a run frame 4, Draft | 0 / 3 | 0 | 0 | none | PASS |
| limits/fx_limit_001 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001, in a run frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001, in a run frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001, in a run frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_001, in a run frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002, in a run frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002, in a run frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002, in a run frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_002, in a run frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004, in a run frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004, in a run frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004, in a run frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_004, in a run frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006, in a run frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006, in a run frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006, in a run frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_006, in a run frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007, in a run frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007, in a run frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007, in a run frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_007, in a run frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_008 frame 0, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008 frame 1, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008 frame 2, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008 frame 3, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008 frame 4, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008 frame 0, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008 frame 1, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008 frame 2, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008 frame 3, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008 frame 4, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008, in a run frame 0, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008, in a run frame 1, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008, in a run frame 2, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008, in a run frame 3, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008, in a run frame 4, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008, in a run frame 0, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008, in a run frame 1, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008, in a run frame 2, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008, in a run frame 3, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_008, in a run frame 4, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_011 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011, in a run frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011, in a run frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011, in a run frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_011, in a run frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012, in a run frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012, in a run frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012, in a run frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_012, in a run frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013, in a run frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013, in a run frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013, in a run frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_013, in a run frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014 frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014 frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014 frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014 frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014, in a run frame 3, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014, in a run frame 4, Full | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014, in a run frame 3, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_014, in a run frame 4, Draft | 0 / 0 | 0 | 0 | none | PASS |
| limits/fx_limit_015 frame 0, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015 frame 1, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015 frame 2, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015 frame 3, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015 frame 4, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015 frame 0, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015 frame 1, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015 frame 2, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015 frame 3, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015 frame 4, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015, in a run frame 0, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015, in a run frame 1, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015, in a run frame 2, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015, in a run frame 3, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015, in a run frame 4, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015, in a run frame 0, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015, in a run frame 1, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015, in a run frame 2, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015, in a run frame 3, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_015, in a run frame 4, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016 frame 0, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016 frame 1, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016 frame 2, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016 frame 3, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016 frame 4, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016 frame 0, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016 frame 1, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016 frame 2, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016 frame 3, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016 frame 4, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016, in a run frame 0, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016, in a run frame 1, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016, in a run frame 2, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016, in a run frame 3, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016, in a run frame 4, Full | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016, in a run frame 0, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016, in a run frame 1, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016, in a run frame 2, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016, in a run frame 3, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| limits/fx_limit_016, in a run frame 4, Draft | 0 / 0 | 0 | 0 | CPU: EFFECT_PARAMETER_INVALID; GPU: EFFECT_PARAMETER_INVALID, GPU_PREVIEW_ON_CPU | PASS: the card refused the frame and said so; the CPU's picture exactly |
| posterize_time/fx_ptime_009 frame 0, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009 frame 1, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009 frame 2, Full | 0 | 1 | 3 | none | PASS |
| posterize_time/fx_ptime_009 frame 3, Full | 0 | 1 | 3 | none | PASS |
| posterize_time/fx_ptime_009 frame 4, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009 frame 5, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009 frame 6, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009 frame 7, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009 frame 2, Draft | 0 | 1 | 1 | none | PASS |
| posterize_time/fx_ptime_009 frame 3, Draft | 0 | 1 | 1 | none | PASS |
| posterize_time/fx_ptime_009 frame 4, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009 frame 5, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009 frame 6, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009 frame 7, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 0, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 1, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 2, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 3, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 4, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 5, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 6, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 7, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 0, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 1, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 2, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 3, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 4, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 5, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 6, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_009, in a run frame 7, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 0, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 1, Full | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 5, Full | 1 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 6, Full | 1 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 7, Full | 1 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 5, Draft | 1 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 6, Draft | 1 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010 frame 7, Draft | 1 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 2, Full | 3 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 3, Full | 3 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 4, Full | 3 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 5, Full | 3 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 6, Full | 3 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 7, Full | 3 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 2, Draft | 3 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 3, Draft | 3 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 4, Draft | 3 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 5, Draft | 3 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 6, Draft | 3 | 0 | 0 | none | PASS |
| posterize_time/fx_ptime_010, in a run frame 7, Draft | 3 | 0 | 0 | none | PASS |
| precomp/fx_pre_008 frame 0, Full | 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_008 frame 1, Full | 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_008 frame 2, Full | 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_008 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_008 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_008 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_008, in a run frame 0, Full | 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_008, in a run frame 1, Full | 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_008, in a run frame 2, Full | 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_008, in a run frame 0, Draft | 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_008, in a run frame 1, Draft | 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_008, in a run frame 2, Draft | 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_009 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_009 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_009 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_009 frame 0, Draft | 0 / 0 | 1 | 1 | none | PASS |
| precomp/fx_pre_009 frame 1, Draft | 0 / 0 | 1 | 1 | none | PASS |
| precomp/fx_pre_009 frame 2, Draft | 0 / 0 | 1 | 1 | none | PASS |
| precomp/fx_pre_009, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_009, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_009, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_009, in a run frame 0, Draft | 0 / 0 | 1 | 1 | none | PASS |
| precomp/fx_pre_009, in a run frame 1, Draft | 0 / 0 | 1 | 1 | none | PASS |
| precomp/fx_pre_009, in a run frame 2, Draft | 0 / 0 | 1 | 1 | none | PASS |
| precomp/fx_pre_010 frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_010 frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_010 frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_010 frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_010 frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_010 frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_010, in a run frame 0, Full | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_010, in a run frame 1, Full | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_010, in a run frame 2, Full | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_010, in a run frame 0, Draft | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_010, in a run frame 1, Draft | 0 / 0 | 0 | 0 | none | PASS |
| precomp/fx_pre_010, in a run frame 2, Draft | 0 / 0 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_001, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_002 frame 0, Full | 0 | 0 | 0 | none | PASS |
| recolor/fx_recolor_002 frame 1, Full | 0 | 0 | 0 | none | PASS |
| recolor/fx_recolor_002 frame 2, Full | 0 | 0 | 0 | none | PASS |
| recolor/fx_recolor_002 frame 3, Full | 0 | 0 | 0 | none | PASS |
| recolor/fx_recolor_002 frame 4, Full | 0 | 0 | 0 | none | PASS |
| recolor/fx_recolor_002 frame 0, Draft | 0 | 1 | 1 | none | PASS |
| recolor/fx_recolor_002 frame 1, Draft | 0 | 1 | 1 | none | PASS |
| recolor/fx_recolor_002 frame 2, Draft | 0 | 1 | 1 | none | PASS |
| recolor/fx_recolor_002 frame 3, Draft | 0 | 1 | 1 | none | PASS |
| recolor/fx_recolor_002 frame 4, Draft | 0 | 1 | 1 | none | PASS |
| recolor/fx_recolor_002, in a run frame 0, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_002, in a run frame 1, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_002, in a run frame 2, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_002, in a run frame 3, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_002, in a run frame 4, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_002, in a run frame 0, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_002, in a run frame 1, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_002, in a run frame 2, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_002, in a run frame 3, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_002, in a run frame 4, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_003, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_004, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_005, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_006, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_007, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_008, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_009, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010 frame 0, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010 frame 1, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010 frame 2, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010 frame 3, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010 frame 4, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_010, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011 frame 0, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011 frame 1, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011 frame 2, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011 frame 3, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011 frame 4, Full | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_011, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| recolor/fx_recolor_012 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_012, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_013, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_014, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_015, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_016, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_017, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| recolor/fx_recolor_018, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_001, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_001, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_001, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_001, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_001, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_001, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_001, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_001, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_001, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_001, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_002, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_002, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_002, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_002, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_002, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_002, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_002, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_002, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_002, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_002, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_003 frame 0, Full | 0 | 0 | 0 | none | PASS |
| select_color/fx_select_003 frame 1, Full | 0 | 0 | 0 | none | PASS |
| select_color/fx_select_003 frame 2, Full | 0 | 0 | 0 | none | PASS |
| select_color/fx_select_003 frame 3, Full | 0 | 0 | 0 | none | PASS |
| select_color/fx_select_003 frame 4, Full | 0 | 0 | 0 | none | PASS |
| select_color/fx_select_003 frame 0, Draft | 0 | 1 | 1 | none | PASS |
| select_color/fx_select_003 frame 1, Draft | 0 | 1 | 1 | none | PASS |
| select_color/fx_select_003 frame 2, Draft | 0 | 1 | 1 | none | PASS |
| select_color/fx_select_003 frame 3, Draft | 0 | 1 | 1 | none | PASS |
| select_color/fx_select_003 frame 4, Draft | 0 | 1 | 1 | none | PASS |
| select_color/fx_select_003, in a run frame 0, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_003, in a run frame 1, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_003, in a run frame 2, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_003, in a run frame 3, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_003, in a run frame 4, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_003, in a run frame 0, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_003, in a run frame 1, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_003, in a run frame 2, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_003, in a run frame 3, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_003, in a run frame 4, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_004 frame 0, Full | 0 | 0 | 0 | none | PASS |
| select_color/fx_select_004 frame 1, Full | 0 | 0 | 0 | none | PASS |
| select_color/fx_select_004 frame 2, Full | 0 | 0 | 0 | none | PASS |
| select_color/fx_select_004 frame 3, Full | 0 | 0 | 0 | none | PASS |
| select_color/fx_select_004 frame 4, Full | 0 | 0 | 0 | none | PASS |
| select_color/fx_select_004 frame 0, Draft | 0 | 1 | 1 | none | PASS |
| select_color/fx_select_004 frame 1, Draft | 0 | 1 | 1 | none | PASS |
| select_color/fx_select_004 frame 2, Draft | 0 | 1 | 1 | none | PASS |
| select_color/fx_select_004 frame 3, Draft | 0 | 1 | 1 | none | PASS |
| select_color/fx_select_004 frame 4, Draft | 0 | 1 | 1 | none | PASS |
| select_color/fx_select_004, in a run frame 0, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_004, in a run frame 1, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_004, in a run frame 2, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_004, in a run frame 3, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_004, in a run frame 4, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_004, in a run frame 0, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_004, in a run frame 1, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_004, in a run frame 2, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_004, in a run frame 3, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_004, in a run frame 4, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_005 frame 0, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_005 frame 1, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_005 frame 2, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_005 frame 3, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_005 frame 4, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_005, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_005, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_005, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_005, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_005, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_005, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_005, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_005, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_005, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_005, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_006, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_006, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_006, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_006, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_006, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_006, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_006, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_006, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_006, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_006, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_007 frame 0, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_007 frame 1, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_007 frame 2, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_007 frame 3, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_007 frame 4, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_007, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_007, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_007, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_007, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_007, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_007, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_007, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_007, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_007, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_007, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_008 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_008 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_008 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_008 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_008 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_008, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_008, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_008, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_008, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_008, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_008, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_008, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_008, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_008, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_008, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_009 frame 0, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_009 frame 1, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_009 frame 2, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_009 frame 3, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_009 frame 4, Full | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| select_color/fx_select_009, in a run frame 0, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_009, in a run frame 1, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_009, in a run frame 2, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_009, in a run frame 3, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_009, in a run frame 4, Full | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_009, in a run frame 0, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_009, in a run frame 1, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_009, in a run frame 2, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_009, in a run frame 3, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_009, in a run frame 4, Draft | 2 | 0 | 0 | none | PASS |
| select_color/fx_select_010 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_010, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_011, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_012, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_013, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_014, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_015, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016 frame 0, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016 frame 1, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016 frame 2, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016 frame 3, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016 frame 4, Full | 0 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016 frame 0, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016 frame 1, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016 frame 2, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016 frame 3, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016 frame 4, Draft | 0 | 1 | 1 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016, in a run frame 0, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016, in a run frame 1, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016, in a run frame 2, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016, in a run frame 3, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016, in a run frame 4, Full | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016, in a run frame 0, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016, in a run frame 1, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016, in a run frame 2, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016, in a run frame 3, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| select_color/fx_select_016, in a run frame 4, Draft | 1 | 0 | 0 | EFFECT_PARAMETER_INVALID, on both | PASS |
| solid/fx_sol_005 frame 0, Full | 0 | 0 | 0 | none | PASS |
| solid/fx_sol_005 frame 1, Full | 0 | 0 | 0 | none | PASS |
| solid/fx_sol_005 frame 2, Full | 0 | 0 | 0 | none | PASS |
| solid/fx_sol_005 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| solid/fx_sol_005 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| solid/fx_sol_005 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| solid/fx_sol_005, in a run frame 0, Full | 0 | 0 | 0 | none | PASS |
| solid/fx_sol_005, in a run frame 1, Full | 0 | 0 | 0 | none | PASS |
| solid/fx_sol_005, in a run frame 2, Full | 0 | 0 | 0 | none | PASS |
| solid/fx_sol_005, in a run frame 0, Draft | 0 | 0 | 0 | none | PASS |
| solid/fx_sol_005, in a run frame 1, Draft | 0 | 0 | 0 | none | PASS |
| solid/fx_sol_005, in a run frame 2, Draft | 0 | 0 | 0 | none | PASS |
| the reference shot with Exposure (1) frame 0, Full | 1 / 2 / 1 | 1 | 194 | none | PASS |
| the reference shot with Exposure (1) frame 100, Full | 1 / 2 / 1 | 1 | 84 | none | PASS |
| the reference shot with Exposure (1) frame 239, Full | 1 / 2 / 1 | 1 | 193 | none | PASS |
| the reference shot with Exposure (1) frame 0, Draft | 1 / 2 / 1 | 1 | 33 | none | PASS |
| the reference shot with Exposure (1) frame 100, Draft | 1 / 2 / 1 | 1 | 53 | none | PASS |
| the reference shot with Exposure (1) frame 239, Draft | 1 / 2 / 1 | 1 | 31 | none | PASS |
| the reference shot with Exposure (1), in a run frame 0, Full | 3 / 4 / 3 | 1 | 480 | none | PASS |
| the reference shot with Exposure (1), in a run frame 100, Full | 3 / 4 / 3 | 1 | 804 | none | PASS |
| the reference shot with Exposure (1), in a run frame 239, Full | 3 / 4 / 3 | 1 | 543 | none | PASS |
| the reference shot with Exposure (1), in a run frame 0, Draft | 3 / 4 / 3 | 1 | 25 | none | PASS |
| the reference shot with Exposure (1), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 17 | none | PASS |
| the reference shot with Exposure (1), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 19 | none | PASS |
| the reference shot with Exposure (2) frame 0, Full | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Exposure (2) frame 100, Full | 1 / 2 / 1 | 1 | 2 | none | PASS |
| the reference shot with Exposure (2) frame 239, Full | 1 / 2 / 1 | 1 | 1 | none | PASS |
| the reference shot with Exposure (2) frame 0, Draft | 1 / 2 / 1 | 1 | 44 | none | PASS |
| the reference shot with Exposure (2) frame 100, Draft | 1 / 2 / 1 | 1 | 42 | none | PASS |
| the reference shot with Exposure (2) frame 239, Draft | 1 / 2 / 1 | 1 | 40 | none | PASS |
| the reference shot with Exposure (2), in a run frame 0, Full | 3 / 4 / 3 | 1 | 565 | none | PASS |
| the reference shot with Exposure (2), in a run frame 100, Full | 3 / 4 / 3 | 1 | 601 | none | PASS |
| the reference shot with Exposure (2), in a run frame 239, Full | 3 / 4 / 3 | 1 | 716 | none | PASS |
| the reference shot with Exposure (2), in a run frame 0, Draft | 3 / 4 / 3 | 1 | 39 | none | PASS |
| the reference shot with Exposure (2), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 45 | none | PASS |
| the reference shot with Exposure (2), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 38 | none | PASS |
| the reference shot with Tint (1) frame 0, Full | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Tint (1) frame 100, Full | 1 / 2 / 1 | 1 | 2 | none | PASS |
| the reference shot with Tint (1) frame 239, Full | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Tint (1) frame 0, Draft | 1 / 2 / 1 | 1 | 42 | none | PASS |
| the reference shot with Tint (1) frame 100, Draft | 1 / 2 / 1 | 1 | 86 | none | PASS |
| the reference shot with Tint (1) frame 239, Draft | 1 / 2 / 1 | 1 | 45 | none | PASS |
| the reference shot with Tint (1), in a run frame 0, Full | 3 / 4 / 3 | 1 | 756 | none | PASS |
| the reference shot with Tint (1), in a run frame 100, Full | 3 / 4 / 3 | 1 | 664 | none | PASS |
| the reference shot with Tint (1), in a run frame 239, Full | 3 / 4 / 3 | 1 | 735 | none | PASS |
| the reference shot with Tint (1), in a run frame 0, Draft | 3 / 4 / 3 | 1 | 75 | none | PASS |
| the reference shot with Tint (1), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 50 | none | PASS |
| the reference shot with Tint (1), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 47 | none | PASS |
| the reference shot with Shift Channels (1) frame 0, Full | 1 / 2 / 1 | 1 | 4004 | none | PASS |
| the reference shot with Shift Channels (1) frame 100, Full | 1 / 2 / 1 | 1 | 1887 | none | PASS |
| the reference shot with Shift Channels (1) frame 239, Full | 1 / 2 / 1 | 1 | 3000 | none | PASS |
| the reference shot with Shift Channels (1) frame 0, Draft | 1 / 2 / 1 | 1 | 145 | none | PASS |
| the reference shot with Shift Channels (1) frame 100, Draft | 1 / 2 / 1 | 1 | 50 | none | PASS |
| the reference shot with Shift Channels (1) frame 239, Draft | 1 / 2 / 1 | 1 | 108 | none | PASS |
| the reference shot with Shift Channels (1), in a run frame 0, Full | 3 / 4 / 3 | 1 | 961 | none | PASS |
| the reference shot with Shift Channels (1), in a run frame 100, Full | 3 / 4 / 3 | 1 | 1197 | none | PASS |
| the reference shot with Shift Channels (1), in a run frame 239, Full | 3 / 4 / 3 | 1 | 871 | none | PASS |
| the reference shot with Shift Channels (1), in a run frame 0, Draft | 3 / 4 / 3 | 1 | 56 | none | PASS |
| the reference shot with Shift Channels (1), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 73 | none | PASS |
| the reference shot with Shift Channels (1), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 58 | none | PASS |
| the reference shot with Shift Channels (2) frame 0, Full | 1 / 2 / 1 | 1 | 38588 | none | PASS |
| the reference shot with Shift Channels (2) frame 100, Full | 1 / 2 / 1 | 1 | 38979 | none | PASS |
| the reference shot with Shift Channels (2) frame 239, Full | 1 / 2 / 1 | 1 | 40469 | none | PASS |
| the reference shot with Shift Channels (2) frame 0, Draft | 1 / 2 / 1 | 1 | 1349 | none | PASS |
| the reference shot with Shift Channels (2) frame 100, Draft | 1 / 2 / 1 | 1 | 1366 | none | PASS |
| the reference shot with Shift Channels (2) frame 239, Draft | 1 / 2 / 1 | 1 | 1422 | none | PASS |
| the reference shot with Shift Channels (2), in a run frame 0, Full | 3 / 4 / 3 | 1 | 1799 | none | PASS |
| the reference shot with Shift Channels (2), in a run frame 100, Full | 3 / 4 / 3 | 1 | 1137 | none | PASS |
| the reference shot with Shift Channels (2), in a run frame 239, Full | 3 / 4 / 3 | 1 | 2018 | none | PASS |
| the reference shot with Shift Channels (2), in a run frame 0, Draft | 3 / 4 / 3 | 1 | 105 | none | PASS |
| the reference shot with Shift Channels (2), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 95 | none | PASS |
| the reference shot with Shift Channels (2), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 123 | none | PASS |
| the reference shot with Shift Channels (3) frame 0, Full | 1 / 2 / 1 | 1 | 1 | none | PASS |
| the reference shot with Shift Channels (3) frame 100, Full | 1 / 2 / 1 | 1 | 1 | none | PASS |
| the reference shot with Shift Channels (3) frame 239, Full | 1 / 2 / 1 | 1 | 1 | none | PASS |
| the reference shot with Shift Channels (3) frame 0, Draft | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Shift Channels (3) frame 100, Draft | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Shift Channels (3) frame 239, Draft | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Shift Channels (3), in a run frame 0, Full | 3 / 4 / 3 | 0 | 0 | none | PASS |
| the reference shot with Shift Channels (3), in a run frame 100, Full | 3 / 4 / 3 | 0 | 0 | none | PASS |
| the reference shot with Shift Channels (3), in a run frame 239, Full | 3 / 4 / 3 | 0 | 0 | none | PASS |
| the reference shot with Shift Channels (3), in a run frame 0, Draft | 3 / 4 / 3 | 0 | 0 | none | PASS |
| the reference shot with Shift Channels (3), in a run frame 100, Draft | 3 / 4 / 3 | 0 | 0 | none | PASS |
| the reference shot with Shift Channels (3), in a run frame 239, Draft | 3 / 4 / 3 | 0 | 0 | none | PASS |
| the reference shot with Solid Composite (1) frame 0, Full | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Solid Composite (1) frame 100, Full | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Solid Composite (1) frame 239, Full | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Solid Composite (1) frame 0, Draft | 1 / 2 / 1 | 1 | 48 | none | PASS |
| the reference shot with Solid Composite (1) frame 100, Draft | 1 / 2 / 1 | 1 | 48 | none | PASS |
| the reference shot with Solid Composite (1) frame 239, Draft | 1 / 2 / 1 | 1 | 40 | none | PASS |
| the reference shot with Solid Composite (1), in a run frame 0, Full | 3 / 4 / 3 | 1 | 431 | none | PASS |
| the reference shot with Solid Composite (1), in a run frame 100, Full | 3 / 4 / 3 | 1 | 390 | none | PASS |
| the reference shot with Solid Composite (1), in a run frame 239, Full | 3 / 4 / 3 | 1 | 456 | none | PASS |
| the reference shot with Solid Composite (1), in a run frame 0, Draft | 3 / 4 / 3 | 1 | 56 | none | PASS |
| the reference shot with Solid Composite (1), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 69 | none | PASS |
| the reference shot with Solid Composite (1), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 70 | none | PASS |
| the reference shot with Solid Composite (2) frame 0, Full | 1 / 2 / 1 | 1 | 801 | none | PASS |
| the reference shot with Solid Composite (2) frame 100, Full | 1 / 2 / 1 | 1 | 1806 | none | PASS |
| the reference shot with Solid Composite (2) frame 239, Full | 1 / 2 / 1 | 1 | 1131 | none | PASS |
| the reference shot with Solid Composite (2) frame 0, Draft | 1 / 2 / 1 | 1 | 72 | none | PASS |
| the reference shot with Solid Composite (2) frame 100, Draft | 1 / 2 / 1 | 1 | 83 | none | PASS |
| the reference shot with Solid Composite (2) frame 239, Draft | 1 / 2 / 1 | 1 | 45 | none | PASS |
| the reference shot with Solid Composite (2), in a run frame 0, Full | 3 / 4 / 3 | 1 | 2641 | none | PASS |
| the reference shot with Solid Composite (2), in a run frame 100, Full | 3 / 4 / 3 | 1 | 938 | none | PASS |
| the reference shot with Solid Composite (2), in a run frame 239, Full | 3 / 4 / 3 | 1 | 890 | none | PASS |
| the reference shot with Solid Composite (2), in a run frame 0, Draft | 3 / 4 / 3 | 1 | 122 | none | PASS |
| the reference shot with Solid Composite (2), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 58 | none | PASS |
| the reference shot with Solid Composite (2), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 56 | none | PASS |
| the reference shot with Solid Composite (3) frame 0, Full | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Solid Composite (3) frame 100, Full | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Solid Composite (3) frame 239, Full | 1 / 2 / 1 | 1 | 3 | none | PASS |
| the reference shot with Solid Composite (3) frame 0, Draft | 1 / 2 / 1 | 1 | 56 | none | PASS |
| the reference shot with Solid Composite (3) frame 100, Draft | 1 / 2 / 1 | 1 | 64 | none | PASS |
| the reference shot with Solid Composite (3) frame 239, Draft | 1 / 2 / 1 | 1 | 54 | none | PASS |
| the reference shot with Solid Composite (3), in a run frame 0, Full | 3 / 4 / 3 | 1 | 423 | none | PASS |
| the reference shot with Solid Composite (3), in a run frame 100, Full | 3 / 4 / 3 | 1 | 256 | none | PASS |
| the reference shot with Solid Composite (3), in a run frame 239, Full | 3 / 4 / 3 | 1 | 336 | none | PASS |
| the reference shot with Solid Composite (3), in a run frame 0, Draft | 3 / 4 / 3 | 1 | 43 | none | PASS |
| the reference shot with Solid Composite (3), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 39 | none | PASS |
| the reference shot with Solid Composite (3), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 38 | none | PASS |
| the reference shot with Solid Composite (4) frame 0, Full | 1 / 2 / 1 | 1 | 145 | none | PASS |
| the reference shot with Solid Composite (4) frame 100, Full | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Solid Composite (4) frame 239, Full | 1 / 2 / 1 | 1 | 40 | none | PASS |
| the reference shot with Solid Composite (4) frame 0, Draft | 1 / 2 / 1 | 1 | 51 | none | PASS |
| the reference shot with Solid Composite (4) frame 100, Draft | 1 / 2 / 1 | 1 | 68 | none | PASS |
| the reference shot with Solid Composite (4) frame 239, Draft | 1 / 2 / 1 | 1 | 60 | none | PASS |
| the reference shot with Solid Composite (4), in a run frame 0, Full | 3 / 4 / 3 | 1 | 1107 | none | PASS |
| the reference shot with Solid Composite (4), in a run frame 100, Full | 3 / 4 / 3 | 1 | 3259 | none | PASS |
| the reference shot with Solid Composite (4), in a run frame 239, Full | 3 / 4 / 3 | 1 | 1329 | none | PASS |
| the reference shot with Solid Composite (4), in a run frame 0, Draft | 3 / 4 / 3 | 1 | 92 | none | PASS |
| the reference shot with Solid Composite (4), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 87 | none | PASS |
| the reference shot with Solid Composite (4), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 91 | none | PASS |
| the reference shot with Change to Color (1) frame 0, Full | 1 / 2 / 1 | 1 | 3718 | none | PASS |
| the reference shot with Change to Color (1) frame 100, Full | 1 / 2 / 1 | 1 | 1415 | none | PASS |
| the reference shot with Change to Color (1) frame 239, Full | 1 / 2 / 1 | 1 | 1741 | none | PASS |
| the reference shot with Change to Color (1) frame 0, Draft | 1 / 2 / 1 | 1 | 126 | none | PASS |
| the reference shot with Change to Color (1) frame 100, Draft | 1 / 2 / 1 | 1 | 32 | none | PASS |
| the reference shot with Change to Color (1) frame 239, Draft | 1 / 2 / 1 | 1 | 47 | none | PASS |
| the reference shot with Change to Color (1), in a run frame 0, Full | 3 / 4 / 3 | 1 | 369 | none | PASS |
| the reference shot with Change to Color (1), in a run frame 100, Full | 3 / 4 / 3 | 1 | 641 | none | PASS |
| the reference shot with Change to Color (1), in a run frame 239, Full | 3 / 4 / 3 | 1 | 425 | none | PASS |
| the reference shot with Change to Color (1), in a run frame 0, Draft | 3 / 4 / 3 | 1 | 48 | none | PASS |
| the reference shot with Change to Color (1), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 46 | none | PASS |
| the reference shot with Change to Color (1), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 46 | none | PASS |
| the reference shot with Change to Color (2) frame 0, Full | 1 / 2 / 1 | 1 | 4 | none | PASS |
| the reference shot with Change to Color (2) frame 100, Full | 1 / 2 / 1 | 1 | 4 | none | PASS |
| the reference shot with Change to Color (2) frame 239, Full | 1 / 2 / 1 | 1 | 3 | none | PASS |
| the reference shot with Change to Color (2) frame 0, Draft | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Change to Color (2) frame 100, Draft | 1 / 2 / 1 | 1 | 2 | none | PASS |
| the reference shot with Change to Color (2) frame 239, Draft | 1 / 2 / 1 | 1 | 1 | none | PASS |
| the reference shot with Change to Color (2), in a run frame 0, Full | 3 / 4 / 3 | 1 | 4 | none | PASS |
| the reference shot with Change to Color (2), in a run frame 100, Full | 3 / 4 / 3 | 1 | 2 | none | PASS |
| the reference shot with Change to Color (2), in a run frame 239, Full | 3 / 4 / 3 | 1 | 2 | none | PASS |
| the reference shot with Change to Color (2), in a run frame 0, Draft | 3 / 4 / 3 | 0 | 0 | none | PASS |
| the reference shot with Change to Color (2), in a run frame 100, Draft | 3 / 4 / 3 | 0 | 0 | none | PASS |
| the reference shot with Change to Color (2), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 1 | none | PASS |
| the reference shot with Color Key (1) frame 0, Full | 1 / 1 / 1 | 1 | 3813 | none | PASS |
| the reference shot with Color Key (1) frame 100, Full | 1 / 1 / 1 | 1 | 1502 | none | PASS |
| the reference shot with Color Key (1) frame 239, Full | 1 / 1 / 1 | 1 | 1853 | none | PASS |
| the reference shot with Color Key (1) frame 0, Draft | 1 / 1 / 1 | 1 | 128 | none | PASS |
| the reference shot with Color Key (1) frame 100, Draft | 1 / 1 / 1 | 1 | 37 | none | PASS |
| the reference shot with Color Key (1) frame 239, Draft | 1 / 1 / 1 | 1 | 51 | none | PASS |
| the reference shot with Color Key (1), in a run frame 0, Full | 2 / 2 / 2 | 1 | 35 | none | PASS |
| the reference shot with Color Key (1), in a run frame 100, Full | 2 / 2 / 2 | 1 | 85 | none | PASS |
| the reference shot with Color Key (1), in a run frame 239, Full | 2 / 2 / 2 | 1 | 82 | none | PASS |
| the reference shot with Color Key (1), in a run frame 0, Draft | 2 / 2 / 2 | 1 | 41 | none | PASS |
| the reference shot with Color Key (1), in a run frame 100, Draft | 2 / 2 / 2 | 1 | 60 | none | PASS |
| the reference shot with Color Key (1), in a run frame 239, Draft | 2 / 2 / 2 | 1 | 50 | none | PASS |
| the reference shot with Color Key (2) frame 0, Full | 1 / 1 / 1 | 1 | 3718 | none | PASS |
| the reference shot with Color Key (2) frame 100, Full | 1 / 1 / 1 | 1 | 1413 | none | PASS |
| the reference shot with Color Key (2) frame 239, Full | 1 / 1 / 1 | 1 | 1740 | none | PASS |
| the reference shot with Color Key (2) frame 0, Draft | 1 / 1 / 1 | 1 | 126 | none | PASS |
| the reference shot with Color Key (2) frame 100, Draft | 1 / 1 / 1 | 1 | 32 | none | PASS |
| the reference shot with Color Key (2) frame 239, Draft | 1 / 1 / 1 | 1 | 47 | none | PASS |
| the reference shot with Color Key (2), in a run frame 0, Full | 2 / 2 / 2 | 1 | 6 | none | PASS |
| the reference shot with Color Key (2), in a run frame 100, Full | 2 / 2 / 2 | 1 | 4 | none | PASS |
| the reference shot with Color Key (2), in a run frame 239, Full | 2 / 2 / 2 | 1 | 7 | none | PASS |
| the reference shot with Color Key (2), in a run frame 0, Draft | 2 / 2 / 2 | 1 | 40 | none | PASS |
| the reference shot with Color Key (2), in a run frame 100, Draft | 2 / 2 / 2 | 1 | 56 | none | PASS |
| the reference shot with Color Key (2), in a run frame 239, Draft | 2 / 2 / 2 | 1 | 45 | none | PASS |
| the reference shot with Select Color (1) frame 0, Full | 1 / 1 / 1 | 0 | 0 | none | PASS |
| the reference shot with Select Color (1) frame 100, Full | 1 / 1 / 1 | 0 | 0 | none | PASS |
| the reference shot with Select Color (1) frame 239, Full | 1 / 1 / 1 | 0 | 0 | none | PASS |
| the reference shot with Select Color (1) frame 0, Draft | 1 / 1 / 1 | 0 | 0 | none | PASS |
| the reference shot with Select Color (1) frame 100, Draft | 1 / 1 / 1 | 0 | 0 | none | PASS |
| the reference shot with Select Color (1) frame 239, Draft | 1 / 1 / 1 | 0 | 0 | none | PASS |
| the reference shot with Select Color (1), in a run frame 0, Full | 2 / 2 / 2 | 0 | 0 | none | PASS |
| the reference shot with Select Color (1), in a run frame 100, Full | 2 / 2 / 2 | 0 | 0 | none | PASS |
| the reference shot with Select Color (1), in a run frame 239, Full | 2 / 2 / 2 | 0 | 0 | none | PASS |
| the reference shot with Select Color (1), in a run frame 0, Draft | 2 / 2 / 2 | 0 | 0 | none | PASS |
| the reference shot with Select Color (1), in a run frame 100, Draft | 2 / 2 / 2 | 0 | 0 | none | PASS |
| the reference shot with Select Color (1), in a run frame 239, Draft | 2 / 2 / 2 | 0 | 0 | none | PASS |
| the reference shot with Select Color (2) frame 0, Full | 1 / 1 / 1 | 1 | 3138 | none | PASS |
| the reference shot with Select Color (2) frame 100, Full | 1 / 1 / 1 | 1 | 185 | none | PASS |
| the reference shot with Select Color (2) frame 239, Full | 1 / 1 / 1 | 1 | 1044 | none | PASS |
| the reference shot with Select Color (2) frame 0, Draft | 1 / 1 / 1 | 1 | 120 | none | PASS |
| the reference shot with Select Color (2) frame 100, Draft | 1 / 1 / 1 | 1 | 24 | none | PASS |
| the reference shot with Select Color (2) frame 239, Draft | 1 / 1 / 1 | 1 | 39 | none | PASS |
| the reference shot with Select Color (2), in a run frame 0, Full | 2 / 2 / 2 | 0 | 0 | none | PASS |
| the reference shot with Select Color (2), in a run frame 100, Full | 2 / 2 / 2 | 1 | 1 | none | PASS |
| the reference shot with Select Color (2), in a run frame 239, Full | 2 / 2 / 2 | 1 | 1 | none | PASS |
| the reference shot with Select Color (2), in a run frame 0, Draft | 2 / 2 / 2 | 1 | 36 | none | PASS |
| the reference shot with Select Color (2), in a run frame 100, Draft | 2 / 2 / 2 | 1 | 31 | none | PASS |
| the reference shot with Select Color (2), in a run frame 239, Draft | 2 / 2 / 2 | 1 | 37 | none | PASS |
| the reference shot with Line Recolor (1) frame 0, Full | 1 / 1 / 1 | 1 | 3783 | none | PASS |
| the reference shot with Line Recolor (1) frame 100, Full | 1 / 1 / 1 | 1 | 1417 | none | PASS |
| the reference shot with Line Recolor (1) frame 239, Full | 1 / 1 / 1 | 1 | 1772 | none | PASS |
| the reference shot with Line Recolor (1) frame 0, Draft | 1 / 1 / 1 | 1 | 128 | none | PASS |
| the reference shot with Line Recolor (1) frame 100, Draft | 1 / 1 / 1 | 1 | 32 | none | PASS |
| the reference shot with Line Recolor (1) frame 239, Draft | 1 / 1 / 1 | 1 | 47 | none | PASS |
| the reference shot with Line Recolor (1), in a run frame 0, Full | 2 / 2 / 2 | 1 | 1 | none | PASS |
| the reference shot with Line Recolor (1), in a run frame 100, Full | 2 / 2 / 2 | 1 | 1 | none | PASS |
| the reference shot with Line Recolor (1), in a run frame 239, Full | 2 / 2 / 2 | 1 | 1 | none | PASS |
| the reference shot with Line Recolor (1), in a run frame 0, Draft | 2 / 2 / 2 | 1 | 41 | none | PASS |
| the reference shot with Line Recolor (1), in a run frame 100, Draft | 2 / 2 / 2 | 1 | 56 | none | PASS |
| the reference shot with Line Recolor (1), in a run frame 239, Draft | 2 / 2 / 2 | 1 | 46 | none | PASS |
| the reference shot with Colorama (1) frame 0, Full | 1 / 2 / 1 | 1 | 814 | none | PASS |
| the reference shot with Colorama (1) frame 100, Full | 1 / 2 / 1 | 1 | 990 | none | PASS |
| the reference shot with Colorama (1) frame 239, Full | 1 / 2 / 1 | 1 | 958 | none | PASS |
| the reference shot with Colorama (1) frame 0, Draft | 1 / 2 / 1 | 1 | 69 | none | PASS |
| the reference shot with Colorama (1) frame 100, Draft | 1 / 2 / 1 | 1 | 60 | none | PASS |
| the reference shot with Colorama (1) frame 239, Draft | 1 / 2 / 1 | 1 | 84 | none | PASS |
| the reference shot with Colorama (1), in a run frame 0, Full | 3 / 4 / 3 | 1 | 736 | none | PASS |
| the reference shot with Colorama (1), in a run frame 100, Full | 3 / 4 / 3 | 1 | 907 | none | PASS |
| the reference shot with Colorama (1), in a run frame 239, Full | 3 / 4 / 3 | 1 | 870 | none | PASS |
| the reference shot with Colorama (1), in a run frame 0, Draft | 3 / 4 / 3 | 1 | 43 | none | PASS |
| the reference shot with Colorama (1), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 56 | none | PASS |
| the reference shot with Colorama (1), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 44 | none | PASS |
| the reference shot with Colorama (2) frame 0, Full | 1 / 2 / 1 | 1 | 1568 | none | PASS |
| the reference shot with Colorama (2) frame 100, Full | 1 / 2 / 1 | 1 | 1180 | none | PASS |
| the reference shot with Colorama (2) frame 239, Full | 1 / 2 / 1 | 1 | 1470 | none | PASS |
| the reference shot with Colorama (2) frame 0, Draft | 1 / 2 / 1 | 1 | 87 | none | PASS |
| the reference shot with Colorama (2) frame 100, Draft | 1 / 2 / 1 | 1 | 55 | none | PASS |
| the reference shot with Colorama (2) frame 239, Draft | 1 / 2 / 1 | 1 | 86 | none | PASS |
| the reference shot with Colorama (2), in a run frame 0, Full | 3 / 4 / 3 | 1 | 2038 | none | PASS |
| the reference shot with Colorama (2), in a run frame 100, Full | 3 / 4 / 3 | 1 | 786 | none | PASS |
| the reference shot with Colorama (2), in a run frame 239, Full | 3 / 4 / 3 | 1 | 1074 | none | PASS |
| the reference shot with Colorama (2), in a run frame 0, Draft | 3 / 4 / 3 | 1 | 116 | none | PASS |
| the reference shot with Colorama (2), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 53 | none | PASS |
| the reference shot with Colorama (2), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 69 | none | PASS |
| the reference shot with Colorama (3) frame 0, Full | 1 / 2 / 1 | 1 | 195 | none | PASS |
| the reference shot with Colorama (3) frame 100, Full | 1 / 2 / 1 | 1 | 169 | none | PASS |
| the reference shot with Colorama (3) frame 239, Full | 1 / 2 / 1 | 1 | 173 | none | PASS |
| the reference shot with Colorama (3) frame 0, Draft | 1 / 2 / 1 | 1 | 33 | none | PASS |
| the reference shot with Colorama (3) frame 100, Draft | 1 / 2 / 1 | 1 | 34 | none | PASS |
| the reference shot with Colorama (3) frame 239, Draft | 1 / 2 / 1 | 1 | 33 | none | PASS |
| the reference shot with Colorama (3), in a run frame 0, Full | 3 / 4 / 3 | 1 | 1991 | none | PASS |
| the reference shot with Colorama (3), in a run frame 100, Full | 3 / 4 / 3 | 1 | 491 | none | PASS |
| the reference shot with Colorama (3), in a run frame 239, Full | 3 / 4 / 3 | 1 | 1648 | none | PASS |
| the reference shot with Colorama (3), in a run frame 0, Draft | 3 / 4 / 3 | 1 | 142 | none | PASS |
| the reference shot with Colorama (3), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 35 | none | PASS |
| the reference shot with Colorama (3), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 117 | none | PASS |
| the reference shot with Colorama (4) frame 0, Full | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Colorama (4) frame 100, Full | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Colorama (4) frame 239, Full | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Colorama (4) frame 0, Draft | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Colorama (4) frame 100, Draft | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Colorama (4) frame 239, Draft | 1 / 2 / 1 | 0 | 0 | none | PASS |
| the reference shot with Colorama (4), in a run frame 0, Full | 3 / 4 / 3 | 1 | 1 | none | PASS |
| the reference shot with Colorama (4), in a run frame 100, Full | 3 / 4 / 3 | 1 | 1 | none | PASS |
| the reference shot with Colorama (4), in a run frame 239, Full | 3 / 4 / 3 | 1 | 1 | none | PASS |
| the reference shot with Colorama (4), in a run frame 0, Draft | 3 / 4 / 3 | 0 | 0 | none | PASS |
| the reference shot with Colorama (4), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 1 | none | PASS |
| the reference shot with Colorama (4), in a run frame 239, Draft | 3 / 4 / 3 | 0 | 0 | none | PASS |
| the reference shot with Extract (1) frame 0, Full | 1 / 2 / 1 | 1 | 3899 | none | PASS |
| the reference shot with Extract (1) frame 100, Full | 1 / 2 / 1 | 1 | 1462 | none | PASS |
| the reference shot with Extract (1) frame 239, Full | 1 / 2 / 1 | 1 | 1883 | none | PASS |
| the reference shot with Extract (1) frame 0, Draft | 1 / 2 / 1 | 1 | 135 | none | PASS |
| the reference shot with Extract (1) frame 100, Draft | 1 / 2 / 1 | 1 | 34 | none | PASS |
| the reference shot with Extract (1) frame 239, Draft | 1 / 2 / 1 | 1 | 60 | none | PASS |
| the reference shot with Extract (1), in a run frame 0, Full | 3 / 4 / 3 | 1 | 380 | none | PASS |
| the reference shot with Extract (1), in a run frame 100, Full | 3 / 4 / 3 | 1 | 602 | none | PASS |
| the reference shot with Extract (1), in a run frame 239, Full | 3 / 4 / 3 | 1 | 441 | none | PASS |
| the reference shot with Extract (1), in a run frame 0, Draft | 3 / 4 / 3 | 1 | 49 | none | PASS |
| the reference shot with Extract (1), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 30 | none | PASS |
| the reference shot with Extract (1), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 43 | none | PASS |
| the reference shot with Extract (2) frame 0, Full | 1 / 2 / 1 | 1 | 677 | none | PASS |
| the reference shot with Extract (2) frame 100, Full | 1 / 2 / 1 | 1 | 1232 | none | PASS |
| the reference shot with Extract (2) frame 239, Full | 1 / 2 / 1 | 1 | 773 | none | PASS |
| the reference shot with Extract (2) frame 0, Draft | 1 / 2 / 1 | 1 | 10 | none | PASS |
| the reference shot with Extract (2) frame 100, Draft | 1 / 2 / 1 | 1 | 9 | none | PASS |
| the reference shot with Extract (2) frame 239, Draft | 1 / 2 / 1 | 1 | 9 | none | PASS |
| the reference shot with Extract (2), in a run frame 0, Full | 3 / 4 / 3 | 1 | 186 | none | PASS |
| the reference shot with Extract (2), in a run frame 100, Full | 3 / 4 / 3 | 1 | 326 | none | PASS |
| the reference shot with Extract (2), in a run frame 239, Full | 3 / 4 / 3 | 1 | 211 | none | PASS |
| the reference shot with Extract (2), in a run frame 0, Draft | 3 / 4 / 3 | 1 | 18 | none | PASS |
| the reference shot with Extract (2), in a run frame 100, Draft | 3 / 4 / 3 | 1 | 32 | none | PASS |
| the reference shot with Extract (2), in a run frame 239, Draft | 3 / 4 / 3 | 1 | 21 | none | PASS |
| the reference shot with Exposure (1), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Exposure (1), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Exposure (2), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Exposure (2), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Tint (1), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Tint (1), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Shift Channels (1), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Shift Channels (1), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Shift Channels (2), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Shift Channels (2), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Shift Channels (3), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Shift Channels (3), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Solid Composite (1), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Solid Composite (1), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Solid Composite (2), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Solid Composite (2), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Solid Composite (3), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Solid Composite (3), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Solid Composite (4), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Solid Composite (4), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Change to Color (1), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Change to Color (1), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Change to Color (2), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Change to Color (2), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Color Key (1), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 10 apart, 7 together | PASS |
| the reference shot with Color Key (1), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 10 apart, 7 together | PASS |
| the reference shot with Color Key (2), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 10 apart, 7 together | PASS |
| the reference shot with Color Key (2), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 10 apart, 7 together | PASS |
| the reference shot with Select Color (1), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 10 apart, 7 together | PASS |
| the reference shot with Select Color (1), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 10 apart, 7 together | PASS |
| the reference shot with Select Color (2), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 10 apart, 7 together | PASS |
| the reference shot with Select Color (2), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 10 apart, 7 together | PASS |
| the reference shot with Line Recolor (1), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 10 apart, 7 together | PASS |
| the reference shot with Line Recolor (1), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 10 apart, 7 together | PASS |
| the reference shot with Colorama (1), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Colorama (1), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Colorama (2), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Colorama (2), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Colorama (3), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Colorama (3), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Colorama (4), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Colorama (4), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Extract (1), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Extract (1), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Extract (2), in a run frame 100, Full: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
| the reference shot with Extract (2), in a run frame 100, Draft: one pass against a pass each | — | byte-identical | — | passes 16 apart, 10 together | PASS |
