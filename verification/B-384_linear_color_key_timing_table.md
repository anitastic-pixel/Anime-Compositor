# B-384: frame times with Linear Color Key (D-504)

Measured on 2026-10-10, built with `cargo test --release --test
b384_linear_color_key` from 574f1b5e (B-384's code), and run with `--ignored
b384_linear_color_key_timing`: once on the card, then once with `B384_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running, and the card was idle.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Linear
Color Key, every eighth frame asked for whole at Full. **Again** is the median of the
loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.8 | 40.6 |
| Noise, then Linear Color Key as added (RGB) | 18.8 | 48.5 |
| Noise, then Linear Color Key, hue, key orange, tolerance 30 | 18.9 | 49.4 |
| Noise, then Linear Color Key, chroma, the matte | 18.8 | 49.9 |

First loops (empty caches): card 16.4, 23.3, 24.6, 22.7; processor 39.9, 48.0, 50.4, 49.6.

**Reading it.** About 2.0 ms a 1080p layer on the card, over the row's Target P1 (1 ms, a one-pixel colour change), as the other keyers are; on the processor about 2.6 to 3.1 ms a layer.
