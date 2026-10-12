# B-445: frame times with Energy Stroke's extras (D-565)

Measured on 2026-10-11, built with `cargo test --release --test b445_energy_stroke_extras` from 880270e3's engine code
(measured before a panel label was amended into that commit),
and run with `--exact --ignored b445_energy_stroke_extras_timing`: once on the card, then once with
`B445_CPU` set on the processor. No other cargo, rustc or test process was seen running when
checked before and after the processor run; the machine is taken as quiet.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second) with a Noise that changes every
frame on its first three layers (so nothing is kept), then Energy Stroke, every eighth frame of 240
asked for whole at Full. **Again** is the median of the loops after the first (seven on the card,
two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 41.8 |
| Noise, then Energy Stroke as added | 14.7 | 44.0 |
| Noise, then Energy Stroke, core 30, both ends flat | 15.7 | 45.0 |
| Noise, then Energy Stroke, distortion 20 at scale 80, noise on the layer | 55.1 | 125.6 |
| Noise, then Energy Stroke, Alpha Mode masks, one mask on each layer (drawn on the processor) | 71.1 | 64.5 |

First loops (empty caches): card 15.9, 18.7, 19.8, 58.2, 65.2; processor 41.2, 43.6, 44.2, 124.7, 64.5.

**Reading it.** Flat ends cost about 1 ms a frame more than round ones on either. Distortion
breaks the beam into a bit every half pixel, each bit's noise worked out on the processor, so a
wobbling beam costs about 40 ms more on the card and 80 on the processor; D-508's along-path
distortion has no quiet figure to set beside it (`verification/B-388_energy_stroke_timing_table.md`
was measured on a busy machine). Alpha Mode on keeps the effect off the card: on the card path it
costs what the processor costs plus the trip off the card, about 7 ms more than drawing on the
processor alone. Every shot draws to the end on both.
