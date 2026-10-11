# B-404: frame times with Fog 3D (D-524)

**PROVISIONAL.** Measured on 2026-10-11, built with `cargo test --release --test b404_fog_3d` from 55706a33 (B-404's code), and run once with `--ignored b404_fog_3d_timing`. Machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11. Four runs of another lane's card test (b155_gpu_chain) and several cargo processes were running when the run started and when it ended, so the table is PROVISIONAL. The owner's app was not touched.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 24.3 | 6.5 |
| Fog 3D as added, then Noise | Full | 20.1 | 5.7 |
| Fog 3D from 4 to 20, density 0, then Noise | Full | 19.9 | 6.2 |

Fog 3D is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-229's way: a 1920 by 1080 EXR still written by the test (a picture, and a depth from 2 at the left to 20 at the right), every eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks. Fog 3D, then a Noise that changes every frame, so no finished frame is kept; the depth is kept by the pass cache as D-348's passes are, and the shares are worked out again each frame. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the next seven loops' 210 frames. Milliseconds a frame.

**Reading it.** With Fog 3D a frame took 5.7 to 6.2 milliseconds again against 6.5 for Noise alone: once the depth has been read and the shares worked out and kept with the file's passes, the card's fog pass costs less than the run-to-run spread, within EFFECTS.md's Target P1. The first loop's figures fall from 24.3 to about 20 because the shots after the first start warmed up. Four runs of another lane's card test (b155_gpu_chain) and several cargo processes were running from start to end, so the numbers are PROVISIONAL and should be measured again on a quiet machine before any speed claim.
