# B-328: frame times with Cartoon (D-448)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b328_cartoon --no-run` and run with `--ignored b328_cartoon_timing`: once on the card, then once with `B328_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 2% busy; 4.0 GB to 4.1 GB of its memory held by the desktop and open programs); the same after each. The processor was 3% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 12.6, processor 44.8; Noise, then Cartoon as added card 48.1, processor 665.1 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Cartoon, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.6 | 42.3 |
| Noise, then Cartoon as added | 47.6 | 538.4 |
| Noise, then Cartoon detail radius 30, edge width 8, enhancement 50 (the widest reach) | 293.2 | 5832.0 |

First loops (empty caches): card 17.2, 50.9, 296.0; processor 42.1, 538.4, 5606.0.

**Reading it.** On the card, Cartoon as added costs about 12 ms a 1080p layer (12.6 to 47.6, three
layers), and at the widest reach (detail radius 30, edge width 8) about 94 ms a layer (293.2). It
is drawn by Bilateral Blur's two passes and two of its own, `cartoonedge` and `cartoon`; the cost
grows with Detail Radius (the smoothing) and Edge Width (the line). On the processor the same
shots cost about 165 and 1930 ms a layer (538.4 and 5832.0 against 42.3), so the card is about 14
to 20 times quicker. The busy run could not say this.
