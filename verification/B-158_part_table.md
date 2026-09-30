# B-158: only the part of the picture on screen

Written by `cargo test -p anime_compositor_app part_on_screen`. Zoomed in, the page asks for the part of the picture it can show, as fractions of the picture across and down, and the window makes and sends only those pixels, with one pixel spare on every side. **The rule: every part is byte for byte the same pixels of the whole frame**, made by a viewer that has remembered nothing, with the same size told to the page and the same processor. Parts asked for, as fractions (left, top, right, bottom): the left half (0,0,0.5,1); a box in the middle (0.3,0.3,0.7,0.6); the bottom right corner (0.8,0.9,1,1); a single point (0.5,0.5,0.5,0.5); the middle quarter, as at 200% (0.25,0.25,0.75,0.75). Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

**1902 of 6548 checks pass.**

## Each group

| Group | Parts checked | Of those, sent whole (with its spare pixel the part is all of the frame) | Pass | First failure |
|---|---:|---:|---:|---|
| reference shot (CPU) | 30 | 0 | 0 of 30 | the reference shot frame 0, Full, the left half: part None sent, Some((0, 0, 961, 1080)) expected, of 1920 by 1080 |
| adjustment layers (CPU) | 260 | 182 | 182 of 260 | fx_adj_001 frame 0, Full, the left half: part None sent, Some((0, 0, 4, 2)) expected, of 6 by 2 |
| Light Wrap (CPU) | 480 | 96 | 96 of 480 | fx_wrap_001 frame 0, Full, the left half: part None sent, Some((0, 0, 9, 10)) expected, of 16 by 10 |
| motion blur (CPU) | 400 | 76 | 76 of 400 | fx_mb_010 frame 0, Full, the left half: part None sent, Some((0, 0, 11, 1)) expected, of 20 by 1 |
| frame blending (CPU) | 580 | 290 | 290 of 580 | fx_fblend_010 frame 0, Full, the left half: part None sent, Some((0, 0, 5, 1)) expected, of 8 by 1 |
| Bloom (CPU) | 560 | 112 | 112 of 560 | fx_bloom_001 frame 0, Full, the left half: part None sent, Some((0, 0, 9, 10)) expected, of 16 by 10 |
| Directional Blur (CPU) | 300 | 60 | 60 of 300 | fx_dirblur_001 frame 0, Full, the left half: part None sent, Some((0, 0, 9, 10)) expected, of 16 by 10 |
| Glow (CPU) | 660 | 132 | 132 of 660 | fx_glow_001 frame 0, Full, the left half: part None sent, Some((0, 0, 9, 10)) expected, of 16 by 10 |
| reference shot (card) | 30 | 0 | 0 of 30 | the reference shot frame 0, Full, the left half: part None sent, Some((0, 0, 961, 1080)) expected, of 1920 by 1080 |
| adjustment layers (card) | 260 | 182 | 182 of 260 | fx_adj_001 frame 0, Full, the left half: part None sent, Some((0, 0, 4, 2)) expected, of 6 by 2 |
| Light Wrap (card) | 480 | 96 | 96 of 480 | fx_wrap_001 frame 0, Full, the left half: part None sent, Some((0, 0, 9, 10)) expected, of 16 by 10 |
| motion blur (card) | 400 | 76 | 76 of 400 | fx_mb_010 frame 0, Full, the left half: part None sent, Some((0, 0, 11, 1)) expected, of 20 by 1 |
| frame blending (card) | 580 | 290 | 290 of 580 | fx_fblend_010 frame 0, Full, the left half: part None sent, Some((0, 0, 5, 1)) expected, of 8 by 1 |
| Bloom (card) | 560 | 112 | 112 of 560 | fx_bloom_001 frame 0, Full, the left half: part None sent, Some((0, 0, 9, 10)) expected, of 16 by 10 |
| Directional Blur (card) | 300 | 60 | 60 of 300 | fx_dirblur_001 frame 0, Full, the left half: part None sent, Some((0, 0, 9, 10)) expected, of 16 by 10 |
| Glow (card) | 660 | 132 | 132 of 660 | fx_glow_001 frame 0, Full, the left half: part None sent, Some((0, 0, 9, 10)) expected, of 16 by 10 |

## Memory, playback and export

| Check | Expected | Actual | Result |
|---|---|---|---|
| after only a part of frame 100 was made, the whole frame is made, not sent from memory | 0 | 1 | FAIL |
| with the whole frame remembered, a part of it is cut from memory | 1 | 1 | PASS |
| and is the same pixels as the part made | identical | different | FAIL |
| all of the picture on screen is the whole frame, with no part header | none | none | PASS |
| and is byte for byte the whole frame | identical | identical | PASS |
| playback is sent whole, whatever is on screen | none | none | PASS |
| an unreadable part is the whole frame, never a guess | none | none | PASS |
| exports never read a part: the export code does not name it | does not | does not | PASS |

