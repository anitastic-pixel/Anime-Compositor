# B-44: the switch in the window

Written by `cargo test -p anime_compositor_app`. The reference shot, through `serve_logged`, the function every frame on screen comes from. Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory.

**9 of 9 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| the switch starts on the CPU | CPU | CPU | PASS |
| pressing it says which card | names the card | names the card | PASS |
| frame 100 at Full says it was drawn on the GPU | GPU | GPU | PASS |
| and is within 1 level of the CPU's picture, at the same size | at most 1, 1920×1080 or its draft | at most 1, 1920×1080 or its draft | PASS |
| frame 100 at Draft says it was drawn on the GPU | GPU | GPU | PASS |
| and is within 1 level of the CPU's picture, at the same size | at most 1, 1920×1080 or its draft | at most 1, 1920×1080 or its draft | PASS |
| pressing it again goes back to the CPU | CPU | CPU | PASS |
| and says so | The preview is drawn on the CPU again. | The preview is drawn on the CPU again. | PASS |
| the CPU's picture after the GPU is byte for byte the one before it | identical | identical | PASS |
