# B-584: frame times with Particle Plane's Textured Square (D-714)

Measured on 2026-10-13, built with `cargo test --release --test b584_particle_plane_square` from
648521af (B-584's code), and run with `--ignored b584_particle_plane_square_timing`, once on the
card and once with `B584_CPU=1` on the processor.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: Windows 11 Education
- Build: release
- How quiet: no other cargo, rustc or test process before or after the runs (process list
  checked both times), processor load 9% both times. One
  run, so read the numbers as a guide, not a promise.

**What is timed.** The reference shot (1920 by 1080, 24 a second) with a Noise that changes every
frame on its first three layers, so nothing is kept, then Particle Plane, every eighth frame asked
for as the viewer asks, whole, at Full. "First" is the median of the first loop's 30 frames,
with empty caches; "again" is the median of the loops after it (7 on the card, 2 on the
processor). Milliseconds a frame for all three layers.

| Shot | Drawn by | First | Again |
|---|---|---:|---:|
| Noise alone | card | 15.9 | 12.2 |
| Noise, then Particle Plane, textured discs, no texture (D-710, for comparison) | card | 51.6 | 38.7 |
| Noise, then Particle Plane, textured squares, no texture | card | 54.2 | 39.4 |
| Noise, then Particle Plane, 1000 a second, textured squares of layer-4 (1920 by 1080), turning, over the drawing | card | 248.4 | 61.8 |
| Noise alone | processor | 41.0 | 41.9 |
| Noise, then Particle Plane, textured discs, no texture (D-710, for comparison) | processor | 48.9 | 49.3 |
| Noise, then Particle Plane, textured squares, no texture | processor | 48.8 | 49.5 |
| Noise, then Particle Plane, 1000 a second, textured squares of layer-4 (1920 by 1080), turning, over the drawing | processor | 217.5 | 216.7 |

**Reading it.** Textured squares cost what textured discs cost: within 0.7 ms a frame on the card
and 0.2 ms on the processor, on the same shot. 1000 textured squares a second showing a 1080p
layer, on each of three layers, add about 50 ms a frame on the card once its passes are built
and 175 ms on the processor.
