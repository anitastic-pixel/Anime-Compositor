# B-392: frame times with Channel Wipe (D-512) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b392_channel_wipe` from
5be80a13 (B-392's code), and run with `--ignored b392_channel_wipe_timing`: once on the card,
then once with `B392_CPU` set on the processor.
**The machine was not quiet.** Three other cargo processes (other lanes' tests, among them
release builds of the GPU chain test `b155_gpu_chain` drawing on the same card) were running
before, during and after both runs. The numbers below are recorded as they came out. They are
not a clean measurement of Channel Wipe: the card's Noise alone took 91.0 ms a frame here,
against 11.8 ms on the quiet machine in B-387's table, and the processor's 109.0 ms against
57.4 ms in B-390's busy table. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Channel
Wipe, every eighth frame asked for whole at Full. **Again** is the median of the loops after the
first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 91.0 | 109.0 |
| Noise, then Channel Wipe, completion 50 (as added otherwise) | 72.4 | 180.8 |
| Noise, then Channel Wipe, completion 50, layer 4 in hue, softness 20, blur 6 | 268.8 | 354.3 |

First loops (empty caches): card 150.6, 124.2, 289.0; processor 112.4, 198.8, 316.8.

**Reading it.** Every shot draws to the end on both. On the card the shot with the layer
itself as the gradient came out faster than the Noise alone, which shows how much the other
tests' use of the card moved these numbers. With layer 4 as the gradient the named layer is
drawn on the processor first and handed to the card (D-189's path, as Gradient Wipe's and
Glass Melt's), which costs more than the wipe. Re-run the two commands above when no other
cargo, rustc or test process is running and the card is idle, and replace this table.
