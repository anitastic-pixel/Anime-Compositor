# B-351: frame times with Fast Blur (Legacy) (D-471)

Measured on 2026-10-11, built with `cargo test --release --test b351_fast_blur_legacy` from d55345c5
(B-351's code), and run with `--ignored b351_fast_blur_legacy_timing`: once on the card, then once
with `B351_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 5.3 GB of its memory in use by another program and these build or test programs were running: b221_gpu_mix-7be658513acc, cargo.exe. Between the card and processor runs, the graphics card had 5.4 GB of its memory in use by another program and these build or test programs were running: b221_gpu_mix-7be658513acc, cargo.exe. After the processor run, the graphics card had 5.5 GB of its memory in use by another program and these build or test programs were running: b221_gpu_mix-7be658513acc, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Fast Blur (Legacy) on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.8 | 41.6 |
| Noise, then Fast Blur (Legacy) Blurriness 10 | 15.4 | 75.7 |
| the same at Blurriness 100 | 23.4 | 116.6 |

First loops (empty caches): card 16.4, 19.4, 29.7; processor 41.1, 70.7, 115.4.

**Reading it.** On the card, Fast Blur (Legacy) costs an unknown amount a frame (the machine was busy) over the Noise alone in the first setting
(12.8 to 15.4, three layers), and the second setting takes 23.4: Fast Box Blur's two passes, each pixel reading 2 x 3 ceil(0.37 B) + 1 neighbours a pass.
At these settings a busy machine cannot say whether the card is quicker than the processor here.
