# B-484: frame times with Liquify (D-605)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b484_liquify --no-run` and run with `--ignored b484_liquify_timing`: once on the card, then once with `B484_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB to 3.0 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-12, on a machine that was not quiet), again: Noise alone card 66.1, processor 99.4; Noise, then Liquify Warp, brush 200, stroked across the middle card 158.0, processor 248.2 ms.

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
| Noise alone | 14.5 | 41.5 |
| Noise, then Liquify Warp, brush 200, stroked across the middle | 22.0 | 61.3 |
| Noise, then Liquify Twirl Clockwise, brush 600, one dab | 19.0 | 59.8 |

First loops (empty caches): card 17.9, 23.4, 22.4; processor 41.1, 60.4, 59.7.

**Reading it.** Liquify is drawn by D-601's backward field, warp mode 151. On the card the Warp
stroke adds about 7.5 ms a frame over three layers (14.5 to 22.0), about 2.5 ms a 1080p layer, and
the single Twirl dab about 4.5 ms (19.0), 1.5 ms a layer. This changes the busy run's reading
(about 30 ms a layer for the stroke), which was the card's memory nearly full of other programs'
work, not the effect. On the processor the stroke adds about 6.6 ms a layer and the dab about 6.1
(41.5 to 61.3 and 59.8), against the busy run's 50 and 20.
