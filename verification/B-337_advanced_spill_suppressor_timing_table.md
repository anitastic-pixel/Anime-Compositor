# B-337: frame times with Advanced Spill Suppressor (D-457)

Measured on 2026-10-10, built with `cargo test --release --test
b337_advanced_spill_suppressor` from 6a8a4c1d (B-337's code), and run with `--ignored
b337_advanced_spill_suppressor_timing`: once on the card, then once with `B337_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Advanced
Spill Suppressor, every eighth frame asked for whole at Full. **Again** is the median of the
loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.1 | 45.3 |
| Noise, then Standard as added | 17.5 | 57.7 |
| Noise, then Ultra as added | 18.1 | 58.8 |

First loops (empty caches): card 15.8, 22.3, 22.4; processor 42.2, 57.6, 60.0.

A second card run straight after gave 11.9, 17.3 and 17.9 again, the same to 0.2 ms.

**Reading it.** This is over Target P1 (1 ms or less for a one-pixel colour change): about 1.8 to 2.0 ms a 1080p layer on the card; on the processor about 4.1 to 4.5 ms a layer.
