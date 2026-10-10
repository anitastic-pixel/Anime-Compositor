# B-296: frame times with Grid (D-417)

**PROVISIONAL.** Measured on 2026-10-10, built with `cargo test --release --test b296_grid` from
4eca322f (B-296's code), and run with `--ignored b296_grid_timing`: once on the card, then once
with `B296_CPU` set on the processor. The machine was not quiet: other lanes' cargo and rustc
builds were running during and after both runs. These numbers are to be measured again on a
quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Grid, every eighth frame asked for whole at Full. **Again** is the median of the loops
after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.0 | 44.6 |
| Noise, then Grid as added (border 2, None) | 17.2 | 69.2 |
| Noise, then Grid, Width Slider 100, border 4, orange at 70, Normal | 18.2 | 56.7 |
| Noise, then Grid, 120 by 80, border 10, feathers 6 and 12, Multiply | 18.6 | 59.7 |

First loops (empty caches): card 16.7, 20.7, 21.4, 21.0; processor 44.9, 66.7, 65.1, 58.5.

**Reading it, provisionally.** A grid costs about 2 ms a 1080p layer on the card (17.2 to 18.6
against 12.0, over three layers) and about 4 to 8 on the processor; the processor's spread is the
other programs' noise.
