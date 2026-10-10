# B-333: frame times with Dust & Scratches (D-453)

**PROVISIONAL.** Measured on 2026-10-10, built with `cargo test --release --test
b333_dust_scratches` from ace9c136 (B-333's code), and run with `--ignored
b333_dust_scratches_timing`: once on the card, then once with `B333_CPU` set on the processor.
The machine was not quiet: the card was fully used by other lanes' tests the whole time (lane A's Curl Noise test alone held 4.8 GB of it, the card at 96 to 100 per cent before and after, 15 GB of its 16.8 in use), and earlier waits found no quiet minute. To be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Dust & Scratches on each, every eighth frame asked for whole at Full. **Again** is the median
of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 201.1 | 49.9 |
| as added (Radius 1, Threshold 0) | 656.9 | 92.9 |
| Radius 3, Threshold 20 | 604.6 | 144.4 |
| Radius 5, Threshold 8, Operate on Alpha on | 975.1 | 241.1 |

First loops (empty caches): card 264.6, 740.6, 704.3, 750.3; processor 47.1, 95.5, 146.8, 242.5.

**Reading it.** No verdict against Target P2 (4 ms or less a layer): the card's times measure the other tests sharing it, not the effect (Noise alone 201.1 ms on the card against 12.9 in B-331's quiet run); on the processor three layers add 43.0 to 191.2 ms a frame.
