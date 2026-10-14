# B-482: frame times with Mesh Warp (D-603)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b482_mesh_warp --no-run` and run with `--ignored b482_mesh_warp_timing`: once on the card, then once with `B482_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 4% busy; 2.9 GB to 3.0 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-12, on a machine that was not quiet), again: Noise alone card 50.0, processor 99.3; Noise, then Mesh Warp 2 by 2 with the middle pushed card 46.1, processor 159.4 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Mesh
Warp at Quality 8, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.4 | 42.4 |
| Noise, then Mesh Warp 2 by 2 with the middle pushed | 20.0 | 63.5 |
| Noise, then Mesh Warp 7 by 7 with three vertices moved, Quality 8 | 25.0 | 72.4 |

First loops (empty caches): card 16.0, 22.4, 25.8; processor 41.8, 63.1, 71.7.

**Reading it.** On the card, Mesh Warp (warp mode 150, as Bezier Warp) adds about 7.6 ms a frame
over three layers at 2 by 2 (12.4 to 20.0) and 12.6 ms at 7 by 7, Quality 8 (25.0), about 2.5 and
4.2 ms a 1080p layer. The busy run could not read the card at all (its Noise alone was slower than
either Mesh Warp shot); the quiet one can. On the processor it adds about 21 and 30 ms a frame
(42.4 to 63.5 and 72.4), about 7 to 10 ms a layer, a half or less of the busy run's 20.
