# B-424: Halation, frame times

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b424_halation --no-run` and run once with `--ignored b424_halation_timing`, which times every row below in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after it. The owner's app was not open. One run, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone 61.3; Noise, then Halation as added (radius 25) 87.7 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

The reference shot with a changing colour Noise on its first three layers, then Halation on each. Every eighth frame of the reference shot, 30 in all, is asked for as the viewer asks, whole, on the graphics card's path at Full quality. The first loop starts with empty caches; Again is the median of seven more loops. Medians in ms.

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 17.1 | 14.5 |
| Noise, then Halation as added (radius 25) | Full | 51.8 | 48.4 |
| Noise, then Halation, radius 200, amount 200 | Full | 56.6 | 53.0 |

**Reading it.** Halation as added (radius 25) adds about 34 ms a frame over the Noise alone (14.5
to 48.4, three layers), about 11 ms a 1080p layer, and at radius 200, amount 200 about 39 ms (53.0):
the radius barely changes the cost.
