# B-301: frame times with Light Burst (D-422)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b301_light_burst --no-run` and run with `--ignored b301_light_burst_timing`: once on the card, then once with `B301_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 12.4, processor 40.8; Noise, then Light Burst as added (straight, ray length 50) card 31.0, processor 1002.8 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Light
Burst, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 13.4 | 44.9 |
| Noise, then Light Burst as added (straight, ray length 50) | 31.6 | 823.7 |
| Noise, then Light Burst, fade, ray length 100 | 31.2 | 992.4 |
| Noise, then Light Burst, center, Set Color on #ff8000 | 31.3 | 641.3 |

First loops (empty caches): card 16.9, 35.3, 33.7, 34.3; processor 44.0, 822.5, 941.8, 642.1.

**Reading it.** On the card, Light Burst costs about 18 ms a frame over the Noise alone as added
(13.4 to 31.6, three layers), 17.8 ms with fade at ray length 100 and 17.9 ms with center and Set
Color on, about 6 ms a 1080p layer. On the processor the same settings take 641 to 992 ms a frame.
At these settings the card is quicker than the processor here, by twenty to thirty times.
