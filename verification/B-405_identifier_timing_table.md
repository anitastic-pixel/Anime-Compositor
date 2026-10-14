# B-405: frame times with IDentifier (D-525)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b405_identifier --no-run` and run once with `--ignored b405_identifier_timing`, which times every row below in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 3.6 GB of its memory held by the desktop and open programs); the same after it. The processor was 2% busy or less before each run. The owner's app was not open. One run, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone 3.9; IDentifier as added (colours), then Noise 4.0 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 5.7 | 2.0 |
| IDentifier as added (colours), then Noise | Full | 5.5 | 2.1 |
| IDentifier alpha matte of 3, then Noise | Full | 3.8 | 1.9 |

IDentifier is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-229's way: a 1920 by 1080 EXR still written by the test (a picture, and object ids 0 to 5 in blocks), every eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks. IDentifier, then a Noise that changes every frame, so no finished frame is kept; the ids are kept by the pass cache as D-349's are, and the picture is worked out again each frame. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the next seven loops' 210 frames. Milliseconds a frame.

**Reading it.** With IDentifier a frame took 1.9 to 2.1 milliseconds again against 2.0 for Noise alone: the picture is worked out once as the ids are read and kept with the file's passes, so a frame only lays it on, within EFFECTS.md's Target P1. The first loop's figures fall from 5.7 to 3.8 because the shots after the first start warmed up; they are not a sign that IDentifier makes frames faster. That is the busy run's reading, now with figures that can be read.
