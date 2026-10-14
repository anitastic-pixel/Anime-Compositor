# B-446: frame times with Turbulent Noise's full type lists (D-567)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b446_turbulent_noise_types --no-run` and run with `--ignored b446_turbulent_noise_types_timing`: once on the card, then once with `B446_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 2% busy; 3.6 GB of its memory held by the desktop and open programs); the same after each. The processor was 2% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-12, on a machine that was not quiet), again: Noise alone card 24.1, processor 81.1; Noise, then Turbulent Noise as added (basic, smooth, complexity 6) card 28.9, processor 138.1 ms.

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
| Noise alone | 12.3 | 41.0 |
| Noise, then Turbulent Noise as added (basic, smooth, complexity 6) | 19.7 | 80.5 |
| Noise, then Turbulent Noise dynamic_twist on soft_linear, complexity 6 | 21.1 | 89.6 |
| Noise, then Turbulent Noise dynamic_twist on spline, size 40, complexity 20 (the most) | 60.6 | 259.0 |

First loops (empty caches): card 15.8, 24.3, 25.2, 64.4; processor 40.4, 76.6, 89.3, 266.3.

**Reading it.** On the card, a type past D-445's four (grade mode 111, the same pass in double precision) costs about as much as basic on smooth at complexity 6 (21.1 against 19.7 a frame, three layers, or about 9 against 7 ms over Noise alone); the heaviest setting, spline at complexity 20, 60.6. The card is quicker than the processor at every setting here, about four times (80.5, 89.6 and 259.0 on the processor). This is what the busy run said; the quiet figures are lower throughout.
