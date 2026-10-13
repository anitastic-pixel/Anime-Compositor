# B-492: frame times with Particle Plane (D-612) — PROVISIONAL

Measured on 2026-10-13, built with `cargo test --release --test b492_particle_plane` from
633c0dba (B-492's code), and run with `--ignored b492_particle_plane_timing`: once on the card,
then once with `B492_CPU` set on the processor.
**The machine was not quiet.** Another lane's test (`b551_drizzle`, about 1.3 GB) was running
through both runs. The numbers below are recorded as they came out. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then Particle Plane,
every eighth frame asked for whole at Full. **Again** is the median of the loops after the first
(seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 51.7 | 67.3 |
| Noise, then Particle Plane as added | 59.4 | 74.6 |
| Noise, then Particle Plane, 1000 a second, stars, faded out | 37.5 | 56.6 |
| Noise, then Particle Plane, 2000 a second living 3 s, vortex, faded spheres, added | 37.5 | 59.1 |

First loops (empty caches): card 90.3, 155.8, 58.9, 74.8; processor 73.3, 82.1, 56.8, 58.8.

**Reading it.** Every shot draws to the end on both. As added, the Plane on three layers costs
about 8 ms a frame over the Noise alone on the card and about 7 on the processor. The two busier
shots came out faster than the Noise alone on both; why was not measured (a guess, not checked:
the Plane draws over a clear layer, so those layers' pictures may cover less of the frame than a
full-frame Noise does). How much of each number is the effect and how much the other run's use of
the machine is not known from this table. Re-run the two commands above when no other cargo,
rustc or test process is running and the card is idle, and replace this table.
