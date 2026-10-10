# B-297: frame times with Fill (D-418)

**PROVISIONAL.** Measured on 2026-10-10, built with `cargo test --release --test b297_fill` from
60a31533 (B-297's code), and run with `--ignored b297_fill_timing`: once on the card, then once
with `B297_CPU` set on the processor. The machine was not quiet: other lanes' cargo builds and
tests (b295_fractal on the card) were running during both runs; Noise alone took twice what it
took for Grid. These numbers are to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Fill, every eighth frame asked for whole at Full. Each layer has three masks of mode None
(circles of radius 400, 220 and 300). **Again** is the median of the loops after the first
(seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 24.3 | 44.6 |
| Noise, then Fill as added (the whole layer) | 17.4 | 55.2 |
| Noise, then Fill, mask 1, feathers 30 and 10, inverted, opacity 80 | 45.1 | 167.3 |
| Noise, then Fill, All Masks, feathers 60 | 90.1 | 313.9 |

First loops (empty caches): card 107.3, 24.4, 68.0, 81.3; processor 56.4, 56.6, 229.9, 302.0.

**Reading it, provisionally.** The whole layer costs less than the noise of this run on the card
(17.4 against 24.3) and about 4 ms a layer on the processor. With masks the covering is made on
the processor (each mask's 4 by 4 samples over the drawing grown by the feather's reach) and
sent to the card: about 7 ms a layer for one mask feathered 30 and 10, about 20 ms a layer for
three masks feathered 60, against 40 and 90 on the processor alone.
