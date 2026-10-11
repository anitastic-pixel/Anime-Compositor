# B-389: frame times with Glass Melt (D-509) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test
b389_glass_melt` from 22793aef (B-389's code), and run with `--ignored
b389_glass_melt_timing`: once on the card, then once with `B389_CPU` set on the processor.
**The machine was not quiet.** Other lanes' cargo and rustc processes were running before,
during and after both runs (three cargo builds and four rustc compiles were still running when
the second run ended). The numbers below are recorded as they came out. They are not a clean
measurement of Glass Melt: the card's Noise alone took 20.3 ms a frame here, against 11.8 ms on
the quiet machine in B-387's table. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Glass Melt,
every eighth frame asked for whole at Full. **Again** is the median of the loops after the first
(seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 20.3 | 69.5 |
| Noise, then Glass Melt, completion 50, revealing layer 4 (as added otherwise) | 66.7 | 209.0 |
| Noise, then Glass Melt, completion 50, layer 4 as the gradient, softness 30 | 93.1 | 234.7 |

First loops (empty caches): card 26.4, 75.1, 67.8; processor 74.2, 209.5, 304.9.

**Reading it.** Every shot draws to the end on both, and the card ran each Glass Melt shot about
three times as fast as the processor. Glass Melt's own share on the card (about 46 to 73 ms a
frame over the Noise alone) cannot be trusted while other compiles share the machine. Re-run the
two commands above when no other cargo, rustc or test process is running and the card is idle,
and replace this table.
