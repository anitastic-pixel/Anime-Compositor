# B-290: frame times with Heat Shimmer (D-411)

Measured on 2026-10-10, built with `cargo test --release --test b290_heat_shimmer` from 3f74cd7c
(B-290's code), and run with `--ignored b290_heat_shimmer_timing`: once on the card, then once
with `B290_CPU` set on the processor. `tasklist` showed no other cargo, rustc or test program
before, between and after the two runs, and the owner's app was not running.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame on each of its first three layers, then the effect, every eighth
frame asked for whole at Full. **Again** is the median of the loops after the first (seven on
the card, two on the processor), milliseconds a frame. Turbulent Displace is amount 30, size 40,
complexity 2, speed 0, After Effects' units.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 39.7 |
| Noise, then Turbulent Displace, no drift (Drift Speed 0) | 17.7 | 124.4 |
| Noise, then Turbulent Displace, drifting up at 4 pixels a frame | 16.9 | 124.2 |

First loops (empty caches): card 16.0, 21.4, 20.1; processor 40.1, 123.5, 123.3.

**Reading it.** The drift costs nothing that can be measured (the drifting row is even a little
faster, inside the run's own spread): it is worked once a frame and moves the point the field is
read at, inside Turbulent Displace's own pass. Turbulent Displace itself adds about 1.5 to 2 ms a
1080p layer on the card, inside Target P2, and about 28 ms a layer on the processor.
