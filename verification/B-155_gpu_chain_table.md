# B-155: a layer's run of effects on the GPU against the CPU

Written by `tests/b155_gpu_chain.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Each of the 70 effects the card can draw is put on the reference shot's first three layers twice: last, after two others the card can draw, and first, before two. A last stack puts Kaleidoscope, which the card does not draw (D-240), in the middle. The card is to draw each layer's whole run from the last effect it cannot draw to the end of the stack, one effect after another on the card (D-224). Bloom, Glow, Paraffin and Kira-kira look at the drawing they are given before the card is asked, so they only begin a run: last in a stack, the card has only them. So does an HSV Key: the hue of a nearly grey pixel swings with the smallest change, and given the card's picture after two others it keyed pixels the CPU did not, 255 levels apart, in this table's first run after the build (D-224).

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU. **The rule: no channel of any pixel more than 1 level of 255 apart**, the same warnings on both, the card drawing the frame itself, and at Full the first layer's whole run on the card. At Draft an effect whose distance the draft cannot take stays on the CPU (B-107), so only the pictures are held there.

**846 of 846 checks pass.**

The CPU drawing each plan made for the card, as it does when the card refuses a frame, draws the plan made for the CPU byte for byte in 282 of 282 (frame 100, Full and Draft); a failing one is listed below.

The worst comparison is "Halftone first, before Hue/Saturation and a Vignette frame 0, Full": largest difference 1 of 255, pixels differing: 138934. Its pictures are in `verification/B-155 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## By where the effect sits

| Where | Frames compared | Largest difference (of 255) | Pass |
|---|---:|---:|---|
| first, before two | 280 | 1 | 280 of 280 |
| last, after two | 280 | 1 | 280 of 280 |
| split by Kaleidoscope | 4 | 1 | 4 of 4 |

## Every frame

Effects left to the card on the first three layers, and the number the first layer must have at Full.

