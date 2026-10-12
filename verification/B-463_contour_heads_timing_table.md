# B-463: frame times with Contour Paint, Arrowheads (D-585)

Measured on 2026-10-12, built with `cargo test --release --test b463_contour_heads` from fd0a0380
(B-463's code), and run with `--ignored b463_contour_heads_timing`: once on the card, then once
with `B463_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, nothing else was running. Between the card and processor runs, nothing else was running. After the processor run, these build or test programs were running: b446_turbulent_noise_type, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Contour Paint on View Edge Map on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 21.0 | 69.1 |
| Noise, then Contour Paint, View Edge Map, Arrowheads off | 32.4 | 175.0 |
| the same with Arrowheads on | 30.5 | 240.9 |

First loops (empty caches): card 25.9, 37.3, 35.8; processor 64.8, 192.9, 168.3.

**Reading it.** On the card, Contour Paint, Arrowheads costs an unknown amount a frame (the machine was busy) over the Noise alone in the first setting
(21.0 to 32.4, three layers), and the second setting takes 30.5: the same passes as the Edge Map without heads, the heads drawn in the same pass.
At these settings a busy machine cannot say whether the card is quicker than the processor here.
