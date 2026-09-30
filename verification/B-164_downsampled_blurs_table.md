# B-164: big blurs worked small on the card (D-235)

Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

The reference shot at frame 100, with the effect on its first three layers. "Factors" is D-235's rule for each of the effect's blurs (Bloom has four): 1 is the exact blur, 2, 4 or 8 the block the picture is shrunk by. "Small, expected" is the blurs the rule works small times the layers left to the card; "small, worked" is what the card did. The difference is in levels of 255 against the CPU's exact frame, as the page receives it; the limit is 1. Export never takes this shortcut.

**4 of 24 checks pass.**

| Case | Factors | Layers on the card | Small, expected | Small, worked | Largest difference | Pixels differing | Diagnostics | Result |
|---|---|---:|---:|---:|---:|---:|---|---|
| Gaussian Blur 20, Full | 2 | 3 | 3 | 0 | 1 | 19799 | none | FAIL |
| Gaussian Blur 20, Draft | 2 | 3 | 3 | 0 | 1 | 858 | none | FAIL |
| Gaussian Blur 50, Full | 8 | 3 | 3 | 0 | 1 | 16751 | none | FAIL |
| Gaussian Blur 50, Draft | 8 | 3 | 3 | 0 | 1 | 731 | none | FAIL |
| Gaussian Blur 100, Full | 8 | 3 | 3 | 0 | 1 | 14952 | none | FAIL |
| Gaussian Blur 100, Draft | 8 | 3 | 3 | 0 | 1 | 701 | none | FAIL |
| Gaussian Blur 200, Full | 8 | 3 | 3 | 0 | 1 | 13397 | none | FAIL |
| Gaussian Blur 200, Draft | 8 | 3 | 3 | 0 | 1 | 557 | none | FAIL |
| Glow 20, Full | 1 | 3 | 0 | 0 | 1 | 845 | none | PASS |
| Glow 20, Draft | 1 | 3 | 0 | 0 | 1 | 53 | none | PASS |
| Glow 50, Full | 2 | 3 | 3 | 0 | 1 | 866 | none | FAIL |
| Glow 50, Draft | 2 | 3 | 3 | 0 | 1 | 40 | none | FAIL |
| Glow 100, Full | 4 | 3 | 3 | 0 | 1 | 923 | none | FAIL |
| Glow 100, Draft | 4 | 3 | 3 | 0 | 1 | 48 | none | FAIL |
| Glow 200, Full | 8 | 3 | 3 | 0 | 1 | 835 | none | FAIL |
| Glow 200, Draft | 8 | 3 | 3 | 0 | 1 | 60 | none | FAIL |
| Bloom 20, Full | 1/1/1/1 | 3 | 0 | 0 | 1 | 957 | none | PASS |
| Bloom 20, Draft | 1/1/1/1 | 3 | 0 | 0 | 1 | 54 | none | PASS |
| Bloom 50, Full | 2/1/1/1 | 3 | 3 | 0 | 1 | 992 | none | FAIL |
| Bloom 50, Draft | 2/1/1/1 | 3 | 3 | 0 | 1 | 51 | none | FAIL |
| Bloom 100, Full | 4/2/1/1 | 3 | 6 | 0 | 1 | 941 | none | FAIL |
| Bloom 100, Draft | 4/2/1/1 | 3 | 6 | 0 | 1 | 53 | none | FAIL |
| Bloom 200, Full | 8/4/2/1 | 3 | 9 | 0 | 1 | 924 | none | FAIL |
| Bloom 200, Draft | 8/4/2/1 | 3 | 9 | 0 | 1 | 81 | none | FAIL |
