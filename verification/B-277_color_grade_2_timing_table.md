# B-277: frame times with Color Grade's second unit (D-398), PROVISIONAL

Measured on 2026-10-09 on the code commit 858894b1, built with
`cargo test --release --test b277_color_grade_2` and run with
`--ignored b277_color_grade_2_timing --exact`, one round on the card, then one with `B277_CPU` set
on the processor. **PROVISIONAL: the machine was not quiet.** `tasklist` showed other cargo
processes and another lane's test (`b155_gpu_chain`, which draws on the same card) running
through both rounds, and `b222_gpu_colour` after them. **The card figures cannot be read**: the
shot with no Color Grade came out slower than either shot with it, so the other test's use of the
card swamps the effect's own cost. They are kept below as measured, with no claim made from them.
The processor figures share the machine with that lane's work and are an upper bound. Both to be
measured again on a quiet machine.

There is no "before" for these sections, which are new; Noise alone is the shot without the
effect, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-276's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame on each of its first three layers, then the effect, every eighth
frame asked for whole at Full. **Again** is the median of the loops after the first (seven on
the card, two on the processor), milliseconds a frame.

| Shot | Card, again | Processor, again |
|---|---:|---:|
| Noise alone | 38.5 | 52.7 |
| Noise, then Color Grade, curves, hue curve and wheels | 27.4 | 114.0 |
| Noise, then Color Grade, an HSL secondary (soft warm key, cooled and dulled) | 19.4 | 90.5 |

On the processor, with the machine shared: about 20 ms a 1080p layer for the curves, hue curve
and wheels, and about 13 ms for the HSL secondary. On the card: not readable in this run (see
above); each section is one more pass of the grade's own chain in double precision, as B-276's
steps. No claim against Target P2 until measured on a quiet machine.

## Raw

### Card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 45.0 | 38.5 |
| Noise, then Color Grade, curves, hue curve and wheels | Full | 60.6 | 27.4 |
| Noise, then Color Grade, an HSL secondary (soft warm key, cooled and dulled) | Full | 23.2 | 19.4 |

### Processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 58.1 | 52.7 |
| Noise, then Color Grade, curves, hue curve and wheels | Full | 103.8 | 114.0 |
| Noise, then Color Grade, an HSL secondary (soft warm key, cooled and dulled) | Full | 103.8 | 90.5 |
