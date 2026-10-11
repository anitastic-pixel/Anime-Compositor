# B-429: frame times with Reduce Interlace Flicker (D-549)

Measured on 2026-10-11, built with `cargo test --release --test b429_reduce_interlace_flicker` from 3f6e4b5f, and run with `--ignored b429_reduce_interlace_flicker_timing`: once on the card, then once with `B429_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. During the runs other lanes' build and test programs were running (cargo.exe, b350_contour_paint, b151_gpu_fx, b412_cc_sphere), and the graphics card had about 3.3 GB of its memory in use by other programs. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Reduce Interlace Flicker on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 30.2 | 63.3 |
| Noise, then Reduce Interlace Flicker as added (softness 1) | 108.8 | 112.1 |
| Noise, then Reduce Interlace Flicker softness 2.5 | 112.5 | 134.2 |

First loops (empty caches): card 58.6, 124.7, 157.2; processor 100.4, 123.1, 122.7.

**Reading it.** Reduce Interlace Flicker is Gaussian Blur's one-way blur, which is drawn on the processor even on the card's path, so the card's lead over the Noise alone (30.2 against 63.3) is lost once it is added: about 109 to 113 ms a frame either way at softness 1, and 112.5 on the card against 134.2 on the processor at 2.5, three layers, on a busy machine. It has no measured stage of its own: in the perf report it is counted as Gaussian Blur's.
