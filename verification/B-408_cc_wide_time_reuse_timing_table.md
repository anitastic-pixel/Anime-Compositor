# B-408: CC Wide Time keeps the frames it draws (D-528)

**PROVISIONAL.** Measured on 2026-10-11 with the same test, `--ignored b400_cc_wide_time_timing`, built with `cargo test --release --test b400_cc_wide_time`: **before** from B-408's test on B-407's engine (516fd328, each frame of the span drawn afresh), **after** from 1acc4201 (B-408's code), run one after the other. Machine: RTX 4070 Ti SUPER, Ryzen 9 9900X, Windows 11; release build; Draw on: GPU; the viewer's cache 16521 MB (Memory: Automatic). Both runs started and ended NOT quiet (other lanes' b155, b426 and b107 test programs were running), so every figure is PROVISIONAL. Before is the same test program built on B-407's engine (each frame of the span drawn afresh); after is B-408's engine (frames kept for the next frame). The owner's app was not touched.

**What is timed**, B-400's shot: the reference shot (1920 by 1080, 24 frames a second) at Full with Draw on: GPU, each frame asked for whole as the viewer asks. On each of the shot's first three layers CC Wide Time, then a Noise that changes every frame, so no finished frame is kept. The first three rows are B-400's: every eighth frame of 240 through a viewer's cache of 1 GiB, **first** the median of the first loop's 30 frames from empty caches, **again** the median of the next seven loops' 210 frames. The two **played** rows ask for frames 0 to 47 in order, as playing does, twice through one cache at Memory: Automatic: **first** the median of the first pass, **again** of the second. Milliseconds a frame.

| Shot | Before: first | Before: again | After: first | After: again |
|---|---:|---:|---:|---:|
| Noise alone | 19.1 | 14.1 | 55.2 | 21.3 |
| CC Wide Time as added (3 and 3, 7 frames), then Noise | 319.9 | 351.5 | 464.4 | 839.1 |
| CC Wide Time 10 and 10 (21 frames), then Noise | 873.9 | 1086.6 | 2310.2 | 1518.4 |
| Noise alone, played | 43.8 | 10.4 | 53.2 | 7.3 |
| CC Wide Time as added, then Noise, played | 1038.5 | 882.0 | 298.4 | 289.8 |

Played rows: frames 0 to 47 in order, twice, the viewer's cache 16521 MB (Memory: Automatic).

**Reading it.** The rows that step every 8th frame cannot gain from B-408: with 3 frames before and 3 after, two frames 8 apart share no frame of their spans, so nothing kept is used again, and their before/after difference is the busy machine (Noise alone itself went from 19.1 to 55.2 ms between the two runs). The played rows are the case B-408 is for: frames 0 to 47 in order, where each frame shares 6 of its 7 span frames with the frame before. There CC Wide Time as added, then Noise, went from 1038.5 / 882.0 ms a frame to 298.4 / 289.8 ms a frame, about three times faster, while Noise alone played stayed near 50 / 10 ms. It is still above the row's Target P4 (100 ms a frame) when played, because each new frame still draws one new span frame and blends all seven at full size on the processor.
