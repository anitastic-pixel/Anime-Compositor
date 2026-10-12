# B-491: frame times with Particle Space's textured particles (D-611) — PROVISIONAL

Measured on 2026-10-12, built with `cargo test --release --test b491_particle_texture` from
77875949 (B-491's code), and run with `--ignored b491_particle_texture_timing`: once on the card,
then once with `B491_CPU` set on the processor.
**The machine was not quiet.** Other lanes' cargo and rustc processes were running (four before
the card run, two between the runs, eight with two rustc building after). The numbers below are
recorded as they came out; the processor's Noise alone took 77.9 ms a frame on its first loop and
55.6 on the next two. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then Particle Space,
every eighth frame asked for whole at Full. The texture is `layer-4`, 1920 by 1080. **Again** is
the median of the loops after the first (seven on the card, two on the processor), milliseconds a
frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 30.7 | 55.6 |
| Noise, then Particle Space, textured squares, no texture | 67.6 | 71.1 |
| Noise, then Particle Space, 1000 a second, textured discs of layer-4, turning, over the drawing | 73.7 | 331.8 |
| Noise, then Particle Space, 2000 a second living 3 s, vortex, textured faded discs of layer-4, added | 227.0 | 339.7 |

First loops (empty caches): card 35.9, 143.4, 343.4, 910.2; processor 77.9, 72.5, 330.2, 355.5.

**Reading it.** Every shot draws to the end on both. Per layer, over the Noise alone: with no
texture, about 12 ms on the card and 5 on the processor; with the 1080p texture, about 14 and 92
(1000 alive), 65 and 95 (6000 alive). The untextured shot has
200 alive as added, so it is not a like-for-like base for the textured ones, and where the time
goes inside the effect was not measured. Known: the texture's smaller copies are made again
every frame on the processor for both (a ponytail note in `src/particles.rs`: keep them with the
layer's picture if large textures are common). How much of each number is the
effect and how much the other runs' use of the machine is not known from this table. Re-run the
two commands above when no other cargo, rustc or test process is running and the card is idle,
and replace this table.
