# B-325: frame times with Turbulent Noise (D-445)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b325_turbulent_noise --no-run` and run with `--ignored b325_turbulent_noise_timing`: once on the card, then once with `B325_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (2% busy; 4.1 GB of its memory held by the desktop and open programs); the same after each. The processor was 1% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 16.6, processor 42.6; Noise, then Turbulent Noise as added (complexity 6) card 30.8, processor 80.5 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Turbulent Noise, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.6 | 42.6 |
| Noise, then Turbulent Noise as added (complexity 6) | 20.0 | 78.4 |
| Noise, then Turbulent Noise turbulent, size 40, complexity 20 (the most) | 30.5 | 135.2 |

First loops (empty caches): card 16.9, 24.5, 34.4; processor 41.8, 77.6, 134.5.

**Reading it.** On the card Turbulent Noise as added costs about 2.5 ms a 1080p layer (12.6 to
20.0, three layers), and at the most complexity about 6 ms (30.5); it is drawn by Fractal Noise's
pass (grade mode 9) and timed as Fractal Noise's stage. On the processor it costs about 12 and 31
ms a layer (78.4 and 135.2 against 42.6), so the card is about five times quicker. The busy run
could not say this.
