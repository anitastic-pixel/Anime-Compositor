# B-530: frame times with Pixel Motion Blur (D-650)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with
`cargo test --release --test b530_pixel_motion_blur --no-run` and run once with
`--ignored b530_pixel_motion_blur_timing`, which times every row below in that one run. Before the
run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB
of its memory held by the desktop and open programs); the same after it. The owner's app was not
open. One run, single figures, not medians of rounds.

Earlier provisional figures (busy machine): on 2026-10-12 at b2715696, again, Noise alone 16.8 and
Pixel Motion Blur as added 42.3 ms at Full; on 2026-10-13 at 60d69b7e, afresh, Pixel Motion Blur as
added 4451.4 ms with its stage 4009.9 ms, and Manual 64 samples 5483.3 ms with its stage 4801.4 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

| Shot | Quality | First | Again | Afresh | Pixel Motion Blur stage, afresh |
|---|---|---:|---:|---:|---:|
| Noise alone | Full | 16.7 | 13.5 | 43.0 | 0.0 |
| Pixel Motion Blur as added (Automatic: 180 degrees, 16 samples; Vector Detail 20), then Noise | Full | 22.0 | 17.6 | 557.1 | 418.3 |
| Pixel Motion Blur Manual 360 degrees, 64 samples, Vector Detail 100, then Noise | Full | 21.4 | 17.3 | 1820.1 | 1635.2 |
| Noise alone | Half | 29.7 | 19.4 | 49.3 | 0.0 |
| Pixel Motion Blur as added, then Noise | Half | 36.5 | 27.0 | 131.3 | 45.2 |

Pixel Motion Blur is new, so there is no "before" build. The rows without it are the comparison.

## What is timed

This follows B-227's method:
- The reference shot: 1920 by 1080, 24 frames a second, 3 layers.
- On each layer, Pixel Motion Blur and then a Noise that changes every frame, so no finished frame is kept.
- Every eighth frame of 240, each asked for whole as the viewer asks. Draw on: GPU.
- **First** is the median of the first loop's 30 frames, started with empty caches. **Again** is the median of the seven loops after it.
- **Afresh** asks for every frame with an empty cache, so each Pixel Motion Blur is worked; the last column is the Pixel Motion Blur stage's own time in those frames.
- Milliseconds a frame.

## Reading it

**First and Again mostly time the viewer's cache, not the estimate.** The reference shot's layers hold the same drawing for many frames. The cache key holds the content of the frames either side (D-650), so whenever two frames have the same drawing before, at and after them, the second reuses the first's blurred picture. That is correct: the picture cannot differ. But it means few of these frames actually run the estimate, and those rows are close to Noise alone.

**The cost when the estimate does run is in the Afresh column.** At Full, Pixel Motion Blur's own stage takes about 0.42 s a frame as added (Vector Detail 20, 16 samples) and about 1.6 s with Manual, 64 samples, Vector Detail 100, over three layers. That is about ten times less than the busy run's 4.0 and 4.8 s, which were mostly the machine. At Half, as added, the stage takes 45.2 ms, about a ninth of Full's 418.3, a little under the quarter the pixel count alone would give; the busy run's figure was the same (46.0), so the Half figure stands and it is Full's that the busy machine had inflated.

Either way, a frame whose drawings change is far above the row's Target P4 (100 ms a frame on the processor) at Full, and far slower than real time (41.7 ms at 24 frames a second).

A card version (D-651, proposed) and a faster estimate would each be separate pieces of work.
