# B-422: frame times with Soft Physical Glow's tint

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b422_glow_tint --no-run` and run once with `--ignored b422_glow_tint_timing`, which times every row below in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (3% busy; 2.9 GB of its memory held by the desktop and open programs); the same after it. The owner's app was not open. One run, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone 15.7; Noise, then Soft Physical Glow, radius 60, threshold 40 smooth 50, no tint 55.6 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

Each shot is the reference shot with animated colour noise on its three layers, then the glow named, if any. Every eighth frame of the reference shot, 30 in all, is asked for as the viewer asks, whole, on the graphics card's path at Full quality. The first loop starts with empty caches; Again is the median of seven more loops. Medians in ms.

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 16.6 | 13.6 |
| Noise, then Soft Physical Glow, radius 60, threshold 40 smooth 50, no tint | Full | 54.8 | 50.7 |
| Noise, then the same, Tint Core to Edge | Full | 54.3 | 51.1 |
| Noise, then Soft Physical Glow as added, radius 500, Tint Core to Edge | Full | 67.5 | 66.0 |

**Reading it.** Soft Physical Glow at radius 60 adds about 37 ms a frame over the Noise alone
(13.6 to 50.7, three layers), about 12 ms a 1080p layer. Tint Core to Edge adds nothing measurable
on top (51.1 against 50.7). The busy run's 25 ms for the tint was the machine. As added, radius
500 with the tint, the frame takes 66.0 ms.
