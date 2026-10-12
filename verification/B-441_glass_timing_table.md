# B-441: frame times with CC Glass's full control set (D-561) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test b441_glass` from 1ee6b229, and run
with `--ignored b441_glass_timing`: once on the card, then once with `B441_CPU` set on the processor.
**The machine was not quiet.** Another lane's `b155_gpu_chain` test and two cargo processes were
running during both runs. The numbers are recorded as they came out; a quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second) with a Noise that changes every
frame on its first three layers (so nothing is kept), then CC Glass, every eighth frame of 240
asked for whole at Full. **Again** is the median of the loops after the first (seven on the card,
two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 40.7 | 109.8 |
| Noise, then CC Glass as added (a distant light) | 47.4 | 174.0 |
| the same, Luminance, softness 30, a point light, metal 0 | 48.7 | 158.3 |

First loops (empty caches): card 50.5, 55.8, 61.3; processor 108.3, 504.1, 146.3.

**Reading it.** Every shot draws to the end on both. On the card CC Glass with its new light costs
about 7 to 8 ms a frame over the Noise, in the one `glass` pass it already had. The processor's 48
to 64 ms over the Noise, and its 504 ms first loop, are likely inflated by the busy machine.
Re-run the two commands above when no other cargo, rustc or test process is running and replace
this table.
