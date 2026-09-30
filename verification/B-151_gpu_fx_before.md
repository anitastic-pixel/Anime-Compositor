# B-151 before the build: the checks run on commit 9624f87

The first run of `tests/b151_gpu_fx.rs`, before any of the eleven was put on the card. Every reference shot row fails because the effect is not yet left to the card, as the check requires. Seventeen Kaleidoscope fixture rows also fail today with nothing of the effect on the card: a layer the CPU finishes is sent to the card in half precision, which turns an alpha of about 1e-15 into 0, and the colour of that invisible pixel, which the page never shows, then reads 0 where the CPU's reads the colour. A layer whose last effect the card does is sent in full precision (D-122), which is what B-151 does for Kaleidoscope.


Written by `tests/b151_gpu_fx.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Each row compares the eight-bit picture the page receives, drawn by the CPU and by the GPU, with the layer's last effect, one of the eleven, done on the card. **The rule: no channel of any pixel more than 1 level of 255 apart** (D-217). An effect that changes nothing or whose settings are invalid is not left to the card; on those rows any difference is the card's layering, held to the same 1 level by D-100. On a reference shot row the effect must in fact be left to the card. On every row both paths must give the same warnings, and the card must draw the frame itself, except a frame with an adjustment layer, which the CPU draws by B-44's rule: that one must be the CPU's picture exactly, the card's message `GPU_PREVIEW_ON_CPU` its only extra warning.

**2225 of 2308 checks pass.**

The worst comparison is "fx_kaleido_005 frame 0, Full": largest difference 246 of 255, pixels differing: 5. Its pictures are in `verification/B-151 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square around every pixel where they do not.

## Each effect

Every fixture frame of the effect and the reference shot with it, at Full and Draft.

| Effect | Frames compared | Frames with it on the card | Largest difference (of 255) | Pass |
|---|---:|---:|---:|---|
| Median | 116 | 0 | 1 | 110 of 116 |
| Smart Blur | 126 | 0 | 1 | 120 of 126 |
| Roughen Edges | 246 | 0 | 1 | 240 of 246 |
| Radial Shadow | 256 | 0 | 1 | 250 of 256 |
| Bevel Alpha | 156 | 0 | 1 | 150 of 156 |
| Snowfall | 296 | 0 | 1 | 290 of 296 |
| Cell Pattern | 336 | 0 | 1 | 330 of 336 |
| Kaleidoscope | 206 | 0 | 246 | 183 of 206 |
| Polar Coordinates | 156 | 0 | 1 | 150 of 156 |
| Optics Compensation | 206 | 0 | 1 | 200 of 206 |
| Corner Pin | 186 | 0 | 1 | 180 of 186 |

## The rows that fail

