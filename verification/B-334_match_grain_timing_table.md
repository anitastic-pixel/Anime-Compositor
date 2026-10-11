# B-334: frame times with Match Grain (D-454)

**PROVISIONAL.** Measured on 2026-10-10, built with `cargo test --release --test
b334_match_grain` from 28fd0d4c and c540033e (B-334's code), and run with `--ignored
b334_match_grain_timing`: once on the card, then once with `B334_CPU` set on the processor.
The machine was not quiet: the graphics card was 97% busy with other work while no cargo, rustc or test process was running. To be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Match Grain on each, every eighth frame asked for whole at Full. **Again** is the median
of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 16.8 | 42.4 |
| as added (no noise source) | 16.8 | 43.2 |
| reading layer2 | 111.8 | 115.8 |
| reading layer2, size 3, softness 0.5, monochromatic | 106.1 | 110.4 |

First loops (empty caches): card 22.0, 21.7, 86.0, 99.3; processor 41.7, 41.8, 114.5, 115.8.

**Reading it.** Reading layer2 draws layer2 whole (with its Noise) and measures its grain, on the
processor, for each Match Grain; the row asks Target P4, 100 ms or less for one effect on the processor, so
this is within Target P4 (100 ms or less on the processor): about 23 to 24 ms a layer on the processor, measuring included; on the card about 30 to 32 ms a layer, the measure being on the processor.
