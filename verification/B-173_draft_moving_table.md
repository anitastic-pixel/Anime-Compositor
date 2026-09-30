# B-173: moving effects at Draft stay on the CPU

Written by `tests/b173_draft_moving.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Each effect the card draws that moves by itself from frame to frame, and three the card draws with one setting keyed across the shot, is put on the reference shot's first three layers (the second layer's after a Drop Shadow, as B-151's check does), and B-155's moving Noise first, before a run of four. A Median, which does not move, is the control.

**The rule (D-246):** at Draft, a layer with an effect set differently at the next frame keeps its stack on the CPU, so the card is left **0** effects on it; at Full, and on a layer whose effects stay the same, the card is left what it was before. Each row also compares the eight-bit picture the page receives, drawn by the CPU and by the GPU: **no channel of any pixel more than 1 level of 255 apart**, the same warnings on both, and the card drawing the frame itself.

**36 of 68 checks pass.**

The largest difference is in "Gaussian Blur, radius keyed 2 to 40 on three layers frame 100, Full": 1 of 255, pixels differing: 26910.

## Every frame

Effects left to the card on the first three layers, and what they must be.

| Case | Left to the card | Must be | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---:|---|---|
| Noise on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 961 | none | PASS |
| Noise on three layers frame 0, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 65 | none | FAIL: not where it should be drawn |
| Noise on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1048 | none | PASS |
| Noise on three layers frame 100, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 67 | none | FAIL: not where it should be drawn |
| Exposure Flicker on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1 | none | PASS |
| Exposure Flicker on three layers frame 0, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 26 | none | FAIL: not where it should be drawn |
| Exposure Flicker on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1605 | none | PASS |
| Exposure Flicker on three layers frame 100, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 48 | none | FAIL: not where it should be drawn |
| Turbulent Displace on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1820 | none | PASS |
| Turbulent Displace on three layers frame 0, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 68 | none | FAIL: not where it should be drawn |
| Turbulent Displace on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 973 | none | PASS |
| Turbulent Displace on three layers frame 100, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 82 | none | FAIL: not where it should be drawn |
| Fractal Noise on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 937 | none | PASS |
| Fractal Noise on three layers frame 0, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 65 | none | FAIL: not where it should be drawn |
| Fractal Noise on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1016 | none | PASS |
| Fractal Noise on three layers frame 100, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 66 | none | FAIL: not where it should be drawn |
| Ripple on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1981 | none | PASS |
| Ripple on three layers frame 0, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 75 | none | FAIL: not where it should be drawn |
| Ripple on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 960 | none | PASS |
| Ripple on three layers frame 100, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 50 | none | FAIL: not where it should be drawn |
| Wave Warp on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 2137 | none | PASS |
| Wave Warp on three layers frame 0, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 86 | none | FAIL: not where it should be drawn |
| Wave Warp on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 953 | none | PASS |
| Wave Warp on three layers frame 100, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 60 | none | FAIL: not where it should be drawn |
| Speed Lines on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 2850 | none | PASS |
| Speed Lines on three layers frame 0, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 94 | none | FAIL: not where it should be drawn |
| Speed Lines on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 990 | none | PASS |
| Speed Lines on three layers frame 100, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 48 | none | FAIL: not where it should be drawn |
| Camera Shake on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1735 | none | PASS |
| Camera Shake on three layers frame 0, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 58 | none | FAIL: not where it should be drawn |
| Camera Shake on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 906 | none | PASS |
| Camera Shake on three layers frame 100, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 58 | none | FAIL: not where it should be drawn |
| Rain on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 3721 | none | PASS |
| Rain on three layers frame 0, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 124 | none | FAIL: not where it should be drawn |
| Rain on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1414 | none | PASS |
| Rain on three layers frame 100, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 39 | none | FAIL: not where it should be drawn |
| Snowfall on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 3711 | none | PASS |
| Snowfall on three layers frame 0, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 130 | none | FAIL: not where it should be drawn |
| Snowfall on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1404 | none | PASS |
| Snowfall on three layers frame 100, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 33 | none | FAIL: not where it should be drawn |
| Roughen Edges on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 3781 | none | PASS |
| Roughen Edges on three layers frame 0, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 128 | none | FAIL: not where it should be drawn |
| Roughen Edges on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1418 | none | PASS |
| Roughen Edges on three layers frame 100, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 33 | none | FAIL: not where it should be drawn |
| Kira-kira on three layers frame 0, Full | 1 / 1 / 1 | 1 / 1 / 1 | 1 | 3783 | none | PASS |
| Kira-kira on three layers frame 0, Draft | 1 / 1 / 1 | 0 / 0 / 0 | 1 | 128 | none | FAIL: not where it should be drawn |
| Kira-kira on three layers frame 100, Full | 1 / 1 / 1 | 1 / 1 / 1 | 1 | 1418 | none | PASS |
| Kira-kira on three layers frame 100, Draft | 1 / 1 / 1 | 0 / 0 / 0 | 1 | 31 | none | FAIL: not where it should be drawn |
| Gaussian Blur, radius keyed 2 to 40 on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 20553 | none | PASS |
| Gaussian Blur, radius keyed 2 to 40 on three layers frame 0, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 2176 | none | FAIL: not where it should be drawn |
| Gaussian Blur, radius keyed 2 to 40 on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 26910 | none | PASS |
| Gaussian Blur, radius keyed 2 to 40 on three layers frame 100, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 774 | none | FAIL: not where it should be drawn |
| Bloom, intensity keyed 0.5 to 3 on three layers frame 0, Full | 1 / 1 / 1 | 1 / 1 / 1 | 1 | 2013 | none | PASS |
| Bloom, intensity keyed 0.5 to 3 on three layers frame 0, Draft | 1 / 1 / 1 | 0 / 0 / 0 | 1 | 84 | none | FAIL: not where it should be drawn |
| Bloom, intensity keyed 0.5 to 3 on three layers frame 100, Full | 1 / 1 / 1 | 1 / 1 / 1 | 1 | 1844 | none | PASS |
| Bloom, intensity keyed 0.5 to 3 on three layers frame 100, Draft | 1 / 1 / 1 | 0 / 0 / 0 | 1 | 68 | none | FAIL: not where it should be drawn |
| Directional Blur, length keyed 5 to 80 on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 12582 | none | PASS |
| Directional Blur, length keyed 5 to 80 on three layers frame 0, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 3753 | none | FAIL: not where it should be drawn |
| Directional Blur, length keyed 5 to 80 on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 22521 | none | PASS |
| Directional Blur, length keyed 5 to 80 on three layers frame 100, Draft | 1 / 2 / 1 | 0 / 0 / 0 | 1 | 1545 | none | FAIL: not where it should be drawn |
| B-155's moving Noise first, before Levels, a Gaussian Blur, Hue/Saturation and a Vignette frame 0, Full | 5 / 4 / 3 | 5 / 4 / 3 | 1 | 1003 | none | PASS |
| B-155's moving Noise first, before Levels, a Gaussian Blur, Hue/Saturation and a Vignette frame 0, Draft | 5 / 4 / 3 | 0 / 0 / 3 | 1 | 48 | none | FAIL: not where it should be drawn |
| B-155's moving Noise first, before Levels, a Gaussian Blur, Hue/Saturation and a Vignette frame 100, Full | 5 / 4 / 3 | 5 / 4 / 3 | 1 | 954 | none | PASS |
| B-155's moving Noise first, before Levels, a Gaussian Blur, Hue/Saturation and a Vignette frame 100, Draft | 5 / 4 / 3 | 0 / 0 / 3 | 1 | 48 | none | FAIL: not where it should be drawn |
| Median, which does not move, on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 3814 | none | PASS |
| Median, which does not move, on three layers frame 0, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 132 | none | PASS |
| Median, which does not move, on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1731 | none | PASS |
| Median, which does not move, on three layers frame 100, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 24 | none | PASS |
