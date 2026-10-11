# B-411: frame times with CC Environment (D-531)

**PROVISIONAL.** Measured on 2026-10-11, built with `cargo test --release --test b411_cc_environment` from c8967264 (B-411's code), and run with `--ignored b411_cc_environment_timing`, once on the card and once with `B411_CPU` set. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11; not quiet (other lanes' builds running). The owner's app was not touched.

## On the card

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the card
- Loops: 8

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 23.4 | 14.8 |
| Noise, then CC Environment, spherical, filtered | Full | 92.9 | 56.9 |
| Noise, then CC Environment, probe | Full | 142.9 | 56.9 |
| Noise, then CC Environment, vertical cross | Full | 71.8 | 50.8 |

## On the processor

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor
- Loops: 3

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 383.5 | 330.3 |
| Noise, then CC Environment, spherical, filtered | Full | 490.1 | 725.7 |
| Noise, then CC Environment, probe | Full | 259.7 | 184.0 |
| Noise, then CC Environment, vertical cross | Full | 126.2 | 102.1 |

CC Environment is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise that changes every frame on its first three layers, so no finished frame is kept, then CC Environment on them with the background layer `layer-4` as the environment, field of view 90; every eighth frame of 240, at Full, each frame asked for whole as the viewer asks. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the loops after it. Milliseconds a frame.

**Reading it.** Other lanes were building at the same time (6 cargo processes before the runs, 7 between them, 4 after), so these are PROVISIONAL. On the card, once warm, the three mappings cost about 36 to 42 ms a frame over Noise alone (14.8 ms), the filtered spherical map (four rays a pixel) the most; the first loop also reads the environment layer, so it is slower. On the processor, three loops each, the rows swing with no order (Noise alone again 330 ms, the vertical cross 102 ms), so no cost can be read from them. A quiet re-measure would settle the figures.
