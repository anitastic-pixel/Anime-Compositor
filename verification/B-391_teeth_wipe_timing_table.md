# B-391: frame times with Teeth Wipe (D-511) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test
b391_teeth_wipe` from fd183759 (B-391's code), and run with `--ignored
b391_teeth_wipe_timing`: once on the card, then once with `B391_CPU` set on the processor.
**The machine was not quiet.** Another lane's cargo test (a debug build of the GPU chain test,
`b155_gpu_chain`, drawing on the same card) was running before, during and after both runs. The
numbers below are recorded as they came out. They are not a clean measurement of Teeth Wipe: the
card's Noise alone took 15.9 ms a frame here, against 11.8 ms on the quiet machine in B-387's
table, and the processor's 116.2 ms against 57.4 ms in B-390's busy table. A quiet re-measure is
owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Teeth Wipe,
every eighth frame asked for whole at Full. **Again** is the median of the loops after the first
(seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 15.9 | 116.2 |
| Noise, then Teeth Wipe, completion 50 (as added otherwise) | 19.2 | 203.4 |
| Noise, then Teeth Wipe, completion 50, blocks, direction 30 | 19.6 | 214.2 |
| Noise, then Teeth Wipe, completion 50, trapezoids 80 high and 20 wide | 23.8 | 194.3 |

First loops (empty caches): card 18.5, 25.5, 26.2, 23.0; processor 119.9, 220.9, 191.9, 186.0.

**Reading it.** Every shot draws to the end on both, and the card ran each Teeth Wipe shot
about eight to eleven times as fast as the processor. Teeth Wipe's own share on the card (about
3 to 8 ms a frame over the Noise alone) cannot be trusted while another test shares the card.
Re-run the two commands above when no other cargo, rustc or test process is running and the
card is idle, and replace this table.
