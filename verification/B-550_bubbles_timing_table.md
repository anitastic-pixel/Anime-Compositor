# B-550: frame times with Bubbles (D-671)

**Measured again on a quiet machine** on 2026-10-14 at 90a33e9a, built with `cargo test --release --test b550_bubbles --no-run` and run with `--ignored b550_bubbles_timing`: once on the card, then once with `B550_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (2 and 5% busy; 3.8 GB to 3.9 GB of its memory held by the desktop and open programs); the same after each. The processor was 3% busy or less before each run. Before that the machine was busy (the card 11 to 25% busy, from programs outside this work), and the runs waited up to 475 s for it to be quiet. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-13, on a machine that was not quiet), again: Noise alone card 16.7, processor 52.6; Noise, then Bubbles as added card 23.6, processor 82.1 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
Bubbles, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 13.1 | 43.1 |
| Noise, then Bubbles as added | 17.1 | 65.0 |
| Noise, then Bubbles, 1000 of size 3, World, Lighten | 56.4 | 254.9 |
| Noise, then Bubbles, speed keyed 0 to 3 (the settings read back to the in point) | 18.3 | 64.0 |

First loops (empty caches): card 17.2, 18.8, 58.7, 19.8; processor 41.8, 63.6, 236.7, 65.3.

**Reading it.** On the card, once warm, Bubbles as added costs about 4 ms a frame over the Noise alone (13.1 to 17.1, three layers, about 1.3 ms a 1080p layer), with speed keyed about 5 ms (18.3), and 1000 bubbles of size 3 in World with Lighten about 43 ms (56.4): the cost grows with the number of bubbles and their size. On the processor the same three cost about 22, 21 and 212 ms (65.0, 64.0 and 254.9 against 43.1), so the card is about four to five times quicker. The busy run could not say that; the quiet run can.
