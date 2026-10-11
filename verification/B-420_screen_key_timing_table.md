# B-420: frame times with Screen Key (D-540)

**PROVISIONAL.** Measured on 2026-10-11, built with `cargo test --release --test
b420_screen_key` from 1e107c2a (B-420's code), and run with `--ignored
b420_screen_key_timing`: once on the card, then once with `B420_CPU` set on the processor.
The machine was **not** quiet: other lanes' cargo and rustc builds and a b155 card test were
running on it at the same time. The card's numbers are disturbed most (its "Noise alone" first
loop, 220.6 ms, is many times B-384's 16.4 on a quiet machine, and its Screen Key shots come out
slower than the processor's), so they say nothing about the card's speed yet; to be measured
again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Screen Key,
every eighth frame asked for whole at Full. **Again** is the median of the loops after the first
(seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 44.9 | 45.3 |
| Noise, then Screen Key, screen colour orange, gain 120, clip 10 to 90 (on the card) | 157.5 | 73.5 |
| Noise, then Screen Key, the same as the screen matte (on the card) | 116.5 | 62.2 |
| Noise, then Screen Key, the same with shrink 2, softness 1.5 and edge refine 3 (the processor) | 1788.8 | 1076.5 |

First loops (empty caches): card 220.6, 120.5, 166.9, 1168.1; processor 42.9, 73.2, 61.2, 903.0.

**Reading it (provisional).** On the processor about 9.4 ms a 1080p layer as a plain key and
5.6 ms as the screen matte; with shrink, softness and edge refine about 344 ms a layer, most of
it Key Cleaner's guided fit (Key Cleaner alone is about 209 ms a layer, B-383), over the row's
Target P2 (4 ms). The card's figures are not readable on a busy machine.
