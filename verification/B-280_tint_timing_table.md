# B-280: frame times with Tint (D-401)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b280_tint --no-run` and run with `--ignored b280_tint_timing`: once on the card, then once with `B280_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-09, on a machine that was not quiet), again: Noise alone card 12.4, processor 41.8; Noise, then Tint as added (black to white, amount 100) card 14.2, processor 51.3 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame on each of its first three layers, then the effect, every eighth
frame asked for whole at Full. **Again** is the median of the loops after the first (seven on
the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.3 | 41.6 |
| Noise, then Tint as added (black to white, amount 100) | 14.6 | 51.1 |
| Noise, then Tint, navy to gold at amount 30 | 14.6 | 51.2 |

First loops (empty caches): card 15.8, 19.1, 18.3; processor 40.6, 50.7, 50.3.

**Reading it.** On the card Tint adds about 2.3 ms over three layers in both settings (12.3 to 14.6): about 0.8 ms a 1080p layer, inside Target P1 (1 ms a 1080p layer on the card). On the processor about 3.2 ms a layer (41.6 to 51.1 and 51.2). The same as before: the earlier card round was already quiet, and the processor's round, marked provisional then, is confirmed.
