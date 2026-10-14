# B-481: frame times with Bezier Warp (D-602)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b481_bezier_warp --no-run` and run with `--ignored b481_bezier_warp_timing`: once on the card, then once with `B481_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-12, on a machine that was not quiet), again: Noise alone card 18.3, processor 199.2; Noise, then Bezier Warp with the top edge sagging card 27.2, processor 206.1 ms.

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
| Noise alone | 12.6 | 41.9 |
| Noise, then Bezier Warp with the top edge sagging | 20.3 | 64.9 |
| Noise, then Bezier Warp folded (bottom right vertex at (-20, -20)) | 22.1 | 68.8 |

First loops (empty caches): card 16.1, 22.7, 23.5; processor 43.3, 64.3, 67.4.

**Reading it.** On the card, Bezier Warp (warp mode 150) adds about 7.7 ms a frame over three
layers with the top edge sagging (12.6 to 20.3) and 9.5 ms folded (22.1), about 2.6 to 3.2 ms a
1080p layer, a little under the busy run's 3 to 4. On the processor it adds about 23 and 27 ms a
frame (41.9 to 64.9 and 68.8), about 8 to 9 ms a layer: the busy run could not say, as its Noise
alone swung by 90 ms between loops; the quiet one can. The card is about three times quicker.
