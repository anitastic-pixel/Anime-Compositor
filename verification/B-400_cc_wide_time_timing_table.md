# B-400: frame times with CC Wide Time (D-520)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b400_cc_wide_time --no-run` and run once with `--ignored b400_cc_wide_time_timing`, which times every row below in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 4.3 GB of its memory held by the desktop and open programs); after it no other build or test was running, but the card was 56% busy with another program (the timing test had ended). The processor was 2% busy or less before each run. The owner's app was not open. One run, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone 14.5; CC Wide Time as added 474.0; CC Wide Time 10 and 10 1194.2 ms. That run had no played rows.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

CC Wide Time is new, so there is no "before" build: the row without it is the comparison. It is
drawn by the processor, as Echo and Moment Map are (several frames of the layer).

**What is timed**, B-227's way: the reference shot (1920 by 1080, 24 frames a second), every
eighth frame of 240, at Full, with Draw on: GPU, each frame asked for whole as the viewer asks.
On each of the shot's first three layers: CC Wide Time, then a Noise that changes every frame,
so no finished frame is kept. **First** is the median of the first loop's 30 frames, started
with empty caches; **Again** the median of the next seven loops' 210 frames. The test now also
times two **played** rows: frames 0 to 47 in order, twice, as playback asks for them.
Milliseconds a frame.

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 16.5 | 12.3 |
| CC Wide Time as added (3 and 3, 7 frames), then Noise | Full | 267.7 | 292.9 |
| CC Wide Time 10 and 10 (21 frames), then Noise | Full | 728.9 | 721.8 |
| Noise alone, played | Full | 17.1 | 5.3 |
| CC Wide Time as added, then Noise, played | Full | 103.1 | 74.4 |

Played rows: frames 0 to 47 in order, twice, the viewer's cache 16521 MB (Memory: Automatic).

**Reading it.** Asked for every eighth frame, as added, seven frames of each layer are drawn for
every frame: about 94 ms a 1080p layer (12.3 to 292.9, three layers), so the frame (292.9) is
still above EFFECTS.md's Target P4 for processor-only time effects (100 ms a frame for the whole
frame). Twenty-one frames
(10 and 10) take about 2.5 times as long, about 237 ms a layer. Played frame after frame, the
frames of the span overlap, and a frame costs about 23 ms a layer (74.4 against 5.3), about a
quarter of the every-eighth-frame cost. The busy run's figures (474.0 and 1194.2, with "again"
slower than "first") were the other lane's test; quiet, "again" and "first" agree.
