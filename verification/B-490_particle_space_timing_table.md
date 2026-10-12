# B-490: frame times with Particle Space (D-610) — PROVISIONAL

Measured on 2026-10-12, built with `cargo test --release --test b490_particle_space` from f93a492f
(B-490's code), and run with `--ignored b490_particle_space_timing`: once on the card, then once
with `B490_CPU` set on the processor.
**The machine was not quiet.** Other lanes' cargo and rustc processes (four cargo, one rustc
building) were running during both runs. The numbers below are recorded as they came out. They
are not a clean measurement of Particle Space: the processor's Noise alone took 47.8 ms a frame on
its first loop and 57.7 on the next two. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then Particle Space,
every eighth frame asked for whole at Full. **Again** is the median of the loops after the first
(seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 22.6 | 57.7 |
| Noise, then Particle Space as added (200 a second, lines) | 62.6 | 79.1 |
| Noise, then Particle Space, 1000 a second, shaded spheres, over the drawing | 68.1 | 81.1 |
| Noise, then Particle Space, 2000 a second living 3 s, vortex, faded spheres, added | 61.2 | 117.2 |

First loops (empty caches): card 28.3, 131.5, 98.5, 141.0; processor 47.8, 72.7, 88.5, 87.7.

**Reading it.** Every shot draws to the end on both. On the card each layer adds about 13 to 15
ms, inside the row's Target P5; the particles are worked out, ordered and put into tiles on the
processor before the one card pass, and on this busy machine that made the card add more over the
Noise than the processor did for the first two shots (about 40 against 21 to 23 ms for three
layers). With 6000 particles alive a layer (the last shot) the processor's drawing costs most.
How much of each number is the effect and how much the other runs' use of the machine is not
known from this table. Re-run the two commands above when no other cargo, rustc or test process is
running and the card is idle, and replace this table.
