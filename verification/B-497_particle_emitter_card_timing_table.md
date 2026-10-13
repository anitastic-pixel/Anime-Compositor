# B-497: frame times with Particle Emitter on the card (D-617) — PROVISIONAL

Measured on 2026-10-13, built with `cargo test --release --test b495_particle_emitter` from
fd7ab8d2 (B-497's code), and run with `--ignored b495_particle_emitter_timing`.
**The machine was not quiet.** Two other lanes' tests (`b532_warp_stabilizer` and
`b554_polygon_shards`) ran through the whole measurement. The shot with no effect drawn by the
card swung from 179 to 429 ms a frame, and by the processor from 57 to 119 ms. The numbers are
recorded as they came out. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with Particle
Emitter on its first layer. Each listed frame is drawn whole at Full, three times, and the table
gives the median of the three, in milliseconds. Every draw starts with empty caches: a new frame
cache each time, and the card's kept pictures dropped (`gpu.forget()`) before each card draw.
Each shot first draws one frame on the card, not timed, so the card's passes are built. "card" is
the frame the page receives from the card; "processor" is the same frame drawn on the processor.
On the card, the particles are still worked out on the processor and only the drawing is on the
card.

| Shot | Drawn by | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---|---:|---:|---:|---:|
| no effect | card | 179.0 ms | 326.7 ms | 429.4 ms | 202.1 ms |
| no effect | processor | 58.1 ms | 118.5 ms | 74.3 ms | 57.1 ms |
| Particle Emitter as added | card | 84.4 ms | 131.2 ms | 169.8 ms | 79.7 ms |
| Particle Emitter as added | processor | 111.2 ms | 97.3 ms | 147.9 ms | 85.0 ms |
| 1000 glowing spheres a second, added | card | 76.8 ms | 79.0 ms | 86.0 ms | 82.6 ms |
| 1000 glowing spheres a second, added | processor | 87.0 ms | 89.5 ms | 96.8 ms | 91.7 ms |
| 2000 sparks a second living 3 s, turbulence, bouncing on a floor | card | 84.5 ms | 170.2 ms | 204.7 ms | 150.9 ms |
| 2000 sparks a second living 3 s, turbulence, bouncing on a floor | processor | 50.9 ms | 134.8 ms | 156.7 ms | 148.6 ms |

**Reading it.** These numbers do not show what the card saves. The shot with no effect, which
draws the least, was the slowest on the card. That points at the other tests and at this test's
first shot, not at the particles. With 1000 glowing spheres a second the card was about 10 ms a
frame faster than the processor at every frame. With 2000 sparks it was slower, but that shot
spends most of its time working the particles out, which is still done on the processor. Re-run
the command above when no other cargo, rustc or test process is running, and replace this table.
