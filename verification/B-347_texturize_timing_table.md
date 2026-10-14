# B-347: frame times with Texturize (D-467)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b347_texturize --no-run` and run with `--ignored b347_texturize_timing`: once on the card, then once with `B347_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 4% busy; 4.2 GB to 4.3 GB of its memory held by the desktop and open programs); the same after each. The processor was 2% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 32.1, processor 124.0; Noise, then Texturize with layer4 tiled as the texture card 122.7, processor 360.6 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Texturize with the fourth layer as the texture on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.2 | 42.9 |
| Noise, then Texturize with layer4 tiled as the texture | 43.0 | 83.6 |
| the same, stretched, Light Direction 45, Texture Contrast 2 | 46.6 | 87.5 |

First loops (empty caches): card 15.8, 46.2, 45.6; processor 42.3, 82.2, 87.1.

**Reading it.** On the card, Texturize with layer4 tiled costs about 10 ms a 1080p layer (12.2 to
43.0, three layers), and about 11.5 ms stretched with Light Direction 45 and Texture Contrast 2
(46.6): two passes, the texture's luminance then the relief, each pixel read a few times, whatever
the placement. On the processor it costs about 14 and 15 ms a layer (83.6 and 87.5 against 42.9),
so the card is only a little quicker here. The busy run could not say this.
