# B-409: Numbers' and Timecode's words, drawn faster (D-529)

**After measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b409_words --no-run` and run once with `--ignored b409_words_timing`, which times every row in that one run. Before the run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 3.7 GB of its memory held by the desktop and open programs); the same after it. The processor was 2% busy or less. The owner's app was not open. One run, single figures, not medians of rounds. Machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan, 16.8 GB), AMD64 Family 26 Model 68 (24 threads), Windows, release build.

The **before** columns (B-408's engine, 1acc4201) and the scratch split below were not built again: they are still the run of 2026-10-11, when other lanes' tests and builds (b155_gpu_chain, b398_warp_blend, b348_channel_threshold, cargo and rustc) were running, and stay PROVISIONAL.

Earlier provisional **after** figures (2026-10-11, at cdae1e00, on a machine that was not quiet), first: Noise alone 7.2; random number 370.2; long date 20.7; Numbers timecode 50.8; Timecode composition 158.2; Timecode drop frame on a box 121.0 ms.

## First, where the time went (B-408's engine)

Before any change, a scratch measurement (on the busy machine) split the long date ("Thursday, December 15, 2005" at size 120) by spacing and by stroke, every eighth frame of 240, two loops, milliseconds a frame:

| Shot | First | Again |
|---|---:|---:|
| Noise alone | 18.4 | 4.9 |
| long date, proportional, fill only | 41.6 | 2.9 |
| long date, monospaced, fill only | 607.0 | 10.1 |
| long date, proportional, stroke over fill 6 | 132.5 | 3.7 |
| long date, monospaced, stroke over fill 6 | 1371.2 | 5.6 |
| one digit, monospaced, stroke over fill 6 | 89.4 | 3.7 |
| timecode 30, monospaced, size 200, fill only | 252.2 | 3.9 |

Proportional spacing draws the line as one text; monospaced (proportional spacing off, and every Timecode) draws each character as its own text, each a whole 1920 by 1080 picture laid over the next one by one. That per-character whole-frame work is where the time went, fill and stroke alike; the stroke roughly doubles it. So the change goes there.

## Before and after

**What is timed**, B-406's and B-407's way: a 1920 by 1080 still at Full with Draw on: GPU, the effect then a Noise that changes every frame, so no finished frame is kept; every eighth frame of 240 through a viewer's cache of 1 GiB, **first** the median of the first loop's 30 frames from empty caches, **again** the median of the next seven loops' 210 frames. Milliseconds a frame.

| Shot | Before: first (busy) | Before: again (busy) | After: first | After: again |
|---|---:|---:|---:|---:|
| Noise alone | 27.1 | 3.7 | 5.4 | 2.0 |
| Numbers, monospaced random number with three places, stroke over fill 6, size 120 (new words every frame), then Noise | 408.9 | 3.1 | 47.8 | 2.0 |
| Numbers, monospaced long date, stroke over fill 6, size 120 (the same words every frame), then Noise | 734.7 | 2.7 | 13.6 | 2.1 |
| Numbers, timecode 30 at size 200 (proportional), then Noise | 48.6 | 3.5 | 20.7 | 2.1 |
| Timecode, composition source at size 200, then Noise | 501.3 | 4.4 | 45.2 | 2.1 |
| Timecode, custom drop frame on a half-opacity box, composite off, then Noise | 360.2 | 5.4 | 36.9 | 1.9 |

**Reading it.** On a quiet machine, a frame not drawn before (the "first" column) costs, after B-409: about 8 ms over Noise alone for the stroked monospaced long date, the same words on every frame (13.6 against 5.4), whose words are drawn once and kept; about 15 ms for the proportional timecode (20.7); about 31 and 40 ms for Timecode, every character its own text drawn over only its own rows (36.9 on a box, 45.2 composition source); and about 42 ms for a monospaced random number, new words every frame (47.8). Every frame drawn again costs nothing measurable (1.9 to 2.1 against 2.0). The order of costs is the busy run's, but every "after" figure is far lower than the busy run gave (the random number most: 370 there, 48 here), so the busy run overstated what was left; the "before" figures were not measured again, so how much B-409 itself gained cannot be put as a quiet factor. New frames are still above EFFECTS.md's Target P2 (4 ms added). The pictures are bit for bit those of B-408's engine (verification/D-529_words_table.md).
