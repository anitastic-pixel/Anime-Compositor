# B-494: frame times with Particle Playground's property mappers (D-614) — PROVISIONAL

Measured on 2026-10-13, built with `cargo test --release --test b494_particle_mapper` from
6569af57 (B-494's code), and run with `--ignored b494_particle_mapper_timing`, on the processor
(the effect has no card path).
**The machine was not quiet.** Other lanes' tests (`b531_timewarp` at about 2.2 GB, `b552_hair`,
`b155_gpu_chain` and `b518_sound_read`) were running through the run; the shot with no effect
took about 95 to 110 ms a frame here against 37 to 47 ms in B-493's run. The numbers are recorded
as they came out. A quiet re-measure is owed.

- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with Particle
Playground on its layers, each listed frame drawn whole at Full with empty caches, three times;
the median of the three, milliseconds.

| Shot | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---:|---:|---:|---:|
| no effect | 95.4 ms | 109.4 ms | 109.7 ms | 109.8 ms |
| Particle Playground as added (no mapper) | 127.0 ms | 151.4 ms | 122.1 ms | 139.1 ms |
| as added, both mappers reading the layer: red to scale, kept; green times opacity, for the moment | 121.3 ms | 122.1 ms | 125.8 ms | 132.5 ms |

**Reading it.** With both mappers on, frames took about 120 to 135 ms, no slower than the
Playground with no mapper (about 120 to 150 ms): on this loaded machine the mappers add nothing
that stands out from the noise. Re-run the command above when no other cargo, rustc or test
process is running, and replace this table.
