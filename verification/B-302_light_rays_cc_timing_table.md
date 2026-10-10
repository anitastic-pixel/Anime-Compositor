# B-302: frame times with Light Rays' CC controls (D-423)

Measured on 2026-10-10, built with `cargo test --release --test b302_light_rays_cc` from 1894d62e
(B-302's code), and run with `--ignored b302_light_rays_cc_timing`: once on the card, then once with
`B302_CPU` set on the processor. The card run was quiet: `tasklist` showed no other cargo, rustc or test program before or after it. **The processor numbers are PROVISIONAL**: another lane's cargo and rustc were running when the processor run ended, so they may be high.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Light
Rays with CC's controls, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 41.1 |
| Noise, then Light Rays as added (round, radius 50, warp 50, none) | 36.3 | 616.0 |
| Noise, then Light Rays, radius 400, warp 300, add | 49.2 | 1905.5 |
| Noise, then Light Rays, square 300 turned 30, warp 0, Color from Source off #ff8000, screen | 32.8 | 701.8 |

First loops (empty caches): card 16.0, 39.4, 51.9, 37.8; processor 40.6, 550.4, 1736.9, 675.0.

**Reading it.** On the card, Light Rays costs about 24 ms a frame over the Noise alone as added
(12.3 to 36.3, three layers), 36.9 ms at radius 400, warp 300, add and
20.5 ms with the square source screened. At these settings the card is quicker than the processor here.
D-124's Light Rays, which older projects keep, is measured in B-76 and its frames are unchanged.
