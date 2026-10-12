# B-482: frame times with Mesh Warp (D-603)

Measured on 2026-10-12, built with `cargo test --release --test b482_mesh_warp` from 0e90fc81
(B-482's code), and run with `--ignored b482_mesh_warp_timing`: once on the card, then once with
`B482_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Other
lanes were building and testing beside it, and before the runs the graphics card had 2.3 GB of its
memory in use by other programs. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Mesh
Warp at Quality 8, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 50.0 | 99.3 |
| Noise, then Mesh Warp 2 by 2 with the middle pushed | 46.1 | 159.4 |
| Noise, then Mesh Warp 7 by 7 with three vertices moved, Quality 8 | 37.8 | 160.8 |

First loops (empty caches): card 72.8, 58.5, 52.6; processor 97.0, 156.0, 164.2.

**Reading it.** On the card the Noise alone took longer than either Mesh Warp shot, so on this busy
machine the card's numbers say only that Mesh Warp (warp mode 150, as Bezier Warp) costs less than
the machine's own swing; Bezier Warp, drawn the same way, measured 3 to 4 ms a 1080p layer. On the
processor the loops agreed with each other, and Mesh Warp added about 60 ms a frame over three
layers, about 20 ms a 1080p layer, much the same at 2 by 2 and 7 by 7.
