# B-409: Numbers' and Timecode's words, drawn faster (D-529)

**PROVISIONAL.** Measured on 2026-10-11 with the same test, `--ignored b409_words_timing`, built with `cargo test --release --test b409_words`: **before** from that test on B-408's engine (1acc4201), **after** from cdae1e00 (B-409's code), run one after the other. Machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11, release build. The machine was not quiet: other lanes' tests and builds (b155_gpu_chain, b398_warp_blend, b348_channel_threshold, cargo and rustc) ran through both runs, so every number here is PROVISIONAL. The owner's app was not touched.

## First, where the time went (B-408's engine)

Before any change, a scratch measurement split the long date ("Thursday, December 15, 2005" at size 120) by spacing and by stroke, every eighth frame of 240, two loops, milliseconds a frame:

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

| Shot | Before: first | Before: again | After: first | After: again |
|---|---:|---:|---:|---:|
| Noise alone | 27.1 | 3.7 | 7.2 | 2.8 |
| Numbers, monospaced random number with three places, stroke over fill 6, size 120 (new words every frame), then Noise | 408.9 | 3.1 | 370.2 | 2.9 |
| Numbers, monospaced long date, stroke over fill 6, size 120 (the same words every frame), then Noise | 734.7 | 2.7 | 20.7 | 3.0 |
| Numbers, timecode 30 at size 200 (proportional), then Noise | 48.6 | 3.5 | 50.8 | 3.3 |
| Timecode, composition source at size 200, then Noise | 501.3 | 4.4 | 158.2 | 3.6 |
| Timecode, custom drop frame on a half-opacity box, composite off, then Noise | 360.2 | 5.4 | 121.0 | 3.2 |

**Reading it.** The work went where the time went. The stroked monospaced long date, the same words on every frame, went from about 735 ms to about 21 ms on a frame not drawn before: its words are drawn once and kept. Timecode, every character its own text, went from about 501 to about 158 ms (composition source) and from about 360 to about 121 ms (custom drop frame on a box), new words each frame, each character now drawn over only its own rows. A monospaced random number, new words every frame, went from about 409 to about 370 ms: what is left is the drawing of the outlines and the stroke themselves, not the whole-frame work. The proportional timecode, one text, is unchanged within the noise (about 49 and 51 ms); Noise alone, which B-409 does not touch, moved with the machine's load (about 27 and 7 ms). Every frame drawn again stays at 3 to 5 ms. The pictures are bit for bit those of B-408's engine (verification/D-529_words_table.md).
