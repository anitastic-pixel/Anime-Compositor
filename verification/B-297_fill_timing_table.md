# B-297: frame times with Fill (D-418)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b297_fill --no-run` and run with `--ignored b297_fill_timing`: once on the card, then once with `B297_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 4.2 GB of its memory held by the desktop and open programs); the same after each. The processor was 2% busy or less before each run. Before that the machine was busy (the card 12 to 24% busy, from programs outside this work), and the runs waited up to 476 s for it to be quiet. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 24.3, processor 44.6; Noise, then Fill as added (the whole layer) card 17.4, processor 55.2 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Fill, every eighth frame asked for whole at Full. Each layer has three masks of mode None
(circles of radius 400, 220 and 300). **Again** is the median of the loops after the first
(seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 13.0 | 43.1 |
| Noise, then Fill as added (the whole layer) | 14.9 | 44.8 |
| Noise, then Fill, mask 1, feathers 30 and 10, inverted, opacity 80 | 41.8 | 181.8 |
| Noise, then Fill, All Masks, feathers 60 | 78.1 | 296.4 |

First loops (empty caches): card 18.7, 19.0, 41.2, 80.6; processor 46.0, 49.5, 177.2, 361.3.

**Reading it.** Filling the whole layer costs well under 1 ms a 1080p layer on the card and on the
processor (14.9 against 13.0, 44.8 against 43.1, over three layers). With masks the covering is
made on the processor (each mask's 4 by 4 samples over the drawing grown by the feather's reach)
and sent to the card: about 10 ms a layer for one mask feathered 30 and 10, about 22 ms a layer
for three masks feathered 60, against about 46 and 84 on the processor alone. That is the busy
run's reading (7 and 20 against 40 and 90); the whole-layer case is now readable.
