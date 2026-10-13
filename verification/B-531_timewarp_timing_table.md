# B-531: frame times with Timewarp (D-652)

Measured on 2026-10-13. The build was `cargo test --release --test b531_timewarp -- --ignored` from 60d69b7e (B-531's code), run once. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11.

**PROVISIONAL.** The machine was busy: other lanes had 4 cargo or rustc processes running before the run and 2 after it, and Pixel Motion Blur's timing ran in the same command. Noise alone took about 260 ms a frame here against about 24 ms in B-530's earlier run, so every number below is inflated by an unknown amount. The owner's app was not touched.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again | Afresh | Timewarp stage, afresh |
|---|---|---:|---:|---:|---:|
| Noise alone | Full | 260.2 | 194.5 | 355.7 | 0.0 |
| Timewarp Whole Frames at Speed 50, then Noise | Full | 483.2 | 403.8 | 659.6 | 112.1 |
| Timewarp Frame Mix at Speed 50, then Noise | Full | 472.3 | 427.4 | 586.0 | 118.1 |
| Timewarp as added (Pixel Motion at Speed 50), then Noise | Full | 756.2 | 984.3 | 2340.4 | 1980.3 |
| Noise alone | Half | 95.0 | 82.1 | 255.2 | 0.0 |
| Timewarp as added, then Noise | Half | 2817.5 | 339.5 | 376.0 | 253.0 |

Timewarp is new, so there is no "before" build. The "Noise alone" rows are the comparison.

## What is timed

This follows B-227's and B-530's method:
- The reference shot: 1920 by 1080, 24 frames a second, 3 layers.
- On each layer, Timewarp and then a Noise that changes every frame, so no finished frame is kept.
- Every eighth frame of 240, each asked for whole as the viewer asks. Draw on: GPU (Timewarp itself runs on the processor).
- **First** is the median of the first loop's 30 frames, started with empty caches. **Again** is the median of the seven loops after it.
- **Afresh** is the median of the same 30 frames, each with empty caches, so every Timewarp is worked. **Timewarp stage, afresh** is the time spent inside Timewarp on those frames.
- Milliseconds a frame.

## Reading it

- **Whole Frames and Frame Mix** cost about 110 to 120 ms a frame inside Timewarp at Full, on this busy machine. That is mostly reading the layer at two source times.
- **Pixel Motion** (the setting when added) costs about 2 s a frame inside Timewarp at Full and about 250 ms at Half, when its motion estimate has to run. Its Again column at Full (984 ms) is not lower than First: the source frames either side change with the source time, so the cache helps less than for Pixel Motion Blur.
- The Half row's First (2.8 s) against its Again and Afresh (about 0.35 s) is not explained; it may be the machine's other work. It is to be re-measured.
- Every Timewarp row is above the row's Target P4 (100 ms a frame on the processor), and Pixel Motion is far slower than real time (41.7 ms at 24 frames a second). A card version and a faster estimate would each be separate pieces of work.

No speed claim is made from this table. It is to be run again on a quiet machine.
