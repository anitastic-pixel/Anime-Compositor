# B-386: frame times with 3D Glasses (D-506)

Measured on 2026-10-11, built with `cargo test --release --test
b386_three_d_glasses` from d3be75e2 (B-386's code), and run with `--ignored
b386_three_d_glasses_timing`: once on the card, then once with `B386_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running, and the card was idle.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the 3D Glasses,
every eighth frame asked for whole at Full. **Again** is the median of the loops after the
first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.8 | 41.3 |
| Noise, then 3D Glasses against layer 4, balanced colored red blue | 46.6 | 78.2 |
| Noise, then 3D Glasses, the layer itself, red blue, convergence 7 | 17.3 | 65.9 |

First loops (empty caches): card 16.1, 45.2, 20.7; processor 40.9, 78.0, 66.6.

**Reading it.** With the layer itself as both views, about 1.5 ms a 1080p layer on the card, over
the row's Target P1 (1 ms), and about 8.2 ms on the processor. With another layer as the right
view, about 11.3 ms a layer on the card and 12.3 on the processor: the view layer is drawn on
the processor first (D-189's path, as Difference Matte's plate is, about 9.5 ms a layer there).
