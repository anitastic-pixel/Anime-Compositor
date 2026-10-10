# B-293: frame times with Circle (D-414)

Measured on 2026-10-10, built with `cargo test --release --test b293_circle` from f89f30c6
(B-293's code), and run with `--ignored b293_circle_timing`: once on the card, then once with
`B293_CPU` set on the processor. `tasklist` showed no other cargo, rustc or test program before, between and after the two runs, and the owner's app was not running.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Circle, every eighth frame asked for whole at Full. **Again** is the median of the loops
after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 11.9 | 44.5 |
| Noise, then Circle as added (radius 75, None) | 16.6 | 53.2 |
| Noise, then Circle, radius 200, feather 40, orange at 70, Normal | 15.9 | 52.9 |
| Noise, then Circle, Thickness 30 at radius 120, Overlay | 18.7 | 63.9 |

First loops (empty caches): card 16.1, 22.0, 19.5, 22.0; processor 42.1, 52.1, 51.5, 66.2.

**Reading it.** A circle costs about 2 ms a 1080p layer on the card and 3 on the
processor (as added, against the Noise alone, divided by the three layers).
