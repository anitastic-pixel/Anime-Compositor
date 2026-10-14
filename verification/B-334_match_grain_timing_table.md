# B-334: frame times with Match Grain (D-454)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b334_match_grain --no-run` and run with `--ignored b334_match_grain_timing`: once on the card, then once with `B334_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 2% busy; 4.2 GB of its memory held by the desktop and open programs); the same after each. The processor was 4% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 16.8, processor 42.4; as added (no noise source) card 16.8, processor 43.2 ms.

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
| Noise alone | 12.3 | 42.0 |
| as added (no noise source) | 12.4 | 42.0 |
| reading layer2 | 105.7 | 99.5 |
| reading layer2, size 3, softness 0.5, monochromatic | 102.7 | 97.6 |

First loops (empty caches): card 16.1, 16.2, 111.2, 82.0; processor 41.4, 41.6, 98.7, 97.2.

**Reading it.** As added, with no noise source, Match Grain costs nothing measurable. Reading layer2
draws layer2 whole (with its Noise) and measures its grain, on the processor, for each Match Grain;
the row asks Target P4, 100 ms or less for one effect on the processor, so this is within Target
P4: about 19 ms a layer on the processor (97.6 and 99.5 against 42.0), measuring included; on the
card about 30 to 31 ms a layer (102.7 and 105.7 against 12.3), the measure being on the processor.
That is the busy run's reading; the processor figure is a little lower (it was 23 to 24).
