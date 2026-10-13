# B-572: frame times with Reshape's Correspondence Points and Interpolation (D-702)

Measured on 2026-10-13. Built with `cargo test --release --test b572_reshape_points` from
4c844e75 (B-572's code) and run with `--ignored b572_reshape_points_timing`: once on the card,
then once on the processor with `B572_CPU` set.

**These numbers are PROVISIONAL.** The machine was not quiet: other lanes were building and
testing alongside these runs. The Noise alone took 92.9 ms on the processor here, against 44.4 in
B-571's run the same day. Treat these numbers as an upper bound and measure again on a quiet
machine.

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
| Noise alone | 23.9 | 92.9 |
| Noise, then Reshape with pairs (0, 0) and (50, 25) | 56.7 | 155.8 |
| Noise, then Reshape, Percent keyed 20 to 100 and back to 40, Smooth | 179.3 | 274.6 |

First loops (empty caches): card 27.7, 75.6 and 135.5; processor 107.7, 181.2 and 286.4.

**Reading it.**

- **With pairs.** The cost is in line with B-485's plain Reshape: about 11 ms per 1080p layer on the
  card and about 20 ms on the processor. Matching points by pairs adds almost nothing.
- **Smooth.** Between keys, Smooth works out four pulls and blends them, so a frame there costs
  about four plain ones. On the card that comes to about 50 ms per layer; on the processor, about
  60 ms per layer.
- **Where the time goes.** On both, the pulls are worked out on the processor; the card only
  draws them. A faster way would blend the matched points before working out one pull, but that
  would be a different rule from the one the fixtures hold. It is listed as a gap, not built.
