# B-553: frame times with Mercury (D-674)

**Measured again on a quiet machine** on 2026-10-14 at 90a33e9a, built with `cargo test --release --test b553_mercury --no-run` and run with `--ignored b553_mercury_timing`: once on the card, then once with `B553_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (2 and 4% busy; 3.8 GB of its memory held by the desktop and open programs); the same after each. The processor was 4% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-13, on a machine that was not quiet), again: Noise alone card 31.8, processor 48.8; Noise, then Mercury as added card 36.1, processor 70.0 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
Mercury, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.2 | 40.7 |
| Noise, then Mercury as added | 17.3 | 59.6 |
| Noise, then Mercury, forty big drops a second running together, a point light | 24.7 | 62.0 |
| Noise, then Mercury, birth rate keyed 0 to 40 (the settings read back to the in point) | 19.6 | 60.4 |

First loops (empty caches): card 15.6, 19.7, 26.7, 20.8; processor 40.7, 58.3, 61.8, 60.9.

**Reading it.** On the card, once warm, Mercury as added costs about 5 ms a frame over the Noise alone (12.2 to 17.3, three layers, under 2 ms a 1080p layer), with birth rate keyed about 7 ms (19.6), and forty big drops a second running together with a point light about 12 ms (24.7): the cost grows with the number of drops alive (birth rate times longevity) and their size. On the processor the same three cost about 19, 20 and 21 ms (59.6, 60.4 and 62.0 against 40.7), so the card is about two to four times quicker for what the effect adds. The busy run could not say that; the quiet run can.
