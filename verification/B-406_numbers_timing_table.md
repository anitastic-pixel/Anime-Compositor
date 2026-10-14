# B-406: frame times with Numbers (D-526)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b406_numbers --no-run` and run once with `--ignored b406_numbers_timing`, which times every row below in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 3.6 GB of its memory held by the desktop and open programs); the same after it. The processor was 5% busy or less before each run. The owner's app was not open. One run, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone 2.5; Numbers, timecode 30 at size 200, then Noise 2.7 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 5.5 | 2.0 |
| Numbers, timecode 30 at size 200, then Noise | Full | 20.2 | 2.1 |
| Numbers, monospaced long date, stroke over fill, then Noise | Full | 13.8 | 2.1 |

Numbers is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-229's way: a 1920 by 1080 still written by the test, every eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks. Numbers, then a Noise that changes every frame, so no finished frame is kept; the words are drawn again each frame. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the next seven loops' 210 frames. Milliseconds a frame.

**Reading it.** The first loop asks for 30 frames Numbers has not drawn yet: with the timecode at size 200 a frame took 20.2 milliseconds against 5.5 for Noise alone, and with the monospaced long date drawn as a 6-pixel stroke over the fill at size 120 it took 13.8. The later loops ask for the same frames again, whose words are kept, and cost 2.1 against 2.0. So a frame played for the first time is still above EFFECTS.md's Target P2 (4 ms added), by about 8 to 15 ms, because the words are drawn again for each new frame even when they have not changed. This changes the busy run's reading in one place: the stroked long date is not the slow case (its 797.5 ms was the machine); it now costs less than the plain timecode. Keeping the words by what they say rather than by frame would make an unchanging date nearly free; that is left for a later speed unit.
