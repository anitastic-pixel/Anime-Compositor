# B-305: frame times with Lens Flare (D-426)

Measured on 2026-10-10, built with `cargo test --release --test b305_lens_flare` from 77a184c1
(B-305's code), and run with `--ignored b305_lens_flare_timing`: once on the card, then once with
`B305_CPU` set on the processor. `tasklist` showed no other cargo, rustc or test program before, between and after the two runs.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Lens Flare, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 13.3 | 41.1 |
| Noise, then Lens Flare as added (the 50-300mm zoom, ten parts) | 17.7 | 86.8 |
| Noise, then Lens Flare, the 35mm prime (six parts), brightness 200 | 15.9 | 74.2 |
| Noise, then Lens Flare, the 105mm prime at 75, 25, blend 30 | 15.9 | 75.1 |

First loops (empty caches): card 16.5, 23.2, 20.2, 19.6; processor 40.2, 85.1, 73.9, 74.6.

**Reading it.** On the card, Lens Flare costs about 4 ms a frame over the Noise alone as added
(13.3 to 17.7, three layers); one pass a layer, each pixel summing the lens's six to
ten parts on its own. At these settings the card is quicker than the processor here.
