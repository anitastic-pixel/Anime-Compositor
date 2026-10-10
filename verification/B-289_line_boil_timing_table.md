# B-289: frame times with Line Boil (D-410)

Measured on 2026-10-10, built with `cargo test --release --test b289_line_boil` from 8ea3cd0e
(B-289's code), and run with `--ignored b289_line_boil_timing`: once on the card, then once with
`B289_CPU` set on the processor. `tasklist` showed no other cargo, rustc or test program before,
between and after the two runs, and the owner's app was not running.

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
| Noise alone | 11.5 | 40.3 |
| Noise, then Turbulent Displace, one seed (New Seed Every 0) | 16.1 | 132.8 |
| Noise, then Turbulent Displace, a new seed every 2 frames | 16.3 | 130.8 |

First loops (empty caches): card 15.4, 20.5, 19.8; processor 39.7, 129.8, 132.9.

**Reading it.** A new seed costs nothing that can be measured: the seed is worked once a frame
before the pass, and the pass is Turbulent Displace's own, unchanged. Turbulent Displace itself
adds about 1.5 ms a 1080p layer on the card, inside Target P2, and about 30 ms a layer on the
processor.
