# B-335: frame times with Remove Grain (D-455)

**PROVISIONAL.** Measured on 2026-10-10, built with `cargo test --release --test
b335_remove_grain` from 3badaee1 (B-335's code), and run with `--ignored
b335_remove_grain_timing`: once on the card, then once with `B335_CPU` set on the processor.
The machine was not quiet: the card was at 96 to 100 per cent with other work and lane A's b327 test was running. To be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Remove Grain on each, every eighth frame asked for whole at Full. **Again** is the median
of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 40.1 | 52.0 |
| as added | 196.0 | 183.9 |
| Noise Reduction 2, three passes, single channel | 325.8 | 480.8 |
| as added with Unsharp Mask 150 | 436.9 | 217.9 |

First loops (empty caches): card 24.5, 142.2, 162.3, 571.9; processor 51.3, 182.5, 503.0, 217.3.

**Reading it.** Each Remove Grain draws its layer again up to the effect (with its Noise) and
measures its grain on the processor before the card's passes; the row asks Target P3, 8 ms or less
added on the card, so this is over Target P3 (8 ms or less added on the card): about 52 to 132 ms a layer on the card, the measure (the layer drawn again up to the effect, then summed) on the processor included; on the processor about 44 to 143 ms a layer.
