# B-292: frame times with Checkerboard (D-413)

Measured on 2026-10-10, built with `cargo test --release --test b292_checkerboard` from 68e0e45a
(B-292's code), and run with `--ignored b292_checkerboard_timing`: once on the card, then once
with `B292_CPU` set on the processor. `tasklist` showed no other cargo, rustc or test program
before, between and after the two runs, and the owner's app was not running.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Checkerboard, every eighth frame asked for whole at Full. **Again** is the median of the
loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.2 | 44.9 |
| Noise, then Checkerboard as added (64 across, None) | 17.8 | 53.5 |
| Noise, then Checkerboard, Normal, orange at 70, width 100, feathers 16 and 8 | 18.1 | 55.1 |
| Noise, then Checkerboard, width 32, Overlay | 22.7 | 78.6 |

First loops (empty caches): card 16.3, 20.4, 22.1, 26.4; processor 43.9, 52.8, 53.9, 76.7.

**Reading it.** A checkerboard costs about 2 ms a 1080p layer on the card (6 ms for the three
layers) and 3 ms on the processor; Overlay, which turns both colours to and from their encoded
values, about 3.5 ms a layer on the card and 11 on the processor.
