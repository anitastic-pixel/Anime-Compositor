# B-294: frame times with Ellipse (D-415)

Measured on 2026-10-10, built with `cargo test --release --test b294_ellipse` from 662e23da
(B-294's code), and run with `--ignored b294_ellipse_timing`: once on the card, then once with
`B294_CPU` set on the processor. `tasklist` showed no other cargo, rustc or test program before, between and after the two runs, and the owner's app was not running.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then an Ellipse, every eighth frame asked for whole at Full. **Again** is the median of the loops
after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 13.8 | 47.4 |
| Noise, then Ellipse as added (200 across, thickness 8, over the layer) | 19.6 | 54.8 |
| Noise, then Ellipse, 900 by 400, thickness 30, softness 80 | 19.5 | 56.8 |
| Noise, then Ellipse, 600 by 600 alone, thickness 20 | 19.9 | 56.8 |

First loops (empty caches): card 18.1, 23.8, 23.1, 22.3; processor 46.0, 55.8, 54.9, 55.7.

**Reading it.** An ellipse costs about 2 ms a 1080p layer on the card and 2 on the
processor (as added, against the Noise alone, divided by the three layers).
