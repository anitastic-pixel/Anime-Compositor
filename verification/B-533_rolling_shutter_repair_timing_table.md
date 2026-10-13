# B-533: frame times with Rolling Shutter Repair (D-654)

Measured on 2026-10-13. The build was `cargo test --release --test b533_rolling_shutter_repair -- --ignored` from ef31a130 (B-533's code), run once. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11.

**PROVISIONAL.** The machine was busy: 9 cargo or rustc processes were running before the run and 6 after it, and Warp Stabilizer's own timing (B-532) ran at the same time on the same card and processor. Noise alone took about 190 ms a frame here at Full against about 24 ms in B-530's earlier run, so every number below is inflated by an unknown amount. The owner's app was not touched.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again | Afresh | Rolling Shutter Repair stage, afresh |
|---|---|---:|---:|---:|---:|
| Noise alone | Full | 188.3 | 58.9 | 102.1 | 0.0 |
| Rolling Shutter Repair as added (Warp), then Noise | Full | 2661.7 | 7260.4 | 7837.8 | 7257.8 |
| Rolling Shutter Repair, Pixel Motion, then Noise | Full | 2462.0 | 1865.0 | 1382.0 | 1036.5 |
| Noise alone | Half | 42.4 | 35.7 | 82.3 | 0.0 |
| Rolling Shutter Repair as added (Warp), then Noise | Half | 438.1 | 372.0 | 613.1 | 387.2 |

Rolling Shutter Repair is new, so there is no "before" build. The "Noise alone" rows are the comparison.

## What is timed

This follows B-227's, B-530's and B-531's method:
- The reference shot: 1920 by 1080, 24 frames a second, 3 layers.
- On each layer, Rolling Shutter Repair and then a Noise that changes every frame, so no finished frame is kept.
- Every eighth frame of 240, each asked for whole as the viewer asks. Draw on: GPU (Rolling Shutter Repair itself runs on the processor).
- **First** is the median of the first loop's 30 frames, started with empty caches. **Again** is the median of the seven loops after it.
- **Afresh** is the median of the same 30 frames, each with empty caches, so every repair is worked. **Rolling Shutter Repair stage, afresh** is the time spent inside the effect on those frames.
- Milliseconds a frame.

## Reading it

- **Pixel Motion** costs about 1 s a frame inside the effect at Full, when D-650's motion estimate has to run for both neighbouring frames.
- **Warp** (the setting when added) costs about 7 s a frame inside the effect at Full but about 390 ms at Half. Its Again column (7.3 s) is far above its First (2.7 s). Neither is explained; both may be Warp Stabilizer's timing running at the same time. It is to be re-measured alone.
- Every row with the effect is above the Target P4 (100 ms a frame on the processor) and far slower than real time (41.7 ms at 24 frames a second). A card version and a faster estimate would each be separate pieces of work.

No speed claim is made from this table. It is to be run again on a quiet machine.
