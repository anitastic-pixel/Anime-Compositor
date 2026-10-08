# B-221: effects mixed below 100 on the GPU against the CPU

Written by `tests/b221_gpu_mix.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

An effect's Mix lays its result back over what it was given (D-202). Until B-221 a layer with an effect mixed below 100 was drawn by the CPU from that effect on; now the card keeps what the effect was given, runs the effect and mixes, in one small pass, or inside the pass of a run of colour effects drawn together (B-172) (D-340).

Each of the 69 effects the card can draw is put last on the reference shot's first three layers, at Mix 0, 37 and 100. Then three more shots: a run of colour effects drawn in one pass with mixed members, growing effects (Glow, Gaussian Blur, Drop Shadow, Directional Blur, Outline, Lens Blur, Motion Tile) mixed, and an adjustment layer with mixed effects.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU, at frames 0 and 100, Full and Draft. **The rule: no channel of any pixel more than 1 level of 255 apart**, the same warnings on both, the card drawing the frame itself, at Full the first layer's whole run on the card, and at Full the card's plan holding the Mix exactly when an effect is mixed below 100.

**1268 of 1268 checks pass.**

The runs with mixed members drawn in one pass match the same runs drawn a pass each, byte for byte, in 8 of 8. The CPU drawing each plan made for the card, as it does when the card refuses a frame, draws the plan made for the CPU byte for byte in 420 of 420 (frame 100, Full and Draft). A failing one is listed below.

The worst comparison is "Gradient Map at Mix 100 frame 100, Full": largest difference 1 of 255, pixels differing: 2072. Its pictures are in `verification/B-221 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## By Mix and shot

| Shot | Frames compared | Largest difference (of 255) | Pass |
|---|---:|---:|---|
| a Glow at 50, a Gaussian Blur at 37, then Levels at 37 (growing effects mixed) | 4 | 1 | 4 of 4 |
| an adjustment layer: Gaussian Blur 37, Levels 50, Curves 37, Hue/Saturation | 4 | 1 | 4 of 4 |
| colour effects in one pass, three of them mixed (Hue/Saturation 37, Vibrance 0, Invert 50) | 4 | 1 | 4 of 4 |
| every card effect at Mix 0 | 276 | 1 | 276 of 276 |
| every card effect at Mix 100 | 276 | 1 | 276 of 276 |
| every card effect at Mix 37 | 276 | 1 | 276 of 276 |

## Every frame

Effects left to the card on the first three layers, the number the first layer must have at Full, and whether the card's plan holds a Mix.

