# B-412: frame times with CC Sphere (D-532)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b412_cc_sphere --no-run` and run with `--ignored b412_cc_sphere_timing`: once on the card, then once with `B412_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 3.6 GB of its memory held by the desktop and open programs); the same after each. The processor was 3% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again, on the card: Noise alone 29.9; Noise, then CC Sphere as added 34.7; Noise, then CC Sphere, render inside, tipped 43.8; Noise, then CC Sphere, radius 100, on its side 47.7. On the processor: Noise alone 87.1; Noise, then CC Sphere as added 120.0; Noise, then CC Sphere, render inside, tipped 103.1; Noise, then CC Sphere, radius 100, on its side 216.1 ms.

## On the card

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the card
- Loops: 8

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 15.9 | 12.2 |
| Noise, then CC Sphere as added | Full | 25.4 | 21.1 |
| Noise, then CC Sphere, render inside, tipped | Full | 21.0 | 18.1 |
| Noise, then CC Sphere, radius 100, on its side | Full | 35.5 | 32.2 |

## On the processor

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor
- Loops: 3

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 40.7 | 40.8 |
| Noise, then CC Sphere as added | Full | 74.0 | 74.9 |
| Noise, then CC Sphere, render inside, tipped | Full | 65.0 | 65.6 |
| Noise, then CC Sphere, radius 100, on its side | Full | 104.2 | 104.4 |

CC Sphere is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise that changes every frame on its first three layers, so no finished frame is kept, then CC Sphere on them; every eighth frame of 240, at Full, each frame asked for whole as the viewer asks. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the loops after it. Milliseconds a frame.

**Reading it.** On the card, once warm, CC Sphere as added costs about 9 ms a frame over Noise alone (21.1 against 12.2, three layers, about 3 ms a 1080p layer), tipped and drawn inside about 6 ms (18.1), and radius 100 on its side, the sphere filling most of the frame, about 20 ms (32.2). On the processor it costs about 34, 25 and 64 ms a frame (74.9, 65.6 and 104.4 against 40.8), so the card is about three to four times quicker. The busy run had the card at about 5, 14 and 18 ms; the order of the first two was the machine (the tipped one, drawn inside, is the cheaper), and its processor figures could not be read.
