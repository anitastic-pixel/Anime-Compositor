# B-154: RAM preview

Written by `cargo test -p anime_compositor_app ram_preview`. The reference shot, 1920 by 1080, through `serve_logged`, the function every frame on screen comes from. "Cold" is a viewer that has remembered nothing. `x-cached` is 1 when the frame was sent from memory. Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

**37 of 37 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| CPU: frame 100, asked for the first time, is made | 0 | 0 | PASS |
| asked for again, it is sent from memory | 1 | 1 | PASS |
| byte for byte the frame made the first time | identical | identical | PASS |
| and the page is told the same about it | 1920 1080 CPU 100 Full | 1920 1080 CPU 100 Full | PASS |
| frame 101 is not frame 100 | 0 | 0 | PASS |
| Draft is not Full | 0 | 0 | PASS |
| Full again is still remembered | 1 | 1 | PASS |
| soloing a layer is not the frame remembered without it | 0 | 0 | PASS |
| and is the soloed frame made cold | identical | identical | PASS |
| unsoloed, the first frame is remembered | 1 | 1 | PASS |
| after an edit (layer 1 hidden) the frame is made again | 0 | 0 | PASS |
| and is the edited frame made cold | identical | identical | PASS |
| after undo the frame from before the edit is sent from memory | 1 | 1 | PASS |
| and is that frame | identical | identical | PASS |
| after layer 1's drawings are replaced on disk the frame is made again | 0 | 0 | PASS |
| and shows the new drawings: it is not the frame from before | different | different | PASS |
| and is the frame made cold from the new drawings | identical | identical | PASS |
| with nothing playing nothing is made ahead | 0 frames | 0 frames | PASS |
| Draft, after play is pressed at frame 0: the other 239 frames of the loop are made ahead | 239 frames | 239 frames | PASS |
| and all 240 are then sent from memory | 240 of 240 | 240 of 240 | PASS |
| the next frame not remembered gives the cels the memory setting less the loop | 15.27 GiB | 15.27 GiB | PASS |
| a frame made ahead (frame 50) is byte for byte the one made when asked | identical | identical | PASS |
| card: frame 100 is not the CPU's remembered frame | 0 | 0 | PASS |
| and is drawn on the card | GPU | GPU | PASS |
| asked for again, it is sent from memory | 1 | 1 | PASS |
| byte for byte the card's frame | identical | identical | PASS |
| and still says the card drew it | GPU | GPU | PASS |
| the first time, the session log says the CPU drew it | says so | says so | PASS |
| a frame the card leaves to the CPU (a Light Wrap at Mix 50), asked for again, is sent from memory | 1 | 1 | PASS |
| and still says the CPU drew it, not the card | CPU, not GPU | CPU, not GPU | PASS |
| and the session log gets the same warning again | GPU_PREVIEW_ON_CPU: The CPU drew this frame: it has a Light Wrap with a Mix below 100, which the GPU does not draw yet. | GPU_PREVIEW_ON_CPU: The CPU drew this frame: it has a Light Wrap with a Mix below 100, which the GPU does not draw yet. | PASS |
| byte for byte the frame the CPU drew | identical | identical | PASS |
| the card's picture, put back on the card from memory, is the same bytes | identical | identical | PASS |
| card, Draft: after play is pressed the other 239 frames are made ahead | 239 frames | 239 frames | PASS |
| and all 240 are then sent from memory | 240 of 240 | 240 of 240 | PASS |
| a frame the card made ahead (frame 50) is byte for byte the one it makes when asked | identical | identical | PASS |
| exports never read remembered frames: the export code does not name them | does not | does not | PASS |
