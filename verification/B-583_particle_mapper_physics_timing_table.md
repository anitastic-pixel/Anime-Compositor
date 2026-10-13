# B-583: frame times with Particle Playground's further mapper properties (D-713)

Measured on 2026-10-13 by `b583_particle_mapper_physics_timing` (tests/b583_particle_mapper_physics.rs), the reference shot (1920 by 1080) with Particle Playground on its first layer, each frame drawn on the processor with empty caches three times, the median shown.

- Card: RTX 4070 Ti SUPER, driver 610.88 (not used: the particles are drawn on the processor here)
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11; release
- How quiet: no other build or test was running before or after the run (process list checked both times), processor load 9%; a browser and desktop tools were open. One run, so read the numbers as a guide, not a promise.

| Shot | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---:|---:|---:|---:|
| no effect | 34.0 ms | 33.8 ms | 35.4 ms | 35.2 ms |
| Particle Playground as added (no mapper) | 43.8 ms | 43.7 ms | 42.7 ms | 49.4 ms |
| as added, both mappers reading the layer: red to X Scale and green to Lifespan, kept; blue to Static Friction, for the moment | 45.2 ms | 45.9 ms | 44.7 ms | 43.8 ms |

With both mappers setting the new properties the Playground took 1.4 to 2.2 ms more a frame at frames 0 to 120 and 5.6 ms less at frame 239, so the new properties cost little next to the effect itself; the frame 239 difference is run-to-run variation.
