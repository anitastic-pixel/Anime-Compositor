# B-173: the card keeps the picture of each run it has drawn

Written by `tests/b173_draft_moving.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Each effect the card draws that moves by itself from frame to frame, and three the card draws with one setting keyed across the shot, is put on the reference shot's first three layers (the second layer's after a Drop Shadow, as B-151's check does), and B-155's moving Noise first, before a run of four. A Median, which does not move, is the control.

**The rule (D-246):** frame 0, asked for again after frame 100, runs **no pass** on the card beyond the one that lays each layer, and gives **the very picture** (every byte) the card drew for it the first time. Every run stays the card's, at Draft as at Full. Each frame drawn also compares the eight-bit picture the page receives, drawn by the CPU and by the GPU: **no channel of any pixel more than 1 level of 255 apart**, the same warnings on both, and the card drawing the frame itself.

**34 of 34 frames asked for again pass; 68 of 68 frames drawn pass.**

The largest difference is in "Gaussian Blur, radius keyed 2 to 40 on three layers frame 100, Full": 1 of 255, pixels differing: 26910.

## Asked for again

| Case | Effects | Passes the card ran for effects | Largest difference from its first picture (of 255) | Pixels differing | Result |
|---|---|---:|---:|---:|---|
| Noise on three layers frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| Noise on three layers frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| Exposure Flicker on three layers frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| Exposure Flicker on three layers frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| Turbulent Displace on three layers frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| Turbulent Displace on three layers frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| Fractal Noise on three layers frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| Fractal Noise on three layers frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| Ripple on three layers frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| Ripple on three layers frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| Wave Warp on three layers frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| Wave Warp on three layers frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| Speed Lines on three layers frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| Speed Lines on three layers frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| Camera Shake on three layers frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| Camera Shake on three layers frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| Rain on three layers frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| Rain on three layers frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| Snowfall on three layers frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| Snowfall on three layers frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| Roughen Edges on three layers frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| Roughen Edges on three layers frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| Kira-kira on three layers frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| Kira-kira on three layers frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| Gaussian Blur, radius keyed 2 to 40 on three layers frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| Gaussian Blur, radius keyed 2 to 40 on three layers frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| Bloom, intensity keyed 0.5 to 3 on three layers frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| Bloom, intensity keyed 0.5 to 3 on three layers frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| Directional Blur, length keyed 5 to 80 on three layers frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| Directional Blur, length keyed 5 to 80 on three layers frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| B-155's moving Noise first, before Levels, a Gaussian Blur, Hue/Saturation and a Vignette frame 0 again, Full | moves | 0 | 0 | 0 | PASS |
| B-155's moving Noise first, before Levels, a Gaussian Blur, Hue/Saturation and a Vignette frame 0 again, Draft | moves | 0 | 0 | 0 | PASS |
| Median, which does not move, on three layers frame 0 again, Full | still | 0 | 0 | 0 | PASS |
| Median, which does not move, on three layers frame 0 again, Draft | still | 0 | 0 | 0 | PASS |

## Every frame drawn

Effects left to the card on the first three layers, and what they must be.

| Case | Left to the card | Must be | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---:|---|---|
| Noise on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 961 | none | PASS |
| Noise on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1048 | none | PASS |
| Noise on three layers frame 0, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 65 | none | PASS |
| Noise on three layers frame 100, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 67 | none | PASS |
| Exposure Flicker on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1 | none | PASS |
| Exposure Flicker on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1605 | none | PASS |
| Exposure Flicker on three layers frame 0, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 26 | none | PASS |
| Exposure Flicker on three layers frame 100, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 48 | none | PASS |
| Turbulent Displace on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1820 | none | PASS |
| Turbulent Displace on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 973 | none | PASS |
| Turbulent Displace on three layers frame 0, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 68 | none | PASS |
| Turbulent Displace on three layers frame 100, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 82 | none | PASS |
| Fractal Noise on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 937 | none | PASS |
| Fractal Noise on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1016 | none | PASS |
| Fractal Noise on three layers frame 0, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 65 | none | PASS |
| Fractal Noise on three layers frame 100, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 66 | none | PASS |
| Ripple on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1981 | none | PASS |
| Ripple on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 960 | none | PASS |
| Ripple on three layers frame 0, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 75 | none | PASS |
| Ripple on three layers frame 100, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 50 | none | PASS |
| Wave Warp on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 2137 | none | PASS |
| Wave Warp on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 953 | none | PASS |
| Wave Warp on three layers frame 0, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 86 | none | PASS |
| Wave Warp on three layers frame 100, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 60 | none | PASS |
| Speed Lines on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 2850 | none | PASS |
| Speed Lines on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 990 | none | PASS |
| Speed Lines on three layers frame 0, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 94 | none | PASS |
| Speed Lines on three layers frame 100, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 48 | none | PASS |
| Camera Shake on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1735 | none | PASS |
| Camera Shake on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 906 | none | PASS |
| Camera Shake on three layers frame 0, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 58 | none | PASS |
| Camera Shake on three layers frame 100, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 58 | none | PASS |
| Rain on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 3721 | none | PASS |
| Rain on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1414 | none | PASS |
| Rain on three layers frame 0, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 124 | none | PASS |
| Rain on three layers frame 100, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 39 | none | PASS |
| Snowfall on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 3711 | none | PASS |
| Snowfall on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1404 | none | PASS |
| Snowfall on three layers frame 0, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 130 | none | PASS |
| Snowfall on three layers frame 100, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 33 | none | PASS |
| Roughen Edges on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 3781 | none | PASS |
| Roughen Edges on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1418 | none | PASS |
| Roughen Edges on three layers frame 0, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 128 | none | PASS |
| Roughen Edges on three layers frame 100, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 33 | none | PASS |
| Kira-kira on three layers frame 0, Full | 1 / 1 / 1 | 1 / 1 / 1 | 1 | 3783 | none | PASS |
| Kira-kira on three layers frame 100, Full | 1 / 1 / 1 | 1 / 1 / 1 | 1 | 1418 | none | PASS |
| Kira-kira on three layers frame 0, Draft | 1 / 1 / 1 | 1 / 1 / 1 | 1 | 128 | none | PASS |
| Kira-kira on three layers frame 100, Draft | 1 / 1 / 1 | 1 / 1 / 1 | 1 | 31 | none | PASS |
| Gaussian Blur, radius keyed 2 to 40 on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 20553 | none | PASS |
| Gaussian Blur, radius keyed 2 to 40 on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 26910 | none | PASS |
| Gaussian Blur, radius keyed 2 to 40 on three layers frame 0, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 2176 | none | PASS |
| Gaussian Blur, radius keyed 2 to 40 on three layers frame 100, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 774 | none | PASS |
| Bloom, intensity keyed 0.5 to 3 on three layers frame 0, Full | 1 / 1 / 1 | 1 / 1 / 1 | 1 | 2013 | none | PASS |
| Bloom, intensity keyed 0.5 to 3 on three layers frame 100, Full | 1 / 1 / 1 | 1 / 1 / 1 | 1 | 1844 | none | PASS |
| Bloom, intensity keyed 0.5 to 3 on three layers frame 0, Draft | 1 / 1 / 1 | 1 / 1 / 1 | 1 | 84 | none | PASS |
| Bloom, intensity keyed 0.5 to 3 on three layers frame 100, Draft | 1 / 1 / 1 | 1 / 1 / 1 | 1 | 68 | none | PASS |
| Directional Blur, length keyed 5 to 80 on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 12582 | none | PASS |
| Directional Blur, length keyed 5 to 80 on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 22521 | none | PASS |
| Directional Blur, length keyed 5 to 80 on three layers frame 0, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 3753 | none | PASS |
| Directional Blur, length keyed 5 to 80 on three layers frame 100, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1545 | none | PASS |
| B-155's moving Noise first, before Levels, a Gaussian Blur, Hue/Saturation and a Vignette frame 0, Full | 5 / 4 / 3 | 5 / 4 / 3 | 1 | 1003 | none | PASS |
| B-155's moving Noise first, before Levels, a Gaussian Blur, Hue/Saturation and a Vignette frame 100, Full | 5 / 4 / 3 | 5 / 4 / 3 | 1 | 954 | none | PASS |
| B-155's moving Noise first, before Levels, a Gaussian Blur, Hue/Saturation and a Vignette frame 0, Draft | 5 / 4 / 3 | 5 / 4 / 3 | 1 | 48 | none | PASS |
| B-155's moving Noise first, before Levels, a Gaussian Blur, Hue/Saturation and a Vignette frame 100, Draft | 5 / 4 / 3 | 5 / 4 / 3 | 1 | 48 | none | PASS |
| Median, which does not move, on three layers frame 0, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 3814 | none | PASS |
| Median, which does not move, on three layers frame 100, Full | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 1731 | none | PASS |
| Median, which does not move, on three layers frame 0, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 132 | none | PASS |
| Median, which does not move, on three layers frame 100, Draft | 1 / 2 / 1 | 1 / 2 / 1 | 1 | 24 | none | PASS |
