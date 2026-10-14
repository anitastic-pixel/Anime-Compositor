# B-423: frame times with Soft Physical Glow's Spectral

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b423_spectral_glow --no-run` and run once with `--ignored b423_spectral_glow_timing`, which times every row below in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (5% busy; 2.9 GB of its memory held by the desktop and open programs); the same after it. The owner's app was not open. One run, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone 51.2; Noise, then Soft Physical Glow, radius 60, threshold 40 smooth 50, Spectral off 141.4 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

Each shot is the reference shot with animated colour noise on its three layers, then the glow named, if any. Every eighth frame of the reference shot, 30 in all, is asked for as the viewer asks, whole, on the graphics card's path at Full quality. The first loop starts with empty caches; Again is the median of seven more loops. Medians in ms.

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 16.5 | 13.4 |
| Noise, then Soft Physical Glow, radius 60, threshold 40 smooth 50, Spectral off | Full | 53.9 | 50.7 |
| Noise, then the same, Spectral on (80, 100, 120) | Full | 121.5 | 118.3 |
| Noise, then Soft Physical Glow as added, radius 500, Spectral on (80, 100, 120) | Full | 171.3 | 168.8 |

**Reading it.** Soft Physical Glow at radius 60 adds about 37 ms a frame over the Noise alone
(13.4 to 50.7, three layers). Spectral on adds about 68 ms more (118.3), about 23 ms a 1080p
layer; as added, radius 500 with Spectral on, the frame takes 168.8 ms. The busy run's 802 ms for
that last shot was the machine, not the effect.
