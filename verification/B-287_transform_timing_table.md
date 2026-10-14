# B-287: frame times with Transform (D-408)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b287_transform --no-run` and run with `--ignored b287_transform_timing`: once on the card, then once with `B287_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 2% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-09, on a machine that was not quiet), again: Noise alone card 11.7, processor 45.2; Noise, then Transform turned 15 degrees (bilinear) card 15.5, processor 56.2 ms.

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
| Noise alone | 13.5 | 41.2 |
| Noise, then Transform turned 15 degrees (bilinear) | 15.2 | 55.0 |
| Noise, then Transform skewed 20, scaled 80 by 120, bicubic | 18.8 | 60.2 |

First loops (empty caches): card 17.5, 18.9, 22.4; processor 41.3, 55.4, 60.6.

**Reading it.** On the card a bilinear Transform adds about 1.7 ms over three layers (13.5 to 15.2), about
0.6 ms a 1080p layer; bicubic, with its 16 samples in f64, about 5.3 ms (18.8), about 1.8 ms a layer.
Both are inside Target P2, as the busy run had it; bicubic costs about half what the busy run gave
(3.7 ms a layer). On the processor about 4.6 ms a layer bilinear and 6.3 ms bicubic.
A Transform through the shutter is drawn on the processor, as D-188 draws every effect of a
motion-blurred layer, and is not timed here.
