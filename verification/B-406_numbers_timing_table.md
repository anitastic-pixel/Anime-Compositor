# B-406: frame times with Numbers (D-526)

**PROVISIONAL.** Measured on 2026-10-11, built with `cargo test --release --test b406_numbers` from 3f16fc4f (B-406's code), and run once with `--ignored b406_numbers_timing`. Machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11. Another lane's card test (b155_gpu_chain), cargo and two rustc builds were running when the run started, and cargo, rustc builds and another lane's test (b425_lens_dirt) when it ended, so the table is PROVISIONAL. The owner's app was not touched.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 6.2 | 2.5 |
| Numbers, timecode 30 at size 200, then Noise | Full | 27.3 | 2.7 |
| Numbers, monospaced long date, stroke over fill, then Noise | Full | 797.5 | 3.7 |

Numbers is new, so there is no "before" build: the row without it is the comparison.

**What is timed**, B-229's way: a 1920 by 1080 still written by the test, every eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks. Numbers, then a Noise that changes every frame, so no finished frame is kept; the words are drawn again each frame. **First** is the median of the first loop's 30 frames, started with empty caches; **Again** the median of the next seven loops' 210 frames. Milliseconds a frame.

**Reading it.** The first loop asks for 30 frames Numbers has not drawn yet: with the timecode at size 200 a frame took 27.3 milliseconds against 6.2 for Noise alone, and with the monospaced long date drawn as a 6-pixel stroke over the fill at size 120 it took 797.5, although that date is the same on every frame. The later loops ask for the same frames again, whose words are kept, and cost 2.7 and 3.7 against 2.5. So a frame played for the first time is far above EFFECTS.md's Target P2 (4 ms added), most of all with a stroke: the words are drawn again for each new frame even when they have not changed, and the stroked long date (more characters, each with an outline 6 pixels wide) costs far more than the plain timecode; which of the two is the slow part was not measured. Keeping the words by what they say rather than by frame would make an unchanging date nearly free; that, and finding what makes the stroked words slow, are left for a later speed unit. Another lane's card test (b155_gpu_chain) was running when the run started, and rustc builds and another lane's test (b425_lens_dirt) when it ended, so the figures are PROVISIONAL and should be measured again on a quiet machine.
