# B-428: frame times with Spill Suppressor (D-548)

Measured on 2026-10-11, built with `cargo test --release --test b428_spill_suppressor` from 3f6e4b5f, and run with `--ignored b428_spill_suppressor_timing`: once on the card, then once with `B428_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. During the runs other lanes' build and test programs were running (cargo.exe, b350_contour_paint, b441_glass, b151_gpu_fx, b412_cc_sphere), and the graphics card had about 3.3 GB of its memory in use by other programs. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Spill Suppressor on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 23.4 | 42.6 |
| Noise, then Spill Suppressor as added (blue, 100) | 37.2 | 70.9 |
| Noise, then Spill Suppressor #28b43c at 60 | 33.8 | 68.4 |

First loops (empty caches): card 29.3, 59.0, 40.1; processor 42.3, 67.9, 76.7.

**Reading it.** On the card Spill Suppressor is Screen Key's tone pass (mode 90), fused after the Noise, each pixel read once; it adds about 10 to 14 ms a frame over the Noise alone here (23.4 to 37.2 and 33.8, three layers), on a busy machine. On the processor it adds about 26 to 28 ms a frame (42.6 to 70.9 and 68.4). Spill Suppressor has no measured stage of its own: in the perf report it is counted as Screen Key's.
