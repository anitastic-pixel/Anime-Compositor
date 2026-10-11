# B-405: frame times with IDentifier (D-525)

**PROVISIONAL.** Measured on 2026-10-11, built with `cargo test --release --test b405_identifier` from 875dbb03 (B-405's code), and run once with `--ignored b405_identifier_timing`. Machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11. Three runs of another lane's card test (b155_gpu_chain) and several cargo processes were running when the run started and when it ended, so the table is PROVISIONAL. The owner's app was not touched.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 21.9 | 3.9 |
| IDentifier as added (colours), then Noise | Full | 9.9 | 4.0 |
| IDentifier alpha matte of 3, then Noise | Full | 6.6 | 3.5 |

IDentifier is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-229's way: a 1920 by 1080 EXR still written by the test (a picture, and object ids 0 to 5 in blocks), every eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks. IDentifier, then a Noise that changes every frame, so no finished frame is kept; the ids are kept by the pass cache as D-349's are, and the picture is worked out again each frame. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the next seven loops' 210 frames. Milliseconds a frame.

**Reading it.** With IDentifier a frame took 3.5 to 4.0 milliseconds again against 3.9 for Noise alone: the picture is worked out once as the ids are read and kept with the file's passes, so a frame only lays it on, within EFFECTS.md's Target P1. The first loop's figures fall from 21.9 to 6.6 because the shots after the first start warmed up; they are not a sign that IDentifier makes frames faster. Three runs of another lane's card test (b155_gpu_chain) and several cargo processes were running from start to end, so the numbers are PROVISIONAL and should be measured again on a quiet machine before any speed claim.
