# B-412: frame times with CC Sphere (D-532)

**PROVISIONAL.** Measured on 2026-10-11, built with `cargo test --release --test b412_cc_sphere` from 6af85b36 (B-412's code), and run with `--ignored b412_cc_sphere_timing`, once on the card and once with `B412_CPU` set. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11; not quiet (other lanes' builds running). The owner's app was not touched.

## On the card

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the card
- Loops: 8

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 34.5 | 29.9 |
| Noise, then CC Sphere as added | Full | 48.2 | 34.7 |
| Noise, then CC Sphere, render inside, tipped | Full | 66.3 | 43.8 |
| Noise, then CC Sphere, radius 100, on its side | Full | 50.7 | 47.7 |

## On the processor

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor
- Loops: 3

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 55.0 | 87.1 |
| Noise, then CC Sphere as added | Full | 183.1 | 120.0 |
| Noise, then CC Sphere, render inside, tipped | Full | 92.5 | 103.1 |
| Noise, then CC Sphere, radius 100, on its side | Full | 127.6 | 216.1 |

CC Sphere is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise that changes every frame on its first three layers, so no finished frame is kept, then CC Sphere on them; every eighth frame of 240, at Full, each frame asked for whole as the viewer asks. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the loops after it. Milliseconds a frame.

**Reading it.** Other lanes were building at the same time (2 cargo or rustc processes before the runs, 7 between them, 2 after), and Noise alone took 29.9 ms a frame once warm against 14.8 ms in B-411's runs, so these are PROVISIONAL. On the card, once warm, CC Sphere as added costs about 5 ms a frame over Noise alone, tipped and drawn inside about 14 ms, and radius 100 on its side, the sphere filling most of the frame, about 18 ms. On the processor, three loops each, the rows swing with no order (Noise alone again 87 ms after 55 ms first), so no cost can be read from them. A quiet re-measure would settle the figures.
