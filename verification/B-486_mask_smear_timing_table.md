# B-486: frame times with Mask Smear (D-607)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b486_mask_smear --no-run` and run with `--ignored b486_mask_smear_timing`: once on the card, then once with `B486_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-13, on a machine that was not quiet), again: Noise alone card 15.4, processor 50.2; Noise, then Mask Smear, the square carried 200 right card 52.9, processor 91.1 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Mask Smear
as shown (the layers' masks: a square, and a large boundary about it, both of mode None), every
eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.4 | 41.7 |
| Noise, then Mask Smear, the square carried 200 right | 41.0 | 77.6 |
| Noise, then Mask Smear, turned 30, scaled to 120, within a boundary, Liquid, 60 per cent | 29.0 | 79.6 |

First loops (empty caches): card 16.4, 38.7, 30.4; processor 41.3, 76.5, 75.3.

**Reading it.** Mask Smear is drawn as Reshape is, through D-601's backward field, warp mode 151:
its steps are worked on the processor at each corner of the 256 by 256 grid, every corner blending
the pulls of 64 source points (and 64 boundary points when one is named), before the card draws.
On the card the carried square adds about 28.6 ms a frame over three layers (12.4 to 41.0), about
9.5 ms a 1080p layer, and the turned, scaled one within a boundary about 16.6 ms (29.0), 5.5 ms a
layer; on the processor about 12.0 and 12.6 ms a layer (41.7 to 77.6 and 79.6). The card figures
match B-485's Reshape to the tenth, as they should for the same drawing. The boundary run is
quicker than the plain run on the card on a quiet machine too, so it is the effect, not the
machine; on the processor the two are close.
