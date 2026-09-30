# B-158: only the part of the picture on screen

Written by `cargo test -p anime_compositor_app part_on_screen`. Zoomed in, the page asks for the part of the picture it can show, as fractions of the picture across and down, and the window makes and sends only those pixels, with one pixel spare on every side. **The rule: every part is byte for byte the same pixels of the whole frame**, made by a viewer that has remembered nothing, with the same size told to the page and the same processor. Parts asked for, as fractions (left, top, right, bottom): the left half (0,0,0.5,1); a box in the middle (0.3,0.3,0.7,0.6); the bottom right corner (0.8,0.9,1,1); a single point (0.5,0.5,0.5,0.5); the middle quarter, as at 200% (0.25,0.25,0.75,0.75). Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

**6548 of 6548 checks pass.**

## Each group

| Group | Parts checked | Of those, sent whole (with its spare pixel the part is all of the frame) | Pass | First failure |
|---|---:|---:|---:|---|
| reference shot (CPU) | 30 | 0 | 30 of 30 | none |
| adjustment layers (CPU) | 260 | 182 | 260 of 260 | none |
| Light Wrap (CPU) | 480 | 96 | 480 of 480 | none |
| motion blur (CPU) | 400 | 76 | 400 of 400 | none |
| frame blending (CPU) | 580 | 290 | 580 of 580 | none |
| Bloom (CPU) | 560 | 112 | 560 of 560 | none |
| Directional Blur (CPU) | 300 | 60 | 300 of 300 | none |
| Glow (CPU) | 660 | 132 | 660 of 660 | none |
| reference shot (card) | 30 | 0 | 30 of 30 | none |
| adjustment layers (card) | 260 | 182 | 260 of 260 | none |
| Light Wrap (card) | 480 | 96 | 480 of 480 | none |
| motion blur (card) | 400 | 76 | 400 of 400 | none |
| frame blending (card) | 580 | 290 | 580 of 580 | none |
| Bloom (card) | 560 | 112 | 560 of 560 | none |
| Directional Blur (card) | 300 | 60 | 300 of 300 | none |
| Glow (card) | 660 | 132 | 660 of 660 | none |

## Memory, playback and export

| Check | Expected | Actual | Result |
|---|---|---|---|
| after only a part of frame 100 was made, the whole frame is made, not sent from memory | 0 | 0 | PASS |
| with the whole frame remembered, a part of it is cut from memory | 1 | 1 | PASS |
| and is the same pixels as the part made | identical | identical | PASS |
| all of the picture on screen is the whole frame, with no part header | none | none | PASS |
| and is byte for byte the whole frame | identical | identical | PASS |
| playback is sent whole, whatever is on screen | none | none | PASS |
| an unreadable part is the whole frame, never a guess | none | none | PASS |
| exports never read a part: the export code does not name it | does not | does not | PASS |

## The reference shot, every part

