# B-288: frame times with Lens Chromatic Aberration (D-409)

**PROVISIONAL.** Measured on 2026-10-10, built with
`cargo test --release --test b288_lens_chromatic_aberration` from e8b67b7b (B-288's code), and run
with `--ignored b288_lens_chromatic_aberration_timing`: two rounds, each one on the card and then
one with `B288_CPU` set on the processor. `tasklist` showed no other cargo, rustc or test program
before the first round, but another lane's cargo and test program (b155) were running when the
second round ended, so the second round may share the machine. The table is the first round; the
second is given under it.

Noise alone is the shot without the effect, timed in the same run.

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
| Noise alone | 15.7 | 42.3 |
| Noise, then radial, amount 10, falloff 100 | 16.8 | 84.5 |
| Noise, then radial, amount 10, falloff 100, fringe blur 100 (11 samples a channel) | 41.4 | 204.8 |
| Noise, then offset, amount 6 at 90 degrees | 15.7 | 69.0 |

Second round, the same order: card 11.8, 16.9, 41.5, 15.9; processor 40.9, 100.8, 232.5, 89.3.
First loops (empty caches), first round: card 16.8, 20.8, 44.5, 18.9; processor 43.1, 83.2,
212.1, 70.3.

**Reading it.** On the card, radial and offset without fringe blur add under 2 ms a 1080p layer
(0.4 and 1.7 ms a layer radial in the two rounds, 0 and 1.4 offset), inside Target P2. Fringe blur
100 at amount 10 takes 11 samples a channel at the corners and adds about 9 ms a layer (8.6 and
9.9), over Target P2's 4 ms; its cost grows with the amount times the fringe blur, as the number
of samples does. On the processor about 14 ms a layer radial, 9 ms offset and 54 ms with fringe
blur 100 (first round).
