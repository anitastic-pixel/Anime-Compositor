# B-336: frame times with Deband (D-456)

Measured on 2026-10-10, built with `cargo test --release --test
b336_deband` from fc729350 (B-336's code), and run with `--ignored
b336_deband_timing`: once on the card, then once with `B336_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Deband on
each of its first three layers, every eighth frame asked for whole at Full. **Again** is the
median of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| The shot alone | 11.4 | 20.3 |
| Deband as added | 52.4 | 234.7 |
| Deband blur 30, diffusion 4, noise 4 | 75.1 | 306.3 |

First loops (empty caches): card 13.8, 54.8, 80.5; processor 19.8, 219.0, 335.4.

**Reading it.** The row asks Target P2, 4 ms or less added on the card, so this is over Target P2 (4 ms or less added on the card): about 13.7 to 21.2 ms a layer on the card; on the processor about 71 to 95 ms a layer.
Most of it is the blur, worked in double precision so the card stays within 1 level of the
processor; a single-precision blur is left as a proposal. D-550 (B-430) took it up: `verification/B-430_deband_single_precision_timing_table.md`.
