# B-338: frame times with CC Simple Wire Removal (D-458)

Measured on 2026-10-10, built with `cargo test --release --test
b338_simple_wire_removal` from 885f5a2e (B-338's code), and run with `--ignored
b338_simple_wire_removal_timing`: once on the card, then once with `B338_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the CC Simple
Wire Removal, every eighth frame asked for whole at Full. **Again** is the median of the
loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.4 | 40.4 |
| Noise, then Displace, thickness 20 | 16.4 | 56.1 |
| Noise, then Frame Offset -1, thickness 20 | 52.0 | 77.8 |

First loops (empty caches): card 15.8, 20.1, 54.4; processor 39.4, 56.8, 77.6.

**Reading it.** The row asks Target P2, 4 ms or less for a neighbourhood effect: Displace about 1.3 ms a 1080p layer on the card (5.2 on the processor), within Target P2 (4 ms or less); Frame Offset about 13.2 on the card (12.5 on the processor), over it, because the layer at the other frame, with the effects before this one, is drawn again by the processor and sent to the card every frame (Color Stabilizer's way).
