# B-555: frame times with Scatterize (D-676)

**Measured again on a quiet machine** on 2026-10-14 at 90a33e9a, built with `cargo test --release --test b555_scatterize --no-run` and run with `--ignored b555_scatterize_timing`: once on the card, then once with `B555_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 7% busy; 3.8 GB to 4.0 GB of its memory held by the desktop and open programs); after a run no other build or test was running, but the card was 13% busy with another program (the timing test had ended). The processor was 6% busy or less before each run. Before that the machine was busy (the card 10 to 23% busy, from programs outside this work), and the runs waited up to 891 s for it to be quiet. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-13, on a machine that was not quiet), again: Noise alone card 16.3, processor 51.5; Noise, then Scatterize as added card 584.3, processor 620.9 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
Scatterize, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 13.1 | 43.4 |
| Noise, then Scatterize as added | 519.1 | 372.5 |
| Noise, then Scatterize, Amount 300, the right edge edge-on | 578.1 | 591.8 |
| Noise, then Scatterize, Amount keyed 0 to 600, Add | 504.5 | 713.9 |

First loops (empty caches): card 16.6, 533.7, 576.1, 515.9; processor 43.0, 373.7, 554.6, 655.9.

**Reading it.** Scatterize is costly either way: as added it adds about 506 ms a frame over the Noise alone on the card (13.1 to 519.1, three layers, about 170 ms a 1080p layer) and about 329 on the processor (43.4 to 372.5); with Amount 300 and the right edge edge-on, about 565 and 548 (578.1 and 591.8); with Amount keyed and Add, about 491 and 671 (504.5 and 713.9). So the processor is quicker as added, the two are about equal at Amount 300, and the card is quicker with Add. The cost grows with the layer's size (one dot per pixel), and every setting is far slower than real time (41.7 ms at 24 a second). The busy run could not say any of this; the quiet run can.
