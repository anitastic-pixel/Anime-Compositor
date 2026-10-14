# B-401: frame times with Time Difference (D-521)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b401_time_difference --no-run` and run once with `--ignored b401_time_difference_timing`, which times every row below in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (9% busy; 4.3 GB of its memory held by the desktop and open programs); the same after it. The processor was 3% busy or less before each run. Before that the machine was busy (the card 23 to 23% busy, from programs outside this work), and the runs waited up to 70 s for it to be quiet. The owner's app was not open. One run, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone 14.3; Time Difference as added (itself, now), then Noise 67.6 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 16.5 | 12.5 |
| Time Difference as added (itself, now), then Noise | Full | 35.9 | 35.0 |
| Time Difference one frame back, then Noise | Full | 47.4 | 45.2 |

Time Difference is new, so there is no "before" build: the row without it is the comparison. It is drawn by the processor.

**What is timed**, B-227's way: the reference shot (1920 by 1080, 24 frames a second), every eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks. On each of the shot's first three layers: Time Difference against the layer itself, then a Noise that changes every frame, so no finished frame is kept. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the next seven loops' 210 frames. Milliseconds a frame.

**Reading it.** With Time Difference on each of the first three layers, a frame took 35.0 milliseconds as added (the layer against itself) and 45.2 one frame back, against 12.5 for Noise alone: about 7.5 and 11 ms a 1080p layer. The extra is the processor's difference over three full-size layers, and one frame back also the drawing of the earlier frame. The busy run's 40 to 70 and 85 to 100 ms a frame were the other lane's test and cargo processes; quiet, "First" and "Again" agree.
