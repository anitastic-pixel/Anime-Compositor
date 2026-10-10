# B-275: frame times with Selective Color (D-396)

**PROVISIONAL: the machine was not quiet.** Measured on 2026-10-09, built with
`cargo test --release --test b275_selective_color` from the working copy that became ee2a7d55 (B-274's code
1949635f and B-275's Selective Color), and run with `--ignored b275_selective_color_timing --exact`, a copy
of the test binary run twice: one round on the card, then one with `B275_CPU` set on the
processor, each time. `tasklist` showed the other lane's test (`b221_gpu_mix`, with two cargo
processes) running before and after every round, so these figures may be high; re-measure on a
quiet machine before quoting them as final.

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
| Noise alone | 12.8 | 13.0 | 44.8 | 44.4 |
| Noise, then Selective Color, reds cyan +100, absolute | 16.2 | 16.0 | 61.1 | 57.4 |
| Noise, then Selective Color, all nine families, relative | 16.7 | 15.9 | 59.3 | 57.7 |

About 1.0 to 1.3 ms a layer on the card, within Target P1; about 4.3 to 5.4 ms a layer on
the processor. Provisional.

## Raw

### Round 1, card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 17.2 | 12.8 |
| Noise, then Selective Color, reds cyan +100, absolute | Full | 21.1 | 16.2 |
| Noise, then Selective Color, all nine families, relative | Full | 20.7 | 16.7 |

### Round 1, processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 44.6 | 44.8 |
| Noise, then Selective Color, reds cyan +100, absolute | Full | 61.7 | 61.1 |
| Noise, then Selective Color, all nine families, relative | Full | 58.2 | 59.3 |

### Round 2, card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 23.5 | 13.0 |
| Noise, then Selective Color, reds cyan +100, absolute | Full | 21.2 | 16.0 |
| Noise, then Selective Color, all nine families, relative | Full | 19.9 | 15.9 |

### Round 2, processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 43.7 | 44.4 |
| Noise, then Selective Color, reds cyan +100, absolute | Full | 57.8 | 57.4 |
| Noise, then Selective Color, all nine families, relative | Full | 56.6 | 57.7 |
