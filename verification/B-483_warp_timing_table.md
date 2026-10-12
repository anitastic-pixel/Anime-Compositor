# B-483: frame times with Warp (D-604)

Measured on 2026-10-12, built with `cargo test --release --test b483_warp` from 7bc68a5a
(B-483's code), and run with `--ignored b483_warp_timing`: once on the card, then once with
`B483_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Other
lanes were building and testing beside it, and before the runs the graphics card had 6.6 GB of its
memory in use by other programs. Treat them as an upper bound, to be measured again on a quiet machine.

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
| Noise alone | 18.0 | 100.8 |
| Noise, then Warp as added (Arc, Bend 50) | 27.0 | 192.1 |
| Noise, then Warp Twist 80 with both distortions 30 | 56.5 | 189.5 |

First loops (empty caches): card 25.5, 40.0, 95.8; processor 54.4, 197.0, 185.3.

**Reading it.** On the card the Arc added about 9 ms a frame over three layers, about 3 ms a 1080p
layer, as Bezier Warp did (both are drawn by D-601's forward mesh, warp mode 150, here 64 by 64).
The Twist with both distortions measured about 13 ms a layer; with the card shared by other
programs this run cannot say whether that is the shape or the machine. On the processor Warp added
about 90 ms a frame over three layers, about 30 ms a 1080p layer, much the same for both shots;
the Noise alone swung from 54.4 to 100.8 between loops.
