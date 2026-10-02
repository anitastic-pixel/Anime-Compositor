# W-40c: the ready frames

Written by `w40c_ready_frames_are_the_ones_memory_would_send` in `app/src/main.rs`. The reference shot, 1920 by 1080, through `serve_logged`, the function every frame on screen comes from, and `ready_frames`, the answer to the page's `/ready`. Frames are listed as runs. Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

**16 of 16 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| a viewer that has made nothing: no frame is ready | none | none | PASS |
| frames 100 and 101 made at Full: those two are ready | 100-101 | 100-101 | PASS |
| the memory holds those two frames: 1920 by 1080, four bytes a pixel, twice | 16588800 bytes | 16588800 bytes | PASS |
| the memory setting given is the Preferences setting | 16521373696 bytes | 16521373696 bytes | PASS |
| asking changes nothing: asked again, the same answer | {"frames":[100,101],"held":16588800,"setting":16521373696} | {"frames":[100,101],"held":16588800,"setting":16521373696} | PASS |
| frame 5 made at Draft: at Draft only frame 5 is ready, not the Full frames | 5 | 5 | PASS |
| back at Full: 100 and 101 are ready, not the Draft frame | 100-101 | 100-101 | PASS |
| a layer soloed: nothing is ready, no frame was made with it soloed | none | none | PASS |
| unsoloed: 100 and 101 are ready again | 100-101 | 100-101 | PASS |
| after an edit (layer 1 hidden) nothing is ready | none | none | PASS |
| after undo the frames from before the edit are ready again | 100-101 | 100-101 | PASS |
| Draft, play pressed at frame 0 and the loop made ahead: every frame is ready | 0-239 | 0-239 | PASS |
| the loop holds 240 Draft frames: 480 by 270, four bytes a pixel | 124416000 bytes | 124416000 bytes | PASS |
| card: the CPU's frames are not ready for the card | none | none | PASS |
| frame 100 drawn on the card: it is ready for the card | 100 | 100 | PASS |
| and the CPU's 100 and 101 are still ready for the CPU | 100-101 | 100-101 | PASS |
