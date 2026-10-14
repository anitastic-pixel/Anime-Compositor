# B-302: frame times with Light Rays' CC controls (D-423)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b302_light_rays_cc --no-run` and run with `--ignored b302_light_rays_cc_timing`: once on the card, then once with `B302_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 12.3, processor 41.1; Noise, then Light Rays as added (round, radius 50, warp 50, none) card 36.3, processor 616.0 ms.

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
| Noise alone | 13.4 | 44.4 |
| Noise, then Light Rays as added (round, radius 50, warp 50, none) | 36.5 | 588.3 |
| Noise, then Light Rays, radius 400, warp 300, add | 48.0 | 1636.8 |
| Noise, then Light Rays, square 300 turned 30, warp 0, Color from Source off #ff8000, screen | 33.1 | 591.9 |

First loops (empty caches): card 16.8, 39.6, 50.5, 36.2; processor 44.5, 562.9, 1644.7, 589.4.

**Reading it.** On the card, Light Rays costs about 23 ms a frame over the Noise alone as added
(13.4 to 36.5, three layers), 34.6 ms at radius 400, warp 300, add and 19.7 ms with the square
source screened. On the processor the same settings take 588 to 1637 ms a frame. At these settings
the card is quicker than the processor here, by sixteen to thirty-four times.
D-124's Light Rays, which older projects keep, is measured in B-76 and its frames are unchanged.
