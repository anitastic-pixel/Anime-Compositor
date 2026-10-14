# B-531: frame times with Timewarp (D-652)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b531_timewarp --no-run` and run once with `--ignored b531_timewarp_timing`, on its own. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 3.7 GB of its memory held by the desktop and open programs); the same after it. The processor was 2% busy or less. The owner's app was not open. One run, single figures, not medians of rounds. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11.

Earlier provisional figures (2026-10-13, at 60d69b7e, on a machine that was not quiet), Full, again / Timewarp stage afresh: Noise alone 194.5; Whole Frames 403.8 / 112.1; Frame Mix 427.4 / 118.1; Pixel Motion as added 984.3 / 1980.3; at Half, as added 339.5 / 253.0 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again | Afresh | Timewarp stage, afresh |
|---|---|---:|---:|---:|---:|
| Noise alone | Full | 15.9 | 12.3 | 40.8 | 0.0 |
| Timewarp Whole Frames at Speed 50, then Noise | Full | 132.9 | 162.1 | 194.8 | 75.7 |
| Timewarp Frame Mix at Speed 50, then Noise | Full | 195.1 | 198.5 | 227.0 | 77.4 |
| Timewarp as added (Pixel Motion at Speed 50), then Noise | Full | 468.4 | 455.8 | 487.8 | 339.3 |
| Noise alone | Half | 27.9 | 18.7 | 49.7 | 0.0 |
| Timewarp as added, then Noise | Half | 193.1 | 178.1 | 216.1 | 116.3 |

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

- **Whole Frames and Frame Mix** cost about 76 to 77 ms a frame inside Timewarp at Full (three layers). That is mostly reading the layer at two source times. The busy run had 110 to 120.
- **Pixel Motion** (the setting when added) costs about 340 ms a frame inside Timewarp at Full and about 116 ms at Half, when its motion estimate has to run; the busy run had about 2 s and 250 ms. Its Again column at Full (455.8 ms) is still hardly lower than First (468.4): the source frames either side change with the source time, so the cache helps less than for Pixel Motion Blur.
- The busy run's Half First of 2.8 s was most likely the machine's other work: quiet, First, Again and Afresh at Half are 193.1, 178.1 and 216.1.
- Every Timewarp row is still above the row's Target P4 (100 ms a frame on the processor) once the frame is drawn whole (Again 162.1 to 455.8 at Full), and Pixel Motion is far slower than real time (41.7 ms at 24 frames a second). A card version and a faster estimate would each be separate pieces of work.
