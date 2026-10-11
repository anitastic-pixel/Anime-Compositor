# B-381: frame times with Difference Matte (D-501)

Measured on 2026-10-10, built with `cargo test --release --test
b381_difference_matte` from e3fcdef7 (B-381's code), and run with `--ignored
b381_difference_matte_timing`: once on the card, then once with `B381_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running, and the card was idle.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Difference
Matte against layer 4, every eighth frame asked for whole at Full. **Again** is the median of the
loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.1 | 40.9 |
| Noise, then Difference Matte against layer 4 (tolerance 15) | 40.6 | 72.3 |
| Noise, then Difference Matte against layer 4, softness 10, blurred at 2 | 47.3 | 145.0 |

First loops (empty caches): card 15.9, 39.6, 46.5; processor 40.2, 70.6, 144.3.

**Reading it.** About 9.5 ms a 1080p layer on the card unblurred and 11.7 blurred at 2, over the row's Target P1 (1 ms), the plate layer drawn on the processor first (D-189 path, as Aerial Haze and Gradient Wipe are, about 10.8 ms a layer for Aerial Haze); on the processor about 10.5 and 34.7 ms a layer.
