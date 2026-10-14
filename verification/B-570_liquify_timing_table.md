# B-570: frame times with Liquify's other brushes (D-700)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b570_liquify_brushes --no-run` and run with `--ignored b570_liquify_brushes_timing`: once on the card, then once with `B570_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (2 and 12% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-13, on a machine that was not quiet), again: Noise alone card 31.6, processor 85.4; Noise, then Liquify Turbulence, brush 300, stroked across the middle card 79.2, processor 90.1 ms.

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
| Noise alone | 12.5 | 42.2 |
| Noise, then Liquify Turbulence, brush 300, stroked across the middle | 19.8 | 62.6 |
| Noise, then Liquify Warp with a freeze area mask | 20.3 | 63.8 |
| Noise, then Liquify Twirl, Reflection and Clone, then Reconstruction (Amplitwist) | 74.2 | 98.1 |

First loops (empty caches): card 16.9, 21.8, 21.8, 57.9; processor 42.4, 61.5, 62.1, 97.4.

**Reading it.** The Turbulence stroke and the frozen Warp stroke add about 7.3 and 7.8 ms a frame on
the card over three layers (12.5 to 19.8 and 20.3), about 2.5 ms a 1080p layer, and about 20 ms on
the processor (42.2 to 62.6 and 63.8), about 7 ms a layer. The last shot, four strokes with the
earlier three kept, adds about 62 ms on the card (74.2) and 56 ms on the processor (98.1), about 20
ms a layer on each, as it should be when both work the same field on the processor before drawing
it. This changes the busy run's reading: its 830 ms a frame on the card for that shot, and its 16 ms
a layer for the single strokes, were the machine; the gap D-700 records for it (in document 14) does not hold up on a quiet machine.
