# B-52: Repeat Edge Pixels on the GPU against the CPU

Written by `tests/b52_gpu_edges.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU, with the layer's last blur, repeating its edges (D-109), done on the card. **The rule: no channel of any pixel more than 1 level of 255 apart**, the tolerance each of the three blurs already has there (D-103, D-106, D-107). FX-EDGES-009's first blur runs on the CPU, before the spin the card does. On every row both paths must give the same warnings.

**108 of 108 checks pass.**

The worst comparison is "the reference shot with a Directional Blur at 45 degrees, 60 long, edges repeat, on the background frame 0, Full": largest difference 1 of 255, pixels differing: 21543. Its pictures are in `verification/B-52 pictures/`: `card_cpu.png`, `card_gpu.png`, and `card_difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

| Case | Blurs left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---|---|
| fx_edges_001 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_001 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_001 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_001 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_001 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_001 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_001 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_001 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_001 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_001 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_002 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_002 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_002 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_002 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_002 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_002 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_002 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_002 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_002 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_002 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_003 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_003 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_003 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_003 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_003 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_003 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_003 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_003 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_003 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_003 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_004 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_004 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_004 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_004 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_004 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_004 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_004 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_004 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_004 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_004 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_005 frame 0, Full | 1 | 1 | 2 | none | PASS |
| fx_edges_005 frame 1, Full | 1 | 1 | 2 | none | PASS |
| fx_edges_005 frame 2, Full | 1 | 1 | 2 | none | PASS |
| fx_edges_005 frame 3, Full | 1 | 1 | 2 | none | PASS |
| fx_edges_005 frame 4, Full | 1 | 1 | 2 | none | PASS |
| fx_edges_005 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_005 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_005 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_005 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_005 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_006 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_006 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_007 frame 0, Full | 1 | 1 | 2 | none | PASS |
| fx_edges_007 frame 1, Full | 1 | 1 | 2 | none | PASS |
| fx_edges_007 frame 2, Full | 1 | 1 | 2 | none | PASS |
| fx_edges_007 frame 3, Full | 1 | 1 | 2 | none | PASS |
| fx_edges_007 frame 4, Full | 1 | 1 | 2 | none | PASS |
| fx_edges_007 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_007 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_007 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_007 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_007 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_008 frame 0, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_008 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_008 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_008 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_008 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_edges_008 frame 0, Draft | 1 | 1 | 1 | none | PASS |
| fx_edges_008 frame 1, Draft | 1 | 1 | 1 | none | PASS |
| fx_edges_008 frame 2, Draft | 1 | 1 | 1 | none | PASS |
| fx_edges_008 frame 3, Draft | 1 | 1 | 1 | none | PASS |
| fx_edges_008 frame 4, Draft | 1 | 1 | 1 | none | PASS |
| fx_edges_009 frame 0, Full | 1 | 1 | 2 | none | PASS |
| fx_edges_009 frame 1, Full | 1 | 1 | 2 | none | PASS |
| fx_edges_009 frame 2, Full | 1 | 1 | 2 | none | PASS |
| fx_edges_009 frame 3, Full | 1 | 1 | 2 | none | PASS |
| fx_edges_009 frame 4, Full | 1 | 1 | 2 | none | PASS |
| fx_edges_009 frame 0, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_009 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_009 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_009 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_edges_009 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| the reference shot with a Gaussian Blur of sigma 10, edges repeat, on the background frame 0, Full | 1 | 1 | 21321 | none | PASS |
| the reference shot with a Gaussian Blur of sigma 10, edges repeat, on the background frame 239, Full | 1 | 1 | 21154 | none | PASS |
| the reference shot with a Gaussian Blur of sigma 10, edges repeat, on the background frame 0, Draft | 1 | 1 | 968 | none | PASS |
| the reference shot with a Gaussian Blur of sigma 10, edges repeat, on the background frame 239, Draft | 1 | 1 | 940 | none | PASS |
| the reference shot with a Directional Blur at 45 degrees, 60 long, edges repeat, on the background frame 0, Full | 1 | 1 | 21543 | none | PASS |
| the reference shot with a Directional Blur at 45 degrees, 60 long, edges repeat, on the background frame 239, Full | 1 | 1 | 21332 | none | PASS |
| the reference shot with a Directional Blur at 45 degrees, 60 long, edges repeat, on the background frame 0, Draft | 1 | 1 | 1232 | none | PASS |
| the reference shot with a Directional Blur at 45 degrees, 60 long, edges repeat, on the background frame 239, Draft | 1 | 1 | 1204 | none | PASS |
| the reference shot with a Radial Blur, spin 30 about the middle, edges repeat, on the background frame 0, Full | 1 | 1 | 16606 | none | PASS |
| the reference shot with a Radial Blur, spin 30 about the middle, edges repeat, on the background frame 239, Full | 1 | 1 | 16462 | none | PASS |
| the reference shot with a Radial Blur, spin 30 about the middle, edges repeat, on the background frame 0, Draft | 1 | 1 | 778 | none | PASS |
| the reference shot with a Radial Blur, spin 30 about the middle, edges repeat, on the background frame 239, Draft | 1 | 1 | 782 | none | PASS |
| the reference shot with a Gaussian Blur of sigma 10, edges repeat, on the background frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with a Gaussian Blur of sigma 10, edges repeat, on the background frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with a Directional Blur at 45 degrees, 60 long, edges repeat, on the background frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with a Directional Blur at 45 degrees, 60 long, edges repeat, on the background frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with a Radial Blur, spin 30 about the middle, edges repeat, on the background frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with a Radial Blur, spin 30 about the middle, edges repeat, on the background frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
