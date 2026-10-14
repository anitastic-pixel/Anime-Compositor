# B-470: frame times with CC Force Motion Blur (D-581)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b470_force_motion_blur --no-run` and run once with `--ignored b470_force_motion_blur_timing`, which times every row below in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (2% busy; 2.9 GB of its memory held by the desktop and open programs); the same after it. The owner's app was not open. One run, single figures, not medians of rounds.

Earlier provisional figures (2026-10-12, on a machine that was not quiet), again: Precomposed, Noise alone 100.0; CC Force Motion Blur, Override 180 degrees, 8 samples, then Noise 448.8 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Precomposed, Noise alone | Full | 54.2 | 54.8 |
| CC Force Motion Blur, Override 180 degrees, 8 samples, then Noise | Full | 198.3 | 200.8 |
| CC Force Motion Blur, Override 360 degrees, 16 samples, then Noise | Full | 354.4 | 356.2 |
| Precomposed, Noise alone, played | Full | 52.3 | 29.6 |
| CC Force Motion Blur, 180 degrees, 8 samples, then Noise, played | Full | 202.7 | 43.2 |

Played rows: frames 0 to 47 in order, twice, the viewer's cache 16521 MB (Memory: Automatic).

CC Force Motion Blur is new, so there is no "before" build: the rows without it are the comparison.

**What is timed**, B-227's way: the reference shot (1920 by 1080, 24 a second) precomposed into one composition layer, with on that layer CC Force Motion Blur (Override on) and then a Noise that changes every frame, so no finished frame is kept; every eighth frame of 240, at Full, each frame asked for whole as the viewer asks, Draw on: GPU. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the seven loops after it. The played rows ask for frames 0 to 47 in order, twice, through the viewer's cache, as playing does. Milliseconds a frame.

**Reading it.** CC Force Motion Blur draws the composition inside N times a frame on the processor, before the card. Once warm, 8 samples at 180 degrees cost about 146 ms a frame over the precomposed shot alone (200.8 against 54.8), and 16 samples at 360 degrees about 301 ms (356.2): about twice as much for twice the samples, as it should be. (The busy run had the 16-sample row faster than the 8-sample row; on a quiet machine that is gone.) Either way a frame with it is far slower than real time (41.7 ms at 24 a second). Played in order through the viewer's cache, the averaged pictures are kept, so the second time through costs 43.2 ms a frame against 29.6 without the effect; the first time through costs 202.7 ms against 52.3. A card pass for the averaging, and drawing the moments on the card, would be separate pieces of work.
