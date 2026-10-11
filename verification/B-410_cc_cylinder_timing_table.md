# B-410: frame times with CC Cylinder (D-530)

**PROVISIONAL.** Measured on 2026-10-11, built with `cargo test --release --test b410_cc_cylinder` from 03d0c98b (B-410's code), and run with `--ignored b410_cc_cylinder_timing`, once on the card and once with `B410_CPU` set. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11; not quiet (other lanes' builds running). The owner's app was not touched.

## On the card

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the card
- Loops: 8

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 202.5 | 87.4 |
| Noise, then CC Cylinder as added | Full | 93.1 | 95.7 |
| Noise, then CC Cylinder, render inside, tipped | Full | 133.9 | 111.0 |
| Noise, then CC Cylinder, radius 100, on its side | Full | 121.5 | 70.2 |

## On the processor

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor
- Loops: 3

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 152.1 | 169.7 |
| Noise, then CC Cylinder as added | Full | 162.5 | 170.7 |
| Noise, then CC Cylinder, render inside, tipped | Full | 172.5 | 143.4 |
| Noise, then CC Cylinder, radius 100, on its side | Full | 144.5 | 242.5 |

CC Cylinder is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise that changes every frame on its first three layers, so no finished frame is kept, then CC Cylinder on them; every eighth frame of 240, at Full, each frame asked for whole as the viewer asks. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the loops after it. Milliseconds a frame.

**Reading it.** Other lanes were building at the same time (cargo and rustc running before, between and after the two runs), so these are PROVISIONAL and the swings of tens of milliseconds between rows are the machine, not the effect: the radius 100 row is faster than Noise alone again. On the card, CC Cylinder as added is about 8 ms a frame over Noise alone once warm, which is the most that can be said. On the processor, three loops each, the rows swing from about 143 to 243 ms with no order, so no cost can be read from them. A quiet re-measure would settle the figure.
