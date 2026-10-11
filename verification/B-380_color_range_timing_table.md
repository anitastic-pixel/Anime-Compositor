# B-380: frame times with Color Range (D-500)

Measured on 2026-10-10, built with `cargo test --release --test
b380_color_range` from cf17fb6a (B-380's code), and run with `--ignored
b380_color_range_timing`: once on the card, then once with `B380_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running, and the card was idle.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Color
Range, every eighth frame asked for whole at Full. **Again** is the median of the
loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.0 | 42.0 |
| Noise, then Color Range, RGB, fuzziness 60 | 17.6 | 49.2 |
| Noise, then Color Range, Lab, fuzziness 30 | 18.6 | 59.5 |

First loops (empty caches): card 15.4, 21.2, 23.1; processor 41.4, 49.0, 59.0.

**Reading it.** About 1.9 to 2.2 ms a 1080p layer on the card, over the row's Target P1 (1 ms, a one-pixel colour change), as the other keyers are; on the processor about 2.4 to 5.8 ms a layer.