| Case | Left to the card | First layer must have | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---:|---|---|
| Radial Blur last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1135 | none | PASS |
| Radial Blur last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1095 | none | PASS |
| Radial Blur last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 58 | none | PASS |
| Radial Blur last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 62 | none | PASS |
| Radial Blur first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 23716 | none | PASS |
| Radial Blur first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 22813 | none | PASS |
| Radial Blur first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 1320 | none | PASS |
| Radial Blur first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 1254 | none | PASS |
| Bloom last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 1 | 1 | 987 | none | PASS |
| Bloom last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 1 | 1 | 723 | none | PASS |
| Bloom last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 65 | none | PASS |
| Bloom last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 37 | none | PASS |
| Bloom first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 24911 | none | PASS |
| Bloom first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 27635 | none | PASS |
| Bloom first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 1504 | none | PASS |
| Bloom first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 1666 | none | PASS |
| Directional Blur last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1103 | none | PASS |
| Directional Blur last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1057 | none | PASS |
| Directional Blur last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 78 | none | PASS |
| Directional Blur last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 88 | none | PASS |
| Directional Blur first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 30457 | none | PASS |
| Directional Blur first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 29975 | none | PASS |
| Directional Blur first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 2104 | none | PASS |
| Directional Blur first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 2015 | none | PASS |
| Gaussian Blur last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1041 | none | PASS |
| Gaussian Blur last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 995 | none | PASS |
| Gaussian Blur last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 65 | none | PASS |
| Gaussian Blur last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 67 | none | PASS |
| Gaussian Blur first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 28996 | none | PASS |
| Gaussian Blur first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 28701 | none | PASS |
| Gaussian Blur first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 1763 | none | PASS |
| Gaussian Blur first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 1752 | none | PASS |
| Glow last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 1 | 1 | 777 | none | PASS |
| Glow last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 1 | 1 | 763 | none | PASS |
| Glow last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 48 | none | PASS |
| Glow last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 48 | none | PASS |
| Glow first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 12734 | none | PASS |
| Glow first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 18896 | none | PASS |
| Glow first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 796 | none | PASS |
| Glow first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 1157 | none | PASS |
| Curves last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 771 | none | PASS |
| Curves last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 858 | none | PASS |
| Curves last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Curves last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 44 | none | PASS |
| Curves first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 725 | none | PASS |
| Curves first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1426 | none | PASS |
| Curves first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 67 | none | PASS |
| Curves first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 75 | none | PASS |
| Levels last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 623 | none | PASS |
| Levels last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 744 | none | PASS |
| Levels last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 30 | none | PASS |
| Levels last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 37 | none | PASS |
| Levels first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1007 | none | PASS |
| Levels first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 4993 | none | PASS |
| Levels first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 54 | none | PASS |
| Levels first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 120 | none | PASS |
| Hue/Saturation last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 951 | none | PASS |
| Hue/Saturation last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 939 | none | PASS |
| Hue/Saturation last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 50 | none | PASS |
| Hue/Saturation last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 53 | none | PASS |
| Hue/Saturation first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 70314 | none | PASS |
| Hue/Saturation first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 69999 | none | PASS |
| Hue/Saturation first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 4310 | none | PASS |
| Hue/Saturation first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 4254 | none | PASS |
| Gradient last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1293 | none | PASS |
| Gradient last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1155 | none | PASS |
| Gradient last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 91 | none | PASS |
| Gradient last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 80 | none | PASS |
| Gradient first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1452 | none | PASS |
| Gradient first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1318 | none | PASS |
| Gradient first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 90 | none | PASS |
| Gradient first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 79 | none | PASS |
| Drop Shadow last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 970 | none | PASS |
| Drop Shadow last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 985 | none | PASS |
| Drop Shadow last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Drop Shadow last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Drop Shadow first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2969 | none | PASS |
| Drop Shadow first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3081 | none | PASS |
| Drop Shadow first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 156 | none | PASS |
| Drop Shadow first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 152 | none | PASS |
| Lens Blur last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 627 | none | PASS |
| Lens Blur last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 795 | none | PASS |
| Lens Blur last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 44 | none | PASS |
| Lens Blur last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 49 | none | PASS |
| Lens Blur first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 809 | none | PASS |
| Lens Blur first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3034 | none | PASS |
| Lens Blur first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 59 | none | PASS |
| Lens Blur first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 200 | none | PASS |
| Rim Light last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1055 | none | PASS |
| Rim Light last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1064 | none | PASS |
| Rim Light last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 58 | none | PASS |
| Rim Light last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 53 | none | PASS |
| Rim Light first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 983 | none | PASS |
| Rim Light first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 783 | none | PASS |
| Rim Light first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Rim Light first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 43 | none | PASS |
| Outline last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1002 | none | PASS |
| Outline last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1009 | none | PASS |
| Outline last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 60 | none | PASS |
| Outline last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Outline first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2973 | none | PASS |
| Outline first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3086 | none | PASS |
| Outline first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 156 | none | PASS |
| Outline first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 152 | none | PASS |
| Noise last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 954 | none | PASS |
| Noise last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 950 | none | PASS |
| Noise last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 62 | none | PASS |
| Noise last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 60 | none | PASS |
| Noise first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1031 | none | PASS |
| Noise first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1026 | none | PASS |
| Noise first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 59 | none | PASS |
| Noise first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 56 | none | PASS |
| Chromatic Aberration last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 947 | none | PASS |
| Chromatic Aberration last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 949 | none | PASS |
| Chromatic Aberration last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 53 | none | PASS |
| Chromatic Aberration last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 46 | none | PASS |
| Chromatic Aberration first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2060 | none | PASS |
| Chromatic Aberration first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 2051 | none | PASS |
| Chromatic Aberration first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 107 | none | PASS |
| Chromatic Aberration first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 90 | none | PASS |
| Distance Gradation last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 973 | none | PASS |
| Distance Gradation last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 988 | none | PASS |
| Distance Gradation last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Distance Gradation last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 54 | none | PASS |
| Distance Gradation first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2970 | none | PASS |
| Distance Gradation first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3100 | none | PASS |
| Distance Gradation first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 156 | none | PASS |
| Distance Gradation first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 152 | none | PASS |
| Light Rays last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 974 | none | PASS |
| Light Rays last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 874 | none | PASS |
| Light Rays last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 63 | none | PASS |
| Light Rays last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 66 | none | PASS |
| Light Rays first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 9239 | none | PASS |
| Light Rays first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 17219 | none | PASS |
| Light Rays first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 588 | none | PASS |
| Light Rays first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 1072 | none | PASS |
| Exposure Flicker last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 860 | none | PASS |
| Exposure Flicker last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 816 | none | PASS |
| Exposure Flicker last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 54 | none | PASS |
| Exposure Flicker last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 56 | none | PASS |
| Exposure Flicker first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 783 | none | PASS |
| Exposure Flicker first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 7908 | none | PASS |
| Exposure Flicker first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 74 | none | PASS |
| Exposure Flicker first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 489 | none | PASS |
| Vignette last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1159 | none | PASS |
| Vignette last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 994 | none | PASS |
| Vignette last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 79 | none | PASS |
| Vignette last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 58 | none | PASS |
| Vignette first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2972 | none | PASS |
| Vignette first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3111 | none | PASS |
| Vignette first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 156 | none | PASS |
| Vignette first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 150 | none | PASS |
| Turbulent Displace last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 971 | none | PASS |
| Turbulent Displace last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1003 | none | PASS |
| Turbulent Displace last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 54 | none | PASS |
| Turbulent Displace last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 49 | none | PASS |
| Turbulent Displace first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2385 | none | PASS |
| Turbulent Displace first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 2140 | none | PASS |
| Turbulent Displace first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 108 | none | PASS |
| Turbulent Displace first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 93 | none | PASS |
| Fractal Noise last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1029 | none | PASS |
| Fractal Noise last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1031 | none | PASS |
| Fractal Noise last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 62 | none | PASS |
| Fractal Noise last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 53 | none | PASS |
| Fractal Noise first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 958 | none | PASS |
| Fractal Noise first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1143 | none | PASS |
| Fractal Noise first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 53 | none | PASS |
| Fractal Noise first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 67 | none | PASS |
| Fractal Noise, turbulent block last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1030 | none | PASS |
| Fractal Noise, turbulent block last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 956 | none | PASS |
| Fractal Noise, turbulent block last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 53 | none | PASS |
| Fractal Noise, turbulent block last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 49 | none | PASS |
| Fractal Noise, turbulent block first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1011 | none | PASS |
| Fractal Noise, turbulent block first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1054 | none | PASS |
| Fractal Noise, turbulent block first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 57 | none | PASS |
| Fractal Noise, turbulent block first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 67 | none | PASS |
| Gradient Map last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1275 | none | PASS |
| Gradient Map last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 2072 | none | PASS |
| Gradient Map last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 79 | none | PASS |
| Gradient Map last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 157 | none | PASS |
| Gradient Map first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1211 | none | PASS |
| Gradient Map first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1134 | none | PASS |
| Gradient Map first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 93 | none | PASS |
| Gradient Map first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 61 | none | PASS |
| Color Balance last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1110 | none | PASS |
| Color Balance last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 945 | none | PASS |
| Color Balance last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 73 | none | PASS |
| Color Balance last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 69 | none | PASS |
| Color Balance first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 851 | none | PASS |
| Color Balance first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 628 | none | PASS |
| Color Balance first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 56 | none | PASS |
| Color Balance first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 50 | none | PASS |
| Offset last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 906 | none | PASS |
| Offset last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 880 | none | PASS |
| Offset last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 61 | none | PASS |
| Offset last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 50 | none | PASS |
| Offset first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2117 | none | PASS |
| Offset first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 2489 | none | PASS |
| Offset first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 100 | none | PASS |
| Offset first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 93 | none | PASS |
| Invert last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1247 | none | PASS |
| Invert last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1461 | none | PASS |
| Invert last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 77 | none | PASS |
| Invert last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 73 | none | PASS |
| Invert first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1041 | none | PASS |
| Invert first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 825 | none | PASS |
| Invert first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 77 | none | PASS |
| Invert first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 58 | none | PASS |
| Brightness & Contrast last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 363 | none | PASS |
| Brightness & Contrast last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 217 | none | PASS |
| Brightness & Contrast last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 29 | none | PASS |
| Brightness & Contrast last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 5 | none | PASS |
| Brightness & Contrast first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 28840 | none | PASS |
| Brightness & Contrast first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 26047 | none | PASS |
| Brightness & Contrast first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 1933 | none | PASS |
| Brightness & Contrast first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 1631 | none | PASS |
| Black & White last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1266 | none | PASS |
| Black & White last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 779 | none | PASS |
| Black & White last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 86 | none | PASS |
| Black & White last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 56 | none | PASS |
| Black & White first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1303 | none | PASS |
| Black & White first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 2101 | none | PASS |
| Black & White first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 88 | none | PASS |
| Black & White first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 108 | none | PASS |
| Posterize last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1 | none | PASS |
| Posterize last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3 | none | PASS |
| Posterize last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 0 | 0 | none | PASS |
| Posterize last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 0 | 0 | none | PASS |
| Posterize first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 14347 | none | PASS |
| Posterize first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 15228 | none | PASS |
| Posterize first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 851 | none | PASS |
| Posterize first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 916 | none | PASS |
| Threshold last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 0 | 0 | none | PASS |
| Threshold last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 0 | 0 | none | PASS |
| Threshold last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 0 | 0 | none | PASS |
| Threshold last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 0 | 0 | none | PASS |
| Threshold first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 98533 | none | PASS |
| Threshold first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 105305 | none | PASS |
| Threshold first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 6290 | none | PASS |
| Threshold first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 6637 | none | PASS |
| Channel Mixer last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1242 | none | PASS |
| Channel Mixer last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1426 | none | PASS |
| Channel Mixer last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 78 | none | PASS |
| Channel Mixer last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 87 | none | PASS |
| Channel Mixer first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 3592 | none | PASS |
| Channel Mixer first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3346 | none | PASS |
| Channel Mixer first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 190 | none | PASS |
| Channel Mixer first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 175 | none | PASS |
| Vibrance last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 746 | none | PASS |
| Vibrance last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 793 | none | PASS |
| Vibrance last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 44 | none | PASS |
| Vibrance last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 52 | none | PASS |
| Vibrance first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 949 | none | PASS |
| Vibrance first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 911 | none | PASS |
| Vibrance first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Vibrance first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 50 | none | PASS |
| Leave Color last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1042 | none | PASS |
| Leave Color last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1030 | none | PASS |
| Leave Color last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 74 | none | PASS |
| Leave Color last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Leave Color first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 921 | none | PASS |
| Leave Color first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 849 | none | PASS |
| Leave Color first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 60 | none | PASS |
| Leave Color first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 71 | none | PASS |
| Solarize last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1604 | none | PASS |
| Solarize last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1533 | none | PASS |
| Solarize last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 92 | none | PASS |
| Solarize last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 104 | none | PASS |
| Solarize first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1593 | none | PASS |
| Solarize first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 2023 | none | PASS |
| Solarize first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 77 | none | PASS |
| Solarize first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 89 | none | PASS |
| Halftone last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 0 | 0 | none | PASS |
| Halftone last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 0 | 0 | none | PASS |
| Halftone last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 0 | 0 | none | PASS |
| Halftone last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 0 | 0 | none | PASS |
| Halftone first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 138934 | none | PASS |
| Halftone first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 122256 | none | PASS |
| Halftone first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 8726 | none | PASS |
| Halftone first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 7632 | none | PASS |
| Mosaic last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 600 | none | PASS |
| Mosaic last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 610 | none | PASS |
| Mosaic last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 82 | none | PASS |
| Mosaic last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 42 | none | PASS |
| Mosaic first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1575 | none | PASS |
| Mosaic first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1461 | none | PASS |
| Mosaic first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 94 | none | PASS |
| Mosaic first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 123 | none | PASS |
| Emboss last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1519 | none | PASS |
| Emboss last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1527 | none | PASS |
| Emboss last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 1966 | none | PASS |
| Emboss last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 1907 | none | PASS |
| Emboss first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1158 | none | PASS |
| Emboss first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1195 | none | PASS |
| Emboss first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 130 | none | PASS |
| Emboss first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 97 | none | PASS |
| Find Edges last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 759 | none | PASS |
| Find Edges last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 813 | none | PASS |
| Find Edges last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 61 | none | PASS |
| Find Edges last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 60 | none | PASS |
| Find Edges first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 63506 | none | PASS |
| Find Edges first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 44374 | none | PASS |
| Find Edges first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 1918 | none | PASS |
| Find Edges first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 1017 | none | PASS |
| Sharpen last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 967 | none | PASS |
| Sharpen last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 910 | none | PASS |
| Sharpen last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 54 | none | PASS |
| Sharpen last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 53 | none | PASS |
| Sharpen first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 970 | none | PASS |
| Sharpen first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 869 | none | PASS |
| Sharpen first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 81 | none | PASS |
| Sharpen first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 80 | none | PASS |
| Diffusion last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 862 | none | PASS |
| Diffusion last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1262 | none | PASS |
| Diffusion last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 43 | none | PASS |
| Diffusion last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 77 | none | PASS |
| Diffusion first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 986 | none | PASS |
| Diffusion first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 870 | none | PASS |
| Diffusion first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 59 | none | PASS |
| Diffusion first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 54 | none | PASS |
| Wave Warp last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1003 | none | PASS |
| Wave Warp last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 935 | none | PASS |
| Wave Warp last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 80 | none | PASS |
| Wave Warp last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 59 | none | PASS |
| Wave Warp first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2647 | none | PASS |
| Wave Warp first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 2610 | none | PASS |
| Wave Warp first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 129 | none | PASS |
| Wave Warp first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 124 | none | PASS |
| Ripple last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 968 | none | PASS |
| Ripple last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 955 | none | PASS |
| Ripple last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 72 | none | PASS |
| Ripple last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 64 | none | PASS |
| Ripple first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2114 | none | PASS |
| Ripple first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 2200 | none | PASS |
| Ripple first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 103 | none | PASS |
| Ripple first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 107 | none | PASS |
| Twirl last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 957 | none | PASS |
| Twirl last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 986 | none | PASS |
| Twirl last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Twirl last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Twirl first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2991 | none | PASS |
| Twirl first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3101 | none | PASS |
| Twirl first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 159 | none | PASS |
| Twirl first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 153 | none | PASS |
| Bulge last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 955 | none | PASS |
| Bulge last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 987 | none | PASS |
| Bulge last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 56 | none | PASS |
| Bulge last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Bulge first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2986 | none | PASS |
| Bulge first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3101 | none | PASS |
| Bulge first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 161 | none | PASS |
| Bulge first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 153 | none | PASS |
| Mirror last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1004 | none | PASS |
| Mirror last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1014 | none | PASS |
| Mirror last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 62 | none | PASS |
| Mirror last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 54 | none | PASS |
| Mirror first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1079 | none | PASS |
| Mirror first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 733 | none | PASS |
| Mirror first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 37 | none | PASS |
| Mirror first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 34 | none | PASS |
| Linear Wipe last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 503 | none | PASS |
| Linear Wipe last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 18 | none | PASS |
| Linear Wipe last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 31 | none | PASS |
| Linear Wipe last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 2 | none | PASS |
| Linear Wipe first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 523 | none | PASS |
| Linear Wipe first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 123 | none | PASS |
| Linear Wipe first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 20 | none | PASS |
| Linear Wipe first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 0 | 0 | none | PASS |
| Radial Wipe last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 472 | none | PASS |
| Radial Wipe last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 973 | none | PASS |
| Radial Wipe last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 24 | none | PASS |
| Radial Wipe last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 52 | none | PASS |
| Radial Wipe first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2446 | none | PASS |
| Radial Wipe first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 2977 | none | PASS |
| Radial Wipe first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 136 | none | PASS |
| Radial Wipe first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 152 | none | PASS |
| Venetian Blinds last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 500 | none | PASS |
| Venetian Blinds last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 512 | none | PASS |
| Venetian Blinds last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 33 | none | PASS |
| Venetian Blinds last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 33 | none | PASS |
| Venetian Blinds first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1750 | none | PASS |
| Venetian Blinds first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1865 | none | PASS |
| Venetian Blinds first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 105 | none | PASS |
| Venetian Blinds first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 111 | none | PASS |
| Iris Wipe last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 954 | none | PASS |
| Iris Wipe last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 970 | none | PASS |
| Iris Wipe last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 54 | none | PASS |
| Iris Wipe last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 54 | none | PASS |
| Iris Wipe first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2955 | none | PASS |
| Iris Wipe first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3086 | none | PASS |
| Iris Wipe first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 154 | none | PASS |
| Iris Wipe first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 150 | none | PASS |
| Simple Choker last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 974 | none | PASS |
| Simple Choker last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 989 | none | PASS |
| Simple Choker last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Simple Choker last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 54 | none | PASS |
| Simple Choker first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2969 | none | PASS |
| Simple Choker first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3095 | none | PASS |
| Simple Choker first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 156 | none | PASS |
| Simple Choker first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 152 | none | PASS |
| Speed Lines last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 787 | none | PASS |
| Speed Lines last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 667 | none | PASS |
| Speed Lines last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 57 | none | PASS |
| Speed Lines last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 60 | none | PASS |
| Speed Lines first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1730 | none | PASS |
| Speed Lines first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1625 | none | PASS |
| Speed Lines first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 101 | none | PASS |
| Speed Lines first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 84 | none | PASS |
| Cross Glare last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 457 | none | PASS |
| Cross Glare last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 190 | none | PASS |
| Cross Glare last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 32 | none | PASS |
| Cross Glare last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 17 | none | PASS |
| Cross Glare first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 22604 | none | PASS |
| Cross Glare first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 45608 | none | PASS |
| Cross Glare first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 1414 | none | PASS |
| Cross Glare first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 2901 | none | PASS |
| Camera Shake last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 908 | none | PASS |
| Camera Shake last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 973 | none | PASS |
| Camera Shake last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 52 | none | PASS |
| Camera Shake last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 70 | none | PASS |
| Camera Shake first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2599 | none | PASS |
| Camera Shake first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 2581 | none | PASS |
| Camera Shake first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 122 | none | PASS |
| Camera Shake first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 116 | none | PASS |
| Rain last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 981 | none | PASS |
| Rain last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 996 | none | PASS |
| Rain last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 58 | none | PASS |
| Rain last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Rain first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2961 | none | PASS |
| Rain first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3072 | none | PASS |
| Rain first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 156 | none | PASS |
| Rain first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 144 | none | PASS |
| Motion Tile last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 975 | none | PASS |
| Motion Tile last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 991 | none | PASS |
| Motion Tile last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Motion Tile last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 54 | none | PASS |
| Motion Tile first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2969 | none | PASS |
| Motion Tile first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3100 | none | PASS |
| Motion Tile first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 156 | none | PASS |
| Motion Tile first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 152 | none | PASS |
| Color Lookup last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1032 | none | PASS |
| Color Lookup last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 933 | none | PASS |
| Color Lookup last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 54 | none | PASS |
| Color Lookup last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 64 | none | PASS |
| Color Lookup first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 626 | none | PASS |
| Color Lookup first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 505 | none | PASS |
| Color Lookup first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 60 | none | PASS |
| Color Lookup first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 53 | none | PASS |
| Line Blur last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1013 | none | PASS |
| Line Blur last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1007 | none | PASS |
| Line Blur last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 62 | none | PASS |
| Line Blur last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 53 | none | PASS |
| Line Blur first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2031 | none | PASS |
| Line Blur first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 2057 | none | PASS |
| Line Blur first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 75 | none | PASS |
| Line Blur first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 75 | none | PASS |
| HSV Key last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 1 | 1 | 757 | none | PASS |
| HSV Key last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 1 | 1 | 915 | none | PASS |
| HSV Key last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 45 | none | PASS |
| HSV Key last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 51 | none | PASS |
| HSV Key first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2869 | none | PASS |
| HSV Key first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3020 | none | PASS |
| HSV Key first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 140 | none | PASS |
| HSV Key first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 147 | none | PASS |
| Paraffin last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 1 | 1 | 908 | none | PASS |
| Paraffin last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 1 | 1 | 999 | none | PASS |
| Paraffin last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Paraffin last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 64 | none | PASS |
| Paraffin first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 793 | none | PASS |
| Paraffin first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 614 | none | PASS |
| Paraffin first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 62 | none | PASS |
| Paraffin first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 45 | none | PASS |
| Kira-kira last, after Levels and a Gaussian Blur frame 0, Full | 1 / 1 / 1 | 1 | 1 | 957 | none | PASS |
| Kira-kira last, after Levels and a Gaussian Blur frame 100, Full | 1 / 1 / 1 | 1 | 1 | 979 | none | PASS |
| Kira-kira last, after Levels and a Gaussian Blur frame 0, Draft | 1 / 1 / 1 | — | 1 | 54 | none | PASS |
| Kira-kira last, after Levels and a Gaussian Blur frame 100, Draft | 1 / 1 / 1 | — | 1 | 53 | none | PASS |
| Kira-kira first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 3029 | none | PASS |
| Kira-kira first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3185 | none | PASS |
| Kira-kira first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 175 | none | PASS |
| Kira-kira first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 172 | none | PASS |
| Median last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1004 | none | PASS |
| Median last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 991 | none | PASS |
| Median last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Median last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 52 | none | PASS |
| Median first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2930 | none | PASS |
| Median first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3000 | none | PASS |
| Median first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 161 | none | PASS |
| Median first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 160 | none | PASS |
| Smart Blur last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1018 | none | PASS |
| Smart Blur last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 956 | none | PASS |
| Smart Blur last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 64 | none | PASS |
| Smart Blur last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Smart Blur first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1547 | none | PASS |
| Smart Blur first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1480 | none | PASS |
| Smart Blur first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 105 | none | PASS |
| Smart Blur first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 84 | none | PASS |
| Roughen Edges last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 971 | none | PASS |
| Roughen Edges last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 988 | none | PASS |
| Roughen Edges last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 62 | none | PASS |
| Roughen Edges last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 66 | none | PASS |
| Roughen Edges first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2975 | none | PASS |
| Roughen Edges first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3106 | none | PASS |
| Roughen Edges first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 156 | none | PASS |
| Roughen Edges first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 152 | none | PASS |
| Radial Shadow last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 971 | none | PASS |
| Radial Shadow last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 980 | none | PASS |
| Radial Shadow last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 58 | none | PASS |
| Radial Shadow last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 58 | none | PASS |
| Radial Shadow first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2968 | none | PASS |
| Radial Shadow first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3096 | none | PASS |
| Radial Shadow first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 156 | none | PASS |
| Radial Shadow first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 152 | none | PASS |
| Bevel Alpha last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 973 | none | PASS |
| Bevel Alpha last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 989 | none | PASS |
| Bevel Alpha last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Bevel Alpha last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 54 | none | PASS |
| Bevel Alpha first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2971 | none | PASS |
| Bevel Alpha first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3102 | none | PASS |
| Bevel Alpha first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 156 | none | PASS |
| Bevel Alpha first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 152 | none | PASS |
| Snowfall last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 976 | none | PASS |
| Snowfall last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 996 | none | PASS |
| Snowfall last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 56 | none | PASS |
| Snowfall last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Snowfall first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2946 | none | PASS |
| Snowfall first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 3036 | none | PASS |
| Snowfall first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 154 | none | PASS |
| Snowfall first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 143 | none | PASS |
| Cell Pattern last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 899 | none | PASS |
| Cell Pattern last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1011 | none | PASS |
| Cell Pattern last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 47 | none | PASS |
| Cell Pattern last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 62 | none | PASS |
| Cell Pattern first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 982 | none | PASS |
| Cell Pattern first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 973 | none | PASS |
| Cell Pattern first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 63 | none | PASS |
| Cell Pattern first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 65 | none | PASS |
| Polar Coordinates last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 695 | none | PASS |
| Polar Coordinates last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1037 | none | PASS |
| Polar Coordinates last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 45 | none | PASS |
| Polar Coordinates last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 70 | none | PASS |
| Polar Coordinates first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1743 | none | PASS |
| Polar Coordinates first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 2183 | none | PASS |
| Polar Coordinates first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 76 | none | PASS |
| Polar Coordinates first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 116 | none | PASS |
| Optics Compensation last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 964 | none | PASS |
| Optics Compensation last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 932 | none | PASS |
| Optics Compensation last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 51 | none | PASS |
| Optics Compensation last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 57 | none | PASS |
| Optics Compensation first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 2104 | none | PASS |
| Optics Compensation first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 2213 | none | PASS |
| Optics Compensation first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 115 | none | PASS |
| Optics Compensation first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 114 | none | PASS |
| Corner Pin last, after Levels and a Gaussian Blur frame 0, Full | 3 / 3 / 2 | 3 | 1 | 886 | none | PASS |
| Corner Pin last, after Levels and a Gaussian Blur frame 100, Full | 3 / 3 / 2 | 3 | 1 | 962 | none | PASS |
| Corner Pin last, after Levels and a Gaussian Blur frame 0, Draft | 3 / 3 / 2 | — | 1 | 57 | none | PASS |
| Corner Pin last, after Levels and a Gaussian Blur frame 100, Draft | 3 / 3 / 2 | — | 1 | 55 | none | PASS |
| Corner Pin first, before Hue/Saturation and a Vignette frame 0, Full | 3 / 3 / 2 | 3 | 1 | 1455 | none | PASS |
| Corner Pin first, before Hue/Saturation and a Vignette frame 100, Full | 3 / 3 / 2 | 3 | 1 | 1957 | none | PASS |
| Corner Pin first, before Hue/Saturation and a Vignette frame 0, Draft | 3 / 3 / 2 | — | 1 | 59 | none | PASS |
| Corner Pin first, before Hue/Saturation and a Vignette frame 100, Draft | 3 / 3 / 2 | — | 1 | 89 | none | PASS |
| Levels, then Kaleidoscope (not on the card), then a Gaussian Blur and Curves frame 0, Full | 2 / 1 / 2 | 2 | 1 | 24826 | none | PASS |
| Levels, then Kaleidoscope (not on the card), then a Gaussian Blur and Curves frame 100, Full | 2 / 1 / 2 | 2 | 1 | 24337 | none | PASS |
| Levels, then Kaleidoscope (not on the card), then a Gaussian Blur and Curves frame 0, Draft | 2 / 1 / 2 | — | 1 | 1348 | none | PASS |
| Levels, then Kaleidoscope (not on the card), then a Gaussian Blur and Curves frame 100, Draft | 2 / 1 / 2 | — | 1 | 1322 | none | PASS |
