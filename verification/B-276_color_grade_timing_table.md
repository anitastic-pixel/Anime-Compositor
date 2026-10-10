# B-276: frame times with Color Grade (D-397), PROVISIONAL

Measured on 2026-10-09 on the code commit 5facd83a, built with
`cargo test --release --test b276_color_grade` and run with
`--ignored b276_color_grade_timing --exact`, one round on the card, then one with `B276_CPU` set
on the processor. **PROVISIONAL: the machine was not quiet.** `tasklist` showed other cargo
processes and another lane's test (`b222_gpu_colour`, which draws on the same card) running
through both rounds, so the card figures here are shared with that test and the processor
figures with that lane's builds. They are an upper bound, to be measured again on a quiet machine.

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

| Shot | Card, again | Processor, again |
|---|---:|---:|
| Noise alone | 14.2 | 54.3 |
| Noise, then Color Grade, basic (temperature 30, exposure 0.5, contrast 40, saturation 120) | 25.4 | 117.4 |
| Noise, then Color Grade, every step but the look (basic, faded film, vibrance, tints, vignette) | 37.7 | 203.3 |

With the card shared: about 3.7 ms a 1080p layer for the basic settings, within Target P2's 4 ms,
and about 7.8 ms a layer with every step but the look, over it; each step is its own pass in
double precision, so the cost grows with the steps used. On the processor about 21 and 50 ms a
layer. To be confirmed on a quiet machine before any claim against Target P2.

## Raw

### Card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 18.0 | 14.2 |
| Noise, then Color Grade, basic (temperature 30, exposure 0.5, contrast 40, saturation 120) | Full | 29.7 | 25.4 |
| Noise, then Color Grade, every step but the look (basic, faded film, vibrance, tints, vignette) | Full | 44.4 | 37.7 |

### Processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 59.2 | 54.3 |
| Noise, then Color Grade, basic (temperature 30, exposure 0.5, contrast 40, saturation 120) | Full | 103.0 | 117.4 |
| Noise, then Color Grade, every step but the look (basic, faded film, vibrance, tints, vignette) | Full | 221.9 | 203.3 |
