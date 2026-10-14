# B-410: frame times with CC Cylinder (D-530)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b410_cc_cylinder --no-run` and run with `--ignored b410_cc_cylinder_timing`: once on the card, then once with `B410_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 3% busy; 3.6 GB to 3.7 GB of its memory held by the desktop and open programs); the same after each. The processor was 4% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again, on the card: Noise alone 87.4; Noise, then CC Cylinder as added 95.7; Noise, then CC Cylinder, render inside, tipped 111.0; Noise, then CC Cylinder, radius 100, on its side 70.2. On the processor: Noise alone 169.7; Noise, then CC Cylinder as added 170.7; Noise, then CC Cylinder, render inside, tipped 143.4; Noise, then CC Cylinder, radius 100, on its side 242.5 ms.

## On the card

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the card
- Loops: 8

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 17.0 | 12.4 |
| Noise, then CC Cylinder as added | Full | 25.5 | 21.4 |
| Noise, then CC Cylinder, render inside, tipped | Full | 21.7 | 18.0 |
| Noise, then CC Cylinder, radius 100, on its side | Full | 27.8 | 24.5 |

## On the processor

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor
- Loops: 3

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 40.1 | 40.6 |
| Noise, then CC Cylinder as added | Full | 77.1 | 77.1 |
| Noise, then CC Cylinder, render inside, tipped | Full | 63.5 | 64.4 |
| Noise, then CC Cylinder, radius 100, on its side | Full | 85.1 | 85.0 |

CC Cylinder is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise that changes every frame on its first three layers, so no finished frame is kept, then CC Cylinder on them; every eighth frame of 240, at Full, each frame asked for whole as the viewer asks. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the loops after it. Milliseconds a frame.

**Reading it.** On the card, once warm, CC Cylinder adds about 9 ms a frame over Noise alone as added (21.4 against 12.4, three layers, about 3 ms a 1080p layer), about 6 ms tipped and drawn inside (18.0), and about 12 ms at radius 100 on its side (24.5). On the processor it adds about 37, 24 and 44 ms a frame (77.1, 64.4 and 85.0 against 40.6), so the card is about four times quicker. The busy run could say only "about 8 ms on the card"; that holds, and the processor's figures, which swung with no order then, can now be read.
