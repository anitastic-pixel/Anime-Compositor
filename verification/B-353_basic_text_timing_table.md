# B-353: frame times with Basic Text (D-473)

Measured on 2026-10-11, built with `cargo test --release --test b353_basic_text` from 116a0de0
(B-353's code), and run with `--ignored b353_basic_text_timing`: once on the card, then once
with `B353_CPU` set on the processor. `tasklist` showed no other cargo, rustc or test program, and `nvidia-smi` showed the card's memory free of other programs, before, between and after the two runs.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Basic Text on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.6 | 44.4 |
| Noise, then Basic Text at size 200 | 37.8 | 62.8 |
| the same in two lines, stroke over fill, width 6 | 37.7 | 61.6 |

First loops (empty caches): card 16.5, 40.3, 39.2; processor 41.7, 61.7, 61.8.

**Reading it.** On the card, Basic Text costs about 25 ms a frame over the Noise alone in the first setting
(12.6 to 37.8, three layers), and the second setting takes 37.7: the words drawn once on the processor and kept, then laid on by one pass of the card.
At these settings the card is quicker than the processor here.
