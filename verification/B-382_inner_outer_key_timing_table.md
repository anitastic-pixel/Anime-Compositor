# B-382: frame times with Inner/Outer Key (D-502)

Measured on 2026-10-10, built with `cargo test --release --test
b382_inner_outer_key` from 797ddfff (B-382's code), and run with `--ignored
b382_inner_outer_key_timing`: once with the card drawing the frame, then once with `B382_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running, and the card was idle.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Inner/Outer
Key by the layer's masks (a circle of radius 250 inside one of 400), every eighth frame asked for whole
at Full. The key is worked on the processor in both runs; the card run draws everything else on the
card. **Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.0 | 44.4 |
| Noise, then Inner/Outer Key, two masks | 240.3 | 242.1 |
| Noise, then Inner/Outer Key, one mask, radius 10 | 224.9 | 215.9 |
| Noise, then Inner/Outer Key, two masks, thin 3, feather 12, threshold 20 | 302.2 | 297.5 |

First loops (empty caches): card 15.8, 243.2, 272.9, 360.6; processor 40.5, 248.0, 214.6, 305.6.

**Reading it.** About 76.1, 71.0 and 96.7 ms a 1080p layer in the card's frame (the key drawn on the processor, the card drawing what follows it), over the row's Target P3 (8 ms); on the processor about 65.9, 57.2 and 84.4 ms a layer.
