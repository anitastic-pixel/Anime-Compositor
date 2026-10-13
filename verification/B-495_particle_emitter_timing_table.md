# B-495: frame times with Particle Emitter (D-615) — PROVISIONAL

Measured on 2026-10-13, built with `cargo test --release --test b495_particle_emitter` from
4a196d84 (B-495's code), and run with `--ignored b495_particle_emitter_timing`, on the processor
(the effect has no card path).
**The machine was not quiet.** Another lane's `b155_gpu_chain` was running through the run, and
the shot with no effect swung from about 42 to 118 ms a frame. The numbers are recorded as they
came out. A quiet re-measure is owed.

- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with Particle
Emitter on its layers, each listed frame drawn whole at Full with empty caches, three times; the
median of the three, milliseconds. A later frame has more particles alive and works each one out
over more steps from its birth.

| Shot | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---:|---:|---:|---:|
| no effect | 106.4 ms | 118.5 ms | 62.9 ms | 41.9 ms |
| Particle Emitter as added | 68.3 ms | 135.6 ms | 118.6 ms | 117.1 ms |
| 1000 glowing spheres a second, added | 126.7 ms | 146.0 ms | 141.3 ms | 141.9 ms |
| 2000 sparks a second living 3 s, turbulence, bouncing on a floor | 116.3 ms | 219.7 ms | 247.8 ms | 255.1 ms |

**Reading it.** The busiest shot, 2000 sparks a second with turbulence and a floor (about 6000
alive once it is full), rises to about 220 to 255 ms a frame; the lighter shots stay at about 120
to 145 ms. The shot with no effect varied by almost three times between frames, so the gap it
leaves to each row is not to be trusted here. Re-run the command above when no other cargo, rustc
or test process is running, and replace this table.
