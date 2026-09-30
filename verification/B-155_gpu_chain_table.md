# B-155: a layer's run of effects on the GPU against the CPU

Written by `tests/b155_gpu_chain.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Each of the 69 effects the card can draw is put on the reference shot's first three layers twice: last, after two others the card can draw, and first, before two. A last stack puts Kaleidoscope, which the card does not draw (D-240), in the middle. The card is to draw each layer's whole run from the last effect it cannot draw to the end of the stack, one effect after another on the card (D-224). Bloom, Glow, Paraffin and Kira-kira look at the drawing they are given before the card is asked, so they only begin a run: last in a stack, the card has only them.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU. **The rule: no channel of any pixel more than 1 level of 255 apart**, the same warnings on both, the card drawing the frame itself, and at Full the first layer's whole run on the card. At Draft an effect whose distance the draft cannot take stays on the CPU (B-107), so only the pictures are held there.

**564 of 834 checks pass.**

The CPU drawing each plan made for the card, as it does when the card refuses a frame, draws the plan made for the CPU byte for byte in 278 of 278 (frame 100, Full and Draft); a failing one is listed below.

The worst comparison is "Halftone first, before Hue/Saturation and a Vignette frame 0, Full": largest difference 1 of 255, pixels differing: 138934. Its pictures are in `verification/B-155 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## By where the effect sits

| Where | Frames compared | Largest difference (of 255) | Pass |
|---|---:|---:|---|
| first, before two | 276 | 1 | 138 of 276 |
| last, after two | 276 | 1 | 146 of 276 |
| split by Kaleidoscope | 4 | 1 | 2 of 4 |

## Every frame

Effects left to the card on the first three layers, and the number the first layer must have at Full.

