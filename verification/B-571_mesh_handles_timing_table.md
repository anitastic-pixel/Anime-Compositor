# B-571: frame times with Mesh Warp's vertex handles (D-701)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b571_mesh_handles --no-run` and run with `--ignored b571_mesh_handles_timing`: once on the card, then once with `B571_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-13, on a machine that was not quiet), again: Noise alone card 21.9, processor 44.4; Noise, then Mesh Warp 2 by 2, the middle pushed and its handles turned card 32.1, processor 68.3 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Mesh Warp
as shown, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.2 | 41.8 |
| Noise, then Mesh Warp 2 by 2, the middle pushed and its handles turned | 19.7 | 63.7 |
| Noise, then Mesh Warp 7 by 7 at rest with four handles moved, Quality 8 | 24.8 | 72.4 |

First loops (empty caches): card 15.7, 22.4, 27.5; processor 41.3, 62.4, 72.1.

**Reading it.** The handles cost about 2.5 to 4.2 ms a 1080p layer on the card (7.5 and 12.6 ms over
three layers, 12.2 to 19.7 and 24.8) and about 7 to 10 ms a layer on the processor (41.8 to 63.7 and
72.4), much as the busy run had it (3 to 4 and 8 to 11), and the same as B-482's Mesh Warp without
handles. The mesh is worked on the processor either way; the card only draws it.
