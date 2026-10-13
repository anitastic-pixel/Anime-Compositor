# B-585: frame times with Particle Playground's Mass (D-715)

Measured on 2026-10-13 by `b585_particle_mapper_mass_timing` (tests/b585_particle_mapper_mass.rs), built with `cargo test --release` from 461be4c5 (B-585's code): the reference shot (1920 by 1080) with Particle Playground on its first layer, each frame drawn on the processor with empty caches three times, the median shown.

- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: Windows 11 Education; release
- Drawn by: the processor
- How quiet: no other cargo, rustc or test process before or after the run (process list checked both times); processor load 6% before, 20% just after (taken as the run ended). One run, so read the numbers as a guide, not a promise.

| Shot | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---:|---:|---:|---:|
| no effect | 35.2 ms | 34.5 ms | 34.4 ms | 34.7 ms |
| Particle Playground as added (no mapper) | 46.6 ms | 48.5 ms | 47.6 ms | 46.0 ms |
| as added, the Persistent mapper reading the layer: green to Y Force 100, kept | 45.4 ms | 43.8 ms | 45.8 ms | 44.1 ms |
| the same, and red to Mass 1 to 5, kept | 45.8 ms | 43.1 ms | 42.8 ms | 43.5 ms |

Adding Mass to the pushed shot changed the frame time by between 0.4 ms more and 3.0 ms less, which is within run-to-run variation: Mass costs nothing measurable.
