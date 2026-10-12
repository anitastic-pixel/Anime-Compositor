# B-471: frame times with CC Rainfall (D-582)

Measured on 2026-10-12, built with `cargo test --release --test b471_cc_rainfall` from 92369655 (B-471's code), and run twice with `--ignored`, one run straight after the other. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11. **PROVISIONAL:** not a quiet machine; 2 cargo or rustc processes from other lanes were counted before and after each run. The owner's app was not touched.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

Run 1:

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 235.1 | 126.2 |
| Noise, then CC Rainfall as added (5000 drops) | Full | 170.5 | 55.1 |
| Noise, then CC Rainfall, 20000 drops, size 4 | Full | 108.7 | 48.2 |
| Noise, then CC Rainfall, 5000 near drops (depth 500), size 6 | Full | 126.9 | 38.5 |

Run 2:

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 99.0 | 72.5 |
| Noise, then CC Rainfall as added (5000 drops) | Full | 205.2 | 46.2 |
| Noise, then CC Rainfall, 20000 drops, size 4 | Full | 234.3 | 62.5 |
| Noise, then CC Rainfall, 5000 near drops (depth 500), size 6 | Full | 83.2 | 38.9 |

CC Rainfall is new, so there is no "before" build: the "Noise alone" row is the comparison.

**What is timed**, B-227's way: the reference shot (1920 by 1080, 24 a second) with a Noise that changes every frame on its first three layers, so no finished frame is kept, and on the first layer CC Rainfall after the Noise; every eighth frame of 240, at Full, each frame asked for whole as the viewer asks, Draw on: GPU. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the seven loops after it. Milliseconds a frame.

**Reading it.** On this busy machine the cost of CC Rainfall could not be told apart from the run-to-run spread: the shot without it measured 126.2 ms a frame once warm in one run and 72.5 ms in the next, while every shot with it measured between 38.5 and 62.5 ms. That the shots with rain came out no slower, and sometimes faster, than the shot without it is not explained and is most likely the other lanes' builds, which is why these numbers are provisional. The safe reading: CC Rainfall at these settings costs no more than that spread, tens of milliseconds a frame at most, and no shot here, with or without it, plays in real time (41.7 ms at 24 a second) on this machine at this load. A quiet re-measure would settle the effect's own cost.
