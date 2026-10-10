# B-263: frame times with Photo Filter (D-384)

Measured on 2026-10-09 on the code commit 24e02e8, built with
`cargo test --release --test b263_photo_filter` and run with
`--ignored b263_photo_filter_timing --exact`, a copy of the test binary run twice: one round on
the card, then one with `B263_CPU` set on the processor, each time. `tasklist` showed no cargo
or rustc process before and after every round, and the owner's app was not running, so the
machine was quiet. Round 2's processor figures with the effect (57.9 and 60.8 ms) sit well above
round 1's while Noise alone did not move, so round 1 is the one quoted for the processor.

There is no "before" for the effect, which is new; Noise alone is the shot without it, timed in
the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame on each of its first three layers, then the effect, every eighth
frame asked for whole at Full. **Again** is the median of the loops after the first (seven on
the card, two on the processor), milliseconds a frame.

| Shot | Card, round 1 | Card, round 2 | Processor, round 1 | Processor, round 2 |
|---|---:|---:|---:|---:|
| Noise alone | 12.0 | 11.7 | 41.8 | 41.3 |
| Noise, then Photo Filter as added (Warming (85), 25, luminosity kept) | 14.4 | 14.4 | 53.9 | 57.9 |
| Noise, then Photo Filter, custom colour at 60, luminosity off | 14.3 | 14.2 | 52.0 | 60.8 |

About 0.8 to 0.9 ms a layer on the card, within Target P1; about 3.4 to 4.0 ms a layer on the
processor (round 1).

## Raw

### Round 1, card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 16.6 | 12.0 |
| Noise, then Photo Filter as added (Warming (85), 25, luminosity kept) | Full | 18.7 | 14.4 |
| Noise, then Photo Filter, custom colour at 60, luminosity off | Full | 17.9 | 14.3 |

### Round 1, processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 41.3 | 41.8 |
| Noise, then Photo Filter as added (Warming (85), 25, luminosity kept) | Full | 52.8 | 53.9 |
| Noise, then Photo Filter, custom colour at 60, luminosity off | Full | 52.0 | 52.0 |

### Round 2, card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 15.3 | 11.7 |
| Noise, then Photo Filter as added (Warming (85), 25, luminosity kept) | Full | 19.3 | 14.4 |
| Noise, then Photo Filter, custom colour at 60, luminosity off | Full | 17.7 | 14.2 |

### Round 2, processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 40.7 | 41.3 |
| Noise, then Photo Filter as added (Warming (85), 25, luminosity kept) | Full | 52.3 | 57.9 |
| Noise, then Photo Filter, custom colour at 60, luminosity off | Full | 59.1 | 60.8 |
