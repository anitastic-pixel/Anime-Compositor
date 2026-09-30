# B-152 before the build: the checks run on commit 9a55de2

The first run of `tests/b152_card_whole_frame.rs`, before the card was allowed to draw a frame with a motion-blurred, frame-mixed or dissolved layer. All 292 failing rows are those frames: the card still hands each of them whole to the CPU, which the check does not allow. Every other frame already passes.

The table as the test wrote it:

## B-152: the whole frame on the card, blurred and mixed layers included

Written by `tests/b152_card_whole_frame.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU. A motion-blurred, frame-mixed or dissolved layer is built by the CPU and laid by the card with the rest of the frame (D-218). **The rule: the card draws the frame itself, no channel of any pixel more than 1 level of 255 apart** (D-100), with the same warnings on both. A frame with an adjustment layer is still drawn by the CPU (B-44): that one must be the CPU's picture exactly, the card's message `GPU_PREVIEW_ON_CPU` its only extra warning. Each reference shot must have at least one frame with a blurred or mixed layer.

**590 of 882 checks pass.**

The worst comparison is "the reference shot with frame mix and dissolve frame 0, Draft": largest difference 1 of 255, pixels differing: 13960. Its pictures are in `verification/B-152 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## Each group

| Group | Frames compared | Frames with a blurred or mixed layer | Largest difference (of 255) | Pass |
|---|---:|---:|---:|---|
| Motion blur | 174 | 136 | 0 | 38 of 174 |
| Frame blending | 706 | 156 | 1 | 550 of 706 |

## Every frame

