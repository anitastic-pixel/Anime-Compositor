# B-425: Lens Dirt, frame times

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b425_lens_dirt --no-run` and run once with `--ignored b425_lens_dirt_timing`, which times every row below in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after it. The owner's app was not open. One run, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone 20.9; Noise, then Soft Physical Glow (radius 40) 66.9 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

The reference shot with a changing colour Noise on its first three layers, then Soft Physical Glow or Lens Flare on each, without and with the fourth layer as its dirt. Every eighth frame of the reference shot, 30 in all, is asked for as the viewer asks, whole, on the graphics card's path at Full quality. The first loop starts with empty caches; Again is the median of seven more loops. Medians in ms.

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 17.6 | 13.4 |
| Noise, then Soft Physical Glow (radius 40) | Full | 59.7 | 56.3 |
| Noise, then Soft Physical Glow with the fourth layer as its dirt | Full | 85.8 | 82.3 |
| Noise, then Lens Flare | Full | 20.4 | 17.2 |
| Noise, then Lens Flare with the fourth layer as its dirt | Full | 44.2 | 42.6 |

**Reading it.** The dirt layer adds about 26 ms a frame to Soft Physical Glow (56.3 to 82.3, three
layers), about 9 ms a 1080p layer, and about 25 ms to Lens Flare (17.2 to 42.6). Lens Flare alone
adds under 4 ms over the Noise alone (13.4 to 17.2).
