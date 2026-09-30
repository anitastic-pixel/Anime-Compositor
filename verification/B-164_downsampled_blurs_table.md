# B-164: big blurs worked small on the card (D-235)

Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

The reference shot at frame 100, with the effect on its first three layers. "Factors" is D-235's rule for each of the effect's blurs (Bloom has four): 1 is the exact blur, 2, 4 or 8 the block the picture is shrunk by. "Small, expected" is the blurs the rule works small times the layers left to the card; "small, worked" is what the card did. The difference is in levels of 255 against the CPU's exact frame, as the page receives it; the limit is 1. Export never takes this shortcut.

**24 of 24 checks pass.**

Of the rows worked small, the furthest from the CPU is "Gaussian Blur 50, Full": largest difference 1 of 255, pixels differing: 112566. Its pictures are in `verification/B-164 pictures/`: `cpu.png`, `gpu.png`, and `difference x64.png`, each channel's difference times 64, so black where the two agree and a dark grey where they are 1 level apart.

| Case | Factors | Layers on the card | Small, expected | Small, worked | Largest difference | Pixels differing | Diagnostics | Result |
|---|---|---:|---:|---:|---:|---:|---|---|
| Gaussian Blur 20, Full | 2 | 3 | 3 | 3 | 1 | 26201 | none | PASS |
| Gaussian Blur 20, Draft | 1 | 3 | 0 | 0 | 1 | 858 | none | PASS |
| Gaussian Blur 50, Full | 8 | 3 | 3 | 3 | 1 | 112566 | none | PASS |
| Gaussian Blur 50, Draft | 2 | 3 | 3 | 3 | 1 | 3755 | none | PASS |
| Gaussian Blur 100, Full | 8 | 3 | 3 | 3 | 1 | 42988 | none | PASS |
| Gaussian Blur 100, Draft | 4 | 3 | 3 | 3 | 1 | 9039 | none | PASS |
| Gaussian Blur 200, Full | 8 | 3 | 3 | 3 | 1 | 19967 | none | PASS |
| Gaussian Blur 200, Draft | 8 | 3 | 3 | 3 | 1 | 10228 | none | PASS |
| Glow 20, Full | 1 | 3 | 0 | 0 | 1 | 845 | none | PASS |
| Glow 20, Draft | 1 | 3 | 0 | 0 | 1 | 53 | none | PASS |
| Glow 50, Full | 2 | 3 | 3 | 3 | 1 | 11267 | none | PASS |
| Glow 50, Draft | 1 | 3 | 0 | 0 | 1 | 40 | none | PASS |
| Glow 100, Full | 4 | 3 | 3 | 3 | 1 | 25272 | none | PASS |
| Glow 100, Draft | 1 | 3 | 0 | 0 | 1 | 48 | none | PASS |
| Glow 200, Full | 8 | 3 | 3 | 3 | 1 | 35882 | none | PASS |
| Glow 200, Draft | 2 | 3 | 3 | 3 | 1 | 1364 | none | PASS |
| Bloom 20, Full | 1/1/1/1 | 3 | 0 | 0 | 1 | 957 | none | PASS |
| Bloom 20, Draft | 1/1/1/1 | 3 | 0 | 0 | 1 | 54 | none | PASS |
| Bloom 50, Full | 2/1/1/1 | 3 | 3 | 3 | 1 | 3742 | none | PASS |
| Bloom 50, Draft | 1/1/1/1 | 3 | 0 | 0 | 1 | 51 | none | PASS |
| Bloom 100, Full | 4/2/1/1 | 3 | 6 | 6 | 1 | 7794 | none | PASS |
| Bloom 100, Draft | 1/1/1/1 | 3 | 0 | 0 | 1 | 53 | none | PASS |
| Bloom 200, Full | 8/4/2/1 | 3 | 9 | 9 | 1 | 12311 | none | PASS |
| Bloom 200, Draft | 2/1/1/1 | 3 | 3 | 3 | 1 | 476 | none | PASS |
