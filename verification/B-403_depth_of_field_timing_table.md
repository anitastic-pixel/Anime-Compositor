# B-403: frame times with Depth of Field (D-523)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b403_depth_of_field --no-run` and run once with `--ignored b403_depth_of_field_timing`, which times every row below in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (1% busy; 4.2 GB of its memory held by the desktop and open programs); the same after it. The processor was 2% busy or less before each run. The owner's app was not open. One run, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone 5.1; Depth of Field, radius 10, then Noise 107.1 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 5.7 | 2.0 |
| Depth of Field, radius 10, then Noise | Full | 46.5 | 46.1 |
| Depth of Field, radius 30, then Noise | Full | 46.4 | 46.9 |
| Depth of Field, radius 60, then Noise | Full | 46.5 | 47.3 |

Depth of Field is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-229's way: a 1920 by 1080 EXR still written by the test (a picture, and a depth from 2 at the left to 20 at the right), every eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks. Depth of Field focused at 8, then a Noise that changes every frame, so no finished frame is kept; the depth is kept by the pass cache as D-348's passes are, and the fractions are worked out again each frame. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the next seven loops' 210 frames. Milliseconds a frame.

**Reading it.** With Depth of Field a frame took about 46 to 47 milliseconds against 2.0 for Noise alone, and the radius barely changes it (10, 30 and 60 give 46.1, 46.9 and 47.3), so the time is not the blur's width but something paid once a frame, such as working out the fractions out of focus from the depth on the processor and the map passes' fixed cost. About 44 ms added is still far above EFFECTS.md's Target P3 (8 ms added), though less than half the busy run's 100 to 135, which another lane's card test inflated. Depth of Field still needs speeding up.
