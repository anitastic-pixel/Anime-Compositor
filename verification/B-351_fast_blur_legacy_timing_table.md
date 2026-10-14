# B-351: frame times with Fast Blur (Legacy) (D-471)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b351_fast_blur_legacy --no-run` and run with `--ignored b351_fast_blur_legacy_timing`: once on the card, then once with `B351_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 1% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 12.8, processor 41.6; Noise, then Fast Blur (Legacy) Blurriness 10 card 15.4, processor 75.7 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Fast Blur (Legacy) on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 13.6 | 44.4 |
| Noise, then Fast Blur (Legacy) Blurriness 10 | 15.9 | 71.8 |
| the same at Blurriness 100 | 24.2 | 98.0 |

First loops (empty caches): card 16.8, 19.3, 27.9; processor 43.5, 70.8, 100.2.

**Reading it.** On the card, Fast Blur (Legacy) at Blurriness 10 costs about 2.3 ms a frame over the
Noise alone (13.6 to 15.9, three layers), under 1 ms a 1080p layer, and at Blurriness 100 about
10.6 ms (24.2): Fast Box Blur's two passes, each pixel reading 2 x 3 ceil(0.37 B) + 1 neighbours a
pass. On the processor the same settings cost about 27 and 54 ms a frame (71.8 and 98.0 against
44.4). At these settings the card is quicker than the processor here, about four to five times.
