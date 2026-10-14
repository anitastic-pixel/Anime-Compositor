# B-277: frame times with Color Grade's second unit (D-398)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b277_color_grade_2 --no-run` and run with `--ignored b277_color_grade_2_timing`: once on the card, then once with `B277_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 4% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-09, on a machine that was not quiet), again: Noise alone card 38.5, processor 52.7; Noise, then Color Grade, curves, hue curve and wheels card 27.4, processor 114.0; Noise, then Color Grade, an HSL secondary (soft warm key, cooled and dulled) card 19.4, processor 90.5 ms.

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
| Noise alone | 12.3 | 41.2 |
| Noise, then Color Grade, curves, hue curve and wheels | 27.9 | 77.7 |
| Noise, then Color Grade, an HSL secondary (soft warm key, cooled and dulled) | 20.0 | 56.0 |

**Reading it.** On the card, the curves, hue curve and wheels add 15.6 ms a frame over the Noise
alone (12.3 to 27.9, three layers), about 5.2 ms a 1080p layer, over Target P2's 4 ms; the HSL
secondary adds 7.7 ms (20.0), about 2.6 ms a layer, within it. Each section is one more pass of the
grade's own chain in double precision, as B-276's steps. The busy run's card figures could not be
read (Noise alone came out slowest); these can. On the processor about 12 ms a layer for the
curves, hue curve and wheels and about 5 ms for the HSL secondary (77.7 and 56.0 against 41.2),
under the busy run's 20 and 13.

## Raw

### Card

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 15.5 | 12.3 |
| Noise, then Color Grade, curves, hue curve and wheels | Full | 31.3 | 27.9 |
| Noise, then Color Grade, an HSL secondary (soft warm key, cooled and dulled) | Full | 23.0 | 20.0 |

### Processor

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 41.2 | 41.2 |
| Noise, then Color Grade, curves, hue curve and wheels | Full | 81.2 | 77.7 |
| Noise, then Color Grade, an HSL secondary (soft warm key, cooled and dulled) | Full | 55.5 | 56.0 |