## The reference shot, every part

| Drawn on | Frame | Part | Pixels sent (x, y, across, down) | Result |
|---|---|---|---|---|
| CPU | frame 0, Full | the left half |  | FAIL: part None sent, Some((0, 0, 961, 1080)) expected, of 1920 by 1080 |
| CPU | frame 0, Full | a box in the middle |  | FAIL: part None sent, Some((575, 323, 770, 326)) expected, of 1920 by 1080 |
| CPU | frame 0, Full | the bottom right corner |  | FAIL: part None sent, Some((1535, 971, 385, 109)) expected, of 1920 by 1080 |
| CPU | frame 0, Full | a single point |  | FAIL: part None sent, Some((959, 539, 2, 2)) expected, of 1920 by 1080 |
| CPU | frame 0, Full | the middle quarter, as at 200% |  | FAIL: part None sent, Some((479, 269, 962, 542)) expected, of 1920 by 1080 |
| CPU | frame 100, Full | the left half |  | FAIL: part None sent, Some((0, 0, 961, 1080)) expected, of 1920 by 1080 |
| CPU | frame 100, Full | a box in the middle |  | FAIL: part None sent, Some((575, 323, 770, 326)) expected, of 1920 by 1080 |
| CPU | frame 100, Full | the bottom right corner |  | FAIL: part None sent, Some((1535, 971, 385, 109)) expected, of 1920 by 1080 |
| CPU | frame 100, Full | a single point |  | FAIL: part None sent, Some((959, 539, 2, 2)) expected, of 1920 by 1080 |
| CPU | frame 100, Full | the middle quarter, as at 200% |  | FAIL: part None sent, Some((479, 269, 962, 542)) expected, of 1920 by 1080 |
| CPU | frame 239, Full | the left half |  | FAIL: part None sent, Some((0, 0, 961, 1080)) expected, of 1920 by 1080 |
| CPU | frame 239, Full | a box in the middle |  | FAIL: part None sent, Some((575, 323, 770, 326)) expected, of 1920 by 1080 |
| CPU | frame 239, Full | the bottom right corner |  | FAIL: part None sent, Some((1535, 971, 385, 109)) expected, of 1920 by 1080 |
| CPU | frame 239, Full | a single point |  | FAIL: part None sent, Some((959, 539, 2, 2)) expected, of 1920 by 1080 |
| CPU | frame 239, Full | the middle quarter, as at 200% |  | FAIL: part None sent, Some((479, 269, 962, 542)) expected, of 1920 by 1080 |
| CPU | frame 0, Draft | the left half |  | FAIL: part None sent, Some((0, 0, 241, 270)) expected, of 480 by 270 |
| CPU | frame 0, Draft | a box in the middle |  | FAIL: part None sent, Some((143, 80, 194, 83)) expected, of 480 by 270 |
| CPU | frame 0, Draft | the bottom right corner |  | FAIL: part None sent, Some((383, 242, 97, 28)) expected, of 480 by 270 |
| CPU | frame 0, Draft | a single point |  | FAIL: part None sent, Some((239, 134, 2, 2)) expected, of 480 by 270 |
| CPU | frame 0, Draft | the middle quarter, as at 200% |  | FAIL: part None sent, Some((119, 66, 242, 138)) expected, of 480 by 270 |
| CPU | frame 100, Draft | the left half |  | FAIL: part None sent, Some((0, 0, 241, 270)) expected, of 480 by 270 |
| CPU | frame 100, Draft | a box in the middle |  | FAIL: part None sent, Some((143, 80, 194, 83)) expected, of 480 by 270 |
| CPU | frame 100, Draft | the bottom right corner |  | FAIL: part None sent, Some((383, 242, 97, 28)) expected, of 480 by 270 |
| CPU | frame 100, Draft | a single point |  | FAIL: part None sent, Some((239, 134, 2, 2)) expected, of 480 by 270 |
| CPU | frame 100, Draft | the middle quarter, as at 200% |  | FAIL: part None sent, Some((119, 66, 242, 138)) expected, of 480 by 270 |
| CPU | frame 239, Draft | the left half |  | FAIL: part None sent, Some((0, 0, 241, 270)) expected, of 480 by 270 |
| CPU | frame 239, Draft | a box in the middle |  | FAIL: part None sent, Some((143, 80, 194, 83)) expected, of 480 by 270 |
| CPU | frame 239, Draft | the bottom right corner |  | FAIL: part None sent, Some((383, 242, 97, 28)) expected, of 480 by 270 |
| CPU | frame 239, Draft | a single point |  | FAIL: part None sent, Some((239, 134, 2, 2)) expected, of 480 by 270 |
| CPU | frame 239, Draft | the middle quarter, as at 200% |  | FAIL: part None sent, Some((119, 66, 242, 138)) expected, of 480 by 270 |
| card | frame 0, Full | the left half |  | FAIL: part None sent, Some((0, 0, 961, 1080)) expected, of 1920 by 1080 |
| card | frame 0, Full | a box in the middle |  | FAIL: part None sent, Some((575, 323, 770, 326)) expected, of 1920 by 1080 |
| card | frame 0, Full | the bottom right corner |  | FAIL: part None sent, Some((1535, 971, 385, 109)) expected, of 1920 by 1080 |
| card | frame 0, Full | a single point |  | FAIL: part None sent, Some((959, 539, 2, 2)) expected, of 1920 by 1080 |
| card | frame 0, Full | the middle quarter, as at 200% |  | FAIL: part None sent, Some((479, 269, 962, 542)) expected, of 1920 by 1080 |
| card | frame 100, Full | the left half |  | FAIL: part None sent, Some((0, 0, 961, 1080)) expected, of 1920 by 1080 |
| card | frame 100, Full | a box in the middle |  | FAIL: part None sent, Some((575, 323, 770, 326)) expected, of 1920 by 1080 |
| card | frame 100, Full | the bottom right corner |  | FAIL: part None sent, Some((1535, 971, 385, 109)) expected, of 1920 by 1080 |
| card | frame 100, Full | a single point |  | FAIL: part None sent, Some((959, 539, 2, 2)) expected, of 1920 by 1080 |
| card | frame 100, Full | the middle quarter, as at 200% |  | FAIL: part None sent, Some((479, 269, 962, 542)) expected, of 1920 by 1080 |
| card | frame 239, Full | the left half |  | FAIL: part None sent, Some((0, 0, 961, 1080)) expected, of 1920 by 1080 |
| card | frame 239, Full | a box in the middle |  | FAIL: part None sent, Some((575, 323, 770, 326)) expected, of 1920 by 1080 |
| card | frame 239, Full | the bottom right corner |  | FAIL: part None sent, Some((1535, 971, 385, 109)) expected, of 1920 by 1080 |
| card | frame 239, Full | a single point |  | FAIL: part None sent, Some((959, 539, 2, 2)) expected, of 1920 by 1080 |
| card | frame 239, Full | the middle quarter, as at 200% |  | FAIL: part None sent, Some((479, 269, 962, 542)) expected, of 1920 by 1080 |
| card | frame 0, Draft | the left half |  | FAIL: part None sent, Some((0, 0, 241, 270)) expected, of 480 by 270 |
| card | frame 0, Draft | a box in the middle |  | FAIL: part None sent, Some((143, 80, 194, 83)) expected, of 480 by 270 |
| card | frame 0, Draft | the bottom right corner |  | FAIL: part None sent, Some((383, 242, 97, 28)) expected, of 480 by 270 |
| card | frame 0, Draft | a single point |  | FAIL: part None sent, Some((239, 134, 2, 2)) expected, of 480 by 270 |
| card | frame 0, Draft | the middle quarter, as at 200% |  | FAIL: part None sent, Some((119, 66, 242, 138)) expected, of 480 by 270 |
| card | frame 100, Draft | the left half |  | FAIL: part None sent, Some((0, 0, 241, 270)) expected, of 480 by 270 |
| card | frame 100, Draft | a box in the middle |  | FAIL: part None sent, Some((143, 80, 194, 83)) expected, of 480 by 270 |
| card | frame 100, Draft | the bottom right corner |  | FAIL: part None sent, Some((383, 242, 97, 28)) expected, of 480 by 270 |
| card | frame 100, Draft | a single point |  | FAIL: part None sent, Some((239, 134, 2, 2)) expected, of 480 by 270 |
| card | frame 100, Draft | the middle quarter, as at 200% |  | FAIL: part None sent, Some((119, 66, 242, 138)) expected, of 480 by 270 |
| card | frame 239, Draft | the left half |  | FAIL: part None sent, Some((0, 0, 241, 270)) expected, of 480 by 270 |
| card | frame 239, Draft | a box in the middle |  | FAIL: part None sent, Some((143, 80, 194, 83)) expected, of 480 by 270 |
| card | frame 239, Draft | the bottom right corner |  | FAIL: part None sent, Some((383, 242, 97, 28)) expected, of 480 by 270 |
| card | frame 239, Draft | a single point |  | FAIL: part None sent, Some((239, 134, 2, 2)) expected, of 480 by 270 |
| card | frame 239, Draft | the middle quarter, as at 200% |  | FAIL: part None sent, Some((119, 66, 242, 138)) expected, of 480 by 270 |
