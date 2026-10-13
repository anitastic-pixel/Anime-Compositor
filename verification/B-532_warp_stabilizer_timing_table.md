# B-532: frame times with Warp Stabilizer (D-653)

Measured on 2026-10-13. The build was `cargo test --release --test b532_warp_stabilizer -- --ignored` from ff1cbcf6 (B-532's code), run once; the run took about 3 hours 25 minutes. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11.

**PROVISIONAL.** The machine was busy for part of the run: other lanes had 4 cargo or rustc processes running before it and none after it, and Rolling Shutter Repair's timing (B-533) ran at the same time for part of it. Which rows overlapped is not known, so every number below may be inflated by an unknown amount. The owner's app was not touched.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again | Afresh | Warp Stabilizer stage, afresh |
|---|---|---:|---:|---:|---:|
| Noise alone | Full | 35.0 | 18.5 | 85.3 | 0.0 |
| Warp Stabilizer as added, then Noise | Full | 5518.2 | 10326.8 | 6837.9 | 6258.3 |
| Warp Stabilizer, Stabilize Only, Detailed Analysis on, then Noise | Full | 27907.7 | 19050.9 | 13367.2 | 12666.7 |
| Noise alone | Half | 29.3 | 20.8 | 51.9 | 0.0 |
| Warp Stabilizer as added, then Noise | Half | 1296.1 | 1320.0 | 1369.6 | 1116.0 |

Warp Stabilizer is new, so there is no "before" build. The "Noise alone" rows are the comparison.

## What is timed

This follows B-227's, B-530's and B-531's method:
- The reference shot: 1920 by 1080, 24 frames a second, 3 layers.
- On each layer, Warp Stabilizer and then a Noise that changes every frame, so no finished frame is kept.
- Every eighth frame of 240, each asked for whole as the viewer asks. Draw on: GPU (Warp Stabilizer itself runs on the processor).
- **First** is the median of the first loop's 30 frames, started with empty caches. **Again** is the median of the seven loops after it.
- **Afresh** is the median of the same 30 frames, each with empty caches, so every stabilization is worked. **Warp Stabilizer stage, afresh** is the time spent inside the effect on those frames.
- Milliseconds a frame.

## Reading it

- At Smoothness 50 each frame works the motion between 12 pairs of neighbouring frames a layer, and none is kept for the next frame, so the cache does not help.
- **As added** costs about 6 s a frame inside the effect at Full and about 1.1 s at Half.
- **Detailed Analysis on** costs about 13 s a frame inside the effect at Full, about twice as much as as added.
- The as-added Again column at Full (10.3 s) is above its First (5.5 s), and the Detailed Analysis First (27.9 s) is above its Again and Afresh. Neither is explained; both may be the other timing running at the same time. They are to be re-measured.
- Every row with the effect is far above the Target P4 (100 ms a frame on the processor) and far from real time (41.7 ms at 24 frames a second). Keeping each pair's motion from frame to frame, a card version and a faster estimate would each be separate pieces of work.

No speed claim is made from this table. It is to be run again on a quiet machine.
