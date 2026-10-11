# B-342: frame times with Mr. Smoothie (D-462)

Measured on 2026-10-11, built with `cargo test --release --test b342_mr_smoothie` from 968fa56e
(B-342's code), and run with `--ignored b342_mr_smoothie_timing`: once on the card, then once
with `B342_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 5.2 GB of its memory in use by another program and these build or test programs were running: b226_gpu_last_four-9751e2, cargo.exe, rustc.exe. Between the card and processor runs, the graphics card had 5.1 GB of its memory in use by another program and these build or test programs were running: b226_gpu_last_four-9751e2, cargo.exe, rustc.exe. After the processor run, these build or test programs were running: b389_glass_melt-c82e33c09, cargo.exe, rustc.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Mr. Smoothie, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 17.7 | 72.5 |
| Noise, then Mr. Smoothie as added | 26.5 | 109.0 |
| the same, Hue, smoothness 30, BAB, phase 90 | 30.3 | 122.6 |

First loops (empty caches): card 18.2, 30.1, 43.6; processor 99.3, 200.4, 115.1.

**Reading it.** On the card, Mr. Smoothie costs an unknown amount a frame (the machine was busy) over the Noise alone as added
(17.7 to 26.5, three layers, smoothness 5), and an unknown amount at Hue, smoothness 30
(30.3): the channel read, the Gaussian's two passes, then one bilinear read a pixel.
At these settings a busy machine cannot say whether the card is quicker than the processor here.
