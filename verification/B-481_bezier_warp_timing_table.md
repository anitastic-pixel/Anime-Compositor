# B-481: frame times with Bezier Warp (D-602)

Measured on 2026-10-12, built with `cargo test --release --test b481_bezier_warp` from f7ef346c
(B-481's code), and run with `--ignored b481_bezier_warp_timing`: once on the card, then once with
`B481_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Other
lanes were building and testing beside it, and before the runs the graphics card had 2.2 GB of its
memory in use by other programs. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Bezier
Warp at Quality 8, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 18.3 | 199.2 |
| Noise, then Bezier Warp with the top edge sagging | 27.2 | 206.1 |
| Noise, then Bezier Warp folded (bottom right vertex at (-20, -20)) | 29.7 | 189.8 |

First loops (empty caches): card 21.1, 34.5, 28.7; processor 109.7, 256.6, 189.3.

**Reading it.** On the card, Bezier Warp (warp mode 150) added about 9 to 11 ms a frame over three
layers, about 3 to 4 ms a 1080p layer. On the processor the Noise alone swung from 109.7 to 199.2
between loops, more than the effect costs, so this busy machine cannot say what the effect adds there.
