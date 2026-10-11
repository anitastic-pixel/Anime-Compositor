# B-350: frame times with Contour Paint (D-470)

Measured on 2026-10-11, built with `cargo test --release --test b350_contour_paint` from 8eaf20f6
(B-350's code), and run with `--ignored b350_contour_paint_timing`: once on the card, then once
with `B350_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, these build or test programs were running: anime_compositor_app-7254, cargo.exe, rustc.exe. Between the card and processor runs, these build or test programs were running: b441_glass-0edd6932c37264, cargo.exe. After the processor run, the graphics card had 7.1 GB of its memory in use by another program and these build or test programs were running: b151_gpu_fx-7cd91d30903cc, b412_cc_sphere-f40a387352, b429_reduce_interlace_fli, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Contour Paint on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 17.6 | 42.8 |
| Noise, then Contour Paint as added (three smears) | 97.7 | 674.1 |
| the same with fills Over Smeared Image, K-Means, 8 colours | 146.0 | 842.4 |

First loops (empty caches): card 18.6, 103.0, 216.9; processor 41.8, 550.4, 887.9.

**Reading it.** On the card, Contour Paint costs an unknown amount a frame (the machine was busy) over the Noise alone in the first setting
(17.6 to 97.7, three layers), and the second setting takes 146.0: per smear pass a luminance read, the tensor, its two Gaussian passes, the tangent and the smear, each pixel walking 2 ceil(2 s) steps.
At these settings a busy machine cannot say whether the card is quicker than the processor here.
