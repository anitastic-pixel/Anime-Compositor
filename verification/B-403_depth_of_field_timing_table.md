# B-403: frame times with Depth of Field (D-523)

**PROVISIONAL.** Measured on 2026-10-11, built with `cargo test --release --test b403_depth_of_field` from 0141dbee (B-403's code), and run once with `--ignored b403_depth_of_field_timing`. Machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11. Four runs of another lane's card test (b155_gpu_chain) were running when the run started and when it ended, so the table is PROVISIONAL. The owner's app was not touched.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 23.0 | 5.1 |
| Depth of Field, radius 10, then Noise | Full | 134.7 | 107.1 |
| Depth of Field, radius 30, then Noise | Full | 112.7 | 102.8 |
| Depth of Field, radius 60, then Noise | Full | 100.7 | 105.1 |

Depth of Field is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-229's way: a 1920 by 1080 EXR still written by the test (a picture, and a depth from 2 at the left to 20 at the right), every eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks. Depth of Field focused at 8, then a Noise that changes every frame, so no finished frame is kept; the depth is kept by the pass cache as D-348's passes are, and the fractions are worked out again each frame. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the next seven loops' 210 frames. Milliseconds a frame.

**Reading it.** With Depth of Field a frame took about 100 to 135 milliseconds against about 5 again (23 first) for Noise alone, and the radius barely changes it (10, 30 and 60 all again about 103 to 107), so the time is not the blur's width but something paid once a frame, such as working out the fractions out of focus from the depth on the processor and the map passes' fixed cost. That is far above EFFECTS.md's Target P3 (8 ms added). Four runs of another lane's card test (b155_gpu_chain) were running from start to end, using the same card, so the figures are PROVISIONAL and likely too high; measure again on a quiet machine before deciding whether Depth of Field needs speeding up.
