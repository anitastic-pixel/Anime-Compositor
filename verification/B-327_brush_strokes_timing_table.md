# B-327: frame times with Brush Strokes (D-447)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b327_brush_strokes --no-run` and run with `--ignored b327_brush_strokes_timing`: once on the card, then once with `B327_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 5% busy; 4.1 GB to 4.2 GB of its memory held by the desktop and open programs); the same after each. The processor was 4% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 250.9, processor 61.7; Noise, then Brush Strokes as added card 498.1, processor 222.9 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Brush Strokes, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.7 | 42.5 |
| Noise, then Brush Strokes as added | 60.9 | 181.7 |
| Noise, then Brush Strokes brush 6, length 30, density 4, randomness 2 (many strokes a pixel) | 292.5 | 921.8 |

First loops (empty caches): card 16.2, 64.0, 296.1; processor 45.2, 183.9, 921.6.

**Reading it.** On the card, Brush Strokes as added costs about 16 ms a 1080p layer (12.7 to 60.9,
three layers), and with many strokes a pixel (brush 6, length 30, density 4) about 93 ms a layer
(292.5). It is drawn by one pass of its own, `brush`, each pixel visiting the cells whose strokes
can reach it, so the cost grows with Brush Size, Stroke Length and Stroke Density. On the
processor the same shots cost about 46 and 293 ms a layer (181.7 and 921.8 against 42.5), so the
card is about three times quicker. The busy run could not say this; both costs are over Target
P3's 8 ms a layer.
