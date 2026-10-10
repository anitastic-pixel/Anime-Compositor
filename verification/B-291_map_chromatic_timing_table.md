# B-291: frame times with Map Chromatic Displacement (D-412)

Measured on 2026-10-10, built with `cargo test --release --test b291_map_chromatic` from ca9c6b84
(B-291's code, d935237c after rebasing onto the other lane's B-283..B-285), and run with `--ignored b291_map_chromatic_timing`: once on the card, then once
with `B291_CPU` set on the processor. `tasklist` showed no other cargo, rustc or test program
before, between and after the two runs, and the owner's app was not running.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Displacement Map of 12 pixels, every eighth frame asked for whole at Full. **Again** is
the median of the loops after the first (seven on the card, two on the processor), milliseconds
a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 41.5 |
| Noise, then Displacement Map, every colour at 100 (D-193 as before) | 52.5 | 76.9 |
| Noise, then Displacement Map, Red 50, Green 100, Blue 150, Spectrum 3 | 53.1 | 83.4 |
| Noise, then Displacement Map, Red 50, Green 100, Blue 150, Spectrum 16 | 66.2 | 145.4 |

First loops (empty caches): card 15.9, 45.0, 57.2, 65.3; processor 40.4, 73.3, 81.1, 134.6.

**Reading it.** Parting the colours into three costs about 0.6 ms a frame on the card (6.5 on
the processor) over the same displacement with every colour alike; a 16-sample rainbow about 14
ms a frame on the card (68 on the processor), three layers each reading the map 16 times a
pixel. The displacement itself, unchanged from D-193, is most of the cost.
