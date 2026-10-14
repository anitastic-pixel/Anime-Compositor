# B-276: frame times with Color Grade (D-397)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b276_color_grade --no-run` and run with `--ignored b276_color_grade_timing`: once on the card, then once with `B276_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-09, on a machine that was not quiet), again: Noise alone card 14.2, processor 54.3; Noise, then Color Grade, basic (temperature 30, exposure 0.5, contrast 40, saturation 120) card 25.4, processor 117.4; Noise, then Color Grade, every step but the look (basic, faded film, vibrance, tints, vignette) card 37.7, processor 203.3 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame on each of its first three layers, then the effect, every eighth
frame asked for whole at Full. **Again** is the median of the loops after the first (seven on
the card, two on the processor), milliseconds a frame.

| Shot | Card, again | Processor, again |
|---|---:|---:|
| Noise alone | 14.2 | 43.9 |
| Noise, then Color Grade, basic (temperature 30, exposure 0.5, contrast 40, saturation 120) | 26.8 | 76.9 |
| Noise, then Color Grade, every step but the look (basic, faded film, vibrance, tints, vignette) | 37.7 | 115.2 |

**Reading it.** On the card, the basic settings add 12.6 ms a frame over the Noise alone (14.2 to
26.8, three layers), about 4.2 ms a 1080p layer, just over Target P2's 4 ms; every step but the
look adds 23.5 ms (37.7), about 7.8 ms a layer, well over it. Each step is its own pass in double
precision, so the cost grows with the steps used. The card figures are the same as the busy run's
(25.4 and 37.7), so the reading against Target P2 holds. On the processor about 11 and 24 ms a
layer (76.9 and 115.2 against 43.9), about half the busy run's 21 and 50.

## Raw

### Card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 17.4 | 14.2 |
| Noise, then Color Grade, basic (temperature 30, exposure 0.5, contrast 40, saturation 120) | Full | 29.8 | 26.8 |
| Noise, then Color Grade, every step but the look (basic, faded film, vibrance, tints, vignette) | Full | 41.4 | 37.7 |

### Processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 43.9 | 43.9 |
| Noise, then Color Grade, basic (temperature 30, exposure 0.5, contrast 40, saturation 120) | Full | 76.1 | 76.9 |
| Noise, then Color Grade, every step but the look (basic, faded film, vibrance, tints, vignette) | Full | 114.5 | 115.2 |
