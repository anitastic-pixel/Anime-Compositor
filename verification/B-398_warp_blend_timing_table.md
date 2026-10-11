# B-398: frame times with Warp Blend (D-518) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b398_warp_blend` from c3a2d2c0
(B-398's code), and run with `--ignored b398_warp_blend_timing`: once on the card, then once with
`B398_CPU` set on the processor.
**The machine was not quiet.** Other lanes' cargo runs (two cargo processes) were running before
the card's run, between the two runs and after them. The numbers below are recorded as they came
out. They are not a clean measurement of Warp Blend: the card's Noise alone took 47.7 ms a frame
here, against 11.8 ms on the quiet machine in B-387's table, and the processor's 79.8 ms against
57.4 ms in B-390's busy table. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Warp Blend,
every eighth frame asked for whole at Full. **Again** is the median of the loops after the first
(seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 47.7 | 79.8 |
| Noise, then Warp Blend, completion 50, revealing layer 4 (as added otherwise) | 219.2 | 254.9 |
| Noise, then Warp Blend, completion 50, local differences, twisting, smoothness 30, revealing layer 4 | 226.2 | 464.4 |
| Noise, then Warp Blend, completion 50, smoothness 0, revealing nothing | 86.0 | 195.5 |

First loops (empty caches): card 58.8, 193.6, 193.5, 85.4; processor 91.4, 246.2, 503.8, 164.2.

**Reading it.** Every shot draws to the end on both. The two shots revealing layer 4 are the slow
ones on the card as on the processor: they draw layer 4 a second time each frame, as D-189's map,
before the `wreact` pass, the blur, the slope and the `warpb` pass; on the processor the wider
blur (smoothness 30) costs most. How much of that is the effect and how much the other runs' use
of the machine is not known from this table. Re-run the two commands above when no other cargo,
rustc or test process is running and the card is idle, and replace this table.
