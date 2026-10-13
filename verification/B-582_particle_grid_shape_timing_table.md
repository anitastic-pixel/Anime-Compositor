# B-582: frame times with Particle Grid's Box Strings and Layer forms (D-712) — PROVISIONAL

Measured on 2026-10-13, built with `cargo test --release --test b582_particle_grid_shape` from
c3f08101 (B-582's code), and run with `--ignored b582_particle_grid_shape_timing`. **The machine
was not quiet:** another `b581_particle_emitter_aux` test process, a compiler (rustc) and two
cargo processes were running when the measurement began, and two cargo processes when it ended.
The numbers hang together (no effect fastest, the processor slower than the card) but are
provisional; a quiet re-measure is owed. Each figure is the median of three, frames drawn with
empty caches, the reference shot (1920 by 1080, 24 a second) with Particle Grid on its first
layer.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release
- Drawn by: the card and the processor (D-618)

| Shot | Drawn by | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---|---:|---:|---:|---:|
| no effect | card | 39.1 ms | 35.8 ms | 37.2 ms | 37.4 ms |
| no effect | processor | 38.2 ms | 33.6 ms | 41.5 ms | 37.1 ms |
| Box Grid, 100 by 100 points (D-616, for comparison) | card | 44.2 ms | 42.7 ms | 44.0 ms | 42.6 ms |
| Box Grid, 100 by 100 points (D-616, for comparison) | processor | 45.8 ms | 49.0 ms | 48.4 ms | 45.9 ms |
| Box Strings, 100 strings of 100 points, rippling | card | 47.1 ms | 48.4 ms | 44.7 ms | 44.9 ms |
| Box Strings, 100 strings of 100 points, rippling | processor | 51.6 ms | 49.9 ms | 51.2 ms | 56.6 ms |
| Layer, 200 by 100 points on layer-4's shape | card | 45.5 ms | 46.8 ms | 45.5 ms | 45.7 ms |
| Layer, 200 by 100 points on layer-4's shape | processor | 49.5 ms | 50.4 ms | 49.6 ms | 50.8 ms |

Read as: Box Strings costs about as much as the same Box Grid (a few milliseconds over the frame
with no effect); Layer's 20,000 points, of which only those on layer-4's shape are drawn, cost
about the same again. Nothing grows with the frame, as no setting is keyed.
