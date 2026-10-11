# B-383: frame times with Key Cleaner (D-503)

Measured on 2026-10-10, built with `cargo test --release --test
b383_key_cleaner` from b92bb627 (B-383's code), and run with `--ignored
b383_key_cleaner_timing`: once with the card drawing the frame, then once with `B383_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running, and the card was idle.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Key Cleaner,
every eighth frame asked for whole at Full. The cleaner is worked on the processor in both runs; the
card run draws everything else on the card. **Again** is the median of the loops after the first
(seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 13.0 | 43.9 |
| Noise, then Key Cleaner as added (radius 10) | 684.3 | 670.9 |
| Noise, then Key Cleaner radius 4, contrast 40 | 695.5 | 666.2 |
| Noise, then Key Cleaner radius 30, strength 60 | 711.7 | 690.2 |

First loops (empty caches): card 16.4, 733.3, 790.4, 841.0; processor 42.0, 720.5, 718.5, 683.2.

**Reading it.** About 223.8, 227.5 and 232.9 ms a 1080p layer in the card's frame (the cleaner drawn on the processor, the card drawing what follows it), over the row's Target P3 (8 ms); on the processor about 209.0, 207.4 and 215.4 ms a layer.
