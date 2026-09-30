# B-160: faster export

Written by `cargo test --release --test b160_faster_export -- --ignored b160_table`, after `b160_every_export_hashed` and `b160_timing` have run for the old build (`B160_LABEL=before`, compiled from the commit before B-160) and for this one.

**D-231 (bit-exact):** an export now draws several frames at once and writes them in order. Every file it writes must be the file the old build wrote, byte for byte, and every report the same report. **D-230 (REJECTED by the owner on 2026-09-29):** the graphics card's video encoder was no faster on this machine and wrote bigger files, so B-160b removed it; every MP4 comes from the software encoder, as before B-160.

## Checks: 18 of 18 pass

| Check | Expected | Actual | Result |
|---|---|---|---|
| lines hashed: every file and every report, old build and new | 28685 | 28685 | pass |
| the same files, named the same, in the same order | the same | the same | pass |
| files and reports whose bytes changed (the MP4 compared with its dates set aside) | 0 of 28684 | 0 of 28684 | pass |
| the same build, the same MP4 twice, a second apart: whole files, then dates set aside | different, then the same | different, then the same | pass |
| reference shot PNG 8-bit, frames 0 to 23: drawn 5 at once, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot PNG 8-bit, frames 0 to 23: drawn as many as the export picks, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot PNG 16-bit premultiplied, frames 0 to 23: drawn 5 at once, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot PNG 16-bit premultiplied, frames 0 to 23: drawn as many as the export picks, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot EXR half, frames 0 to 23: drawn 5 at once, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot EXR half, frames 0 to 23: drawn as many as the export picks, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot EXR float, frames 0 to 23: drawn 5 at once, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot EXR float, frames 0 to 23: drawn as many as the export picks, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot GIF, frames 0 to 23: drawn 5 at once, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot GIF, frames 0 to 23: drawn as many as the export picks, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot animated PNG, frames 0 to 23: drawn 5 at once, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot animated PNG, frames 0 to 23: drawn as many as the export picks, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot MP4, frames 0 to 23: drawn 5 at once, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot MP4, frames 0 to 23: drawn as many as the export picks, against one at a time | the same bytes and report | the same bytes and report | pass |

## Measurements (not pass or fail)

| What | Measured |
|---|---|
| fixture projects exported, every composition, as PNG and EXR half | 2296 |

## Speed, median of 7, three rounds

Measured again on 2026-09-30 with no other cargo process or build running (the quiet re-measure
that ended the GPU plan; the first figures, 79980 ms old and 11998 ms new for the PNG export,
were taken beside two other agents' builds). `b160_timing` run on the old build (9624, the commit
before B-160, in its own folder with its own build) and on today's program (c9bf93e), and
`b160_one_at_a_time_timing` on today's, turn about, three rounds. Each round is itself the median
of 7 whole exports; the figure below is the median of the three rounds.

- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads, 64 GB
- Card: NVIDIA GeForce RTX 4070 Ti SUPER, driver 610.88 (an export does not use it)
- System: Windows 11; release build (opt-level 3)

| Export of the whole reference shot, 240 frames at 1920x1080 | Round 1 | Round 2 | Round 3 | Median |
|---|---:|---:|---:|---:|
| reference shot PNG 8-bit, old build, one frame at a time | 78341 ms | 78654 ms | 78333 ms | **78341 ms** |
| reference shot MP4, old build, one frame at a time | 19479 ms | 18763 ms | 19152 ms | **19152 ms** |
| reference shot PNG 8-bit, new build, several frames at once | 11713 ms | 11889 ms | 11594 ms | **11713 ms** |
| reference shot MP4, new build, several frames at once | 12951 ms | 13354 ms | 13285 ms | **13285 ms** |
| reference shot PNG 8-bit, new build told to draw one frame at a time | 78442 ms | 77067 ms | 78690 ms | **78442 ms** |

**What it shows.** The PNG export of the reference shot takes 11.7 s instead of 78.3 s, about 6.7
times as fast; the MP4 13.3 s instead of 19.2 s, where the software video encoder, which takes one
frame after another, is most of what is left. The new build told to draw one frame at a time takes
78.4 s, the same as the old build (78.3 s): the 93.7 s first measured here (97.4 s in D-231) was
the other builds on the machine, not a cost of the new way of exporting.
