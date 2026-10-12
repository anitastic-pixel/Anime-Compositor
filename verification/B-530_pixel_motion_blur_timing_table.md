# B-530: frame times with Pixel Motion Blur (D-650)

Measured on 2026-10-12. The build was `cargo test --release --test b530_pixel_motion_blur` from b2715696 (B-530's code), run once with `--ignored`. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11.

**PROVISIONAL.** The machine was not quiet: other lanes had 4 cargo or rustc processes running before the run and 2 after it. The owner's app was not touched.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 23.6 | 16.8 |
| Pixel Motion Blur as added (Automatic: 180 degrees, 16 samples; Vector Detail 20), then Noise | Full | 25.8 | 42.3 |
| Pixel Motion Blur Manual 360 degrees, 64 samples, Vector Detail 100, then Noise | Full | 38.3 | 17.6 |
| Noise alone | Half | 21.7 | 17.8 |
| Pixel Motion Blur as added, then Noise | Half | 36.9 | 29.1 |

Pixel Motion Blur is new, so there is no "before" build. The rows without it are the comparison.

## What is timed

This follows B-227's method:
- The reference shot: 1920 by 1080, 24 frames a second, 3 layers.
- On each layer, Pixel Motion Blur and then a Noise that changes every frame, so no finished frame is kept.
- Every eighth frame of 240, each asked for whole as the viewer asks. Draw on: GPU.
- **First** is the median of the first loop's 30 frames, started with empty caches. **Again** is the median of the seven loops after it.
- Milliseconds a frame.

## Reading it

**These rows mostly time the viewer's cache, not the estimate.** The reference shot's layers hold the same drawing for many frames. The cache key holds the content of the frames either side (D-650), so whenever two frames have the same drawing before, at and after them, the second reuses the first's blurred picture. That is correct: the picture cannot differ. But it means few of these frames actually run the estimate, and the rows are close to Noise alone.

**The cost when the estimate does run is much higher.** It was measured with a scratch test during building, on one 1920 by 1080 layer, with the same busy machine, so treat it as rough:
- as added (Vector Detail 20): about 200 to 400 ms for the two estimates, and about 30 ms for the 16 samples;
- Manual, 64 samples, Vector Detail 100: about 1.5 to 4 s.

The estimate is most of the cost. Either way, a frame whose drawings change is far above the row's Target P4 (100 ms a frame on the processor) and far slower than real time (41.7 ms at 24 frames a second).

**Still to measure.** `tests/b530_pixel_motion_blur.rs` now also writes an "Afresh" column (every frame with an empty cache, so each Pixel Motion Blur is worked) and the Pixel Motion Blur stage's own time. It was not run, because builds and test runs were paused while drive C was full. It is to be run, on a quiet machine, before any speed claim is made.

A card version (D-651, proposed) and a faster estimate would each be separate pieces of work.
