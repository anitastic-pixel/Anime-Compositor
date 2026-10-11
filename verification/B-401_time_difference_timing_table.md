# B-401: frame times with Time Difference (D-521)

**PROVISIONAL.** Measured on 2026-10-11, built with `cargo test --release --test b401_time_difference` from 5016e2b4 (B-401's code), and run once with `--ignored b401_time_difference_timing`. Machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11. The machine was quiet when the run started; another lane's test and two cargo processes were running when it ended, so the table is PROVISIONAL. The owner's app was not touched.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 18.3 | 14.3 |
| Time Difference as added (itself, now), then Noise | Full | 41.8 | 67.6 |
| Time Difference one frame back, then Noise | Full | 98.4 | 85.4 |

Time Difference is new, so there is no "before" build: the row without it is the comparison. It is drawn by the processor.

**What is timed**, B-227's way: the reference shot (1920 by 1080, 24 frames a second), every eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks. On each of the shot's first three layers: Time Difference against the layer itself, then a Noise that changes every frame, so no finished frame is kept. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the next seven loops' 210 frames. Milliseconds a frame.

**Reading it.** With Time Difference on each of the first three layers, a frame took about 40 to 70 milliseconds as added (the layer against itself) and about 85 to 100 one frame back, against about 14 to 18 for Noise alone. The extra is the processor's difference over three full-size layers, and one frame back also the drawing of the earlier frame. The machine was quiet when the run started but another lane's test (b420_screen_key) and two cargo processes were running when it ended, which is likely why "Again" as added came out slower than "First"; the numbers are PROVISIONAL and should be measured again on a quiet machine before any speed claim.
