# B-261: frame times with Gamma/Pedestal/Gain (D-382)

Measured on 2026-10-09 on the code commit 1de2270, built with
`cargo test --release --test b261_gamma_pedestal_gain` and run with
`--ignored b261_gamma_pedestal_gain_timing`, a copy of the test binary run twice: one round on
the card, then one with `B261_CPU` set on the processor, each time. `tasklist` showed no cargo
or rustc process before and after every round. The owner's app was not checked. Round 1's
processor figure for Noise alone (60.8 ms) is far above every other quiet round (41 to 42 ms),
so something else was running; round 2 is the one quoted.

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
| Noise alone | 14.5 | 12.8 | 60.8 | 41.6 |
| Noise, then Gamma/Pedestal/Gain, black stretch 1.5 and a gain | 16.8 | 15.0 | 69.8 | 52.4 |
| Noise, then Gamma/Pedestal/Gain, every channel its own curve | 18.6 | 14.9 | 73.0 | 53.6 |

About 0.7 ms a layer on the card (round 2), within Target P1; about 3.6 to 4.0 ms a layer on the
processor.

## Raw

### Round 1, card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 19.7 | 14.5 |
| Noise, then Gamma/Pedestal/Gain, black stretch 1.5 and a gain | Full | 25.3 | 16.8 |
| Noise, then Gamma/Pedestal/Gain, every channel its own curve | Full | 24.2 | 18.6 |

### Round 1, processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 45.7 | 60.8 |
| Noise, then Gamma/Pedestal/Gain, black stretch 1.5 and a gain | Full | 73.2 | 69.8 |
| Noise, then Gamma/Pedestal/Gain, every channel its own curve | Full | 69.6 | 73.0 |

### Round 2, card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 16.7 | 12.8 |
| Noise, then Gamma/Pedestal/Gain, black stretch 1.5 and a gain | Full | 19.1 | 15.0 |
| Noise, then Gamma/Pedestal/Gain, every channel its own curve | Full | 18.5 | 14.9 |

### Round 2, processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 40.8 | 41.6 |
| Noise, then Gamma/Pedestal/Gain, black stretch 1.5 and a gain | Full | 52.0 | 52.4 |
| Noise, then Gamma/Pedestal/Gain, every channel its own curve | Full | 54.0 | 53.6 |

