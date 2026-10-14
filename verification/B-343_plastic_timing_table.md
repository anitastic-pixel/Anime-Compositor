# B-343: frame times with Plastic (D-463)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b343_plastic --no-run` and run with `--ignored b343_plastic_timing`: once on the card, then once with `B343_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 4% busy; 4.3 GB of its memory held by the desktop and open programs); the same after each. The processor was 6% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 15.7, processor 57.4; Noise, then Plastic as added card 21.5, processor 138.4 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Plastic, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 13.2 | 41.7 |
| Noise, then Plastic as added | 19.5 | 81.1 |
| the same, Luminance, softness 30, a point light, metal 100 | 22.2 | 92.9 |

First loops (empty caches): card 16.3, 23.0, 26.0; processor 41.1, 81.0, 92.7.

**Reading it.** On the card, Plastic as added costs about 2.1 ms a 1080p layer (13.2 to 19.5, three
layers, softness 10), and about 3 ms at Luminance, softness 30, a point light, metal 100 (22.2):
the channel read, the Gaussian's two passes, then one lighting pass. On the processor it costs
about 13 and 17 ms a layer (81.1 and 92.9 against 41.7), so the card is about six times quicker.
The busy run could not say this.
