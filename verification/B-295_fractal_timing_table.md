# B-295: frame times with Fractal (D-416)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b295_fractal --no-run` and run with `--ignored b295_fractal_timing`: once on the card, then once with `B295_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 4.0 GB to 4.2 GB of its memory held by the desktop and open programs); after a run no other build or test was running, but the card was 11% busy with another program (the timing test had ended). The processor was 4% busy or less before each run. Before that the machine was busy (the card 11 to 21% busy, from programs outside this work), and the runs waited up to 189 s for it to be quiet. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 28.2, processor 59.8; Noise, then Fractal as added (Escape Limit 100, Edge Detect 2 by 2) card 206.3, processor 82.1 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Fractal, every eighth frame asked for whole at Full. **Again** is the median of the loops
after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 14.2 | 50.1 |
| Noise, then Fractal as added (Escape Limit 100, Edge Detect 2 by 2) | 39.7 | 69.0 |
| Noise, then Fractal, Julia, Hue Wheel | 40.9 | 68.9 |
| Noise, then Fractal, Brute Force 4 by 4 | 131.5 | 254.5 |
| Noise, then Fractal, magnification 20 at the seahorse valley, Escape Limit 1000 | 1209.1 | 1235.8 |

First loops (empty caches): card 23.5, 42.7, 43.8, 139.1, 1232.3; processor 47.6, 72.0, 70.0, 249.5, 1238.3.3,
307.2, 1473.7.

**Reading it.** A Fractal as added costs about 8.5 ms a 1080p layer on the card (14.2 to 39.7, over
three layers) and about 6 to 8 ms on the processor (69.0 against a Noise alone of 50.1 on this
run, which came out higher than the 42 to 46 the other quiet runs give; against 45 it is about 8).
The Julia set with Hue Wheel costs the same. Brute Force 4 by 4 costs about 39 ms a layer on the
card and about 70 on the processor, and the deep zoom at Escape Limit 1000 about 400 ms a layer on
either. This changes the busy run's reading in one place: the card is no longer slower than the
processor (its 15 ms a layer was the machine), and with Brute Force it is about twice as quick.
At deep zooms the two cost the same: the card works every step in the processor's own
double-precision rounding so the escape counts match exactly, and its double-precision speed is a
small share of its single-precision speed. A faster single-precision path would change pixels
past 1 level at deep zooms, so it is not taken here.
