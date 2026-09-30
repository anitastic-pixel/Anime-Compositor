# B-156b: motion blur summed on the card

Written by `tests/b156b_gpu_motion_blur.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU, for every frame of every motion blur fixture and for the reference shot with motion blur on every layer. **The rule: the card draws the frame itself, adding up each motion-blurred layer's moments there, no channel of any pixel more than 1 level of 255 apart** (D-100), with the same warnings on both. Only a frame whose matte is motion-blurred may still be drawn by the CPU, and then it must be the CPU's picture exactly, the card's message `GPU_PREVIEW_ON_CPU` its only extra warning. Every frame of the reference shot with motion blur must be drawn on the card.

**44 of 175 checks pass.**

The worst comparison is "fx_mb_010 frame 0, Full": largest difference 0 of 255, pixels differing: 0. Its pictures are in `verification/B-156b pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

| Case | Drawn on | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---|---:|---:|---|---|
| fx_mb_010 frame 0, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_010 frame 1, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_010 frame 2, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_010 frame 0, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_010 frame 1, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_010 frame 2, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_011 frame 0, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_011 frame 1, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_011 frame 2, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_011 frame 0, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_011 frame 1, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_011 frame 2, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_012 frame 0, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_012 frame 1, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_012 frame 2, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_012 frame 0, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_012 frame 1, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_012 frame 2, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_013 frame 0, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_013 frame 1, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_013 frame 2, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_013 frame 0, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_013 frame 1, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_013 frame 2, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_014 frame 0, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_014 frame 1, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_014 frame 2, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_014 frame 0, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_014 frame 1, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_014 frame 2, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_015 frame 0, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_015 frame 1, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_015 frame 2, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_015 frame 0, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_015 frame 1, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_015 frame 2, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_016 frame 0, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_016 frame 1, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_016 frame 2, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_016 frame 0, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_016 frame 1, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_016 frame 2, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_017 frame 0, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_017 frame 1, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_017 frame 2, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_017 frame 0, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_017 frame 1, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_017 frame 2, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_018 frame 0, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_018 frame 1, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_018 frame 2, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_018 frame 0, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_018 frame 1, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_018 frame 2, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_019 frame 0, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_019 frame 1, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_019 frame 2, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_019 frame 0, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_019 frame 1, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_019 frame 2, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_020 frame 0, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_020 frame 1, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_020 frame 2, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_020 frame 0, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_020 frame 1, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_020 frame 2, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_021 frame 0, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_021 frame 1, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_021 frame 2, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_021 frame 0, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_021 frame 1, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_021 frame 2, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_022 frame 0, Full | GPU | 0 | 0 | none | PASS |
| fx_mb_022 frame 1, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_022 frame 2, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_022 frame 0, Draft | GPU | 0 | 0 | none | PASS |
| fx_mb_022 frame 1, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_022 frame 2, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_023 frame 0, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_023 frame 1, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_023 frame 2, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_023 frame 0, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_023 frame 1, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_023 frame 2, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_024 frame 0, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_024 frame 1, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_024 frame 2, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_024 frame 0, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_024 frame 1, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_024 frame 2, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_025 frame 0, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: a motion-blurred matte, so the CPU drew it |
| fx_mb_025 frame 1, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: a motion-blurred matte, so the CPU drew it |
| fx_mb_025 frame 2, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: a motion-blurred matte, so the CPU drew it |
| fx_mb_025 frame 0, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: a motion-blurred matte, so the CPU drew it |
| fx_mb_025 frame 1, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: a motion-blurred matte, so the CPU drew it |
| fx_mb_025 frame 2, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | PASS: a motion-blurred matte, so the CPU drew it |
| fx_mb_026 frame 0, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_026 frame 1, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_026 frame 2, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_026 frame 0, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_026 frame 1, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_026 frame 2, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_027 frame 0, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_027 frame 1, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_027 frame 2, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_027 frame 0, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_027 frame 1, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_027 frame 2, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_028 frame 0, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_028 frame 1, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_028 frame 2, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_028 frame 0, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_028 frame 1, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_028 frame 2, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 0, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 1, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 2, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 3, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 4, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 5, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 6, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 7, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 8, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 9, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 10, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 11, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 12, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 13, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 14, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 15, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 16, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 17, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 18, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 19, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 20, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 21, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 22, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 23, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 24, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 0, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 1, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 2, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 3, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 4, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 5, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 6, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 7, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 8, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 9, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 10, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 11, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 12, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 13, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 14, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 15, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 16, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 17, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 18, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 19, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 20, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 21, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 22, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 23, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| fx_mb_050 frame 24, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 0, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 50, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 100, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 150, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 239, Full | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 0, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 50, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 100, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 150, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur frame 239, Draft | CPU | 0 | 0 | CPU: ; GPU: GPU_PREVIEW_ON_CPU | FAIL: the CPU drew it |
| the reference shot with motion blur: frames drawn on the card | 0 of 10 | — | — | — | FAIL: not every frame |
