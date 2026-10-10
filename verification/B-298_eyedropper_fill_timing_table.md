# B-298: frame times with Eyedropper Fill (D-419)

**PROVISIONAL.** Measured on 2026-10-10, built with `cargo test --release --test
b298_eyedropper_fill` from 47eaf612 (B-298's code), and run with `--ignored
b298_eyedropper_fill_timing`: once on the card, then once with `B298_CPU` set on the processor.
The machine was not quiet: other lanes' cargo builds and a b302_light_rays_cc test were running
during both runs; Noise alone took 133.2 ms on the card against 24.3 when Fill was measured.
These numbers are to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then an Eyedropper Fill, every eighth frame asked for whole at Full. **Again** is the median of
the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 133.2 | 102.3 |
| Noise, then Eyedropper Fill as added (one pixel) | 136.8 | 158.9 |
| Noise, then Eyedropper Fill, radius 200, All | 145.0 | 147.5 |
| Noise, then Eyedropper Fill, radius 1000, Including Alpha, alpha kept | 53.0 | 198.4 |

First loops (empty caches): card 156.0, 77.2, 206.7, 47.7; processor 98.3, 71.3, 156.9, 194.2.

**Reading it.** Not yet: the card's rows move by more than the effect could cost (the largest
area is faster than Noise alone), so the busy machine dominates. Measure again when quiet.
