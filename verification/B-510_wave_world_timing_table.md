# B-510: frame times with Wave World (D-630)

Measured on 2026-10-12, built with `cargo test --release --test b510_wave_world` and run with `--ignored`. The machine: AMD Ryzen 9 9900X (24 threads), NVIDIA GeForce RTX 4070 Ti SUPER, Windows 11. **PROVISIONAL:** not a quiet machine; 6 to 31 cargo or rustc processes from other lanes were counted before and after the runs, and another lane's GPU test was running. The owner's app was not touched.

- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor (Wave World has no card path)

**What is timed:** the reference shot (1920 by 1080, 24 a second) with Wave World as added on its first layer (Grid 60), and again at Grid 145; frames 0, 60, 120 and 239, each drawn whole on the processor at Full with empty caches, three times; the median of the three, in milliseconds. Wave World runs the water again from the layer's in point for every frame, so a later frame costs more.

## The build as committed (bd895a5d)

Run 1 (8 other processes counted):

| Shot | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---:|---:|---:|---:|
| no effect | 41.1 | 40.6 | 53.0 | 41.1 |
| Wave World, Grid 60 (as added) | 61.7 | 60.9 | 115.7 | 179.0 |
| Wave World, Grid 145 | 184.7 | 147.9 | 205.4 | 475.2 |

Run 2 (6 to 8 other processes counted):

| Shot | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---:|---:|---:|---:|
| no effect | 108.4 | 113.9 | 113.4 | 112.8 |
| Wave World, Grid 60 (as added) | 158.4 | 159.3 | 178.6 | 197.0 |
| Wave World, Grid 145 | 161.5 | 221.7 | 278.6 | 482.3 |

## Before and after bd895a5d

The first build (076a1b6d) sent each step's rows to the threads. Run straight before run 2 above, on the same machine:

| Shot | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---:|---:|---:|---:|
| no effect | 110.3 | 50.3 | 54.7 | 104.7 |
| Wave World, Grid 60 (as added) | 122.4 | 222.7 | 460.5 | 375.2 |
| Wave World, Grid 145 | 114.3 | 351.6 | 568.7 | 879.4 |

So bd895a5d keeps the steps of a grid under 65536 cells on one thread: at frame 239, Grid 145 went from 879.4 to 482.3 ms and Grid 60 from 375.2 to 197.0 ms. Two earlier runs of 076a1b6d, with another lane's GPU test or up to 31 build processes running, measured 3987.8 and 5343.9 ms at Grid 145, frame 239.

**Reading it.** As added, Wave World adds roughly 80 to 140 ms a frame 10 seconds after the in point on this busy machine, and at Grid 145 roughly 370 to 430 ms. The water's share grows in step with the time since the in point: a frame a minute in runs six times the steps of a frame 10 seconds in (not measured here). That ceiling is logged as a gap (D-630); the upgrade, a store of the water every so many frames, gives the same pixels and waits until a timing on a real shot asks for it. A quiet re-measure would settle the numbers.
