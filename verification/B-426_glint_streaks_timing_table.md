# B-426: Glint Streaks, frame times

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b426_glint_streaks --no-run` and run once with `--ignored b426_glint_streaks_timing`, which times every row below in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after it. The owner's app was not open. One run, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone 18.1; Noise, then Cross Glare as added 40.1 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

The reference shot with a changing colour Noise on its first three layers, then Cross Glare on each, as added, with alternating ray colours, and with gradient colours and full shimmer. Every eighth frame of the reference shot, 30 in all, is asked for as the viewer asks, whole, on the graphics card's path at Full quality. The first loop starts with empty caches; Again is the median of seven more loops. Medians in ms.

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 17.0 | 13.5 |
| Noise, then Cross Glare as added | Full | 31.0 | 26.8 |
| Noise, then Cross Glare, ray colours alternate | Full | 30.3 | 26.7 |
| Noise, then Cross Glare, gradient, shimmer 100 | Full | 30.4 | 26.7 |

**Reading it.** Cross Glare adds about 13 ms a frame over the Noise alone (13.5 to 26.8, three
layers), about 4.4 ms a 1080p layer, whichever ray colours and shimmer are chosen (26.7 and 26.7):
alternating or gradient colours and shimmer cost nothing measurable.
