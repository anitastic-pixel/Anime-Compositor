# B-153: the card keeps its memory from frame to frame

Written by `tests/b153_card_kept_memory.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Each shot is played twice at Draft and at Full, every eighth frame, as the viewer asks. **The rules:** every 48th frame of the second time through is byte for byte the frame a card opened afresh draws, with nothing kept; and the second time through makes no new memory on the card (textures and the memory drawings are sent through), because what the first time made is used again (D-219).

**12 of 12 checks pass.**

| Shot | Quality | What was found | Result |
|---|---|---|---|
| the reference shot with motion blur | Draft | 5 of 5 frames byte for byte a fresh card's | PASS |
| the reference shot with motion blur | Draft | 9 pieces of memory made the first time through, 0 the second | PASS |
| the reference shot with motion blur | Full | 5 of 5 frames byte for byte a fresh card's | PASS |
| the reference shot with motion blur | Full | 9 pieces of memory made the first time through, 0 the second | PASS |
| the reference shot with frame mix and dissolve | Draft | 5 of 5 frames byte for byte a fresh card's | PASS |
| the reference shot with frame mix and dissolve | Draft | 18 pieces of memory made the first time through, 0 the second | PASS |
| the reference shot with frame mix and dissolve | Full | 5 of 5 frames byte for byte a fresh card's | PASS |
| the reference shot with frame mix and dissolve | Full | 18 pieces of memory made the first time through, 0 the second | PASS |
| the reference shot with motion blur and Roughen Edges | Draft | 5 of 5 frames byte for byte a fresh card's | PASS |
| the reference shot with motion blur and Roughen Edges | Draft | 6 pieces of memory made the first time through, 0 the second | PASS |
| the reference shot with motion blur and Roughen Edges | Full | 5 of 5 frames byte for byte a fresh card's | PASS |
| the reference shot with motion blur and Roughen Edges | Full | 8 pieces of memory made the first time through, 0 the second | PASS |
