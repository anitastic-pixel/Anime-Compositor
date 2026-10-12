# B-470: frame times with CC Force Motion Blur (D-581)

Measured on 2026-10-12, built with `cargo test --release --test b470_force_motion_blur` from 1cac5bad (B-470's code), and run once with `--ignored`. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11. **PROVISIONAL:** not a quiet machine; 4 cargo or rustc processes from other lanes were counted before the run and 4 after it. The owner's app was not touched.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Precomposed, Noise alone | Full | 202.5 | 100.0 |
| CC Force Motion Blur, Override 180 degrees, 8 samples, then Noise | Full | 472.0 | 448.8 |
| CC Force Motion Blur, Override 360 degrees, 16 samples, then Noise | Full | 604.7 | 409.8 |
| Precomposed, Noise alone, played | Full | 44.7 | 28.6 |
| CC Force Motion Blur, 180 degrees, 8 samples, then Noise, played | Full | 224.3 | 31.5 |

Played rows: frames 0 to 47 in order, twice, the viewer's cache 16521 MB (Memory: Automatic).

CC Force Motion Blur is new, so there is no "before" build: the rows without it are the comparison.

**What is timed**, B-227's way: the reference shot (1920 by 1080, 24 a second) precomposed into one composition layer, with on that layer CC Force Motion Blur (Override on) and then a Noise that changes every frame, so no finished frame is kept; every eighth frame of 240, at Full, each frame asked for whole as the viewer asks, Draw on: GPU. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the seven loops after it. The played rows ask for frames 0 to 47 in order, twice, through the viewer's cache, as playing does. Milliseconds a frame.

**Reading it.** CC Force Motion Blur draws the composition inside N times a frame on the processor, before the card. Once warm, 8 samples at 180 degrees cost about 349 ms a frame over the precomposed shot alone (448.8 against 100.0), and 16 samples at 360 degrees about 310 ms (409.8); that the 16-sample row is not slower than the 8-sample row is not explained and may be the other lanes' builds, which is one reason these numbers are provisional. Either way a frame with it is far slower than real time (41.7 ms at 24 a second). Played in order through the viewer's cache, the averaged pictures are kept, so the second time through costs 31.5 ms a frame against 28.6 without the effect; the first time through costs 224.3 ms against 44.7. A card pass for the averaging, and drawing the moments on the card, would be separate pieces of work.
