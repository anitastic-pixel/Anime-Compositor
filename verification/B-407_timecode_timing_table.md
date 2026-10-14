# B-407: frame times with Timecode (D-527)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b407_timecode --no-run` and run once with `--ignored b407_timecode_timing`, which times every row below in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (1% busy; 3.6 GB of its memory held by the desktop and open programs); the same after it. The processor was 2% busy or less before each run. The owner's app was not open. One run, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone 3.9; Timecode, composition source at size 200, then Noise 3.9 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 5.4 | 2.0 |
| Timecode, composition source at size 200, then Noise | Full | 45.3 | 2.0 |
| Timecode, custom drop frame on a half-opacity box, composite off, then Noise | Full | 36.1 | 1.9 |

Timecode is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-229's way: a 1920 by 1080 still written by the test, every eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks. Timecode, then a Noise that changes every frame, so no finished frame is kept; the box and words are drawn again each frame. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the next seven loops' 210 frames. Milliseconds a frame.

**Reading it.** The first loop asks for 30 frames Timecode has not drawn yet: with the composition's timecode at size 200 a frame took 45.3 milliseconds and with the custom drop-frame count at size 120 on a half-opacity box 36.1, against 5.4 for Noise alone. The later loops ask for the same frames again, which are kept, and cost 2.0 and 1.9 against 2.0. So a frame played for the first time is still far above EFFECTS.md's Target P2 (4 ms added), about 31 to 40 ms added: the box and the characters are drawn again for every new frame, on the processor. That is about two to three times what Numbers' timecode at the same size took in B-406's quiet run (20.2), not the ten times the busy runs suggested; why it costs more was not measured. Finding the slow part and making a new frame's characters cheap (each digit's outline drawn once and reused) is left for a later speed unit.
