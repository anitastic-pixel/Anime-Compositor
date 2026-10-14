# B-485: frame times with Reshape (D-606)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b485_reshape --no-run` and run with `--ignored b485_reshape_timing`: once on the card, then once with `B485_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-13, on a machine that was not quiet), again: Noise alone card 45.1, processor 205.8; Noise, then Reshape, square to the square moved right card 92.2, processor 424.3 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Reshape
as shown (the layers' masks: a square, the same square moved 200 right, and a large boundary about
both, all of mode None), every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 13.1 | 43.4 |
| Noise, then Reshape, square to the square moved right | 41.7 | 78.4 |
| Noise, then Reshape, the same within a boundary, Liquid, 60 per cent | 29.9 | 69.1 |

First loops (empty caches): card 17.3, 39.1, 31.4; processor 42.0, 79.1, 68.4.

**Reading it.** Reshape is drawn by D-601's backward field, warp mode 151; its steps are worked on
the processor at each corner of the 256 by 256 grid, every corner blending the pulls of 64 source
points (and 64 boundary points when one is named), before the card draws. On the card the square
pulled right adds about 28.6 ms a frame over three layers (13.1 to 41.7), about 9.5 ms a 1080p
layer, and the same within a boundary about 16.8 ms (29.9), 5.6 ms a layer; on the processor
about 11.7 and 8.6 ms a layer (43.4 to 78.4 and 69.1). That is about half the busy run's figures.
The boundary run is quicker than the plain run on both here too, on a quiet machine, so that is the
effect, not the machine as the busy run supposed; these figures do not show why, though it does
more work for each grid corner.