| Case | Blurred or mixed | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---|---:|---:|---|---|
| fx_mb_010 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_mb_010 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_mb_010 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_mb_010 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_mb_010 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_mb_010 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_mb_011 frame 0, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_011 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_011 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_011 frame 0, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_011 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_011 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_012 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_mb_012 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_mb_012 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_mb_012 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_mb_012 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_mb_012 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_mb_013 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_mb_013 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_mb_013 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_mb_013 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_mb_013 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_mb_013 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_mb_014 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_mb_014 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_mb_014 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_mb_014 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_mb_014 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_mb_014 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_mb_015 frame 0, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_015 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_015 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_mb_015 frame 0, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_015 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_015 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_mb_016 frame 0, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_016 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_016 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_016 frame 0, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_016 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_016 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_017 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_mb_017 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_mb_017 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_mb_017 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_mb_017 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_mb_017 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_mb_018 frame 0, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_018 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_018 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_018 frame 0, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_018 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_018 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_019 frame 0, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_019 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_019 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_019 frame 0, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_019 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_019 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_020 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_mb_020 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_020 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_mb_020 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_mb_020 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_020 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_mb_021 frame 0, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_021 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_021 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_021 frame 0, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_021 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_021 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_022 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_mb_022 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_022 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_022 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_mb_022 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_022 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_023 frame 0, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_023 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_023 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_023 frame 0, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_023 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_023 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_024 frame 0, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_024 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_024 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_024 frame 0, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_024 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_024 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_025 frame 0, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_025 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_025 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_025 frame 0, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_025 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_025 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_026 frame 0, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_026 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_026 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_026 frame 0, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_026 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_026 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_027 frame 0, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_027 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_027 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_027 frame 0, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_027 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_027 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_028 frame 0, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_028 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_028 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_028 frame 0, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_028 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_028 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 0, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 3, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 4, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 5, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 6, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 7, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 8, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 9, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 10, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 11, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 12, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 13, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 14, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 15, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 16, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 17, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 18, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 19, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 20, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 21, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 22, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 23, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 24, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 0, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 3, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 4, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 5, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 6, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 7, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 8, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 9, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 10, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 11, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 12, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 13, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 14, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 15, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 16, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 17, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 18, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 19, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 20, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 21, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 22, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 23, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 24, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_010 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 3, Full | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 3, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_010 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 3, Full | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 3, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_011 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_012 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_012 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_012 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_012 frame 3, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_012 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_012 frame 5, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_012 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_012 frame 7, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_012 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_012 frame 9, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_012 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_012 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_012 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_012 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_012 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_012 frame 3, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_012 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_012 frame 5, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_012 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_012 frame 7, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_012 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_012 frame 9, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_012 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_012 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 3, Full | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 3, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_013 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 3, Full | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 3, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_014 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_015 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_015 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_015 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_015 frame 3, Full | no | 0 | 0 | none | PASS |
| fx_fblend_015 frame 4, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_015 frame 5, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_015 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_015 frame 7, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_015 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_015 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_015 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_015 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_015 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_015 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_015 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_015 frame 3, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_015 frame 4, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_015 frame 5, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_015 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_015 frame 7, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_015 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_015 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_015 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_015 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_016 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_016 frame 3, Full | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_016 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_016 frame 3, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_016 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_017 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 3, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_017 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 7, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_017 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 9, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_017 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_017 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 3, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_017 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 7, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_017 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 9, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_017 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_017 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_018 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_018 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_018 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_018 frame 3, Full | yes | 0 | 0 | CPU: MEDIA_SEQUENCE_GAP; GPU: GPU_PREVIEW_ON_CPU, MEDIA_SEQUENCE_GAP | FAIL: the CPU drew it |
| fx_fblend_018 frame 4, Full | no | 0 | 0 | MEDIA_SEQUENCE_GAP, on both | PASS |
| fx_fblend_018 frame 5, Full | no | 0 | 0 | MEDIA_SEQUENCE_GAP, on both | PASS |
| fx_fblend_018 frame 6, Full | no | 0 | 0 | MEDIA_SEQUENCE_GAP, on both | PASS |
| fx_fblend_018 frame 7, Full | yes | 0 | 0 | CPU: MEDIA_SEQUENCE_GAP; GPU: GPU_PREVIEW_ON_CPU, MEDIA_SEQUENCE_GAP | FAIL: the CPU drew it |
| fx_fblend_018 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_018 frame 9, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_018 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_018 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_018 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_018 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_018 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_018 frame 3, Draft | yes | 0 | 0 | CPU: MEDIA_SEQUENCE_GAP; GPU: GPU_PREVIEW_ON_CPU, MEDIA_SEQUENCE_GAP | FAIL: the CPU drew it |
| fx_fblend_018 frame 4, Draft | no | 0 | 0 | MEDIA_SEQUENCE_GAP, on both | PASS |
| fx_fblend_018 frame 5, Draft | no | 0 | 0 | MEDIA_SEQUENCE_GAP, on both | PASS |
| fx_fblend_018 frame 6, Draft | no | 0 | 0 | MEDIA_SEQUENCE_GAP, on both | PASS |
| fx_fblend_018 frame 7, Draft | yes | 0 | 0 | CPU: MEDIA_SEQUENCE_GAP; GPU: GPU_PREVIEW_ON_CPU, MEDIA_SEQUENCE_GAP | FAIL: the CPU drew it |
| fx_fblend_018 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_018 frame 9, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_018 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_018 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_019 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 3, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_019 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 5, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_019 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 7, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_019 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_019 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 3, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_019 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 5, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_019 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 7, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_019 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_019 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 3, Full | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 4, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_020 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 6, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_020 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 8, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_020 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 10, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_020 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 3, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 4, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_020 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 6, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_020 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 8, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_020 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_020 frame 10, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_020 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_021 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_021 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_021 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_021 frame 3, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_021 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_021 frame 5, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_021 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_021 frame 7, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_021 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_021 frame 9, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_021 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_021 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_021 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_021 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_021 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_021 frame 3, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_021 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_021 frame 5, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_021 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_021 frame 7, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_021 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_021 frame 9, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_021 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_021 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_022 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_022 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_022 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_022 frame 3, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_022 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_022 frame 5, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_022 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_022 frame 7, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_022 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_022 frame 9, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_022 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_022 frame 11, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_022 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_022 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_022 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_022 frame 3, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_022 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_022 frame 5, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_022 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_022 frame 7, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_022 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_022 frame 9, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_022 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_022 frame 11, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_023 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_023 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_023 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_023 frame 3, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_023 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_023 frame 5, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_023 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_023 frame 7, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_023 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_023 frame 9, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_023 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_023 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_023 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_023 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_023 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_023 frame 3, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_023 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_023 frame 5, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_023 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_023 frame 7, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_023 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_023 frame 9, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_023 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_023 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 3, Full | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 3, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_024 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 3, Full | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 3, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_025 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_026 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_026 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_fblend_026 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_026 frame 3, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_026 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_026 frame 5, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_026 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_026 frame 7, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_026 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_026 frame 9, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_026 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_026 frame 11, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_026 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_026 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_026 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_026 frame 3, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_026 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_026 frame 5, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_026 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_026 frame 7, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_026 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_026 frame 9, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_026 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_026 frame 11, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_027 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 3, Full | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 3, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_027 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 3, Full | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 3, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_028 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_030 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 3, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_030 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_030 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 3, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_030 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_030 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_031 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_031 frame 3, Full | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 4, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_031 frame 5, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_031 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_031 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_031 frame 3, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 4, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_031 frame 5, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_031 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_031 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_032 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 3, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_032 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_032 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 3, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_032 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_032 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 3, Full | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 3, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_033 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 3, Full | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 3, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_034 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_035 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 3, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_035 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_035 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 3, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_035 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_035 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 1, Full | yes | 0 | 0 | CPU: MEDIA_SEQUENCE_GAP; GPU: GPU_PREVIEW_ON_CPU, MEDIA_SEQUENCE_GAP | FAIL: the CPU drew it |
| fx_fblend_036 frame 2, Full | no | 0 | 0 | MEDIA_SEQUENCE_GAP, on both | PASS |
| fx_fblend_036 frame 3, Full | yes | 0 | 0 | CPU: MEDIA_SEQUENCE_GAP; GPU: GPU_PREVIEW_ON_CPU, MEDIA_SEQUENCE_GAP | FAIL: the CPU drew it |
| fx_fblend_036 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 1, Draft | yes | 0 | 0 | CPU: MEDIA_SEQUENCE_GAP; GPU: GPU_PREVIEW_ON_CPU, MEDIA_SEQUENCE_GAP | FAIL: the CPU drew it |
| fx_fblend_036 frame 2, Draft | no | 0 | 0 | MEDIA_SEQUENCE_GAP, on both | PASS |
| fx_fblend_036 frame 3, Draft | yes | 0 | 0 | CPU: MEDIA_SEQUENCE_GAP; GPU: GPU_PREVIEW_ON_CPU, MEDIA_SEQUENCE_GAP | FAIL: the CPU drew it |
| fx_fblend_036 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_036 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_037 frame 2, Full | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 3, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_037 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 6, Full | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 7, Full | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_037 frame 2, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 3, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_037 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 6, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 7, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_037 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_038 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_038 frame 1, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_038 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_038 frame 3, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_038 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_038 frame 5, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_038 frame 6, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_038 frame 7, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_038 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_038 frame 9, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_038 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_038 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_038 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_038 frame 1, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_038 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_038 frame 3, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_038 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_038 frame 5, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_038 frame 6, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_038 frame 7, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_038 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_038 frame 9, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_038 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_038 frame 11, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 0, Full | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 1, Full | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 2, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_039 frame 3, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_039 frame 4, Full | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 5, Full | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 6, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_039 frame 7, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_039 frame 8, Full | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 9, Full | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 10, Full | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 11, Full | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 0, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 1, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 2, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_039 frame 3, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_039 frame 4, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 5, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 6, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_039 frame 7, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_fblend_039 frame 8, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 9, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 10, Draft | no | 0 | 0 | none | PASS |
| fx_fblend_039 frame 11, Draft | no | 0 | 0 | none | PASS |
| the reference shot with motion blur frame 0, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 50, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 100, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 150, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 239, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 0, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 50, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 100, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 150, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 239, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur: frames with a blurred or mixed layer | 10 | — | — | — | PASS |
| the reference shot with frame mix and dissolve frame 0, Full | no | 1 | 4566 | none | PASS |
| the reference shot with frame mix and dissolve frame 50, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with frame mix and dissolve frame 100, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with frame mix and dissolve frame 150, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with frame mix and dissolve frame 239, Full | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with frame mix and dissolve frame 0, Draft | no | 1 | 13960 | none | PASS |
| the reference shot with frame mix and dissolve frame 50, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with frame mix and dissolve frame 100, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with frame mix and dissolve frame 150, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with frame mix and dissolve frame 239, Draft | yes | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with frame mix and dissolve: frames with a blurred or mixed layer | 8 | — | — | — | PASS |
