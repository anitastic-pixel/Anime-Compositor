# B-335: frame times with Remove Grain (D-455)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b335_remove_grain --no-run` and run with `--ignored b335_remove_grain_timing`: once on the card, then once with `B335_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 4.2 GB of its memory held by the desktop and open programs); the same after each. The processor was 2% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 40.1, processor 52.0; as added card 196.0, processor 183.9 ms.

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
| Noise alone | 12.3 | 41.4 |
| as added | 72.3 | 138.9 |
| Noise Reduction 2, three passes, single channel | 74.6 | 433.1 |
| as added with Unsharp Mask 150 | 74.8 | 190.8 |

First loops (empty caches): card 15.7, 63.1, 77.4, 76.1; processor 40.6, 138.2, 432.4, 190.1.

**Reading it.** Each Remove Grain draws its layer again up to the effect (with its Noise) and
measures its grain on the processor before the card's passes; the row asks Target P3, 8 ms or less
added on the card, so this is over Target P3: about 20 to 21 ms a layer on the card at every
setting measured (72.3 to 74.8 against 12.3), the measure on the processor included. On the
processor alone about 32 ms a layer as added, 50 with Unsharp Mask and 131 with three passes of
Noise Reduction (138.9, 190.8 and 433.1 against 41.4). The busy run's 52 to 132 ms a layer on the
card were the machine; quiet, the card is about two to six times quicker than the processor.
