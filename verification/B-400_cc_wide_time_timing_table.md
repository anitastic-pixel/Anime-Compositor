# B-400: frame times with CC Wide Time (D-520)

**PROVISIONAL.** Measured on 2026-10-11, built with `cargo test --release --test
b400_cc_wide_time` from 938a6872 (B-400's code), and run once with `--ignored
b400_cc_wide_time_timing`. No other cargo or rustc process was running when it started, but
another lane's test (b155) was running when it finished, so the machine was not quiet for the
whole run. The owner's app was not touched.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

CC Wide Time is new, so there is no "before" build: the row without it is the comparison. It is
drawn by the processor, as Echo and Moment Map are (several frames of the layer).

**What is timed**, B-227's way: the reference shot (1920 by 1080, 24 frames a second), every
eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks.
On each of the shot's first three layers: CC Wide Time, then a Noise that changes every frame,
so no finished frame is kept. **First** is the median of the first loop's 30 frames, started
with empty caches; **Again** the median of the next seven loops' 210 frames. Milliseconds a
frame.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 18.3 | 14.5 |
| CC Wide Time as added (3 and 3, 7 frames), then Noise | Full | 297.4 | 474.0 |
| CC Wide Time 10 and 10 (21 frames), then Noise | Full | 1153.0 | 1194.2 |

**Reading it.** As added, seven frames of each layer are drawn for every frame: about 150 ms a
1080p layer, above EFFECTS.md's Target P4 for processor-only time effects (100 ms a frame for
the whole frame). Twenty-one frames take about three times as long. Every eighth frame is asked
for, so no frame drawn for one is reused for the next; played frame after frame, the frames of
the span overlap, but they are drawn afresh today. Again ran slower than first as added, which
the other lane's test starting during the run may explain; measure again on a quiet machine
before relying on these figures.
