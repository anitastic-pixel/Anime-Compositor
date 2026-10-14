# B-510: frame times with Wave World (D-630)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b510_wave_world --no-run` and run once with `--ignored b510_wave_world_timing`. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 3.7 GB of its memory held by the desktop and open programs); the same after it. The processor was 2% busy before the run and 6% after. The owner's app was not open. One run; each figure is the median of three draws. Machine: AMD Ryzen 9 9900X (24 threads), NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88), Windows 11, release build.

Earlier provisional figures for the build as committed (2026-10-12, at bd895a5d, on a machine that was not quiet, two runs), frame 239: no effect 41.1 and 112.8; Grid 60 179.0 and 197.0; Grid 145 475.2 and 482.3 ms.

- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor (Wave World has no card path)

**What is timed:** the reference shot (1920 by 1080, 24 a second) with Wave World as added on its first layer (Grid 60), and again at Grid 145; frames 0, 60, 120 and 239, each drawn whole on the processor at Full with empty caches, three times; the median of the three, in milliseconds. Wave World runs the water again from the layer's in point for every frame, so a later frame costs more.

## The build as committed (bd895a5d), measured at 90a33e9a

| Shot | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---:|---:|---:|---:|
| no effect | 35.2 ms | 34.7 ms | 35.0 ms | 35.2 ms |
| Wave World, Grid 60 (as added) | 50.6 ms | 57.2 ms | 61.2 ms | 71.6 ms |
| Wave World, Grid 145 | 47.5 ms | 73.5 ms | 98.8 ms | 144.0 ms |

## Before and after bd895a5d

The first build (076a1b6d) sent each step's rows to the threads. Not built again: this is still the busy run of 2026-10-12 (6 to 8 other build processes), run straight before the busy run 2 of the committed build, and stays PROVISIONAL:

| Shot | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---:|---:|---:|---:|
| no effect | 110.3 | 50.3 | 54.7 | 104.7 |
| Wave World, Grid 60 (as added) | 122.4 | 222.7 | 460.5 | 375.2 |
| Wave World, Grid 145 | 114.3 | 351.6 | 568.7 | 879.4 |

In that busy run bd895a5d, which keeps the steps of a grid under 65536 cells on one thread, brought Grid 145 at frame 239 from 879.4 to 482.3 ms and Grid 60 from 375.2 to 197.0 ms; two earlier runs of 076a1b6d, with another lane's GPU test or up to 31 build processes running, measured 3987.8 and 5343.9 ms at Grid 145, frame 239. Since the first build was not measured again, how much bd895a5d gained cannot be put as a quiet factor.

**Reading it.** On a quiet machine, Wave World as added (Grid 60) adds about 15 ms a frame over no effect at the in point (50.6 against 35.2) and about 36 ms 10 seconds after it (71.6 at frame 239); at Grid 145, about 12 ms at the in point and about 109 ms at frame 239 (144.0). The busy run put frame 239 at roughly 80 to 140 ms added as added and 370 to 430 at Grid 145; quiet, it is far less. The water's share still grows in step with the time since the in point: a frame a minute in runs six times the steps of a frame 10 seconds in (not measured here). That ceiling is logged as a gap (D-630); the upgrade, a store of the water every so many frames, gives the same pixels and waits until a timing on a real shot asks for it.
