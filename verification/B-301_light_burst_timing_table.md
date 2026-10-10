# B-301: frame times with Light Burst (D-422)

Measured on 2026-10-10, built with `cargo test --release --test b301_light_burst` from 70ec7c5c
(B-301's code), and run with `--ignored b301_light_burst_timing`: once on the card, then once with
`B301_CPU` set on the processor. **PROVISIONAL**: other lanes' cargo and test programs were running, so these numbers may be high.

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
| Noise alone | 12.4 | 40.8 |
| Noise, then Light Burst as added (straight, ray length 50) | 31.0 | 1002.8 |
| Noise, then Light Burst, fade, ray length 100 | 30.5 | 1410.3 |
| Noise, then Light Burst, center, Set Color on #ff8000 | 30.6 | 673.2 |

First loops (empty caches): card 16.1, 34.8, 33.9, 33.9; processor 40.4, 840.6, 1591.3, 892.2.

**Reading it.** On the card, Light Burst costs about 19 ms a frame over the Noise alone as added
(12.4 to 31.0, three layers), 18.1 ms with fade at ray length 100 and
18.2 ms with center and Set Color on. At these settings the card is quicker than the processor here.
