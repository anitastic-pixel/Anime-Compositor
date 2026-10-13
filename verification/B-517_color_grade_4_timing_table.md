# B-517: frame times with Color Grade's fourth unit (D-637)

Measured on 2026-10-13, built with `cargo test --release --test b517_color_grade_4` from df7b0888, and run with `--ignored b517_color_grade_4_timing`: once on the card, then once with `B517_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. During the runs another lane's b155_gpu_chain (on the same card) and four other cargo processes were running. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Color Grade on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 34.2 | 124.5 |
| Noise, then Color Grade: a dark vignette at roundness -60 | 42.9 | 473.4 |
| Noise, then Color Grade: sharpen 50 (drawn by the processor either way) | 269.2 | 486.4 |

First loops (empty caches): card 143.8, 86.0, 471.2; processor 157.6, 219.3, 620.5.

The squarer vignette adds about 3 ms a 1080p layer on the card. A sharpened grade is drawn by the processor, so with the card on it costs a trip to the processor and back; the processor's vignette figure (473.4) is far above its Noise alone and its sharpen figure, which a busy machine explains better than the effect does.
