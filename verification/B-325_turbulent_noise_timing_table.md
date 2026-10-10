# B-325: frame times with Turbulent Noise (D-445)

Measured on 2026-10-10, built with `cargo test --release --test b325_turbulent_noise` from cb534f27
(B-325's code), and run with `--ignored b325_turbulent_noise_timing`: once on the card, then once
with `B325_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 11.1 GB of its memory in use by another program. Between the card and processor runs, the graphics card had 11.1 GB of its memory in use by another program. After the processor run, the graphics card had 11.1 GB of its memory in use by another program. Treat them as an upper bound, to be measured again on a quiet machine.

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
| Noise alone | 16.6 | 42.6 |
| Noise, then Turbulent Noise as added (complexity 6) | 30.8 | 80.5 |
| Noise, then Turbulent Noise turbulent, size 40, complexity 20 (the most) | 44.9 | 140.5 |

First loops (empty caches): card 21.2, 35.6, 51.4; processor 41.0, 79.5, 138.4.

**Reading it.** On the card, Turbulent Noise costs an unknown amount a frame (the machine was busy) over the Noise alone as
added (16.6 to 30.8, three layers), drawn by Fractal Noise's pass (grade mode 9)
and timed as Fractal Noise's stage. At these settings a busy machine cannot say whether the card is quicker than the processor here.
