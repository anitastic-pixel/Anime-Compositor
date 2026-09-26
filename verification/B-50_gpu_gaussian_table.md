# B-50: Gaussian Blur on the GPU against the CPU

Written by `tests/b50_gpu_gaussian.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU, with the layer's last Gaussian Blur done on the card. **The rule: no channel of any pixel more than 1 level of 255 apart** (D-107, proposed). A sigma too small to reach a neighbour changes nothing, and a blur on a composition layer stays on the CPU, so neither is left to the card; on those rows any difference is the card's layering, held to the same 1 level by D-100. On every row both paths must give the same warnings.

**24 of 24 checks pass.**

The worst comparison is "the reference shot with three Gaussian Blurs frame 0, Full": largest difference 1 of 255, pixels differing: 20915. Its pictures are in `verification/B-50 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

| Case | Blurs left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---|---|
| fx_fxk_006 frame 0, Full | 0 | 0 | 0 | none | PASS |
| fx_fxk_006 frame 1, Full | 1 | 0 | 0 | none | PASS |
| fx_fxk_006 frame 2, Full | 1 | 0 | 0 | none | PASS |
| fx_fxk_006 frame 3, Full | 1 | 0 | 0 | none | PASS |
| fx_fxk_006 frame 4, Full | 1 | 0 | 0 | none | PASS |
| fx_fxk_006 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_fxk_006 frame 1, Draft | 1 | 0 | 0 | none | PASS |
| fx_fxk_006 frame 2, Draft | 1 | 0 | 0 | none | PASS |
| fx_fxk_006 frame 3, Draft | 1 | 0 | 0 | none | PASS |
| fx_fxk_006 frame 4, Draft | 1 | 0 | 0 | none | PASS |
| fx_pre_002 frame 0, Full | 0 | 1 | 2 | none | PASS |
| fx_pre_002 frame 1, Full | 0 | 1 | 2 | none | PASS |
| fx_pre_002 frame 2, Full | 0 | 1 | 2 | none | PASS |
| fx_pre_002 frame 0, Draft | 0 | 0 | 0 | none | PASS |
| fx_pre_002 frame 1, Draft | 0 | 0 | 0 | none | PASS |
| fx_pre_002 frame 2, Draft | 0 | 0 | 0 | none | PASS |
| the reference shot with three Gaussian Blurs frame 0, Full | 3 | 1 | 20915 | none | PASS |
| the reference shot with three Gaussian Blurs frame 100, Full | 3 | 1 | 20512 | none | PASS |
| the reference shot with three Gaussian Blurs frame 239, Full | 3 | 1 | 20596 | none | PASS |
| the reference shot with three Gaussian Blurs frame 0, Draft | 3 | 1 | 952 | none | PASS |
| the reference shot with three Gaussian Blurs frame 100, Draft | 3 | 1 | 920 | none | PASS |
| the reference shot with three Gaussian Blurs frame 239, Draft | 3 | 1 | 924 | none | PASS |
| the reference shot with three Gaussian Blurs frame 100, Full: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
| the reference shot with three Gaussian Blurs frame 100, Draft: the plan made for the card, drawn by the CPU | — | — | byte-identical | — | PASS |
