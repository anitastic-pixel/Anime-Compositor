# B-551: frame times with Drizzle (D-672)

**Measured again on a quiet machine** on 2026-10-14 at 90a33e9a, built with `cargo test --release --test b551_drizzle --no-run` and run with `--ignored b551_drizzle_timing`: once on the card, then once with `B551_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 4% busy; 3.8 GB of its memory held by the desktop and open programs); the same after each. The processor was 8% busy or less before each run. Before that the machine was busy (the processor 22 to 22% busy, from programs outside this work), and the runs waited up to 37 s for it to be quiet. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-13, on a machine that was not quiet), again: Noise alone card 14.0, processor 58.2; Noise, then Drizzle as added card 26.4, processor 96.5 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
Drizzle, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.7 | 41.9 |
| Noise, then Drizzle as added | 21.4 | 76.0 |
| Noise, then Drizzle, thirty drops a second, Rippling 720, Spreading 150, a point light | 24.5 | 94.7 |
| Noise, then Drizzle, drip rate keyed 0 to 40 (the settings read back to the in point) | 22.6 | 79.4 |

First loops (empty caches): card 17.0, 24.0, 27.1, 24.2; processor 41.5, 74.6, 92.5, 79.2.

**Reading it.** On the card, once warm, Drizzle as added costs about 9 ms a frame over the Noise alone (12.7 to 21.4, three layers, about 3 ms a 1080p layer), with drip rate keyed about 10 ms (22.6), and thirty drops a second with wide ripples and a point light about 12 ms (24.5): the cost grows with the drops alive at once and how far their ripples spread. On the processor the same three cost about 34, 38 and 53 ms (76.0, 79.4 and 94.7 against 41.9), so the card is about four times quicker. The busy run could not say that; the quiet run can.
