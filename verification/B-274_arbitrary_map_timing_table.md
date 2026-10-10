# B-274: frame times with Arbitrary Map (D-395)

**PROVISIONAL: the machine was not quiet.** Measured on 2026-10-09, built with
`cargo test --release --test b274_arbitrary_map` from the working copy that became ee2a7d55 (B-274's code
1949635f and B-275's Selective Color), and run with `--ignored b274_arbitrary_map_timing --exact`, a copy
of the test binary run twice: one round on the card, then one with `B274_CPU` set on the
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
| Noise alone | 12.4 | 13.4 | 43.4 | 42.9 |
| Noise, then Arbitrary Map, master_rgb_1024.amp, phase 40 | 15.6 | 16.6 | 58.0 | 58.1 |

About 1.1 ms a layer on the card, within Target P1; about 4.9 to 5.1 ms a layer on the
processor. Provisional.

## Raw

### Round 1, card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 16.9 | 12.4 |
| Noise, then Arbitrary Map, master_rgb_1024.amp, phase 40 | Full | 26.8 | 15.6 |

### Round 1, processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 42.5 | 43.4 |
| Noise, then Arbitrary Map, master_rgb_1024.amp, phase 40 | Full | 57.1 | 58.0 |

### Round 2, card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 17.2 | 13.4 |
| Noise, then Arbitrary Map, master_rgb_1024.amp, phase 40 | Full | 22.4 | 16.6 |

### Round 2, processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 42.5 | 42.9 |
| Noise, then Arbitrary Map, master_rgb_1024.amp, phase 40 | Full | 58.7 | 58.1 |