| Case | Left to the card | First layer must have | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---:|---|---|
| Radial Blur last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 9413 | none | FAIL: not the whole run on the card |
| Radial Blur last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 8439 | none | FAIL: not the whole run on the card |
| Radial Blur last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 855 | none | PASS |
| Radial Blur last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 816 | none | PASS |
| Radial Blur first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1154 | none | FAIL: not the whole run on the card |
| Radial Blur first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1094 | none | FAIL: not the whole run on the card |
| Radial Blur first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Radial Blur first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 70 | none | PASS |
| Bloom last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 1 | 1 | 455 | none | PASS |
| Bloom last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 1 | 1 | 171 | none | PASS |
| Bloom last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 36 | none | PASS |
| Bloom last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 12 | none | PASS |
| Bloom first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 22961 | none | FAIL: not the whole run on the card |
| Bloom first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 25870 | none | FAIL: not the whole run on the card |
| Bloom first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 1443 | none | PASS |
| Bloom first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 1608 | none | PASS |
| Directional Blur last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 14366 | none | FAIL: not the whole run on the card |
| Directional Blur last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 13900 | none | FAIL: not the whole run on the card |
| Directional Blur last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 1574 | none | PASS |
| Directional Blur last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 1562 | none | PASS |
| Directional Blur first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 976 | none | FAIL: not the whole run on the card |
| Directional Blur first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 949 | none | FAIL: not the whole run on the card |
| Directional Blur first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 65 | none | PASS |
| Directional Blur first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 49 | none | PASS |
| Gaussian Blur last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 6318 | none | FAIL: not the whole run on the card |
| Gaussian Blur last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 6163 | none | FAIL: not the whole run on the card |
| Gaussian Blur last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 2511 | none | PASS |
| Gaussian Blur last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 2613 | none | PASS |
| Gaussian Blur first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 997 | none | FAIL: not the whole run on the card |
| Gaussian Blur first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 949 | none | FAIL: not the whole run on the card |
| Gaussian Blur first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 44 | none | PASS |
| Gaussian Blur first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 60 | none | PASS |
| Glow last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 1 | 1 | 777 | none | PASS |
| Glow last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 1 | 1 | 763 | none | PASS |
| Glow last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 48 | none | PASS |
| Glow last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 48 | none | PASS |
| Glow first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 12733 | none | FAIL: not the whole run on the card |
| Glow first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 18893 | none | FAIL: not the whole run on the card |
| Glow first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 796 | none | PASS |
| Glow first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 1157 | none | PASS |
| Curves last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 766 | none | FAIL: not the whole run on the card |
| Curves last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 853 | none | FAIL: not the whole run on the card |
| Curves last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Curves last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 43 | none | PASS |
| Curves first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 725 | none | FAIL: not the whole run on the card |
| Curves first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1426 | none | FAIL: not the whole run on the card |
| Curves first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 67 | none | PASS |
| Curves first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 75 | none | PASS |
| Levels last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 625 | none | FAIL: not the whole run on the card |
| Levels last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 745 | none | FAIL: not the whole run on the card |
| Levels last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 28 | none | PASS |
| Levels last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 36 | none | PASS |
| Levels first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 997 | none | FAIL: not the whole run on the card |
| Levels first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 4974 | none | FAIL: not the whole run on the card |
| Levels first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 53 | none | PASS |
| Levels first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 119 | none | PASS |
| Hue/Saturation last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 923 | none | FAIL: not the whole run on the card |
| Hue/Saturation last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 908 | none | FAIL: not the whole run on the card |
| Hue/Saturation last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 51 | none | PASS |
| Hue/Saturation last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Hue/Saturation first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 70313 | none | FAIL: not the whole run on the card |
| Hue/Saturation first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 69999 | none | FAIL: not the whole run on the card |
| Hue/Saturation first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 4310 | none | PASS |
| Hue/Saturation first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 4254 | none | PASS |
| Gradient last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1295 | none | FAIL: not the whole run on the card |
| Gradient last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1157 | none | FAIL: not the whole run on the card |
| Gradient last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 91 | none | PASS |
| Gradient last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 79 | none | PASS |
| Gradient first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1452 | none | FAIL: not the whole run on the card |
| Gradient first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1317 | none | FAIL: not the whole run on the card |
| Gradient first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 89 | none | PASS |
| Gradient first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 79 | none | PASS |
| Drop Shadow last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 967 | none | FAIL: not the whole run on the card |
| Drop Shadow last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 982 | none | FAIL: not the whole run on the card |
| Drop Shadow last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Drop Shadow last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Drop Shadow first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2969 | none | FAIL: not the whole run on the card |
| Drop Shadow first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3081 | none | FAIL: not the whole run on the card |
| Drop Shadow first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 156 | none | PASS |
| Drop Shadow first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 152 | none | PASS |
| Lens Blur last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 631 | none | FAIL: not the whole run on the card |
| Lens Blur last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 794 | none | FAIL: not the whole run on the card |
| Lens Blur last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 43 | none | PASS |
| Lens Blur last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 48 | none | PASS |
| Lens Blur first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 809 | none | FAIL: not the whole run on the card |
| Lens Blur first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3034 | none | FAIL: not the whole run on the card |
| Lens Blur first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 59 | none | PASS |
| Lens Blur first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 200 | none | PASS |
| Rim Light last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 998 | none | FAIL: not the whole run on the card |
| Rim Light last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1023 | none | FAIL: not the whole run on the card |
| Rim Light last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 57 | none | PASS |
| Rim Light last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 53 | none | PASS |
| Rim Light first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 846 | none | FAIL: not the whole run on the card |
| Rim Light first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 578 | none | FAIL: not the whole run on the card |
| Rim Light first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 51 | none | PASS |
| Rim Light first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 39 | none | PASS |
| Outline last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 950 | none | FAIL: not the whole run on the card |
| Outline last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 970 | none | FAIL: not the whole run on the card |
| Outline last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 57 | none | PASS |
| Outline last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 53 | none | PASS |
| Outline first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2972 | none | FAIL: not the whole run on the card |
| Outline first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3085 | none | FAIL: not the whole run on the card |
| Outline first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 156 | none | PASS |
| Outline first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 152 | none | PASS |
| Noise last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 963 | none | FAIL: not the whole run on the card |
| Noise last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 952 | none | FAIL: not the whole run on the card |
| Noise last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 62 | none | PASS |
| Noise last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 59 | none | PASS |
| Noise first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1031 | none | FAIL: not the whole run on the card |
| Noise first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1026 | none | FAIL: not the whole run on the card |
| Noise first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 59 | none | PASS |
| Noise first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 56 | none | PASS |
| Chromatic Aberration last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 948 | none | FAIL: not the whole run on the card |
| Chromatic Aberration last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 953 | none | FAIL: not the whole run on the card |
| Chromatic Aberration last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Chromatic Aberration last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 48 | none | PASS |
| Chromatic Aberration first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2059 | none | FAIL: not the whole run on the card |
| Chromatic Aberration first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 2051 | none | FAIL: not the whole run on the card |
| Chromatic Aberration first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 107 | none | PASS |
| Chromatic Aberration first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 89 | none | PASS |
| Distance Gradation last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 964 | none | FAIL: not the whole run on the card |
| Distance Gradation last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 980 | none | FAIL: not the whole run on the card |
| Distance Gradation last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Distance Gradation last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Distance Gradation first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2970 | none | FAIL: not the whole run on the card |
| Distance Gradation first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3100 | none | FAIL: not the whole run on the card |
| Distance Gradation first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 156 | none | PASS |
| Distance Gradation first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 152 | none | PASS |
| Light Rays last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 973 | none | FAIL: not the whole run on the card |
| Light Rays last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 876 | none | FAIL: not the whole run on the card |
| Light Rays last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 63 | none | PASS |
| Light Rays last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 65 | none | PASS |
| Light Rays first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 9147 | none | FAIL: not the whole run on the card |
| Light Rays first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 17138 | none | FAIL: not the whole run on the card |
| Light Rays first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 583 | none | PASS |
| Light Rays first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 1071 | none | PASS |
| Exposure Flicker last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 854 | none | FAIL: not the whole run on the card |
| Exposure Flicker last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 812 | none | FAIL: not the whole run on the card |
| Exposure Flicker last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 53 | none | PASS |
| Exposure Flicker last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 56 | none | PASS |
| Exposure Flicker first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 783 | none | FAIL: not the whole run on the card |
| Exposure Flicker first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 7908 | none | FAIL: not the whole run on the card |
| Exposure Flicker first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 74 | none | PASS |
| Exposure Flicker first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 489 | none | PASS |
| Vignette last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1161 | none | FAIL: not the whole run on the card |
| Vignette last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 998 | none | FAIL: not the whole run on the card |
| Vignette last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 78 | none | PASS |
| Vignette last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 57 | none | PASS |
| Vignette first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2972 | none | FAIL: not the whole run on the card |
| Vignette first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3110 | none | FAIL: not the whole run on the card |
| Vignette first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 156 | none | PASS |
| Vignette first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 150 | none | PASS |
| Turbulent Displace last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 936 | none | FAIL: not the whole run on the card |
| Turbulent Displace last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 993 | none | FAIL: not the whole run on the card |
| Turbulent Displace last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 58 | none | PASS |
| Turbulent Displace last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 45 | none | PASS |
| Turbulent Displace first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2231 | none | FAIL: not the whole run on the card |
| Turbulent Displace first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 2041 | none | FAIL: not the whole run on the card |
| Turbulent Displace first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 101 | none | PASS |
| Turbulent Displace first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 88 | none | PASS |
| Fractal Noise last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1022 | none | FAIL: not the whole run on the card |
| Fractal Noise last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1026 | none | FAIL: not the whole run on the card |
| Fractal Noise last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 61 | none | PASS |
| Fractal Noise last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 53 | none | PASS |
| Fractal Noise first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 957 | none | FAIL: not the whole run on the card |
| Fractal Noise first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1143 | none | FAIL: not the whole run on the card |
| Fractal Noise first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 53 | none | PASS |
| Fractal Noise first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 67 | none | PASS |
| Gradient Map last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1274 | none | FAIL: not the whole run on the card |
| Gradient Map last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 2075 | none | FAIL: not the whole run on the card |
| Gradient Map last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 78 | none | PASS |
| Gradient Map last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 156 | none | PASS |
| Gradient Map first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1211 | none | FAIL: not the whole run on the card |
| Gradient Map first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1134 | none | FAIL: not the whole run on the card |
| Gradient Map first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 93 | none | PASS |
| Gradient Map first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 61 | none | PASS |
| Color Balance last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1097 | none | FAIL: not the whole run on the card |
| Color Balance last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 931 | none | FAIL: not the whole run on the card |
| Color Balance last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 73 | none | PASS |
| Color Balance last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 69 | none | PASS |
| Color Balance first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 851 | none | FAIL: not the whole run on the card |
| Color Balance first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 629 | none | FAIL: not the whole run on the card |
| Color Balance first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 56 | none | PASS |
| Color Balance first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 50 | none | PASS |
| Offset last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 911 | none | FAIL: not the whole run on the card |
| Offset last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 886 | none | FAIL: not the whole run on the card |
| Offset last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 61 | none | PASS |
| Offset last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 50 | none | PASS |
| Offset first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2117 | none | FAIL: not the whole run on the card |
| Offset first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 2489 | none | FAIL: not the whole run on the card |
| Offset first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 100 | none | PASS |
| Offset first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 93 | none | PASS |
| Invert last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1231 | none | FAIL: not the whole run on the card |
| Invert last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1453 | none | FAIL: not the whole run on the card |
| Invert last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 77 | none | PASS |
| Invert last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 73 | none | PASS |
| Invert first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1039 | none | FAIL: not the whole run on the card |
| Invert first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 821 | none | FAIL: not the whole run on the card |
| Invert first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 76 | none | PASS |
| Invert first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 58 | none | PASS |
| Brightness & Contrast last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 365 | none | FAIL: not the whole run on the card |
| Brightness & Contrast last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 216 | none | FAIL: not the whole run on the card |
| Brightness & Contrast last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 29 | none | PASS |
| Brightness & Contrast last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 5 | none | PASS |
| Brightness & Contrast first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 28840 | none | FAIL: not the whole run on the card |
| Brightness & Contrast first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 26047 | none | FAIL: not the whole run on the card |
| Brightness & Contrast first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 1933 | none | PASS |
| Brightness & Contrast first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 1631 | none | PASS |
| Black & White last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1263 | none | FAIL: not the whole run on the card |
| Black & White last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 779 | none | FAIL: not the whole run on the card |
| Black & White last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 86 | none | PASS |
| Black & White last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 57 | none | PASS |
| Black & White first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1302 | none | FAIL: not the whole run on the card |
| Black & White first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 2100 | none | FAIL: not the whole run on the card |
| Black & White first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 88 | none | PASS |
| Black & White first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 108 | none | PASS |
| Posterize last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2 | none | FAIL: not the whole run on the card |
| Posterize last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3 | none | FAIL: not the whole run on the card |
| Posterize last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 0 | 0 | none | PASS |
| Posterize last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 0 | 0 | none | PASS |
| Posterize first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 14346 | none | FAIL: not the whole run on the card |
| Posterize first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 15228 | none | FAIL: not the whole run on the card |
| Posterize first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 851 | none | PASS |
| Posterize first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 916 | none | PASS |
| Threshold last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 0 | 0 | none | FAIL: not the whole run on the card |
| Threshold last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 0 | 0 | none | FAIL: not the whole run on the card |
| Threshold last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 0 | 0 | none | PASS |
| Threshold last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 0 | 0 | none | PASS |
| Threshold first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 98533 | none | FAIL: not the whole run on the card |
| Threshold first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 105305 | none | FAIL: not the whole run on the card |
| Threshold first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 6290 | none | PASS |
| Threshold first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 6637 | none | PASS |
| Channel Mixer last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1232 | none | FAIL: not the whole run on the card |
| Channel Mixer last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1415 | none | FAIL: not the whole run on the card |
| Channel Mixer last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 78 | none | PASS |
| Channel Mixer last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 87 | none | PASS |
| Channel Mixer first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 3592 | none | FAIL: not the whole run on the card |
| Channel Mixer first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3347 | none | FAIL: not the whole run on the card |
| Channel Mixer first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 190 | none | PASS |
| Channel Mixer first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 175 | none | PASS |
| Vibrance last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 732 | none | FAIL: not the whole run on the card |
| Vibrance last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 783 | none | FAIL: not the whole run on the card |
| Vibrance last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 43 | none | PASS |
| Vibrance last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 52 | none | PASS |
| Vibrance first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 949 | none | FAIL: not the whole run on the card |
| Vibrance first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 911 | none | FAIL: not the whole run on the card |
| Vibrance first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Vibrance first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 50 | none | PASS |
| Leave Color last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1040 | none | FAIL: not the whole run on the card |
| Leave Color last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1030 | none | FAIL: not the whole run on the card |
| Leave Color last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 73 | none | PASS |
| Leave Color last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Leave Color first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 920 | none | FAIL: not the whole run on the card |
| Leave Color first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 849 | none | FAIL: not the whole run on the card |
| Leave Color first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 60 | none | PASS |
| Leave Color first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 71 | none | PASS |
| Solarize last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1588 | none | FAIL: not the whole run on the card |
| Solarize last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1515 | none | FAIL: not the whole run on the card |
| Solarize last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 89 | none | PASS |
| Solarize last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 101 | none | PASS |
| Solarize first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1593 | none | FAIL: not the whole run on the card |
| Solarize first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 2023 | none | FAIL: not the whole run on the card |
| Solarize first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 77 | none | PASS |
| Solarize first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 89 | none | PASS |
| Halftone last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 0 | 0 | none | FAIL: not the whole run on the card |
| Halftone last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 0 | 0 | none | FAIL: not the whole run on the card |
| Halftone last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 0 | 0 | none | PASS |
| Halftone last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 0 | 0 | none | PASS |
| Halftone first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 138934 | none | FAIL: not the whole run on the card |
| Halftone first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 122256 | none | FAIL: not the whole run on the card |
| Halftone first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 8726 | none | PASS |
| Halftone first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 7632 | none | PASS |
| Mosaic last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 700 | none | FAIL: not the whole run on the card |
| Mosaic last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 710 | none | FAIL: not the whole run on the card |
| Mosaic last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 82 | none | PASS |
| Mosaic last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 42 | none | PASS |
| Mosaic first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1575 | none | FAIL: not the whole run on the card |
| Mosaic first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1461 | none | FAIL: not the whole run on the card |
| Mosaic first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 94 | none | PASS |
| Mosaic first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 123 | none | PASS |
| Emboss last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1530 | none | FAIL: not the whole run on the card |
| Emboss last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1538 | none | FAIL: not the whole run on the card |
| Emboss last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 1968 | none | PASS |
| Emboss last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 1909 | none | PASS |
| Emboss first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1162 | none | FAIL: not the whole run on the card |
| Emboss first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1195 | none | FAIL: not the whole run on the card |
| Emboss first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 129 | none | PASS |
| Emboss first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 95 | none | PASS |
| Find Edges last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 756 | none | FAIL: not the whole run on the card |
| Find Edges last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 811 | none | FAIL: not the whole run on the card |
| Find Edges last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 61 | none | PASS |
| Find Edges last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 60 | none | PASS |
| Find Edges first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 63506 | none | FAIL: not the whole run on the card |
| Find Edges first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 44374 | none | FAIL: not the whole run on the card |
| Find Edges first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 1918 | none | PASS |
| Find Edges first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 1017 | none | PASS |
| Sharpen last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 948 | none | FAIL: not the whole run on the card |
| Sharpen last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 896 | none | FAIL: not the whole run on the card |
| Sharpen last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Sharpen last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 53 | none | PASS |
| Sharpen first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 963 | none | FAIL: not the whole run on the card |
| Sharpen first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 859 | none | FAIL: not the whole run on the card |
| Sharpen first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 82 | none | PASS |
| Sharpen first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 80 | none | PASS |
| Diffusion last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 863 | none | FAIL: not the whole run on the card |
| Diffusion last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1267 | none | FAIL: not the whole run on the card |
| Diffusion last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 43 | none | PASS |
| Diffusion last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 77 | none | PASS |
| Diffusion first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 983 | none | FAIL: not the whole run on the card |
| Diffusion first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 866 | none | FAIL: not the whole run on the card |
| Diffusion first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 60 | none | PASS |
| Diffusion first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Wave Warp last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1010 | none | FAIL: not the whole run on the card |
| Wave Warp last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 930 | none | FAIL: not the whole run on the card |
| Wave Warp last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 78 | none | PASS |
| Wave Warp last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 59 | none | PASS |
| Wave Warp first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2724 | none | FAIL: not the whole run on the card |
| Wave Warp first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 2595 | none | FAIL: not the whole run on the card |
| Wave Warp first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 129 | none | PASS |
| Wave Warp first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 124 | none | PASS |
| Ripple last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 968 | none | FAIL: not the whole run on the card |
| Ripple last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 947 | none | FAIL: not the whole run on the card |
| Ripple last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 71 | none | PASS |
| Ripple last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 64 | none | PASS |
| Ripple first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2124 | none | FAIL: not the whole run on the card |
| Ripple first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 2194 | none | FAIL: not the whole run on the card |
| Ripple first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 105 | none | PASS |
| Ripple first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 108 | none | PASS |
| Twirl last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 946 | none | FAIL: not the whole run on the card |
| Twirl last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 976 | none | FAIL: not the whole run on the card |
| Twirl last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Twirl last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Twirl first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2991 | none | FAIL: not the whole run on the card |
| Twirl first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3101 | none | FAIL: not the whole run on the card |
| Twirl first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 159 | none | PASS |
| Twirl first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 153 | none | PASS |
| Bulge last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 944 | none | FAIL: not the whole run on the card |
| Bulge last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 977 | none | FAIL: not the whole run on the card |
| Bulge last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 56 | none | PASS |
| Bulge last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Bulge first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2986 | none | FAIL: not the whole run on the card |
| Bulge first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3101 | none | FAIL: not the whole run on the card |
| Bulge first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 161 | none | PASS |
| Bulge first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 153 | none | PASS |
| Mirror last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 994 | none | FAIL: not the whole run on the card |
| Mirror last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1009 | none | FAIL: not the whole run on the card |
| Mirror last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 62 | none | PASS |
| Mirror last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 53 | none | PASS |
| Mirror first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1079 | none | FAIL: not the whole run on the card |
| Mirror first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 733 | none | FAIL: not the whole run on the card |
| Mirror first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 37 | none | PASS |
| Mirror first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 34 | none | PASS |
| Linear Wipe last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 498 | none | FAIL: not the whole run on the card |
| Linear Wipe last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 16 | none | FAIL: not the whole run on the card |
| Linear Wipe last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 31 | none | PASS |
| Linear Wipe last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 1 | none | PASS |
| Linear Wipe first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 523 | none | FAIL: not the whole run on the card |
| Linear Wipe first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 123 | none | FAIL: not the whole run on the card |
| Linear Wipe first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 20 | none | PASS |
| Linear Wipe first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 0 | 0 | none | PASS |
| Radial Wipe last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 466 | none | FAIL: not the whole run on the card |
| Radial Wipe last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 965 | none | FAIL: not the whole run on the card |
| Radial Wipe last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 24 | none | PASS |
| Radial Wipe last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 53 | none | PASS |
| Radial Wipe first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2446 | none | FAIL: not the whole run on the card |
| Radial Wipe first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 2977 | none | FAIL: not the whole run on the card |
| Radial Wipe first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 136 | none | PASS |
| Radial Wipe first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 152 | none | PASS |
| Venetian Blinds last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 491 | none | FAIL: not the whole run on the card |
| Venetian Blinds last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 506 | none | FAIL: not the whole run on the card |
| Venetian Blinds last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 32 | none | PASS |
| Venetian Blinds last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 32 | none | PASS |
| Venetian Blinds first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1751 | none | FAIL: not the whole run on the card |
| Venetian Blinds first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1865 | none | FAIL: not the whole run on the card |
| Venetian Blinds first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 105 | none | PASS |
| Venetian Blinds first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 111 | none | PASS |
| Iris Wipe last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 945 | none | FAIL: not the whole run on the card |
| Iris Wipe last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 963 | none | FAIL: not the whole run on the card |
| Iris Wipe last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Iris Wipe last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 53 | none | PASS |
| Iris Wipe first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2955 | none | FAIL: not the whole run on the card |
| Iris Wipe first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3086 | none | FAIL: not the whole run on the card |
| Iris Wipe first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 154 | none | PASS |
| Iris Wipe first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 150 | none | PASS |
| Simple Choker last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 965 | none | FAIL: not the whole run on the card |
| Simple Choker last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 982 | none | FAIL: not the whole run on the card |
| Simple Choker last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Simple Choker last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Simple Choker first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2969 | none | FAIL: not the whole run on the card |
| Simple Choker first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3095 | none | FAIL: not the whole run on the card |
| Simple Choker first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 156 | none | PASS |
| Simple Choker first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 152 | none | PASS |
| Speed Lines last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 781 | none | FAIL: not the whole run on the card |
| Speed Lines last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 660 | none | FAIL: not the whole run on the card |
| Speed Lines last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 58 | none | PASS |
| Speed Lines last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 61 | none | PASS |
| Speed Lines first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1730 | none | FAIL: not the whole run on the card |
| Speed Lines first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1625 | none | FAIL: not the whole run on the card |
| Speed Lines first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 101 | none | PASS |
| Speed Lines first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 84 | none | PASS |
| Cross Glare last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 427 | none | FAIL: not the whole run on the card |
| Cross Glare last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 160 | none | FAIL: not the whole run on the card |
| Cross Glare last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 31 | none | PASS |
| Cross Glare last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 15 | none | PASS |
| Cross Glare first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 22455 | none | FAIL: not the whole run on the card |
| Cross Glare first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 45527 | none | FAIL: not the whole run on the card |
| Cross Glare first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 1409 | none | PASS |
| Cross Glare first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 2898 | none | PASS |
| Camera Shake last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 909 | none | FAIL: not the whole run on the card |
| Camera Shake last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 961 | none | FAIL: not the whole run on the card |
| Camera Shake last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 52 | none | PASS |
| Camera Shake last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 69 | none | PASS |
| Camera Shake first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2593 | none | FAIL: not the whole run on the card |
| Camera Shake first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 812 | none | FAIL: not the whole run on the card |
| Camera Shake first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 122 | none | PASS |
| Camera Shake first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 116 | none | PASS |
| Rain last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 971 | none | FAIL: not the whole run on the card |
| Rain last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 987 | none | FAIL: not the whole run on the card |
| Rain last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 58 | none | PASS |
| Rain last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Rain first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2960 | none | FAIL: not the whole run on the card |
| Rain first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3070 | none | FAIL: not the whole run on the card |
| Rain first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 156 | none | PASS |
| Rain first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 144 | none | PASS |
| Motion Tile last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 964 | none | FAIL: not the whole run on the card |
| Motion Tile last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 981 | none | FAIL: not the whole run on the card |
| Motion Tile last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Motion Tile last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Motion Tile first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2969 | none | FAIL: not the whole run on the card |
| Motion Tile first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3100 | none | FAIL: not the whole run on the card |
| Motion Tile first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 156 | none | PASS |
| Motion Tile first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 152 | none | PASS |
| Color Lookup last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1018 | none | FAIL: not the whole run on the card |
| Color Lookup last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 921 | none | FAIL: not the whole run on the card |
| Color Lookup last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 53 | none | PASS |
| Color Lookup last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 63 | none | PASS |
| Color Lookup first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 626 | none | FAIL: not the whole run on the card |
| Color Lookup first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 505 | none | FAIL: not the whole run on the card |
| Color Lookup first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 60 | none | PASS |
| Color Lookup first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 53 | none | PASS |
| Line Blur last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 985 | none | FAIL: not the whole run on the card |
| Line Blur last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 980 | none | FAIL: not the whole run on the card |
| Line Blur last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 61 | none | PASS |
| Line Blur last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 52 | none | PASS |
| Line Blur first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1970 | none | FAIL: not the whole run on the card |
| Line Blur first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 2013 | none | FAIL: not the whole run on the card |
| Line Blur first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 74 | none | PASS |
| Line Blur first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 74 | none | PASS |
| HSV Key last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 757 | none | FAIL: not the whole run on the card |
| HSV Key last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 915 | none | FAIL: not the whole run on the card |
| HSV Key last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 45 | none | PASS |
| HSV Key last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 51 | none | PASS |
| HSV Key first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2869 | none | FAIL: not the whole run on the card |
| HSV Key first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3020 | none | FAIL: not the whole run on the card |
| HSV Key first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 140 | none | PASS |
| HSV Key first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 147 | none | PASS |
| Paraffin last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 1 | 1 | 908 | none | PASS |
| Paraffin last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 1 | 1 | 999 | none | PASS |
| Paraffin last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Paraffin last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 64 | none | PASS |
| Paraffin first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 794 | none | FAIL: not the whole run on the card |
| Paraffin first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 614 | none | FAIL: not the whole run on the card |
| Paraffin first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 62 | none | PASS |
| Paraffin first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 45 | none | PASS |
| Kira-kira last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 1 | 1 | 957 | none | PASS |
| Kira-kira last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 1 | 1 | 979 | none | PASS |
| Kira-kira last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Kira-kira last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 53 | none | PASS |
| Kira-kira first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 3028 | none | FAIL: not the whole run on the card |
| Kira-kira first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3184 | none | FAIL: not the whole run on the card |
| Kira-kira first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 175 | none | PASS |
| Kira-kira first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 172 | none | PASS |
| Median last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1000 | none | FAIL: not the whole run on the card |
| Median last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 989 | none | FAIL: not the whole run on the card |
| Median last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Median last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 52 | none | PASS |
| Median first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2930 | none | FAIL: not the whole run on the card |
| Median first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3000 | none | FAIL: not the whole run on the card |
| Median first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 161 | none | PASS |
| Median first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 160 | none | PASS |
| Smart Blur last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1018 | none | FAIL: not the whole run on the card |
| Smart Blur last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 955 | none | FAIL: not the whole run on the card |
| Smart Blur last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 63 | none | PASS |
| Smart Blur last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Smart Blur first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1547 | none | FAIL: not the whole run on the card |
| Smart Blur first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1480 | none | FAIL: not the whole run on the card |
| Smart Blur first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 105 | none | PASS |
| Smart Blur first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 84 | none | PASS |
| Roughen Edges last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 966 | none | FAIL: not the whole run on the card |
| Roughen Edges last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 987 | none | FAIL: not the whole run on the card |
| Roughen Edges last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Roughen Edges last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 56 | none | PASS |
| Roughen Edges first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2976 | none | FAIL: not the whole run on the card |
| Roughen Edges first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3106 | none | FAIL: not the whole run on the card |
| Roughen Edges first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 156 | none | PASS |
| Roughen Edges first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 152 | none | PASS |
| Radial Shadow last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 967 | none | FAIL: not the whole run on the card |
| Radial Shadow last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 984 | none | FAIL: not the whole run on the card |
| Radial Shadow last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Radial Shadow last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Radial Shadow first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2968 | none | FAIL: not the whole run on the card |
| Radial Shadow first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3096 | none | FAIL: not the whole run on the card |
| Radial Shadow first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 156 | none | PASS |
| Radial Shadow first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 152 | none | PASS |
| Bevel Alpha last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 965 | none | FAIL: not the whole run on the card |
| Bevel Alpha last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 982 | none | FAIL: not the whole run on the card |
| Bevel Alpha last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Bevel Alpha last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Bevel Alpha first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2971 | none | FAIL: not the whole run on the card |
| Bevel Alpha first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3102 | none | FAIL: not the whole run on the card |
| Bevel Alpha first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 156 | none | PASS |
| Bevel Alpha first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 152 | none | PASS |
| Snowfall last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 964 | none | FAIL: not the whole run on the card |
| Snowfall last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 986 | none | FAIL: not the whole run on the card |
| Snowfall last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 56 | none | PASS |
| Snowfall last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Snowfall first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2947 | none | FAIL: not the whole run on the card |
| Snowfall first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 3034 | none | FAIL: not the whole run on the card |
| Snowfall first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 154 | none | PASS |
| Snowfall first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 143 | none | PASS |
| Cell Pattern last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 891 | none | FAIL: not the whole run on the card |
| Cell Pattern last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1003 | none | FAIL: not the whole run on the card |
| Cell Pattern last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 46 | none | PASS |
| Cell Pattern last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 61 | none | PASS |
| Cell Pattern first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 983 | none | FAIL: not the whole run on the card |
| Cell Pattern first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 972 | none | FAIL: not the whole run on the card |
| Cell Pattern first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 62 | none | PASS |
| Cell Pattern first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 65 | none | PASS |
| Polar Coordinates last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 696 | none | FAIL: not the whole run on the card |
| Polar Coordinates last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1038 | none | FAIL: not the whole run on the card |
| Polar Coordinates last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 44 | none | PASS |
| Polar Coordinates last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 70 | none | PASS |
| Polar Coordinates first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1737 | none | FAIL: not the whole run on the card |
| Polar Coordinates first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 2141 | none | FAIL: not the whole run on the card |
| Polar Coordinates first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 75 | none | PASS |
| Polar Coordinates first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 115 | none | PASS |
| Optics Compensation last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 958 | none | FAIL: not the whole run on the card |
| Optics Compensation last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 932 | none | FAIL: not the whole run on the card |
| Optics Compensation last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 51 | none | PASS |
| Optics Compensation last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 56 | none | PASS |
| Optics Compensation first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 2116 | none | FAIL: not the whole run on the card |
| Optics Compensation first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 2227 | none | FAIL: not the whole run on the card |
| Optics Compensation first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 116 | none | PASS |
| Optics Compensation first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 115 | none | PASS |
| Corner Pin last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 3 | 1 | 881 | none | FAIL: not the whole run on the card |
| Corner Pin last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 3 | 1 | 954 | none | FAIL: not the whole run on the card |
| Corner Pin last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 56 | none | PASS |
| Corner Pin last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 55 | none | PASS |
| Corner Pin first, before Hue/Saturation and a Vignette frame 0, Full | 1 / 1 / 1 | 3 | 1 | 1444 | none | FAIL: not the whole run on the card |
| Corner Pin first, before Hue/Saturation and a Vignette frame 100, Full | 1 / 1 / 1 | 3 | 1 | 1937 | none | FAIL: not the whole run on the card |
| Corner Pin first, before Hue/Saturation and a Vignette frame 0, Draft | 1 / 1 / 1 | — | 1 | 60 | none | PASS |
| Corner Pin first, before Hue/Saturation and a Vignette frame 100, Draft | 1 / 1 / 1 | — | 1 | 91 | none | PASS |
| Levels, then Kaleidoscope (not on the card), then a Gaussian Blur and Curves frame 0, Full | 1 / 1 / 1 | 2 | 1 | 994 | none | FAIL: not the whole run on the card |
| Levels, then Kaleidoscope (not on the card), then a Gaussian Blur and Curves frame 100, Full | 1 / 1 / 1 | 2 | 1 | 724 | none | FAIL: not the whole run on the card |
| Levels, then Kaleidoscope (not on the card), then a Gaussian Blur and Curves frame 0, Draft | 1 / 1 / 1 | — | 1 | 69 | none | PASS |
| Levels, then Kaleidoscope (not on the card), then a Gaussian Blur and Curves frame 100, Draft | 1 / 1 / 1 | — | 1 | 51 | none | PASS |