| Drawn on | Frame | Part | Pixels sent (x, y, across, down) | Result |
|---|---|---|---|---|
| CPU | frame 0, Full | the left half | 0,0,961,1080 | PASS |
| CPU | frame 0, Full | a box in the middle | 575,323,770,326 | PASS |
| CPU | frame 0, Full | the bottom right corner | 1535,971,385,109 | PASS |
| CPU | frame 0, Full | a single point | 959,539,2,2 | PASS |
| CPU | frame 0, Full | the middle quarter, as at 200% | 479,269,962,542 | PASS |
| CPU | frame 100, Full | the left half | 0,0,961,1080 | PASS |
| CPU | frame 100, Full | a box in the middle | 575,323,770,326 | PASS |
| CPU | frame 100, Full | the bottom right corner | 1535,971,385,109 | PASS |
| CPU | frame 100, Full | a single point | 959,539,2,2 | PASS |
| CPU | frame 100, Full | the middle quarter, as at 200% | 479,269,962,542 | PASS |
| CPU | frame 239, Full | the left half | 0,0,961,1080 | PASS |
| CPU | frame 239, Full | a box in the middle | 575,323,770,326 | PASS |
| CPU | frame 239, Full | the bottom right corner | 1535,971,385,109 | PASS |
| CPU | frame 239, Full | a single point | 959,539,2,2 | PASS |
| CPU | frame 239, Full | the middle quarter, as at 200% | 479,269,962,542 | PASS |
| CPU | frame 0, Draft | the left half | 0,0,241,270 | PASS |
| CPU | frame 0, Draft | a box in the middle | 143,80,194,83 | PASS |
| CPU | frame 0, Draft | the bottom right corner | 383,242,97,28 | PASS |
| CPU | frame 0, Draft | a single point | 239,134,2,2 | PASS |
| CPU | frame 0, Draft | the middle quarter, as at 200% | 119,66,242,138 | PASS |
| CPU | frame 100, Draft | the left half | 0,0,241,270 | PASS |
| CPU | frame 100, Draft | a box in the middle | 143,80,194,83 | PASS |
| CPU | frame 100, Draft | the bottom right corner | 383,242,97,28 | PASS |
| CPU | frame 100, Draft | a single point | 239,134,2,2 | PASS |
| CPU | frame 100, Draft | the middle quarter, as at 200% | 119,66,242,138 | PASS |
| CPU | frame 239, Draft | the left half | 0,0,241,270 | PASS |
| CPU | frame 239, Draft | a box in the middle | 143,80,194,83 | PASS |
| CPU | frame 239, Draft | the bottom right corner | 383,242,97,28 | PASS |
| CPU | frame 239, Draft | a single point | 239,134,2,2 | PASS |
| CPU | frame 239, Draft | the middle quarter, as at 200% | 119,66,242,138 | PASS |
| card | frame 0, Full | the left half | 0,0,961,1080 | PASS |
| card | frame 0, Full | a box in the middle | 575,323,770,326 | PASS |
| card | frame 0, Full | the bottom right corner | 1535,971,385,109 | PASS |
| card | frame 0, Full | a single point | 959,539,2,2 | PASS |
| card | frame 0, Full | the middle quarter, as at 200% | 479,269,962,542 | PASS |
| card | frame 100, Full | the left half | 0,0,961,1080 | PASS |
| card | frame 100, Full | a box in the middle | 575,323,770,326 | PASS |
| card | frame 100, Full | the bottom right corner | 1535,971,385,109 | PASS |
| card | frame 100, Full | a single point | 959,539,2,2 | PASS |
| card | frame 100, Full | the middle quarter, as at 200% | 479,269,962,542 | PASS |
| card | frame 239, Full | the left half | 0,0,961,1080 | PASS |
| card | frame 239, Full | a box in the middle | 575,323,770,326 | PASS |
| card | frame 239, Full | the bottom right corner | 1535,971,385,109 | PASS |
| card | frame 239, Full | a single point | 959,539,2,2 | PASS |
| card | frame 239, Full | the middle quarter, as at 200% | 479,269,962,542 | PASS |
| card | frame 0, Draft | the left half | 0,0,241,270 | PASS |
| card | frame 0, Draft | a box in the middle | 143,80,194,83 | PASS |
| card | frame 0, Draft | the bottom right corner | 383,242,97,28 | PASS |
| card | frame 0, Draft | a single point | 239,134,2,2 | PASS |
| card | frame 0, Draft | the middle quarter, as at 200% | 119,66,242,138 | PASS |
| card | frame 100, Draft | the left half | 0,0,241,270 | PASS |
| card | frame 100, Draft | a box in the middle | 143,80,194,83 | PASS |
| card | frame 100, Draft | the bottom right corner | 383,242,97,28 | PASS |
| card | frame 100, Draft | a single point | 239,134,2,2 | PASS |
| card | frame 100, Draft | the middle quarter, as at 200% | 119,66,242,138 | PASS |
| card | frame 239, Draft | the left half | 0,0,241,270 | PASS |
| card | frame 239, Draft | a box in the middle | 143,80,194,83 | PASS |
| card | frame 239, Draft | the bottom right corner | 383,242,97,28 | PASS |
| card | frame 239, Draft | a single point | 239,134,2,2 | PASS |
| card | frame 239, Draft | the middle quarter, as at 200% | 119,66,242,138 | PASS |
