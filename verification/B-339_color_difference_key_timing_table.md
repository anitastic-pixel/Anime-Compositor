# B-339: frame times with Color Difference Key (D-459)

Measured on 2026-10-10, built with `cargo test --release --test
b339_color_difference_key` from 5943f5ec (B-339's code), and run with `--ignored
b339_color_difference_key_timing`: once on the card, then once with `B339_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running, and the card was idle.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Color
Difference Key, every eighth frame asked for whole at Full. **Again** is the median of the
loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.1 | 40.9 |
| Noise, then Color Difference Key as added | 18.5 | 52.1 |
| Noise, then Color Difference Key, every levels set | 18.4 | 54.7 |

First loops (empty caches): card 15.8, 22.9, 22.1; processor 40.1, 51.7, 54.6.

**Reading it.** About 2.1 ms a 1080p layer on the card, within the row's Target P2 (4 ms or less) but over Target P1 (1 ms, a one-pixel colour change), as Advanced Spill Suppressor's is; on the processor about 3.7 to 4.6 ms a layer.
