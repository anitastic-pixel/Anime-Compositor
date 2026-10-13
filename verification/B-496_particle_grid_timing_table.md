# B-496: frame times with Particle Grid (D-616) — PROVISIONAL, NOT USABLE

Measured on 2026-10-13, built with `cargo test --release --test b496_particle_grid` from
6d242375 (B-496's code), and run with `--ignored b496_particle_grid_timing`, on the processor
(the effect has no card path).
**The machine was busy.** Other lanes' tests (two `b155_gpu_chain` runs and `b485_reshape`) were
running through the run. The shot with no effect took from 120 to 229 ms a frame, and some
shots with 100,000 points were faster than the shot with no effect. These numbers say nothing
about Particle Grid's cost. They are recorded only so that the run is on file. A quiet
re-measure is owed.

- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with Particle Grid
on its layers, each listed frame drawn whole at Full with empty caches, three times; the median
of the three, milliseconds.

| Shot | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---:|---:|---:|---:|
| no effect | 178.3 ms | 228.6 ms | 217.7 ms | 120.5 ms |
| Particle Grid as added (1271 points) | 138.9 ms | 123.1 ms | 194.0 ms | 205.0 ms |
| 100 by 100 glowing points, rippling and drifting, added | 248.2 ms | 170.7 ms | 95.5 ms | 118.0 ms |
| 200 by 100 by 5 points tipped, coloured and pushed by a layer | 326.6 ms | 200.5 ms | 86.6 ms | 132.5 ms |

**Reading it.** Nothing can be read from these numbers. Re-run the command above when no other
cargo, rustc or test process is running, and replace this table.
