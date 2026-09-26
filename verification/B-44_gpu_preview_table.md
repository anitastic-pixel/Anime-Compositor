# B-44: the GPU's picture against the CPU's

Written by `tests/b44_gpu_preview.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU. **The rule: no channel of any pixel more than 1 level of 255 apart** (D-100). Frames with an adjustment layer are drawn by the CPU (D-66) and must be byte-identical.

**105 of 105 checks pass.**

The worst comparison is "P-07's hard case: frame 100 turned 0.001 deg about a point 8,388,608 px off canvas": largest difference 1 of 255, pixels differing: 396. Its pictures are in `verification/B-44 pictures/`: `cpu (old).png`, `gpu (new).png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

| Case | Expected | Actual | Result |
|---|---|---|---|
| H-01 frame 0, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| H-01 frame 14, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| H-01 frame 100, Full | at most 1 level | largest difference 1 of 255, pixels differing: 1 | PASS |
| H-01 frame 239, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| H-01 frame 0, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 1 | PASS |
| H-01 frame 14, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| H-01 frame 100, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| H-01 frame 239, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 0, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 12, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 24, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 36, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 48, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 60, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 72, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 84, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 96, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 108, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 120, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 132, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 144, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 156, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 168, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 180, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 192, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 204, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 216, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 228, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 239, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 0, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 1 | PASS |
| the reference shot frame 12, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 24, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 1 | PASS |
| the reference shot frame 36, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 48, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 1 | PASS |
| the reference shot frame 60, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 72, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 1 | PASS |
| the reference shot frame 84, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 96, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 1 | PASS |
| the reference shot frame 108, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 120, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 1 | PASS |
| the reference shot frame 132, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 144, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 1 | PASS |
| the reference shot frame 156, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 168, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 1 | PASS |
| the reference shot frame 180, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 192, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 204, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 216, Draft | at most 1 level | largest difference 1 of 255, pixels differing: 1 | PASS |
| the reference shot frame 228, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the reference shot frame 239, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 0, Full | at most 1 level | largest difference 1 of 255, pixels differing: 2 | PASS |
| the ten-layer fixture frame 12, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5 | PASS |
| the ten-layer fixture frame 24, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 36, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5 | PASS |
| the ten-layer fixture frame 48, Full | at most 1 level | largest difference 1 of 255, pixels differing: 2 | PASS |
| the ten-layer fixture frame 60, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5 | PASS |
| the ten-layer fixture frame 72, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 84, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5 | PASS |
| the ten-layer fixture frame 96, Full | at most 1 level | largest difference 1 of 255, pixels differing: 2 | PASS |
| the ten-layer fixture frame 108, Full | at most 1 level | largest difference 1 of 255, pixels differing: 3 | PASS |
| the ten-layer fixture frame 120, Full | at most 1 level | largest difference 1 of 255, pixels differing: 2 | PASS |
| the ten-layer fixture frame 132, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5 | PASS |
| the ten-layer fixture frame 144, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 156, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5 | PASS |
| the ten-layer fixture frame 168, Full | at most 1 level | largest difference 1 of 255, pixels differing: 2 | PASS |
| the ten-layer fixture frame 180, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5 | PASS |
| the ten-layer fixture frame 192, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 204, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5 | PASS |
| the ten-layer fixture frame 216, Full | at most 1 level | largest difference 1 of 255, pixels differing: 2 | PASS |
| the ten-layer fixture frame 228, Full | at most 1 level | largest difference 1 of 255, pixels differing: 5 | PASS |
| the ten-layer fixture frame 239, Full | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 0, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 12, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 24, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 36, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 48, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 60, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 72, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 84, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 96, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 108, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 120, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 132, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 144, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 156, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 168, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 180, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 192, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 204, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 216, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 228, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| the ten-layer fixture frame 239, Draft | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| P-07's hard case: frame 100 turned 0.001 deg about a point 8,388,608 px off canvas | at most 1 level | largest difference 1 of 255, pixels differing: 396 | PASS |
| ten-layer frame 100, Full, every layer normal, every other at 60% | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| ten-layer frame 100, Full, every layer multiply, every other at 60% | at most 1 level | largest difference 1 of 255, pixels differing: 2 | PASS |
| ten-layer frame 100, Full, every layer screen, every other at 60% | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| ten-layer frame 100, Full, every layer add, every other at 60% | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| ten-layer frame 100, Draft, every layer normal, every other at 60% | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| ten-layer frame 100, Draft, every layer multiply, every other at 60% | at most 1 level | largest difference 1 of 255, pixels differing: 1 | PASS |
| ten-layer frame 100, Draft, every layer screen, every other at 60% | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| ten-layer frame 100, Draft, every layer add, every other at 60% | at most 1 level | largest difference 0 of 255, pixels differing: 0 | PASS |
| ten-layer frame 100, Full, drawn twice on the GPU | the same bytes both times | the same bytes | PASS |
| the reference shot, every fifth frame at Draft, the card keeping one drawing at a time | at most 1 level | worst frame: largest difference 1 of 255, pixels differing: 1 | PASS |
| fx_adj_001 frame 0, Full: an adjustment layer, so the CPU draws it | the frame log says so, and the picture is the CPU's byte for byte | GPU_PREVIEW_ON_CPU: The CPU drew this frame: it has an adjustment layer, which the GPU does not draw yet.; byte-identical | PASS |
| fx_adj_001 frame 0, Draft: an adjustment layer, so the CPU draws it | the frame log says so, and the picture is the CPU's byte for byte | GPU_PREVIEW_ON_CPU: The CPU drew this frame: it has an adjustment layer, which the GPU does not draw yet.; byte-identical | PASS |
