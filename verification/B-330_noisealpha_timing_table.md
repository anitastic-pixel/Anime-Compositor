# B-330: frame times with Noise Alpha (D-450)

Measured on 2026-10-10, built with `cargo test --release --test
b330_noisealpha` from ec6fa256 (B-330's code), and run with `--ignored
b330_noisealpha_timing`: once on the card, then once with `B330_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Noise Alpha on each, every eighth frame asked for whole at Full. **Again** is the median
of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.1 | 40.3 |
| Noise, then Noise Alpha as added (Uniform Random, Clamp) | 13.3 | 44.9 |
| Noise, then Noise Alpha Squared Random, Add, Wrap Back | 13.5 | 51.0 |
| Noise, then Noise Alpha Uniform Animation, phase 200, Scale | 13.4 | 45.8 |

First loops (empty caches): card 16.1, 18.0, 16.8, 16.5; processor 39.7, 44.9, 51.0, 45.7.

**Reading it.** Three Noise Alphas add 1.2 to 1.4 ms a frame on the card over Noise alone,
one grade pass each on a 1920 by 1080 layer; Target P1 asks 1 ms or less for one effect.
