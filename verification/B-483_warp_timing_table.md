# B-483: frame times with Warp (D-604)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b483_warp --no-run` and run with `--ignored b483_warp_timing`: once on the card, then once with `B483_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 2% busy; 3.0 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-12, on a machine that was not quiet), again: Noise alone card 18.0, processor 100.8; Noise, then Warp as added (Arc, Bend 50) card 27.0, processor 192.1 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Warp
as shown, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.2 | 42.2 |
| Noise, then Warp as added (Arc, Bend 50) | 23.1 | 70.3 |
| Noise, then Warp Twist 80 with both distortions 30 | 23.7 | 71.7 |

First loops (empty caches): card 16.5, 25.4, 25.1; processor 41.4, 69.6, 70.4.

**Reading it.** On the card the Arc adds about 10.9 ms a frame over three layers (12.2 to 23.1),
about 3.6 ms a 1080p layer, and the Twist with both distortions about the same, 11.5 ms (23.7),
3.8 ms a layer: the busy run's 13 ms a layer for the Twist was the machine, not the shape. Both are
drawn by D-601's forward mesh, warp mode 150, here 64 by 64, as Bezier Warp is. On the processor
Warp adds about 28 to 30 ms a frame (42.2 to 70.3 and 71.7), about 9.5 ms a 1080p layer, a third
of the busy run's 30.
