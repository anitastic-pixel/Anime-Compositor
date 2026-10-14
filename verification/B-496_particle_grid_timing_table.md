# B-496: frame times with Particle Grid (D-616)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b496_particle_grid --no-run` and run once with `--ignored b496_particle_grid_timing`, which now times every shot both on the card and on the processor in that one run (the card path came with D-618). Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 3.7 GB of its memory held by the desktop and open programs); the same after it. The processor was 3% busy or less. The owner's app was not open. One run; each figure is the median of three draws.

Earlier figures (2026-10-13, at 6d242375, processor only, on a machine so busy they were marked not usable), frame 0: no effect 178.3; as added 138.9; 100 by 100 glowing 248.2; 200 by 100 by 5 326.6 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the card and the processor (D-618)

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with Particle Grid on its layers, each listed frame drawn whole at Full with empty caches (on the card, its kept pictures dropped first, so each layer is sent to the card again), three times; the median of the three, milliseconds.

| Shot | Drawn by | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---|---:|---:|---:|---:|
| no effect | card | 95.6 ms | 57.3 ms | 59.8 ms | 57.4 ms |
| no effect | processor | 33.8 ms | 39.6 ms | 35.0 ms | 35.8 ms |
| Particle Grid as added (1271 points) | card | 67.4 ms | 40.3 ms | 41.2 ms | 40.4 ms |
| Particle Grid as added (1271 points) | processor | 40.9 ms | 43.6 ms | 41.5 ms | 41.6 ms |
| 100 by 100 glowing points, rippling and drifting, added | card | 45.3 ms | 45.4 ms | 44.7 ms | 45.0 ms |
| 100 by 100 glowing points, rippling and drifting, added | processor | 51.2 ms | 45.6 ms | 45.8 ms | 49.6 ms |
| 200 by 100 by 5 points tipped, coloured and pushed by a layer | card | 80.9 ms | 80.1 ms | 59.1 ms | 55.8 ms |
| 200 by 100 by 5 points tipped, coloured and pushed by a layer | processor | 61.3 ms | 57.9 ms | 62.5 ms | 59.1 ms |

**Reading it.** On the processor, Particle Grid as added costs about 6 ms a frame over no effect (about 42 against 36), 100 by 100 glowing points about 10 to 14 ms (about 46 to 50), and 200 by 100 by 5 points tipped, coloured and pushed by a layer about 23 to 26 ms (about 58 to 63). On the card the shots with the effect take about 40 to 45 ms a frame, and the heaviest 56 to 81; the card is no quicker than the processor here, and slower for the heaviest shot, most likely because each frame is drawn from empty caches and every layer is sent to the card again. The card's "no effect" row (57 to 60 ms, 96 at frame 0) is slower than its rows with the effect; it is the first shot timed, so it most likely carries the card's first-use costs, but that is not proven, and the card's cost of the effect itself cannot be read from it. Unlike the busy run, the processor rows now read in order: more points cost more.
