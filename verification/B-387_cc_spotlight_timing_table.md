# B-387: frame times with CC Spotlight (D-507)

Measured on 2026-10-11, built with `cargo test --release --test
b387_cc_spotlight` from dce14dce (B-387's code), and run with `--ignored
b387_cc_spotlight_timing`: once on the card, then once with `B387_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running before, between or
after the two runs, and the card was idle.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the CC
Spotlight, every eighth frame asked for whole at Full. **Again** is the median of the loops
after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 11.8 | 45.9 |
| Noise, then CC Spotlight as added (light shadow) | 14.7 | 56.6 |
| Noise, then CC Spotlight, light add, orange, intensity 250 | 14.5 | 56.0 |
| Noise, then CC Spotlight, light add plus, low and wide | 14.5 | 54.0 |

First loops (empty caches): card 15.7, 19.2, 17.3, 18.7; processor 40.7, 56.4, 55.0, 53.9.

**Reading it.** About 0.9 to 1.0 ms a 1080p layer on the card, at the row's Target P1 (1 ms),
and about 2.7 to 3.6 ms on the processor. The processor's Noise alone ran slower again than
first (45.9 against 40.7), so its per-layer figure is the rougher of the two.