| Case | Effects left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |
|---|---:|---:|---:|---|---|
| fx_kaleido_002 frame 0, Draft | 0 | 208 | 6 | none | FAIL |
| fx_kaleido_002 frame 1, Draft | 0 | 208 | 6 | none | FAIL |
| fx_kaleido_002 frame 2, Draft | 0 | 208 | 6 | none | FAIL |
| fx_kaleido_002 frame 3, Draft | 0 | 208 | 6 | none | FAIL |
| fx_kaleido_002 frame 4, Draft | 0 | 208 | 6 | none | FAIL |
| fx_kaleido_003 frame 0, Draft | 0 | 208 | 6 | none | FAIL |
| fx_kaleido_003 frame 1, Draft | 0 | 208 | 6 | none | FAIL |
| fx_kaleido_003 frame 2, Draft | 0 | 208 | 6 | none | FAIL |
| fx_kaleido_003 frame 3, Draft | 0 | 208 | 6 | none | FAIL |
| fx_kaleido_003 frame 4, Draft | 0 | 208 | 6 | none | FAIL |
| fx_kaleido_005 frame 0, Full | 0 | 246 | 5 | none | FAIL |
| fx_kaleido_005 frame 1, Full | 0 | 246 | 5 | none | FAIL |
| fx_kaleido_005 frame 2, Full | 0 | 246 | 5 | none | FAIL |
| fx_kaleido_005 frame 3, Full | 0 | 246 | 5 | none | FAIL |
| fx_kaleido_005 frame 4, Full | 0 | 246 | 5 | none | FAIL |
| fx_kaleido_010 frame 2, Full | 0 | 246 | 5 | none | FAIL |
| fx_kaleido_010 frame 4, Full | 0 | 246 | 1 | none | FAIL |
| the reference shot with Median frame 0, Full | 0 | 1 | 6550 | none | FAIL: the effect was not left to the card |
| the reference shot with Median frame 100, Full | 0 | 1 | 7993 | none | FAIL: the effect was not left to the card |
| the reference shot with Median frame 239, Full | 0 | 1 | 6863 | none | FAIL: the effect was not left to the card |
| the reference shot with Median frame 0, Draft | 0 | 1 | 16972 | none | FAIL: the effect was not left to the card |
| the reference shot with Median frame 100, Draft | 0 | 1 | 16941 | none | FAIL: the effect was not left to the card |
| the reference shot with Median frame 239, Draft | 0 | 1 | 17204 | none | FAIL: the effect was not left to the card |
| the reference shot with Smart Blur frame 0, Full | 0 | 1 | 48785 | none | FAIL: the effect was not left to the card |
| the reference shot with Smart Blur frame 100, Full | 0 | 1 | 49790 | none | FAIL: the effect was not left to the card |
| the reference shot with Smart Blur frame 239, Full | 0 | 1 | 48585 | none | FAIL: the effect was not left to the card |
| the reference shot with Smart Blur frame 0, Draft | 0 | 1 | 4376 | none | FAIL: the effect was not left to the card |
| the reference shot with Smart Blur frame 100, Draft | 0 | 1 | 4280 | none | FAIL: the effect was not left to the card |
| the reference shot with Smart Blur frame 239, Draft | 0 | 1 | 4380 | none | FAIL: the effect was not left to the card |
| the reference shot with Roughen Edges frame 0, Full | 0 | 1 | 5157 | none | FAIL: the effect was not left to the card |
| the reference shot with Roughen Edges frame 100, Full | 0 | 1 | 5783 | none | FAIL: the effect was not left to the card |
| the reference shot with Roughen Edges frame 239, Full | 0 | 1 | 5400 | none | FAIL: the effect was not left to the card |
| the reference shot with Roughen Edges frame 0, Draft | 0 | 1 | 16618 | none | FAIL: the effect was not left to the card |
| the reference shot with Roughen Edges frame 100, Draft | 0 | 1 | 16658 | none | FAIL: the effect was not left to the card |
| the reference shot with Roughen Edges frame 239, Draft | 0 | 1 | 16829 | none | FAIL: the effect was not left to the card |
| the reference shot with Radial Shadow frame 0, Full | 0 | 1 | 7030 | none | FAIL: the effect was not left to the card |
| the reference shot with Radial Shadow frame 100, Full | 0 | 1 | 9572 | none | FAIL: the effect was not left to the card |
| the reference shot with Radial Shadow frame 239, Full | 0 | 1 | 6652 | none | FAIL: the effect was not left to the card |
| the reference shot with Radial Shadow frame 0, Draft | 0 | 1 | 16414 | none | FAIL: the effect was not left to the card |
| the reference shot with Radial Shadow frame 100, Draft | 0 | 1 | 16062 | none | FAIL: the effect was not left to the card |
| the reference shot with Radial Shadow frame 239, Draft | 0 | 1 | 16453 | none | FAIL: the effect was not left to the card |
| the reference shot with Bevel Alpha frame 0, Full | 0 | 1 | 7545 | none | FAIL: the effect was not left to the card |
| the reference shot with Bevel Alpha frame 100, Full | 0 | 1 | 8329 | none | FAIL: the effect was not left to the card |
| the reference shot with Bevel Alpha frame 239, Full | 0 | 1 | 7921 | none | FAIL: the effect was not left to the card |
| the reference shot with Bevel Alpha frame 0, Draft | 0 | 1 | 16504 | none | FAIL: the effect was not left to the card |
| the reference shot with Bevel Alpha frame 100, Draft | 0 | 1 | 16519 | none | FAIL: the effect was not left to the card |
| the reference shot with Bevel Alpha frame 239, Draft | 0 | 1 | 16681 | none | FAIL: the effect was not left to the card |
| the reference shot with Snowfall frame 0, Full | 0 | 1 | 6810 | none | FAIL: the effect was not left to the card |
| the reference shot with Snowfall frame 100, Full | 0 | 1 | 7158 | none | FAIL: the effect was not left to the card |
| the reference shot with Snowfall frame 239, Full | 0 | 1 | 6962 | none | FAIL: the effect was not left to the card |
| the reference shot with Snowfall frame 0, Draft | 0 | 1 | 16221 | none | FAIL: the effect was not left to the card |
| the reference shot with Snowfall frame 100, Draft | 0 | 1 | 16241 | none | FAIL: the effect was not left to the card |
| the reference shot with Snowfall frame 239, Draft | 0 | 1 | 16409 | none | FAIL: the effect was not left to the card |
| the reference shot with Cell Pattern frame 0, Full | 0 | 1 | 4782 | EFFECT_PARAMETER_INVALID, on both | FAIL: the effect was not left to the card |
| the reference shot with Cell Pattern frame 100, Full | 0 | 1 | 5437 | EFFECT_PARAMETER_INVALID, on both | FAIL: the effect was not left to the card |
| the reference shot with Cell Pattern frame 239, Full | 0 | 1 | 5069 | EFFECT_PARAMETER_INVALID, on both | FAIL: the effect was not left to the card |
| the reference shot with Cell Pattern frame 0, Draft | 0 | 1 | 16961 | EFFECT_PARAMETER_INVALID, on both | FAIL: the effect was not left to the card |
| the reference shot with Cell Pattern frame 100, Draft | 0 | 1 | 17039 | EFFECT_PARAMETER_INVALID, on both | FAIL: the effect was not left to the card |
| the reference shot with Cell Pattern frame 239, Draft | 0 | 1 | 17155 | EFFECT_PARAMETER_INVALID, on both | FAIL: the effect was not left to the card |
| the reference shot with Kaleidoscope frame 0, Full | 0 | 1 | 35349 | none | FAIL: the effect was not left to the card |
| the reference shot with Kaleidoscope frame 100, Full | 0 | 1 | 35414 | none | FAIL: the effect was not left to the card |
| the reference shot with Kaleidoscope frame 239, Full | 0 | 1 | 33124 | none | FAIL: the effect was not left to the card |
| the reference shot with Kaleidoscope frame 0, Draft | 0 | 1 | 4242 | none | FAIL: the effect was not left to the card |
| the reference shot with Kaleidoscope frame 100, Draft | 0 | 1 | 4303 | none | FAIL: the effect was not left to the card |
| the reference shot with Kaleidoscope frame 239, Draft | 0 | 1 | 3815 | none | FAIL: the effect was not left to the card |
| the reference shot with Polar Coordinates frame 0, Full | 0 | 1 | 35828 | none | FAIL: the effect was not left to the card |
| the reference shot with Polar Coordinates frame 100, Full | 0 | 1 | 35363 | none | FAIL: the effect was not left to the card |
| the reference shot with Polar Coordinates frame 239, Full | 0 | 1 | 35367 | none | FAIL: the effect was not left to the card |
| the reference shot with Polar Coordinates frame 0, Draft | 0 | 1 | 2745 | none | FAIL: the effect was not left to the card |
| the reference shot with Polar Coordinates frame 100, Draft | 0 | 1 | 2703 | none | FAIL: the effect was not left to the card |
| the reference shot with Polar Coordinates frame 239, Draft | 0 | 1 | 2716 | none | FAIL: the effect was not left to the card |
| the reference shot with Optics Compensation frame 0, Full | 0 | 1 | 31457 | none | FAIL: the effect was not left to the card |
| the reference shot with Optics Compensation frame 100, Full | 0 | 1 | 31437 | none | FAIL: the effect was not left to the card |
| the reference shot with Optics Compensation frame 239, Full | 0 | 1 | 31082 | none | FAIL: the effect was not left to the card |
| the reference shot with Optics Compensation frame 0, Draft | 0 | 1 | 2884 | none | FAIL: the effect was not left to the card |
| the reference shot with Optics Compensation frame 100, Draft | 0 | 1 | 2827 | none | FAIL: the effect was not left to the card |
| the reference shot with Optics Compensation frame 239, Draft | 0 | 1 | 2885 | none | FAIL: the effect was not left to the card |
| the reference shot with Corner Pin frame 0, Full | 0 | 1 | 31113 | none | FAIL: the effect was not left to the card |
| the reference shot with Corner Pin frame 100, Full | 0 | 1 | 30890 | none | FAIL: the effect was not left to the card |
| the reference shot with Corner Pin frame 239, Full | 0 | 1 | 30555 | none | FAIL: the effect was not left to the card |
| the reference shot with Corner Pin frame 0, Draft | 0 | 1 | 2870 | none | FAIL: the effect was not left to the card |
| the reference shot with Corner Pin frame 100, Draft | 0 | 1 | 2962 | none | FAIL: the effect was not left to the card |
| the reference shot with Corner Pin frame 239, Draft | 0 | 1 | 2866 | none | FAIL: the effect was not left to the card |
