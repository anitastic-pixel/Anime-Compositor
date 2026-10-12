# B-352: frame times with Gaussian Blur (Legacy) (D-472)

Measured on 2026-10-11, built with `cargo test --release --test b352_gaussian_blur_legacy` from fae7fd1f
(B-352's code), and run with `--ignored b352_gaussian_blur_legacy_timing`: once on the card, then once
with `B352_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, these build or test programs were running: b155_gpu_chain-c4cee726c1, b442_cc_vignette-2c442ec5, cargo.exe, rustc.exe. Between the card and processor runs, these build or test programs were running: b155_gpu_chain-c4cee726c1, cargo.exe, rustc.exe. After the processor run, these build or test programs were running: anime_compositor_app-5586, b155_gpu_chain-c4cee726c1, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Gaussian Blur (Legacy) on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 29.7 | 124.1 |
| Noise, then Gaussian Blur (Legacy) Blurriness 10 | 63.5 | 251.9 |
| the same at Blurriness 100 | 77.7 | 334.6 |

First loops (empty caches): card 26.8, 78.0, 72.9; processor 127.3, 244.0, 220.7.

**Reading it.** On the card, Gaussian Blur (Legacy) costs an unknown amount a frame (the machine was busy) over the Noise alone in the first setting
(29.7 to 63.5, three layers), and the second setting takes 77.7: Gaussian Blur's two passes, each pixel reading 2 ceil(6.5 x 0.3 B) + 1 neighbours a pass.
At these settings a busy machine cannot say whether the card is quicker than the processor here.
