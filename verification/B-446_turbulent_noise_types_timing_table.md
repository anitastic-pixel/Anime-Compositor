# B-446: frame times with Turbulent Noise's full type lists (D-567)

Measured on 2026-10-12, built with `cargo test --release --test b446_turbulent_noise_types` from 358d86e3
(B-446's code), and run with `--ignored b446_turbulent_noise_types_timing`: once on the card, then once
with `B446_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Other
lanes were building and testing beside it, and afterwards the graphics card had 6.8 GB of its memory in use
by other programs. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Turbulent Noise, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 24.1 | 81.1 |
| Noise, then Turbulent Noise as added (basic, smooth, complexity 6) | 28.9 | 138.1 |
| Noise, then Turbulent Noise dynamic_twist on soft_linear, complexity 6 | 30.9 | 126.3 |
| Noise, then Turbulent Noise dynamic_twist on spline, size 40, complexity 20 (the most) | 69.2 | 339.8 |

First loops (empty caches): card 32.4, 41.7, 37.7, 79.2; processor 83.7, 119.1, 153.4, 335.0.

**Reading it.** On the card, a type past D-445's four (grade mode 111, the same pass in double
precision) cost about as much as basic on smooth at complexity 6 (30.9 against 28.9 a frame, three
layers); the heaviest setting, spline at complexity 20, 69.2. The card was quicker than the processor
at every setting here, about four to five times. A busy machine cannot say more than that.
