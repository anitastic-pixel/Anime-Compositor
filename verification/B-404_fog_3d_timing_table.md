# B-404: frame times with Fog 3D (D-524)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b404_fog_3d --no-run` and run once with `--ignored b404_fog_3d_timing`, which times every row below in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 4.1 GB of its memory held by the desktop and open programs); the same after it. The processor was 4% busy or less before each run. The owner's app was not open. One run, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone 6.5; Fog 3D as added, then Noise 5.7 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 5.8 | 2.0 |
| Fog 3D as added, then Noise | Full | 6.3 | 2.1 |
| Fog 3D from 4 to 20, density 0, then Noise | Full | 5.5 | 2.1 |

Fog 3D is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-229's way: a 1920 by 1080 EXR still written by the test (a picture, and a depth from 2 at the left to 20 at the right), every eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks. Fog 3D, then a Noise that changes every frame, so no finished frame is kept; the depth is kept by the pass cache as D-348's passes are, and the shares are worked out again each frame. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the next seven loops' 210 frames. Milliseconds a frame.

**Reading it.** With Fog 3D a frame took 2.1 milliseconds again against 2.0 for Noise alone: once the depth has been read and the shares worked out and kept with the file's passes, the card's fog pass costs about 0.1 ms, within EFFECTS.md's Target P1. The first loop's figures (5.5 to 6.3) include reading the file and warming up. That is the busy run's reading, now with figures that can be read.
