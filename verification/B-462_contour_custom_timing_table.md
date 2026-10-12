# B-462: frame times with Contour Paint, Match to Nearest Custom Color (D-584)

Measured on 2026-10-11, built with `cargo test --release --test b462_contour_custom` from 41a09a7e
(B-462's code), and run with `--ignored b462_contour_custom_timing`: once on the card, then once
with `B462_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, nothing else was running. Between the card and processor runs, these build or test programs were running: anime_compositor_app-7254, cargo.exe. After the processor run, these build or test programs were running: anime_compositor_app-7254, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Contour Paint filling Over Smeared Image matched to custom colours on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.1 | 47.9 |
| Noise, then Contour Paint, fills Over Smeared Image matched to 2 custom colours | 101.5 | 705.1 |
| the same matched to 8 custom colours | 104.1 | 617.9 |

First loops (empty caches): card 15.8, 104.9, 104.5; processor 48.7, 1154.3, 623.3.

**Reading it.** On the card, Contour Paint, Match to Nearest Custom Color costs an unknown amount a frame (the machine was busy) over the Noise alone in the first setting
(12.1 to 101.5, three layers), and the second setting takes 104.1: the same passes as K-Means' fills, with the palette set by hand instead of found.
At these settings a busy machine cannot say whether the card is quicker than the processor here.
