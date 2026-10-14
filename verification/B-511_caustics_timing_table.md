# B-511: frame times with Caustics (D-631)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b511_caustics --no-run` and run once with `--ignored b511_caustics_timing`. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 3.7 GB of its memory held by the desktop and open programs); the same after it. The processor was 4% busy before the run and 1% after. The owner's app was not open. One run; each figure is the median of three draws. Machine: AMD Ryzen 9 9900X (24 threads), NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88), Windows 11, release build.

Earlier provisional figures (2026-10-12, at de4daf9d, on a machine with more than thirty build processes, two runs), frame 0: no effect 684.8 and 56.8; as added 261.0 and 245.9; water surface, caustics 1 1138.3 and 1865.6; with Bottom Blur 50 2909.8 and 2848.2 ms.

- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor (Caustics has no card path)

**What is timed:** the reference shot (1920 by 1080, 24 a second) with Caustics on its first layer: as added (no water surface), with the layer as its own water surface and Caustics Strength 1, and the same with Bottom Blur 50 (the blur's cost ceiling); frames 0 and 120, each drawn whole on the processor at Full with empty caches, three times; the median of the three, in milliseconds.

| Shot | Frame 0 | Frame 120 |
|---|---:|---:|
| no effect | 36.4 ms | 34.8 ms |
| Caustics as added (no water surface) | 115.3 ms | 112.3 ms |
| Caustics, the layer its own water surface, caustics 1 | 137.3 ms | 143.3 ms |
| the same with Bottom Blur 50 (the ceiling) | 415.1 ms | 396.6 ms |

**Reading it.** On a quiet machine Caustics as added (no water surface) adds about 78 ms a frame over no effect (115.3 and 112.3 against 36.4 and 34.8), with the layer as its own water surface and caustics about 100 to 110 ms (137.3 and 143.3), and with Bottom Blur 50 as well about 360 to 380 ms (415.1 and 396.6). Frame 120 costs no more than frame 0: Caustics carries nothing from frame to frame, so a late frame does not run longer for being late. The busy run's 0.25 to 2.9 s a frame were the other builds; quiet, it is a few times less. The blur's direct taps are the logged ceiling (D-631), and at about 250 to 280 ms of the frame they are most of Caustics' cost when the blur is on; the upgrade, a running box or the FFT path, waits until a timing asks for it.
