# B-287: frame times with Transform (D-408)

**PROVISIONAL.** Measured on 2026-10-09, built with
`cargo test --release --test b287_transform` from bf642e8f (B-287's code), and run with
`--ignored b287_transform_timing`: one round on the card, then one with `B287_CPU` set on the
processor. `tasklist` showed no other cargo, rustc or test program before the card round, but
another lane's cargo was running between the two rounds and after them, so the card round's end
and the whole processor round may share the machine.

Noise alone is the shot without Transform, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame on each of its first three layers, then the effect, every eighth
frame asked for whole at Full. **Again** is the median of the loops after the first (seven on
the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 11.7 | 45.2 |
| Noise, then Transform turned 15 degrees (bilinear) | 15.5 | 56.2 |
| Noise, then Transform skewed 20, scaled 80 by 120, bicubic | 22.9 | 62.3 |

First loops (empty caches): card 15.3, 18.1, 29.4; processor 45.1, 59.3, 63.0.

**Reading it.** On the card a bilinear Transform adds about 3.8 ms over three layers, about
1.3 ms a 1080p layer; bicubic, with its 16 samples in f64, about 11.2 ms, about 3.7 ms a layer.
Both are inside Target P2. On the processor about 3.7 ms a layer bilinear and 5.7 ms bicubic.
A Transform through the shutter is drawn on the processor, as D-188 draws every effect of a
motion-blurred layer, and is not timed here.
