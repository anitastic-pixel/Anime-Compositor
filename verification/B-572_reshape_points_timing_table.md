# B-572: frame times with Reshape's Correspondence Points and Interpolation (D-702)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b572_reshape_points --no-run` and run with `--ignored b572_reshape_points_timing`: once on the card, then once with `B572_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB to 3.2 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-13, on a machine that was not quiet), again: Noise alone card 23.9, processor 92.9; Noise, then Reshape with pairs (0, 0) and (50, 25) card 56.7, processor 155.8 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 frames a second, 240 frames) with B-485's
three masks on each of its first three layers. On each of those layers is a Noise that changes
every frame (amount 12, colour, seed 7), then a Reshape from mask 1 to mask 2 as listed below.
Every eighth frame is drawn whole at Full. **Again** is the median of the loops after the first
(seven on the card, two on the processor), in milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.5 | 45.1 |
| Noise, then Reshape with pairs (0, 0) and (50, 25) | 41.7 | 87.7 |
| Noise, then Reshape, Percent keyed 20 to 100 and back to 40, Smooth | 121.7 | 134.1 |

First loops (empty caches): card 16.2, 42.3, 113.7; processor 44.5, 81.0, 140.7.

**Reading it.**

- **With pairs.** The cost is in line with B-485's plain Reshape: about 9.7 ms per 1080p layer on
  the card (12.5 to 41.7 over three layers) and about 14 ms on the processor (45.1 to 87.7).
  Matching points by pairs adds almost nothing.
- **Smooth.** Between keys, Smooth works out four pulls and blends them. On the card a frame there
  costs about 36 ms per layer (121.7), close to four plain ones; on the processor about 30 ms per
  layer (134.1), about twice a plain one. The busy run gave about 50 and 60; the reading is the
  same, the figures lower.
- **Where the time goes.** On both, the pulls are worked out on the processor; the card only
  draws them. A faster way would blend the matched points before working out one pull, but that
  would be a different rule from the one the fixtures hold. It is listed as a gap, not built.
