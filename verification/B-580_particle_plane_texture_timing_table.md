# B-580: frame times with Particle Plane's textured particles (D-710) — PROVISIONAL

Measured on 2026-10-13, built with `cargo test --release --test b580_particle_plane_texture` from
47eaa949 (B-580's code), and run with `--ignored b580_particle_plane_texture_timing`, once on the
card and once with `B580_CPU=1` on the processor. **The machine was not quiet:** another lane's
`b155_gpu_chain` test, which draws on the same graphics card, and four cargo processes ran
through both measurements. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second) with a Noise that changes every
frame on its first three layers, so nothing is kept, then Particle Plane, every eighth frame asked
for as the viewer asks, whole, at Full. "First" is the median of the first loop's 30 frames,
with empty caches; "again" is the median of the loops after it (7 on the card, 2 on the
processor). Milliseconds a frame for all three layers.

| Shot | Drawn by | First | Again |
|---|---|---:|---:|
| Noise alone | card | 80.6 | 96.4 |
| Noise, then Particle Plane, textured discs, no texture | card | 278.8 | 152.7 |
| Noise, then Particle Plane, 1000 a second, textured discs of layer-4 (1920 by 1080), turning, over the drawing | card | 771.9 | 164.4 |
| Noise, then Particle Plane, 2000 a second living 3 s, vortex, textured faded discs of layer-4, added | card | 460.5 | 116.0 |
| Noise alone | processor | 72.6 | 81.9 |
| Noise, then Particle Plane, textured discs, no texture | processor | 145.5 | 102.6 |
| Noise, then Particle Plane, 1000 a second, textured discs of layer-4 (1920 by 1080), turning, over the drawing | processor | 413.5 | 326.6 |
| Noise, then Particle Plane, 2000 a second living 3 s, vortex, textured faded discs of layer-4, added | processor | 432.3 | 430.2 |

**Reading it.** On the processor, textured discs of a 1080p layer cost about 250 to 350 ms a
frame over the Noise alone for three layers; on the card about 20 to 70 ms. The card's first loops
are slow (the passes being built and the texture sent), and the card's Noise-alone line moved by
16 ms between loops, so differences under about 20 ms say nothing here.
