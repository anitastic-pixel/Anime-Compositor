# B-463: frame times with Contour Paint, Arrowheads (D-585)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b463_contour_heads --no-run` and run with `--ignored b463_contour_heads_timing`: once on the card, then once with `B463_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (1% busy; 3.7 GB of its memory held by the desktop and open programs); the same after each. The processor was 2% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-12, on a machine that was not quiet), again: Noise alone card 21.0, processor 69.1; Noise, then Contour Paint, View Edge Map, Arrowheads off card 32.4, processor 175.0 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Contour Paint on View Edge Map on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.2 | 41.0 |
| Noise, then Contour Paint, View Edge Map, Arrowheads off | 20.7 | 119.6 |
| the same with Arrowheads on | 21.2 | 121.4 |

First loops (empty caches): card 16.0, 25.0, 24.6; processor 40.7, 120.4, 120.5.

**Reading it.** On the card, once warm, Contour Paint's View Edge Map adds about 8.5 ms a frame over the Noise alone (12.2 to 20.7, three layers, about 3 ms a 1080p layer), and Arrowheads on adds about half a millisecond more (21.2): the heads are drawn in the same pass. On the processor the two cost 119.6 and 121.4 against 41.0, about 79 to 80 ms added, so the card is about nine times quicker for what the effect adds. The busy run could not say that; the quiet run can.
