# B-295: frame times with Fractal (D-416)

**PROVISIONAL.** Measured on 2026-10-10, built with `cargo test --release --test b295_fractal` from
d7f7b182 (B-295's code), and run with `--ignored b295_fractal_timing`: once on the card, then once
with `B295_CPU` set on the processor. The machine was not quiet: other lanes' rustc and test
programs were running, and the card showed 56 to 94 per cent busy from other work during the card
run. The card's loops after the first came out slower than its first loop on the lighter shots,
which is that interference, not Fractal. These numbers are to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Fractal, every eighth frame asked for whole at Full. **Again** is the median of the loops
after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 28.2 | 59.8 |
| Noise, then Fractal as added (Escape Limit 100, Edge Detect 2 by 2) | 206.3 | 82.1 |
| Noise, then Fractal, Julia, Hue Wheel | 130.3 | 90.9 |
| Noise, then Fractal, Brute Force 4 by 4 | 318.1 | 315.7 |
| Noise, then Fractal, magnification 20 at the seahorse valley, Escape Limit 1000 | 1437.3 | 1387.7 |

First loops (empty caches): card 40.3, 85.6, 156.9, 293.1, 1514.4; processor 56.4, 79.8, 92.3,
307.2, 1473.7.

**Reading it, provisionally.** On the processor a Fractal as added costs about 7 ms a 1080p layer
(82.1 against 59.8, over three layers); on the card's first loop about 15. Brute Force 4 by 4 costs
about 85 ms a layer and the deep zoom at Escape Limit 1000 over 400 ms a layer, on either. On these
runs the card was no faster than the processor: the card works every step in the processor's own
double-precision rounding so the escape counts match exactly, and its double-precision speed is a
small share of its single-precision speed. A faster single-precision path would change pixels
past 1 level at deep zooms, so it is not taken here.
