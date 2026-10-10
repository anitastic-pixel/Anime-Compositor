# B-279: frame times with Shadow/Highlight (D-400)

**Quiet machine for round 1.** Measured on 2026-10-09, built with
`cargo test --release --test b279_shadow_highlight` from 94fca5fa (B-279's code), and run with
`--ignored b279_shadow_highlight_timing`: one round on the card, then one with `B279_CPU` set on
the processor. `tasklist` showed no other cargo or rustc before, between and after round 1.
Round 2 is given for comparison only: three cargo processes (the other lane) were running when it
started, so it is not quoted.

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

| Shot | Card, round 1 (quiet) | Card, round 2 (busy) | Processor, round 1 (quiet) | Processor, round 2 (busy) |
|---|---:|---:|---:|---:|
| Noise alone | 12.6 | 12.5 | 42.3 | 45.6 |
| Noise, then Shadow/Highlight as added (shadow 50, radius 30) | 24.6 | 24.0 | 199.7 | 210.6 |
| Noise, then Shadow/Highlight, shadow 80, highlight 60, radii 12 and 40 | 30.8 | 31.6 | 256.5 | 261.7 |

First loops (empty caches), round 1: card 16.5, 28.3, 34.6; processor 42.2, 200.0, 255.0.

**Reading it.** On the card, as added, 12.0 ms over three layers: about 4.0 ms a 1080p layer, at
the edge of Target P2 (4 ms). With two different radii a second blur and a copy are added: 18.2 ms
over three, about 6.1 ms a layer, over Target P2. On the processor about 52 ms a layer as added
and 71 ms with two radii. The cost is the blur of the lightness at each radius (a radius-30
Gaussian is a 181-tap kernel each way) and the L*a*b* round trip in double precision; nothing is
proposed to speed it up yet.
