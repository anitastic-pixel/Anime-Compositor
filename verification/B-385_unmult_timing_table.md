# B-385: frame times with Unmult (D-505)

Measured on 2026-10-10, built with `cargo test --release --test
b385_unmult` from f525d8c9 (B-385's code), and run with `--ignored
b385_unmult_timing`: once on the card, then once with `B385_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running, and the card was idle.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Unmult,
every eighth frame asked for whole at Full. **Again** is the median of the loops after the
first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.1 | 40.8 |
| Noise, then Unmult as added (black) | 17.3 | 42.9 |
| Noise, then Unmult, white, white level 80, softness 20 | 17.5 | 43.3 |
| Noise, then Unmult, black level 20, softness 10, matting off | 17.2 | 43.2 |

First loops (empty caches): card 15.7, 21.5, 21.3, 21.0; processor 40.2, 42.7, 43.5, 42.6.

**Reading it.** About 1.7 to 1.8 ms a 1080p layer on the card, over the row's Target P1 (1 ms, a one-pixel colour change), as the keyers before it are; on the processor about 0.7 to 0.8 ms a layer.
