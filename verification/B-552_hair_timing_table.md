# B-552: frame times with Hair (D-673)

**Measured again on a quiet machine** on 2026-10-14 at 90a33e9a, built with `cargo test --release --test b552_hair --no-run` and run with `--ignored b552_hair_timing`: once on the card, then once with `B552_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (6 and 7% busy; 3.8 GB of its memory held by the desktop and open programs); the same after each. The processor was 6% busy or less before each run. Before that the machine was busy (the card 18% busy, from programs outside this work), and the runs waited up to 37 s for it to be quiet. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-13, on a machine that was not quiet), again: Noise alone card 53.7, processor 54.1; Noise, then Hair as added card 290.7, processor 518.2 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
Hair, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.5 | 43.8 |
| Noise, then Hair as added | 220.0 | 226.8 |
| Noise, then Hair, a long fur: length 200, thickness 6, density 60 | 2828.0 | 2371.7 |
| Noise, then Hair, weight keyed -100 to 200, constant mass | 262.3 | 263.4 |

First loops (empty caches): card 16.5, 224.6, 2821.6, 287.0; processor 41.2, 229.3, 2306.6, 264.2.

**Reading it.** Hair as added costs about 208 ms a frame over the Noise alone on the card (12.5 to 220.0, three layers, about 69 ms a 1080p layer) and about 183 on the processor (43.8 to 226.8); with weight keyed about 250 and 220 (262.3 and 263.4); and a long fur (length 200, thickness 6, density 60) about 2.8 s and 2.3 s a frame (2828.0 and 2371.7). The cost grows with the number of strands, their length and their thickness. The card is no quicker than the processor here: the whole frame takes about the same either way, and for the long fur the processor is a little quicker. Every setting is far slower than real time (41.7 ms at 24 a second). The busy run could not say any of this; the quiet run can.
