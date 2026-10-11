# B-407: frame times with Timecode (D-527)

**PROVISIONAL.** Measured on 2026-10-11, built with `cargo test --release --test b407_timecode` from 516fd328 (B-407's code), and run once with `--ignored b407_timecode_timing`. Machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11. Two other lanes' tests (b425_lens_dirt, b396_stretch_wipe) and cargo were running when the run started, and b396_stretch_wipe and cargo when it ended, so the table is PROVISIONAL. The owner's app was not touched.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 12.0 | 3.9 |
| Timecode, composition source at size 200, then Noise | Full | 290.0 | 3.9 |
| Timecode, custom drop frame on a half-opacity box, composite off, then Noise | Full | 229.0 | 6.1 |

Timecode is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-229's way: a 1920 by 1080 still written by the test, every eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks. Timecode, then a Noise that changes every frame, so no finished frame is kept; the box and words are drawn again each frame. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the next seven loops' 210 frames. Milliseconds a frame.

**Reading it.** The first loop asks for 30 frames Timecode has not drawn yet: with the composition's timecode at size 200 a frame took 290.0 milliseconds and with the custom drop-frame count at size 120 on a half-opacity box 229.0, against 12.0 for Noise alone. The later loops ask for the same frames again, which are kept, and cost 3.9 and 6.1 against 3.9. So a frame played for the first time is far above EFFECTS.md's Target P2 (4 ms added): the box and the characters are drawn again for every new frame, on the processor. This is about ten times what Numbers' timecode at the same size took in B-406's run (27.3), although both draw through the same words; why was not measured, and both runs were on a busy machine. Finding the slow part and making a new frame's characters cheap (each digit's outline drawn once and reused) is left for a later speed unit. Two other lanes' tests (b425_lens_dirt, b396_stretch_wipe) and cargo were running when the run started, and one when it ended, so the figures are PROVISIONAL and should be measured again on a quiet machine.
