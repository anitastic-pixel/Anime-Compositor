# B-571: frame times with Mesh Warp's vertex handles (D-701)

Measured on 2026-10-13, built with `cargo test --release --test b571_mesh_handles` from 2a75e621
(B-571's code), and run with `--ignored b571_mesh_handles_timing`: once on the card, then once
with `B571_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet.
Other lanes were building and testing beside it, and before the runs the graphics card had 7.4 GB of
its memory in use by other programs. Treat them as an upper bound, to be measured again on a quiet
machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Mesh Warp
as shown, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 21.9 | 44.4 |
| Noise, then Mesh Warp 2 by 2, the middle pushed and its handles turned | 32.1 | 68.3 |
| Noise, then Mesh Warp 7 by 7 at rest with four handles moved, Quality 8 | 28.5 | 78.3 |

First loops (empty caches): card 38.2, 36.6, 43.2; processor 43.8, 69.8, 79.7.

**Reading it.** The handles cost about 3 to 4 ms a 1080p layer on the card (about 7 to 10 ms over
three layers) and about 8 to 11 ms a layer on the processor. The mesh is worked on the processor either way; the card only draws it.
