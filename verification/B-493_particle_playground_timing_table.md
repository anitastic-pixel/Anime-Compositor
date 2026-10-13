# B-493: frame times with Particle Playground (D-613) — PROVISIONAL

Measured on 2026-10-13, built with `cargo test --release --test b493_particle_playground` from
16010ed0 (B-493's code), and run with `--ignored b493_particle_playground_timing`, on the
processor (the effect has no card path).
**The machine was not quiet.** Other lanes' tests (`b530_pixel_motion_blur` and `b552_hair`, about
1.4 to 1.9 GB each, and `b481_bezier_warp`) were running through the run. The numbers are
recorded as they came out. A quiet re-measure is owed.

- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with Particle
Playground on its layers, each listed frame drawn whole at Full with empty caches, three times;
the median of the three, milliseconds. A later frame works every particle out over more steps
from its birth.

| Shot | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---:|---:|---:|---:|
| no effect | 36.6 ms | 42.9 ms | 43.3 ms | 46.8 ms |
| Particle Playground as added | 65.7 ms | 58.8 ms | 64.8 ms | 53.7 ms |
| fireworks: 10 shells a second, each bursting into 16 sparks | 60.2 ms | 53.9 ms | 57.2 ms | 68.2 ms |
| the layer exploded into 16-pixel pieces, falling | 51.7 ms | 57.4 ms | 52.5 ms | 52.3 ms |

**Reading it.** Every shot draws on the processor at about 50 to 70 ms a frame, against about 37
to 47 ms with no effect: roughly 10 to 25 ms for the Playground. The
rows do not rise steadily toward frame 239, and the shot as added was sometimes slower than the
busier ones; with three other tests using the machine, those differences are within the noise and
were not looked into. Re-run the command above when no other cargo, rustc or test process is
running, and replace this table.
