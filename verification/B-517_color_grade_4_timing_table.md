# B-517: frame times with Color Grade's fourth unit (D-637)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b517_color_grade_4 --no-run` and run with `--ignored b517_color_grade_4_timing`: once on the card, then once with `B517_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-13, on a machine that was not quiet), again: Noise alone card 34.2, processor 124.5; Noise, then Color Grade: a dark vignette at roundness -60 card 42.9, processor 473.4 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Color Grade on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 46.4 |
| Noise, then Color Grade: a dark vignette at roundness -60 | 15.5 | 54.5 |
| Noise, then Color Grade: sharpen 50 (drawn by the processor either way) | 97.1 | 103.5 |

First loops (empty caches): card 16.4, 19.8, 88.8; processor 42.7, 54.7, 98.6.

**Reading it.** The squarer vignette adds 3.2 ms a frame over the Noise alone on the card (12.3 to 15.5, three layers), about 1.1 ms a 1080p layer, and about 2.7 ms a layer on the processor (46.4 to 54.5). A sharpened grade is drawn by the processor, so with the card on it costs a trip to the processor and back: 97.1 ms a frame on the card's path against 103.5 on the processor's own, about 28 and 19 ms a layer over the Noise alone. This changes the busy run's reading: the vignette costs about a third of the 3 ms a layer it gave, and the processor's odd vignette figure (473.4) is gone (54.5), so it was the machine.
