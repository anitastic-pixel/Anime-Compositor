# B-570: frame times with Liquify's other brushes (D-700)

Measured on 2026-10-13, built with `cargo test --release --test b570_liquify_brushes` from 7e4db06b
(B-570's code), and run with `--ignored b570_liquify_brushes_timing`: once on the card, then once
with `B570_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet.
Other lanes were building and testing beside it, and before the runs the graphics card had 8.3 GB of
its memory in use by other programs. Treat them as an upper bound, to be measured again on a quiet
machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Liquify
as shown, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 31.6 | 85.4 |
| Noise, then Liquify Turbulence, brush 300, stroked across the middle | 79.2 | 90.1 |
| Noise, then Liquify Warp with a freeze area mask | 79.3 | 94.3 |
| Noise, then Liquify Twirl, Reflection and Clone, then Reconstruction (Amplitwist) | 859.0 | 214.8 |

First loops (empty caches): card 35.7, 85.6, 95.8, 1054.7; processor 65.6, 123.6, 89.1, 201.6.

**Reading it.** The Turbulence stroke and the frozen Warp stroke each added about 48 ms a frame on
the card over three layers, about 16 ms a 1080p layer, and on the processor a few milliseconds to
about 10 over a Noise-alone that itself wandered by 20 ms between loops. The last shot, four
strokes with the earlier three kept, is the odd one: about 830 ms a frame on the card against about
130 ms on the processor, though both work the same field on the processor before drawing it. This
run cannot say why; it is logged as a gap, to be measured again on a quiet machine before anyone
reads it as the card being slower.
