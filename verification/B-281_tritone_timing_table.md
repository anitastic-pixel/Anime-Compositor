# B-281: frame times with Tritone (D-402)

**Both rounds on a quiet machine.** Measured on 2026-10-09, built with
`cargo test --release --test b281_tritone` from 617b6baa (B-281's code, with Aerial Haze's
uncommitted lines also in the build, which this shot does not use), and run with
`--ignored b281_tritone_timing`: one round on the card, then one with `B281_CPU` set on the
processor. `tasklist` showed no other cargo, rustc or test program before, between and after
the two rounds.

Noise alone is the shot without Tritone, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame on each of its first three layers, then the effect, every eighth
frame asked for whole at Full. **Again** is the median of the loops after the first (seven on
the card, two on the processor), milliseconds a frame.

| Shot | Card (quiet) | Processor (quiet) |
|---|---:|---:|
| Noise alone | 12.7 | 42.6 |
| Noise, then Tritone as added (white, sepia, black, blend 0) | 14.2 | 52.4 |
| Noise, then Tritone, night to sunset at blend 70 | 14.2 | 50.9 |

First loops (empty caches): card 16.2, 18.9, 17.8; processor 42.4, 51.7, 50.5.

**Reading it.** On the card Tritone adds about 1.5 ms over three layers: about 0.5 ms a 1080p
layer, inside Target P1 (1 ms a 1080p layer on the card). On the processor about 2.8 to 3.3 ms a
layer.
