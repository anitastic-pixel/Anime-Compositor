# B-275: frame times with Selective Color (D-396)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b275_selective_color --no-run` and run with `--ignored b275_selective_color_timing`: once on the card, then once with `B275_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-09, on a machine that was not quiet), again: Noise alone card 12.8 and 13.0, processor 44.8 and 44.4; Noise, then Selective Color, reds cyan +100, absolute card 16.2 and 16.0, processor 61.1 and 57.4; Noise, then Selective Color, all nine families, relative card 16.7 and 15.9, processor 59.3 and 57.7 ms.

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
| Noise alone | 13.4 | 43.6 |
| Noise, then Selective Color, reds cyan +100, absolute | 21.3 | 58.0 |
| Noise, then Selective Color, all nine families, relative | 22.1 | 58.2 |

**Reading it.** Selective Color adds 7.9 ms a frame over the Noise alone on the card with reds only
(13.4 to 21.3, three layers) and 8.7 ms with all nine families (22.1), about 2.6 to 2.9 ms a 1080p
layer, over Target P1's 1 ms; on the processor 14.4 and 14.6 ms, about 4.8 ms a layer. This
changes the busy run's reading of about 1.0 to 1.3 ms a layer within Target P1: the busy runs gave
about 16 ms for the frame, this quiet one 21 to 22. One quiet run, so a second would show whether
the card figure moves this much from run to run.

## Raw

### Card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 17.0 | 13.4 |
| Noise, then Selective Color, reds cyan +100, absolute | Full | 25.3 | 21.3 |
| Noise, then Selective Color, all nine families, relative | Full | 25.5 | 22.1 |

### Processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 43.0 | 43.6 |
| Noise, then Selective Color, reds cyan +100, absolute | Full | 57.3 | 58.0 |
| Noise, then Selective Color, all nine families, relative | Full | 57.3 | 58.2 |
