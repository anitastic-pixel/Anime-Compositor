# B-303: frame times with Glue Gun (D-424)

Measured on 2026-10-10, built with `cargo test --release --test b303_glue_gun` from a7467e8f
(B-303's code), and run with `--ignored b303_glue_gun_timing`: once on the card, then once with
`B303_CPU` set on the processor. `tasklist` showed no other cargo, rustc or test program before, between and after the two runs.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Glue
Gun, every eighth frame asked for whole at Full. "Keyed across" is the brush keyed from 10, 50
per cent at frame 0 to 90, 50 at frame 239.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.2 | 44.3 |
| Noise, then Glue Gun as added (width 20, density 5, time span 1) | 17.3 | 65.2 |
| Noise, then Glue Gun, the brush keyed across, width 60, density 10, time span 2 | 37.1 | 88.1 |
| Noise, then Glue Gun, the brush keyed across, wobbly 30 by 20, a point light, width 40 | 20.3 | 64.4 |

First loops (empty caches): card 15.8, 21.0, 39.2, 23.4; processor 43.1, 66.0, 88.4, 64.2.

**Reading it.** On the card, Glue Gun costs about 5 ms a frame over the Noise alone as added
(12.2 to 17.3, three layers), 24.9 ms with the brush keyed across at width 60,
density 10, time span 2 (481 blobs a layer) and 8.1 ms wobbly with a point light. The blobs
are laid on the processor each frame and handed to the card; reading the brush's 24 or 48 frames
before is part of the cost. At these settings the card is quicker than the processor here.
