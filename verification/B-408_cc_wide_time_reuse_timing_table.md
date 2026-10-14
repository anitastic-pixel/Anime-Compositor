# B-408: CC Wide Time keeps the frames it draws (D-528)

**After measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b400_cc_wide_time --no-run` and run once with `--ignored b400_cc_wide_time_timing`, which times every row below in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (4% busy; 3.7 GB of its memory held by the desktop and open programs); the same after it. The processor was 2% busy or less. The owner's app was not open. One run, single figures, not medians of rounds. The **before** columns (B-407's engine, 516fd328, each frame of the span drawn afresh) were not built again: they are still the busy run of 2026-10-11, when other lanes' b155, b426 and b107 test programs were running, and stay PROVISIONAL.

Earlier provisional **after** figures (2026-10-11, at 1acc4201, on a machine that was not quiet), again: Noise alone 21.3; as added 839.1; 10 and 10 1518.4; Noise alone played 7.3; as added played 289.8 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed**, B-400's shot: the reference shot (1920 by 1080, 24 frames a second) at Full with Draw on: GPU, each frame asked for whole as the viewer asks. On each of the shot's first three layers CC Wide Time, then a Noise that changes every frame, so no finished frame is kept. The first three rows are B-400's: every eighth frame of 240 through a viewer's cache of 1 GiB, **first** the median of the first loop's 30 frames from empty caches, **again** the median of the next seven loops' 210 frames. The two **played** rows ask for frames 0 to 47 in order, as playing does, twice through one cache at Memory: Automatic: **first** the median of the first pass, **again** of the second. Milliseconds a frame.

| Shot | Before: first (busy) | Before: again (busy) | After: first | After: again |
|---|---:|---:|---:|---:|
| Noise alone | 19.1 | 14.1 | 15.6 | 12.2 |
| CC Wide Time as added (3 and 3, 7 frames), then Noise | 319.9 | 351.5 | 271.7 | 312.0 |
| CC Wide Time 10 and 10 (21 frames), then Noise | 873.9 | 1086.6 | 747.5 | 758.4 |
| Noise alone, played | 43.8 | 10.4 | 17.9 | 5.5 |
| CC Wide Time as added, then Noise, played | 1038.5 | 882.0 | 94.2 | 74.1 |

Played rows: frames 0 to 47 in order, twice, the viewer's cache 16521 MB (Memory: Automatic).

**Reading it.** The rows that step every 8th frame cannot gain from B-408: with 3 frames before and 3 after, two frames 8 apart share no frame of their spans, so nothing kept is used again; quiet, they cost about the same as the busy "before" run. The played rows are the case B-408 is for: frames 0 to 47 in order, where each frame shares 6 of its 7 span frames with the frame before. Quiet, CC Wide Time as added, then Noise, played costs 94.2 / 74.1 ms a frame, against 271.7 / 312.0 when stepping every 8th frame, about a quarter. (The busy "before" played figures, 1038.5 / 882.0, were too disturbed to give a fair factor against.) This changes the busy run's reading: played, CC Wide Time is now within the row's Target P4 (100 ms a frame), though each new frame still draws one new span frame and blends all seven at full size on the processor. B-400's own quiet run of the same test gives the same figures.
