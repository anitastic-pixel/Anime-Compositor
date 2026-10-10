# B-323: frame times with Add Grain (D-443)

Measured on 2026-10-10, built with `cargo test --release --test
b323_addgrain` from 1af2cd95 (B-323's code), and run with `--ignored
b323_addgrain_timing`: once on the card, then once with `B323_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then an Add Grain on each, every eighth frame asked for whole at Full. **Again** is the median
of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 41.7 |
| Noise, then Add Grain as added (size 1, blocky, in colour) | 16.6 | 62.4 |
| Noise, then Add Grain size 4, softness 1 (smooth) | 18.1 | 67.8 |
| Noise, then Add Grain size 3, softness 0.5 (both), Overlay | 18.7 | 75.5 |

First loops (empty caches): card 15.7, 20.4, 21.5, 22.3; processor 40.0, 61.9, 67.0, 75.8.

**Reading it.** Three Add Grains add 4.3 to 6.4 ms a frame on the card over Noise alone,
one grade pass each on a 1920 by 1080 layer: about 1.4 to 2.1 ms a layer, within Target P2 (4 ms
or less for one 1920 by 1080 layer).
