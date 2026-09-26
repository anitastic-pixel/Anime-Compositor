# B-44: the GPU's picture against the CPU's

Written by `tests/b44_gpu_preview.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU. **The rule: no channel of any pixel more than 1 level of 255 apart** (D-100). Frames with an adjustment layer are drawn by the CPU (D-66) and must be byte-identical.

**107 of 107 checks pass.**

The worst comparison is "ten-layer frame 100, Draft, every layer multiply, every other at 60%": largest difference 1 of 255, pixels differing: 14780. Its pictures are in `verification/B-44 pictures/`: `cpu (old).png`, `gpu (new).png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

| Case | Expected | Actual | Result |
|---|---|---|---|
| H-01 frame 0, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4566 | PASS |
| H-01 frame 14, Full | at most 1 level | largest difference 1 of 255, pixels differing: 6350 | PASS |
| H-01 frame 100, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4939 | PASS |
| H-01 frame 239, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4780 | PASS |
| H-01 frame 0, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 13960 | PASS |
| H-01 frame 14, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14443 | PASS |
| H-01 frame 100, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14237 | PASS |
| H-01 frame 239, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14108 | PASS |
| the reference shot frame 0, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4566 | PASS |
| the reference shot frame 12, Full | at most 1 level | largest difference 1 of 255, pixels differing: 6079 | PASS |
| the reference shot frame 24, Full | at most 1 level | largest difference 1 of 255, pixels differing: 3184 | PASS |
| the reference shot frame 36, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5115 | PASS |
| the reference shot frame 48, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4493 | PASS |
| the reference shot frame 60, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4487 | PASS |
| the reference shot frame 72, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4994 | PASS |
| the reference shot frame 84, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4688 | PASS |
| the reference shot frame 96, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4634 | PASS |
| the reference shot frame 108, Full | at most 1 level | largest difference 1 of 255, pixels differing: 3570 | PASS |
| the reference shot frame 120, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4398 | PASS |
| the reference shot frame 132, Full | at most 1 level | largest difference 1 of 255, pixels differing: 6165 | PASS |
| the reference shot frame 144, Full | at most 1 level | largest difference 1 of 255, pixels differing: 3303 | PASS |
| the reference shot frame 156, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5115 | PASS |
| the reference shot frame 168, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4493 | PASS |
| the reference shot frame 180, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4487 | PASS |
| the reference shot frame 192, Full | at most 1 level | largest difference 1 of 255, pixels differing: 3872 | PASS |
| the reference shot frame 204, Full | at most 1 level | largest difference 1 of 255, pixels differing: 3915 | PASS |
| the reference shot frame 216, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4857 | PASS |
| the reference shot frame 228, Full | at most 1 level | largest difference 1 of 255, pixels differing: 3520 | PASS |
| the reference shot frame 239, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4780 | PASS |
| the reference shot frame 0, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 13960 | PASS |
| the reference shot frame 12, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14118 | PASS |
| the reference shot frame 24, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14016 | PASS |
| the reference shot frame 36, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14456 | PASS |
| the reference shot frame 48, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 13846 | PASS |
| the reference shot frame 60, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14471 | PASS |
| the reference shot frame 72, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14122 | PASS |
| the reference shot frame 84, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14239 | PASS |
| the reference shot frame 96, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14129 | PASS |
| the reference shot frame 108, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14487 | PASS |
| the reference shot frame 120, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 13888 | PASS |
| the reference shot frame 132, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14225 | PASS |
| the reference shot frame 144, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14152 | PASS |
| the reference shot frame 156, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14456 | PASS |
| the reference shot frame 168, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 13846 | PASS |
| the reference shot frame 180, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14471 | PASS |
| the reference shot frame 192, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14184 | PASS |
| the reference shot frame 204, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14244 | PASS |
| the reference shot frame 216, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14099 | PASS |
| the reference shot frame 228, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14385 | PASS |
| the reference shot frame 239, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14108 | PASS |
| the ten-layer fixture frame 0, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5110 | PASS |
| the ten-layer fixture frame 12, Full | at most 1 level | largest difference 1 of 255, pixels differing: 9249 | PASS |
| the ten-layer fixture frame 24, Full | at most 1 level | largest difference 1 of 255, pixels differing: 3275 | PASS |
| the ten-layer fixture frame 36, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4741 | PASS |
| the ten-layer fixture frame 48, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5878 | PASS |
| the ten-layer fixture frame 60, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4508 | PASS |
| the ten-layer fixture frame 72, Full | at most 1 level | largest difference 1 of 255, pixels differing: 9372 | PASS |
| the ten-layer fixture frame 84, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4612 | PASS |
| the ten-layer fixture frame 96, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4343 | PASS |
| the ten-layer fixture frame 108, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5020 | PASS |
| the ten-layer fixture frame 120, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5258 | PASS |
| the ten-layer fixture frame 132, Full | at most 1 level | largest difference 1 of 255, pixels differing: 10164 | PASS |
| the ten-layer fixture frame 144, Full | at most 1 level | largest difference 1 of 255, pixels differing: 3293 | PASS |
| the ten-layer fixture frame 156, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4741 | PASS |
| the ten-layer fixture frame 168, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5878 | PASS |
| the ten-layer fixture frame 180, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4508 | PASS |
| the ten-layer fixture frame 192, Full | at most 1 level | largest difference 1 of 255, pixels differing: 7697 | PASS |
| the ten-layer fixture frame 204, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4369 | PASS |
| the ten-layer fixture frame 216, Full | at most 1 level | largest difference 1 of 255, pixels differing: 4374 | PASS |
| the ten-layer fixture frame 228, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5072 | PASS |
| the ten-layer fixture frame 239, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5610 | PASS |
| the ten-layer fixture frame 0, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 13881 | PASS |
| the ten-layer fixture frame 12, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14220 | PASS |
| the ten-layer fixture frame 24, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 13907 | PASS |
| the ten-layer fixture frame 36, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14396 | PASS |
| the ten-layer fixture frame 48, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 13859 | PASS |
| the ten-layer fixture frame 60, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14395 | PASS |
| the ten-layer fixture frame 72, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14195 | PASS |
| the ten-layer fixture frame 84, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14137 | PASS |
| the ten-layer fixture frame 96, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14075 | PASS |
| the ten-layer fixture frame 108, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14428 | PASS |
| the ten-layer fixture frame 120, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 13853 | PASS |
| the ten-layer fixture frame 132, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14316 | PASS |
| the ten-layer fixture frame 144, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14033 | PASS |
| the ten-layer fixture frame 156, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14396 | PASS |
| the ten-layer fixture frame 168, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 13859 | PASS |
| the ten-layer fixture frame 180, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14395 | PASS |
| the ten-layer fixture frame 192, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14245 | PASS |
| the ten-layer fixture frame 204, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14116 | PASS |
| the ten-layer fixture frame 216, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14065 | PASS |
| the ten-layer fixture frame 228, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14320 | PASS |
| the ten-layer fixture frame 239, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 14072 | PASS |
| P-07's hard case: frame 100 turned 0.001 deg about a point 8,388,608 px off canvas | at most 1 level | largest difference 1 of 255, pixels differing: 11007 | PASS |
| ten-layer frame 100, Full, every layer normal, every other at 60% | at most 1 level | largest difference 1 of 255, pixels differing: 4628 | PASS |
| ten-layer frame 100, Full, every layer multiply, every other at 60% | at most 1 level | largest difference 1 of 255, pixels differing: 6067 | PASS |
| ten-layer frame 100, Full, every layer screen, every other at 60% | at most 1 level | largest difference 1 of 255, pixels differing: 3415 | PASS |
| ten-layer frame 100, Full, every layer add, every other at 60% | at most 1 level | largest difference 1 of 255, pixels differing: 3357 | PASS |
| ten-layer frame 100, Draft, every layer normal, every other at 60% | at most 1 level | largest difference 1 of 255, pixels differing: 14137 | PASS |
| ten-layer frame 100, Draft, every layer multiply, every other at 60% | at most 1 level | largest difference 1 of 255, pixels differing: 14780 | PASS |
| ten-layer frame 100, Draft, every layer screen, every other at 60% | at most 1 level | largest difference 1 of 255, pixels differing: 14266 | PASS |
| ten-layer frame 100, Draft, every layer add, every other at 60% | at most 1 level | largest difference 1 of 255, pixels differing: 14199 | PASS |
| ten-layer frame 100, Full, drawn twice on the GPU | the same bytes both times | the same bytes | PASS |
| the reference shot, every fifth frame at Draft, the card keeping 1 drawing at a time | at most 1 level | worst frame: largest difference 1 of 255, pixels differing: 14916 | PASS |
| the reference shot, every fifth frame at Full, the card keeping 3 drawings at a time | at most 1 level | worst frame: largest difference 1 of 255, pixels differing: 6404 | PASS |
| ten-layer frame 100, Full, read again into a new CPU cache after the first let go | no drawing sent again, and at most 1 level | drawings sent again: 0; largest difference 1 of 255 | PASS |
| fx_adj_001 frame 0, Full: an adjustment layer, so the CPU draws it | the frame log says so, and the picture is the CPU's byte for byte | GPU_PREVIEW_ON_CPU: The CPU drew this frame: it has an adjustment layer, which the GPU does not draw yet.; byte-identical | PASS |
| fx_adj_001 frame 0, Draft: an adjustment layer, so the CPU draws it | the frame log says so, and the picture is the CPU's byte for byte | GPU_PREVIEW_ON_CPU: The CPU drew this frame: it has an adjustment layer, which the GPU does not draw yet.; byte-identical | PASS |