| Case | Left to the card | First layer must have | Mix on the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---|---:|---:|---|---|
| Radial Blur at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Radial Blur at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Radial Blur at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Radial Blur at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Radial Blur at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1172 | none | PASS |
| Radial Blur at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 1032 | none | PASS |
| Radial Blur at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 79 | none | PASS |
| Radial Blur at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 68 | none | PASS |
| Radial Blur at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1135 | none | PASS |
| Radial Blur at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1095 | none | PASS |
| Radial Blur at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 58 | none | PASS |
| Radial Blur at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 62 | none | PASS |
| Bloom at Mix 0 frame 0, Full | 1 / 1 / 1 | 1 | yes | 1 | 964 | none | PASS |
| Bloom at Mix 0 frame 100, Full | 1 / 1 / 1 | 1 | yes | 1 | 981 | none | PASS |
| Bloom at Mix 0 frame 0, Draft | 1 / 1 / 1 | — | yes | 1 | 55 | none | PASS |
| Bloom at Mix 0 frame 100, Draft | 1 / 1 / 1 | — | yes | 1 | 54 | none | PASS |
| Bloom at Mix 37 frame 0, Full | 1 / 1 / 1 | 1 | yes | 1 | 1093 | none | PASS |
| Bloom at Mix 37 frame 100, Full | 1 / 1 / 1 | 1 | yes | 1 | 1164 | none | PASS |
| Bloom at Mix 37 frame 0, Draft | 1 / 1 / 1 | — | yes | 1 | 68 | none | PASS |
| Bloom at Mix 37 frame 100, Draft | 1 / 1 / 1 | — | yes | 1 | 55 | none | PASS |
| Bloom at Mix 100 frame 0, Full | 1 / 1 / 1 | 1 | no | 1 | 987 | none | PASS |
| Bloom at Mix 100 frame 100, Full | 1 / 1 / 1 | 1 | no | 1 | 723 | none | PASS |
| Bloom at Mix 100 frame 0, Draft | 1 / 1 / 1 | — | no | 1 | 65 | none | PASS |
| Bloom at Mix 100 frame 100, Draft | 1 / 1 / 1 | — | no | 1 | 37 | none | PASS |
| Directional Blur at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Directional Blur at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Directional Blur at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Directional Blur at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Directional Blur at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1040 | none | PASS |
| Directional Blur at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 1004 | none | PASS |
| Directional Blur at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 58 | none | PASS |
| Directional Blur at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 63 | none | PASS |
| Directional Blur at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1103 | none | PASS |
| Directional Blur at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1057 | none | PASS |
| Directional Blur at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 78 | none | PASS |
| Directional Blur at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 88 | none | PASS |
| Gaussian Blur at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Gaussian Blur at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Gaussian Blur at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Gaussian Blur at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Gaussian Blur at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1035 | none | PASS |
| Gaussian Blur at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 999 | none | PASS |
| Gaussian Blur at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 58 | none | PASS |
| Gaussian Blur at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 64 | none | PASS |
| Gaussian Blur at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1041 | none | PASS |
| Gaussian Blur at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 995 | none | PASS |
| Gaussian Blur at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 65 | none | PASS |
| Gaussian Blur at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 67 | none | PASS |
| Glow at Mix 0 frame 0, Full | 1 / 1 / 1 | 1 | yes | 1 | 964 | none | PASS |
| Glow at Mix 0 frame 100, Full | 1 / 1 / 1 | 1 | yes | 1 | 981 | none | PASS |
| Glow at Mix 0 frame 0, Draft | 1 / 1 / 1 | — | yes | 1 | 55 | none | PASS |
| Glow at Mix 0 frame 100, Draft | 1 / 1 / 1 | — | yes | 1 | 54 | none | PASS |
| Glow at Mix 37 frame 0, Full | 1 / 1 / 1 | 1 | yes | 1 | 794 | none | PASS |
| Glow at Mix 37 frame 100, Full | 1 / 1 / 1 | 1 | yes | 1 | 866 | none | PASS |
| Glow at Mix 37 frame 0, Draft | 1 / 1 / 1 | — | yes | 1 | 48 | none | PASS |
| Glow at Mix 37 frame 100, Draft | 1 / 1 / 1 | — | yes | 1 | 48 | none | PASS |
| Glow at Mix 100 frame 0, Full | 1 / 1 / 1 | 1 | no | 1 | 777 | none | PASS |
| Glow at Mix 100 frame 100, Full | 1 / 1 / 1 | 1 | no | 1 | 763 | none | PASS |
| Glow at Mix 100 frame 0, Draft | 1 / 1 / 1 | — | no | 1 | 48 | none | PASS |
| Glow at Mix 100 frame 100, Draft | 1 / 1 / 1 | — | no | 1 | 48 | none | PASS |
| Curves at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Curves at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Curves at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Curves at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Curves at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 838 | none | PASS |
| Curves at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 977 | none | PASS |
| Curves at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 42 | none | PASS |
| Curves at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 56 | none | PASS |
| Curves at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 771 | none | PASS |
| Curves at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 858 | none | PASS |
| Curves at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Curves at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 44 | none | PASS |
| Levels at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Levels at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Levels at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Levels at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Levels at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 939 | none | PASS |
| Levels at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 1032 | none | PASS |
| Levels at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 67 | none | PASS |
| Levels at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 60 | none | PASS |
| Levels at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 623 | none | PASS |
| Levels at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 744 | none | PASS |
| Levels at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 30 | none | PASS |
| Levels at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 37 | none | PASS |
| Hue/Saturation at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Hue/Saturation at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Hue/Saturation at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Hue/Saturation at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Hue/Saturation at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 849 | none | PASS |
| Hue/Saturation at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 958 | none | PASS |
| Hue/Saturation at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 58 | none | PASS |
| Hue/Saturation at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 63 | none | PASS |
| Hue/Saturation at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 951 | none | PASS |
| Hue/Saturation at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 939 | none | PASS |
| Hue/Saturation at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 50 | none | PASS |
| Hue/Saturation at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 53 | none | PASS |
| Gradient at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Gradient at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Gradient at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Gradient at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Gradient at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1070 | none | PASS |
| Gradient at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 1080 | none | PASS |
| Gradient at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 68 | none | PASS |
| Gradient at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 65 | none | PASS |
| Gradient at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1293 | none | PASS |
| Gradient at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1155 | none | PASS |
| Gradient at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 91 | none | PASS |
| Gradient at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 80 | none | PASS |
| Drop Shadow at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Drop Shadow at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Drop Shadow at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Drop Shadow at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Drop Shadow at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 970 | none | PASS |
| Drop Shadow at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 984 | none | PASS |
| Drop Shadow at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 56 | none | PASS |
| Drop Shadow at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Drop Shadow at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 970 | none | PASS |
| Drop Shadow at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 985 | none | PASS |
| Drop Shadow at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Drop Shadow at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Lens Blur at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Lens Blur at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Lens Blur at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Lens Blur at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Lens Blur at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 864 | none | PASS |
| Lens Blur at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 966 | none | PASS |
| Lens Blur at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 57 | none | PASS |
| Lens Blur at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 63 | none | PASS |
| Lens Blur at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 627 | none | PASS |
| Lens Blur at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 795 | none | PASS |
| Lens Blur at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 44 | none | PASS |
| Lens Blur at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 49 | none | PASS |
| Rim Light at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Rim Light at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Rim Light at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Rim Light at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Rim Light at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1012 | none | PASS |
| Rim Light at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 1022 | none | PASS |
| Rim Light at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 59 | none | PASS |
| Rim Light at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Rim Light at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1055 | none | PASS |
| Rim Light at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1064 | none | PASS |
| Rim Light at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 58 | none | PASS |
| Rim Light at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 53 | none | PASS |
| Outline at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Outline at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Outline at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Outline at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Outline at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 981 | none | PASS |
| Outline at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 992 | none | PASS |
| Outline at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 59 | none | PASS |
| Outline at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Outline at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1002 | none | PASS |
| Outline at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1009 | none | PASS |
| Outline at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 60 | none | PASS |
| Outline at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Noise at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Noise at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Noise at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Noise at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Noise at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Noise at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 976 | none | PASS |
| Noise at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 62 | none | PASS |
| Noise at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 65 | none | PASS |
| Noise at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 954 | none | PASS |
| Noise at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 950 | none | PASS |
| Noise at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 62 | none | PASS |
| Noise at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 60 | none | PASS |
| Chromatic Aberration at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Chromatic Aberration at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Chromatic Aberration at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Chromatic Aberration at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Chromatic Aberration at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 976 | none | PASS |
| Chromatic Aberration at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 998 | none | PASS |
| Chromatic Aberration at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 64 | none | PASS |
| Chromatic Aberration at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 57 | none | PASS |
| Chromatic Aberration at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 947 | none | PASS |
| Chromatic Aberration at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 949 | none | PASS |
| Chromatic Aberration at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 53 | none | PASS |
| Chromatic Aberration at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 46 | none | PASS |
| Distance Gradation at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Distance Gradation at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Distance Gradation at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Distance Gradation at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Distance Gradation at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 973 | none | PASS |
| Distance Gradation at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 990 | none | PASS |
| Distance Gradation at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Distance Gradation at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Distance Gradation at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 973 | none | PASS |
| Distance Gradation at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 988 | none | PASS |
| Distance Gradation at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Distance Gradation at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 54 | none | PASS |
| Light Rays at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Light Rays at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Light Rays at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Light Rays at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Light Rays at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 927 | none | PASS |
| Light Rays at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 910 | none | PASS |
| Light Rays at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 47 | none | PASS |
| Light Rays at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 46 | none | PASS |
| Light Rays at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 974 | none | PASS |
| Light Rays at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 874 | none | PASS |
| Light Rays at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 63 | none | PASS |
| Light Rays at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 66 | none | PASS |
| Exposure Flicker at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Exposure Flicker at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Exposure Flicker at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Exposure Flicker at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Exposure Flicker at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 946 | none | PASS |
| Exposure Flicker at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 882 | none | PASS |
| Exposure Flicker at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 67 | none | PASS |
| Exposure Flicker at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 50 | none | PASS |
| Exposure Flicker at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 860 | none | PASS |
| Exposure Flicker at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 816 | none | PASS |
| Exposure Flicker at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 54 | none | PASS |
| Exposure Flicker at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 56 | none | PASS |
| Vignette at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Vignette at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Vignette at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Vignette at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Vignette at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1252 | none | PASS |
| Vignette at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 1003 | none | PASS |
| Vignette at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 72 | none | PASS |
| Vignette at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Vignette at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1159 | none | PASS |
| Vignette at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 994 | none | PASS |
| Vignette at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 79 | none | PASS |
| Vignette at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 58 | none | PASS |
| Turbulent Displace at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Turbulent Displace at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Turbulent Displace at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Turbulent Displace at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Turbulent Displace at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1017 | none | PASS |
| Turbulent Displace at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 985 | none | PASS |
| Turbulent Displace at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 74 | none | PASS |
| Turbulent Displace at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 56 | none | PASS |
| Turbulent Displace at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 971 | none | PASS |
| Turbulent Displace at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1003 | none | PASS |
| Turbulent Displace at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 54 | none | PASS |
| Turbulent Displace at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 49 | none | PASS |
| Fractal Noise at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Fractal Noise at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Fractal Noise at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Fractal Noise at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Fractal Noise at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 852 | none | PASS |
| Fractal Noise at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 963 | none | PASS |
| Fractal Noise at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 44 | none | PASS |
| Fractal Noise at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 70 | none | PASS |
| Fractal Noise at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1029 | none | PASS |
| Fractal Noise at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1031 | none | PASS |
| Fractal Noise at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 62 | none | PASS |
| Fractal Noise at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 53 | none | PASS |
| Gradient Map at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Gradient Map at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Gradient Map at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Gradient Map at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Gradient Map at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1063 | none | PASS |
| Gradient Map at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 1041 | none | PASS |
| Gradient Map at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Gradient Map at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 77 | none | PASS |
| Gradient Map at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1275 | none | PASS |
| Gradient Map at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 2072 | none | PASS |
| Gradient Map at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 79 | none | PASS |
| Gradient Map at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 157 | none | PASS |
| Color Balance at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Color Balance at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Color Balance at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Color Balance at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Color Balance at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Color Balance at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 951 | none | PASS |
| Color Balance at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Color Balance at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 57 | none | PASS |
| Color Balance at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1110 | none | PASS |
| Color Balance at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 945 | none | PASS |
| Color Balance at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 73 | none | PASS |
| Color Balance at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 69 | none | PASS |
| Offset at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Offset at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Offset at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Offset at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Offset at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1011 | none | PASS |
| Offset at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 962 | none | PASS |
| Offset at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 47 | none | PASS |
| Offset at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 70 | none | PASS |
| Offset at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 906 | none | PASS |
| Offset at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 880 | none | PASS |
| Offset at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 61 | none | PASS |
| Offset at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 50 | none | PASS |
| Invert at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Invert at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Invert at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Invert at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Invert at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1022 | none | PASS |
| Invert at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 965 | none | PASS |
| Invert at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 52 | none | PASS |
| Invert at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 50 | none | PASS |
| Invert at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1247 | none | PASS |
| Invert at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1461 | none | PASS |
| Invert at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 77 | none | PASS |
| Invert at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 73 | none | PASS |
| Brightness & Contrast at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Brightness & Contrast at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Brightness & Contrast at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Brightness & Contrast at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Brightness & Contrast at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 806 | none | PASS |
| Brightness & Contrast at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 802 | none | PASS |
| Brightness & Contrast at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Brightness & Contrast at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 51 | none | PASS |
| Brightness & Contrast at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 363 | none | PASS |
| Brightness & Contrast at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 217 | none | PASS |
| Brightness & Contrast at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 29 | none | PASS |
| Brightness & Contrast at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 5 | none | PASS |
| Black & White at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Black & White at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Black & White at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Black & White at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Black & White at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1147 | none | PASS |
| Black & White at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 967 | none | PASS |
| Black & White at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 56 | none | PASS |
| Black & White at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 53 | none | PASS |
| Black & White at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1266 | none | PASS |
| Black & White at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 779 | none | PASS |
| Black & White at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 86 | none | PASS |
| Black & White at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 56 | none | PASS |
| Posterize at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Posterize at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Posterize at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Posterize at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Posterize at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 848 | none | PASS |
| Posterize at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 800 | none | PASS |
| Posterize at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 52 | none | PASS |
| Posterize at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Posterize at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1 | none | PASS |
| Posterize at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 3 | none | PASS |
| Posterize at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 0 | 0 | none | PASS |
| Posterize at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 0 | 0 | none | PASS |
| Threshold at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Threshold at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Threshold at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Threshold at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Threshold at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 816 | none | PASS |
| Threshold at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 861 | none | PASS |
| Threshold at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Threshold at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Threshold at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 0 | 0 | none | PASS |
| Threshold at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 0 | 0 | none | PASS |
| Threshold at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 0 | 0 | none | PASS |
| Threshold at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 0 | 0 | none | PASS |
| Channel Mixer at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Channel Mixer at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Channel Mixer at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Channel Mixer at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Channel Mixer at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1139 | none | PASS |
| Channel Mixer at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 973 | none | PASS |
| Channel Mixer at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 62 | none | PASS |
| Channel Mixer at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 60 | none | PASS |
| Channel Mixer at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1242 | none | PASS |
| Channel Mixer at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1426 | none | PASS |
| Channel Mixer at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 78 | none | PASS |
| Channel Mixer at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 87 | none | PASS |
| Vibrance at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Vibrance at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Vibrance at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Vibrance at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Vibrance at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 852 | none | PASS |
| Vibrance at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 932 | none | PASS |
| Vibrance at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 59 | none | PASS |
| Vibrance at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 58 | none | PASS |
| Vibrance at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 746 | none | PASS |
| Vibrance at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 793 | none | PASS |
| Vibrance at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 44 | none | PASS |
| Vibrance at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 52 | none | PASS |
| Leave Color at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Leave Color at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Leave Color at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Leave Color at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Leave Color at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1039 | none | PASS |
| Leave Color at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 953 | none | PASS |
| Leave Color at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 70 | none | PASS |
| Leave Color at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Leave Color at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1042 | none | PASS |
| Leave Color at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1030 | none | PASS |
| Leave Color at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 74 | none | PASS |
| Leave Color at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Solarize at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Solarize at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Solarize at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Solarize at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Solarize at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 856 | none | PASS |
| Solarize at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 916 | none | PASS |
| Solarize at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 48 | none | PASS |
| Solarize at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 65 | none | PASS |
| Solarize at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1604 | none | PASS |
| Solarize at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1533 | none | PASS |
| Solarize at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 92 | none | PASS |
| Solarize at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 104 | none | PASS |
| Halftone at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Halftone at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Halftone at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Halftone at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Halftone at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 844 | none | PASS |
| Halftone at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 913 | none | PASS |
| Halftone at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 46 | none | PASS |
| Halftone at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 58 | none | PASS |
| Halftone at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 0 | 0 | none | PASS |
| Halftone at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 0 | 0 | none | PASS |
| Halftone at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 0 | 0 | none | PASS |
| Halftone at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 0 | 0 | none | PASS |
| Mosaic at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Mosaic at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Mosaic at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Mosaic at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Mosaic at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1029 | none | PASS |
| Mosaic at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 952 | none | PASS |
| Mosaic at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 60 | none | PASS |
| Mosaic at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 57 | none | PASS |
| Mosaic at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 600 | none | PASS |
| Mosaic at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 610 | none | PASS |
| Mosaic at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 82 | none | PASS |
| Mosaic at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 42 | none | PASS |
| Emboss at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Emboss at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Emboss at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Emboss at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Emboss at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1031 | none | PASS |
| Emboss at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 1020 | none | PASS |
| Emboss at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 64 | none | PASS |
| Emboss at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 59 | none | PASS |
| Emboss at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1519 | none | PASS |
| Emboss at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1527 | none | PASS |
| Emboss at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 1966 | none | PASS |
| Emboss at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 1907 | none | PASS |
| Find Edges at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Find Edges at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Find Edges at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Find Edges at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Find Edges at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 865 | none | PASS |
| Find Edges at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 918 | none | PASS |
| Find Edges at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 64 | none | PASS |
| Find Edges at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 53 | none | PASS |
| Find Edges at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 759 | none | PASS |
| Find Edges at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 813 | none | PASS |
| Find Edges at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 61 | none | PASS |
| Find Edges at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 60 | none | PASS |
| Sharpen at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Sharpen at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Sharpen at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Sharpen at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Sharpen at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 937 | none | PASS |
| Sharpen at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 931 | none | PASS |
| Sharpen at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 53 | none | PASS |
| Sharpen at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 50 | none | PASS |
| Sharpen at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 967 | none | PASS |
| Sharpen at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 910 | none | PASS |
| Sharpen at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 54 | none | PASS |
| Sharpen at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 53 | none | PASS |
| Diffusion at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Diffusion at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Diffusion at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Diffusion at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Diffusion at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 809 | none | PASS |
| Diffusion at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 905 | none | PASS |
| Diffusion at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 45 | none | PASS |
| Diffusion at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 49 | none | PASS |
| Diffusion at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 862 | none | PASS |
| Diffusion at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1262 | none | PASS |
| Diffusion at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 43 | none | PASS |
| Diffusion at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 77 | none | PASS |
| Wave Warp at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Wave Warp at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Wave Warp at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Wave Warp at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Wave Warp at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 990 | none | PASS |
| Wave Warp at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 930 | none | PASS |
| Wave Warp at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Wave Warp at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Wave Warp at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1003 | none | PASS |
| Wave Warp at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 935 | none | PASS |
| Wave Warp at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 80 | none | PASS |
| Wave Warp at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 59 | none | PASS |
| Ripple at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Ripple at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Ripple at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Ripple at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Ripple at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Ripple at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 922 | none | PASS |
| Ripple at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 75 | none | PASS |
| Ripple at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 66 | none | PASS |
| Ripple at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 968 | none | PASS |
| Ripple at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 955 | none | PASS |
| Ripple at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 72 | none | PASS |
| Ripple at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 64 | none | PASS |
| Twirl at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Twirl at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Twirl at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Twirl at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Twirl at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 977 | none | PASS |
| Twirl at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 990 | none | PASS |
| Twirl at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Twirl at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Twirl at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 957 | none | PASS |
| Twirl at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 986 | none | PASS |
| Twirl at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Twirl at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Bulge at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Bulge at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Bulge at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Bulge at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Bulge at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 960 | none | PASS |
| Bulge at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 986 | none | PASS |
| Bulge at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Bulge at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Bulge at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 955 | none | PASS |
| Bulge at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 987 | none | PASS |
| Bulge at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 56 | none | PASS |
| Bulge at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Mirror at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Mirror at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Mirror at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Mirror at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Mirror at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1034 | none | PASS |
| Mirror at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 1017 | none | PASS |
| Mirror at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 63 | none | PASS |
| Mirror at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 50 | none | PASS |
| Mirror at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1004 | none | PASS |
| Mirror at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1014 | none | PASS |
| Mirror at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 62 | none | PASS |
| Mirror at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 54 | none | PASS |
| Linear Wipe at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Linear Wipe at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Linear Wipe at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Linear Wipe at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Linear Wipe at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 941 | none | PASS |
| Linear Wipe at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 1067 | none | PASS |
| Linear Wipe at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 64 | none | PASS |
| Linear Wipe at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 71 | none | PASS |
| Linear Wipe at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 503 | none | PASS |
| Linear Wipe at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 18 | none | PASS |
| Linear Wipe at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 31 | none | PASS |
| Linear Wipe at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 2 | none | PASS |
| Radial Wipe at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Radial Wipe at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Radial Wipe at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Radial Wipe at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Radial Wipe at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1059 | none | PASS |
| Radial Wipe at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 995 | none | PASS |
| Radial Wipe at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 47 | none | PASS |
| Radial Wipe at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 53 | none | PASS |
| Radial Wipe at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 472 | none | PASS |
| Radial Wipe at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 973 | none | PASS |
| Radial Wipe at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 24 | none | PASS |
| Radial Wipe at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 52 | none | PASS |
| Venetian Blinds at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Venetian Blinds at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Venetian Blinds at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Venetian Blinds at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Venetian Blinds at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1015 | none | PASS |
| Venetian Blinds at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 1042 | none | PASS |
| Venetian Blinds at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Venetian Blinds at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 61 | none | PASS |
| Venetian Blinds at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 500 | none | PASS |
| Venetian Blinds at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 512 | none | PASS |
| Venetian Blinds at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 33 | none | PASS |
| Venetian Blinds at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 33 | none | PASS |
| Iris Wipe at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Iris Wipe at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Iris Wipe at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Iris Wipe at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Iris Wipe at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 976 | none | PASS |
| Iris Wipe at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 990 | none | PASS |
| Iris Wipe at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 56 | none | PASS |
| Iris Wipe at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Iris Wipe at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 954 | none | PASS |
| Iris Wipe at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 970 | none | PASS |
| Iris Wipe at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 54 | none | PASS |
| Iris Wipe at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 54 | none | PASS |
| Simple Choker at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Simple Choker at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Simple Choker at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Simple Choker at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Simple Choker at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 976 | none | PASS |
| Simple Choker at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 992 | none | PASS |
| Simple Choker at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Simple Choker at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Simple Choker at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 974 | none | PASS |
| Simple Choker at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 989 | none | PASS |
| Simple Choker at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Simple Choker at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 54 | none | PASS |
| Speed Lines at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Speed Lines at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Speed Lines at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Speed Lines at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Speed Lines at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 949 | none | PASS |
| Speed Lines at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 987 | none | PASS |
| Speed Lines at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 58 | none | PASS |
| Speed Lines at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 64 | none | PASS |
| Speed Lines at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 787 | none | PASS |
| Speed Lines at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 667 | none | PASS |
| Speed Lines at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 57 | none | PASS |
| Speed Lines at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 60 | none | PASS |
| Cross Glare at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Cross Glare at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Cross Glare at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Cross Glare at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Cross Glare at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 555 | none | PASS |
| Cross Glare at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 524 | none | PASS |
| Cross Glare at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 34 | none | PASS |
| Cross Glare at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 46 | none | PASS |
| Cross Glare at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 457 | none | PASS |
| Cross Glare at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 190 | none | PASS |
| Cross Glare at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 32 | none | PASS |
| Cross Glare at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 17 | none | PASS |
| Camera Shake at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Camera Shake at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Camera Shake at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Camera Shake at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Camera Shake at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 986 | none | PASS |
| Camera Shake at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 1011 | none | PASS |
| Camera Shake at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Camera Shake at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Camera Shake at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 908 | none | PASS |
| Camera Shake at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 973 | none | PASS |
| Camera Shake at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 52 | none | PASS |
| Camera Shake at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 70 | none | PASS |
| Rain at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Rain at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Rain at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Rain at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Rain at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 971 | none | PASS |
| Rain at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 1004 | none | PASS |
| Rain at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 56 | none | PASS |
| Rain at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 61 | none | PASS |
| Rain at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 981 | none | PASS |
| Rain at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 996 | none | PASS |
| Rain at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 58 | none | PASS |
| Rain at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Motion Tile at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Motion Tile at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Motion Tile at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Motion Tile at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Motion Tile at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Motion Tile at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Motion Tile at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Motion Tile at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Motion Tile at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 975 | none | PASS |
| Motion Tile at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 991 | none | PASS |
| Motion Tile at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Motion Tile at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 54 | none | PASS |
| Color Lookup at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Color Lookup at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Color Lookup at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Color Lookup at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Color Lookup at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 835 | none | PASS |
| Color Lookup at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 890 | none | PASS |
| Color Lookup at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 60 | none | PASS |
| Color Lookup at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Color Lookup at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1032 | none | PASS |
| Color Lookup at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 933 | none | PASS |
| Color Lookup at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 54 | none | PASS |
| Color Lookup at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 64 | none | PASS |
| Line Blur at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Line Blur at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Line Blur at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Line Blur at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Line Blur at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 992 | none | PASS |
| Line Blur at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 973 | none | PASS |
| Line Blur at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 53 | none | PASS |
| Line Blur at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 53 | none | PASS |
| Line Blur at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1013 | none | PASS |
| Line Blur at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1007 | none | PASS |
| Line Blur at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 62 | none | PASS |
| Line Blur at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 53 | none | PASS |
| HSV Key at Mix 0 frame 0, Full | 1 / 1 / 1 | 1 | yes | 1 | 964 | none | PASS |
| HSV Key at Mix 0 frame 100, Full | 1 / 1 / 1 | 1 | yes | 1 | 981 | none | PASS |
| HSV Key at Mix 0 frame 0, Draft | 1 / 1 / 1 | — | yes | 1 | 55 | none | PASS |
| HSV Key at Mix 0 frame 100, Draft | 1 / 1 / 1 | — | yes | 1 | 54 | none | PASS |
| HSV Key at Mix 37 frame 0, Full | 1 / 1 / 1 | 1 | yes | 1 | 1028 | none | PASS |
| HSV Key at Mix 37 frame 100, Full | 1 / 1 / 1 | 1 | yes | 1 | 997 | none | PASS |
| HSV Key at Mix 37 frame 0, Draft | 1 / 1 / 1 | — | yes | 1 | 57 | none | PASS |
| HSV Key at Mix 37 frame 100, Draft | 1 / 1 / 1 | — | yes | 1 | 56 | none | PASS |
| HSV Key at Mix 100 frame 0, Full | 1 / 1 / 1 | 1 | no | 1 | 757 | none | PASS |
| HSV Key at Mix 100 frame 100, Full | 1 / 1 / 1 | 1 | no | 1 | 915 | none | PASS |
| HSV Key at Mix 100 frame 0, Draft | 1 / 1 / 1 | — | no | 1 | 45 | none | PASS |
| HSV Key at Mix 100 frame 100, Draft | 1 / 1 / 1 | — | no | 1 | 51 | none | PASS |
| Paraffin at Mix 0 frame 0, Full | 1 / 1 / 1 | 1 | yes | 1 | 964 | none | PASS |
| Paraffin at Mix 0 frame 100, Full | 1 / 1 / 1 | 1 | yes | 1 | 981 | none | PASS |
| Paraffin at Mix 0 frame 0, Draft | 1 / 1 / 1 | — | yes | 1 | 55 | none | PASS |
| Paraffin at Mix 0 frame 100, Draft | 1 / 1 / 1 | — | yes | 1 | 54 | none | PASS |
| Paraffin at Mix 37 frame 0, Full | 1 / 1 / 1 | 1 | yes | 1 | 903 | none | PASS |
| Paraffin at Mix 37 frame 100, Full | 1 / 1 / 1 | 1 | yes | 1 | 1007 | none | PASS |
| Paraffin at Mix 37 frame 0, Draft | 1 / 1 / 1 | — | yes | 1 | 57 | none | PASS |
| Paraffin at Mix 37 frame 100, Draft | 1 / 1 / 1 | — | yes | 1 | 60 | none | PASS |
| Paraffin at Mix 100 frame 0, Full | 1 / 1 / 1 | 1 | no | 1 | 908 | none | PASS |
| Paraffin at Mix 100 frame 100, Full | 1 / 1 / 1 | 1 | no | 1 | 999 | none | PASS |
| Paraffin at Mix 100 frame 0, Draft | 1 / 1 / 1 | — | no | 1 | 54 | none | PASS |
| Paraffin at Mix 100 frame 100, Draft | 1 / 1 / 1 | — | no | 1 | 64 | none | PASS |
| Kira-kira at Mix 0 frame 0, Full | 1 / 1 / 1 | 1 | yes | 1 | 964 | none | PASS |
| Kira-kira at Mix 0 frame 100, Full | 1 / 1 / 1 | 1 | yes | 1 | 981 | none | PASS |
| Kira-kira at Mix 0 frame 0, Draft | 1 / 1 / 1 | — | yes | 1 | 55 | none | PASS |
| Kira-kira at Mix 0 frame 100, Draft | 1 / 1 / 1 | — | yes | 1 | 54 | none | PASS |
| Kira-kira at Mix 37 frame 0, Full | 1 / 1 / 1 | 1 | yes | 1 | 967 | none | PASS |
| Kira-kira at Mix 37 frame 100, Full | 1 / 1 / 1 | 1 | yes | 1 | 985 | none | PASS |
| Kira-kira at Mix 37 frame 0, Draft | 1 / 1 / 1 | — | yes | 1 | 58 | none | PASS |
| Kira-kira at Mix 37 frame 100, Draft | 1 / 1 / 1 | — | yes | 1 | 52 | none | PASS |
| Kira-kira at Mix 100 frame 0, Full | 1 / 1 / 1 | 1 | no | 1 | 957 | none | PASS |
| Kira-kira at Mix 100 frame 100, Full | 1 / 1 / 1 | 1 | no | 1 | 979 | none | PASS |
| Kira-kira at Mix 100 frame 0, Draft | 1 / 1 / 1 | — | no | 1 | 54 | none | PASS |
| Kira-kira at Mix 100 frame 100, Draft | 1 / 1 / 1 | — | no | 1 | 53 | none | PASS |
| Median at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Median at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Median at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Median at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Median at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 1004 | none | PASS |
| Median at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 976 | none | PASS |
| Median at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 58 | none | PASS |
| Median at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 60 | none | PASS |
| Median at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1004 | none | PASS |
| Median at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 991 | none | PASS |
| Median at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Median at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 52 | none | PASS |
| Smart Blur at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Smart Blur at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Smart Blur at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Smart Blur at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Smart Blur at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 972 | none | PASS |
| Smart Blur at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 942 | none | PASS |
| Smart Blur at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 53 | none | PASS |
| Smart Blur at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 62 | none | PASS |
| Smart Blur at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 1018 | none | PASS |
| Smart Blur at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 956 | none | PASS |
| Smart Blur at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 64 | none | PASS |
| Smart Blur at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Roughen Edges at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Roughen Edges at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Roughen Edges at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Roughen Edges at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Roughen Edges at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 977 | none | PASS |
| Roughen Edges at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 992 | none | PASS |
| Roughen Edges at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Roughen Edges at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Roughen Edges at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 971 | none | PASS |
| Roughen Edges at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 988 | none | PASS |
| Roughen Edges at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 62 | none | PASS |
| Roughen Edges at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 66 | none | PASS |
| Radial Shadow at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Radial Shadow at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Radial Shadow at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Radial Shadow at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Radial Shadow at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 980 | none | PASS |
| Radial Shadow at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 994 | none | PASS |
| Radial Shadow at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Radial Shadow at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Radial Shadow at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 971 | none | PASS |
| Radial Shadow at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 980 | none | PASS |
| Radial Shadow at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 58 | none | PASS |
| Radial Shadow at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 58 | none | PASS |
| Bevel Alpha at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Bevel Alpha at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Bevel Alpha at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Bevel Alpha at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Bevel Alpha at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 978 | none | PASS |
| Bevel Alpha at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 993 | none | PASS |
| Bevel Alpha at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Bevel Alpha at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Bevel Alpha at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 973 | none | PASS |
| Bevel Alpha at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 989 | none | PASS |
| Bevel Alpha at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Bevel Alpha at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 54 | none | PASS |
| Snowfall at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Snowfall at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Snowfall at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Snowfall at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Snowfall at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 974 | none | PASS |
| Snowfall at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Snowfall at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 56 | none | PASS |
| Snowfall at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Snowfall at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 976 | none | PASS |
| Snowfall at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 996 | none | PASS |
| Snowfall at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 56 | none | PASS |
| Snowfall at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| Cell Pattern at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Cell Pattern at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Cell Pattern at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Cell Pattern at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Cell Pattern at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 866 | none | PASS |
| Cell Pattern at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 939 | none | PASS |
| Cell Pattern at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 62 | none | PASS |
| Cell Pattern at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 62 | none | PASS |
| Cell Pattern at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 899 | none | PASS |
| Cell Pattern at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1011 | none | PASS |
| Cell Pattern at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 47 | none | PASS |
| Cell Pattern at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 62 | none | PASS |
| Polar Coordinates at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Polar Coordinates at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Polar Coordinates at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Polar Coordinates at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Polar Coordinates at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 939 | none | PASS |
| Polar Coordinates at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 929 | none | PASS |
| Polar Coordinates at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 79 | none | PASS |
| Polar Coordinates at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Polar Coordinates at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 695 | none | PASS |
| Polar Coordinates at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 1037 | none | PASS |
| Polar Coordinates at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 45 | none | PASS |
| Polar Coordinates at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 70 | none | PASS |
| Optics Compensation at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Optics Compensation at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Optics Compensation at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Optics Compensation at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Optics Compensation at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 959 | none | PASS |
| Optics Compensation at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 938 | none | PASS |
| Optics Compensation at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 53 | none | PASS |
| Optics Compensation at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Optics Compensation at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 964 | none | PASS |
| Optics Compensation at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 932 | none | PASS |
| Optics Compensation at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 51 | none | PASS |
| Optics Compensation at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 57 | none | PASS |
| Corner Pin at Mix 0 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 975 | none | PASS |
| Corner Pin at Mix 0 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 991 | none | PASS |
| Corner Pin at Mix 0 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| Corner Pin at Mix 0 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 54 | none | PASS |
| Corner Pin at Mix 37 frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 974 | none | PASS |
| Corner Pin at Mix 37 frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 930 | none | PASS |
| Corner Pin at Mix 37 frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 63 | none | PASS |
| Corner Pin at Mix 37 frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 69 | none | PASS |
| Corner Pin at Mix 100 frame 0, Full | 3 / 3 / 2 | 3 | no | 1 | 886 | none | PASS |
| Corner Pin at Mix 100 frame 100, Full | 3 / 3 / 2 | 3 | no | 1 | 962 | none | PASS |
| Corner Pin at Mix 100 frame 0, Draft | 3 / 3 / 2 | — | no | 1 | 57 | none | PASS |
| Corner Pin at Mix 100 frame 100, Draft | 3 / 3 / 2 | — | no | 1 | 55 | none | PASS |
| colour effects in one pass, three of them mixed (Hue/Saturation 37, Vibrance 0, Invert 50) frame 0, Full | 5 / 3 / 2 | 5 | yes | 1 | 1369 | none | PASS |
| colour effects in one pass, three of them mixed (Hue/Saturation 37, Vibrance 0, Invert 50) frame 100, Full | 5 / 3 / 2 | 5 | yes | 1 | 664 | none | PASS |
| colour effects in one pass, three of them mixed (Hue/Saturation 37, Vibrance 0, Invert 50) frame 0, Draft | 5 / 3 / 2 | — | yes | 1 | 55 | none | PASS |
| colour effects in one pass, three of them mixed (Hue/Saturation 37, Vibrance 0, Invert 50) frame 100, Draft | 5 / 3 / 2 | — | yes | 1 | 76 | none | PASS |
| a Glow at 50, a Gaussian Blur at 37, then Levels at 37 (growing effects mixed) frame 0, Full | 3 / 3 / 2 | 3 | yes | 1 | 780 | none | PASS |
| a Glow at 50, a Gaussian Blur at 37, then Levels at 37 (growing effects mixed) frame 100, Full | 3 / 3 / 2 | 3 | yes | 1 | 865 | none | PASS |
| a Glow at 50, a Gaussian Blur at 37, then Levels at 37 (growing effects mixed) frame 0, Draft | 3 / 3 / 2 | — | yes | 1 | 42 | none | PASS |
| a Glow at 50, a Gaussian Blur at 37, then Levels at 37 (growing effects mixed) frame 100, Draft | 3 / 3 / 2 | — | yes | 1 | 43 | none | PASS |
| an adjustment layer: Gaussian Blur 37, Levels 50, Curves 37, Hue/Saturation frame 0, Full | 0 / 0 / 0 | — | yes | 1 | 14 | none | PASS |
| an adjustment layer: Gaussian Blur 37, Levels 50, Curves 37, Hue/Saturation frame 100, Full | 0 / 0 / 0 | — | yes | 1 | 16 | none | PASS |
| an adjustment layer: Gaussian Blur 37, Levels 50, Curves 37, Hue/Saturation frame 0, Draft | 0 / 0 / 0 | — | yes | 1 | 2 | none | PASS |
| an adjustment layer: Gaussian Blur 37, Levels 50, Curves 37, Hue/Saturation frame 100, Draft | 0 / 0 / 0 | — | yes | 1 | 1 | none | PASS |
